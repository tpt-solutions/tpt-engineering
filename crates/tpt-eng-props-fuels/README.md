# tpt-eng-props-fuels

Fuel properties for energy and combustion calculations: lower/higher heating
values, density, stoichiometric air–fuel ratio, and CO₂ emission factors for
natural gas, hydrogen, hydrogen–natural-gas blends, and diesel. `no_std`-capable.

## Example

```rust
use tpt_math_units::uom::si::f64::*;
use tpt_eng_props_fuels::Fuel;

assert!((Fuel::Methane.lhv_mj_kg() - 50.0).abs() < 1.0);
// Diesel needs more air per kg than methane.
assert!(Fuel::Diesel.stoichiometric_air_fuel_ratio() < Fuel::Methane.stoichiometric_air_fuel_ratio());
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
