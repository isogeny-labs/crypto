use super::*;
use crate::cryptanalysis::{
    ecbench::{
        methods::{self, MethodSpec},
        workload::{CurveSpec, Instance},
    },
    ic_boundary::PrimeCurve,
};

fn method(changes: &[(&str, &str)]) -> ResolvedMethod {
    let mut params: BTreeMap<String, String> = [
        ("encoding", "line3"),
        ("roots", "exact"),
        ("stop", "first_informative"),
        ("labels", "guaranteed"),
        ("subset_size", "2"),
        ("attempts", "8"),
        ("degree", "24"),
        ("budget_ms", "10000"),
        ("sampling_draws", "1000"),
        ("orbit", "none"),
        ("beta", "0"),
        ("lambda", "0"),
        ("prune", "false"),
    ]
    .map(|(k, v)| (k.into(), v.into()))
    .into();
    params.extend(changes.iter().map(|(k, v)| ((*k).into(), (*v).into())));
    methods::resolve(&MethodSpec {
        id: ID.into(),
        params,
    })
    .unwrap()
}

fn fixture() -> (PrimeInstance, PrimePoint) {
    let inst = PrimeInstance {
        name: "aps-development-control".into(),
        curve: PrimeCurve {
            p: 101,
            a: 59,
            b: 6,
        },
        group_order: 107,
        r: 107,
        cofactor: 1,
        generator: (78, 4),
    };
    let q = inst
        .curve
        .mul(&mut GroupOps::default(), inst.generator_point(), 37);
    (inst, q)
}

fn assert_accounting(rep: &SolveReport) {
    assert_eq!(
        rep.phases.iter().map(|p| p.wall_ns.unwrap()).sum::<u64>(),
        rep.solve_wall_ns
    );
    assert_eq!(rep.phases.iter().map(|p| p.gae).sum::<f64>(), rep.total_gae);
    let online = rep.online.as_ref().unwrap();
    assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
    assert_eq!(online.phases_ns.len(), 5);
    assert_eq!(rep.detail["native_work_accounting_complete"], false);
    assert_eq!(rep.detail["costs_attested"], false);
    assert!(!rep.deterministic);
    assert!(!rep.unpriced.is_empty());
}

#[test]
fn streaming_and_exhaustive_recover_with_closed_phase_accounts() {
    let (inst, q) = fixture();
    for stop in ["exhaustive", "first_informative"] {
        let rep = solve(&method(&[("stop", stop)]), &inst, q, 20261026).unwrap();
        assert_eq!(rep.recovered, Some(37));
        assert!(!rep.exhausted);
        assert_accounting(&rep);
        assert!(rep.total_gae > 0.0);
        assert!(rep.online.as_ref().unwrap().phases_ns["target_descent"] > 0);
        assert!(rep.online.as_ref().unwrap().phases_ns["target_recovery_check"] > 0);
        for attempt in rep.detail["attempts"].as_array().unwrap() {
            assert!(attempt["system_sha256"].as_str().is_some());
        }
    }
}

#[test]
fn same_seed_and_system_are_preserved_across_root_controls() {
    let (inst, q) = fixture();
    let exact = solve(&method(&[("stop", "exhaustive")]), &inst, q, 20261026).unwrap();
    let legacy = solve(
        &method(&[("stop", "exhaustive"), ("roots", "legacy")]),
        &inst,
        q,
        20261026,
    )
    .unwrap();
    assert_eq!(exact.recovered, legacy.recovered);
    let a = exact.detail["attempts"].as_array().unwrap();
    let b = legacy.detail["attempts"].as_array().unwrap();
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert_eq!(a["base"], b["base"]);
        assert_eq!(a["system"], b["system"]);
        assert_eq!(
            a["algebra"]["coordinate_sets"],
            b["algebra"]["coordinate_sets"]
        );
        assert_eq!(a["recovery"], b["recovery"]);
    }
    assert!(legacy.counters["aps_legacy_root_count_unknown_attempts"] > 0);
    assert_accounting(&legacy);
}

