//! Bounded public-toy-fixture RR5 collector, without scalar hints or enumeration.
//! Native solver execution is separate from the injected-driver unit controls.
use auxiliary_rr5_adapter::{AuxiliaryBase, Equation, Presentation};
use crypto_lib::cryptanalysis::f4_fp::{
    eval, normalise, solve_visit_traced, F4Options, F4ResourceLimits, Ordering, Poly, VisitEvent,
    VisitReport,
};
use prime31_algebraic_ic::Arithmetic;
use prime31_four_summand::extractor::Status;
use prime31_four_summand::relation_adapter::SignedRelation;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub budget: Duration,
    pub max_terms: usize,
    pub max_assignments: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            budget: Duration::from_secs(3),
            max_terms: 20_000,
            max_assignments: 4096,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PreparedSystem {
    pub prime: u64,
    pub target: (u64, u64),
    pub presentation: String,
    pub names: Vec<String>,
    pub equations: Vec<Equation>,
    pub leaf_offsets: [usize; 4],
    /// Each guard is (left variable, optional right variable, constant, sign).
    /// The guarded expression is left + sign*right + constant modulo p.
    guards: Vec<(usize, Option<usize>, u64, i64)>,
    guard_offset: usize,
}
impl PreparedSystem {
    pub fn satisfied(&self, assignment: &[u64]) -> bool {
        assignment.len() == self.names.len()
            && assignment.iter().all(|&v| v < self.prime)
            && self
                .equations
                .iter()
                .all(|e| eval(e, assignment, self.prime) == 0)
    }
    pub fn terms(&self) -> usize {
        self.equations.iter().map(Vec::len).sum()
    }
}

#[derive(Debug, Serialize)]
pub struct Collection {
    pub status: Status,
    pub reason: String,
    pub input: Option<PreparedSystem>,
    pub assignments: Vec<Vec<u64>>,
    /// Validated prefix unless status is Complete or Empty.
    pub relations: Vec<SignedRelation>,
    pub rejected_assignments: usize,
    pub validated_assignments: usize,
    pub unreplayed_assignments: usize,
    pub duplicate_relations: usize,
    pub refused_fallbacks: usize,
    pub engine_calls: usize,
    pub engine_error: Option<String>,
    pub replay_errors: Vec<(usize, String)>,
    pub traversal: Option<VisitReport>,
}
impl Collection {
    pub fn is_complete(&self) -> bool {
        matches!(self.status, Status::Complete | Status::Empty)
    }
    fn stopped(status: Status, reason: impl Into<String>) -> Self {
        Self {
            status,
            reason: reason.into(),
            input: None,
            assignments: vec![],
            relations: vec![],
            rejected_assignments: 0,
            validated_assignments: 0,
            unreplayed_assignments: 0,
            duplicate_relations: 0,
            refused_fallbacks: 0,
            engine_calls: 0,
            engine_error: None,
            replay_errors: vec![],
            traversal: None,
        }
    }
}

fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
#[cfg(test)]
fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn polynomial(p: u64, variables: usize, terms: &[(u64, Vec<(usize, u32)>)]) -> Poly {
    let mut out = BTreeMap::<Vec<u32>, u64>::new();
    for (coefficient, factors) in terms {
        let mut exponent = vec![0; variables];
        for &(i, degree) in factors {
            exponent[i] += degree;
        }
        let value = (out.get(&exponent).copied().unwrap_or(0) + coefficient) % p;
        if value == 0 {
            out.remove(&exponent);
        } else {
            out.insert(exponent, value);
        }
    }
    normalise(&out.into_iter().collect::<Vec<_>>(), p, Ordering::Grevlex)
}

