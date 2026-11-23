//! Report section 2 and the sparse half of section 3: sparse matrices over
//! `F_p`, matrix-vector products, singleton/2-core peeling, MST-guided
//! `k`-way merges, structured Gaussian elimination with Markowitz pivoting,
//! minimum-degree ordering and symbolic fill.
//!
//! Row combinations always carry **provenance**: a sparse vector over the
//! original rows, so a filtered relation can be replayed against the source
//! relations (`y^T A`), which the report requires of every filter.

use super::field::Fp;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Sparse vector: sorted `(index, nonzero value)` pairs.
pub type SparseVec = Vec<(usize, u64)>;

/// Canonicalize a sparse vector: combine repeats, drop zeros, sort.
pub fn canonical(f: Fp, v: &[(usize, u64)]) -> SparseVec {
    let mut m: HashMap<usize, u64> = HashMap::with_capacity(v.len());
    for &(i, a) in v {
        let e = m.entry(i).or_insert(0);
        *e = f.add(*e, f.reduce(a));
    }
    let mut out: SparseVec = m.into_iter().filter(|&(_, a)| a != 0).collect();
    out.sort_unstable();
    out
}

/// `a + s*b` for sorted sparse vectors.
pub fn axpy(f: Fp, a: &[(usize, u64)], s: u64, b: &[(usize, u64)]) -> SparseVec {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push(a[i]);
            i += 1;
        } else if i == a.len() || b[j].0 < a[i].0 {
            let v = f.mul(s, b[j].1);
            if v != 0 {
                out.push((b[j].0, v));
            }
            j += 1;
        } else {
            let v = f.mul_add(a[i].1, s, b[j].1);
            if v != 0 {
                out.push((a[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

pub fn scale(f: Fp, a: &[(usize, u64)], s: u64) -> SparseVec {
    if s == 0 {
        return Vec::new();
    }
    a.iter().map(|&(i, v)| (i, f.mul(v, s))).collect()
}

/// Sparse `rows x cols` matrix over `F_p`, row-major, canonical rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SparseMatrix {
    pub nrows: usize,
    pub ncols: usize,
    pub rows: Vec<SparseVec>,
}

impl SparseMatrix {
    pub fn new(f: Fp, ncols: usize, rows: Vec<Vec<(usize, u64)>>) -> Self {
        let rows: Vec<SparseVec> = rows.iter().map(|r| canonical(f, r)).collect();
        assert!(rows.iter().all(|r| r.iter().all(|&(c, _)| c < ncols)));
        Self {
            nrows: rows.len(),
            ncols,
            rows,
        }
    }
    /// Number of stored nonzeros `Z` (the report's `I` for a bipartite support).
    pub fn nnz(&self) -> usize {
        self.rows.iter().map(|r| r.len()).sum()
    }
    /// `y = A x`, `O(Z + N)` field operations.
    pub fn mul_vec(&self, f: Fp, x: &[u64]) -> Vec<u64> {
        assert_eq!(x.len(), self.ncols);
        self.rows
            .iter()
            .map(|r| r.iter().fold(0u64, |acc, &(c, a)| f.mul_add(acc, a, x[c])))
            .collect()
    }
    /// `y = A^T x`.
    pub fn mul_vec_transpose(&self, f: Fp, x: &[u64]) -> Vec<u64> {
        assert_eq!(x.len(), self.nrows);
        let mut y = vec![0u64; self.ncols];
        for (i, r) in self.rows.iter().enumerate() {
            if x[i] == 0 {
                continue;
            }
            for &(c, a) in r {
                y[c] = f.mul_add(y[c], a, x[i]);
            }
        }
        y
    }
    /// Left product `y^T A` for sparse `y`.
    pub fn left_combination(&self, f: Fp, y: &[(usize, u64)]) -> SparseVec {
        let mut acc = Vec::new();
        for &(r, s) in y {
            acc = axpy(f, &acc, s, &self.rows[r]);
        }
        acc
    }
    pub fn transpose(&self) -> Self {
        let mut rows = vec![Vec::new(); self.ncols];
        for (i, r) in self.rows.iter().enumerate() {
            for &(c, a) in r {
                rows[c].push((i, a));
            }
        }
        Self {
            nrows: self.ncols,
            ncols: self.nrows,
            rows,
        }
    }
    pub fn to_dense(&self) -> super::dense::DenseMatrix {
        let mut d = super::dense::DenseMatrix::zeros(self.nrows, self.ncols);
        for (i, r) in self.rows.iter().enumerate() {
            for &(c, a) in r {
                d.set(i, c, a);
            }
        }
        d
    }
    pub fn column_degrees(&self) -> Vec<usize> {
        let mut d = vec![0; self.ncols];
        for r in &self.rows {
            for &(c, _) in r {
                d[c] += 1;
            }
        }
        d
    }
}

/// Partial relations split into factor-base part `A_F` (columns of a
/// `SparseMatrix`) and large-prime part `A_L` with explicit LP columns.
#[derive(Clone, Debug)]
pub struct LpBlock {
    /// Row `i`'s LP entries.
    pub lp: Vec<SparseVec>,
    pub lp_columns: usize,
}

/// Result of singleton peeling: surviving rows and the peel order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PeelResult {
    pub kept_rows: Vec<usize>,
    pub removed_rows: Vec<usize>,
    /// LP columns that still have nonzeros in kept rows (the 2-core support).
    pub core_columns: Vec<usize>,
    pub incidence_visits: usize,
}

/// Singleton / hypergraph 2-core peeling in `O(R + C + I)` after
/// canonicalization: repeatedly delete a row holding an LP column of degree
/// one. Every left-kernel vector with `y^T A_L = 0` has zero weight on the
/// deleted rows (report section 5), so the dependency space is preserved.
pub fn singleton_peel(block: &LpBlock) -> PeelResult {
    let nr = block.lp.len();
    let mut col_rows: Vec<Vec<usize>> = vec![Vec::new(); block.lp_columns];
    let mut deg = vec![0usize; block.lp_columns];
    for (i, r) in block.lp.iter().enumerate() {
        for &(c, _) in r {
            col_rows[c].push(i);
            deg[c] += 1;
        }
    }
    let mut alive = vec![true; nr];
    let mut queue: Vec<usize> = (0..block.lp_columns).filter(|&c| deg[c] == 1).collect();
    let mut removed = Vec::new();
    let mut visits = 0;
    while let Some(c) = queue.pop() {
        if deg[c] != 1 {
            continue;
        }
        let Some(&r) = col_rows[c].iter().find(|&&r| alive[r]) else {
            continue;
        };
        visits += col_rows[c].len();
        alive[r] = false;
        removed.push(r);
        for &(d, _) in &block.lp[r] {
            visits += 1;
            deg[d] -= 1;
            if deg[d] == 1 {
                queue.push(d);
            }
        }
    }
    let kept: Vec<usize> = (0..nr).filter(|&i| alive[i]).collect();
    let core_columns = (0..block.lp_columns).filter(|&c| deg[c] > 0).collect();
    PeelResult {
        kept_rows: kept,
        removed_rows: removed,
        core_columns,
        incidence_visits: visits,
    }
}

/// A filtered row with provenance over original rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedRow {
    pub entries: SparseVec,
    pub provenance: SparseVec,
}

/// Merge cost model for MST merges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MergeWeight {
    /// Number of nonzeros of `B_i - B_j` after modular cancellation.
    Support,
    /// Nonzeros counting only columns `>= heavy_from` (the "large" part).
    SupportFrom(usize),
}

/// Statistics of an MST merge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergeStats {
    pub k: usize,
    pub total_support: usize,
    pub pair_evaluations: usize,
    pub tree_weight: usize,
}

