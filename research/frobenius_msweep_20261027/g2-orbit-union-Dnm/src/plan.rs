//! Design arithmetic only (closed forms; no curve is enumerated here):
//! class counts, expected yields, resolution, work estimates for the cell grid.

use crate::combi::{binom, d_nm, n_patterns};
use crate::curve::koblitz_order;
use crate::stats::{rho_min, MIN_D_FOR_RHO};

pub const NV: usize = 16; // base draws per arm
pub const NT: usize = 48; // shared targets per base draw
pub const MAX_LEFT: usize = 4_000_000;
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
    pub rho_min: f64,
    pub rho_resolved: bool,
}

pub fn plan_row(a: u32, n: u32, m: usize, l: usize) -> PlanRow {
    let d = d_nm(n as u64, m as u64);
    let order = koblitz_order(a, n) as f64;
    let x_block = ((1u64 << l) - 1) as f64 / 2.0;
    let nx = (n as f64 * x_block).round() as u64;
    let e_total = binom(nx, m as u64) as f64 * (1u64 << m) as f64 / order;
    let mu_class_half = (NV / 2 * NT) as f64 * e_total / d as f64;
    let aa = (m + 1) / 2;
    let left = binom(nx, aa as u64) as f64 * (1u64 << aa) as f64;
    let est_sec_arm = left * (NV * NT) as f64 * NS_PER_LOOKUP * 1e-9;
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
