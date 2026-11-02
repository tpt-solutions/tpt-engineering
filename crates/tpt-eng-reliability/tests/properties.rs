//! Property-based invariants for fatigue life and Miner's rule.

use proptest::prelude::*;
use tpt_eng_reliability::{basquin_cycles, miners_rule};

proptest! {
    /// Higher stress amplitude means fewer cycles to failure (Basquin).
    #[test]
    fn basquin_life_falls_with_stress(s in 10.0f64..1000.0, factor in 1.01f64..4.0, m in 2.0f64..6.0) {
        let n1 = basquin_cycles(s, m, 12.0).unwrap();
        let n2 = basquin_cycles(s * factor, m, 12.0).unwrap();
        prop_assert!(n2 < n1);
    }

    /// Miner's damage is additive over blocks and never negative.
    #[test]
    fn miners_damage_is_additive(a in 0.0f64..1.0e4, b in 0.0f64..1.0e4, nf1 in 1.0f64..1.0e6, nf2 in 1.0f64..1.0e6) {
        let d1 = miners_rule(&[(a, nf1)]);
        let d2 = miners_rule(&[(b, nf2)]);
        let both = miners_rule(&[(a, nf1), (b, nf2)]);
        prop_assert!(both >= 0.0);
        prop_assert!((both - (d1 + d2)).abs() < 1e-9 * both.max(1.0));
    }
}
