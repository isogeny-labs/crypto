//! Line-sieve relation source (feasibility probe; LEADS-20261114 lead 1,
//! after Sarkar–Singh, ePrint 2014/815).  Not under a frozen protocol yet.
//!
//! For a reduced `D = (u, v)` with `deg u = g` and `a_c = v + c·u` (`c ∈ F_p`),
//! `u | f − a_c²` and `w_c = (f − a_c²)/u` is monic of degree `g + 1`.  The
//! function `y − a_c` has divisor `D + E_c − (2g+1)∞`, where `E_c` is the
//! effective divisor cut out by `w_c` on the branch `y = a_c(x)`.  So
//! `[D] = −[E_c]` in the Jacobian.
//!
//! A base place `F_j = (x_j, y_j)` lies on `E_c` with sign `s` (the point
//! `(x_j, s·y_j)`) exactly when `a_c(x_j) = s·y_j`, i.e.
//! `c = (s·y_j − v(x_j)) / u(x_j)` (needs `u(x_j) ≠ 0`).  One pass over the
//! base buckets these `c`; a `c` hit at `g + 1` distinct `x_j` has
//! `w_c = Π (x − x_j)` and gives the relation
//! `log D = −Σ s_i log F_{j_i}` with no smoothness test.  Roots with
//! multiplicity, and roots at `u(x) = 0`, are missed (lost yield, never a
//! wrong relation).
//!
//! Lines are walk positions `D_t = α_t D₁ + β_t D₂`, each step adding `D₁`
//! or `D₂` (one group operation per line).  Rows use `ic.rs`'s convention:
//! `Σ c_i y_i − b·k ≡ a`, here `c_i = −s_i`, `a = α`, `b = β`.

use std::time::Instant;

use crate::fp::{op_counts, Fp, OpCounts};
use crate::instance::factor_base;
use crate::jac::{Curve, Div};
use crate::rng::Rng;
use crate::solve::{solve_for_last, Row, SolveStats, Solver};

#[derive(Clone, Debug)]
pub struct SieveParams {
    pub extra_relations: usize,
    pub max_lines: u64,
    pub solver: Solver,
    pub seed: u64,
    /// Check this many relations in the Jacobian (`D + Σ s_i F_i = 0`).
    pub verify_relations: usize,
    /// Stop taking a line's relations once the solve target is reached
    /// (otherwise a whole line is kept: up to `p/(g+1)!` extra rows).
    pub stop_mid_line: bool,
}

