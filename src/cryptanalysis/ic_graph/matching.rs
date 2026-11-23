//! Report section 2: Hopcroft-Karp maximum matching and the
//! Dulmage-Mendelsohn decomposition of a row-column support graph.
//!
//! Structural rank (the matching size) is an **upper bound** on field rank:
//! the support of `[[1,1],[1,1]]` has a perfect matching but rank one.

use super::sparse::SparseMatrix;
use std::collections::VecDeque;

/// Bipartite graph: `left` vertices (rows) with adjacency into `right` (columns).
#[derive(Clone, Debug)]
pub struct Bipartite {
    pub left: usize,
    pub right: usize,
    pub adj: Vec<Vec<usize>>,
}

impl Bipartite {
    pub fn from_support(a: &SparseMatrix) -> Self {
        Self {
            left: a.nrows,
            right: a.ncols,
            adj: a
                .rows
                .iter()
                .map(|r| r.iter().map(|&(c, _)| c).collect())
                .collect(),
        }
    }
}

/// A maximum matching: `row_match[r] = Some(c)`, `col_match[c] = Some(r)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matching {
    pub row_match: Vec<Option<usize>>,
    pub col_match: Vec<Option<usize>>,
    pub size: usize,
    pub phases: usize,
}

/// Hopcroft-Karp: `O((R + C + I) sqrt(R + C))` time, `O(R + C + I)` space.
pub fn hopcroft_karp(g: &Bipartite) -> Matching {
    const INF: usize = usize::MAX;
    let mut row_match: Vec<Option<usize>> = vec![None; g.left];
    let mut col_match: Vec<Option<usize>> = vec![None; g.right];
    let mut dist = vec![INF; g.left];
    let mut size = 0;
    let mut phases = 0;
    loop {
        // BFS layering from free rows.
        let mut q = VecDeque::new();
        for r in 0..g.left {
            if row_match[r].is_none() {
                dist[r] = 0;
                q.push_back(r);
            } else {
                dist[r] = INF;
            }
        }
        let mut found = false;
        while let Some(r) = q.pop_front() {
            for &c in &g.adj[r] {
                match col_match[c] {
                    None => found = true,
                    Some(r2) => {
                        if dist[r2] == INF {
                            dist[r2] = dist[r] + 1;
                            q.push_back(r2);
                        }
                    }
                }
            }
        }
        if !found {
            break;
        }
        phases += 1;
        // Iterative DFS along layers.
        let mut it = vec![0usize; g.left];
        for s in 0..g.left {
            if row_match[s].is_some() {
                continue;
            }
            let mut stack: Vec<usize> = vec![s];
            let mut path_cols: Vec<usize> = Vec::new();
            while let Some(&r) = stack.last() {
                if it[r] < g.adj[r].len() {
                    let c = g.adj[r][it[r]];
                    it[r] += 1;
                    match col_match[c] {
                        None => {
                            // Augment along the stack.
                            path_cols.push(c);
                            for (i, &rr) in stack.iter().enumerate() {
                                let cc = path_cols[i];
                                row_match[rr] = Some(cc);
                                col_match[cc] = Some(rr);
                            }
                            size += 1;
                            stack.clear();
                            break;
                        }
                        Some(r2) if dist[r2] == dist[r] + 1 => {
                            path_cols.push(c);
                            stack.push(r2);
                        }
                        _ => {}
                    }
                } else {
                    dist[r] = INF;
                    stack.pop();
                    path_cols.pop();
                }
            }
        }
    }
    Matching {
        row_match,
        col_match,
        size,
        phases,
    }
}

/// Coarse and fine Dulmage-Mendelsohn decomposition.
#[derive(Clone, Debug, Default)]
pub struct DmDecomposition {
    /// Overdetermined part (rows reachable from unmatched rows).
    pub over_rows: Vec<usize>,
    pub over_cols: Vec<usize>,
    /// Underdetermined part (columns reachable from unmatched columns).
    pub under_rows: Vec<usize>,
    pub under_cols: Vec<usize>,
    /// Well-determined square blocks `(rows, cols)` in block upper triangular
    /// order: rows of block `i` reference square columns of blocks `>= i`
    /// only (and possibly overdetermined columns, never underdetermined ones).
    pub square_blocks: Vec<(Vec<usize>, Vec<usize>)>,
    pub structural_rank: usize,
}

