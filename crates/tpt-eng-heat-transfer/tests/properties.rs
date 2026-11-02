//! Property-based invariants for the convection correlations.

use proptest::prelude::*;
use tpt_eng_heat_transfer::{
    FlowRegime, nusselt_cylinder, nusselt_flat_plate, nusselt_internal_pipe,
};

proptest! {
    /// Heat transfer rises with Reynolds number for a cylinder in cross-flow.
    #[test]
    fn cylinder_nusselt_increases_with_re(re in 1.0f64..1.0e6, factor in 1.01f64..10.0, pr in 0.5f64..10.0) {
        prop_assert!(nusselt_cylinder(re * factor, pr) > nusselt_cylinder(re, pr));
    }

    /// Nusselt numbers are positive and finite for valid Re and Pr.
    #[test]
    fn correlations_are_positive(re in 1.0f64..1.0e6, pr in 0.5f64..10.0) {
        for nu in [
            nusselt_cylinder(re, pr),
            nusselt_flat_plate(re, pr, FlowRegime::Laminar),
            nusselt_internal_pipe(re, pr, true),
        ] {
            prop_assert!(nu.is_finite() && nu > 0.0, "nu = {nu}");
        }
    }

    /// Heating a fluid (Pr^0.4) gives a higher internal-pipe Nusselt number than
    /// cooling it (Pr^0.3) when Pr > 1.
    #[test]
    fn heating_exceeds_cooling_above_unit_prandtl(re in 1.0e4f64..1.0e6, pr in 1.01f64..10.0) {
        prop_assert!(nusselt_internal_pipe(re, pr, true) > nusselt_internal_pipe(re, pr, false));
    }

    /// Invalid dimensionless groups are rejected by a panic, not a silent value.
    #[test]
    fn non_positive_re_is_rejected(re in -1.0e6f64..=0.0) {
        let result = std::panic::catch_unwind(|| nusselt_cylinder(re, 0.7));
        prop_assert!(result.is_err());
    }
}
