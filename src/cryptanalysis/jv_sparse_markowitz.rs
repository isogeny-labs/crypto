//! Online incremental minimum-Markowitz sparse Gaussian elimination and exact
//! codimension-one kernel certification modulo a prime subgroup order `l`.
//!
//! Designed for Phase-2 homogeneous relation matrices emitted by large-prime
//! graph collectors (`mean_row_weight ~ 16`, `columns ~ 150..800`), replacing
//! repeated full-matrix Lanczos checkpoints with:
//! 1. An `O(nnz)` exact column-support pre-gate (if `< columns - 1` columns are
//!    populated, rank `< columns - 1` is certified with zero modular products).
//! 2. Minimum-Markowitz-cost (`(row_weight - 1) * (col_degree - 1)`) sparse
//!    upper-triangular elimination with zero-fill singleton/unit fast paths.
//! 3. Persistent upper-triangular pivot retention across checkpoints so newly
//!    arriving rows reduce incrementally against existing pivots without
//!    refactoring previously ingested rows.
//! 4. Exact `O(nnz)` reverse-order sparse back-substitution of the unique 1D
//!    kernel vector `h` normalized with `h[columns - 1] = 1`.
//!
//! Every modular multiplication, addition/subtraction, inversion, and word
//! operation is explicitly charged through `jv_bitops`.

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::jv_bitops as work;
use crate::cryptanalysis::residual_walk::inv_mod;

#[inline]
fn mul_mod_l(a: u64, b: u64, l: u64) -> u64 {
    work::hook_mul(l);
    ((a as u128 * b as u128) % l as u128) as u64
}

#[inline]
fn add_mod_l(a: u64, b: u64, l: u64) -> u64 {
    work::hook_add(l);
    ((a as u128 + b as u128) % l as u128) as u64
}

#[inline]
fn sub_mod_l(a: u64, b: u64, l: u64) -> u64 {
    work::hook_add(l);
    if a >= b {
        a - b
    } else {
        ((a as u128 + l as u128 - b as u128) % l as u128) as u64
    }
}

#[inline]
fn neg_mod_l(a: u64, l: u64) -> u64 {
    if a == 0 {
        0
    } else {
        work::hook_add(l);
        l - a
    }
}

/// Canonicalize a sparse row by sorting columns ascending and summing duplicates mod `l`.
fn canonicalize_sparse_row(
    raw: &[(usize, u64)],
    columns: usize,
    l: u64,
) -> Result<Vec<(usize, u64)>, String> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    let mut tmp: Vec<(usize, u64)> = Vec::with_capacity(raw.len());
    for &(col, val) in raw {
        work::words(2);
        if col >= columns {
            return Err("sparse Markowitz column index out of bounds".into());
        }
        let v = val % l;
        if v != 0 {
            tmp.push((col, v));
        }
    }
    if tmp.len() > 1 {
        tmp.sort_unstable_by_key(|&(c, _)| c);
        work::words((tmp.len() * 2) as u64);
    }
    let mut out: Vec<(usize, u64)> = Vec::with_capacity(tmp.len());
    for (col, val) in tmp {
        if let Some(last) = out.last_mut() {
            if last.0 == col {
                last.1 = add_mod_l(last.1, val, l);
                if last.1 == 0 {
                    out.pop();
                }
                continue;
            }
        }
        out.push((col, val));
    }
    Ok(out)
}