#[test]
fn sampling_aborts_are_records_and_do_not_erase_retries() {
    let (inst, q) = fixture();
    let rep = solve(
        &method(&[("sampling_draws", "1"), ("attempts", "3")]),
        &inst,
        q,
        u64::MAX,
    )
    .unwrap();
    let attempts = rep.detail["attempts"].as_array().unwrap();
    assert_eq!(attempts.len(), 3);
    assert_eq!(attempts[0]["seed"], u64::MAX);
    assert_eq!(attempts[1]["seed"], 0);
    assert!(attempts
        .iter()
        .all(|a| a["error"].as_str().unwrap().contains("sampling budget")));
    assert!(attempts.iter().all(|a| a["error_stage"] == "sampling"));
    assert_eq!(rep.counters["aps_failed_attempts"], 3);
    assert!(rep.exhausted);
    assert_accounting(&rep);
}

#[test]
fn invalid_live_targets_are_retained_before_any_transcript() {
    let (inst, _) = fixture();
    for q in [
        PrimePoint::INFINITY,
        PrimePoint::affine(1, 1),
        PrimePoint::affine(101, 1),
    ] {
        let rep = solve(&method(&[]), &inst, q, 1).unwrap();
        assert!(rep.detail["target_error"].is_string());
        assert!(rep.detail["attempts"].as_array().unwrap().is_empty());
        assert!(rep.exhausted);
        assert_accounting(&rep);
    }
}

#[test]
fn plan_refuses_unsupported_combinations_and_width() {
    let (inst, _) = fixture();
    for changes in [
        vec![("roots", "legacy")],
        vec![("prune", "true")],
        vec![("beta", "1")],
        vec![("subset_size", "0")],
        vec![("budget_ms", "0")],
        vec![("degree", "25")],
        vec![("attempts", "129")],
        vec![("orbit", "cubic")],
    ] {
        assert!(applicable(&method(&changes), &inst).is_err(), "{changes:?}");
    }
    let mut too_wide = inst;
    too_wide.curve.p = 65537;
    assert!(applicable(&method(&[]), &too_wide).is_err());
}

#[test]
fn independent_witness_check_rejects_corrupt_labels_signs_and_denominator() {
    let (inst, q) = fixture();
    let f = fast(&inst);
    let g = f.point(78, 4);
    let qf = f.point(q.x, q.y);
    let attempts = core::solve_dlp(
        &f,
        g,
        qf,
        Encoding::Line3,
        Labels::Guaranteed,
        2,
        20261026,
        8,
        Duration::from_secs(10),
    )
    .unwrap();
    let row = attempts.last().unwrap();
    let r = row.recovery.as_ref().unwrap();
    check_witness(&inst, q, &row.base, r, &mut GroupOps::default()).unwrap();
    let mut base = row.base.clone();
    base.points.get_mut(&r.coordinates[0]).unwrap().u ^= 1;
    assert!(check_witness(&inst, q, &base, r, &mut GroupOps::default()).is_err());
    let mut bad = r.clone();
    bad.b = 0;
    assert!(check_witness(&inst, q, &row.base, &bad, &mut GroupOps::default()).is_err());
    bad = r.clone();
    bad.signs[0] = 0;
    assert!(check_witness(&inst, q, &row.base, &bad, &mut GroupOps::default()).is_err());
    bad = r.clone();
    bad.coordinates[1] = bad.coordinates[0];
    assert!(check_witness(&inst, q, &row.base, &bad, &mut GroupOps::default()).is_err());
}

#[test]
fn exported_polynomials_bind_actual_equations_and_msolve_syntax() {
    let (inst, q) = fixture();
    let f = fast(&inst);
    let base = core::sample_base(
        &f,
        f.point(78, 4),
        f.point(q.x, q.y),
        &[2, 2, 2],
        Labels::Guaranteed,
        20261026,
        1000,
    )
    .unwrap();
    let system = core::build_system(&f, &base.parts, Encoding::Partitioned3).unwrap();
    let mut export = SystemExport::new(&system);
    assert_eq!(export.equations, system.equations());
    assert!(export.msolve().starts_with("x0,x1,x2\n101\n"));
    let hash = export.sha256().unwrap();
    export.equations[0][0].1 ^= 1;
    assert_ne!(hash, export.sha256().unwrap());
}

