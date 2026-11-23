//! Graph algorithms for the large-prime (LP) block of index-calculus relations.
//!
//! This module implements the algorithms of the graph-algorithms catalog
//! (`docs/research/graph_algorithms_ecdlp_20261122/`, PR #4917, section 8)
//! that the large-prime pipeline did not already have. It deliberately does
//! **not** reimplement what exists:
//!
//! * singleton / hypergraph two-core peeling and Markowitz-ordered sparse
//!   elimination: [`super::large_prime_filter::filter_large_primes`];
//! * MST-guided `k`-way merges: [`super::large_prime_filter::filter_large_primes_mst`];
//! * conservative unweighted rank screening: [`super::jv_large_prime_rank_bound`];
//! * the k=3 gain-forest harvester and coset-filtered MITM: PR #4905.
//!
//! What is here, each with the runtime the catalog states:
//!
//! | item | function | cost |
//! | --- | --- | --- |
//! | exact gain-aware DSU rank (catalog §6) | [`GainDsu`], [`gain_dsu_rank`] | `O((v + R) α(v))` field and pointer ops |
//! | core numbers (Batagelj–Zaveršnik) | [`core_numbers`] | `O(v + e)` |
//! | Hopcroft–Karp structural rank | [`hopcroft_karp`] | `O(I √(R + C))` |
//! | Dulmage–Mendelsohn decomposition | [`dulmage_mendelsohn`] | `O(R + C + I)` after matching |
//! | BFS spanning forest | [`bfs_forest`] | `O(v + e)` |
//! | low-stretch spanning forest (AKPW-style ball growing; heuristic) | [`low_stretch_forest`] | `O(e log v)` levels × linear |
//! | fundamental-cycle total length `L` | [`fundamental_cycle_length`] | `O(v + e + L)` |
//! | minimum-degree elimination ordering, fill and width | [`minimum_degree`] | `O(Σ d_i² log v)` |
//! | FM hypergraph partitioning, column-net `Σ (λ_j − 1)` | [`fm_partition`], [`connectivity_minus_one`] | `O(I log R)` per pass |
//!
//! All field arithmetic is over `F_ℓ` for an odd prime modulus `ℓ < 2^63`.
//! Primality is a caller obligation, exactly as in `large_prime_filter`.
//! Nothing here claims a speedup of a complete ECDLP solve; the evaluator in
//! [`eval`] records the measurements the catalog asks for.

use super::large_prime_filter::PartialRelation;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};

pub mod eval;
pub mod witness;
pub mod projected_rank;

// ---------------------------------------------------------------------------
// Field helpers (uncounted; the evaluator times phases instead)
// ---------------------------------------------------------------------------

#[inline]
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

#[inline]
fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

#[inline]
fn neg_mod(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}

fn inv_mod(a: u64, p: u64) -> Option<u64> {
    let (mut old_r, mut r) = (a as i128, p as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    (old_r == 1).then(|| old_s.rem_euclid(p as i128) as u64)
}

fn valid_modulus(p: u64) -> bool {
    (3..(1u64 << 63)).contains(&p) && p % 2 == 1
}

/// Errors from this module.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LpGraphError {
    InvalidModulus,
    /// A gain-DSU row had more than two nonzero LP coefficients.
    RowTooWide {
        row: usize,
        entries: usize,
    },
    /// A coefficient was not invertible: the modulus is not prime.
    NonInvertible {
        value: u64,
    },
    ColumnOutOfRange {
        column: usize,
    },
    /// A direct caller supplied a zero/out-of-range coefficient or a repeated
    /// LP column. Use LpIndex::build to normalize collector input first.
    NonCanonicalRow { row: usize },
}

// ---------------------------------------------------------------------------
// LP index: canonical sparse LP block
// ---------------------------------------------------------------------------

/// The large-prime block `A_L` of a relation batch with LP identities renamed
/// to dense column indices `0..columns.len()`. Repeated LP keys in one row are
/// combined and zero coefficients dropped, as the catalog requires before any
/// graph argument ("raw multiplicity is not algebraic degree").
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LpIndex {
    pub modulus: u64,
    /// Sorted LP identities; column `j` is `columns[j]`.
    pub columns: Vec<u64>,
    /// Per row, sorted `(column, nonzero coefficient)`.
    pub rows: Vec<Vec<(usize, u64)>>,
    /// Collector id of each row.
    pub row_ids: Vec<u64>,
}

impl LpIndex {
    pub fn build(relations: &[PartialRelation], modulus: u64) -> Result<Self, LpGraphError> {
        if !valid_modulus(modulus) {
            return Err(LpGraphError::InvalidModulus);
        }
        let columns: Vec<u64> = relations
            .iter()
            .flat_map(|r| r.large.iter().map(|&(q, _)| q))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let position: HashMap<u64, usize> =
            columns.iter().enumerate().map(|(j, &q)| (q, j)).collect();
        let rows = relations
            .iter()
            .map(|r| {
                let mut merged = BTreeMap::<usize, u64>::new();
                for &(q, c) in &r.large {
                    let slot = merged.entry(position[&q]).or_default();
                    *slot = add_mod(*slot, c % modulus, modulus);
                }
                merged.into_iter().filter(|&(_, c)| c != 0).collect()
            })
            .collect();
        Ok(Self {
            modulus,
            columns,
            rows,
            row_ids: relations.iter().map(|r| r.id).collect(),
        })
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    /// Nonzero incidences `I`.
    pub fn incidences(&self) -> usize {
        self.rows.iter().map(Vec::len).sum()
    }

    /// Row adjacency as column lists (the bipartite support graph).
    pub fn row_supports(&self) -> Vec<Vec<usize>> {
        self.rows
            .iter()
            .map(|r| r.iter().map(|&(c, _)| c).collect())
            .collect()
    }

    /// Edges `(u, v, row)` of the LP multigraph formed by rows with exactly
    /// two LP entries. Rows of other arity are not edges.
    pub fn two_entry_edges(&self) -> Vec<(usize, usize, usize)> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.len() == 2)
            .map(|(i, r)| (r[0].0, r[1].0, i))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// 1. Gain-aware DSU (catalog §6)
// ---------------------------------------------------------------------------

/// What one processed row did to the LP rank.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GainRowEffect {
    /// Joined two components.
    Union { rank_increment: u8 },
    /// Closed a cycle inside one component (`γ = a p_u + b p_v`).
    Cycle { balanced: bool, rank_increment: u8 },
    /// A one-entry row pinned its component.
    Singleton { rank_increment: u8 },
    /// An all-zero LP row: dependent, nothing to do.
    Zero,
}

