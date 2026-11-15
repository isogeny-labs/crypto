//! Structured Gaussian elimination, then block Wiedemann, for one unknown
//! (plan track T3: a solver for line-sieve rows whose cost grows slowly in
//! `m`).
//!
//! Port of the `lpsgebw2` arm of `examples/ic_improve2_06_genus3_kappa_bw.rs`
//! (crypto `origin/main`: `Sys::{fill, eliminate, sge, trim, compact}`,
//! `wiedemann_k`, `block_wiedemann_k`) to this crate's `u64` rows and [`Fp`].
//!
//! 1. **SGE** (`Sys::sge`).  Columns `0..m` are eliminated in Markowitz order
//!    of estimated fill `(w − 1)(r − 2) − r` (`w` the column weight, `r` the
//!    lightest holder's length), pivoting on the lightest holder and dropping
//!    the pivot row.  Weight-1 columns (singletons) have negative fill and go
//!    first: their row is dropped and the cascade continues.  Elimination
//!    stops at the first column whose fill reaches `nnz/ncols + cx`, the point
//!    where the Wiedemann cost `2·nnz·n + cx·n²` stops decreasing
//!    ([`DEFAULT_CX`] = 32).  The `k` column `m` is never eliminated.
//! 2. **Trim and fold.**  Drop the heaviest rows, cascading singletons
//!    (`Sys::trim`), until at most `EXCESS` = 16 surplus rows remain; compact
//!    the live columns.  The square matrix is `B = A_top + R·A_ex` (first `n`
//!    rows plus a random combination of the surplus rows, `MIX` = 1 random
//!    entry per row of `R`), applied as `A_top v + R (A_ex v)`, with the
//!    right-hand side folded the same way.
//! 3. **Block Wiedemann** (`block_wiedemann_k`), blocking `(b, 1)` with `b`
//!    coordinate projections (`e_k` first): the vector sequence
//!    `Xᵀ Aⁱ rhs` for `≈ n(1 + 1/b)` terms, an order basis of `[A(x) | −I_b]`
//!    (iterative mbasis) for the generator `g`, and
//!    `k = −(1/g₀) Σ_{i≥1} g_i (A^{i−1} rhs)[k]`.  Scalar Wiedemann
//!    (Berlekamp–Massey) when `n < b + 1`.
//!
//! Deviations from the prior art, all deliberate:
//! - **Counting.**  Every product mod `ℓ` is counted in `stats.mulmods`,
//!   including products by ±1 (the prior art did not count SGE products
//!   whose factor is ±1; [`crate::solve::solve_fast`] counts them, so this
//!   keeps the two on the same basis).  Per SGE row operation: the factor,
//!   one product per pivot entry other than the eliminated column, and the
//!   right-hand side.  Per matrix-vector product: `nnz + MIX·n`; folding the
//!   right-hand side: `MIX·n`.  Generator: every
//!   product in the residuals, the basis elimination and the final sum.
//!   Inversions are not counted (Fast does not count its own).
//! - **Early exit.**  A row that becomes `k`-only during SGE answers at once
//!   (Fast's rule); so does a `k`-only input row.
//! - **Fold instead of trimming to square.**  The prior art trims to
//!   exactly square (`trim(0)`).  On line-sieve rows that loses rank: on
//!   g3 p=61 a full-rank 20×14 residual became a 13×13 matrix of rank 11, and
//!   Wiedemann returned the same wrong `k` on every retry (the sieve never
//!   finished).  Folding the surplus rows in at random keeps `B` nonsingular
//!   when the residual has full column rank.  Cost: `MIX·n` products per
//!   matrix-vector product, counted.
//! - **`cx` = 32** instead of 2.0 (tuned on the measured rows; see
//!   [`DEFAULT_CX`]).
//! - **Reduction.**  u128 accumulators are reduced every
//!   `⌊(2^128 − 1 − ℓ)/(ℓ − 1)²⌋` products by a reduction valid for any u128
//!   (the prior art's `Mont::reduce`), so `ℓ` up to `2^62` is safe.
//!
//! Correctness: SGE row operations and row drops keep every live row a
//! consequence of the input rows, so a `k` read off a `k`-only row is the
//! true `k` when the system is consistent.  Wiedemann's `k` is the true one
//! when the folded square matrix is nonsingular; when it is singular the result
//! is `None` (`g₀ = 0`) or possibly a wrong `k`, which the callers catch by
//! checking `k·D₁ = D₂`.  The early termination (`ET` = 20 terms past the
//! generator's apparent degree) can also fail with probability about `ℓ^{-20}`.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::fp::Fp;
use crate::solve::{Row, SolveStats};

/// SGE stopping constant.  The prior art's default was 2.0; on line-sieve
/// rows a sweep (2, 16, 32, 64, 128 on g2 m=2005/8205, g3 m=1997, one seed)
/// gave the fewest counted mulmods at 32–64, 13–20% below 2.0, so 32.
pub const DEFAULT_CX: f64 = 32.0;
/// Number of projections (the prior art's `lpsgebw2`).
pub const DEFAULT_BLOCK: usize = 2;
/// Early-termination margin of the Wiedemann sequences.
const ET: usize = 20;
/// Surplus rows kept after SGE and folded into the square matrix.
const EXCESS: usize = 16;
/// Surplus rows folded into each row of the square matrix.
const MIX: usize = 1;

