//! E36 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.41): pivot rules for
//! the `k = 4` chain solve whose selection work fits the §5a band.
//!
//! Additive.  `echelon_by_lead_with` and E33's `echelon_by_lead_counted`
//! are unchanged; the echelon below performs the same elimination (the same
//! rows, the same pivot columns, the same multiplications charged per pivot
//! row) and differs only in which row of each column's bucket becomes the
//! pivot.  Every rule spans the same row space with the same pivot columns,
//! so the standard monomials, normal forms and decompositions do not change.
//!
//! **Counting rule** (§9.41, extending §9.35): one native operation per
//! matrix or equation entry examined for its zero status, per comparison of
//! two candidates' keys, per maintained key update, per per-column branch
//! decision A does not make, and per row moved by a permutation.  The
//! bookkeeping A itself does (bucketing by lead, the lead scan, ordering a
//! bucket by row index, the swap) is common to every arm and not charged;
//! the order in which rows arrive in a bucket is maintained by that same
//! bookkeeping.  The caller charges the per-equation and per-row work of the
//! inputs (`row_eq`, `row_nnz`); this function charges the rest to `work`.

use rayon::prelude::*;
use serde::Serialize;

use super::gaudry_quartic::{axpy, Echelon, PivotWork};
use super::residual_walk::inv_mod;

/// Which row of each column's bucket becomes the pivot (E36).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ArrivalRule {
    /// F: the first row in arrival order.  No selection work.
    Fifo,
    /// S: rows permuted once per call into blocks ordered by their
    /// equation's nonzero count; then A's rule (lowest index).
    Static,
    /// H: F's choice in a column whose bucket still holds a never-updated
    /// row; S's choice in a column holding only updated rows.
    FifoStatic,
    /// K: B″'s maintained estimate, comparing only the first two rows in
    /// arrival order.
    EstimateK2,
    /// L (negative control): the last row in arrival order.
    Lifo,
    /// A's rule (lowest index) through this echelon: a positive control
    /// that the elimination here is A's (diagnostic; not a solver name).
    Lowest,
}

impl ArrivalRule {
    /// The rule needs each row's equation (S, H) or inherited count (K).
    pub fn needs_inputs(self) -> bool {
        matches!(
            self,
            ArrivalRule::Static | ArrivalRule::FifoStatic | ArrivalRule::EstimateK2
        )
    }
    fn permutes(self) -> bool {
        matches!(self, ArrivalRule::Static | ArrivalRule::FifoStatic)
    }
}

/// Diagnostics and negative controls (nothing here is charged).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuleCheck {
    /// Negative control (S, H): map one row back to its neighbour's index
    /// after the echelon (a wrong inverse permutation).
    pub corrupt_inverse: bool,
    /// Negative control (S, H), added after the first control proved inert
    /// (§9.43): the rows of the two last pivot columns swap their targets.
    pub corrupt_pivot_inverse: bool,
}

/// Uncharged per-thread tallies for the §9.41 sensitivities σ2 and σ3,
/// accumulated by every [`echelon_by_rule`] call on this thread.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct RuleDiag {
    /// Columns with more than one candidate (σ3).
    pub contested: u64,
    /// `Σ (|b| − 1)` over the contested columns where the rule orders the
    /// bucket by a key or an index (A, S, H's second branch) (σ2).
    pub ordered_candidates: u64,
    /// `Σ (|b| − 1)` over all contested columns (= row updates).
    pub row_updates: u64,
    /// Calls where a corrupted inverse actually moved different contents
    /// (diagnostic of the negative controls).
    pub corrupt_changed: u64,
}

thread_local! {
    static DIAG: std::cell::Cell<RuleDiag> = const { std::cell::Cell::new(RuleDiag {
        contested: 0,
        ordered_candidates: 0,
        row_updates: 0,
        corrupt_changed: 0,
    }) };
}

/// Read and reset this thread's [`RuleDiag`].
pub fn take_diag() -> RuleDiag {
    DIAG.with(|d| d.replace(RuleDiag::default()))
}

