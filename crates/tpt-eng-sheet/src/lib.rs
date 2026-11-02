//! # tpt-eng-sheet
//!
//! Declarative calculation sheets. A sheet is a small TOML or JSON file that
//! names one calculation and gives its inputs. The unit is part of each field
//! name (`pressure_kpa`, `span_m`), so a sheet cannot silently mix units.
//! [`run`] executes the sheet and returns a [`Report`].
//!
//! The format is described by a JSON Schema, generated from [`Sheet`] by
//! [`json_schema`] and committed as `schema/calc-sheet.schema.json`. A test
//! fails if the committed file falls out of date.
//!
//! ## Example
//!
//! ```
//! use tpt_eng_sheet::{Sheet, run};
//!
//! let sheet = Sheet::from_toml(
//!     r#"
//! title = "Water at 300 K and 3 MPa"
//!
//! [calc]
//! kind = "water-state"
//! temperature_c = 26.85
//! pressure_kpa = 3000.0
//! "#,
//! )
//! .unwrap();
//! let report = run(&sheet).unwrap();
//! assert!(tpt_eng_report::to_markdown(&report).contains("Enthalpy"));
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tpt_eng_report::Report;

mod calc;

/// A calculation sheet: a title, an optional author, and one calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Sheet {
    /// Title of the generated report.
    pub title: String,
    /// Author shown in the report header.
    #[serde(default)]
    pub author: Option<String>,
    /// The calculation to run.
    pub calc: Calc,
}

/// The calculations a sheet can run. Selected by the `kind` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Calc {
    /// Water/steam state from IAPWS-IF97 at one temperature and pressure.
    ///
    /// Inputs outside the IF97 range are still computed, and the report flags them.
    WaterState {
        /// Temperature in degrees Celsius.
        temperature_c: f64,
        /// Pressure in kilopascals.
        pressure_kpa: f64,
    },
    /// Simply supported rectangular beam under a uniform load over its span.
    BeamUdl {
        /// Span in metres.
        span_m: f64,
        /// Uniform load in newtons per metre (downward).
        load_n_per_m: f64,
        /// Section width in metres.
        width_m: f64,
        /// Section depth (height) in metres. Bending is about the x axis.
        height_m: f64,
        /// Allowable bending stress in megapascals. When given, the report
        /// includes a utilisation check against 1.0.
        #[serde(default)]
        allowable_stress_mpa: Option<f64>,
    },
    /// Closed-loop step response of a PID controller on a first-order plant.
    PidStep {
        /// Proportional gain.
        kp: f64,
        /// Integral gain.
        ki: f64,
        /// Derivative gain.
        kd: f64,
        /// Target value of the measurement.
        setpoint: f64,
        /// Plant time constant in seconds.
        tau_s: f64,
        /// Controller sample period in seconds. Must be smaller than `tau_s`.
        dt_s: f64,
        /// Simulated duration in seconds.
        duration_s: f64,
        /// Symmetric limit on the controller output. Omit for no limit.
        #[serde(default)]
        output_limit: Option<f64>,
    },
}

/// Errors from parsing or running a sheet.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SheetError {
    /// The file is not valid TOML or JSON for this format.
    #[error("could not parse sheet: {0}")]
    Parse(String),
    /// The sheet parsed, but an input is missing a valid value.
    #[error("invalid input: {0}")]
    Invalid(String),
}

impl Sheet {
    /// Parse a sheet from TOML text.
    ///
    /// # Errors
    ///
    /// Returns [`SheetError::Parse`] for malformed TOML, unknown fields, unknown
    /// `kind` values, or missing required fields.
    pub fn from_toml(text: &str) -> Result<Self, SheetError> {
        toml::from_str(text).map_err(|e| SheetError::Parse(e.to_string()))
    }

    /// Parse a sheet from JSON text.
    ///
    /// # Errors
    ///
    /// Returns [`SheetError::Parse`] for the same reasons as [`Sheet::from_toml`].
    pub fn from_json(text: &str) -> Result<Self, SheetError> {
        serde_json::from_str(text).map_err(|e| SheetError::Parse(e.to_string()))
    }
}

/// Run a sheet and return its report.
///
/// # Errors
///
/// Returns [`SheetError::Invalid`] for non-finite or out-of-domain inputs, and
/// for water states the IF97 correlations cannot evaluate.
pub fn run(sheet: &Sheet) -> Result<Report, SheetError> {
    let report = match &sheet.calc {
        Calc::WaterState {
            temperature_c,
            pressure_kpa,
        } => calc::water_state(&sheet.title, *temperature_c, *pressure_kpa)?,
        Calc::BeamUdl {
            span_m,
            load_n_per_m,
            width_m,
            height_m,
            allowable_stress_mpa,
        } => calc::beam_udl(
            &sheet.title,
            calc::BeamInputs {
                span_m: *span_m,
                load_n_per_m: *load_n_per_m,
                width_m: *width_m,
                height_m: *height_m,
                allowable_stress_mpa: *allowable_stress_mpa,
            },
        )?,
        Calc::PidStep {
            kp,
            ki,
            kd,
            setpoint,
            tau_s,
            dt_s,
            duration_s,
            output_limit,
        } => calc::pid_step(
            &sheet.title,
            calc::PidInputs {
                kp: *kp,
                ki: *ki,
                kd: *kd,
                setpoint: *setpoint,
                tau_s: *tau_s,
                dt_s: *dt_s,
                duration_s: *duration_s,
                output_limit: *output_limit,
            },
        )?,
    };
    Ok(match &sheet.author {
        Some(author) => report.with_author(author.clone()),
        None => report,
    })
}

