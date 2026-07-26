# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/stdex/slint-tree-view/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/stdex/slint-tree-view/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/stdex/slint-tree-view/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/stdex/slint-tree-view/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/stdex/slint-tree-view/releases/tag/v0.1.0
