# Changelog

All notable changes to the `tpt-engineering` workspace and its `tpt-eng-*`
crates are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

## [0.1.0] — pending

Initial 13-crate release of applied engineering primitives:
`props-{water,air,fuels,props}`, `timeseries-{core,align,gap,timeseries}`,
`geo-{asset,topology}`, `network-matrix`, `controls`, `structural`.

[Unreleased]: https://github.com/tpt-solutions/tpt-engineering/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/tpt-solutions/tpt-engineering/releases/tag/v0.1.0