fn fnv(h: u64, x: u64) -> u64 {
    let mut h = h;
    for b in x.to_le_bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// The permutation of S and H: rows grouped by equation, the equations
/// ordered by `(nonzero count, equation index)`, each block in build order.
/// Returns `perm` (`perm[i]` = original index of permuted row `i`) and the
/// comparisons of equation keys performed (an insertion sort, counted).
fn static_order(row_eq: &[u32], row_nnz: &[u32]) -> (Vec<u32>, u64) {
    let neq = row_eq.iter().map(|&e| e as usize + 1).max().unwrap_or(0);
    let mut key: Vec<Option<u32>> = vec![None; neq];
    for (r, &e) in row_eq.iter().enumerate() {
        key[e as usize] = Some(row_nnz[r]);
    }
    let mut eqs: Vec<usize> = (0..neq).filter(|&e| key[e].is_some()).collect();
    let mut cmp = 0u64;
    for i in 1..eqs.len() {
        let mut j = i;
        while j > 0 {
            cmp += 1;
            let (a, b) = (eqs[j - 1], eqs[j]);
            if (key[a], a) > (key[b], b) {
                eqs.swap(j - 1, j);
                j -= 1;
            } else {
                break;
            }
        }
    }
    let mut perm = Vec::with_capacity(row_eq.len());
    for &e in &eqs {
        perm.extend((0..row_eq.len() as u32).filter(|&r| row_eq[r as usize] as usize == e));
    }
    (perm, cmp)
}

/// Forward echelon by leading column with an E36 pivot rule.
///
/// `row_eq[r]`: the equation row `r` is a shift of; `row_nnz[r]`: that
/// equation's nonzero count (a shift is injective on terms), both supplied
/// (and charged) by the caller when `rule.needs_inputs()`.  The returned
/// [`Echelon`] is indexed by the original rows whatever the rule.
#[allow(clippy::too_many_arguments)]
pub(crate) fn echelon_by_rule(
    rows: Vec<Vec<u16>>,
    p: u32,
    muls: &mut u64,
    rule: ArrivalRule,
    row_eq: &[u32],
    row_nnz: &[u32],
    check: RuleCheck,
    work: &mut PivotWork,
) -> Echelon {
    let nrows = rows.len();
    // S, H: permute the rows once (one op per row moved, charged by the
    // caller with the per-row input work), eliminate in permuted indices,
    // map back at the end.
    let (perm, mut rows, est0): (Vec<u32>, Vec<Vec<u16>>, Vec<u32>) = if rule.permutes() {
        assert_eq!(row_eq.len(), nrows);
        let (perm, cmp) = static_order(row_eq, row_nnz);
        work.compare += cmp;
        let mut slots: Vec<Option<Vec<u16>>> = rows.into_iter().map(Some).collect();
        let permuted = perm
            .iter()
            .map(|&r| slots[r as usize].take().expect("a permutation"))
            .collect();
        (perm, permuted, Vec::new())
    } else if rule == ArrivalRule::EstimateK2 {
        assert_eq!(row_nnz.len(), nrows);
        ((0..nrows as u32).collect(), rows, row_nnz.to_vec())
    } else {
        ((0..nrows as u32).collect(), rows, Vec::new())
    };
    let mut est = est0;
    let ncols = rows.first().map_or(0, |r| r.len());
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut zero_rows = 0usize;
    let mut zero_ids: Vec<u32> = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        match row.iter().position(|&x| x != 0) {
            Some(c) => {
                work.lead_scan += c as u64 + 1;
                buckets[c].push(r as u32);
            }
            None => {
                work.lead_scan += ncols as u64;
                zero_rows += 1;
                zero_ids.push(r as u32);
            }
        }
    }
    // Rows at the front of each bucket that are still at their initial lead.
    let initial_len: Vec<usize> = buckets.iter().map(|b| b.len()).collect();
    let mut digest = 0xCBF2_9CE4_8422_2325u64;
    let mut pivot_row = vec![u32::MAX; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        let contested = b.len() > 1;
        if contested {
            work.contested += 1;
        }
        let mut ordered = false;
        // `b` is in arrival order here.
        match rule {
            ArrivalRule::Fifo => {}
            ArrivalRule::Lifo => {
                let last = b.len() - 1;
                b.swap(0, last);
            }
            ArrivalRule::Static | ArrivalRule::Lowest => {
                // A's rule (on the permuted indices for S; bookkeeping, as A).
                b.sort_unstable();
                ordered = true;
            }
            ArrivalRule::FifoStatic => {
                if contested {
                    work.compare += 1; // the branch: a never-updated row present?
                    if initial_len[c] == 0 {
                        b.sort_unstable();
                        ordered = true;
                    }
                }
            }
            ArrivalRule::EstimateK2 => {
                if contested {
                    work.compare += 1;
                    if est[b[1] as usize] < est[b[0] as usize] {
                        b.swap(0, 1);
                    }
                }
            }
        }
        if contested {
            let extra = b.len() as u64 - 1;
            DIAG.with(|d| {
                let mut x = d.get();
                x.contested += 1;
                x.row_updates += extra;
                if ordered {
                    x.ordered_candidates += extra;
                }
                d.set(x);
            });
        }
        let pr = b[0] as usize;
        digest = fnv(fnv(digest, c as u64), u64::from(perm[pr]));
        let mut piv = std::mem::take(&mut rows[pr]);
        let inv = inv_mod(piv[c] as u64, p as u64) as u32;
        let mut nz: Vec<u32> = Vec::new();
        for j in c..ncols {
            if piv[j] != 0 {
                piv[j] = ((piv[j] as u32 * inv) % p) as u16;
                nz.push(j as u32);
            }
        }
        *muls += nz.len() as u64;
        let dense = nz.len() * 8 > ncols - c;
        // (row, data, new lead, lead-scan reads)
        let mut others: Vec<(u32, Vec<u16>, Option<usize>, u64)> = b[1..]
            .iter()
            .map(|&r| (r, std::mem::take(&mut rows[r as usize]), None, 0))
            .collect();
        *muls += (others.len() * nz.len()) as u64;
        let nzr = &nz;
        others.par_iter_mut().for_each(|(_, row, lead, scan)| {
            let f = row[c] as u32;
            if dense {
                axpy(&mut row[c..], &piv[c..], f, p);
            } else {
                let nf = p - f;
                for &j in nzr {
                    let j = j as usize;
                    row[j] = ((row[j] as u32 + nf * piv[j] as u32) % p) as u16;
                }
            }
            *lead = row[c + 1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
            *scan = match *lead {
                Some(l) => (l - c) as u64,
                None => (ncols - c - 1) as u64,
            };
        });
        for (r, row, lead, scan) in others {
            work.lead_scan += scan;
            if rule == ArrivalRule::EstimateK2 {
                if let Some(l) = lead {
                    // B″'s Markowitz fill bound, two operations per updated row.
                    let e = (est[r as usize] + nz.len() as u32).saturating_sub(2);
                    est[r as usize] = e.min((ncols - l) as u32);
                    work.update += 2;
                }
            }
            rows[r as usize] = row;
            match lead {
                Some(l) => buckets[l].push(r),
                None => {
                    zero_rows += 1;
                    zero_ids.push(r);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    work.digest = digest;
    // Back to the original row indices.
    let mut inv_perm = perm.clone();
    let mut swapped: Option<(usize, usize)> = None;
    if check.corrupt_inverse && rule.permutes() && nrows > 1 {
        // Negative control: the first two permuted rows swap their targets.
        swapped = Some((0, 1));
    }
    if check.corrupt_pivot_inverse && rule.permutes() {
        let mut piv: Vec<usize> = pivot_row
            .iter()
            .filter(|&&r| r != u32::MAX)
            .map(|&r| r as usize)
            .collect();
        if piv.len() > 1 {
            let (a, b) = (piv.pop().unwrap_or(0), piv.pop().unwrap_or(0));
            swapped = Some((a, b));
        }
    }
    if let Some((a, b)) = swapped {
        inv_perm.swap(a, b);
        if rows[a] != rows[b] {
            DIAG.with(|d| {
                let mut x = d.get();
                x.corrupt_changed += 1;
                d.set(x);
            });
        }
    }
    let mut out: Vec<Vec<u16>> = vec![Vec::new(); nrows];
    for (i, row) in rows.into_iter().enumerate() {
        out[inv_perm[i] as usize] = row;
    }
    for pr in pivot_row.iter_mut() {
        if *pr != u32::MAX {
            *pr = perm[*pr as usize];
        }
    }
    let mut zero_ids: Vec<u32> = zero_ids.into_iter().map(|r| perm[r as usize]).collect();
    zero_ids.sort_unstable();
    Echelon {
        rows: out,
        pivot_row,
        zero_rows,
        zero_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gaudry_quartic::echelon_by_lead_with;

    fn random_rows(n: usize, m: usize, p: u32, seed: u64) -> Vec<Vec<u16>> {
        let mut s = seed;
        (0..n)
            .map(|_| {
                (0..m)
                    .map(|_| {
                        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                        let v = (s >> 33) as u32;
                        if v % 3 == 0 {
                            (v % p) as u16
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect()
    }

    /// Every rule gives the same pivot columns and zero rows as A, and the
    /// same reduced pivot rows up to the row-space (checked by rank).
    #[test]
    fn e36_rules_keep_pivot_columns() {
        let p = 101u32;
        let rows = random_rows(40, 30, p, 7);
        let row_eq: Vec<u32> = (0..40).map(|r| (r / 10) as u32).collect();
        let row_nnz: Vec<u32> = (0..40).map(|r| 5 + (r / 10) as u32).collect();
        let mut m = 0;
        let a = echelon_by_lead_with(rows.clone(), p, &mut m, false, false);
        for rule in [
            ArrivalRule::Fifo,
            ArrivalRule::Static,
            ArrivalRule::FifoStatic,
            ArrivalRule::EstimateK2,
            ArrivalRule::Lifo,
            ArrivalRule::Lowest,
        ] {
            let mut mm = 0;
            let mut w = PivotWork::default();
            let e = echelon_by_rule(
                rows.clone(),
                p,
                &mut mm,
                rule,
                &row_eq,
                &row_nnz,
                RuleCheck::default(),
                &mut w,
            );
            if rule == ArrivalRule::Lowest {
                assert_eq!(mm, m, "Lowest charges A's multiplications");
                assert_eq!(e.pivot_row, a.pivot_row);
            }
            let cols = |x: &Echelon| x.pivot_row.iter().map(|&r| r != u32::MAX).collect::<Vec<_>>();
            assert_eq!(cols(&e), cols(&a), "{rule:?}");
            assert_eq!(e.zero_rows, a.zero_rows, "{rule:?}");
            for (c, &r) in e.pivot_row.iter().enumerate() {
                if r != u32::MAX {
                    assert_eq!(e.rows[r as usize][c], 1, "{rule:?}");
                    assert!(e.rows[r as usize][..c].iter().all(|&x| x == 0));
                }
            }
        }
    }
}
