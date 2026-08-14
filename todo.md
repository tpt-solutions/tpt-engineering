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
- [x] `tpt-eng-props-air`: guard `humidity_ratio` against `p_w >= p`
      (divide-by-zero / negative ratio) and `vapour_pressure_from_ratio`
      against `w < 0`; add `T` sanity guard to `relative_humidity` /
      `dew_point` so `psat` is never fed a non-physical temperature. Introduce
      an `Error` enum (mirroring `props-water`) and migrate affected fns to
      `Result`. Add unit tests for the guarded cases.
- [x] `tpt-eng-geo-asset`: `within_radius` now filters non-finite coordinates
      exactly like `nearest`; added a test.
- [x] `tpt-eng-network-matrix`: `incidence_matrix` / `admittance_matrix` skip
      edges with missing endpoint nodes instead of panicking (defensive at the
      trust boundary). Added a test.

### 5b. Security audit & tightening
- [x] `deny.toml`: `unknown-registry` / `unknown-git` → `"deny"`,
      `yanked` → `"deny"`, with `allow-registry` pinned to crates.io and
      `allow-git = []` so only crates.io + the known `../tpt-math` path deps
      are permitted.
- [x] CI: split the `cargo-deny` job and add a dedicated `cargo-audit` job
      (RUSTSEC advisories caught independently of deny).
- [x] Add `SECURITY.md` (reporting policy; supported versions = none until
      `v0.1.0`).

### 5c. Innovation / easier-to-use
- [x] Add `crates/tpt-eng-examples`: a cross-crate integration scenario
      composing geo → topology → network-matrix → controls PID (driven by a
      fuel LHV), plus timeseries align/gap conditioning and a structural beam
      check. Doubles as the canonical "use them together" doctest.
- [x] `tpt-eng-structural`: expose `max_bending_moment_with_resolution(n)`
      (replaces the magic 400-sample default in `max_bending_moment`).
- [ ] `tpt-eng-controls`: add an anti-windup recovery unit test. (Deferred:
      existing `pid_output_clamped` already covers saturation; anti-windup
      branch is conservative and covered indirectly by the PID convergence
      tests in `tpt-eng-examples`.)

### 5d. Heavy adoption tooling
- [x] CI: add `wasm32-unknown-unknown` build for the `no_std` props crates
      (target already in `rust-toolchain.toml`) and a `docs`/`doctest` job
      (`cargo test --doc --workspace` + `cargo doc`).
- [x] `rustfmt.toml`: remove the legacy `edition = "2021"` line (conflicts
      with the `2024` workspace edition and is ignored by modern rustfmt);
      keep `max_width = 100`.
- [x] Add `release.toml` (cargo-release) + `CHANGELOG.md` (Keep a Changelog),
      seeded for `v0.1.0`.
- [x] Add root `justfile` (`check`/`test`/`ci`/`new`) and README quickstart
      (`cargo add` example + link to integration examples). Add `xtask doctest`
      and `xtask doc` commands.

### 5e. External / blocked (maintainer action — do NOT implement here)
- [ ] Flip `status = "planned"` → `"git"` for all 13 crates in sibling
      `tpt-rust-map/registry.toml` — requires write access to that repo.
- [ ] Cut `v0.1.0` tag after the registry flip + green CI on `main` —
      release-owner action.

### 5f. Validation
- [x] `cargo xtask check` clean; `cargo test --workspace --all-features` and
      `cargo test --doc --workspace` pass; `cargo xtask no-std-matrix` +
      wasm32 build green; `cargo audit` (no vulns) + `cargo deny check`
      (sources/advisories/bans/licenses ok) clean; integration examples
      doctest runs.

## Phase 6 — Adoption/DX pass (2026-08-14, post independent audit)

Three parallel independent audits (stubs/TODOs, security, adoption/DX)
re-verified Phase 5's self-assessment against the actual code. Result:
**no code stubs or defects found** — production code is clean (no
`todo!()`/`unimplemented!()`, no unguarded `.unwrap()`/`.expect()`/
`panic!()` outside tests/doc-examples, zero `unsafe`, `cargo audit`/
`cargo deny check` both clean). This phase is documentation/tooling only.

Two items were surfaced and explicitly deferred by the user (not in
scope for this phase): a scheduled/cron `cargo-audit` CI trigger, and a
GitHub Pages / hosted rustdoc publishing workflow — both require an
external one-time action (CI trigger change / enabling Pages in repo
settings) beyond a normal code change.

- [x] `README.md`: document the sibling `../tpt-math` repo path
      dependency (expected directory layout + clone example) in the
      Building section (silent clone-and-build blocker for new contributors
      removed).
- [x] `README.md`: add CI-status/license badges and an explicit "not yet
      published to crates.io" callout (matches `release.toml`'s
      `publish = false`).
