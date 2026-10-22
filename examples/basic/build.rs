// Compiles the example's UI. The `@TreeView` import is resolved by the
// slint-compiler reading the `DEP_TREEVIEW_SLINT_LIBRARY_*` env vars
// that Cargo derives from the `slint-tree-view` crate's `links`
// declaration — no manual path wiring needed.
//
// The live preview (`SLINT_LIVE_PREVIEW=1`) is the exception: it compiles
// `ui/main.slint` again when the app starts, where those env vars are
// gone, so it needs the library's path up front. A normal build ignores
// it.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut library_paths = std::collections::HashMap::new();
    if let Ok(source) = std::env::var("DEP_TREEVIEW_SLINT_LIBRARY_SOURCE") {
        library_paths.insert("TreeView".to_string(), source.into());
    }
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config("ui/main.slint", config)?;
    Ok(())
}
