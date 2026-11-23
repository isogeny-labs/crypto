//! Report section 4 and section 7 ("a conditional exponent opportunity from
//! treewidth"): tree and path decompositions, exact elimination ordered by a
//! supplied decomposition, and nested dissection.
//!
//! * [`TreeDecomposition::validate`] checks the three decomposition axioms.
//! * [`min_degree_decomposition`] is a heuristic discovery routine. Its width
//!   is an upper bound only; a large heuristic width does not establish large
//!   treewidth (report section 7). The report's literature discovery routine
//!   (Fomin et al., `O(t^7 N log N)`, width `O(t^2)`) is not implemented.
//! * [`path_decomposition_from_tree`] is the centroid recursion of section 7:
//!   width `O(t log N)` from width `t`.
//! * [`solve_by_elimination_order`] performs exact Gaussian elimination over
//!   `F_p` on the bipartite row/column graph in a supplied vertex order (from
//!   a tree or path decomposition or nested dissection). Correctness does not
//!   depend on the order; cost does. The `O((R+C) t^3)` and
//!   `O((R+C) t^2 log^2(R+C))` bounds are Fomin-Lokshtanov-Saurabh-Pilipczuk-
//!   Wrochna's for their scheme; here fill and field operations are counted
//!   and returned, not proved.
//! * [`nested_dissection_order`]: recursive BFS level-structure separators.

use super::field::Fp;
use super::sparse::{axpy, SparseMatrix, SparseVec};
use std::collections::{BTreeSet, HashMap, VecDeque};

/// Undirected simple graph as sorted adjacency sets.
pub type AdjSets = Vec<BTreeSet<usize>>;

/// Bipartite row/column support graph of `a`: rows are `0..R`, columns are
/// `R..R+C`.
pub fn bipartite_support(a: &SparseMatrix) -> AdjSets {
    let r = a.nrows;
    let mut g = vec![BTreeSet::new(); a.nrows + a.ncols];
    for (i, row) in a.rows.iter().enumerate() {
        for &(c, _) in row {
            g[i].insert(r + c);
            g[r + c].insert(i);
        }
    }
    g
}

/// A tree decomposition: bags of vertices and tree edges between bags.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TreeDecomposition {
    pub bags: Vec<Vec<usize>>,
    pub edges: Vec<(usize, usize)>,
}

impl TreeDecomposition {
    /// Width = largest bag size minus one (`0` for an empty decomposition).
    pub fn width(&self) -> usize {
        self.bags
            .iter()
            .map(|b| b.len())
            .max()
            .unwrap_or(1)
            .saturating_sub(1)
    }

