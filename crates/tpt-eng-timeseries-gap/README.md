# tpt-eng-timeseries-gap

Staleness and gap handling for sensor streams that drop out or freeze:
[`detect_gaps`], [`is_stale`], and interpolation/fill strategies
(hold / linear / zero).

## Example

```rust
use tpt_eng_timeseries_core::{Sample, Series, Timestamp};
use tpt_eng_timeseries_gap::{detect_gaps, is_stale};

let s = Series::from_samples([
    Sample::new(Timestamp::from_seconds(0.0), 1.0),
    Sample::new(Timestamp::from_seconds(1.0), 2.0),
    Sample::new(Timestamp::from_seconds(9.0), 3.0), // 8 s gap (> 2 s max)
]);
assert_eq!(detect_gaps(&s, 2.0).len(), 1);
assert!(!is_stale(&s, 9.5, 2.0));
assert!(is_stale(&s, 20.0, 2.0));
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
