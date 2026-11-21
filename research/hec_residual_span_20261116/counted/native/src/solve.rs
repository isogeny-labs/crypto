//! Sparse elimination mod a prime `ℓ < 2^62` that recovers one unknown.
//!
//! Unknowns are columns `0..m` (factor-base logarithms) and column `m`
//! (the target `k`).  Each row is `Σ coef·x_col = rhs`.  Markowitz-style
//! elimination: repeatedly take the factor-base column with the fewest
//! non-zeros, pivot on its shortest row, eliminate it from the other rows
//! and drop the pivot row (no back-substitution is needed: only `k` is
//! wanted).  A row left with only the `k` column determines `k`.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::fp::Fp;

pub type Row = Vec<(u32, u64)>;

#[derive(Clone, Copy, Debug, Default)]
pub struct SolveStats {
    /// FNV-style hash of the `(column, pivot row)` sequence: two solvers with
    /// equal hashes made the same pivot choices in the same order.
    pub pivot_hash: u64,
    pub pivots: u64,
    pub row_ops: u64,
    pub mulmods: u64,
    pub max_row_len: usize,
}

/// Reference and Fast make identical pivots and perform identical arithmetic.
/// Hybrid changes the elimination order and uses a dense residual block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Solver {
    /// The S1 solver, kept as the exactness reference.
    Reference,
    /// S2a: holder lists get a row id only for columns new to that row, no
    /// list for `k`, merge buffers reused.
    #[default]
    Fast,
    /// Sparse elimination followed by a dense block (caller supplies hot columns).
    Hybrid,
    /// S2b (PLAN.md §8): the step columns and `k` are a dense block per row;
    /// sparse columns are eliminated first, the residual block densely.
    /// Different pivot order; same `Option<k>` on every row set.
    Dense,
}

#[inline]
fn mix(h: u64, c: u32, r: u32) -> u64 {
    (h ^ (((c as u64) << 32) | r as u64)).wrapping_mul(0x0000_0100_0000_01B3)
}

/// `dense_cols` names the factor-base columns that appear in nearly every
/// row (the walk's step places); only [`Solver::Dense`] uses it.
pub fn solve_for_last(
    rows: &[(Row, u64)],
    m: usize,
    ell: u64,
    solver: Solver,
    dense_cols: &[u32],
    stats: &mut SolveStats,
) -> Option<u64> {
    match solver {
        Solver::Reference => solve_reference(rows, m, ell, stats),
        Solver::Fast => solve_fast(rows, m, ell, stats),
        Solver::Hybrid => crate::hybrid::solve(rows, m, ell, &[], stats),
        Solver::Dense => solve_dense(rows, m, ell, dense_cols, stats),
    }
}