impl GainRowEffect {
    pub fn rank_increment(self) -> usize {
        match self {
            Self::Union { rank_increment }
            | Self::Cycle { rank_increment, .. }
            | Self::Singleton { rank_increment } => rank_increment as usize,
            Self::Zero => 0,
        }
    }
}

/// Weighted union-find with multiplicative potentials over `F_ℓ`: exact rank
/// accounting for LP rows with at most two nonzero coefficients.
///
/// Invariant: `x_u = p_u · t_root(u)` with `p_u ≠ 0` for every solution of the
/// tree equations; a root is *flagged* (pinned) once a singleton row or an
/// inconsistent cycle forces its homogeneous parameter to zero. Then
/// `rank(A_L) = v − #unflagged components` (the frame-rank principle `v − b`,
/// Zaslavsky). Union by rank and path compression give `O((v + R) α(v))`
/// pointer operations and field operations, one inversion counted as one
/// field operation. Witness expansion is not part of this bound.
#[derive(Clone, Debug)]
pub struct GainDsu {
    modulus: u64,
    parent: Vec<usize>,
    /// `pot[u]`: `x_u = pot[u] · x_parent(u)`; `1` at roots.
    pot: Vec<u64>,
    union_rank: Vec<u8>,
    /// Valid at roots.
    pinned: Vec<bool>,
    unflagged: usize,
    rows: usize,
    rank: usize,
    unions: usize,
    balanced_cycles: usize,
    unbalanced_cycles: usize,
    singletons: usize,
    zero_rows: usize,
}

/// Rank metadata after a batch of rows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GainDsuSnapshot {
    pub vertices: usize,
    pub rows: usize,
    pub rank: usize,
    /// `dim ker(A_Lᵀ) = R − rank`: independent LP-cancelling combinations.
    pub left_nullity: usize,
    pub components: usize,
    pub unflagged_components: usize,
    pub unions: usize,
    pub balanced_cycles: usize,
    pub unbalanced_cycles: usize,
    pub singletons: usize,
    pub zero_rows: usize,
}

impl GainDsu {
    pub fn new(vertices: usize, modulus: u64) -> Result<Self, LpGraphError> {
        if !valid_modulus(modulus) {
            return Err(LpGraphError::InvalidModulus);
        }
        Ok(Self {
            modulus,
            parent: (0..vertices).collect(),
            pot: vec![1; vertices],
            union_rank: vec![0; vertices],
            pinned: vec![false; vertices],
            unflagged: vertices,
            rows: 0,
            rank: 0,
            unions: 0,
            balanced_cycles: 0,
            unbalanced_cycles: 0,
            singletons: 0,
            zero_rows: 0,
        })
    }

    /// Root of `u` and `p_u` with `x_u = p_u · x_root`. Path compression.
    pub fn find(&mut self, u: usize) -> (usize, u64) {
        let p = self.modulus;
        let mut path = Vec::new();
        let mut v = u;
        while self.parent[v] != v {
            path.push(v);
            v = self.parent[v];
        }
        let root = v;
        // Walk from the node nearest the root outwards, folding potentials.
        for &w in path.iter().rev() {
            let par = self.parent[w];
            if par != root {
                self.pot[w] = mul_mod(self.pot[w], self.pot[par], p);
            }
            self.parent[w] = root;
        }
        (root, if u == root { 1 } else { self.pot[u] })
    }