/// MST-guided `k`-way merge (field-general adaptation of Cavallar's local
/// strategy, report section 2): rows sharing `pivot` are normalized to
/// `B_i = a_i^{-1} A_i`; every pair is weighted by the support of `B_i - B_j`
/// (`O(k S)`), dense Prim selects a spanning tree (`O(k^2)`), and each tree
/// edge yields one row `B_i - B_j` with the pivot cancelled. The `k-1`
/// coefficient vectors are independent; the materialized rows may not be.
pub fn mst_merge(
    f: Fp,
    rows: &[TrackedRow],
    pivot: usize,
    weight: MergeWeight,
) -> (Vec<TrackedRow>, MergeStats) {
    let k = rows.len();
    let mut stats = MergeStats {
        k,
        total_support: rows.iter().map(|r| r.entries.len()).sum(),
        ..Default::default()
    };
    if k < 2 {
        return (Vec::new(), stats);
    }
    let norm: Vec<TrackedRow> = rows
        .iter()
        .map(|r| {
            let a = r
                .entries
                .iter()
                .find(|&&(c, _)| c == pivot)
                .map(|&(_, a)| a)
                .expect("every merged row contains the pivot");
            let ai = f.inv(a).unwrap();
            TrackedRow {
                entries: scale(f, &r.entries, ai),
                provenance: scale(f, &r.provenance, ai),
            }
        })
        .collect();
    let minus1 = f.neg(1);
    let cost = |i: usize, j: usize| -> usize {
        let d = axpy(f, &norm[i].entries, minus1, &norm[j].entries);
        match weight {
            MergeWeight::Support => d.len(),
            MergeWeight::SupportFrom(h) => d.iter().filter(|&&(c, _)| c >= h).count(),
        }
    };
    let mut w = vec![vec![0usize; k]; k];
    for i in 0..k {
        for j in i + 1..k {
            let c = cost(i, j);
            stats.pair_evaluations += 1;
            w[i][j] = c;
            w[j][i] = c;
        }
    }
    // Dense Prim O(k^2); ties broken by index.
    let mut in_tree = vec![false; k];
    let mut best = vec![(usize::MAX, usize::MAX); k];
    in_tree[0] = true;
    for j in 1..k {
        best[j] = (w[0][j], 0);
    }
    let mut out = Vec::with_capacity(k - 1);
    for _ in 1..k {
        let j = (0..k)
            .filter(|&j| !in_tree[j])
            .min_by_key(|&j| (best[j].0, j))
            .unwrap();
        let i = best[j].1;
        in_tree[j] = true;
        stats.tree_weight += best[j].0;
        out.push(TrackedRow {
            entries: axpy(f, &norm[j].entries, minus1, &norm[i].entries),
            provenance: axpy(f, &norm[j].provenance, minus1, &norm[i].provenance),
        });
        for t in 0..k {
            if !in_tree[t] && w[j][t] < best[t].0 {
                best[t] = (w[j][t], j);
            }
        }
    }
    (out, stats)
}

