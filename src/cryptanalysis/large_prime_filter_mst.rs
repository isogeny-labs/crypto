//! MST-based large-prime elimination with original-input replay witnesses.
//! A tree of coefficient-normalized row differences eliminates one column.
//! Prim's algorithm minimizes the sum of actual sparse edge-row weights.
use super::*;

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

pub fn filter_large_primes_mst(
    relations: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: &MstLimits,
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
    let output = eliminate(relations, options, limits, &mut report, &mut stats)?;
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
    let mut rows: Vec<_> = relations
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
    loop {
        let persistent = rows.iter().try_fold(0usize, |s, r| s.checked_add(atoms(r)));
        if persistent.is_none_or(|v| v > limits.max_active_coefficients)
            || rows.iter().any(|r| !fits(r, options))
        {
            stats.budget_refusals += 1;
            return Ok(None);
        }
        stats.largest_active_coefficients =
            stats.largest_active_coefficients.max(persistent.unwrap());
        let mut columns = BTreeMap::<u64, Vec<usize>>::new();
        for (i, r) in rows.iter().enumerate() {
            for &(j, _) in &r.large {
                columns.entry(j).or_default().push(i);
            }
        }
        let Some((&j, members)) = columns.iter().min_by_key(|(j, v)| (v.len(), **j)) else {
            break;
        };
        let k = members.len();
        stats.largest_column_degree = stats.largest_column_degree.max(k);
        if k > limits.max_column_degree {
            stats.budget_refusals += 1;
            return Ok(None);
        }
        let mut normalized = Vec::with_capacity(k);
        for &i in members {
            let a = rows[i].coefficient(j);
            let inverse = inverse_mod(a, p).ok_or(LargePrimeFilterError::NonInvertiblePivot {
                large_prime: j,
                coefficient: a,
            })?;
            normalized.push(scale_row(&rows[i], inverse, p));
            stats.normalized_rows += 1;
        }
        let Some(edges) = tree_rows(&normalized, options, limits, stats) else {
            report.rejected_fill += 1;
            return Ok(None);
        };
        debug_assert!(edges.iter().all(|r| r.coefficient(j) == 0));
        let chosen: BTreeSet<_> = members.iter().copied().collect();
        let mut next: Vec<_> = rows
            .into_iter()
            .enumerate()
            .filter_map(|(i, r)| (!chosen.contains(&i)).then_some(r))
            .collect();
        report.pivots += 1;
        report.merges += edges.len();
        next.extend(edges);
        rows = next;
    }
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
