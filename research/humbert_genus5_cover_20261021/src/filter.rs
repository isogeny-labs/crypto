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
    /// rows removed by clique removal, and the trees they formed
    pub clique_rows: u64,
    pub cliques: u64,
    /// rows minus columns after pruning, before clique removal
    pub excess_before: i64,
    /// HUMBERT_CLIQUE_SAFE: trees skipped because removing them (with the singleton
    /// cascade it triggers) would leave a base column without a row
    pub cliques_skipped: u64,
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
    build_x(raw, is_lp, ncols_small, merge, r, None)
}

/// `keep_excess`: when set, remove whole trees of rows (rows linked by large primes of
/// degree 2, with one fewer such large prime than rows) heaviest first, until rows minus
/// columns is at most that value. Each tree removed lowers the excess by exactly one and
/// takes its rows' non-zeros and its large-prime columns with it (Cavallar's clique
/// removal). The removed rows are discarded: they are redundant, not needed for logs.
pub fn build_x(raw: &[Vec<(usize, u64)>], is_lp: &dyn Fn(usize) -> bool, ncols_small: usize, merge: bool, r: u64, keep_excess: Option<i64>) -> Filtered {
    build_xs(raw, is_lp, ncols_small, merge, r, keep_excess, None)
}

/// `build_x` with an optional safe clique rule (HUMBERT_CLIQUE_SAFE). With
/// `safe_base = Some(is_base)`, where `is_base(k)` says whether coordinate `k` is a
/// base (non-large-prime) column of the final matrix, clique removal never removes a
/// tree whose removal, together with the singleton cascade it causes, would leave a
/// base column that still has a row with none. Such trees are skipped and the next
/// heaviest tree is tried. With `None` the behaviour is exactly `build_x`'s.
pub fn build_xs(raw: &[Vec<(usize, u64)>], is_lp: &dyn Fn(usize) -> bool, ncols_small: usize, merge: bool, r: u64, keep_excess: Option<i64>, safe_base: Option<&dyn Fn(usize) -> bool>) -> Filtered {
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
    let excess_of = |rows: &Vec<Option<Vec<(usize, u64)>>>, occ: &HashMap<usize, BTreeSet<usize>>| -> i64 {
        rows.iter().flatten().count() as i64 - (ncols_small + occ.len()) as i64
    };
    let excess_before = excess_of(&rows, &occ);
    let (mut clique_rows, mut cliques) = (0u64, 0u64);
    let mut cliques_skipped = 0u64;
    if let (Some(keep), Some(is_base)) = (keep_excess, safe_base) {
        safe_cliques(&mut rows, &mut occ, &mut backsub, &mut pruned_rows, is_lp, is_base, ncols_small, keep, &mut clique_rows, &mut cliques, &mut cliques_skipped);
    } else if let Some(keep) = keep_excess {
        loop {
            let need = excess_of(&rows, &occ) - keep;
            if need <= 0 {
                break;
            }
            // components of the alive rows under degree-2 large primes (union-find)
            let n = rows.len();
            let mut par: Vec<usize> = (0..n).collect();
            fn find(par: &mut Vec<usize>, mut i: usize) -> usize {
                while par[i] != i {
                    par[i] = par[par[i]];
                    i = par[i];
                }
                i
            }
            let mut edges: HashMap<usize, u64> = HashMap::new();
            let mut deg2: Vec<(usize, usize)> = Vec::new();
            for s in occ.values() {
                if s.len() == 2 {
                    let mut it = s.iter();
                    deg2.push((*it.next().unwrap(), *it.next().unwrap()));
                }
            }
            for &(a, b) in &deg2 {
                let (ra, rb) = (find(&mut par, a), find(&mut par, b));
                if ra != rb {
                    par[ra] = rb;
                }
            }
            for &(a, _) in &deg2 {
                let ra = find(&mut par, a);
                *edges.entry(ra).or_default() += 1;
            }
            let mut comp: HashMap<usize, (u64, u64, Vec<usize>)> = HashMap::new();
            for i in 0..n {
                if let Some(row) = &rows[i] {
                    let ri = find(&mut par, i);
                    let e = comp.entry(ri).or_default();
                    e.0 += row.len() as u64;
                    e.1 += 1;
                    e.2.push(i);
                }
            }
            // trees only: edges == rows - 1 (a cycle would not lower the excess)
            let mut trees: Vec<(u64, usize)> = comp.iter().filter(|(k, v)| edges.get(k).copied().unwrap_or(0) + 1 == v.1).map(|(k, v)| (v.0, *k)).collect();
            if trees.is_empty() {
                break;
            }
            trees.sort_by(|a, b| b.cmp(a));
            let take = ((need + 1) / 2).max(1) as usize;
            for &(_, k) in trees.iter().take(take) {
                for &i in &comp[&k].2 {
                    let row = rows[i].take().unwrap();
                    for &(k2, _) in &row {
                        if is_lp(k2) {
                            let s2 = occ.get_mut(&k2).unwrap();
                            s2.remove(&i);
                            if s2.is_empty() {
                                occ.remove(&k2);
                            }
                        }
                    }
                    clique_rows += 1;
                }
                cliques += 1;
            }
            prune(&mut rows, &mut occ, &mut backsub, &mut pruned_rows);
        }
    }
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
    Filtered { rows: rows.into_iter().flatten().collect(), lp_cols, backsub, pruned_rows, merges, nnz_before_merge, clique_rows, cliques, excess_before, cliques_skipped }
}