/// Star merge baseline ("lightest-row pivot"): use the lightest row as the
/// pivot row for all others. Same output count and semantics as [`mst_merge`].
pub fn star_merge(f: Fp, rows: &[TrackedRow], pivot: usize) -> Vec<TrackedRow> {
    if rows.len() < 2 {
        return Vec::new();
    }
    let p = (0..rows.len())
        .min_by_key(|&i| (rows[i].entries.len(), i))
        .unwrap();
    let a_p = rows[p]
        .entries
        .iter()
        .find(|&&(c, _)| c == pivot)
        .unwrap()
        .1;
    let ip = f.inv(a_p).unwrap();
    (0..rows.len())
        .filter(|&i| i != p)
        .map(|i| {
            let a_i = rows[i]
                .entries
                .iter()
                .find(|&&(c, _)| c == pivot)
                .unwrap()
                .1;
            let s = f.neg(f.mul(a_i, ip));
            TrackedRow {
                entries: axpy(f, &rows[i].entries, s, &rows[p].entries),
                provenance: axpy(f, &rows[i].provenance, s, &rows[p].provenance),
            }
        })
        .collect()
}

/// Pivot rule for structured Gaussian elimination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PivotRule {
    /// Markowitz score `(r_i - 1)(c_j - 1)`, minimized over eligible columns.
    Markowitz,
    /// Lightest row among the rows of the lowest-degree column.
    LightestRow,
    /// MST merge on the lowest-degree column.
    MstMerge,
}

