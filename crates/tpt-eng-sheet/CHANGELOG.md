# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-03

### Added

- Initial release of `tpt-eng-sheet`: declarative calculation sheets. A TOML or JSON sheet names one calculation and its inputs; field names carry their units (`pressure_kpa`, `span_m`). `run` executes the sheet and returns a `tpt-eng-report` `Report`.
- Three calculation kinds: `water-state` (IF97 state), `beam-udl` (simply supported beam under a uniform load, with an optional utilisation check), and `pid-step` (closed-loop step response).
- Unknown fields and unknown `kind` values are rejected rather than ignored.
- A JSON Schema generated from the `Sheet` types (`json_schema`), committed as `schema/calc-sheet.schema.json`. A test fails if it is out of date.
- The `tpt-eng sheet` commands are provided by `tpt-eng-cli`, in a later release.
