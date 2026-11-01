//! # tpt-eng-props
//!
//! Umbrella crate re-exporting the `tpt-eng-props-*` fluid-property crates:
//!
//! * [`tpt_eng_props_water`] — IAPWS-IF97 water/steam tables.
//! * [`tpt_eng_props_air`] — ASHRAE moist-air psychrometrics.
//! * [`tpt_eng_props_fuels`] — fuel heating values and combustion properties.
//! * [`tpt_eng_props_mixture`] — general real-gas / VLE property lookups for
//!   arbitrary process fluids.
//!
//! This crate is `no_std`; the individual sub-crates are also `no_std`.
//!
//! ## Example
//!
//! ```
//! use tpt_eng_props::water::saturation_pressure;
//! use tpt_math_units::uom::si::{
//!     f64::ThermodynamicTemperature, pressure::kilopascal, thermodynamic_temperature::kelvin,
//! };
//!
//! // Saturation pressure of water at 100 °C is about 101.3 kPa.
//! let p = saturation_pressure(ThermodynamicTemperature::new::<kelvin>(373.15));
//! assert!((p.get::<kilopascal>() - 101.3).abs() < 0.2);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use tpt_eng_props_air as air;
pub use tpt_eng_props_fuels as fuels;
pub use tpt_eng_props_mixture as mixture;
pub use tpt_eng_props_water as water;