/// Options for [`structured_elimination`].
#[derive(Clone, Copy, Debug)]
pub struct EliminationOptions {
    /// Only columns `< eliminate_below` may be eliminated (e.g. LP columns
    /// placed first); others are kept as residual columns.
    pub eliminate_below: usize,
    /// Do not eliminate a column with more than this many rows.
    pub max_column_weight: usize,
    /// Refuse a pivot whose combined rows would exceed this weight.
    pub max_row_weight: usize,
    pub rule: PivotRule,
}

impl Default for EliminationOptions {
    fn default() -> Self {
        Self {
            eliminate_below: usize::MAX,
            max_column_weight: usize::MAX,
            max_row_weight: usize::MAX,
            rule: PivotRule::Markowitz,
        }
    }
}

/// Transcript and residual of a structured elimination.
#[derive(Clone, Debug, Default)]
pub struct EliminationResult {
    /// Residual rows (none containing an eliminated column), with provenance.
    pub rows: Vec<TrackedRow>,
    /// Eliminated columns in order.
    pub eliminated: Vec<usize>,
    /// Pivot rows consumed (with provenance), aligned with `eliminated` for
    /// Markowitz/LightestRow; recovery data for back-substitution.
    pub pivots: Vec<(usize, TrackedRow)>,
    pub field_ops: u64,
    /// `(N, Z)` after each step: rows remaining and nonzeros.
    pub trace: Vec<(usize, usize)>,
    /// Columns dropped because singletons (row deleted, no combination).
    pub singleton_columns: Vec<usize>,
}

