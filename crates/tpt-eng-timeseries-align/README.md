# tpt-eng-timeseries-align

Align irregular, multi-rate sensor streams (e.g. 1 Hz CAN bus vs 10 s Modbus
polls) onto a single deterministic time grid via clamped linear interpolation.

## Example

```rust
use tpt_eng_timeseries_core::{Sample, Series, Timestamp};
use tpt_eng_timeseries_align::align_to_grid;

let src = Series::from_samples([
    Sample::new(Timestamp::from_seconds(0.0), 0.0),
    Sample::new(Timestamp::from_seconds(2.0), 10.0),
]);
assert_eq!(align_to_grid(&src, &[1.0]), vec![5.0]);
```

Dual-licensed under MIT OR Apache-2.0. Copyright (c) 2026 TPT Solutions.
