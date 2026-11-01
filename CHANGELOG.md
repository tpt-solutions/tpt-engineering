# Changelog

All notable changes to the `tpt-engineering` workspace and its `tpt-eng-*`
crates are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 14 new domain crates (Phase 9, `docs/spec2.txt`): `tpt-eng-props-mixture`,
  `tpt-eng-electrical`, `tpt-eng-schedule`, `tpt-eng-biomech`,
  `tpt-eng-crystallography`, `tpt-eng-geotech`, `tpt-eng-heat-transfer`,
  `tpt-eng-vehicle-dynamics`, `tpt-eng-power-components`, `tpt-eng-pcb`,
  `tpt-eng-renewables`, `tpt-eng-building-sys`, `tpt-eng-thermal-mgmt`,
  `tpt-eng-unit-ops`. Published to crates.io at `0.1.0` in Batches 6-8 — see
  `docs/PUBLISH_TRACKING.md`.

### Changed
- `tpt-eng-electrical::three_phase_power` now returns `Option` and takes a
  signed power factor (negative = leading); `tpt-eng-building-sys`,
  `tpt-eng-examples`, and `tpt-eng-power-components` updated to match.
- `tpt-eng-heat-transfer::nusselt_sphere` takes a `viscosity_ratio` argument
  (Whitaker `(μ/μ_s)^¼` term); `water_film_properties` returns `Option` and
  reports cp rather than enthalpy.
- `tpt-eng-props-mixture` and the `tpt-eng-props` umbrella are now no_std-clean
  (`libm` math, no allocation) and are in the no_std / wasm CI matrix.
- `tpt-eng-structural::Beam::try_add` added; `Beam::add` panics on loads outside
  the span.

### Documentation
- Crate-level examples added to all 42 library crates; `cargo xtask doctest-check`
  (run by `cargo xtask check`) fails if one is missing.
- `docs/CONVENTIONS.md` records the error strategy, unit convention and numeric
  precision rules.
- Planning and reference files moved to `docs/` (`spec.txt`, `spec2.txt`,
  `CRATE_AUDIT.md`, `PUBLISH_TRACKING.md`, `history/`).
- `tpt-engineering` umbrella docs now state it is not published.

### Fixed
- `tpt-eng-structural`: partial UDL moment double-counted the load for points
  beyond its end; zero-length UDLs produced NaN shear.
- `tpt-eng-unit-ops::gilliland_stages` now uses the Eduljee correlation and the
  `N = (N_min + Y)/(1 − Y)` inversion.
- `tpt-eng-pcb::microstrip_impedance` now uses Hammerstad–Jensen (continuous at
  `w/h = 1`); the previous closed form was ~30 % off there.
- `tpt-eng-props-air`: saturation pressure uses the over-ice form below 273.15 K;
  `dew_point` returns an error outside the search range instead of clamping.
- `tpt-eng-controls`: PID anti-windup integrated one extra step at the limit;
  `TransferFunction::new` rejects a zero leading denominator coefficient.
- `tpt-eng-renewables`: single-diode solve uses Newton iteration with a
  convergence check (was an unchecked fixed-point loop); `I_0` includes the T³
  term.
- `tpt-eng-reliability::miners_rule`: a block at or beyond failure now yields
  infinite damage instead of zero.
- `tpt-eng-tolerance::monte_carlo`: zero-tolerance dimensions no longer panic.
- `tpt-eng-electrical::skin_effect_ratio` no longer grows as `f` at high
  frequency.
- Guards for zero divisors and invalid domains in `tpt-eng-geotech`,
  `tpt-eng-power-components`, `tpt-eng-electrical` (capacitor, per-unit bases),
  and `tpt-eng-heat-transfer` (Re/Pr preconditions).
- `tpt-eng-electrical::impedance_parallel` treats a zero-impedance branch as a
  short (returns 0) instead of NaN.

## [0.1.0] - 2026-08-16

### Published (2026-08-15)
- All 29 `tpt-eng-*` crates from the initial workspace (13-crate Phase-0
  scope plus the structural/mechanical and geometry/CAD crates added
  post-Phase-4) released to crates.io at `0.1.0` — see `RELEASE.md` Batches
  1-5. No `v0.1.0` git tag has been cut yet.

### Added
- `tpt-eng-examples`: cross-crate integration scenario (`thermal_loop`)
  composing geo-topology → network-matrix → controls → timeseries → structural.
- `xtask doctest` and `xtask doc` commands.
- `tpt-eng-structural::Beam::max_bending_moment_with_resolution`.
- `SECURITY.md` vulnerability-reporting policy and supply-chain posture.
- `release.toml` (cargo-release) and root `justfile` onboarding targets.

### Changed
- `tpt-eng-props-air` now returns `Result` from `humidity_ratio`,
  `vapour_pressure_from_ratio`, `relative_humidity`, and `dew_point`, guarding
  against divide-by-zero and non-physical inputs (new `Error` enum).
- `tpt-eng-geo-asset::within_radius` now filters non-finite coordinates,
  matching `nearest`.
- `tpt-eng-network-matrix` no longer panics on dangling edge endpoints.
- CI: added `cargo-audit`, `wasm32-unknown-unknown`, and `docs`/`doctest`
  jobs; `cargo-deny` now denies unknown sources and yanked crates.
- `rustfmt.toml` no longer pins a conflicting `edition` (derived from
  `Cargo.toml`, workspace edition 2024).

Initial 13-crate release of applied engineering primitives:
`props-{water,air,fuels,props}`, `timeseries-{core,align,gap,timeseries}`,
`geo-{asset,topology}`, `network-matrix`, `controls`, `structural`.

[Unreleased]: https://github.com/tpt-solutions/tpt-engineering/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/tpt-solutions/tpt-engineering/releases/tag/v0.1.0