/// Structured Gaussian elimination that cancels columns by combining rows
/// (the filtering semantics: output rows lie in the row space and have the
/// eliminated columns removed). Explicit count: `O(sum r_i c_i)` arithmetic
/// plus pivot search (a full Markowitz scan is `O(sum Z_i)` per step here).
pub fn structured_elimination(
    f: Fp,
    a: &SparseMatrix,
    opts: EliminationOptions,
) -> EliminationResult {
    let mut rows: Vec<Option<TrackedRow>> = a
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            Some(TrackedRow {
                entries: r.clone(),
                provenance: vec![(i, 1)],
            })
        })
        .collect();
    let mut col_rows: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); a.ncols];
    for (i, r) in a.rows.iter().enumerate() {
        for &(c, _) in r {
            col_rows[c].insert(i);
        }
    }
    let mut res = EliminationResult::default();
    let mut done = vec![false; a.ncols];
    let limit = opts.eliminate_below.min(a.ncols);
    let mut refused: HashSet<usize> = HashSet::new();
    loop {
        // Candidate column selection.
        let mut best: Option<(u128, usize, usize)> = None; // (score, col, row)
        for c in 0..limit {
            if done[c] || refused.contains(&c) {
                continue;
            }
            let deg = col_rows[c].len();
            if deg == 0 || deg > opts.max_column_weight {
                continue;
            }
            match opts.rule {
                PivotRule::Markowitz => {
                    for &r in &col_rows[c] {
                        let rw = rows[r].as_ref().unwrap().entries.len();
                        let score = (rw as u128 - 1) * (deg as u128 - 1);
                        if best.is_none_or(|b| (score, c, r) < b) {
                            best = Some((score, c, r));
                        }
                    }
                }
                PivotRule::LightestRow | PivotRule::MstMerge => {
                    let r = *col_rows[c]
                        .iter()
                        .min_by_key(|&&r| (rows[r].as_ref().unwrap().entries.len(), r))
                        .unwrap();
                    let score = deg as u128;
                    if best.is_none_or(|b| (score, c, r) < b) {
                        best = Some((score, c, r));
                    }
                }
            }
        }
        let Some((_, c, pr)) = best else { break };
        let members: Vec<usize> = col_rows[c].iter().copied().collect();
        // Build new rows.
        let new_rows: Vec<TrackedRow> = if members.len() == 1 {
            Vec::new()
        } else if opts.rule == PivotRule::MstMerge {
            let rs: Vec<TrackedRow> = members.iter().map(|&r| rows[r].clone().unwrap()).collect();
            mst_merge(f, &rs, c, MergeWeight::Support).0
        } else {
            let p = rows[pr].as_ref().unwrap();
            let ap = p.entries.iter().find(|&&(cc, _)| cc == c).unwrap().1;
            let ip = f.inv(ap).unwrap();
            members
                .iter()
                .filter(|&&r| r != pr)
                .map(|&r| {
                    let row = rows[r].as_ref().unwrap();
                    let ar = row.entries.iter().find(|&&(cc, _)| cc == c).unwrap().1;
                    let s = f.neg(f.mul(ar, ip));
                    TrackedRow {
                        entries: axpy(f, &row.entries, s, &p.entries),
                        provenance: axpy(f, &row.provenance, s, &p.provenance),
                    }
                })
                .collect()
        };
        if new_rows
            .iter()
            .any(|r| r.entries.len() > opts.max_row_weight)
        {
            refused.insert(c); // atomic refusal: nothing changes
            continue;
        }
        res.field_ops += new_rows
            .iter()
            .map(|r| (r.entries.len() + r.provenance.len()) as u64)
            .sum::<u64>();
        // Remove old member rows.
        for &r in &members {
            let row = rows[r].take().unwrap();
            for &(cc, _) in &row.entries {
                col_rows[cc].remove(&r);
            }
            if r == pr && opts.rule != PivotRule::MstMerge {
                res.pivots.push((c, row));
            }
        }
        if members.len() == 1 {
            res.singleton_columns.push(c);
        }
        // Insert new rows at fresh indices.
        for nr in new_rows {
            if nr.entries.is_empty() && nr.provenance.is_empty() {
                continue;
            }
            let idx = rows.len();
            for &(cc, _) in &nr.entries {
                col_rows[cc].insert(idx);
            }
            rows.push(Some(nr));
        }
        done[c] = true;
        res.eliminated.push(c);
        let n = rows.iter().filter(|r| r.is_some()).count();
        let z = rows.iter().flatten().map(|r| r.entries.len()).sum();
        res.trace.push((n, z));
    }
    res.rows = rows.into_iter().flatten().collect();
    res
}

/// Symmetric elimination graph on `n` vertices for ordering heuristics.
pub fn pattern_graph(a: &SparseMatrix) -> Vec<BTreeSet<usize>> {
    // Column intersection graph A^T A pattern: columns adjacent if they share a row.
    let mut g = vec![BTreeSet::new(); a.ncols];
    for r in &a.rows {
        for &(c, _) in r {
            for &(d, _) in r {
                if c != d {
                    g[c].insert(d);
                }
            }
        }
    }
    g
}

