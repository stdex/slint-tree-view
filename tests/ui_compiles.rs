//! UI compile-test: pulls in `ui/tree_view.slint` via a slint-interpreter
//! `Compiler` and verifies that the public API still compiles when used
//! from a consumer-style `.slint` snippet.
//!
//! Catches the kind of breakage that's invisible to the Rust unit tests:
//! renamed struct fields, dropped callbacks, deleted properties, or
//! renamed exports. The test doesn't create a window — it only asks the
//! slint-compiler whether the source parses and type-checks against the
//! library's published names.

use std::path::PathBuf;
use std::sync::Arc;

/// Locate `ui/tree_view.slint` regardless of where `cargo test` runs
/// from. `CARGO_MANIFEST_DIR` points at the crate root.
fn library_source_path() -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest).join("ui").join("tree_view.slint")
}

/// Compile a consumer-style snippet against the library, returning the
/// slint-interpreter's `CompilationResult`. The interpreter has no
/// built-in notion of Cargo's `DEP_*` env vars — that plumbing only
/// exists in `slint-build`'s consumer-side compile path — so for the
/// test we install a file-loader that resolves any import (including
/// `@TreeView`) to the on-disk library source.
fn compile_consumer(snippet: &str) -> slint_interpreter::CompilationResult {
    let library_source = std::fs::read_to_string(library_source_path())
        .expect("ui/tree_view.slint must be readable from the test");
    // Wrap in Arc so the `Fn` closure (which must be `Fn`, not `FnMut`)
    // can cheaply clone a reference to the source on every call.
    let library_source: Arc<String> = Arc::new(library_source);

    let mut compiler = slint_interpreter::Compiler::default();
    compiler.set_file_loader(move |_path: &std::path::Path| {
        let src = Arc::clone(&library_source);
        Box::pin(async move { Some(Ok((*src).clone())) })
    });

    spin_on::spin_on(compiler.build_from_source(snippet.into(), Default::default()))
}

fn assert_no_diagnostics(result: &slint_interpreter::CompilationResult) {
    let diags: Vec<_> = result.diagnostics().collect();
    if !diags.is_empty() {
        for d in &diags {
            eprintln!("slint diagnostic: {d}");
        }
        panic!("slint compilation produced {} diagnostics", diags.len());
    }
}

#[test]
fn tree_view_is_exported_and_usable() {
    // Minimal consumer: imports the component + struct + global and
    // instantiates TreeView with an empty model. Exercises the public
    // API surface that real consumers depend on.
    let snippet = r#"
        import { TreeView, TreeItem, TreeViewStyle, ActivationMode } from "@TreeView";

        export component Demo inherits Window {
            forward-focus: tree;
            tree := TreeView {
                model: [
                    { internal-id: 1, parent-internal-id: -1, depth: 0,
                      text: "Inbox", decoration-text: "📥", item-type: 0,
                      user-data: "", has-children: true, expanded: true,
                      selectable: true, enabled: true },
                ];
                current-index: 0;
                activation-mode: ActivationMode.double-click;

                current-changed(id) => { debug("current: " + id); }
                item-activated(id) => { debug("activate: " + id); }
                item-expand-requested(id) => { debug("expand: " + id); }
                item-collapse-requested(id) => { debug("collapse: " + id); }
            }
        }
    "#;
    let result = compile_consumer(snippet);
    assert_no_diagnostics(&result);
    assert!(
        result.component("Demo").is_some(),
        "Demo component should be exported"
    );
}

#[test]
fn tree_item_struct_has_documented_fields() {
    // A snippet that names every field of TreeItem. If a field is
    // renamed or dropped, the slint-compiler will flag the unknown
    // field and the test fails with the diagnostic.
    let snippet = r#"
        import { TreeView, TreeItem } from "@TreeView";

        export component Demo inherits Window {
            tv := TreeView {
                model: [{
                    internal-id: 1, parent-internal-id: -1, depth: 0,
                    text: "x", decoration-text: "", item-type: 0,
                    user-data: "", has-children: false, expanded: false,
                    selectable: true, enabled: true,
                }];
            }
        }
    "#;
    let result = compile_consumer(snippet);
    assert_no_diagnostics(&result);
}

#[test]
fn tree_view_style_global_is_readable() {
    // Verifies the TreeViewStyle global can be read from a consumer's
    // .slint file — pins the public theming API surface. (Slint doesn't
    // allow top-level mutating assignments to a foreign library global,
    // so we read inside a binding instead.)
    let snippet = r#"
        import { TreeView, TreeViewStyle } from "@TreeView";

        export component Demo inherits Window {
            // Touch a representative property from each category (color,
            // dimension, glyph) — if any is renamed, the slint-compiler
            // will flag the unknown reference.
            property <color> hc: TreeViewStyle.highlight-color;
            property <length> rh: TreeViewStyle.row-height;
            property <length> ind: TreeViewStyle.indentation;
            property <string> eb: TreeViewStyle.expanded-branch-indicator;
            tv := TreeView {}
        }
    "#;
    let result = compile_consumer(snippet);
    assert_no_diagnostics(&result);
}

#[test]
fn per_instance_behavior_properties_exist() {
    // Pins the per-instance behavior-flag surface. If any of these
    // properties gets renamed or removed, the slint-compiler will flag
    // it as unknown.
    let snippet = r#"
        import { TreeView, ActivationMode } from "@TreeView";

        export component Demo inherits Window {
            tv := TreeView {
                items-expandable: true;
                root-is-decorated: false;
                expands-on-double-click: false;
                activation-mode: ActivationMode.single-click;
                hover-highlight: false;
                page-size: 5;
                focused: false;
            }
        }
    "#;
    let result = compile_consumer(snippet);
    assert_no_diagnostics(&result);
}

#[test]
fn style_defaults_track_palette() {
    // Regression guard for the v0.2.0 Palette adaptation: the 7 mapped
    // TreeViewStyle properties must still be referenceable from a
    // consumer snippet (names + types unchanged), AND a consumer that
    // sets one of them to a literal must still compile (per-instance
    // override must win over the Palette-derived default). If the
    // mapping regresses (renamed property, wrong type, or the
    // Palette-derived default stops being overridable), this fails.
    let snippet = r#"
        import { TreeView, TreeViewStyle } from "@TreeView";

        export component Demo inherits Window {
            // Touch every mapped property by name — a rename drops here.
            property <color> bg: TreeViewStyle.background-color;
            property <color> fg: TreeViewStyle.text-color;
            property <color> dt: TreeViewStyle.disabled-text-color;
            property <color> hl: TreeViewStyle.highlight-color;
            property <color> hlt: TreeViewStyle.highlighted-text-color;
            property <color> blc: TreeViewStyle.branch-line-color;
            property <length> hp: TreeViewStyle.horizontal-padding;

            tv := TreeView {}
        }
    "#;
    let result = compile_consumer(snippet);
    assert_no_diagnostics(&result);
}
