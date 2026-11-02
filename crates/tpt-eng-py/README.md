# tpt-eng (Python)

Python bindings for [tpt-engineering](../../README.md), built with PyO3 and maturin.

| Module | Contents |
| --- | --- |
| `tpt_eng` | IAPWS-IF97 water and steam properties |
| `tpt_eng.air` | Moist-air psychrometrics: saturation pressure, humidity ratio, relative humidity, enthalpy, dew point |
| `tpt_eng.fuels` | `Fuel` and `BlendedFuel`: heating values, density, air–fuel ratio, CO₂ factor |
| `tpt_eng.controls` | `Pid` discrete-time controller |
| `tpt_eng.structural` | `SimplySupportedBeam` statics and `section_utilization` |

```python
import tpt_eng
from tpt_eng.controls import Pid
from tpt_eng.structural import SimplySupportedBeam, section_utilization

p_sat = tpt_eng.water_saturation_pressure_pa(373.15)   # ~101.3 kPa
s = tpt_eng.water_state(300.0, 3.0e6)
print(s.region, s.enthalpy_kj_per_kg, s.entropy_kj_per_kg_k)

pid = Pid(2.0, 1.0, 0.0, output_min=-5.0, output_max=5.0)
pid.set_setpoint(10.0)
u = pid.update(measurement=0.0, dt_s=0.01)

beam = SimplySupportedBeam(4.0)           # 4 m span
beam.add_point_load(2.0, 10_000.0)        # 10 kN at midspan
print(beam.max_bending_moment_nm())       # 10 000 N·m
print(section_utilization(10_000.0, 1.0e-4, 200.0e6))  # 0.5
```

## Conventions

- Inputs and outputs are plain `float`. Units are in each name: `_k`, `_pa`,
  `_m`, `_n`, `_nm`, `_s`, `_kj_per_kg`.
- Out-of-range or invalid inputs raise `ValueError`. Region 3 (near-critical)
  water raises `NotImplementedError`.
- Positions are metres from the pin. Downward forces are positive. Moments are
  counter-clockwise positive.

## Provenance-tagged results

Some correlations are only valid over a range of inputs. The `*_tagged`
functions return the same value as their plain counterparts, wrapped in a
`tpt_eng.Tagged` object. That object reports the source, the validity range,
and a warning for each input outside its range.

```python
from tpt_eng import air

t = air.relative_humidity_tagged(1000.0, 500.0)  # 500 K is above the 473.15 K limit
t.value          # same number as air.relative_humidity would give
t.in_range       # False
t.warnings       # ["temperature = 500 K is outside the validated range ..."]
t.source         # "ASHRAE Fundamentals, Ch. 1 (Hyland-Wexler)"

air.relative_humidity_tagged(1000.0, 500.0, strict=True)  # raises ValueError
```

Available: `tpt_eng.water_saturation_pressure_tagged`,
`tpt_eng.water_state_tagged`, `tpt_eng.air.saturation_pressure_water_tagged`,
and `tpt_eng.air.relative_humidity_tagged`. Hard input errors, such as
non-finite values or a temperature of zero, still raise even without
`strict`.

## Build and test

Requires Rust 1.85+ and Python 3.10+.

```bash
pip install maturin pytest
cd crates/tpt-eng-py
maturin develop            # builds into the current virtualenv
pytest tests
```

To build a wheel: `maturin build --release`. The wheel is `abi3`, so one
wheel covers every CPython 3.10 and later.