/// Minimum-degree ordering with explicit clique updates on the elimination
/// graph: `O(sum d_i^2)` adjacency work plus selection. Returns the order
/// and the number of fill edges created.
pub fn minimum_degree_ordering(adj: &[BTreeSet<usize>]) -> (Vec<usize>, usize) {
    let n = adj.len();
    let mut g: Vec<BTreeSet<usize>> = adj.to_vec();
    let mut alive = vec![true; n];
    let mut heap: BTreeSet<(usize, usize)> = (0..n).map(|v| (g[v].len(), v)).collect();
    let mut order = Vec::with_capacity(n);
    let mut fill = 0;
    while let Some((_, v)) = heap.pop_first() {
        alive[v] = false;
        order.push(v);
        let nb: Vec<usize> = g[v].iter().copied().filter(|&u| alive[u]).collect();
        for &u in &nb {
            heap.remove(&(g[u].len(), u));
            g[u].remove(&v);
        }
        for i in 0..nb.len() {
            for j in i + 1..nb.len() {
                let (x, y) = (nb[i], nb[j]);
                if g[x].insert(y) {
                    g[y].insert(x);
                    fill += 1;
                }
            }
        }
        for &u in &nb {
            heap.insert((g[u].len(), u));
        }
        g[v].clear();
    }
    (order, fill)
}

