//! Native descriptive statistics and tests on the recorded solves.  Python
//! only plots what these write.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

pub fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

pub fn var(v: &[f64]) -> f64 {
    let m = mean(v);
    v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0).max(1.0)
}

/// Descriptive summary of one sample.
pub fn summary(v: &[f64]) -> serde_json::Value {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len() as f64;
    let m = mean(&s);
    let sd = var(&s).sqrt();
    let q = |p: f64| s[((p * (n - 1.0)).round() as usize).min(s.len() - 1)];
    let m3 = s.iter().map(|x| (x - m).powi(3)).sum::<f64>() / n;
    let m4 = s.iter().map(|x| (x - m).powi(4)).sum::<f64>() / n;
    let sd_pop = (s.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n).sqrt();
    let geo = (s.iter().map(|x| x.max(1e-300).ln()).sum::<f64>() / n).exp();
    let (lo, hi) = bootstrap_ci(&s, 2000, 0x5eed_b007);
    json!({
        "n": s.len(), "min": s[0], "max": s[s.len() - 1], "mean": m,
        "median": q(0.5), "p10": q(0.1), "p25": q(0.25), "p75": q(0.75), "p90": q(0.9),
        "sd": sd, "cv": sd / m, "ci95_half": 1.96 * sd / n.sqrt(),
        "boot_ci95": [lo, hi],
        "skew": if sd_pop > 0.0 { m3 / sd_pop.powi(3) } else { 0.0 },
        "kurt_excess": if sd_pop > 0.0 { m4 / sd_pop.powi(4) - 3.0 } else { 0.0 },
        "geomean": geo,
    })
}

/// Percentile bootstrap 95 % interval of the mean.
pub fn bootstrap_ci(v: &[f64], reps: usize, seed: u64) -> (f64, f64) {
    if v.len() < 2 {
        return (v[0], v[0]);
    }
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut means: Vec<f64> = (0..reps)
        .map(|_| {
            (0..v.len())
                .map(|_| v[rng.gen_range(0..v.len())])
                .sum::<f64>()
                / v.len() as f64
        })
        .collect();
    means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        means[(reps as f64 * 0.025) as usize],
        means[(reps as f64 * 0.975) as usize],
    )
}

/// Two-sample Kolmogorov–Smirnov `D` and its asymptotic two-sided p-value.
pub fn ks(a: &[f64], b: &[f64]) -> (f64, f64) {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    x.sort_by(|p, q| p.partial_cmp(q).unwrap());
    y.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let (mut i, mut j, mut d) = (0usize, 0usize, 0f64);
    while i < x.len() && j < y.len() {
        if x[i] <= y[j] {
            i += 1;
        } else {
            j += 1;
        }
        d = d.max((i as f64 / x.len() as f64 - j as f64 / y.len() as f64).abs());
    }
    let ne = (x.len() * y.len()) as f64 / (x.len() + y.len()) as f64;
    let lam = (ne.sqrt() + 0.12 + 0.11 / ne.sqrt()) * d;
    let mut p = 0.0;
    for k in 1..100 {
        p += 2.0 * (-1f64).powi(k - 1) * (-2.0 * (k as f64).powi(2) * lam * lam).exp();
    }
    (d, p.clamp(0.0, 1.0))
}

/// Mann–Whitney U of `a` against `b`: the normal-approximated `z` of `a`'s
/// rank sum (positive when `a` tends to be larger).
pub fn mann_whitney_z(a: &[f64], b: &[f64]) -> f64 {
    let mut all: Vec<(f64, u8)> = a
        .iter()
        .map(|&v| (v, 0))
        .chain(b.iter().map(|&v| (v, 1)))
        .collect();
    all.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap());
    let mut ranks = vec![0f64; all.len()];
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j + 1 < all.len() && all[j + 1].0 == all[i].0 {
            j += 1;
        }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for rank in ranks.iter_mut().take(j + 1).skip(i) {
            *rank = r;
        }
        i = j + 1;
    }
    let ra: f64 = all
        .iter()
        .zip(&ranks)
        .filter(|(v, _)| v.1 == 0)
        .map(|(_, r)| r)
        .sum();
    let (na, nb) = (a.len() as f64, b.len() as f64);
    let u = ra - na * (na + 1.0) / 2.0;
    let mu = na * nb / 2.0;
    let sigma = (na * nb * (na + nb + 1.0) / 12.0).sqrt();
    (u - mu) / sigma
}

