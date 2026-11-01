//! Python bindings for tpt-engineering (PyO3).
//!
//! v0.1 exposes IAPWS-IF97 water/steam properties from `tpt-eng-props-water`.
//!
//! Conventions for every binding:
//!
//! * Inputs and outputs are plain `f64`. SI units are part of each name
//!   (`_k`, `_pa`, `_kj_per_kg`), so Python callers never guess units.
//! * Core errors become Python exceptions. Out-of-range inputs raise
//!   `ValueError`, and Region 3 raises `NotImplementedError`.
//! * Validity ranges are checked here, not left silent. The core functions
//!   quietly return values outside their documented range.

use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use tpt_eng_props_water::{self as water, Error, Region};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::mass_density::kilogram_per_cubic_meter;
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::specific_heat_capacity::kilojoule_per_kilogram_kelvin;
use tpt_math_units::uom::si::specific_volume::cubic_meter_per_kilogram;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

/// Lower bound of the saturation-line validity range (triple point), in K.
const T_SAT_MIN_K: f64 = 273.15;
/// Upper bound of the saturation-line validity range (critical point), in K.
const T_SAT_MAX_K: f64 = 647.096;

fn to_py_err(err: Error) -> PyErr {
    match err {
        Error::TemperatureOutOfRange => {
            PyValueError::new_err("temperature must be at least 273.15 K")
        }
        Error::NegativePressure => PyValueError::new_err("pressure must be non-negative"),
        Error::Region3Unsupported => PyNotImplementedError::new_err(
            "state is in IAPWS-IF97 Region 3 (near-critical), which is not implemented yet",
        ),
    }
}

fn kelvin_of(temperature_k: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(temperature_k)
}

fn psat_pa(temperature_k: f64) -> f64 {
    water::saturation_pressure(kelvin_of(temperature_k)).get::<pascal>()
}

/// Saturation pressure of water at `temperature_k`, in pascals.
///
/// Valid for 273.15 K <= T <= 647.096 K.
#[pyfunction]
fn water_saturation_pressure_pa(temperature_k: f64) -> PyResult<f64> {
    if !(T_SAT_MIN_K..=T_SAT_MAX_K).contains(&temperature_k) {
        return Err(PyValueError::new_err(format!(
            "saturation pressure is valid for {T_SAT_MIN_K} K to {T_SAT_MAX_K} K, got {temperature_k} K"
        )));
    }
    Ok(psat_pa(temperature_k))
}

/// Saturation temperature of water at `pressure_pa`, in kelvin.
///
/// Valid for pressures between the saturation pressures at the triple point
/// and the critical point (about 611 Pa to 22.06 MPa).
#[pyfunction]
fn water_saturation_temperature_k(pressure_pa: f64) -> PyResult<f64> {
    let (p_min, p_max) = (psat_pa(T_SAT_MIN_K), psat_pa(T_SAT_MAX_K));
    if !(p_min..=p_max).contains(&pressure_pa) {
        return Err(PyValueError::new_err(format!(
            "saturation temperature is valid for {p_min} Pa to {p_max} Pa, got {pressure_pa} Pa"
        )));
    }
    Ok(water::saturation_temperature(Pressure::new::<pascal>(pressure_pa)).get::<kelvin>())
}

/// Full single-phase water/steam state. Units are in each field name.
#[pyclass(module = "tpt_eng", frozen, get_all)]
#[derive(Clone)]
pub struct WaterState {
    temperature_k: f64,
    pressure_pa: f64,
    specific_volume_m3_per_kg: f64,
    density_kg_per_m3: f64,
    enthalpy_kj_per_kg: f64,
    entropy_kj_per_kg_k: f64,
    internal_energy_kj_per_kg: f64,
    isobaric_heat_capacity_kj_per_kg_k: f64,
    isochoric_heat_capacity_kj_per_kg_k: f64,
    /// One of "liquid", "vapour" or "saturation".
    region: &'static str,
}

#[pymethods]
impl WaterState {
    fn __repr__(&self) -> String {
        format!(
            "WaterState(temperature_k={}, pressure_pa={}, region={:?})",
            self.temperature_k, self.pressure_pa, self.region
        )
    }
}

/// Compute the water/steam state at `temperature_k` and `pressure_pa`.
#[pyfunction]
fn water_state(temperature_k: f64, pressure_pa: f64) -> PyResult<WaterState> {
    let s = water::state(
        kelvin_of(temperature_k),
        Pressure::new::<pascal>(pressure_pa),
    )
    .map_err(to_py_err)?;
    Ok(WaterState {
        temperature_k,
        pressure_pa,
        specific_volume_m3_per_kg: s.specific_volume.get::<cubic_meter_per_kilogram>(),
        density_kg_per_m3: s.density.get::<kilogram_per_cubic_meter>(),
        enthalpy_kj_per_kg: s.enthalpy,
        entropy_kj_per_kg_k: s.entropy.get::<kilojoule_per_kilogram_kelvin>(),
        internal_energy_kj_per_kg: s.internal_energy,
        isobaric_heat_capacity_kj_per_kg_k: s
            .isobaric_heat_capacity
            .get::<kilojoule_per_kilogram_kelvin>(),
        isochoric_heat_capacity_kj_per_kg_k: s
            .isochoric_heat_capacity
            .get::<kilojoule_per_kilogram_kelvin>(),
        region: match s.region {
            Region::One => "liquid",
            Region::Two => "vapour",
            Region::Saturation => "saturation",
        },
    })
}

#[pymodule]
fn tpt_eng(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WaterState>()?;
    m.add_function(wrap_pyfunction!(water_saturation_pressure_pa, m)?)?;
    m.add_function(wrap_pyfunction!(water_saturation_temperature_k, m)?)?;
    m.add_function(wrap_pyfunction!(water_state, m)?)?;
    Ok(())
}
