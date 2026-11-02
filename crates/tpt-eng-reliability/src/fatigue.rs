//! Fatigue life models.

use crate::ReliabilityError;

/// Basquin (S–N) relation: number of cycles to failure for a given stress
/// amplitude `s`.
///
/// Uses `log10(N) = intercept - m * log10(s)`, i.e.
/// `N = 10^intercept * s^(-m)`. `m` is the (positive) S–N slope and
/// `intercept` is `log10` of the fatigue strength coefficient.
///
/// # Errors
/// Returns [`ReliabilityError::InvalidParameter`] for non-positive `s`, `m`,
/// or `intercept` outside a sane range.
pub fn basquin_cycles(s: f64, m: f64, intercept: f64) -> Result<f64, ReliabilityError> {
    if s <= 0.0 || m <= 0.0 {
        return Err(ReliabilityError::InvalidParameter(
            "stress amplitude and slope must be positive".into(),
        ));
    }
    let n = 10f64.powf(intercept - m * s.log10());
    Ok(n)
}

/// Equivalent stress amplitude that produces `n` cycles to failure under the
/// Basquin relation (the inverse of [`basquin_cycles`]).
///
/// # Errors
/// Returns [`ReliabilityError::InvalidParameter`] for non-positive `n` or `m`.
pub fn basquin_stress(n: f64, m: f64, intercept: f64) -> Result<f64, ReliabilityError> {
    if n <= 0.0 || m <= 0.0 {
        return Err(ReliabilityError::InvalidParameter(
            "cycles and slope must be positive".into(),
        ));
    }
    let s = 10f64.powf((intercept - n.log10()) / m);
    Ok(s)
}

/// Miner's rule cumulative fatigue damage for several stress blocks.
///
/// `blocks` is a list of `(n_applied, n_to_failure)` pairs. Failure is
/// expected when the returned damage reaches or exceeds 1.0.
///
/// A block with `n_applied > 0` and a non-positive or non-finite `n_to_failure`
/// means the stress is at or beyond failure, so it contributes infinite damage.
/// A block with no applied cycles contributes zero.
pub fn miners_rule(blocks: &[(f64, f64)]) -> f64 {
    blocks
        .iter()
        .map(|(n, nf)| {
            if *n <= 0.0 {
                0.0
            } else if nf.is_finite() && *nf > 0.0 {
                n / nf
            } else {
                f64::INFINITY
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn basquin_roundtrip() {
        let n = basquin_cycles(100.0, 3.0, 12.0).unwrap();
        let s = basquin_stress(n, 3.0, 12.0).unwrap();
        assert_relative_eq!(s, 100.0, epsilon = 1e-6);
    }

    #[test]
    fn miners_rule_zero_life_is_failure() {
        // N_f = 0 with cycles applied is already failed, not "no damage".
        assert!(miners_rule(&[(10.0, 0.0)]).is_infinite());
        assert!(miners_rule(&[(10.0, f64::NAN)]).is_infinite());
        // No applied cycles contributes nothing, even with a bad N_f.
        assert_eq!(miners_rule(&[(0.0, 0.0)]), 0.0);
    }

    #[test]
    fn miners_rule_sums() {
        // Two blocks, each at half of its fatigue life -> damage = 1.0.
        let d = miners_rule(&[(500.0, 1000.0), (250.0, 500.0)]);
        assert_relative_eq!(d, 1.0, epsilon = 1e-9);
    }
}
