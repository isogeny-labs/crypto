//! Lead P2-6(b): Non-enumerative Shared-Coefficient Remainder Tower and
//! Quotient Divisibility Extractors (`M_{A,V,R}(X) | H_B(X)^8` and
//! `M_{A,V,R}(X) | H_B(X)`), Blinded `B = 4, 8` Complete-Fibre Grader Controls,
//! and `B = {8, 12, 16, 24, 31}` Scaling + Whole-Solve ECDLP Benchmarks.

use crate::f4_fp::{solve_visit, F4Options, Ordering, Poly as F4Poly};
use crate::prime31_field::{
    fp_add, fp_mul, fp_neg, fp_sub, poly_mul, poly_rem_monic, Curve, CurveArith, Point,
};
use crate::rr9_wiedemann::{direct_monic_m, forbidden_calls, reset_forbidden_counter};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TowerMode {
    /// Frozen PR #4692 single-ideal 3-layer squaring remainder tower (`44/52/56/60/68/75` vars):
    /// `H_B = Q_0 M + R_0` (`B >= 8`), `R_{j-1}^2 = Q_j M + R_j` (`j=1..3`), `R_3 = 0`.
    FrozenCubicRemainderTower,
    /// Quadratic shared-coefficient (`m_0..m_7`) single-ideal squaring remainder tower.
    QuadraticSharedRemainderTower,
    /// Symmetric Yun squarefree-stratum quotient tower (`\prod_k G_k(X) Q(X) = H_B(X)` and
    /// `\prod_k G_k(X)^k = M_{A,V,R}(X)` over the `<= 22` universal integer partitions of 8):
    /// 100% public-polynomial (`(p, a, R, H_B)` only, `0` root extractions of `H_B`,
    /// `0` factor-base tuple enumerations) complete-fibre extractor!
    YunSymmetricQuotientTower,
    /// Symmetric 7-variable Reciprocal Resultant (`Res_X(H_B(X) - T, M_{A,V,R}(X)) == T^8`):
    /// zero auxiliary variables (`n_vars = 7`), exact `M_{A,V,R}(X) | H_B(X)^8` ideal!
    ReciprocalResultant7Var,
    /// Direct single-ideal polynomial quotient divisibility `H_B(X) = Q_0(X) * M_{A,V,R}(X)`
    /// in `7 + (B - 8) = B - 1` variables (`B >= 8`, input degree `<= 3`).
    DirectCubicQuotient,
    /// 4-variable triangularly reduced quotient divisibility (`v2, v1, v0` eliminated
    /// analytically from the top 3 equations of `H_B(X) = Q_0(X) * M_{A,V,R}(X)`, `n_vars = B - 4`).
    Reduced4VarQuotient,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TowerSystemSpec {
    pub mode: TowerMode,
    pub b_size: usize,
    pub n_strata: usize,
    pub n_vars: usize,
    pub n_equations: usize,
    pub max_input_degree: u32,
}

type SparsePoly = BTreeMap<Vec<u32>, u64>;

fn sp_zero() -> SparsePoly {
    BTreeMap::new()
}

fn sp_const(c: u64, n_vars: usize, p: u64) -> SparsePoly {
    let mut m = BTreeMap::new();
    let v = c % p;
    if v != 0 {
        m.insert(vec![0u32; n_vars], v);
    }
    m
}

fn sp_var(idx: usize, n_vars: usize) -> SparsePoly {
    let mut e = vec![0u32; n_vars];
    e[idx] = 1;
    let mut m = BTreeMap::new();
    m.insert(e, 1);
    m
}

fn sp_add_scaled(dst: &mut SparsePoly, src: &SparsePoly, scale: u64, p: u64) {
    let scale = scale % p;
    if scale == 0 {
        return;
    }
    for (e, &c) in src {
        let term = fp_mul(c, scale, p);
        if term == 0 {
            continue;
        }
        let next = fp_add(dst.get(e).copied().unwrap_or(0), term, p);
        if next == 0 {
            dst.remove(e);
        } else {
            dst.insert(e.clone(), next);
        }
    }
}

fn sp_mul(a: &SparsePoly, b: &SparsePoly, p: u64) -> SparsePoly {
    let mut out = BTreeMap::new();
    for (ea, &ca) in a {
        for (eb, &cb) in b {
            let e: Vec<u32> = ea.iter().zip(eb).map(|(&x, &y)| x + y).collect();
            let prod = fp_mul(ca, cb, p);
            if prod == 0 {
                continue;
            }
            let next = fp_add(out.get(&e).copied().unwrap_or(0), prod, p);
            if next == 0 {
                out.remove(&e);
            } else {
                out.insert(e, next);
            }
        }
    }
    out
}

fn sp_convolve(left: &[SparsePoly], right: &[SparsePoly], p: u64) -> Vec<SparsePoly> {
    let mut out = vec![sp_zero(); left.len() + right.len() - 1];
    for (i, li) in left.iter().enumerate() {
        if li.is_empty() {
            continue;
        }
        for (j, rj) in right.iter().enumerate() {
            if rj.is_empty() {
                continue;
            }
            let prod = sp_mul(li, rj, p);
            sp_add_scaled(&mut out[i + j], &prod, 1, p);
        }
    }
    out
}

fn sp_poly_pow(base: &[SparsePoly], exp: usize, n_vars: usize, p: u64) -> Vec<SparsePoly> {
    let mut acc = vec![sp_const(1, n_vars, p)];
    for _ in 0..exp {
        acc = sp_convolve(&acc, base, p);
    }
    acc
}

fn to_f4_poly(sp: &SparsePoly, ord: Ordering) -> F4Poly {
    let mut v: F4Poly = sp.iter().map(|(e, &c)| (e.clone(), c)).collect();
    v.sort_by(|(ea, _), (eb, _)| crate::groebner_f4::cmp_monomial(eb, ea, ord));
    v
}

fn build_m_expr(n_vars: usize, p: u64, curve_a: u64, r: u64, s: u64) -> Vec<SparsePoly> {
    let d0 = fp_add(fp_mul(r, r, p), curve_a, p);
    let two_s = fp_mul(2, s, p);
    let ca: [SparsePoly; 4] = std::array::from_fn(|i| sp_var(i, n_vars));
    let cv: [SparsePoly; 4] = [
        sp_var(4, n_vars),
        sp_var(5, n_vars),
        sp_var(6, n_vars),
        sp_const(1, n_vars, p),
    ];
    let d_poly = [
        sp_const(d0, n_vars, p),
        sp_const(r, n_vars, p),
        sp_const(1, n_vars, p),
    ];
    let x_minus_r = [sp_const(fp_neg(r, p), n_vars, p), sp_const(1, n_vars, p)];
    let vv = sp_convolve(&cv, &cv, p);
    let mut m_expr = sp_convolve(&d_poly, &vv, p);
    let aa = sp_convolve(&ca, &ca, p);
    let xaa = sp_convolve(&x_minus_r, &aa, p);
    let av = sp_convolve(&ca, &cv, p);
    for (i, term) in xaa.iter().enumerate() {
        sp_add_scaled(&mut m_expr[i], term, p - 1, p);
    }
    for (i, term) in av.iter().enumerate() {
        sp_add_scaled(&mut m_expr[i], term, fp_neg(two_s, p), p);
    }
    assert_eq!(m_expr.len(), 9);
    m_expr
}

/// Enumerate all universal multiplicity profiles `(d_1, ..., d_8)` of degree `8`:
/// `\sum_{k=1}^8 k * d_k == 8` with `\sum_{k=1}^8 d_k <= b_size`.
/// Depends ONLY on the integer `8` and `min(8, b_size)` (`<= 22` strata for any `B`!).
pub fn universal_degree8_multiplicity_strata(b_size: usize) -> Vec<[usize; 8]> {
    let mut out = Vec::new();
    let mut cur = [0usize; 8];
    fn rec(k: usize, rem_deg: usize, used_roots: usize, max_roots: usize, cur: &mut [usize; 8], out: &mut Vec<[usize; 8]>) {
        if rem_deg == 0 {
            out.push(*cur);
            return;
        }
        if k == 0 || k > rem_deg {
            if k > rem_deg && k > 1 {
                rec(rem_deg, rem_deg, used_roots, max_roots, cur, out);
            }
            return;
        }
        let max_d = (rem_deg / k).min(max_roots - used_roots);
        for d in (0..=max_d).rev() {
            cur[k - 1] = d;
            rec(k - 1, rem_deg - k * d, used_roots + d, max_roots, cur, out);
            cur[k - 1] = 0;
        }
    }
    rec(8, 8, 0, b_size.min(8), &mut cur, &mut out);
    out
}

/// Build one single-ideal system (`FrozenCubicRemainderTower`, `QuadraticSharedRemainderTower`,
/// `DirectCubicQuotient`, or `Reduced4VarQuotient`) from public `(p, curve_a, residual, h_b)`.
pub fn build_single_ideal_system(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    mode: TowerMode,
) -> (TowerSystemSpec, Vec<F4Poly>, bool) {
    let b_size = h_b.len() - 1;
    assert_eq!(h_b[b_size], 1, "H_B(X) must be monic");
    let (r, s) = residual;
    let ord = Ordering::Grevlex;

    match mode {
        TowerMode::DirectCubicQuotient => {
            assert!(b_size >= 8, "DirectCubicQuotient requires B >= 8");
            let q_vars = b_size - 8;
            // Variables: 0..4 = a0..a3, 4..7 = v0..v2, 7..(7+q_vars) = q_0..q_{B-9}.
            // NO redundant m_0..m_7 variables! `n_vars = B - 1` (`7` at B=8, `11` at B=12, ...).
            let n_vars = 7 + q_vars;
            let m_expr = build_m_expr(n_vars, p, curve_a, r, s);
            let mut q_sym: Vec<SparsePoly> = (0..q_vars).map(|j| sp_var(7 + j, n_vars)).collect();
            q_sym.push(sp_const(1, n_vars, p));
            let qm = sp_convolve(&q_sym, &m_expr, p);

            let mut eqs_sp = Vec::with_capacity(b_size);
            for k in 0..b_size {
                let mut eq = qm[k].clone();
                let hk = sp_const(h_b[k], n_vars, p);
                sp_add_scaled(&mut eq, &hk, p - 1, p);
                eqs_sp.push(eq);
            }
            let polys: Vec<F4Poly> = eqs_sp
                .iter()
                .filter(|sp| !sp.is_empty())
                .map(|sp| to_f4_poly(sp, ord))
                .collect();
            let max_deg = eqs_sp
                .iter()
                .flat_map(|sp| sp.keys())
                .map(|e| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            (
                TowerSystemSpec {
                    mode,
                    b_size,
                    n_strata: 1,
                    n_vars,
                    n_equations: polys.len(),
                    max_input_degree: max_deg,
                },
                polys,
                true,
            )
        }

        TowerMode::Reduced4VarQuotient => {
            assert!(b_size >= 8, "Reduced4VarQuotient requires B >= 8");
            let q_vars = b_size - 8;
            let n_vars = 4 + q_vars;
            let inv2 = (p + 1) / 2;
            let neg_inv2 = p - inv2;
            let d0 = fp_add(fp_mul(r, r, p), curve_a, p);
            let two_s = fp_mul(2, s, p);

            let ca: [SparsePoly; 4] = std::array::from_fn(|i| sp_var(i, n_vars));
            let mut q_poly_sym: Vec<SparsePoly> =
                (0..q_vars).map(|j| sp_var(4 + j, n_vars)).collect();
            q_poly_sym.push(sp_const(1, n_vars, p));

            let d_poly = [
                sp_const(d0, n_vars, p),
                sp_const(r, n_vars, p),
                sp_const(1, n_vars, p),
            ];
            let x_minus_r = [sp_const(fp_neg(r, p), n_vars, p), sp_const(1, n_vars, p)];
            let aa = sp_convolve(&ca, &ca, p);
            let xaa = sp_convolve(&x_minus_r, &aa, p);

            let mut cv = [sp_zero(), sp_zero(), sp_zero(), sp_const(1, n_vars, p)];

            let eval_qm_minus_h = |cv_cur: &[SparsePoly; 4]| -> Vec<SparsePoly> {
                let vv = sp_convolve(cv_cur, cv_cur, p);
                let mut m_cur = sp_convolve(&d_poly, &vv, p);
                let av = sp_convolve(&ca, cv_cur, p);
                for (i, term) in xaa.iter().enumerate() {
                    sp_add_scaled(&mut m_cur[i], term, p - 1, p);
                }
                for (i, term) in av.iter().enumerate() {
                    sp_add_scaled(&mut m_cur[i], term, fp_neg(two_s, p), p);
                }
                let mut qm = sp_convolve(&q_poly_sym, &m_cur, p);
                for (i, &hi) in h_b.iter().enumerate() {
                    let c = sp_const(hi, n_vars, p);
                    sp_add_scaled(&mut qm[i], &c, p - 1, p);
                }
                qm
            };

            for (step, v_idx) in [(1usize, 2usize), (2, 1), (3, 0)] {
                let qm = eval_qm_minus_h(&cv);
                let piv_zero = &qm[b_size - step];
                let mut r_v = sp_zero();
                sp_add_scaled(&mut r_v, piv_zero, neg_inv2, p);
                cv[v_idx] = r_v;
            }

            let final_qm = eval_qm_minus_h(&cv);
            let polys: Vec<F4Poly> = final_qm[..(b_size - 3)]
                .iter()
                .filter(|sp| !sp.is_empty())
                .map(|sp| to_f4_poly(sp, ord))
                .collect();
            let max_deg = final_qm[..(b_size - 3)]
                .iter()
                .flat_map(|sp| sp.keys())
                .map(|e| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            (
                TowerSystemSpec {
                    mode,
                    b_size,
                    n_strata: 1,
                    n_vars,
                    n_equations: polys.len(),
                    max_input_degree: max_deg,
                },
                polys,
                false,
            )
        }

        TowerMode::FrozenCubicRemainderTower => {
            // Exact PR #4692 frozen cubic construction (`44/52/56/60/68/75` vars):
            // - Variables: `a0..a3, v0..v2` (7 vars)
            // - For `B >= 8`: `Q_0` (`B - 8` vars) + `R_0` (`8` vars); for `B < 8`: `R_0 = H_B` (0 vars)
            // - Layer 1: `Q_1` (7 vars) + `R_1` (8 vars)
            // - Layer 2: `Q_2` (7 vars) + `R_2` (8 vars)
            // - Layer 3: `Q_3` (7 vars), `R_3 = 0`.
            // Total vars: `7 + (if B>=8 { B } else { 0 }) + 15 + 15 + 7 = 44` (B=4), `52` (B=8), `56` (B=12), `60` (B=16), `68` (B=24), `75` (B=31)!
            let mut next_var = 7usize;
            let mut alloc = |len: usize| -> usize {
                let s0 = next_var;
                next_var += len;
                s0
            };
            let q0_off = (b_size > 8).then(|| alloc(b_size - 8));
            let r0_off = (b_size >= 8).then(|| alloc(8));
            let q1_off = alloc(7);
            let r1_off = alloc(8);
            let q2_off = alloc(7);
            let r2_off = alloc(8);
            let q3_off = alloc(7);
            let n_vars = next_var;

            let m_expr = build_m_expr(n_vars, p, curve_a, r, s);
            let mut eqs_sp = Vec::new();

            let r0_sym: Vec<SparsePoly> = if b_size < 8 {
                (0..8)
                    .map(|k| sp_const(h_b.get(k).copied().unwrap_or(0), n_vars, p))
                    .collect()
            } else {
                let ro = r0_off.unwrap();
                let r0: Vec<SparsePoly> = (0..8).map(|k| sp_var(ro + k, n_vars)).collect();
                let mut q0: Vec<SparsePoly> = if let Some(qo) = q0_off {
                    (0..(b_size - 8)).map(|j| sp_var(qo + j, n_vars)).collect()
                } else {
                    Vec::new()
                };
                q0.push(sp_const(1, n_vars, p));
                let q0m = sp_convolve(&q0, &m_expr, p);
                for k in 0..b_size {
                    let mut eq = q0m[k].clone();
                    if k < 8 {
                        sp_add_scaled(&mut eq, &r0[k], 1, p);
                    }
                    let hk = sp_const(h_b[k], n_vars, p);
                    sp_add_scaled(&mut eq, &hk, p - 1, p);
                    eqs_sp.push(eq);
                }
                r0
            };

            let mut prev_r = r0_sym;
            for (qo, ro_opt) in [(q1_off, Some(r1_off)), (q2_off, Some(r2_off)), (q3_off, None)] {
                let qj: Vec<SparsePoly> = (0..7).map(|j| sp_var(qo + j, n_vars)).collect();
                let rj: Option<Vec<SparsePoly>> =
                    ro_opt.map(|ro| (0..8).map(|k| sp_var(ro + k, n_vars)).collect());
                let r_sq = sp_convolve(&prev_r, &prev_r, p);
                let qjm = sp_convolve(&qj, &m_expr, p);
                for k in 0..15 {
                    let mut eq = r_sq[k].clone();
                    sp_add_scaled(&mut eq, &qjm[k], p - 1, p);
                    if let Some(ref r_cur) = rj {
                        if k < 8 {
                            sp_add_scaled(&mut eq, &r_cur[k], p - 1, p);
                        }
                    }
                    eqs_sp.push(eq);
                }
                if let Some(r_cur) = rj {
                    prev_r = r_cur;
                }
            }

            let polys: Vec<F4Poly> = eqs_sp
                .iter()
                .filter(|sp| !sp.is_empty())
                .map(|sp| to_f4_poly(sp, ord))
                .collect();
            let max_deg = eqs_sp
                .iter()
                .flat_map(|sp| sp.keys())
                .map(|e| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            (
                TowerSystemSpec {
                    mode,
                    b_size,
                    n_strata: 1,
                    n_vars,
                    n_equations: polys.len(),
                    max_input_degree: max_deg,
                },
                polys,
                true,
            )
        }

        TowerMode::QuadraticSharedRemainderTower => {
            let mut next_var = 15usize;
            let mut alloc = |len: usize| -> usize {
                let s0 = next_var;
                next_var += len;
                s0
            };
            let q0_off = (b_size > 8).then(|| alloc(b_size - 8));
            let r0_off = (b_size >= 8).then(|| alloc(8));
            let need_l1 = b_size > 4;
            let q1_off = need_l1.then(|| alloc(7));
            let r1_off = need_l1.then(|| alloc(8));
            let q2_off = alloc(7);
            let r2_off = alloc(8);
            let q3_off = alloc(7);
            let n_vars = next_var;

            let m_expr = build_m_expr(n_vars, p, curve_a, r, s);
            let mut eqs_sp = Vec::new();
            let mut m_sym: Vec<SparsePoly> = (0..8).map(|k| sp_var(7 + k, n_vars)).collect();
            m_sym.push(sp_const(1, n_vars, p));
            for k in 0..8 {
                let mut eq = sp_var(7 + k, n_vars);
                sp_add_scaled(&mut eq, &m_expr[k], p - 1, p);
                eqs_sp.push(eq);
            }

            let r1_sym: Vec<SparsePoly> = if b_size <= 4 {
                let h_sq = poly_mul(h_b, h_b, p);
                if h_sq.len() == 9 {
                    (0..8)
                        .map(|k| {
                            let mut rk = sp_const(h_sq[k], n_vars, p);
                            sp_add_scaled(&mut rk, &m_sym[k], p - 1, p);
                            rk
                        })
                        .collect()
                } else {
                    (0..8)
                        .map(|k| sp_const(h_sq.get(k).copied().unwrap_or(0), n_vars, p))
                        .collect()
                }
            } else {
                let r0_sym: Vec<SparsePoly> = if b_size < 8 {
                    (0..8)
                        .map(|k| sp_const(h_b.get(k).copied().unwrap_or(0), n_vars, p))
                        .collect()
                } else {
                    let ro = r0_off.unwrap();
                    let r0: Vec<SparsePoly> = (0..8).map(|k| sp_var(ro + k, n_vars)).collect();
                    let mut q0: Vec<SparsePoly> = if let Some(qo) = q0_off {
                        (0..(b_size - 8)).map(|j| sp_var(qo + j, n_vars)).collect()
                    } else {
                        Vec::new()
                    };
                    q0.push(sp_const(1, n_vars, p));
                    let q0m = sp_convolve(&q0, &m_sym, p);
                    for k in 0..b_size {
                        let mut eq = q0m[k].clone();
                        if k < 8 {
                            sp_add_scaled(&mut eq, &r0[k], 1, p);
                        }
                        let hk = sp_const(h_b[k], n_vars, p);
                        sp_add_scaled(&mut eq, &hk, p - 1, p);
                        eqs_sp.push(eq);
                    }
                    r0
                };
                let qo = q1_off.unwrap();
                let ro = r1_off.unwrap();
                let q1: Vec<SparsePoly> = (0..7).map(|j| sp_var(qo + j, n_vars)).collect();
                let r1: Vec<SparsePoly> = (0..8).map(|k| sp_var(ro + k, n_vars)).collect();
                let r0_sq = sp_convolve(&r0_sym, &r0_sym, p);
                let q1m = sp_convolve(&q1, &m_sym, p);
                for k in 0..15 {
                    let mut eq = r0_sq[k].clone();
                    sp_add_scaled(&mut eq, &q1m[k], p - 1, p);
                    if k < 8 {
                        sp_add_scaled(&mut eq, &r1[k], p - 1, p);
                    }
                    eqs_sp.push(eq);
                }
                r1
            };

            for (qo, ro_opt) in [(q2_off, Some(r2_off)), (q3_off, None)] {
                let qj: Vec<SparsePoly> = (0..7).map(|j| sp_var(qo + j, n_vars)).collect();
                let rj: Option<Vec<SparsePoly>> =
                    ro_opt.map(|ro| (0..8).map(|k| sp_var(ro + k, n_vars)).collect());
                let r_sq = sp_convolve(&r1_sym, &r1_sym, p);
                // Note: for layer 3 we square r2, not r1!
                let _ = r_sq;
                let _ = qj;
                let _ = rj;
            }
            // Write layer 2 and layer 3 explicitly:
            let q2: Vec<SparsePoly> = (0..7).map(|j| sp_var(q2_off + j, n_vars)).collect();
            let r2: Vec<SparsePoly> = (0..8).map(|k| sp_var(r2_off + k, n_vars)).collect();
            let r1_sq = sp_convolve(&r1_sym, &r1_sym, p);
            let q2m = sp_convolve(&q2, &m_sym, p);
            for k in 0..15 {
                let mut eq = r1_sq[k].clone();
                sp_add_scaled(&mut eq, &q2m[k], p - 1, p);
                if k < 8 {
                    sp_add_scaled(&mut eq, &r2[k], p - 1, p);
                }
                eqs_sp.push(eq);
            }
            let q3: Vec<SparsePoly> = (0..7).map(|j| sp_var(q3_off + j, n_vars)).collect();
            let r2_sq = sp_convolve(&r2, &r2, p);
            let q3m = sp_convolve(&q3, &m_sym, p);
            for k in 0..15 {
                let mut eq = r2_sq[k].clone();
                sp_add_scaled(&mut eq, &q3m[k], p - 1, p);
                eqs_sp.push(eq);
            }

            let polys: Vec<F4Poly> = eqs_sp
                .iter()
                .filter(|sp| !sp.is_empty())
                .map(|sp| to_f4_poly(sp, ord))
                .collect();
            let max_deg = eqs_sp
                .iter()
                .flat_map(|sp| sp.keys())
                .map(|e| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            (
                TowerSystemSpec {
                    mode,
                    b_size,
                    n_strata: 1,
                    n_vars,
                    n_equations: polys.len(),
                    max_input_degree: max_deg,
                },
                polys,
                true,
            )
        }

        TowerMode::ReciprocalResultant7Var => {
            assert!(b_size == 4, "ReciprocalResultant7Var 4x4 determinant is specialized to B=4");
            let n_vars = 7usize;
            let m_expr = build_m_expr(n_vars, p, curve_a, r, s);

            // Precompute `X^e mod (H_4(X) - T)` for `e = 0..11` in basis `{1, X, X^2, X^3}`
            // where each coordinate is a `Vec<u64>` polynomial in `T` (len <= 4):
            let mut x_pow = vec![vec![vec![0u64; 4]; 4]; 12];
            x_pow[0][0][0] = 1;
            for e in 1..12 {
                // Multiply `x_pow[e-1]` by `X` modulo `X^4 - T + h_3 X^3 + h_2 X^2 + h_1 X + h_0`:
                let top = x_pow[e - 1][3].clone();
                // `X * (c0 + c1 X + c2 X^2 + c3 X^3) = c3 * (T - h0) + (c0 - c3*h1) X + (c1 - c3*h2) X^2 + (c2 - c3*h3) X^3`
                for t_deg in 0..4 {
                    let c3_t = top[t_deg];
                    if c3_t != 0 {
                        if t_deg + 1 < 4 {
                            x_pow[e][0][t_deg + 1] =
                                fp_add(x_pow[e][0][t_deg + 1], c3_t, p);
                        }
                        x_pow[e][0][t_deg] =
                            fp_sub(x_pow[e][0][t_deg], fp_mul(c3_t, h_b[0], p), p);
                    }
                    for i in 1..4 {
                        let mut val = x_pow[e - 1][i - 1][t_deg];
                        if c3_t != 0 {
                            val = fp_sub(val, fp_mul(c3_t, h_b[i], p), p);
                        }
                        x_pow[e][i][t_deg] = val;
                    }
                }
            }

            // Build `4 x 4` matrix `mat[row][col]` where each entry is a `Vec<SparsePoly>` in `T` (deg <= 2):
            let mut mat: Vec<Vec<Vec<SparsePoly>>> =
                vec![vec![vec![sp_zero(); 3]; 4]; 4];
            for col in 0..4 {
                for (k, mk) in m_expr.iter().enumerate() {
                    for row in 0..4 {
                        for t_deg in 0..3 {
                            let coeff = x_pow[k + col][row][t_deg];
                            if coeff != 0 {
                                sp_add_scaled(&mut mat[row][col][t_deg], mk, coeff, p);
                            }
                        }
                    }
                }
            }

            // Expand `4 x 4` determinant by two `2 x 2` Laplace blocks (rows 0,1 vs rows 2,3):
            let pairs: [(usize, usize, [usize; 2], bool); 6] = [
                (0, 1, [2, 3], true),
                (0, 2, [1, 3], false),
                (0, 3, [1, 2], true),
                (1, 2, [0, 3], true),
                (1, 3, [0, 2], false),
                (2, 3, [0, 1], true),
            ];
            let mut det_t = vec![sp_zero(); 9];
            for &(c0, c1, [d0_c, d1_c], pos_sign) in &pairs {
                let mut minor_top = sp_convolve(&mat[0][c0], &mat[1][c1], p);
                let top_sub = sp_convolve(&mat[0][c1], &mat[1][c0], p);
                for (t_i, term) in top_sub.iter().enumerate() {
                    sp_add_scaled(&mut minor_top[t_i], term, p - 1, p);
                }
                let mut minor_bot = sp_convolve(&mat[2][d0_c], &mat[3][d1_c], p);
                let bot_sub = sp_convolve(&mat[2][d1_c], &mat[3][d0_c], p);
                for (t_i, term) in bot_sub.iter().enumerate() {
                    sp_add_scaled(&mut minor_bot[t_i], term, p - 1, p);
                }
                let prod4 = sp_convolve(&minor_top, &minor_bot, p);
                let scale = if pos_sign { 1 } else { p - 1 };
                for (t_i, term) in prod4.iter().enumerate() {
                    if t_i < 9 {
                        sp_add_scaled(&mut det_t[t_i], term, scale, p);
                    }
                }
            }

            let polys: Vec<F4Poly> = det_t[..8]
                .iter()
                .filter(|sp| !sp.is_empty())
                .map(|sp| to_f4_poly(sp, ord))
                .collect();
            let max_deg = det_t[..8]
                .iter()
                .flat_map(|sp| sp.keys())
                .map(|e| e.iter().sum::<u32>())
                .max()
                .unwrap_or(0);
            (
                TowerSystemSpec {
                    mode,
                    b_size,
                    n_strata: 1,
                    n_vars,
                    n_equations: polys.len(),
                    max_input_degree: max_deg,
                },
                polys,
                true,
            )
        }

        TowerMode::YunSymmetricQuotientTower => {
            unreachable!("YunSymmetricQuotientTower is solved via build_yun_stratum_system")
        }
    }
}

/// Build one symmetric Yun multiplicity-stratum system for `d = [d_1, ..., d_8]`:
/// - Variables `0..7`: `a0, a1, a2, a3, v0, v1, v2`
/// - Monic squarefree-radical block polynomials `G_k(X)` of degree `d_k` for each `d_k > 0`
/// - Monic quotient `Q(X)` of degree `B - \sum d_k`
/// - Equations (purely quadratic/bilinear via binary product staging whenever needed!):
///   1) `(\prod_{k: d_k > 0} G_k(X)) * Q(X) - H_B(X) == 0`
///   2) `(\prod_{k: d_k > 0} G_k(X)^k) - M_{A,V,R}(X) == 0`
/// Uses ONLY `(p, curve_a, residual, H_B(X))` (`0` roots of `H_B`, `0` tuple enumerations!).
fn sp_poly_rem_monic(num: &[SparsePoly], den_monic: &[SparsePoly], p: u64) -> Vec<SparsePoly> {
    assert!(!den_monic.is_empty());
    let d_deg = den_monic.len() - 1;
    if num.len() <= d_deg {
        return num.to_vec();
    }
    let mut r = num.to_vec();
    for pos in (d_deg..r.len()).rev() {
        let lead = std::mem::take(&mut r[pos]);
        if lead.is_empty() {
            continue;
        }
        let shift = pos - d_deg;
        for j in 0..d_deg {
            if !den_monic[j].is_empty() {
                let prod = sp_mul(&lead, &den_monic[j], p);
                sp_add_scaled(&mut r[shift + j], &prod, p - 1, p);
            }
        }
    }
    r.truncate(d_deg);
    r
}

pub fn debug_yun_stratum(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    d_profile: &[usize; 8],
) -> (usize, usize, u32, Vec<F4Poly>) {
    build_yun_stratum_system(p, curve_a, residual, h_b, d_profile)
}

fn build_yun_stratum_system(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    d_profile: &[usize; 8],
) -> (usize, usize, u32, Vec<F4Poly>) {
    let b_size = h_b.len() - 1;
    let total_d: usize = d_profile.iter().sum();
    assert!(total_d <= b_size);
    let d1 = d_profile[0];
    let r_ge2: usize = d_profile[1..].iter().sum();
    let q_deg = if d1 > 0 { b_size - d1 - r_ge2 } else { 0 };
    let (r, s) = residual;
    let ord = Ordering::Grevlex;

    // Special case `1^8` (`r_ge2 == 0`): `M_{A,V,R}(X) * Q(X) == H_B(X)`!
    if r_ge2 == 0 {
        let (_, polys, _) =
            build_single_ideal_system(p, curve_a, residual, h_b, TowerMode::DirectCubicQuotient);
        let nv = 7 + (b_size - 8);
        let in_deg = if b_size == 8 { 2 } else { 3 };
        return (nv, polys.len(), in_deg, polys);
    }

    // G_1(X)-cancelled polynomial-remainder formulation:
    // Variables:
    //   0..7: `a0..a3, v0..v2`
    //   7..(7 + r_ge2): coefficients of `G_k(X)` for `k >= 2` (`r_ge2 <= 4` ALWAYS!)
    //   if `d1 > 0`: `(7 + r_ge2)..(7 + r_ge2 + q_deg)` for `Q(X)`!
    let use_q_cancel = d1 > 0 && q_deg < d1;
    let active_aux_deg = if use_q_cancel {
        r_ge2 + q_deg
    } else {
        r_ge2 + d1
    };
    let n_vars = 7 + active_aux_deg;
    let mut next_var = 7usize;
    let mut gk_ge2: Vec<(usize, Vec<SparsePoly>)> = Vec::new();
    for (idx, &dk) in d_profile[1..].iter().enumerate() {
        let k = idx + 2;
        if dk > 0 {
            let mut gk: Vec<SparsePoly> = (0..dk).map(|j| sp_var(next_var + j, n_vars)).collect();
            next_var += dk;
            gk.push(sp_const(1, n_vars, p));
            gk_ge2.push((k, gk));
        }
    }
    let g1_or_q_opt: Option<Vec<SparsePoly>> = if use_q_cancel {
        Some({
            let mut qp: Vec<SparsePoly> = (0..q_deg).map(|j| sp_var(next_var + j, n_vars)).collect();
            next_var += q_deg;
            qp.push(sp_const(1, n_vars, p));
            qp
        })
    } else if d1 > 0 {
        Some({
            let mut g1p: Vec<SparsePoly> = (0..d1).map(|j| sp_var(next_var + j, n_vars)).collect();
            next_var += d1;
            g1p.push(sp_const(1, n_vars, p));
            g1p
        })
    } else {
        None
    };
    assert_eq!(next_var, n_vars);

    let hb_sp: Vec<SparsePoly> = h_b.iter().map(|&c| sp_const(c, n_vars, p)).collect();
    let m_expr = build_m_expr(n_vars, p, curve_a, r, s);
    let mut eqs_sp: Vec<SparsePoly> = Vec::new();

    // `W_ge2(X) = \prod_{k >= 2} G_k(X)` (monic of degree `r_ge2 <= 4`):
    let mut w_ge2 = vec![sp_const(1, n_vars, p)];
    for (_, gk) in &gk_ge2 {
        w_ge2 = sp_convolve(&w_ge2, gk, p);
    }

    // Helper: reduce any univariate polynomial in `var_idx` modulo `H_B(-z) == 0`
    let reduce_univariate_mod_hb = |poly_vec: &mut [SparsePoly], var_idx: usize| {
        if b_size >= 8 {
            return;
        }
        // Build monic univariate `Z^{b_size} - h_{b-1} Z^{b-1} + ... + (-1)^b h_0` for `Z = g_{k,0}` (`X + Z | H_B(X) => H_B(-Z) == 0`):
        let sign_b = if b_size % 2 == 0 { 1 } else { p - 1 };
        let mut uni_den = vec![sp_zero(); b_size + 1];
        for (i, &hc) in h_b.iter().enumerate() {
            let sgn = if i % 2 == 0 { 1 } else { p - 1 };
            let coeff = fp_mul(fp_mul(hc, sgn, p), sign_b, p);
            uni_den[i] = sp_const(coeff, n_vars, p);
        }
        for coeff_sp in poly_vec.iter_mut() {
            // Only if `coeff_sp` depends purely on `var_idx`:
            let max_g = coeff_sp.keys().map(|e| e[var_idx] as usize).max().unwrap_or(0);
            if max_g >= b_size
                && coeff_sp
                    .keys()
                    .all(|e| e.iter().enumerate().all(|(vi, &ev)| vi == var_idx || ev == 0))
            {
                let mut num_g = vec![sp_zero(); max_g + 1];
                for (e, &c) in coeff_sp.iter() {
                    num_g[e[var_idx] as usize] = sp_const(c, n_vars, p);
                }
                let rem_g = sp_poly_rem_monic(&num_g, &uni_den, p);
                let mut rebuilt = sp_zero();
                for (dg, rsp) in rem_g.iter().enumerate() {
                    if let Some(&c) = rsp.values().next() {
                        let mut ex = vec![0u32; n_vars];
                        ex[var_idx] = dg as u32;
                        rebuilt.insert(ex, c);
                    }
                }
                *coeff_sp = rebuilt;
            }
        }
    };

    // Add individual `H_B(X) mod G_k(X) == 0` for EVERY `G_k` (`k >= 2`), so any degree-1 `G_k = X + g`
    // immediately injects the univariate `H_B(-g) == 0` into the root ideal!
    let mut cur_off = 7usize;
    let mut gk_var_offsets = Vec::with_capacity(gk_ge2.len());
    for (_, gk) in &gk_ge2 {
        let dk = gk.len() - 1;
        gk_var_offsets.push((cur_off, dk));
        cur_off += dk;
        if gk_ge2.len() > 1 || d1 > 0 {
            let rem_single = sp_poly_rem_monic(&hb_sp, gk, p);
            eqs_sp.extend(rem_single);
        }
    }

    if !use_q_cancel {
        // Direct `G_1(X)` formulation (`d_1 == 0` or `d_1 <= q_deg`, ZERO `Q` variables!):
        let mut w_all = w_ge2.clone();
        if let Some(ref g1_poly) = g1_or_q_opt {
            if d1 == 1 {
                eqs_sp.extend(sp_poly_rem_monic(&hb_sp, g1_poly, p));
            }
            w_all = sp_convolve(&w_all, g1_poly, p);
        }
        let rem_hb = sp_poly_rem_monic(&hb_sp, &w_all, p);
        eqs_sp.extend(rem_hb);

        let mut m_div = g1_or_q_opt.unwrap_or_else(|| vec![sp_const(1, n_vars, p)]);
        for (idx, (k, gk)) in gk_ge2.iter().enumerate() {
            let mut gk_pow = sp_poly_pow(gk, *k, n_vars, p);
            let (v_off, dk) = gk_var_offsets[idx];
            if dk == 1 {
                reduce_univariate_mod_hb(&mut gk_pow, v_off);
            }
            m_div = sp_convolve(&m_div, &gk_pow, p);
        }
        for j in 0..8 {
            let mut eq = m_div[j].clone();
            sp_add_scaled(&mut eq, &m_expr[j], p - 1, p);
            eqs_sp.push(eq);
        }
    } else {
        let q_poly = g1_or_q_opt.unwrap();
        if q_deg == 1 {
            let rem_q = sp_poly_rem_monic(&hb_sp, &q_poly, p);
            eqs_sp.extend(rem_q);
        }
        let wq = sp_convolve(&w_ge2, &q_poly, p);
        let rem_hb = sp_poly_rem_monic(&hb_sp, &wq, p);
        eqs_sp.extend(rem_hb);

        let mut mult_excess = vec![sp_const(1, n_vars, p)];
        for (idx, (k, gk)) in gk_ge2.iter().enumerate() {
            let mut gk_km1 = sp_poly_pow(gk, *k - 1, n_vars, p);
            let (v_off, dk) = gk_var_offsets[idx];
            if dk == 1 {
                reduce_univariate_mod_hb(&mut gk_km1, v_off);
            }
            mult_excess = sp_convolve(&mult_excess, &gk_km1, p);
        }
        let lhs = sp_convolve(&hb_sp, &mult_excess, p);
        let rhs = sp_convolve(&m_expr, &q_poly, p);
        assert_eq!(lhs.len(), rhs.len());
        for j in 0..(lhs.len() - 1) {
            let mut eq = lhs[j].clone();
            sp_add_scaled(&mut eq, &rhs[j], p - 1, p);
            eqs_sp.push(eq);
        }
    }

    let polys: Vec<F4Poly> = eqs_sp
        .iter()
        .filter(|sp| !sp.is_empty())
        .map(|sp| to_f4_poly(sp, ord))
        .collect();
    let max_deg = eqs_sp
        .iter()
        .flat_map(|sp| sp.keys())
        .map(|e| e.iter().sum::<u32>())
        .max()
        .unwrap_or(0);
    (n_vars, polys.len(), max_deg, polys)
}

/// Reconstruct `(v0, v1, v2)` from `(a0..a3)` and quotient `(q_0..q_{B-9})` when using
/// `Reduced4VarQuotient`.
fn reconstruct_7chart_from_4var_and_quotient(
    p: u64,
    curve_a: u64,
    r: u64,
    s: u64,
    h_b: &[u64],
    a: &[u64; 4],
    q_coeffs: &[u64],
) -> [u64; 7] {
    let b_size = h_b.len() - 1;
    let inv2 = (p + 1) / 2;
    let neg_inv2 = p - inv2;
    let d_poly = [fp_add(fp_mul(r, r, p), curve_a, p), r, 1];
    let aa = poly_mul(a, a, p);
    let xaa = poly_mul(&[fp_neg(r, p), 1], &aa, p);
    let two_s = fp_mul(2, s, p);
    let mut q_full = q_coeffs.to_vec();
    q_full.push(1);

    let mut v = [0u64, 0, 0, 1];
    for (step, v_idx) in [(1usize, 2usize), (2, 1), (3, 0)] {
        let vv = poly_mul(&v, &v, p);
        let mut m_cur = poly_mul(&d_poly, &vv, p);
        let av = poly_mul(a, &v, p);
        for (i, &t) in xaa.iter().enumerate() {
            m_cur[i] = fp_sub(m_cur[i], t, p);
        }
        for (i, &t) in av.iter().enumerate() {
            m_cur[i] = fp_sub(m_cur[i], fp_mul(two_s, t, p), p);
        }
        let qm = poly_mul(&q_full, &m_cur, p);
        let diff = fp_sub(qm[b_size - step], h_b[b_size - step], p);
        v[v_idx] = fp_mul(diff, neg_inv2, p);
    }
    [a[0], a[1], a[2], a[3], v[0], v[1], v[2]]
}

/// Direct public polynomial divisibility check `H_B(X)^8 mod M_{A,V,R}(X) == 0`.
pub fn check_m_divides_hb8(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    chart7: &[u64; 7],
) -> bool {
    let (r, s) = residual;
    let m = direct_monic_m(chart7, p, curve_a, r, s);
    let mut rem = poly_rem_monic(h_b, &m, p);
    for _ in 0..3 {
        let sq = poly_mul(&rem, &rem, p);
        rem = poly_rem_monic(&sq, &m, p);
    }
    rem.iter().all(|&c| c == 0)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TowerExtractReceipt {
    pub spec: TowerSystemSpec,
    pub status: String,
    pub complete_traversal: bool,
    pub inconsistent_unsat: bool,
    pub timed_out: bool,
    pub f4_runs: usize,
    pub f4_field_ops: u64,
    pub root_field_muls: u64,
    pub full_field_branches: u64,
    pub max_solving_degree: u32,
    pub max_matrix_rows: usize,
    pub max_matrix_cols: usize,
    pub forbidden_tuple_enumerations: u64,
    pub chart_solutions: Vec<[u64; 7]>,
    pub wall_ns: u128,
}

/// Public-only non-enumerative candidate extractor: accepts ONLY `(p, curve_a, residual, h_b)`!
pub fn extract_via_remainder_tower(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    mode: TowerMode,
    degree_cap: u32,
    timeout: Duration,
) -> Result<TowerExtractReceipt, String> {
    let t0 = Instant::now();
    let before_forbidden = forbidden_calls();
    let b_size = h_b.len() - 1;
    let (r, s) = residual;

    if mode == TowerMode::YunSymmetricQuotientTower {
        let strata = universal_degree8_multiplicity_strata(b_size);
        let deadline = Instant::now() + timeout;
        let mut charts: Vec<[u64; 7]> = Vec::new();
        let mut max_vars = 0usize;
        let mut max_eqs = 0usize;
        let mut max_in_deg = 0u32;
        let mut total_f4_runs = 0usize;
        let mut total_field_ops = 0u64;
        let mut total_root_muls = 0u64;
        let mut total_ff_branches = 0u64;
        let mut max_sdeg = 0u32;
        let mut max_rows = 0usize;
        let mut max_cols = 0usize;
        let mut all_complete = true;
        let mut any_timeout = false;

        for d_prof in &strata {
            let now = Instant::now();
            if now >= deadline {
                all_complete = false;
                any_timeout = true;
                break;
            }
            let rem_budget = deadline.saturating_duration_since(now);
            let (nv, ne, in_deg, polys) =
                build_yun_stratum_system(p, curve_a, residual, h_b, d_prof);
            max_vars = max_vars.max(nv);
            max_eqs = max_eqs.max(ne);
            max_in_deg = max_in_deg.max(in_deg);

            let opts = F4Options::new(Ordering::Grevlex, degree_cap).with_budget(rem_budget);
            let domains = vec![None; nv];
            let rep = solve_visit(&polys, nv, p, &opts, &domains, |sol| {
                let c7 = [sol[0], sol[1], sol[2], sol[3], sol[4], sol[5], sol[6]];
                if check_m_divides_hb8(p, curve_a, residual, h_b, &c7) && !charts.contains(&c7) {
                    charts.push(c7);
                }
                false // FALSE = keep traversing to collect COMPLETE fibre!
            })?;

            total_f4_runs += rep.f4_runs;
            total_field_ops += rep.field_ops;
            total_root_muls += rep.root_field_multiplications;
            total_ff_branches += rep.full_field_branches;
            max_sdeg = max_sdeg.max(rep.solving_degree);
            max_rows = max_rows.max(rep.max_rows);
            max_cols = max_cols.max(rep.max_cols);
            if !rep.complete {
                all_complete = false;
            }
            if rep.timed_out {
                any_timeout = true;
                break;
            }
        }

        charts.sort_unstable();
        let delta_forbidden = forbidden_calls() - before_forbidden;
        assert_eq!(delta_forbidden, 0, "forbidden enumeration occurred in candidate");
        let status = if all_complete {
            if charts.is_empty() {
                "complete_unsat"
            } else {
                "complete_positive"
            }
        } else if any_timeout {
            "censored_timeout"
        } else {
            "incomplete"
        }
        .to_string();

        return Ok(TowerExtractReceipt {
            spec: TowerSystemSpec {
                mode,
                b_size,
                n_strata: strata.len(),
                n_vars: max_vars,
                n_equations: max_eqs,
                max_input_degree: max_in_deg,
            },
            status,
            complete_traversal: all_complete,
            inconsistent_unsat: all_complete && charts.is_empty(),
            timed_out: any_timeout,
            f4_runs: total_f4_runs,
            f4_field_ops: total_field_ops,
            root_field_muls: total_root_muls,
            full_field_branches: total_ff_branches,
            max_solving_degree: max_sdeg,
            max_matrix_rows: max_rows,
            max_matrix_cols: max_cols,
            forbidden_tuple_enumerations: delta_forbidden,
            chart_solutions: charts,
            wall_ns: t0.elapsed().as_nanos(),
        });
    }

    let (spec, polys, direct_7chart) = build_single_ideal_system(p, curve_a, residual, h_b, mode);
    let opts = F4Options::new(Ordering::Grevlex, degree_cap).with_budget(timeout);
    let domains = vec![None; spec.n_vars];
    let mut charts: Vec<[u64; 7]> = Vec::new();

    let rep = solve_visit(&polys, spec.n_vars, p, &opts, &domains, |sol| {
        let c7 = if direct_7chart {
            [sol[0], sol[1], sol[2], sol[3], sol[4], sol[5], sol[6]]
        } else {
            let a = [sol[0], sol[1], sol[2], sol[3]];
            let q_slice = &sol[4..(4 + b_size - 8)];
            reconstruct_7chart_from_4var_and_quotient(p, curve_a, r, s, h_b, &a, q_slice)
        };
        if check_m_divides_hb8(p, curve_a, residual, h_b, &c7) && !charts.contains(&c7) {
            charts.push(c7);
        }
        false // FALSE = keep traversing to collect COMPLETE fibre!
    })?;

    charts.sort_unstable();
    let delta_forbidden = forbidden_calls() - before_forbidden;
    assert_eq!(delta_forbidden, 0, "forbidden enumeration occurred in candidate");

    let inconsistent = rep.complete && charts.is_empty();
    let status = if rep.complete {
        if charts.is_empty() {
            "complete_unsat"
        } else {
            "complete_positive"
        }
    } else if rep.timed_out {
        "censored_timeout"
    } else {
        "incomplete"
    }
    .to_string();

    Ok(TowerExtractReceipt {
        spec,
        status,
        complete_traversal: rep.complete,
        inconsistent_unsat: inconsistent,
        timed_out: rep.timed_out,
        f4_runs: rep.f4_runs,
        f4_field_ops: rep.field_ops,
        root_field_muls: rep.root_field_multiplications,
        full_field_branches: rep.full_field_branches,
        max_solving_degree: rep.solving_degree,
        max_matrix_rows: rep.max_rows,
        max_matrix_cols: rep.max_cols,
        forbidden_tuple_enumerations: delta_forbidden,
        chart_solutions: charts,
        wall_ns: t0.elapsed().as_nanos(),
    })
}

/// ===========================================================================
/// GRADER-ONLY COMPLETE-FIBRE GROUND TRUTH ORACLE (`B = 4, 8`)
/// Strictly separated from candidate paths: enumerates all `C(2B + 8 - 1, 8)`
/// signed 8-multisets ONLY inside the grader to build sealed ground-truth
/// fibre sets for blinded `B = 4, 8` verification!
/// ===========================================================================
pub mod grader_only {
    use super::*;

    pub type SignedMultiset = Vec<(usize, i8, u8)>;

    pub struct CompleteFibreOracle {
        pub b_size: usize,
        pub total_multisets: usize,
        pub by_residual: BTreeMap<(u64, u64), BTreeSet<SignedMultiset>>,
        pub distinct_abscissa_by_residual: BTreeMap<(u64, u64), BTreeSet<SignedMultiset>>,
    }

    impl CompleteFibreOracle {
        pub fn build_exhaustive(curve: Curve, base: &[(u64, u64)]) -> Self {
            let b = base.len();
            let mut ar = CurveArith::default();
            let mut signed_tokens: Vec<((usize, i8), Point)> = Vec::with_capacity(2 * b);
            for (i, &(x, y)) in base.iter().enumerate() {
                signed_tokens.push(((i, 1), Some((x, y))));
                signed_tokens.push(((i, -1), Some((x, fp_neg(y, curve.p)))));
            }

            let mut total_multisets = 0usize;
            let mut by_residual: BTreeMap<(u64, u64), BTreeSet<SignedMultiset>> = BTreeMap::new();
            let mut distinct_abscissa_by_residual: BTreeMap<(u64, u64), BTreeSet<SignedMultiset>> =
                BTreeMap::new();

            let mut cur_counts = vec![0u8; 2 * b];

            #[allow(clippy::too_many_arguments)]
            fn rec(
                idx: usize,
                rem: u8,
                cur_sum: Point,
                cur_counts: &mut [u8],
                tokens: &[((usize, i8), Point)],
                curve: Curve,
                ar: &mut CurveArith,
                total: &mut usize,
                by_res: &mut BTreeMap<(u64, u64), BTreeSet<SignedMultiset>>,
                distinct_by_res: &mut BTreeMap<(u64, u64), BTreeSet<SignedMultiset>>,
            ) {
                if rem == 0 {
                    *total += 1;
                    if let Some(res_pt) = cur_sum {
                        let mut ms: SignedMultiset = Vec::new();
                        let mut distinct_x = true;
                        let b_len = tokens.len() / 2;
                        for bi in 0..b_len {
                            let cp = cur_counts[2 * bi];
                            let cn = cur_counts[2 * bi + 1];
                            if cp + cn > 1 {
                                distinct_x = false;
                            }
                            if cp > 0 {
                                ms.push((bi, 1, cp));
                            }
                            if cn > 0 {
                                ms.push((bi, -1, cn));
                            }
                        }
                        by_res.entry(res_pt).or_default().insert(ms.clone());
                        if distinct_x {
                            distinct_by_res.entry(res_pt).or_default().insert(ms);
                        }
                    }
                    return;
                }
                if idx == tokens.len() {
                    return;
                }
                let pt = tokens[idx].1;
                let mut running = cur_sum;
                for c in 0..=rem {
                    cur_counts[idx] = c;
                    rec(
                        idx + 1,
                        rem - c,
                        running,
                        cur_counts,
                        tokens,
                        curve,
                        ar,
                        total,
                        by_res,
                        distinct_by_res,
                    );
                    if c < rem {
                        running = ar.add(curve, running, pt);
                    }
                }
                cur_counts[idx] = 0;
            }

            rec(
                0,
                8,
                None,
                &mut cur_counts,
                &signed_tokens,
                curve,
                &mut ar,
                &mut total_multisets,
                &mut by_residual,
                &mut distinct_abscissa_by_residual,
            );

            Self {
                b_size: b,
                total_multisets,
                by_residual,
                distinct_abscissa_by_residual,
            }
        }
    }
}

pub fn reset_and_assert_zero_forbidden() {
    reset_forbidden_counter();
    assert_eq!(forbidden_calls(), 0);
}
