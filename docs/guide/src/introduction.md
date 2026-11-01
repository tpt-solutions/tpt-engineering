# Introduction

`tpt-engineering` is a set of small, single-purpose `tpt-eng-*` crates for
physical-systems engineering: fluid and gas properties, time-series
conditioning, infrastructure topology, controls, structural analysis,
tolerancing, reliability, geometry and CAD primitives, and domain component
models.

Each crate is published on crates.io at `0.1.0`. You can depend on one crate
or several. The crates do not depend on a shared runtime.

```toml
[dependencies]
tpt-eng-structural = "0.1"
```

```rust,ignore
use tpt_eng_structural::{Beam, Load};
use tpt_math_units::uom::si::f64::{Force, Length};
use tpt_math_units::uom::si::{force::kilonewton, length::meter};

let mut beam = Beam::new(Length::new::<meter>(10.0));
beam.add(Load::point(Length::new::<meter>(5.0), Force::new::<kilonewton>(10.0)));
```

This guide is a cookbook. The API reference is on each crate's documentation
(`cargo doc --workspace --no-deps`). Numerical reference points are listed in
[Validation](validation.md).
