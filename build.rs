// slint-tree-view build script.
//
// Compiles `ui/tree_view.slint` as a Slint *library* named `TreeView` that
// other Slint projects can pull in via `import { ... } from "@TreeView"`.
//
// This uses the `experimental-module-builds` feature of `slint-build`
// (Slint ≥ 1.14, see https://www.kdab.com/building-reusable-slint-ui-libraries-with-rust-crates/).
// The build script emits `cargo::metadata=SLINT_LIBRARY_NAME=TreeView` (and
// friends), which the consuming crate's slint-compiler reads via the
// `DEP_<PKG>_SLINT_LIBRARY_NAME` env vars Cargo derives from this crate's
// name (uppercased, `-` → `_`).
//
// `rust_module("tree_view")` tells the *consumer's* slint-compiler which
// Rust module name to expose the library's types under when generating the
// consumer's `slint::include_modules!()` output. (Inside this crate the
// types are still re-exported at the lib root by `slint_build`.)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = slint_build::CompilerConfiguration::new()
        .as_library("TreeView")
        .rust_module("tree_view");
    slint_build::compile_with_config("ui/tree_view.slint", config)?;
    Ok(())
}
