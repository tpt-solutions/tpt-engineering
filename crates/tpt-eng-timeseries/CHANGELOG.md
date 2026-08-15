# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026

### Added

- Initial release of `tpt-eng-timeseries`: umbrella crate re-exporting the `tpt-eng-timeseries-*`
  crates as `core` (`Series`, `Sample`, `Timestamp`), `align` (irregular multi-rate stream
  alignment), and `gap` (staleness/gap detection and interpolation).