fn prepare(
    base: &AuxiliaryBase,
    target: Option<(u64, u64)>,
    presentation: Presentation,
    limits: Limits,
    deadline: Instant,
) -> Result<PreparedSystem, (Status, String)> {
    let fail = |status, reason: &str| (status, reason.to_string());
    let curve = base.public_curve();
    let p = curve.p;
    let target = target.ok_or_else(|| fail(Status::UnsupportedChart, "identity target chart"))?;
    if !Arithmetic::default().on_curve(curve, Some(target)) {
        return Err(fail(Status::InvalidInput, "invalid public target"));
    }
    if base.points().iter().any(|point| point.0 == target.0) {
        return Err(fail(Status::UnsupportedChart, "target-in-base chart"));
    }
    if base.points().len() < 4 {
        return Err(fail(
            Status::Empty,
            "four distinct leaves require at least four base points",
        ));
    }
    if base.points().len() > 16 {
        return Err(fail(
            Status::UnsupportedChart,
            "bounded collector base-size limit",
        ));
    }
    let leaf = base.leaf_system(presentation);
    let width = leaf.variable_names().len();
    let mut names = ["t0", "a1", "a0"].map(str::to_string).to_vec();
    let mut leaf_offsets = [0; 4];
    for (i, offset) in leaf_offsets.iter_mut().enumerate() {
        *offset = names.len();
        names.extend(
            leaf.variable_names()
                .iter()
                .map(|name| format!("l{i}_{name}")),
        );
    }
    let mut guards = vec![];
    for i in 0..4 {
        for j in i + 1..4 {
            guards.push((leaf_offsets[i], Some(leaf_offsets[j]), 0, -1));
        }
        guards.push((leaf_offsets[i], None, sub(0, target.0, p), 1));
        guards.push((leaf_offsets[i], Some(0), 0, 1));
    }
    let guard_offset = names.len();
    names.extend((0..guards.len()).map(|i| format!("guard{i}")));
    if names.len() > 64 {
        return Err(fail(Status::BuildLimit, "variable limit"));
    }
    let variables = names.len();
    let mut equations = vec![];
    for &offset in &leaf_offsets {
        if Instant::now() >= deadline {
            return Err(fail(Status::Deadline, "construction deadline"));
        }
        for equation in leaf.equations() {
            let mut lifted = vec![];
            for (powers, coefficient) in equation {
                let mut exponent = vec![0; variables];
                exponent[offset..offset + width].copy_from_slice(powers);
                lifted.push((exponent, *coefficient));
            }
            equations.push(normalise(&lifted, p, Ordering::Grevlex));
        }
        let (x, y) = (offset, offset + 1);
        let (r, s) = target;
        equations.push(polynomial(
            p,
            variables,
            &[
                (1, vec![(y, 1), (x, 1)]),
                (1, vec![(y, 1), (0, 1)]),
                (s, vec![(x, 1)]),
                (s, vec![(0, 1)]),
                (1, vec![(1, 1), (x, 2)]),
                (1, vec![(2, 1), (x, 1)]),
                (sub(0, r, p), vec![(1, 1), (x, 1)]),
                (sub(0, r, p), vec![(2, 1)]),
            ],
        ));
    }
    for (i, &(left, right, constant, sign)) in guards.iter().enumerate() {
        let inverse = guard_offset + i;
        let mut terms = vec![
            (1, vec![(left, 1), (inverse, 1)]),
            (constant, vec![(inverse, 1)]),
            (p - 1, vec![]),
        ];
        if let Some(right) = right {
            terms.push((
                if sign == 1 { 1 } else { p - 1 },
                vec![(right, 1), (inverse, 1)],
            ));
        }
        equations.push(polynomial(p, variables, &terms));
    }
    let system = PreparedSystem {
        prime: p,
        target,
        presentation: format!("{presentation:?}"),
        names,
        equations,
        leaf_offsets,
        guards,
        guard_offset,
    };
    if system.terms() > limits.max_terms {
        return Err(fail(Status::BuildLimit, "input term limit"));
    }
    if Instant::now() >= deadline {
        return Err(fail(Status::Deadline, "construction deadline"));
    }
    Ok(system)
}