/// [`solve_sge_bw_with`] with [`DEFAULT_CX`] and [`DEFAULT_BLOCK`], each
/// overridable for experiments by the environment variables `HEC_SGE_CX`
/// and `HEC_BW_BLOCK`.  `HEC_BW_TRACE=1` prints one `BWTRACE {...}` JSON
/// line per call on stderr.
pub fn solve_sge_bw(rows: &[(Row, u64)], m: usize, ell: u64, stats: &mut SolveStats) -> Option<u64> {
    let cx = std::env::var("HEC_SGE_CX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CX);
    let block = std::env::var("HEC_BW_BLOCK")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_BLOCK)
        .max(1);
    let before = *stats;
    let k = solve_sge_bw_with(rows, m, ell, cx, block, stats);
    if std::env::var("HEC_BW_TRACE").as_deref() == Ok("1") {
        eprintln!(
            "BWTRACE {{\"m\":{m},\"ell\":{ell},\"rows_in\":{},\"cx\":{cx},\"block\":{block},\
\"mulmods\":{},\"sge_mulmods\":{},\"bw_mulmods\":{},\"matvecs\":{},\"row_ops\":{},\
\"sge_rows_left\":{},\"sge_cols_left\":{},\"sge_nnz_left\":{},\"bw_degree\":{},\"found\":{}}}",
            rows.len(),
            stats.mulmods - before.mulmods,
            stats.sge_mulmods - before.sge_mulmods,
            stats.bw_mulmods - before.bw_mulmods,
            stats.matvecs - before.matvecs,
            stats.row_ops - before.row_ops,
            stats.sge_rows_left - before.sge_rows_left,
            stats.sge_cols_left - before.sge_cols_left,
            stats.sge_nnz_left - before.sge_nnz_left,
            stats.bw_degree - before.bw_degree,
            k.is_some()
        );
    }
    k
}

/// `Some(k)` for column `m` of `rows` (sorted by column, coefficients
/// reduced, non-zero), or `None`.  See the module documentation.
pub fn solve_sge_bw_with(
    rows: &[(Row, u64)],
    m: usize,
    ell: u64,
    cx: f64,
    block: usize,
    stats: &mut SolveStats,
) -> Option<u64> {
    let f = Fp::new_any(ell);
    let kcol = m as u32;
    for (r, rhs) in rows {
        if r.len() == 1 && r[0].0 == kcol {
            stats.mulmods += 1;
            stats.sge_mulmods += 1;
            return Some(f.mul(*rhs, f.inv(r[0].1)));
        }
    }
    let mut sys = Sys::new(rows, m, &f);
    let found = sys.sge(cx);
    stats.mulmods += sys.mulmods;
    stats.sge_mulmods += sys.mulmods;
    stats.row_ops += sys.row_ops;
    stats.pivots += sys.pivots;
    stats.max_row_len = stats.max_row_len.max(sys.max_row_len);
    if found.is_some() || sys.inconsistent {
        return found;
    }
    // Keep up to EXCESS surplus rows (dropping the heaviest, cascading
    // singletons) and fold them into the square system at random.
    let surplus = sys.nrows.saturating_sub(sys.ncols).min(EXCESS);
    sys.trim(surplus);
    let (crows, crhs, n, kc) = sys.compact();
    stats.sge_rows_left += crows.len() as u64;
    stats.sge_cols_left += n as u64;
    stats.sge_nnz_left += crows.iter().map(|r| r.len() as u64).sum::<u64>();
    let kc = kc?;
    if n == 0 || crows.len() < n {
        return None;
    }
    let mut bw = BwCount::default();
    let (a, crhs) = SpMat::folded(&crows, &crhs, n, &f, &mut bw);
    let k = if n < block + 1 || block < 2 {
        wiedemann_k(&a, &crhs, kc, &f, &mut bw)
    } else {
        block_wiedemann_k(&a, &crhs, kc, block, &f, &mut bw)
    };
    stats.mulmods += bw.mulmods;
    stats.bw_mulmods += bw.mulmods;
    stats.matvecs += bw.matvecs;
    stats.bw_degree += bw.degree;
    k
}

/// `t mod ℓ` for any `u128` (the prior art's `Mont::reduce`):
/// [`Fp::reduce128`] needs `t < ℓ·2^64`, so the high word is reduced first.
#[inline(always)]
fn red(f: &Fp, t: u128) -> u64 {
    let hi = (t >> 64) as u64;
    if hi >= f.p {
        f.reduce128((((hi % f.p) as u128) << 64) | (t as u64 as u128))
    } else {
        f.reduce128(t)
    }
}

/// Products `< (ℓ − 1)²` that fit in a u128 accumulator on top of a
/// reduced value `< ℓ`.
fn red_every(ell: u64) -> usize {
    let sq = (ell as u128 - 1) * (ell as u128 - 1);
    ((u128::MAX - ell as u128) / sq.max(1)).clamp(1, 1 << 20) as usize
}

// ---------------------------------------------------------------------------
// Structured Gaussian elimination (the prior art's `Sys`).

