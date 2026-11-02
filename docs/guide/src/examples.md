# Worked examples

Each scenario writes a Markdown report and an SVG chart to `target/examples/`.

Three runnable scenarios live in `tpt-eng-examples`. Each composes several
crates the way a real project would.

```sh
for e in thermal_loop solar_pv mechanical_design distillation construction_schedule hvac_load pcb_thermal; do
  cargo run -p tpt-eng-examples --example "$e"
done
```

- **thermal_loop**: a hydraulic topology, a fuel-sized burner, a PID controller
  on conditioned telemetry, and a beam check on the pipe rack.
- **solar_pv**: a single-diode maximum-power-point sweep, scaled to an array and
  checked against a three-phase grid tie.
- **mechanical_design**: a cross-section and material, a tolerance stack-up, a
  GD&T check, and a Markdown calculation report.
- **distillation**: Fenske, Underwood, Gilliland and McCabe–Thiele sizing for a
  saturated-liquid feed, with a reboiler effectiveness check.
- **construction_schedule**: critical-path schedule for a small frame project,
  PERT for the uncertain excavation, crew smoothing, and earned value.
- **hvac_load**: wall U-value from its layers, envelope conductance, design
  heating and cooling loads with infiltration, and seasonal heating energy.
- **pcb_thermal**: a 2.9 mm microstrip, a 2 oz power-trace ampacity and resistance,
  and a junction temperature on a heat sink.
