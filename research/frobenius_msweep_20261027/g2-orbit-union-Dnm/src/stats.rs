//! Frozen analysis (PROTOCOL.md section 6): dispersion phi, split-half
//! replicability rho, top-decile transfer, with a size-aware and a naive null.

use std::collections::BTreeMap;

use crate::combi::{
    binom_f, pattern_weight, pattern_weight_mod4, pattern_weight_parity, PatternTable,
};
use crate::runner::Row;
use crate::Rng;

/// Which independence null the class totals are compared with.
/// Naive: all blocks get the mean liftable count, no parity (diagnostic; positive control only).
/// Size:  true per-block liftable counts, no parity (the first-draft null; confounded by the
///        trace-parity constraint, kept as a diagnostic column only).
/// Parity: true per-block counts split by the class of the point in E/2E (a = 1: Z/2) or E/4E
///        (a = 0: Z/4, `pattern_weight_mod4`); the frozen null.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NullMode {
    Naive,
    Size,
    Parity,
}
impl From<bool> for NullMode {
    fn from(aware: bool) -> NullMode {
        if aware {
            NullMode::Parity
        } else {
            NullMode::Naive
        }
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
                        NullMode::Parity if row.a == 0 => {
                            pattern_weight_mod4(&pt.pats[p], &xs, &row.xodd, &row.xtwo)
                        }
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
pub fn pool(
    rows: &[&Row],
    pt: &PatternTable,
    aware: impl Into<NullMode> + Copy,
    use_frac: bool,
) -> Pooled {
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
    if k > 0 {
        acc / k as f64
    } else {
        f64::NAN
    }
}

fn resid(p: &Pooled) -> Vec<f64> {
    (0..p.y.len())
        .map(|c| {
            if p.mu[c] > 0.0 {
                (p.y[c] - p.mu[c]) / p.mu[c].sqrt()
            } else {
                0.0
            }
        })
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
    /// sd of rho over simulated null datasets analysed with THE SAME code and THE SAME 40 splits
    pub rho_se: f64,
    /// mean of rho over those simulated null datasets (small and negative at low D)
    pub rho_null_mean: f64,
    pub transfer: f64,
    pub transfer_sd: f64,
    pub phi: f64,
    pub nsplit: usize,
}

/// Mean over `nsplit` random balanced splits of the V draws of Pearson(r_A, r_B), and of the
/// top-decile transfer. `ys[i]` are the class counts of row i, `m0[i]` its UNSCALED null (each row's
/// null is rescaled to the row's total inside the pooling, exactly as in `pool`).
fn split_core(ys: &[&[f64]], m0: &[Vec<f64>], seed: u64, nsplit: usize) -> (f64, f64, f64) {
    let d = m0[0].len();
    let nrows = ys.len();
    let pool_idx = |idx: &[usize]| -> Pooled {
        let (mut y, mut mu) = (vec![0.0; d], vec![0.0; d]);
        for &i in idx {
            let (sy, sm): (f64, f64) = (ys[i].iter().sum(), m0[i].iter().sum());
            let k = if sm > 0.0 { sy / sm } else { 0.0 };
            for c in 0..d {
                y[c] += ys[i][c];
                mu[c] += m0[i][c] * k;
            }
        }
        Pooled { y, mu }
    };
    let mut rng = Rng(seed);
    let q = ((d as f64 * 0.1).ceil() as usize).max(1);
    let (mut rhos, mut trs) = (vec![], vec![]);
    for _ in 0..nsplit {
        let mut idx: Vec<usize> = (0..nrows).collect();
        for i in (1..idx.len()).rev() {
            idx.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let h = idx.len() / 2;
        let (pa, pb) = (pool_idx(&idx[..h]), pool_idx(&idx[h..2 * h]));
        let (ra, rb) = (resid(&pa), resid(&pb));
        rhos.push(pearson(&ra, &rb));
        let mut order: Vec<usize> = (0..d).collect();
        order.sort_by(|&i, &j| ra[j].partial_cmp(&ra[i]).unwrap());
        let (sy, sm): (f64, f64) = (pb.y.iter().sum(), pb.mu.iter().sum());
        let ty: f64 = order[..q].iter().map(|&c| pb.y[c]).sum();
        let tm: f64 = order[..q].iter().map(|&c| pb.mu[c]).sum();
        trs.push(if sy > 0.0 && tm > 0.0 {
            (ty / sy) / (tm / sm)
        } else {
            f64::NAN
        });
    }
    let rho = rhos.iter().sum::<f64>() / rhos.len() as f64;
    let tr = trs.iter().sum::<f64>() / trs.len() as f64;
    let tr_sd =
        (trs.iter().map(|t| (t - tr).powi(2)).sum::<f64>() / trs.len().max(1) as f64).sqrt();
    (rho, tr, tr_sd)
}

/// Split-half replicability over nsplit random balanced splits of the V draws.
///
/// Calibration (review round 2, blocking defect 2): the 40 splits reuse the same 16 draws, so the
/// split-averaged rho has a larger null sd than a class-label permutation null (which treats the
/// averaged correlation as if its terms were independent) or 1/sqrt(D-1) suggest; z built on those
/// had sd 1.4 to 1.6 under the null. Here the SE and the null mean are taken from `nperm` SIMULATED
/// null datasets: class counts drawn Poisson around each row's own null (rescaled to the row's
/// observed total), pushed through the SAME splits and the SAME code. Under that null z has sd 1
/// up to the Monte-Carlo error of the SE (about 1/sqrt(2 nperm)); a test checks this at two D.
pub fn split_stats(
    rows: &[&Row],
    pt: &PatternTable,
    aware: impl Into<NullMode> + Copy,
    seed: u64,
    nsplit: usize,
    nperm: usize,
) -> SplitStat {
    let all = pool(rows, pt, aware, true);
    let phi_all = phi(&all);
    let m0: Vec<Vec<f64>> = rows.iter().map(|r| mu_classes(r, pt, aware)).collect();
    let ys: Vec<&[f64]> = rows.iter().map(|r| r.yfrac.as_slice()).collect();
    let (rho, tr, tr_sd) = split_core(&ys, &m0, seed, nsplit);
    let mut prng = Rng(seed ^ 0xA5A5_5A5A);
    let ks: Vec<f64> = (0..rows.len())
        .map(|i| {
            let (sy, sm): (f64, f64) = (ys[i].iter().sum(), m0[i].iter().sum());
            if sm > 0.0 {
                sy / sm
            } else {
                0.0
            }
        })
        .collect();
    let mut null = vec![];
    for _ in 0..nperm {
        let sim: Vec<Vec<f64>> = (0..rows.len())
            .map(|i| m0[i].iter().map(|&u| prng.poisson(u * ks[i])).collect())
            .collect();
        let simr: Vec<&[f64]> = sim.iter().map(|v| v.as_slice()).collect();
        null.push(split_core(&simr, &m0, seed, nsplit).0);
    }
    let nm = null.iter().sum::<f64>() / null.len().max(1) as f64;
    let se = (null.iter().map(|v| (v - nm).powi(2)).sum::<f64>() / (null.len().max(2) - 1) as f64)
        .sqrt();
    SplitStat {
        rho,
        rho_se: se,
        rho_null_mean: nm,
        transfer: tr,
        transfer_sd: tr_sd,
        phi: phi_all,
        nsplit,
    }
}

/// Calibrated z of one arm: (rho - null mean) / null sd.
pub fn z_of(s: &SplitStat) -> f64 {
    if s.rho_se > 0.0 {
        (s.rho - s.rho_null_mean) / s.rho_se
    } else {
        0.0
    }
}

/// Direct frob-versus-frobperm contrast: difference of the two arms' calibrated rho over the
/// quadrature sum of their null sds (the two arms share V, so under a shared nuisance their
/// rhos are positively correlated and this z is conservative; under independent nulls it is exact).
pub fn contrast_z(f: &SplitStat, p: &SplitStat) -> f64 {
    let sd = (f.rho_se.powi(2) + p.rho_se.powi(2)).sqrt();
    if sd > 0.0 {
        ((f.rho - f.rho_null_mean) - (p.rho - p.rho_null_mean)) / sd
    } else {
        0.0
    }
}

/// Rows grouped by cell then arm.
pub type Cells = BTreeMap<(u32, u32, usize, usize), BTreeMap<String, Vec<Row>>>;

pub fn group(rows: Vec<Row>) -> Cells {
    let mut m: Cells = BTreeMap::new();
    for r in rows {
        m.entry((r.a, r.n, r.m, r.l))
            .or_default()
            .entry(r.arm.clone())
            .or_default()
            .push(r);
    }
    m
}

/// Null sd of the split-averaged rho is RHO_SD_C / sqrt(D - 1): measured by simulating the cell's
/// own analysis (40 splits of 16 draws) on Poisson null datasets at D = 12, 26, 57, 91, 140, 273,
/// 285; the product se * sqrt(D - 1) came out 0.77, 0.78, 0.71, 0.72, 0.71, 0.75, 0.70 (test
/// `z_is_calibrated_under_the_null_at_two_values_of_d` and the revision-3 log in PROTOCOL.md).
/// The earlier "1/sqrt(D - 1)" was neither this statistic's sd nor its permutation SE.
pub const RHO_SD_C: f64 = 0.73;

/// Minimum detectable split-half correlation at z = 4 and ~90% power, taking the sd under the
/// alternative equal to the sd under the null: (4 + 1.28) * RHO_SD_C / sqrt(D - 1).
pub fn rho_min(d: usize) -> f64 {
    if d < 3 {
        return f64::INFINITY;
    }
    (4.0 + 1.28) * RHO_SD_C / ((d - 1) as f64).sqrt()
}

/// Smallest D with rho_min <= 0.5: (5.28 * 0.73 / 0.5)^2 + 1 = 60.4, so D >= 61.
pub const MIN_D_FOR_RHO: usize = 61;

/// Bootstrap (over V draws, rows resampled with replacement within each arm)
/// of phi(frob) / max(phi(controls)). Returns (point, lo95, hi95).
pub fn phi_ratio_boot(
    frob: &[&Row],
    ctrls: &[Vec<&Row>],
    pt: &PatternTable,
    seed: u64,
    nboot: usize,
) -> (f64, f64, f64) {
    let ratio = |f: &[&Row], cs: &[Vec<&Row>]| {
        let pf = phi(&pool(f, pt, true, true));
        let pc = cs
            .iter()
            .map(|c| phi(&pool(c, pt, true, true)))
            .fold(f64::MIN, f64::max);
        pf / pc
    };
    let point = ratio(frob, ctrls);
    let mut rng = Rng(seed);
    let rs = |v: &[&Row], rng: &mut Rng| -> Vec<usize> {
        (0..v.len())
            .map(|_| rng.below(v.len() as u64) as usize)
            .collect()
    };
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