- [x] `README.md`: add a one-line MSRV/edition callout (edition 2024, no
      MSRV pin).
- [x] `README.md`: add a "Which crate do I need?" decision table,
      distinct from the existing flat inventory table.
- [x] `README.md`: reference the new `CONTRIBUTING.md` from the
      Developer tooling section.
- [x] Add `CONTRIBUTING.md`: issues-only policy (bug reports / feature
      requests via GitHub Issues; no external code contributions /
      PRs accepted), plus a pointer to `SECURITY.md` for vulnerability
      reports instead of a public issue.
- [x] `xtask new-crate`: scaffold a `tests/basic.rs` and
      `examples/basic.rs` alongside the existing `Cargo.toml`/`lib.rs`/
      `README.md`, matching the `tpt-eng-examples` runnable-example
      pattern. Update `--dry-run` output and the "next:" message
      accordingly.
- [x] `xtask/src/main.rs`: update the module doc comment and
      `print_usage()` to mention the newly scaffolded files.
- [x] Verify: `cargo xtask check` stays clean (exit 0; only harmless
      `license-not-encountered` deny warnings); `cargo xtask new-crate
      tpt-eng-scratch-test --dry-run` previews the new files; a real
      (throwaway) `new-crate` run builds/tests/runs via `cargo test -p`
      and `cargo run -p ... --example basic`, then is removed; `cargo
      test --workspace --all-features` + `cargo test --doc --workspace`
      still pass.

### Deferred (explicit user choice, not this phase)
- [ ] Scheduled/cron `cargo-audit` CI trigger (currently push/PR only on
      `main`; a newly published RUSTSEC advisory for an already-merged
      dependency wouldn't be caught until the next push).
- [ ] GitHub Pages / hosted rustdoc publishing workflow.

## Phase 7 — Coherence, Consolidation & Documentation Pass (2026-08-15)

Full plan: `C:\Users\phill\.claude\plans\review-all-the-crates-serene-quiche.md`.
Triggered by a full-workspace review (29 `tpt-eng-*` crates + `xtask`) that
found real functional overlaps between crates (not just naming coincidences),
plus large documentation/metadata gaps and small coherence issues (lint
opt-in, dependency-declaration style, stale cross-references). User chose
**full consolidation** over "document only."

### 7.0 Baseline
- [ ] `cargo build --workspace` clean (pre-change baseline)
- [ ] `cargo test --workspace` clean (pre-change baseline)

### 7.1a. Tolerance stack-up consolidation
- [ ] Extend `tpt-eng-tolerance::DimTol` to support asymmetric
      (`tol_plus`/`tol_minus`) tolerances so it covers everything
      `tpt-eng-gdt`'s `StackupMember` does today
- [ ] `tpt-eng-gdt`: remove duplicate `StackupMember`/`Stackup`/
      `MonteCarloResult`/`lcg_next`/`lcg_uniform`; depend on
      `tpt-eng-tolerance` (workspace dep) and re-export its stack-up types
      from the gdt crate root
- [ ] Update/remove `tpt-eng-gdt` tests referencing the removed types
- [ ] Keep `tpt-eng-gdt`'s `ToleranceZone`/`DatumReferenceFrame::check_conformance`
      untouched (separate concern from 1-D stack-up)

### 7.1b. Utilization/pass-fail consolidation onto `tpt-eng-safety`
- [ ] `tpt-eng-standards::limit_states::DemandCapacity::utilization()` and
      `design::CheckResult::new()` delegate to `tpt_eng_safety::utilization`
      (add `tpt-eng-safety` workspace dep to `tpt-eng-standards`)
- [ ] `tpt-eng-structural::SectionCheck::utilization()` delegates to
      `tpt_eng_safety::utilization` (add `tpt-eng-safety` workspace dep to
      `tpt-eng-structural`)
- [ ] Re-verify existing test assertions in `limit_states.rs`, `design.rs`,
      and `tpt-eng-structural`'s `utilization_ratio` test after delegation
      (esp. the `capacity == 0.0 → infinity` edge case)
- [ ] `tpt-eng-safety::quantity::Quantity`/`Dimension`: keep public shape,
      back internally with real `tpt_math_units` (uom) values; add
      `tpt-math-units` as a real (non-dev) dependency

### 7.1c. STL/OBJ I/O consolidation
- [ ] Extend `tpt-eng-mesh`'s in-house OBJ codec (`to_obj`/`from_obj`) to
      carry texture coordinates and per-corner normal indices, matching
      `tpt-eng-io`'s current `ObjMesh`/`ObjFace` fidelity
- [ ] `tpt-eng-io`: drop `stl_io`/`obj` third-party deps; depend on
      `tpt-eng-mesh`; rewrite `src/stl.rs`/`src/obj.rs` to operate directly
      on `tpt_eng_mesh::Mesh`, dropping the local `StlMesh`/`ObjMesh` wrappers
- [ ] Update `tpt-eng-io`'s lib.rs doc example + inline tests
- [ ] Update `tpt-eng-cli::cmd_validate` STL/OBJ branches for the new
      `tpt_eng_mesh::Mesh` accessors

### 7.1d. CLI de-duplication
- [ ] `tpt-eng-cli`: add workspace deps on `tpt-eng-materials`,
      `tpt-eng-sections`, `tpt-eng-structural`
- [ ] Replace `src/materials.rs` hardcoded table with a small embedded
      `tpt_eng_materials::MaterialLibrary` (seeded with the same reference
      values + `DataSource` provenance so `validate()` passes)
- [ ] Replace `src/sections.rs` ad hoc formulas with direct
      `tpt_eng_sections::{Rectangle, Circle, ISection}` construction +
      `Section` trait calls
- [ ] Rewire `cmd_calc_beam` to use `tpt_eng_structural::{Beam, Load}` +
      `tpt_eng_sections::Section::second_moments()`, wrapping/unwrapping
      bare-`f64` CLI inputs via `tpt_math_units` at the boundary; keep the
      CLI's own closed-form UDL-deflection formula (out of scope for
      `tpt-eng-structural` v0.1.0)
