# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.2](https://github.com/hseeberger/configured/compare/v0.9.1...v0.9.2) - 2026-07-16

### Other

- *(deps)* bump dtolnay/rust-toolchain
- *(deps)* bump taiki-e/install-action from 2.82.11 to 2.83.0
- *(deps)* bump taiki-e/install-action in the ci-patches group
- *(deps)* bump taiki-e/install-action in the ci-patches group
- *(deps)* bump taiki-e/install-action in the ci-patches group
- add coverage for non-string field types
- *(deps)* bump Rust to 1.97.0

## [0.9.1](https://github.com/hseeberger/configured/compare/v0.9.0...v0.9.1) - 2026-07-07

### Other

- re-export Case from config instead of convert_case

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

## [0.7.1](https://github.com/hseeberger/configured/compare/v0.7.0...v0.7.1) - 2023-11-05

### Other
- Revert "build(release): configure pre-release-commit-message"
- add release-plz
