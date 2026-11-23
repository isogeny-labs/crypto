//! Special functions and OLS with one nuisance covariate.

pub fn ln_gamma(x: f64) -> f64 {
    let c = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
    let mut y = x;
    let tmp = x + 5.5 - (x + 0.5) * (x + 5.5).ln();
    let mut ser = 1.000000000190015;
    for cj in c {
        y += 1.0;
        ser += cj / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let tiny = 1e-300;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..400 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
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

/// Regularised incomplete beta I_x(a, b).
pub fn betai(a: f64, b: f64, x: f64) -> f64 {
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

/// Two-sided p-value of Student t with df degrees of freedom.
pub fn t_p_two_sided(t: f64, df: f64) -> f64 {
    betai(df / 2.0, 0.5, df / (df + t * t))
}
/// Upper-tail quantile: t such that two-sided p = 2*(1-q) ... returns t_{q,df}, q in (0.5,1).
pub fn t_quantile(q: f64, df: f64) -> f64 {
    let target = 2.0 * (1.0 - q);
    let (mut lo, mut hi) = (0.0, 1e3);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if t_p_two_sided(mid, df) > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

pub fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 2.0 / (2.0 + z);
    let ty = 4.0 * t - 2.0;
    let cof = [
        -1.3026537197817094, 6.4196979235649026e-1, 1.9476473204185836e-2, -9.561514786808631e-3, -9.46595344482036e-4,
        3.66839497852761e-4, 4.2523324806907e-5, -2.0278578112534e-5, -1.624290004647e-6, 1.303655835580e-6, 1.5626441722e-8,
        -8.5238095915e-8, 6.529054439e-9, 5.059343495e-9, -9.91364156e-10, -2.27365122e-10, 9.6467911e-11, 2.394038e-12,
        -6.886027e-12, 8.94487e-13, 3.13092e-13, -1.12708e-13, 3.81e-16, 7.106e-15, -1.523e-15, -9.4e-17, 1.21e-16, -2.8e-17,
    ];
    let (mut d, mut dd) = (0.0, 0.0);
    for j in (1..cof.len()).rev() {
        let tmp = d;
        d = ty * d - dd + cof[j];
        dd = tmp;
    }
    let r = t * (-z * z + 0.5 * (cof[0] + ty * d) - dd).exp();
    if x >= 0.0 { r } else { 2.0 - r }
}
pub fn norm_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / std::f64::consts::SQRT_2)
}
pub fn norm_quantile(q: f64) -> f64 {
    let (mut lo, mut hi) = (-40.0, 40.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if norm_cdf(mid) < q {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

pub fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
pub fn var(v: &[f64]) -> f64 {
    let m = mean(v);
    v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)
}
pub fn standardize(v: &[f64]) -> Option<Vec<f64>> {
    let (m, s) = (mean(v), var(v).sqrt());
    if !(s > 0.0) {
        return None;
    }
    Some(v.iter().map(|x| (x - m) / s).collect())
}
pub fn quantile(sorted: &[f64], q: f64) -> f64 {
    let pos = q * (sorted.len() as f64 - 1.0);
    let (lo, hi) = (pos.floor() as usize, pos.ceil() as usize);
    sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64)
}

#[derive(Clone, Copy, Debug)]
pub struct Fit {
    pub b: f64,
    pub se: f64,
    pub t: f64,
    pub p: f64,
    pub df: f64,
    pub lo: f64,
    pub hi: f64,
}

/// OLS y = b0 + b1 z + b2 w + e; z, w must already be standardised (any scale works).
pub fn ols(y: &[f64], z: &[f64], w: &[f64]) -> Fit {
    let m = y.len();
    let cols: [&[f64]; 3] = [&vec![1.0; m], z, w];
    let mut a = [[0.0f64; 3]; 3];
    let mut r = [0.0f64; 3];
    for i in 0..3 {
        for j in 0..3 {
            a[i][j] = (0..m).map(|k| cols[i][k] * cols[j][k]).sum();
        }
        r[i] = (0..m).map(|k| cols[i][k] * y[k]).sum();
    }
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    let mut inv = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let (r1, r2) = ((j + 1) % 3, (j + 2) % 3);
            let (c1, c2) = ((i + 1) % 3, (i + 2) % 3);
            inv[i][j] = (a[r1][c1] * a[r2][c2] - a[r1][c2] * a[r2][c1]) / det;
        }
    }
    let mut beta = [0.0f64; 3];
    for i in 0..3 {
        beta[i] = (0..3).map(|j| inv[i][j] * r[j]).sum();
    }
    let rss: f64 = (0..m)
        .map(|k| {
            let f = beta[0] + beta[1] * z[k] + beta[2] * w[k];
            (y[k] - f) * (y[k] - f)
        })
        .sum();
    let df = m as f64 - 3.0;
    let s2 = rss / df;
    let se = (s2 * inv[1][1]).sqrt();
    let t = beta[1] / se;
    let p = t_p_two_sided(t, df);
    let q = t_quantile(0.975, df);
    Fit { b: beta[1], se, t, p, df, lo: beta[1] - q * se, hi: beta[1] + q * se }
}
