//! Tolerance analysis for dimension and parameter stack-up.
//!
//! Provides the standard stack-up methods used in mechanical tolerancing:
//! worst-case, root-sum-square (RSS), and Monte-Carlo evaluation, together
//! with sensitivity and contributor-ranking helpers.

use rand::Rng;
use rand_distr::{Distribution as Distr, Uniform};
use tpt_eng_uncertainty::SampleStats;

/// A single dimension with a bilateral (±) tolerance about its nominal value.
#[derive(Debug, Clone)]
pub struct DimTol {
    /// Identifier for the dimension.
    pub name: String,
    /// Nominal (design) value.
    pub nominal: f64,
    /// Bilateral tolerance (the dimension varies in `[nominal - tol, nominal + tol]`).
    pub tol: f64,
}

impl DimTol {
    /// Build a dimension.
    pub fn new(name: impl Into<String>, nominal: f64, tol: f64) -> Self {
        Self {
            name: name.into(),
            nominal,
            tol: tol.abs(),
        }
    }

    /// Lower bound of the tolerance interval.
    pub fn min(&self) -> f64 {
        self.nominal - self.tol
    }

    /// Upper bound of the tolerance interval.
    pub fn max(&self) -> f64 {
        self.nominal + self.tol
    }
}

/// Result of a Monte-Carlo stack-up evaluation.
#[derive(Debug, Clone)]
pub struct StackupResult {
    /// Number of samples.
    pub n: usize,
    /// Mean of the stack-up.
    pub mean: f64,
    /// Standard deviation of the stack-up.
    pub std: f64,
    /// Minimum observed stack-up.
    pub min: f64,
    /// Maximum observed stack-up.
    pub max: f64,
    /// Estimated yield (fraction of samples within `spec`), if a spec was given.
    pub yield_fraction: Option<f64>,
}

/// Worst-case stack-up interval: `[sum nominal - sum |tol|, sum nominal + sum |tol|]`.
pub fn worst_case(dims: &[DimTol]) -> (f64, f64) {
    let nom: f64 = dims.iter().map(|d| d.nominal).sum();
    let t: f64 = dims.iter().map(|d| d.tol).sum();
    (nom - t, nom + t)
}

/// Root-sum-square (RSS) stack-up interval assuming independent, normally
/// distributed dimensions: `[sum nominal - 3*sigma, sum nominal + 3*sigma]`
/// where `sigma = sqrt(sum tol_i^2 / 3)` (each ±tol spans ±3σ).
pub fn rss(dims: &[DimTol]) -> (f64, f64) {
    let nom: f64 = dims.iter().map(|d| d.nominal).sum();
    let var: f64 = dims.iter().map(|d| d.tol * d.tol / 9.0).sum();
    let sigma = var.sqrt();
    (nom - 3.0 * sigma, nom + 3.0 * sigma)
}

/// Monte-Carlo stack-up evaluation. Each dimension is sampled uniformly in
/// its tolerance interval; the stack-up is the sum. If `spec` is provided as
/// `(low, high)`, the yield fraction is also reported.
pub fn monte_carlo<R: Rng + ?Sized>(
    dims: &[DimTol],
    n: usize,
    spec: Option<(f64, f64)>,
    rng: &mut R,
) -> StackupResult {
    let mut samples = Vec::with_capacity(n);
    let mut in_spec = 0usize;
    for _ in 0..n {
        let mut total = 0.0;
        for d in dims {
            let u = Uniform::new(d.min(), d.max());
            total += u.sample(rng);
        }
        if let Some((lo, hi)) = spec {
            if total >= lo && total <= hi {
                in_spec += 1;
            }
        }
        samples.push(total);
    }
    let stats = SampleStats::from_samples(&samples);
    let yield_fraction = spec.map(|_| in_spec as f64 / n as f64);
    StackupResult {
        n,
        mean: stats.mean,
        std: stats.std,
        min: stats.min,
        max: stats.max,
        yield_fraction,
    }
}

/// RSS variance share of each dimension: `tol_i^2 / sum(tol^2)`.
///
/// Returns a vector aligned with `dims` (empty if no tolerance).
pub fn rss_contributions(dims: &[DimTol]) -> Vec<f64> {
    let total: f64 = dims.iter().map(|d| d.tol * d.tol).sum();
    if total == 0.0 {
        return vec![0.0; dims.len()];
    }
    dims.iter().map(|d| d.tol * d.tol / total).collect()
}

