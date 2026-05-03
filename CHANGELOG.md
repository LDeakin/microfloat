# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/LDeakin/microfloat/compare/v0.1.2...HEAD)

### Added

- Add const `next_up()` and `next_down()` methods to all float types

### Changed

- Make the following methods `const` on all float types:
  - `abs()`
  - `clamp()`
  - `classify()`
  - `classify_bits()`
  - `copysign()`
  - `is_finite()`
  - `is_infinite()`
  - `is_infinity_bits()`
  - `is_nan()`
  - `is_normal()`
  - `max()`
  - `min()`
  - `signum()`
  - `total_cmp()`

## [0.1.2](https://github.com/LDeakin/microfloat/releases/tag/v0.1.2) - 2026-04-29

### Added

- Add `has_inf()`, `has_nan()`, and `is_finite_only()` const methods to all float types for querying format properties
- Add additional tests against the OCP spec
- Document and test differences from the `float8` crate

## [0.1.1](https://github.com/LDeakin/microfloat/releases/tag/v0.1.1) - 2026-04-28

### Added

- Enable trusted publishing
- Enable semver checks in CI
- Add `documentation` to `Cargo.toml`

### Fixed

- Fix WASM build

## [0.1.0](https://github.com/LDeakin/microfloat/releases/tag/v0.1.0) - 2026-04-28

### Added

- Initial release