impl Default for SieveParams {
    fn default() -> Self {
        SieveParams {
            extra_relations: 8,
            max_lines: 10_000_000,
            solver: Solver::Fast,
            seed: 20260916,
            verify_relations: usize::MAX,
            stop_mid_line: true,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SieveReport {
    pub m: usize,
    pub lines: u64,
    /// Lines skipped because `deg u < g`.
    pub degenerate_lines: u64,
    pub relations: u64,
    /// Base points visited with `u(x) ≠ 0`, summed over lines.
    pub sieve_points: u64,
    /// Hits pushed (one per sign; one for `y = 0`).
    pub hits: u64,
    /// Counted `Fp` multiplications and inversions in the sieve (Horner,
    /// batched inversion, the `c` values), not the group operations.
    pub field_mults: u64,
    pub field_invs: u64,
    pub group_ops: u64,
    pub solve_rounds: u32,
    pub solve: SolveStats,
    pub relations_verified: u64,
    /// Building the factor base (as `IcReport::fb_ns`).
    pub fb_ns: u128,
    pub sieve_ns: u128,
    pub solve_ns: u128,
    pub total_ns: u128,
    /// Verified: `k·D₁ = D₂`.
    pub k: Option<u64>,
    /// Operation counts per phase (feature `count-ops`; zero otherwise).
    /// `ops_rel` is everything after the factor base except the solve, the
    /// `k` check and relation verification: line additions, evaluation,
    /// bucketing and row building (S4, §10).
    pub ops_fb: OpCounts,
    pub ops_rel: OpCounts,
    pub ops_solve: OpCounts,
    pub ops_kcheck: OpCounts,
    pub ops_verify: OpCounts,
}

pub fn solve_dlp_sieve(curve: &Curve, d1: &Div, d2: &Div, ell: u64, params: &SieveParams) -> SieveReport {
    let t_all = Instant::now();
    let fp: &Fp = &curve.fp;
    let fl = Fp::new(ell);
    let g = curve.g;
    let p = fp.p;
    assert!(p <= u32::MAX as u64, "the sieve buckets c in a dense table");
    let mut rep = SieveReport::default();
    let mut rng = Rng::new(params.seed);
    let ops0 = op_counts();

    let t = Instant::now();
    let fb = factor_base(curve);
    let m = fb.len();
    rep.m = m;
    rep.fb_ns = t.elapsed().as_nanos();
    let ops_after_fb = op_counts();
    rep.ops_fb = ops_after_fb.since(&ops0);

    // Bucket state, reset through `touched` after each line.
    let mut cnt = vec![0u8; p as usize];
    let mut slot = vec![u32::MAX; p as usize];
    let mut touched: Vec<u32> = Vec::with_capacity(2 * m);
    let mut hits: Vec<(u32, u32, i8)> = Vec::with_capacity(2 * m);
    // Batched inversion scratch.
    let mut idx: Vec<u32> = Vec::with_capacity(m);
    let mut uval: Vec<u64> = Vec::with_capacity(m);
    let mut vval: Vec<u64> = Vec::with_capacity(m);
    let mut pref: Vec<u64> = Vec::with_capacity(m);
    let mut groups: Vec<Vec<(u32, i8)>> = Vec::new();

    let mut r = curve.add(d1, d2);
    rep.group_ops += 1;
    let (mut a, mut b) = (1u64 % ell, 1u64 % ell);
    let mut rows: Vec<(Row, u64)> = Vec::new();
    let mut wanted = m + params.extra_relations;

    while rep.lines < params.max_lines {
        rep.lines += 1;
        if rep.lines > 1 {
            if rng.below(2) == 0 {
                a = fl.add(a, 1);
                r = curve.add(&r, d1);
            } else {
                b = fl.add(b, 1);
                r = curve.add(&r, d2);
            }
            rep.group_ops += 1;
        }
        if r.u.deg() != g as isize {
            rep.degenerate_lines += 1;
            continue;
        }
        let t = Instant::now();
        // u(x_j), v(x_j) for every base point with u(x_j) ≠ 0.
        idx.clear();
        uval.clear();
        vval.clear();
        for (j, &(x, _)) in fb.iter().enumerate() {
            let ux = r.u.eval(x, fp);
            if ux == 0 {
                continue;
            }
            idx.push(j as u32);
            uval.push(ux);
            vval.push(r.v.eval(x, fp));
        }
        let n = idx.len();
        rep.sieve_points += n as u64;
        rep.field_mults += (n * (g + r.v.deg().max(0) as usize)) as u64;
        // Batched inversion of the u(x_j) (Montgomery's trick).
        pref.clear();
        let mut acc = 1u64;
        for &ux in &uval {
            pref.push(acc);
            acc = fp.mul(acc, ux);
        }
        let mut inv = fp.inv(acc);
        rep.field_invs += 1;
        for i in (0..n).rev() {
            let ui = fp.mul(inv, pref[i]);
            inv = fp.mul(inv, uval[i]);
            uval[i] = ui; // now 1/u(x_j)
        }
        rep.field_mults += 3 * n.saturating_sub(1) as u64;
        // c = (±y − v(x)) / u(x); bucket.
        hits.clear();
        touched.clear();
        for i in 0..n {
            let j = idx[i];
            let y = fb[j as usize].1;
            let signs: &[i8] = if y == 0 { &[1] } else { &[1, -1] };
            for &s in signs {
                let sy = if s == 1 { y } else { fp.neg(y) };
                let c = fp.mul(fp.sub(sy, vval[i]), uval[i]) as u32;
                rep.field_mults += 1;
                if cnt[c as usize] == 0 {
                    touched.push(c);
                }
                cnt[c as usize] = cnt[c as usize].saturating_add(1);
                hits.push((c, j, s));
            }
        }
        rep.hits += hits.len() as u64;
        // Group the hits of every c with exactly g + 1 hits.
        groups.clear();
        for &(c, j, s) in &hits {
            if cnt[c as usize] as usize != g + 1 {
                continue;
            }
            let sl = &mut slot[c as usize];
            if *sl == u32::MAX {
                *sl = groups.len() as u32;
                groups.push(Vec::with_capacity(g + 1));
            }
            groups[*sl as usize].push((j, s));
        }
        for &c in &touched {
            cnt[c as usize] = 0;
            slot[c as usize] = u32::MAX;
        }
        rep.sieve_ns += t.elapsed().as_nanos();

        for grp in &groups {
            debug_assert_eq!(grp.len(), g + 1);
            if params.stop_mid_line && rows.len() >= wanted {
                break; // the rest of this line is not needed yet
            }
            if (rep.relations_verified as usize) < params.verify_relations {
                let o0 = op_counts();
                // D + Σ s_i F_i = 0.
                let mut e = curve.zero();
                for &(j, s) in grp {
                    let (x, y) = fb[j as usize];
                    let yy = if s == 1 { y } else { fp.neg(y) };
                    e = curve.add_place(&e, x, yy);
                }
                assert!(
                    curve.is_zero(&curve.add(&e, &r)),
                    "sieve relation fails in the Jacobian"
                );
                rep.relations_verified += 1;
                rep.ops_verify.add(&op_counts().since(&o0));
            }
            let mut row: Row = Vec::with_capacity(g + 2);
            let mut sorted = grp.clone();
            sorted.sort_unstable_by_key(|e| e.0);
            for &(j, s) in &sorted {
                let v = fl.from_i64(-(s as i64));
                if v != 0 {
                    row.push((j, v));
                }
            }
            let kc = fl.neg(b);
            if kc != 0 {
                row.push((m as u32, kc));
            }
            rows.push((row, a));
            rep.relations += 1;
        }

        if rows.len() >= wanted {
            let t = Instant::now();
            rep.solve_rounds += 1;
            let o0 = op_counts();
            let k = solve_for_last(&rows, m, ell, params.solver, &[], &mut rep.solve);
            let o1 = op_counts();
            rep.ops_solve.add(&o1.since(&o0));
            rep.solve_ns += t.elapsed().as_nanos();
            if let Some(k) = k {
                let ok = curve.mul(d1, k) == *d2;
                rep.ops_kcheck.add(&op_counts().since(&o1));
                if ok {
                    rep.k = Some(k);
                    break;
                }
            }
            wanted += params.extra_relations + m / 16;
        }
    }
    let mut not_rel = rep.ops_solve;
    not_rel.add(&rep.ops_kcheck);
    not_rel.add(&rep.ops_verify);
    rep.ops_rel = op_counts().since(&ops_after_fb).since(&not_rel);
    rep.total_ns = t_all.elapsed().as_nanos();
    rep
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::pick_instance;

    /// Every sieve relation holds in the Jacobian (asserted inside), and the
    /// planted `k` is recovered and verified, on small rows of every genus.
    #[test]
    fn sieve_solves_and_verifies() {
        for &(g, p) in &[(2usize, 101u64), (2, 503), (3, 61), (3, 211), (4, 31), (4, 61)] {
            let inst = pick_instance(p, g).unwrap();
            let k_true = inst.ell / 3 + 7;
            let d2 = inst.curve.mul(&inst.d1, k_true);
            for seed in 0..3u64 {
                let params = SieveParams {
                    seed: 1000 + seed,
                    ..SieveParams::default()
                };
                let rep = solve_dlp_sieve(&inst.curve, &inst.d1, &d2, inst.ell, &params);
                assert_eq!(rep.k, Some(k_true), "g={g} p={p} seed={seed}");
                assert_eq!(rep.relations_verified, rep.relations);
            }
        }
    }
}
