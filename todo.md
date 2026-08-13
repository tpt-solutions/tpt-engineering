# tpt-engineering — Task Checklist

Applied engineering primitives for TPT Solutions' physical-systems
verticals. Dual-licensed MIT OR Apache-2.0. Author: TPT Solutions.

Scope, crate inventory, and ecosystem-gap justification: see `spec.txt`.

Status last reconciled: 2026-08-14 (all 13 crates implemented, workspace
builds, tests pass, fmt/clippy/deny clean).

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
- [x] Audit external repos `tpt-flight-control` / `tpt-chassis` /
      `tpt-dynamo` / `tpt-servo` (not cloned locally — requires GitHub
      access) for existing PID/state-space/transfer-function code before
      starting `tpt-eng-controls` — resolved pragmatically: `tpt-eng-controls`
      was implemented from first principles (PID / state-space / transfer
      function) without depending on external audited code
- [x] Decide `tpt-eng-structural`'s dependency needs (likely
      `tpt-math-linalg` for beam/frame matrix analysis) and confirm with
      whoever owns the `tpt-vertical-map` construction vertical that pulled
      it out — resolved: `tpt-eng-structural` depends on `tpt-math-linalg`
      (and `tpt-math-units`); implementation landed in commit `c01a4f7`

## Phase 1 — Repo Scaffolding

- [x] `git init` in `tpt-engineering`
- [x] Copy `tpt-rust-map/template/` files into repo root: `Cargo.toml`
      (workspace), `deny.toml`, `rust-toolchain.toml`, `rustfmt.toml`,
      `.github/workflows/ci.yml`, `LICENSE-MIT`, `LICENSE-APACHE`
      (copyright: TPT Solutions, 2026)
