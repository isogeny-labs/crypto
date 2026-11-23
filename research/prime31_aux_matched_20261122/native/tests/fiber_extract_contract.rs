//! Fixed small-field correctness and closure contracts. No Prime31 campaign,
//! timing comparison, natural-query census or performance inference.
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use prime31_aux_matched::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use prime31_aux_matched::fiber_extract::{
    self, FiberIndex, FiberIndexReport, FiberLimits, FiberVisitReport,
};
use prime31_aux_matched::oracle::{
    self, OracleLimits, OracleStatus, PreparedOracle, Presentation, TowerDescription,
};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

fn term(n: usize, variables: &[(usize, u32)], coefficient: u64) -> (Vec<u32>, u64) {
    let mut powers = vec![0; n];
    for &(i, e) in variables {
        powers[i] = e;
    }
    (powers, coefficient)
}

fn q(i: usize, n: usize) -> Poly {
    // Full-two-torsion first quartic over F31, terminal z=5.
    vec![
        term(n, &[(i, 4)], 1),
        term(n, &[(i, 3)], 26),
        term(n, &[(i, 2)], 2),
        term(n, &[(i, 1)], 5),
        term(n, &[], 1),
    ]
}

fn index(presentation: Presentation) -> FiberIndex {
    FiberIndex::prepare(
        31,
        &[8, 13, 19, 27],
        &[8, 13, 19, 27],
        &TowerDescription::Velu {
            steps: vec![(0, 30), (0, 4)],
            terminal: 5,
        },
        presentation,
    )
    .unwrap()
}

fn options() -> F4Options {
    let mut options = F4Options::new(Ordering::Grevlex, 8);
    // These configure the old visitor, not the dedicated fiber driver. The
    // effective-policy assertions must remain explicit despite these inputs.
    options.domain_roots = f4_fp::DomainRootMode::Frobenius;
    options.sibling_batch = true;
    options.deadline = Some(Instant::now() + Duration::from_secs(20));
    options
}

fn run(
    input: &[Poly],
    n: usize,
    index: &FiberIndex,
    caps: FiberLimits,
) -> (BTreeSet<Vec<u64>>, FiberVisitReport) {
    let mut points = BTreeSet::new();
    let report = fiber_extract::solve_visit(input, n, 31, &options(), index, 2, caps, |point| {
        assert!(input.iter().all(|g| f4_fp::eval(g, point, 31) == 0));
        points.insert(point.to_vec());
        false
    })
    .unwrap();
    (points, report)
}

fn assert_same_index_structure(reused: &FiberIndexReport, standalone: &FiberIndexReport) {
    let structure = |report: &FiberIndexReport| {
        (
            report.full_leaf_values,
            report.live_leaf_values,
            report.block_width,
            report.variable_levels,
            report.full_nodes,
            report.live_nodes,
            report.live_edges,
            report.construction_edge_insertions,
            report.logical_value_payload_bytes,
        )
    };
    assert_eq!(structure(reused), structure(standalone));
    assert!(reused.reuses_fixed_validation_paths);
    assert!(!standalone.reuses_fixed_validation_paths);
    assert_eq!(reused.construction_field_multiplications, 0);
    assert_eq!(standalone.reused_block_path_values, 0);
    assert_eq!(standalone.logical_temporary_path_payload_bytes, 0);
    assert_eq!(
        reused.reused_block_path_values,
        reused.full_leaf_values * (reused.variable_levels + 1)
    );
    assert_eq!(
        reused.logical_temporary_path_payload_bytes,
        reused.reused_block_path_values * std::mem::size_of::<u64>()
    );
}