    /// Process one row with distinct columns and coefficients in 1..ℓ.
    /// Column order is immaterial (including closing edges in reverse order).
    /// Validate the entire input before changing *any* rank/count metadata.
    /// Normalize raw collector rows with LpIndex::build first.
    pub fn add_row(&mut self, terms: &[(usize, u64)]) -> Result<GainRowEffect, LpGraphError> {
        let p = self.modulus;
        if terms.len() > 2 {
            return Err(LpGraphError::RowTooWide {
                row: self.rows,
                entries: terms.len(),
            });
        }
        for (i, &(c, a)) in terms.iter().enumerate() {
            if c >= self.parent.len() {
                return Err(LpGraphError::ColumnOutOfRange { column: c });
            }
            if a == 0 || a >= p || (i == 1 && c == terms[0].0) {
                return Err(LpGraphError::NonCanonicalRow { row: self.rows });
            }
        }
        let effect = match *terms {
            [] => {
                self.zero_rows += 1;
                GainRowEffect::Zero
            }
            [(u, _)] => {
                self.singletons += 1;
                let (r, _) = self.find(u);
                let inc = u8::from(!self.pinned[r]);
                if !self.pinned[r] {
                    self.pinned[r] = true;
                    self.unflagged -= 1;
                }
                GainRowEffect::Singleton {
                    rank_increment: inc,
                }
            }
            [(u, a), (v, b)] => {
                let (ru, pu) = self.find(u);
                let (rv, pv) = self.find(v);
                let au = mul_mod(a, pu, p);
                let bv = mul_mod(b, pv, p);
                if ru != rv {
                    let before = usize::from(!self.pinned[ru]) + usize::from(!self.pinned[rv]);
                    let pinned = self.pinned[ru] || self.pinned[rv];
                    // a p_u t_ru + b p_v t_rv = 0.
                    let (child, root, num, den) = if self.union_rank[ru] < self.union_rank[rv] {
                        (ru, rv, bv, au) // t_ru = -(b p_v / a p_u) t_rv
                    } else {
                        (rv, ru, au, bv) // t_rv = -(a p_u / b p_v) t_ru
                    };
                    let inv = inv_mod(den, p).ok_or(LpGraphError::NonInvertible { value: den })?;
                    self.unions += 1;
                    self.parent[child] = root;
                    self.pot[child] = neg_mod(mul_mod(num, inv, p), p);
                    if self.union_rank[ru] == self.union_rank[rv] {
                        self.union_rank[root] += 1;
                    }
                    self.pinned[root] = pinned;
                    let after = usize::from(!pinned);
                    self.unflagged = self.unflagged + after - before;
                    GainRowEffect::Union {
                        rank_increment: (before - after) as u8,
                    }
                } else {
                    let gamma = add_mod(au, bv, p);
                    let balanced = gamma == 0;
                    if balanced {
                        self.balanced_cycles += 1;
                    } else {
                        self.unbalanced_cycles += 1;
                    }
                    let inc = u8::from(!balanced && !self.pinned[ru]);
                    if inc == 1 {
                        self.pinned[ru] = true;
                        self.unflagged -= 1;
                    }
                    GainRowEffect::Cycle {
                        balanced,
                        rank_increment: inc,
                    }
                }
            }
            _ => unreachable!("arity checked before mutation"),
        };
        self.rows += 1;
        self.rank += effect.rank_increment();
        Ok(effect)
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn snapshot(&self) -> GainDsuSnapshot {
        let components = (0..self.parent.len())
            .filter(|&u| self.parent[u] == u)
            .count();
        debug_assert_eq!(self.rank, self.parent.len() - self.unflagged);
        GainDsuSnapshot {
            vertices: self.parent.len(),
            rows: self.rows,
            rank: self.rank,
            left_nullity: self.rows - self.rank,
            components,
            unflagged_components: self.unflagged,
            unions: self.unions,
            balanced_cycles: self.balanced_cycles,
            unbalanced_cycles: self.unbalanced_cycles,
            singletons: self.singletons,
            zero_rows: self.zero_rows,
        }
    }
}

/// Exact `rank(A_L)` and left nullity of an LP block whose rows all have at
/// most two entries. Fails with [`LpGraphError::RowTooWide`] otherwise.
pub fn gain_dsu_rank(index: &LpIndex) -> Result<GainDsuSnapshot, LpGraphError> {
    let mut dsu = GainDsu::new(index.num_columns(), index.modulus)?;
    for (i, row) in index.rows.iter().enumerate() {
        if row.len() > 2 {
            return Err(LpGraphError::RowTooWide {
                row: i,
                entries: row.len(),
            });
        }
        dsu.add_row(row)?;
    }
    Ok(dsu.snapshot())
}

/// Reference rank by dense Gaussian elimination over `F_ℓ`, `O(R C min(R, C))`.
/// For audits at small sizes only.
pub fn dense_rank(
    rows: &[Vec<(usize, u64)>],
    columns: usize,
    modulus: u64,
) -> Result<usize, LpGraphError> {
    let p = modulus;
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| {
            let mut d = vec![0u64; columns];
            for &(c, a) in r {
                d[c] = add_mod(d[c], a % p, p);
            }
            d
        })
        .collect();
    let mut rank = 0;
    for col in 0..columns {
        let Some(piv) = (rank..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(rank, piv);
        let inv = inv_mod(m[rank][col], p).ok_or(LpGraphError::NonInvertible {
            value: m[rank][col],
        })?;
        let pivot_row = m[rank].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != rank && row[col] != 0 {
                let f = mul_mod(row[col], inv, p);
                for (x, &y) in row.iter_mut().zip(&pivot_row) {
                    *x = add_mod(*x, neg_mod(mul_mod(f, y, p), p), p);
                }
            }
        }
        rank += 1;
        if rank == m.len() {
            break;
        }
    }
    Ok(rank)
}

// ---------------------------------------------------------------------------
// 2. Core numbers (Batagelj–Zaveršnik)
// ---------------------------------------------------------------------------

/// Core number of every vertex of an undirected (multi)graph given by
/// adjacency lists; parallel edges count toward degree. `O(v + e)`.
///
/// A 2-core locates potential cyclic structure. Pruning a higher core is not
/// a general algebraic preservation rule (catalog §1).
pub fn core_numbers(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut deg: Vec<usize> = adj.iter().map(Vec::len).collect();
    let max_deg = deg.iter().copied().max().unwrap_or(0);
    let mut bin = vec![0usize; max_deg + 1];
    for &d in &deg {
        bin[d] += 1;
    }
    let mut start = 0;
    for b in bin.iter_mut() {
        let count = *b;
        *b = start;
        start += count;
    }
    let mut pos = vec![0usize; n];
    let mut vert = vec![0usize; n];
    for v in 0..n {
        pos[v] = bin[deg[v]];
        vert[pos[v]] = v;
        bin[deg[v]] += 1;
    }
    for d in (1..=max_deg).rev() {
        bin[d] = bin[d - 1];
    }
    if max_deg > 0 || n > 0 {
        bin[0] = 0;
    }
    for i in 0..n {
        let v = vert[i];
        for &u in &adj[v] {
            if deg[u] > deg[v] {
                let du = deg[u];
                let pu = pos[u];
                let pw = bin[du];
                let w = vert[pw];
                if u != w {
                    pos[u] = pw;
                    vert[pu] = w;
                    pos[w] = pu;
                    vert[pw] = u;
                }
                bin[du] += 1;
                deg[u] -= 1;
            }
        }
    }
    deg
}