/// `Some(k)` if the rows determine column `m`; `None` otherwise.
/// Rows must be sorted by column with non-zero coefficients reduced mod `ℓ`.
/// This is the S1 solver, unchanged except for the pivot hash.
pub fn solve_reference(
    rows: &[(Row, u64)],
    m: usize,
    ell: u64,
    stats: &mut SolveStats,
) -> Option<u64> {
    let f = Fp::new_any(ell);
    let kcol = m as u32;
    let mut live: Vec<Option<(Row, u64)>> = rows.iter().cloned().map(Some).collect();
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); m + 1];
    let mut count: Vec<u32> = vec![0; m + 1];
    for (ri, (r, _)) in rows.iter().enumerate() {
        for &(c, _) in r {
            col_rows[c as usize].push(ri as u32);
            count[c as usize] += 1;
        }
    }
    let mut done = vec![false; m + 1];
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = (0..m as u32)
        .filter(|&c| count[c as usize] > 0)
        .map(|c| Reverse((count[c as usize], c)))
        .collect();

    // A row that is already k-only answers immediately.
    let try_k = |row: &Row, rhs: u64| -> Option<u64> {
        if row.len() == 1 && row[0].0 == kcol {
            Some(f.mul(rhs, f.inv(row[0].1)))
        } else {
            None
        }
    };
    for (r, rhs) in rows {
        if let Some(k) = try_k(r, *rhs) {
            return Some(k);
        }
    }

    while let Some(Reverse((cnt, c))) = heap.pop() {
        let cu = c as usize;
        if done[cu] || cnt != count[cu] || count[cu] == 0 {
            continue;
        }
        // Pivot: shortest live row containing c.
        let mut holders: Vec<u32> = std::mem::take(&mut col_rows[cu]);
        holders.sort_unstable();
        holders.dedup();
        holders.retain(|&ri| {
            live[ri as usize]
                .as_ref()
                .is_some_and(|(r, _)| r.binary_search_by_key(&c, |e| e.0).is_ok())
        });
        col_rows[cu].clear();
        let Some(&pr) = holders
            .iter()
            .min_by_key(|&&ri| live[ri as usize].as_ref().unwrap().0.len())
        else {
            continue;
        };
        done[cu] = true;
        stats.pivots += 1;
        stats.pivot_hash = mix(stats.pivot_hash, c, pr);
        let (prow, prhs) = live[pr as usize].take().unwrap();
        // Every column whose count changes gets a fresh heap entry below;
        // entries left with an old count are skipped as stale.
        let mut touched: Vec<u32> = Vec::with_capacity(prow.len() * 4);
        for &(cc, _) in &prow {
            count[cc as usize] -= 1;
            touched.push(cc);
        }
        let pval = prow[prow.binary_search_by_key(&c, |e| e.0).unwrap()].1;
        let pinv = f.inv(pval);
        for &ri in &holders {
            if ri == pr {
                continue;
            }
            let (row, rhs) = live[ri as usize].take().unwrap();
            let rv = row[row.binary_search_by_key(&c, |e| e.0).unwrap()].1;
            let factor = f.mul(rv, pinv); // row −= factor · prow
            for &(cc, _) in &row {
                count[cc as usize] -= 1;
                touched.push(cc);
            }
            let mut out: Row = Vec::with_capacity(row.len() + prow.len());
            let (mut i, mut j) = (0, 0);
            while i < row.len() || j < prow.len() {
                let take_row = j >= prow.len() || (i < row.len() && row[i].0 < prow[j].0);
                let take_p = i >= row.len() || (j < prow.len() && prow[j].0 < row[i].0);
                if take_row {
                    out.push(row[i]);
                    i += 1;
                } else if take_p {
                    out.push((prow[j].0, f.neg(f.mul(factor, prow[j].1))));
                    stats.mulmods += 1;
                    j += 1;
                } else {
                    let v = f.sub(row[i].1, f.mul(factor, prow[j].1));
                    stats.mulmods += 1;
                    if v != 0 {
                        out.push((row[i].0, v));
                    }
                    i += 1;
                    j += 1;
                }
            }
            let new_rhs = f.sub(rhs, f.mul(factor, prhs));
            stats.row_ops += 1;
            stats.max_row_len = stats.max_row_len.max(out.len());
            if out.is_empty() {
                // Dependent (rhs must be 0 for a consistent system).
                debug_assert_eq!(new_rhs, 0, "inconsistent relation system");
                continue;
            }
            if let Some(k) = try_k(&out, new_rhs) {
                return Some(k);
            }
            for &(cc, _) in &out {
                let cu2 = cc as usize;
                count[cu2] += 1;
                touched.push(cc);
                // Index lists may hold duplicates and stale ids; the holder
                // filter above re-checks membership.
                col_rows[cu2].push(ri);
            }
            live[ri as usize] = Some((out, new_rhs));
        }
        touched.sort_unstable();
        touched.dedup();
        for cc in touched {
            let cu2 = cc as usize;
            if cc != kcol && !done[cu2] && count[cu2] > 0 {
                heap.push(Reverse((count[cu2], cc)));
            }
        }
    }
    None
}

