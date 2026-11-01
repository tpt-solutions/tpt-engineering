# tpt-eng (Python)

Python bindings for [tpt-engineering](../../README.md), built with PyO3 and maturin.

v0.1 covers IAPWS-IF97 water and steam properties.

```python
import tpt_eng

p_sat = tpt_eng.water_saturation_pressure_pa(373.15)   # ~101.3 kPa
t_sat = tpt_eng.water_saturation_temperature_k(101_325.0)

s = tpt_eng.water_state(300.0, 3.0e6)
print(s.region, s.enthalpy_kj_per_kg, s.entropy_kj_per_kg_k)
```

## Conventions

- Inputs and outputs are plain `float`. Units are in each name: `_k`, `_pa`,
  `_kj_per_kg`, `_kj_per_kg_k`.
- Out-of-range inputs raise `ValueError`. Region 3 (near-critical) raises
  `NotImplementedError`.

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
