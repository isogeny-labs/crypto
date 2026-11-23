//! Semantic controls nested under independent_tests to reuse its independent
//! integer arithmetic. The expensive B677 completion gate is a separate binary.

use super::*;
use crate::b677_fixture::{Hybrid677, Y7_CSV_SHA256};
use crate::extractor::{extract_cofactor, extract_hybrid677};
use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;

fn dickson1024(mut x: u64) -> u64 {
    for _ in 0..10 {
        x = (product(x, x, PRIME31.p) + PRIME31.p - 2) % PRIME31.p;
    }
    x
}

fn positive_case(fixture: &Hybrid677) -> (Affine, Vec<(usize, i64)>) {
    let mut dickson = Vec::new();
    let mut extra = Vec::new();
    for (index, &(x, _)) in fixture.points().iter().enumerate() {
        if dickson1024(x) == fixture.terminal() {
            dickson.push(index);
        } else {
            extra.push(index);
        }
    }
    assert_eq!(dickson.len(), 509);
    assert_eq!(extra.len(), 168);
    // This fixed witness is a control-harness value. Neither extractor accepts
    // these indices, signs, expected rows, or a known logarithm as an argument.
    let mut witness = vec![
        (dickson[0], 1),
        (dickson[1], -1),
        (extra[0], 1),
        (extra[1], 1),
    ];
    witness.sort_unstable_by_key(|term| term.0);
    let q = sum_row(PRIME31, fixture.points(), &witness);
    (q, witness)
}

#[test]
fn b677_fixture_has_exact_public_selector_and_independent_curve_replay() {
    let fixture = Hybrid677::build().expect("frozen public hybrid fixture");
    let e = PRIME31;
    assert_eq!(fixture.points().len(), 677);
    assert_eq!(
        Y7_CSV_SHA256,
        "fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46"
    );
    assert!(fixture
        .points()
        .windows(2)
        .all(|pair| pair[0].0 < pair[1].0));
    let mut dickson = 0;
    let mut extras = Vec::new();
    let mut coset_roots = [0usize; 8];
    for &(x, y) in fixture.points() {
        assert!(y != 0 && y <= e.p - y && on_curve(e, Some((x, y))));
        if dickson1024(x) == fixture.terminal() {
            dickson += 1;
        } else {
            let seventh = power(rhs(e, x), 7, e.p);
            let matching: Vec<_> = fixture
                .coset_squares()
                .iter()
                .enumerate()
                .filter_map(|(i, &c2)| (seventh == c2).then_some(i))
                .collect();
            assert_eq!(matching.len(), 1);
            coset_roots[matching[0]] += 1;
            extras.push((x, y));
        }
    }
    assert_eq!(dickson, 509);
    assert_eq!(coset_roots, [21; 8]);
    let csv_points: Vec<_> =
        include_str!("../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();
                (x, y.min(e.p - y))
            })
            .collect();
    assert_eq!(extras, csv_points);
    assert_eq!(
        &extras[..2],
        &[(1_530_610, 717_141_936), (13_438_236, 532_309_251)]
    );

    // Resolve the historical final-c transcription through independent bigint
    // modular products, without relying on a fixture-candidate equality check.
    let corrected_c = power(136_769_428, 7, e.p);
    assert_eq!(corrected_c, 1_852_955_275);
    assert_ne!(corrected_c, 1_852_957_275);
    assert_eq!(
        fixture.coset_squares()[7],
        product(corrected_c, corrected_c, e.p)
    );

    let mut horizontal = std::collections::BTreeMap::<u64, Vec<usize>>::new();
    for (i, &(_, y)) in extras.iter().enumerate() {
        horizontal.entry(y).or_default().push(i);
    }
    assert_eq!(horizontal.len(), 56);
    for indices in horizontal.values() {
        assert_eq!(indices.len(), 3);
        let row: Vec<_> = indices.iter().map(|&index| (index, 1)).collect();
        assert_eq!(sum_row(e, &extras, &row), None);
    }
    let certificate = fixture.certificate();
    assert_eq!(certificate.dickson_x_roots, 1024);
    assert_eq!(certificate.dickson_off_curve_x_roots, 515);
    assert_eq!(certificate.y7_coefficient_factorizations, 8);
    assert_eq!(certificate.horizontal_triples_replayed, 56);
    assert_eq!(certificate.selector_degree, 1192);
    assert!(fixture.counts().fp_multiplications > 0);
    assert!(fixture.counts().group_additions > 0);
}