/// Dulmage-Mendelsohn decomposition in `O(R + C + I)` after the matching.
pub fn dulmage_mendelsohn(g: &Bipartite) -> DmDecomposition {
    let m = hopcroft_karp(g);
    let mut col_adj: Vec<Vec<usize>> = vec![Vec::new(); g.right];
    for r in 0..g.left {
        for &c in &g.adj[r] {
            col_adj[c].push(r);
        }
    }
    // Overdetermined: from unmatched rows, alternate row -(any edge)-> col -(matched)-> row.
    let mut row_over = vec![false; g.left];
    let mut col_over = vec![false; g.right];
    let mut q: VecDeque<usize> = (0..g.left).filter(|&r| m.row_match[r].is_none()).collect();
    for &r in &q {
        row_over[r] = true;
    }
    while let Some(r) = q.pop_front() {
        for &c in &g.adj[r] {
            if !col_over[c] {
                col_over[c] = true;
                if let Some(r2) = m.col_match[c] {
                    if !row_over[r2] {
                        row_over[r2] = true;
                        q.push_back(r2);
                    }
                }
            }
        }
    }
    // Underdetermined: from unmatched columns, col -(any edge)-> row -(matched)-> col.
    let mut row_under = vec![false; g.left];
    let mut col_under = vec![false; g.right];
    let mut q: VecDeque<usize> = (0..g.right).filter(|&c| m.col_match[c].is_none()).collect();
    for &c in &q {
        col_under[c] = true;
    }
    while let Some(c) = q.pop_front() {
        for &r in &col_adj[c] {
            if !row_under[r] {
                row_under[r] = true;
                if let Some(c2) = m.row_match[r] {
                    if !col_under[c2] {
                        col_under[c2] = true;
                        q.push_back(c2);
                    }
                }
            }
        }
    }
    let mut out = DmDecomposition {
        structural_rank: m.size,
        ..Default::default()
    };
    out.over_rows = (0..g.left).filter(|&r| row_over[r]).collect();
    out.over_cols = (0..g.right).filter(|&c| col_over[c]).collect();
    out.under_rows = (0..g.left)
        .filter(|&r| row_under[r] && !row_over[r])
        .collect();
    out.under_cols = (0..g.right)
        .filter(|&c| col_under[c] && !col_over[c])
        .collect();
    // Square part: matched pairs in neither set. Fine decomposition via SCCs
    // of the digraph on rows: r -> r' when r has an entry in col matched to r'.
    let sq_rows: Vec<usize> = (0..g.left)
        .filter(|&r| !row_over[r] && !row_under[r])
        .collect();
    let mut idx = vec![usize::MAX; g.left];
    for (i, &r) in sq_rows.iter().enumerate() {
        idx[r] = i;
    }
    let k = sq_rows.len();
    let mut dadj: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (i, &r) in sq_rows.iter().enumerate() {
        for &c in &g.adj[r] {
            if let Some(r2) = m.col_match[c] {
                if idx[r2] != usize::MAX && r2 != r {
                    dadj[i].push(idx[r2]);
                }
            }
        }
    }
    let comps = tarjan_scc(&dadj);
    // Tarjan emits SCCs in reverse topological order (sinks first). A block
    // references columns matched to rows in the same or later-emitted...
    // we output in topological order (sources first) = upper triangular.
    for comp in comps.into_iter().rev() {
        let rows: Vec<usize> = comp.iter().map(|&i| sq_rows[i]).collect();
        let cols: Vec<usize> = rows.iter().map(|&r| m.row_match[r].unwrap()).collect();
        out.square_blocks.push((rows, cols));
    }
    out
}

