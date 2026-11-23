//! Report section 3: exact Krylov solvers over `F_p`.
//!
//! * [`berlekamp_massey`] and scalar Wiedemann ([`wiedemann_solve`],
//!   [`wiedemann_kernel`]): `O(N Z + N^2)` for a successful solve, `O(Z + N)`
//!   storage.
//! * Block Wiedemann ([`block_wiedemann_kernel`], [`block_wiedemann_solve`]):
//!   `O(N/b)` sequential block rounds of `O(Z b)` products, a matrix generator
//!   by the iterative order-basis ("M-Basis", Giorgi-Jeannerod-Villard /
//!   Beckermann-Labahn) algorithm, which is the quadratic generator stage, not
//!   Thome's subquadratic one.
//! * Finite-field Lanczos ([`lanczos_solve`]): `O(N Z + N^2)` on the
//!   symmetrized operator `A^T D A` with a random diagonal preconditioner;
//!   self-orthogonal breakdowns trigger a retry with a fresh `D`.
//! * [`sparse_rank`], [`sparse_left_kernel`], [`sparse_inverse`]: structured
//!   elimination followed by dense algebra on the residual. The report's
//!   Casacuberta-Kyng `O(N^2.2131)` algorithm is a different method; it is not
//!   what is implemented here.
//!
//! Every returned solution or kernel vector is checked against the original
//! operator before it is returned; failures retry with fresh randomness and
//! finally return `None`. Counters record matrix-vector products.

use super::dense::DenseMatrix;
use super::field::{Fp, SplitMix64};
use super::sparse::{structured_elimination, EliminationOptions, SparseMatrix, SparseVec};

/// A square linear operator `F_p^N -> F_p^N`.
pub trait LinearOperator {
    fn dim(&self) -> usize;
    fn apply(&self, f: Fp, x: &[u64]) -> Vec<u64>;
}

impl LinearOperator for SparseMatrix {
    fn dim(&self) -> usize {
        assert_eq!(self.nrows, self.ncols, "operator must be square");
        self.nrows
    }
    fn apply(&self, f: Fp, x: &[u64]) -> Vec<u64> {
        self.mul_vec(f, x)
    }
}

/// `[[A, -b], [0, 0]]`: kernel vectors with last coordinate `t != 0` give
/// `A (x/t) = b`. Used to turn a kernel method into a solver.
pub struct Augmented<'a, O: LinearOperator> {
    pub a: &'a O,
    pub b: &'a [u64],
}

impl<O: LinearOperator> LinearOperator for Augmented<'_, O> {
    fn dim(&self) -> usize {
        self.a.dim() + 1
    }
    fn apply(&self, f: Fp, x: &[u64]) -> Vec<u64> {
        let n = self.a.dim();
        let mut y = self.a.apply(f, &x[..n]);
        let t = x[n];
        if t != 0 {
            for (yi, &bi) in y.iter_mut().zip(self.b) {
                *yi = f.sub(*yi, f.mul(t, bi));
            }
        }
        y.push(0);
        y
    }
}

/// Berlekamp-Massey: shortest connection polynomial `C` (with `C[0] = 1`)
/// and linear complexity `L` such that `sum_{i=0}^{L} C[i] s[n-i] = 0` for
/// all `L <= n < s.len()`.
pub fn berlekamp_massey(f: Fp, s: &[u64]) -> (Vec<u64>, usize) {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bd = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            d = f.mul_add(d, c[i], s[n - i]);
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = f.neg(f.mul(d, f.inv(bd).unwrap()));
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bi) in b.iter().enumerate() {
            c[i + m] = f.mul_add(c[i + m], coef, bi);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            bd = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    c.resize(l + 1, 0);
    (c, l)
}

/// Product counter shared by the solvers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KrylovStats {
    pub matvecs: u64,
    pub attempts: u32,
}

fn random_vec(f: Fp, n: usize, rng: &mut SplitMix64) -> Vec<u64> {
    (0..n).map(|_| rng.element(f)).collect()
}

fn is_zero(v: &[u64]) -> bool {
    v.iter().all(|&x| x == 0)
}