#[test]
fn b677_frozen_mixed_positive_and_public_walk_targets_are_valid() {
    let fixture = Hybrid677::build().unwrap();
    let (q, witness) = positive_case(&fixture);
    let q = q.expect("fixed mixed-subset positive target must be finite");
    assert!(on_curve(PRIME31, Some(q)));
    assert!(!fixture.points().iter().any(|point| point.0 == q.0));
    assert_eq!(witness.len(), 4);
    assert_eq!(witness.iter().filter(|(_, sign)| *sign == -1).count(), 1);
    assert_eq!(sum_row(PRIME31, fixture.points(), &witness), Some(q));
    for target in [(18, 221_278_090), (101_000_006, 1_077_058_598)] {
        assert!(on_curve(PRIME31, Some(target)));
        assert!(!fixture.points().iter().any(|point| point.0 == target.0));
    }
}

#[test]
fn b677_forced_construction_caps_do_not_claim_empty_or_complete() {
    let fixture = Hybrid677::build().unwrap();
    let (q, _) = positive_case(&fixture);
    let limits = Limits {
        max_degree: 5,
        seconds: 5,
        max_polynomial_terms: 1,
        max_assignments: 1,
    };
    let resources = F4ResourceLimits {
        max_pairs: 20_000,
        max_symbolic_monomials: 8_192,
        max_matrix_entries: 2_000_000,
    };
    let structured = extract_hybrid677(&fixture, q, limits, resources);
    let cofactor = extract_cofactor(PRIME31, fixture.points(), q, limits, resources);
    for result in [structured, cofactor] {
        assert!(matches!(
            result.status,
            Status::BuildLimit | Status::Deadline
        ));
        assert!(result.relations.is_empty());
        assert_eq!(result.diagnostics.assignments_extracted, 0);
        assert_eq!(result.diagnostics.forbidden_branch_attempts, 0);
        assert_eq!(result.diagnostics.forbidden_domain_evaluations, 0);
    }
}

#[test]
fn b677_zero_native_resource_caps_fail_closed_before_algebraic_search() {
    let fixture = Hybrid677::build().unwrap();
    // This is an admission control, not one of the registered positive/H0/W0
    // extraction requests. Zero native budgets prohibit algebraic search.
    let q = Some(PRIME31.g);
    assert!(!fixture.points().iter().any(|point| point.0 == PRIME31.g.0));
    let limits = Limits {
        max_degree: 10,
        seconds: 30,
        max_polynomial_terms: 20_000,
        max_assignments: 4096,
    };
    let resources = F4ResourceLimits {
        max_pairs: 0,
        max_symbolic_monomials: 0,
        max_matrix_entries: 0,
    };
    let structured = extract_hybrid677(&fixture, q, limits, resources);
    let cofactor = extract_cofactor(PRIME31, fixture.points(), q, limits, resources);
    for result in [structured, cofactor] {
        assert_eq!(result.status, Status::ResourceLimit, "{}", result.reason);
        assert!(result.relations.is_empty());
        assert_eq!(result.diagnostics.assignments_extracted, 0);
        assert_eq!(result.diagnostics.forbidden_branch_attempts, 0);
        assert_eq!(result.diagnostics.forbidden_domain_evaluations, 0);
        assert_eq!(result.diagnostics.forbidden_fallbacks_refused, 0);
        let trace = result.traversal.as_ref().expect("native admission receipt");
        assert!(!trace.complete && !trace.timed_out && !trace.trace_stopped);
        assert_eq!(trace.f4_runs, 1);
        assert_eq!(trace.solutions_visited, 0);
        assert_eq!(trace.full_field_branches, 0);
        assert_eq!(trace.domain_evaluations, 0);
        let marker = trace
            .resource_limit
            .as_ref()
            .expect("explicit resource stop");
        assert_eq!(marker.limit, 0);
        assert!(marker.observed > 0 || marker.overflowed);
    }
}
