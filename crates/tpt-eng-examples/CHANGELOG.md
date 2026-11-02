# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-11-03

### Added

- Scenario functions `run_construction_schedule`, `run_distillation`, `run_hvac_load`, `run_mechanical_design`, `run_pcb_thermal`, `run_solar_pv_sizing` and `run_thermal_loop`.

### Changed

- `run_solar_pv_sizing` uses the `tpt-eng-electrical` 0.2.0 `three_phase_power`, which returns `Option`.

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-examples`: cross-crate integration scenarios composing the `tpt-eng-*` engineering primitives (topology → network matrix → controls → time-series conditioning → structural check) as self-contained, tested functions.
