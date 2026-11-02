//! Property-based invariants for the separation and heat-exchanger relations.

use proptest::prelude::*;
use tpt_eng_unit_ops::{epsilon_ntu_counterflow, fenske_min_stages, ntu_from_epsilon_counterflow};

proptest! {
    /// A larger relative volatility separates the components more easily, so
    /// fewer minimum stages are needed.
    #[test]
    fn fenske_stages_fall_with_volatility(alpha in 1.05f64..20.0, factor in 1.01f64..5.0) {
        let n1 = fenske_min_stages(0.95, 0.05, alpha).unwrap();
        let n2 = fenske_min_stages(0.95, 0.05, alpha * factor).unwrap();
        prop_assert!(n2 < n1, "alpha {alpha}: {n1} vs {n2}");
    }

    /// A sharper split needs more stages.
    #[test]
    fn fenske_stages_rise_with_purity(xd in 0.80f64..0.98, delta in 0.005f64..0.01) {
        let loose = fenske_min_stages(xd, 0.05, 2.5).unwrap();
        let sharp = fenske_min_stages(xd + delta, 0.05, 2.5).unwrap();
        prop_assert!(sharp > loose);
    }

    /// The counterflow effectiveness is in [0, 1) and round-trips through NTU.
    #[test]
    fn counterflow_effectiveness_round_trips(ntu in 0.05f64..8.0, cr in 0.0f64..1.0) {
        let eps = epsilon_ntu_counterflow(ntu, cr).unwrap();
        prop_assert!((0.0..=1.0).contains(&eps));
        let back = ntu_from_epsilon_counterflow(eps, cr).unwrap();
        prop_assert!((back - ntu).abs() < 1e-6 * ntu.max(1.0), "ntu {ntu} -> {back}");
    }
}