/// Working system.  The `k` column `kc` is never eliminated and has no
/// holder list.
struct Sys<'a> {
    f: &'a Fp,
    kc: u32,
    rows: Vec<Option<Row>>,
    rhs: Vec<u64>,
    col_rows: Vec<Vec<u32>>,
    nnz: u64,
    nrows: usize,
    /// Active columns, `k` included.
    ncols: usize,
    inconsistent: bool,
    mulmods: u64,
    row_ops: u64,
    pivots: u64,
    max_row_len: usize,
}

impl<'a> Sys<'a> {
    fn new(rows: &[(Row, u64)], m: usize, f: &'a Fp) -> Self {
        let kc = m as u32;
        let mut col_rows = vec![Vec::new(); m];
        let mut nnz = 0u64;
        let mut srows = Vec::with_capacity(rows.len());
        let mut rhs = Vec::with_capacity(rows.len());
        for (i, (r, b)) in rows.iter().enumerate() {
            let mut r = r.clone();
            if !r.windows(2).all(|w| w[0].0 < w[1].0) {
                r.sort_unstable_by_key(|e| e.0);
            }
            nnz += r.len() as u64;
            for &(j, _) in &r {
                if j != kc {
                    col_rows[j as usize].push(i as u32);
                }
            }
            srows.push(Some(r));
            rhs.push(*b);
        }
        let ncols = col_rows.iter().filter(|v| !v.is_empty()).count() + 1;
        Sys {
            f,
            kc,
            rows: srows,
            rhs,
            col_rows,
            nnz,
            nrows: rows.len(),
            ncols,
            inconsistent: false,
            mulmods: 0,
            row_ops: 0,
            pivots: 0,
            max_row_len: 0,
        }
    }

    fn detach_col(&mut self, j: u32, i: u32) {
        let v = &mut self.col_rows[j as usize];
        if let Some(pos) = v.iter().position(|&x| x == i) {
            v.swap_remove(pos);
            if v.is_empty() {
                self.ncols -= 1;
            }
        }
    }

    /// Remove row `i`; returns the columns left with weight 1.
    fn remove_row(&mut self, i: u32) -> Vec<u32> {
        let r = self.rows[i as usize].take().unwrap();
        self.nnz -= r.len() as u64;
        self.nrows -= 1;
        let mut ones = vec![];
        for &(j, _) in &r {
            if j != self.kc {
                self.detach_col(j, i);
                if self.col_rows[j as usize].len() == 1 {
                    ones.push(j);
                }
            }
        }
        ones
    }

    fn cascade(&mut self, mut stack: Vec<u32>) {
        while let Some(j) = stack.pop() {
            if self.col_rows[j as usize].len() == 1 {
                let i = self.col_rows[j as usize][0];
                let more = self.remove_row(i);
                stack.extend(more);
            }
        }
    }

