# Stability policy and roadmap

## Versions

All `tpt-eng-*` crates are pre-1.0 (`0.x`). Each crate versions on its own. The
current versions are in [`CRATE_VERSIONS.md`](CRATE_VERSIONS.md), generated from
crates.io by `cargo xtask versions`.

Under Cargo's pre-1.0 rules, a breaking change bumps the minor version (`0.1.x`
to `0.2.0`), and everything else bumps the patch version.

- **Breaking** (minor bump): a changed public signature, a changed return type
  (for example `Option` where there was a plain value), a removed item, or a
  changed unit convention.
- **Fix** (patch bump): a corrected formula, a corrected constant, a new panic
  guard on an input that was already invalid, or a new item.
- **Deprecation**: where practical, keep the old item with `#[deprecated]` and a
  note naming the replacement for one minor release before removing it.

A correctness fix that changes a result for a valid input is a fix, not a
breaking change. Its CHANGELOG entry must state the old and new behaviour and
the reference that decides it (see [`VALIDATION.md`](VALIDATION.md)).

## Toolchain

No minimum Rust version is pinned. The workspace uses edition 2024. A rust-version
floor is added only when a consumer needs one, and then with a CI job that tests it.

## Publishing

Publishing is a release action. The release steps are in [`../RELEASE.md`](../RELEASE.md).
Version numbers for a release are chosen when it is cut, from the CHANGELOG
`Unreleased` section.

## Roadmap

Open work is tracked in [`../todo.md`](../todo.md). Near-term items:

- Validation CSVs with a citation per value (`tests/reference/`).
- Remaining example workflows (HVAC, PV with storage, distillation, PCB thermal,
  construction schedule).
- `cargo-semver-checks` in CI, enabled at the next version bump.
- Python bindings (`tpt-eng-py`), in progress.
