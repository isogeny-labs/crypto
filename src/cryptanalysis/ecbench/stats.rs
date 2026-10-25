//! The statistics a comparison reports, deterministic in their seed.
//!
//! Rho, BSGS on a random target, and the kangaroo are Las Vegas
//! algorithms: their cost is a random variable over the target and the
//! walk's seed, with a spread of the same order as its mean.  A single
//! run says little; a ratio of two single runs says less.  Every ratio
//! here therefore comes with a bootstrap interval, resampled within
//! workloads (stratified) and over the pairs a round produced, so the
//! interval reflects the variance the design actually has.

/// Arithmetic mean; `None` when empty.
pub fn mean(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}

/// The `q`-quantile (`0 ≤ q ≤ 1`) by linear interpolation of the sorted
/// sample; `None` when empty.
pub fn quantile(xs: &[f64], q: f64) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    let pos = q.clamp(0.0, 1.0) * (v.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    Some(v[lo] + (v[hi] - v[lo]) * (pos - lo as f64))
}

pub fn median(xs: &[f64]) -> Option<f64> {
    quantile(xs, 0.5)
}

/// Sample standard deviation; `None` below two points.
pub fn stdev(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    Some((xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() - 1) as f64).sqrt())
}

/// Geometric mean of positive values; `None` when empty or any is not
/// positive.
pub fn geomean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() || xs.iter().any(|x| *x <= 0.0 || !x.is_finite()) {
        return None;
    }
    Some((xs.iter().map(|x| x.ln()).sum::<f64>() / xs.len() as f64).exp())
}

/// A small deterministic generator for resampling.
pub struct Resampler(u64);

impl Resampler {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    pub fn below(&mut self, n: usize) -> usize {
        self.0 = crate::cryptanalysis::ecbench::canonical::splitmix64(self.0);
        (self.0 % n as u64) as usize
    }
}

/// Percentile bootstrap of `stat` over strata: each resample draws, in
/// every stratum, as many items as it has, with replacement, and
/// evaluates `stat` on the resampled strata.  Returns the `(2.5 %, 97.5
/// %)` interval, or `None` when a stratum is empty or the statistic is
/// undefined on too many resamples.
///
/// This treats the strata as fixed: it answers "how sure are we about
/// *these* workloads".  [`cluster_bootstrap_ci`] also resamples the
/// strata, which is the question a claim about a curve asks.
pub fn bootstrap_ci<T: Clone>(
    strata: &[Vec<T>],
    resamples: usize,
    seed: u64,
    stat: impl Fn(&[Vec<T>]) -> Option<f64>,
) -> Option<(f64, f64)> {
    resample(strata, resamples, seed, false, stat)
}

/// Two-stage (cluster) bootstrap: each resample first draws as many
/// strata as there are, with replacement, then items within each drawn
/// stratum.  With workloads as strata, the interval covers both the
/// variation between targets (BSGS's cost is fixed by its target, so all
/// its variance is here) and the variation between seeds within a
/// target (rho's).  Undefined below two strata, where it would be the
/// one-stage interval in disguise.
pub fn cluster_bootstrap_ci<T: Clone>(
    strata: &[Vec<T>],
    resamples: usize,
    seed: u64,
    stat: impl Fn(&[Vec<T>]) -> Option<f64>,
) -> Option<(f64, f64)> {
    if strata.len() < 2 {
        return None;
    }
    resample(strata, resamples, seed, true, stat)
}

fn resample<T: Clone>(
    strata: &[Vec<T>],
    resamples: usize,
    seed: u64,
    clusters: bool,
    stat: impl Fn(&[Vec<T>]) -> Option<f64>,
) -> Option<(f64, f64)> {
    if strata.is_empty() || strata.iter().any(|s| s.is_empty()) {
        return None;
    }
    let mut rng = Resampler::new(seed);
    let mut values = Vec::with_capacity(resamples);
    for _ in 0..resamples {
        let picked: Vec<&Vec<T>> = if clusters {
            (0..strata.len())
                .map(|_| &strata[rng.below(strata.len())])
                .collect()
        } else {
            strata.iter().collect()
        };
        let draw: Vec<Vec<T>> = picked
            .iter()
            .map(|s| {
                (0..s.len())
                    .map(|_| s[rng.below(s.len())].clone())
                    .collect()
            })
            .collect();
        if let Some(v) = stat(&draw) {
            if v.is_finite() {
                values.push(v);
            }
        }
    }
    if values.len() < resamples * 9 / 10 {
        return None;
    }
    Some((quantile(&values, 0.025)?, quantile(&values, 0.975)?))
}

