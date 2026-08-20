# tpt-engineering — crates.io Release Plan (v0.1.0)

Last updated: 2026-08-20
Workspace: `tpt-engineering` (43 `tpt-eng-*` crates + `xtask`)
Tool used for the dry run: `cargo publish --dry-run` (cargo 1.97.1)

---

## 1. Prerequisites

- [x] **`tpt-math-*` substrate already on crates.io** — verified live:
  `tpt-math-units`, `tpt-math-numeric`, `tpt-math-linalg`, `tpt-math-stats`
  are all published at `0.1.0`, matching the workspace `version = "0.1.0"`
  requirement. Every `tpt-eng-*` crate depends on these, so this must be true
  before anything here can publish (it is).
- [x] **crates.io token** exported: `set CARGO_REGISTRY_TOKEN=<token>` (or
      `~/.cargo/credentials` configured). — verified: `~/.cargo/credentials.toml`
      has a `[registry] token` entry and authenticated for Batch 1.
- [x] **Clean tree (recommended)** — Batch 1 published **without**
      `--allow-dirty` (the only working-tree change was an untracked root file,
      which is outside every crate's package directory). If a later batch has
      uncommitted changes inside a crate directory, either commit them or add
      `--allow-dirty` to that `cargo publish` command.
- [ ] **Version** — all crates are at `0.1.0`. This plan publishes `0.1.0` of
      each. Do not bump versions mid-release.
- [ ] **`xtask` is excluded** — it is a workspace dev tool, not a publishable
      library, and is omitted from all batches.

> Note: `release.toml` sets `publish = false`, but that is **cargo-release**
> config only and does **not** block `cargo publish`. The dry run proved every
> crate's Cargo.toml is publishable as-is.

---

## 2. Dry-run results (2026-08-15)

`cargo publish --dry-run --allow-dirty` was run for all 29 crates.

| Result | Count | Crates |
|--------|-------|--------|
| Package **+** verify build PASS (deps only `tpt-math-*`, already on crates.io) | 13 | all "Batch 1 + Batch 2" foundation crates (L0) |
| Package BLOCKED — only by an unpublished `tpt-eng-*` dependency | 16 | everything in Batches 3–5 |

The 16 "blocked" crates fail **only** with
`no matching package named 'tpt-eng-<dep>' found ... location searched: crates.io index`.
There were **zero** missing-readme / missing-license / exclude / metadata errors.
Once the dependencies are published (in order, below), those 16 crates package
and verify cleanly — this is exactly what the 5-batch ordering guarantees.

Full per-crate logs: `C:\Users\phill\AppData\Local\Temp\kilo\dryrun\summary.txt`
(and `pkg_*.log` / `ver_*.log` alongside it).

---

## 3. How to release

Release **batch by batch, in order**. Within a batch, publish crates in the
listed order (a crate's dependencies always appear in an earlier batch, or
earlier in the same batch).

Manual command for a single crate (run from the workspace root):

```powershell
cargo publish --allow-dirty -p <crate>
```

Convenience: publish an entire batch at once (PowerShell), in the correct order:

```powershell
foreach ($c in @('tpt-eng-geometry','tpt-eng-materials','tpt-eng-sections',
                  'tpt-eng-plot','tpt-eng-report','tpt-eng-geo-asset')) {
    cargo publish --allow-dirty -p $c
}
```

`cargo publish` always re-runs the package + verify build before uploading, so
you get the same safety net the dry run provided. Watch for the
`Uploading ... v0.1.0` line confirming success.

---

## 4. Release batches (topological order)

Legend: `[ ]` = not yet released · `[x]` = released to crates.io.

### Batch 1 — Foundation (no `tpt-eng` deps; only `tpt-math-*`) — RELEASED 2026-08-15
- [x] `tpt-eng-geometry`
- [x] `tpt-eng-materials`
- [x] `tpt-eng-sections`
- [x] `tpt-eng-plot`
- [x] `tpt-eng-report`
- [x] `tpt-eng-geo-asset`  (→ tpt-math-units)

> Note: crates.io limits **new** crate publishes to a burst of 5, then one per
> ~10 minutes. The 6th crate in this batch (`tpt-eng-geo-asset`) hit
> `429 Too Many Requests` and succeeded on retry after the window reset. Expect
> the same throttle in Batches 2–5: on 429, wait until the time given in the
> error and re-run the same `cargo publish` command.

### Batch 2 — Foundation continued (only `tpt-math-*` deps) — RELEASED 2026-08-15
- [x] `tpt-eng-controls`       (→ tpt-math-linalg)
- [x] `tpt-eng-reliability`     (→ tpt-math-stats)
- [x] `tpt-eng-tolerance`       (→ tpt-math-stats)
- [x] `tpt-eng-props-water`     (→ tpt-math-units, tpt-math-numeric)
- [x] `tpt-eng-props-air`       (→ tpt-math-units, tpt-math-numeric)
- [x] `tpt-eng-props-fuels`     (→ tpt-math-units, tpt-math-numeric)

> As predicted, the 6th crate (`tpt-eng-props-fuels`) hit the new-crate
> `429` throttle and was published on retry after the ~5-minute window reset.

### Batch 3 — Level 1 (depend on Batches 1–2) — RELEASED 2026-08-15
- [x] `tpt-eng-timeseries-core` (→ tpt-math-numeric)
- [x] `tpt-eng-mesh`            (→ geometry)
- [x] `tpt-eng-geo-topology`    (→ geo-asset)
- [x] `tpt-eng-gdt`             (→ geometry, tolerance)
- [x] `tpt-eng-network-matrix`  (→ geo-topology, tpt-math-linalg)
- [x] `tpt-eng-safety`          (→ reliability, tolerance, tpt-math-units)

> The 5th and 6th crates (`network-matrix`, `safety`) hit the new-crate `429`
> throttle and were published on retry. Note: because the burst allowance is
> shared across the account, the throttle now kicks in around the 5th crate of
> a batch and the wait can be ~7–10 min.

### Batch 4 — Level 2 (depend on Batches 1–3) — RELEASED 2026-08-15
- [x] `tpt-eng-structural`      (→ safety, tpt-math-units)
- [x] `tpt-eng-nurbs`           (→ geometry, mesh)
- [x] `tpt-eng-timeseries-align`(→ timeseries-core)
- [x] `tpt-eng-timeseries-gap`  (→ timeseries-core)
- [x] `tpt-eng-io`              (→ mesh, geometry)
- [x] `tpt-eng-cad`             (→ geometry, mesh, nurbs, gdt)

> Published in dependency order (`nurbs` before `cad`). The 6th crate
> (`tpt-eng-cad`) hit the new-crate `429` throttle and was published on retry
> (this time only ~26 s wait — the account burst had partially refilled).

### Batch 5 — Top level (depend on everything above) — RELEASED 2026-08-15
- [x] `tpt-eng-standards`       (→ safety)
- [x] `tpt-eng-props`           (→ props-water, props-air, props-fuels)
- [x] `tpt-eng-timeseries`      (→ timeseries-core, align, gap)
- [x] `tpt-eng-cli`             (→ io, report, plot, materials, sections, structural, controls, tolerance, props-{water,air,fuels})
- [x] `tpt-eng-examples`        (→ geo-topology, network-matrix, controls, props-fuels, timeseries-{core,align,gap}, structural, sections, materials, tolerance, gdt, geometry, report)

> Final batch. Only `tpt-eng-standards` fit in the account's new-crate burst;
> `props`, `timeseries`, `cli`, and `examples` all hit the `429` throttle and
> were published one-by-one on retry (waits ~4–10 min each as the burst
> refilled). All five live at `0.1.0`.

---

### Batch 6 — New domain crates, foundation (deps satisfied by Batches 1–5 + `tpt-math-*` only) — RELEASED 2026-08-20
- [x] `tpt-eng-props-mixture`  (→ tpt-math-units, tpt-math-numeric)
- [x] `tpt-eng-electrical`     (→ tpt-math-units, tpt-math-numeric)
- [x] `tpt-eng-schedule`       (→ tpt-math-numeric)
- [x] `tpt-eng-biomech`        (→ materials, geometry)
- [x] `tpt-eng-crystallography`(→ geometry)

### Batch 7 — New domain crates, level 1 (depend on Batches 1–6) — RELEASED 2026-08-20
- [x] `tpt-eng-geotech`        (→ materials)
- [x] `tpt-eng-heat-transfer`  (→ props-air, props-water)
- [x] `tpt-eng-vehicle-dynamics` (→ geometry, structural, tpt-math-linalg)
- [x] `tpt-eng-power-components` (→ electrical)
- [x] `tpt-eng-props` **republished at 0.1.1** (→ props-mixture; adds the `mixture`
      re-export — published after `tpt-eng-props-mixture` went live)

> `tpt-eng-heat-transfer`'s `Cargo.toml` originally listed `categories =
> ["science", "algorithms", "physics"]`; crates.io rejected the publish with
> `400 Bad Request: category slugs ... physics` (not a supported slug). Fixed
> by dropping `"physics"` (commit `72c8cd6`) and republishing.
>
> `tpt-eng-power-components` hit the new-crate `429` throttle after 4
> back-to-back publishes in this batch; published on retry ~8 minutes later.

### Batch 8 — New domain crates, level 2 (depend on Batches 1–7) — RELEASED 2026-08-20
- [x] `tpt-eng-pcb`            (→ electrical, materials)
- [x] `tpt-eng-renewables`     (→ electrical, props-air, reliability)
- [x] `tpt-eng-building-sys`   (→ heat-transfer, electrical, props-air)
- [x] `tpt-eng-thermal-mgmt`   (→ heat-transfer, props-air)
- [x] `tpt-eng-unit-ops`       (→ tpt-eng-props 0.1.1, heat-transfer)

> No `429` throttling hit in this batch.

### Batch 9 — Patch republish for new examples (Batches 1–4 crates) — RELEASED 2026-08-20
Six crates already published in Batches 1–4 gained brand-new `examples/`
directories they previously lacked. Since examples ship inside the published
crate tarball (what docs.rs/crates.io show consumers), they weren't visible
until a version bump + republish. Published in dependency order:
- [x] `tpt-eng-safety` **0.1.0 → 0.1.1**
- [x] `tpt-eng-tolerance` **0.1.0 → 0.1.1**
- [x] `tpt-eng-gdt` **0.1.0 → 0.1.1** (→ tolerance)
- [x] `tpt-eng-structural` **0.1.0 → 0.1.1** (→ safety)
- [x] `tpt-eng-standards` **0.1.0 → 0.1.1** (→ safety)
- [x] `tpt-eng-cad` **0.1.0 → 0.1.1** (→ gdt)

> Since `version.workspace = true` is shared by many unrelated crates,
> versions were overridden explicitly per-crate (`version = "0.1.1"`) rather
> than bumping `[workspace.package].version`, to avoid dragging every other
> crate's version along.

> See `PUBLISH_TRACKING.md` for the working checklist with exact `cargo publish`
> commands for this round (Batches 6–9 were added 2026-08-19/20, after the
> workspace grew from 29 to 43 `tpt-eng-*` crates).

---

## 5. Post-release checklist

- [x] All 43 `tpt-eng-*` crates show as published on crates.io.
      (Verified per-crate via the crates.io API at the end of each batch; all
      Batch 1–8 crates at `0.1.0` except the six Batch 9 crates and
      `tpt-eng-props`, which are at `0.1.1`; none yanked.)
- [x] `cargo add tpt-eng-<name>` resolves for a fresh consumer (confirms the
      dependency graph is satisfiable end-to-end).
      (Verified: a throwaway crate `cargo add tpt-eng-cli tpt-eng-examples` +
      `cargo update` resolved the full transitive `tpt-eng-*` graph from
      crates.io with exit 0.)
- [ ] Flip `status = "planned"` → `"git"` for the crates in the sibling
      `tpt-rust-map/registry.toml` (external repo — maintainer action).
- [ ] Cut the `v0.1.0` git tag once CI is green on `main` (release-owner action).