#[test]
fn exact_four_child_filter_keeps_original_equations_without_root_factorization() {
    let index = index(Presentation::Tower4);
    let setup = index.setup_report();
    assert_eq!(setup.full_nodes, 5);
    assert_eq!(setup.live_edges, 4);
    assert_eq!(setup.construction_field_multiplications, 72);
    let input = vec![
        q(0, 2),
        q(1, 2),
        // x in {8,13}, y=19. The last original equation retains only x=8.
        vec![
            term(2, &[(0, 2)], 1),
            term(2, &[(0, 1)], 10),
            term(2, &[], 11),
        ],
        vec![term(2, &[(1, 1)], 1), term(2, &[], 12)],
        vec![
            term(2, &[(0, 1)], 1),
            term(2, &[(1, 1)], 1),
            term(2, &[], 4),
        ],
    ];
    let (points, report) = run(&input, 2, &index, FiberLimits::default());
    assert!(report.common.complete, "{report:?}");
    assert_eq!(points, BTreeSet::from([vec![8, 19]]));
    assert_eq!(report.common.f4_runs, 0);
    assert_eq!(report.stats.root_core_calls, 0);
    assert!(report.stats.immediate_parent_domains > 0);
    assert!(report.stats.original_equation_checks > 0);
    assert_eq!(report.common.full_field_branches, 0);
    assert_eq!(
        serde_json::to_value(report.effective_policy).unwrap(),
        serde_json::json!({
            "schema":"prime31-fiber-extraction-policy/v1",
            "selector_domain":"descendants_of_nearest_fixed_selector_ancestor",
            "univariate_roots":"direct_domain_horner_at_limit_else_pinned_dense_roots_fp",
            "direct_domain_limit":4,
            "root_filter":"all_available_univariates_and_conditional_domain",
            "substitution":"pinned_substitute_var_per_child_and_forced_ancestor",
            "selector_branch":"strict_domain_reduction_or_singleton",
            "preserves_incoming_equations_and_f4_consequences":true,
            "legacy_domain_roots_applied":false,
            "legacy_sibling_batch_applied":false,
            "static_cartesian_fallback":false,
            "full_field_scan_fallback":false
        })
    );
}

#[test]
fn coupled_f4_consequences_form_an_exact_cover_without_cartesian_fallback() {
    let input = vec![
        q(0, 2),
        q(1, 2),
        vec![
            term(2, &[(0, 1)], 1),
            term(2, &[(1, 1)], 1),
            term(2, &[], 4),
        ],
        vec![term(2, &[(0, 1), (1, 1)], 1), term(2, &[], 3)],
    ];
    let (points, report) = run(
        &input,
        2,
        &index(Presentation::Tower4),
        FiberLimits::default(),
    );
    assert!(report.common.complete, "{report:?}");
    assert_eq!(points, BTreeSet::from([vec![8, 19], vec![19, 8]]));
    assert!(report.common.f4_runs >= 1);
    assert!(
        report.stats.retained_incoming_terms >= input.iter().map(Vec::len).sum::<usize>() as u64
    );
    assert!(report.stats.retained_consequence_terms > 0);
    assert_eq!(report.common.full_field_branches, 0);
}

#[test]
fn selector_only_system_is_unknown_and_never_becomes_a_tuple_enumerator() {
    let input = vec![q(0, 2), q(1, 2)];
    let (points, report) = run(
        &input,
        2,
        &index(Presentation::Tower4),
        FiberLimits::default(),
    );
    assert!(points.is_empty());
    assert!(!report.common.complete);
    assert_eq!(
        report.reason.as_deref(),
        Some("no_informative_univariate_after_f4")
    );
    assert_eq!(report.stats.branches, 0);
    assert_eq!(report.common.full_field_branches, 0);
    assert!(report.stats.selector_univariates_without_pruning >= 2);
}

