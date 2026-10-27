//! Frozen analysis (PROTOCOL.md section 6): dispersion phi, split-half
//! replicability rho, top-decile transfer, with a size-aware and a naive null.

use std::collections::BTreeMap;

use crate::combi::{binom_f, pattern_weight, pattern_weight_parity, PatternTable};
use crate::runner::Row;
use crate::Rng;

/// Which independence null the class totals are compared with.
/// Naive: all blocks get the mean liftable count, no parity (diagnostic; positive control only).
/// Size:  true per-block liftable counts, no parity (the first-draft null; confounded by the
///        trace-parity constraint, kept as a diagnostic column only).
/// Parity: true per-block counts split by the halving class Tr(x) vs Tr(a); the frozen null.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NullMode {
    Naive,
    Size,
    Parity,
}
impl From<bool> for NullMode {
    fn from(aware: bool) -> NullMode {
        if aware { NullMode::Parity } else { NullMode::Naive }
    }
}

/// Expected pooled class totals over the row's nt targets under the
/// independence null: nt * 2^m / #E * sum_{patterns in class} weight(pattern).
pub fn mu_classes(row: &Row, pt: &PatternTable, mode: impl Into<NullMode>) -> Vec<f64> {
    let mode: NullMode = mode.into();
    let xs: Vec<u64> = if mode != NullMode::Naive || row.xs.len() <= 1 {
        row.xs.clone()
    } else {
        let mean = row.xs.iter().sum::<u64>() as f64 / row.xs.len() as f64;
        vec![mean.round() as u64; row.xs.len()]
    };
    let scale = row.nt as f64 * (1u64 << row.m) as f64 / row.order as f64;
    pt.class_pats
        .iter()
        .map(|ps| {
            scale
                * ps.iter()
                    .map(|&p| match mode {
                        NullMode::Parity => pattern_weight_parity(&pt.pats[p], &xs, &row.xodd),
                        _ => pattern_weight(&pt.pats[p], &xs),
                    })
                    .sum::<f64>()
        })
        .collect()
}

/// Analytic expected decompositions per target, whole base (distinct x, any signs).
pub fn expected_total_per_target(xs: &[u64], m: usize, order: u64) -> f64 {
    // sum over patterns of prod C(X_k, mu) = C(sum X_k, m) when blocks are disjoint (Vandermonde)
    let s: u64 = xs.iter().sum();
    binom_f(s, m as u64) * (1u64 << m) as f64 / order as f64
}

pub struct Pooled {
    pub y: Vec<f64>,
    pub mu: Vec<f64>,
}

/// Pool rows; each row's null is rescaled to that row's observed total.
pub fn pool(rows: &[&Row], pt: &PatternTable, aware: impl Into<NullMode> + Copy, use_frac: bool) -> Pooled {
    let d = pt.n_classes();
    let (mut y, mut mu) = (vec![0.0; d], vec![0.0; d]);
    for r in rows {
        let yy = if use_frac { &r.yfrac } else { &r.ycov };
        let m0 = mu_classes(r, pt, aware);
        let (sy, sm): (f64, f64) = (yy.iter().sum(), m0.iter().sum());
        let k = if sm > 0.0 { sy / sm } else { 0.0 };
        for c in 0..d {
            y[c] += yy[c];
            mu[c] += m0[c] * k;
        }
    }
    Pooled { y, mu }
}

pub fn phi(p: &Pooled) -> f64 {
    let mut x2 = 0.0;
    let mut k = 0;
    for c in 0..p.y.len() {
        if p.mu[c] > 0.0 {
            x2 += (p.y[c] - p.mu[c]).powi(2) / p.mu[c];
            k += 1;
        }
    }
    if k > 1 {
        x2 / (k - 1) as f64
    } else {
        f64::NAN
    }
}

/// Poisson-de-noised mean squared RELATIVE class deviation: mean_c [((Y_c - mu_c)^2 - Y_c) / mu_c^2].
/// Scale-free (does not grow with the counts), so it can be compared across l; it also contains
/// non-replicable V-to-V variance, so only the frob - frobperm difference is read (descriptive).
pub fn rel_bias(p: &Pooled) -> f64 {
    let (mut acc, mut k) = (0.0, 0usize);
    for c in 0..p.y.len() {
        if p.mu[c] > 0.0 {
            acc += ((p.y[c] - p.mu[c]).powi(2) - p.y[c]) / (p.mu[c] * p.mu[c]);
            k += 1;
        }
    }
    if k > 0 { acc / k as f64 } else { f64::NAN }
}

fn resid(p: &Pooled) -> Vec<f64> {
    (0..p.y.len())
        .map(|c| if p.mu[c] > 0.0 { (p.y[c] - p.mu[c]) / p.mu[c].sqrt() } else { 0.0 })
        .collect()
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let (ma, mb) = (a.iter().sum::<f64>() / n, b.iter().sum::<f64>() / n);
    let (mut sab, mut saa, mut sbb) = (0.0, 0.0, 0.0);
    for i in 0..a.len() {
        sab += (a[i] - ma) * (b[i] - mb);
        saa += (a[i] - ma).powi(2);
        sbb += (b[i] - mb).powi(2);
    }
    if saa == 0.0 || sbb == 0.0 {
        0.0
    } else {
        sab / (saa * sbb).sqrt()
    }
}

pub struct SplitStat {
    pub rho: f64,
    pub rho_se: f64,
    pub transfer: f64,
    pub transfer_sd: f64,
    pub phi: f64,
    pub nsplit: usize,
}

