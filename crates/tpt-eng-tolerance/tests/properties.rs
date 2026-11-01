//! Property-based invariants for stack-up intervals.

use proptest::prelude::*;
use tpt_eng_tolerance::{DimTol, rss, worst_case};

fn dims_strategy() -> impl Strategy<Value = Vec<DimTol>> {
    prop::collection::vec((-100.0f64..100.0, 0.0f64..5.0), 1..8).prop_map(|v| {
        v.into_iter()
            .enumerate()
            .map(|(i, (nom, tol))| DimTol::new(format!("d{i}"), nom, tol))
            .collect()
    })
}

proptest! {
    /// The RSS half-width is `sqrt(Σ tᵢ²)`, which never exceeds the worst-case
    /// half-width `Σ tᵢ` for non-negative tolerances.
    #[test]
    fn rss_is_never_wider_than_worst_case(dims in dims_strategy()) {
        let (wlo, whi) = worst_case(&dims);
        let (rlo, rhi) = rss(&dims);
        let w_half = (whi - wlo) / 2.0;
        let r_half = (rhi - rlo) / 2.0;
        prop_assert!(r_half <= w_half + 1e-9, "rss {r_half} > worst {w_half}");
    }

    /// Both intervals contain the nominal stack-up.
    #[test]
    fn intervals_contain_nominal(dims in dims_strategy()) {
        let nom: f64 = dims.iter().map(|d| d.nominal).sum();
        let (wlo, whi) = worst_case(&dims);
        let (rlo, rhi) = rss(&dims);
        prop_assert!(wlo <= nom + 1e-9 && nom <= whi + 1e-9);
        prop_assert!(rlo <= nom + 1e-9 && nom <= rhi + 1e-9);
    }
}