/// Rank dimensions by their RSS contribution (largest first).
///
/// Returns `(original_index, contribution)` pairs sorted descending.
pub fn rank_contributors(dims: &[DimTol]) -> Vec<(usize, f64)> {
    let shares = rss_contributions(dims);
    let mut ranked: Vec<(usize, f64)> = shares.into_iter().enumerate().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    ranked
}

/// Pearson correlation of each dimension with the stack-up output, estimated
/// from a Monte-Carlo run. This measures linear sensitivity of the stack-up
/// to each input's variation.
pub fn monte_carlo_sensitivities<R: Rng + ?Sized>(
    dims: &[DimTol],
    n: usize,
    rng: &mut R,
) -> Vec<f64> {
    let m = dims.len();
    let mut inputs: Vec<Vec<f64>> = (0..m).map(|_| Vec::with_capacity(n)).collect();
    let mut outputs = Vec::with_capacity(n);
    for _ in 0..n {
        let mut total = 0.0;
        for (j, d) in dims.iter().enumerate() {
            let u = Uniform::new(d.min(), d.max());
            let v = u.sample(rng);
            inputs[j].push(v);
            total += v;
        }
        outputs.push(total);
    }
    let out_mean = outputs.iter().sum::<f64>() / n as f64;
    let out_var = outputs.iter().map(|o| (o - out_mean).powi(2)).sum::<f64>();
    let out_std = out_var.sqrt();
    let mut corr = Vec::with_capacity(m);
    for col in &inputs {
        let in_mean = col.iter().sum::<f64>() / n as f64;
        let cov = col
            .iter()
            .zip(&outputs)
            .map(|(x, y)| (x - in_mean) * (y - out_mean))
            .sum::<f64>();
        let in_std = col
            .iter()
            .map(|x| (x - in_mean).powi(2))
            .sum::<f64>()
            .sqrt();
        let denom = in_std * out_std;
        corr.push(if denom > 0.0 { cov / denom } else { 0.0 });
    }
    corr
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn dims() -> Vec<DimTol> {
        vec![
            DimTol::new("a", 10.0, 0.1),
            DimTol::new("b", 20.0, 0.2),
            DimTol::new("c", 5.0, 0.1),
        ]
    }

    #[test]
    fn worst_case_bounds() {
        let (lo, hi) = worst_case(&dims());
        assert_relative_eq!(lo, 35.0 - 0.4);
        assert_relative_eq!(hi, 35.0 + 0.4);
    }

    #[test]
    fn rss_is_tighter_than_worst_case() {
        let (wlo, whi) = worst_case(&dims());
        let (rlo, rhi) = rss(&dims());
        assert!(rlo > wlo);
        assert!(rhi < whi);
    }

    #[test]
    fn monte_carlo_mean_near_nominal() {
        let mut rng = rand::thread_rng();
        let r = monte_carlo(&dims(), 50_000, None, &mut rng);
        assert_relative_eq!(r.mean, 35.0, epsilon = 0.05);
    }

    #[test]
    fn yield_within_wide_spec() {
        let mut rng = rand::thread_rng();
        // Worst-case bounds are 34.6..35.4; a wider spec yields ~100%.
        let r = monte_carlo(&dims(), 20_000, Some((34.0, 36.0)), &mut rng);
        assert!((r.yield_fraction.unwrap() - 1.0).abs() < 0.02);
    }

    #[test]
    fn contributor_ranking() {
        let ranked = rank_contributors(&dims());
        // "b" has the largest tolerance (0.2) so should rank first.
        assert_eq!(ranked[0].0, 1);
        let shares: Vec<f64> = ranked.iter().map(|(_, s)| *s).collect();
        let sum: f64 = shares.iter().sum();
        assert_relative_eq!(sum, 1.0, epsilon = 1e-9);
    }

    #[test]
    fn sensitivity_uniform_correlations() {
        let mut rng = rand::thread_rng();
        let corr = monte_carlo_sensitivities(&dims(), 50_000, &mut rng);
        // For a sum of independent uniforms, corr(X_i, sum) = sqrt(var_i / sum_var),
        // so it grows with each dimension's tolerance. "b" (tol 0.2) must rank
        // highest and "a"/"c" (tol 0.1) must be equal.
        assert!(corr[1] > corr[0], "b should correlate most");
        assert!(corr[1] > corr[2], "b should correlate most");
        assert_relative_eq!(corr[0], corr[2], epsilon = 0.05);
        assert!(corr[0] > 0.0);
    }
}