#[test]
fn ancestors_propagate_and_unrelated_prefix_coordinates_keep_the_full_field() {
    let index = index(Presentation::Tower);
    // For m=2: leaves are variables0,1; the next selector level is2,3.
    // Variable4 is an unrelated prefix and may equal17, outside the leaf set.
    let mut input = Vec::new();
    for (leaf, parent) in [(0, 2), (1, 3)] {
        input.push(vec![
            term(5, &[(leaf, 2)], 1),
            term(5, &[(leaf, 1), (parent, 1)], 30),
            term(5, &[], 30),
        ]);
        input.push(vec![
            term(5, &[(parent, 2)], 1),
            term(5, &[(parent, 1)], 26),
            term(5, &[], 4),
        ]);
    }
    input.extend([
        vec![term(5, &[(0, 1)], 1), term(5, &[], 23)],
        vec![term(5, &[(1, 1)], 1), term(5, &[], 12)],
        vec![term(5, &[(4, 1)], 1), term(5, &[], 14)],
    ]);
    let (points, report) = run(&input, 5, &index, FiberLimits::default());
    assert!(report.common.complete, "{report:?}");
    assert_eq!(points, BTreeSet::from([vec![8, 19, 4, 1, 17]]));
    assert!(report.stats.forced_ancestor_assignments >= 2);
    assert_eq!(report.common.full_field_branches, 0);
}

#[test]
fn limits_deadline_and_visitor_cancellation_remain_unknown_or_stopped() {
    let index = index(Presentation::Tower4);
    let input = vec![q(0, 2), q(1, 2)];
    for (caps, reason) in [
        (
            FiberLimits {
                max_nodes: 0,
                ..FiberLimits::default()
            },
            "fiber_node_limit",
        ),
        (
            FiberLimits {
                max_f4_runs: 0,
                ..FiberLimits::default()
            },
            "fiber_f4_run_limit",
        ),
        (
            FiberLimits {
                max_root_degree: 1,
                ..FiberLimits::default()
            },
            "fiber_root_degree_limit",
        ),
        (
            FiberLimits {
                max_retained_terms: 1,
                ..FiberLimits::default()
            },
            "fiber_retained_term_limit",
        ),
    ] {
        let (_, report) = run(&input, 2, &index, caps);
        assert!(!report.common.complete);
        assert_eq!(report.reason.as_deref(), Some(reason));
    }
    let mut expired = options();
    expired.deadline = Some(Instant::now());
    let timeout = fiber_extract::solve_visit(
        &input,
        2,
        31,
        &expired,
        &index,
        2,
        FiberLimits::default(),
        |_| panic!("expired callback"),
    )
    .unwrap();
    assert!(!timeout.common.complete && timeout.common.timed_out);

    let mut positive = input;
    positive.extend([
        vec![term(2, &[(0, 1)], 1), term(2, &[], 23)],
        vec![term(2, &[(1, 1)], 1), term(2, &[], 12)],
    ]);
    let mut callbacks = 0;
    let stopped = fiber_extract::solve_visit(
        &positive,
        2,
        31,
        &options(),
        &index,
        2,
        FiberLimits::default(),
        |point| {
            assert_eq!(point, &[8, 19]);
            callbacks += 1;
            true
        },
    )
    .unwrap();
    assert_eq!(callbacks, 1);
    assert!(!stopped.common.complete && stopped.common.stopped_early);
}

#[test]
fn complete_empty_and_invalid_fibers_have_distinct_outcomes() {
    let mut input = vec![q(0, 2), q(1, 2)];
    input.push(vec![term(2, &[], 1)]);
    let (points, report) = run(
        &input,
        2,
        &index(Presentation::Tower4),
        FiberLimits::default(),
    );
    assert!(points.is_empty() && report.common.complete);
    assert_eq!(report.common.f4_runs, 0);
    for (full, allowed, tower) in [
        (
            vec![8, 13, 19, 27],
            vec![8, 13, 19, 27],
            TowerDescription::Velu {
                steps: vec![(0, 30), (0, 4)],
                terminal: 6,
            },
        ),
        (
            vec![0, 13, 19, 27],
            vec![0, 13],
            TowerDescription::Velu {
                steps: vec![(0, 30), (0, 4)],
                terminal: 5,
            },
        ),
        (
            vec![8, 13, 19, 27],
            vec![8, 9],
            TowerDescription::Velu {
                steps: vec![(0, 30), (0, 4)],
                terminal: 5,
            },
        ),
    ] {
        assert!(FiberIndex::prepare(31, &full, &allowed, &tower, Presentation::Tower4).is_err());
    }
}