/// S2a: the same elimination as [`solve_reference`] — same pivot order,
/// same pivot rows, same arithmetic — with cheaper bookkeeping:
///
/// - a row id is appended to a column's holder list only when the column is
///   new to that row (it came in from the pivot row), so the hot step
///   columns' lists no longer grow by one entry per row operation;
/// - no holder list is kept for `k`, which is never pivoted;
/// - counts change per entry during the merge (unchanged columns are not
///   decremented and re-incremented), and only changed columns get a fresh
///   heap entry;
/// - merge output goes into a reused buffer, swapped with the old row.
///
/// Heap order is unchanged: every live column always has an entry keyed by
/// its current count, and [`solve_reference`]'s extra pushes of unchanged
/// keys are duplicates of that entry, which `done` later skips.
pub fn solve_fast(rows: &[(Row, u64)], m: usize, ell: u64, stats: &mut SolveStats) -> Option<u64> {
    let f = Fp::new_any(ell);
    let kcol = m as u32;
    for (r, rhs) in rows {
        if r.len() == 1 && r[0].0 == kcol {
            return Some(f.mul(*rhs, f.inv(r[0].1)));
        }
    }
    let mut live: Vec<Option<(Row, u64)>> = rows.iter().cloned().map(Some).collect();
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); m];
    let mut count: Vec<u32> = vec![0; m + 1];
    for (ri, (r, _)) in rows.iter().enumerate() {
        for &(c, _) in r {
            if c != kcol {
                col_rows[c as usize].push(ri as u32);
            }
            count[c as usize] += 1;
        }
    }
    let mut done = vec![false; m + 1];
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = (0..m as u32)
        .filter(|&c| count[c as usize] > 0)
        .map(|c| Reverse((count[c as usize], c)))
        .collect();
    let mut buf: Row = Vec::new();
    let mut touched: Vec<u32> = Vec::new();

    while let Some(Reverse((cnt, c))) = heap.pop() {
        let cu = c as usize;
        if done[cu] || cnt != count[cu] || count[cu] == 0 {
            continue;
        }
        let mut holders: Vec<u32> = std::mem::take(&mut col_rows[cu]);
        holders.sort_unstable();
        holders.dedup();
        holders.retain(|&ri| {
            live[ri as usize]
                .as_ref()
                .is_some_and(|(r, _)| r.binary_search_by_key(&c, |e| e.0).is_ok())
        });
        let Some(&pr) = holders
            .iter()
            .min_by_key(|&&ri| live[ri as usize].as_ref().unwrap().0.len())
        else {
            continue;
        };
        done[cu] = true;
        stats.pivots += 1;
        stats.pivot_hash = mix(stats.pivot_hash, c, pr);
        let (prow, prhs) = live[pr as usize].take().unwrap();
        touched.clear();
        for &(cc, _) in &prow {
            count[cc as usize] -= 1;
            touched.push(cc);
        }
        let pval = prow[prow.binary_search_by_key(&c, |e| e.0).unwrap()].1;
        let pinv = f.inv(pval);
        for &ri in &holders {
            if ri == pr {
                continue;
            }
            let (mut row, rhs) = live[ri as usize].take().unwrap();
            let rv = row[row.binary_search_by_key(&c, |e| e.0).unwrap()].1;
            let factor = f.mul(rv, pinv); // row −= factor · prow
            buf.clear();
            buf.reserve(row.len() + prow.len());
            let (mut i, mut j) = (0, 0);
            while i < row.len() || j < prow.len() {
                let take_row = j >= prow.len() || (i < row.len() && row[i].0 < prow[j].0);
                let take_p = i >= row.len() || (j < prow.len() && prow[j].0 < row[i].0);
                if take_row {
                    buf.push(row[i]);
                    i += 1;
                } else if take_p {
                    let cc = prow[j].0;
                    buf.push((cc, f.neg(f.mul(factor, prow[j].1))));
                    stats.mulmods += 1;
                    count[cc as usize] += 1;
                    touched.push(cc);
                    if cc != kcol {
                        col_rows[cc as usize].push(ri);
                    }
                    j += 1;
                } else {
                    let v = f.sub(row[i].1, f.mul(factor, prow[j].1));
                    stats.mulmods += 1;
                    if v != 0 {
                        buf.push((row[i].0, v));
                    } else {
                        count[row[i].0 as usize] -= 1;
                        touched.push(row[i].0);
                    }
                    i += 1;
                    j += 1;
                }
            }
            let new_rhs = f.sub(rhs, f.mul(factor, prhs));
            stats.row_ops += 1;
            stats.max_row_len = stats.max_row_len.max(buf.len());
            std::mem::swap(&mut row, &mut buf);
            if row.is_empty() {
                debug_assert_eq!(new_rhs, 0, "inconsistent relation system");
                continue;
            }
            if row.len() == 1 && row[0].0 == kcol {
                return Some(f.mul(new_rhs, f.inv(row[0].1)));
            }
            live[ri as usize] = Some((row, new_rhs));
        }
        touched.sort_unstable();
        touched.dedup();
        for &cc in &touched {
            let cu2 = cc as usize;
            if cc != kcol && !done[cu2] && count[cu2] > 0 {
                heap.push(Reverse((count[cu2], cc)));
            }
        }
    }
    None
}

