// Compiles the example's UI. The `@TreeView` import is resolved by the
// slint-compiler reading the `DEP_TREEVIEW_SLINT_LIBRARY_*` env vars
// that Cargo derives from the `slint-tree-view` crate's `links`
// declaration — no manual path wiring needed.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("ui/main.slint")?;
    Ok(())
}
