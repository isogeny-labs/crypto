//! Design arithmetic only (closed forms; no curve is enumerated here):
//! class counts, expected yields, resolution, work estimates for the cell grid.

use crate::combi::{binom, d_nm, n_patterns};
use crate::curve::koblitz_order;
use crate::stats::{rho_min, MIN_D_FOR_RHO};

pub const NV: usize = 16; // base draws per arm
pub const NT: usize = 48; // shared targets per base draw
/// Left-table cap in tuples (28 bytes each, about 280 MB per process at the cap).
/// Raised from 4e6: at (17,5,l=3) plainbig has about 127 +/- 8 liftable x and the old cap
/// tripped at 146 (about 1 percent per draw). `plainbig_worst_left` checks mean + 6 sd.
pub const MAX_LEFT: usize = 10_000_000;
pub const NS_PER_LOOKUP: f64 = 150.0; // assumed cost of one left-tuple probe (add + hash)

#[derive(Debug, Clone)]
pub struct PlanRow {
    pub a: u32,
    pub n: u32,
    pub m: usize,
    pub l: usize,
    pub d: u128,
    pub n_pat: u128,
    pub baseline_ratio: f64,
    pub x_block: f64,
    pub e_total: f64,
    pub mu_class_half: f64,
    pub left: f64,
    pub est_sec_arm: f64,
    pub left_big: f64,
    pub left_big_worst: f64,
    pub est_sec_cell: f64,
    pub rho_min: f64,
    pub rho_resolved: bool,
}

/// Expected number of distinct liftable x in the union of n blocks of 2^l - 1 nonzero x
/// (inclusion-exclusion under independence; the Frobenius blocks overlap about as much).
fn union_nx(n: u32, l: usize) -> f64 {
    let q1 = (1u64 << n) as f64 - 1.0;
    let s = ((1u64 << l) - 1) as f64;
    let distinct = q1 * (1.0 - (1.0 - s / q1).powi(n as i32));
    distinct / 2.0
}

fn left_tuples(nx: f64, m: usize) -> f64 {
    let aa = (m + 1) / 2;
    binom(nx.round() as u64, aa as u64) as f64 * (1u64 << aa) as f64
}

/// Effective number of targets: distinct orbits of <Frobenius, negation> in hE (about
/// (|hE| - 1)/(2n); +-R share x), at most NT. n = 7 and n = 11 are affected; the runner returns
/// the exact number (review round 2: the earlier count used Frobenius orbits only and overstated
/// independent targets by up to 2x).
pub fn nt_effective(a: u32, n: u32) -> f64 {
    let orbits =
        ((koblitz_order(a, n) / crate::curve::cofactor(a)) as f64 - 1.0) / (2.0 * n as f64);
    orbits.round().max(1.0).min(NT as f64)
}

pub fn plainbig_dim(n: u32, l: usize) -> usize {
    (l + (n as f64).log2().ceil() as usize).min(n as usize - 1)
}

pub fn plan_row(a: u32, n: u32, m: usize, l: usize) -> PlanRow {
    let d = d_nm(n as u64, m as u64);
    let order = koblitz_order(a, n) as f64;
    let x_block = ((1u64 << l) - 1) as f64 / 2.0;
    let nx = union_nx(n, l);
    let e_total = binom(nx.round() as u64, m as u64) as f64 * (1u64 << m) as f64 / order;
    let nt_eff = nt_effective(a, n);
    let mu_class_half = (NV / 2) as f64 * nt_eff * e_total / d as f64;
    let left = left_tuples(nx, m);
    let passes = NV as f64 * nt_eff;
    let est_sec_arm = left * passes * NS_PER_LOOKUP * 1e-9;
    // plainbig: one subspace of dimension l + ceil(log2 n); liftable x ~ Binomial(2^d - 1, 1/2)
    let big_n = ((1u64 << plainbig_dim(n, l)) - 1) as f64;
    let (big_nx, big_sd) = (big_n / 2.0, big_n.sqrt() / 2.0);
    let left_big = left_tuples(big_nx, m);
    let left_big_worst = left_tuples(big_nx + 6.0 * big_sd, m);
    // per cell: six union arms (frob, geom, frobperm, rsub, rset, skew; skew is slightly larger,
    // ignored) + plain (negligible) + plainbig
    let est_sec_cell = (6.0 * left + left_big) * passes * NS_PER_LOOKUP * 1e-9;
    PlanRow {
        a,
        n,
        m,
        l,
        d,
        n_pat: n_patterns(n as u64, m as u64),
        baseline_ratio: crate::combi::baseline_ratio(n as u64, m as u64),
        x_block,
        e_total,
        mu_class_half,
        left,
        est_sec_arm,
        left_big,
        left_big_worst,
        est_sec_cell,
        rho_min: rho_min(d as usize),
        rho_resolved: d as usize >= MIN_D_FOR_RHO,
    }
}

/// Choose l: start at max(2, round(n/m)) (the m*l ~ n regime of the audit) and
/// raise it until the pooled per-class expectation per half is >= 5 or the
/// left table would exceed MAX_LEFT.
pub fn choose_l(a: u32, n: u32, m: usize) -> usize {
    let mut l = (((n as f64) / m as f64).round() as usize).max(2);
    loop {
        let r = plan_row(a, n, m, l);
        if r.mu_class_half >= 5.0 || l + 1 >= n as usize {
            return l;
        }
        let nxt = plan_row(a, n, m, l + 1);
        if nxt.left > MAX_LEFT as f64 {
            return l;
        }
        l += 1;
    }
}

pub fn grid() -> Vec<(u32, u32, usize)> {
    let mut g = vec![];
    for &a in &[0u32, 1] {
        for &n in &[7u32, 11, 13, 17] {
            for m in 2..=5usize {
                g.push((a, n, m));
            }
        }
    }
    g
}

/// All frozen cells (a, n, m, l): the base grid at the default l = choose_l, the fixed-l
/// column (l = 3 for every m at n = 13 and 17, so that m is not confounded with l), and the
/// l ladder at m = 4 (n = 13, 17; l = 2..5), which separates a Frobenius-specific excess of
/// relative size ~2^-l (a toy-size artefact) from one that persists. Deduplicated, sorted.
pub fn cells() -> Vec<(u32, u32, usize, usize)> {
    let mut v: Vec<(u32, u32, usize, usize)> = vec![];
    for (a, n, m) in grid() {
        v.push((a, n, m, choose_l(a, n, m)));
    }
    for &a in &[0u32, 1] {
        for &n in &[13u32, 17] {
            for m in 2..=5usize {
                v.push((a, n, m, 3));
            }
            for l in 2..=5usize {
                v.push((a, n, 4, l));
            }
        }
    }
    v.sort();
    v.dedup();
    v
}
