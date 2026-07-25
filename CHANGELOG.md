# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/stdex/slint-tree-view/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/stdex/slint-tree-view/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/stdex/slint-tree-view/releases/tag/v0.1.0
