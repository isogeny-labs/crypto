//! Numerical helpers for the frozen wide-sparse-LA analysis.
//!
//! * `Rng`: xoshiro256** (Blackman & Vigna 2018), state seeded from a 64-bit
//!   seed by four SplitMix64 steps, bounded draws by Lemire's unbiased
//!   multiply-and-reject method.  The protocol's bootstrap seed is 20261112.
//!   (numpy's PCG64 used by the earlier Python analysis is NOT reproduced, so
//!   bootstrap intervals differ slightly; point estimates do not depend on it.)
//! * ordinary least squares by Householder QR, percentiles with numpy's
//!   "linear" rule, medians, and the Student-t quantile (regularized
//!   incomplete beta by continued fraction, inverted by bisection).

pub mod notes;

/// xoshiro256** seeded through SplitMix64.
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        let mut z = seed;
        let mut s = [0u64; 4];
        for w in s.iter_mut() {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            *w = x ^ (x >> 31);
        }
        Rng { s }
    }

    pub fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// Uniform integer in [0, n), n >= 1 (Lemire, unbiased).
    pub fn below(&mut self, n: u64) -> u64 {
        let mut m = (self.next_u64() as u128) * (n as u128);
        let mut l = m as u64;
        if l < n {
            let t = n.wrapping_neg() % n;
            while l < t {
                m = (self.next_u64() as u128) * (n as u128);
                l = m as u64;
            }
        }
        (m >> 64) as u64
    }
}

/// Median (mean of the two middle values for even n), as numpy.median.
pub fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}

/// numpy.percentile(v, q) with the default "linear" method; q in [0, 100].
pub fn percentile(v: &[f64], q: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    let virt = (n as f64 - 1.0) * (q / 100.0);
    let prev = virt.floor();
    let gamma = virt - prev;
    let pi = (prev as usize).min(n - 1);
    let ni = (pi + 1).min(n - 1);
    let (a, b) = (s[pi], s[ni]);
    let diff = b - a;
    if gamma >= 0.5 {
        b - diff * (1.0 - gamma)
    } else {
        a + diff * gamma
    }
}

pub fn min_f(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}

pub fn max_f(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

/// Least squares solution of X c = y (X is n x p, full column rank) by Householder QR.
pub fn lstsq(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    let p = x[0].len();
    let mut a: Vec<Vec<f64>> = x.to_vec();
    let mut b = y.to_vec();
    for k in 0..p {
        let norm = (k..n).map(|i| a[i][k] * a[i][k]).sum::<f64>().sqrt();
        if norm == 0.0 {
            continue;
        }
        let alpha = if a[k][k] > 0.0 { -norm } else { norm };
        let mut v: Vec<f64> = (k..n).map(|i| a[i][k]).collect();
        v[0] -= alpha;
        let vn2: f64 = v.iter().map(|t| t * t).sum();
        if vn2 == 0.0 {
            continue;
        }
        for j in k..p {
            let dot: f64 = (k..n).map(|i| v[i - k] * a[i][j]).sum();
            let f = 2.0 * dot / vn2;
            for i in k..n {
                a[i][j] -= f * v[i - k];
            }
        }
        let dot: f64 = (k..n).map(|i| v[i - k] * b[i]).sum();
        let f = 2.0 * dot / vn2;
        for i in k..n {
            b[i] -= f * v[i - k];
        }
    }
    let mut c = vec![0.0; p];
    for k in (0..p).rev() {
        let mut s = b[k];
        for j in k + 1..p {
            s -= a[k][j] * c[j];
        }
        c[k] = s / a[k][k];
    }
    c
}

/// Moore-Penrose pseudo-inverse rows (p x n) of a full-column-rank X (n x p).
pub fn pinv(x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = x.len();
    let p = x[0].len();
    let mut out = vec![vec![0.0; n]; p];
    for i in 0..n {
        let mut e = vec![0.0; n];
        e[i] = 1.0;
        let c = lstsq(x, &e);
        for k in 0..p {
            out[k][i] = c[k];
        }
    }
    out
}

/// Line fit y = a + slope x.
pub fn ols_line(x: &[f64], y: &[f64]) -> (f64, f64) {
    let d: Vec<Vec<f64>> = x.iter().map(|&t| vec![1.0, t]).collect();
    let c = lstsq(&d, y);
    (c[0], c[1])
}

/// Percentile bootstrap draws: resample indices within each group (same count),
/// refit by the fixed pseudo-inverse, return the coefficient vector of every resample.
pub fn bootstrap_coefs(
    design: &[Vec<f64>],
    y: &[f64],
    groups: &[Vec<usize>],
    resamples: usize,
    rng: &mut Rng,
) -> Vec<Vec<f64>> {
    let n = y.len();
    let p = pinv(design);
    let mut idx = vec![0usize; n];
    let mut out = Vec::with_capacity(resamples);
    for _ in 0..resamples {
        for g in groups {
            for &slot in g {
                idx[slot] = g[rng.below(g.len() as u64) as usize];
            }
        }
        let coef: Vec<f64> = p
            .iter()
            .map(|row| (0..n).map(|i| row[i] * y[idx[i]]).sum::<f64>())
            .collect();
        out.push(coef);
    }
    out
}

// ------------------------------------------------------------------ Student t

/// ln Gamma(z) for z >= 0.5 (Lanczos, g = 7, n = 9; relative error about 1e-15).
pub fn lgamma(z: f64) -> f64 {
    const C: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_13,
        -176.615_029_162_140_59,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let z = z - 1.0;
    let mut x = C[0];
    for (i, c) in C.iter().enumerate().skip(1) {
        x += c / (z + i as f64);
    }
    let t = z + 7.5;
    0.5 * (2.0 * std::f64::consts::PI).ln() + (z + 0.5) * t.ln() - t + x.ln()
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    d = 1.0 / if d.abs() > tiny { d } else { tiny };
    let mut h = d;
    for m in 1..400 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let mut aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        d = 1.0 / if d.abs() > tiny { d } else { tiny };
        c = 1.0 + aa / c;
        if c.abs() <= tiny {
            c = tiny;
        }
        h *= d * c;
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        d = 1.0 / if d.abs() > tiny { d } else { tiny };
        c = 1.0 + aa / c;
        if c.abs() <= tiny {
            c = tiny;
        }
        let de = d * c;
        h *= de;
        if (de - 1.0).abs() < 1e-15 {
            break;
        }
    }
    h
}