    /// Drop the heaviest rows (cascading singletons) until
    /// `rows ≤ cols + excess`.
    fn trim(&mut self, excess: usize) {
        let mut order: Vec<(usize, u32)> = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.as_ref().map(|r| (r.len(), i as u32)))
            .collect();
        order.sort_by(|a, b| b.cmp(a));
        for &(_, i) in &order {
            if self.nrows <= self.ncols + excess {
                break;
            }
            if self.rows[i as usize].is_some() {
                let ones = self.remove_row(i);
                self.cascade(ones);
            }
        }
    }

    /// Markowitz fill of eliminating column `j` with its lightest holder as
    /// pivot (no cancellation assumed), and that holder.
    fn fill(&self, j: u32) -> Option<(i64, u32)> {
        let rs = &self.col_rows[j as usize];
        if rs.is_empty() {
            return None;
        }
        let (mut best, mut bi) = (usize::MAX, 0u32);
        for &i in rs {
            let w = self.rows[i as usize].as_ref().unwrap().len();
            if w < best {
                best = w;
                bi = i;
            }
        }
        let w = rs.len() as i64;
        let r = best as i64;
        Some(((w - 1) * (r - 2) - r, bi))
    }

    /// Eliminate column `j` with pivot row `pi` and drop the pivot row.
    /// Returns the columns whose weight changed, or `Err(k)` when a row
    /// became `k`-only.
    fn eliminate(&mut self, j: u32, pi: u32) -> Result<Vec<u32>, u64> {
        let f = self.f;
        let piv = self.rows[pi as usize].take().unwrap();
        let prhs = self.rhs[pi as usize];
        let apiv = piv.iter().find(|e| e.0 == j).unwrap().1;
        let inv = f.inv(apiv);
        self.pivots += 1;
        let others: Vec<u32> = self.col_rows[j as usize]
            .iter()
            .copied()
            .filter(|&i| i != pi)
            .collect();
        let mut touched: Vec<u32> = piv
            .iter()
            .map(|e| e.0)
            .filter(|&c| c != self.kc && c != j)
            .collect();
        let mut out: Row = Vec::new();
        let mut rem: Vec<u32> = Vec::new();
        let mut add: Vec<u32> = Vec::new();
        let mut result = Ok(());
        for &ri in &others {
            let old = self.rows[ri as usize].take().unwrap();
            let ar = old.iter().find(|e| e.0 == j).unwrap().1;
            let fm = f.pre(f.mul(ar, inv)); // row −= fac · piv
            self.mulmods += 1;
            out.clear();
            out.reserve(old.len() + piv.len());
            let (mut a, mut b) = (0, 0);
            while a < old.len() || b < piv.len() {
                // Column j cancels by construction; not computed.
                if a < old.len() && old[a].0 == j {
                    a += 1;
                    continue;
                }
                if b < piv.len() && piv[b].0 == j {
                    b += 1;
                    continue;
                }
                if b >= piv.len() || (a < old.len() && old[a].0 < piv[b].0) {
                    out.push(old[a]);
                    a += 1;
                } else if a >= old.len() || piv[b].0 < old[a].0 {
                    let x = f.mul_pre(piv[b].1, fm);
                    self.mulmods += 1;
                    if x != 0 {
                        out.push((piv[b].0, f.neg(x)));
                    }
                    b += 1;
                } else {
                    let x = f.mul_pre(piv[b].1, fm);
                    self.mulmods += 1;
                    let y = f.sub(old[a].1, x);
                    if y != 0 {
                        out.push((old[a].0, y));
                    }
                    a += 1;
                    b += 1;
                }
            }
            let x = f.mul_pre(prhs, fm);
            self.mulmods += 1;
            let r = &mut self.rhs[ri as usize];
            *r = f.sub(*r, x);
            self.row_ops += 1;
            self.max_row_len = self.max_row_len.max(out.len());
            // Holder lists: removals (cancelled, and j) and insertions (fill).
            rem.clear();
            add.clear();
            let (mut a, mut b) = (0, 0);
            while a < old.len() || b < out.len() {
                if b >= out.len() || (a < old.len() && old[a].0 < out[b].0) {
                    rem.push(old[a].0);
                    a += 1;
                } else if a >= old.len() || out[b].0 < old[a].0 {
                    add.push(out[b].0);
                    b += 1;
                } else {
                    a += 1;
                    b += 1;
                }
            }
            self.nnz = self.nnz - old.len() as u64 + out.len() as u64;
            for &c in &rem {
                if c != self.kc {
                    self.detach_col(c, ri);
                    touched.push(c);
                }
            }
            for &c in &add {
                if c != self.kc {
                    if self.col_rows[c as usize].is_empty() {
                        self.ncols += 1;
                    }
                    self.col_rows[c as usize].push(ri);
                }
            }
            if out.is_empty() {
                // Dependent row: 0 = rhs.
                if self.rhs[ri as usize] != 0 {
                    self.inconsistent = true;
                }
                self.nrows -= 1;
            } else {
                if out.len() == 1 && out[0].0 == self.kc && result.is_ok() {
                    self.mulmods += 1;
                    result = Err(f.mul(self.rhs[ri as usize], f.inv(out[0].1)));
                }
                self.rows[ri as usize] = Some(std::mem::take(&mut out));
            }
            if result.is_err() {
                break;
            }
        }
        self.rows[pi as usize] = Some(piv);
        result?;
        let ones = self.remove_row(pi);
        touched.extend(ones);
        touched.sort_unstable();
        touched.dedup();
        Ok(touched)
    }

    /// Markowitz SGE until the fill reaches `nnz/ncols + cx`.  `Some(k)` if a
    /// row became `k`-only.
    fn sge(&mut self, cx: f64) -> Option<u64> {
        let mut heap: BinaryHeap<Reverse<(i64, u32)>> = BinaryHeap::new();
        for j in 0..self.col_rows.len() as u32 {
            if let Some((fl, _)) = self.fill(j) {
                heap.push(Reverse((fl, j)));
            }
        }
        while let Some(Reverse((fl, j))) = heap.pop() {
            let Some((fnow, pi)) = self.fill(j) else {
                continue;
            };
            if fnow != fl {
                heap.push(Reverse((fnow, j)));
                continue;
            }
            if self.col_rows[j as usize].len() == 1 {
                let ones = self.remove_row(pi);
                for c in ones {
                    if let Some((f2, _)) = self.fill(c) {
                        heap.push(Reverse((f2, c)));
                    }
                }
                continue;
            }
            let avg = self.nnz as f64 / self.ncols as f64;
            if (fl as f64) >= avg + cx {
                break;
            }
            match self.eliminate(j, pi) {
                Err(k) => return Some(k),
                Ok(touched) => {
                    if self.inconsistent {
                        return None;
                    }
                    for c in touched {
                        if let Some((f2, _)) = self.fill(c) {
                            heap.push(Reverse((f2, c)));
                        }
                    }
                }
            }
        }
        None
    }

    /// Live rows over consecutive column indices: rows, rhs, column count and
    /// the index of `k` (if it is still present).
    fn compact(&self) -> (Vec<Row>, Vec<u64>, usize, Option<usize>) {
        let mut map: Vec<u32> = vec![u32::MAX; self.col_rows.len() + 1];
        let mut n = 0u32;
        let mut rows = vec![];
        let mut rhs = vec![];
        for (i, r) in self.rows.iter().enumerate() {
            if let Some(r) = r {
                let mut rr: Row = Vec::with_capacity(r.len());
                for &(j, c) in r {
                    let mj = &mut map[j as usize];
                    if *mj == u32::MAX {
                        *mj = n;
                        n += 1;
                    }
                    rr.push((*mj, c));
                }
                rr.sort_unstable_by_key(|e| e.0);
                rows.push(rr);
                rhs.push(self.rhs[i]);
            }
        }
        let kc = map[self.kc as usize];
        let kc = (kc != u32::MAX).then_some(kc as usize);
        (rows, rhs, n as usize, kc)
    }
}

