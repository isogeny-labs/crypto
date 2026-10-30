//! Target-labelled pencil geometry on a valid Scholten source fixture.
//! Every accepted three-point relation is checked in the Jacobian.
//! Cohort mode does not perform an independent dual-norm check.
//! Fork of frozen pencil_sparse producer blob e974121a123e13316ce3fcc820b4eca3f65a8a3a.
//! Graph accounting is incomplete: outputs MUST NOT be priced by old analyzers.
use super::{engine, kernel, measurement};
use engine::{Counts, Divisor, Point, Scholten, StudyInstance};
use kernel::{
    allocation::{ObservedSystem, Region},
    collector::{Admission, Collector},
    meter,
    residual::Limits,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

struct GraphSolver {
    collector: Collector,
    nodes: Vec<Option<usize>>,
    field_prime: u64,
    modulus: u64,
    node_count: usize,
    denominator: usize,
    failure: Option<&'static str>,
}

#[cfg(test)]
mod graph_adapter_tests {
    use super::*;

    #[test]
    fn rho_boundary_hook_is_once_and_preserves_recorded_work() {
        let inst = engine::scholten_instance(739, 23).expect("small synthetic fixture");
        let target = inst.curve.mul(inst.generator, 7);
        for (fold, transferred) in [(false, false), (true, false), (true, true)] {
            let plain = engine::solve_rho(&inst, target, 19, 64, fold, transferred);
            let mut calls = 0;
            let hooked = engine::solve_rho_with_target_hook(
                &inst,
                target,
                19,
                64,
                fold,
                transferred,
                || calls += 1,
            );
            assert_eq!(calls, 1);
            assert_eq!(plain.recovered, hooked.recovered);
            assert_eq!(plain.native, hooked.native);
            assert_eq!(
                serde_json::to_value(plain.counts).unwrap(),
                serde_json::to_value(hooked.counts).unwrap()
            );
            assert_eq!(
                serde_json::to_value(plain.phases).unwrap(),
                serde_json::to_value(hooked.phases).unwrap()
            );
        }
    }

    #[test]
    fn online_scopes_exclude_setup_and_cover_all_candidate_stages() {
        let s = Scholten::new(739);
        let mut phases = BTreeMap::new();
        let session = measurement::Session::begin().unwrap();
        measured(None, &s, &mut phases, "solver_setup", || ());
        measurement::begin_online(measurement::Phase::TargetQuery);
        for name in [
            "target_validation",
            "walk_generation",
            "sieve",
            "candidate_verification",
            "elimination",
            "recovery_check",
        ] {
            measured(None, &s, &mut phases, name, || ());
        }
        measurement::end_online();
        let snapshot = session.finish().unwrap();
        assert!(!snapshot.online_phases_ns.contains_key("setup"));
        assert!(snapshot.phases_ns["setup"].is_some());
        for name in [
            "target_query",
            "target_pdp",
            "target_relation_check",
            "target_descent",
            "recovery_check",
        ] {
            assert!(snapshot.online_phases_ns[name].is_some());
        }
        assert_eq!(
            snapshot.online_wall_ns,
            Some(snapshot.online_phases_ns.values().flatten().sum())
        );
    }

    #[test]
    fn explicit_config_and_unavailable_observer() {
        assert!(Config {
            denominator: 3,
            matrix_coefficient_cap: 1_000_000
        }
        .validate()
        .is_err());
        assert!(Config {
            denominator: 2,
            matrix_coefficient_cap: 25_000_001
        }
        .validate()
        .is_err());
        let s = Scholten::new(739);
        let mut phases = BTreeMap::new();
        measured(None, &s, &mut phases, "solver_setup", || vec![1u8; 17]);
        assert!(phases["solver_setup"].allocation.is_none());
        assert!(serde_json::to_value(&phases).unwrap()["solver_setup"]["allocation"].is_null());
        let input = StudyInstance {
            curve: engine::Elliptic {
                f: engine::Field::new(739, 2, 1),
                c: [0; 4],
                binary: true,
            },
            order: 3,
            r: 3,
            h: 1,
            generator: Point::O,
            family: "scholten".into(),
            parameter: 0,
            evidence: Value::Null,
        };
        assert!(solve_one(
            &input,
            Point::O,
            1,
            Config {
                denominator: 2,
                matrix_coefficient_cap: 1_000_000
            },
            None
        )
        .is_err());
        PRICE_ACTIVE.with(|v| v.set(false));
        let _ = std::panic::catch_unwind(|| {
            let _guard = PriceGuard(PRICE_ACTIVE.with(|v| v.replace(true)));
            panic!("fixture");
        });
        assert!(!PRICE_ACTIVE.with(Cell::get));
    }

    #[test]
    fn atomic_scalar_ledger_matches_actual_inverse_instrumentation() {
        use engine::Ring;
        use kernel::scalar_ledger::{pencil_atomic, rho_atomic};
        let modulus = 101;
        BASE_MODULUS.with(|v| v.set(739));
        SCALAR_MODULUS.with(|v| v.set(modulus));
        PRICE_ACTIVE.with(|v| v.set(true));
        reset_arithmetic_counts();
        let candidate = pow(7, modulus - 2, modulus);
        let counts = arithmetic_counts();
        PRICE_ACTIVE.with(|v| v.set(false));
        let ring = Ring::new(modulus);
        assert_eq!(candidate, ring.inv(7));
        let reference = ring.counts.get();
        assert_eq!(
            pencil_atomic(
                modulus,
                counts.scalar_adds as u128,
                0,
                counts.scalar_muls as u128,
                counts.scalar_inversions as u128
            )
            .unwrap(),
            rho_atomic(
                reference.add as u128,
                reference.mul as u128,
                reference.inv as u128
            )
        );
    }

    #[test]
    fn phase_contract_rejects_missing_work_and_keeps_overhead() {
        let field = Counts {
            m: 3,
            ..Counts::default()
        };
        let arithmetic = ArithmeticCounts {
            base_muls: 7,
            ..ArithmeticCounts::default()
        };
        let symbolic = meter::Snapshot {
            map_calls: 11,
            ..meter::Snapshot::default()
        };
        let phases = BTreeMap::from([(
            "fixture",
            PhaseTotals {
                elapsed_ns: 20,
                field,
                arithmetic,
                symbolic,
                ..PhaseTotals::default()
            },
        )]);
        assert_eq!(validate_phases(&phases, field, arithmetic, symbolic, 29), 9);
        assert!(std::panic::catch_unwind(|| validate_phases(
            &phases,
            Counts::default(),
            arithmetic,
            symbolic,
            29
        ))
        .is_err());
        assert!(std::panic::catch_unwind(|| validate_phases(
            &phases,
            field,
            ArithmeticCounts::default(),
            symbolic,
            29
        ))
        .is_err());
        assert!(std::panic::catch_unwind(|| validate_phases(
            &phases,
            field,
            arithmetic,
            meter::Snapshot::default(),
            29
        ))
        .is_err());
        assert!(std::panic::catch_unwind(|| validate_phases(
            &phases, field, arithmetic, symbolic, 19
        ))
        .is_err());
    }

    #[test]
    fn setup_and_elimination_partition_symbolic_events() {
        let s = Scholten::new(739);
        meter::reset();
        let mut phases = BTreeMap::new();
        let mut graph = measured(None, &s, &mut phases, "solver_setup", || {
            let reference = LinearSolver::new(7, 7, &[vec![1, 6]]);
            GraphSolver::new(&reference, 100, 2)
        });
        assert_eq!(phases["solver_setup"].symbolic.primality_checks, 2);
        measured(None, &s, &mut phases, "elimination", || {
            graph.relation(&[[0, 1]], 2, 1)
        });
        let mut sum = meter::Snapshot::default();
        for phase in phases.values() {
            sum += phase.symbolic;
        }
        assert_eq!(sum, meter::snapshot());
        assert!(phases["elimination"].symbolic.scalar_inversions > 0);
    }

    #[test]
    #[should_panic(expected = "inconsistent verified relation")]
    fn constant_only_relation_is_rejected() {
        let reference = LinearSolver::new(7, 7, &[vec![1, 6]]);
        let mut graph = GraphSolver::new(&reference, 100, 2);
        graph.relation(&[], 1, 0);
    }

    #[test]
    fn signed_point_encoding_matches_frozen_eliminator() {
        // Algebraic fixtures only: exercise the legacy point-label convention,
        // not a claim that these coordinates lie on a particular curve.
        for points in [
            vec![[0, 1]],
            vec![[0, 6]],
            vec![[0, 1], [0, 6]],
            vec![[0, 0]],
            vec![[0, 1], [1, 6]],
        ] {
            let ys = vec![vec![1, 6], vec![1, 6]];
            let mut reference = LinearSolver::new(7, 7, &ys);
            let mut graph = GraphSolver::new(&reference, 100, 2);
            let first = graph.relation(&points, 2, 1);
            assert_eq!(first, reference.relation(&points, 2, 1));
            if first.is_none() {
                assert_eq!(
                    graph.relation(&points, 3, 2),
                    reference.relation(&points, 3, 2)
                );
            }
            assert!(graph.failure.is_none());
        }
    }
}
impl GraphSolver {
    fn new(source: &LinearSolver, cap: usize, denominator: usize) -> Self {
        assert!([2, 4, 8, 16].contains(&denominator));
        assert_eq!(source.node_order, "identity");
        let n = source.node_count;
        let partition: Vec<_> = (0..n + 2).map(|i| i >= n || i % denominator == 0).collect();
        Self {
            collector: Collector::new(
                source.modulus,
                &partition,
                Limits {
                    retained_coefficients: cap,
                    working_coefficients: cap,
                },
            )
            .expect("prime subgroup field"),
            nodes: source.nodes.clone(),
            field_prime: source.field_prime,
            modulus: source.modulus,
            node_count: n,
            denominator,
            failure: None,
        }
    }
    fn relation(&mut self, points: &[[u64; 2]], a: u64, b: u64) -> Option<u64> {
        let r = self.modulus;
        let n = self.node_count;
        let mut row = Vec::new();
        for &[x, y] in points {
            if y == 0 {
                continue;
            }
            let col = self.nodes[x as usize].expect("checked rational point");
            row.push((col, if y <= self.field_prime / 2 { 1 } else { r - 1 }));
        }
        row.push((n, b));
        row.push((n + 1, sub(0, a, r)));
        match self.collector.insert(&row).expect("bound columns") {
            Admission::StoppedWorkingCap => {
                self.failure = Some("symbolic_working_cap");
                return None;
            }
            Admission::StoppedRetainedCap => {
                self.failure = Some("symbolic_retained_cap");
                return None;
            }
            Admission::AlreadyStopped => panic!("terminal cap was ignored"),
            Admission::Independent => {}
            _ => return None,
        }
        let target_column = self.collector.small_columns().len() - 2;
        assert!(
            self.collector.basis().pivot(target_column + 1).is_none(),
            "inconsistent verified relation"
        );
        if let Some(row) = self.collector.basis().pivot(target_column) {
            // Normalized target pivot is one; same signed convention as
            // the frozen LinearSolver, with independent source replay below.
            return Some(row.get(&(target_column + 1)).copied().unwrap_or(0));
        }
        None
    }
    fn detail(&self) -> Value {
        let c = self.collector.counts();
        let b = self.collector.basis();
        let a = b.counts();
        json!({"partition":"identity_column_mod_denominator", "denominator":self.denominator,
            "submitted_rows":c.submitted_rows,"submitted_terms":c.submitted_terms,
            "admitted_rows":c.admitted_rows,"rejected_three_large":c.rejected_three_large,
            "emitted_constraints":c.emitted_constraints,"residual_rank":b.rank(),
            "retained_coefficients":b.retained_coefficients(),
            "input_terms":a.input_terms,"pivot_lookups":a.pivot_lookups,
            "coefficient_updates":a.coefficient_updates,
            "normalization_multiplications":a.normalization_multiplications,
            "inversions":a.inversions,"peak_working_coefficients":a.peak_working_coefficients,
            "pending_constraint":self.collector.pending_constraint(),
            "semantic_counts":meter::snapshot(),
            "cost_status":"incomplete_graph_and_allocation_accounting",
            "candidate_over_rho":Value::Null})
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
struct ArithmeticCounts {
    base_adds: u64,
    base_muls: u64,
    base_inversions: u64,
    scalar_adds: u64,
    scalar_muls: u64,
    scalar_inversions: u64,
}
impl ArithmeticCounts {
    fn difference(self, before: Self) -> Self {
        Self {
            base_adds: self.base_adds - before.base_adds,
            base_muls: self.base_muls - before.base_muls,
            base_inversions: self.base_inversions - before.base_inversions,
            scalar_adds: self.scalar_adds - before.scalar_adds,
            scalar_muls: self.scalar_muls - before.scalar_muls,
            scalar_inversions: self.scalar_inversions - before.scalar_inversions,
        }
    }
    fn add_assign(&mut self, other: Self) {
        self.base_adds += other.base_adds;
        self.base_muls += other.base_muls;
        self.base_inversions += other.base_inversions;
        self.scalar_adds += other.scalar_adds;
        self.scalar_muls += other.scalar_muls;
        self.scalar_inversions += other.scalar_inversions;
    }
}
#[derive(Default, Serialize)]
struct PhaseTotals {
    elapsed_ns: u128,
    field: Counts,
    arithmetic: ArithmeticCounts,
    symbolic: meter::Snapshot,
    allocation: Option<Region>,
}
fn group_counts(s: &Scholten) -> Counts {
    s.e.f.counts() + s.j.f.counts()
}
fn measured<T>(
    observer: Option<&ObservedSystem>,
    s: &Scholten,
    phases: &mut BTreeMap<&'static str, PhaseTotals>,
    name: &'static str,
    f: impl FnOnce() -> T,
) -> T {
    // The enclosing query scope owns loop/control work between these scopes.
    // Begin/end are managed by solve_one; standalone callers without a session
    // retain the existing diagnostic intervals and these hooks are inert.
    use measurement::Phase;
    let _online_phase = measurement::scope(match name {
        "solver_setup" => Phase::Setup,
        "sieve" => Phase::TargetPdp,
        "candidate_verification" => Phase::TargetRelationCheck,
        "elimination" => Phase::TargetDescent,
        "recovery_check" => Phase::RecoveryCheck,
        "target_validation" | "walk_generation" => Phase::TargetQuery,
        _ => panic!("unmapped pencil phase"),
    });
    let before_field = group_counts(s);
    let before_arithmetic = arithmetic_counts();
    let before_symbolic = meter::snapshot();
    let before_allocation = observer.map(ObservedSystem::begin);
    let start = Instant::now();
    let result = f();
    let elapsed_ns = start.elapsed().as_nanos();
    let allocation = observer.zip(before_allocation).map(|(o, b)| o.finish(b));
    let phase = phases.entry(name).or_default();
    phase.elapsed_ns += elapsed_ns;
    phase.field = phase.field + (group_counts(s) - before_field);
    phase.symbolic += meter::snapshot() - before_symbolic;
    if let Some(allocation) = allocation {
        phase
            .allocation
            .get_or_insert_with(Region::default)
            .add_assign(allocation);
    }
    phase
        .arithmetic
        .add_assign(arithmetic_counts().difference(before_arithmetic));
    result
}
// Reject an export if measured arithmetic escaped the named phases. The
// envelope remainder includes bookkeeping and control flow, not missing field work.
fn validate_phases(
    phases: &BTreeMap<&'static str, PhaseTotals>,
    field: Counts,
    arithmetic: ArithmeticCounts,
    symbolic: meter::Snapshot,
    envelope_ns: u128,
) -> u128 {
    let mut field_sum = Counts::default();
    let mut arithmetic_sum = ArithmeticCounts::default();
    let mut symbolic_sum = meter::Snapshot::default();
    let mut elapsed_sum = 0u128;
    for phase in phases.values() {
        field_sum = field_sum + phase.field;
        arithmetic_sum.add_assign(phase.arithmetic);
        symbolic_sum += phase.symbolic;
        elapsed_sum = elapsed_sum
            .checked_add(phase.elapsed_ns)
            .expect("phase time overflow");
    }
    assert_eq!(field_sum, field, "unattributed field work");
    assert_eq!(arithmetic_sum, arithmetic, "unattributed native arithmetic");
    assert_eq!(symbolic_sum, symbolic, "unattributed symbolic work");
    envelope_ns
        .checked_sub(elapsed_sum)
        .expect("phase time exceeds envelope")
}
thread_local! {
    static PRICE_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static PRICE_COUNTS: Cell<ArithmeticCounts> = Cell::new(ArithmeticCounts::default());
    static BASE_MODULUS: Cell<u64> = const { Cell::new(739) };
    static SCALAR_MODULUS: Cell<u64> = const { Cell::new(34147) };
}
fn charge(modulus: u64, action: impl FnOnce(&mut ArithmeticCounts)) {
    PRICE_ACTIVE.with(|active| {
        if active.get() {
            PRICE_COUNTS.with(|counts| {
                let mut c = counts.get();
                assert!(
                    BASE_MODULUS.with(Cell::get) == modulus
                        || SCALAR_MODULUS.with(Cell::get) == modulus
                );
                action(&mut c);
                counts.set(c);
            });
        }
    });
}
fn arithmetic_counts() -> ArithmeticCounts {
    PRICE_COUNTS.with(Cell::get)
}
fn reset_arithmetic_counts() {
    PRICE_COUNTS.with(|c| c.set(ArithmeticCounts::default()));
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_muls += 1
        } else {
            c.scalar_muls += 1
        }
    });
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    assert_eq!(n, p - 2);
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_inversions += 1
        } else {
            c.scalar_inversions += 1
        }
    });
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn pmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(x, y, p), p);
        }
    }
    trim(out)
}
fn peval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |v, &c| add(mul(v, x, p), c, p))
}
fn divide_monic(mut a: Vec<u64>, b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    assert_eq!(b.last(), Some(&1));
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let c = *a.last().unwrap();
        q[shift] = c;
        for (i, &bi) in b.iter().enumerate() {
            a[i + shift] = sub(a[i + shift], mul(c, bi, p), p);
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn complement(d: Divisor, lambda: u64, h: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let u = &d.u[..=d.degree];
    let mut w = vec![0; u.len()];
    for i in 0..u.len() {
        let vi = if i < d.degree { d.v[i] } else { 0 };
        w[i] = add(vi, mul(lambda, u[i], p), p);
    }
    let w2 = pmul(&w, &w, p);
    let numerator = trim(
        (0..h.len())
            .map(|i| sub(*w2.get(i).unwrap_or(&0), h[i], p))
            .collect(),
    );
    let (quotient, remainder) = divide_monic(numerator, u, p);
    assert!(remainder.is_empty());
    (quotient, w)
}
fn rational_y_by_x(h: &[u64], p: u64) -> Vec<Vec<u64>> {
    let mut square_roots = vec![Vec::new(); p as usize];
    for y in 0..p {
        square_roots[mul(y, y, p) as usize].push(y);
    }
    (0..p)
        .map(|x| square_roots[peval(h, x, p) as usize].clone())
        .collect()
}
type SieveResult = (
    BTreeSet<u64>,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    Vec<Vec<u64>>,
);
fn point_sieve_candidates(d: Divisor, h: &[u64], p: u64, rational_y: &[Vec<u64>]) -> SieveResult {
    let mut buckets = vec![Vec::<u64>::new(); p as usize];
    let mut point_probes = 0;
    let mut denominators = Vec::new();
    let mut exceptional_denominators = Vec::new();
    let mut exceptional_points = Vec::new();
    let mut all_lambda_points = Vec::new();
    let u = &d.u[..=d.degree];
    let v = &d.v[..d.degree];
    let (q0, _) = complement(d, 0, h, p);
    let mut exceptional = BTreeSet::new();
    for x in 0..p {
        let ux = peval(u, x, p);
        let vx = peval(v, x, p);
        if ux == 0 {
            // q_lambda = q_0 + 2 lambda v + lambda^2 u.  At a root
            // of u, this is linear in lambda even for a double root.
            let qx = peval(&q0, x, p);
            let slope = mul(2, vx, p);
            if slope != 0 {
                exceptional_denominators.push((x, qx, slope));
            } else if qx == 0 {
                exceptional.extend(0..p);
                all_lambda_points.push(x);
            }
            continue;
        }
        if rational_y[x as usize].is_empty() {
            continue;
        }
        denominators.push((x, ux, vx));
    }
    let ordinary_count = denominators.len();
    let all_denominators: Vec<u64> = denominators
        .iter()
        .map(|&(_, ux, _)| ux)
        .chain(exceptional_denominators.iter().map(|&(_, _, slope)| slope))
        .collect();
    let mut prefix = vec![1; all_denominators.len() + 1];
    let mut batch_multiplies = 0;
    for (i, &d) in all_denominators.iter().enumerate() {
        prefix[i + 1] = mul(prefix[i], d, p);
        batch_multiplies += 1;
    }
    let batch_inversions = usize::from(!all_denominators.is_empty());
    let mut suffix = if all_denominators.is_empty() {
        1
    } else {
        pow(*prefix.last().unwrap(), p - 2, p)
    };
    for i in (0..all_denominators.len()).rev() {
        let d_i = all_denominators[i];
        let inv = mul(prefix[i], suffix, p);
        suffix = mul(suffix, d_i, p);
        batch_multiplies += 2;
        assert_eq!(mul(inv, d_i, p), 1);
        if i >= ordinary_count {
            let (x, qx, _) = exceptional_denominators[i - ordinary_count];
            let lambda = mul(sub(0, qx, p), inv, p);
            exceptional.insert(lambda);
            exceptional_points.push((lambda, x));
            continue;
        }
        let (x, ux, vx) = denominators[i];
        assert_eq!(mul(inv, ux, p), 1);
        for &y in &rational_y[x as usize] {
            point_probes += 1;
            let lambda = mul(sub(y, vx, p), inv, p);
            buckets[lambda as usize].push(x);
        }
    }
    let collision_candidates = buckets.iter().filter(|b| b.len() >= 3).count();
    let exceptional_count = exceptional.len();
    let mut candidates = exceptional;
    for (lambda, bucket) in buckets.iter().enumerate() {
        if bucket.len() >= 3 {
            assert_eq!(
                bucket.iter().copied().collect::<BTreeSet<_>>().len(),
                bucket.len()
            );
            candidates.insert(lambda as u64);
        }
    }
    for (lambda, x) in exceptional_points {
        buckets[lambda as usize].push(x);
    }
    for x in all_lambda_points {
        for bucket in &mut buckets {
            bucket.push(x);
        }
    }
    (
        candidates,
        ordinary_count,
        exceptional_denominators.len(),
        batch_inversions,
        batch_multiplies,
        point_probes,
        collision_candidates,
        exceptional_count,
        buckets,
    )
}
struct LinearSolver {
    field_prime: u64,
    modulus: u64,
    nodes: Vec<Option<usize>>,
    node_count: usize,
    #[cfg(test)]
    pivots: Vec<Option<BTreeMap<usize, u64>>>,
    #[cfg(test)]
    rank: usize,
    #[cfg(test)]
    stored_nonzero_coefficients: usize,
    #[cfg(test)]
    row_updates: u64,
    #[cfg(test)]
    inversions: u64,
    #[cfg(test)]
    inserted_relations: u64,
    #[cfg(test)]
    dependent_relations: u64,
    node_order: String,
    node_order_ops: u64,
}
impl LinearSolver {
    fn new(p: u64, r: u64, rational_y: &[Vec<u64>]) -> Self {
        let mut nodes = vec![None; p as usize];
        let mut count = 0;
        for (x, ys) in rational_y.iter().enumerate() {
            if ys.iter().any(|&y| y != 0) {
                nodes[x] = Some(count);
                count += 1;
            }
        }
        let node_order = "identity".to_string();
        let node_order_ops = 0;
        Self {
            field_prime: p,
            modulus: r,
            nodes,
            node_count: count,
            #[cfg(test)]
            pivots: vec![None; count],
            #[cfg(test)]
            rank: 0,
            #[cfg(test)]
            stored_nonzero_coefficients: 0,
            #[cfg(test)]
            row_updates: 0,
            #[cfg(test)]
            inversions: 0,
            #[cfg(test)]
            inserted_relations: 0,
            #[cfg(test)]
            dependent_relations: 0,
            node_order,
            node_order_ops,
        }
    }
    #[cfg(test)]
    fn relation(&mut self, points: &[[u64; 2]], a: u64, b: u64) -> Option<u64> {
        let n = self.pivots.len();
        let r = self.modulus;
        let mut row = BTreeMap::<usize, u64>::new();
        for &[x, y] in points {
            if y == 0 {
                continue;
            }
            let col = self.nodes[x as usize].expect("rational point has a node");
            // The hyperelliptic involution negates the point class.
            let sign = if y <= self.field_prime / 2 { 1 } else { r - 1 };
            let value = add(row.get(&col).copied().unwrap_or(0), sign, r);
            if value == 0 {
                row.remove(&col);
            } else {
                row.insert(col, value);
            }
        }
        if b != 0 {
            row.insert(n, b);
        }
        let neg_a = sub(0, a, r);
        if neg_a != 0 {
            row.insert(n + 1, neg_a);
        }
        self.inserted_relations += 1;
        while let Some((&col, &coefficient)) = row.first_key_value() {
            if col >= n {
                break;
            }
            if let Some(pivot) = &self.pivots[col] {
                for (&j, &v) in pivot {
                    let updated = sub(row.get(&j).copied().unwrap_or(0), mul(coefficient, v, r), r);
                    if updated == 0 {
                        row.remove(&j);
                    } else {
                        row.insert(j, updated);
                    }
                    self.row_updates += 1;
                }
            } else {
                let inv = pow(coefficient, r - 2, r);
                self.inversions += 1;
                for item in row.values_mut() {
                    *item = mul(*item, inv, r);
                    self.row_updates += 1;
                }
                self.stored_nonzero_coefficients += row.len();
                self.pivots[col] = Some(row);
                self.rank += 1;
                return None;
            }
        }
        self.dependent_relations += 1;
        let target_coefficient = row.get(&n).copied().unwrap_or(0);
        let constant = row.get(&(n + 1)).copied().unwrap_or(0);
        if target_coefficient == 0 {
            assert_eq!(constant, 0, "inconsistent verified relation");
            return None;
        }
        self.inversions += 1;
        Some(mul(constant, pow(target_coefficient, r - 2, r), r))
    }
}

/// Fixed algorithm choices. The archived trial_cap belongs to rho, not this solver.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub denominator: usize,
    pub matrix_coefficient_cap: usize,
}
impl Config {
    pub fn validate(&self) -> Result<(), &'static str> {
        if ![2, 4, 8, 16].contains(&self.denominator) {
            return Err("invalid small-base denominator");
        }
        if !(1_000_000..=25_000_000).contains(&self.matrix_coefficient_cap) {
            return Err("invalid coefficient cap");
        }
        Ok(())
    }
}

