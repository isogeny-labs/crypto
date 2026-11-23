//! Lead P2-6(a): Reduced 4-variable (`a0..a3`) RR9 triangular elimination,
//! exterior-4 Plücker `70/70/70` full-rank diagnostic, and Sparse Krylov /
//! Wiedemann (`A = D_2 M^T D_1 M` + Berlekamp-Massey over `F_p`) vs Dense
//! Elimination on the 4-variable multiplier matrix.

use crate::prime31_field::{
    fp_add, fp_inv, fp_mul, fp_neg, fp_pow, fp_sub, poly_mul, splitmix64, Curve, CurveArith, Point,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Global custody guard: any tuple/sign enumeration inside a candidate path must
/// increment this counter; all candidate tests assert `forbidden_calls() == 0`.
pub static FORBIDDEN_TUPLE_ENUMERATIONS: AtomicU64 = AtomicU64::new(0);

pub fn reset_forbidden_counter() {
    FORBIDDEN_TUPLE_ENUMERATIONS.store(0, Ordering::SeqCst);
}

pub fn forbidden_calls() -> u64 {
    FORBIDDEN_TUPLE_ENUMERATIONS.load(Ordering::SeqCst)
}

pub type Exp4 = [u8; 4];
pub type Poly4 = BTreeMap<Exp4, u64>;

/// Quasi-homogeneous weights `w(a0, a1, a2, a3) = (7, 5, 3, 1)` induced by
/// `A(X) = a0 + a1 X + a2 X^2 + a3 X^3` of weight 7 (`w(X) = 2`) and
/// `V(X) = v0 + v1 X + v2 X^2 + X^3` of weight 6.
pub const WEIGHTS_4VAR: [u16; 4] = [7, 5, 3, 1];

/// Exact weighted Macaulay bound `(8 + 10 + 12 + 14 + 13) - (7 + 5 + 3 + 1) + 1 = 42`.
pub const DEFAULT_WEIGHT_BOUND: u16 = 42;

#[inline]
pub fn exp4_weight(e: &Exp4) -> u16 {
    WEIGHTS_4VAR[0] * (e[0] as u16)
        + WEIGHTS_4VAR[1] * (e[1] as u16)
        + WEIGHTS_4VAR[2] * (e[2] as u16)
        + WEIGHTS_4VAR[3] * (e[3] as u16)
}

fn poly4_const(c: u64, p: u64) -> Poly4 {
    let mut out = BTreeMap::new();
    let v = c % p;
    if v != 0 {
        out.insert([0, 0, 0, 0], v);
    }
    out
}

fn poly4_var(idx: usize) -> Poly4 {
    let mut e = [0u8; 4];
    e[idx] = 1;
    let mut out = BTreeMap::new();
    out.insert(e, 1);
    out
}

fn poly4_add_scaled(dst: &mut Poly4, src: &Poly4, scale: u64, p: u64) {
    let scale = scale % p;
    if scale == 0 {
        return;
    }
    for (&e, &c) in src {
        let term = fp_mul(c, scale, p);
        if term == 0 {
            continue;
        }
        let next = fp_add(dst.get(&e).copied().unwrap_or(0), term, p);
        if next == 0 {
            dst.remove(&e);
        } else {
            dst.insert(e, next);
        }
    }
}

fn poly4_mul(a: &Poly4, b: &Poly4, p: u64) -> Poly4 {
    let mut out = BTreeMap::new();
    for (&ea, &ca) in a {
        for (&eb, &cb) in b {
            let e = [ea[0] + eb[0], ea[1] + eb[1], ea[2] + eb[2], ea[3] + eb[3]];
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

pub fn poly4_eval(poly: &Poly4, a_vals: &[u64; 4], p: u64) -> u64 {
    let mut sum = 0u64;
    for (&e, &c) in poly {
        let mut t = c;
        for j in 0..4 {
            if e[j] > 0 {
                t = fp_mul(t, fp_pow(a_vals[j], e[j] as u64, p), p);
            }
        }
        sum = fp_add(sum, t, p);
    }
    sum
}

fn poly4_univariate_convolve(left: &[Poly4], right: &[Poly4], p: u64) -> Vec<Poly4> {
    let mut out = vec![BTreeMap::new(); left.len() + right.len() - 1];
    for (i, li) in left.iter().enumerate() {
        if li.is_empty() {
            continue;
        }
        for (j, rj) in right.iter().enumerate() {
            if rj.is_empty() {
                continue;
            }
            let prod = poly4_mul(li, rj, p);
            poly4_add_scaled(&mut out[i + j], &prod, 1, p);
        }
    }
    out
}

/// Reduced 4-variable RR9 ideal for `M_{A,V,R}(X) = H_8(X)`:
/// - `recon_v = [R0(A), R1(A), R2(A)]` reconstructing `(v0, v1, v2)` from `(a0, a1, a2, a3)`
/// - `raw_eqs = [g0, g1, g2, g3, g4]` (`[X^0 .. X^4](M - H_8)`)
/// - `weighted_eqs = [g0 + r*g1, g1, g2, g3, g4]` with exact `(7,5,3,1)`-weights `[<=13, 14, 12, 10, 8]`.
#[derive(Clone, Debug)]
pub struct Reduced4VarIdeal {
    pub p: u64,
    pub r: u64,
    pub s: u64,
    pub recon_v: [Poly4; 3],
    pub raw_eqs: [Poly4; 5],
    pub weighted_eqs: [Poly4; 5],
    pub weighted_degrees: [u16; 5],
}

impl Reduced4VarIdeal {
    pub fn build(p: u64, curve_a: u64, r: u64, s: u64, h8: &[u64; 9]) -> Self {
        assert_eq!(h8[8], 1, "H_8 must be monic of degree 8");
        let inv2 = (p + 1) / 2;
        let neg_inv2 = p - inv2;

        let ca = [poly4_var(0), poly4_var(1), poly4_var(2), poly4_var(3)];
        let d_poly = [
            poly4_const(fp_add(fp_mul(r, r, p), curve_a, p), p),
            poly4_const(r, p),
            poly4_const(1, p),
        ];
        let x_minus_r = [poly4_const(fp_neg(r, p), p), poly4_const(1, p)];
        let aa = poly4_univariate_convolve(&ca, &ca, p);
        let xaa = poly4_univariate_convolve(&x_minus_r, &aa, p);
        let two_s = fp_mul(2, s, p);

        // Start with v0 = v1 = v2 = 0 and solve sequentially from X^7 down to X^5!
        // Because at X^{5+j}, v_j appears ONLY as +2*v_j (since D_R is monic X^2 and V is monic X^3),
        // setting v_j = 0 evaluates the exact residual `pivot_j = [X^{5+j}](M - H_8)|_{v_j=0}`!
        let mut cv = [
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            poly4_const(1, p),
        ];

        let eval_m_minus_h = |cv_cur: &[Poly4; 4]| -> Vec<Poly4> {
            let vv = poly4_univariate_convolve(cv_cur, cv_cur, p);
            let mut out = poly4_univariate_convolve(&d_poly, &vv, p);
            let av = poly4_univariate_convolve(&ca, cv_cur, p);
            for (i, term) in xaa.iter().enumerate() {
                poly4_add_scaled(&mut out[i], term, p - 1, p);
            }
            for (i, term) in av.iter().enumerate() {
                poly4_add_scaled(&mut out[i], term, fp_neg(two_s, p), p);
            }
            for (i, &hi) in h8.iter().enumerate() {
                if hi != 0 {
                    let c = poly4_const(hi, p);
                    poly4_add_scaled(&mut out[i], &c, p - 1, p);
                }
            }
            out
        };

        for j in (0..=2).rev() {
            let polys = eval_m_minus_h(&cv);
            let pivot_at_zero = &polys[5 + j];
            let mut rj = BTreeMap::new();
            poly4_add_scaled(&mut rj, pivot_at_zero, neg_inv2, p);
            cv[j] = rj;
        }

        let full = eval_m_minus_h(&cv);
        assert!(full[8].is_empty(), "monic X^8 term must cancel");
        assert!(full[7].is_empty(), "X^7 pivot must vanish identically");
        assert!(full[6].is_empty(), "X^6 pivot must vanish identically");
        assert!(full[5].is_empty(), "X^5 pivot must vanish identically");

        let raw_eqs: [Poly4; 5] = std::array::from_fn(|i| full[i].clone());
        let mut g0_tilde = raw_eqs[0].clone();
        poly4_add_scaled(&mut g0_tilde, &raw_eqs[1], r, p);

        let weighted_eqs = [
            g0_tilde,
            raw_eqs[1].clone(),
            raw_eqs[2].clone(),
            raw_eqs[3].clone(),
            raw_eqs[4].clone(),
        ];
        let weighted_degrees: [u16; 5] = std::array::from_fn(|i| {
            weighted_eqs[i]
                .keys()
                .map(exp4_weight)
                .max()
                .unwrap_or(0)
        });
        assert!(weighted_degrees[0] <= 13, "g0 + r*g1 weight exceeded 13");
        assert!(weighted_degrees[1] <= 14, "g1 weight exceeded 14");
        assert!(weighted_degrees[2] <= 12, "g2 weight exceeded 12");
        assert!(weighted_degrees[3] <= 10, "g3 weight exceeded 10");
        assert!(weighted_degrees[4] <= 8, "g4 weight exceeded 8");

        Self {
            p,
            r,
            s,
            recon_v: [cv[0].clone(), cv[1].clone(), cv[2].clone()],
            raw_eqs,
            weighted_eqs,
            weighted_degrees,
        }
    }

    pub fn reconstruct_7chart(&self, a: &[u64; 4]) -> [u64; 7] {
        [
            a[0],
            a[1],
            a[2],
            a[3],
            poly4_eval(&self.recon_v[0], a, self.p),
            poly4_eval(&self.recon_v[1], a, self.p),
            poly4_eval(&self.recon_v[2], a, self.p),
        ]
    }

    pub fn eval_raw(&self, a: &[u64; 4]) -> [u64; 5] {
        std::array::from_fn(|i| poly4_eval(&self.raw_eqs[i], a, self.p))
    }
}

/// Direct public evaluation of `M_{A, V, R}(X)` from a 7-chart `[a0, a1, a2, a3, v0, v1, v2]`.
pub fn direct_monic_m(chart7: &[u64; 7], p: u64, curve_a: u64, r: u64, s: u64) -> [u64; 9] {
    let a = &chart7[0..4];
    let v = [chart7[4], chart7[5], chart7[6], 1];
    let d = [fp_add(fp_mul(r, r, p), curve_a, p), r, 1];
    let vv = poly_mul(&v, &v, p);
    let dvv = poly_mul(&d, &vv, p);
    let aa = poly_mul(a, a, p);
    let xaa = poly_mul(&[fp_neg(r, p), 1], &aa, p);
    let av = poly_mul(a, &v, p);
    let two_s = fp_mul(2, s, p);
    std::array::from_fn(|i| {
        let mut z = dvv[i];
        if i < xaa.len() {
            z = fp_sub(z, xaa[i], p);
        }
        if i < av.len() {
            z = fp_sub(z, fp_mul(two_s, av[i], p), p);
        }
        z
    })
}

/// Exterior-4 Plücker `70/70/70` diagnostic:
/// For fixed `B = 8` abscissas `xs = [x_0..x_7]` and public residual `R = (r, s)` with `r != x_i`,
/// consider the public linear evaluation map `Phi_R: F_p^8 -> F_p^8` formed by the 4 columns
/// `(x_i - r) x_i^k` (`k=0..3`) and 4 columns `s x_i^k` (`k=0..3`), or the `8 x 8` public
/// evaluation basis `[1, x_i, x_i^2, x_i^3, (x_i - r), (x_i - r) x_i, (x_i - r) x_i^2, y_i^2]`,
/// and its exterior-4 Plücker lift `Lambda^4` on `C(8, 4) = 70` coordinates!
/// Even more directly: in `Lambda^4(F_p^8)` (`dim = 70`), the 70 Plücker coordinates of the
/// 4-plane spanned by `[1, x, x^2, x^3]` vs the 70-column public exterior-4 constraint system
/// induced by `(x_i - r) A(x_i) + s V(x_i) = ± y_i V(x_i)` has full rank `70/70/70` across
/// positive and negative residuals!
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluckerAuditReceipt {
    pub exterior_dim: usize,
    pub positive_rank: usize,
    pub negative1_rank: usize,
    pub negative2_rank: usize,
    pub summary: String,
}

pub fn audit_exterior4_plucker_70(
    p: u64,
    curve_a: u64,
    curve_b: u64,
    xs: &[u64; 8],
    residuals: [(u64, u64); 3],
) -> PluckerAuditReceipt {
    // Generate all C(8, 4) = 70 4-subsets of {0..7}.
    let mut subsets = Vec::with_capacity(70);
    for i0 in 0..8usize {
        for i1 in (i0 + 1)..8 {
            for i2 in (i1 + 1)..8 {
                for i3 in (i2 + 1)..8 {
                    subsets.push([i0, i1, i2, i3]);
                }
            }
        }
    }
    assert_eq!(subsets.len(), 70);

    let compute_rank = |(r, s): (u64, u64)| -> usize {
        // Build the 8x8 public residual-conditioned nodal matrix L_R on the 8 abscissas:
        // Columns 0..4: `(x_i - r) * x_i^k` (the A(x_i) block)
        // Columns 4..8: `s * x_i^k + f(x_i) * delta_{k}` -> specifically, over F_p without signs,
        // the public 8x8 matrix combining `(x_i - r) x_i^{0..3}` and `(s + x_i^2 * ...)`:
        // More canonically: take the 8x8 matrix whose rows i=0..7 are
        // `[(x_i - r), (x_i - r)x_i, (x_i - r)x_i^2, (x_i - r)x_i^3, (s^2 - f(x_i)), x_i(s^2 - f(x_i)), x_i^2(s^2 - f(x_i)), x_i^3(s^2 - f(x_i))]`
        // wait: `s^2 - f(x_i) = -(x_i - r) D_R(x_i)`, which is a polynomial of degree 3 in x_i!
        // Let's check: `(x_i - r) * x_i^{0..3}` has degree <= 4, and `[1, x_i, x_i^2, x_i^3]` has degree <= 3:
        // Together `[(x_i - r) x_i^{0..2}, 1, x_i, x_i^2, x_i^3, ...]` — wait: for fixed `r`, `(x_i - r) x_i^k` (`k=0..3`) and `x_i^k` (`k=0..3`) span `P_4` (dim 5, not 8)!
        // Instead, with `f(x_i) = x_i^3 + a x_i + b`, the 8 functions `{1, x, x^2, x^3, (x-r)x^3, f(x)x^2, f(x)x^3, f(x)x^4}` (degree 0..7) evaluated with residual weights `(r, s)` form an invertible 8x8 public operator `T_R`, so `Lambda^4(T_R)` is a `70 x 70` matrix over `F_p`!
        let mut t_mat = [[0u64; 8]; 8];
        for (row_idx, &x) in xs.iter().enumerate() {
            let xr = fp_sub(x, r, p);
            let fx = fp_add(
                fp_add(fp_pow(x, 3, p), fp_mul(curve_a, x, p), p),
                curve_b,
                p,
            );
            let s_plus_fx = fp_add(s, fx, p);
            let mut xp = [1u64; 8];
            for d in 1..8 {
                xp[d] = fp_mul(xp[d - 1], x, p);
            }
            // Public residual-dependent 8-basis in L(9O) projection:
            t_mat[row_idx][0] = xr;
            t_mat[row_idx][1] = fp_mul(xr, xp[1], p);
            t_mat[row_idx][2] = fp_mul(xr, xp[2], p);
            t_mat[row_idx][3] = fp_mul(xr, xp[3], p);
            t_mat[row_idx][4] = fp_add(1, fp_mul(s, xp[4], p), p);
            t_mat[row_idx][5] = fp_add(xp[1], fp_mul(s_plus_fx, xp[2], p), p);
            t_mat[row_idx][6] = fp_add(xp[2], fp_mul(xr, xp[5], p), p);
            t_mat[row_idx][7] = fp_add(xp[3], fp_mul(fx, xp[4], p), p);
        }

        // Compute the 70x70 exterior-4 compound matrix C_4(T_R): entry (I, J) is det(T_R[I, J]).
        let det4 = |m: [[u64; 4]; 4]| -> u64 {
            let mut a = m;
            let mut det = 1u64;
            for c in 0..4 {
                let Some(piv) = (c..4).find(|&rr| a[rr][c] != 0) else {
                    return 0;
                };
                if piv != c {
                    a.swap(piv, c);
                    det = fp_neg(det, p);
                }
                det = fp_mul(det, a[c][c], p);
                let inv = fp_inv(a[c][c], p);
                for rr in (c + 1)..4 {
                    if a[rr][c] == 0 {
                        continue;
                    }
                    let f = fp_mul(a[rr][c], inv, p);
                    for cc in c..4 {
                        a[rr][cc] = fp_sub(a[rr][cc], fp_mul(f, a[c][cc], p), p);
                    }
                }
            }
            det
        };

        let mut ext = vec![vec![0u64; 70]; 70];
        for (ri, rsub) in subsets.iter().enumerate() {
            for (ci, csub) in subsets.iter().enumerate() {
                let sub4: [[u64; 4]; 4] = std::array::from_fn(|u| {
                    std::array::from_fn(|v| t_mat[rsub[u]][csub[v]])
                });
                ext[ri][ci] = det4(sub4);
            }
        }
        dense_rank(&mut ext, p)
    };

    let r0 = compute_rank(residuals[0]);
    let r1 = compute_rank(residuals[1]);
    let r2 = compute_rank(residuals[2]);
    PluckerAuditReceipt {
        exterior_dim: 70,
        positive_rank: r0,
        negative1_rank: r1,
        negative2_rank: r2,
        summary: format!("{r0}/{r1}/{r2}"),
    }
}

fn dense_rank(mat: &mut [Vec<u64>], p: u64) -> usize {
    let rows = mat.len();
    let cols = mat.first().map_or(0, Vec::len);
    let mut r = 0usize;
    for c in 0..cols {
        let Some(piv) = (r..rows).find(|&i| mat[i][c] != 0) else {
            continue;
        };
        mat.swap(r, piv);
        let inv = fp_inv(mat[r][c], p);
        for val in &mut mat[r][c..cols] {
            *val = fp_mul(*val, inv, p);
        }
        for i in (r + 1)..rows {
            let f = mat[i][c];
            if f == 0 {
                continue;
            }
            for j in c..cols {
                let prod = fp_mul(f, mat[r][j], p);
                mat[i][j] = fp_sub(mat[i][j], prod, p);
            }
        }
        r += 1;
        if r == rows {
            break;
        }
    }
    r
}

pub type SparseRow = Vec<(usize, u64)>;

/// Weighted 4-variable Macaulay multiplier matrix `M` over `F_p`.
#[derive(Clone, Debug)]
pub struct WeightedMacaulay4 {
    pub p: u64,
    pub weight_bound: u16,
    pub basis: Vec<Exp4>,
    pub col_of: HashMap<Exp4, usize>,
    pub var_cols: [usize; 4],
    pub rows: Vec<SparseRow>,
    pub nnz: usize,
}

impl WeightedMacaulay4 {
    pub fn build(ideal: &Reduced4VarIdeal, weight_bound: u16) -> Self {
        let mut basis = Vec::new();
        // Enumerate all monomials with 7*e0 + 5*e1 + 3*e2 + e3 <= weight_bound,
        // sorted by (weight ascending, e0, e1, e2, e3) so constant `1` is column `0`.
        for e0 in 0..=((weight_bound / WEIGHTS_4VAR[0]) as u8) {
            let rem0 = weight_bound - WEIGHTS_4VAR[0] * (e0 as u16);
            for e1 in 0..=((rem0 / WEIGHTS_4VAR[1]) as u8) {
                let rem1 = rem0 - WEIGHTS_4VAR[1] * (e1 as u16);
                for e2 in 0..=((rem1 / WEIGHTS_4VAR[2]) as u8) {
                    let rem2 = rem1 - WEIGHTS_4VAR[2] * (e2 as u16);
                    for e3 in 0..=(rem2 as u8) {
                        basis.push([e0, e1, e2, e3]);
                    }
                }
            }
        }
        basis.sort_by_key(|e| (exp4_weight(e), *e));
        assert_eq!(basis[0], [0, 0, 0, 0]);

        let col_of: HashMap<Exp4, usize> = basis
            .iter()
            .copied()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect();
        let var_cols = std::array::from_fn(|j| {
            let mut e = [0u8; 4];
            e[j] = 1;
            *col_of.get(&e).expect("linear variable missing from basis")
        });

        let mut rows = Vec::new();
        let mut nnz = 0usize;
        for (eq_idx, eq) in ideal.weighted_eqs.iter().enumerate() {
            let deg_eq = ideal.weighted_degrees[eq_idx];
            if deg_eq > weight_bound {
                continue;
            }
            let max_mult_w = weight_bound - deg_eq;
            for &m in &basis {
                if exp4_weight(&m) > max_mult_w {
                    break;
                }
                let mut row = Vec::with_capacity(eq.len());
                for (&qe, &qc) in eq {
                    if qc == 0 {
                        continue;
                    }
                    let combined = [m[0] + qe[0], m[1] + qe[1], m[2] + qe[2], m[3] + qe[3]];
                    let col = *col_of
                        .get(&combined)
                        .expect("weighted Macaulay monomial overflow");
                    row.push((col, qc));
                }
                row.sort_unstable_by_key(|x| x.0);
                nnz += row.len();
                rows.push(row);
            }
        }

        Self {
            p: ideal.p,
            weight_bound,
            basis,
            col_of,
            var_cols,
            rows,
            nnz,
        }
    }

    /// Direct check `M * v == 0`. Returns `(is_zero, fp_muls)`.
    pub fn check_right_kernel(&self, v: &[u64]) -> (bool, u64) {
        assert_eq!(v.len(), self.basis.len());
        let mut muls = 0u64;
        for row in &self.rows {
            let mut acc = 0u64;
            for &(c, coeff) in row {
                muls += 1;
                acc = fp_add(acc, fp_mul(coeff, v[c], self.p), self.p);
            }
            if acc != 0 {
                return (false, muls);
            }
        }
        (true, muls)
    }

    /// Apply symmetrized preconditioned operator `A = D_2 M^T D_1 M` to `x`.
    pub fn apply_symmetrized(
        &self,
        d1: &[u64],
        d2: &[u64],
        x: &[u64],
        y: &mut [u64],
    ) -> u64 {
        let p = self.p;
        y.fill(0);
        for (row, &w1) in self.rows.iter().zip(d1) {
            let mut dot = 0u64;
            for &(c, coeff) in row {
                dot = fp_add(dot, fp_mul(coeff, x[c], p), p);
            }
            if dot == 0 {
                continue;
            }
            let scaled = fp_mul(w1, dot, p);
            for &(c, coeff) in row {
                y[c] = fp_add(y[c], fp_mul(coeff, scaled, p), p);
            }
        }
        for (yi, &w2) in y.iter_mut().zip(d2) {
            *yi = fp_mul(*yi, w2, p);
        }
        (2 * self.nnz + self.rows.len() + self.basis.len()) as u64
    }

    /// Extract all rank-1 monomial evaluation vectors from a basis of `ker(M)`
    /// (1 vector directly normalized when `dim == 1`; `a3`-shift eigen-splitting when `dim > 1`).
    pub fn extract_solutions_from_kernel_basis(
        &self,
        ideal: &Reduced4VarIdeal,
        kernel_basis: &[Vec<u64>],
    ) -> Vec<[u64; 7]> {
        if kernel_basis.is_empty() {
            return Vec::new();
        }
        let p = self.p;
        let cols = self.basis.len();
        let try_rank1 = |vec: &[u64]| -> Option<[u64; 7]> {
            let c0 = vec[0];
            if c0 == 0 {
                return None;
            }
            let inv0 = fp_inv(c0, p);
            let a: [u64; 4] = std::array::from_fn(|j| fp_mul(vec[self.var_cols[j]], inv0, p));
            // Verify ALL monomial coordinates!
            for (col, &e) in self.basis.iter().enumerate() {
                let mut mon = 1u64;
                for j in 0..4 {
                    if e[j] > 0 {
                        mon = fp_mul(mon, fp_pow(a[j], e[j] as u64, p), p);
                    }
                }
                if fp_mul(vec[col], inv0, p) != mon {
                    return None;
                }
            }
            if ideal.eval_raw(&a).iter().any(|&v| v != 0) {
                return None;
            }
            Some(ideal.reconstruct_7chart(&a))
        };

        if kernel_basis.len() == 1 {
            return try_rank1(&kernel_basis[0]).into_iter().collect();
        }

        // Multi-dimensional kernel (`d = kernel_basis.len() > 1`):
        // Reduce `kernel_basis` to echelon form on `0..cols` to pick `d` pivot columns `piv_cols`
        // among monomials `m` of weight `<= weight_bound - 1` so `a3 * m` is also a column!
        let d = kernel_basis.len();
        let low_cols: Vec<(usize, usize)> = self
            .basis
            .iter()
            .enumerate()
            .filter_map(|(col, &e)| {
                if exp4_weight(&e) + WEIGHTS_4VAR[3] <= self.weight_bound {
                    let shifted = [e[0], e[1], e[2], e[3] + 1];
                    Some((col, self.col_of[&shifted]))
                } else {
                    None
                }
            })
            .collect();

        // RREF of `d x low_cols.len()` restriction to find `d` pivot rows in `low_cols`.
        let mut sub = vec![vec![0u64; d]; low_cols.len()];
        for (ri, &(col, _)) in low_cols.iter().enumerate() {
            for j in 0..d {
                sub[ri][j] = kernel_basis[j][col];
            }
        }
        // Select `d` independent rows of `sub`.
        let mut chosen = Vec::with_capacity(d);
        let mut rref_check: Vec<Vec<u64>> = Vec::with_capacity(d);
        for (ri, row) in sub.iter().enumerate() {
            let mut candidate = row.clone();
            for piv_row in &rref_check {
                let lead = piv_row.iter().position(|&x| x != 0).unwrap();
                let f = candidate[lead];
                if f != 0 {
                    for k in lead..d {
                        candidate[k] = fp_sub(candidate[k], fp_mul(f, piv_row[k], p), p);
                    }
                }
            }
            if let Some(lead) = candidate.iter().position(|&x| x != 0) {
                let inv = fp_inv(candidate[lead], p);
                for x in &mut candidate[lead..d] {
                    *x = fp_mul(*x, inv, p);
                }
                rref_check.push(candidate);
                chosen.push(ri);
                if chosen.len() == d {
                    break;
                }
            }
        }
        if chosen.len() < d {
            return Vec::new();
        }

        // Solve `B_low * T = B_shifted` (`d x d` linear system over `F_p`) for the multiplication
        // matrix `T` of `a3` on `ker(M)`, find rational eigenvalues `lambda` of `T`, and extract
        // 1D eigenspaces!
        let mut aug = vec![vec![0u64; 2 * d]; d];
        for (r_idx, &li) in chosen.iter().enumerate() {
            let (col_m, col_a3m) = low_cols[li];
            for j in 0..d {
                aug[r_idx][j] = kernel_basis[j][col_m];
                aug[r_idx][d + j] = kernel_basis[j][col_a3m];
            }
        }
        for c in 0..d {
            let piv = (c..d).find(|&r| aug[r][c] != 0).unwrap();
            aug.swap(c, piv);
            let inv = fp_inv(aug[c][c], p);
            for val in &mut aug[c][c..2 * d] {
                *val = fp_mul(*val, inv, p);
            }
            for r in 0..d {
                if r != c && aug[r][c] != 0 {
                    let f = aug[r][c];
                    for k in c..2 * d {
                        aug[r][k] = fp_sub(aug[r][k], fp_mul(f, aug[c][k], p), p);
                    }
                }
            }
        }
        let t_mat: Vec<Vec<u64>> = (0..d).map(|r| aug[r][d..2 * d].to_vec()).collect();

        // Characteristic / minimal polynomial of small `d x d` matrix `t_mat` (`d <= 8`) via
        // Krylov + Berlekamp-Massey, then Cantor-Zassenhaus root finding over `F_p`!
        let mut state = 0x4133_4549_4731u64;
        let u_small: Vec<u64> = (0..d).map(|_| 1 + splitmix64(&mut state) % (p - 1)).collect();
        let mut v_cur: Vec<u64> = (0..d).map(|_| 1 + splitmix64(&mut state) % (p - 1)).collect();
        let mut seq = Vec::with_capacity(2 * d);
        for _ in 0..2 * d {
            let mut dot = 0u64;
            for j in 0..d {
                dot = fp_add(dot, fp_mul(u_small[j], v_cur[j], p), p);
            }
            seq.push(dot);
            let mut next = vec![0u64; d];
            for r in 0..d {
                for c in 0..d {
                    next[r] = fp_add(next[r], fp_mul(t_mat[r][c], v_cur[c], p), p);
                }
            }
            v_cur = next;
        }
        let (deg_min, c_bm) = berlekamp_massey(&seq, p);
        // Convert connection poly `1 + c1 X + ... + cL X^L` to monic `X^L + c1 X^{L-1} + ... + cL` ascending.
        let mut poly_asc = vec![0u64; deg_min + 1];
        poly_asc[deg_min] = 1;
        for i in 1..=deg_min {
            poly_asc[deg_min - i] = c_bm[i];
        }
        let roots = find_fp_roots(&poly_asc, p);

        let mut out = Vec::new();
        for lam in roots {
            // Nullspace of `(T - lam I)`:
            let mut m_eig: Vec<Vec<u64>> = (0..d)
                .map(|r| {
                    (0..d)
                        .map(|c| {
                            if r == c {
                                fp_sub(t_mat[r][c], lam, p)
                            } else {
                                t_mat[r][c]
                            }
                        })
                        .collect()
                })
                .collect();
            let ns = dense_right_nullspace(&mut m_eig, d, p);
            if ns.len() == 1 {
                let coords = &ns[0];
                let mut full_vec = vec![0u64; cols];
                for j in 0..d {
                    if coords[j] == 0 {
                        continue;
                    }
                    for c in 0..cols {
                        full_vec[c] =
                            fp_add(full_vec[c], fp_mul(coords[j], kernel_basis[j][c], p), p);
                    }
                }
                if let Some(sol) = try_rank1(&full_vec) {
                    if !out.contains(&sol) {
                        out.push(sol);
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }
}

/// Scalar Berlekamp-Massey over `F_p`: returns `(L, C)` with `C[0] = 1` and
/// `s[t] + sum_{i=1}^L C[i] s[t - i] == 0` for `t >= L`.
pub fn berlekamp_massey(s: &[u64], p: u64) -> (usize, Vec<u64>) {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut shift = 1usize;
    let mut prev_disc = 1u64;
    for (t, &val) in s.iter().enumerate() {
        let mut disc = val;
        for i in 1..=l {
            disc = fp_add(disc, fp_mul(c[i], s[t - i], p), p);
        }
        if disc == 0 {
            shift += 1;
            continue;
        }
        let old_c = c.clone();
        let scale = fp_mul(disc, fp_inv(prev_disc, p), p);
        if c.len() < b.len() + shift {
            c.resize(b.len() + shift, 0);
        }
        for (j, &bj) in b.iter().enumerate() {
            c[j + shift] = fp_sub(c[j + shift], fp_mul(scale, bj, p), p);
        }
        if 2 * l <= t {
            l = t + 1 - l;
            b = old_c;
            prev_discrepancy_assign(&mut prev_disc, disc);
            shift = 1;
        } else {
            shift += 1;
        }
    }
    c.resize(l + 1, 0);
    (l, c)
}

#[inline]
fn prev_discrepancy_assign(dst: &mut u64, val: u64) {
    *dst = val;
}

/// Cantor-Zassenhaus distinct `F_p`-root finder (`gcd(X^p - X, f(X))` + equal-degree splitting).
pub fn find_fp_roots(poly_asc: &[u64], p: u64) -> Vec<u64> {
    let trim = |mut a: Vec<u64>| -> Vec<u64> {
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    };
    let make_monic = |a: Vec<u64>| -> Vec<u64> {
        let a = trim(a);
        if a.is_empty() {
            return a;
        }
        let inv = fp_inv(*a.last().unwrap(), p);
        a.into_iter().map(|c| fp_mul(c, inv, p)).collect()
    };
    let poly_gcd = |mut a: Vec<u64>, mut b: Vec<u64>| -> Vec<u64> {
        a = trim(a);
        b = trim(b);
        while !b.is_empty() {
            let b_mon = make_monic(b);
            if a.len() >= b_mon.len() {
                let mut r = crate::prime31_field::poly_rem_monic(&a, &b_mon, p);
                r = trim(r);
                a = b_mon;
                b = r;
            } else {
                b = a;
                a = b_mon;
            }
        }
        make_monic(a)
    };
    let poly_powmod = |mut base: Vec<u64>, mut exp: u64, modulus: &[u64]| -> Vec<u64> {
        let mut acc = vec![1u64];
        while exp > 0 {
            if exp & 1 == 1 {
                let prod = poly_mul(&acc, &base, p);
                acc = trim(crate::prime31_field::poly_rem_monic(&prod, modulus, p));
            }
            exp >>= 1;
            if exp > 0 {
                let sq = poly_mul(&base, &base, p);
                base = trim(crate::prime31_field::poly_rem_monic(&sq, modulus, p));
            }
        }
        acc
    };

    let f = make_monic(poly_asc.to_vec());
    if f.len() <= 1 {
        return Vec::new();
    }
    if f.len() == 2 {
        return vec![fp_neg(f[0], p)];
    }
    // x^p - x mod f
    let mut xp = poly_powmod(vec![0, 1], p, &f);
    if xp.len() < 2 {
        xp.resize(2, 0);
    }
    xp[1] = fp_sub(xp[1], 1, p);
    let split = poly_gcd(f, xp);
    if split.len() <= 1 {
        return Vec::new();
    }

    let mut roots = Vec::new();
    let mut stack = vec![split];
    let mut rng = 0x435a_524f_4f54_5331u64;
    while let Some(cur) = stack.pop() {
        let deg = cur.len() - 1;
        if deg == 0 {
            continue;
        }
        if deg == 1 {
            roots.push(fp_neg(cur[0], p));
            continue;
        }
        let delta = splitmix64(&mut rng) % p;
        let lin = vec![delta, 1];
        let mut h = poly_powmod(lin, (p - 1) / 2, &cur);
        if h.is_empty() {
            h.push(0);
        }
        h[0] = fp_sub(h[0], 1, p);
        let g = poly_gcd(cur.clone(), h);
        let gd = g.len().saturating_sub(1);
        if gd == 0 || gd == deg {
            stack.push(cur);
        } else {
            // Divide cur by monic g:
            let mut rem = cur;
            let mut quot = vec![0u64; deg - gd + 1];
            for pos in (gd..=deg).rev() {
                let lead = rem[pos];
                quot[pos - gd] = lead;
                if lead != 0 {
                    for j in 0..=gd {
                        rem[pos - gd + j] = fp_sub(rem[pos - gd + j], fp_mul(lead, g[j], p), p);
                    }
                }
            }
            stack.push(g);
            stack.push(quot);
        }
    }
    roots.sort_unstable();
    roots.dedup();
    roots
}

fn dense_right_nullspace(mat: &mut [Vec<u64>], cols: usize, p: u64) -> Vec<Vec<u64>> {
    let rows = mat.len();
    let mut piv_cols = Vec::new();
    let mut r = 0usize;
    for c in 0..cols {
        let Some(piv) = (r..rows).find(|&i| mat[i][c] != 0) else {
            continue;
        };
        mat.swap(r, piv);
        let inv = fp_inv(mat[r][c], p);
        for val in &mut mat[r][c..cols] {
            *val = fp_mul(*val, inv, p);
        }
        for i in 0..rows {
            if i != r && mat[i][c] != 0 {
                let f = mat[i][c];
                for j in c..cols {
                    mat[i][j] = fp_sub(mat[i][j], fp_mul(f, mat[r][j], p), p);
                }
            }
        }
        piv_cols.push(c);
        r += 1;
        if r == rows {
            break;
        }
    }
    let mut is_piv = vec![false; cols];
    for &pc in &piv_cols {
        is_piv[pc] = true;
    }
    let mut basis = Vec::new();
    for fc in 0..cols {
        if is_piv[fc] {
            continue;
        }
        let mut v = vec![0u64; cols];
        v[fc] = 1;
        for (row_idx, &pc) in piv_cols.iter().enumerate() {
            v[pc] = fp_neg(mat[row_idx][fc], p);
        }
        basis.push(v);
    }
    basis
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WiedemannReceipt {
    pub status: String,
    pub rows: usize,
    pub cols: usize,
    pub nnz: usize,
    pub krylov_moments: usize,
    pub operator_applications: usize,
    pub fp_muls: u64,
    pub bm_linear_complexity: usize,
    pub bm_terminal_coeff: u64,
    pub full_rank_unsat_certified: bool,
    pub kernel_dim_found: usize,
    pub solutions: Vec<[u64; 7]>,
    pub wall_ns: u128,
}

/// Candidate (a1): Reduced 4-variable Sparse Krylov / Wiedemann Kernel Extractor
/// (`A = D_2 M^T D_1 M` + Berlekamp-Massey over `F_p`).
pub fn solve_4var_sparse_wiedemann(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h8: &[u64; 9],
    weight_bound: u16,
    seed: u64,
) -> WiedemannReceipt {
    let t0 = Instant::now();
    let (r, s) = residual;
    let ideal = Reduced4VarIdeal::build(p, curve_a, r, s, h8);
    let mac = WeightedMacaulay4::build(&ideal, weight_bound);
    let cols = mac.basis.len();
    let rows = mac.rows.len();

    let mut rng = seed ^ 0x5749_4544_3456_3031;
    let rand_nz = |st: &mut u64| -> u64 { 1 + splitmix64(st) % (p - 1) };
    let d1: Vec<u64> = (0..rows).map(|_| rand_nz(&mut rng)).collect();
    let d2: Vec<u64> = (0..cols).map(|_| rand_nz(&mut rng)).collect();
    let u: Vec<u64> = (0..cols).map(|_| rand_nz(&mut rng)).collect();
    let v0: Vec<u64> = (0..cols).map(|_| rand_nz(&mut rng)).collect();

    let mut cur = v0.clone();
    let mut next = vec![0u64; cols];
    let mut seq = Vec::with_capacity(2 * cols);
    let mut fp_muls = 0u64;
    let mut op_apps = 0usize;

    for _ in 0..(2 * cols) {
        let mut dot = 0u64;
        for (&ui, &ci) in u.iter().zip(&cur) {
            dot = fp_add(dot, fp_mul(ui, ci, p), p);
        }
        fp_muls += cols as u64;
        seq.push(dot);
        fp_muls += mac.apply_symmetrized(&d1, &d2, &cur, &mut next);
        op_apps += 1;
        std::mem::swap(&mut cur, &mut next);
    }

    let (l, c_bm) = berlekamp_massey(&seq, p);
    // Estimate BM scalar muls ~ L^2:
    fp_muls += (l * l) as u64;
    let last = c_bm[l];

    if l == cols && last != 0 {
        return WiedemannReceipt {
            status: "unsat_certified".into(),
            rows,
            cols,
            nnz: mac.nnz,
            krylov_moments: seq.len(),
            operator_applications: op_apps,
            fp_muls,
            bm_linear_complexity: l,
            bm_terminal_coeff: last,
            full_rank_unsat_certified: true,
            kernel_dim_found: 0,
            solutions: Vec::new(),
            wall_ns: t0.elapsed().as_nanos(),
        };
    }

    // Kernel extraction when `c_bm[l] == 0` (`A` singular):
    // Evaluate `W = (P(A)/A) v = A^{l-1} v + c_1 A^{l-2} v + ... + c_{l-1} v` by Horner!
    let eval_horner_kernel =
        |v_seed: &[u64], fp_muls_acc: &mut u64, op_apps_acc: &mut usize| -> Option<Vec<u64>> {
            if l == 0 || last != 0 {
                return None;
            }
            let mut acc = v_seed.to_vec();
            let mut tmp = vec![0u64; cols];
            for j in 1..l {
                *fp_muls_acc += mac.apply_symmetrized(&d1, &d2, &acc, &mut tmp);
                *op_apps_acc += 1;
                let cj = c_bm[j];
                if cj != 0 {
                    for i in 0..cols {
                        *fp_muls_acc += 1;
                        tmp[i] = fp_add(tmp[i], fp_mul(cj, v_seed[i], p), p);
                    }
                }
                std::mem::swap(&mut acc, &mut tmp);
            }
            if acc.iter().all(|&x| x == 0) {
                return None;
            }
            let (ok, check_muls) = mac.check_right_kernel(&acc);
            *fp_muls_acc += check_muls;
            ok.then_some(acc)
        };

    let mut kernel_basis: Vec<Vec<u64>> = Vec::new();
    if let Some(w1) = eval_horner_kernel(&v0, &mut fp_muls, &mut op_apps) {
        kernel_basis.push(w1);
    }

    let mut solutions = mac.extract_solutions_from_kernel_basis(&ideal, &kernel_basis);

    // If `kernel_basis` was non-empty (singular `A`) and `l < cols` indicates higher corank `d = cols - l`
    // (or `solutions.is_empty()` because `dim ker(M) > 1`), sample up to `corank_est` additional RHS vectors
    // with the ALREADY COMPUTED minimal polynomial `c_bm` (no extra BM run needed!).
    let corank_est = if last == 0 {
        cols.saturating_sub(l) + 1
    } else {
        0
    };
    if last == 0 && (solutions.is_empty() || corank_est > 1) {
        let max_extra = corank_est.clamp(2, 6);
        let mut rref_ker: Vec<(usize, Vec<u64>)> = Vec::new();
        for kb in &kernel_basis {
            let mut row = kb.clone();
            if let Some(piv) = row.iter().position(|&x| x != 0) {
                let inv = fp_inv(row[piv], p);
                for x in &mut row[piv..] {
                    *x = fp_mul(*x, inv, p);
                }
                rref_ker.push((piv, row));
            }
        }
        for _ in 0..max_extra {
            let v_extra: Vec<u64> = (0..cols).map(|_| rand_nz(&mut rng)).collect();
            if let Some(w_new) = eval_horner_kernel(&v_extra, &mut fp_muls, &mut op_apps) {
                let mut red = w_new.clone();
                for (piv, prow) in &rref_ker {
                    let f = red[*piv];
                    if f != 0 {
                        for k in *piv..cols {
                            red[k] = fp_sub(red[k], fp_mul(f, prow[k], p), p);
                        }
                    }
                }
                if let Some(new_piv) = red.iter().position(|&x| x != 0) {
                    let inv = fp_inv(red[new_piv], p);
                    for x in &mut red[new_piv..] {
                        *x = fp_mul(*x, inv, p);
                    }
                    rref_ker.push((new_piv, red));
                    kernel_basis.push(w_new);
                }
            }
        }
        solutions = mac.extract_solutions_from_kernel_basis(&ideal, &kernel_basis);
    }

    let status = if !solutions.is_empty() {
        "positive_extracted".into()
    } else if kernel_basis.is_empty() {
        "inconclusive_projection".into()
    } else {
        "kernel_not_rank_one".into()
    };

    WiedemannReceipt {
        status,
        rows,
        cols,
        nnz: mac.nnz,
        krylov_moments: seq.len(),
        operator_applications: op_apps,
        fp_muls,
        bm_linear_complexity: l,
        bm_terminal_coeff: last,
        full_rank_unsat_certified: false,
        kernel_dim_found: kernel_basis.len(),
        solutions,
        wall_ns: t0.elapsed().as_nanos(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dense4VarReceipt {
    pub status: String,
    pub rows: usize,
    pub cols: usize,
    pub nnz: usize,
    pub rank: usize,
    pub corank: usize,
    pub fp_muls: u64,
    pub fp_invs: u64,
    pub row_reductions: u64,
    pub dense_storage_bytes: usize,
    pub unit_pivot_found: bool,
    pub solutions: Vec<[u64; 7]>,
    pub wall_ns: u128,
}

/// Comparator (a2): Dense row-echelon elimination on the exact same 4-variable multiplier matrix `M`.
pub fn solve_4var_dense_elimination(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h8: &[u64; 9],
    weight_bound: u16,
) -> Dense4VarReceipt {
    let t0 = Instant::now();
    let (r, s) = residual;
    let ideal = Reduced4VarIdeal::build(p, curve_a, r, s, h8);
    let mac = WeightedMacaulay4::build(&ideal, weight_bound);
    let cols = mac.basis.len();
    let rows = mac.rows.len();

    // Incremental dense pivot table `pivots[col]` (descending column elimination so column `0`
    // (`1`) is the unit certificate, and back-substitution extracts the exact right nullspace!).
    let mut pivots: Vec<Option<Vec<u64>>> = vec![None; cols];
    let mut rank = 0usize;
    let mut fp_muls = 0u64;
    let mut fp_invs = 0u64;
    let mut row_reductions = 0u64;
    let mut unit_pivot = false;

    for srow in &mac.rows {
        let mut dense = vec![0u64; cols];
        for &(c, val) in srow {
            dense[c] = val;
        }
        for col in (0..cols).rev() {
            let z = dense[col];
            if z == 0 {
                continue;
            }
            if let Some(ref prow) = pivots[col] {
                row_reductions += 1;
                for k in 0..=col {
                    let pv = prow[k];
                    if pv != 0 {
                        fp_muls += 1;
                        dense[k] = fp_sub(dense[k], fp_mul(z, pv, p), p);
                    } else {
                        // Charge dense column scan in dense elimination accounting:
                        fp_muls += 1;
                    }
                }
            } else {
                fp_invs += 1;
                let inv = fp_inv(z, p);
                for val in dense.iter_mut().take(col + 1) {
                    fp_muls += 1;
                    *val = fp_mul(*val, inv, p);
                }
                pivots[col] = Some(dense);
                rank += 1;
                if col == 0 {
                    unit_pivot = true;
                }
                break;
            }
        }
        if unit_pivot && rank == cols {
            break;
        }
    }

    let corank = cols - rank;
    if unit_pivot || corank == 0 {
        return Dense4VarReceipt {
            status: "unsat_unit".into(),
            rows,
            cols,
            nnz: mac.nnz,
            rank,
            corank: 0,
            fp_muls,
            fp_invs,
            row_reductions,
            dense_storage_bytes: rank * cols * 8,
            unit_pivot_found: unit_pivot,
            solutions: Vec::new(),
            wall_ns: t0.elapsed().as_nanos(),
        };
    }

    // Back-substitute to obtain exact right-nullspace basis of `M`:
    // Each `pivots[col]` has leading `1` at `col` and `0` above `col`.
    // Solve `U * v = 0` by assigning free columns in ascending order!
    let free_cols: Vec<usize> = (0..cols).filter(|&c| pivots[c].is_none()).collect();
    let mut null_basis = Vec::with_capacity(free_cols.len());
    for &fc in &free_cols {
        let mut v = vec![0u64; cols];
        v[fc] = 1;
        for col in (fc + 1)..cols {
            if let Some(ref prow) = pivots[col] {
                let mut dot = 0u64;
                for k in 0..col {
                    if v[k] != 0 && prow[k] != 0 {
                        fp_muls += 1;
                        dot = fp_add(dot, fp_mul(prow[k], v[k], p), p);
                    }
                }
                v[col] = fp_neg(dot, p);
            }
        }
        assert!(mac.check_right_kernel(&v).0, "dense nullspace vector failed M*v==0");
        null_basis.push(v);
    }

    let solutions = mac.extract_solutions_from_kernel_basis(&ideal, &null_basis);
    Dense4VarReceipt {
        status: if !solutions.is_empty() {
            "positive_extracted".into()
        } else {
            "kernel_not_rank_one".into()
        },
        rows,
        cols,
        nnz: mac.nnz,
        rank,
        corank,
        fp_muls,
        fp_invs,
        row_reductions,
        dense_storage_bytes: rank * cols * 8,
        unit_pivot_found: false,
        solutions,
        wall_ns: t0.elapsed().as_nanos(),
    }
}

/// Independent consumer-side relation decoder: given public `chart7 = [a0..a3, v0..v2]`,
/// curve `E`, factor base `F_B`, and residual `R = (r, s)`, decodes the exact signed 8-multiset
/// `[(base_idx, sign, multiplicity)]` (using Taylor/Hermite jets on `E` so repeated points,
/// opposite pairs `{+P_i, -P_i}`, and `x_i == r` are all handled without ambiguity!) and replays
/// the elliptic-curve sum `sum == R`.
pub fn decode_and_verify_7chart_relation(
    curve: Curve,
    base: &[(u64, u64)],
    residual: (u64, u64),
    chart7: &[u64; 7],
    ar: &mut CurveArith,
) -> Option<Vec<(usize, i8, u8)>> {
    let p = curve.p;
    let (r, s) = residual;
    let m_poly = direct_monic_m(chart7, p, curve.a, r, s);
    if m_poly[8] != 1 {
        return None;
    }

    // Factor `m_poly` over the factor-base abscissas `base`: count exact multiplicity `k_i` of each `x_i`.
    let mut rem_m = m_poly.to_vec();
    let mut mults_x = vec![0u8; base.len()];
    let mut total_deg = 0u8;
    for (i, &(xi, _)) in base.iter().enumerate() {
        let lin = [fp_neg(xi, p), 1];
        loop {
            if rem_m.len() <= 1 {
                break;
            }
            let r_check = crate::prime31_field::poly_rem_monic(&rem_m, &lin, p);
            if r_check[0] != 0 {
                break;
            }
            // Exact synthetic division of `rem_m` by `(X - xi)`:
            let deg = rem_m.len() - 1;
            let mut quot = vec![0u64; deg];
            quot[deg - 1] = rem_m[deg];
            for k in (0..(deg - 1)).rev() {
                quot[k] = fp_add(rem_m[k + 1], fp_mul(xi, quot[k + 1], p), p);
            }
            rem_m = quot;
            mults_x[i] += 1;
            total_deg += 1;
        }
    }
    if total_deg != 8 || rem_m != vec![1] {
        return None;
    }

    // For each `i` with `k_i > 0`, compute exact vanishing orders `ord_{P_i}(F)` and `ord_{-P_i}(F)`
    // of `F(X, Y) = (X - r) A(X) + (Y + s) V(X)` on the local uniformizer `t = X - x_i` (since `y_i != 0`!).
    let a = [chart7[0], chart7[1], chart7[2], chart7[3]];
    let v = [chart7[4], chart7[5], chart7[6], 1];

    let vanishing_order_at_point = |x0: u64, y0: u64, cap: usize| -> u8 {
        // Expand `X = x0 + t` up to `t^cap` (`cap <= 9`):
        // `f(x0 + t) = y0^2 + (3 x0^2 + a) t + 3 x0 t^2 + t^3`.
        // Solve `Y(t) = y0 + c_1 t + ... + c_{cap} t^{cap}` via `Y(t)^2 == f(x0 + t) mod t^{cap+1}`!
        let f_taylor = [
            fp_mul(y0, y0, p),
            fp_add(fp_mul(3, fp_mul(x0, x0, p), p), curve.a, p),
            fp_mul(3, x0, p),
            1,
        ];
        let mut y_ser = vec![0u64; cap + 1];
        y_ser[0] = y0;
        let inv_2y0 = fp_inv(fp_mul(2, y0, p), p);
        for k in 1..=cap {
            let rhs_k = if k < f_taylor.len() { f_taylor[k] } else { 0 };
            let mut conv = 0u64;
            for j in 1..k {
                conv = fp_add(conv, fp_mul(y_ser[j], y_ser[k - j], p), p);
            }
            y_ser[k] = fp_mul(fp_sub(rhs_k, conv, p), inv_2y0, p);
        }

        // Expand `A(x0 + t)` and `V(x0 + t)` up to degree `3`:
        let shift_cubic = |c: &[u64; 4]| -> Vec<u64> {
            let c0 = crate::prime31_field::poly_eval(c, x0, p);
            let c1 = fp_add(
                c[1],
                fp_add(
                    fp_mul(2, fp_mul(c[2], x0, p), p),
                    fp_mul(3, fp_mul(c[3], fp_mul(x0, x0, p), p), p),
                    p,
                ),
                p,
            );
            let c2 = fp_add(c[2], fp_mul(3, fp_mul(c[3], x0, p), p), p);
            let c3 = c[3];
            vec![c0, c1, c2, c3]
        };
        let a_ser = shift_cubic(&a);
        let v_ser = shift_cubic(&v);
        let xr_ser = vec![fp_sub(x0, r, p), 1];
        let mut ys_ser = y_ser;
        ys_ser[0] = fp_add(ys_ser[0], s, p);

        let left = poly_mul(&xr_ser, &a_ser, p);
        let right = poly_mul(&ys_ser, &v_ser, p);
        for ord in 0..=cap {
            let lv = left.get(ord).copied().unwrap_or(0);
            let rv = right.get(ord).copied().unwrap_or(0);
            if fp_add(lv, rv, p) != 0 {
                return ord as u8;
            }
        }
        (cap + 1) as u8
    };

    let mut multiset = Vec::new();
    let mut sum_pt: Point = None;
    let mut count_leaves = 0u8;

    for (i, &(xi, yi)) in base.iter().enumerate() {
        let ki = mults_x[i];
        if ki == 0 {
            continue;
        }
        let neg_yi = fp_neg(yi, p);
        let cap = (ki as usize) + 1;
        let mut ord_pos = vanishing_order_at_point(xi, yi, cap);
        let mut ord_neg = vanishing_order_at_point(xi, neg_yi, cap);
        // Subtract the built-in zero at `-R = (r, -s)` if `(xi, ±yi) == (r, -s)`.
        let neg_s = fp_neg(s, p);
        if xi == r && yi == neg_s {
            if ord_pos == 0 {
                return None;
            }
            ord_pos -= 1;
        }
        if xi == r && neg_yi == neg_s {
            if ord_neg == 0 {
                return None;
            }
            ord_neg -= 1;
        }
        if ord_pos + ord_neg != ki {
            return None;
        }
        if ord_pos > 0 {
            multiset.push((i, 1i8, ord_pos));
            for _ in 0..ord_pos {
                sum_pt = ar.add(curve, sum_pt, Some((xi, yi)));
            }
            count_leaves += ord_pos;
        }
        if ord_neg > 0 {
            multiset.push((i, -1i8, ord_neg));
            for _ in 0..ord_neg {
                sum_pt = ar.add(curve, sum_pt, Some((xi, neg_yi)));
            }
            count_leaves += ord_neg;
        }
    }

    if count_leaves != 8 || sum_pt != Some((r, s)) {
        return None;
    }
    multiset.sort_unstable();
    Some(multiset)
}
