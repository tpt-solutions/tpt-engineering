# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Added

- `saturation_pressure_water_tagged` and `relative_humidity_tagged`: provenance-tagged variants of `saturation_pressure_water` and `relative_humidity`. Each returns the same value as its untagged counterpart, with the source, the documented validity range, and an out-of-range warning. Uses `tpt-eng-provenance`.

### Changed

- Example code formatted with `cargo fmt` (no functional change).
- Depends on `tpt-eng-provenance` 0.1.0.

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-props-air`: ASHRAE moist-air (psychrometric) properties for HVAC and
  combustion-air work, including the Hyland–Wexler saturation-pressure correlation (0–200 °C) plus
  humidity ratio, relative humidity, specific enthalpy, and dew-point temperature, with `uom`-typed
  temperatures and pressures and `no_std` support.
