# tpt-eng-props-water

IAPWS-IF97 water/steam property tables (Region 1 liquid, Region 2 vapour,
Region 4 saturation). `no_std`-capable.

## Example

```rust
use tpt_eng_props_water::state;
use tpt_math_units::uom::si::f64::*;
use tpt_math_units::uom::si::{
    pressure::megapascal, specific_volume::cubic_meter_per_kilogram,
    thermodynamic_temperature::kelvin,
};

// v(300 K, 3 MPa) ≈ 0.00100215 m³/kg (IAPWS-IF97 reference value).
let t = ThermodynamicTemperature::new::<kelvin>(300.0);
let p = Pressure::new::<megapascal>(3.0);
let s = state(t, p).unwrap();
assert!((s.specific_volume.get::<cubic_meter_per_kilogram>() - 0.00100215168).abs() < 1e-9);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
