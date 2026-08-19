# tpt-engineering â€” Task Checklist

Applied engineering primitives for TPT Solutions' physical-systems
verticals. Dual-licensed MIT OR Apache-2.0. Author: TPT Solutions.

Scope, crate inventory, and ecosystem-gap justification: see `spec.txt`.

Status last reconciled: 2026-08-19 (43-crate workspace; Phase 9 complete —
14 `spec2.txt` crates scaffolded and registered per the dependency tables:
3 foundational (`electrical`, `heat-transfer`, `props-mixture`, the last
wired into the `tpt-eng-props` umbrella) + 11 domain-specific component
models; README inventory / "Which crate do I need?" tables and
`CRATE_AUDIT.md` updated; `cargo build`/`test`/`clippy -D warnings`/
`xtask check` all green).

---

## Phase 0 â€” Ecosystem Verification (pre-work)

- [x] Confirm all `tpt-eng-*` crates (12 from spec.txt + `tpt-eng-structural`)
      are registered in `tpt-rust-map/registry.toml` with `status = "planned"`
- [x] Confirm dependency crates `tpt-math-units`, `tpt-math-numeric`,
      `tpt-math-linalg` exist in sibling repo `tpt-math`, are dual-licensed,
      and are buildable today
- [x] Confirm `tpt-rust2` (spec.txt's claimed consolidation source) does not
      exist on this machine â€” treat all 13 crates as new implementations,
      not ports
- [x] Audit external repos `tpt-flight-control` / `tpt-chassis` /
      `tpt-dynamo` / `tpt-servo` (not cloned locally â€” requires GitHub
      access) for existing PID/state-space/transfer-function code before
      starting `tpt-eng-controls` â€” resolved pragmatically: `tpt-eng-controls`
      was implemented from first principles (PID / state-space / transfer
      function) without depending on external audited code
- [x] Decide `tpt-eng-structural`'s dependency needs (likely
      `tpt-math-linalg` for beam/frame matrix analysis) and confirm with
      whoever owns the `tpt-vertical-map` construction vertical that pulled
      it out â€” resolved: `tpt-eng-structural` depends on `tpt-math-linalg`
      (and `tpt-math-units`); implementation landed in commit `c01a4f7`

## Phase 1 â€” Repo Scaffolding

- [x] `git init` in `tpt-engineering`
- [x] Copy `tpt-rust-map/template/` files into repo root: `Cargo.toml`
      (workspace), `deny.toml`, `rust-toolchain.toml`, `rustfmt.toml`,
      `.github/workflows/ci.yml`, `LICENSE-MIT`, `LICENSE-APACHE`
      (copyright: TPT Solutions, 2026)
