//! # tpt-eng-examples
//!
//! End-to-end scenarios that compose the `tpt-eng-*` primitives the way a
//! real physical-systems vertical would:
//!
//! 1. **Thermal loop** — build an infrastructure topology, derive its network
//!    matrix, regulate a plant with a PID controller sized from a fuel's heating
//!    value, condition the gappy telemetry that feeds the controller, and run a
//!    structural check on the supporting structure ([`thermal_loop`]).
//! 2. **Mechanical design** — define a cross-section and material, roll up a
//!    dimensional tolerance stack-up, check measured points against a GD&T zone,
//!    and render the result as a calculation report ([`mechanical_design`]).
//! 3. **Solar PV sizing** — sweep a single-diode PV cell to its maximum-power
//!    point, scale to an array, and size the three-phase grid-tie connection
//!    ([`solar_pv`]).
//!
//! Three further scenarios cover the remaining workflows, each with a runnable
//! binary under `examples/`:
//!
//! * **Distillation** ([`distillation`]) — Fenske, Underwood, Gilliland and
//!   McCabe–Thiele sizing, with a reboiler ε-NTU check.
//! * **Construction schedule** ([`construction_schedule`]) — CPM, PERT, crew
//!   leveling and earned value.
//! * **Building HVAC** ([`hvac_load`]) — envelope conductance, design loads and
//!   seasonal heating energy.
//! * **PCB thermal** ([`pcb_thermal`]) — microstrip impedance, trace ampacity and
//!   a junction temperature budget on a heat sink.
//!
//! Each scenario lives in its own module as a self-contained function plus a
//! unit test, so the crate doubles as executable documentation for "how do I
//! use these crates together?".
//!
//! ## Example
//!
//! ```
//! use tpt_eng_examples::solar_pv::run_solar_pv_sizing;
//!
//! let report = run_solar_pv_sizing();
//! // The array DC power must be recovered by the three-phase tie.
//! let p_ac = 3.0f64.sqrt() * report.grid_v_ll * report.line_current_a * report.power_factor;
//! assert!((p_ac - report.array_dc_w).abs() < 1e-6);
//! ```

/// Directory for scenario outputs: `target/examples/` at the workspace root.
pub fn output_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/examples")
}

/// Write `<name>.md` (the calculation report) and `<name>.svg` (a bar chart of
/// `bars`) into [`output_dir`], and return both paths.
///
/// # Errors
///
/// Returns the I/O or plotting error as text.
pub fn emit(
    name: &str,
    report: &tpt_eng_report::Report,
    chart_title: &str,
    bars: Vec<(String, f64)>,
) -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
    let dir = output_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let md = dir.join(format!("{name}.md"));
    std::fs::write(&md, tpt_eng_report::to_markdown(report)).map_err(|e| e.to_string())?;
    let svg = dir.join(format!("{name}.svg"));
    tpt_eng_plot::chart::plot_result_chart_svg(chart_title, bars, &svg)
        .map_err(|e| e.to_string())?;
    Ok((md, svg))
}

pub mod construction_schedule;
pub mod distillation;
pub mod hvac_load;
pub mod mechanical_design;
pub mod pcb_thermal;
pub mod solar_pv;
pub mod thermal_loop;

pub use construction_schedule::{ScheduleReport, run_construction_schedule};
pub use distillation::{DistillationReport, run_distillation};
pub use hvac_load::{HvacReport, run_hvac_load};
pub use mechanical_design::{MechanicalDesignReport, design_report, run_mechanical_design};
pub use pcb_thermal::{PcbThermalReport, run_pcb_thermal};
pub use solar_pv::{SolarPvReport, run_solar_pv_sizing};
pub use thermal_loop::ThermalLoopReport;
