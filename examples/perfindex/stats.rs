//! The statistics of the performance index.  Everything here is exact or
//! driven by a seeded generator: a recorded run can be recomputed from its
//! raw rows and agree to the last digit.

use crate::util::SplitMix64;

/// Bootstrap resamples, for both a kernel's median and the index.
pub const BOOTSTRAP: usize = 2000;

pub fn median(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "median of nothing");
    let mut v = xs.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

/// 95% percentile interval of the median of `values`, by bootstrap.  With
/// fewer than two values there is nothing to resample.
pub fn bootstrap_median_ci(values: &[f64], rng: &mut SplitMix64) -> (f64, f64) {
    if values.len() < 2 {
        let v = median(values);
        return (v, v);
    }
    let mut boots: Vec<f64> = (0..BOOTSTRAP)
        .map(|_| {
            let sample: Vec<f64> = values
                .iter()
                .map(|_| values[rng.below(values.len())])
                .collect();
            median(&sample)
        })
        .collect();
    boots.sort_by(|a, b| a.total_cmp(b));
    (
        boots[(0.025 * BOOTSTRAP as f64) as usize],
        boots[(0.975 * BOOTSTRAP as f64) as usize - 1],
    )
}

/// Binomial coefficient as `f64`: exact for the round counts used here
/// (below 2^53 for n up to about 56) and a good approximation beyond.
fn choose(n: usize, k: usize) -> f64 {
    let k = k.min(n - k);
    let mut r = 1.0f64;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r.round()
}

/// Exact two-sided sign test of "the median of `d` is zero".  Zeros carry no
/// sign and are dropped; `None` when nothing is left.
pub fn sign_test_p(d: &[f64]) -> Option<f64> {
    let pos = d.iter().filter(|&&x| x > 0.0).count();
    let neg = d.iter().filter(|&&x| x < 0.0).count();
    let n = pos + neg;
    if n == 0 {
        return None;
    }
    let k = pos.min(neg);
    let tail: f64 = (0..=k).map(|i| choose(n, i)).sum::<f64>() / 2f64.powi(n as i32);
    Some((2.0 * tail).min(1.0))
}

/// The distribution-free confidence interval for the median given by order
/// statistics, and its true coverage.  With few rounds the coverage is below
/// `conf` (five rounds give the sample range at 93.75%): it says how little
/// five rounds can show, where a percentile bootstrap of five values looks
/// narrower than the data allow.
pub fn median_ci_exact(d: &[f64], conf: f64) -> Option<(f64, f64, f64)> {
    let n = d.len();
    if n == 0 {
        return None;
    }
    let mut xs = d.to_vec();
    xs.sort_by(|a, b| a.total_cmp(b));
    let cover =
        |k: usize| 1.0 - 2.0 * (0..k).map(|i| choose(n, i)).sum::<f64>() / 2f64.powi(n as i32);
    let mut best = 0;
    for k in 1..=n / 2 {
        if cover(k) >= conf {
            best = k;
        }
    }
    let k = best.max(1);
    Some((xs[k - 1], xs[n - k], cover(k).max(0.0)))
}

/// Median of the Walsh averages: a robust location estimate that uses every pair.
pub fn hodges_lehmann(d: &[f64]) -> Option<f64> {
    if d.is_empty() {
        return None;
    }
    let mut walsh = Vec::with_capacity(d.len() * (d.len() + 1) / 2);
    for i in 0..d.len() {
        for j in i..d.len() {
            walsh.push((d[i] + d[j]) / 2.0);
        }
    }
    Some(median(&walsh))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd_even() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), 2.5);
    }

    #[test]
    fn sign_test_exact_values() {
        assert!((sign_test_p(&[1.0; 5]).unwrap() - 2.0 / 32.0).abs() < 1e-15);
        assert!((sign_test_p(&[1.0; 6]).unwrap() - 2.0 / 64.0).abs() < 1e-15);
        assert_eq!(sign_test_p(&[1.0, 1.0, 1.0, -1.0, -1.0]), Some(1.0));
        assert_eq!(sign_test_p(&[0.0, 0.0]), None);
        // zeros carry no sign
        assert!((sign_test_p(&[1.0, 1.0, 1.0, 1.0, 1.0, 0.0]).unwrap() - 2.0 / 32.0).abs() < 1e-15);
        // four rounds all one way: p = 2/16, which the 0.05 level cannot reach
        assert_eq!(sign_test_p(&[-1.0; 4]), Some(0.125));
    }

    #[test]
    fn five_rounds_cannot_reach_five_percent() {
        // the smallest p a five-round sign test can give
        assert!(sign_test_p(&[1.0; 5]).unwrap() > 0.05);
        assert!(sign_test_p(&[1.0; 6]).unwrap() < 0.05);
    }

    #[test]
    fn median_ci_exact_is_the_range_at_five_rounds() {
        let (lo, hi, cover) = median_ci_exact(&[3.0, 1.0, 2.0, 5.0, 4.0], 0.95).unwrap();
        assert_eq!((lo, hi), (1.0, 5.0));
        assert!((cover - (1.0 - 2.0 / 32.0)).abs() < 1e-15);
        let d: Vec<f64> = (0..10).map(f64::from).collect();
        let (lo, hi, cover) = median_ci_exact(&d, 0.95).unwrap();
        assert_eq!((lo, hi), (1.0, 8.0));
        assert!((cover - (1.0 - 2.0 * 11.0 / 1024.0)).abs() < 1e-12);
        assert!(cover >= 0.95);
        assert!(median_ci_exact(&[], 0.95).is_none());
    }

    #[test]
    fn hodges_lehmann_basic() {
        assert_eq!(hodges_lehmann(&[1.0]), Some(1.0));
        assert_eq!(hodges_lehmann(&[0.0, 2.0]), Some(1.0));
        assert_eq!(hodges_lehmann(&[]), None);
    }

    #[test]
    fn bootstrap_of_one_value_is_that_value() {
        let mut r = SplitMix64::new(1);
        assert_eq!(bootstrap_median_ci(&[0.5], &mut r), (0.5, 0.5));
    }

    #[test]
    fn bootstrap_interval_lies_inside_the_data_range_and_is_seeded() {
        let d = [0.1, 0.2, 0.3, 0.4, 0.5];
        let a = bootstrap_median_ci(&d, &mut SplitMix64::new(7));
        let b = bootstrap_median_ci(&d, &mut SplitMix64::new(7));
        assert_eq!(a, b);
        assert!(a.0 >= 0.1 && a.1 <= 0.5 && a.0 <= a.1);
    }

    #[test]
    fn choose_is_exact_for_the_round_counts_in_use() {
        assert_eq!(choose(5, 2), 10.0);
        assert_eq!(choose(10, 3), 120.0);
        assert_eq!(choose(30, 15), 155117520.0);
    }
}
