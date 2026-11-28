//! Native F4 -> finite staircase -> counted border normal forms -> existing
//! multiplication-matrix/eigenvalue/cubic recovery. Unsupported quotients are
//! handed back to the existing charged oracle, never treated as empty roots.
use super::*;
use crate::cryptanalysis::f4_fp as f4;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Extraction {
    Dense,
    ScalarKrylov,
    DelayedKrylov,
}

#[derive(Clone, PartialEq, Eq, Serialize)]
struct TraceFamily {
    p: u64,
    c: u64,
    modulus: Option<[u64; 3]>,
    a: E3,
    b: E3,
    generator: Pt3,
    target: Pt3,
    subgroup_order: u64,
}

impl TraceFamily {
    fn new(inst: &Instance3) -> Self {
        Self {
            p: inst.curve.field.p,
            c: inst.curve.field.c,
            modulus: inst.curve.field.modulus,
            a: inst.curve.a,
            b: inst.curve.b,
            generator: inst.curve.g,
            target: inst.q,
            subgroup_order: inst.curve.n,
        }
    }
}

/// A cold, bounded trace pool for one original DLP instance and S4 encoding.
/// Sampled residuals vary within this family; the original target is bound.
pub struct S4TraceState {
    family: TraceFamily,
    pool: f4::TracePool,
    family_rejections: u64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct S4TraceStats {
    pub family_rejections: u64,
    pub pool: f4::TraceStats,
}

impl S4TraceState {
    pub fn new(inst: &Instance3) -> Self {
        Self::with_pair_graph(inst, false)
    }

    /// Product/chain dependency certificates; the exhaustive state is retained.
    pub fn new_pair_graph(inst: &Instance3) -> Self {
        Self::with_pair_graph(inst, true)
    }

    fn with_pair_graph(inst: &Instance3, pair_graph: bool) -> Self {
        let family = TraceFamily::new(inst);
        let bytes = serde_json::to_vec(&family).expect("integer trace-family serialization");
        let label = format!("cubic-s4:{}", blake3::hash(&bytes).to_hex());
        Self {
            family,
            pool: if pair_graph {
                f4::TracePool::new_pair_graph(&label, 4, 1_000_000)
            } else {
                f4::TracePool::new(&label, 4, 1_000_000)
            }
            .expect("fixed bounded trace-pool parameters"),
            family_rejections: 0,
        }
    }

    pub fn family(&self) -> &str {
        self.pool.family()
    }

