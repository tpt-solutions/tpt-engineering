# tpt-eng-sheet

Declarative calculation sheets. A sheet is a small TOML or JSON file that names
one calculation and gives its inputs. The unit is part of each field name, such
as `pressure_kpa` or `span_m`. Running a sheet produces a calculation report.

```toml
title = "Lintel check"

[calc]
kind = "beam-udl"
span_m = 4.0
load_n_per_m = 2000.0
width_m = 0.1
height_m = 0.2
allowable_stress_mpa = 12.0
```

```bash
tpt-eng sheet run lintel.toml                 # Markdown to stdout
tpt-eng sheet run lintel.toml --out lintel.html
tpt-eng --json sheet run lintel.toml          # JSON to stdout
tpt-eng sheet schema --out calc-sheet.schema.json
```

## Calculations

| `kind` | Inputs | What it reports |
| --- | --- | --- |
| `water-state` | `temperature_c`, `pressure_kpa` | IF97 state, with a warning outside the validated range |
| `beam-udl` | `span_m`, `load_n_per_m`, `width_m`, `height_m`, optional `allowable_stress_mpa` | Reactions, maximum moment and stress, and a utilisation check if an allowable stress is given |
| `pid-step` | `kp`, `ki`, `kd`, `setpoint`, `tau_s`, `dt_s`, `duration_s`, optional `output_limit` | Final value, peak and overshoot of the closed-loop step response |

Unknown fields and unknown `kind` values are rejected, so a misspelt input is
an error rather than being ignored.

## Schema

[`schema/calc-sheet.schema.json`](schema/calc-sheet.schema.json) is generated
from the Rust types and committed. A unit test fails if it falls out of date.
Regenerate it with the command shown above.

## Library use

```rust
use tpt_eng_sheet::{Sheet, run};

let sheet = Sheet::from_toml(&std::fs::read_to_string("lintel.toml")?)?;
let report = run(&sheet)?;
println!("{}", tpt_eng_report::to_markdown(&report));
```
