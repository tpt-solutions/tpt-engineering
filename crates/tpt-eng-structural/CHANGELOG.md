# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- `Beam::moment_at` counted a partial uniform load twice at positions beyond its end. Moments there change, and now match statics.
- A zero-length uniform load now acts as a point load. Previously it caused a divide-by-zero in `shear_at` and `moment_at`.
- Loads and spans outside the beam are rejected by `try_add`, with guards added on the span.

## [0.1.1] - 2026-08-20

### Added

- New examples: `basic` (a simply-supported beam's reactions, shear/moment diagram, and an allowable-stress section check) and `frame` (a single-bay portal frame analysed member by member, with a rafter bending check and a column axial+bending interaction check via `tpt-eng-safety`).

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-structural`: structural-engineering primitives — load definitions, simply-supported beam analysis (reactions, shear, and bending moment), and demand/capacity code-compliance checks in an ASCE 7 / Eurocode-style utilization-ratio form. Utilization/pass-fail evaluation delegates to `tpt-eng-safety`.
