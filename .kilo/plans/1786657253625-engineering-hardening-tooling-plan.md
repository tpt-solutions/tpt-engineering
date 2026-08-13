# tpt-engineering — Hardening, Innovation & Adoption Tooling Plan

Status as reviewed (2026-08-14): All 13 `tpt-eng-*` crates are **real, substantive implementations** — no `todo!()` / `unimplemented!()` stubs. The only literal "TODO" strings are (a) a doc comment in `controls` and (b) the `xtask new-crate` default `desc`. These are cosmetic and listed under "Cosmetic" below, not as code stubs. Build relies on sibling `../tpt-math` (confirmed present).

Scope confirmed with user: **Everything incl. heavy tooling**. The two open `todo.md` items (registry.toml `git` flip + `v0.1.0` tag) are **external/blocked** — documented here as such, not implemented in this repo.

---

## 1. Correctness / robustness defects (fix first)

### 1.1 `tpt-eng-props-air` — divide-by-zero / non-physical inputs
`crates/tpt-eng-props-air/src/lib.rs`:
- `humidity_ratio` (line 85): `W = 0.621945·p_w/(p − p_w)` → divides by zero when `p_w == p`, negative when `p_w > p`. Add guard: return `Result<f64, Error>` or `0.0`/clamp; document that `p_w ≤ p` is required.
- `vapour_pressure_from_ratio` (line 93): fine, but validate `w ≥ 0`.
- `relative_humidity` (line 101): divides by `psat`; `psat` can be non-finite if `tk ≤ 0`. Add `T` sanity guard.
- `dew_point` (line 117): `humidity_ratio`/`psat` negative-path not reachable here, but clamps `pw ≤ 0` → bisect degenerate. Guard `pw > 0`.

Add an `Error` enum (`NonPhysicalInput`, `SaturationNotDefined`) like `props-water`, and migrate the affected fns to `Result`. Add unit tests for the guarded cases.

### 1.2 `tpt-eng-geo-asset` — inconsistent non-finite handling
`crates/tpt-eng-geo-asset/src/lib.rs`:
- `nearest` (line 111) filters non-finite coords; `within_radius` (line 123) does **not**, so a malformed entry can yield `NaN`-distance matches. Apply the same `is_finite()` filter in `within_radius`. Add a test.

### 1.3 `tpt-eng-network-matrix` — panic on dangling edge endpoints
`crates/tpt-eng-network-matrix/src/lib.rs` (lines 86–88): `idx[&e.from]` / `idx[&e.to]` panic if an edge references a node never added via `Topology::add_node`. `Topology::add_edge` auto-inserts endpoint nodes (line 99), so in practice safe, but the public API also exposes `edges()` mutation paths and the matrix fn is the trust boundary. Change `incidence_matrix` / `admittance_matrix` to skip edges whose endpoints are missing and/or return `Result<_, Error>`. Mirror the `props-water` `Error` pattern. Add a test for a dangling reference.

---

## 2. Security audit & tightening

Positives to preserve: `[workspace.lints.rust] unsafe_code = "forbid"`, pure-`std` `xtask`, `cargo-deny` in CI (advisories/licences/bans/sources).

Tighten:
- `deny.toml` (lines 34–36): change `unknown-registry = "warn"` and `unknown-git = "warn"` → `"deny"` so an unexpected dependency source fails CI instead of warning. Keep path deps to `../tpt-math` (legitimate).
- Add `[advisories] yanked = "deny"` (currently `"warn"`) to block yanked deps.
- Add `git` provenance: ensure `Cargo.lock` is committed (it is) and enable `cargo-deny` `sources.allow-registry = "https://github.com/rust-lang/crates.io-index"` + `allow-git = []` so only crates.io + the known path deps are permitted.
- CI: split the existing `cargo-deny` job and add a dedicated **`cargo-audit`** step (`cargo install cargo-audit` or `rustsec/audit-check@v2`) so RUSTSEC advisories are caught independently.
- Document a `SECURITY.md` (how to report, supported versions = none until `v0.1.0`).

No secrets, crypto, or network I/O exist in the crates; nothing to redact.

---

## 3. Innovation / easier-to-use additions

