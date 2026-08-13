# tpt-eng-timeseries-core

Core time-series types shared by the `tpt-eng-timeseries-*` family: a
[`Timestamp`], a [`Sample`], and an ordered [`Series`].

## Example

```rust
use tpt_eng_timeseries_core::{Sample, Series, Timestamp};

let mut s = Series::new();
s.push(Sample::new(Timestamp::from_seconds(1.0), 10.0));
s.push(Sample::new(Timestamp::from_seconds(3.0), 20.0));
assert_eq!(s.len(), 2);
assert!(s.is_sorted());
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