    /// Check: tree (forest) on bags; vertex coverage; edge coverage; each
    /// vertex's bags form a connected subtree.
    pub fn validate(&self, g: &AdjSets) -> Result<(), String> {
        let nb = self.bags.len();
        let mut uf = super::graph::UnionFind::new(nb);
        for &(a, b) in &self.edges {
            if a >= nb || b >= nb {
                return Err("tree edge out of range".into());
            }
            if !uf.union(a, b) {
                return Err("bag graph has a cycle".into());
            }
        }
        let n = g.len();
        let mut holders: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, bag) in self.bags.iter().enumerate() {
            for &v in bag {
                if v >= n {
                    return Err(format!("vertex {v} out of range"));
                }
                holders[v].push(i);
            }
        }
        for v in 0..n {
            if holders[v].is_empty() {
                return Err(format!("vertex {v} not covered"));
            }
        }
        let bag_sets: Vec<BTreeSet<usize>> = self
            .bags
            .iter()
            .map(|b| b.iter().copied().collect())
            .collect();
        for u in 0..n {
            for &v in &g[u] {
                if u < v && !holders[u].iter().any(|&b| bag_sets[b].contains(&v)) {
                    return Err(format!("edge {u}-{v} not covered"));
                }
            }
        }
        // Connectivity of holders: count edges of the bag tree inside holders.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); nb];
        for &(a, b) in &self.edges {
            adj[a].push(b);
            adj[b].push(a);
        }
        for v in 0..n {
            let set: BTreeSet<usize> = holders[v].iter().copied().collect();
            let start = holders[v][0];
            let mut seen = BTreeSet::from([start]);
            let mut q = VecDeque::from([start]);
            while let Some(b) = q.pop_front() {
                for &c in &adj[b] {
                    if set.contains(&c) && seen.insert(c) {
                        q.push_back(c);
                    }
                }
            }
            if seen.len() != set.len() {
                return Err(format!("bags of vertex {v} are not connected"));
            }
        }
        Ok(())
    }

    /// Elimination order: process bags in post-order from `root` and emit
    /// each vertex at its topmost bag (vertices forgotten deepest first).
    pub fn elimination_order(&self, n: usize) -> Vec<usize> {
        let nb = self.bags.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); nb];
        for &(a, b) in &self.edges {
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut depth = vec![usize::MAX; nb];
        let mut post = Vec::with_capacity(nb);
        for root in 0..nb {
            if depth[root] != usize::MAX {
                continue;
            }
            depth[root] = 0;
            let mut stack = vec![(root, 0usize)];
            while let Some(&mut (b, ref mut i)) = stack.last_mut() {
                if *i < adj[b].len() {
                    let c = adj[b][*i];
                    *i += 1;
                    if depth[c] == usize::MAX {
                        depth[c] = depth[b] + 1;
                        stack.push((c, 0));
                    }
                } else {
                    post.push(b);
                    stack.pop();
                }
            }
        }
        // Topmost bag of v: minimal depth among bags containing v.
        let mut top = vec![(usize::MAX, usize::MAX); n];
        for (i, bag) in self.bags.iter().enumerate() {
            for &v in bag {
                if depth[i] < top[v].0 {
                    top[v] = (depth[i], i);
                }
            }
        }
        let mut by_bag: HashMap<usize, Vec<usize>> = HashMap::new();
        for v in 0..n {
            if top[v].1 != usize::MAX {
                by_bag.entry(top[v].1).or_default().push(v);
            }
        }
        let mut order = Vec::with_capacity(n);
        for b in post {
            if let Some(vs) = by_bag.remove(&b) {
                order.extend(vs);
            }
        }
        // Vertices absent from every bag go last.
        let mut placed = vec![false; n];
        for &v in &order {
            placed[v] = true;
        }
        order.extend((0..n).filter(|&v| !placed[v]));
        order
    }
}

/// Heuristic tree decomposition by greedy minimum-degree elimination.
pub fn min_degree_decomposition(g: &AdjSets) -> TreeDecomposition {
    let (order, _) = super::sparse::minimum_degree_ordering(g);
    decomposition_from_order(g, &order)
}

/// Tree decomposition induced by an elimination order (bags `{v} + N^+(v)`
/// in the filled graph; each bag hangs from the bag of its first-eliminated
/// higher neighbour).
pub fn decomposition_from_order(g: &AdjSets, order: &[usize]) -> TreeDecomposition {
    let n = g.len();
    let mut pos = vec![0usize; n];
    for (i, &v) in order.iter().enumerate() {
        pos[v] = i;
    }
    let mut h: AdjSets = g.to_vec();
    let mut bags = Vec::with_capacity(n);
    let mut higher: Vec<Vec<usize>> = Vec::with_capacity(n);
    for &v in order {
        let nb: Vec<usize> = h[v].iter().copied().filter(|&u| pos[u] > pos[v]).collect();
        for i in 0..nb.len() {
            for j in i + 1..nb.len() {
                h[nb[i]].insert(nb[j]);
                h[nb[j]].insert(nb[i]);
            }
        }
        let mut bag = nb.clone();
        bag.push(v);
        bag.sort_unstable();
        bags.push(bag);
        higher.push(nb);
    }
    let mut edges = Vec::new();
    for (i, nb) in higher.iter().enumerate() {
        if let Some(&u) = nb.iter().min_by_key(|&&u| pos[u]) {
            edges.push((i, pos[u]));
        }
    }
    TreeDecomposition { bags, edges }
}

/// Path decomposition (sequence of bags) from a tree decomposition by
/// centroid recursion: width at most `(t + 1)(log2(#bags) + 1) - 1`.
pub fn path_decomposition_from_tree(td: &TreeDecomposition) -> Vec<Vec<usize>> {
    let nb = td.bags.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); nb];
    for &(a, b) in &td.edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut removed = vec![false; nb];
    let mut out = Vec::new();
    let mut seen = vec![false; nb];
    for s in 0..nb {
        if seen[s] {
            continue;
        }
        // Collect the component.
        let mut comp = vec![s];
        seen[s] = true;
        let mut i = 0;
        while i < comp.len() {
            let b = comp[i];
            i += 1;
            for &c in &adj[b] {
                if !seen[c] {
                    seen[c] = true;
                    comp.push(c);
                }
            }
        }
        out.extend(centroid_path(td, &adj, &mut removed, s, &[]));
    }
    out
}