/// Singleton removal to a fixed point, starting from the large primes in `stack`
/// (the same rule as `build_x`'s `prune`, local to the large primes a removal touched).
/// Keeps the per-base-column row counts `bcnt` in step with the rows it removes.
fn prune_local(rows: &mut [Option<Vec<(usize, u64)>>], occ: &mut HashMap<usize, BTreeSet<usize>>, backsub: &mut Vec<(usize, Vec<(usize, u64)>)>, pruned: &mut u64, is_lp: &dyn Fn(usize) -> bool, is_base: &dyn Fn(usize) -> bool, bcnt: &mut HashMap<usize, u64>, mut stack: Vec<usize>) {
    stack.sort();
    stack.dedup();
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
                } else if !is_lp(k2) && is_base(k2) {
                    *bcnt.get_mut(&k2).unwrap() -= 1;
                }
            }
            backsub.push((k, row));
            *pruned += 1;
        }
        occ.remove(&k);
    }
}

/// Safe clique removal (HUMBERT_CLIQUE_SAFE=1): as `build_x`'s loop (components under
/// degree-2 large primes, trees only, heaviest first, about half the remaining excess per
/// round), but a tree is removed only if no base column that has a row loses its last
/// one, counting the rows the singleton cascade of the removal sends to back substitution.
/// Each accepted tree is applied, and its cascade pruned, before the next is checked, so
/// every check sees the current counts. Stops when the excess reaches `keep` or a round
/// accepts no tree. Pure bookkeeping: no modular arithmetic.
#[allow(clippy::too_many_arguments)]
fn safe_cliques(rows: &mut Vec<Option<Vec<(usize, u64)>>>, occ: &mut HashMap<usize, BTreeSet<usize>>, backsub: &mut Vec<(usize, Vec<(usize, u64)>)>, pruned: &mut u64, is_lp: &dyn Fn(usize) -> bool, is_base: &dyn Fn(usize) -> bool, ncols_small: usize, keep: i64, clique_rows: &mut u64, cliques: &mut u64, skipped: &mut u64) {
    let n = rows.len();
    let mut bcnt: HashMap<usize, u64> = HashMap::new();
    for row in rows.iter().flatten() {
        for &(k, _) in row {
            if !is_lp(k) && is_base(k) {
                *bcnt.entry(k).or_default() += 1;
            }
        }
    }
    fn find(par: &mut [usize], mut i: usize) -> usize {
        while par[i] != i {
            par[i] = par[par[i]];
            i = par[i];
        }
        i
    }
    loop {
        let excess = rows.iter().flatten().count() as i64 - (ncols_small + occ.len()) as i64;
        let need = excess - keep;
        if need <= 0 {
            break;
        }
        let mut par: Vec<usize> = (0..n).collect();
        let mut deg2: Vec<(usize, usize)> = Vec::new();
        for s in occ.values() {
            if s.len() == 2 {
                let mut it = s.iter();
                deg2.push((*it.next().unwrap(), *it.next().unwrap()));
            }
        }
        for &(a, b) in &deg2 {
            let (ra, rb) = (find(&mut par, a), find(&mut par, b));
            if ra != rb {
                par[ra] = rb;
            }
        }
        let mut edges: HashMap<usize, u64> = HashMap::new();
        for &(a, _) in &deg2 {
            let ra = find(&mut par, a);
            *edges.entry(ra).or_default() += 1;
        }
        let mut comp: HashMap<usize, (u64, u64, Vec<usize>)> = HashMap::new();
        for i in 0..n {
            if let Some(row) = &rows[i] {
                let ri = find(&mut par, i);
                let e = comp.entry(ri).or_default();
                e.0 += row.len() as u64;
                e.1 += 1;
                e.2.push(i);
            }
        }
        let mut trees: Vec<(u64, usize)> = comp.iter().filter(|(k, v)| edges.get(k).copied().unwrap_or(0) + 1 == v.1).map(|(k, v)| (v.0, *k)).collect();
        if trees.is_empty() {
            break;
        }
        trees.sort_by(|a, b| b.cmp(a));
        let take = ((need + 1) / 2).max(1) as usize;
        let mut accepted = 0usize;
        for &(_, k) in &trees {
            if accepted >= take {
                break;
            }
            let members = &comp[&k].2;
            // an earlier tree's cascade this round may have taken one of these rows
            if members.iter().any(|&i| rows[i].is_none()) {
                continue;
            }
            // simulate the removal and its singleton cascade
            let mut gone: BTreeSet<usize> = members.iter().cloned().collect();
            let mut dec: HashMap<usize, usize> = HashMap::new();
            let mut bdec: HashMap<usize, u64> = HashMap::new();
            let mut queue: Vec<usize> = members.clone();
            while let Some(i) = queue.pop() {
                for &(k2, _) in rows[i].as_ref().unwrap() {
                    if is_lp(k2) {
                        let d = dec.entry(k2).or_default();
                        *d += 1;
                        // one unprocessed row left: if it is not already queued it
                        // becomes a singleton and is pruned in turn
                        let set = &occ[&k2];
                        if set.len() - *d == 1 {
                            if let Some(&j) = set.iter().find(|j| !gone.contains(j)) {
                                gone.insert(j);
                                queue.push(j);
                            }
                        }
                    } else if is_base(k2) {
                        *bdec.entry(k2).or_default() += 1;
                    }
                }
            }
            if bdec.iter().any(|(c, d)| bcnt[c] <= *d) {
                *skipped += 1;
                continue;
            }
            // apply: the tree's rows are discarded, the cascade goes to back substitution
            let mut stack = Vec::new();
            for &i in members {
                let row = rows[i].take().unwrap();
                for &(k2, _) in &row {
                    if is_lp(k2) {
                        let s2 = occ.get_mut(&k2).unwrap();
                        s2.remove(&i);
                        if s2.is_empty() {
                            occ.remove(&k2);
                        } else if s2.len() == 1 {
                            stack.push(k2);
                        }
                    } else if is_base(k2) {
                        *bcnt.get_mut(&k2).unwrap() -= 1;
                    }
                }
                *clique_rows += 1;
            }
            prune_local(rows, occ, backsub, pruned, is_lp, is_base, &mut bcnt, stack);
            debug_assert!(gone.iter().all(|&i| rows[i].is_none()));
            *cliques += 1;
            accepted += 1;
        }
        if accepted == 0 {
            break;
        }
    }
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
        for (merge, keep) in [(false, None), (true, None), (true, Some(2i64)), (false, Some(0i64))] {
            let f = build_x(&raw, &is_lp, 4, merge, r, keep);
            if let Some(k) = keep {
                assert!(f.rows.len() as i64 - (4 + f.lp_cols.len()) as i64 <= k, "excess cut to the target");
            }
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

    #[test]
    fn safe_cliques_never_empty_a_base_column() {
        // base columns 0..7; large primes 100, 200, 201. Column 5 lives only in the
        // heaviest-but-one tree {A, B} (linked by the degree-2 prime 100); column 6 only
        // in row E, which shares the degree-3 prime 200 with the heaviest tree {C, D}:
        // removing {C, D} sends E to back substitution by the singleton cascade.
        let r = 1_000_003u64;
        let y = |k: usize| -> u64 { (k as u64 * 7919 + 13) % r };
        let mut rng = 777u64;
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };
        let mut plant = |cols: &[usize]| -> Vec<(usize, u64)> {
            let mut row: Vec<(usize, u64)> = cols.iter().map(|&k| (k, 1 + next() % (r - 1))).collect();
            let rest: u64 = row[1..].iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r);
            row[0].1 = mulr((r - rest) % r, crate::curve::invr(y(row[0].0), r), r);
            assert_ne!(row[0].1, 0);
            row
        };
        let mut raw = vec![
            plant(&[1, 2, 3, 4, 5, 100]),
            plant(&[0, 1, 2, 3, 100]),
            plant(&[0, 1, 2, 3, 200, 201]),
            plant(&[0, 1, 2, 4, 200, 201]),
            plant(&[0, 6, 200]),
        ];
        for i in 0..12 {
            raw.push(plant(&[i % 5, (i + 1) % 5, (i + 3) % 5].iter().cloned().collect::<BTreeSet<usize>>().into_iter().collect::<Vec<_>>()));
        }
        let is_lp = |k: usize| k >= 100;
        let is_base = |k: usize| k < 7;
        let covered = |f: &Filtered| -> BTreeSet<usize> { f.rows.iter().flatten().map(|e| e.0).filter(|&k| is_base(k)).collect() };
        // the unsafe rule empties columns 5 and 6 on this input (the test has teeth)
        let unsafe_f = build_x(&raw, &is_lp, 7, false, r, Some(0));
        assert!(!covered(&unsafe_f).contains(&5) && !covered(&unsafe_f).contains(&6));
        for merge in [false, true] {
            let f = build_xs(&raw, &is_lp, 7, merge, r, Some(0), Some(&is_base));
            assert_eq!(covered(&f), (0..7).collect::<BTreeSet<usize>>(), "every base column keeps a row");
            assert!(f.cliques_skipped >= 2, "the two unsafe trees are skipped");
            assert!(f.rows.len() as i64 - (7 + f.lp_cols.len()) as i64 <= 0, "the excess still reaches the target");
            for row in f.rows.iter().chain(f.backsub.iter().map(|(_, row)| row)) {
                let s = row.iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r);
                assert_eq!(s, 0, "every output row vanishes on the planted solution");
            }
            assert!(f.lp_cols.iter().all(|k| f.rows.iter().filter(|row| row.iter().any(|e| e.0 == *k)).count() >= 2));
        }
        // the earlier random instance: the safe rule keeps the solution space and the
        // invariants, and never uncovers a base column covered after the first prune
        let mut rng2 = 12345u64;
        let mut next2 = || {
            rng2 ^= rng2 << 13;
            rng2 ^= rng2 >> 7;
            rng2 ^= rng2 << 17;
            rng2
        };
        let is_lp10 = |k: usize| k >= 10;
        let is_base4 = |k: usize| k < 4;
        let mut raw2 = Vec::new();
        for _ in 0..60 {
            let mut row: Vec<(usize, u64)> = Vec::new();
            for _ in 0..3 {
                row.push(((next2() % 4) as usize, 1 + next2() % (r - 1)));
            }
            for _ in 0..(next2() % 3) {
                row.push((10 + (next2() % 15) as usize, 1 + next2() % (r - 1)));
            }
            row.sort();
            row.dedup_by_key(|e| e.0);
            let rest: u64 = row[1..].iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r);
            row[0].1 = mulr((r - rest) % r, crate::curve::invr(y(row[0].0), r), r);
            if row[0].1 != 0 {
                raw2.push(row);
            }
        }
        let base_cov = build_x(&raw2, &is_lp10, 4, false, r, None);
        let before: BTreeSet<usize> = base_cov.rows.iter().flatten().map(|e| e.0).filter(|&k| is_base4(k)).collect();
        for keep in [0i64, 2] {
            let f = build_xs(&raw2, &is_lp10, 4, false, r, Some(keep), Some(&is_base4));
            let after: BTreeSet<usize> = f.rows.iter().flatten().map(|e| e.0).filter(|&k| is_base4(k)).collect();
            assert_eq!(after, before);
            for row in f.rows.iter().chain(f.backsub.iter().map(|(_, row)| row)) {
                assert_eq!(row.iter().fold(0, |a, &(k, v)| (a + mulr(v, y(k), r)) % r), 0);
            }
        }
    }
}
