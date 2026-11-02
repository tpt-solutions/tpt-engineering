# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Added

- `pr_saturation_pressure_tagged`, `bubble_point_tagged`, and `dew_point_tagged`: provenance-tagged variants of the Peng–Robinson saturation pressure, bubble point, and dew point. Each returns the same value as its untagged counterpart, with the source, the documented validity ranges, and an out-of-range warning. Uses `tpt-eng-provenance`.

### Changed

- Example code formatted with `cargo fmt` (no functional change).
- Depends on `tpt-eng-provenance` 0.1.0.

## [0.1.0] - 2026-08-19

### Added

- Initial release of `tpt-eng-props-mixture`: general real-gas / vapor-liquid-equilibrium property lookups for arbitrary process mixtures via the Peng-Robinson equation of state (compressibility-factor `Z` roots, component fugacity coefficients, bubble/dew-point pressure) with van der Waals one-fluid mixing rules; `no_std`-capable via `tpt-math-numeric::libm`, re-exported by `tpt-eng-props` as `mixture`.