#[test]
fn native_method_is_plannable_and_runs_through_ecbench_dispatch() {
    let spec = CurveSpec::PrimeExplicit {
        p: 101,
        a: 59,
        b: 6,
        group_order: 107,
        r: 107,
        gx: 78,
        gy: 4,
    };
    let inst = spec.build().unwrap();
    let facts = inst.facts(&spec);
    let (_, q) = fixture();
    let m = method(&[]);
    methods::applicable(&m, &inst, &facts).unwrap();
    let rep = methods::solve(
        &m,
        &inst,
        &facts,
        &[format!("{:x}", q.x), format!("{:x}", q.y)],
        20261026,
    )
    .unwrap();
    assert_eq!(rep.recovered, Some(37));
    assert_accounting(&rep);
    assert!(matches!(inst, Instance::Prime(_)));
}

#[test]
fn observers_preserve_existing_exhaustive_recovery_and_error_semantics() {
    let (inst, q) = fixture();
    let f = fast(&inst);
    let g = f.point(78, 4);
    let q = f.point(q.x, q.y);
    // The shipped complete fixture recovers on its third base. The first base
    // has no roots and cannot exercise a rejecting visitor.
    let base = core::sample_base(&f, g, q, &[2, 2, 2], Labels::Guaranteed, 20261028, 1000).unwrap();
    let values = vec![base.points.keys().copied().collect()];
    let system = core::build_system(&f, &values, Encoding::Line3).unwrap();
    let report = core::solve_system(&system, 101, 24, Duration::from_secs(10)).unwrap();
    assert!(!report.coordinate_sets.is_empty());
    let old = core::recover(&f, g, q, &base, &report.coordinate_sets).unwrap();
    let mut phases = Vec::new();
    let observed =
        core::recover_observed(&f, g, q, &base, &report.coordinate_sets, |p| phases.push(p))
            .unwrap();
    assert_eq!(
        serde_json::to_value(&old).unwrap(),
        serde_json::to_value(&observed).unwrap()
    );
    assert!(phases.contains(&core::RecoveryPhase::RelationCheck));
    assert!(
        core::visit_system(&system, 101, 24, Duration::from_secs(10), |_| Err(
            "visitor rejection".into()
        ))
        .unwrap_err()
        .contains("visitor rejection")
    );
}

#[test]
fn late_or_unverified_witnesses_are_retained_without_becoming_timed_successes() {
    let (inst, q) = fixture();
    let f = fast(&inst);
    let g = f.point(78, 4);
    let target = f.point(q.x, q.y);
    let base = core::sample_base(
        &f,
        g,
        target,
        &[2, 2, 2],
        Labels::Guaranteed,
        20261028,
        1000,
    )
    .unwrap();
    let system = core::build_system(
        &f,
        &[base.points.keys().copied().collect()],
        Encoding::Line3,
    )
    .unwrap();
    let algebra = core::solve_system(&system, 101, 24, Duration::from_secs(10)).unwrap();
    let recovery = core::recover(&f, g, target, &base, &algebra.coordinate_sets)
        .unwrap()
        .unwrap();
    check_witness(&inst, q, &base, &recovery, &mut GroupOps::default()).unwrap();
    let row = Attempt {
        index: 0,
        seed: 20261028,
        base: Some(base),
        system: Some(SystemExport::new(&system)),
        system_sha256: None,
        visited_coordinate_sets: algebra.coordinate_sets.clone(),
        algebra: Some(algebra),
        recovery: Some(recovery),
        independent_witness_verified: true,
        outcome: "not_started",
        error: None,
        error_stage: None,
        deadline_exhausted: false,
    };
    let budget = Duration::from_nanos(20);
    let mut on_time = row.clone();
    assert_eq!(
        finish_attempt(&mut on_time, Duration::from_nanos(19), budget),
        Some(37)
    );
    assert_eq!(on_time.outcome, "verified");
    let mut late = row.clone();
    assert_eq!(finish_attempt(&mut late, budget, budget), None);
    assert_eq!(late.outcome, "verified_after_deadline");
    assert_eq!(late.recovery.as_ref().unwrap().logarithm, 37);
    assert!(late.independent_witness_verified);
    let mut cancelled = row.clone();
    cancelled.deadline_exhausted = true;
    assert_eq!(
        finish_attempt(&mut cancelled, Duration::from_nanos(1), budget),
        None
    );
    let mut rejected = row.clone();
    rejected.error = Some("independent check rejected candidate".into());
    assert_eq!(
        finish_attempt(&mut rejected, Duration::from_nanos(1), budget),
        None
    );
    assert_eq!(rejected.outcome, "error");
    let mut unverified = row;
    unverified.independent_witness_verified = false;
    assert_eq!(
        finish_attempt(&mut unverified, Duration::from_nanos(1), budget),
        None
    );
}

