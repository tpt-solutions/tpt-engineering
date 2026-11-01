//! Moist-air psychrometrics (`tpt_eng.air`).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_eng_props_air::{self as air, Error};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

/// Validity range of the ASHRAE saturation-pressure correlation, in K.
const T_MIN_K: f64 = 173.15;
const T_MAX_K: f64 = 473.15;

fn to_py_err(err: Error) -> PyErr {
    let msg = match err {
        Error::VapourPressureExceedsTotal => {
            "vapour pressure must be finite, positive where required, and below total pressure"
        }
        Error::TemperatureOutOfRange => {
            "temperature or vapour pressure is outside the supported range"
        }
        Error::RatioOutOfRange => "humidity ratio must be finite and non-negative",
    };
    PyValueError::new_err(msg)
}

fn check_temperature(temperature_k: f64) -> PyResult<()> {
    if (T_MIN_K..=T_MAX_K).contains(&temperature_k) {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "temperature must be between {T_MIN_K} K and {T_MAX_K} K, got {temperature_k} K"
        )))
    }
}

fn kelvin_of(temperature_k: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(temperature_k)
}

fn pa(pressure_pa: f64) -> Pressure {
    Pressure::new::<pascal>(pressure_pa)
}

/// Saturation vapour pressure of water at `temperature_k`, in pascals.
///
/// Valid for 173.15 K <= T <= 473.15 K.
#[pyfunction]
fn saturation_pressure_water_pa(temperature_k: f64) -> PyResult<f64> {
    check_temperature(temperature_k)?;
    Ok(air::saturation_pressure_water(kelvin_of(temperature_k)).get::<pascal>())
}

/// Humidity ratio (kg water per kg dry air) for vapour partial pressure
/// `vapour_pressure_pa` at total pressure `total_pressure_pa`.
#[pyfunction]
fn humidity_ratio(vapour_pressure_pa: f64, total_pressure_pa: f64) -> PyResult<f64> {
    air::humidity_ratio(pa(vapour_pressure_pa), pa(total_pressure_pa)).map_err(to_py_err)
}

/// Vapour partial pressure, in pascals, implied by humidity ratio `w` at total
/// pressure `total_pressure_pa`. The inverse of `humidity_ratio`.
#[pyfunction]
fn vapour_pressure_from_ratio_pa(w: f64, total_pressure_pa: f64) -> PyResult<f64> {
    air::vapour_pressure_from_ratio(w, pa(total_pressure_pa))
        .map(|p| p.get::<pascal>())
        .map_err(to_py_err)
}

/// Relative humidity, from 0 to 1, for vapour partial pressure
/// `vapour_pressure_pa` at `temperature_k`.
#[pyfunction]
fn relative_humidity(vapour_pressure_pa: f64, temperature_k: f64) -> PyResult<f64> {
    check_temperature(temperature_k)?;
    air::relative_humidity(pa(vapour_pressure_pa), kelvin_of(temperature_k)).map_err(to_py_err)
}

/// Moist-air specific enthalpy, in kJ per kg of dry air, at dry-bulb
/// `temperature_k` and humidity ratio `w`.
#[pyfunction]
fn moist_air_enthalpy_kj_per_kg(temperature_k: f64, w: f64) -> PyResult<f64> {
    check_temperature(temperature_k)?;
    if !(w.is_finite() && w >= 0.0) {
        return Err(to_py_err(Error::RatioOutOfRange));
    }
    Ok(air::moist_air_enthalpy(kelvin_of(temperature_k), w))
}

/// Dew-point temperature, in kelvin, for vapour partial pressure
/// `vapour_pressure_pa`.
#[pyfunction]
fn dew_point_k(vapour_pressure_pa: f64) -> PyResult<f64> {
    air::dew_point(pa(vapour_pressure_pa))
        .map(|t| t.get::<kelvin>())
        .map_err(to_py_err)
}

/// Register the air bindings on the `tpt_eng.air` submodule.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(saturation_pressure_water_pa, m)?)?;
    m.add_function(wrap_pyfunction!(humidity_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(vapour_pressure_from_ratio_pa, m)?)?;
    m.add_function(wrap_pyfunction!(relative_humidity, m)?)?;
    m.add_function(wrap_pyfunction!(moist_air_enthalpy_kj_per_kg, m)?)?;
    m.add_function(wrap_pyfunction!(dew_point_k, m)?)?;
    Ok(())
}