pub struct PencilResult {
    pub recovered: Option<u64>,
    pub source_verified: bool,
    pub setup: Value,
    pub detail: Value,
    // Retain candidate allocations across a subsequent matched reference call,
    // v2 has its own live-memory baseline; setup temporaries drop on return.
    _solver: LinearSolver,
    _graph: GraphSolver,
}

struct PriceGuard(bool);
impl Drop for PriceGuard {
    fn drop(&mut self) {
        PRICE_ACTIVE.with(|active| active.set(self.0));
    }
}

/// One independent public target, including cold setup. No planted answer is accepted.
/// observer must be the process's installed global allocator, or None. The
/// caller must serialize executions: arithmetic counters are thread-local and
/// allocation regions are process-wide. No output or rho invocation occurs here.
pub fn solve_one(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    config: Config,
    observer: Option<&ObservedSystem>,
) -> Result<PencilResult, &'static str> {
    config.validate()?;
    if inst.family != "scholten" {
        return Err("pencil requires Scholten family");
    }
    let p = inst.curve.f.p;
    if inst.curve.binary
        || inst.curve.f.n != 2
        || inst.curve.f.modulus != 1
        || p <= 13
        || p % 4 != 3
        || !engine::is_prime(p)
        || inst.generator == Point::O
    {
        return Err("invalid Scholten source representation");
    }
    let r = inst.r;
    let order = inst.order;
    if r <= 2
        || !engine::is_prime(r)
        || !order.is_multiple_of(r)
        || order == 0
        || inst.h != order / r
    {
        return Err("invalid scalar subgroup");
    }
    let cofactor = order / r;
    let generator = inst.generator;
    let denominator = config.denominator;
    let matrix_coefficient_cap = config.matrix_coefficient_cap;
    BASE_MODULUS.with(|cell| cell.set(p));
    SCALAR_MODULUS.with(|cell| cell.set(r));
    let _price_guard = PriceGuard(PRICE_ACTIVE.with(|active| active.replace(true)));
    let setup_allocation_start = observer.map(ObservedSystem::begin);
    let setup_clock = Instant::now();
    reset_arithmetic_counts();
    let s = Scholten::new(p);
    let source = s.short_curve();
    assert_eq!(source.c, inst.curve.c, "source curve mismatch");
    assert!(source.valid(generator));
    assert_eq!(source.mul(generator, r), Point::O);
    let transferred = s.tau(s.from_short(generator));
    assert_eq!(s.j.mul(transferred, r), Divisor::O);
    let h = &s.j.polynomial;
    let rational_y = rational_y_by_x(h, p);
    let rational_curve_points: usize = rational_y.iter().map(Vec::len).sum();
    let mut projected_points_checked = 0;
    for (x, ys) in rational_y.iter().enumerate() {
        for &y in ys {
            if projected_points_checked == 16 {
                break;
            }
            let projected = s.j.mul(s.j.point(x as u64, y), cofactor);
            assert_eq!(s.j.mul(projected, r), Divisor::O);
            projected_points_checked += 1;
        }
        if projected_points_checked == 16 {
            break;
        }
    }
    assert_eq!(projected_points_checked, 16);
    let setup_field = group_counts(&s);
    let setup_arithmetic = arithmetic_counts();
    let setup_elapsed_ns = setup_clock.elapsed().as_nanos();
    let setup_allocation = observer
        .zip(setup_allocation_start)
        .map(|(o, b)| o.finish(b));
    s.e.f.reset();
    s.j.f.reset();
    reset_arithmetic_counts();
    meter::reset();
    let candidate_envelope = observer.map(ObservedSystem::begin_envelope);
    let candidate_clock = Instant::now();
    let mut phases = BTreeMap::new();
    let (solver, mut graph) = measured(observer, &s, &mut phases, "solver_setup", || {
        let solver = LinearSolver::new(p, r, &rational_y);
        let graph = GraphSolver::new(&solver, matrix_coefficient_cap, denominator);
        (solver, graph)
    });
    measurement::begin_online(measurement::Phase::TargetQuery);
    measured(observer, &s, &mut phases, "target_validation", || {
        assert!(source.valid(target), "invalid target");
    });
    let mut recovered = None;
    let mut failure = None;
    let mut walks = 0;
    let mut candidate_lambdas = 0;
    let mut verified_relations = 0;
    let mut rejected_candidates = 0;
    let mut solution_key = None;
    'walks: for walk_index in 0..16u64 {
        let (a, b, d) = measured(observer, &s, &mut phases, "walk_generation", || {
            let a = walk_label(seed, walk_index, 0x4455, r);
            let mut b = walk_label(seed, walk_index, 0x7788, r);
            if b == 0 {
                b = 1;
            }
            let point = source.add(source.mul(generator, a), source.mul(target, b));
            (a, b, s.tau(s.from_short(point)))
        });
        walks += 1;
        if d.degree != 2 {
            continue;
        }
        let (candidates, _, _, _, _, _, _, _, buckets) =
            measured(observer, &s, &mut phases, "sieve", || {
                point_sieve_candidates(d, h, p, &rational_y)
            });
        candidate_lambdas += candidates.len();
        for lambda in candidates {
            let points = measured(observer, &s, &mut phases, "candidate_verification", || {
                let (quotient, w) = complement(d, lambda, h, p);
                let roots = &buckets[lambda as usize];
                if roots.len() != 3 {
                    return None;
                }
                let mut sum = Divisor::O;
                let mut points = Vec::with_capacity(3);
                for &x in roots {
                    assert_eq!(peval(&quotient, x, p), 0);
                    let y = peval(&w, x, p);
                    assert_eq!(mul(y, y, p), peval(h, x, p));
                    sum = s.j.add(sum, s.j.point(x, y));
                    points.push([x, y]);
                }
                assert_eq!(s.j.add(d, sum), Divisor::O);
                Some(points)
            });
            let Some(points) = points else {
                rejected_candidates += 1;
                continue;
            };
            verified_relations += 1;
            recovered = measured(observer, &s, &mut phases, "elimination", || {
                graph.relation(&points, a, b)
            });
            if recovered.is_some() {
                solution_key = Some([walk_index, lambda]);
                break 'walks;
            }
            if let Some(reason) = graph.failure {
                failure = Some(reason);
                break 'walks;
            }
        }
    }
    if recovered.is_none() && failure.is_none() {
        failure = Some("walk_cap");
    }
    let verified = if let Some(k) = recovered {
        measured(observer, &s, &mut phases, "recovery_check", || {
            source.mul(generator, k) == target
        })
    } else {
        false
    };
    measurement::end_online();
    let candidate_elapsed_ns = candidate_clock.elapsed().as_nanos();
    let candidate_allocation = observer
        .zip(candidate_envelope)
        .map(|(o, b)| o.finish_envelope(b));
    let mut pencil_arithmetic = arithmetic_counts();
    let candidate_phase_overhead_ns = validate_phases(
        &phases,
        group_counts(&s),
        pencil_arithmetic,
        meter::snapshot(),
        candidate_elapsed_ns,
    );
    pencil_arithmetic.add_assign(setup_arithmetic);
    let pencil_field = group_counts(&s) + setup_field;
    let setup = json!({"schema":"pencil-symbolic-setup/v2","p":p,"r":r,
        "allocation":setup_allocation,"elapsed_ns":setup_elapsed_ns,
        "order":order,"cofactor":cofactor,"rational_curve_points":rational_curve_points,
        "projected_points_checked":projected_points_checked,"matrix_coefficient_cap":matrix_coefficient_cap,
        "field":setup_field,"arithmetic":setup_arithmetic});
    let detail = json!({"schema":"pencil-symbolic-result/v2","p":p,"r":r,
        "symbolic":graph.detail(),"cost_status":"incomplete_graph_and_allocation_accounting",
        "candidate_over_rho":Value::Null,"candidate_envelope_allocation":candidate_allocation,
        "candidate_elapsed_ns":candidate_elapsed_ns,"candidate_phase_overhead_ns":candidate_phase_overhead_ns,
        "cold_setup_allocation":setup_allocation,"cold_setup_elapsed_ns":setup_elapsed_ns,
        "recovered":recovered,"verified":verified,"failure":failure,"walks":walks,
        "candidate_lambdas":candidate_lambdas,"verified_relations":verified_relations,
        "rejected_candidates":rejected_candidates,"solution_key":solution_key,
        "solver_mode":"symbolic","matrix_coefficient_cap":matrix_coefficient_cap,
        "solver_nodes":solver.node_count,"solver_rank":graph.collector.basis().rank(),
        "solver_node_order":solver.node_order,"solver_node_order_ops":solver.node_order_ops,
        "solver_stored_nonzero_coefficients":graph.collector.basis().retained_coefficients(),
        "counts":pencil_field,"arithmetic":pencil_arithmetic,"phases":phases,
        "allocation_observer_available":observer.is_some()});
    Ok(PencilResult {
        recovered,
        source_verified: verified,
        setup,
        detail,
        _solver: solver,
        _graph: graph,
    })
}

fn walk_label(seed: u64, walk: u64, lane: u64, modulus: u64) -> u64 {
    let mut x = seed ^ walk.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ lane;
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (x ^ (x >> 31)) % modulus
}
