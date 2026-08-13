//! # tpt-eng-examples
//!
//! End-to-end scenarios that compose the `tpt-eng-*` primitives the way a
//! real physical-systems vertical would: build an infrastructure topology,
//! derive its network matrix, regulate a plant with a PID controller sized
//! from a fuel's heating value, condition the gappy telemetry that feeds the
//! controller, and run a structural check on the supporting structure.
//!
//! Each scenario lives in its own module as a self-contained function plus a
//! unit test, so the crate doubles as executable documentation for "how do I
//! use these crates together?".
//!
//! ## Example
//!
//! ```
//! use tpt_eng_examples::thermal_loop::run_thermal_loop;
//!
//! let report = run_thermal_loop();
//! // The PID must converge the supply temperature onto the setpoint.
//! assert!((report.supply_temperature - report.setpoint).abs() < 0.5);
//! // Telemetry conditioning must have repaired the dropout.
//! assert!(report.max_gap_seconds < 2.0);
//! ```

pub mod thermal_loop;

pub use thermal_loop::ThermalLoopReport;