/// Adjacency lists of an undirected multigraph on `n` vertices.
pub fn adjacency(n: usize, edges: &[(usize, usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v, _) in edges {
        adj[u].push(v);
        if u != v {
            adj[v].push(u);
        }
    }
    adj
}

/// The bipartite row–column incidence graph on `R + C` vertices
/// (rows first). Its 2-core is the hypergraph core after peeling.
pub fn incidence_adjacency(index: &LpIndex) -> Vec<Vec<usize>> {
    let r = index.num_rows();
    let mut adj = vec![Vec::new(); r + index.num_columns()];
    for (i, row) in index.rows.iter().enumerate() {
        for &(c, _) in row {
            adj[i].push(r + c);
            adj[r + c].push(i);
        }
    }
    adj
}

// ---------------------------------------------------------------------------
// 3. Hopcroft–Karp and Dulmage–Mendelsohn
// ---------------------------------------------------------------------------

/// A maximum matching of a bipartite graph `L × R`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Matching {
    pub mate_left: Vec<Option<usize>>,
    pub mate_right: Vec<Option<usize>>,
    pub size: usize,
    /// BFS phases run (`O(√(L + R))`).
    pub phases: usize,
}

/// Hopcroft–Karp maximum matching, `O(I √(L + R))`. `adj[l]` lists the right
/// vertices of left vertex `l`. On the row–column support of `A_L` the
/// matching size is the structural rank, an upper bound on the field rank.
pub fn hopcroft_karp(n_left: usize, n_right: usize, adj: &[Vec<usize>]) -> Matching {
    const INF: usize = usize::MAX;
    let mut mate_l = vec![None; n_left];
    let mut mate_r: Vec<Option<usize>> = vec![None; n_right];
    let mut dist = vec![INF; n_left];
    let mut size = 0;
    let mut phases = 0;
    loop {
        // BFS from free left vertices.
        let mut queue = VecDeque::new();
        for l in 0..n_left {
            if mate_l[l].is_none() {
                dist[l] = 0;
                queue.push_back(l);
            } else {
                dist[l] = INF;
            }
        }
        let mut found = false;
        while let Some(l) = queue.pop_front() {
            for &r in &adj[l] {
                match mate_r[r] {
                    None => found = true,
                    Some(l2) if dist[l2] == INF => {
                        dist[l2] = dist[l] + 1;
                        queue.push_back(l2);
                    }
                    _ => {}
                }
            }
        }
        if !found {
            break;
        }
        phases += 1;
        // Iterative DFS along layered graph.
        let mut it = vec![0usize; n_left];
        for start in 0..n_left {
            if mate_l[start].is_some() {
                continue;
            }
            let mut stack = vec![start];
            let mut path_found = false;
            while let Some(&l) = stack.last() {
                if it[l] >= adj[l].len() {
                    dist[l] = INF;
                    stack.pop();
                    continue;
                }
                let r = adj[l][it[l]];
                it[l] += 1;
                match mate_r[r] {
                    None => {
                        // Augment along the stack.
                        let mut r_cur = r;
                        for &lv in stack.iter().rev() {
                            let prev = mate_l[lv];
                            mate_l[lv] = Some(r_cur);
                            mate_r[r_cur] = Some(lv);
                            match prev {
                                Some(pr) => r_cur = pr,
                                None => break,
                            }
                        }
                        path_found = true;
                        break;
                    }
                    Some(l2) if dist[l2] == dist[l] + 1 => stack.push(l2),
                    _ => {}
                }
            }
            if path_found {
                size += 1;
            }
        }
    }
    Matching {
        mate_left: mate_l,
        mate_right: mate_r,
        size,
        phases,
    }
}

/// Coarse and fine Dulmage–Mendelsohn decomposition of the row–column support.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DmDecomposition {
    pub structural_rank: usize,
    /// Rows/columns of the over-determined block (reachable from unmatched rows).
    pub over_rows: usize,
    pub over_columns: usize,
    /// Rows/columns of the under-determined block (reachable from unmatched columns).
    pub under_rows: usize,
    pub under_columns: usize,
    /// The square, perfectly matched well-determined block.
    pub well_size: usize,
    /// Fine blocks (strongly connected components) of the well-determined part,
    /// in block-triangular order.
    pub fine_blocks: Vec<usize>,
    /// `R − structural rank`: a lower bound on the LP left nullity.
    pub structural_left_nullity_lower_bound: usize,
    /// Per row: 0 over, 1 well, 2 under.
    #[serde(skip)]
    pub row_class: Vec<u8>,
    /// Per column: 0 over, 1 well, 2 under.
    #[serde(skip)]
    pub column_class: Vec<u8>,
}

