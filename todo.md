# tpt-engineering — Task Checklist

Applied engineering primitives for TPT Solutions' physical-systems
verticals. Dual-licensed MIT OR Apache-2.0. Author: TPT Solutions.

Scope, crate inventory, and ecosystem-gap justification: see `spec.txt`.

---

## Phase 0 — Ecosystem Verification (pre-work)

- [x] Confirm all `tpt-eng-*` crates (12 from spec.txt + `tpt-eng-structural`)
      are registered in `tpt-rust-map/registry.toml` with `status = "planned"`
- [x] Confirm dependency crates `tpt-math-units`, `tpt-math-numeric`,
      `tpt-math-linalg` exist in sibling repo `tpt-math`, are dual-licensed,
      and are buildable today
- [x] Confirm `tpt-rust2` (spec.txt's claimed consolidation source) does not
      exist on this machine — treat all 13 crates as new implementations,
      not ports
- [ ] Audit external repos `tpt-flight-control` / `tpt-chassis` /
      `tpt-dynamo` / `tpt-servo` (not cloned locally — requires GitHub
      access) for existing PID/state-space/transfer-function code before
      starting `tpt-eng-controls`
- [ ] Decide `tpt-eng-structural`'s dependency needs (likely
      `tpt-math-linalg` for beam/frame matrix analysis) and confirm with
      whoever owns the `tpt-vertical-map` construction vertical that pulled
      it out

## Phase 1 — Repo Scaffolding

- [ ] `git init` in `tpt-engineering`
- [ ] Copy `tpt-rust-map/template/` files into repo root: `Cargo.toml`
      (workspace), `deny.toml`, `rust-toolchain.toml`, `rustfmt.toml`,
      `.github/workflows/ci.yml`, `LICENSE-MIT`, `LICENSE-APACHE`
      (copyright: TPT Solutions, 2026)
- [ ] Fill `[workspace.package]`: `description`, `authors = ["TPT Solutions"]`,
      `edition = "2024"` (override template's 2021), **no `rust-version` key**
      (no MSRV pin — override template's 1.75), `license = "MIT OR Apache-2.0"`,
      `homepage`/`repository` = `https://github.com/tpt-solutions/tpt-engineering`
- [ ] Add `[workspace.dependencies]` entries for cross-repo deps, following
      `tpt-science`'s proven pattern:
      `tpt-math-units = { version = "0.1.0", path = "../tpt-math/crates/tpt-math-units" }`
      (same for `tpt-math-numeric`, `tpt-math-linalg`)
- [ ] Add root `README.md`: repo overview, crate list, license badges
- [ ] Verify root `spec.txt` (already present) matches `template/spec.txt`'s
      expected structure — no changes needed unless drift is found

## Phase 2 — Crate Scaffolding

No `xtask new-crate` tool exists yet — each crate gets a hand-created
`Cargo.toml` + `src/lib.rs` stub (mirror `tpt-science`'s per-crate pattern:
`edition.workspace = true`, `license.workspace = true`, `authors.workspace = true`,
`[lints] workspace = true`), then added to the workspace `members` list and
`[workspace.dependencies]`.

- [ ] `tpt-eng-props-water` — no_std; depends on `tpt-math-units`
- [ ] `tpt-eng-props-air` — no_std; depends on `tpt-math-units`
- [ ] `tpt-eng-props-fuels` — no_std; depends on `tpt-math-units`
- [ ] `tpt-eng-props` (umbrella) — no_std; re-exports water + air + fuels
- [ ] `tpt-eng-timeseries-core` — depends on `tpt-math-numeric`
- [ ] `tpt-eng-timeseries-align` — depends on `tpt-eng-timeseries-core`
- [ ] `tpt-eng-timeseries-gap` — depends on `tpt-eng-timeseries-core`
- [ ] `tpt-eng-timeseries` (umbrella) — re-exports core + align + gap
- [ ] `tpt-eng-geo-asset`
- [ ] `tpt-eng-geo-topology` — depends on `tpt-eng-geo-asset`
- [ ] `tpt-eng-network-matrix` — depends on `tpt-math-linalg`, `tpt-eng-geo-topology`
- [ ] `tpt-eng-controls` — depends on `tpt-math-linalg`; **blocked on Phase 0
      external audit item**
- [ ] `tpt-eng-structural` — depends on TBD (likely `tpt-math-linalg`)

## Phase 3 — Implementation

### 3a. Fluid/gas properties
- [ ] Implement `tpt-eng-props-water`: IAPWS-IF97 water/steam property tables
- [ ] Implement `tpt-eng-props-air`: ASHRAE psychrometrics / moist-air properties
- [ ] Implement `tpt-eng-props-fuels`: heating values, density, combustion
      properties (natural gas, hydrogen blends, diesel)
- [ ] Wire up `tpt-eng-props` umbrella re-exports
- [ ] Tests: verify against known IAPWS-IF97 / ASHRAE reference points

### 3b. Timeseries
- [ ] Implement `tpt-eng-timeseries-core`: core timeseries types
- [ ] Implement `tpt-eng-timeseries-align`: align irregular multi-rate
      sensor streams (e.g. 1Hz CAN vs 10s Modbus) onto a unified,
      deterministic time grid
- [ ] Implement `tpt-eng-timeseries-gap`: staleness detection, freeze
      detection, interpolation strategies for sensor dropouts/timeouts
- [ ] Wire up `tpt-eng-timeseries` umbrella re-exports
- [ ] Tests: irregular sampling, dropout/gap edge cases

### 3c. Infrastructure topology
- [ ] Implement `tpt-eng-geo-asset`: geographic-coordinate-to-logical-device/
      network-node registry
- [ ] Implement `tpt-eng-geo-topology`: spatial infrastructure graphs
      (pipes/wires/ducts) with upstream/downstream traversal and
      flow-direction logic
- [ ] Implement `tpt-eng-network-matrix`: automated incidence/admittance
      matrix generation from topology graphs
- [ ] Tests: traversal + matrix generation against known small networks

### 3d. Controls
- [ ] Implement `tpt-eng-controls`: PID / state-space / transfer-function
      primitives (do not start until Phase 0 external audit is resolved)
- [ ] Tests: step response, known control-theory reference cases

### 3e. Structural
- [ ] Implement `tpt-eng-structural`: load calculations, beam/frame
      analysis, code-compliance checks (ASCE 7 / Eurocode-style)
- [ ] Tests: known textbook beam/frame reference solutions

## Phase 4 — Cross-cutting / CI / Release Readiness

- [ ] `cargo fmt --check`, `cargo clippy --workspace --all-targets
      --all-features`, `cargo test --workspace` all clean
- [ ] no_std CI matrix passing for `tpt-eng-props-water/air/fuels` +
      `tpt-eng-props` umbrella (per `no_std = true` in registry.toml)
- [ ] `cargo deny check` clean
- [ ] Per-crate `README.md` with usage examples
- [ ] Update `tpt-rust-map/registry.toml`: flip each crate's `status` from
      `planned` to `git` as it lands
- [ ] Tag `v0.1.0` once the full 13-crate set compiles and tests pass
