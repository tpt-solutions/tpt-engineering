# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this crate adheres to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-03

### Fixed

- PID anti-windup has a recovery test. The integral is frozen while the output is saturated, as the documentation says.
- `TransferFunction::new` documents its panic on an empty or zero-leading denominator. The assertion message says why.

### Changed

- Example code formatted with `cargo fmt` (no functional change).

## [0.1.0] - 2026-08-16

### Added

- Initial release of `tpt-eng-controls`: discrete-time PID controller (with derivative-on-measurement and clamped-integral anti-windup), discrete transfer-function model, and continuous-time state-space model with a forward-Euler step.
