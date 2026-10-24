//! The one scalar a study minimises.
//!
//! A trial is scored on a ladder of rungs (sizes).  On each rung its
//! band-priced `S` is divided by the frozen baseline's on the same instance
//! and seeds, and the **worst case over the band** is kept:
//!
//! ```text
//! ratio(rung) = max over w in {0, 0.1, 1} of S_trial(w) / S_baseline(w)
//! ```
//!
//! so a winner is better than the baseline at every weight of AGENTS.md §5a,
//! not at the flattering one.  The study's objective is `log2` of that ratio
//! *projected to the target size*: a least-squares line of `log2 ratio`
//! against `log2 r` over the rungs, read at the target.  That is an
//! extrapolation, and it is labelled as one: with fewer than `min_fit`
//! rungs there is no fit and the objective is the last rung's measured
//! value instead (`projected: false`).  Optimising a ratio at one toy size
//! picks configurations that only work at that size; optimising the fitted
//! line also rewards the ones whose advantage grows with size.

use super::trial::Band;

/// Worst `S_trial(w) / S_baseline(w)` over the band.
pub fn worst_ratio(trial: &Band, baseline: &Band) -> Result<f64, String> {
    if trial.weights != baseline.weights || trial.s.len() != baseline.s.len() {
        return Err("trial and baseline were scored on different bands".into());
    }
    let mut worst = f64::NEG_INFINITY;
    for (t, b) in trial.s.iter().zip(&baseline.s) {
        if !(*b > 0.0) || !t.is_finite() {
            return Err(format!("cannot form a ratio from S = {t} over a baseline S = {b}"));
        }
        worst = worst.max(t / b);
    }
    Ok(worst)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fit {
    pub slope: f64,
    pub intercept: f64,
}

impl Fit {
    pub fn at(&self, x: f64) -> f64 {
        self.intercept + self.slope * x
    }
}

/// Ordinary least squares of `y` on `x`; `None` unless there are two
/// distinct `x`.
pub fn fit(points: &[(f64, f64)]) -> Option<Fit> {
    let n = points.len() as f64;
    if points.len() < 2 {
        return None;
    }
    let mx = points.iter().map(|p| p.0).sum::<f64>() / n;
    let my = points.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = points.iter().map(|p| (p.0 - mx).powi(2)).sum();
    if sxx <= 1e-12 {
        return None;
    }
    let sxy: f64 = points.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let slope = sxy / sxx;
    Some(Fit {
        slope,
        intercept: my - slope * mx,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    /// `log2` of the worst-case ratio: below zero beats the baseline.
    pub value: f64,
    /// Whether `value` is a fit read at the target size (otherwise the last
    /// rung's measurement).
    pub projected: bool,
    /// Slope of `log2 ratio` per bit of `log2 r`, when fitted.
    pub slope: Option<f64>,
    pub rungs: usize,
}

/// `points` are `(log2 r, log2 ratio)`, one per rung, in rung order.
pub fn project(points: &[(f64, f64)], target_log2_r: f64, min_fit: usize) -> Option<Projection> {
    let last = points.last()?;
    if points.len() >= min_fit.max(2) {
        if let Some(f) = fit(points) {
            return Some(Projection {
                value: f.at(target_log2_r),
                projected: true,
                slope: Some(f.slope),
                rungs: points.len(),
            });
        }
    }
    Some(Projection {
        value: last.1,
        projected: false,
        slope: None,
        rungs: points.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn band(s: [f64; 3]) -> Band {
        Band {
            weights: vec![0.0, 0.1, 1.0],
            s: s.to_vec(),
            group_gae: 0.0,
            native_ops: 0.0,
            solver_ops: 0.0,
        }
    }

    #[test]
    fn the_ratio_is_the_worst_case_over_the_band() {
        // Twice as good at w = 0, but only 9% better at w = 0.1: the winner
        // has to hold at every weight, so the score is the weakest.
        // Ratios 0.5, 3/3.3 and 0.9: the middle weight is the weakest.
        let r = worst_ratio(&band([1.0, 3.0, 9.0]), &band([2.0, 3.3, 10.0])).unwrap();
        assert!((r - 3.0 / 3.3).abs() < 1e-12, "{r}");
        // Better at the cheap weights, worse at w = 1: not a win.
        let r = worst_ratio(&band([1.0, 3.0, 12.0]), &band([2.0, 4.0, 10.0])).unwrap();
        assert!(r > 1.0);
    }

    #[test]
    fn mismatched_bands_and_zero_baselines_are_errors() {
        let mut other = band([1.0, 1.0, 1.0]);
        other.weights = vec![0.0, 0.5, 1.0];
        assert!(worst_ratio(&band([1.0, 1.0, 1.0]), &other).is_err());
        assert!(worst_ratio(&band([1.0, 1.0, 1.0]), &band([0.0, 1.0, 1.0])).is_err());
    }

    #[test]
    fn the_fit_recovers_a_line() {
        let pts: Vec<(f64, f64)> = (0..6).map(|i| (10.0 + 4.0 * i as f64, 0.5 + 0.2 * (10.0 + 4.0 * i as f64))).collect();
        let f = fit(&pts).unwrap();
        assert!((f.slope - 0.2).abs() < 1e-9 && (f.intercept - 0.5).abs() < 1e-9);
        assert!(fit(&[(3.0, 1.0), (3.0, 2.0)]).is_none(), "one distinct x has no slope");
        assert!(fit(&[(3.0, 1.0)]).is_none());
    }

    #[test]
    fn four_rungs_project_to_the_target_fewer_fall_back_to_the_last_measurement() {
        let line = |x: f64| -1.0 + 0.1 * x;
        let pts: Vec<(f64, f64)> = [11.0, 17.0, 21.0, 39.0].iter().map(|&x| (x, line(x))).collect();
        let p = project(&pts, 81.0, 4).unwrap();
        assert!(p.projected && p.rungs == 4);
        assert!((p.value - line(81.0)).abs() < 1e-9);
        assert!((p.slope.unwrap() - 0.1).abs() < 1e-9);
        // Three rungs: not enough to call a fit, so report the measurement.
        let p = project(&pts[..3], 81.0, 4).unwrap();
        assert!(!p.projected && p.slope.is_none());
        assert_eq!(p.value, pts[2].1);
        assert!(project(&[], 81.0, 4).is_none());
    }

    #[test]
    fn a_flatter_line_wins_at_the_target_even_if_worse_at_the_small_rungs() {
        let steep: Vec<(f64, f64)> = [11.0, 17.0, 21.0, 39.0].iter().map(|&x| (x, -1.0 + 0.10 * (x - 11.0))).collect();
        let flat: Vec<(f64, f64)> = [11.0, 17.0, 21.0, 39.0].iter().map(|&x| (x, -0.5 + 0.02 * (x - 11.0))).collect();
        assert!(steep[0].1 < flat[0].1, "steep is better at the smallest rung");
        let (s, f) = (project(&steep, 81.0, 4).unwrap(), project(&flat, 81.0, 4).unwrap());
        assert!(f.value < s.value, "…but the flatter line is better at the target");
    }
}