fn centroid_path(
    td: &TreeDecomposition,
    adj: &[Vec<usize>],
    removed: &mut [bool],
    start: usize,
    carry: &[usize],
) -> Vec<Vec<usize>> {
    // Component of `start` among non-removed bags.
    let mut comp = vec![start];
    let mut parent = HashMap::from([(start, usize::MAX)]);
    let mut i = 0;
    while i < comp.len() {
        let b = comp[i];
        i += 1;
        for &c in &adj[b] {
            if !removed[c] && !parent.contains_key(&c) {
                parent.insert(c, b);
                comp.push(c);
            }
        }
    }
    // Subtree sizes (reverse BFS order) and centroid.
    let mut size: HashMap<usize, usize> = comp.iter().map(|&b| (b, 1)).collect();
    for &b in comp.iter().rev() {
        let p = parent[&b];
        if p != usize::MAX {
            let s = size[&b];
            *size.get_mut(&p).unwrap() += s;
        }
    }
    let total = comp.len();
    let centroid = *comp
        .iter()
        .find(|&&b| {
            let mut biggest = total - size[&b];
            for &c in &adj[b] {
                if !removed[c] && parent.get(&c) == Some(&b) {
                    biggest = biggest.max(size[&c]);
                }
            }
            biggest * 2 <= total
        })
        .unwrap();
    removed[centroid] = true;
    let mut extra: Vec<usize> = carry.to_vec();
    extra.extend(td.bags[centroid].iter().copied());
    extra.sort_unstable();
    extra.dedup();
    let mut out = Vec::new();
    for &c in &adj[centroid] {
        if !removed[c] {
            out.extend(centroid_path(td, adj, removed, c, &extra));
        }
    }
    out.push(extra);
    out
}

/// Convert a path decomposition to a [`TreeDecomposition`] (path-shaped).
pub fn path_as_tree(path: &[Vec<usize>]) -> TreeDecomposition {
    TreeDecomposition {
        bags: path.to_vec(),
        edges: (1..path.len()).map(|i| (i - 1, i)).collect(),
    }
}

/// Counters from an ordered elimination.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EliminationCost {
    pub field_ops: u64,
    pub max_row_weight: usize,
    pub pivots: usize,
}

/// Exact solve of `A x = b` over `F_p` by Gaussian elimination in the
/// supplied order of bipartite vertices (rows `0..R`, columns `R..R+C`):
/// columns are eliminated in order, each with the unused row containing it
/// that appears earliest in the order. Returns `None` if inconsistent; free
/// columns are set to zero.
pub fn solve_by_elimination_order(
    f: Fp,
    a: &SparseMatrix,
    b: &[u64],
    order: &[usize],
) -> (Option<Vec<u64>>, EliminationCost) {
    let r = a.nrows;
    assert_eq!(b.len(), r);
    let mut pos = vec![usize::MAX; r + a.ncols];
    for (i, &v) in order.iter().enumerate() {
        pos[v] = i;
    }
    // Rows carry the RHS in an extra column index `ncols`.
    let rhs_col = a.ncols;
    let mut rows: Vec<Option<SparseVec>> = a
        .rows
        .iter()
        .zip(b)
        .map(|(row, &bi)| {
            let mut v = row.clone();
            if f.reduce(bi) != 0 {
                v.push((rhs_col, f.reduce(bi)));
            }
            Some(v)
        })
        .collect();
    let mut col_rows: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); a.ncols];
    for (i, row) in a.rows.iter().enumerate() {
        for &(c, _) in row {
            col_rows[c].insert(i);
        }
    }
    let mut cost = EliminationCost::default();
    let mut cols: Vec<usize> = (0..a.ncols).collect();
    cols.sort_by_key(|&c| (pos[r + c], c));
    let mut pivots: Vec<(usize, SparseVec)> = Vec::new();
    for c in cols {
        let Some(&pr) = col_rows[c].iter().min_by_key(|&&i| (pos[i], i)) else {
            continue;
        };
        let prow = rows[pr].take().unwrap();
        for &(cc, _) in &prow {
            if cc < a.ncols {
                col_rows[cc].remove(&pr);
            }
        }
        let ap = prow.iter().find(|&&(cc, _)| cc == c).unwrap().1;
        let ip = f.inv(ap).unwrap();
        let others: Vec<usize> = col_rows[c].iter().copied().collect();
        for i in others {
            let row = rows[i].take().unwrap();
            let ai = row.iter().find(|&&(cc, _)| cc == c).unwrap().1;
            let s = f.neg(f.mul(ai, ip));
            let nr = axpy(f, &row, s, &prow);
            cost.field_ops += (row.len() + prow.len()) as u64;
            for &(cc, _) in &row {
                if cc < a.ncols {
                    col_rows[cc].remove(&i);
                }
            }
            for &(cc, _) in &nr {
                if cc < a.ncols {
                    col_rows[cc].insert(i);
                }
            }
            cost.max_row_weight = cost.max_row_weight.max(nr.len());
            rows[i] = Some(nr);
        }
        cost.max_row_weight = cost.max_row_weight.max(prow.len());
        pivots.push((c, prow));
    }
    cost.pivots = pivots.len();
    // Consistency: remaining rows must be zero including RHS.
    if rows.iter().flatten().any(|row| !row.is_empty()) {
        return (None, cost);
    }
    let mut x = vec![0u64; a.ncols];
    for (c, prow) in pivots.iter().rev() {
        let mut acc = 0u64;
        let mut ac = 0u64;
        for &(cc, v) in prow {
            if cc == rhs_col {
                acc = f.add(acc, v);
            } else if cc == *c {
                ac = v;
            } else {
                acc = f.sub(acc, f.mul(v, x[cc]));
            }
        }
        x[*c] = f.div(acc, ac).unwrap();
        cost.field_ops += prow.len() as u64;
    }
    (Some(x), cost)
}