/// Scalar Wiedemann for a nonsingular square system `A x = b`.
pub fn wiedemann_solve<O: LinearOperator>(
    f: Fp,
    a: &O,
    b: &[u64],
    seed: u64,
    max_attempts: u32,
) -> (Option<Vec<u64>>, KrylovStats) {
    let n = a.dim();
    let mut rng = SplitMix64::new(seed);
    let mut st = KrylovStats::default();
    if is_zero(b) {
        return (Some(vec![0; n]), st);
    }
    for _ in 0..max_attempts {
        st.attempts += 1;
        let u = random_vec(f, n, &mut rng);
        let mut seq = Vec::with_capacity(2 * n);
        let mut v = b.to_vec();
        for _ in 0..2 * n {
            seq.push(f.dot(&u, &v));
            v = a.apply(f, &v);
            st.matvecs += 1;
        }
        let (c, l) = berlekamp_massey(f, &seq);
        if l == 0 || c[l] == 0 {
            continue;
        }
        // f(lambda) = sum_i c[i] lambda^{L-i};  x = -(1/c_L) sum_{i<L} c[i] A^{L-1-i} b.
        let mut x = vec![0u64; n];
        // Horner: x = (((c0) A + c1) A + ...) b  over i = 0..L-1.
        for i in 0..l {
            if i > 0 {
                x = a.apply(f, &x);
                st.matvecs += 1;
            }
            for (xj, &bj) in x.iter_mut().zip(b) {
                *xj = f.mul_add(*xj, c[i], bj);
            }
        }
        let s = f.neg(f.inv(c[l]).unwrap());
        for xj in x.iter_mut() {
            *xj = f.mul(*xj, s);
        }
        st.matvecs += 1;
        if a.apply(f, &x) == b {
            return (Some(x), st);
        }
    }
    (None, st)
}

/// Scalar Wiedemann kernel vector of a singular square operator: compute the
/// projected minimal polynomial `lambda^k g(lambda)` of a random start `z`,
/// form `w = g(A) z` and return the last nonzero vector of `w, Aw, A^2 w, ...`.
pub fn wiedemann_kernel<O: LinearOperator>(
    f: Fp,
    a: &O,
    seed: u64,
    max_attempts: u32,
) -> (Option<Vec<u64>>, KrylovStats) {
    let n = a.dim();
    let mut rng = SplitMix64::new(seed);
    let mut st = KrylovStats::default();
    for _ in 0..max_attempts {
        st.attempts += 1;
        let u = random_vec(f, n, &mut rng);
        let z = random_vec(f, n, &mut rng);
        let mut seq = Vec::with_capacity(2 * n + 2);
        let mut v = z.clone();
        for _ in 0..2 * n + 2 {
            seq.push(f.dot(&u, &v));
            v = a.apply(f, &v);
            st.matvecs += 1;
        }
        let (c, l) = berlekamp_massey(f, &seq);
        // Trailing zeros of c are the lambda^k factor.
        let mut k = 0;
        while k < l && c[l - k] == 0 {
            k += 1;
        }
        if k == 0 {
            // Projection did not reveal singularity; retry.
            continue;
        }
        // g_j = c[L - j - k] for j = 0..L-k; w = sum_j g_j A^j z by Horner
        // from the top coefficient g_{L-k} = c[0].
        let deg = l - k;
        let mut w = vec![0u64; n];
        for j in (0..=deg).rev() {
            if j < deg {
                w = a.apply(f, &w);
                st.matvecs += 1;
            }
            let g = c[l - j - k];
            for (wi, &zi) in w.iter_mut().zip(&z) {
                *wi = f.mul_add(*wi, g, zi);
            }
        }
        if let Some(kv) = last_nonzero_power(f, a, w, k + 1, &mut st) {
            return (Some(kv), st);
        }
    }
    (None, st)
}

/// Given `w`, walk `w, A w, ...` up to `limit` products and return the last
/// nonzero vector before reaching zero (a kernel vector), verified.
fn last_nonzero_power<O: LinearOperator>(
    f: Fp,
    a: &O,
    mut w: Vec<u64>,
    limit: usize,
    st: &mut KrylovStats,
) -> Option<Vec<u64>> {
    if is_zero(&w) {
        return None;
    }
    for _ in 0..limit {
        let nw = a.apply(f, &w);
        st.matvecs += 1;
        if is_zero(&nw) {
            return Some(w);
        }
        w = nw;
    }
    None
}

