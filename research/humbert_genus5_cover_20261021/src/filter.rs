//! Matrix construction for the double-large-prime relation set without full
//! elimination of the large primes (dlp_filter_20261115/PROTOCOL.md).
//!
//! Rows are in the attack's "big" coordinates (x-index < nx, plus c and D_oo).
//! A large prime is an x-index outside the small base. `build` keeps the large
//! primes as matrix columns, removes singletons (a large prime in exactly one
//! row: that row cannot help and is kept only for back substitution), and, with
//! `merge`, eliminates a large prime only while that lowers the estimated
//! Wiedemann cost  columns × non-zeros.
use crate::curve::mulr;
use std::collections::{BTreeSet, HashMap};

pub struct Filtered {
    /// rows of the reduced matrix, big coordinates
    pub rows: Vec<Vec<(usize, u64)>>,
    /// large primes that remain matrix columns, ascending
    pub lp_cols: Vec<usize>,
    /// (large prime, row) pairs that determine that large prime's log once the
    /// row's other entries are known (pruned singletons and merge pivots)
    pub backsub: Vec<(usize, Vec<(usize, u64)>)>,
    pub pruned_rows: u64,
    pub merges: u64,
    pub nnz_before_merge: u64,
}

fn combine(a: &[(usize, u64)], fa: u64, b: &[(usize, u64)], fb: u64, r: u64) -> Vec<(usize, u64)> {
    // fa*a + fb*b, sorted inputs, zeros dropped
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() || j < b.len() {
        let (c, v) = if j >= b.len() || (i < a.len() && a[i].0 < b[j].0) {
            let e = (a[i].0, mulr(a[i].1, fa, r));
            i += 1;
            e
        } else if i >= a.len() || b[j].0 < a[i].0 {
            let e = (b[j].0, mulr(b[j].1, fb, r));
            j += 1;
            e
        } else {
            let e = (a[i].0, (mulr(a[i].1, fa, r) + mulr(b[j].1, fb, r)) % r);
            i += 1;
            j += 1;
            e
        };
        if v != 0 {
            out.push((c, v));
        }
    }
    out
}

