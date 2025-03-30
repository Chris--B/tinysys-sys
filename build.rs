#![allow(clippy::print_literal)]
#![allow(unused)]

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // TODO: We should skip bindgen for rust-analyzer, clippy, and cargo-doc
    #[cfg(feature = "bindgen")]
    generate_sdk_rs("src/sdk.rs");

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "riscv32" {
        use_native_lib("c_sdk", "tinysys_sdk");
    }
}

fn cargo_rerun_if(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().display());
}

/// Writes a slice as the entire contents of a file, but only if the contents have changed.
///
/// This avoids marking files as changed when they are not.
///
/// ## Safety
/// Note: Does not handle filesystem race conditions! Not even a little!
fn write_if_different<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
    let path = path.as_ref();
    let contents = contents.as_ref();
    let old_contents = std::fs::read(path)?;

    cargo_rerun_if(path);

    if contents != old_contents {
        // Note: Sometimes the bindings are the same save for minor formatting changes.
        // This still triggers and I'm not sure how to get around that.
        eprintln!("Saving {} bytes to {}", contents.len(), path.display());
        std::fs::write(path, contents)
    } else {
        eprintln!(
            "Saving {} bytes to {}, but they're identical. Not updating.",
            contents.len(),
            path.display()
        );
        Ok(())
    }
}

fn use_native_lib(lib_path: &str, lib_name: &str) {
    let out_path = env::var("OUT_DIR").unwrap();
    let lib_filename = format!("lib{lib_name}.a");

    // Copy the library into the output folder and instruct cargo to link against it
    fs::copy(
        format!("{}/{}", lib_path, lib_filename),
        format!("{}/{}", out_path, lib_filename),
    )
    .unwrap_or_else(|err| {
        panic!(
            "Failed to copy native lib (\"{}\") to output directory: {}",
            lib_filename, err
        )
    });

    println!("cargo:rustc-link-lib={lib_name}");
    println!("cargo:rustc-link-search=native={}", out_path);
}

#[cfg(feature = "bindgen")]
fn generate_sdk_rs(rs_out: impl AsRef<Path>) {
    use walkdir::*;

    // Find .h and .c files, we'll need to track them for changes.
    let mut headers = vec![];
    let mut sources = vec![];
    for entry in WalkDir::new("c_sdk/SDK") {
        let entry: DirEntry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading filesystem entry {err:#?}");
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let name = entry.path().as_os_str();
        let name = name.to_string_lossy().to_string();
        let short_name = name.trim_start_matches("c_sdk/SDK/").to_string();

        if name.ends_with(".h") {
            cargo_rerun_if(&name);
            headers.push(short_name);
        } else if name.ends_with(".c") {
            cargo_rerun_if(&name);
            sources.push(short_name);
        } else {
            eprintln!("Ignoring {}", entry.path().display());
        }
    }
    headers.sort();
    sources.sort();
    drop(sources);

    // Generate our sdk.h file. This lets us handle changing header
    // names/counts while letting our main wrapping code not care.
    let mut sdk_h_lines = vec![
        r"// **NOTE**".into(),
        r"// This file is autogenerated to include all of the headers from the tinysys SDK.".into(),
        r"// See `update_rs.sh` for details".into(),
        r"//".into(),
        r"".into(),
    ];
    for header in &headers {
        sdk_h_lines.push(format!("#include <{}>", header));
    }
    sdk_h_lines.push("".into());
    write_if_different("src/include/sdk.h", sdk_h_lines.join("\n")).unwrap();

    // TODO: We could do this from Rust instead?
    //      export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(riscv64-unknown-elf-gcc -print-sysroot)"
    println!("cargo:rerun-if-env-changed={}", "BINDGEN_EXTRA_CLANG_ARGS");

    let bindings = bindgen::Builder::default()
        .header("src/include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .formatter(bindgen::Formatter::Rustfmt)
        .clang_args(["-I", "c_sdk/SDK"])
        .clang_args(["-x", "c++"])
        // NOTE: Keep these in sync with the flags in ./scripts/02-build_c_sdk.sh
        .clang_args([
            "-std=c++20".to_string(),
            "--target=riscv32".to_string(),
            "-D_LIBCPP_HAS_NO_THREADS".to_string(),
            "-march=rv32im_zicsr_zifencei_zfinx".to_string(),
            "-mabi=ilp32".to_string(),
            // TODO:
            // format!("--sysroot={riscv_toolchain}"),
        ])
        // ## Which things we generate bindings for
        // This accepts all functions, but now bindgen will only consider
        // items needed by a function definition. This eliminates >80% of
        // the symbols typically found.
        .allowlist_function(".*")
        // Capture C Macros
        // Note: Most (All?) macro functions have to be ported manually
        .allowlist_item("[A-Z]+[A-Z_0-9]+")
        // _ names are usually special and not part of the SDK we want
        .opaque_type("_.*")
        .blocklist_function("_.*")
        // block all libc functions
        .blocklist_function("[a-z0-9]+")
        // ## How we generate bindings for the above things
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .merge_extern_blocks(true)
        // # And go!
        .generate()
        // Note: If you're seeing this crash, make sure you're using update_rs.sh or otherwise telling bindgen uses your cross compiler.
        // It doesn't by default and likes to crash in **host** C headers with nonsense like:
        //      thread 'main' panicked at build.rs:40:10:
        //      Unable to generate bindings: ClangDiagnostic("/Library/Developer/CommandLineTools/usr/lib/clang/16/include/inttypes.h:21:15: fatal error: 'inttypes.h' file not found\n")
        .unwrap_or_else(|e| {
            println!("cargo:warning=Unable to generate sdk.rs: {e:#?}");
            panic!("Unable to generate bindings: {e:#?}");
        });

    let mut code = vec![];
    bindings
        .write(Box::new(std::io::Cursor::new(&mut code)))
        .expect("Couldn't write bindings!");

    let code = String::from_utf8(code).unwrap();
    let mut code: Vec<&str> = code.lines().collect();
    code.insert(0, "#![allow(bad_style)]");

    let code: String = code.join("\n");
    write_if_different(rs_out.as_ref(), code).unwrap();
}