// ---------------------------------------------------------------------------
// Wiedemann.

#[derive(Default)]
struct BwCount {
    mulmods: u64,
    matvecs: u64,
    degree: u64,
}

/// Square matrix `B = A_top + R·A_ex` in CSR form: `A_top` the first `n`
/// rows, `A_ex` the `E` surplus rows, `R` an `n × E` matrix with `MIX`
/// random entries per row (none when `E = 0`).  `B v` is applied as
/// `A_top v + R (A_ex v)` without forming `B`.
struct SpMat {
    n: usize,
    ptr: Vec<usize>,
    col: Vec<u32>,
    val: Vec<u64>,
    /// `R`: for row `i`, entries `mix[i·MIX..(i+1)·MIX]` = (surplus row, coef).
    mix: Vec<(u32, u64)>,
    every: usize,
}

impl SpMat {
    /// `B` and `R·rhs` folded into the top `rhs`; the folding products are
    /// counted in `c`.
    fn folded(rows: &[Row], rhs: &[u64], n: usize, f: &Fp, c: &mut BwCount) -> (Self, Vec<u64>) {
        let mut ptr = Vec::with_capacity(rows.len() + 1);
        let mut col = Vec::new();
        let mut val = Vec::new();
        ptr.push(0);
        for r in rows {
            for &(j, v) in r {
                col.push(j);
                val.push(v);
            }
            ptr.push(col.len());
        }
        let e = rows.len() - n;
        let mut mix = Vec::new();
        let mut b: Vec<u64> = rhs[..n].to_vec();
        if e > 0 {
            let mut rng = crate::rng::Rng::new((n as u64) << 32 ^ f.p ^ 0x5eed);
            mix.reserve(n * MIX);
            for bi in b.iter_mut() {
                for _ in 0..MIX {
                    let t = rng.below(e as u64) as u32;
                    let r = 1 + rng.below(f.p - 1);
                    mix.push((t, r));
                    *bi = f.add(*bi, f.mul(r, rhs[n + t as usize]));
                    c.mulmods += 1;
                }
            }
        }
        (
            SpMat {
                n,
                ptr,
                col,
                val,
                mix,
                every: red_every(f.p),
            },
            b,
        )
    }

    #[inline]
    fn row_dot(&self, i: usize, v: &[u64], f: &Fp) -> u64 {
        let mut acc: u128 = 0;
        let mut cnt = 0usize;
        for t in self.ptr[i]..self.ptr[i + 1] {
            acc += self.val[t] as u128 * v[self.col[t] as usize] as u128;
            cnt += 1;
            if cnt == self.every {
                acc = red(f, acc) as u128;
                cnt = 0;
            }
        }
        red(f, acc)
    }

    /// `out = B v`, counted as `nnz(A) + MIX·n` products (`nnz(A)` alone
    /// when nothing is folded).
    fn mul(&self, v: &[u64], out: &mut [u64], f: &Fp, c: &mut BwCount) {
        c.mulmods += self.col.len() as u64 + self.mix.len() as u64;
        c.matvecs += 1;
        let e = self.ptr.len() - 1 - self.n;
        let u: Vec<u64> = (0..e).map(|t| self.row_dot(self.n + t, v, f)).collect();
        for i in 0..self.n {
            let mut x = self.row_dot(i, v, f);
            if e > 0 {
                for &(t, r) in &self.mix[i * MIX..(i + 1) * MIX] {
                    x = f.add(x, f.mul(r, u[t as usize]));
                }
            }
            out[i] = x;
        }
    }
}

/// `Σ a_j b_j mod ℓ` over paired iterators, with chunked reduction.
#[inline]
fn dot(f: &Fp, every: usize, it: impl Iterator<Item = (u64, u64)>) -> u64 {
    let mut acc: u128 = 0;
    let mut cnt = 0usize;
    for (a, b) in it {
        acc += a as u128 * b as u128;
        cnt += 1;
        if cnt == every {
            acc = red(f, acc) as u128;
            cnt = 0;
        }
    }
    red(f, acc)
}

