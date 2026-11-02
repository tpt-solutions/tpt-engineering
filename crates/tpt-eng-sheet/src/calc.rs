//! The calculations a sheet can run. Each one records its inputs, the formula
//! for every step, its results with pass/fail, and its references, and lays
//! them out as a [`Report`].

use tpt_eng_props_water::{Error as WaterError, Region, state_tagged};
use tpt_eng_report::{Calculation, NamedValue, Report, ResultEntry, Step, ValidationStatus};
use tpt_eng_sections::{Rectangle, Section};
use tpt_eng_standards::limit_states::DemandCapacity;
use tpt_eng_structural::{Beam, Load};
use tpt_math_units::uom::si::f64::{Force, Length, Pressure, ThermodynamicTemperature, Torque};
use tpt_math_units::uom::si::force::newton;
use tpt_math_units::uom::si::length::meter;
use tpt_math_units::uom::si::mass_density::kilogram_per_cubic_meter;
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::specific_heat_capacity::kilojoule_per_kilogram_kelvin;
use tpt_math_units::uom::si::specific_volume::cubic_meter_per_kilogram;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;
use tpt_math_units::uom::si::torque::newton_meter;

use crate::SheetError;

/// Largest number of PID samples a sheet may request.
const MAX_PID_SAMPLES: f64 = 1_000_000.0;

/// Reference for the IF97 water and steam correlations.
const IF97_REFERENCE: &str = "IAPWS (2007), Release on the IAPWS Industrial Formulation 1997 \
     for the Thermodynamic Properties of Water and Steam";

/// Reference for the simply supported beam formulas.
const BEAM_REFERENCE: &str = "Young & Budynas, Roark's Formulas for Stress and Strain: \
     simply supported beam with a uniform load";

fn finite(name: &str, value: f64) -> Result<(), SheetError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SheetError::Invalid(format!(
            "{name} must be a finite number"
        )))
    }
}

fn positive(name: &str, value: f64) -> Result<(), SheetError> {
    finite(name, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(SheetError::Invalid(format!(
            "{name} must be greater than zero"
        )))
    }
}

fn water_error(err: WaterError) -> SheetError {
    SheetError::Invalid(match err {
        WaterError::TemperatureOutOfRange => "temperature must be at least 0 °C".to_owned(),
        WaterError::NegativePressure => "pressure must be non-negative".to_owned(),
        WaterError::Region3Unsupported => {
            "this state is in IF97 Region 3 (near-critical), which is not implemented".to_owned()
        }
    })
}

/// Water/steam state at `temperature_c` and `pressure_kpa`.
pub(crate) fn water_state(
    title: &str,
    temperature_c: f64,
    pressure_kpa: f64,
) -> Result<Report, SheetError> {
    finite("temperature_c", temperature_c)?;
    finite("pressure_kpa", pressure_kpa)?;
    let tagged = state_tagged(
        ThermodynamicTemperature::new::<kelvin>(temperature_c + 273.15),
        Pressure::new::<pascal>(pressure_kpa * 1000.0),
    )
    .map_err(water_error)?;
    let s = tagged.value;

    let region = match s.region {
        Region::One => "liquid",
        Region::Two => "vapour",
        Region::Saturation => "saturation",
    };
    let mut calc = Calculation::new(title)
        .input(NamedValue::new("Temperature", temperature_c).with_unit("°C"))
        .input(NamedValue::new("Pressure", pressure_kpa).with_unit("kPa"))
        .step(
            Step::new(
                "Absolute temperature",
                "T = t + 273.15",
                temperature_c + 273.15,
            )
            .with_unit("K"),
        )
        .step(
            Step::new(
                "Absolute pressure",
                "p = p_kPa × 1000",
                pressure_kpa * 1000.0,
            )
            .with_unit("Pa"),
        )
        .result(info(
            "Specific volume",
            s.specific_volume.get::<cubic_meter_per_kilogram>(),
            "m³/kg",
        ))
        .result(info(
            "Density",
            s.density.get::<kilogram_per_cubic_meter>(),
            "kg/m³",
        ))
        .result(info("Enthalpy", s.enthalpy, "kJ/kg"))
        .result(info(
            "Entropy",
            s.entropy.get::<kilojoule_per_kilogram_kelvin>(),
            "kJ/(kg·K)",
        ))
        .result(info(
            "Isobaric heat capacity",
            s.isobaric_heat_capacity
                .get::<kilojoule_per_kilogram_kelvin>(),
            "kJ/(kg·K)",
        ))
        .note(format!("Region: {region}."))
        .reference(IF97_REFERENCE);
    if !tagged.provenance.warnings.is_empty() {
        calc = calc.note(
            "Warning: at least one input lies outside the range this correlation is \
             documented for. The values are shown, but use them with caution.",
        );
    }
    Ok(calc.into_report())
}

