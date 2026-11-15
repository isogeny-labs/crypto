//! Statistics for phase 3, copied verbatim from phase2.rs (whose sha256 is pinned
//! in phase2/PROTOCOL.sha256, so that file stays unchanged): Kruskal-Wallis with the
//! chi-square tail, Spearman with a seeded permutation p, one-sample KS against U(0,1).
#![allow(dead_code)]

pub struct SplitMix(pub u64);
impl SplitMix {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

fn mean(x: &[f64]) -> f64 { x.iter().sum::<f64>() / x.len() as f64 }
fn var(x: &[f64]) -> f64 { let m = mean(x); x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() as f64 - 1.0) }

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] { j += 1; }
        for k in i..=j { r[idx[k]] = (i + j) as f64 / 2.0 + 1.0; }
        i = j + 1;
    }
    r
}

/// ln Gamma (Lanczos), for the chi-square tail.
fn ln_gamma(x: f64) -> f64 {
    let g = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for c in g { y += 1.0; ser += c / y; }
    -tmp + (2.5066282746310005 * ser / x).ln()
}
/// Upper regularized incomplete gamma Q(a, x).
pub fn gamma_q(a: f64, x: f64) -> f64 {
    if x <= 0.0 { return 1.0; }
    if x < a + 1.0 {
        let (mut sum, mut del, mut ap) = (1.0 / a, 1.0 / a, a);
        for _ in 0..10000 { ap += 1.0; del *= x / ap; sum += del; if del.abs() < sum.abs() * 1e-15 { break; } }
        1.0 - sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        let (mut b, mut c, mut d) = (x + 1.0 - a, 1e300, 1.0 / (x + 1.0 - a));
        let mut h = d;
        for i in 1..10000 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b; if d.abs() < 1e-300 { d = 1e-300; }
            c = b + an / c; if c.abs() < 1e-300 { c = 1e-300; }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 { break; }
        }
        (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

/// Kruskal-Wallis H with tie correction; (H, df, p) by the chi-square tail.
pub fn kruskal(groups: &[Vec<f64>]) -> (f64, usize, f64) {
    let all: Vec<f64> = groups.iter().flatten().cloned().collect();
    let n = all.len() as f64;
    let r = ranks(&all);
    let mut h = 0.0;
    let mut off = 0;
    for g in groups {
        let s: f64 = r[off..off + g.len()].iter().sum();
        h += s * s / g.len() as f64;
        off += g.len();
    }
    h = 12.0 / (n * (n + 1.0)) * h - 3.0 * (n + 1.0);
    let mut sorted = all.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut tie = 0.0;
    let mut i = 0;
    while i < sorted.len() {
        let mut j = i;
        while j + 1 < sorted.len() && sorted[j + 1] == sorted[i] { j += 1; }
        let t = (j - i + 1) as f64;
        tie += t * t * t - t;
        i = j + 1;
    }
    h /= 1.0 - tie / (n * n * n - n);
    let df = groups.len() - 1;
    (h, df, gamma_q(df as f64 / 2.0, h / 2.0))
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let (ma, mb) = (mean(a), mean(b));
    let (mut s, mut sa, mut sb) = (0.0, 0.0, 0.0);
    for i in 0..a.len() { s += (a[i] - ma) * (b[i] - mb); sa += (a[i] - ma).powi(2); sb += (b[i] - mb).powi(2); }
    s / (sa * sb).sqrt()
}

/// Spearman rho and a two-sided permutation p (seeded shuffles of y).
pub fn spearman_perm(x: &[f64], y: &[f64], perms: usize, seed: u64) -> (f64, f64) {
    let rx = ranks(x);
    let mut ry = ranks(y);
    let rho = pearson(&rx, &ry);
    let mut rng = SplitMix(seed);
    let mut ge = 0usize;
    for _ in 0..perms {
        for i in (1..ry.len()).rev() { let j = rng.below(i as u64 + 1) as usize; ry.swap(i, j); }
        if pearson(&rx, &ry).abs() >= rho.abs() - 1e-12 { ge += 1; }
    }
    (rho, (ge + 1) as f64 / (perms + 1) as f64)
}

/// One-sample KS of p-values against U(0,1); (D, asymptotic p).
pub fn ks_uniform(ps: &[f64]) -> (f64, f64) {
    let mut v = ps.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len() as f64;
    let mut d: f64 = 0.0;
    for (i, &x) in v.iter().enumerate() {
        d = d.max((i as f64 + 1.0) / n - x).max(x - i as f64 / n);
    }
    let lam = (n.sqrt() + 0.12 + 0.11 / n.sqrt()) * d;
    let mut p = 0.0;
    for k in 1..200 { let k = k as f64; p += 2.0 * (-1f64).powf(k - 1.0) * (-2.0 * k * k * lam * lam).exp(); }
    (d, p.clamp(0.0, 1.0))
}

