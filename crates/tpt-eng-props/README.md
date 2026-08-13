# tpt-eng-props

Umbrella crate re-exporting the `tpt-eng-props-*` fluid-property crates:
`tpt_eng_props_water` (IAPWS-IF97), `tpt_eng_props_air` (ASHRAE
psychrometrics), and `tpt_eng_props_fuels` (fuel combustion properties). `no_std`-capable.

```rust
pub use tpt_eng_props_air as air;
pub use tpt_eng_props_fuels as fuels;
pub use tpt_eng_props_water as water;
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