/// Inputs to [`beam_udl`], before validation.
pub(crate) struct BeamInputs {
    pub span_m: f64,
    pub load_n_per_m: f64,
    pub width_m: f64,
    pub height_m: f64,
    pub allowable_stress_mpa: Option<f64>,
}

/// Simply supported rectangular beam under a uniform load over its full span.
pub(crate) fn beam_udl(title: &str, input: BeamInputs) -> Result<Report, SheetError> {
    positive("span_m", input.span_m)?;
    finite("load_n_per_m", input.load_n_per_m)?;
    positive("width_m", input.width_m)?;
    positive("height_m", input.height_m)?;
    if let Some(allow) = input.allowable_stress_mpa {
        positive("allowable_stress_mpa", allow)?;
    }

    let span = input.span_m;
    let mut beam = Beam::new(Length::new::<meter>(span));
    beam.add(Load::uniform(
        Length::new::<meter>(0.0),
        Length::new::<meter>(span),
        Force::new::<newton>(input.load_n_per_m * span),
    ));
    let reaction_a = beam.reaction_a().get::<newton>();
    let reaction_b = beam.reaction_b().get::<newton>();
    let max_moment: Torque = beam.max_bending_moment();
    let moment_nm = max_moment.get::<newton_meter>();

    // Bending is about the x axis, so the depth is the height and Z = Sx.
    let z = Rectangle::new(input.width_m, input.height_m)
        .properties()
        .section_modulus_x;
    let stress_mpa = moment_nm / z / 1.0e6;

    let mut calc = Calculation::new(title)
        .input(NamedValue::new("Span", input.span_m).with_unit("m"))
        .input(NamedValue::new("Uniform load", input.load_n_per_m).with_unit("N/m"))
        .input(NamedValue::new("Section width", input.width_m).with_unit("m"))
        .input(NamedValue::new("Section depth", input.height_m).with_unit("m"))
        .step(
            Step::new("Reaction A", "R_A = w·L / 2", reaction_a)
                .with_unit("N")
                .with_reference(BEAM_REFERENCE),
        )
        .step(
            Step::new("Max bending moment", "M_max = w·L² / 8", moment_nm)
                .with_unit("N·m")
                .with_reference(BEAM_REFERENCE),
        )
        .step(
            Step::new("Section modulus", "Z = b·h² / 6", z)
                .with_unit("m³")
                .with_reference("Elastic flexure, rectangular section"),
        )
        .step(
            Step::new("Max bending stress", "σ = M_max / Z", stress_mpa)
                .with_unit("MPa")
                .with_reference("Elastic flexure, σ = M·y / I with y = h/2"),
        )
        .result(info("Reaction A", reaction_a, "N"))
        .result(info("Reaction B", reaction_b, "N"))
        .result(info("Max bending moment", moment_nm, "N·m"))
        .result(info("Max bending stress", stress_mpa, "MPa"))
        .reference(BEAM_REFERENCE)
        .reference("Elastic flexure, rectangular section")
        .reference("Elastic flexure, σ = M·y / I with y = h/2")
        .note(
            "Shear and deflection are not checked. The bending stress uses the \
             elastic section modulus.",
        );

    if let Some(allow) = input.allowable_stress_mpa {
        // The design check compares demand to capacity. Here the demand is the
        // bending stress and the capacity is the allowable stress.
        let check = DemandCapacity::new(stress_mpa, allow, 1.0);
        let status = if check.passes() {
            ValidationStatus::Pass
        } else {
            ValidationStatus::Fail
        };
        calc = calc.result(ResultEntry::new(
            "Utilisation",
            sig6(check.utilization()),
            None,
            status,
            Some("limit: bending stress ≤ allowable stress (utilisation ≤ 1)".to_owned()),
        ));
    }

    Ok(calc.into_report())
}