/// Dulmage–Mendelsohn decomposition from a maximum matching, `O(R + C + I)`.
/// `rows[i]` lists the columns of row `i`. Coefficient singularities may
/// remain: the decomposition is structural (catalog §2).
#[allow(clippy::needless_range_loop)]
pub fn dulmage_mendelsohn(
    rows: &[Vec<usize>],
    n_cols: usize,
    matching: &Matching,
) -> DmDecomposition {
    let n_rows = rows.len();
    let mut col_rows = vec![Vec::new(); n_cols];
    for (i, r) in rows.iter().enumerate() {
        for &c in r {
            col_rows[c].push(i);
        }
    }
    // Over-determined: alternating BFS from unmatched rows
    // (row -> any column; column -> its matched row).
    let mut row_over = vec![false; n_rows];
    let mut col_over = vec![false; n_cols];
    let mut q = VecDeque::new();
    for i in 0..n_rows {
        if matching.mate_left[i].is_none() {
            row_over[i] = true;
            q.push_back(i);
        }
    }
    while let Some(i) = q.pop_front() {
        for &c in &rows[i] {
            if !col_over[c] {
                col_over[c] = true;
                if let Some(i2) = matching.mate_right[c] {
                    if !row_over[i2] {
                        row_over[i2] = true;
                        q.push_back(i2);
                    }
                }
            }
        }
    }
    // Under-determined: alternating BFS from unmatched columns
    // (column -> any row; row -> its matched column).
    let mut row_under = vec![false; n_rows];
    let mut col_under = vec![false; n_cols];
    for c in 0..n_cols {
        if matching.mate_right[c].is_none() {
            col_under[c] = true;
            q.push_back(c);
        }
    }
    while let Some(c) = q.pop_front() {
        for &i in &col_rows[c] {
            if !row_under[i] {
                row_under[i] = true;
                if let Some(c2) = matching.mate_left[i] {
                    if !col_under[c2] {
                        col_under[c2] = true;
                        q.push_back(c2);
                    }
                }
            }
        }
    }
    let row_class: Vec<u8> = (0..n_rows)
        .map(|i| {
            if row_over[i] {
                0
            } else if row_under[i] {
                2
            } else {
                1
            }
        })
        .collect();
    let column_class: Vec<u8> = (0..n_cols)
        .map(|c| {
            if col_over[c] {
                0
            } else if col_under[c] {
                2
            } else {
                1
            }
        })
        .collect();
    // Fine decomposition of the well part: directed graph on well columns,
    // c -> c' when the row matched to c has an entry in c' (also well).
    let well_cols: Vec<usize> = (0..n_cols).filter(|&c| column_class[c] == 1).collect();
    let mut local = vec![usize::MAX; n_cols];
    for (k, &c) in well_cols.iter().enumerate() {
        local[c] = k;
    }
    let digraph: Vec<Vec<usize>> = well_cols
        .iter()
        .map(|&c| {
            let i = matching.mate_right[c].expect("well columns are matched");
            rows[i]
                .iter()
                .filter(|&&c2| c2 != c && column_class[c2] == 1)
                .map(|&c2| local[c2])
                .collect()
        })
        .collect();
    let fine_blocks = scc_sizes(&digraph);
    DmDecomposition {
        structural_rank: matching.size,
        over_rows: row_over.iter().filter(|&&b| b).count(),
        over_columns: col_over.iter().filter(|&&b| b).count(),
        under_rows: row_class.iter().filter(|&&k| k == 2).count(),
        under_columns: col_under.iter().filter(|&&b| b).count(),
        well_size: well_cols.len(),
        fine_blocks,
        structural_left_nullity_lower_bound: n_rows - matching.size,
        row_class,
        column_class,
    }
}

/// Sizes of the strongly connected components (iterative Tarjan), in reverse
/// topological order of the condensation.
pub fn scc_sizes(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut index = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut next = 0;
    let mut sizes = Vec::new();
    for s in 0..n {
        if index[s] != usize::MAX {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(s, 0)];
        index[s] = next;
        low[s] = next;
        next += 1;
        stack.push(s);
        on_stack[s] = true;
        while let Some(&mut (v, ref mut i)) = call.last_mut() {
            if *i < g[v].len() {
                let w = g[v][*i];
                *i += 1;
                if index[w] == usize::MAX {
                    index[w] = next;
                    low[w] = next;
                    next += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    call.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                call.pop();
                if let Some(&(u, _)) = call.last() {
                    low[u] = low[u].min(low[v]);
                }
                if low[v] == index[v] {
                    let mut size = 0;
                    loop {
                        let w = stack.pop().expect("tarjan stack");
                        on_stack[w] = false;
                        size += 1;
                        if w == v {
                            break;
                        }
                    }
                    sizes.push(size);
                }
            }
        }
    }
    sizes
}

// ---------------------------------------------------------------------------
// 4. Spanning forests and fundamental cycles
// ---------------------------------------------------------------------------

/// A spanning forest of a multigraph, as a set of edge positions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanningForest {
    /// `true` for edges (positions in the input edge list) in the forest.
    pub tree_edge: Vec<bool>,
    pub components: usize,
}

/// BFS spanning forest, `O(v + e)`.
pub fn bfs_forest(n: usize, edges: &[(usize, usize, usize)]) -> SpanningForest {
    let mut inc = vec![Vec::new(); n];
    for (k, &(u, v, _)) in edges.iter().enumerate() {
        inc[u].push((v, k));
        inc[v].push((u, k));
    }
    let mut seen = vec![false; n];
    let mut tree_edge = vec![false; edges.len()];
    let mut components = 0;
    for s in 0..n {
        if seen[s] {
            continue;
        }
        components += 1;
        seen[s] = true;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &(v, k) in &inc[u] {
                if !seen[v] {
                    seen[v] = true;
                    tree_edge[k] = true;
                    q.push_back(v);
                }
            }
        }
    }
    SpanningForest {
        tree_edge,
        components,
    }
}

/// Parameters of the ball-growing low-stretch heuristic.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LowStretchParams {
    /// Grow a ball until `boundary ≤ ratio · volume`.
    pub boundary_ratio: f64,
}

impl Default for LowStretchParams {
    fn default() -> Self {
        Self {
            boundary_ratio: 0.5,
        }
    }
}