// ── Linearised (delta-method) ratio intervals with a Student-t quantile ──

/// `ln Γ(x)` for `x > 0` (Lanczos, g = 7, n = 9): relative error below
/// `1e-14` over the range the t distribution needs.
fn ln_gamma(x: f64) -> f64 {
    const G: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        // Reflection.
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = G[0];
    let t = x + 7.5;
    for (i, g) in G.iter().enumerate().skip(1) {
        a += g / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

/// Continued fraction of the regularised incomplete beta function
/// (modified Lentz).
fn beta_cf(a: f64, b: f64, x: f64) -> f64 {
    const TINY: f64 = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < TINY {
        d = TINY;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=500 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-15 {
            break;
        }
    }
    h
}

/// Regularised incomplete beta `I_x(a, b)`.
fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let front =
        (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_cf(a, b, x) / a
    } else {
        1.0 - front * beta_cf(b, a, 1.0 - x) / b
    }
}

/// The Student-t CDF with `df > 0` degrees of freedom.
pub fn t_cdf(t: f64, df: f64) -> f64 {
    if t == 0.0 {
        return 0.5;
    }
    let x = df / (df + t * t);
    let tail = 0.5 * betai(df / 2.0, 0.5, x);
    if t > 0.0 {
        1.0 - tail
    } else {
        tail
    }
}

/// The `p`-quantile of the Student-t distribution with `df > 0` degrees
/// of freedom (`0 < p < 1`); `None` outside that domain.  Closed forms
/// for one and two degrees of freedom; otherwise a bracketed bisection on
/// [`t_cdf`] polished by Newton steps, accurate to about `1e-12`.
pub fn t_quantile(p: f64, df: f64) -> Option<f64> {
    if p.is_nan() || p <= 0.0 || p >= 1.0 || df.is_nan() || df <= 0.0 || !df.is_finite() {
        return None;
    }
    if p == 0.5 {
        return Some(0.0);
    }
    if p < 0.5 {
        return t_quantile(1.0 - p, df).map(|q| -q);
    }
    if df == 1.0 {
        return Some((std::f64::consts::PI * (p - 0.5)).tan());
    }
    if df == 2.0 {
        let a = 4.0 * p * (1.0 - p);
        return Some((2.0 * (p - 0.5)) * (2.0 / a).sqrt());
    }
    // Bracket [lo, hi] with cdf(lo) <= p <= cdf(hi).
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    while t_cdf(hi, df) < p {
        lo = hi;
        hi *= 2.0;
        if hi > 1e12 {
            return None;
        }
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if t_cdf(mid, df) < p {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo < 1e-13 * hi.max(1.0) {
            break;
        }
    }
    let mut x = 0.5 * (lo + hi);
    // Newton polish with the exact density.
    let ln_c =
        ln_gamma((df + 1.0) / 2.0) - ln_gamma(df / 2.0) - 0.5 * (df * std::f64::consts::PI).ln();
    for _ in 0..3 {
        let pdf = (ln_c - (df + 1.0) / 2.0 * (1.0 + x * x / df).ln()).exp();
        if pdf > 0.0 {
            let step = (t_cdf(x, df) - p) / pdf;
            if step.abs() < 1e-6 * x.max(1.0) {
                x -= step;
            }
        }
    }
    Some(x)
}

/// The two-sided 95 % Student-t multiplier for `df` degrees of freedom.
pub fn t_975(df: f64) -> Option<f64> {
    t_quantile(0.975, df)
}

/// A linearised (delta-method) estimate: `estimate`, its standard error
/// and the degrees of freedom of the Student-t reference distribution
/// (clusters − 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Linearised {
    pub estimate: f64,
    pub se: f64,
    pub df: f64,
    pub clusters: usize,
}

impl Linearised {
    /// The 95 % interval `estimate ± t(df) · se`.
    pub fn ci95(&self) -> Option<(f64, f64)> {
        let q = t_975(self.df)?;
        let (lo, hi) = (self.estimate - q * self.se, self.estimate + q * self.se);
        (lo.is_finite() && hi.is_finite()).then_some((lo, hi))
    }

    /// The same interval on the log scale, `estimate · exp(± t · se /
    /// estimate)`: positive by construction, as a ratio of costs must be.
    /// `None` for a non-positive estimate.
    pub fn ci95_log(&self) -> Option<(f64, f64)> {
        if self.estimate.is_nan() || self.estimate <= 0.0 {
            return None;
        }
        let q = t_975(self.df)?;
        let half = q * self.se / self.estimate;
        let (lo, hi) = (self.estimate * (-half).exp(), self.estimate * half.exp());
        (lo.is_finite() && hi.is_finite()).then_some((lo, hi))
    }
}

/// The standard error of an estimator `θ̂` whose first-order expansion is
/// `θ̂ − θ ≈ Σ_k u_k` over `K` independent clusters: `K/(K−1) · Σ (u_k −
/// ū)²`, square-rooted.  `None` below two clusters or on non-finite input.
pub fn influence_se(u: &[f64]) -> Option<f64> {
    let k = u.len();
    if k < 2 || u.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let ubar = u.iter().sum::<f64>() / k as f64;
    let ss: f64 = u.iter().map(|x| (x - ubar) * (x - ubar)).sum();
    Some((k as f64 / (k as f64 - 1.0) * ss).sqrt())
}

/// The ratio of totals `Σ b / Σ a` over clusters `(a_k, b_k)` with its
/// linearised standard error.  The cluster influence values are `z_k =
/// (b_k − R a_k) / mean(a)`, the standard error `sd(z) / √K`, the
/// reference `t(K − 1)`.  `None` below two clusters, or when `Σ a ≤ 0`.
pub fn linearised_ratio(clusters: &[(f64, f64)]) -> Option<Linearised> {
    let k = clusters.len();
    let sa: f64 = clusters.iter().map(|c| c.0).sum();
    let sb: f64 = clusters.iter().map(|c| c.1).sum();
    if sa.is_nan() || sa <= 0.0 || !sb.is_finite() {
        return None;
    }
    let r = sb / sa;
    let u: Vec<f64> = clusters.iter().map(|c| (c.1 - r * c.0) / sa).collect();
    Some(Linearised {
        estimate: r,
        se: influence_se(&u)?,
        df: k as f64 - 1.0,
        clusters: k,
    })
}

/// Combine a ratio `est1` (standard error `se1`, `df1`) with a correction
/// ratio `est2` as `est1 / est2`, treating the two as independent on the
/// log scale (conservative when they share an arm).  The degrees of
/// freedom follow Welch–Satterthwaite on the log scale.
pub fn divide_independent(n: &Linearised, d: &Linearised) -> Option<Linearised> {
    if !(n.estimate > 0.0 && d.estimate > 0.0) {
        return None;
    }
    let (s1, s2) = (n.se / n.estimate, d.se / d.estimate);
    let est = n.estimate / d.estimate;
    let v = s1 * s1 + s2 * s2;
    let df = if v > 0.0 {
        v * v / (s1.powi(4) / n.df + s2.powi(4) / d.df)
    } else {
        n.df.min(d.df)
    };
    Some(Linearised {
        estimate: est,
        se: est * v.sqrt(),
        df,
        clusters: n.clusters.min(d.clusters),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantiles_interpolate() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(quantile(&[], 0.5), None);
        assert!((geomean(&[1.0, 4.0]).unwrap() - 2.0).abs() < 1e-12);
        assert_eq!(geomean(&[1.0, 0.0]), None);
    }

    #[test]
    fn bootstrap_is_deterministic_and_brackets_the_mean() {
        let strata = vec![vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![10.0, 11.0, 12.0]];
        let f = |s: &[Vec<f64>]| mean(&s.concat());
        let a = bootstrap_ci(&strata, 2000, 7, f).unwrap();
        let b = bootstrap_ci(&strata, 2000, 7, f).unwrap();
        assert_eq!(a, b);
        let m = mean(&strata.concat()).unwrap();
        assert!(a.0 < m && m < a.1);
    }

    #[test]
    fn cluster_bootstrap_sees_between_stratum_spread() {
        // No spread inside strata, all of it between them: the one-stage
        // interval collapses to a point, the two-stage one does not.
        let strata = vec![vec![1.0; 4], vec![2.0; 4], vec![3.0; 4]];
        let f = |s: &[Vec<f64>]| mean(&s.concat());
        let one = bootstrap_ci(&strata, 2000, 1, f).unwrap();
        let two = cluster_bootstrap_ci(&strata, 2000, 1, f).unwrap();
        assert_eq!(one.0, one.1);
        assert!(two.1 - two.0 > 0.5);
        assert!(cluster_bootstrap_ci(&strata[..1], 100, 1, f).is_none());
    }

    #[test]
    fn t_quantiles_match_published_tables() {
        // Two-sided 95 % (p = 0.975) critical values, NIST/e-Handbook table.
        for (df, want) in [
            (1.0, 12.706_204_736),
            (2.0, 4.302_652_730),
            (3.0, 3.182_446_305),
            (4.0, 2.776_445_105),
            (5.0, 2.570_581_836),
            (6.0, 2.446_911_851),
            (7.0, 2.364_624_252),
            (8.0, 2.306_004_135),
            (9.0, 2.262_157_163),
            (10.0, 2.228_138_852),
            (15.0, 2.131_449_546),
            (20.0, 2.085_963_447),
            (30.0, 2.042_272_456),
            (60.0, 2.000_297_822),
            (120.0, 1.979_930_405),
        ] {
            let got = t_quantile(0.975, df).unwrap();
            assert!((got - want).abs() < 1e-6, "df {df}: {got} vs {want}");
        }
        // Other tail probabilities and the normal limit.
        let cases = [
            (0.95, 5.0, 2.015_048_373),
            (0.99, 10.0, 2.763_769_458),
            (0.995, 10.0, 3.169_272_667),
            (0.90, 3.0, 1.637_744_354),
            (0.975, 1e6, 1.959_966),
        ];
        for (p, df, want) in cases {
            let got = t_quantile(p, df).unwrap();
            assert!((got - want).abs() < 2e-6, "p {p} df {df}: {got} vs {want}");
        }
        // Symmetry, CDF round trip, domain.
        assert!((t_quantile(0.025, 7.0).unwrap() + t_quantile(0.975, 7.0).unwrap()).abs() < 1e-12);
        for df in [1.0, 2.0, 3.5, 12.0, 40.0] {
            for p in [0.6, 0.9, 0.975, 0.999] {
                let q = t_quantile(p, df).unwrap();
                assert!((t_cdf(q, df) - p).abs() < 1e-10, "df {df} p {p}");
            }
        }
        assert!(t_quantile(0.0, 3.0).is_none());
        assert!(t_quantile(1.0, 3.0).is_none());
        assert!(t_quantile(0.9, 0.0).is_none());
        assert!(t_quantile(0.9, f64::NAN).is_none());
    }

    #[test]
    fn linearised_ratio_is_the_totals_ratio_with_a_t_interval() {
        // Equal-sized clusters, b = 2a exactly: zero influence, point interval.
        let c = [(1.0, 2.0), (3.0, 6.0), (2.0, 4.0)];
        let l = linearised_ratio(&c).unwrap();
        assert!((l.estimate - 2.0).abs() < 1e-15);
        assert!(l.se.abs() < 1e-15);
        assert_eq!(l.df, 2.0);
        // Hand-computed: clusters (1,1) and (1,3): R = 4/2 = 2, z = (b-Ra)/mean(a)
        // = -1 and +1, sd(z) = sqrt(2), se = sqrt(2)/sqrt(2) = 1, t(1) = 12.7062.
        let l = linearised_ratio(&[(1.0, 1.0), (1.0, 3.0)]).unwrap();
        assert!((l.estimate - 2.0).abs() < 1e-12);
        assert!((l.se - 1.0).abs() < 1e-12);
        let (lo, hi) = l.ci95().unwrap();
        assert!((lo - (2.0 - 12.706_204_736)).abs() < 1e-5);
        assert!((hi - (2.0 + 12.706_204_736)).abs() < 1e-5);
        assert!(linearised_ratio(&[(1.0, 1.0)]).is_none());
        assert!(influence_se(&[1.0]).is_none());
    }

    /// Rho-vs-BSGS-like costs of `coverage.py`: A is Rayleigh with mean
    /// `√(π/4)` per run (varies with the seed), B is fixed per target,
    /// uniform on [1, 2].  The true ratio of expectations is `1.5/√(π/4)`.
    fn draw_cluster_model(rng: &mut Resampler, k: usize, rounds: usize) -> Vec<Vec<(f64, f64)>> {
        let sigma = (std::f64::consts::PI / 4.0).sqrt() / (std::f64::consts::PI / 2.0).sqrt();
        let unif = |rng: &mut Resampler| (rng.below(1 << 30) as f64 + 0.5) / (1u64 << 30) as f64;
        (0..k)
            .map(|_| {
                let b = 1.0 + unif(rng);
                (0..rounds)
                    .map(|_| (sigma * (-2.0 * (1.0 - unif(rng)).ln()).sqrt(), b))
                    .collect()
            })
            .collect()
    }

    fn truth() -> f64 {
        1.5 / (std::f64::consts::PI / 4.0).sqrt()
    }

    #[test]
    fn linearised_interval_covers_at_few_workloads() {
        // Deterministic seed, modest size: a few seconds in release.
        let mut rng = Resampler::new(0xEC_BE_4C);
        for k in [2usize, 4, 8] {
            let reps = 3000;
            let (mut hit, mut hit_log) = (0, 0);
            for _ in 0..reps {
                let strata = draw_cluster_model(&mut rng, k, 3);
                let clusters: Vec<(f64, f64)> = strata
                    .iter()
                    .map(|s| (s.iter().map(|p| p.0).sum(), s.iter().map(|p| p.1).sum()))
                    .collect();
                let l = linearised_ratio(&clusters).unwrap();
                let (lo, hi) = l.ci95().unwrap();
                hit += (lo <= truth() && truth() <= hi) as usize;
                let (lo, hi) = l.ci95_log().unwrap();
                hit_log += (lo <= truth() && truth() <= hi) as usize;
            }
            let cov = hit as f64 / reps as f64;
            let cov_log = hit_log as f64 / reps as f64;
            eprintln!("K = {k}: linearised coverage {cov:.4}, log scale {cov_log:.4}");
            assert!(
                (0.92..=0.98).contains(&cov),
                "K = {k}: linearised coverage {cov}"
            );
            assert!(
                (0.92..=0.99).contains(&cov_log),
                "K = {k}: log-scale linearised coverage {cov_log}"
            );
        }
    }

    #[test]
    fn percentile_cluster_bootstrap_undercovers_at_two_workloads() {
        // The finding the linearised interval replaces (DEEP_DIVE 2.1).
        let mut rng = Resampler::new(0x0B007);
        let (reps, mut hit) = (500usize, 0usize);
        for i in 0..reps {
            let strata = draw_cluster_model(&mut rng, 2, 3);
            let stat = |s: &[Vec<(f64, f64)>]| {
                let (a, b) = s
                    .iter()
                    .flatten()
                    .fold((0.0, 0.0), |acc, p| (acc.0 + p.0, acc.1 + p.1));
                Some(b / a)
            };
            let (lo, hi) = cluster_bootstrap_ci(&strata, 400, i as u64 + 1, stat).unwrap();
            hit += (lo <= truth() && truth() <= hi) as usize;
        }
        let cov = hit as f64 / reps as f64;
        assert!(cov < 0.90, "bootstrap coverage at K = 2 was {cov}");
    }
}
