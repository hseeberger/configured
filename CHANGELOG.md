# Changelog

## [0.10.0](https://github.com/hseeberger/configured/compare/v0.9.2...v0.10.0) - 2026-09-14

### Features

- [**breaking**] Replace load(case) with load(LoadOptions) ([#91](https://github.com/hseeberger/configured/pull/91))

## [0.9.2](https://github.com/hseeberger/configured/compare/v0.9.1...v0.9.2) - 2026-08-23

### Documentation

- Turn on warnings for missing docs ([#66](https://github.com/hseeberger/configured/pull/66))

## [0.9.1](https://github.com/hseeberger/configured/compare/v0.9.0...v0.9.1) - 2026-07-07

### Refactor

- Re-export Case from config instead of convert_case ([#51](https://github.com/hseeberger/configured/pull/51))

## [0.9.0](https://github.com/hseeberger/configured/compare/v0.8.0...v0.9.0) - 2026-07-07

### Added

- [**breaking**] support configurable case styles via load(Case)

### Changed
- bump `config` from 0.14 to 0.15
- bump `thiserror` from 1.0 to 2.0
- upgrade to Rust edition 2024
- gate file formats behind the `yaml` (default) and `toml` features
- support configurable case styles: `load` now takes a `Case` (re-exported from `convert_case`) instead of being hard-coded to kebab-case
- change the default environment variable prefix from `APP` to `CFG`

## [0.8.0](https://github.com/hseeberger/configured/compare/v0.7.1...v0.8.0) - 2026-07-07

### Features

- [**breaking**] Gate formats behind yaml/toml features and bump config to 0.15 ([#45](https://github.com/hseeberger/configured/pull/45))

## [0.7.1](https://github.com/hseeberger/configured/compare/v0.7.0...v0.7.1) - 2023-11-05

### Other

- Revert "build(release): configure pre-release-commit-message"

This reverts commit f456d4cd6aaa920d62b1a45c013d7b5d91862ce4.

## [0.7.0](https://github.com/hseeberger/configured/compare/v0.6.2...v0.7.0) - 2023-11-05

### Bug Fixes

- Fix CONFIG_ENV_PREFIX spelling in constant ([#7](https://github.com/hseeberger/configured/pull/7))

## [0.6.1](https://github.com/hseeberger/configured/compare/v0.6.0...v0.6.1) - 2023-10-19

### Bug Fixes

- RustDoc

## [0.6.0](https://github.com/hseeberger/configured/compare/v0.5.0...v0.6.0) - 2023-10-19

### Features

- Overlays instead of single environment ([#4](https://github.com/hseeberger/configured/pull/4))

### Documentation

- Improve RustDoc and README

### Miscellaneous Tasks

- Badges, update GH workflows, dependabot ([#3](https://github.com/hseeberger/configured/pull/3))

## [0.5.0](https://github.com/hseeberger/configured/compare/v0.4.0...v0.5.0) - 2022-10-12

### Other

- Kebab-case ([#2](https://github.com/hseeberger/configured/pull/2))

## [0.4.0](https://github.com/hseeberger/configured/compare/v0.3.0...v0.4.0) - 2022-10-12

### Miscellaneous Tasks

- Updates

### Other

- Revert "feat: use kebab-case and single underscore separator"

This reverts commit 6168fd0c6602e8370e4f74cc3d7c86ae3ed8a917.

## [0.3.0](https://github.com/hseeberger/configured/compare/v0.2.0...v0.3.0) - 2022-09-07

### Features

- Use kebab-case and single underscore separator ([#1](https://github.com/hseeberger/configured/pull/1))

## [0.1.0](https://github.com/hseeberger/configured/releases/tag/v0.1.0) - 2022-06-08

### Features

- First cut

### Documentation

- Improve README and doc comments