/// Polynomial matrix as coefficient list: `coef[d][i][j]`.
type PolyMat = Vec<Vec<Vec<u64>>>;

/// Iterative order basis (M-Basis): returns `P` (s x s polynomial matrix,
/// `P[i]` = row `i` as coefficient vectors) and the shifted row degrees with
/// `P(x) F(x) = 0 mod x^order`. `series[k]` is the `s x m` coefficient of
/// `x^k` in `F`.
fn order_basis(
    f: Fp,
    series: &[Vec<Vec<u64>>],
    order: usize,
    shifts: &[usize],
) -> (Vec<PolyMat>, Vec<usize>) {
    let s = shifts.len();
    let m = series[0][0].len();
    // rows[i] = list of coefficient vectors (each length s): P_i(x).
    let mut rows: Vec<Vec<Vec<u64>>> = (0..s)
        .map(|i| {
            let mut e = vec![0u64; s];
            e[i] = 1;
            vec![e]
        })
        .collect();
    let mut delta = shifts.to_vec();
    for k in 0..order {
        // Discrepancy Delta_i = coeff_k(P_i F) = sum_d P_i[d] * series[k-d].
        let mut disc: Vec<Vec<u64>> = vec![vec![0u64; m]; s];
        for i in 0..s {
            for (d, pc) in rows[i].iter().enumerate() {
                if d > k {
                    break;
                }
                let fk = &series[k - d];
                for (t, &pt) in pc.iter().enumerate() {
                    if pt == 0 {
                        continue;
                    }
                    for j in 0..m {
                        disc[i][j] = f.mul_add(disc[i][j], pt, fk[t][j]);
                    }
                }
            }
        }
        let mut order_idx: Vec<usize> = (0..s).collect();
        order_idx.sort_by_key(|&i| (delta[i], i));
        let mut pivots: Vec<(usize, usize)> = Vec::new(); // (row, col)
        for &i in &order_idx {
            for &(pr, pc) in &pivots {
                let c = disc[i][pc];
                if c == 0 {
                    continue;
                }
                let factor = f.neg(f.mul(c, f.inv(disc[pr][pc]).unwrap()));
                let (src_disc, src_row) = (disc[pr].clone(), rows[pr].clone());
                for j in 0..m {
                    disc[i][j] = f.mul_add(disc[i][j], factor, src_disc[j]);
                }
                if rows[i].len() < src_row.len() {
                    rows[i].resize(src_row.len(), vec![0u64; s]);
                }
                for (d, sc) in src_row.iter().enumerate() {
                    for t in 0..s {
                        rows[i][d][t] = f.mul_add(rows[i][d][t], factor, sc[t]);
                    }
                }
            }
            if let Some(pc) = (0..m).find(|&j| disc[i][j] != 0) {
                pivots.push((i, pc));
            }
        }
        for &(pr, _) in &pivots {
            rows[pr].insert(0, vec![0u64; s]); // multiply by x
            delta[pr] += 1;
        }
    }
    let out: Vec<PolyMat> = rows.into_iter().map(|r| vec![r]).collect();
    (out, delta)
}

