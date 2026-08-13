//! # tpt-eng-props-air
//!
//! ASHRAE moist-air (psychrometric) properties for HVAC and combustion-air
//! calculations.
//!
//! Implements the [ASHRAE Fundamentals](https://www.ashrae.org/) saturation
//! pressure correlation (Hyland–Wexler, valid 0–200 °C over liquid water) and
//! the standard moist-air relations: humidity ratio, relative humidity,
//! specific enthalpy, and dew-point temperature.
//!
//! Temperature is [`uom`](tpt_math_units)-typed in kelvin; pressures in
//! pascals. Dimensionless ratios and per-mass enthalpies are plain `f64`.
//!
//! ## Example
//!
//! ```
//! use tpt_math_units::uom::si::f64::*;
//! use tpt_math_units::uom::si::{pressure::kilopascal, thermodynamic_temperature::degree_celsius};
//! use tpt_eng_props_air::saturation_pressure_water;
//!
//! // Saturation pressure of water at 100 °C ≈ 101.325 kPa.
//! let psat = saturation_pressure_water(ThermodynamicTemperature::new::<degree_celsius>(100.0));
//! assert!((psat.get::<kilopascal>() - 101.325).abs() < 0.5);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use tpt_math_numeric::libm;
use tpt_math_units::uom::si::f64::{Pressure, ThermodynamicTemperature};

#[cfg(not(feature = "std"))]
#[inline]
fn exp(x: f64) -> f64 {
    libm::exp(x)
}
#[cfg(not(feature = "std"))]
#[inline]
fn ln(x: f64) -> f64 {
    libm::ln(x)
}
#[cfg(feature = "std")]
#[inline]
fn exp(x: f64) -> f64 {
    x.exp()
}
#[cfg(feature = "std")]
#[inline]
fn ln(x: f64) -> f64 {
    x.ln()
}

/// Molecular-weight ratio of water vapour to dry air (≈ 18.01528 / 28.9644).
const WATER_TO_DRYAIR: f64 = 0.621_945;

/// Specific heat of dry air, kJ/(kg·K).
const C_PA: f64 = 1.006;
/// Latent heat of vaporisation of water at 0 °C, kJ/kg.
const H_FG0: f64 = 2501.0;
/// Specific heat of water vapour, kJ/(kg·K).
const C_PV: f64 = 1.86;

/// Saturation vapour pressure of water over **liquid water** (Hyland–Wexler),
/// in pascals, for `t` in kelvin (valid ~273.15–473.15 K).
///
/// This is the ASHRAE Fundamentals formulation used for moist-air
/// psychrometrics.
pub fn saturation_pressure_water(t: ThermodynamicTemperature) -> Pressure {
    let tk = t.get::<tpt_math_units::uom::si::thermodynamic_temperature::kelvin>();
    let c8 = -5.800_220_6e3;
    let c9 = 1.391_499_3;
    let c10 = -4.864_023_9e-2;
    let c11 = 4.176_476_8e-5;
    let c12 = -1.445_209_3e-8;
    let c13 = 6.545_967_3;
    let ln_ps = c8 / tk + c9 + c10 * tk + c11 * tk * tk + c12 * tk.powi(3) + c13 * ln(tk);
    Pressure::new::<tpt_math_units::uom::si::pressure::pascal>(exp(ln_ps))
}

/// Humidity ratio `W` (kg water / kg dry air) for partial vapour pressure
/// `p_w` (Pa) at total pressure `p` (Pa):
/// `W = 0.621945 · p_w / (p − p_w)`.
pub fn humidity_ratio(p_w: Pressure, p: Pressure) -> f64 {
    let pw = p_w.get::<tpt_math_units::uom::si::pressure::pascal>();
    let pt = p.get::<tpt_math_units::uom::si::pressure::pascal>();
    WATER_TO_DRYAIR * pw / (pt - pw)
}