/// Low-stretch spanning forest by Alon–Karp–Peleg–West-style iterated ball
/// growing and contraction. **Heuristic**: it does not carry the
/// Abraham–Neiman `O(e log v log log v)` total-stretch guarantee the catalog
/// cites; the evaluator measures `L` against BFS instead. Each level is linear
/// in the contracted graph; the number of levels is at most `log₂ v` when every
/// ball absorbs a neighbour, and at most `v` in the worst case.
pub fn low_stretch_forest(
    n: usize,
    edges: &[(usize, usize, usize)],
    params: LowStretchParams,
) -> SpanningForest {
    let mut cluster: Vec<usize> = (0..n).collect(); // original vertex -> current cluster
    let mut n_clusters = n;
    let mut tree_edge = vec![false; edges.len()];
    let mut uf = SimpleUf::new(n);
    loop {
        // Inter-cluster edges in the contracted graph.
        let mut inc: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n_clusters];
        let mut any = false;
        for (k, &(u, v, _)) in edges.iter().enumerate() {
            let (a, b) = (cluster[u], cluster[v]);
            if a != b {
                inc[a].push((b, k));
                inc[b].push((a, k));
                any = true;
            }
        }
        if !any {
            break;
        }
        let mut assign = vec![usize::MAX; n_clusters];
        let mut next_id = 0;
        let mut merged = false;
        for s in 0..n_clusters {
            if assign[s] != usize::MAX {
                continue;
            }
            let id = next_id;
            next_id += 1;
            assign[s] = id;
            let mut ball = vec![s];
            let mut frontier = vec![s];
            let mut volume: usize = inc[s].len();
            loop {
                let boundary: usize = frontier
                    .iter()
                    .flat_map(|&x| inc[x].iter())
                    .filter(|&&(y, _)| assign[y] == usize::MAX)
                    .count();
                if boundary == 0
                    || (volume > 0
                        && boundary as f64 <= params.boundary_ratio * volume as f64
                        && ball.len() > 1)
                {
                    break;
                }
                // Grow one BFS layer, recording tree edges.
                let mut next_frontier = Vec::new();
                for &x in &frontier {
                    for &(y, k) in &inc[x] {
                        if assign[y] == usize::MAX {
                            assign[y] = id;
                            let (eu, ev, _) = edges[k];
                            if uf.union(eu, ev) {
                                tree_edge[k] = true;
                            }
                            merged = true;
                            volume += inc[y].len();
                            next_frontier.push(y);
                            ball.push(y);
                        }
                    }
                }
                if next_frontier.is_empty() {
                    break;
                }
                frontier = next_frontier;
            }
        }
        for c in cluster.iter_mut() {
            *c = assign[*c];
        }
        n_clusters = next_id;
        if !merged {
            break;
        }
    }
    let components = (0..n).filter(|&v| uf.find(v) == v).count();
    SpanningForest {
        tree_edge,
        components,
    }
}

struct SimpleUf {
    parent: Vec<usize>,
}

impl SimpleUf {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn find(&mut self, mut v: usize) -> usize {
        while self.parent[v] != v {
            self.parent[v] = self.parent[self.parent[v]];
            v = self.parent[v];
        }
        v
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.parent[ra] = rb;
        true
    }
}

/// Fundamental-cycle statistics of a spanning forest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycleStats {
    /// Non-tree edges = fundamental cycles (cyclomatic number `e − v + c`).
    pub cycles: usize,
    /// Total edge appearances `L` over all fundamental cycles.
    pub total_length: usize,
    pub max_length: usize,
}

/// Total length `L` of the fundamental cycles of `forest`, `O(v + e + L)`
/// (the walk to the lowest common ancestor is paid per cycle edge).
pub fn fundamental_cycle_length(
    n: usize,
    edges: &[(usize, usize, usize)],
    forest: &SpanningForest,
) -> CycleStats {
    let mut adj = vec![Vec::new(); n];
    for (k, &(u, v, _)) in edges.iter().enumerate() {
        if forest.tree_edge[k] {
            adj[u].push(v);
            adj[v].push(u);
        }
    }
    let mut parent = vec![usize::MAX; n];
    let mut depth = vec![0usize; n];
    let mut seen = vec![false; n];
    for s in 0..n {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    parent[v] = u;
                    depth[v] = depth[u] + 1;
                    q.push_back(v);
                }
            }
        }
    }
    let mut stats = CycleStats::default();
    for (k, &(u, v, _)) in edges.iter().enumerate() {
        if forest.tree_edge[k] {
            continue;
        }
        let (mut a, mut b) = (u, v);
        let mut len = 1; // the chord
        while a != b {
            if depth[a] >= depth[b] {
                a = parent[a];
            } else {
                b = parent[b];
            }
            len += 1;
        }
        stats.cycles += 1;
        stats.total_length += len;
        stats.max_length = stats.max_length.max(len);
    }
    stats
}

// ---------------------------------------------------------------------------
// 5. Minimum-degree ordering
// ---------------------------------------------------------------------------

/// Result of symbolic minimum-degree elimination.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EliminationStats {
    #[serde(skip)]
    pub order: Vec<usize>,
    pub vertices: usize,
    pub edges: usize,
    /// Edges created by elimination (symbolic fill).
    pub fill_edges: usize,
    /// Largest neighbourhood at elimination: an upper bound on treewidth,
    /// the `t` of the catalog's `O(N min{t³, t² log² N})` route.
    pub width: usize,
    /// `Σ d_i²`, the clique-update work actually spent.
    pub clique_work: u64,
    /// `true` when `max_fill` stopped the elimination early.
    pub truncated: bool,
}

