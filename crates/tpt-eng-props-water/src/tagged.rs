//! Provenance-tagged variants of the IAPWS-IF97 functions.
//!
//! Each `*_tagged` function returns the same number as its untagged
//! counterpart. It adds the source and the documented validity range, and sets
//! [`Warnings::OUT_OF_RANGE`](tpt_eng_provenance::Warnings::OUT_OF_RANGE) when
//! an input lies outside that range.

use tpt_eng_provenance::{Provenance, Source, Tagged, Validity, range_warning};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::pressure::pascal;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

use crate::{Error, WaterState, saturation_pressure, state};

const SOURCE: Source = Source::Standard("IAPWS-IF97 (2007)");

/// Saturation line (Region 4): triple point to critical point.
const SATURATION_TEMPERATURE: Validity = Validity {
    input: "temperature",
    unit: "K",
    min: 273.15,
    max: 647.096,
};

const SATURATION_RANGES: &[Validity] = &[SATURATION_TEMPERATURE];

/// Single-phase regions 1 and 2: 273.15 K to 1073.15 K, up to 100 MPa.
const TEMPERATURE: Validity = Validity {
    input: "temperature",
    unit: "K",
    min: 273.15,
    max: 1073.15,
};

const PRESSURE: Validity = Validity {
    input: "pressure",
    unit: "Pa",
    min: 0.0,
    max: 100.0e6,
};

const STATE_RANGES: &[Validity] = &[TEMPERATURE, PRESSURE];

/// Tagged [`saturation_pressure`].
pub fn saturation_pressure_tagged(t: ThermodynamicTemperature) -> Tagged<Pressure> {
    let warnings = range_warning(&SATURATION_TEMPERATURE, t.get::<kelvin>());
    Tagged::new(
        saturation_pressure(t),
        Provenance::new(SOURCE, SATURATION_RANGES).with_warnings(warnings),
    )
}

/// Tagged [`state`].
///
/// # Errors
///
/// Returns the same errors as [`state`]. Inputs that are valid but outside the
/// documented range are returned with a warning, not as an error.
pub fn state_tagged(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
) -> Result<Tagged<WaterState>, Error> {
    let value = state(temperature, pressure)?;
    let mut warnings = range_warning(&TEMPERATURE, temperature.get::<kelvin>());
    warnings.insert(range_warning(&PRESSURE, pressure.get::<pascal>()));
    Ok(Tagged::new(
        value,
        Provenance::new(SOURCE, STATE_RANGES).with_warnings(warnings),
    ))
}
