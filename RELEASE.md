# tpt-engineering — crates.io Release Plan (v0.1.0)

Last updated: 2026-08-15
Workspace: `tpt-engineering` (29 `tpt-eng-*` crates + `xtask`)
Tool used for the dry run: `cargo publish --dry-run` (cargo 1.97.1)

---

## 1. Prerequisites

- [x] **`tpt-math-*` substrate already on crates.io** — verified live:
  `tpt-math-units`, `tpt-math-numeric`, `tpt-math-linalg`, `tpt-math-stats`
  are all published at `0.1.0`, matching the workspace `version = "0.1.0"`
  requirement. Every `tpt-eng-*` crate depends on these, so this must be true
  before anything here can publish (it is).
- [ ] **crates.io token** exported: `set CARGO_REGISTRY_TOKEN=<token>` (or
      `~/.cargo/credentials` configured).
- [ ] **Clean tree (recommended)** — commit the current working-tree changes
      (`Cargo.toml`, `README.md`, `Cargo.lock`, `crates/tpt-eng-cli/*`) first,
      then publish without `--allow-dirty`. If you publish with uncommitted
      changes, add `--allow-dirty` to every `cargo publish` command below.
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

### Batch 1 — Foundation (no `tpt-eng` deps; only `tpt-math-*`)
- [ ] `tpt-eng-geometry`
- [ ] `tpt-eng-materials`
- [ ] `tpt-eng-sections`
- [ ] `tpt-eng-plot`
- [ ] `tpt-eng-report`
- [ ] `tpt-eng-geo-asset`  (→ tpt-math-units)

### Batch 2 — Foundation continued (only `tpt-math-*` deps)
- [ ] `tpt-eng-controls`       (→ tpt-math-linalg)
- [ ] `tpt-eng-reliability`     (→ tpt-math-stats)
- [ ] `tpt-eng-tolerance`       (→ tpt-math-stats)
- [ ] `tpt-eng-props-water`     (→ tpt-math-units, tpt-math-numeric)
- [ ] `tpt-eng-props-air`       (→ tpt-math-units, tpt-math-numeric)
- [ ] `tpt-eng-props-fuels`     (→ tpt-math-units, tpt-math-numeric)

### Batch 3 — Level 1 (depend on Batches 1–2)
- [ ] `tpt-eng-timeseries-core` (→ tpt-math-numeric)
- [ ] `tpt-eng-mesh`            (→ geometry)
- [ ] `tpt-eng-geo-topology`    (→ geo-asset)
- [ ] `tpt-eng-gdt`             (→ geometry, tolerance)
- [ ] `tpt-eng-network-matrix`  (→ geo-topology, tpt-math-linalg)
- [ ] `tpt-eng-safety`          (→ reliability, tolerance, tpt-math-units)

### Batch 4 — Level 2 (depend on Batches 1–3)
- [ ] `tpt-eng-structural`      (→ safety, tpt-math-units)
- [ ] `tpt-eng-nurbs`           (→ geometry, mesh)
- [ ] `tpt-eng-timeseries-align`(→ timeseries-core)
- [ ] `tpt-eng-timeseries-gap`  (→ timeseries-core)
- [ ] `tpt-eng-io`              (→ mesh, geometry)
- [ ] `tpt-eng-cad`             (→ geometry, mesh, nurbs, gdt)

### Batch 5 — Top level (depend on everything above)
- [ ] `tpt-eng-standards`       (→ safety)
- [ ] `tpt-eng-props`           (→ props-water, props-air, props-fuels)
- [ ] `tpt-eng-timeseries`      (→ timeseries-core, align, gap)
- [ ] `tpt-eng-cli`             (→ io, report, plot, materials, sections, structural, controls, tolerance, props-{water,air,fuels})
- [ ] `tpt-eng-examples`        (→ geo-topology, network-matrix, controls, props-fuels, timeseries-{core,align,gap}, structural, sections, materials, tolerance, gdt, geometry, report)

---

## 5. Post-release checklist

- [ ] All 29 `tpt-eng-*` crates show as published `0.1.0` on crates.io.
- [ ] `cargo add tpt-eng-<name>` resolves for a fresh consumer (confirms the
      dependency graph is satisfiable end-to-end).
- [ ] Flip `status = "planned"` → `"git"` for the crates in the sibling
      `tpt-rust-map/registry.toml` (external repo — maintainer action).
- [ ] Cut the `v0.1.0` git tag once CI is green on `main` (release-owner action).