/// Iterative Tarjan strongly connected components; returns components in
/// reverse topological order of the condensation (sink components first).
pub fn tarjan_scc(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut index = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut on = vec![false; n];
    let mut st = Vec::new();
    let mut comps = Vec::new();
    let mut counter = 0;
    for s in 0..n {
        if index[s] != usize::MAX {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(s, 0)];
        index[s] = counter;
        low[s] = counter;
        counter += 1;
        st.push(s);
        on[s] = true;
        while let Some(&mut (v, ref mut i)) = call.last_mut() {
            if *i < adj[v].len() {
                let w = adj[v][*i];
                *i += 1;
                if index[w] == usize::MAX {
                    index[w] = counter;
                    low[w] = counter;
                    counter += 1;
                    st.push(w);
                    on[w] = true;
                    call.push((w, 0));
                } else if on[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                call.pop();
                if let Some(&(p, _)) = call.last() {
                    low[p] = low[p].min(low[v]);
                }
                if low[v] == index[v] {
                    let mut comp = Vec::new();
                    loop {
                        let w = st.pop().unwrap();
                        on[w] = false;
                        comp.push(w);
                        if w == v {
                            break;
                        }
                    }
                    comps.push(comp);
                }
            }
        }
    }
    comps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::field::{Fp, SplitMix64};

    fn brute_matching(g: &Bipartite) -> usize {
        // Simple augmenting-path (Kuhn) as an independent oracle.
        fn try_kuhn(r: usize, g: &Bipartite, seen: &mut [bool], cm: &mut [Option<usize>]) -> bool {
            for &c in &g.adj[r] {
                if !seen[c] {
                    seen[c] = true;
                    if cm[c].is_none() || try_kuhn(cm[c].unwrap(), g, seen, cm) {
                        cm[c] = Some(r);
                        return true;
                    }
                }
            }
            false
        }
        let mut cm = vec![None; g.right];
        (0..g.left)
            .filter(|&r| try_kuhn(r, g, &mut vec![false; g.right], &mut cm))
            .count()
    }

    #[test]
    fn hopcroft_karp_matches_kuhn_and_bounds_rank() {
        let f = Fp::new(7).unwrap();
        let mut rng = SplitMix64::new(11);
        for _ in 0..100 {
            let r = 1 + rng.below(15) as usize;
            let c = 1 + rng.below(15) as usize;
            let rows: Vec<Vec<(usize, u64)>> = (0..r)
                .map(|_| {
                    (0..rng.below(4))
                        .map(|_| (rng.below(c as u64) as usize, rng.nonzero(f)))
                        .collect()
                })
                .collect();
            let a = SparseMatrix::new(f, c, rows);
            let g = Bipartite::from_support(&a);
            let m = hopcroft_karp(&g);
            assert_eq!(m.size, brute_matching(&g));
            for (r, cm) in m.row_match.iter().enumerate() {
                if let Some(c) = cm {
                    assert_eq!(m.col_match[*c], Some(r));
                    assert!(g.adj[r].contains(c));
                }
            }
            assert!(a.to_dense().rank(f) <= m.size);
            let dm = dulmage_mendelsohn(&g);
            let sq: usize = dm.square_blocks.iter().map(|b| b.0.len()).sum();
            assert_eq!(dm.over_rows.len() + dm.under_rows.len() + sq, r);
            assert_eq!(dm.over_cols.len() + dm.under_cols.len() + sq, c);
            // Square blocks are upper block-triangular: rows of block i only
            // touch square columns in blocks j >= i.
            let mut block_of_col = vec![usize::MAX; c];
            for (i, (_, cols)) in dm.square_blocks.iter().enumerate() {
                for &cc in cols {
                    block_of_col[cc] = i;
                }
            }
            for (i, (rows, _)) in dm.square_blocks.iter().enumerate() {
                for &rr in rows {
                    for &cc in &g.adj[rr] {
                        assert!(
                            !dm.under_cols.contains(&cc),
                            "square row touches an underdetermined column"
                        );
                        if block_of_col[cc] != usize::MAX {
                            assert!(block_of_col[cc] >= i);
                        }
                    }
                }
            }
            // Overdetermined rows touch only overdetermined columns.
            for &rr in &dm.over_rows {
                for &cc in &g.adj[rr] {
                    assert!(dm.over_cols.contains(&cc));
                }
            }
        }
    }

    #[test]
    fn structural_rank_exceeds_field_rank() {
        let f = Fp::new(13).unwrap();
        let a = SparseMatrix::new(f, 2, vec![vec![(0, 1), (1, 1)], vec![(0, 1), (1, 1)]]);
        assert_eq!(hopcroft_karp(&Bipartite::from_support(&a)).size, 2);
        assert_eq!(a.to_dense().rank(f), 1);
    }
}