### 3.1 Cross-crate integration examples crate (highest value)
Add `crates/tpt-eng-examples` (or `examples/` at root) demonstrating an **end-to-end physical-systems scenario** that composes crates:
- Geo asset → topology → network matrix → (feed) → controls PID stabilizing a plant driven by a fuel's LHV; timeseries align/gap conditioning a simulated sensor stream; structural beam check on the same structure.
- This becomes the canonical "how do I use these together" doc + doctest.

### 3.2 Small ergonomics wins
- `tpt-eng-timeseries-align`: `align_to_grid` returns `vec![0.0; …]` for empty series — make documented & add `Option`/error-free note; consider `Result` if grid empty. (Keep API, just document.)
- `tpt-eng-controls::Pid`: the anti-windup branch (lines 139–142) is fragile/duplicated; add a unit test for integral clamping with limits + setpoint step. (Already has `pid_output_clamped`; add windup-recovery test.)
- `tpt-eng-structural`: `max_bending_moment` samples at 400 points (magic number) — expose `max_bending_moment_with_resolution(n)`.

---

## 4. Heavy adoption tooling

### 4.1 CI hardening (`.github/workflows/ci.yml`)
- Split `cargo-deny` into advisory(`deny`) job; add `cargo-audit` job.
- Add **`wasm32-unknown-unknown`** build job for the `no_std` props crates (toolchain already lists the target).
- Add **`no_std` for `tpt-eng-props` umbrella** already covered by `xtask no-std-matrix` — add a `docs`/`doctest` job: `cargo test --doc --workspace` + `cargo doc`.
- Add **MSRV / edition check**: workspace is `edition = "2024"`; pin CI to a known-good stable and assert no `edition` drift.

### 4.2 Toolchain/config fixes
- `rustfmt.toml` currently has `edition = "2021"` (line 1) which conflicts with the `2024` workspace edition and is a legacy/ignored key. Remove the `edition` line (rustfmt derives edition from `Cargo.toml`); keep `max_width = 100`.
- Add `.cargo/config.toml` (or extend existing `.cargo/`) with `[build] target = …` only if needed; otherwise document the `no_std` target in README.

### 4.3 Release & changelog tooling
- Add `release.toml` (or `cargo-release` config) + a `xtask release` helper (dry-run friendly, version-bumps all 13 crates consistently using the workspace `version = "0.1.0"` convention).
- Add `CHANGELOG.md` (Keep a Changelog format) seeded for `v0.1.0`.

### 4.4 One-command onboarding
- Add a root `justfile` (or `Makefile`) with: `just check` (`cargo xtask check`), `just test`, `just ci` (runs the full local CI replica), `just new <name>` → `cargo xtask new-crate`.
- README quickstart: `cargo add tpt-eng-props tpt-eng-controls` example + link to the integration examples crate.
- `xtask` new command: `xtask doctest` and `xtask doc` wrapping `cargo test --doc` / `cargo doc`.

### 4.5 External / blocked (do NOT implement here)
- **registry.toml flip** `planned → git` for all 13 crates — requires write access to sibling `tpt-rust-map` repo; document as maintainer action.
- **`v0.1.0` tag** — cut only after the registry flip + green CI on `main`; document as release-owner action.
- Update `todo.md` to mark these two as "blocked: external repo / maintainer".

---

## 5. Validation

1. `cargo xtask check` (fmt+clippy+deny) clean.
2. `cargo test --workspace --all-features` + `cargo test --doc --workspace` pass.
3. `cargo xtask no-std-matrix` (thumbv6m) + new wasm32 build both green.
4. New guard tests in `props-air`, `geo-asset`, `network-matrix` pass.
5. `cargo audit` and `cargo deny check` clean with `deny` severities.
6. Integration examples crate builds & its doctest executes.

## 6. Open questions / risks
- `tpt-math` path-dependency versions (`0.1.0`) must stay in lockstep when `tpt-engineering` publishes — add a CI job that fails if sibling `tpt-math` is missing (already local-only).
- `edition = "2024"` requires Rust ≥ 1.85; CI uses `dtolnay/rust-toolchain@stable` so fine, but document the floor in README.