/// Nested dissection ordering of a symmetric graph: recursively split each
/// component by the middle BFS level from a pseudo-peripheral vertex; order
/// part A, part B, then the separator. Parts of at most `leaf_size` vertices
/// are ordered by minimum degree.
pub fn nested_dissection_order(g: &AdjSets, leaf_size: usize) -> Vec<usize> {
    let n = g.len();
    let mut order = Vec::with_capacity(n);
    let all: Vec<usize> = (0..n).collect();
    nd_rec(g, &all, leaf_size.max(1), &mut order);
    order
}

fn nd_rec(g: &AdjSets, verts: &[usize], leaf: usize, order: &mut Vec<usize>) {
    if verts.is_empty() {
        return;
    }
    if verts.len() <= leaf {
        let idx: HashMap<usize, usize> = verts.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let sub: AdjSets = verts
            .iter()
            .map(|&v| g[v].iter().filter_map(|u| idx.get(u).copied()).collect())
            .collect();
        let (o, _) = super::sparse::minimum_degree_ordering(&sub);
        order.extend(o.into_iter().map(|i| verts[i]));
        return;
    }
    let inside: BTreeSet<usize> = verts.iter().copied().collect();
    let bfs = |s: usize| -> HashMap<usize, usize> {
        let mut d = HashMap::from([(s, 0usize)]);
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &v in &g[u] {
                if inside.contains(&v) && !d.contains_key(&v) {
                    d.insert(v, d[&u] + 1);
                    q.push_back(v);
                }
            }
        }
        d
    };
    // Split by components first.
    let d0 = bfs(verts[0]);
    if d0.len() < verts.len() {
        let comp: Vec<usize> = verts
            .iter()
            .copied()
            .filter(|v| d0.contains_key(v))
            .collect();
        let rest: Vec<usize> = verts
            .iter()
            .copied()
            .filter(|v| !d0.contains_key(v))
            .collect();
        nd_rec(g, &comp, leaf, order);
        nd_rec(g, &rest, leaf, order);
        return;
    }
    // Pseudo-peripheral vertex: farthest from an arbitrary start.
    let far = *d0
        .iter()
        .max_by_key(|&(&v, &d)| (d, std::cmp::Reverse(v)))
        .unwrap()
        .0;
    let d = bfs(far);
    let depth = *d.values().max().unwrap();
    if depth < 2 {
        // Clique-like: no useful separator.
        let mut vs = verts.to_vec();
        vs.sort_unstable();
        order.extend(vs);
        return;
    }
    // Choose the level that best balances the two sides.
    let mut counts = vec![0usize; depth + 1];
    for &l in d.values() {
        counts[l] += 1;
    }
    let half = verts.len() / 2;
    let mut acc = 0;
    let mut mid = 1;
    for (l, &c) in counts.iter().enumerate() {
        if acc + c > half {
            mid = l.clamp(1, depth - 1);
            break;
        }
        acc += c;
    }
    let a: Vec<usize> = verts.iter().copied().filter(|v| d[v] < mid).collect();
    let b: Vec<usize> = verts.iter().copied().filter(|v| d[v] > mid).collect();
    let s: Vec<usize> = verts.iter().copied().filter(|v| d[v] == mid).collect();
    nd_rec(g, &a, leaf, order);
    nd_rec(g, &b, leaf, order);
    order.extend(s);
}