/// Partial vapour pressure (Pa) implied by humidity ratio `w` at total
/// pressure `p` (Pa). Inverse of [`humidity_ratio`].
pub fn vapour_pressure_from_ratio(w: f64, p: Pressure) -> Pressure {
    let pt = p.get::<tpt_math_units::uom::si::pressure::pascal>();
    let pw = w * pt / (WATER_TO_DRYAIR + w);
    Pressure::new::<tpt_math_units::uom::si::pressure::pascal>(pw)
}

/// Relative humidity `φ ∈ [0, 1]` for vapour partial pressure `p_w` at
/// temperature `t` (uses [`saturation_pressure_water`]).
pub fn relative_humidity(p_w: Pressure, t: ThermodynamicTemperature) -> f64 {
    let psat = saturation_pressure_water(t)
        .get::<tpt_math_units::uom::si::pressure::pascal>();
    p_w.get::<tpt_math_units::uom::si::pressure::pascal>() / psat
}

/// Moist-air specific enthalpy (kJ per kg dry air) for dry-bulb temperature
/// `t` in kelvin and humidity ratio `w`:
/// `h = 1.006·T_c + w·(2501 + 1.86·T_c)`, with `T_c` in °C.
pub fn moist_air_enthalpy(t: ThermodynamicTemperature, w: f64) -> f64 {
    let tc = t.get::<tpt_math_units::uom::si::thermodynamic_temperature::kelvin>() - 273.15;
    C_PA * tc + w * (H_FG0 + C_PV * tc)
}

/// Dew-point temperature (kelvin) for vapour partial pressure `p_w`.
///
/// Inverts [`saturation_pressure_water`] by bisection over 173–373 K.
pub fn dew_point(p_w: Pressure) -> ThermodynamicTemperature {
    let pw = p_w.get::<tpt_math_units::uom::si::pressure::pascal>();
    let (mut lo, mut hi) = (173.0_f64, 373.0_f64);
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let psat = saturation_pressure_water(ThermodynamicTemperature::new::<
            tpt_math_units::uom::si::thermodynamic_temperature::kelvin,
        >(mid))
        .get::<tpt_math_units::uom::si::pressure::pascal>();
        if psat < pw {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    ThermodynamicTemperature::new::<tpt_math_units::uom::si::thermodynamic_temperature::kelvin>(
        0.5 * (lo + hi),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tpt_math_units::uom::si::{
        pressure::{kilopascal, pascal},
        thermodynamic_temperature::{degree_celsius, kelvin},
    };

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn sat_pressure_100c() {
        let p = saturation_pressure_water(ThermodynamicTemperature::new::<degree_celsius>(100.0));
        assert!(approx(p.get::<kilopascal>(), 101.325, 0.5));
    }

    #[test]
    fn sat_pressure_25c() {
        let p = saturation_pressure_water(ThermodynamicTemperature::new::<degree_celsius>(25.0));
        // ASHRAE reference ≈ 3.169 kPa.
        assert!(approx(p.get::<kilopascal>(), 3.169, 0.02));
    }

    #[test]
    fn humidity_ratio_roundtrip() {
        let p = Pressure::new::<kilopascal>(101.325);
        let pw = Pressure::new::<kilopascal>(2.0);
        let w = humidity_ratio(pw, p);
        let pw2 = vapour_pressure_from_ratio(w, p);
        assert!(approx(pw2.get::<pascal>(), pw.get::<pascal>(), 1e-6));
    }

    #[test]
    fn dew_point_consistent() {
        let p = Pressure::new::<kilopascal>(101.325);
        let pw = Pressure::new::<kilopascal>(2.0);
        let tdp = dew_point(pw);
        let psat = saturation_pressure_water(tdp);
        assert!(approx(psat.get::<pascal>(), pw.get::<pascal>(), 1.0));
        // 2 kPa vapour ≈ dew point around 17 °C.
        assert!(tdp.get::<kelvin>() > 273.0 && tdp.get::<kelvin>() < 295.0);
    }
}