/// Scalar Wiedemann for coordinate `kc` of `A^{-1} b` (the prior art's
/// `wiedemann_k`): Berlekamp–Massey on `s_i = (Aⁱ b)[kc]`, stopping once
/// `2L + ET` terms leave `L` unchanged.
fn wiedemann_k(a: &SpMat, rhs: &[u64], kc: usize, f: &Fp, cn: &mut BwCount) -> Option<u64> {
    let n = a.n;
    let ell = f.p;
    let every = red_every(ell);
    let maxlen = 2 * n + ET + 2;
    let mut v: Vec<u64> = rhs.to_vec();
    let mut w: Vec<u64> = vec![0; n];
    let mut s: Vec<u64> = Vec::with_capacity(maxlen);
    let mut c: Vec<u64> = vec![1];
    let mut bpoly: Vec<u64> = vec![1];
    let mut l = 0usize;
    let mut mstep = 1usize;
    let mut binv = 1u64;
    let mut i = 0usize;
    loop {
        if i > 0 {
            a.mul(&v, &mut w, f, cn);
            std::mem::swap(&mut v, &mut w);
        }
        s.push(v[kc]);
        let mut d = s[i];
        if l > 0 {
            cn.mulmods += l as u64;
            let x = dot(f, every, (1..=l).map(|j| (c[j], s[i - j])));
            d = f.add(d, x);
        }
        if d == 0 {
            mstep += 1;
        } else {
            cn.mulmods += 1;
            let coef = f.mul(d, binv);
            let need = bpoly.len() + mstep;
            let t = if 2 * l <= i { Some(c.clone()) } else { None };
            if c.len() < need {
                c.resize(need, 0);
            }
            cn.mulmods += bpoly.len() as u64;
            let cm = f.pre(coef);
            for (j, &bj) in bpoly.iter().enumerate() {
                let x = f.mul_pre(bj, cm);
                let y = &mut c[j + mstep];
                *y = f.sub(*y, x);
            }
            if let Some(t) = t {
                l = i + 1 - l;
                if c.len() < l + 1 {
                    c.resize(l + 1, 0);
                }
                bpoly = t;
                binv = f.inv(d);
                mstep = 1;
            } else {
                mstep += 1;
            }
        }
        i += 1;
        if i >= 2 * l + ET || i >= maxlen {
            break;
        }
    }
    c.resize(l + 1, 0);
    cn.degree += l as u64;
    // g_i = c_{L−i}.
    let g0 = c[l];
    if g0 == 0 || l == 0 {
        return None;
    }
    cn.mulmods += l as u64 + 1;
    let acc = dot(f, every, (1..=l).map(|ii| (c[l - ii], s[ii - 1])));
    Some(f.mul(f.neg(acc), f.inv(g0)))
}

