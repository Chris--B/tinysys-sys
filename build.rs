#![allow(clippy::print_literal)]

use std::env;
use std::fs;

fn main() {
    // TODO: We should skip bindgen for rust-analyzer, clippy, and cargo-doc
    #[cfg(feature = "bindgen")]
    do_bindgen();

    use_native_lib("tinysys_c_sdk", "tinysys_sdk");
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
fn do_bindgen() {
    for file in [
        "src/include/sdk.h",
        "src/include/wrapper.h",
        "tinysys_sys/tinysys_c_sdk/git-HEAD.txt",
    ] {
        println!("cargo:rerun-if-changed={}", file);
    }

    // TODO: We use this to set the sysroot when cross compiling (from update_rs.sh), but could probably do that logic here
    println!("cargo:rerun-if-env-changed={}", "BINDGEN_EXTRA_CLANG_ARGS");

    let blocklist_c_funcs_regex = format!("({})", BLOCKLIST_C_FUNCS.join("|"));

    let bindings = bindgen::Builder::default()
        .header("src/include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .clang_args(["-I", "tinysys_c_sdk/SDK"])
        .clang_args(["-x", "c++"])
        // NOTE: Keep these in sync with the flags in ./scripts/build_c_sdk.sh
        .clang_args([
            "-std=c++20",
            "-mcmodel=medany",
            "-march=rv32im_zicsr_zifencei_zfinx",
            "-mabi=ilp32",
        ])
        // This accepts all functions, but now bindgen will only consider
        // items needed by a function definition. This eliminates >80% of
        // the symbols typically found.
        .allowlist_function(".*")
        .allowlist_item("[A-Z]+[A-Z_]+")
        // _ names are usually special and not part of the SDK we want
        .opaque_type("_.*")
        .blocklist_function("_.*")
        .blocklist_function(blocklist_c_funcs_regex)
        .merge_extern_blocks(true)
        .generate()
        // Note: If you're seeing this crash, make sure you're using update_rs.sh or otherwise telling bindgen uses your cross compiler.
        // It doesn't by default and likes to crash in **host** C headers with nonsense like:
        //      thread 'main' panicked at build.rs:40:10:
        //      Unable to generate bindings: ClangDiagnostic("/Library/Developer/CommandLineTools/usr/lib/clang/16/include/inttypes.h:21:15: fatal error: 'inttypes.h' file not found\n")
        .expect("Unable to generate bindings");

    let mut code = vec![];
    bindings
        .write(Box::new(std::io::Cursor::new(&mut code)))
        .expect("Couldn't write bindings!");

    let code = String::from_utf8(code).unwrap();
    let mut code: Vec<&str> = code.lines().collect();
    code.insert(0, "#![allow(bad_style)]");

    let code: String = code.join("\n");

    // Only update the file (to the OS) if it REALLY changed,
    // to avoid unecessary build-system triggers.
    let old_code = std::fs::read_to_string("src/sdk.rs").unwrap();
    if old_code != code {
        std::fs::write("src/sdk.rs", code).unwrap();
    }
}

// Note: This is unused when bindgen is not enabled (which is most of the time)
#[allow(unused)]
const BLOCKLIST_C_FUNCS: &[&str] = &[
    // Block all math functions that use 128 bit ints. u128 isn't well behaved over FFI,
    // and we don't want to export a bunch of libc functions anyway.
    "acoshl",
    "acosl",
    "asinhl",
    "asinl",
    "atan2l",
    "atanhl",
    "atanl",
    "cbrtl",
    "ceill",
    "copysignl",
    "coshl",
    "cosl",
    "erfcl",
    "erfl",
    "exp2l",
    "expl",
    "expm1l",
    "fabsl",
    "fdiml",
    "finitel",
    "floorl",
    "fmal",
    "fmaxl",
    "fminl",
    "fmodl",
    "frexpl",
    "hypotl",
    "ilogbl",
    "ldexpl",
    "lgammal",
    "llrintl",
    "llroundl",
    "log10l",
    "log1pl",
    "log2l",
    "logbl",
    "logl",
    "lrintl",
    "lroundl",
    "modfl",
    "nanl",
    "nearbyintl",
    "nextafterl",
    "nexttoward",
    "nexttowardf",
    "nexttowardl",
    "powl",
    "remainderl",
    "remquol",
    "rintl",
    "roundl",
    "scalblnl",
    "scalbnl",
    "sinhl",
    "sinl",
    "sqrtl",
    "tanhl",
    "tanl",
    "tgammal",
    "truncl",
];