#[test]
fn prepared_fiber_signed_relations_and_infinity_chart_match_the_original_oracle() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let independent = Curve { p: 17, a: 2, b: 2 };
    let values = vec![1, 3];
    let tower = TowerDescription::Velu {
        steps: vec![(0, 3)],
        terminal: 4,
    };
    let fb = FactorBase::from_column_map(
        "fiber-signed-p17".into(),
        vec![PrimePoint::affine(3, 1), PrimePoint::affine(3, 16)],
        vec![0, 0],
        vec![1, 18],
        1,
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )
    .unwrap();
    let limits = OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: false,
        max_degree: 8,
        ..Default::default()
    };
    let prepared = PreparedOracle::prepare_fiber(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        3,
        Presentation::Tower4,
    )
    .unwrap();
    assert_eq!(prepared.setup_report().mode, "prepared_fiber_triangular");
    assert_eq!(prepared.setup_report().validation_field_multiplications, 16);
    assert_eq!(
        prepared
            .setup_report()
            .fiber_index
            .as_ref()
            .unwrap()
            .construction_field_multiplications,
        0
    );
    let standalone = FiberIndex::prepare(17, &values, &[3], &tower, Presentation::Tower4).unwrap();
    assert_eq!(
        standalone.setup_report().construction_field_multiplications,
        16
    );
    assert_same_index_structure(
        prepared.setup_report().fiber_index.as_ref().unwrap(),
        standalone.setup_report(),
    );
    for target in [PrimePoint::INFINITY, fb.points[0]] {
        let legacy = oracle::decompose(
            &curve,
            19,
            &fb,
            &values,
            Some(&tower),
            target,
            3,
            Presentation::Tower4,
            &limits,
            &mut GroupOps::default(),
        )
        .unwrap();
        let result = prepared
            .decompose_fiber(target, &limits, &mut GroupOps::default())
            .unwrap();
        assert!(result.complete, "{result:?}");
        assert_eq!(result.status, legacy.status);
        assert_eq!(
            result
                .relations
                .iter()
                .map(|r| r.row.clone())
                .collect::<BTreeSet<_>>(),
            legacy
                .relations
                .iter()
                .map(|r| r.row.clone())
                .collect::<BTreeSet<_>>()
        );
        for relation in &result.relations {
            let mut sum = Point::IDENTITY;
            for &i in &relation.indices {
                let q = fb.points[i];
                sum = independent.add(sum, Point::affine(q.x, q.y)).unwrap();
            }
            let want = if target.infinity {
                Point::IDENTITY
            } else {
                Point::affine(target.x, target.y)
            };
            assert_eq!(sum, want);
        }
        assert_eq!(result.counters.validation_field_multiplications, 0);
        assert_eq!(result.counters.full_field_branches, 0);
        assert_eq!(
            result.backend_work.as_ref().unwrap()["mode"],
            "prepared_fiber_triangular"
        );
        let policy = serde_json::to_value(fiber_extract::effective_policy()).unwrap();
        assert_eq!(
            result.backend_work.as_ref().unwrap()["effective_policy"],
            policy
        );
        for chart in &result.charts {
            if chart.visit.get("driver").is_some() {
                assert_eq!(chart.visit["effective_policy"], policy);
            }
        }
    }
    let capped = OracleLimits {
        max_solutions: 0,
        ..limits.clone()
    };
    assert_eq!(
        prepared
            .decompose_fiber(fb.points[0], &capped, &mut GroupOps::default())
            .unwrap()
            .status,
        OracleStatus::UnknownLimit
    );
    let plain = PreparedOracle::prepare(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        3,
        Presentation::Tower4,
    )
    .unwrap();
    assert!(plain.setup_report().fiber_index.is_none());
    assert!(plain
        .decompose_fiber(fb.points[0], &limits, &mut GroupOps::default())
        .is_err());
    assert!(PreparedOracle::prepare_fiber(
        &curve,
        19,
        &fb,
        &values,
        Some(&tower),
        3,
        Presentation::Dense
    )
    .is_err());
}