#[test]
fn partial_native_counts_cannot_be_promoted_to_complete_bitop_cost() {
    let spec = CurveSpec::PrimeExplicit {
        p: 101,
        a: 59,
        b: 6,
        group_order: 107,
        r: 107,
        gx: 78,
        gy: 4,
    };
    let instance = spec.build().unwrap();
    let facts = instance.facts(&spec);
    let (inst, q) = fixture();
    let method = method(&[]);
    let report = solve(&method, &inst, q, 20261026).unwrap();
    let priced = crate::cryptanalysis::ecbench::work::price(
        &crate::cryptanalysis::ecbench::work::Priceable {
            method_id: &method.id,
            method_params: &method.params,
            curve: &facts,
            phases: &report.phases,
            counters: &report.counters,
            solver: report.solver.as_ref(),
        },
    )
    .unwrap();
    assert!(!priced.complete);
    assert!(priced
        .unpriced
        .iter()
        .any(|s| s == "aps_f4_multiplications_uncharged"));
    assert!(!priced.known_uncounted.is_empty());
}

#[test]
fn development_control_specs_plan_all_four_leads_with_identical_aa_and_paired_seeds() {
    use crate::cryptanalysis::ecbench::spec::{plan, Spec};
    for text in [
        include_str!("../../../../research/aps_measurement_20261029/spec-controls.json"),
        include_str!("../../../../research/aps_measurement_20261029/spec-orbit-controls.json"),
    ] {
        let p = plan(Spec::from_json(text).unwrap()).unwrap();
        assert_eq!(p.workloads.len(), 2);
        let controls: Vec<_> = p.arms.iter().filter(|a| a.name.ends_with("-aa")).collect();
        assert_eq!(controls.len(), 2);
        for aa in controls {
            assert!(p
                .arms
                .iter()
                .any(|a| a.name != aa.name && a.method == aa.method));
        }
        let mut seeds = BTreeMap::new();
        for run in p.executions {
            if let Some(previous) =
                seeds.insert((run.workload, run.round, run.warmup), run.algorithm_seed)
            {
                assert_eq!(previous, run.algorithm_seed);
            }
        }
    }
}

#[test]
fn integer_parameter_aliases_keep_one_resolved_method_identity() {
    let baseline = method(&[]);
    let aliases = method(&[
        ("subset_size", "02"),
        ("degree", "+024"),
        ("budget_ms", "010000"),
        ("beta", "-0"),
    ]);
    assert_eq!(baseline, aliases);
}

#[test]
fn shared_recovery_context_keeps_candidate_checks_and_refuses_bad_initial_labels() {
    let (inst, q) = fixture();
    let f = fast(&inst);
    let g = f.point(78, 4);
    let target = f.point(q.x, q.y);
    let base = core::sample_base(
        &f,
        g,
        target,
        &[2, 2, 2],
        Labels::Guaranteed,
        20261028,
        1000,
    )
    .unwrap();
    let system = core::build_system(
        &f,
        &[base.points.keys().copied().collect()],
        Encoding::Line3,
    )
    .unwrap();
    let report = core::solve_system(&system, 101, 24, Duration::from_secs(10)).unwrap();
    assert!(!report.coordinate_sets.is_empty());
    let context = core::RecoveryContext::new(&f, g, target, &base).unwrap();
    for coordinates in &report.coordinate_sets {
        let sets = std::slice::from_ref(coordinates);
        let expected = core::recover(&f, g, target, &base, sets).unwrap();
        let actual = context.recover_observed(sets, |_| {}).unwrap();
        assert_eq!(
            serde_json::to_value(expected).unwrap(),
            serde_json::to_value(actual).unwrap()
        );
    }
    assert!(context
        .recover_observed(&[vec![101, 2, 3]], |_| {})
        .is_err());
    let mut bad = base.clone();
    let entry = bad.points.values_mut().next().unwrap();
    entry.u = (entry.u + 1) % inst.r;
    assert!(core::RecoveryContext::new(&f, g, target, &bad).is_err());
}