/// Inputs to [`pid_step`], before validation.
pub(crate) struct PidInputs {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub setpoint: f64,
    pub tau_s: f64,
    pub dt_s: f64,
    pub duration_s: f64,
    pub output_limit: Option<f64>,
}

/// Closed-loop step response of a PID controller on a unit-gain first-order plant.
pub(crate) fn pid_step(title: &str, input: PidInputs) -> Result<Report, SheetError> {
    finite("kp", input.kp)?;
    finite("ki", input.ki)?;
    finite("kd", input.kd)?;
    finite("setpoint", input.setpoint)?;
    positive("tau_s", input.tau_s)?;
    positive("dt_s", input.dt_s)?;
    positive("duration_s", input.duration_s)?;
    if input.dt_s >= input.tau_s {
        return Err(SheetError::Invalid(
            "dt_s must be smaller than tau_s, or the explicit-Euler plant is unstable".to_owned(),
        ));
    }
    let samples = (input.duration_s / input.dt_s).round();
    if samples > MAX_PID_SAMPLES {
        return Err(SheetError::Invalid(format!(
            "duration_s / dt_s must not exceed {MAX_PID_SAMPLES}"
        )));
    }
    if let Some(limit) = input.output_limit {
        positive("output_limit", limit)?;
    }

    let response = simulate(&input, samples as usize);
    let final_value = *response.last().unwrap_or(&0.0);
    let peak = response.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let mut calc = Calculation::new(title)
        .input(NamedValue::new("Kp", input.kp))
        .input(NamedValue::new("Ki", input.ki))
        .input(NamedValue::new("Kd", input.kd))
        .input(NamedValue::new("Setpoint", input.setpoint))
        .input(NamedValue::new("Plant time constant", input.tau_s).with_unit("s"))
        .input(NamedValue::new("Sample period", input.dt_s).with_unit("s"))
        .input(NamedValue::new("Duration", input.duration_s).with_unit("s"))
        .step(Step::new("Samples", "N = round(T / Δt)", samples))
        .step(
            Step::new("Euler gain", "k = Δt / τ", input.dt_s / input.tau_s)
                .with_reference("Explicit Euler on y' = (u − y) / τ"),
        )
        .result(info("Final value", final_value, ""))
        .result(info("Peak value", peak, ""))
        .note(
            "Unit-gain first-order plant y' = (u − y) / τ, integrated with explicit Euler \
             at the sample period. Overshoot is relative to the setpoint.",
        );
    if input.setpoint != 0.0 {
        let overshoot = ((peak - input.setpoint) / input.setpoint).max(0.0) * 100.0;
        calc = calc.result(info("Overshoot", overshoot, "%"));
    }
    if let Some(limit) = input.output_limit {
        calc = calc.input(NamedValue::new("Output limit (±)", limit));
    }
    Ok(calc.into_report())
}

fn simulate(input: &PidInputs, steps: usize) -> Vec<f64> {
    use tpt_eng_controls::{Pid, PidGains};

    let mut pid = Pid::new(PidGains::new(input.kp, input.ki, input.kd));
    if let Some(limit) = input.output_limit {
        pid = pid.with_output_limit(limit);
    }
    pid.set_setpoint(input.setpoint);

    let mut y = 0.0;
    let mut response = Vec::with_capacity(steps + 1);
    response.push(y);
    for _ in 0..steps {
        let u = pid.update(y, input.dt_s);
        y += input.dt_s * (u - y) / input.tau_s;
        response.push(y);
    }
    response
}

/// Round to six significant figures, so floating-point noise such as
/// `5.999999999999998` is not printed in the report.
fn sig6(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let decimals = 5 - value.abs().log10().floor() as i32;
    let scale = 10f64.powi(decimals);
    (value * scale).round() / scale
}

fn info(name: &str, value: f64, unit: &str) -> ResultEntry {
    let unit = (!unit.is_empty()).then(|| unit.to_owned());
    ResultEntry::new(name, sig6(value), unit, ValidationStatus::Info, None)
}