/// S2b (PLAN.md §8): structured elimination with a dense block.
///
/// Each row splits into a sparse part (factor-base columns that are not in
/// `dense_cols`) and a dense array over `dense_cols` then `k` (`d` entries).
///
/// - Phase 1: Markowitz over the sparse columns only. The pivot column has
///   the fewest holders; the pivot row is the holder with the shortest sparse
///   part, lowest id on ties. A row operation merges sparse parts (S2a
///   bookkeeping) and does `row −= c·pivot` on the dense array, one REDC per
///   product. Pivot rows are dropped.
/// - Phase 2: the live rows now have empty sparse parts. Eliminate the
///   non-`k` dense columns in order, dropping pivot rows; a remaining row with
///   non-zero `k` gives `k`.
///
/// Exact elimination: after both phases the live rows span the row space's
/// intersection with the `k` axis, so this returns `Some(k)` exactly when the
/// rows determine `k`, as [`solve_fast`] does.
pub fn solve_dense(
    rows: &[(Row, u64)],
    m: usize,
    ell: u64,
    dense_cols: &[u32],
    stats: &mut SolveStats,
) -> Option<u64> {
    let f = Fp::new_any(ell);
    let d = dense_cols.len() + 1;
    let mut didx = vec![u32::MAX; m + 1];
    for (t, &c) in dense_cols.iter().enumerate() {
        assert!(
            (c as usize) < m && didx[c as usize] == u32::MAX,
            "dense_cols: distinct base columns"
        );
        didx[c as usize] = t as u32;
    }
    didx[m] = (d - 1) as u32;

    let n = rows.len();
    let mut sparse: Vec<Row> = Vec::with_capacity(n);
    let mut dense: Vec<u64> = vec![0; n * d];
    let mut rhs: Vec<u64> = Vec::with_capacity(n);
    let mut live = vec![true; n];
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); m];
    let mut count: Vec<u32> = vec![0; m];
    for (ri, (r, b)) in rows.iter().enumerate() {
        let mut s = Row::new();
        for &(c, v) in r {
            let t = didx[c as usize];
            if t == u32::MAX {
                s.push((c, v));
                col_rows[c as usize].push(ri as u32);
                count[c as usize] += 1;
            } else {
                dense[ri * d + t as usize] = v;
            }
        }
        sparse.push(s);
        rhs.push(*b);
    }

    // Phase 1: sparse columns.
    let mut done = vec![false; m];
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = (0..m as u32)
        .filter(|&c| count[c as usize] > 0)
        .map(|c| Reverse((count[c as usize], c)))
        .collect();
    let mut buf: Row = Vec::new();
    let mut touched: Vec<u32> = Vec::new();
    let mut pnz: Vec<(usize, u64)> = Vec::with_capacity(d);
    while let Some(Reverse((cnt, c))) = heap.pop() {
        let cu = c as usize;
        if done[cu] || cnt != count[cu] || count[cu] == 0 {
            continue;
        }
        let mut holders: Vec<u32> = std::mem::take(&mut col_rows[cu]);
        holders.sort_unstable();
        holders.dedup();
        holders.retain(|&ri| {
            live[ri as usize]
                && sparse[ri as usize]
                    .binary_search_by_key(&c, |e| e.0)
                    .is_ok()
        });
        let Some(&pr) = holders
            .iter()
            .min_by_key(|&&ri| (sparse[ri as usize].len(), ri))
        else {
            continue;
        };
        done[cu] = true;
        stats.pivots += 1;
        stats.pivot_hash = mix(stats.pivot_hash, c, pr);
        let pru = pr as usize;
        live[pru] = false;
        let prow = std::mem::take(&mut sparse[pru]);
        let prhs = rhs[pru];
        touched.clear();
        for &(cc, _) in &prow {
            count[cc as usize] -= 1;
            touched.push(cc);
        }
        let pval = prow[prow.binary_search_by_key(&c, |e| e.0).unwrap()].1;
        let pinv = f.inv(pval);
        pnz.clear();
        for t in 0..d {
            let v = dense[pru * d + t];
            if v != 0 {
                pnz.push((t, v));
            }
        }
        for &ri in &holders {
            if ri == pr {
                continue;
            }
            let riu = ri as usize;
            let row = std::mem::take(&mut sparse[riu]);
            let rv = row[row.binary_search_by_key(&c, |e| e.0).unwrap()].1;
            let fm = f.pre(f.mul(rv, pinv)); // row −= factor · prow
            buf.clear();
            buf.reserve(row.len() + prow.len());
            let (mut i, mut j) = (0, 0);
            while i < row.len() || j < prow.len() {
                let take_row = j >= prow.len() || (i < row.len() && row[i].0 < prow[j].0);
                let take_p = i >= row.len() || (j < prow.len() && prow[j].0 < row[i].0);
                if take_row {
                    buf.push(row[i]);
                    i += 1;
                } else if take_p {
                    let cc = prow[j].0;
                    buf.push((cc, f.neg(f.mul_pre(prow[j].1, fm))));
                    stats.mulmods += 1;
                    count[cc as usize] += 1;
                    touched.push(cc);
                    col_rows[cc as usize].push(ri);
                    j += 1;
                } else {
                    let v = f.sub(row[i].1, f.mul_pre(prow[j].1, fm));
                    stats.mulmods += 1;
                    if v != 0 {
                        buf.push((row[i].0, v));
                    } else {
                        count[row[i].0 as usize] -= 1;
                        touched.push(row[i].0);
                    }
                    i += 1;
                    j += 1;
                }
            }
            let base = riu * d;
            for &(t, pv) in &pnz {
                dense[base + t] = f.sub(dense[base + t], f.mul_pre(pv, fm));
            }
            rhs[riu] = f.sub(rhs[riu], f.mul_pre(prhs, fm));
            stats.mulmods += pnz.len() as u64 + 1;
            stats.row_ops += 1;
            stats.max_row_len = stats.max_row_len.max(buf.len() + d);
            sparse[riu] = std::mem::replace(&mut buf, row);
        }
        touched.sort_unstable();
        touched.dedup();
        for &cc in &touched {
            let cu2 = cc as usize;
            if !done[cu2] && count[cu2] > 0 {
                heap.push(Reverse((count[cu2], cc)));
            }
        }
    }

    // Phase 2: the dense block. Every live row's sparse part is now empty: a
    // sparse column with holders always has a heap entry and is pivoted.
    let mut rem: Vec<usize> = (0..n).filter(|&ri| live[ri]).collect();
    debug_assert!(rem.iter().all(|&ri| sparse[ri].is_empty()));
    for j in 0..d - 1 {
        let Some(pos) = rem.iter().position(|&ri| dense[ri * d + j] != 0) else {
            continue;
        };
        let pr = rem.remove(pos);
        stats.pivots += 1;
        stats.pivot_hash = mix(stats.pivot_hash, (m + j) as u32, pr as u32);
        let pinv = f.inv(dense[pr * d + j]);
        let prhs = rhs[pr];
        pnz.clear();
        for t in j + 1..d {
            let v = dense[pr * d + t];
            if v != 0 {
                pnz.push((t, v));
            }
        }
        for &ri in &rem {
            let v = dense[ri * d + j];
            if v == 0 {
                continue;
            }
            let fm = f.pre(f.mul(v, pinv));
            let base = ri * d;
            dense[base + j] = 0;
            for &(t, pv) in &pnz {
                dense[base + t] = f.sub(dense[base + t], f.mul_pre(pv, fm));
            }
            rhs[ri] = f.sub(rhs[ri], f.mul_pre(prhs, fm));
            stats.mulmods += pnz.len() as u64 + 2;
            stats.row_ops += 1;
        }
    }
    for &ri in &rem {
        let kv = dense[ri * d + d - 1];
        if kv != 0 {
            return Some(f.mul(rhs[ri], f.inv(kv)));
        }
        debug_assert_eq!(rhs[ri], 0, "inconsistent relation system");
    }
    None
}

