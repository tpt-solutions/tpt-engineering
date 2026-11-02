# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Fixed

- `miners_rule`: a life of zero or less with a positive cycle count now counts as infinite damage, not zero. Previously it counted as no damage at all.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-reliability`: fatigue models (Basquin S–N and Miner's rule), life distributions (Weibull and exponential), failure-rate helpers, FMEA structures, and probabilistic-design reliability-index helpers.