    pub fn stats(&self) -> S4TraceStats {
        S4TraceStats {
            family_rejections: self.family_rejections,
            pool: self.pool.stats(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct S4F4Stats {
    pub calls: u64,
    pub completed: u64,
    pub fallback_calls: u64,
    pub incomplete_bases: u64,
    pub unsupported_quotients: u64,
    pub basis_row_muls: u64,
    pub normal_form_muls: u64,
    pub quotient_dimension_total: u64,
    pub max_basis_columns: usize,
    pub counter_scope: &'static str,
    pub krylov: KrylovStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<S4TraceStats>,
}

fn packed_basis(basis: &[f4::Poly]) -> Option<Vec<Vec<([u8; 3], u64)>>> {
    basis
        .iter()
        .map(|poly| {
            if poly.is_empty() || poly[0].1 != 1 {
                return None;
            }
            poly.iter()
                .map(|(e, c)| {
                    if e.len() != 3 {
                        return None;
                    }
                    Some((
                        [
                            u8::try_from(e[0]).ok()?,
                            u8::try_from(e[1]).ok()?,
                            u8::try_from(e[2]).ok()?,
                        ],
                        *c,
                    ))
                })
                .collect()
        })
        .collect()
}

fn staircase(basis: &[Vec<([u8; 3], u64)>]) -> Option<Vec<[u8; 3]>> {
    if basis.iter().any(|f| f[0].0 == [0; 3]) {
        return Some(Vec::new());
    }
    let mut bounds = [u8::MAX; 3];
    for f in basis {
        let lm = f[0].0;
        for k in 0..3 {
            if lm[k] > 0 && (0..3).all(|j| j == k || lm[j] == 0) {
                bounds[k] = bounds[k].min(lm[k]);
            }
        }
    }
    if bounds.contains(&u8::MAX) || bounds.iter().map(|&x| x as usize).product::<usize>() > 4096 {
        return None;
    }
    let mut standard = Vec::new();
    for a in 0..bounds[0] {
        for b in 0..bounds[1] {
            for c in 0..bounds[2] {
                let m = [a, b, c];
                if !basis.iter().any(|f| (0..3).all(|k| f[0].0[k] <= m[k])) {
                    standard.push(m);
                }
            }
        }
    }
    if standard.len() > 64 {
        return None;
    }
    standard.sort_by(|a, b| grevlex_cmp(b, a));
    Some(standard)
}

fn normal_form(
    monomial: [u8; 3],
    basis: &[Vec<([u8; 3], u64)>],
    p: u64,
    muls: &mut u64,
) -> Option<HashMap<[u8; 3], u64>> {
    let mut pending = HashMap::from([(monomial, 1u64)]);
    let mut out = HashMap::new();
    while let Some((&m, &coefficient)) = pending.iter().max_by(|(a, _), (b, _)| grevlex_cmp(a, b)) {
        pending.remove(&m);
        if coefficient == 0 {
            continue;
        }
        if let Some(poly) = basis.iter().find(|f| (0..3).all(|k| f[0].0[k] <= m[k])) {
            let lead = poly[0].0;
            for &(tail, c) in &poly[1..] {
                let mut exponent = [0; 3];
                for k in 0..3 {
                    exponent[k] = (m[k] - lead[k]).checked_add(tail[k])?;
                }
                let value = sm(
                    pending.get(&exponent).copied().unwrap_or(0),
                    mm(coefficient, c, p),
                    p,
                );
                *muls += 1;
                if value == 0 {
                    pending.remove(&exponent);
                } else {
                    pending.insert(exponent, value);
                }
            }
        } else {
            out.insert(m, coefficient);
        }
    }
    Some(out)
}

/// A native coordinate solve. None means unsupported/incomplete, not no roots.
/// The F4 report's row counter is process-wide; use serial solves for costing.
/// Scalar polynomial housekeeping remains unpriced in the legacy cost model.
pub fn solve_s4_f4(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Option<Vec<[u64; 3]>> {
    solve_with(inst, pre, x_r, rng, stats, work, Extraction::Dense)
}

/// Same native basis and border construction, with checked sparse extraction.
pub fn solve_s4_f4_krylov(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Option<Vec<[u64; 3]>> {
    solve_with(inst, pre, x_r, rng, stats, work, Extraction::ScalarKrylov)
}

/// Checked sparse extraction with a guarded delayed-integer graph kernel.
pub fn solve_s4_f4_krylov_delayed(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Option<Vec<[u64; 3]>> {
    solve_with(inst, pre, x_r, rng, stats, work, Extraction::DelayedKrylov)
}

/// Complete coordinate recovery with certified trace reuse and delayed sparse
/// extraction. Learning, rejected replay and certification work are charged.
/// A changed original curve/field/target returns None, never empty roots.
pub fn solve_s4_f4_trace_delayed(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    trace: &mut S4TraceState,
) -> Option<Vec<[u64; 3]>> {
    solve_with_trace(
        inst,
        pre,
        x_r,
        rng,
        stats,
        work,
        Extraction::DelayedKrylov,
        Some(trace),
    )
}

fn solve_with(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    mode: Extraction,
) -> Option<Vec<[u64; 3]>> {
    solve_with_trace(inst, pre, x_r, rng, stats, work, mode, None)
}

#[allow(clippy::too_many_arguments)]
fn solve_with_trace(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    mode: Extraction,
    mut trace: Option<&mut S4TraceState>,
) -> Option<Vec<[u64; 3]>> {
    work.calls += 1;
    work.counter_scope="native F4 row counter plus counted restriction/border/root work; polynomial scalar housekeeping unpriced; serial costing required";
    stats.solves += 1;
    let mut muls = 0;
    let mut trial_rng = rng.clone();
    let result = if trace
        .as_ref()
        .is_some_and(|s| s.family != TraceFamily::new(inst))
    {
        trace.as_mut().unwrap().family_rejections += 1;
        None
    } else {
        solve_inner(
            inst,
            pre,
            x_r,
            &mut trial_rng,
            stats,
            work,
            &mut muls,
            mode,
            trace.as_deref_mut(),
        )
    };
    if let Some(trace) = trace {
        work.trace = Some(trace.stats());
    }
    stats.fp_muls += muls;
    if result.is_some() {
        work.completed += 1;
        *rng = trial_rng;
    } else {
        work.unsupported_quotients += 1;
    }
    result
}

#[allow(clippy::too_many_arguments)]
fn solve_inner(
    inst: &Instance3,
    pre: &SymmetrisedS4,
    x_r: &E3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    muls: &mut u64,
    mode: Extraction,
    trace: Option<&mut S4TraceState>,
) -> Option<Vec<[u64; 3]>> {
    let f = &inst.curve.field;
    let before = f.muls();
    let comps = pre.weil_restrict(f, x_r);
    *muls += f.muls() - before;
    let input = comps.clone().map(|component| {
        let terms: Vec<_> = component
            .into_iter()
            .map(|(e, c)| (e.map(u32::from).to_vec(), c))
            .collect();
        // normalise performs one scaling product per coefficient. Further
        // engine scalar housekeeping is outside its native row counter.
        *muls += terms.len() as u64;
        f4::normalise(&terms, f.p, f4::Ordering::Grevlex)
    });
    let opts = f4::F4Options::new(f4::Ordering::Grevlex, 24);
    let report = if let Some(trace) = trace {
        trace.pool.run(&input, 3, f.p, &opts, &[None; 3])
    } else {
        f4::f4(&input, 3, f.p, &opts)
    };
    *muls += report.field_ops;
    work.basis_row_muls += report.field_ops;
    work.max_basis_columns = work.max_basis_columns.max(report.max_cols);
    if report.timed_out
        || report.pairs_above_bound != 0
        || report.staircase_at_stop.is_some()
        || report.trace_mismatch
    {
        work.incomplete_bases += 1;
        return None;
    }
    let basis = packed_basis(&report.basis)?;
    let standard_monomials = staircase(&basis)?;
    let dim = standard_monomials.len();
    work.quotient_dimension_total += dim as u64;
    stats.quotient_dim_total += dim as u64;
    if dim == 0 {
        return Some(Vec::new());
    }
    let standard_set: HashSet<_> = standard_monomials.into_iter().collect();
    let mut all = standard_set.clone();
    for b in &standard_set {
        for k in 0..3 {
            let mut m = *b;
            m[k] = m[k].checked_add(1)?;
            all.insert(m);
        }
    }
    let mut cols: Vec<_> = all.into_iter().collect();
    cols.sort_by(|a, b| grevlex_cmp(b, a));
    let col_index: HashMap<_, _> = cols
        .iter()
        .copied()
        .enumerate()
        .map(|(i, m)| (m, i))
        .collect();
    let standard: Vec<_> = cols
        .iter()
        .enumerate()
        .filter_map(|(i, m)| standard_set.contains(m).then_some(i))
        .collect();
    let std_index: HashMap<_, _> = standard
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    let mut rows = Vec::new();
    let mut pivots = Vec::new();
    let before_nf = *muls;
    for (column, &m) in cols.iter().enumerate() {
        if standard_set.contains(&m) {
            continue;
        }
        let nf = normal_form(m, &basis, f.p, muls)?;
        let mut row = vec![0; cols.len()];
        row[column] = 1;
        for (term, coefficient) in nf {
            if !standard_set.contains(&term) {
                return None;
            }
            let c = *col_index.get(&term)?;
            if c <= column {
                return None;
            }
            row[c] = sm(0, coefficient, f.p);
        }
        pivots.push(column);
        rows.push(row);
    }
    work.normal_form_muls += *muls - before_nf;
    let mut nfs = NormalForms::new(&mut rows, &pivots, &std_index, dim, f.p);
    let matrix = mult_matrix(&cols, &col_index, &standard, [1, 0, 0], &mut nfs, muls)?;
    if mode != Extraction::Dense {
        let indices: Option<Vec<_>> = [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|m| col_index.get(m).and_then(|c| std_index.get(c)).copied())
            .collect();
        if let Some(indices) = indices {
            let mut trial = rng.clone();
            if let Some(triples) = krylov::read(
                &matrix,
                indices.try_into().ok()?,
                &comps,
                f.p,
                &mut trial,
                stats,
                &mut work.krylov,
                muls,
                mode == Extraction::DelayedKrylov,
            ) {
                *rng = trial;
                return Some(triples);
            }
        }
        work.krylov.dense_fallbacks += 1;
    }
    match read_solutions(
        &matrix, &cols, &col_index, &standard, &std_index, &pivots, &mut nfs, &comps, f.p, rng,
        stats, muls, false, 24, true,
    ) {
        Read::Solved(triples) => Some(triples),
        _ => None,
    }
}

pub(super) fn triple_oracle(
    inst: &Instance3,
    base: &SubspaceBase,
    pre: &SymmetrisedS4,
    r: &Pt3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Vec<Vec<(usize, i64)>> {
    triple_with(inst, base, pre, r, rng, stats, work, Extraction::Dense)
}

pub(super) fn triple_oracle_krylov(
    inst: &Instance3,
    base: &SubspaceBase,
    pre: &SymmetrisedS4,
    r: &Pt3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Vec<Vec<(usize, i64)>> {
    triple_with(
        inst,
        base,
        pre,
        r,
        rng,
        stats,
        work,
        Extraction::ScalarKrylov,
    )
}

pub(super) fn triple_oracle_krylov_delayed(
    inst: &Instance3,
    base: &SubspaceBase,
    pre: &SymmetrisedS4,
    r: &Pt3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
) -> Vec<Vec<(usize, i64)>> {
    triple_with(
        inst,
        base,
        pre,
        r,
        rng,
        stats,
        work,
        Extraction::DelayedKrylov,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn triple_oracle_trace_delayed(
    inst: &Instance3,
    base: &SubspaceBase,
    pre: &SymmetrisedS4,
    r: &Pt3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    trace: &mut S4TraceState,
) -> Vec<Vec<(usize, i64)>> {
    if r.inf {
        return Vec::new();
    }
    if let Some(triples) = solve_s4_f4_trace_delayed(inst, pre, &r.x, rng, stats, work, trace) {
        let found = base
            .lookup(&inst.curve.field, r)
            .map(|(i, s)| vec![vec![(i, s)]])
            .unwrap_or_default();
        settle_triple_signs(inst, base, r, triples, found)
    } else {
        work.fallback_calls += 1;
        subspace_triple_oracle_groebner(inst, base, pre, r, rng, stats)
    }
}

#[allow(clippy::too_many_arguments)]
fn triple_with(
    inst: &Instance3,
    base: &SubspaceBase,
    pre: &SymmetrisedS4,
    r: &Pt3,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut S4F4Stats,
    mode: Extraction,
) -> Vec<Vec<(usize, i64)>> {
    if r.inf {
        return Vec::new();
    }
    if let Some(triples) = solve_with(inst, pre, &r.x, rng, stats, work, mode) {
        let found = base
            .lookup(&inst.curve.field, r)
            .map(|(i, s)| vec![vec![(i, s)]])
            .unwrap_or_default();
        settle_triple_signs(inst, base, r, triples, found)
    } else {
        work.fallback_calls += 1;
        subspace_triple_oracle_groebner(inst, base, pre, r, rng, stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certified_trace_preserves_distinct_residual_coordinates_and_charges_checks() {
        let inst = generate_instance3(67, 3);
        let pre = SymmetrisedS4::precompute(&inst.curve);
        let mut trace = S4TraceState::new(&inst);
        let mut graph = S4TraceState::new_pair_graph(&inst);
        let mut work = S4F4Stats::default();
        let mut stats = SolveStats::default();
        for d in 1..=8 {
            let r = inst.curve.mul(&inst.curve.g, d);
            let expected = solve_s4_f4_krylov_delayed(
                &inst,
                &pre,
                &r.x,
                &mut StdRng::seed_from_u64(101 + d),
                &mut SolveStats::default(),
                &mut S4F4Stats::default(),
            );
            let got = solve_s4_f4_trace_delayed(
                &inst,
                &pre,
                &r.x,
                &mut StdRng::seed_from_u64(101 + d),
                &mut stats,
                &mut work,
                &mut trace,
            );
            assert_eq!(got, expected, "distinct residual d={d}");
            let got_graph = solve_s4_f4_trace_delayed(
                &inst,
                &pre,
                &r.x,
                &mut StdRng::seed_from_u64(101 + d),
                &mut SolveStats::default(),
                &mut S4F4Stats::default(),
                &mut graph,
            );
            assert_eq!(got_graph, expected, "pair graph residual d={d}");
        }
        let trace_work = trace.stats();
        assert_eq!(work.calls, 8);
        assert!(stats.fp_muls > 0);
        assert!(trace_work.pool.learned > 0);
        assert!(trace_work.pool.accepted > 0);
        assert!(trace_work.pool.certification_pairs > 0);
        assert!(trace_work.pool.certification_multiplications > 0);
        assert!(trace_work.pool.retained_words <= 1_000_000);
        assert_eq!(trace_work.family_rejections, 0);
        let graph_stats = graph.stats().pool;
        assert_eq!(graph_stats.accepted, trace_work.pool.accepted);
        assert_eq!(graph_stats.learned, trace_work.pool.learned);
        assert!(
            graph_stats.certification_multiplications
                < trace_work.pool.certification_multiplications
        );
        let edges = graph_stats.pair_graph.unwrap();
        assert!(edges.chain_edges + edges.product_edges > 0);
        assert_eq!(
            edges.chain_edges + edges.product_edges + edges.direct_edges,
            graph_stats.certification_pairs
        );
    }

    #[test]
    fn changed_trace_family_is_unsupported_without_consuming_root_randomness() {
        let inst = generate_instance3(7, 3);
        let mut other = generate_instance3(7, 3);
        other.q = other.curve.add(&other.q, &other.curve.g);
        let pre = SymmetrisedS4::precompute(&other.curve);
        let mut state = S4TraceState::new(&inst);
        let mut rng = StdRng::seed_from_u64(107);
        let mut before = rng.clone();
        let mut work = S4F4Stats::default();
        assert!(solve_s4_f4_trace_delayed(
            &other,
            &pre,
            &other.curve.g.x,
            &mut rng,
            &mut SolveStats::default(),
            &mut work,
            &mut state
        )
        .is_none());
        assert_eq!(rng.gen::<u64>(), before.gen::<u64>());
        assert_eq!(state.stats().family_rejections, 1);
        assert_eq!(state.stats().pool.learned, 0);
        assert_eq!(work.unsupported_quotients, 1);
    }

    #[test]
    fn certified_trace_graph_and_pivot_keep_the_full_relation_stream() {
        let inst = generate_instance3(7, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        let mut opts = GaudryOptions {
            solver: Solver::F4KrylovDelayed,
            small_base: 1,
            max_large_primes: 2,
            max_residuals: 10000,
            ..GaudryOptions::default()
        };
        let expected = run_gaudry_with_sampler(
            &inst,
            &base,
            5,
            &opts,
            LargePrimeBackend::SymbolicGraph,
            ResidualSampler3::RAddingLong,
        );
        assert_eq!(expected.correct, Some(true));
        for solver in [Solver::F4TraceDelayed, Solver::F4TracePairGraph] {
            opts.solver = solver;
            for backend in [LargePrimeBackend::SymbolicGraph, LargePrimeBackend::Pivot] {
                let got = run_gaudry_with_sampler(
                    &inst,
                    &base,
                    5,
                    &opts,
                    backend,
                    ResidualSampler3::RAddingLong,
                );
                assert_eq!(got.correct, Some(true));
                assert_eq!(got.lp_input_digest, expected.lp_input_digest);
                assert_eq!(got.residuals, expected.residuals);
                assert_eq!(got.recovered_scalar, expected.recovered_scalar);
                assert_eq!(got.group_ops, expected.group_ops);
                assert!(got.s4_f4_stats.unwrap().trace.unwrap().pool.learned > 0);
            }
        }
    }

    fn brute(inst: &Instance3, base: &SubspaceBase, r: &Pt3) -> Vec<Vec<(usize, i64)>> {
        let c = &inst.curve;
        let mut pairs = HashMap::<Pt3, Vec<Vec<(usize, i64)>>>::new();
        for i in 0..base.len() {
            for j in i..base.len() {
                for si in [1, -1] {
                    for sj in [1, -1] {
                        let point = c.add(
                            &c.mul_signed(&base.points[i], si),
                            &c.mul_signed(&base.points[j], sj),
                        );
                        pairs.entry(point).or_default().push(vec![(i, si), (j, sj)]);
                    }
                }
            }
        }
        let mut out = Vec::new();
        for k in 0..base.len() {
            for sk in [1, -1] {
                let rest = c.sub(r, &c.mul_signed(&base.points[k], sk));
                for pair in pairs.get(&rest).into_iter().flatten() {
                    let mut terms = HashMap::<usize, i64>::new();
                    for &(i, s) in pair {
                        *terms.entry(i).or_default() += s;
                    }
                    *terms.entry(k).or_default() += sk;
                    let mut row: Vec<_> = terms.into_iter().filter(|(_, s)| *s != 0).collect();
                    row.sort_unstable();
                    if !row.is_empty() {
                        out.push(row);
                    }
                }
            }
        }
        out.sort();
        out.dedup();
        out
    }
    #[test]
    fn every_tiny_target_matches_independent_signed_triples() {
        let inst = generate_instance3(7, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        let pre = SymmetrisedS4::precompute(&inst.curve);
        for d in 1..inst.curve.n {
            let r = inst.curve.mul(&inst.curve.g, d);
            let got = triple_oracle(
                &inst,
                &base,
                &pre,
                &r,
                &mut StdRng::seed_from_u64(91),
                &mut SolveStats::default(),
                &mut S4F4Stats::default(),
            );
            assert_eq!(got, brute(&inst, &base, &r), "d={d}");
        }
    }
    #[test]
    fn f4_closes_the_two_observed_reference_border_failures() {
        for (p, seed, ordinal) in [(67, 61, 0u64), (127, 59, 3)] {
            let inst = generate_instance3(p, seed);
            let pre = SymmetrisedS4::precompute(&inst.curve);
            let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(seed));
            let mut input_rng = StdRng::seed_from_u64(seed ^ 0x5334_4634);
            let mut r = Pt3::INFINITY;
            for _ in 0..=ordinal {
                r = loop {
                    let a = input_rng.gen_range(0..inst.curve.n);
                    let b = input_rng.gen_range(1..inst.curve.n);
                    let point = inst.curve.add(
                        &inst.curve.mul(&inst.curve.g, a),
                        &inst.curve.mul(&inst.q, b),
                    );
                    if !point.inf {
                        break point;
                    }
                };
            }
            let rng_seed = seed ^ ordinal.wrapping_mul(0x9e37_79b9) ^ 0x534f_4c56;
            assert!(solve_s4_subspace_with(
                &inst,
                &pre,
                &r.x,
                &mut StdRng::seed_from_u64(rng_seed),
                &mut SolveStats::default(),
                SolveMode::default()
            )
            .is_none());
            let mut work = S4F4Stats::default();
            let got = triple_oracle(
                &inst,
                &base,
                &pre,
                &r,
                &mut StdRng::seed_from_u64(rng_seed),
                &mut SolveStats::default(),
                &mut work,
            );
            assert_eq!(work.quotient_dimension_total, 60);
            assert_eq!(work.fallback_calls, 0);
            assert_eq!(got, brute(&inst, &base, &r));
        }
    }
    #[test]
    fn protected_graph_and_pivot_recover_with_native_f4() {
        let inst = generate_instance3(19, 11);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(11));
        let opts = GaudryOptions {
            solver: Solver::F4Groebner,
            small_base: 2,
            max_large_primes: 2,
            max_residuals: 10000,
            ..GaudryOptions::default()
        };
        let p = run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &opts,
            LargePrimeBackend::Pivot,
            ResidualSampler3::RAddingLong,
        );
        let g = run_gaudry_with_sampler(
            &inst,
            &base,
            11,
            &opts,
            LargePrimeBackend::SymbolicGraph,
            ResidualSampler3::RAddingLong,
        );
        assert_eq!(p.correct, Some(true));
        assert_eq!(g.correct, Some(true));
        assert_eq!(p.lp_input_digest, g.lp_input_digest);
        assert_eq!(p.residuals, g.residuals);
        assert!(g.s4_f4_stats.unwrap().completed > 0);
    }
}