/// Block Wiedemann with `bm` coordinate projections and one right vector
/// (the prior art's `block_wiedemann_k`; blocking `(bm, 1)`).
///
/// The sequence `a_i = Xᵀ Aⁱ b ∈ K^{bm}`, `X = [e_kc, e_j2, …]` (projections
/// read for free), determines the minimal recurrence of `b` after about
/// `n(1 + 1/bm)` terms.  Generator: an order basis of `[A(x) | −I]` by
/// iterative mbasis with shifts `(0, 1, …, 1)`; a basis column is
/// `(f_c, ρ_c, δ_c)` with `r_c = (A f_c mod x^σ) + x^σ ρ_c`.  With
/// `g(x) = x^L f(1/x)` for the non-zero `f` column of least shifted degree
/// `L`, `k = −(1/g₀) Σ_{i≥1} g_i (A^{i−1} b)[kc]`.
fn block_wiedemann_k(
    a: &SpMat,
    rhs: &[u64],
    kc: usize,
    bm: usize,
    f: &Fp,
    cn: &mut BwCount,
) -> Option<u64> {
    let n = a.n;
    let ell = f.p;
    let every = red_every(ell);
    let maxlen = n + n / bm + ET + 2;
    // Projections: kc first, then bm − 1 distinct pseudo-random coordinates.
    let mut proj = vec![kc];
    let mut st: u64 = (n as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (kc as u64 + 1);
    while proj.len() < bm {
        st ^= st << 13;
        st ^= st >> 7;
        st ^= st << 17;
        let j = (st % n as u64) as usize;
        if !proj.contains(&j) {
            proj.push(j);
        }
    }
    let k = bm + 1;
    let mut seq: Vec<Vec<u64>> = vec![Vec::with_capacity(maxlen); bm];
    let mut fp: Vec<Vec<u64>> = vec![Vec::new(); k];
    fp[0] = vec![1];
    let mut rho: Vec<Vec<u64>> = vec![vec![0; bm]; k];
    for t in 0..bm {
        rho[t + 1][t] = 1;
    }
    let mut delta: Vec<usize> = vec![1; k];
    delta[0] = 0;
    let mut v: Vec<u64> = rhs.to_vec();
    let mut w: Vec<u64> = vec![0; n];
    let mut res: Vec<Vec<u64>> = vec![vec![0; bm]; k];
    let mut sigma = 0usize;
    loop {
        if sigma > 0 {
            a.mul(&v, &mut w, f, cn);
            std::mem::swap(&mut v, &mut w);
        }
        for t in 0..bm {
            seq[t].push(v[proj[t]]);
        }
        // Residuals R[c][t] = coeff_σ(A_t f_c) − ρ_c[t].
        for c in 0..k {
            let fc = &fp[c];
            let len = fc.len().min(sigma + 1);
            cn.mulmods += (bm * len) as u64;
            for t in 0..bm {
                let s = &seq[t];
                let x = if len > 0 {
                    dot(f, every, (0..len).map(|j| (fc[j], s[sigma - j])))
                } else {
                    0
                };
                res[c][t] = f.sub(x, rho[c][t]);
            }
        }
        // Gaussian elimination in increasing shifted degree.
        let mut order: Vec<usize> = (0..k).collect();
        order.sort_by_key(|&c| (delta[c], c));
        let mut pivots: Vec<(usize, usize, u64)> = vec![]; // (column, row, pivot⁻¹)
        let mut is_piv = vec![false; k];
        for &c in &order {
            for pi in 0..pivots.len() {
                let (p, tr, inv) = pivots[pi];
                let x = res[c][tr];
                if x == 0 {
                    continue;
                }
                cn.mulmods += 1;
                let fm = f.pre(f.mul(x, inv));
                cn.mulmods += (2 * bm) as u64;
                for t in 0..bm {
                    res[c][t] = f.sub(res[c][t], f.mul_pre(res[p][t], fm));
                    rho[c][t] = f.sub(rho[c][t], f.mul_pre(rho[p][t], fm));
                }
                let fpp = fp[p].clone();
                if fp[c].len() < fpp.len() {
                    fp[c].resize(fpp.len(), 0);
                }
                for (j, &y) in fpp.iter().enumerate() {
                    if y != 0 {
                        cn.mulmods += 1;
                        fp[c][j] = f.sub(fp[c][j], f.mul_pre(y, fm));
                    }
                }
            }
            if let Some(tr) = (0..bm).find(|&t| res[c][t] != 0) {
                pivots.push((c, tr, f.inv(res[c][tr])));
                is_piv[c] = true;
            }
        }
        for c in 0..k {
            if is_piv[c] {
                if !fp[c].is_empty() {
                    fp[c].insert(0, 0);
                }
                delta[c] += 1;
            } else {
                for t in 0..bm {
                    rho[c][t] = 0;
                }
            }
        }
        sigma += 1;
        let dmin = (0..k)
            .filter(|&c| fp[c].iter().any(|&y| y != 0))
            .map(|c| delta[c])
            .min()
            .unwrap_or(usize::MAX);
        if dmin != usize::MAX && sigma >= dmin + dmin.div_ceil(bm) + ET {
            break;
        }
        if sigma >= maxlen {
            break;
        }
    }
    // Generator: the non-zero f column of least shifted degree.
    let g = (0..k)
        .filter(|&c| fp[c].iter().any(|&y| y != 0))
        .min_by_key(|&c| (delta[c], c))?;
    let l = delta[g];
    cn.degree += l as u64;
    let mut fg = fp[g].clone();
    fg.resize(l + 1, 0);
    let g0 = fg[l];
    if g0 == 0 || l == 0 || l > seq[0].len() {
        return None;
    }
    cn.mulmods += l as u64 + 1;
    let s = &seq[0];
    // g_i = f_{L−i}.
    let acc = dot(f, every, (1..=l).map(|ii| (fg[l - ii], s[ii - 1])));
    Some(f.mul(f.neg(acc), f.inv(g0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;
    use crate::solve::{solve_for_last, Solver};

    fn planted(
        rng: &mut Rng,
        ell: u64,
        m: usize,
        nrows: usize,
        weight: usize,
        unit: bool,
        kprob: u64,
    ) -> (Vec<(Row, u64)>, Vec<u64>) {
        let f = Fp::new(ell);
        let xs: Vec<u64> = (0..=m).map(|_| rng.below(ell)).collect();
        let mut rows = Vec::new();
        for _ in 0..nrows {
            let mut cols: Vec<u32> = (0..weight).map(|_| rng.below(m as u64) as u32).collect();
            if rng.below(kprob) != 0 {
                cols.push(m as u32);
            }
            cols.sort_unstable();
            cols.dedup();
            let r: Row = cols
                .into_iter()
                .map(|c| {
                    let v = if unit && c != m as u32 {
                        if rng.below(2) == 0 {
                            1
                        } else {
                            ell - 1
                        }
                    } else {
                        1 + rng.below(ell - 1)
                    };
                    (c, v)
                })
                .collect();
            let rhs = r
                .iter()
                .fold(0u64, |acc, &(c, v)| f.add(acc, f.mul(v, xs[c as usize])));
            rows.push((r, rhs));
        }
        (rows, xs)
    }

    /// On random planted systems (general and ±1 coefficients, `ℓ` from
    /// 10^6 to just under 2^62), whenever Fast determines `k`, SgeBw returns
    /// the same `k`, and the counters add up.
    #[test]
    fn sge_bw_matches_fast_random() {
        let ells = [
            1_000_003u64,
            269_286_877,
            201_001_232_461,
            (1u64 << 61) - 1,
            (1u64 << 62) - 57,
        ];
        let mut rng = Rng::new(20261115);
        let (mut agree, mut fast_some, mut bw_some_when_none) = (0, 0, 0);
        let mut bw_runs = 0;
        for trial in 0..150usize {
            let ell = ells[trial % ells.len()];
            let m = 30 + (trial % 7) * 40;
            let weight = 2 + trial % 4;
            let unit = trial % 2 == 0;
            let nrows = match trial % 5 {
                0 => m / 2,
                1 => m,
                _ => m + 8,
            };
            let (rows, xs) = planted(&mut rng, ell, m, nrows, weight, unit, 4);
            for &(cx, block) in &[(DEFAULT_CX, DEFAULT_BLOCK), (0.0, 1), (1e9, 3)] {
                let mut sf = SolveStats::default();
                let mut sb = SolveStats::default();
                let kf = solve_for_last(&rows, m, ell, Solver::Fast, &[], &mut sf);
                let kb = solve_sge_bw_with(&rows, m, ell, cx, block, &mut sb);
                assert_eq!(sb.mulmods, sb.sge_mulmods + sb.bw_mulmods);
                if sb.matvecs > 0 {
                    bw_runs += 1;
                }
                match kf {
                    Some(k) => {
                        assert_eq!(k, xs[m]);
                        fast_some += 1;
                        assert_eq!(kb, Some(k), "trial={trial} m={m} ell={ell} cx={cx} b={block}");
                        agree += 1;
                    }
                    None => {
                        if kb.is_some() {
                            bw_some_when_none += 1;
                        }
                    }
                }
            }
        }
        eprintln!(
            "fast_some={fast_some} agree={agree} bw_some_when_fast_none={bw_some_when_none} bw_runs={bw_runs}"
        );
        assert!(fast_some >= 150 && bw_runs >= 100, "coverage: {fast_some} {bw_runs}");
    }

    /// Sieve-shaped systems: `g + 1` distinct ±1 base entries, `k`
    /// coefficient `−β` with few distinct `β` (one per line), at sizes where
    /// Wiedemann runs on a non-trivial residual.
    #[test]
    fn sge_bw_matches_fast_sieve_shaped() {
        let mut rng = Rng::new(7);
        let mut runs_with_bw = 0;
        for trial in 0..24usize {
            let g = 2 + trial % 3;
            let m = 200 + 150 * (trial % 4);
            let ell = [1_000_003u64, 31_978_869_199, (1u64 << 62) - 57][trial % 3];
            let f = Fp::new(ell);
            let xs: Vec<u64> = (0..=m).map(|_| rng.below(ell)).collect();
            let mut rows = Vec::new();
            let mut beta = 1u64;
            while rows.len() < m + 8 {
                // One line: a run of rows sharing the k coefficient −β.
                beta += rng.below(2);
                let kv = f.neg(beta % ell);
                for _ in 0..(m / 4).max(3) {
                    let mut cols: Vec<u32> = Vec::new();
                    while cols.len() < g + 1 {
                        let c = rng.below(m as u64) as u32;
                        if !cols.contains(&c) {
                            cols.push(c);
                        }
                    }
                    cols.sort_unstable();
                    let mut r: Row = cols
                        .iter()
                        .map(|&c| (c, if rng.below(2) == 0 { 1 } else { ell - 1 }))
                        .collect();
                    r.push((m as u32, kv));
                    let rhs = r
                        .iter()
                        .fold(0, |acc, &(c, v)| f.add(acc, f.mul(v, xs[c as usize])));
                    rows.push((r, rhs));
                    if rows.len() >= m + 8 {
                        break;
                    }
                }
            }
            let mut sf = SolveStats::default();
            let mut sb = SolveStats::default();
            let kf = solve_for_last(&rows, m, ell, Solver::Fast, &[], &mut sf);
            let kb = solve_sge_bw_with(&rows, m, ell, DEFAULT_CX, DEFAULT_BLOCK, &mut sb);
            if sb.matvecs > 0 {
                runs_with_bw += 1;
            }
            if let Some(k) = kf {
                assert_eq!(k, xs[m]);
                assert_eq!(kb, Some(k), "trial={trial} g={g} m={m}");
            }
        }
        assert!(runs_with_bw >= 8, "Wiedemann ran on {runs_with_bw} systems");
    }

    /// End to end: the line sieve with `Solver::SgeBw` recovers the planted
    /// `k` on small rows of every genus, the same `k` as with `Solver::Fast`.
    #[test]
    fn sieve_with_sge_bw() {
        use crate::instance::pick_instance;
        use crate::sieve::{solve_dlp_sieve, SieveParams};
        for &(g, p) in &[(2usize, 101u64), (2, 503), (3, 61), (3, 211), (4, 31), (4, 61)] {
            let inst = pick_instance(p, g).unwrap();
            let k_true = inst.ell / 3 + 7;
            let d2 = inst.curve.mul(&inst.d1, k_true);
            for seed in 0..3u64 {
                let run = |solver| {
                    let params = SieveParams {
                        seed: 1000 + seed,
                        solver,
                        ..SieveParams::default()
                    };
                    solve_dlp_sieve(&inst.curve, &inst.d1, &d2, inst.ell, &params)
                };
                let (rf, rb) = (run(Solver::Fast), run(Solver::SgeBw));
                assert_eq!(rb.k, Some(k_true), "g={g} p={p} seed={seed}");
                assert_eq!(rb.k, rf.k, "g={g} p={p} seed={seed}");
                // A singular square residual shows up as extra rounds.
                assert_eq!(rb.solve_rounds, 1, "g={g} p={p} seed={seed}");
                assert_eq!(rb.solve.mulmods, rb.solve.sge_mulmods + rb.solve.bw_mulmods);
            }
        }
    }
}