fn replay(
    base: &AuxiliaryBase,
    system: &PreparedSystem,
    a: &[u64],
) -> Result<SignedRelation, String> {
    if !system.satisfied(a) {
        return Err("assignment fails original equations or canonical shape".into());
    }
    let p = system.prime;
    let mut points = vec![];
    let mut terms = vec![];
    let mut xs = BTreeSet::new();
    for &offset in &system.leaf_offsets {
        let point = (a[offset], a[offset + 1]);
        let label = base.label(point).ok_or("leaf is outside the signed base")?;
        if !xs.insert(point.0) || point.0 == system.target.0 || (point.0 + a[0]) % p == 0 {
            return Err("assignment violates the requested regular chart".into());
        }
        points.push(point);
        terms.push((
            label.unsigned_abs() as usize - 1,
            if label > 0 { 1 } else { -1 },
        ));
    }
    let mut arithmetic = Arithmetic::default();
    let sum = points.into_iter().fold(None, |sum, point| {
        arithmetic.add(base.public_curve(), sum, Some(point))
    });
    if sum != Some(system.target) {
        return Err("native signed group replay failed".into());
    }
    terms.sort_unstable();
    Ok(SignedRelation { terms })
}

fn native_basis_allowed(generators: &[Poly]) -> bool {
    generators.iter().any(|g| {
        if !g.iter().any(|(_, coefficient)| *coefficient != 0) {
            return false;
        }
        let used = g
            .iter()
            .filter(|(_, c)| *c != 0)
            .flat_map(|(e, _)| {
                e.iter()
                    .enumerate()
                    .filter(|(_, degree)| **degree != 0)
                    .map(|(i, _)| i)
            })
            .collect::<BTreeSet<_>>();
        used.len() <= 1
    })
}
fn trace_gate(event: VisitEvent<'_>, refused: &mut usize) -> Result<(), String> {
    let error = match event {
        VisitEvent::NativeResult {
            report,
            generating_set,
            ..
        } if report.resource_limit.is_none()
            && !report.timed_out
            && !native_basis_allowed(generating_set) =>
        {
            Some("no algebraic univariate; enumeration refused".to_string())
        }
        VisitEvent::Branch { source, .. } if source != "univariate_roots" => {
            Some(format!("forbidden enumeration branch {source}"))
        }
        _ => None,
    };
    if let Some(error) = error {
        *refused += 1;
        Err(error)
    } else {
        Ok(())
    }
}

struct Capture {
    assignments: Vec<Vec<u64>>,
    max: usize,
    deadline: Instant,
    output_limit: bool,
    timed_out: bool,
}
impl Capture {
    fn visit(&mut self, a: &[u64]) -> bool {
        if Instant::now() >= self.deadline {
            self.timed_out = true;
            return true;
        }
        if self.assignments.len() >= self.max {
            self.output_limit = true;
            return true;
        }
        self.assignments.push(a.to_vec());
        false
    }
}
struct EngineOutput {
    capture: Capture,
    report: Result<VisitReport, String>,
    refused: usize,
}
fn native_driver(system: &PreparedSystem, deadline: Instant, limits: Limits) -> EngineOutput {
    let mut options = F4Options::new(Ordering::Grevlex, 16);
    options.deadline = Some(deadline);
    options.resource_limits = Some(F4ResourceLimits {
        max_pairs: 20_000,
        max_symbolic_monomials: 8192,
        max_matrix_entries: 2_000_000,
    });
    let mut capture = Capture {
        assignments: vec![],
        max: limits.max_assignments,
        deadline,
        output_limit: false,
        timed_out: false,
    };
    let mut refused = 0;
    let report = solve_visit_traced(
        &system.equations,
        system.names.len(),
        system.prime,
        &options,
        &vec![None; system.names.len()],
        |a| capture.visit(a),
        |event| trace_gate(event, &mut refused),
    );
    EngineOutput {
        capture,
        report,
        refused,
    }
}

/// The real native backend. Unit qualification injects a driver below instead.
pub fn collect(
    base: &AuxiliaryBase,
    target: Option<(u64, u64)>,
    presentation: Presentation,
    limits: Limits,
) -> Collection {
    run_with(base, target, presentation, limits, native_driver)
}

