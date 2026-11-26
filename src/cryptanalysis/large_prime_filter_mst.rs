//! MST-based large-prime elimination with original-input replay witnesses.
//! A tree of coefficient-normalized row differences eliminates one column.
//! Prim's algorithm minimizes the sum of actual sparse edge-row weights.
//!
//! [`MstStrategy::OverlapSensitive`] computes the same tree without evaluating
//! every pair: for two normalized rows whose supports are disjoint apart from
//! the eliminated column, the difference's key is additive in per-row counts,
//! so only overlapping pairs need an explicit sparse difference.
use super::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// How [`filter_large_primes_mst_with`] finds each column's spanning tree.
/// All strategies return identical relations, witnesses and legacy stats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MstStrategy {
    /// Evaluate all `k(k-1)/2` normalized row differences (original Prim).
    #[default]
    Exhaustive,
    /// Implicit keys for support-disjoint pairs, explicit sparse differences
    /// for overlapping pairs only. A column whose pairwise cap condition
    /// fails (some disjoint pair could be infeasible) uses `Exhaustive`.
    OverlapSensitive,
    /// Exact key and feasibility of every pair from shared-key counts
    /// (shared keys, and shared keys with equal normalized coefficients),
    /// with no sparse difference formed except the `k-1` selected rows. No
    /// cap condition; a disconnected feasible graph uses `Exhaustive`.
    SharedKey,
}

