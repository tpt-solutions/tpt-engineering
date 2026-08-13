# tpt-engineering

Applied engineering primitives for the [TPT Solutions](https://github.com/tpt-solutions)
physical-systems verticals.

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

## Scope

A set of small, single-purpose `tpt-eng-*` crates providing reusable
engineering maths — fluid/gas properties, time-series conditioning,
infrastructure topology, controls, and structural primitives — drawn out of
the product verticals as foundation-pillar building blocks rather than
product-shaped packages.

All crates are new implementations built from scratch (the spec's claimed
consolidation source `tpt-rust2` does not exist on this machine, and the
external vertical repos were not available for audit from this build
environment).

## Crate inventory

| Crate | Domain | no_std | Description |
|-------|--------|--------|-------------|
| `tpt-eng-props-water` | fluid-properties | yes | IAPWS-IF97 water/steam property tables |
| `tpt-eng-props-air` | fluid-properties | yes | ASHRAE psychrometrics / moist-air |
| `tpt-eng-props-fuels` | fluid-properties | yes | Fuel heating values, density, combustion |
| `tpt-eng-props` | fluid-properties | yes | Umbrella: re-exports water + air + fuels |
| `tpt-eng-timeseries-core` | timeseries | | Core time-series types |
| `tpt-eng-timeseries-align` | timeseries | | Irregular multi-rate stream alignment |
| `tpt-eng-timeseries-gap` | timeseries | | Staleness/gap detection + interpolation |
| `tpt-eng-timeseries` | timeseries | | Umbrella: re-exports core + align + gap |
| `tpt-eng-geo-asset` | geo | | Geographic asset registry |
| `tpt-eng-geo-topology` | geo | | Directional infrastructure graphs |
| `tpt-eng-network-matrix` | geo | | Incidence/admittance matrix generation |
| `tpt-eng-controls` | controls | | PID / state-space / transfer-function |
| `tpt-eng-structural` | structural | | Loads, beam/frame analysis, code checks |
| `tpt-eng-examples` | integration | | Cross-crate scenario composing all of the above |

## Building

```sh
cargo build --workspace
cargo test --workspace
```

The `tpt-eng-props-*` family is `no_std`; it builds for bare-metal targets
with the default features disabled.

## Quickstart

Add the crates you need (they re-export `tpt-math`'s `uom`-typed units):

```sh
cargo add tpt-eng-props tpt-eng-controls tpt-eng-structural
```

```rust
use tpt_eng_controls::{Pid, PidGains};

let mut pid = Pid::new(PidGains::new(2.0, 1.0, 0.0)).with_output_limit(1.0);
pid.set_setpoint(10.0);
let mut y = 0.0;
for _ in 0..1000 {
    let u = pid.update(y, 0.01);
    y += 0.01 * u; // first-order plant, τ = 1 s
}
assert!((y - 10.0).abs() < 0.5);
```

For a full, cross-crate worked example (topology → network matrix → controls →
time-series conditioning → structural check), see
[`tpt-eng-examples`](crates/tpt-eng-examples).

## Developer tooling

`cargo xtask` provides one-stop hygiene and scaffolding (`check`, `test`,
`doctest`, `doc`, `no-std-matrix`, `new-crate`). A root [`justfile`](justfile)
mirrors these for non-Cargo users.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)
at your option.

Copyright (c) 2026 TPT Solutions.