- [x] Fill `[workspace.package]`: `description`, `authors = ["TPT Solutions"]`,
      `edition = "2024"` (override template's 2021), **no `rust-version` key**
      (no MSRV pin â€” override template's 1.75), `license = "MIT OR Apache-2.0"`,
      `homepage`/`repository` = `https://github.com/tpt-solutions/tpt-engineering`
- [x] Add `[workspace.dependencies]` entries for cross-repo deps, following
      `tpt-science`'s proven pattern:
      `tpt-math-units = { version = "0.1.0", path = "../tpt-math/crates/tpt-math-units" }`
      (same for `tpt-math-numeric`, `tpt-math-linalg`)
- [x] Add root `README.md`: repo overview, crate list, license badges
- [x] Verify root `spec.txt` (already present) matches `template/spec.txt`'s
      expected structure â€” no changes needed unless drift is found

## Phase 2 â€” Crate Scaffolding

Each crate gets a hand-created `Cargo.toml` + `src/lib.rs` stub (mirror
`tpt-science`'s per-crate pattern: `edition.workspace = true`,
`license.workspace = true`, `authors.workspace = true`, `[lints] workspace = true`),
then added to the workspace `members` list and `[workspace.dependencies]`.
New crates can now be scaffolded with `cargo xtask new-crate <tpt-eng-name>`
(added 2026-08-14), which performs the scaffolding and registration
automatically.

- [x] `tpt-eng-props-water` â€” no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props-air` â€” no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props-fuels` â€” no_std; depends on `tpt-math-units`
- [x] `tpt-eng-props` (umbrella) â€” no_std; re-exports water + air + fuels
- [x] `tpt-eng-timeseries-core` â€” depends on `tpt-math-numeric`
- [x] `tpt-eng-timeseries-align` â€” depends on `tpt-eng-timeseries-core`
- [x] `tpt-eng-timeseries-gap` â€” depends on `tpt-eng-timeseries-core`
- [x] `tpt-eng-timeseries` (umbrella) â€” re-exports core + align + gap
- [x] `tpt-eng-geo-asset`
- [x] `tpt-eng-geo-topology` â€” depends on `tpt-eng-geo-asset`
- [x] `tpt-eng-network-matrix` â€” depends on `tpt-math-linalg`, `tpt-eng-geo-topology`
- [x] `tpt-eng-controls` â€” depends on `tpt-math-linalg`; **blocking Phase 0
      external audit item resolved (implemented from first principles)**
- [x] `tpt-eng-structural` â€” depends on `tpt-math-linalg` (+ `tpt-math-units`)

## Phase 3 â€” Implementation

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

## Phase 4 â€” Cross-cutting / CI / Release Readiness

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
      `planned` to `git` as it lands â€” **external repo, not yet done**
- [ ] Tag `v0.1.0` once the full 13-crate set compiles and tests pass â€”
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
- [ ] Flip `status = "planned"` â†’ `"git"` for all 13 crates in the
      sibling `tpt-rust-map/registry.toml` â€” **BLOCKED: requires write access
      to the external `tpt-rust-map` repo (maintainer action)**
- [ ] Cut `v0.1.0` tag after the above are closed and CI is green on
      `main` â€” **BLOCKED: release-owner action; depends on the registry flip
      and green CI**

## Phase 5 â€” Hardening, Innovation & Adoption Tooling (2026-08-14, post-review)

Status last reconciled: 2026-08-14 (review found **no code stubs** â€” all 13
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
- [x] `deny.toml`: `unknown-registry` / `unknown-git` â†’ `"deny"`,
      `yanked` â†’ `"deny"`, with `allow-registry` pinned to crates.io and
      `allow-git = []` so only crates.io + the known `../tpt-math` path deps
      are permitted.
- [x] CI: split the `cargo-deny` job and add a dedicated `cargo-audit` job
      (RUSTSEC advisories caught independently of deny).
- [x] Add `SECURITY.md` (reporting policy; supported versions = none until
      `v0.1.0`).

### 5c. Innovation / easier-to-use
- [x] Add `crates/tpt-eng-examples`: a cross-crate integration scenario
      composing geo â†’ topology â†’ network-matrix â†’ controls PID (driven by a
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

### 5e. External / blocked (maintainer action â€” do NOT implement here)
- [ ] Flip `status = "planned"` â†’ `"git"` for all 13 crates in sibling
      `tpt-rust-map/registry.toml` â€” requires write access to that repo.
- [ ] Cut `v0.1.0` tag after the registry flip + green CI on `main` â€”
      release-owner action.

### 5f. Validation
- [x] `cargo xtask check` clean; `cargo test --workspace --all-features` and
      `cargo test --doc --workspace` pass; `cargo xtask no-std-matrix` +
      wasm32 build green; `cargo audit` (no vulns) + `cargo deny check`
      (sources/advisories/bans/licenses ok) clean; integration examples
      doctest runs.

## Phase 6 â€” Adoption/DX pass (2026-08-14, post independent audit)

Three parallel independent audits (stubs/TODOs, security, adoption/DX)
re-verified Phase 5's self-assessment against the actual code. Result:
**no code stubs or defects found** â€” production code is clean (no
`todo!()`/`unimplemented!()`, no unguarded `.unwrap()`/`.expect()`/
`panic!()` outside tests/doc-examples, zero `unsafe`, `cargo audit`/
`cargo deny check` both clean). This phase is documentation/tooling only.

Two items were surfaced and explicitly deferred by the user (not in
scope for this phase): a scheduled/cron `cargo-audit` CI trigger, and a
GitHub Pages / hosted rustdoc publishing workflow â€” both require an
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

## Phase 7 â€” Coherence, Consolidation & Documentation Pass (2026-08-15)

Full plan: `C:\Users\phill\.claude\plans\review-all-the-crates-serene-quiche.md`.
Triggered by a full-workspace review (29 `tpt-eng-*` crates + `xtask`) that
found real functional overlaps between crates (not just naming coincidences),
plus large documentation/metadata gaps and small coherence issues (lint
opt-in, dependency-declaration style, stale cross-references). User chose
**full consolidation** over "document only."

### 7.0 Baseline
- [x] `cargo build --workspace` clean (pre-change baseline)
- [x] `cargo test --workspace` clean (pre-change baseline)

### 7.1a. Tolerance stack-up consolidation
- [x] Extend `tpt-eng-tolerance::DimTol` to support asymmetric
      (`tol_plus`/`tol_minus`) tolerances so it covers everything
      `tpt-eng-gdt`'s `StackupMember` does today
- [x] `tpt-eng-gdt`: remove duplicate `StackupMember`/`Stackup`/
      `MonteCarloResult`/`lcg_next`/`lcg_uniform`; depend on
      `tpt-eng-tolerance` (workspace dep) and re-export its stack-up types
      from the gdt crate root
- [x] Update/remove `tpt-eng-gdt` tests referencing the removed types
- [x] Keep `tpt-eng-gdt`'s `ToleranceZone`/`DatumReferenceFrame::check_conformance`
      untouched (separate concern from 1-D stack-up)

### 7.1b. Utilization/pass-fail consolidation onto `tpt-eng-safety`
- [x] `tpt-eng-standards::limit_states::DemandCapacity::utilization()` and
      `design::CheckResult::new()` delegate to `tpt_eng_safety::utilization`
      (add `tpt-eng-safety` workspace dep to `tpt-eng-standards`)
- [x] `tpt-eng-structural::SectionCheck::utilization()` delegates to
      `tpt_eng_safety::utilization` (add `tpt-eng-safety` workspace dep to
      `tpt-eng-structural`)
- [x] Re-verify existing test assertions in `limit_states.rs`, `design.rs`,
      and `tpt-eng-structural`'s `utilization_ratio` test after delegation
      (esp. the `capacity == 0.0 â†’ infinity` edge case)
- [x] `tpt-eng-safety::quantity::Quantity`/`Dimension`: keep public shape,
      back internally with real `tpt_math_units` (uom) values; add
      `tpt-math-units` as a real (non-dev) dependency

### 7.1c. STL/OBJ I/O consolidation
- [x] Extend `tpt-eng-mesh`'s in-house OBJ codec (`to_obj`/`from_obj`) to
      carry texture coordinates and per-corner normal indices, matching
      `tpt-eng-io`'s current `ObjMesh`/`ObjFace` fidelity
- [x] `tpt-eng-io`: drop `stl_io`/`obj` third-party deps; depend on
      `tpt-eng-mesh`; rewrite `src/stl.rs`/`src/obj.rs` to operate directly
      on `tpt_eng_mesh::Mesh`, dropping the local `StlMesh`/`ObjMesh` wrappers
- [x] Update `tpt-eng-io`'s lib.rs doc example + inline tests
- [x] Update `tpt-eng-cli::cmd_validate` STL/OBJ branches for the new
      `tpt_eng_mesh::Mesh` accessors

### 7.1d. CLI de-duplication
- [x] `tpt-eng-cli`: add workspace deps on `tpt-eng-materials`,
      `tpt-eng-sections`, `tpt-eng-structural`
- [x] Replace `src/materials.rs` hardcoded table with a small embedded
      `tpt_eng_materials::MaterialLibrary` (seeded with the same reference
      values + `DataSource` provenance so `validate()` passes)
- [x] Replace `src/sections.rs` ad hoc formulas with direct
      `tpt_eng_sections::{Rectangle, Circle, ISection}` construction +
      `Section` trait calls
- [x] Rewire `cmd_calc_beam` to use `tpt_eng_structural::{Beam, Load}` +
      `tpt_eng_sections::Section::second_moments()`, wrapping/unwrapping
      bare-`f64` CLI inputs via `tpt_math_units` at the boundary; keep the
      CLI's own closed-form UDL-deflection formula (out of scope for
      `tpt-eng-structural` v0.1.0)
- [x] Update `tpt-eng-cli/tests/integration.rs` for any output-text
      assertions that shift

### 7.1e. `tpt-eng-sections` â†” `tpt-eng-geometry` boundary
- [x] Rewrite the stale "geometry integration deferred until that crate
      exists" language in `tpt-eng-sections/src/lib.rs` and README to state
      the 2D/3D split is a deliberate, permanent domain separation (no
      dependency edge added)

### 7.2. Lint & dependency-declaration fixes
- [x] Add `[lints]\nworkspace = true` to the 12 crates missing it: `cad`,
      `gdt`, `geometry`, `mesh`, `nurbs`, `io`, `plot`, `report`, `cli`,
      `reliability`, `safety`, `tolerance`
- [x] `cargo clippy --workspace --all-targets` after opt-in; fix newly
      surfaced warnings (expect `missing_errors_doc`/`missing_panics_doc`
      gaps, esp. in `cad`, `mesh`, `cli`)
- [x] Fix `tpt-eng-props`/`-air`/`-fuels`/`-water`'s hardcoded relative-path
      `tpt-math-units`/`tpt-math-numeric` deps to use
      `{ workspace = true, default-features = false, features = [...] }`;
      verify per-crate build with each feature combination the crate
      currently exercises

### 7.3. Cargo.toml metadata
- [x] Add `description` (where missing: `cli`, `io`, `plot`, `report`),
      `keywords` (â‰¤5), and `categories` (valid crates.io slugs) to the ~21
      crates currently missing them, in the style of the 8 crates that
      already have this metadata

### 7.4. READMEs
- [x] Write fresh README for the 7 crates with none: `io`, `mesh`, `plot`,
      `reliability`, `report`, `safety`, `tolerance`
- [x] Fix stale "Related crates" links (`tpt-eng-linalg`/`tpt-eng-optimize`
      don't exist) in `materials`, `sections`, `standards` READMEs
- [x] Fix stale project name "tpt-eng3" â†’ "tpt-engineering" in `cad`, `gdt`,
      `geometry`, `nurbs` READMEs and `tpt-eng-cad/examples/integration.rs`
- [x] Update READMEs for crates whose public API changed in 7.1
      (`gdt`, `safety`, `standards`, `structural`, `io`, `cli`)

### 7.5. CHANGELOGs
- [x] Add `CHANGELOG.md` (`[0.1.0] - 2026`, "Added" bullets) to the 26
      crates lacking one, reflecting the final consolidated state
- [x] Correct the 3 existing CHANGELOGs' (`materials`, `sections`,
      `standards`) dates from "2024" to "2026"

### 7.6. Verification
- [x] `cargo build --workspace` clean
- [x] `cargo test --workspace` clean (esp. gdt stack-up tests, `safety`'s
      `tests/cross_crate.rs`, `standards`'s `limit_states.rs`/`design.rs`
      tests, `structural`'s `utilization_ratio` test, `io`'s STL/OBJ tests
      (full rewrite), `cli/tests/integration.rs`)
- [x] `cargo clippy --workspace --all-targets --all-features` clean
- [x] `cargo doc --workspace --no-deps` clean
- [x] `cargo tree --workspace` diffed against the 7.0 baseline â€” only the
      intended new edges exist (`gdtâ†’tolerance`, `standardsâ†’safety`,
      `structuralâ†’safety`, `ioâ†’mesh`, `cliâ†’materials/sections/structural`),
      no cycles

## Phase 8 â€” Platform Review & Adoption Follow-up (2026-08-15)

Triggered by a full-platform review (bugs/TODOs/missing features/adoption
friction) run via three parallel Explore agents against the current
29-crate workspace. Confirmed: Phase 7's crate "consolidations" merged
*logic*, not crates â€” `tpt-eng-gdt`, `tpt-eng-io`, `tpt-eng-standards`, and
`tpt-eng-structural` all still exist and now delegate to
`tpt-eng-tolerance`/`tpt-eng-mesh`/`tpt-eng-safety` respectively instead of
duplicating code. No `todo!()`/`unimplemented!()`/`unsafe` found workspace-
wide; only one real panic risk identified (tolerance NaN sort, below).

### 8a. Bug fixes
- [x] `tpt-eng-tolerance::rank_contributors`: remove the
      `partial_cmp(...).unwrap()` panic-on-NaN in its sort comparator (use
      `total_cmp` or `unwrap_or(Equal)`); update the `# Panics` doc note;
      add a non-finite-input regression test. — **already done in the
      codebase** (`rank_contributors` uses `total_cmp`; non-finite
      regression test present).
- [x] Root `Cargo.toml`: remove dead `stl_io`/`obj` entries from
      `[workspace.dependencies]` (unused since `tpt-eng-io` was rewritten
      in Phase 7.1c to depend on `tpt-eng-mesh` instead). — **already done
      (no such entries present in the root manifest).**

### 8b. Docs sync
- [x] `README.md`: expand "Crate inventory" and "Which crate do I need?"
      tables from 14 to all 29 crates (materials, sections, standards,
      geometry, mesh, nurbs, gdt, cad, tolerance, reliability, safety, io,
      report, plot, cli were added post-Phase-4 and are currently
      undocumented at the root level); update the "Scope" paragraph.
      — **already done** (README lists all 29 crates + Scope note).
- [x] `spec.txt`: note/reflect the 15 crates added beyond the original
      Phase-0 scope so it isn't read as authoritative-but-stale. — **already
      done** ("POST-PHASE-0 ADDITIONS" section present).

### 8c. CLI expansion
- [x] `tpt-eng-cli`: add subcommands surfacing currently library-only
      domains — `props water|air|fuel` (property lookups), `pid` (step-
      response simulation), `tolerance stackup` (worst-case/RSS calc),
      `beam` (extend existing `calc`). 18 of 29 crates currently have zero
      CLI exposure. — **done** (added `props`/`pid`/`tolerance` subcommands;
      `beam` already existed as `calc beam`).
- [x] Update `tpt-eng-cli/README.md` and `tests/integration.rs` for the
      new commands. — **done** (README command table + examples; 11 new
      integration tests added).

### 8d. Adoption tooling
- [x] `tpt-eng-examples`: add a second integration scenario (e.g.
      `mechanical_design.rs`: sections → materials → tolerance → gdt →
      report) — the existing `thermal_loop.rs` only exercises the original
      14-crate set. — **done** (`src/mechanical_design.rs` added + exported).
- [x] Add `.github/dependabot.yml` (cargo ecosystem, weekly). — **done**.

### 8e. Deferred (explicit scope-bounding, not this pass)
- [ ] Workspace-wide `tpt-eng` prelude/meta crate spanning all 29 domains
      behind Cargo features (only per-family umbrellas exist today: props,
      timeseries).
- [ ] cargo-generate template / devcontainer for downstream *consumer*
      projects (distinct from `xtask new-crate`, which only scaffolds
      crates inside this workspace).
- [ ] Code-coverage CI job (tarpaulin/llvm-cov + Codecov).
- [ ] Release-automation / changelog-generation workflow (CHANGELOG.md is
      currently hand-edited).
- [ ] (carried over from Phase 6) scheduled/cron `cargo-audit` CI trigger;
      GitHub Pages / hosted rustdoc publishing.

### 8f. Verification
- [x] `cargo build --workspace` / `cargo test --workspace` clean
- [x] `cargo clippy --workspace --all-targets --all-features` clean
- [x] `cargo xtask check` clean — fmt `--check`, clippy `-D warnings`, and
      `cargo deny check` (advisories/bans/licenses/sources ok) all pass.

## Phase 9 — Expanded-Vision Crate Planning (2026-08-18)

Triggered by `spec2.txt`, which proposes 11 new "Domain-Specific Component
Models" crates extending the workspace from mechanical/civil/general
engineering into electronics, energy, transport, medical, process, and
earth verticals (pcb, thermal-mgmt, power-components, renewables,
vehicle-dynamics, biomech, unit-ops, crystallography, geotech,
building-sys, schedule). This phase is **planning/docs-only** — no crates
were scaffolded or implemented.

### 9a. Gap analysis
Reviewing spec2.txt against the existing layering (props-* for fluid data,
materials/sections/structural for mechanical, controls for systems theory)
found that several of the 11 proposed crates assume foundational primitives
that don't exist anywhere in the workspace today:
- **Electrical/electronics base math** — `tpt-eng-pcb`,
  `tpt-eng-power-components`, `tpt-eng-renewables`, and
  `tpt-eng-building-sys` all need impedance, per-unit systems, and
  three-phase power math. `tpt-eng-controls` is transfer-function/
  state-space systems theory, not circuit analysis — nothing else covers
  this.
- **Heat-transfer correlations** — `tpt-eng-thermal-mgmt`,
  `tpt-eng-unit-ops`, and `tpt-eng-building-sys` all need convection/
  conduction correlations (Nu/Re/Pr, radiation view factors).
  `tpt-eng-props-water`/`-air` are property *tables*, not transfer-rate
  correlations.
- **General process-fluid properties** — `tpt-eng-unit-ops` needs real-gas/
  VLE property data for arbitrary process streams, beyond the three
  specific media (water/steam, moist air, combustion fuels) covered today.

### 9b. Decision
Add foundational crates for the electrical and heat-transfer gaps
(following the existing props/materials/sections layering pattern rather
than letting each domain crate re-derive the same math), plus a general
process-fluid props crate for the mixture gap. Finalized list — 14 new
crates total, recorded in `spec2.txt` (§3, §4):

- New Foundational Primitives: `tpt-eng-electrical`, `tpt-eng-heat-transfer`,
  `tpt-eng-props-mixture` (joins the `tpt-eng-props` umbrella).
- Domain-Specific Component Models (spec2.txt's original 11, now with
  dependency edges onto the foundational crates and existing crates —
  see spec2.txt's updated table for the full per-crate dependency list).

`tpt-eng-biomech`'s hyperelastic constitutive models (Mooney-Rivlin, Ogden)
stay domain-local rather than folding into `tpt-eng-materials`, matching
the precedent of `tpt-eng-structural` keeping its own closed-form beam
math instead of depending on a general FEM crate.

### 9c. Crate scaffolding
Each crate gets scaffolded with `cargo xtask new-crate <tpt-eng-name>`
(Cargo.toml + src/lib.rs + README.md + tests/basic.rs + examples/basic.rs),
then wired to the dependencies listed in spec2.txt's updated tables and
registered in the workspace `members`/`[workspace.dependencies]` lists.
Foundational crates must land first since most domain crates depend on them.

New Foundational Primitives:
- [x] `tpt-eng-electrical` — no deps beyond `tpt-math-units`, `tpt-math-numeric`.
      Impedance/reactance (R/L/C, series/parallel combination), per-unit
      system conversions, three-phase (balanced) power calculations,
      conductor/insulator base property lookups (resistivity, dielectric
      constant, ampacity) with `DataSource` provenance like `tpt-eng-materials`.
- [x] `tpt-eng-heat-transfer` — depends on `tpt-eng-props-air`, `tpt-eng-props-water`.
      Convection correlations (Nu/Re/Pr for flat plate, cylinder, sphere,
      internal pipe flow — laminar and turbulent), 1D and radial (cylindrical
      shell) conduction, radiation view factors and black/grey-body exchange,
      thermal-resistance-network composition (series/parallel R_th).
- [x] `tpt-eng-props-mixture` — no_std; depends on `tpt-math-units`. General
      real-gas equation-of-state property lookups (e.g. Peng-Robinson or
      similar) and basic VLE (bubble/dew point, K-values) for arbitrary
      user-defined process fluids/mixtures; wire into the `tpt-eng-props`
      umbrella's re-exports alongside water/air/fuels.

Domain-Specific Component Models (can be scaffolded in parallel once the
foundational crates above are registered):
- [x] `tpt-eng-pcb` — depends on `tpt-eng-electrical`, `tpt-eng-materials`.
      PCB layer stackup definitions (copper/dielectric layer ordering,
      thickness), trace routing primitives (width/spacing/current-capacity
      via `tpt-eng-electrical` ampacity), via and footprint (pad/hole)
      geometry definitions.
- [x] `tpt-eng-thermal-mgmt` — depends on `tpt-eng-heat-transfer`, `tpt-eng-props-air`.
      Heat sink sizing (fin arrays, base spreading resistance), fan curves
      (pressure-flow characteristic, operating-point intersection), and
      thermal-resistance-network assembly for electronics/enclosure cooling.
- [x] `tpt-eng-power-components` — depends on `tpt-eng-electrical`.
      Transformer equivalent-circuit models (turns ratio, leakage/magnetizing
      impedance), synchronous/induction generator models, transmission-line
      parameters (series impedance, shunt admittance per unit length).
- [x] `tpt-eng-renewables` — depends on `tpt-eng-electrical`, `tpt-eng-props-air`, `tpt-eng-reliability`.
      Solar PV I-V/P-V curve models (single-diode equivalent circuit), wind
      turbine power curves (cut-in/rated/cut-out, Betz-limited power),
      battery degradation/cycle-life models built on `tpt-eng-reliability`'s
      life-distribution machinery.
- [x] `tpt-eng-vehicle-dynamics` — depends on `tpt-eng-geometry`, `tpt-eng-structural`, `tpt-math-linalg`.
      Tire force models (Pacejka "magic formula"), aerodynamic drag/lift
      coefficients, suspension kinematics (double-wishbone/MacPherson
      geometry, roll-center calculation).
- [x] `tpt-eng-biomech` — depends on `tpt-eng-materials`, `tpt-eng-geometry`.
      Hyperelastic tissue constitutive models (Mooney-Rivlin, Ogden strain-
      energy functions and stress-strain evaluation), implant geometry
      primitives (stem/cup/plate parametric shapes) built on `tpt-eng-geometry`.
- [x] `tpt-eng-unit-ops` — depends on `tpt-eng-props`, `tpt-eng-heat-transfer`.
      Distillation column stage-by-stage (McCabe-Thiele style) calculations,
      shell-and-tube/plate heat exchanger sizing via LMTD and ε-NTU methods,
      pump/compressor performance-curve fitting and operating-point solving.
- [x] `tpt-eng-crystallography` — depends on `tpt-eng-geometry`.
      Miller index notation and plane/direction vector conversion, slip
      system enumeration for common crystal structures (FCC/BCC/HCP), crystal
      symmetry operations (point groups) over `tpt-eng-geometry` frames.
- [x] `tpt-eng-geotech` — depends on `tpt-eng-materials`, `tpt-math-linalg`.
      Soil constitutive models (Mohr-Coulomb failure envelope, Cam-Clay
      critical-state model), borehole/stratigraphy data structures (layer
      depth, soil classification, index properties) with `tpt-eng-materials`
      provenance tracking.
- [x] `tpt-eng-building-sys` — depends on `tpt-eng-props-air`, `tpt-eng-heat-transfer`, `tpt-eng-electrical`.
      HVAC load calculation (envelope conduction/infiltration/internal gains
      via `tpt-eng-heat-transfer` + `tpt-eng-props-air`), plumbing fixture-unit
      sizing tables, electrical panel scheduling (circuit/branch load
      tabulation via `tpt-eng-electrical`).
- [x] `tpt-eng-schedule` — depends on `tpt-math-numeric` only. CPM/PERT
      network construction (activity-on-node, forward/backward pass, float
      calculation), resource leveling, Earned Value Management metrics
      (PV/EV/AC, CPI/SPI).

### 9d. Explicitly deferred
- [ ] Implement each crate's actual domain logic (scaffolding above only
      produces empty-but-building stubs) — separate follow-up
      implementation task per crate/family.
- [x] Update `CRATE_AUDIT.md` and root `README.md` crate tables once the new
      crates exist.
- [x] `cargo build --workspace` / `cargo test --workspace` /
      `cargo xtask check` clean after all 14 crates are scaffolded and
      registered.
