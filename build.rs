#![allow(clippy::print_literal)]

fn main() {
    // TODO: We should skip bindgen for rust-analyzer, clippy, and cargo-doc
    #[cfg(feature = "bindgen")]
    do_bindgen();
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
        // This accepts all functions, but now bindgen will only consider
        // items needed by a function definition. This eliminates >80% of
        // the symbols typically found.
        .allowlist_function(".*")
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