/// Split-half replicability over nsplit random balanced splits of the V draws.
pub fn split_stats(rows: &[&Row], pt: &PatternTable, aware: impl Into<NullMode> + Copy, seed: u64, nsplit: usize, nperm: usize) -> SplitStat {
    let d = pt.n_classes();
    let all = pool(rows, pt, aware, true);
    let phi_all = phi(&all);
    let mut rng = Rng(seed);
    let mut halves: Vec<(Vec<f64>, Vec<f64>, Pooled, Pooled)> = vec![];
    for _ in 0..nsplit {
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        for i in (1..idx.len()).rev() {
            idx.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let h = idx.len() / 2;
        let a: Vec<&Row> = idx[..h].iter().map(|&i| rows[i]).collect();
        let b: Vec<&Row> = idx[h..2 * h].iter().map(|&i| rows[i]).collect();
        let (pa, pb) = (pool(&a, pt, aware, true), pool(&b, pt, aware, true));
        halves.push((resid(&pa), resid(&pb), pa, pb));
    }
    let q = ((d as f64 * 0.1).ceil() as usize).max(1);
    let (mut rhos, mut trs) = (vec![], vec![]);
    for (ra, rb, _pa, pb) in &halves {
        rhos.push(pearson(ra, rb));
        let mut order: Vec<usize> = (0..d).collect();
        order.sort_by(|&i, &j| ra[j].partial_cmp(&ra[i]).unwrap());
        let top = &order[..q];
        let (sy, sm): (f64, f64) = (pb.y.iter().sum(), pb.mu.iter().sum());
        let ty: f64 = top.iter().map(|&c| pb.y[c]).sum();
        let tm: f64 = top.iter().map(|&c| pb.mu[c]).sum();
        trs.push(if sy > 0.0 && tm > 0.0 { (ty / sy) / (tm / sm) } else { f64::NAN });
    }
    let rho = rhos.iter().sum::<f64>() / rhos.len() as f64;
    let tr = trs.iter().sum::<f64>() / trs.len() as f64;
    let tr_sd = (trs.iter().map(|t| (t - tr).powi(2)).sum::<f64>() / trs.len().max(1) as f64).sqrt();
    // permutation null for the mean rho: permute class labels of the B residuals
    let mut prng = Rng(seed ^ 0xA5A5_5A5A);
    let mut null = vec![];
    for _ in 0..nperm {
        let mut perm: Vec<usize> = (0..d).collect();
        for i in (1..d).rev() {
            perm.swap(i, prng.below(i as u64 + 1) as usize);
        }
        let mut s = 0.0;
        for (ra, rb, _, _) in &halves {
            let pb: Vec<f64> = perm.iter().map(|&i| rb[i]).collect();
            s += pearson(ra, &pb);
        }
        null.push(s / halves.len() as f64);
    }
    let nm = null.iter().sum::<f64>() / null.len().max(1) as f64;
    let se = (null.iter().map(|v| (v - nm).powi(2)).sum::<f64>() / null.len().max(2) as f64).sqrt();
    SplitStat { rho, rho_se: se, transfer: tr, transfer_sd: tr_sd, phi: phi_all, nsplit }
}

/// Rows grouped by cell then arm.
pub type Cells = BTreeMap<(u32, u32, usize, usize), BTreeMap<String, Vec<Row>>>;

pub fn group(rows: Vec<Row>) -> Cells {
    let mut m: Cells = BTreeMap::new();
    for r in rows {
        m.entry((r.a, r.n, r.m, r.l)).or_default().entry(r.arm.clone()).or_default().push(r);
    }
    m
}

/// Minimum detectable split-half correlation at z = 4 and ~90% power.
pub fn rho_min(d: usize) -> f64 {
    if d < 3 {
        return f64::INFINITY;
    }
    (4.0 + 1.28) / ((d - 1) as f64).sqrt()
}

pub const MIN_D_FOR_RHO: usize = 113; // rho_min <= 0.5

/// Bootstrap (over V draws, rows resampled with replacement within each arm)
/// of phi(frob) / max(phi(controls)). Returns (point, lo95, hi95).
pub fn phi_ratio_boot(frob: &[&Row], ctrls: &[Vec<&Row>], pt: &PatternTable, seed: u64, nboot: usize) -> (f64, f64, f64) {
    let ratio = |f: &[&Row], cs: &[Vec<&Row>]| {
        let pf = phi(&pool(f, pt, true, true));
        let pc = cs.iter().map(|c| phi(&pool(c, pt, true, true))).fold(f64::MIN, f64::max);
        pf / pc
    };
    let point = ratio(frob, ctrls);
    let mut rng = Rng(seed);
    let rs = |v: &[&Row], rng: &mut Rng| -> Vec<usize> { (0..v.len()).map(|_| rng.below(v.len() as u64) as usize).collect() };
    let mut vals = vec![];
    for _ in 0..nboot {
        let fi = rs(frob, &mut rng);
        let f2: Vec<&Row> = fi.iter().map(|&i| frob[i]).collect();
        let cs2: Vec<Vec<&Row>> = ctrls
            .iter()
            .map(|c| rs(c, &mut rng).iter().map(|&i| c[i]).collect())
            .collect();
        vals.push(ratio(&f2, &cs2));
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| vals[((vals.len() as f64 - 1.0) * p).round() as usize];
    (point, q(0.025), q(0.975))
}
