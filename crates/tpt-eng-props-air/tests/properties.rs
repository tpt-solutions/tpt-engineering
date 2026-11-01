//! Property-based invariants for saturation pressure.

use proptest::prelude::*;
use tpt_eng_props_air::saturation_pressure_water;
use tpt_math_units::uom::si::f64::ThermodynamicTemperature;
use tpt_math_units::uom::si::thermodynamic_temperature::kelvin;

fn psat(t: f64) -> f64 {
    saturation_pressure_water(ThermodynamicTemperature::new::<kelvin>(t)).value
}

proptest! {
    /// Saturation pressure rises with temperature over the supported range,
    /// across the ice/liquid join at 273.15 K.
    #[test]
    fn saturation_is_monotone(t1 in 180.0f64..370.0, dt in 0.01f64..20.0) {
        let t2 = (t1 + dt).min(373.0);
        prop_assume!(t2 > t1);
        prop_assert!(psat(t2) > psat(t1), "psat({t1})={} psat({t2})={}", psat(t1), psat(t2));
    }

    /// Saturation pressure is positive and finite.
    #[test]
    fn saturation_is_positive(t in 180.0f64..470.0) {
        let p = psat(t);
        prop_assert!(p.is_finite() && p > 0.0);
    }
}
