//! Growth-rate fits for a measured metric against an instance-size parameter.
//!
//! An index-calculus cost story turns on how a quantity — the degree of
//! regularity, the solving degree, the first-fall degree, the relation count,
//! the counted operations — grows with the instance size (field bits `log2 r`,
//! the number of variables, the factor-base dimension). This module fits that
//! growth three ways and reports the slope with a 95% interval, so a run can
//! say whether a metric grows linearly, as a power, or exponentially in the
//! parameter, and how fast.
//!
//! The fit is ordinary least squares on the recorded `(size, value)` pairs,
//! after the model's transform:
//! - `linear`: `value = a + b·size`; `b` is the per-unit growth.
//! - `power`: `ln value = a + b·ln size`; `b` is the power-law exponent.
//! - `exponential`: `ln value = a + b·size`; `b` is the exponential rate.
//!
//! It is descriptive — a handful of instances, one host — and, like the rest of
//! the degree diagnostics, never priced into `S` or used as a stopping rule.

use serde::{Deserialize, Serialize};

/// Which transform the fit was taken under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Model {
    Linear,
    Power,
    Exponential,
}

impl Model {
    pub fn label(self) -> &'static str {
        match self {
            Model::Linear => "linear",
            Model::Power => "power",
            Model::Exponential => "exponential",
        }
    }
    /// What the slope means for this model.
    pub fn slope_meaning(self) -> &'static str {
        match self {
            Model::Linear => "value per unit size",
            Model::Power => "power-law exponent",
            Model::Exponential => "exponential rate per unit size",
        }
    }
}

/// One ordinary-least-squares growth fit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrowthFit {
    pub model: Model,
    /// The fitted slope in the model's transform.
    pub slope: f64,
    /// 95% interval on the slope (`slope ± 1.96·SE`), when `n ≥ 3`.
    pub slope_ci95: Option<[f64; 2]>,
    pub intercept: f64,
    /// Coefficient of determination on the transformed response.
    pub r2: f64,
    pub n_points: usize,
}

/// Ordinary least squares of `ys` on `xs` (already transformed). `None` when
/// there are fewer than two distinct `x`, or a non-finite value appears.
fn ols(xs: &[f64], ys: &[f64], model: Model) -> Option<GrowthFit> {
    let n = xs.len();
    if n < 2 || n != ys.len() || xs.iter().chain(ys).any(|v| !v.is_finite()) {
        return None;
    }
    let nf = n as f64;
    let xbar = xs.iter().sum::<f64>() / nf;
    let ybar = ys.iter().sum::<f64>() / nf;
    let sxx: f64 = xs.iter().map(|x| (x - xbar).powi(2)).sum();
    if sxx <= 0.0 {
        return None;
    }
    let sxy: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (x - xbar) * (y - ybar))
        .sum();
    let slope = sxy / sxx;
    let intercept = ybar - slope * xbar;
    let ss_tot: f64 = ys.iter().map(|y| (y - ybar).powi(2)).sum();
    let ss_res: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - (intercept + slope * x)).powi(2))
        .sum();
    let r2 = if ss_tot > 0.0 {
        1.0 - ss_res / ss_tot
    } else {
        1.0
    };
    // Standard error of the slope needs at least one residual degree of freedom.
    let slope_ci95 = (n >= 3).then(|| {
        let resid_var = ss_res / (nf - 2.0);
        let se = (resid_var / sxx).sqrt();
        [slope - 1.96 * se, slope + 1.96 * se]
    });
    Some(GrowthFit {
        model,
        slope,
        slope_ci95,
        intercept,
        r2,
        n_points: n,
    })
}

/// `value = a + b·size`.
pub fn fit_linear(points: &[(f64, f64)]) -> Option<GrowthFit> {
    let xs: Vec<f64> = points.iter().map(|&(x, _)| x).collect();
    let ys: Vec<f64> = points.iter().map(|&(_, y)| y).collect();
    ols(&xs, &ys, Model::Linear)
}

/// `ln value = a + b·ln size` (both strictly positive).
pub fn fit_power(points: &[(f64, f64)]) -> Option<GrowthFit> {
    if points.iter().any(|&(x, y)| x <= 0.0 || y <= 0.0) {
        return None;
    }
    let xs: Vec<f64> = points.iter().map(|&(x, _)| x.ln()).collect();
    let ys: Vec<f64> = points.iter().map(|&(_, y)| y.ln()).collect();
    ols(&xs, &ys, Model::Power)
}

/// `ln value = a + b·size` (value strictly positive).
pub fn fit_exponential(points: &[(f64, f64)]) -> Option<GrowthFit> {
    if points.iter().any(|&(_, y)| y <= 0.0) {
        return None;
    }
    let xs: Vec<f64> = points.iter().map(|&(x, _)| x).collect();
    let ys: Vec<f64> = points.iter().map(|&(_, y)| y.ln()).collect();
    ols(&xs, &ys, Model::Exponential)
}

