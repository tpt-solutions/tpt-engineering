# Worked examples

Three runnable scenarios live in `tpt-eng-examples`. Each composes several
crates the way a real project would.

```sh
cargo run -p tpt-eng-examples --example thermal_loop
cargo run -p tpt-eng-examples --example solar_pv
cargo run -p tpt-eng-examples --example mechanical_design
```

- **thermal_loop**: a hydraulic topology, a fuel-sized burner, a PID controller
  on conditioned telemetry, and a beam check on the pipe rack.
- **solar_pv**: a single-diode maximum-power-point sweep, scaled to an array and
  checked against a three-phase grid tie.
- **mechanical_design**: a cross-section and material, a tolerance stack-up, a
  GD&T check, and a Markdown calculation report.
