# tpt-eng-props-air

ASHRAE moist-air (psychrometric) properties for HVAC and combustion-air
calculations. `no_std`-capable.

## Example

```rust
use tpt_math_units::uom::si::f64::*;
use tpt_math_units::uom::si::{pressure::kilopascal, thermodynamic_temperature::degree_celsius};
use tpt_eng_props_air::saturation_pressure_water;

// Saturation pressure of water at 100 °C ≈ 101.325 kPa.
let psat = saturation_pressure_water(ThermodynamicTemperature::new::<degree_celsius>(100.0));
assert!((psat.get::<kilopascal>() - 101.325).abs() < 0.5);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