/// All applicable fits, best `r2` first — the reported growth rate is the top
/// fit's slope under its model.
pub fn fit_all(points: &[(f64, f64)]) -> Vec<GrowthFit> {
    let mut fits: Vec<GrowthFit> = [fit_linear, fit_power, fit_exponential]
        .iter()
        .filter_map(|f| f(points))
        .collect();
    fits.sort_by(|a, b| b.r2.total_cmp(&a.r2));
    fits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_fit_recovers_a_known_slope() {
        // value = 1 + 3·size, exact.
        let pts: Vec<(f64, f64)> = (1..=5).map(|n| (n as f64, 1.0 + 3.0 * n as f64)).collect();
        let f = fit_linear(&pts).unwrap();
        assert!((f.slope - 3.0).abs() < 1e-9, "slope {}", f.slope);
        assert!((f.intercept - 1.0).abs() < 1e-9);
        assert!((f.r2 - 1.0).abs() < 1e-9);
        let ci = f.slope_ci95.unwrap();
        assert!(ci[0] <= 3.0 && 3.0 <= ci[1]);
    }

    #[test]
    fn power_fit_recovers_a_known_exponent() {
        // value = 2·size^2, exact -> ln value = ln2 + 2·ln size.
        let pts: Vec<(f64, f64)> = (1..=6)
            .map(|n| (n as f64, 2.0 * (n as f64).powi(2)))
            .collect();
        let f = fit_power(&pts).unwrap();
        assert!((f.slope - 2.0).abs() < 1e-9, "exponent {}", f.slope);
        assert!((f.r2 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn exponential_fit_recovers_a_known_rate() {
        // value = e^(0.5·size), exact -> ln value = 0.5·size.
        let pts: Vec<(f64, f64)> = (0..=5)
            .map(|n| (n as f64, (0.5 * n as f64).exp()))
            .collect();
        let f = fit_exponential(&pts).unwrap();
        assert!((f.slope - 0.5).abs() < 1e-9, "rate {}", f.slope);
    }

    #[test]
    fn fit_all_ranks_the_true_model_first() {
        // Exactly power-law data: the power fit must win (r2 == 1).
        let pts: Vec<(f64, f64)> = (1..=6).map(|n| (n as f64, (n as f64).powi(3))).collect();
        let fits = fit_all(&pts);
        assert_eq!(fits[0].model, Model::Power);
        assert!((fits[0].slope - 3.0).abs() < 1e-9);
    }

    #[test]
    fn degenerate_inputs_return_none() {
        assert!(fit_linear(&[(1.0, 2.0)]).is_none()); // one point
        assert!(fit_linear(&[(2.0, 1.0), (2.0, 5.0)]).is_none()); // no x spread
        assert!(fit_power(&[(1.0, 0.0), (2.0, 1.0)]).is_none()); // non-positive y
    }

    /// Records a growth dataset for the research-visuals growth figure: the
    /// measured degree of regularity and solving degree of the regular sequence
    /// {x_i^2 - 1 : i < n} over F_101, swept over n = 2..6 variables, with the
    /// growth-rate fits of each metric against n. No-op unless
    /// `ECBENCH_GROWTH_OUT` names an output path. The engine measures the
    /// degrees and this module fits them; the figure only lays them out.
    #[test]
    fn emit_growth_dataset() {
        let Ok(out) = std::env::var("ECBENCH_GROWTH_OUT") else {
            return;
        };
        use crate::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
        let p = 101u64;
        let mut rows = Vec::new();
        for n in 2..=6usize {
            // x_i^2 - 1 for each i: a regular sequence of n quadrics in n vars,
            // whose degree of regularity is n + 1.
            let input: Vec<Poly> = (0..n)
                .map(|i| {
                    let mut sq = vec![0u32; n];
                    sq[i] = 2;
                    vec![(sq, 1u64), (vec![0u32; n], p - 1)]
                })
                .collect();
            let mut opts = F4Options::new(Ordering::Grevlex, n as u32 + 3);
            opts.record_degree_falls = true;
            let r = f4_fp::f4(&input, n, p, &opts);
            rows.push((n, r.degree_of_regularity_observed, r.solving_degree));
        }
        let dreg_pts: Vec<(f64, f64)> = rows
            .iter()
            .filter_map(|(n, d, _)| d.map(|d| (*n as f64, f64::from(d))))
            .collect();
        let sdeg_pts: Vec<(f64, f64)> = rows
            .iter()
            .map(|(n, _, s)| (*n as f64, f64::from(*s)))
            .collect();
        let dataset = serde_json::json!({
            "schema": "ecbench.growth-dataset/v1",
            "field": p,
            "family": "regular sequence {x_i^2 - 1}, n = 2..6 variables over F_101",
            "metric_x": "n_vars",
            "points": rows.iter().map(|(n, d, s)| serde_json::json!({
                "n_vars": n,
                "degree_of_regularity_observed": d,
                "solving_degree": s,
            })).collect::<Vec<_>>(),
            "fits": {
                "degree_of_regularity_vs_n": fit_all(&dreg_pts),
                "solving_degree_vs_n": fit_all(&sdeg_pts),
            },
        });
        std::fs::write(&out, serde_json::to_string_pretty(&dataset).unwrap()).unwrap();
    }
}
