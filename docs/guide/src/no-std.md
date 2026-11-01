# no_std and embedded

These crates build without `std`, and CI checks them on `thumbv6m-none-eabi`
and `wasm32-unknown-unknown`:

- `tpt-eng-props-water`, `tpt-eng-props-air`, `tpt-eng-props-fuels`
- `tpt-eng-props-mixture`
- `tpt-eng-props` (umbrella)

Build one without default features:

```sh
cargo build -p tpt-eng-props-water --no-default-features --target thumbv6m-none-eabi
```

The check is `cargo xtask no-std-matrix`. The crates use `libm` for
transcendental functions, so the results match with and without `std`.
