//! WebAssembly bindings for the browser demos in `www/`.
//!
//! Three demos share this crate:
//!
//! * **Steam tables.** [`steam_state`] returns the IF97 state at a temperature
//!   and pressure, with an `in_range` flag from the provenance tags.
//! * **Section properties.** [`rectangle_section`], [`circle_section`] and
//!   [`i_section`] return the main section properties in SI units.
//! * **PID tuner.** [`pid_step_response`] simulates a PID loop on a first-order
//!   plant and returns the measured response.
//!
//! Inputs and outputs are SI, as elsewhere in the workspace. Invalid inputs
//! raise a JavaScript `Error`.

use tpt_eng_controls::{Pid, PidGains};
use tpt_eng_props_water::{Error as WaterError, Region, state_tagged};
use tpt_eng_sections::{Circle, ISection, Rectangle, Section, SectionProperties};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::mass_density::kilogram_per_cubic_meter;
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::specific_heat_capacity::kilojoule_per_kilogram_kelvin;
use tpt_math_units::uom::si::specific_volume::cubic_meter_per_kilogram;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;
use wasm_bindgen::prelude::*;

/// Upper bound on simulation samples, so one call cannot exhaust browser memory.
const MAX_SAMPLES: f64 = 1_000_000.0;

fn require_finite_positive(name: &str, value: f64) -> Result<(), String> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(format!("{name} must be a finite number greater than zero"))
    }
}

// ---------------------------------------------------------------------------
// Steam tables
// ---------------------------------------------------------------------------

/// Water/steam state at one temperature and pressure.
#[wasm_bindgen]
pub struct SteamState {
    region: &'static str,
    specific_volume_m3_per_kg: f64,
    density_kg_per_m3: f64,
    enthalpy_kj_per_kg: f64,
    entropy_kj_per_kg_k: f64,
    isobaric_heat_capacity_kj_per_kg_k: f64,
    in_range: bool,
}

#[wasm_bindgen]
impl SteamState {
    /// `"liquid"`, `"vapour"` or `"saturation"`.
    #[wasm_bindgen(getter)]
    pub fn region(&self) -> String {
        self.region.to_owned()
    }

    /// Specific volume, m³/kg.
    #[wasm_bindgen(getter)]
    pub fn specific_volume_m3_per_kg(&self) -> f64 {
        self.specific_volume_m3_per_kg
    }

    /// Density, kg/m³.
    #[wasm_bindgen(getter)]
    pub fn density_kg_per_m3(&self) -> f64 {
        self.density_kg_per_m3
    }

    /// Specific enthalpy, kJ/kg.
    #[wasm_bindgen(getter)]
    pub fn enthalpy_kj_per_kg(&self) -> f64 {
        self.enthalpy_kj_per_kg
    }

    /// Specific entropy, kJ/(kg·K).
    #[wasm_bindgen(getter)]
    pub fn entropy_kj_per_kg_k(&self) -> f64 {
        self.entropy_kj_per_kg_k
    }

    /// Isobaric heat capacity, kJ/(kg·K).
    #[wasm_bindgen(getter)]
    pub fn isobaric_heat_capacity_kj_per_kg_k(&self) -> f64 {
        self.isobaric_heat_capacity_kj_per_kg_k
    }

    /// `false` when the inputs lie outside the documented IF97 range.
    #[wasm_bindgen(getter)]
    pub fn in_range(&self) -> bool {
        self.in_range
    }
}

fn water_error_message(err: WaterError) -> String {
    match err {
        WaterError::TemperatureOutOfRange => "temperature must be at least 273.15 K".to_owned(),
        WaterError::NegativePressure => "pressure must be non-negative".to_owned(),
        WaterError::Region3Unsupported => {
            "this state is in IF97 Region 3 (near-critical), which is not implemented".to_owned()
        }
    }
}

fn steam_state_core(temperature_k: f64, pressure_pa: f64) -> Result<SteamState, String> {
    if !(temperature_k.is_finite() && pressure_pa.is_finite()) {
        return Err("temperature and pressure must be finite numbers".to_owned());
    }
    let tagged = state_tagged(
        ThermodynamicTemperature::new::<kelvin>(temperature_k),
        Pressure::new::<pascal>(pressure_pa),
    )
    .map_err(water_error_message)?;
    let s = tagged.value;
    Ok(SteamState {
        region: match s.region {
            Region::One => "liquid",
            Region::Two => "vapour",
            Region::Saturation => "saturation",
        },
        specific_volume_m3_per_kg: s.specific_volume.get::<cubic_meter_per_kilogram>(),
        density_kg_per_m3: s.density.get::<kilogram_per_cubic_meter>(),
        enthalpy_kj_per_kg: s.enthalpy,
        entropy_kj_per_kg_k: s.entropy.get::<kilojoule_per_kilogram_kelvin>(),
        isobaric_heat_capacity_kj_per_kg_k: s
            .isobaric_heat_capacity
            .get::<kilojoule_per_kilogram_kelvin>(),
        in_range: tagged.provenance.warnings.is_empty(),
    })
}

