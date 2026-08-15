//! # tpt-eng-examples
//!
//! End-to-end scenarios that compose the `tpt-eng-*` primitives the way a
//! real physical-systems vertical would: build an infrastructure topology,
//! derive its network matrix, regulate a plant with a PID controller sized
//! from a fuel's heating value, condition the gappy telemetry that feeds the
//!    controller, and run a structural check on the supporting structure.
//! 2. **Mechanical design** — define a cross-section and material, roll up a
//!    dimensional tolerance stack-up, check measured points against a GD&T zone,
//!    and render the result as a calculation report ([`mechanical_design`]).
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
//! // Telemetry conditioning must have repaired the dropout (a 5 s gap).
//! assert!(report.max_gap_seconds < 6.0);
//! ```

pub mod mechanical_design;
pub mod thermal_loop;

pub use mechanical_design::{MechanicalDesignReport, design_report, run_mechanical_design};
pub use thermal_loop::ThermalLoopReport;
