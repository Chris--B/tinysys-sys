fn main() {
    #[cfg(feature = "bindgen")]
    do_bindgen();
}

#[cfg(feature = "bindgen")]
fn do_bindgen() {
    let bindings = bindgen::Builder::default()
        .header("src/include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .clang_args(["-I", "tinysys_c_sdk/SDK"])
        // This accepts all functions, but now bindgen will only consider
        // items needed by a function definition. This eliminates >80% of
        // the symbols typically found.
        .allowlist_function(".*")
        .generate()
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