/// Block Wiedemann kernel vector of a singular square operator with block
/// width `blk` (`m = n = blk`). Returns a verified nonzero `w` with `A w = 0`.
pub fn block_wiedemann_kernel<O: LinearOperator>(
    f: Fp,
    a: &O,
    blk: usize,
    seed: u64,
    max_attempts: u32,
) -> (Option<Vec<u64>>, KrylovStats) {
    let n = a.dim();
    let blk = blk.max(1).min(n.max(1));
    let mut rng = SplitMix64::new(seed);
    let mut st = KrylovStats::default();
    for _ in 0..max_attempts {
        st.attempts += 1;
        let xs: Vec<Vec<u64>> = (0..blk).map(|_| random_vec(f, n, &mut rng)).collect();
        let zs: Vec<Vec<u64>> = (0..blk).map(|_| random_vec(f, n, &mut rng)).collect();
        // Y = A Z.
        let mut ys: Vec<Vec<u64>> = zs.iter().map(|z| a.apply(f, z)).collect();
        st.matvecs += blk as u64;
        let len = n / blk + n / blk + 8;
        // series[k] is (n_blk + m_blk) x m_blk: [S_k^T ; -I (only k = 0)].
        let s_dim = 2 * blk;
        let mut series: Vec<Vec<Vec<u64>>> = Vec::with_capacity(len);
        for k in 0..len {
            let mut coef = vec![vec![0u64; blk]; s_dim];
            // S_k[i][j] = X_i . (A^k Y_j); transpose into rows 0..blk.
            for j in 0..blk {
                for i in 0..blk {
                    coef[j][i] = f.dot(&xs[i], &ys[j]);
                }
            }
            if k == 0 {
                for i in 0..blk {
                    coef[blk + i][i] = f.neg(1);
                }
            }
            series.push(coef);
            if k + 1 < len {
                ys = ys.iter().map(|y| a.apply(f, y)).collect();
                st.matvecs += blk as u64;
            }
        }
        let mut shifts = vec![0usize; s_dim];
        for sh in shifts.iter_mut().skip(blk) {
            *sh = 1;
        }
        let (basis, _delta) = order_basis(f, &series, len, &shifts);
        for row in basis {
            let coeffs = &row[0]; // coeffs[d][t]
                                  // v-part: first blk entries; degree of v.
            let deg_v = (0..coeffs.len())
                .rev()
                .find(|&d| coeffs[d][..blk].iter().any(|&x| x != 0));
            let Some(dv) = deg_v else { continue };
            // z' = sum_j A^j Z w_j with w_j = coeff of x^{dv - j} in v.
            let mut zp = vec![0u64; n];
            for j in (0..=dv).rev() {
                if j < dv {
                    zp = a.apply(f, &zp);
                    st.matvecs += 1;
                }
                let wj = &coeffs[dv - j][..blk];
                for (t, &c) in wj.iter().enumerate() {
                    if c == 0 {
                        continue;
                    }
                    for (zi, &zz) in zp.iter_mut().zip(&zs[t]) {
                        *zi = f.mul_add(*zi, c, zz);
                    }
                }
            }
            if let Some(kv) = last_nonzero_power(f, a, zp, dv + 3, &mut st) {
                return (Some(kv), st);
            }
        }
    }
    (None, st)
}

/// Block Wiedemann solve of a nonsingular `A x = b` through the kernel of
/// `[[A, -b], [0, 0]]` (see [`Augmented`]).
pub fn block_wiedemann_solve<O: LinearOperator>(
    f: Fp,
    a: &O,
    b: &[u64],
    blk: usize,
    seed: u64,
    max_attempts: u32,
) -> (Option<Vec<u64>>, KrylovStats) {
    let n = a.dim();
    let aug = Augmented { a, b };
    let mut total = KrylovStats::default();
    for t in 0..max_attempts {
        let (kv, st) =
            block_wiedemann_kernel(f, &aug, blk, seed.wrapping_add(t as u64 * 0x9E37), 1);
        total.matvecs += st.matvecs;
        total.attempts += 1;
        if let Some(kv) = kv {
            if kv[n] != 0 {
                let ti = f.inv(kv[n]).unwrap();
                let x: Vec<u64> = kv[..n].iter().map(|&v| f.mul(v, ti)).collect();
                total.matvecs += 1;
                if a.apply(f, &x) == b {
                    return (Some(x), total);
                }
            }
        }
    }
    (None, total)
}

