# tpt-eng-structural

Structural-engineering primitives: load definitions, simply-supported beam
analysis (reactions, shear, and bending moment), and demand/capacity
code-compliance checks in an ASCE 7 / Eurocode-style utilisation-ratio form.

## Example

```rust
use tpt_math_units::uom::si::f64::*;
use tpt_math_units::uom::si::{force::kilonewton, length::meter, torque::kilonewton_meter};
use tpt_eng_structural::{Beam, Load};

// 10 m simply-supported beam with a 10 kN point load at mid-span.
let mut beam = Beam::new(Length::new::<meter>(10.0));
beam.add(Load::point(Length::new::<meter>(5.0), Force::new::<kilonewton>(10.0)));

assert!((beam.reaction_a().get::<kilonewton>() - 5.0).abs() < 1e-9);
assert!((beam.max_bending_moment().get::<kilonewton_meter>() - 25.0).abs() < 1e-9);
```

## Related crates

- [`tpt-eng-safety`](../tpt-eng-safety) — consolidated utilization/pass-fail evaluation that this crate delegates to.
- [`tpt-eng-standards`](../tpt-eng-standards) — standards modeled as data; also delegates utilization to `tpt-eng-safety`.
- [`tpt-eng-sections`](../tpt-eng-sections) — cross-section properties for section checks.

Utilization and pass/fail evaluation are **consolidated in
[`tpt-eng-safety`](../tpt-eng-safety)** (`tpt_eng_safety::utilization`); this
crate delegates its demand/capacity utilization checks to it rather than
re-implementing the math.

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
