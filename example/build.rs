use c_closures_build::{c_closure_header_include_dir, BindgenBuilderExt};
use std::{env, path::PathBuf};

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
        .c_closures_enhancements() // c_closures custom extension to the bindgen builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cc::Build::new()
        .include(c_closure_header_include_dir())
        .file("example.c")
        .compile("example");
}
