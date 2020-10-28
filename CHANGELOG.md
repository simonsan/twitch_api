# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## Fixed
- Fix for top games parse error from [@MarkJGx](https://github.com/MarkJGx)

## Added
- Using Bors build bot (repository)
- Using Cargo release to release versions
- New Rust fmt settings using nightly toolchain
- Added example credentials for easier testing
- Added status for API versions in readme
- Adapt imports

## Changed
- Reorganize APIv5
- Changed crate type
- Adopting GH actions to nightly toolchain as well
- Replaced hyper/hyper-rustls with reqwest crate

## Removed
- Used thiserror and removed removed self-written Error trait implements
- removed outdated "extern crate" due to 2018 edition 

## [0.1.2] - 2019-10-16
### Added
- Added some useful information to Readme
- Documentation on docs.rs


## [0.1.1] - 2019-10-16
### Added
- Badges for Readme
- Checks with Github actions for formatting and lintering
- Added functions to read write credentials from [@lavisheng](https://github.com/lavisheng)
- Integration test for credentials

### Changed
- More formatting
- r#try is replaced by `?` operator
- ignore expensive tests

## [0.1.0] - 2019-10-08
### Added
- Using Github actions
- Formatting with Rustfmt

### Changed
- Started SemVer from beginning, overtaking repository
- License changed to AGPL-3.0
- Replaced try! with raw identifier syntax r#try!


[Unreleased]: https://github.com/age-rs/libtwitch-rs/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/age-rs/libtwitch-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/age-rs/libtwitch-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/age-rs/libtwitch-rs/releases/tag/v0.1.0