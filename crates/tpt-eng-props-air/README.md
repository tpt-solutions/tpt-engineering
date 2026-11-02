# tpt-eng-props-air

ASHRAE moist-air (psychrometric) properties for HVAC and combustion-air
calculations. `no_std`-capable.

This crate implements the ASHRAE Fundamentals saturation-pressure correlation
(Hyland–Wexler, valid ~0–200 °C over liquid water) and the standard moist-air
relations: saturation pressure, humidity ratio, relative humidity, specific
enthalpy, and dew-point temperature.

Temperatures are [`uom`](https://docs.rs/uom)-typed in kelvin; pressures in pascals.
Dimensionless ratios and per-mass enthalpies are plain `f64`. All functions
validate their inputs and return a typed [`Error`] rather than panicking on
non-physical values.

## Features

- `saturation_pressure_water` — Hyland–Wexler saturation pressure of water over
  liquid water (ASHRAE Fundamentals formulation).
- `humidity_ratio` / `vapour_pressure_from_ratio` — forward and inverse
  water-vapour / dry-air ratio.
- `relative_humidity` — vapour pressure fraction of saturation pressure.
- `moist_air_enthalpy` — specific enthalpy `h = 1.006·T + w·(2501 + 1.86·T)`
  (kJ/kg dry air).
- `dew_point` — dew-point temperature from a vapour partial pressure (bisection
  of the saturation correlation).
- Input validation returning [`Error`] for non-physical temperatures, pressures,
  and ratios.
- **Provenance-tagged variants** — [`saturation_pressure_water_tagged`] and
  [`relative_humidity_tagged`] return the same value as their untagged
  counterparts, plus the source, the validity range, and an out-of-range warning
  (see [tpt-eng-provenance](../tpt-eng-provenance/)).

## Installation

```toml
[dependencies]
tpt-eng-props-air = "0.1"
```

## Quick start

```rust
use tpt_math_units::uom::si::f64::*;
use tpt_math_units::uom::si::{pressure::kilopascal, thermodynamic_temperature::degree_celsius};
use tpt_eng_props_air::{humidity_ratio, saturation_pressure_water};

// Saturation pressure of water at 100 °C ≈ 101.325 kPa.
let psat = saturation_pressure_water(ThermodynamicTemperature::new::<degree_celsius>(100.0));
assert!((psat.get::<kilopascal>() - 101.325).abs() < 0.5);

// At 2 kPa vapour in 101.325 kPa total air, humidity ratio W ≈ 0.0125.
let pw = Pressure::new::<kilopascal>(2.0);
let p = Pressure::new::<kilopascal>(101.325);
let w = humidity_ratio(pw, p).unwrap();
assert!((w - 0.0125).abs() < 1e-3);
```

## Crate items

| Item | Purpose |
| --- | --- |
| `saturation_pressure_water` | Saturation vapour pressure of water over liquid water (Pa). |
| `humidity_ratio` | `W = 0.621945·p_w/(p − p_w)` from vapour and total pressure. |
| `vapour_pressure_from_ratio` | Inverse of `humidity_ratio`. |
| `relative_humidity` | `φ = p_w / p_sat` in `[0, 1]`. |
| `moist_air_enthalpy` | Specific enthalpy of moist air, kJ/kg dry air. |
| `dew_point` | Dew-point temperature (K) from a vapour partial pressure. |
| `saturation_pressure_water_tagged` / `relative_humidity_tagged` | Provenance-tagged `saturation_pressure_water` and `relative_humidity`. |
| `Error` | Validation errors for non-physical inputs. |

## Related crates

- [tpt-eng-props](../tpt-eng-props/) — umbrella re-exporting this crate as `props::air`.
- [tpt-eng-props-fuels](../tpt-eng-props-fuels/) — combustion fuel properties.
- [tpt-eng-props-water](../tpt-eng-props-water/) — IAPWS-IF97 water/steam.

## Status

`0.1.1` adds the provenance-tagged functions. `no_std`-capable.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) OR [Apache-2.0](../../LICENSE-APACHE).
