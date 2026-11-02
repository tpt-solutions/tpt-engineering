# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking:** `three_phase_power` returns `Option<(f64, f64)>`, not `(f64, f64)`,
  and takes a signed power factor (negative = leading). It returns `None` for non-finite
  or out-of-range inputs. Under the pre-1.0 rules in `docs/STABILITY.md` this needs a
  minor bump, so the release is 0.2.0, not 0.1.1. `cargo semver-checks` did not flag
  this change, so the review was done by reading the diff.

### Fixed

- `skin_effect_ratio` is physical at high frequency.
- `Complex::div`, `impedance_*` and `PerUnitSystem` are guarded against zero input, instead of silently returning inf or NaN.
- `three_phase_power` reports lead or lag from the sign, and checks the power-factor range.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-19

### Added

- Initial release of `tpt-eng-electrical`: electrical primitives — complex impedance/reactance (series/parallel R/L/C combination), per-unit system conversions, balanced three-phase power, and conductor/insulator property lookups with `DataSource` provenance, built on `tpt-math-units` and `tpt-math-numeric`.
