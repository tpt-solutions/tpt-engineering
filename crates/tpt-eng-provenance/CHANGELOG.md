# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-03

### Added

- Initial release of `tpt-eng-provenance`: tags for engineering results. `Tagged<T>` pairs a value with its `Provenance`, which holds the `Source` of the correlation (standard, literature, manufacturer, or calculated), the `Validity` range of each input, and `Warnings` raised by the call (`OUT_OF_RANGE`, `EXTRAPOLATED`, `APPROXIMATE_MODEL`).
- `out_of_range` and `range_warning` to check an input against its validity range.
- `no_std` with no dependencies.