/// Symmetrized pattern of a square matrix `A + A^T` (off-diagonal).
pub fn symmetric_pattern(a: &SparseMatrix) -> AdjSets {
    assert_eq!(a.nrows, a.ncols);
    let mut g = vec![BTreeSet::new(); a.nrows];
    for (i, row) in a.rows.iter().enumerate() {
        for &(c, _) in row {
            if c != i {
                g[i].insert(c);
                g[c].insert(i);
            }
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    fn grid(w: usize, h: usize) -> AdjSets {
        let mut g = vec![BTreeSet::new(); w * h];
        for y in 0..h {
            for x in 0..w {
                let v = y * w + x;
                if x + 1 < w {
                    g[v].insert(v + 1);
                    g[v + 1].insert(v);
                }
                if y + 1 < h {
                    g[v].insert(v + w);
                    g[v + w].insert(v);
                }
            }
        }
        g
    }

    #[test]
    fn decompositions_validate() {
        let g = grid(6, 5);
        let td = min_degree_decomposition(&g);
        td.validate(&g).unwrap();
        assert!(
            td.width() >= 5 && td.width() <= 8,
            "grid width {}",
            td.width()
        );
        let pd = path_decomposition_from_tree(&td);
        let ptd = path_as_tree(&pd);
        ptd.validate(&g).unwrap();
        let log = (td.bags.len() as f64).log2().floor() as usize + 1;
        assert!(ptd.width() < (td.width() + 1) * (log + 1));
        // A bad decomposition is rejected.
        let mut bad = td.clone();
        for bag in bad.bags.iter_mut() {
            bag.retain(|&v| v != 7);
        }
        assert!(bad.validate(&g).is_err());
    }

    #[test]
    fn ordered_elimination_solves_exactly() {
        let f = Fp::new(1_000_003).unwrap();
        let mut rng = SplitMix64::new(42);
        for trial in 0..20 {
            let n = 30;
            // Banded matrix (low treewidth) plus random diagonal.
            let rows: Vec<Vec<(usize, u64)>> = (0..n)
                .map(|i| {
                    let mut r = vec![(i, rng.nonzero(f))];
                    if i + 1 < n {
                        r.push((i + 1, rng.element(f)));
                    }
                    if i + 3 < n && trial % 2 == 0 {
                        r.push((i + 3, rng.element(f)));
                    }
                    r
                })
                .collect();
            let a = SparseMatrix::new(f, n, rows);
            let b: Vec<u64> = (0..n).map(|_| rng.element(f)).collect();
            let g = bipartite_support(&a);
            let td = min_degree_decomposition(&g);
            td.validate(&g).unwrap();
            let orders = [
                td.elimination_order(g.len()),
                path_as_tree(&path_decomposition_from_tree(&td)).elimination_order(g.len()),
                nested_dissection_order(&g, 4),
                (0..g.len()).collect::<Vec<_>>(),
            ];
            let dense = a.to_dense().solve(f, &b);
            for o in &orders {
                let (x, _) = solve_by_elimination_order(f, &a, &b, o);
                match (&dense, x) {
                    (Some(_), Some(x)) => assert_eq!(a.mul_vec(f, &x), b),
                    (None, None) => {}
                    (d, x) => panic!("mismatch dense {:?} ordered {:?}", d.is_some(), x.is_some()),
                }
            }
        }
    }

    #[test]
    fn nested_dissection_is_permutation_and_reduces_fill_on_grid() {
        let g = grid(12, 12);
        let nd = nested_dissection_order(&g, 4);
        let mut s = nd.clone();
        s.sort_unstable();
        assert_eq!(s, (0..144).collect::<Vec<_>>());
        let natural: Vec<usize> = (0..144).collect();
        let fill_nd = super::super::sparse::symbolic_fill(&g, &nd);
        let fill_nat = super::super::sparse::symbolic_fill(&g, &natural);
        assert!(fill_nd < fill_nat, "nd {fill_nd} natural {fill_nat}");
    }
}
