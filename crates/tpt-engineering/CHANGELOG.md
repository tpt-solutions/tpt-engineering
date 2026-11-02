# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-03

### Added

- Initial release of `tpt-engineering`: an umbrella crate that re-exports every `tpt-eng-*` domain crate behind its own feature flag. Each feature is named after its crate without the `tpt-eng-` prefix. The `full` feature enables all of them.
- Published last in Batch 11, after every crate it re-exports was live at its pinned version.
