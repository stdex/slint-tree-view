# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Screen-reader support.** Navigating the tree with the keyboard used
  to announce nothing but the window title: the keyboard focus sat in an
  internal `FocusScope` with no accessible node of its own. The
  `TreeView` is now an accessible `tree` that hands the screen reader's
  focus to the current row, and rows report their selected, enabled and
  expanded state and their position.
- `TreeItem::label` and `.with_label()`: what a screen reader announces
  for the row instead of `text`, when not empty.

### Changed

- **Breaking:** `TreeItem` has a new `label` field, so Rust code that
  builds one with a struct literal has to set it (the constructors do).

## [0.3.1] — 2026-10-17

### Changed

- The `slint` dependency no longer enables Slint's default features. A
  widget library must not choose the application's window backend,
  renderers or accessibility stack, and because Cargo features only add
  up, it did: depending on this crate switched Slint's default
  `backend-winit` (X11 included) back on even for an application that
  builds for Wayland only. Applications pick those features on their own
  `slint` dependency — one that already depends on `slint = "1.17"` with
  its defaults sees no change.

## [0.3.0] — 2026-08-30

### Fixed

- **Theming from a consumer had no effect.** Both documented routes were
  broken: `TreeViewStyle::get(&window)` from Rust resolves to the
  consumer's own globals table rather than the one this library module
  reads, so its setters silently did nothing; and the `.slint`
  "override" syntax shown in the README (`TreeViewStyle { … }`) is not
  valid Slint — a consumer cannot re-open a global, it is a parse error.
  A consumer could therefore not change any color, dimension or glyph,
  while the README advertised "full theming".

### Added

- Every `TreeViewStyle` property is now mirrored as an `in property` on
  the `TreeView` component and read from there, so styling works per
  instance — from `.slint`, or through the generated `set_*` on the
  component. The global remains and still supplies the defaults, so
  existing code that never set it is unaffected.
- Test coverage pinning the whole per-instance property surface by name,
  and that the global is still exported.

### Changed

- README's theming section rewritten: it documented two routes that did
  not work.

## [0.2.1] — 2026-07-26

### Fixed
- `branch-line-color` default reverted to `transparent`. The v0.2.0
  change to `Palette.border` painted a gray strip down the left side
  of every indented item — `Palette.border` is a *border* color, not
  an indent-fill color, so it was the wrong slot.
- Hover no longer overrides the current (selected) item's background.
  Previously a hovered current item showed the pale-blue `hover-color`
  while keeping the white `highlighted-text-color` (now sourced from
  `Palette.selection-foreground` since v0.2.0) — producing unreadable
  light-on-light text. The selected row now keeps its
  `highlight-color`/`highlighted-text-color` pair when moused over;
  hover applies only to non-current items.

## [0.2.0] — 2026-07-26

### Changed
- `TreeViewStyle` semantic defaults now derive from Slint's `Palette` /
  `StyleMetrics` globals instead of hardcoded values:
  `background-color` → `Palette.background`,
  `text-color` → `Palette.foreground`,
  `disabled-text-color` → `Palette.alternate-foreground`,
  `highlight-color` → `Palette.selection-background`,
  `highlighted-text-color` → `Palette.selection-foreground`,
  `branch-line-color` → `Palette.border`,
  `horizontal-padding` → `StyleMetrics.layout-padding`.
- The widget now follows the host app's theme automatically, including
  light/dark mode via `Palette.color-scheme`. Per-instance overrides
  still win. **No public API change** — same properties, same types;
  this is a behavior-of-defaults change, hence the minor bump.

## [0.1.1] — 2026-07-26

### Changed
- Bump `rust-version` from `1.77` to `1.85` (the provable minimum for
  default features — set by `accesskit_winit`'s `edition = "2024"`
  manifest, which Cargo < 1.85 cannot parse).
- Shorten crate `description`.

### Removed
- Pinned-toolchain MSRV CI job. Slint's default `accessibility` feature
  pulls a Unix a11y stack (`accesskit` → `zbus`/`zvariant`/`atspi`)
  whose MSRV drifts upward on every release and isn't under our control;
  a pinned MSRV job broke on every lockfile drift. The remaining CI jobs
  run on `stable`, which tracks the floor implicitly.

## [0.1.0] — 2026-07-25

### Added
- Init

[Unreleased]: https://github.com/stdex/slint-tree-view/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/stdex/slint-tree-view/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/stdex/slint-tree-view/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/stdex/slint-tree-view/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/stdex/slint-tree-view/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/stdex/slint-tree-view/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/stdex/slint-tree-view/releases/tag/v0.1.0