/// Greedy minimum-degree ordering on a simple undirected graph with explicit
/// clique updates, `O(Σ d_i² log v)`. Global minimum fill is NP-hard
/// (Yannakakis); this is the standard heuristic. Stops once `max_fill` fill
/// edges have been created, reporting `truncated`.
pub fn minimum_degree(adj: &[Vec<usize>], max_fill: usize) -> EliminationStats {
    let n = adj.len();
    let mut nb: Vec<BTreeSet<usize>> = adj
        .iter()
        .enumerate()
        .map(|(v, a)| a.iter().copied().filter(|&u| u != v).collect())
        .collect();
    let edges = nb.iter().map(BTreeSet::len).sum::<usize>() / 2;
    let mut heap: BinaryHeap<Reverse<(usize, usize)>> =
        (0..n).map(|v| Reverse((nb[v].len(), v))).collect();
    let mut done = vec![false; n];
    let mut stats = EliminationStats {
        vertices: n,
        edges,
        ..Default::default()
    };
    while let Some(Reverse((d, v))) = heap.pop() {
        if done[v] || d != nb[v].len() {
            continue;
        }
        done[v] = true;
        stats.order.push(v);
        let ns: Vec<usize> = nb[v].iter().copied().collect();
        stats.width = stats.width.max(ns.len());
        stats.clique_work += (ns.len() as u64).pow(2);
        for &u in &ns {
            nb[u].remove(&v);
        }
        for (i, &a) in ns.iter().enumerate() {
            for &b in &ns[i + 1..] {
                if nb[a].insert(b) {
                    nb[b].insert(a);
                    stats.fill_edges += 1;
                }
            }
        }
        nb[v].clear();
        for &u in &ns {
            heap.push(Reverse((nb[u].len(), u)));
        }
        if stats.fill_edges > max_fill {
            stats.truncated = true;
            break;
        }
    }
    stats
}

/// Column interaction graph (pattern of `AᵀA`): columns adjacent when they
/// share a row. Rows wider than `max_row` are skipped (counted in the return).
pub fn column_interaction_graph(
    rows: &[Vec<usize>],
    n_cols: usize,
    max_row: usize,
) -> (Vec<Vec<usize>>, usize) {
    let mut sets = vec![BTreeSet::new(); n_cols];
    let mut skipped = 0;
    for r in rows {
        if r.len() > max_row {
            skipped += 1;
            continue;
        }
        for (i, &a) in r.iter().enumerate() {
            for &b in &r[i + 1..] {
                if a != b {
                    sets[a].insert(b);
                    sets[b].insert(a);
                }
            }
        }
    }
    (
        sets.into_iter().map(|s| s.into_iter().collect()).collect(),
        skipped,
    )
}

// ---------------------------------------------------------------------------
// 6. Hypergraph partitioning for SpMV (column-net model)
// ---------------------------------------------------------------------------

/// A `k`-way row partition and its column-net communication volume.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Partition {
    #[serde(skip)]
    pub part: Vec<usize>,
    pub parts: usize,
    pub part_sizes: Vec<usize>,
    /// `Σ_j (λ_j − 1)`: column data communicated per product (Çatalyürek–Aykanat).
    pub connectivity_minus_one: usize,
    pub cut_nets: usize,
    pub fm_passes: usize,
    pub fm_moves: usize,
}

/// `Σ_j (λ_j − 1)` and the number of cut nets for row partition `part`.
pub fn connectivity_minus_one(
    rows: &[Vec<usize>],
    n_cols: usize,
    part: &[usize],
) -> (usize, usize) {
    let mut parts_of: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n_cols];
    for (i, r) in rows.iter().enumerate() {
        for &c in r {
            parts_of[c].insert(part[i]);
        }
    }
    let mut total = 0;
    let mut cut = 0;
    for s in &parts_of {
        if s.len() > 1 {
            total += s.len() - 1;
            cut += 1;
        }
    }
    (total, cut)
}

/// Contiguous block partition of the rows in their given order: the baseline.
pub fn block_partition(rows: &[Vec<usize>], n_cols: usize, k: usize) -> Partition {
    let n = rows.len();
    let k = k.max(1);
    let part: Vec<usize> = (0..n).map(|i| i * k / n.max(1)).collect();
    finish_partition(rows, n_cols, part, k, 0, 0)
}

fn finish_partition(
    rows: &[Vec<usize>],
    n_cols: usize,
    part: Vec<usize>,
    k: usize,
    passes: usize,
    moves: usize,
) -> Partition {
    let mut part_sizes = vec![0; k];
    for &p in &part {
        part_sizes[p] += 1;
    }
    let (connectivity_minus_one, cut_nets) = connectivity_minus_one(rows, n_cols, &part);
    Partition {
        part,
        parts: k,
        part_sizes,
        connectivity_minus_one,
        cut_nets,
        fm_passes: passes,
        fm_moves: moves,
    }
}

/// Parameters for [`fm_partition`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FmParams {
    pub parts: usize,
    /// Allowed imbalance per bisection: each side ≤ `⌈(1 + ε)·target⌉ + 1`
    /// (one vertex of slack so a pass can leave an exact split).
    pub imbalance: f64,
    pub max_passes: usize,
}

impl Default for FmParams {
    fn default() -> Self {
        Self {
            parts: 4,
            imbalance: 0.03,
            max_passes: 8,
        }
    }
}

/// `k`-way row partition by recursive bisection, each bisection started from
/// the contiguous split and refined by Fiduccia–Mattheyses passes on the
/// column-net hypergraph (vertices = rows, nets = columns). For two parts the
/// cut-net count equals `Σ (λ_j − 1)`, so FM minimises the communication
/// metric directly. Each pass costs `O(I log R)` here (lazy binary heaps
/// instead of FM's gain buckets).
pub fn fm_partition(rows: &[Vec<usize>], n_cols: usize, params: FmParams) -> Partition {
    let n = rows.len();
    let mut part = vec![0usize; n];
    let mut passes = 0;
    let mut moves = 0;
    let k = params.parts.max(1);
    let all: Vec<usize> = (0..n).collect();
    recurse(
        rows,
        n_cols,
        &all,
        0,
        k,
        params,
        &mut part,
        &mut passes,
        &mut moves,
    );
    finish_partition(rows, n_cols, part, k, passes, moves)
}

