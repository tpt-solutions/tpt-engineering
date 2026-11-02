# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Added

- `Calculation` builder and `Step` for formula-based reports. A calculation lists its inputs, the formula for each step (with an optional unit and reference), its results, references, and notes. `Calculation::into_report` converts it to an ordinary `Report`, so the existing renderers need no changes.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-report`: a serializable `Report` data model plus Markdown, HTML, and JSON (calculation-trail) exporters, with validation helpers that annotate results with a pass/warn/fail status.
