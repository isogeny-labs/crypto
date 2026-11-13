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
    pub pivots: u64,
    pub row_ops: u64,
    pub mulmods: u64,
    pub max_row_len: usize,
}

/// `Some(k)` if the rows determine column `m`; `None` otherwise.
/// Rows must be sorted by column with non-zero coefficients reduced mod `ℓ`.
pub fn solve_for_last(
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
            let mut st = SolveStats::default();
            let k = solve_for_last(&rows, m, ell, &mut st);
            assert_eq!(k, Some(xs[m]), "m={m}");
        }
    }
}
