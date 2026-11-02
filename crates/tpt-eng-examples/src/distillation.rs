//! Binary distillation sizing: minimum stages (Fenske), minimum reflux
//! (Underwood), actual stages (Gilliland), and a stage-by-stage McCabe–Thiele
//! count with the feed location. A heat-exchanger ε-NTU check closes the loop
//! on the reboiler duty.

use tpt_eng_unit_ops::{
    epsilon_ntu_phase_change, fenske_min_stages, gilliland_stages, mccabe_thiele_stages,
    underwood_rmin,
};

/// Result of the distillation sizing scenario.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistillationReport {
    /// Relative volatility of the light key to the heavy key (constant).
    pub alpha: f64,
    /// Minimum theoretical stages at total reflux (Fenske).
    pub n_min: f64,
    /// Minimum reflux ratio (Underwood).
    pub r_min: f64,
    /// Operating reflux ratio, `1.3 · R_min`.
    pub reflux: f64,
    /// Theoretical stages at the operating reflux (Gilliland).
    pub stages_gilliland: f64,
    /// Theoretical stages from the stage-by-stage McCabe–Thiele count.
    pub stages_mccabe_thiele: f64,
    /// Stage at which the feed enters, counted from the top.
    pub feed_stage: Option<usize>,
    /// Effectiveness of a reboiler exchanger sized for the duty above.
    pub reboiler_effectiveness: f64,
}

/// Size a benzene–toluene-like column (α = 2.5) for 95 % overhead purity from a
/// 50 % saturated-liquid feed, 5 % bottoms.
///
/// # Panics
///
/// Panics if a correlation rejects its inputs; the fixed inputs here are valid.
pub fn run_distillation() -> DistillationReport {
    let (xd, xb, xf, q, alpha) = (0.95, 0.05, 0.50, 1.0, 2.5);

    let n_min = fenske_min_stages(xd, xb, alpha).expect("valid Fenske inputs");
    let r_min = underwood_rmin(xd, xb, xf, q, alpha).expect("valid Underwood inputs");
    let reflux = 1.3 * r_min;
    let stages_gilliland = gilliland_stages(n_min, r_min, reflux).expect("reflux above minimum");
    let mt = mccabe_thiele_stages(xd, xb, xf, q, alpha, reflux)
        .expect("feasible McCabe-Thiele operating point");

    // Reboiler (boiling side): UA = h·A = 10 W/(m²·K) × 12 m² = 120 W/K over a
    // minimum capacity rate of 1000 W/K gives NTU = 0.12.
    let ntu = 10.0 * 12.0 / 1000.0;
    let reboiler_effectiveness = epsilon_ntu_phase_change(ntu).expect("valid effectiveness inputs");

    DistillationReport {
        alpha,
        n_min,
        r_min,
        reflux,
        stages_gilliland,
        stages_mccabe_thiele: mt.stages,
        feed_stage: mt.feed_stage,
        reboiler_effectiveness,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stages_are_ordered() {
        let r = run_distillation();
        // Minimum stages at total reflux are the floor.
        assert!(r.stages_gilliland >= r.n_min);
        assert!(r.stages_mccabe_thiele >= r.n_min);
        // Operating reflux is above the minimum.
        assert!(r.reflux > r.r_min);
        assert!(r.feed_stage.is_some());
    }
}