/// Fill edges produced by eliminating vertices in `order` (symbolic only).
pub fn symbolic_fill(adj: &[BTreeSet<usize>], order: &[usize]) -> usize {
    let n = adj.len();
    let mut g: Vec<BTreeSet<usize>> = adj.to_vec();
    let mut alive = vec![true; n];
    let mut fill = 0;
    for &v in order {
        alive[v] = false;
        let nb: Vec<usize> = g[v].iter().copied().filter(|&u| alive[u]).collect();
        for i in 0..nb.len() {
            for j in i + 1..nb.len() {
                if g[nb[i]].insert(nb[j]) {
                    g[nb[j]].insert(nb[i]);
                    fill += 1;
                }
            }
        }
    }
    fill
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    fn random_sparse(f: Fp, r: usize, c: usize, w: usize, seed: u64) -> SparseMatrix {
        let mut rng = SplitMix64::new(seed);
        let rows = (0..r)
            .map(|_| {
                (0..w)
                    .map(|_| (rng.below(c as u64) as usize, rng.nonzero(f)))
                    .collect()
            })
            .collect();
        SparseMatrix::new(f, c, rows)
    }

    #[test]
    fn spmv_matches_dense() {
        let f = Fp::new(65537).unwrap();
        let a = random_sparse(f, 20, 15, 4, 1);
        let d = a.to_dense();
        let x: Vec<u64> = (0..15).map(|i| i * 7 + 1).collect();
        assert_eq!(a.mul_vec(f, &x), d.mul_vec(f, &x));
        let y: Vec<u64> = (0..20).map(|i| i * 3 + 2).collect();
        assert_eq!(a.mul_vec_transpose(f, &y), d.transpose().mul_vec(f, &y));
    }

    #[test]
    fn peeling_preserves_left_kernel_dimension() {
        let f = Fp::new(101).unwrap();
        for seed in 0..30 {
            let a = random_sparse(f, 25, 20, 2, seed);
            let block = LpBlock {
                lp: a.rows.clone(),
                lp_columns: 20,
            };
            let peel = singleton_peel(&block);
            let full = a.to_dense().left_nullspace(f).len();
            let kept = SparseMatrix::new(
                f,
                20,
                peel.kept_rows.iter().map(|&i| a.rows[i].clone()).collect(),
            );
            let after = kept.to_dense().left_nullspace(f).len();
            assert_eq!(full, after, "seed {seed}");
            // No kept column is a singleton.
            let deg = kept.column_degrees();
            assert!(deg.iter().all(|&d| d != 1));
        }
    }

    #[test]
    fn mst_merge_cancels_pivot_and_has_valid_provenance() {
        let f = Fp::new(1_000_003).unwrap();
        let mut rng = SplitMix64::new(3);
        for _ in 0..20 {
            let k = 2 + rng.below(6) as usize;
            let orig: Vec<SparseVec> = (0..k)
                .map(|_| {
                    let mut r: Vec<(usize, u64)> = (0..4)
                        .map(|_| (1 + rng.below(10) as usize, rng.nonzero(f)))
                        .collect();
                    r.push((0, rng.nonzero(f)));
                    canonical(f, &r)
                })
                .collect();
            let a = SparseMatrix::new(f, 11, orig.clone());
            let tracked: Vec<TrackedRow> = (0..k)
                .map(|i| TrackedRow {
                    entries: a.rows[i].clone(),
                    provenance: vec![(i, 1)],
                })
                .collect();
            let (out, stats) = mst_merge(f, &tracked, 0, MergeWeight::Support);
            assert_eq!(out.len(), k - 1);
            assert_eq!(stats.pair_evaluations, k * (k - 1) / 2);
            for r in &out {
                assert!(r.entries.iter().all(|&(c, _)| c != 0));
                assert_eq!(a.left_combination(f, &r.provenance), r.entries);
            }
            // Provenance vectors are independent.
            let prov = SparseMatrix::new(f, k, out.iter().map(|r| r.provenance.clone()).collect());
            assert_eq!(prov.to_dense().rank(f), k - 1);
            let star = star_merge(f, &tracked, 0);
            assert_eq!(star.len(), k - 1);
            for r in &star {
                assert_eq!(a.left_combination(f, &r.provenance), r.entries);
            }
        }
    }

    #[test]
    fn structured_elimination_preserves_row_space_semantics() {
        let f = Fp::new(10007).unwrap();
        for (seed, rule) in [
            (1, PivotRule::Markowitz),
            (2, PivotRule::LightestRow),
            (3, PivotRule::MstMerge),
        ] {
            for t in 0..10 {
                let a = random_sparse(f, 30, 25, 3, seed * 100 + t);
                let opts = EliminationOptions {
                    eliminate_below: 15,
                    rule,
                    ..Default::default()
                };
                let res = structured_elimination(f, &a, opts);
                for r in &res.rows {
                    assert_eq!(a.left_combination(f, &r.provenance), r.entries);
                    assert!(r.entries.iter().all(|&(c, _)| !res.eliminated.contains(&c)));
                }
                // Left kernel of A restricted to "cancels eliminated columns"
                // equals that obtained from residual rows (dimension check):
                // dim{y : y^T A = 0} == dim{z : z^T Residual = 0} for rows
                // with independent provenance.
                let full = a.to_dense().left_nullspace(f).len();
                let resid =
                    SparseMatrix::new(f, 25, res.rows.iter().map(|r| r.entries.clone()).collect());
                let prov = SparseMatrix::new(
                    f,
                    30,
                    res.rows.iter().map(|r| r.provenance.clone()).collect(),
                );
                let pr = prov.to_dense().rank(f);
                assert_eq!(pr, res.rows.len(), "provenance independent");
                let after = resid.to_dense().left_nullspace(f).len();
                assert!(after <= full);
                // Dependencies among residual rows are true dependencies.
                for z in resid.to_dense().left_nullspace(f) {
                    let y: Vec<u64> = prov.to_dense().transpose().mul_vec(f, &z);
                    let yv: Vec<(usize, u64)> = y
                        .iter()
                        .enumerate()
                        .filter(|(_, &v)| v != 0)
                        .map(|(i, &v)| (i, v))
                        .collect();
                    assert!(a.left_combination(f, &yv).is_empty());
                }
            }
        }
    }

    #[test]
    fn minimum_degree_on_star_and_path() {
        // Star: eliminating leaves first creates no fill; centre first fills K_{n-1}.
        let n = 8;
        let mut adj = vec![BTreeSet::new(); n];
        for v in 1..n {
            adj[0].insert(v);
            adj[v].insert(0);
        }
        let (order, fill) = minimum_degree_ordering(&adj);
        assert_eq!(fill, 0);
        assert_eq!(symbolic_fill(&adj, &order), 0);
        let centre_first: Vec<usize> = (0..n).collect();
        assert_eq!(symbolic_fill(&adj, &centre_first), (n - 1) * (n - 2) / 2);
    }
}