/// `ncols_small`: number of matrix columns besides the large primes (the cost
/// estimate needs the total column count).
pub fn build(raw: &[Vec<(usize, u64)>], is_lp: &dyn Fn(usize) -> bool, ncols_small: usize, merge: bool, r: u64) -> Filtered {
    let mut rows: Vec<Option<Vec<(usize, u64)>>> = raw.iter().map(|x| Some(x.clone())).collect();
    let mut occ: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    for (i, row) in raw.iter().enumerate() {
        for &(k, _) in row {
            if is_lp(k) {
                occ.entry(k).or_default().insert(i);
            }
        }
    }
    let mut backsub = Vec::new();
    let mut pruned_rows = 0u64;
    // singleton removal to a fixed point
    let prune = |rows: &mut Vec<Option<Vec<(usize, u64)>>>, occ: &mut HashMap<usize, BTreeSet<usize>>, backsub: &mut Vec<(usize, Vec<(usize, u64)>)>, pruned: &mut u64| {
        let mut stack: Vec<usize> = occ.iter().filter(|(_, s)| s.len() <= 1).map(|(&k, _)| k).collect();
        stack.sort();
        while let Some(k) = stack.pop() {
            let Some(set) = occ.get(&k) else { continue };
            if set.len() > 1 {
                continue;
            }
            if let Some(&i) = set.iter().next() {
                let row = rows[i].take().unwrap();
                for &(k2, _) in &row {
                    if k2 != k && is_lp(k2) {
                        let s2 = occ.get_mut(&k2).unwrap();
                        s2.remove(&i);
                        if s2.len() <= 1 {
                            stack.push(k2);
                        }
                    }
                }
                backsub.push((k, row));
                *pruned += 1;
            }
            occ.remove(&k);
        }
    };
    prune(&mut rows, &mut occ, &mut backsub, &mut pruned_rows);
    let nnz = |rows: &Vec<Option<Vec<(usize, u64)>>>| -> u64 { rows.iter().flatten().map(|x| x.len() as u64).sum() };
    let nnz_before_merge = nnz(&rows);
    let mut merges = 0u64;
    if merge {
        let mut cur_nnz = nnz_before_merge as i64;
        loop {
            let mut changed = false;
            let mut order: Vec<(usize, usize)> = occ.iter().map(|(&k, s)| (s.len(), k)).collect();
            order.sort();
            for (_, k) in order {
                let Some(set) = occ.get(&k) else { continue };
                let d = set.len();
                if d < 2 || d > 12 {
                    continue;
                }
                let members: Vec<usize> = set.iter().cloned().collect();
                let p = *members.iter().min_by_key(|&&i| (rows[i].as_ref().unwrap().len(), i)).unwrap();
                let prow = rows[p].clone().unwrap();
                let pv = prow.iter().find(|e| e.0 == k).unwrap().1;
                let mut new_rows = Vec::new();
                let mut delta: i64 = -(prow.len() as i64);
                for &i in members.iter().filter(|&&i| i != p) {
                    let row = rows[i].as_ref().unwrap();
                    let v = row.iter().find(|e| e.0 == k).unwrap().1;
                    // pv*row - v*prow cancels column k
                    let nr = combine(row, pv, &prow, (r - v) % r, r);
                    debug_assert!(nr.iter().all(|e| e.0 != k));
                    delta += nr.len() as i64 - row.len() as i64;
                    new_rows.push((i, nr));
                }
                let ncols_total = (ncols_small + occ.len()) as i64;
                // accept iff (N - 1) * (nnz + delta) < N * nnz
                if (ncols_total - 1) * (cur_nnz + delta) >= ncols_total * cur_nnz {
                    continue;
                }
                // apply
                for &(k2, _) in &prow {
                    if is_lp(k2) && k2 != k {
                        occ.get_mut(&k2).unwrap().remove(&p);
                    }
                }
                rows[p] = None;
                for (i, nr) in new_rows {
                    let old = rows[i].take().unwrap();
                    for &(k2, _) in &old {
                        if is_lp(k2) && k2 != k {
                            occ.get_mut(&k2).unwrap().remove(&i);
                        }
                    }
                    for &(k2, _) in &nr {
                        if is_lp(k2) {
                            occ.entry(k2).or_default().insert(i);
                        }
                    }
                    if !nr.is_empty() {
                        rows[i] = Some(nr);
                    }
                }
                occ.remove(&k);
                backsub.push((k, prow));
                cur_nnz += delta;
                merges += 1;
                changed = true;
            }
            prune(&mut rows, &mut occ, &mut backsub, &mut pruned_rows);
            cur_nnz = nnz(&rows) as i64;
            if !changed {
                break;
            }
        }
    }
    let mut lp_cols: Vec<usize> = occ.keys().cloned().collect();
    lp_cols.sort();
    Filtered { rows: rows.into_iter().flatten().collect(), lp_cols, backsub, pruned_rows, merges, nnz_before_merge }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prune_and_merge_keep_the_solution_space() {
        // columns 0..3 small, 10.. large primes; a known solution y
        let r = 1_000_003u64;
        let y = |k: usize| -> u64 { (k as u64 * 7919 + 13) % r };
        let mut rng = 12345u64;
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        let is_lp = |k: usize| k >= 10;
        let mut raw = Vec::new();
        for _ in 0..60 {
            let mut row: Vec<(usize, u64)> = Vec::new();
            for _ in 0..3 {
                row.push(((next() % 4) as usize, 1 + next() % (r - 1)));
            }
            for _ in 0..(next() % 3) {
                row.push((10 + (next() % 15) as usize, 1 + next() % (r - 1)));
            }
            row.sort();
            row.dedup_by_key(|e| e.0);
            // make the row vanish on y: fix the first coefficient
            let rest: u64 = row[1..].iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r);
            let inv = crate::curve::invr(y(row[0].0), r);
            row[0].1 = mulr((r - rest) % r, inv, r);
            if row[0].1 != 0 {
                raw.push(row);
            }
        }
        for merge in [false, true] {
            let f = build(&raw, &is_lp, 4, merge, r);
            for row in f.rows.iter().chain(f.backsub.iter().map(|(_, row)| row)) {
                let s = row.iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r);
                assert_eq!(s, 0, "every output row vanishes on the planted solution");
            }
            for row in &f.rows {
                for &(k, _) in row {
                    assert!(!is_lp(k) || f.lp_cols.binary_search(&k).is_ok(), "large primes in rows are columns");
                }
            }
            assert!(f.lp_cols.iter().all(|k| f.rows.iter().filter(|row| row.iter().any(|e| e.0 == *k)).count() >= 2));
        }
    }
}