#[test]
fn captured_shifted_dickson_paths_preserve_index_structure_and_signed_queries() {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    // Original paths start at x, shift once by 1, then apply t -> t^2-2
    // twice. All four listed leaves have terminal 0; only x=6 is live here.
    let values = vec![6, 9, 10, 13];
    let tower = TowerDescription::Dickson {
        depth: 2,
        shift: 1,
        terminal: 0,
    };
    let fb = FactorBase::from_column_map(
        "captured-shifted-dickson-p17".into(),
        vec![PrimePoint::affine(6, 3), PrimePoint::affine(6, 14)],
        vec![0, 0],
        vec![1, 18],
        1,
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )
    .unwrap();
    let limits = OracleLimits {
        timeout_ms: 20_000,
        stop_after_first: false,
        ..Default::default()
    };
    for presentation in [Presentation::Tower, Presentation::Tower4] {
        let prepared =
            PreparedOracle::prepare_fiber(&curve, 19, &fb, &values, Some(&tower), 2, presentation)
                .unwrap();
        let standalone = FiberIndex::prepare(17, &values, &[6], &tower, presentation).unwrap();
        assert_eq!(prepared.setup_report().validation_field_multiplications, 8);
        assert_eq!(
            standalone.setup_report().construction_field_multiplications,
            8
        );
        assert_same_index_structure(
            prepared.setup_report().fiber_index.as_ref().unwrap(),
            standalone.setup_report(),
        );
        let (equations, variables) = oracle::equations_for_chart(
            &curve,
            &values,
            Some(&tower),
            PrimePoint::INFINITY,
            2,
            presentation,
            0,
            &limits,
        )
        .unwrap()
        .unwrap();
        let mut coordinates = BTreeSet::new();
        let visit = fiber_extract::solve_visit(
            &equations,
            variables,
            17,
            &options(),
            &standalone,
            2,
            FiberLimits::default(),
            |point| {
                assert!(equations.iter().all(|g| f4_fp::eval(g, point, 17) == 0));
                coordinates.insert(point.to_vec());
                false
            },
        )
        .unwrap();
        assert!(visit.common.complete);
        assert_eq!(coordinates, BTreeSet::from([vec![6; variables]]));
        let reused = prepared
            .decompose_fiber(PrimePoint::INFINITY, &limits, &mut GroupOps::default())
            .unwrap();
        let legacy = oracle::decompose(
            &curve,
            19,
            &fb,
            &values,
            Some(&tower),
            PrimePoint::INFINITY,
            2,
            presentation,
            &limits,
            &mut GroupOps::default(),
        )
        .unwrap();
        assert_eq!(reused.status, OracleStatus::CompletePositive);
        assert_eq!(reused.status, legacy.status);
        assert_eq!(reused.counters.coordinate_solutions, coordinates.len());
        assert_eq!(reused.counters.validation_field_multiplications, 0);
        assert_eq!(
            reused
                .relations
                .iter()
                .map(|r| r.row.clone())
                .collect::<BTreeSet<_>>(),
            legacy
                .relations
                .iter()
                .map(|r| r.row.clone())
                .collect::<BTreeSet<_>>()
        );
        for relation in reused.relations {
            let sum = relation
                .indices
                .iter()
                .fold(PrimePoint::INFINITY, |sum, &i| {
                    curve.add(&mut GroupOps::default(), sum, fb.points[i])
                });
            assert_eq!(sum, PrimePoint::INFINITY);
        }
    }
}