impl Fp {
    /// Like [`Fp::new`] but accepts any odd prime `< 2^62` (the solver's
    /// modulus is the subgroup order `ℓ`, not the curve's `p`).
    pub fn new_any(p: u64) -> Self {
        Fp::new(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn recovers_planted_k() {
        let ell = 1_000_003u64;
        let f = Fp::new(ell);
        let mut rng = Rng::new(11);
        for trial in 0..20 {
            let m = 50 + trial * 10;
            let xs: Vec<u64> = (0..=m).map(|_| rng.below(ell)).collect();
            let mut rows = Vec::new();
            for _ in 0..m + 6 {
                let mut r: Row = Vec::new();
                let mut cols: Vec<u32> = (0..5).map(|_| rng.below(m as u64) as u32).collect();
                cols.push(m as u32);
                cols.sort_unstable();
                cols.dedup();
                for c in cols {
                    r.push((c, 1 + rng.below(ell - 1)));
                }
                let rhs = r
                    .iter()
                    .fold(0u64, |acc, &(c, v)| f.add(acc, f.mul(v, xs[c as usize])));
                rows.push((r, rhs));
            }
            let mut st_r = SolveStats::default();
            let mut st_f = SolveStats::default();
            let kr = solve_for_last(&rows, m, ell, Solver::Reference, &[], &mut st_r);
            let kf = solve_for_last(&rows, m, ell, Solver::Fast, &[], &mut st_f);
            assert_eq!(kr, Some(xs[m]), "m={m}");
            assert_eq!(kf, kr, "m={m}");
            assert_eq!(
                (st_f.pivot_hash, st_f.pivots, st_f.row_ops, st_f.mulmods),
                (st_r.pivot_hash, st_r.pivots, st_r.row_ops, st_r.mulmods),
                "m={m}"
            );
        }
    }

    /// G5 (PLAN.md §8): on random systems with a dense block, some
    /// determining `k` and some not, `Dense` returns what `Fast` returns.
    #[test]
    fn dense_matches_fast() {
        let ell = 1_000_003u64;
        let f = Fp::new(ell);
        let mut rng = Rng::new(29);
        let (mut some, mut none) = (0, 0);
        for trial in 0..90usize {
            let m = 40 + (trial % 6) * 15;
            let nd = 1 + rng.below(24) as usize;
            let mut order: Vec<u32> = (0..m as u32).collect();
            for i in 0..nd {
                let j = i + rng.below((m - i) as u64) as usize;
                order.swap(i, j);
            }
            let dense_cols = order[..nd].to_vec();
            let xs: Vec<u64> = (0..=m).map(|_| rng.below(ell)).collect();
            // A third of the systems have too few rows to pin k down.
            let nrows = match trial % 3 {
                0 => m / 3 + rng.below(m as u64 / 2) as usize,
                1 => m + 2,
                _ => m + 10,
            };
            let mut rows = Vec::new();
            for _ in 0..nrows {
                let mut cols: Vec<u32> = (0..1 + rng.below(4))
                    .map(|_| rng.below(m as u64) as u32)
                    .collect();
                for &c in &dense_cols {
                    if rng.below(4) != 0 {
                        cols.push(c);
                    }
                }
                if rng.below(8) != 0 {
                    cols.push(m as u32);
                }
                cols.sort_unstable();
                cols.dedup();
                let r: Row = cols
                    .into_iter()
                    .map(|c| (c, 1 + rng.below(ell - 1)))
                    .collect();
                let rhs = r
                    .iter()
                    .fold(0u64, |acc, &(c, v)| f.add(acc, f.mul(v, xs[c as usize])));
                rows.push((r, rhs));
            }
            let mut st_f = SolveStats::default();
            let mut st_d = SolveStats::default();
            let kf = solve_for_last(&rows, m, ell, Solver::Fast, &dense_cols, &mut st_f);
            let kd = solve_for_last(&rows, m, ell, Solver::Dense, &dense_cols, &mut st_d);
            assert_eq!(kd, kf, "trial={trial} m={m} nd={nd} rows={nrows}");
            match kf {
                Some(k) => {
                    assert_eq!(k, xs[m], "trial={trial}");
                    some += 1;
                }
                None => none += 1,
            }
        }
        assert!(
            some >= 20 && none >= 10,
            "coverage: some={some} none={none}"
        );
    }
}
