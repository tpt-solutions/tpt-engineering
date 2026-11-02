# tpt-eng-provenance

Tags for engineering results: where a published correlation comes from, the
input ranges it is valid over, and any warnings raised by a call.

A correlation evaluated outside its validity range still returns a number.
Without a tag, the caller cannot tell that the number is suspect. The
`*_tagged` functions in the property crates return a `Tagged<T>` instead. The
value is the same as the untagged function's result.

`no_std`, with no dependencies.

## Features

- **`Tagged<T>`** — a `value` with its `Provenance`.
- **`Provenance`** — the `Source`, up to `MAX_VALIDITY` input `Validity` ranges,
  and `Warnings`.
- **`Source`** — `Standard`, `Literature`, `Manufacturer` (each with a
  reference label), or `Calculated`.
- **`Warnings`** — `OUT_OF_RANGE`, `EXTRAPOLATED`, and `APPROXIMATE_MODEL`,
  combined as flags.
- **`out_of_range` / `range_warning`** — check an input against its validity
  range.

## Installation

```toml
[dependencies]
tpt-eng-provenance = "0.1"
```

## Quick start

```rust
use tpt_eng_provenance::{Source, Validity, Warnings, out_of_range};

const TEMPERATURE: Validity = Validity {
    input: "temperature",
    unit: "K",
    min: 173.15,
    max: 473.15,
};

assert!(!out_of_range(&TEMPERATURE, 300.0));
assert!(out_of_range(&TEMPERATURE, 600.0));
assert_eq!(Source::Standard("ASHRAE Fundamentals").label(), "ASHRAE Fundamentals");
assert!(Warnings::NONE.is_empty());
```

## Related crates

- [tpt-eng-props-water](../tpt-eng-props-water/) — `*_tagged` IAPWS-IF97 functions.
- [tpt-eng-props-air](../tpt-eng-props-air/) — `*_tagged` ASHRAE moist-air functions.
- [tpt-eng-props-mixture](../tpt-eng-props-mixture/) — `*_tagged` Peng–Robinson functions.

## Status

Initial `0.1.0` release.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) OR [Apache-2.0](../../LICENSE-APACHE).