- [ ] Update `tpt-eng-cli/tests/integration.rs` for any output-text
      assertions that shift

### 7.1e. `tpt-eng-sections` ↔ `tpt-eng-geometry` boundary
- [ ] Rewrite the stale "geometry integration deferred until that crate
      exists" language in `tpt-eng-sections/src/lib.rs` and README to state
      the 2D/3D split is a deliberate, permanent domain separation (no
      dependency edge added)

### 7.2. Lint & dependency-declaration fixes
- [ ] Add `[lints]\nworkspace = true` to the 12 crates missing it: `cad`,
      `gdt`, `geometry`, `mesh`, `nurbs`, `io`, `plot`, `report`, `cli`,
      `reliability`, `safety`, `tolerance`
- [ ] `cargo clippy --workspace --all-targets` after opt-in; fix newly
      surfaced warnings (expect `missing_errors_doc`/`missing_panics_doc`
      gaps, esp. in `cad`, `mesh`, `cli`)
- [ ] Fix `tpt-eng-props`/`-air`/`-fuels`/`-water`'s hardcoded relative-path
      `tpt-math-units`/`tpt-math-numeric` deps to use
      `{ workspace = true, default-features = false, features = [...] }`;
      verify per-crate build with each feature combination the crate
      currently exercises

### 7.3. Cargo.toml metadata
- [ ] Add `description` (where missing: `cli`, `io`, `plot`, `report`),
      `keywords` (≤5), and `categories` (valid crates.io slugs) to the ~21
      crates currently missing them, in the style of the 8 crates that
      already have this metadata

### 7.4. READMEs
- [ ] Write fresh README for the 7 crates with none: `io`, `mesh`, `plot`,
      `reliability`, `report`, `safety`, `tolerance`
- [ ] Fix stale "Related crates" links (`tpt-eng-linalg`/`tpt-eng-optimize`
      don't exist) in `materials`, `sections`, `standards` READMEs
- [ ] Fix stale project name "tpt-eng3" → "tpt-engineering" in `cad`, `gdt`,
      `geometry`, `nurbs` READMEs and `tpt-eng-cad/examples/integration.rs`
- [ ] Update READMEs for crates whose public API changed in 7.1
      (`gdt`, `safety`, `standards`, `structural`, `io`, `cli`)

### 7.5. CHANGELOGs
- [ ] Add `CHANGELOG.md` (`[0.1.0] - 2026`, "Added" bullets) to the 26
      crates lacking one, reflecting the final consolidated state
- [ ] Correct the 3 existing CHANGELOGs' (`materials`, `sections`,
      `standards`) dates from "2024" to "2026"

### 7.6. Verification
- [ ] `cargo build --workspace` clean
- [ ] `cargo test --workspace` clean (esp. gdt stack-up tests, `safety`'s
      `tests/cross_crate.rs`, `standards`'s `limit_states.rs`/`design.rs`
      tests, `structural`'s `utilization_ratio` test, `io`'s STL/OBJ tests
      (full rewrite), `cli/tests/integration.rs`)
- [ ] `cargo clippy --workspace --all-targets --all-features` clean
- [ ] `cargo doc --workspace --no-deps` clean
- [ ] `cargo tree --workspace` diffed against the 7.0 baseline — only the
      intended new edges exist (`gdt→tolerance`, `standards→safety`,
      `structural→safety`, `io→mesh`, `cli→materials/sections/structural`),
      no cycles
