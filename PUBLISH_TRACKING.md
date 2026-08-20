# Publish tracking — 2026-08-19 round

Working checklist for publishing the 14 crates added in `e3c74b9` ("Add new
engineering domain crates and restructure workspace") plus the `tpt-eng-props`
0.1.0 → 0.1.1 republish. Mirrors Batches 6–8 in `RELEASE.md`; check items off here
as you run them yourself, batch by batch, 5 at a time.

Before running any of these for real, each item was checked with
`cargo publish --dry-run --allow-dirty -p <crate>`. All 5 Batch 6 crates pass the
dry run cleanly right now (verified 2026-08-19). Batch 7 and 8 crates (and the
`tpt-eng-props` republish) can't be dry-run successfully yet — they fail with
`no matching package named '<dep>' found ... location searched: crates.io index`,
because their `tpt-eng-*` deps aren't live yet. That's expected (same failure mode
the original 29-crate release saw for its own later batches — see `RELEASE.md` §2);
re-run the dry run for a Batch 7/8 crate once its Batch 6/7 dependency is actually
published, right before you publish it for real.

Run from the workspace root. On a `429 Too Many Requests` (crates.io's new-crate
burst throttle — expect it after ~5 new crates), wait for the window given in the
error and re-run the same command.

---

## Batch 6 — foundation (only Batches 1–5 + `tpt-math-*` deps, no interdependencies)

- [x] `tpt-eng-props-mixture` — `cargo publish -p tpt-eng-props-mixture`
- [x] `tpt-eng-electrical` — `cargo publish -p tpt-eng-electrical`
- [x] `tpt-eng-schedule` — `cargo publish -p tpt-eng-schedule`
- [x] `tpt-eng-biomech` — `cargo publish -p tpt-eng-biomech`
- [x] `tpt-eng-crystallography` — `cargo publish -p tpt-eng-crystallography`

Why this order: none of these five depend on each other or on anything outside the
already-published 29 crates, so they can go in any order within the batch.

## Batch 7 — depends on Batch 6

- [x] `tpt-eng-geotech` — `cargo publish -p tpt-eng-geotech`
- [x] `tpt-eng-heat-transfer` — `cargo publish -p tpt-eng-heat-transfer`
- [x] `tpt-eng-vehicle-dynamics` — `cargo publish -p tpt-eng-vehicle-dynamics`
- [x] `tpt-eng-power-components` — `cargo publish -p tpt-eng-power-components`
  (needs `tpt-eng-electrical` from Batch 6)
- [x] `tpt-eng-props` **0.1.0 → 0.1.1** — `cargo publish -p tpt-eng-props`
  (needs `tpt-eng-props-mixture` from Batch 6 — do this one *last* in the batch)

Why this order: `geotech`, `heat-transfer`, and `vehicle-dynamics` only need
already-published crates, so they can go first. `power-components` needs
`electrical` (Batch 6, already live by the time you reach this batch). The
`tpt-eng-props` republish needs `props-mixture` (Batch 6) live first — safe once
Batch 6 is fully done.

## Batch 8 — depends on Batch 7

- [x] `tpt-eng-pcb` — `cargo publish -p tpt-eng-pcb`
- [x] `tpt-eng-renewables` — `cargo publish -p tpt-eng-renewables`
- [x] `tpt-eng-building-sys` — `cargo publish -p tpt-eng-building-sys`
  (needs `heat-transfer` + `electrical`)
- [x] `tpt-eng-thermal-mgmt` — `cargo publish -p tpt-eng-thermal-mgmt`
  (needs `heat-transfer`)
- [x] `tpt-eng-unit-ops` — `cargo publish -p tpt-eng-unit-ops`
  (needs `tpt-eng-props` 0.1.1 + `heat-transfer` — do this one *last*)

---

## Batch 9 — patch republish for new examples (2026-08-20)

Six crates already published in Batches 1-4 gained brand-new `examples/`
directories (they previously had none): `tpt-eng-cad`, `tpt-eng-gdt`,
`tpt-eng-safety`, `tpt-eng-standards`, `tpt-eng-structural`,
`tpt-eng-tolerance`. Per-crate `CHANGELOG.md`s are updated under
`[Unreleased]`. Since examples ship inside the published crate tarball and
are what docs.rs/crates.io show consumers, these won't be visible until a
version bump + republish:

- [x] Bump these 6 crates to `0.1.1` (`Cargo.toml` + `CHANGELOG.md` date)
      and `cargo publish -p <crate>` each once approved.

A further 29 crates (see each crate's `CHANGELOG.md` `[Unreleased] > Changed`
entry) only had their existing examples reformatted with `cargo fmt` — no
functional or public-API change, so **no version bump or republish is
needed** for those; the `[Unreleased]` note exists purely for the audit
trail.

## Post-publish

- [ ] Flip the `[ ]` → `[x]` checkboxes for Batches 6–8 in `RELEASE.md`, matching
  the style of Batches 1–5.
- [ ] Spot-check a few crates via `https://crates.io/api/v1/crates/<name>` to
  confirm `max_version` matches what was just published.
- [ ] Once everything above is live, this file can be deleted (its content folds
  into `RELEASE.md`'s Batches 6–8, which stay as the permanent record).
