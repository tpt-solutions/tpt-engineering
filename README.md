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

## Building

```sh
cargo build --workspace
cargo test --workspace
```

The `tpt-eng-props-*` family is `no_std`; it builds for bare-metal targets
with the default features disabled.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)
at your option.

Copyright (c) 2026 TPT Solutions.
