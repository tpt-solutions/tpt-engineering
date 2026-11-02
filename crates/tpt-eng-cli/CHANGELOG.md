# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Added

- `sheet run` and `sheet schema` subcommands, backed by `tpt-eng-sheet`. `sheet run` executes a TOML or JSON calculation sheet and writes its report as Markdown, HTML, or JSON. `sheet schema` writes the sheet's JSON Schema.

### Changed

- Version is now set explicitly to `0.1.1` in `Cargo.toml`, instead of inheriting the workspace version.

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-cli`: a command-line front-end for the TPT engineering toolkit providing unit conversion, material/section inspection, simply supported beam calculations, and engineering input-file validation. It now depends on `tpt-eng-materials`, `tpt-eng-sections`, and `tpt-eng-structural` for material/section/beam evaluation, and on `tpt-eng-report`/`tpt-eng-plot` for report and chart output.