/// The JSON Schema for [`Sheet`], pretty-printed with a trailing newline.
///
/// # Errors
///
/// Returns a [`serde_json::Error`] if the schema cannot be serialised.
pub fn json_schema() -> Result<String, serde_json::Error> {
    let mut text = serde_json::to_string_pretty(&schemars::schema_for!(Sheet))?;
    text.push('\n');
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tpt_eng_report::Section;

    const WATER: &str = include_str!("../examples/water-state.toml");
    const BEAM: &str = include_str!("../examples/beam-udl.toml");
    const PID: &str = include_str!("../examples/pid-step.toml");

    #[test]
    fn examples_parse_and_run() {
        for text in [WATER, BEAM, PID] {
            let sheet = Sheet::from_toml(text).unwrap();
            run(&sheet).unwrap();
        }
    }

    #[test]
    fn toml_and_json_describe_the_same_sheet() {
        let from_toml = Sheet::from_toml(BEAM).unwrap();
        let json = serde_json::to_string(&from_toml).unwrap();
        assert_eq!(Sheet::from_json(&json).unwrap(), from_toml);
    }

    #[test]
    fn misspelt_field_is_rejected_not_ignored() {
        let text = WATER.replace("pressure_kpa", "pressure_kpaa");
        assert!(matches!(Sheet::from_toml(&text), Err(SheetError::Parse(_))));
    }

    #[test]
    fn unknown_kind_is_rejected() {
        let text = WATER.replace("water-state", "steam-table");
        assert!(matches!(Sheet::from_toml(&text), Err(SheetError::Parse(_))));
    }

    #[test]
    fn non_positive_span_is_invalid() {
        let text = BEAM.replace("span_m = 4.0", "span_m = -4.0");
        let sheet = Sheet::from_toml(&text).unwrap();
        assert!(matches!(run(&sheet), Err(SheetError::Invalid(_))));
    }

    #[test]
    fn beam_results_match_closed_form() {
        // w = 2 kN/m over 4 m: R = wL/2 = 4 kN, M_max = wL²/8 = 4 kN·m.
        // Rectangle 0.1 m × 0.2 m: Z = bh²/6, so σ = 4000 / (0.1·0.04/6) Pa = 6 MPa.
        let sheet = Sheet::from_toml(
            r#"
title = "beam"
[calc]
kind = "beam-udl"
span_m = 4.0
load_n_per_m = 2000.0
width_m = 0.1
height_m = 0.2
allowable_stress_mpa = 12.0
"#,
        )
        .unwrap();
        let report = run(&sheet).unwrap();
        let value = |name: &str| {
            report
                .sections
                .iter()
                .find_map(|s| match s {
                    Section::Results(entries) => entries.iter().find(|r| r.name == name),
                    _ => None,
                })
                .map(|r| r.value)
                .unwrap()
        };
        assert!((value("Reaction A") - 4000.0).abs() < 1e-6);
        assert!((value("Max bending moment") - 4000.0).abs() < 1e-6);
        assert!((value("Max bending stress") - 6.0).abs() < 1e-9);
        assert!((value("Utilisation") - 0.5).abs() < 1e-9);
    }

    #[test]
    fn water_outside_range_is_flagged_in_the_report() {
        let sheet = Sheet::from_toml(
            r#"
title = "hot steam"
[calc]
kind = "water-state"
temperature_c = 827.0
pressure_kpa = 100.0
"#,
        )
        .unwrap();
        let md = tpt_eng_report::to_markdown(&run(&sheet).unwrap());
        assert!(md.contains("Warning: at least one input lies outside"));
    }

    #[test]
    fn water_below_zero_celsius_is_invalid() {
        let text = WATER.replace("temperature_c = 26.85", "temperature_c = -50.0");
        let sheet = Sheet::from_toml(&text).unwrap();
        assert!(matches!(run(&sheet), Err(SheetError::Invalid(_))));
    }

    #[test]
    fn committed_schema_is_up_to_date() {
        let committed = include_str!("../schema/calc-sheet.schema.json");
        assert_eq!(
            json_schema().unwrap(),
            committed,
            "regenerate with `cargo run -p tpt-eng-cli -- sheet schema --out crates/tpt-eng-sheet/schema/calc-sheet.schema.json`"
        );
    }
}
