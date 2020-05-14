use c_closures_build::{c_closure_header_include_dir, enhance_closure_bindings};
use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("example.h")
        // In the event of a C/C++ compile error this will provide more info
        .clang_arg("-v")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(true)
        .derive_copy(false)
        .generate_inline_functions(false)
        .clang_arg(format!("-I{}", c_closure_header_include_dir().display()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .to_string();
    println!("cargo:rerun-if-changed=example.h");
    let bindings = enhance_closure_bindings(&bindings);

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap()
        .write_all(bindings.as_bytes())
        .expect("Couldn't write bindings!");
    cc::Build::new()
        .include(c_closure_header_include_dir())
        .file("example.c")
        .compile("example");
}
