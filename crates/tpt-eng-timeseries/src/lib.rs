//! # tpt-eng-timeseries
//!
//! Umbrella crate re-exporting the `tpt-eng-timeseries-*` crates:
//!
//! * [`tpt_eng_timeseries_core`] — core time-series types ([`Series`](core::Series),
//!   [`Sample`](core::Sample), [`Timestamp`](tpt_eng_timeseries_core::Timestamp)).
//! * [`tpt_eng_timeseries_align`] — irregular multi-rate stream alignment.
//! * [`tpt_eng_timeseries_gap`] — staleness/gap detection and interpolation.
//!
//! ## Example
//!
//! ```
//! use tpt_eng_timeseries::core::{Sample, Series, Timestamp};
//!
//! // The umbrella re-exports the core, align and gap sub-crates.
//! let series = Series::from_samples([Sample::new(Timestamp::from_seconds(1.0), 3.0)]);
//! assert!(!series.is_empty());
//! ```

pub use tpt_eng_timeseries_align as align;
pub use tpt_eng_timeseries_core as core;
pub use tpt_eng_timeseries_gap as gap;
