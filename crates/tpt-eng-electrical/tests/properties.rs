//! Property-based invariants for three-phase power.

use proptest::prelude::*;
use tpt_eng_electrical::three_phase_power;

proptest! {
    /// `P² + Q² = S²` with `S = √3·V·I`, for every valid input.
    #[test]
    fn apparent_power_identity(v in 1.0f64..1.0e5, i in 0.0f64..1.0e3, pf in -1.0f64..=1.0) {
        let (p, q) = three_phase_power(v, i, pf).unwrap();
        let s = 3.0f64.sqrt() * v * i;
        let rel = ((p * p + q * q).sqrt() - s).abs() / s.max(1e-12);
        prop_assert!(rel < 1e-9, "P={p} Q={q} S={s}");
    }

    /// Reactive power carries the sign of the power factor: lagging is
    /// positive, leading is negative, and real power is never negative.
    #[test]
    fn reactive_sign_follows_power_factor(v in 1.0f64..1.0e5, i in 1.0f64..1.0e3, pf in -0.999f64..=0.999) {
        let (p, q) = three_phase_power(v, i, pf).unwrap();
        prop_assert!(p >= 0.0);
        prop_assert!(q == 0.0 || q.signum() == pf.signum());
    }

    /// Out-of-range power factors are rejected rather than clamped.
    #[test]
    fn out_of_range_pf_rejected(v in 1.0f64..1.0e5, i in 1.0f64..1.0e3, pf in 1.000_001f64..10.0) {
        prop_assert!(three_phase_power(v, i, pf).is_none());
        prop_assert!(three_phase_power(v, i, -pf).is_none());
    }
}