/// Water/steam state at `temperature_k` (K) and `pressure_pa` (Pa).
///
/// # Errors
///
/// Returns a JavaScript error for non-finite inputs, temperatures below
/// 273.15 K, negative pressures, and states in IF97 Region 3.
#[wasm_bindgen]
pub fn steam_state(temperature_k: f64, pressure_pa: f64) -> Result<SteamState, JsError> {
    steam_state_core(temperature_k, pressure_pa).map_err(|e| JsError::new(&e))
}

// ---------------------------------------------------------------------------
// Section properties
// ---------------------------------------------------------------------------

/// Section properties in SI units: area in m², second moments in m⁴, section
/// moduli in m³, and the torsional constant in m⁴.
#[wasm_bindgen]
pub struct SectionReport {
    props: SectionProperties,
}

#[wasm_bindgen]
impl SectionReport {
    /// Cross-sectional area, m².
    #[wasm_bindgen(getter)]
    pub fn area(&self) -> f64 {
        self.props.area
    }

    /// Second moment about the centroidal x axis, m⁴.
    #[wasm_bindgen(getter)]
    pub fn second_moment_x(&self) -> f64 {
        self.props.second_moment_x
    }

    /// Second moment about the centroidal y axis, m⁴.
    #[wasm_bindgen(getter)]
    pub fn second_moment_y(&self) -> f64 {
        self.props.second_moment_y
    }

    /// Elastic section modulus about x, m³.
    #[wasm_bindgen(getter)]
    pub fn section_modulus_x(&self) -> f64 {
        self.props.section_modulus_x
    }

    /// Elastic section modulus about y, m³.
    #[wasm_bindgen(getter)]
    pub fn section_modulus_y(&self) -> f64 {
        self.props.section_modulus_y
    }

    /// Plastic modulus about x, m³.
    #[wasm_bindgen(getter)]
    pub fn plastic_modulus_x(&self) -> f64 {
        self.props.plastic_modulus_x
    }

    /// Torsional constant, m⁴.
    #[wasm_bindgen(getter)]
    pub fn torsional_constant(&self) -> f64 {
        self.props.torsional_constant
    }
}

fn report<S: Section>(shape: &S) -> SectionReport {
    SectionReport {
        props: shape.properties(),
    }
}

/// A solid rectangle, `width_m` by `height_m`.
///
/// # Errors
///
/// Returns a JavaScript error if either dimension is not a positive finite number.
#[wasm_bindgen]
pub fn rectangle_section(width_m: f64, height_m: f64) -> Result<SectionReport, JsError> {
    require_finite_positive("width_m", width_m)
        .and_then(|()| require_finite_positive("height_m", height_m))
        .map(|()| report(&Rectangle::new(width_m, height_m)))
        .map_err(|e| JsError::new(&e))
}

/// A solid circle of `diameter_m`.
///
/// # Errors
///
/// Returns a JavaScript error if the diameter is not a positive finite number.
#[wasm_bindgen]
pub fn circle_section(diameter_m: f64) -> Result<SectionReport, JsError> {
    require_finite_positive("diameter_m", diameter_m)
        .map(|()| report(&Circle::new(diameter_m)))
        .map_err(|e| JsError::new(&e))
}

fn i_section_core(
    depth_m: f64,
    flange_width_m: f64,
    flange_thickness_m: f64,
    web_thickness_m: f64,
) -> Result<SectionReport, String> {
    require_finite_positive("depth_m", depth_m)?;
    require_finite_positive("flange_width_m", flange_width_m)?;
    require_finite_positive("flange_thickness_m", flange_thickness_m)?;
    require_finite_positive("web_thickness_m", web_thickness_m)?;
    if 2.0 * flange_thickness_m >= depth_m {
        return Err("two flange thicknesses must be less than the depth".to_owned());
    }
    if web_thickness_m >= flange_width_m {
        return Err("web thickness must be less than the flange width".to_owned());
    }
    Ok(report(&ISection::new(
        depth_m,
        flange_width_m,
        flange_thickness_m,
        web_thickness_m,
    )))
}

/// An I-section (doubly symmetric), with the dimensions in metres.
///
/// # Errors
///
/// Returns a JavaScript error for non-positive dimensions, flanges thicker than
/// half the depth, or a web at least as wide as the flange.
#[wasm_bindgen]
pub fn i_section(
    depth_m: f64,
    flange_width_m: f64,
    flange_thickness_m: f64,
    web_thickness_m: f64,
) -> Result<SectionReport, JsError> {
    i_section_core(depth_m, flange_width_m, flange_thickness_m, web_thickness_m)
        .map_err(|e| JsError::new(&e))
}

