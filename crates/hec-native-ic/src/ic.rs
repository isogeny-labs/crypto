//! Full-base index calculus with the factor-base walk (round five).
//!
//! Position `R = a·D₁ + b·D₂ + Σ n_j F_j`.  Each trial takes one step —
//! `+D₁` or `+D₂` with probability `1/d_step_in` each, otherwise `+F_j` for
//! one of `branches` places chosen once from the base — and asks the oracle
//! whether `R`'s `u` splits.  A split `R = Σ c_i F_i` gives the row
//! `Σ (c_i − n_i) y_i − b·k ≡ a (mod ℓ)`, with `y_i = log F_i` and
//! `k = log D₂`, logarithms to the base `D₁`.

use std::time::Instant;

use crate::fp::Fp;
use crate::instance::factor_base;
use crate::jac::{Curve, Div};
use crate::oracle::{split, OracleStats};
use crate::rng::Rng;
use crate::solve::{solve_for_last, Row, SolveStats};

#[derive(Clone, Debug)]
pub struct IcParams {
    pub branches: usize,
    pub d_step_in: u64,
    pub extra_relations: usize,
    pub max_trials: u64,
    pub seed: u64,
}

impl Default for IcParams {
    /// The BigUint engine's `RelationSearch::factor_base_walk()` with round
    /// eight's `extra_relations = 8`.
    fn default() -> Self {
        IcParams {
            branches: 64,
            d_step_in: 4,
            extra_relations: 8,
            max_trials: 50_000_000,
            seed: 20260916,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IcReport {
    pub m: usize,
    pub trials: u64,
    pub relations: u64,
    pub group_ops: u64,
    pub precompute_ops: u64,
    pub solve_rounds: u32,
    pub oracle: OracleStats,
    pub solve: SolveStats,
    pub fb_ns: u128,
    pub walk_ns: u128,
    pub oracle_ns: u128,
    pub row_ns: u128,
    pub solve_ns: u128,
    pub total_ns: u128,
    /// Verified: `k·D₁ = D₂`.
    pub k: Option<u64>,
}

impl IcReport {
    pub fn smoothness_rate(&self) -> f64 {
        self.oracle.smooth as f64 / self.trials.max(1) as f64
    }
}

pub fn solve_dlp(curve: &Curve, d1: &Div, d2: &Div, ell: u64, params: &IcParams) -> IcReport {
    let t_all = Instant::now();
    let fp: &Fp = &curve.fp;
    let fl = Fp::new(ell);
    let mut rep = IcReport::default();
    let mut rng = Rng::new(params.seed);

    // Factor base and its x-index.
    let t = Instant::now();
    let fb = factor_base(curve);
    let m = fb.len();
    rep.m = m;
    assert!(
        fp.p <= u32::MAX as u64,
        "S1 indexes the base by x in a dense table"
    );
    let mut by_x = vec![u32::MAX; fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by_x[x as usize] = j as u32;
    }
    // Distinct step places: a partial Fisher–Yates shuffle.
    let branches = params.branches.min(m).max(1);
    let mut order: Vec<u32> = (0..m as u32).collect();
    for i in 0..branches {
        let j = i + rng.below((m - i) as u64) as usize;
        order.swap(i, j);
    }
    let steps: Vec<u32> = order[..branches].to_vec();
    rep.fb_ns = t.elapsed().as_nanos();

    // Start at D₁ + D₂ (one group operation, charged to precomputation).
    let mut r = curve.add(d1, d2);
    rep.group_ops += 1;
    rep.precompute_ops += 1;
    // Rows are absolute: each carries the cumulative count of every step
    // place taken so far (≈ `branches` non-zeros).  Storing differences of
    // consecutive relations instead was tried on 2026-11-13; the rows get
    // sparser but chained, and elimination fill-in made the genus-2
    // `p = 4001` solve 4× slower.  Solver structure is track T3.
    let (mut a, mut b) = (1u64 % ell, 1u64 % ell);
    let mut taken = vec![0i64; branches];

    let mut rows: Vec<(Row, u64)> = Vec::new();
    let mut wanted = m + params.extra_relations;
    let mut scratch: Vec<(u32, i64)> = Vec::with_capacity(branches + 8);

    while rep.trials < params.max_trials {
        rep.trials += 1;
        let t = Instant::now();
        let roll = rng.below(params.d_step_in.max(2));
        r = if roll == 0 {
            a = fl.add(a, 1);
            curve.add(&r, d1)
        } else if roll == 1 {
            b = fl.add(b, 1);
            curve.add(&r, d2)
        } else {
            let s = rng.below(branches as u64) as usize;
            taken[s] += 1;
            let (x, y) = fb[steps[s] as usize];
            curve.add_place(&r, x, y)
        };
        rep.group_ops += 1;
        let t2 = Instant::now();
        rep.walk_ns += (t2 - t).as_nanos();

        let roots = split(&r.u, fp, &mut rep.oracle);
        let t3 = Instant::now();
        rep.oracle_ns += (t3 - t2).as_nanos();
        let Some(roots) = roots else {
            continue;
        };

        // Build the row.
        scratch.clear();
        for &(x, mult) in roots.as_slice() {
            let j = by_x[x as usize];
            debug_assert_ne!(j, u32::MAX, "a root of u is a point of C");
            let y = fb[j as usize].1;
            let vx = r.v.eval(x, fp);
            let sign = if vx == y { 1 } else { -1 };
            scratch.push((j, sign * mult as i64));
        }
        for (s, &n) in taken.iter().enumerate() {
            if n != 0 {
                scratch.push((steps[s], -n));
            }
        }
        scratch.sort_unstable_by_key(|e| e.0);
        let mut row: Row = Vec::with_capacity(scratch.len() + 1);
        let mut i = 0;
        while i < scratch.len() {
            let col = scratch[i].0;
            let mut s = 0i64;
            while i < scratch.len() && scratch[i].0 == col {
                s += scratch[i].1;
                i += 1;
            }
            let v = fl.from_i64(s);
            if v != 0 {
                row.push((col, v));
            }
        }
        let kc = fl.neg(b);
        if kc != 0 {
            row.push((m as u32, kc));
        }
        rep.row_ns += t3.elapsed().as_nanos();
        if row.is_empty() {
            continue; // 0 = a: impossible unless a ≡ 0; carries nothing.
        }
        rows.push((row, a));
        rep.relations += 1;

        if rows.len() >= wanted {
            let t = Instant::now();
            rep.solve_rounds += 1;
            let k = solve_for_last(&rows, m, ell, &mut rep.solve);
            rep.solve_ns += t.elapsed().as_nanos();
            if let Some(k) = k {
                if curve.mul(d1, k) == *d2 {
                    rep.k = Some(k);
                    break;
                }
            }
            // Walked rows are correlated; grow the margin and try again.
            wanted += params.extra_relations + m / 16;
        }
    }
    rep.total_ns = t_all.elapsed().as_nanos();
    rep
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::pick_instance;

    /// G4 on small rows of every genus, several seeds.
    #[test]
    fn solves_and_verifies() {
        for &(g, p) in &[(2usize, 101u64), (3, 61), (4, 31)] {
            let inst = pick_instance(p, g).unwrap();
            let k_true = inst.ell / 3 + 7;
            let d2 = inst.curve.mul(&inst.d1, k_true);
            for seed in 0..3u64 {
                let params = IcParams {
                    seed: 1000 + seed,
                    ..IcParams::default()
                };
                let rep = solve_dlp(&inst.curve, &inst.d1, &d2, inst.ell, &params);
                assert_eq!(rep.k, Some(k_true), "g={g} p={p} seed={seed}");
            }
        }
    }
}
