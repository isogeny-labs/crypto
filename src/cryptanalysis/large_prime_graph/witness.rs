//! Exact, provenance-bearing LP-cancellation certificates for canonical arity <= 2.
//!
//! This is an intentionally conservative forest/anchor reference extractor.
//! Each returned witness is a combination of ORIGINAL LP rows (indices into
//! LpIndex::rows). It cancels every LP column over F_ell. This module does not
//! project or rank the factor-base payload, solve ECDLP, or claim a faster
//! asymptotic bound than existing filtering.
//!
//! The accepted independent rows form a forest of two-ended equations plus
//! at most one anchoring row in each forest component. When two anchored
//! components are linked, the new row is already dependent: treating every
//! graph bridge as an independent LP rank increase would be incorrect.

use super::{add_mod, inv_mod, mul_mod, neg_mod, valid_modulus, LpGraphError, LpIndex};
use crate::cryptanalysis::large_prime_filter::{
    replay_complete_relation, CompleteRelation, PartialRelation,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The first occurrence of row_index has coefficient one in `sources`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationWitness {
    pub row_index: usize,
    /// Sorted (original input row index, nonzero coefficient modulo ell).
    pub sources: Vec<(usize, u64)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessStats {
    pub rows: usize,
    pub independent_rows: usize,
    pub tree_edges: usize,
    pub anchors: usize,
    pub dependent_rows: usize,
    pub source_terms: usize,
    /// Legacy BFS adjacency-list scan counter (zero for parent-path traversal).
    pub adjacency_scans: u64,
    /// Parent edges reversed when joining a smaller forest component.
    #[serde(default)]
    pub reroot_steps: u64,
    /// Tree edge steps used while reducing original or anchor rows.
    pub path_steps: u64,
    /// Explicit modular inverse calls, including input validation.
    pub inverse_calls: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessBatch {
    pub modulus: u64,
    pub lp_columns: usize,
    /// Basis size of the LP row space, not useful factor-base rank.
    pub rank: usize,
    pub witnesses: Vec<CancellationWitness>,
    pub stats: WitnessStats,
}

#[derive(Clone, Debug)]
struct Reduced {
    roots: BTreeMap<usize, u64>,
    /// Adding these basis rows to the input row leaves only root variables.
    basis: BTreeMap<usize, u64>,
    scans: u64,
    steps: u64,
    inversions: u64,
}

fn add_term(map: &mut BTreeMap<usize, u64>, key: usize, value: u64, p: u64) {
    if value == 0 {
        return;
    }
    let next = add_mod(map.get(&key).copied().unwrap_or(0), value, p);
    if next == 0 {
        map.remove(&key);
    } else {
        map.insert(key, next);
    }
}

fn coeff_at(row: &[(usize, u64)], vertex: usize) -> u64 {
    row.iter()
        .find(|&&(v, _)| v == vertex)
        .expect("tree edge must contain both endpoints")
        .1
}

/// Streaming frame-matroid row basis plus exact left-kernel witnesses.
///
/// Paths now follow explicit parent edges rather than allocating a V-entry
/// BFS predecessor array for each query. Joining two trees reroots only the
/// smaller component; each vertex is rerooted at most O(log V) times, so all
/// reroot work is O(V log V). For R rows, maximum forest depth D and L
/// emitted source terms, the conservative ordered-map work is
/// O((R D + V log V) log(R+2) + L), excluding modular inverse bit costs.
/// D may be V, and witness materialization may dominate; no complete-cost
/// or asymptotic ECDLP advantage is assumed.
#[derive(Clone, Debug)]
pub struct ForestWitnessExtractor {
    p: u64,
    parent: Vec<usize>,
    size: Vec<usize>,
    /// Forest parent edges are distinct from the component union-find parent.
    /// The tree root remains the corresponding union-find component root.
    tree_parent: Vec<usize>,
    /// Input-row index for the edge to tree_parent; None at the tree root.
    tree_parent_edge: Vec<Option<usize>>,
    /// Exactly one independent anchor row, if pinned, per root.
    anchor: Vec<Option<usize>>,
    /// Every admitted row is retained for source reconstruction.
    rows: Vec<Vec<(usize, u64)>>,
    stats: WitnessStats,
}

impl ForestWitnessExtractor {
    pub fn new(vertices: usize, modulus: u64) -> Result<Self, LpGraphError> {
        if !valid_modulus(modulus) {
            return Err(LpGraphError::InvalidModulus);
        }
        Ok(Self {
            p: modulus,
            parent: (0..vertices).collect(),
            size: vec![1; vertices],
            tree_parent: (0..vertices).collect(),
            tree_parent_edge: vec![None; vertices],
            anchor: vec![None; vertices],
            rows: Vec::new(),
            stats: WitnessStats::default(),
        })
    }

    fn root(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }
        v
    }

    /// Follow the directed equation-forest path from start to its component
    /// root, without searching unrelated forest edges or allocating V slots.
    fn path_to_root(&self, start: usize, root: usize) -> (Vec<(usize, usize, usize)>, u64) {
        let mut path = Vec::new();
        let mut node = start;
        while node != root {
            let next = self.tree_parent[node];
            let edge = self.tree_parent_edge[node]
                .expect("nonroot forest vertex must have a parent equation");
            debug_assert_ne!(node, next);
            path.push((node, next, edge));
            node = next;
        }
        // There are no adjacency-list scans in the direct-parent arm.
        (path, 0)
    }

    /// Reverse parent pointers along a path in the *smaller* component.
    /// Then its endpoint can be attached to any vertex of the larger tree
    /// while preserving a single directed path to the common component root.
    /// Every vertex can lie on such a reversal path at most O(log V) times:
    /// whenever this happens, its component at least doubles in size.
    fn reroot_smaller_tree(&mut self, vertex: usize) -> u64 {
        let mut node = vertex;
        let mut previous = vertex;
        let mut previous_edge = None;
        let mut flips = 0;
        loop {
            let next = self.tree_parent[node];
            let next_edge = self.tree_parent_edge[node];
            self.tree_parent[node] = previous;
            self.tree_parent_edge[node] = previous_edge;
            if node == next {
                break;
            }
            previous = node;
            previous_edge = next_edge;
            node = next;
            flips += 1;
        }
        flips
    }

    fn project(&self, row: &[(usize, u64)]) -> Result<Reduced, LpGraphError> {
        let p = self.p;
        let mut out = Reduced {
            roots: BTreeMap::new(),
            basis: BTreeMap::new(),
            scans: 0,
            steps: 0,
            inversions: 0,
        };
        for &(vertex, a) in row {
            let root = self.root(vertex);
            let (path, scans) = self.path_to_root(vertex, root);
            out.scans += scans;
            let mut coefficient = a;
            for (child, par, edge_idx) in path {
                let edge = &self.rows[edge_idx];
                let child_a = coeff_at(edge, child);
                let parent_b = coeff_at(edge, par);
                let inverse = inv_mod(child_a, p).ok_or(LpGraphError::NonInvertible {
                    value: child_a,
                })?;
                let factor = mul_mod(coefficient, inverse, p);
                add_term(&mut out.basis, edge_idx, neg_mod(factor, p), p);
                coefficient = neg_mod(mul_mod(factor, parent_b, p), p);
                out.steps += 1;
                out.inversions += 1;
            }
            add_term(&mut out.roots, root, coefficient, p);
        }
        Ok(out)
    }

    /// Admit one canonical row. Returns `Some(w)` iff it is dependent.
    /// The witness's coefficient of the newly admitted row is exactly 1.
    /// Invalid rows are rejected before changing basis, row or counter state.
    pub fn add_row(
        &mut self,
        terms: &[(usize, u64)],
    ) -> Result<Option<CancellationWitness>, LpGraphError> {
        let row_idx = self.rows.len();
        let p = self.p;
        if terms.len() > 2 {
            return Err(LpGraphError::RowTooWide {
                row: row_idx,
                entries: terms.len(),
            });
        }
        for (i, &(v, a)) in terms.iter().enumerate() {
            if v >= self.parent.len() {
                return Err(LpGraphError::ColumnOutOfRange { column: v });
            }
            if a == 0 || a >= p || (i == 1 && v == terms[0].0) {
                return Err(LpGraphError::NonCanonicalRow { row: row_idx });
            }
            if inv_mod(a, p).is_none() {
                return Err(LpGraphError::NonInvertible { value: a });
            }
        }
        let original_roots = if terms.len() == 2 {
            Some((self.root(terms[0].0), self.root(terms[1].0)))
        } else {
            None
        };
        let projection = self.project(terms)?;
        let mut source = projection.basis.clone();
        add_term(&mut source, row_idx, 1, p);
        let mut outstanding = Vec::new();
        let mut scans = projection.scans;
        let mut steps = projection.steps;
        let mut inversions = projection.inversions + terms.len() as u64;
        for (&root, &gamma) in &projection.roots {
            if let Some(anchor_idx) = self.anchor[root] {
                let anchor_projection = self.project(&self.rows[anchor_idx])?;
                scans += anchor_projection.scans;
                steps += anchor_projection.steps;
                inversions += anchor_projection.inversions;
                let alpha = anchor_projection.roots.get(&root).copied().unwrap_or(0);
                let inverse = inv_mod(alpha, p).ok_or(LpGraphError::NonInvertible {
                    value: alpha,
                })?;
                inversions += 1;
                let factor = neg_mod(mul_mod(gamma, inverse, p), p);
                add_term(&mut source, anchor_idx, factor, p);
                for (i, a) in anchor_projection.basis {
                    add_term(&mut source, i, mul_mod(factor, a, p), p);
                }
            } else {
                outstanding.push(root);
            }
        }
        // All arithmetic has succeeded: metadata and forest changes follow.
        self.stats.adjacency_scans += scans;
        self.stats.path_steps += steps;
        self.stats.inverse_calls += inversions;
        self.stats.rows += 1;
        self.rows.push(terms.to_vec());
        if outstanding.is_empty() {
            let witness = CancellationWitness {
                row_index: row_idx,
                sources: source.into_iter().collect(),
            };
            debug_assert_eq!(
                witness.sources.iter().find(|&&(i, _)| i == row_idx).map(|x| x.1),
                Some(1),
            );
            self.stats.dependent_rows += 1;
            self.stats.source_terms += witness.sources.len();
            return Ok(Some(witness));
        }
        self.stats.independent_rows += 1;
        match original_roots {
            Some((u, v)) if u != v => {
                // At least one side was unanchored, else this would be dependent.
                let (big, small) = if self.size[u] >= self.size[v] { (u, v) } else { (v, u) };
                debug_assert!(self.anchor[u].is_none() || self.anchor[v].is_none());
                // Reroot only the smaller equation tree at its joining
                // endpoint, then attach this endpoint to the larger tree.
                let (small_endpoint, big_endpoint) = if small == u {
                    (terms[0].0, terms[1].0)
                } else {
                    (terms[1].0, terms[0].0)
                };
                let reversals = self.reroot_smaller_tree(small_endpoint);
                self.stats.reroot_steps += reversals;
                self.tree_parent[small_endpoint] = big_endpoint;
                self.tree_parent_edge[small_endpoint] = Some(row_idx);
                self.parent[small] = big;
                self.size[big] += self.size[small];
                self.anchor[big] = self.anchor[u].or(self.anchor[v]);
                self.anchor[small] = None;
                self.stats.tree_edges += 1;
            }
            _ => {
                debug_assert_eq!(outstanding.len(), 1);
                let root = outstanding[0];
                debug_assert!(self.anchor[root].is_none());
                self.anchor[root] = Some(row_idx);
                self.stats.anchors += 1;
            }
        }
        Ok(None)
    }

    pub fn stats(&self) -> WitnessStats {
        self.stats.clone()
    }

    pub fn rank(&self) -> usize {
        self.stats.independent_rows
    }

    pub fn rows(&self) -> &[Vec<(usize, u64)>] {
        &self.rows
    }
}

/// Produce the complete left-kernel certificate basis for a canonical stream.
/// Every dependent row has one source coefficient exactly equal to 1.
pub fn collect_witnesses(index: &LpIndex) -> Result<WitnessBatch, LpGraphError> {
    let mut f = ForestWitnessExtractor::new(index.num_columns(), index.modulus)?;
    let mut witnesses = Vec::new();
    for row in &index.rows {
        if let Some(w) = f.add_row(row)? {
            witnesses.push(w);
        }
    }
    let stats = f.stats();
    debug_assert_eq!(stats.rows, stats.independent_rows + stats.dependent_rows);
    Ok(WitnessBatch {
        modulus: index.modulus,
        lp_columns: index.num_columns(),
        rank: f.rank(),
        witnesses,
        stats,
    })
}

/// Independently reconstruct an LP witness from original coefficients.
/// Useful as a cheap admission gate before collecting any small-only row.
pub fn witness_cancels_lp(index: &LpIndex, witness: &CancellationWitness) -> bool {
    if witness.row_index >= index.rows.len()
        || witness.sources.iter().filter(|&&(i, _)| i == witness.row_index).count() != 1
        || witness.sources.iter().find(|&&(i, _)| i == witness.row_index).map(|x| x.1) != Some(1)
    {
        return false;
    }
    let mut acc = BTreeMap::new();
    for &(row, weight) in &witness.sources {
        if row >= index.rows.len() || weight == 0 || weight >= index.modulus {
            return false;
        }
        for &(column, a) in &index.rows[row] {
            add_term(&mut acc, column, mul_mod(a, weight, index.modulus), index.modulus);
        }
    }
    acc.is_empty()
}

/// Materialize original small-only rows and RHS using the existing replay API.
///
/// This is an atomic, source-verified adapter: one invalid source ID, cap,
/// coefficient, or uncancelled large prime refuses the entire projection.
/// `index.row_ids` and `inputs` must have identical ordering and unique IDs.
/// This method does NOT establish that the underlying elliptic-curve source
/// relations are valid; callers must separately check those group equations.
pub fn materialize_complete_relations(
    index: &LpIndex,
    batch: &WitnessBatch,
    inputs: &[PartialRelation],
    max_small_weight: usize,
    max_sources: usize,
) -> Option<Vec<CompleteRelation>> {
    if index.modulus != batch.modulus
        || index.rows.len() != inputs.len()
        || batch.stats.rows != inputs.len()
        || batch.rank + batch.witnesses.len() != inputs.len()
        || index.row_ids.iter().zip(inputs).any(|(id, r)| *id != r.id)
        || index.row_ids.iter().collect::<BTreeSet<_>>().len() != inputs.len()
    {
        return None;
    }
    let mut complete = Vec::with_capacity(batch.witnesses.len());
    for witness in &batch.witnesses {
        if witness.sources.len() > max_sources || !witness_cancels_lp(index, witness) {
            return None;
        }
        let proposed = CompleteRelation {
            small: Vec::new(),
            rhs: 0,
            sources: witness
                .sources
                .iter()
                .map(|&(i, a)| (index.row_ids[i], a))
                .collect(),
        };
        // The existing replay kernel recomputes from canonicalized partial
        // rows, including independently checking complete LP cancellation.
        let replayed = replay_complete_relation(&proposed, inputs, index.modulus)?;
        if replayed.small.len() > max_small_weight
            || replayed.sources.len() > max_sources
            || replayed.sources != proposed.sources
        {
            return None;
        }
        complete.push(replayed);
    }
    Some(complete)
}