/// Regularized incomplete beta function I_x(a, b).
pub fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (lgamma(a + b) - lgamma(a) - lgamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

pub fn t_cdf(t: f64, df: f64) -> f64 {
    let x = df / (df + t * t);
    let p = 0.5 * betai(df / 2.0, 0.5, x);
    if t > 0.0 {
        1.0 - p
    } else {
        p
    }
}

/// Student-t quantile by bisection on the CDF (200 halvings of [0, 1e4]).
pub fn t_ppf(p: f64, df: f64) -> f64 {
    let (mut lo, mut hi) = (0.0f64, 1e4f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if t_cdf(mid, df) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// ------------------------------------------------------------------ formatting

/// C-style "%.{p}g".
pub fn fmt_g(x: f64, p: usize) -> String {
    if x == 0.0 {
        return if x.is_sign_negative() {
            "-0".into()
        } else {
            "0".into()
        };
    }
    if !x.is_finite() {
        return if x.is_nan() {
            "nan".into()
        } else if x > 0.0 {
            "inf".into()
        } else {
            "-inf".into()
        };
    }
    let s = format!("{:.*e}", p - 1, x);
    let (mant, exp) = s.split_once('e').unwrap();
    let e: i32 = exp.parse().unwrap();
    fn strip(s: &str) -> String {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s.to_string()
        }
    }
    if e < -4 || e >= p as i32 {
        format!(
            "{}e{}{:02}",
            strip(mant),
            if e < 0 { '-' } else { '+' },
            e.abs()
        )
    } else {
        let dec = (p as i32 - 1 - e) as usize;
        strip(&format!("{:.*}", dec, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol * b.abs().max(1e-300)
    }

    #[test]
    fn ols_line_recovers_exact_line() {
        let x: Vec<f64> = (12..21).map(|v| v as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| -3.5 + 0.97 * v).collect();
        let (a, s) = ols_line(&x, &y);
        assert!(close(a, -3.5, 1e-12), "a={a}");
        assert!(close(s, 0.97, 1e-12), "s={s}");
    }

    #[test]
    fn ols_line_matches_closed_form_on_noisy_data() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y = [2.1, 3.9, 6.2, 7.8, 10.1, 12.3];
        let n = x.len() as f64;
        let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
        let sxy: f64 = x.iter().zip(&y).map(|(a, b)| (a - mx) * (b - my)).sum();
        let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
        let (a, s) = ols_line(&x, &y);
        assert!(close(s, sxy / sxx, 1e-12));
        assert!(close(a, my - sxy / sxx * mx, 1e-12));
    }

    #[test]
    fn quadratic_ols_recovers_polynomial() {
        let x: Vec<f64> = (-3..=4).map(|v| v as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| 2.0 - 0.5 * v + 0.125 * v * v).collect();
        let d: Vec<Vec<f64>> = x.iter().map(|&t| vec![1.0, t, t * t]).collect();
        let c = lstsq(&d, &y);
        assert!(close(c[0], 2.0, 1e-12));
        assert!(close(c[1], -0.5, 1e-12));
        assert!(close(c[2], 0.125, 1e-12));
    }

    #[test]
    fn pinv_is_left_inverse() {
        let d: Vec<Vec<f64>> = (0..7)
            .map(|i| vec![1.0, i as f64, (i * i) as f64])
            .collect();
        let p = pinv(&d);
        for r in 0..3 {
            for c in 0..3 {
                let v: f64 = (0..7).map(|i| p[r][i] * d[i][c]).sum();
                assert!((v - if r == c { 1.0 } else { 0.0 }).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn t_quantiles_match_tables() {
        assert!(close(t_ppf(0.975, 10.0), 2.228_138_851_964_938_5, 1e-9));
        assert!(close(t_ppf(0.975, 25.0), 2.059_538_552_753_294_4, 1e-9));
        assert!(close(t_ppf(0.975, 1.0), 12.706_204_736_432_095, 1e-9));
        assert!(close(t_ppf(0.975, 100.0), 1.983_971_518_523_552_4, 1e-9));
        assert!(close(t_ppf(0.975, 10.0), 2.228139, 1e-6));
        assert!(close(t_ppf(0.975, 25.0), 2.059539, 1e-6));
        assert!((t_cdf(0.0, 7.0) - 0.5).abs() < 1e-15);
    }

    #[test]
    fn lgamma_known_values() {
        assert!(close(lgamma(0.5), 0.5 * std::f64::consts::PI.ln(), 1e-13));
        assert!(close(lgamma(10.0), 362880f64.ln(), 1e-13));
    }

    #[test]
    fn median_odd_even_and_unsorted() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), 2.5);
        assert_eq!(median(&[7.0]), 7.0);
    }

    #[test]
    fn percentile_is_numpy_linear() {
        let v = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&v, 50.0), 3.0);
        assert!(close(percentile(&v, 25.0), 2.0, 1e-15));
        assert!(close(percentile(&v, 2.5), 1.1, 1e-14));
        assert!(close(percentile(&v, 97.5), 4.9, 1e-14));
        assert_eq!(percentile(&v, 100.0), 5.0);
    }

    #[test]
    fn rng_is_deterministic_and_bounded() {
        let mut a = Rng::new(20261112);
        let mut b = Rng::new(20261112);
        for _ in 0..1000 {
            let (x, y) = (a.below(3), b.below(3));
            assert_eq!(x, y);
            assert!(x < 3);
        }
        let mut counts = [0usize; 3];
        let mut r = Rng::new(1);
        for _ in 0..30000 {
            counts[r.below(3) as usize] += 1;
        }
        for c in counts {
            assert!((c as f64 - 10000.0).abs() < 400.0, "{counts:?}");
        }
    }

    #[test]
    fn fmt_g_matches_c() {
        assert_eq!(fmt_g(1.0 / 3.0, 3), "0.333");
        assert_eq!(fmt_g(1.97, 3), "1.97");
        assert_eq!(fmt_g(1.0, 3), "1");
        assert_eq!(fmt_g(3.33e17, 3), "3.33e+17");
        assert_eq!(fmt_g(0.00317, 3), "0.00317");
        assert_eq!(fmt_g(-0.000_169, 3), "-0.000169");
        assert_eq!(fmt_g(0.000_012_3, 3), "1.23e-05");
        assert_eq!(fmt_g(1234.0, 3), "1.23e+03");
        assert_eq!(fmt_g(999.9, 3), "1e+03");
    }
}
