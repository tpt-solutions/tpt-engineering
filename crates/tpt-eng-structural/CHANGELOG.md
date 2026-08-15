# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026

### Added

- Initial release of `tpt-eng-structural`: structural-engineering primitives — load definitions, simply-supported beam analysis (reactions, shear, and bending moment), and demand/capacity code-compliance checks in an ASCE 7 / Eurocode-style utilization-ratio form. Utilization/pass-fail evaluation delegates to `tpt-eng-safety`.
