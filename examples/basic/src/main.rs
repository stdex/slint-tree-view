// Minimal slint-tree-view consumer.
//
// Builds a tiny tree (one folder + a few leaves), wires the four core
// callbacks, and lets the user drive the TreeView with mouse and
// keyboard. The host owns the model + current-index + expansion state;
// the TreeView just reports interactions and renders whatever the host
// gives back.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};
use slint_tree_view::{TreeItem, NO_PARENT};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // ---- 1. Build the initial model --------------------------------
    // The constructors take `impl Into<SharedString>`, so &str works
    // directly without an extra String allocation. The fluent builder
    // methods layer on the structural / theming fields.
    let items: Vec<TreeItem> = vec![
        TreeItem::branch(1, NO_PARENT, 0, "Inbox")
            .with_icon("📥"),
        TreeItem::leaf(2, 1, 1, "Welcome", "Hello from slint-tree-view!")
            .with_icon("✉️"),
        TreeItem::leaf(3, 1, 1, "Release notes", "v0.1.0 — first cut.")
            .with_icon("✉️"),
        TreeItem::branch(4, NO_PARENT, 0, "Drafts")
            .with_icon("📝")
            .collapsed(),  // start collapsed
        TreeItem::leaf(5, 4, 1, "Reply to Alice", "Sure, let's ship it."),
        TreeItem::section(100, NO_PARENT, 0, "Archived"),  // non-selectable
        TreeItem::leaf(6, NO_PARENT, 0, "Old draft", "…"),
    ];

    // The state the closures share: the flat `Vec<TreeItem>` and the
    // collapsed-set. In a real app you'd keep these in your data layer;
    // here we use a `Rc<RefCell<…>>` because the callbacks are `FnMut`.
    let state = Rc::new(RefCell::new(AppState::new(items)));

    // ---- 2. Build the window and push the initial model ------------
    let app = App::new()?;
    app.set_model(ModelRc::new(VecModel::from(
        state.borrow().flat_model(),
    )));

    // ---- 3. Wire the four core callbacks ---------------------------
    //
    // current-changed: TreeView reports a new current item by id. We
    //   don't need to do anything beyond letting current-index be
    //   mirrored (the .slint does that); here we just update the status
    //   line so the user sees the click registered.
    //
    let weak = app.as_weak();
    app.on_current_changed(move |id| {
        let Some(w) = weak.upgrade() else { return };
        w.set_status_text(format!("Current item: id={id}").into());
    });

    //
    // item-activated: double-click / Enter (per activation-mode). For
    //   this demo, dump the activated item's user-data into the status
    //   line so the user can see the payload round-trips.
    //
    let state_for_activate = Rc::clone(&state);
    let weak = app.as_weak();
    app.on_item_activated(move |id| {
        let Some(w) = weak.upgrade() else { return };
        let body = state_for_activate
            .borrow()
            .find_user_data(id)
            .unwrap_or_else(|| "<no user-data>".into());
        w.set_status_text(format!("Activated id={id}: {body}").into());
    });

    //
    // item-expand-requested / item-collapse-requested: the TreeView is
    //   fully controlled, so on expand/collapse we have to mutate our
    //   model (set `expanded` on the matching branch + re-flatten with
    //   the now-visible descendants) and push the new model back.
    //
    let state_for_expand = Rc::clone(&state);
    let weak = app.as_weak();
    app.on_item_expand_requested(move |id| {
        let Some(w) = weak.upgrade() else { return };
        state_for_expand.borrow_mut().set_expanded(id, true);
        w.set_model(ModelRc::new(VecModel::from(
            state_for_expand.borrow().flat_model(),
        )));
        w.set_status_text(format!("Expanded id={id}").into());
    });

    let state_for_collapse = Rc::clone(&state);
    let weak = app.as_weak();
    app.on_item_collapse_requested(move |id| {
        let Some(w) = weak.upgrade() else { return };
        state_for_collapse.borrow_mut().set_expanded(id, false);
        w.set_model(ModelRc::new(VecModel::from(
            state_for_collapse.borrow().flat_model(),
        )));
        w.set_status_text(format!("Collapsed id={id}").into());
    });

    app.run()
}

// ─────────────────────────── app state ─────────────────────────────

/// Owns the source-of-truth tree + a collapsed-set, and produces the
/// flat `[TreeItem]` the TreeView renders.
///
/// For clarity, the "tree" here is just the initial flat list — a real
/// app would keep a hierarchical structure and DFS-flatten it. The
/// point of this type is to show the round-trip: change `expanded`,
/// re-flatten, push back.
struct AppState {
    /// The full set of items, in DFS order, with their canonical
    /// `expanded` state. We mutate `expanded` in place when an expand/
    /// collapse request arrives.
    items: Vec<TreeItem>,
}

impl AppState {
    fn new(items: Vec<TreeItem>) -> Self {
        Self { items }
    }

    /// Flip `expanded` on the branch with the given id.
    fn set_expanded(&mut self, id: i32, expanded: bool) {
        for item in &mut self.items {
            if item.internal_id == id {
                item.expanded = expanded;
            }
        }
    }

    /// Produce the visible flat model: omit descendants of collapsed
    /// branches.
    ///
    /// This is the canonical slint-tree-view host pattern. The walk
    /// tracks the `depth` of the last still-open branch; when it hits
    /// an item whose `depth` is greater than the current open-depth,
    /// the item is inside a collapsed branch and is skipped.
    fn flat_model(&self) -> Vec<TreeItem> {
        let mut out = Vec::with_capacity(self.items.len());
        // The depth at which items are still visible. Starts unbounded
        // (everything at top level is visible). When we encounter a
        // collapsed branch, we record its depth as a "fence" — items
        // at greater depth are skipped until we see an item at or
        // below the fence's depth again.
        let mut hidden_below_depth: Option<i32> = None;

        for item in &self.items {
            match hidden_below_depth {
                Some(fence) if item.depth > fence => {
                    // Inside a collapsed branch — skip.
                    continue;
                }
                _ => {
                    // Either no fence, or we've emerged past it.
                    hidden_below_depth = None;
                }
            }
            // If this branch is collapsed, set the fence just below its
            // own depth so its children get filtered on the next loop.
            if item.has_children && !item.expanded {
                hidden_below_depth = Some(item.depth);
            }
            out.push(item.clone());
        }
        out
    }

    /// Look up an item's `user-data` by id (used by `item-activated` to
    /// show the payload round-trips). `SharedString::default()` if not
    /// found.
    fn find_user_data(&self, id: i32) -> Option<SharedString> {
        self.items
            .iter()
            .find(|i| i.internal_id == id)
            .map(|i| i.user_data.clone())
    }
}
