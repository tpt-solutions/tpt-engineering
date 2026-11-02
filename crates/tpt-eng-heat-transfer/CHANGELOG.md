# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking:** `nusselt_sphere` takes a third argument, `viscosity_ratio`, for the
  Whitaker `(μ/μ_s)^¼` term. `cargo semver-checks` reports `function_parameter_count_changed`.
  Under the pre-1.0 rules in `docs/STABILITY.md` this needs a minor bump, so the release
  is 0.2.0, not 0.1.1.

### Fixed

- `water_film_properties` returns the specific heat capacity, as documented. It returned enthalpy before, so the second value changes.
- `water_film_properties` no longer panics outside the IF97 range.
- `nusselt_sphere` uses the Whitaker form with the viscosity-ratio term. Sphere Nusselt numbers change.
- The Nusselt correlations check the Reynolds and Prandtl number ranges they are valid for.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-19

### Added

- Initial release of `tpt-eng-heat-transfer`: heat-transfer correlations — Nusselt-number convection correlations (flat plate, cylinder, sphere, internal pipe flow; laminar and turbulent), 1D and radial conduction, grey-body radiation view-factor exchange, and series/parallel thermal-resistance network composition, using `tpt-eng-props-air` and `tpt-eng-props-water`.