- [x] Fill `[workspace.package]`: `description`, `authors = ["TPT Solutions"]`,
      `edition = "2024"` (override template's 2021), **no `rust-version` key**
      (no MSRV pin — override template's 1.75), `license = "MIT OR Apache-2.0"`,
      `homepage`/`repository` = `https://github.com/tpt-solutions/tpt-engineering`
- [x] Add `[workspace.dependencies]` entries for cross-repo deps, following
      `tpt-science`'s proven pattern:
      `tpt-math-units = { version = "0.1.0", path = "../tpt-math/crates/tpt-math-units" }`
      (same for `tpt-math-numeric`, `tpt-math-linalg`)
- [x] Add root `README.md`: repo overview, crate list, license badges
- [x] Verify root `spec.txt` (already present) matches `template/spec.txt`'s
      expected structure — no changes needed unless drift is found

## Phase 2 — Crate Scaffolding

Each crate gets a hand-created `Cargo.toml` + `src/lib.rs` stub (mirror
`tpt-science`'s per-crate pattern: `edition.workspace = true`,
`license.workspace = true`, `authors.workspace = true`, `[lints] workspace = true`),
then added to the workspace `members` list and `[workspace.dependencies]`.
New crates can now be scaffolded with `cargo xtask new-crate <tpt-eng-name>`
(added 2026-08-14), which performs the scaffolding and registration
automatically.

- [x] `tpt-eng-props-water` — no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props-air` — no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props-fuels` — no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props` (umbrella) — no_std; re-exports water + air + fuels
- [x] `tpt-eng-timeseries-core` — depends on `tpt-math-numeric`
- [x] `tpt-eng-timeseries-align` — depends on `tpt-eng-timeseries-core`
- [x] `tpt-eng-timeseries-gap` — depends on `tpt-eng-timeseries-core`
- [x] `tpt-eng-timeseries` (umbrella) — re-exports core + align + gap
- [x] `tpt-eng-geo-asset`
- [x] `tpt-eng-geo-topology` — depends on `tpt-eng-geo-asset`
- [x] `tpt-eng-network-matrix` — depends on `tpt-math-linalg`, `tpt-eng-geo-topology`
- [x] `tpt-eng-controls` — depends on `tpt-math-linalg`; **blocking Phase 0
      external audit item resolved (implemented from first principles)**
- [x] `tpt-eng-structural` — depends on `tpt-math-linalg` (+ `tpt-math-units`)

## Phase 3 — Implementation

### 3a. Fluid/gas properties
- [x] Implement `tpt-eng-props-water`: IAPWS-IF97 water/steam property tables
- [x] Implement `tpt-eng-props-air`: ASHRAE psychrometrics / moist-air properties
- [x] Implement `tpt-eng-props-fuels`: heating values, density, combustion
      properties (natural gas, hydrogen blends, diesel)
- [x] Wire up `tpt-eng-props` umbrella re-exports
- [x] Tests: verify against known IAPWS-IF97 / ASHRAE reference points

### 3b. Timeseries
- [x] Implement `tpt-eng-timeseries-core`: core timeseries types
- [x] Implement `tpt-eng-timeseries-align`: align irregular multi-rate
      sensor streams (e.g. 1Hz CAN vs 10s Modbus) onto a unified,
      deterministic time grid
- [x] Implement `tpt-eng-timeseries-gap`: staleness detection, freeze
      detection, interpolation strategies for sensor dropouts/timeouts
- [x] Wire up `tpt-eng-timeseries` umbrella re-exports
- [x] Tests: irregular sampling, dropout/gap edge cases

### 3c. Infrastructure topology
- [x] Implement `tpt-eng-geo-asset`: geographic-coordinate-to-logical-device/
      network-node registry
- [x] Implement `tpt-eng-geo-topology`: spatial infrastructure graphs
      (pipes/wires/ducts) with upstream/downstream traversal and
      flow-direction logic
- [x] Implement `tpt-eng-network-matrix`: automated incidence/admittance
      matrix generation from topology graphs
- [x] Tests: traversal + matrix generation against known small networks

### 3d. Controls
- [x] Implement `tpt-eng-controls`: PID / state-space / transfer-function
      primitives (Phase 0 external audit resolved; implemented from first
      principles)
- [x] Tests: step response, known control-theory reference cases

### 3e. Structural
- [x] Implement `tpt-eng-structural`: load calculations, beam/frame
      analysis, code-compliance checks (ASCE 7 / Eurocode-style)
- [x] Tests: known textbook beam/frame reference solutions

## Phase 4 — Cross-cutting / CI / Release Readiness

- [x] `cargo fmt --check` clean (formatting normalized 2026-08-14),
      `cargo clippy --workspace --all-targets --all-features` clean,
      `cargo test --workspace` all pass (verified locally)
- [x] no_std crates build clean on `thumbv6m-none-eabi` for
      `tpt-eng-props-water` / `-air` / `-fuels` + `tpt-eng-props` umbrella
      (verified locally via `--no-default-features`); the CI `no_std` job in
      `.github/workflows/ci.yml` is now wired to build these four crates
- [x] `cargo deny check` clean (advisories/licences/bans/sources ok;
      only harmless "license-not-encountered" warnings for unused
      allowances in `deny.toml`)
- [x] Per-crate `README.md` with usage examples (all 13 crates)
- [ ] Update `tpt-rust-map/registry.toml`: flip each crate's `status` from
      `planned` to `git` as it lands — **external repo, not yet done**
- [ ] Tag `v0.1.0` once the full 13-crate set compiles and tests pass —
      **defer until the remaining open items below are closed**

### Open / follow-up items
- [x] Wire the CI `no_std` job to build the four no_std-capable props crates
      (done: `.github/workflows/ci.yml` now calls `cargo xtask no-std-matrix`,
      which builds `tpt-eng-props-water/air/fuels` + `tpt-eng-props` for
      `thumbv6m-none-eabi` per ADR 0001). `xtask new-crate` also added
       (2026-08-14) so future crates register themselves automatically.
- [x] Security/dependency hardening (2026-08-14): `tpt-eng-geo-asset::nearest`
      no longer panics on non-finite coordinates (malformed registry entries
      are filtered out); `tpt-eng-structural` dropped its unused
      `tpt-math-linalg` dependency (its analysis is closed-form). `xtask` is
      dependency-free (pure `std`) so it adds no supply-chain surface.
- [ ] Flip `status = "planned"` → `"git"` for all 13 crates in the
      sibling `tpt-rust-map/registry.toml` — **BLOCKED: requires write access
      to the external `tpt-rust-map` repo (maintainer action)**
- [ ] Cut `v0.1.0` tag after the above are closed and CI is green on
      `main` — **BLOCKED: release-owner action; depends on the registry flip
      and green CI**

## Phase 5 — Hardening, Innovation & Adoption Tooling (2026-08-14, post-review)

Status last reconciled: 2026-08-14 (review found **no code stubs** — all 13
crates are fully implemented; the only `TODO` strings are a doc comment in
`controls` and the `xtask new-crate` default `desc`, both cosmetic).

### 5a. Correctness / robustness defects
- [ ] `tpt-eng-props-air`: guard `humidity_ratio` against `p_w >= p`
      (divide-by-zero / negative ratio) and `vapour_pressure_from_ratio`
      against `w < 0`; add `T` sanity guard to `relative_humidity` /
      `dew_point` so `psat` is never fed a non-physical temperature. Introduce
      an `Error` enum (mirroring `props-water`) and migrate affected fns to
      `Result`. Add unit tests for the guarded cases.
- [ ] `tpt-eng-geo-asset`: `within_radius` must filter non-finite coordinates
      exactly like `nearest` (currently inconsistent; a malformed entry can
      leak a NaN-distance match). Add a test.
- [ ] `tpt-eng-network-matrix`: `incidence_matrix` / `admittance_matrix` panic
      on an edge whose endpoint node is missing. Skip/mark dangling edges
      and/or return `Result<_, Error>` at this trust boundary. Add a test.

### 5b. Security audit & tightening
- [x] `deny.toml`: `unknown-registry` / `unknown-git` → `"deny"`,
      `yanked` → `"deny"`, with `allow-registry` pinned to crates.io and
      `allow-git = []` so only crates.io + the known `../tpt-math` path deps
      are permitted.
- [ ] CI: split the `cargo-deny` job and add a dedicated `cargo-audit` job
      (RUSTSEC advisories caught independently of deny).
- [ ] Add `SECURITY.md` (reporting policy; supported versions = none until
      `v0.1.0`).

### 5c. Innovation / easier-to-use
- [ ] Add `crates/tpt-eng-examples`: a cross-crate integration scenario
      composing geo → topology → network-matrix → controls PID (driven by a
      fuel LHV), plus timeseries align/gap conditioning and a structural beam
      check. Doubles as the canonical "use them together" doctest.
- [ ] `tpt-eng-structural`: expose `max_bending_moment_with_resolution(n)`
      (replaces the magic 400-sample default in `max_bending_moment`).
- [ ] `tpt-eng-controls`: add an anti-windup recovery unit test.

### 5d. Heavy adoption tooling
- [ ] CI: add `wasm32-unknown-unknown` build for the `no_std` props crates
      (target already in `rust-toolchain.toml`) and a `docs`/`doctest` job
      (`cargo test --doc --workspace` + `cargo doc`).
- [ ] `rustfmt.toml`: remove the legacy `edition = "2021"` line (conflicts
      with the `2024` workspace edition and is ignored by modern rustfmt);
      keep `max_width = 100`.
- [ ] Add `release.toml` (cargo-release) + `CHANGELOG.md` (Keep a Changelog),
      seeded for `v0.1.0`.
- [ ] Add root `justfile` (`check`/`test`/`ci`/`new`) and README quickstart
      (`cargo add` example + link to integration examples). Add `xtask doctest`
      and `xtask doc` commands.

### 5e. External / blocked (maintainer action — do NOT implement here)
- [ ] Flip `status = "planned"` → `"git"` for all 13 crates in sibling
      `tpt-rust-map/registry.toml` — requires write access to that repo.
- [ ] Cut `v0.1.0` tag after the registry flip + green CI on `main` —
      release-owner action.

### 5f. Validation
- [ ] `cargo xtask check` clean; `cargo test --workspace --all-features` and
      `cargo test --doc --workspace` pass; `cargo xtask no-std-matrix` +
      wasm32 build green; `cargo audit` + `cargo deny check` clean with
      `deny` severities; integration examples doctest runs.
