//! Provenance-tagged variants of the ASHRAE psychrometric functions.
//!
//! Each `*_tagged` function returns the same number as its untagged
//! counterpart. It adds the source and the documented validity range, and sets
//! [`Warnings::OUT_OF_RANGE`](tpt_eng_provenance::Warnings::OUT_OF_RANGE) when
//! the temperature lies outside that range.

use tpt_eng_provenance::{Provenance, Source, Tagged, Validity, range_warning};
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

use crate::{Error, relative_humidity, saturation_pressure_water};

const SOURCE: Source = Source::Standard("ASHRAE Fundamentals, Ch. 1 (Hyland-Wexler)");

/// Saturation correlation range: over ice below 273.15 K, over liquid above.
const TEMPERATURE: Validity = Validity {
    input: "temperature",
    unit: "K",
    min: 173.15,
    max: 473.15,
};

const RANGES: &[Validity] = &[TEMPERATURE];

/// Tagged [`saturation_pressure_water`].
pub fn saturation_pressure_water_tagged(t: ThermodynamicTemperature) -> Tagged<Pressure> {
    let warnings = range_warning(&TEMPERATURE, t.get::<kelvin>());
    Tagged::new(
        saturation_pressure_water(t),
        Provenance::new(SOURCE, RANGES).with_warnings(warnings),
    )
}

/// Tagged [`relative_humidity`].
///
/// # Errors
///
/// Returns the same errors as [`relative_humidity`]. A temperature outside the
/// correlation range is returned with a warning, not as an error.
pub fn relative_humidity_tagged(
    p_w: Pressure,
    t: ThermodynamicTemperature,
) -> Result<Tagged<f64>, Error> {
    let value = relative_humidity(p_w, t)?;
    let warnings = range_warning(&TEMPERATURE, t.get::<kelvin>());
    Ok(Tagged::new(
        value,
        Provenance::new(SOURCE, RANGES).with_warnings(warnings),
    ))
}
