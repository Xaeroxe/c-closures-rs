use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

/// Provides the path containing `rust_closures.h`, this only works if a dependency on `c-closures` is declared
/// in your Cargo.toml. This path is automatically included by `BindgenBuilderExt::c_closures_enhancements`.
/// You'll need to include this path to compile any C/C++ code making use of this crate's `Closure` type.
pub fn c_closure_header_include_dir() -> PathBuf {
    let mut c_closure_manifest_path = MetadataCommand::new()
        .exec()
        .unwrap()
        .packages
        .into_iter()
        .filter(|p| p.name == "c-closures" && p.version.major == 0 && p.version.minor == 2)
        .map(|p| p.manifest_path)
        .next()
        .unwrap();
    c_closure_manifest_path.pop();
    c_closure_manifest_path
}

/// Extends the `bindgen::Builder` with a function you'll need to call on it when generating bindings to
/// your own C/C++ code that include "rust_closures.h"
pub trait BindgenBuilderExt: Sized {
    fn c_closures_enhancements(self) -> Self;
}

impl BindgenBuilderExt for bindgen::Builder {
    fn c_closures_enhancements(self) -> Self {
        self.raw_line("impl ::c_closures::ClosureMarkerTrait for Closure {}")
            .raw_line(
                "impl Drop for Closure { fn drop(&mut self) { unsafe { closure_release(self) } } }",
            )
            .clang_arg(format!("-I{}", c_closure_header_include_dir().display()))
    }
}
