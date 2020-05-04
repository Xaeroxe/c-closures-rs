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
        .filter(|p| p.name == "c-closures" && p.version.major == 0 && p.version.minor == 3)
        .map(|p| p.manifest_path)
        .next()
        .unwrap();
    c_closure_manifest_path.pop();
    c_closure_manifest_path
}

/// Extends the `bindgen::Builder` with a function you'll need to call on it when generating bindings to
/// your own C/C++ code that include "rust_closures.h"
///
/// `fully_qualified_closure` is the complete path to the generated `Closure` type. Probably just "Closure"
/// but `enable_cxx_namespaces` can change that value.
pub trait BindgenBuilderExt: Sized {
    fn c_closures_enhancements(self, fully_qualified_closure: &str) -> Self;
}

impl BindgenBuilderExt for bindgen::Builder {
    fn c_closures_enhancements(self, fully_qualified_closure: &str) -> Self {
        let closure_path = fully_qualified_closure.rmatch_indices("::").next().map(|i| &fully_qualified_closure[0..i.0]);
        self.raw_line(format!(
            "impl ::c_closures::ClosureMarkerTrait for {} {{}}",
            fully_qualified_closure
        ))
        .raw_line(format!(
            "impl Drop for {} {{ fn drop(&mut self) {{ unsafe {{ {}closure_release(self) }} }} }}",
            fully_qualified_closure,
            closure_path.map(|s| format!("{}::", s)).unwrap_or_else(String::default),
        ))
        .clang_arg(format!("-I{}", c_closure_header_include_dir().display()))
    }
}