/// Kruskal–Wallis `H` over groups, with its degrees of freedom and the
/// chi-square survival p-value.
pub fn kruskal_wallis(groups: &[Vec<f64>]) -> (f64, usize, f64) {
    let mut all: Vec<(f64, usize)> = groups
        .iter()
        .enumerate()
        .flat_map(|(g, v)| v.iter().map(move |&x| (x, g)))
        .collect();
    all.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap());
    let n = all.len() as f64;
    let mut rank_sum = vec![0f64; groups.len()];
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j + 1 < all.len() && all[j + 1].0 == all[i].0 {
            j += 1;
        }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for item in all.iter().take(j + 1).skip(i) {
            rank_sum[item.1] += r;
        }
        i = j + 1;
    }
    let h = 12.0 / (n * (n + 1.0))
        * groups
            .iter()
            .enumerate()
            .map(|(g, v)| rank_sum[g].powi(2) / v.len() as f64)
            .sum::<f64>()
        - 3.0 * (n + 1.0);
    let df = groups.len() - 1;
    (h, df, chi2_sf(h, df as f64))
}

/// One-way ANOVA F statistic over groups with its p-value, and the
/// between-group variance ratio `var(group means) / (pooled var / n_i)`.
pub fn anova(groups: &[Vec<f64>]) -> (f64, f64, f64) {
    let k = groups.len() as f64;
    let n: f64 = groups.iter().map(|g| g.len() as f64).sum();
    let grand = groups.iter().flatten().sum::<f64>() / n;
    let ssb: f64 = groups
        .iter()
        .map(|g| g.len() as f64 * (mean(g) - grand).powi(2))
        .sum();
    let ssw: f64 = groups
        .iter()
        .map(|g| g.iter().map(|x| (x - mean(g)).powi(2)).sum::<f64>())
        .sum();
    let f = (ssb / (k - 1.0)) / (ssw / (n - k));
    let p = f_sf(f, k - 1.0, n - k);
    let means: Vec<f64> = groups.iter().map(|g| mean(g)).collect();
    let pooled = ssw / (n - k);
    let ni = n / k;
    let ratio = var(&means) / (pooled / ni);
    (f, p, ratio)
}

/// Regularised upper incomplete gamma `Q(a, x)`.
fn gammq(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    if x < a + 1.0 {
        // series for P
        let (mut sum, mut del, mut ap) = (1.0 / a, 1.0 / a, a);
        for _ in 0..500 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-14 {
                break;
            }
        }
        1.0 - sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        // continued fraction for Q
        let (mut b, mut c, mut d) = (x + 1.0 - a, 1e300, 1.0 / (x + 1.0 - a));
        let mut h = d;
        for i in 1..500 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < 1e-300 {
                d = 1e-300;
            }
            c = b + an / c;
            if c.abs() < 1e-300 {
                c = 1e-300;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-14 {
                break;
            }
        }
        (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

fn ln_gamma(x: f64) -> f64 {
    let g = 7.0;
    let coef = [
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
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = coef[0];
    let t = x + g + 0.5;
    for (i, &c) in coef.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

pub fn chi2_sf(x: f64, df: f64) -> f64 {
    gammq(df / 2.0, x / 2.0)
}

/// Regularised incomplete beta `I_x(a, b)` by continued fraction.
fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < 1e-300 {
        d = 1e-300;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..300 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-300 {
            d = 1e-300;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-300 {
            c = 1e-300;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-300 {
            d = 1e-300;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-300 {
            c = 1e-300;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-14 {
            break;
        }
    }
    h
}

/// Survival function of the F distribution.
pub fn f_sf(f: f64, d1: f64, d2: f64) -> f64 {
    if f <= 0.0 {
        return 1.0;
    }
    betai(d2 / 2.0, d1 / 2.0, d2 / (d2 + d1 * f))
}

/// Two-sided normal p-value of `z`.
pub fn normal_two_sided(z: f64) -> f64 {
    // erfc(|z|/√2)
    let x = z.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.5 * x);
    let y = t
        * (-x * x - 1.265_512_23
            + t * (1.000_023_68
                + t * (0.374_091_96
                    + t * (0.096_784_18
                        + t * (-0.186_288_06
                            + t * (0.278_868_07
                                + t * (-1.135_203_98
                                    + t * (1.488_515_87
                                        + t * (-0.822_152_23 + t * 0.170_872_77)))))))))
            .exp();
    y.min(1.0)
}