#[derive(Clone, Copy, Debug)]
pub struct MstLimits {
    pub max_rows: usize,
    pub max_column_degree: usize,
    pub max_edge_evaluations: usize,
    /// Persistent small/large/source terms, not a peak-allocation counter.
    pub max_active_coefficients: usize,
}
impl Default for MstLimits {
    fn default() -> Self {
        Self {
            max_rows: 4096,
            max_column_degree: 64,
            max_edge_evaluations: 1_000_000,
            max_active_coefficients: 1_000_000,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MstStats {
    pub trees: usize,
    pub edge_evaluations: usize,
    pub infeasible_edge_candidates: usize,
    pub normalized_rows: usize,
    pub selected_edges: usize,
    pub largest_column_degree: usize,
    pub largest_active_coefficients: usize,
    pub input_nonzeros: usize,
    pub output_nonzeros: usize,
    pub budget_refusals: usize,
    /// Overlap-sensitive strategy only: sparse differences actually formed
    /// for overlapping pairs (selected implicit edges are counted apart).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub explicit_edge_evaluations: usize,
    /// Overlap-sensitive strategy only: selected edges whose key was known
    /// implicitly and whose row was materialized after selection.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub implicit_selected_edges: usize,
    /// Overlap-sensitive strategy only: overlapping row pairs found.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub overlapping_pairs: usize,
    /// Overlap-sensitive strategy only: trees that failed the cap
    /// condition and ran the exhaustive Prim instead.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub exhaustive_fallback_trees: usize,
    /// Overlap-sensitive strategy only: trees whose implicit key prediction
    /// failed (must stay zero; the tree is then recomputed exhaustively).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub implicit_key_mismatches: usize,
    /// Overlap-sensitive strategy only: sparse differences formed by the
    /// exhaustive Prim in fallback trees (included in `edge_evaluations`).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub fallback_edge_evaluations: usize,
    /// Shared-key strategy only: per-pair counter updates made while
    /// building the shared-key tables (one per row pair per shared key).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub shared_key_pair_updates: usize,
}

fn is_zero(v: &usize) -> bool {
    *v == 0
}

#[derive(Clone, Debug)]
pub struct MstAttempt {
    /// None means atomic refusal of the entire projection, never a partial
    /// result to pass to a solver. The caller retains/charges its old solve.
    pub relations: Option<Vec<CompleteRelation>>,
    pub report: LargePrimeFilterReport,
    pub stats: MstStats,
}

fn atoms(r: &WorkRow) -> usize {
    r.small
        .len()
        .saturating_add(r.large.len())
        .saturating_add(r.sources.len())
}
fn fits(r: &WorkRow, o: &LargePrimeFilterOptions) -> bool {
    r.small.len() <= o.max_small_weight && r.sources.len() <= o.max_sources
}
fn scale_row(r: &WorkRow, c: u64, p: u64) -> WorkRow {
    WorkRow {
        small: r
            .small
            .iter()
            .map(|&(i, a)| (i, mul_mod(c, a, p)))
            .collect(),
        large: r
            .large
            .iter()
            .map(|&(i, a)| (i, mul_mod(c, a, p)))
            .collect(),
        rhs: mul_mod(c, r.rhs, p),
        sources: r
            .sources
            .iter()
            .map(|&(i, a)| (i, mul_mod(c, a, p)))
            .collect(),
    }
}

struct Edge {
    from: usize,
    to: usize,
    row: WorkRow,
}
impl Edge {
    fn key(&self) -> (usize, usize, usize, usize) {
        (
            self.row.small.len() + self.row.large.len(),
            self.row.sources.len(),
            self.from,
            self.to,
        )
    }
}

/// Complete graph edges are exact normalized differences. Infeasible edges
/// are absent. A spanning tree of the feasible graph still preserves all
/// cancellation constraints; disconnected feasible graphs refuse atomically.
fn tree_rows(
    normalized: &[WorkRow],
    column: u64,
    strategy: MstStrategy,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    stats: &mut MstStats,
) -> Option<Vec<WorkRow>> {
    let k = normalized.len();
    if k == 0 {
        return Some(Vec::new());
    }
    let needed = k.checked_mul(k - 1)?.checked_div(2)?;
    if k > limits.max_column_degree
        || stats
            .edge_evaluations
            .checked_add(needed)
            .is_none_or(|v| v > limits.max_edge_evaluations)
    {
        stats.budget_refusals += 1;
        return None;
    }
    if strategy != MstStrategy::Exhaustive && k > 1 {
        let fast = if strategy == MstStrategy::SharedKey {
            shared_key_tree_rows(normalized, options, limits, stats)
        } else {
            overlap_tree_rows(normalized, column, options, limits, stats)
        };
        match fast {
            Ok(rows) => {
                stats.edge_evaluations += needed;
                stats.trees += 1;
                stats.selected_edges += rows.len();
                return Some(rows);
            }
            Err(defect) => {
                stats.exhaustive_fallback_trees += 1;
                stats.implicit_key_mismatches += usize::from(defect);
                let before = stats.edge_evaluations;
                let rows = exhaustive_tree_rows(normalized, options, limits, stats);
                stats.fallback_edge_evaluations += stats.edge_evaluations - before;
                return rows;
            }
        }
    }
    exhaustive_tree_rows(normalized, options, limits, stats)
}

fn exhaustive_tree_rows(
    normalized: &[WorkRow],
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    stats: &mut MstStats,
) -> Option<Vec<WorkRow>> {
    let k = normalized.len();
    let mut visited = vec![false; k];
    let mut best: Vec<Option<Edge>> = (0..k).map(|_| None).collect();
    let mut result = Vec::with_capacity(k - 1);
    for step in 0..k {
        let v = if step == 0 {
            0
        } else {
            let choice = (0..k)
                .filter(|&i| !visited[i])
                .filter_map(|i| best[i].as_ref().map(|e| (e.key(), i)))
                .min()
                .map(|(_, i)| i);
            let Some(i) = choice else {
                stats.budget_refusals += 1;
                return None;
            };
            result.push(best[i].take().expect("selected present edge").row);
            i
        };
        visited[v] = true;
        for u in 0..k {
            if visited[u] {
                continue;
            }
            stats.edge_evaluations += 1;
            let row = normalized[v].axpy(options.modulus - 1, &normalized[u], options.modulus);
            if !fits(&row, options) || atoms(&row) > limits.max_active_coefficients {
                stats.infeasible_edge_candidates += 1;
                continue;
            }
            let e = Edge {
                from: v,
                to: u,
                row,
            };
            if best[u].as_ref().is_none_or(|old| e.key() < old.key()) {
                best[u] = Some(e);
            }
        }
    }
    stats.trees += usize::from(k > 1);
    stats.selected_edges += result.len();
    Some(result)
}

type EdgeKey = (usize, usize, usize, usize);
type VertexKey = (usize, usize, usize);

fn top_two_sum(values: impl Iterator<Item = usize>) -> usize {
    let (mut a, mut b) = (0usize, 0usize);
    for v in values {
        if v > a {
            b = a;
            a = v;
        } else if v > b {
            b = v;
        }
    }
    a.saturating_add(b)
}

/// Exact Prim on the same complete feasible graph as `tree_rows`, without
/// forming the sparse difference of a support-disjoint pair.
///
/// Let `r_v` be normalized so the eliminated column has coefficient one, with
/// `w_v` small plus other-large terms and `q_v` source terms. If `r_v`, `r_u`
/// share no small column, no other large column and no source id, `r_v - r_u`
/// has no cancellation and its edge key is `K(v,u) = (w_v+w_u, q_v+q_u, v, u)`.
/// Two facts make the search exact:
///
/// 1. Subadditivity: for any pair the difference's support lies in the union,
///    so its key is at most `K(v,u)` (same `(v,u)` tail). Under the cap
///    condition (the two largest rows fit together in every cap) every pair,
///    disjoint or not, is feasible, so the feasible graph is complete.
///    A shared key merges two terms into one, so an overlapping pair's key
///    is strictly below `K(v,u)` in its first two components.
/// 2. Let `m` be the visited vertex minimal in `(w, q, index)`. For unvisited
///    `u` disjoint from `m`, `K(m,u)` beats `K(v,u)` for every other visited
///    disjoint `v`; for `u` overlapping `m`, the explicit edge `(m,u)` is
///    strictly below `K(m,u) <= K(v,u)`, so no implicit edge is `u`'s best.
/// 3. Over unvisited `u`, `K(m,u)` is minimized by the first `u` in
///    `(w, q, index)` order; if that `u` overlaps `m`, the explicit `(m,u)`
///    edge is strictly smaller, so comparing against `K(m,u)` is harmless.
///
/// Hence each step selects the smaller of the best explicit (overlapping)
/// edge and `K(m,u)` for the first unvisited `u` in `(w, q, index)` order.
/// Ties, edge order and the selected rows are the exhaustive Prim's.
/// Returns `Err` (the caller runs the exhaustive Prim) when the cap
/// condition fails, or `Err(true)` if a materialized implicit row disagrees
/// with its predicted key, which would be a defect in this proof.
fn overlap_tree_rows(
    normalized: &[WorkRow],
    column: u64,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    stats: &mut MstStats,
) -> Result<Vec<WorkRow>, bool> {
    let k = normalized.len();
    let p = options.modulus;
    if normalized.iter().any(|r| r.coefficient(column) != 1) {
        return Err(false);
    }
    let small: Vec<usize> = normalized.iter().map(|r| r.small.len()).collect();
    let src: Vec<usize> = normalized.iter().map(|r| r.sources.len()).collect();
    let weight: Vec<usize> = normalized
        .iter()
        .map(|r| r.small.len() + r.large.len() - 1)
        .collect();
    if top_two_sum(small.iter().copied()) > options.max_small_weight
        || top_two_sum(src.iter().copied()) > options.max_sources
        || top_two_sum((0..k).map(|v| weight[v] + src[v])) > limits.max_active_coefficients
    {
        return Err(false);
    }
    // Overlap graph from an inverted index over the three key spaces.
    let mut index = BTreeMap::<(u8, u64), Vec<usize>>::new();
    for (v, r) in normalized.iter().enumerate() {
        for &(i, _) in &r.small {
            index.entry((0, u64::from(i))).or_default().push(v);
        }
        for &(i, _) in r.large.iter().filter(|x| x.0 != column) {
            index.entry((1, i)).or_default().push(v);
        }
        for &(i, _) in &r.sources {
            index.entry((2, i)).or_default().push(v);
        }
    }
    let mut ov: Vec<Vec<usize>> = vec![Vec::new(); k];
    for list in index.values() {
        for (a, &x) in list.iter().enumerate() {
            for &y in &list[a + 1..] {
                ov[x].push(y);
                ov[y].push(x);
            }
        }
    }
    for list in &mut ov {
        list.sort_unstable();
        list.dedup();
    }
    let overlapping_pairs = ov.iter().map(Vec::len).sum::<usize>() / 2;
    let ord = |v: usize| -> VertexKey { (weight[v], src[v], v) };

    let mut visited = vec![false; k];
    let mut unvisited: BTreeSet<VertexKey> = (0..k).map(ord).collect();
    let mut explicit: Vec<Option<(EdgeKey, WorkRow)>> = (0..k).map(|_| None).collect();
    let mut heap = BinaryHeap::<Reverse<(EdgeKey, usize)>>::new();
    let mut m = 0usize;
    let (mut explicit_evaluations, mut implicit_selected) = (0usize, 0usize);
    let mut result = Vec::with_capacity(k - 1);
    let mut next = 0usize;
    for step in 0..k {
        if step > 0 {
            while let Some(&Reverse((key, u))) = heap.peek() {
                if visited[u] || explicit[u].as_ref().map(|e| e.0) != Some(key) {
                    heap.pop();
                } else {
                    break;
                }
            }
            let from_explicit = heap.peek().map(|&Reverse(x)| x);
            // No overlap test needed: if `u` overlaps `m`, the live explicit
            // edge `(m,u)` already has a strictly smaller key (fact 2).
            let from_implicit = unvisited
                .iter()
                .next()
                .map(|&(wu, su, u)| ((weight[m] + wu, src[m] + su, m, u), u));
            let ((key, u), is_explicit) = match (from_explicit, from_implicit) {
                (Some(e), Some(i)) if e < i => (e, true),
                (Some(e), None) => (e, true),
                (_, Some(i)) => (i, false),
                // Unreachable: under the cap condition the graph is complete.
                (None, None) => {
                    stats.explicit_edge_evaluations += explicit_evaluations;
                    return Err(true);
                }
            };
            let row = if is_explicit {
                explicit[u].take().expect("live heap entry").1
            } else {
                let row = normalized[key.2].axpy(p - 1, &normalized[u], p);
                if (row.small.len() + row.large.len(), row.sources.len()) != (key.0, key.1) {
                    stats.explicit_edge_evaluations += explicit_evaluations;
                    return Err(true);
                }
                implicit_selected += 1;
                row
            };
            result.push(row);
            next = u;
        }
        let w = next;
        visited[w] = true;
        unvisited.remove(&ord(w));
        if ord(w) < ord(m) {
            m = w;
        }
        for &u in &ov[w] {
            if visited[u] {
                continue;
            }
            explicit_evaluations += 1;
            let row = normalized[w].axpy(p - 1, &normalized[u], p);
            debug_assert!(fits(&row, options) && atoms(&row) <= limits.max_active_coefficients);
            let key = (row.small.len() + row.large.len(), row.sources.len(), w, u);
            if explicit[u].as_ref().is_none_or(|old| key < old.0) {
                heap.push(Reverse((key, u)));
                explicit[u] = Some((key, row));
            }
        }
    }
    stats.explicit_edge_evaluations += explicit_evaluations;
    stats.implicit_selected_edges += implicit_selected;
    stats.overlapping_pairs += overlapping_pairs;
    Ok(result)
}

/// Exact Prim from shared-key counts. For normalized rows `r_v`, `r_u` and one
/// key space (small columns, large columns, source ids) with supports `A`,
/// `B`, the difference `r_v - r_u` has `|A| + |B| - c - e` terms, where `c` is
/// the number of shared keys and `e` the number of shared keys whose
/// coefficients are equal (they cancel; unequal ones merge into one term).
/// The eliminated column is shared with coefficient one, so it cancels like
/// any other equal key. This gives every pair's exact key
/// `(small+large, sources, v, u)` and exact feasibility (small, sources and
/// atom caps) without forming the difference, so the Prim below sees the
/// same edges, keys and ties as the exhaustive one. Only the `k-1` selected
/// rows are formed; each is checked against its predicted counts.
/// `Err(false)`: disconnected feasible graph (exhaustive Prim refuses with its
/// own stats); `Err(true)`: a prediction mismatch, a defect.
fn shared_key_tree_rows(
    normalized: &[WorkRow],
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    stats: &mut MstStats,
) -> Result<Vec<WorkRow>, bool> {
    let k = normalized.len();
    let p = options.modulus;
    // red[space][v*k+u] = shared + equal-coefficient shared keys, v < u.
    let mut red = [vec![0u32; k * k], vec![0u32; k * k], vec![0u32; k * k]];
    let mut updates = 0usize;
    // Flat (key, row, coefficient) incidences per key space, sorted so that
    // rows sharing a key are adjacent and appear in increasing row order.
    let mut incidences: Vec<(u64, usize, u64)> = Vec::new();
    for (space, table) in red.iter_mut().enumerate() {
        incidences.clear();
        for (v, r) in normalized.iter().enumerate() {
            match space {
                0 => incidences.extend(r.small.iter().map(|&(i, a)| (u64::from(i), v, a))),
                1 => incidences.extend(r.large.iter().map(|&(i, a)| (i, v, a))),
                _ => incidences.extend(r.sources.iter().map(|&(i, a)| (i, v, a))),
            }
        }
        incidences.sort_unstable();
        for group in incidences.chunk_by(|x, y| x.0 == y.0) {
            for (x, &(_, v, a)) in group.iter().enumerate() {
                for &(_, u, b) in &group[x + 1..] {
                    // Each row holds a key once, so v < u within a group.
                    table[v * k + u] += 1 + u32::from(a == b);
                    updates += 1;
                }
            }
        }
    }
    let len = |r: &WorkRow| [r.small.len(), r.large.len(), r.sources.len()];
    let lens: Vec<[usize; 3]> = normalized.iter().map(len).collect();
    // Exact (small, large, sources) term counts of r_v - r_u.
    let counts = |v: usize, u: usize| -> [usize; 3] {
        let (a, b) = (v.min(u), v.max(u));
        std::array::from_fn(|s| lens[v][s] + lens[u][s] - red[s][a * k + b] as usize)
    };
    let feasible = |c: &[usize; 3]| {
        c[0] <= options.max_small_weight
            && c[2] <= options.max_sources
            && c[0] + c[1] + c[2] <= limits.max_active_coefficients
    };
    let mut infeasible = 0usize;
    for v in 0..k {
        for u in v + 1..k {
            infeasible += usize::from(!feasible(&counts(v, u)));
        }
    }
    let mut visited = vec![false; k];
    let mut best: Vec<Option<EdgeKey>> = vec![None; k];
    let mut result = Vec::with_capacity(k - 1);
    let mut v = 0usize;
    for step in 0..k {
        if step > 0 {
            let Some((key, u)) = (0..k)
                .filter(|&i| !visited[i])
                .filter_map(|i| best[i].map(|e| (e, i)))
                .min()
            else {
                stats.shared_key_pair_updates += updates;
                return Err(false);
            };
            let row = normalized[key.2].axpy(p - 1, &normalized[u], p);
            let c = counts(key.2, u);
            if [row.small.len(), row.large.len(), row.sources.len()] != c
                || (c[0] + c[1], c[2]) != (key.0, key.1)
            {
                stats.shared_key_pair_updates += updates;
                return Err(true);
            }
            result.push(row);
            v = u;
        }
        visited[v] = true;
        for u in 0..k {
            if visited[u] {
                continue;
            }
            let c = counts(v, u);
            if !feasible(&c) {
                continue;
            }
            let key = (c[0] + c[1], c[2], v, u);
            if best[u].is_none_or(|old| key < old) {
                best[u] = Some(key);
            }
        }
    }
    stats.shared_key_pair_updates += updates;
    stats.infeasible_edge_candidates += infeasible;
    stats.implicit_selected_edges += k - 1;
    Ok(result)
}

pub fn filter_large_primes_mst(
    relations: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
) -> Result<MstAttempt, LargePrimeFilterError> {
    filter_large_primes_mst_with(relations, options, limits, MstStrategy::Exhaustive)
}

/// [`filter_large_primes_mst`] with an explicit tree strategy. The budget
/// check still charges `k(k-1)/2` logical edges per column, so refusals,
/// relations, witnesses and the legacy stats agree across strategies.
pub fn filter_large_primes_mst_with(
    relations: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    strategy: MstStrategy,
) -> Result<MstAttempt, LargePrimeFilterError> {
    let p = options.modulus;
    if !(3..1u64 << 63).contains(&p) || p.is_multiple_of(2) {
        return Err(LargePrimeFilterError::InvalidModulus);
    }
    let mut report = LargePrimeFilterReport {
        input_relations: relations.len(),
        ..Default::default()
    };
    let mut stats = MstStats::default();
    let output = eliminate(
        relations,
        options,
        limits,
        strategy,
        &mut report,
        &mut stats,
    )?;
    Ok(MstAttempt {
        relations: output,
        report,
        stats,
    })
}

fn eliminate(
    mut relations: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
    strategy: MstStrategy,
    report: &mut LargePrimeFilterReport,
    stats: &mut MstStats,
) -> Result<Option<Vec<CompleteRelation>>, LargePrimeFilterError> {
    let p = options.modulus;
    let input_atoms = relations.iter().try_fold(0usize, |s, r| {
        s.checked_add(r.small.len())?
            .checked_add(r.large.len())?
            .checked_add(1)
    });
    if relations.len() > limits.max_rows
        || input_atoms.is_none_or(|n| n > limits.max_active_coefficients)
    {
        stats.budget_refusals += 1;
        return Ok(None);
    }
    let mut ids = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for r in &mut relations {
        if !ids.insert(r.id) {
            return Err(LargePrimeFilterError::DuplicateId(r.id));
        }
        r.normalise(p);
    }
    relations.retain(|r| {
        let fresh = seen.insert(duplicate_key(r));
        report.duplicate_relations += usize::from(!fresh);
        fresh
    });
    stats.input_nonzeros = relations
        .iter()
        .map(|r| r.small.len() + r.large.len())
        .sum();
    let mut incident = BTreeMap::<u64, Vec<usize>>::new();
    for (i, r) in relations.iter().enumerate() {
        for &(j, _) in &r.large {
            incident.entry(j).or_default().push(i);
        }
    }
    report.input_large_vertices = incident.len();
    let mut active = vec![true; relations.len()];
    let mut degree: BTreeMap<_, _> = incident.iter().map(|(&j, v)| (j, v.len())).collect();
    let mut queue: VecDeque<_> = degree
        .iter()
        .filter_map(|(&j, &d)| (d == 1).then_some(j))
        .collect();
    while let Some(j) = queue.pop_front() {
        if degree[&j] != 1 {
            continue;
        }
        let Some(i) = incident[&j].iter().copied().find(|&i| active[i]) else {
            continue;
        };
        active[i] = false;
        report.peeled_relations += 1;
        for &(other, _) in &relations[i].large {
            let d = degree.get_mut(&other).expect("recorded incident column");
            *d -= 1;
            if *d == 1 {
                queue.push_back(other);
            }
        }
    }
    let rows: Vec<_> = relations
        .into_iter()
        .enumerate()
        .filter(|(i, r)| r.large.is_empty() || active[*i])
        .map(|(_, r)| WorkRow::from_partial(r, p))
        .collect();
    report.core_relations = rows.iter().filter(|r| !r.large.is_empty()).count();
    report.core_large_vertices = rows
        .iter()
        .flat_map(|r| r.large.iter().map(|x| x.0))
        .collect::<BTreeSet<_>>()
        .len();
    // Incremental state. `slab` keeps every row under an append-only id, so
    // the surviving rows in id order are exactly the row vector the original
    // loop rebuilt after each pivot (survivors in order, then new edge rows).
    // Each column's member set is ordered by id, which reproduces the
    // original member order and so the Prim vertex numbering and ties;
    // `order` holds `(degree, column)` so its first element is the original
    // `min_by_key(|(j, v)| (v.len(), j))`. The atom total and the number of
    // rows over the weight/source caps replace the per-pivot rescans.
    let mut slab: Vec<Option<WorkRow>> = rows.into_iter().map(Some).collect();
    let mut columns = BTreeMap::<u64, BTreeSet<usize>>::new();
    let mut persistent: u128 = 0;
    let mut unfit = 0usize;
    for (i, r) in slab.iter().enumerate() {
        let r = r.as_ref().expect("fresh slab");
        persistent += atoms(r) as u128;
        unfit += usize::from(!fits(r, options));
        for &(j, _) in &r.large {
            columns.entry(j).or_default().insert(i);
        }
    }
    let mut order: BTreeSet<(usize, u64)> = columns.iter().map(|(&j, m)| (m.len(), j)).collect();
    loop {
        if persistent > limits.max_active_coefficients as u128 || unfit > 0 {
            stats.budget_refusals += 1;
            return Ok(None);
        }
        stats.largest_active_coefficients =
            stats.largest_active_coefficients.max(persistent as usize);
        let Some(&(k, j)) = order.first() else {
            break;
        };
        stats.largest_column_degree = stats.largest_column_degree.max(k);
        if k > limits.max_column_degree {
            stats.budget_refusals += 1;
            return Ok(None);
        }
        let members: Vec<usize> = columns[&j].iter().copied().collect();
        let mut normalized = Vec::with_capacity(k);
        for &i in &members {
            let row = slab[i].as_ref().expect("live member");
            let a = row.coefficient(j);
            let inverse = inverse_mod(a, p).ok_or(LargePrimeFilterError::NonInvertiblePivot {
                large_prime: j,
                coefficient: a,
            })?;
            normalized.push(scale_row(row, inverse, p));
            stats.normalized_rows += 1;
        }
        let Some(edges) = tree_rows(&normalized, j, strategy, options, limits, stats) else {
            report.rejected_fill += 1;
            return Ok(None);
        };
        debug_assert!(edges.iter().all(|r| r.coefficient(j) == 0));
        let mut move_column =
            |columns: &mut BTreeMap<u64, BTreeSet<usize>>, jj: u64, id: usize, insert: bool| {
                let set = columns.entry(jj).or_default();
                order.remove(&(set.len(), jj));
                if insert {
                    set.insert(id);
                } else {
                    set.remove(&id);
                }
                if set.is_empty() {
                    columns.remove(&jj);
                } else {
                    order.insert((set.len(), jj));
                }
            };
        for &i in &members {
            let row = slab[i].take().expect("live member");
            persistent -= atoms(&row) as u128;
            unfit -= usize::from(!fits(&row, options));
            for &(jj, _) in &row.large {
                move_column(&mut columns, jj, i, false);
            }
        }
        report.pivots += 1;
        report.merges += edges.len();
        for row in edges {
            let id = slab.len();
            persistent += atoms(&row) as u128;
            unfit += usize::from(!fits(&row, options));
            for &(jj, _) in &row.large {
                move_column(&mut columns, jj, id, true);
            }
            slab.push(Some(row));
        }
    }
    let rows: Vec<WorkRow> = slab.into_iter().flatten().collect();
    stats.output_nonzeros = rows.iter().map(|r| r.small.len()).sum();
    report.complete_relations = rows.len();
    report.cycles_closed = rows.iter().filter(|r| r.sources.len() > 1).count();
    Ok(Some(rows.into_iter().map(WorkRow::complete).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: u64, small: &[(u32, u64)], large: &[(u64, u64)], rhs: u64) -> PartialRelation {
        PartialRelation {
            id,
            small: small.to_vec(),
            large: large.to_vec(),
            rhs,
        }
    }
    fn options(p: u64) -> LargePrimeFilterOptions {
        LargePrimeFilterOptions {
            modulus: p,
            ..Default::default()
        }
    }
    fn next(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    // Dense reference: eliminate all designated large columns first, then
    // canonicalize every surviving small/RHS constraint. Equality checks the
    // complete projection, including inconsistent or rank-deficient inputs.
    fn reduce(a: &mut [Vec<u64>], columns: usize, p: u64) -> usize {
        let mut rank = 0;
        for j in 0..columns {
            let Some(i) = (rank..a.len()).find(|&i| a[i][j] != 0) else {
                continue;
            };
            a.swap(i, rank);
            let inv = inverse_mod(a[rank][j], p).unwrap();
            for v in &mut a[rank] {
                *v = mul_mod(*v, inv, p);
            }
            let pivot = a[rank].clone();
            for (i, r) in a.iter_mut().enumerate() {
                if i == rank {
                    continue;
                }
                let scale = neg_mod(r[j], p);
                for (v, &b) in r.iter_mut().zip(&pivot) {
                    *v = add_mod(*v, mul_mod(scale, b, p), p);
                }
            }
            rank += 1;
            if rank == a.len() {
                break;
            }
        }
        rank
    }
    fn canonical(mut a: Vec<Vec<u64>>, width: usize, p: u64) -> Vec<Vec<u64>> {
        reduce(&mut a, width, p);
        a.retain(|r| r.iter().any(|&v| v != 0));
        a
    }
    fn dense_projection(
        inputs: &[PartialRelation],
        small: usize,
        large: usize,
        p: u64,
    ) -> Vec<Vec<u64>> {
        let mut a: Vec<_> = inputs
            .iter()
            .map(|r| {
                let mut v = vec![0; large + small + 1];
                for &(i, c) in &r.large {
                    v[i as usize] = add_mod(v[i as usize], c % p, p);
                }
                for &(i, c) in &r.small {
                    v[large + i as usize] = add_mod(v[large + i as usize], c % p, p);
                }
                v[large + small] = r.rhs % p;
                v
            })
            .collect();
        reduce(&mut a, large, p);
        let constraints = a
            .into_iter()
            .filter(|r| r[..large].iter().all(|&v| v == 0))
            .map(|r| r[large..].to_vec())
            .collect();
        canonical(constraints, small + 1, p)
    }
    fn complete_projection(rows: &[CompleteRelation], small: usize, p: u64) -> Vec<Vec<u64>> {
        let a = rows
            .iter()
            .map(|r| {
                let mut v = vec![0; small + 1];
                for &(i, c) in &r.small {
                    v[i as usize] = c;
                }
                v[small] = r.rhs;
                v
            })
            .collect();
        canonical(a, small + 1, p)
    }
    fn assert_replay(rows: &[CompleteRelation], inputs: &[PartialRelation], p: u64) {
        for r in rows {
            let replay = replay_complete_relation(r, inputs, p).unwrap();
            assert_eq!(replay.small, r.small);
            assert_eq!(replay.rhs, r.rhs);
        }
    }

    #[test]
    fn a_minimum_spanning_tree_is_strictly_lighter_than_every_star() {
        let inputs = vec![
            row(0, &[(0, 1), (1, 1)], &[(0, 1)], 3),
            row(1, &[(0, 1), (2, 1)], &[(0, 1)], 5),
            row(2, &[(2, 1), (3, 1)], &[(0, 1)], 7),
            row(3, &[(3, 1), (4, 1)], &[(0, 1)], 11),
        ];
        let attempt =
            filter_large_primes_mst(inputs.clone(), &options(101), &MstLimits::default()).unwrap();
        let out = attempt.relations.unwrap();
        assert_eq!(attempt.stats.output_nonzeros, 6);
        assert_eq!(attempt.stats.trees, 1);
        assert_eq!(attempt.stats.edge_evaluations, 6);
        assert_eq!(attempt.stats.selected_edges, 3);
        let w: Vec<_> = inputs
            .iter()
            .cloned()
            .map(|r| WorkRow::from_partial(r, 101))
            .collect();
        let best_star = (0..w.len())
            .map(|i| {
                (0..w.len())
                    .filter(|&j| i != j)
                    .map(|j| w[i].axpy(100, &w[j], 101).small.len())
                    .sum::<usize>()
            })
            .min()
            .unwrap();
        assert_eq!(best_star, 8);
        assert_replay(&out, &inputs, 101);
        assert_eq!(
            complete_projection(&out, 5, 101),
            dense_projection(&inputs, 5, 1, 101)
        );
    }

    #[test]
    fn full_projected_rowspaces_match_dense_elimination_over_odd_primes() {
        let mut state = 0x81bfec5a75299131u64;
        let mut cases = 0;
        for p in [7, 101, (1u64 << 61) - 1] {
            for small in [1, 3, 7] {
                for large in [0, 1, 3, 8] {
                    for count in [3, 7, 15, 23] {
                        for _ in 0..4 {
                            let inputs: Vec<_> = (0..count)
                                .map(|i| {
                                    let small = (0..small)
                                        .filter_map(|j| {
                                            let a = next(&mut state) % p;
                                            (next(&mut state).is_multiple_of(3) && a != 0)
                                                .then_some((j as u32, a))
                                        })
                                        .collect();
                                    let large = (0..large)
                                        .filter_map(|j| {
                                            let a = next(&mut state) % p;
                                            (next(&mut state).is_multiple_of(3) && a != 0)
                                                .then_some((j as u64, a))
                                        })
                                        .collect();
                                    PartialRelation {
                                        id: i as u64,
                                        small,
                                        large,
                                        rhs: next(&mut state) % p,
                                    }
                                })
                                .collect();
                            let attempt = filter_large_primes_mst(
                                inputs.clone(),
                                &options(p),
                                &MstLimits::default(),
                            )
                            .unwrap();
                            let out = attempt.relations.expect("uncapped small reference case");
                            assert_replay(&out, &inputs, p);
                            assert_eq!(
                                complete_projection(&out, small, p),
                                dense_projection(&inputs, small, large, p),
                                "p={p} small={small} large={large} rows={count}"
                            );
                            cases += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(cases, 576);
    }

    #[test]
    fn prim_weights_equal_exhaustive_four_vertex_spanning_trees() {
        let mut state = 345817u64;
        let pairs = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        for _ in 0..64 {
            let inputs: Vec<_> = (0..4)
                .map(|i| {
                    let small = (0..8)
                        .filter_map(|j| {
                            let a = next(&mut state) % 101;
                            (next(&mut state).is_multiple_of(2) && a != 0).then_some((j as u32, a))
                        })
                        .collect();
                    PartialRelation {
                        id: i,
                        small,
                        large: vec![(0, 1)],
                        rhs: 0,
                    }
                })
                .collect();
            let work: Vec<_> = inputs
                .iter()
                .cloned()
                .map(|r| WorkRow::from_partial(r, 101))
                .collect();
            let weights: Vec<_> = pairs
                .iter()
                .map(|&(a, b)| work[a].axpy(100, &work[b], 101).small.len())
                .collect();
            let mut optimum = usize::MAX;
            for a in 0..6 {
                for b in a + 1..6 {
                    for c in b + 1..6 {
                        let mut reached = [true, false, false, false];
                        for _ in 0..4 {
                            for e in [a, b, c] {
                                let (u, v) = pairs[e];
                                if reached[u] || reached[v] {
                                    reached[u] = true;
                                    reached[v] = true;
                                }
                            }
                        }
                        if reached.iter().all(|&v| v) {
                            optimum = optimum.min(weights[a] + weights[b] + weights[c]);
                        }
                    }
                }
            }
            let out = filter_large_primes_mst(inputs.clone(), &options(101), &MstLimits::default())
                .unwrap();
            assert_eq!(out.stats.output_nonzeros, optimum);
            assert_replay(out.relations.as_ref().unwrap(), &inputs, 101);
        }
    }

    #[test]
    fn refusals_are_atomic_and_never_return_the_partial_projection() {
        let inputs = vec![
            row(1, &[(0, 1)], &[(0, 2)], 4),
            row(2, &[(1, 1)], &[(0, 3)], 5),
        ];
        for kind in 0..5 {
            let mut l = MstLimits::default();
            let mut o = options(101);
            match kind {
                0 => l.max_rows = 1,
                1 => l.max_column_degree = 1,
                2 => l.max_edge_evaluations = 0,
                3 => l.max_active_coefficients = 1,
                _ => o.max_sources = 1,
            }
            let out = filter_large_primes_mst(inputs.clone(), &o, &l).unwrap();
            assert!(out.relations.is_none());
            assert!(out.stats.budget_refusals > 0);
        }
    }

    #[test]
    fn duplicates_units_and_damaged_witnesses_have_explicit_outcomes() {
        let inputs = vec![
            row(1, &[(0, 1), (0, 100)], &[(0, 2)], 4),
            row(2, &[(1, 1)], &[(0, 3)], 5),
        ];
        let mut duplicated = inputs.clone();
        duplicated.push(inputs[0].clone());
        assert_eq!(
            filter_large_primes_mst(duplicated, &options(101), &MstLimits::default()).unwrap_err(),
            LargePrimeFilterError::DuplicateId(1)
        );
        assert_eq!(
            filter_large_primes_mst(
                vec![row(1, &[], &[(0, 5)], 1), row(2, &[], &[(0, 5)], 2)],
                &options(15),
                &MstLimits::default()
            )
            .unwrap_err(),
            LargePrimeFilterError::NonInvertiblePivot {
                large_prime: 0,
                coefficient: 5
            }
        );
        let out = filter_large_primes_mst(inputs.clone(), &options(101), &MstLimits::default())
            .unwrap()
            .relations
            .unwrap();
        assert_replay(&out, &inputs, 101);
        let mut bad = out[0].clone();
        bad.rhs = add_mod(bad.rhs, 1, 101);
        assert_ne!(
            replay_complete_relation(&bad, &inputs, 101).unwrap().rhs,
            bad.rhs
        );
    }
}