#[allow(clippy::too_many_arguments)]
fn recurse(
    rows: &[Vec<usize>],
    n_cols: usize,
    verts: &[usize],
    first: usize,
    k: usize,
    params: FmParams,
    part: &mut [usize],
    passes: &mut usize,
    moves: &mut usize,
) {
    if k <= 1 || verts.len() <= 1 {
        for &v in verts {
            part[v] = first;
        }
        return;
    }
    let k_left = k / 2;
    let target_left = verts.len() * k_left / k;
    let side = fm_bisect(rows, n_cols, verts, target_left, params, passes, moves);
    let left: Vec<usize> = verts
        .iter()
        .zip(&side)
        .filter(|(_, &s)| s == 0)
        .map(|(&v, _)| v)
        .collect();
    let right: Vec<usize> = verts
        .iter()
        .zip(&side)
        .filter(|(_, &s)| s == 1)
        .map(|(&v, _)| v)
        .collect();
    recurse(
        rows, n_cols, &left, first, k_left, params, part, passes, moves,
    );
    recurse(
        rows,
        n_cols,
        &right,
        first + k_left,
        k - k_left,
        params,
        part,
        passes,
        moves,
    );
}

/// One FM bisection of `verts` (side per local vertex), target `target_left`
/// on side 0.
fn fm_bisect(
    rows: &[Vec<usize>],
    n_cols: usize,
    verts: &[usize],
    target_left: usize,
    params: FmParams,
    passes: &mut usize,
    moves: &mut usize,
) -> Vec<u8> {
    let n = verts.len();
    // Local nets restricted to these vertices.
    let mut net_pins: HashMap<usize, Vec<usize>> = HashMap::new();
    for (lv, &v) in verts.iter().enumerate() {
        for &c in &rows[v] {
            debug_assert!(c < n_cols);
            net_pins.entry(c).or_default().push(lv);
        }
    }
    let nets: Vec<Vec<usize>> = net_pins.into_values().filter(|p| p.len() > 1).collect();
    let mut v_nets = vec![Vec::new(); n];
    for (j, pins) in nets.iter().enumerate() {
        for &lv in pins {
            v_nets[lv].push(j);
        }
    }
    let mut side: Vec<u8> = (0..n).map(|i| u8::from(i >= target_left)).collect();
    let cap = [
        ((target_left as f64) * (1.0 + params.imbalance)).ceil() as usize + 1,
        (((n - target_left) as f64) * (1.0 + params.imbalance)).ceil() as usize + 1,
    ];
    let cut_of = |side: &[u8]| -> i64 {
        nets.iter()
            .filter(|p| p.iter().any(|&v| side[v] == 0) && p.iter().any(|&v| side[v] == 1))
            .count() as i64
    };
    let mut best_cut = cut_of(&side);
    for _ in 0..params.max_passes {
        *passes += 1;
        let mut count = vec![[0usize; 2]; nets.len()];
        for (j, pins) in nets.iter().enumerate() {
            for &v in pins {
                count[j][side[v] as usize] += 1;
            }
        }
        let mut size = [0usize; 2];
        for &s in &side {
            size[s as usize] += 1;
        }
        let mut gain = vec![0i64; n];
        for v in 0..n {
            let f = side[v] as usize;
            for &j in &v_nets[v] {
                if count[j][f] == 1 {
                    gain[v] += 1;
                }
                if count[j][1 - f] == 0 {
                    gain[v] -= 1;
                }
            }
        }
        let mut heaps: [BinaryHeap<(i64, Reverse<usize>)>; 2] =
            [BinaryHeap::new(), BinaryHeap::new()];
        for v in 0..n {
            heaps[side[v] as usize].push((gain[v], Reverse(v)));
        }
        let mut locked = vec![false; n];
        let mut history = Vec::new();
        let mut cut = best_cut;
        let mut best_prefix = 0;
        let mut pass_best = best_cut;
        loop {
            // Best feasible top among the two sides.
            let mut choice: Option<(i64, usize)> = None;
            for f in 0..2 {
                if size[1 - f] + 1 > cap[1 - f] {
                    continue;
                }
                while let Some(&(g, Reverse(v))) = heaps[f].peek() {
                    if locked[v] || side[v] as usize != f || g != gain[v] {
                        heaps[f].pop();
                        continue;
                    }
                    if choice.is_none_or(|(bg, _)| g > bg) {
                        choice = Some((g, v));
                    }
                    break;
                }
            }
            let Some((g, v)) = choice else { break };
            let f = side[v] as usize;
            let t = 1 - f;
            locked[v] = true;
            for &j in &v_nets[v] {
                // Before the move.
                if count[j][t] == 0 {
                    for &u in &nets[j] {
                        if !locked[u] {
                            gain[u] += 1;
                            heaps[side[u] as usize].push((gain[u], Reverse(u)));
                        }
                    }
                } else if count[j][t] == 1 {
                    for &u in &nets[j] {
                        if side[u] as usize == t && !locked[u] {
                            gain[u] -= 1;
                            heaps[t].push((gain[u], Reverse(u)));
                        }
                    }
                }
                count[j][f] -= 1;
                count[j][t] += 1;
                // After the move.
                if count[j][f] == 0 {
                    for &u in &nets[j] {
                        if !locked[u] {
                            gain[u] -= 1;
                            heaps[side[u] as usize].push((gain[u], Reverse(u)));
                        }
                    }
                } else if count[j][f] == 1 {
                    for &u in &nets[j] {
                        if side[u] as usize == f && !locked[u] {
                            gain[u] += 1;
                            heaps[f].push((gain[u], Reverse(u)));
                        }
                    }
                }
            }
            side[v] = t as u8;
            size[f] -= 1;
            size[t] += 1;
            cut -= g;
            history.push(v);
            if cut < pass_best {
                pass_best = cut;
                best_prefix = history.len();
            }
        }
        // Roll back moves after the best prefix.
        for &v in &history[best_prefix..] {
            side[v] ^= 1;
        }
        *moves += best_prefix;
        debug_assert_eq!(cut_of(&side), pass_best);
        if pass_best >= best_cut {
            break;
        }
        best_cut = pass_best;
    }
    side
}
