#![deny(unsafe_code)]

/// Sentinel for an item with no parent (top-level). Used as the
/// `parent-internal-id` of root items.
pub const NO_PARENT: i32 = -1;

/// Generated Slint types live under this submodule so the consumer's
/// slint-compiler can find them at `slint_tree_view::tree_view::*` (the
/// path advertised by `rust_module("tree_view")` in `build.rs`).
//
// Slint's generated code uses `unsafe` for its vtable plumbing, so we
// locally allow it here — the deny-at-crate-root above still catches any
// hand-written `unsafe` elsewhere in the crate.
#[allow(unsafe_code)]
pub mod tree_view {
    slint::include_modules!();
}

// Convenience re-exports at the crate root.
pub use tree_view::{ActivationMode, TreeItem, TreeView, TreeViewStyle};

/// Convenience constructors for [`TreeItem`], pre-populating the fields
/// a typical host wants for each structural kind. The view itself stays
/// agnostic — `branch()` / `leaf()` / `section()` just save you writing
/// 11-field struct literals.
///
/// Naming: these use **structural** CS tree terms, not domain terms. The
/// view never knows whether an item is a "folder" or a "file" — that's
/// the host's call. So the constructors are named after the item's
/// *structural* role in the tree (branch = has children, leaf =
/// doesn't), and the host layers domain semantics (folder / mailbox /
/// project / category) on top via `item-type` and `decoration-text`.
impl TreeItem {
    /// A branch item — a node that holds children. `expanded` defaults
    /// to true (matches the "everything visible" convention most apps
    /// start with); pass `.collapsed()` for a collapsed branch.
    pub fn branch(
        internal_id: i32,
        parent_internal_id: i32,
        depth: i32,
        text: impl Into<slint::SharedString>,
    ) -> Self {
        Self {
            internal_id,
            parent_internal_id,
            depth,
            text: text.into(),
            decoration_text: "".into(),
            item_type: 0,
            user_data: String::new().into(),
            has_children: true,
            expanded: true,
            selectable: true,
            enabled: true,
        }
    }

    /// A leaf item — a node with no children. `user-data` is the opaque
    /// payload the host can read back via `model[current-index]`; the
    /// view never interprets it.
    pub fn leaf(
        internal_id: i32,
        parent_internal_id: i32,
        depth: i32,
        text: impl Into<slint::SharedString>,
        user_data: impl Into<slint::SharedString>,
    ) -> Self {
        Self {
            internal_id,
            parent_internal_id,
            depth,
            text: text.into(),
            decoration_text: "".into(),
            item_type: 0,
            user_data: user_data.into(),
            has_children: false,
            expanded: false,
            selectable: true,
            enabled: true,
        }
    }

    /// A non-selectable section header / separator. Visible but skipped
    /// by mouse selection and `current-changed`. Useful for grouping
    /// items under a category label.
    pub fn section(
        internal_id: i32,
        parent_internal_id: i32,
        depth: i32,
        text: impl Into<slint::SharedString>,
    ) -> Self {
        Self {
            internal_id,
            parent_internal_id,
            depth,
            text: text.into(),
            decoration_text: "".into(),
            item_type: 0,
            user_data: String::new().into(),
            has_children: false,
            expanded: false,
            // Sections are visible but not selectable.
            selectable: false,
            enabled: true,
        }
    }

    // ---- Builder methods (fluent overrides on the above defaults) ----

    /// Chain a decoration glyph onto an item (any of the constructors
    /// above). Alias of [`with_decoration`](Self::with_decoration).
    pub fn with_icon(self, icon: impl Into<slint::SharedString>) -> Self {
        self.with_decoration(icon)
    }

    /// Chain an arbitrary decoration string (the icon/glyph shown before
    /// the item's text).
    pub fn with_decoration(mut self, decoration: impl Into<slint::SharedString>) -> Self {
        self.decoration_text = decoration.into();
        self
    }

    /// Override the default `expanded = true` on a branch.
    pub fn collapsed(mut self) -> Self {
        self.expanded = false;
        self
    }