fn run_with(
    base: &AuxiliaryBase,
    target: Option<(u64, u64)>,
    presentation: Presentation,
    limits: Limits,
    driver: impl FnOnce(&PreparedSystem, Instant, Limits) -> EngineOutput,
) -> Collection {
    if limits.budget.is_zero() {
        return Collection::stopped(Status::Deadline, "zero budget");
    }
    if limits.budget > Duration::from_secs(60)
        || !(1..=20_000).contains(&limits.max_terms)
        || !(1..=4096).contains(&limits.max_assignments)
    {
        return Collection::stopped(Status::InvalidInput, "invalid bounded collector limits");
    }
    let deadline = Instant::now() + limits.budget;
    let system = match prepare(base, target, presentation, limits, deadline) {
        Ok(system) => system,
        Err((status, reason)) => return Collection::stopped(status, reason),
    };
    if Instant::now() >= deadline {
        return Collection::stopped(Status::Deadline, "pre-driver deadline");
    }
    let engine = driver(&system, deadline, limits);
    let mut result = Collection::stopped(Status::Inconclusive, "incomplete traversal");
    result.engine_calls = 1;
    result.refused_fallbacks = engine.refused;
    result.assignments = engine.capture.assignments;
    let mut complete = false;
    match engine.report {
        Err(error) => {
            result.engine_error = Some(error.clone());
            result.reason = error;
        }
        Ok(report) => {
            complete = report.complete
                && !report.stopped_early
                && !report.timed_out
                && !report.trace_stopped
                && report.trace_error.is_none()
                && report.resource_limit.is_none()
                && engine.refused == 0
                && report.full_field_branches == 0
                && report.domain_evaluations == 0;
            if report.timed_out {
                result.status = Status::Deadline;
                result.reason = "native deadline".into();
            } else if let Some(limit) = &report.resource_limit {
                result.status = Status::ResourceLimit;
                result.reason = limit.to_string();
            } else if let Some(error) = &report.trace_error {
                result.reason = error.clone();
            }
            result.traversal = Some(report);
        }
    }
    if engine.capture.output_limit {
        complete = false;
        result.status = Status::OutputLimit;
        result.reason = "assignment cap".into();
    }
    if engine.capture.timed_out {
        complete = false;
        result.status = Status::Deadline;
        result.reason = "capture deadline".into();
    }
    let mut distinct = BTreeSet::new();
    for (index, assignment) in result.assignments.iter().enumerate() {
        if Instant::now() >= deadline {
            complete = false;
            result.status = Status::Deadline;
            result.reason = "replay deadline".into();
            break;
        }
        match replay(base, &system, assignment) {
            Ok(row) => {
                result.validated_assignments += 1;
                if distinct.insert(row.terms.clone()) {
                    result.relations.push(row);
                } else {
                    result.duplicate_relations += 1;
                }
            }
            Err(error) => {
                complete = false;
                result.rejected_assignments += 1;
                result.replay_errors.push((index, error.clone()));
                if !matches!(
                    result.status,
                    Status::Deadline | Status::OutputLimit | Status::ResourceLimit
                ) {
                    result.status = Status::Inconclusive;
                    result.reason = error;
                }
            }
        }
    }
    if Instant::now() >= deadline {
        complete = false;
        result.status = Status::Deadline;
        result.reason = "final deadline".into();
    }
    if complete {
        result.status = if result.relations.is_empty() {
            Status::Empty
        } else {
            Status::Complete
        };
        result.reason = "complete native traversal and signed replay".into();
    }
    result.input = Some(system);
    result.unreplayed_assignments =
        result.assignments.len() - result.validated_assignments - result.rejected_assignments;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use auxiliary_two_isogeny::{Chain, Curve};
    use prime31_algebraic_ic::Curve as NativeCurve;

    fn base() -> AuxiliaryBase {
        let e = Curve::auxiliary(31).unwrap();
        let g = e.point(6, 6).unwrap();
        AuxiliaryBase::new(
            NativeCurve {
                p: 31,
                a: 1,
                b: 0,
                n: 32,
                g: (6, 6),
            },
            &Chain::new(31, g, 3).unwrap(),
            g,
        )
        .unwrap()
    }
    const LEAVES: [(u64, u64); 4] = [(6, 6), (17, 1), (13, 3), (23, 21)];
    const TARGET: (u64, u64) = (2, 14);
    fn inverse(x: u64) -> u64 {
        assert_ne!(x, 0);
        let mut value = 1;
        for _ in 0..29 {
            value = mm(value, x, 31);
        }
        value
    }
    fn assignment(
        base: &AuxiliaryBase,
        system: &PreparedSystem,
        mode: Presentation,
        leaves: [(u64, u64); 4],
    ) -> Vec<u64> {
        let mut a = vec![23, 18, 26];
        for (x, y) in leaves {
            a.extend(base.witness(mode, x, y).unwrap());
        }
        assert_eq!(a.len(), system.guard_offset);
        for &(left, right, constant, sign) in &system.guards {
            let mut value = (a[left] + constant) % 31;
            if let Some(right) = right {
                value = if sign == 1 {
                    (value + a[right]) % 31
                } else {
                    sub(value, a[right], 31)
                };
            }
            a.push(inverse(value));
        }
        a
    }
    fn prepared(base: &AuxiliaryBase, mode: Presentation) -> PreparedSystem {
        prepare(
            base,
            Some(TARGET),
            mode,
            Limits::default(),
            Instant::now() + Duration::from_secs(3),
        )
        .unwrap()
    }
    fn injected(
        a: Vec<Vec<u64>>,
        report: Result<VisitReport, String>,
        deadline: Instant,
        max: usize,
    ) -> EngineOutput {
        EngineOutput {
            capture: Capture {
                assignments: a,
                max,
                deadline,
                output_limit: false,
                timed_out: false,
            },
            report,
            refused: 0,
        }
    }
    fn completed() -> VisitReport {
        VisitReport {
            complete: true,
            ..VisitReport::default()
        }
    }

    #[test]
    fn both_presentations_accept_the_fixed_regular_rr5_relation() {
        let base = base();
        for (mode, variables, equations) in [
            (Presentation::Dense, 29, 30),
            (Presentation::Chained, 37, 38),
        ] {
            let system = prepared(&base, mode);
            let a = assignment(&base, &system, mode, LEAVES);
            assert_eq!(system.names.len(), variables);
            assert_eq!(system.equations.len(), equations);
            assert!(system.satisfied(&a));
            assert_eq!(
                replay(&base, &system, &a).unwrap().terms,
                vec![(0, 1), (3, 1), (4, 1), (5, -1)]
            );
            if mode == Presentation::Chained {
                assert!(system
                    .equations
                    .iter()
                    .flatten()
                    .all(|(e, _)| e.iter().sum::<u32>() <= 3));
            }
        }
        let mut ar = Arithmetic::default();
        let q = LEAVES
            .into_iter()
            .fold(None, |q, point| ar.add(base.public_curve(), q, Some(point)));
        assert_eq!(q, Some(TARGET));
    }

    #[test]
    fn every_witness_coordinate_and_malformed_shape_is_checked() {
        let base = base();
        let system = prepared(&base, Presentation::Chained);
        let a = assignment(&base, &system, Presentation::Chained, LEAVES);
        for i in 0..a.len() {
            let mut corrupt = a.clone();
            corrupt[i] = (corrupt[i] + 1) % 31;
            assert!(replay(&base, &system, &corrupt).is_err(), "index {i}");
        }
        let mut outside = a.clone();
        outside[0] += 31;
        assert!(replay(&base, &system, &outside).is_err());
        assert!(replay(&base, &system, &a[..a.len() - 1]).is_err());
    }

    #[test]
    fn injected_complete_permutations_are_deduplicated_after_replay() {
        let base = base();
        let mode = Presentation::Chained;
        let result = run_with(
            &base,
            Some(TARGET),
            mode,
            Limits::default(),
            |system, deadline, limits| {
                let a = assignment(&base, system, mode, LEAVES);
                let mut permuted = LEAVES;
                permuted.swap(0, 3);
                injected(
                    vec![a.clone(), a, assignment(&base, system, mode, permuted)],
                    Ok(completed()),
                    deadline,
                    limits.max_assignments,
                )
            },
        );
        assert_eq!(result.status, Status::Complete);
        assert_eq!(result.engine_calls, 1);
        assert_eq!(result.assignments.len(), 3);
        assert_eq!(result.relations.len(), 1);
        assert_eq!(result.duplicate_relations, 2);
        assert_eq!(result.rejected_assignments, 0);
        assert_eq!(result.validated_assignments, 3);
        assert_eq!(result.unreplayed_assignments, 0);
    }

    #[test]
    fn invalid_assignment_prevents_complete_answer_but_retains_valid_prefix() {
        let base = base();
        let mode = Presentation::Dense;
        let result = run_with(
            &base,
            Some(TARGET),
            mode,
            Limits::default(),
            |system, deadline, limits| {
                let a = assignment(&base, system, mode, LEAVES);
                let mut bad = a.clone();
                bad[0] = (bad[0] + 1) % 31;
                injected(
                    vec![a, bad],
                    Ok(completed()),
                    deadline,
                    limits.max_assignments,
                )
            },
        );
        assert_eq!(result.status, Status::Inconclusive);
        assert!(!result.is_complete());
        assert_eq!(result.assignments.len(), 2);
        assert_eq!(result.relations.len(), 1);
        assert_eq!(result.rejected_assignments, 1);
        assert_eq!(result.replay_errors.len(), 1);
        assert_eq!(result.replay_errors[0].0, 1);
    }

    #[test]
    fn incomplete_error_timeout_and_empty_driver_statuses_stay_distinct() {
        let base = base();
        let mode = Presentation::Dense;
        for (report, expected) in [
            (Ok(VisitReport::default()), Status::Inconclusive),
            (
                Ok(VisitReport {
                    resource_limit: Some(crypto_lib::cryptanalysis::f4_fp::F4ResourceLimit {
                        stage: "injected",
                        resource: "matrix",
                        observed: 2,
                        limit: 1,
                        overflowed: false,
                    }),
                    ..VisitReport::default()
                }),
                Status::ResourceLimit,
            ),
            (Err("injected backend error".into()), Status::Inconclusive),
            (
                Ok(VisitReport {
                    timed_out: true,
                    ..VisitReport::default()
                }),
                Status::Deadline,
            ),
            (
                Ok(VisitReport {
                    complete: true,
                    stopped_early: true,
                    ..VisitReport::default()
                }),
                Status::Inconclusive,
            ),
            (
                Ok(VisitReport {
                    complete: true,
                    trace_error: Some("injected trace error".into()),
                    ..VisitReport::default()
                }),
                Status::Inconclusive,
            ),
            (
                Ok(VisitReport {
                    complete: true,
                    domain_evaluations: 1,
                    ..VisitReport::default()
                }),
                Status::Inconclusive,
            ),
        ] {
            let result = run_with(
                &base,
                Some(TARGET),
                mode,
                Limits::default(),
                |system, deadline, limits| {
                    injected(
                        vec![assignment(&base, system, mode, LEAVES)],
                        report,
                        deadline,
                        limits.max_assignments,
                    )
                },
            );
            assert_eq!(result.status, expected);
            assert!(!result.is_complete());
            assert_eq!(result.relations.len(), 1);
        }
        let empty = run_with(
            &base,
            Some(TARGET),
            mode,
            Limits::default(),
            |_, deadline, limits| {
                injected(vec![], Ok(completed()), deadline, limits.max_assignments)
            },
        );
        assert_eq!(empty.status, Status::Empty);
        assert!(empty.is_complete());
    }

    #[test]
    fn capture_caps_and_deadlines_stop_without_claiming_completion() {
        let base = base();
        let mode = Presentation::Dense;
        let result = run_with(
            &base,
            Some(TARGET),
            mode,
            Limits {
                max_assignments: 1,
                ..Limits::default()
            },
            |system, deadline, limits| {
                let a = assignment(&base, system, mode, LEAVES);
                let mut e = injected(vec![], Ok(completed()), deadline, limits.max_assignments);
                assert!(!e.capture.visit(&a));
                assert!(e.capture.visit(&a));
                e
            },
        );
        assert_eq!(result.status, Status::OutputLimit);
        assert!(!result.is_complete());
        assert_eq!(result.assignments.len(), 1);
        assert_eq!(result.relations.len(), 1);
        let mut capture = Capture {
            assignments: vec![],
            max: 1,
            deadline: Instant::now(),
            output_limit: false,
            timed_out: false,
        };
        assert!(capture.visit(&[1]));
        assert!(capture.timed_out);
        assert!(capture.assignments.is_empty());
        let expired = run_with(
            &base,
            Some(TARGET),
            mode,
            Limits::default(),
            |system, _, limits| {
                let a = assignment(&base, system, mode, LEAVES);
                let mut output = injected(
                    vec![],
                    Ok(completed()),
                    Instant::now(),
                    limits.max_assignments,
                );
                assert!(output.capture.visit(&a));
                output
            },
        );
        assert_eq!(expired.status, Status::Deadline);
        assert!(!expired.is_complete());
    }

    #[test]
    fn invalid_limits_inputs_and_unsupported_charts_never_call_the_driver() {
        let base = base();
        let mode = Presentation::Dense;
        for (target, limits, expected) in [
            (None, Limits::default(), Status::UnsupportedChart),
            (
                Some(base.points()[0]),
                Limits::default(),
                Status::UnsupportedChart,
            ),
            (Some((31, 0)), Limits::default(), Status::InvalidInput),
            (
                Some(TARGET),
                Limits {
                    budget: Duration::ZERO,
                    ..Limits::default()
                },
                Status::Deadline,
            ),
            (
                Some(TARGET),
                Limits {
                    max_assignments: 0,
                    ..Limits::default()
                },
                Status::InvalidInput,
            ),
            (
                Some(TARGET),
                Limits {
                    max_terms: 1,
                    ..Limits::default()
                },
                Status::BuildLimit,
            ),
            (
                Some(TARGET),
                Limits {
                    budget: Duration::MAX,
                    ..Limits::default()
                },
                Status::InvalidInput,
            ),
        ] {
            let result = run_with(&base, target, mode, limits, |_, _, _| {
                panic!("driver must not run")
            });
            assert_eq!(result.status, expected);
            assert_eq!(result.engine_calls, 0);
        }
    }

    #[test]
    fn trace_policy_requires_algebraic_roots_and_refuses_enumeration() {
        let mut refused = 0;
        assert!(trace_gate(
            VisitEvent::Branch {
                node_id: 0,
                original_variable: 0,
                value: 1,
                source: "univariate_roots"
            },
            &mut refused
        )
        .is_ok());
        assert!(trace_gate(
            VisitEvent::Branch {
                node_id: 0,
                original_variable: 0,
                value: 1,
                source: "full_field"
            },
            &mut refused
        )
        .is_err());
        assert_eq!(refused, 1);
        assert!(native_basis_allowed(&[vec![(vec![0, 0], 1)]]));
        assert!(native_basis_allowed(&[vec![
            (vec![2, 0], 1),
            (vec![0, 0], 1)
        ]]));
        assert!(!native_basis_allowed(&[vec![(vec![1, 1], 1)]]));
        assert!(!native_basis_allowed(&[vec![]]));
        assert!(!native_basis_allowed(&[vec![(vec![0, 0], 0)]]));
    }
}
