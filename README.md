# tpt-eng-sections

Cross-section properties for the TPT engineering ecosystem.

Every section implements the `Section` trait, which exposes area, centroid,
centroidal second moments, elastic/plastic section moduli, and the torsional
constant. Properties are assembled into a single `SectionProperties` bundle via
`Section::properties`.

## Features

- **Section trait** (`section`): uniform `Section` interface for all shapes.
- **Properties** (`properties`): `SectionProperties` bundle (area, centroid,
  second moments, section/plastic moduli, torsion).
- **Standard shapes** (`shapes`): `Rectangle`, `Circle`, `Tube`, `ISection`,
  `Channel`, `Angle`.
- **Arbitrary polygons** (`polygon`): `CustomPolygon` via exact Green's-theorem
  formulas, with plastic moduli and torsion on a confined grid.
- **Composite evaluation** (`compose`): rectangle decomposition for composite
  sections and grid helpers for centroids, second moments, plastic moduli, and
  torsion.

## Supported sections

- `Rectangle`
- `Circle`
- `Tube` (circular hollow)
- `ISection`
- `Channel`
- `Angle`
- `CustomPolygon` (arbitrary simply-connected polygon)

Composite sections (I-section, channel, angle) are evaluated by rectangle
decomposition (`compose`); arbitrary polygons use exact Green's-theorem formulas
for area/centroid/second moments, with plastic moduli and torsion computed on a
grid confined to the polygon.

All quantities are reported in the section's own consistent length units; the
caller is responsible for unit consistency (integration with `tpt-eng-units` is
deferred).

Geometry integration with `tpt-eng3` (`tpt-eng-geometry`) is deferred until that
repository/crate exists.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tpt-eng-sections = "0.1"
```

## Quick start

```rust
use tpt_eng_sections::{ISection, Section};

// W-shape: depth 10, flange width 6, flange thickness 1, web thickness 0.5.
let s = ISection::new(10.0, 6.0, 1.0, 0.5);
// Area = 2*6*1 + 0.5*(10 - 2) = 16.
assert!((s.area() - 16.0).abs() < 1e-9);
let props = s.properties();
assert!(props.area > 0.0);
```

## Crate modules

| Module | Purpose |
| --- | --- |
| `section` | `Section` trait and shared interface. |
| `properties` | `SectionProperties` bundle. |
| `shapes` | `Rectangle`, `Circle`, `Tube`, `ISection`, `Channel`, `Angle`. |
| `polygon` | `CustomPolygon` arbitrary-section evaluator. |
| `compose` | Rectangle decomposition and grid helpers for composites. |

The `prelude` module re-exports the most commonly used items.

## Related crates

- [`tpt-eng-linalg`](../tpt-eng-linalg) — linear algebra utilities.
- [`tpt-eng-optimize`](../tpt-eng-optimize) — engineering optimization.
- [`tpt-eng-materials`](../tpt-eng-materials) — material property modeling.
- [`tpt-eng-standards`](../tpt-eng-standards) — standards modeling as data.

## Status

Initial `0.1.0` release. Depends on `tpt-eng-core` / `tpt-eng-math` from the
sibling `tpt-eng1` repository. Unit and geometry integrations are deferred.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) OR
[Apache-2.0](../../LICENSE-APACHE).