/// Finite-field Lanczos for a square sparse `A x = b`, run on the symmetric
/// `B = A^T D A` with random nonzero diagonal `D`, three-term recurrence,
/// retry on self-orthogonal breakdown. Returns a verified solution of the
/// original system.
pub fn lanczos_solve(
    f: Fp,
    a: &SparseMatrix,
    b: &[u64],
    seed: u64,
    max_attempts: u32,
) -> (Option<Vec<u64>>, KrylovStats) {
    let n = a.ncols;
    let mut rng = SplitMix64::new(seed);
    let mut st = KrylovStats::default();
    for _ in 0..max_attempts {
        st.attempts += 1;
        let d: Vec<u64> = (0..a.nrows).map(|_| rng.nonzero(f)).collect();
        let apply_b = |x: &[u64], st: &mut KrylovStats| -> Vec<u64> {
            let mut y = a.mul_vec(f, x);
            for (yi, &di) in y.iter_mut().zip(&d) {
                *yi = f.mul(*yi, di);
            }
            st.matvecs += 2;
            a.mul_vec_transpose(f, &y)
        };
        let mut db = b.to_vec();
        for (x, &di) in db.iter_mut().zip(&d) {
            *x = f.mul(*x, di);
        }
        let rhs = a.mul_vec_transpose(f, &db);
        if is_zero(&rhs) {
            if is_zero(b) {
                return (Some(vec![0; n]), st);
            }
            continue;
        }
        let mut x = vec![0u64; n];
        let mut w_prev: Vec<u64> = vec![0; n];
        let mut bw_prev: Vec<u64> = vec![0; n];
        let mut wbw_prev = 1u64; // unused on first step
        let mut w = rhs.clone();
        let mut ok = false;
        for iter in 0..=n + 1 {
            let bw = apply_b(&w, &mut st);
            let wbw = f.dot(&w, &bw);
            if wbw == 0 {
                ok = is_zero(&w);
                break;
            }
            // x += (w . rhs)/(w . B w) w
            let coef = f.div(f.dot(&w, &rhs), wbw).unwrap();
            for (xi, &wi) in x.iter_mut().zip(&w) {
                *xi = f.mul_add(*xi, coef, wi);
            }
            // w_next = B w - (Bw.Bw)/(w.Bw) w - (Bw.Bw_prev)/(w_prev.Bw_prev) w_prev
            let c1 = f.div(f.dot(&bw, &bw), wbw).unwrap();
            let c2 = if iter == 0 {
                0
            } else {
                f.div(f.dot(&bw, &bw_prev), wbw_prev).unwrap()
            };
            let mut nw = bw.clone();
            for i in 0..n {
                nw[i] = f.sub(nw[i], f.mul(c1, w[i]));
                nw[i] = f.sub(nw[i], f.mul(c2, w_prev[i]));
            }
            w_prev = std::mem::replace(&mut w, nw);
            bw_prev = bw;
            wbw_prev = wbw;
        }
        if ok {
            st.matvecs += 1;
            if a.mul_vec(f, &x) == b {
                return (Some(x), st);
            }
        }
    }
    (None, st)
}

/// Rank of a sparse matrix: structured elimination then dense residual rank.
/// `rank(A) = #eliminated columns + rank(residual)`.
pub fn sparse_rank(f: Fp, a: &SparseMatrix) -> usize {
    let res = structured_elimination(f, a, EliminationOptions::default());
    let resid = SparseMatrix::new(
        f,
        a.ncols,
        res.rows.iter().map(|r| r.entries.clone()).collect(),
    );
    res.eliminated.len() + resid.to_dense().rank(f)
}

/// Basis of the left kernel `{y : y^T A = 0}` (sparse vectors over rows),
/// lifted from the residual through the elimination provenance.
pub fn sparse_left_kernel(f: Fp, a: &SparseMatrix) -> Vec<SparseVec> {
    let res = structured_elimination(f, a, EliminationOptions::default());
    let resid = SparseMatrix::new(
        f,
        a.ncols,
        res.rows.iter().map(|r| r.entries.clone()).collect(),
    );
    let mut out = Vec::new();
    for z in resid.to_dense().left_nullspace(f) {
        let mut acc: SparseVec = Vec::new();
        for (i, &zi) in z.iter().enumerate() {
            if zi != 0 {
                acc = super::sparse::axpy(f, &acc, zi, &res.rows[i].provenance);
            }
        }
        out.push(acc);
    }
    out
}

/// Basis of the right kernel `{x : A x = 0}` (dense).
pub fn sparse_right_kernel(f: Fp, a: &SparseMatrix) -> Vec<Vec<u64>> {
    a.to_dense().right_nullspace(f)
}

