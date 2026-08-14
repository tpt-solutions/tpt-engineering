# tpt-eng-cli

Command-line interface for the TPT engineering ecosystem.

## Commands

| Command | Description |
| --- | --- |
| `validate <file>` | Validate an engineering input file (`json`, `csv`, `stl`, `obj`). |
| `report --out <path>` | Generate a calculation report. The output format is chosen by the extension (`.md`, `.html`, `.json`). Use `--chart <png>` to also emit a results chart. |
| `units convert <value> <from> <to>` | Convert between units (length, mass, force, pressure, temperature). |
| `units list` | List supported units. |
| `materials [name]` | Inspect a built-in material, or list available materials. |
| `sections inspect <shape> ...` | Inspect built-in cross-section properties (rectangle / circle / i-beam). |
| `calc beam <span> <load> --material <m> <section> [--report <path>] [--plot <png>]` | Simply supported beam under a uniformly distributed load. |

## Examples

```sh
# Validate a CSV file
tpt-eng-cli validate data.csv

# Convert 1 metre to millimetres
tpt-eng-cli units convert 1 m mm

# Inspect structural steel
tpt-eng-cli materials steel

# Properties of a 100 mm x 200 mm rectangle
tpt-eng-cli sections inspect rectangle 0.1 0.2

# Run a simply supported beam calc and emit a report + deflection diagram
tpt-eng-cli calc beam 5 10000 --material steel rectangle 0.1 0.2 \
    --report beam.md --plot beam.png
```

## Notes

- The `calc beam` example uses SI units (metres, newtons). Material properties and section
  second moments of area are looked up from the built-in tables.
- Plotting uses a self-contained bitmap font so that no external (copyleft) font dependencies
  are required.