/// Sparse linear combination `target - factor * pivot (mod l)` for sorted sparse rows.
/// Assumes `factor != 0` and `pivot` has coefficient `1` at `pivot_col` so `pivot_col` cancels.
fn eliminate_sparse_entry(
    target: &[(usize, u64)],
    pivot: &[(usize, u64)],
    factor: u64,
    l: u64,
) -> Vec<(usize, u64)> {
    let mut out = Vec::with_capacity(target.len() + pivot.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < target.len() && j < pivot.len() {
        work::words(2);
        let (tc, tv) = target[i];
        let (pc, pv) = pivot[j];
        if tc < pc {
            out.push((tc, tv));
            i += 1;
        } else if pc < tc {
            let prod = mul_mod_l(factor, pv, l);
            let neg = neg_mod_l(prod, l);
            if neg != 0 {
                out.push((pc, neg));
            }
            j += 1;
        } else {
            let prod = mul_mod_l(factor, pv, l);
            let diff = sub_mod_l(tv, prod, l);
            if diff != 0 {
                out.push((tc, diff));
            }
            i += 1;
            j += 1;
        }
    }
    while i < target.len() {
        work::words(1);
        out.push(target[i]);
        i += 1;
    }
    while j < pivot.len() {
        work::words(1);
        let (pc, pv) = pivot[j];
        let prod = mul_mod_l(factor, pv, l);
        let neg = neg_mod_l(prod, l);
        if neg != 0 {
            out.push((pc, neg));
        }
        j += 1;
    }
    out
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MarkowitzCheckpoint {
    pub b_count: u64,
    pub rows_seen: usize,
    pub active_columns: usize,
    pub rank_after: usize,
    pub pivot_nnz_total: usize,
    pub pre_gate_skipped: bool,
    pub certified_kernel: bool,
}

/// Online incremental minimum-Markowitz sparse echelon tracker modulo `l`.
pub struct SparseMarkowitzTracker {
    modulus: u64,
    columns: usize,
    next_rows: usize,
    surplus: usize,
    failed_stride: usize,
    max_checkpoints: usize,
    ingested_rows: usize,
    col_seen: Vec<bool>,
    distinct_cols_seen: usize,
    is_pivot_col: Vec<bool>,
    /// Ordered upper-triangular monic pivots `(pivot_col, sorted_row)`.
    pivots: Vec<(usize, Vec<(usize, u64)>)>,
    pub checkpoints: Vec<MarkowitzCheckpoint>,
    pub kernel: Option<Vec<u64>>,
    pub free_column: Option<usize>,
}

impl SparseMarkowitzTracker {
    pub fn new(
        modulus: u64,
        columns: usize,
        surplus: usize,
        failed_stride: usize,
        max_checkpoints: usize,
    ) -> Result<Self, String> {
        if modulus < 3 {
            return Err("sparse Markowitz modulus < 3".into());
        }
        if !(2..=4096).contains(&columns) {
            return Err("sparse Markowitz columns out of range".into());
        }
        if !(0..=256).contains(&surplus)
            || !(1..=256).contains(&failed_stride)
            || !(1..=64).contains(&max_checkpoints)
        {
            return Err("sparse Markowitz schedule out of range".into());
        }
        Ok(Self {
            modulus,
            columns,
            next_rows: (columns - 1) + surplus,
            surplus,
            failed_stride,
            max_checkpoints,
            ingested_rows: 0,
            col_seen: vec![false; columns],
            distinct_cols_seen: 0,
            is_pivot_col: vec![false; columns],
            pivots: Vec::with_capacity(columns),
            checkpoints: Vec::new(),
            kernel: None,
            free_column: None,
        })
    }

    pub fn due(&self, rows_len: usize) -> bool {
        self.kernel.is_none() && !self.exhausted() && rows_len >= self.next_rows
    }

    pub fn exhausted(&self) -> bool {
        self.kernel.is_none() && self.checkpoints.len() >= self.max_checkpoints
    }

    pub fn rank(&self) -> usize {
        self.pivots.len()
    }

    pub fn surplus(&self) -> usize {
        self.surplus
    }

    /// Run one incremental Markowitz elimination checkpoint on `rows[0..]`.
    pub fn check(&mut self, b_count: u64, rows: &[Vec<(usize, u64)>]) -> Result<(), String> {
        let l = self.modulus;
        let rank_target = self.columns - 1;

        // 1. Update distinct column support on newly arrived rows in O(nnz_new) word ops.
        for row in &rows[self.ingested_rows..] {
            for &(c, v) in row {
                work::words(2);
                if c >= self.columns {
                    return Err("column out of range in Markowitz pre-gate".into());
                }
                if v % l != 0 && !self.col_seen[c] {
                    self.col_seen[c] = true;
                    self.distinct_cols_seen += 1;
                }
            }
        }

        // Exact algebraic pre-gate: if fewer than `columns - 1` columns have any nonzero entry,
        // rank `< columns - 1` is guaranteed with zero modular arithmetic.
        if self.distinct_cols_seen < rank_target {
            self.next_rows = rows.len() + self.failed_stride;
            self.checkpoints.push(MarkowitzCheckpoint {
                b_count,
                rows_seen: rows.len(),
                active_columns: self.distinct_cols_seen,
                rank_after: self.pivots.len(),
                pivot_nnz_total: self.pivots.iter().map(|(_, r)| r.len()).sum(),
                pre_gate_skipped: true,
                certified_kernel: false,
            });
            return Ok(());
        }

        // 2. Reduce only newly arrived rows `rows[self.ingested_rows..]` through existing pivots.
        let mut pool: Vec<Vec<(usize, u64)>> = Vec::with_capacity(rows.len() - self.ingested_rows);
        for raw_row in &rows[self.ingested_rows..] {
            let mut r = canonicalize_sparse_row(raw_row, self.columns, l)?;
            if r.is_empty() {
                continue;
            }
            for (p_col, p_row) in &self.pivots {
                work::words(2);
                if let Ok(pos) = r.binary_search_by_key(p_col, |&(c, _)| c) {
                    let factor = r[pos].1;
                    if factor != 0 {
                        r = eliminate_sparse_entry(&r, p_row, factor, l);
                        if r.is_empty() {
                            break;
                        }
                    }
                }
            }
            if !r.is_empty() {
                pool.push(r);
            }
        }
        self.ingested_rows = rows.len();

        // 3. Minimum-Markowitz-cost sparse elimination on `pool`.
        if !pool.is_empty() && self.pivots.len() < rank_target {
            let mut alive = vec![true; pool.len()];
            let mut col_deg = vec![0usize; self.columns];
            let mut col_members: Vec<Vec<usize>> = vec![Vec::new(); self.columns];
            for (r_idx, row) in pool.iter().enumerate() {
                for &(c, _) in row {
                    col_deg[c] += 1;
                    col_members[c].push(r_idx);
                    work::words(2);
                }
            }

            while self.pivots.len() < rank_target {
                // Select `(best_r, best_c)` minimizing `(row_len - 1) * (col_deg[c] - 1)`.
                let mut best: Option<(usize, usize, usize, usize)> = None; // (markowitz_score, row_len, r_idx, col)
                'search: for (r_idx, is_alive) in alive.iter().enumerate() {
                    if !*is_alive {
                        continue;
                    }
                    let row = &pool[r_idx];
                    if row.is_empty() {
                        continue;
                    }
                    let rw_m1 = row.len() - 1;
                    if rw_m1 == 0 {
                        // Unit row: zero fill-in!
                        best = Some((0, 1, r_idx, row[0].0));
                        break 'search;
                    }
                    work::words(row.len() as u64);
                    for &(c, _) in row {
                        let cd_m1 = col_deg[c].saturating_sub(1);
                        let score = rw_m1 * cd_m1;
                        let cand = (score, row.len(), r_idx, c);
                        if best.is_none_or(|cur| cand < cur) {
                            best = Some(cand);
                            if score == 0 {
                                // Singleton column: zero fill-in!
                                break 'search;
                            }
                        }
                    }
                }

                let Some((_, _, piv_r, piv_c)) = best else {
                    break;
                };

                alive[piv_r] = false;
                let mut piv_row = std::mem::take(&mut pool[piv_r]);
                for &(c, _) in &piv_row {
                    col_deg[c] = col_deg[c].saturating_sub(1);
                    work::words(1);
                }

                // Scale `piv_row` so entry at `piv_c` is `1`.
                let piv_pos = piv_row
                    .binary_search_by_key(&piv_c, |&(c, _)| c)
                    .map_err(|_| "selected Markowitz pivot column missing from row")?;
                let lead = piv_row[piv_pos].1;
                if lead != 1 {
                    let inv_lead = inv_mod(lead, l); // charges `hook_inv(l)`
                    for (idx, entry) in piv_row.iter_mut().enumerate() {
                        if idx == piv_pos {
                            entry.1 = 1;
                        } else {
                            entry.1 = mul_mod_l(entry.1, inv_lead, l);
                        }
                    }
                }

                // Eliminate `piv_c` from every remaining alive row that contains `piv_c`.
                let members = std::mem::take(&mut col_members[piv_c]);
                for target_r in members {
                    work::words(2);
                    if !alive[target_r] {
                        continue;
                    }
                    let Ok(pos) = pool[target_r].binary_search_by_key(&piv_c, |&(c, _)| c) else {
                        continue;
                    };
                    let factor = pool[target_r][pos].1;
                    if factor == 0 {
                        continue;
                    }
                    let old_row = std::mem::take(&mut pool[target_r]);
                    for &(c, _) in &old_row {
                        col_deg[c] = col_deg[c].saturating_sub(1);
                    }
                    let new_row = eliminate_sparse_entry(&old_row, &piv_row, factor, l);
                    if new_row.is_empty() {
                        alive[target_r] = false;
                    } else {
                        for &(c, _) in &new_row {
                            col_deg[c] += 1;
                            if old_row.binary_search_by_key(&c, |&(oc, _)| oc).is_err() {
                                col_members[c].push(target_r);
                            }
                            work::words(2);
                        }
                        pool[target_r] = new_row;
                    }
                }

                self.is_pivot_col[piv_c] = true;
                self.pivots.push((piv_c, piv_row));
            }
        }

        // 4. If rank reached `columns - 1`, solve the unique 1D kernel by sparse back-substitution.
        let mut certified = false;
        if self.pivots.len() == rank_target {
            let free_col = (0..self.columns)
                .find(|&c| !self.is_pivot_col[c])
                .ok_or("missing free column at codimension-1 rank")?;
            work::words(self.columns as u64);

            let mut h = vec![0u64; self.columns];
            h[free_col] = 1;
            for (p_col, p_row) in self.pivots.iter().rev() {
                let mut sum = 0u64;
                for &(c, coef) in p_row {
                    work::words(2);
                    if c == *p_col {
                        continue;
                    }
                    let hv = h[c];
                    if hv != 0 {
                        let prod = mul_mod_l(coef, hv, l);
                        sum = add_mod_l(sum, prod, l);
                    }
                }
                h[*p_col] = neg_mod_l(sum, l);
            }

            // Normalize so `h[columns - 1] == 1` when `h[columns - 1] != 0` for drop-in
            // compatibility with `Quotient::from_rank_certified_kernel`.
            let last_col = self.columns - 1;
            let final_free = if h[last_col] == 1 {
                last_col
            } else if h[last_col] != 0 {
                let inv_last = inv_mod(h[last_col], l);
                for entry in &mut h {
                    work::words(1);
                    if *entry != 0 {
                        *entry = mul_mod_l(*entry, inv_last, l);
                    }
                }
                h[last_col] = 1;
                last_col
            } else {
                free_col
            };

            self.kernel = Some(h);
            self.free_column = Some(final_free);
            certified = true;
        } else {
            self.next_rows = rows.len() + self.failed_stride;
        }

        self.checkpoints.push(MarkowitzCheckpoint {
            b_count,
            rows_seen: rows.len(),
            active_columns: self.distinct_cols_seen,
            rank_after: self.pivots.len(),
            pivot_nnz_total: self.pivots.iter().map(|(_, r)| r.len()).sum(),
            pre_gate_skipped: false,
            certified_kernel: certified,
        });
        Ok(())
    }
}

/// Apply `acc = (acc + coef * val) mod l` using pure `ModrAdd`/`sub_mod_l` addition chains
/// whenever `coef <= 8` or `l - coef <= 8` (`> 99%` of entries in `SignedMatroidGraph`),
/// charging `0` `ModrMul`s on small integer entries.
#[inline]
fn accum_small_coef_mod_l(acc: u64, coef: u64, val: u64, l: u64) -> u64 {
    if coef == 0 || val == 0 {
        return acc;
    }
    let small_mul = |k: u64, v: u64| -> u64 {
        match k {
            1 => v,
            2 => add_mod_l(v, v, l),
            3 => {
                let v2 = add_mod_l(v, v, l);
                add_mod_l(v2, v, l)
            }
            4 => {
                let v2 = add_mod_l(v, v, l);
                add_mod_l(v2, v2, l)
            }
            5 => {
                let v2 = add_mod_l(v, v, l);
                let v4 = add_mod_l(v2, v2, l);
                add_mod_l(v4, v, l)
            }
            6 => {
                let v2 = add_mod_l(v, v, l);
                let v3 = add_mod_l(v2, v, l);
                add_mod_l(v3, v3, l)
            }
            7 => {
                let v2 = add_mod_l(v, v, l);
                let v3 = add_mod_l(v2, v, l);
                let v4 = add_mod_l(v2, v2, l);
                add_mod_l(v4, v3, l)
            }
            8 => {
                let v2 = add_mod_l(v, v, l);
                let v4 = add_mod_l(v2, v2, l);
                add_mod_l(v4, v4, l)
            }
            _ => unreachable!(),
        }
    };
    if coef <= 8 {
        let term = small_mul(coef, val);
        if acc == 0 {
            term
        } else {
            add_mod_l(acc, term, l)
        }
    } else {
        let neg = l - coef;
        if neg <= 8 {
            let term = small_mul(neg, val);
            sub_mod_l(acc, term, l)
        } else {
            let term = mul_mod_l(coef, val, l);
            if acc == 0 {
                term
            } else {
                add_mod_l(acc, term, l)
            }
        }
    }
}

/// Berlekamp-Massey over `F_l` returning connection coefficients `C(z) = 1 + c_1 z + ... + c_L z^L`
/// (`out[0] = 1, out[k] = c_k`) satisfying `s[n] + sum_{k=1}^L c_k s[n - k] == 0 mod l` for `n >= L`.
fn berlekamp_massey_mod_l(seq: &[u64], l: u64) -> Vec<u64> {
    let n = seq.len();
    let mut c = vec![0u64; n + 1];
    let mut b = vec![0u64; n + 1];
    c[0] = 1;
    b[0] = 1;
    let mut len_l = 0usize;
    let mut m_shift = 1usize;
    let mut b_disc = 1u64;

    for idx in 0..n {
        let mut d = seq[idx];
        for i in 1..=len_l {
            work::words(1);
            let ci = c[i];
            let si = seq[idx - i];
            if ci != 0 && si != 0 {
                d = add_mod_l(d, mul_mod_l(ci, si, l), l);
            }
        }
        if d == 0 {
            m_shift += 1;
        } else if 2 * len_l <= idx {
            let t = c[..=len_l].to_vec();
            let factor = mul_mod_l(d, inv_mod(b_disc, l), l);
            let new_len = idx + 1 - len_l;
            for i in 0..=len_l.min(n - m_shift) {
                work::words(1);
                let bi = b[i];
                if bi != 0 {
                    let prod = mul_mod_l(factor, bi, l);
                    c[i + m_shift] = sub_mod_l(c[i + m_shift], prod, l);
                }
            }
            len_l = new_len;
            b[..t.len()].copy_from_slice(&t);
            for slot in &mut b[t.len()..=len_l.min(n)] {
                *slot = 0;
            }
            b_disc = d;
            m_shift = 1;
        } else {
            let factor = mul_mod_l(d, inv_mod(b_disc, l), l);
            for i in 0..=len_l {
                work::words(1);
                let bi = b[i];
                if bi != 0 {
                    let prod = mul_mod_l(factor, bi, l);
                    c[i + m_shift] = sub_mod_l(c[i + m_shift], prod, l);
                }
            }
            m_shift += 1;
        }
    }
    c.truncate(len_l + 1);
    c
}

/// Addition-chain Scalar Wiedemann homogeneous codimension-1 kernel tracker modulo `l`.
///
/// Replaces fill-prone sparse Gaussian elimination and 3x dense-vector Lanczos checkpoints with:
/// 1. `O(nnz)` column-support pre-gating (`>= columns - 1` active columns + row surplus).
/// 2. Factored `N x N` (`N = columns - 1`) operator `A = S * B` that keeps all input rows in
///    their original ultra-sparse `±1, ±2` form (`0` `ModrMul`s on `|coef| <= 8`) and folds the
///    `s` surplus rows via `3 * s` scalar multiplications per SpMV.
/// 3. `{-1, +1}^N` linear projection `s_k = <u, A^k b_0>` (`0` `ModrMul`s!) with automatic
///    uniform-`F_l^N` retry if `u` ever misses an invariant factor, followed by exact `O(N^2)`
///    Berlekamp-Massey and full-matrix orthogonality verification `M * h == 0 mod l`.
pub struct WiedemannTracker {
    modulus: u64,
    columns: usize,
    next_rows: usize,
    failed_stride: usize,
    max_checkpoints: usize,
    seed: u64,
    ingested_support_rows: usize,
    col_seen: Vec<bool>,
    distinct_cols_seen: usize,
    attempts: usize,
    rank_lb: usize,
    pub kernel: Option<Vec<u64>>,
}

impl WiedemannTracker {
    pub fn new(
        modulus: u64,
        columns: usize,
        seed: u64,
        surplus: usize,
        failed_stride: usize,
        max_checkpoints: usize,
    ) -> Result<Self, String> {
        if modulus < 5 || !(2..=4096).contains(&columns) {
            return Err("WiedemannTracker parameters out of range".into());
        }
        Ok(Self {
            modulus,
            columns,
            next_rows: (columns - 1) + surplus,
            failed_stride: failed_stride.max(1),
            max_checkpoints: max_checkpoints.max(1),
            seed,
            ingested_support_rows: 0,
            col_seen: vec![false; columns],
            distinct_cols_seen: 0,
            attempts: 0,
            rank_lb: 0,
            kernel: None,
        })
    }

    pub fn due(&self, rows_len: usize) -> bool {
        self.kernel.is_none() && !self.exhausted() && rows_len >= self.next_rows
    }

    pub fn exhausted(&self) -> bool {
        self.kernel.is_none() && self.attempts >= self.max_checkpoints
    }

    pub fn rank(&self) -> usize {
        self.rank_lb
    }

    pub fn check(&mut self, _b_count: u64, rows: &[Vec<(usize, u64)>]) -> Result<(), String> {
        let l = self.modulus;
        let n = self.columns - 1;

        for row in &rows[self.ingested_support_rows..] {
            for &(c, v) in row {
                work::words(2);
                if c >= self.columns {
                    return Err("column out of range in Wiedemann pre-gate".into());
                }
                if v % l != 0 && !self.col_seen[c] {
                    self.col_seen[c] = true;
                    self.distinct_cols_seen += 1;
                }
            }
        }
        self.ingested_support_rows = rows.len();

        // Need all `columns` covered (or at least `n` columns, with last column present so `h[n] = 1`).
        if self.distinct_cols_seen < self.columns || !self.col_seen[n] || rows.len() < n {
            self.next_rows = rows.len() + self.failed_stride;
            return Ok(());
        }

        self.attempts += 1;

        // Canonicalize rows once and split into `B` (`cols 0..n`) and RHS `b_raw = -col[n] mod l`.
        let mut b_rows: Vec<Vec<(usize, u64)>> = Vec::with_capacity(rows.len());
        let mut rhs_raw: Vec<u64> = Vec::with_capacity(rows.len());
        for raw in rows {
            let canon = canonicalize_sparse_row(raw, self.columns, l)?;
            if canon.is_empty() {
                continue;
            }
            let mut b_entry = 0u64;
            let mut body = Vec::with_capacity(canon.len());
            for (c, v) in canon {
                if c == n {
                    b_entry = neg_mod_l(v, l);
                } else {
                    body.push((c, v));
                }
            }
            b_rows.push(body);
            rhs_raw.push(b_entry);
        }

        if b_rows.len() < n {
            self.next_rows = rows.len() + self.failed_stride;
            return Ok(());
        }

        // Deterministic PRNG without charging `jv_bitops` RNG state noise (`WordOp` charged explicitly).
        let mut z = self
            .seed
            .wrapping_add((self.attempts as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let mut next_u64 = || -> u64 {
            work::words(2);
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };

        // Fold surplus rows `n..b_rows.len()` into `0..n` via 4 random targets per surplus row
        // with random `F_l` scalars (`4 * surplus` ModrMuls per SpMV).
        let surplus_count = b_rows.len() - n;
        let mut folds: Vec<[(usize, u64); 4]> = Vec::with_capacity(surplus_count);
        for _ in 0..surplus_count {
            let mut arr = [(0usize, 0u64); 4];
            for slot in &mut arr {
                let idx = (next_u64() as usize) % n;
                let wt = (next_u64() % (l - 1)) + 1;
                *slot = (idx, wt);
            }
            folds.push(arr);
        }

        let fold_vec = |full: &[u64], out: &mut [u64]| {
            out.copy_from_slice(&full[..n]);
            work::words(n as u64);
            for (s_idx, targets) in folds.iter().enumerate() {
                let val = full[n + s_idx];
                if val == 0 {
                    continue;
                }
                for &(dst, wt) in targets {
                    let prod = mul_mod_l(wt, val, l);
                    out[dst] = add_mod_l(out[dst], prod, l);
                }
            }
        };

        let mut full_tmp = vec![0u64; b_rows.len()];
        let apply_a = |w: &[u64], full_buf: &mut [u64], out: &mut [u64]| {
            for (r_idx, row) in b_rows.iter().enumerate() {
                let mut acc = 0u64;
                for &(c, coef) in row {
                    work::words(1);
                    let wv = w[c];
                    if wv != 0 {
                        acc = accum_small_coef_mod_l(acc, coef, wv, l);
                    }
                }
                full_buf[r_idx] = acc;
            }
            fold_vec(full_buf, out);
        };

        let mut b0 = vec![0u64; n];
        fold_vec(&rhs_raw, &mut b0);

        // Try `u in {-1, +1}^N` first (`0` ModrMuls in `<u, w>`); fallback once to uniform `F_l^N` if needed.
        for mode in 0..2 {
            let mut u_signs = vec![false; n];
            let mut u_full = Vec::new();
            if mode == 0 {
                for slot in &mut u_signs {
                    *slot = (next_u64() & 1) != 0;
                }
            } else {
                u_full.resize(n, 0u64);
                for slot in &mut u_full {
                    *slot = (next_u64() % (l - 1)) + 1;
                }
            }

            let mut seq = Vec::with_capacity(2 * n);
            let mut w = b0.clone();
            let mut next_w = vec![0u64; n];
            for step in 0..(2 * n) {
                let mut dot = 0u64;
                if mode == 0 {
                    for i in 0..n {
                        work::words(1);
                        let wi = w[i];
                        if wi != 0 {
                            dot = if u_signs[i] {
                                sub_mod_l(dot, wi, l)
                            } else {
                                add_mod_l(dot, wi, l)
                            };
                        }
                    }
                } else {
                    for i in 0..n {
                        work::words(1);
                        let wi = w[i];
                        if wi != 0 {
                            dot = add_mod_l(dot, mul_mod_l(u_full[i], wi, l), l);
                        }
                    }
                }
                seq.push(dot);
                if step + 1 < 2 * n {
                    apply_a(&w, &mut full_tmp, &mut next_w);
                    std::mem::swap(&mut w, &mut next_w);
                }
            }

            let c_poly = berlekamp_massey_mod_l(&seq, l);
            let deg_l = c_poly.len().saturating_sub(1);
            self.rank_lb = self.rank_lb.max(deg_l.min(n));
            if deg_l == 0 {
                continue;
            }
            let c_last = c_poly[deg_l];
            if c_last == 0 {
                // Singular projected operator: if `mode == 0`, check if `deg_l < n` was merely a `+-1` collision;
                // if `deg_l < n` in `mode == 1` (or `c_last == 0` in `mode == 1`), rank `< n` genuinely!
                if mode == 0 {
                    continue;
                } else {
                    break;
                }
            }

            // Reconstruct `x = (-c_L)^{-1} * sum_{j=0}^{L-1} c_{L - 1 - j} A^j b_0` (`c_0 = 1`).
            let mut acc_x = vec![0u64; n];
            let mut cur = b0.clone();
            for j in 0..deg_l {
                let cj = c_poly[deg_l - 1 - j]; // for `j = deg_l - 1`, `c_0 = 1`!
                if cj == 1 {
                    for i in 0..n {
                        work::words(1);
                        if cur[i] != 0 {
                            acc_x[i] = add_mod_l(acc_x[i], cur[i], l);
                        }
                    }
                } else if cj != 0 {
                    for i in 0..n {
                        work::words(1);
                        let vi = cur[i];
                        if vi != 0 {
                            acc_x[i] = add_mod_l(acc_x[i], mul_mod_l(cj, vi, l), l);
                        }
                    }
                }
                if j + 1 < deg_l {
                    apply_a(&cur, &mut full_tmp, &mut next_w);
                    std::mem::swap(&mut cur, &mut next_w);
                }
            }

            let neg_inv_cl = inv_mod(neg_mod_l(c_last, l), l);
            let mut h = Vec::with_capacity(self.columns);
            for &xi in &acc_x {
                h.push(if xi == 0 {
                    0
                } else {
                    mul_mod_l(xi, neg_inv_cl, l)
                });
            }
            h.push(1u64);

            // Verify exact orthogonality `row . h == 0 mod l` across ALL raw rows!
            let mut all_orth = true;
            for raw in rows {
                let mut dot = 0u64;
                for &(col, coef) in raw {
                    work::words(1);
                    let hv = h[col];
                    if hv != 0 {
                        dot = accum_small_coef_mod_l(dot, coef % l, hv, l);
                    }
                }
                if dot != 0 {
                    all_orth = false;
                    break;
                }
            }

            if all_orth {
                self.rank_lb = n;
                self.kernel = Some(h);
                return Ok(());
            }
        }

        // Rank `< n` (or free column wasn't `n` yet); advance by `failed_stride`.
        self.next_rows = rows.len() + self.failed_stride;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn incremental_markowitz_recovers_exact_planted_kernel() {
        let l: u64 = 263_807_419_614_139_553;
        let cols = 64usize;
        let mut rng = StdRng::seed_from_u64(0xDEC0_DED1);
        let mut secret = vec![0u64; cols];
        for entry in secret.iter_mut().take(cols - 1) {
            *entry = rng.gen_range(1..l);
        }
        secret[cols - 1] = 1;

        // Build 58 rows first (insufficient rank), check incremental step, then add 14 more rows.
        let mut rows: Vec<Vec<(usize, u64)>> = Vec::new();
        for _ in 0..72 {
            let mut idxs: Vec<usize> = (0..6).map(|_| rng.gen_range(0..cols - 1)).collect();
            idxs.sort_unstable();
            idxs.dedup();
            let mut sum = 0u128;
            let mut r = Vec::new();
            for c in idxs {
                let coef = rng.gen_range(1..l);
                sum = (sum + coef as u128 * secret[c] as u128) % l as u128;
                r.push((c, coef));
            }
            let last_coef = ((l as u128 - sum) % l as u128) as u64;
            if last_coef != 0 {
                r.push((cols - 1, last_coef));
            }
            rows.push(r);
        }

        let mut tracker = SparseMarkowitzTracker::new(l, cols, 2, 8, 16).unwrap();
        // First pass on 60 rows (may or may not be full rank).
        tracker.check(1024, &rows[..65]).unwrap();
        if tracker.kernel.is_none() {
            tracker.check(2048, &rows).unwrap();
        }
        let h = tracker.kernel.expect("certified kernel");
        assert_eq!(h, secret);
        for r in &rows {
            let mut dot = 0u128;
            for &(c, v) in r {
                dot = (dot + v as u128 * h[c] as u128) % l as u128;
            }
            assert_eq!(dot, 0);
        }
    }
}