/// Inverse of a square nonsingular sparse matrix (dense output: the full
/// inverse can require quadratic space even when one solve would do).
pub fn sparse_inverse(f: Fp, a: &SparseMatrix) -> Option<DenseMatrix> {
    a.to_dense().inverse(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_square(f: Fp, n: usize, w: usize, seed: u64) -> SparseMatrix {
        let mut rng = SplitMix64::new(seed);
        let rows = (0..n)
            .map(|i| {
                let mut r: Vec<(usize, u64)> = (0..w)
                    .map(|_| (rng.below(n as u64) as usize, rng.nonzero(f)))
                    .collect();
                r.push((i, rng.nonzero(f))); // keep it likely nonsingular
                r
            })
            .collect();
        SparseMatrix::new(f, n, rows)
    }

    #[test]
    fn bm_finds_fibonacci() {
        let f = Fp::new(1_000_003).unwrap();
        let mut s = vec![0u64, 1];
        for i in 2..20 {
            s.push(f.add(s[i - 1], s[i - 2]));
        }
        let (c, l) = berlekamp_massey(f, &s);
        assert_eq!(l, 2);
        assert_eq!(c, vec![1, f.neg(1), f.neg(1)]);
    }

    #[test]
    fn scalar_block_lanczos_solve_agree_with_dense() {
        let mut solved = 0;
        for &p in &[10007u64, 1_000_000_007] {
            let f = Fp::new(p).unwrap();
            for seed in 0..6 {
                let n = 20 + seed as usize * 7;
                let a = random_square(f, n, 3, seed);
                let mut rng = SplitMix64::new(99 + seed);
                let b: Vec<u64> = (0..n).map(|_| rng.element(f)).collect();
                let dense = a.to_dense().solve(f, &b);
                if a.to_dense().rank(f) < n {
                    continue;
                }
                let x0 = dense.unwrap();
                let (x1, _) = wiedemann_solve(f, &a, &b, seed, 8);
                assert_eq!(x1.unwrap(), x0);
                for blk in [1, 2, 4] {
                    let (x2, st) = block_wiedemann_solve(f, &a, &b, blk, seed, 8);
                    assert_eq!(x2.unwrap(), x0, "blk {blk} p {p} seed {seed} {:?}", st);
                }
                let (x3, _) = lanczos_solve(f, &a, &b, seed, 16);
                assert_eq!(x3.unwrap(), x0);
                solved += 1;
            }
        }
        assert!(solved >= 8, "only {solved} nonsingular instances exercised");
    }

    #[test]
    fn kernels_of_singular_matrices() {
        let f = Fp::new(65537).unwrap();
        for seed in 0..8 {
            let n = 25;
            let mut rng = SplitMix64::new(500 + seed);
            // Rank-deficient: last 3 rows are combinations of others.
            let mut rows: Vec<Vec<(usize, u64)>> = (0..n - 3)
                .map(|_| {
                    (0..4)
                        .map(|_| (rng.below(n as u64) as usize, rng.nonzero(f)))
                        .collect()
                })
                .collect();
            let base = SparseMatrix::new(f, n, rows.clone());
            for _ in 0..3 {
                let i = rng.below((n - 3) as u64) as usize;
                let j = rng.below((n - 3) as u64) as usize;
                let comb =
                    super::super::sparse::axpy(f, &base.rows[i], rng.nonzero(f), &base.rows[j]);
                rows.push(comb);
            }
            let a = SparseMatrix::new(f, n, rows);
            let (k1, _) = wiedemann_kernel(f, &a, seed, 10);
            let k1 = k1.expect("scalar kernel");
            assert!(!is_zero(&k1) && is_zero(&a.mul_vec(f, &k1)));
            for blk in [2, 3] {
                let (k2, _) = block_wiedemann_kernel(f, &a, blk, seed, 10);
                let k2 = k2.expect("block kernel");
                assert!(!is_zero(&k2) && is_zero(&a.mul_vec(f, &k2)));
            }
            let dense_rank = a.to_dense().rank(f);
            assert_eq!(sparse_rank(f, &a), dense_rank);
            let lk = sparse_left_kernel(f, &a);
            assert_eq!(lk.len(), n - dense_rank);
            for y in &lk {
                assert!(a.left_combination(f, y).is_empty());
            }
            assert_eq!(sparse_right_kernel(f, &a).len(), n - dense_rank);
        }
    }
}