// ---------------------------------------------------------------------------
// PID tuner
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn pid_step_core(
    kp: f64,
    ki: f64,
    kd: f64,
    setpoint: f64,
    tau_s: f64,
    dt_s: f64,
    duration_s: f64,
    output_limit: Option<f64>,
) -> Result<Vec<f64>, String> {
    for (name, v) in [("kp", kp), ("ki", ki), ("kd", kd), ("setpoint", setpoint)] {
        if !v.is_finite() {
            return Err(format!("{name} must be a finite number"));
        }
    }
    require_finite_positive("tau_s", tau_s)?;
    require_finite_positive("dt_s", dt_s)?;
    require_finite_positive("duration_s", duration_s)?;
    if dt_s >= tau_s {
        // Explicit Euler on y' = (u - y)/tau is unstable when dt >= tau.
        return Err("dt_s must be smaller than tau_s for a stable simulation".to_owned());
    }
    let steps = (duration_s / dt_s).round();
    if steps > MAX_SAMPLES {
        return Err(format!("duration_s / dt_s must not exceed {MAX_SAMPLES}"));
    }
    let steps = steps as usize;

    let mut pid = Pid::new(PidGains::new(kp, ki, kd));
    if let Some(limit) = output_limit {
        if !(limit.is_finite() && limit > 0.0) {
            return Err("output_limit must be a finite number greater than zero".to_owned());
        }
        pid = pid.with_output_limit(limit);
    }
    pid.set_setpoint(setpoint);

    let mut y = 0.0;
    let mut response = Vec::with_capacity(steps + 1);
    response.push(y);
    for _ in 0..steps {
        let u = pid.update(y, dt_s);
        y += dt_s * (u - y) / tau_s;
        response.push(y);
    }
    Ok(response)
}

/// Simulate a PID loop on a unit-gain first-order plant with time constant
/// `tau_s`. Returns the measured output at `0, dt, 2·dt, …` up to
/// `duration_s`, as a `Float64Array`.
///
/// `output_limit` saturates the controller output to `±output_limit`. Pass
/// `undefined` for no limit.
///
/// # Errors
///
/// Returns a JavaScript error for non-finite gains, non-positive times, a step
/// `dt_s` not smaller than `tau_s`, more than one million samples, or a
/// non-positive output limit.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn pid_step_response(
    kp: f64,
    ki: f64,
    kd: f64,
    setpoint: f64,
    tau_s: f64,
    dt_s: f64,
    duration_s: f64,
    output_limit: Option<f64>,
) -> Result<Vec<f64>, JsError> {
    pid_step_core(kp, ki, kd, setpoint, tau_s, dt_s, duration_s, output_limit)
        .map_err(|e| JsError::new(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_state_in_range_matches_iapws_reference() {
        let s = steam_state_core(300.0, 3.0e6).unwrap();
        assert_eq!(s.region, "liquid");
        assert!(s.in_range);
        assert!((s.enthalpy_kj_per_kg - 115.331).abs() < 1e-2);
    }

    #[test]
    fn steam_state_above_range_is_flagged_not_rejected() {
        let s = steam_state_core(1100.0, 1.0e5).unwrap();
        assert!(!s.in_range);
    }

    #[test]
    fn steam_state_hard_errors_are_reported() {
        let err = steam_state_core(200.0, 1.0e5).err().unwrap();
        assert!(err.contains("273.15"));
        assert!(steam_state_core(f64::NAN, 1.0e5).is_err());
    }

    #[test]
    fn rectangle_second_moment_matches_bh3_over_12() {
        let r = report(&Rectangle::new(0.1, 0.2));
        assert!((r.props.area - 0.02).abs() < 1e-12);
        assert!((r.props.second_moment_x - 0.1 * 0.2_f64.powi(3) / 12.0).abs() < 1e-12);
    }

    #[test]
    fn i_section_rejects_inconsistent_dimensions() {
        assert!(i_section_core(0.2, 0.1, 0.01, 0.2).is_err());
        assert!(i_section_core(0.2, 0.1, 0.12, 0.005).is_err());
        assert!(i_section_core(0.2, 0.1, 0.01, 0.006).is_ok());
    }

    #[test]
    fn pid_with_integral_action_reaches_setpoint() {
        let y = pid_step_core(2.0, 1.0, 0.0, 1.0, 1.0, 0.01, 20.0, None).unwrap();
        assert_eq!(y.len(), 2001);
        assert!((y[y.len() - 1] - 1.0).abs() < 0.02);
    }

    #[test]
    fn pid_rejects_unstable_step_and_huge_runs() {
        assert!(pid_step_core(1.0, 0.0, 0.0, 1.0, 0.5, 0.5, 10.0, None).is_err());
        assert!(pid_step_core(1.0, 0.0, 0.0, 1.0, 1.0, 1e-6, 1e6, None).is_err());
        assert!(pid_step_core(1.0, 0.0, 0.0, 1.0, 1.0, 0.01, 10.0, Some(-1.0)).is_err());
    }
}
