# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Fixed

- The PV fixed-point iteration checks for convergence.
- `saturation_current` includes the T³ term. PV outputs change.
- Wind cut-in, rated and cut-out speeds, and the cell constants, now carry their references.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-19

### Added

- Initial release of `tpt-eng-renewables`: renewable-energy component models — single-diode solar PV I-V/P-V curves, Betz-limited wind-turbine power curves, and Li-ion battery Weibull capacity-fade degradation, built on `tpt-eng-electrical`, `tpt-eng-props-air`, and `tpt-eng-reliability`.
