# Changelog

All notable changes to the `tpt-engineering` workspace and its `tpt-eng-*`
crates are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 14 new domain crates (Phase 9, `spec2.txt`): `tpt-eng-props-mixture`,
  `tpt-eng-electrical`, `tpt-eng-schedule`, `tpt-eng-biomech`,
  `tpt-eng-crystallography`, `tpt-eng-geotech`, `tpt-eng-heat-transfer`,
  `tpt-eng-vehicle-dynamics`, `tpt-eng-power-components`, `tpt-eng-pcb`,
  `tpt-eng-renewables`, `tpt-eng-building-sys`, `tpt-eng-thermal-mgmt`,
  `tpt-eng-unit-ops`. Not yet published to crates.io — see
  `PUBLISH_TRACKING.md` (Batches 6-8).

### Changed
- `tpt-eng-tolerance` dropped the `rand_distr` dependency in favour of the
  `Uniform` distribution now provided by `rand::distributions`.

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