    /// Set `expanded` explicitly.
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Mark the item disabled (greyed out, ignored by mouse & keyboard).
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Mark the item non-selectable (visible, but `current-changed` is
    /// not emitted for it).
    pub fn non_selectable(mut self) -> Self {
        self.selectable = false;
        self
    }

    /// Set the application-defined `item-type` discriminator (lets the
    /// host distinguish folder/file/clip/whatever — the view doesn't
    /// interpret it).
    pub fn with_item_type(mut self, item_type: i32) -> Self {
        self.item_type = item_type;
        self
    }

    /// Set the opaque `user-data` payload (the host can read it back from
    /// `model[current-index]`; the view never does).
    pub fn with_user_data(mut self, user_data: impl Into<slint::SharedString>) -> Self {
        self.user_data = user_data.into();
        self
    }
}

// ---- Free-function helpers (the host does a lot of id↔index math) ----

/// Find the index of the first item with the given `internal-id`, or
/// `None`. Hosts need this constantly when handling TreeView callbacks
/// (which report ids) against their own `Vec<TreeItem>`.
pub fn index_of_id(items: &[TreeItem], internal_id: i32) -> Option<usize> {
    items.iter().position(|r| r.internal_id == internal_id)
}

/// Same lookup against a Slint `ModelRc<TreeItem>` (what `TreeView`'s
/// `get_model()` returns). The linear walk is fine for typical trees
/// (≤ a few thousand items); for huge trees the host should maintain its
/// own `HashMap<i32, usize>` side index.
pub fn index_of_id_model(model: &slint::ModelRc<TreeItem>, internal_id: i32) -> Option<usize> {
    use slint::Model as _;
    (0..model.row_count()).find(|&i| {
        model
            .row_data(i)
            .is_some_and(|r| r.internal_id == internal_id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_blank_leaf() {
        // Slint-derived Default: bool fields default to false. So a raw
        // `TreeItem::default()` is a non-selectable disabled leaf — hosts
        // should reach for the named constructors instead.
        let item = TreeItem::default();
        assert_eq!(item.depth, 0);
        assert_eq!(item.internal_id, 0);
        assert_eq!(item.parent_internal_id, 0);
        assert!(!item.has_children);
        assert!(!item.expanded);
    }

    #[test]
    fn branch_constructor_marks_has_children_and_expanded() {
        let item = TreeItem::branch(7, NO_PARENT, 0, "Inbox");
        assert_eq!(item.internal_id, 7);
        assert_eq!(item.parent_internal_id, NO_PARENT);
        assert_eq!(item.depth, 0);
        assert_eq!(item.text.as_str(), "Inbox");
        assert!(item.has_children);
        assert!(item.expanded, "branches default to expanded");
        assert!(item.selectable);
        assert!(item.enabled);
        assert_eq!(
            item.decoration_text.as_str(),
            "",
            "no default decoration — host decides"
        );
    }

    #[test]
    fn leaf_constructor_marks_no_children() {
        let item = TreeItem::leaf(3, 7, 1, "Welcome", "Hello world");
        assert_eq!(item.internal_id, 3);
        assert_eq!(item.parent_internal_id, 7);
        assert_eq!(item.depth, 1);
        assert_eq!(item.user_data.as_str(), "Hello world");
        assert!(!item.has_children);
        assert!(!item.expanded);
    }

    #[test]
    fn section_constructor_is_non_selectable() {
        let item = TreeItem::section(100, NO_PARENT, 0, "Archived");
        assert!(!item.selectable, "sections are skipped by selection");
        assert!(item.enabled, "sections are still visible/enabled");
        assert!(!item.has_children);
    }

    #[test]
    fn builder_methods_chain() {
        let item = TreeItem::branch(1, NO_PARENT, 0, "Inbox")
            .with_icon("📥")
            .collapsed()
            .with_item_type(42);
        assert_eq!(item.decoration_text.as_str(), "📥");
        assert!(!item.expanded, "collapsed() flips the default");
        assert_eq!(item.item_type, 42);
        assert!(
            item.has_children,
            "chaining must not clobber structural fields"
        );
    }

    #[test]
    fn disabled_and_non_selectable_chain() {
        let item = TreeItem::leaf(2, NO_PARENT, 0, "x", "y")
            .disabled()
            .non_selectable();
        assert!(!item.enabled);
        assert!(!item.selectable);
    }

    #[test]
    fn index_of_id_finds_first_match() {
        let items = [
            TreeItem::leaf(1, NO_PARENT, 0, "a", ""),
            TreeItem::leaf(2, NO_PARENT, 0, "b", ""),
            TreeItem::leaf(2, NO_PARENT, 0, "dup-of-b", ""),
        ];
        assert_eq!(index_of_id(&items, 1), Some(0));
        assert_eq!(index_of_id(&items, 2), Some(1), "first match wins");
        assert_eq!(index_of_id(&items, 99), None);
    }

    #[test]
    fn index_of_id_on_empty() {
        let items: [TreeItem; 0] = [];
        assert_eq!(index_of_id(&items, 1), None);
    }

    #[test]
    fn no_parent_is_minus_one() {
        // Pinned: the sentinel is part of the public contract — hosts
        // compare `parent_internal_id == NO_PARENT` to detect roots.
        assert_eq!(NO_PARENT, -1);
    }
}
