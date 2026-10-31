use super::*;
use sha2::{Digest, Sha256};
include!("frozen_helper.rs");

fn enable(p: u64) -> PriceGuard {
    BASE_MODULUS.with(|v| v.set(p));
    SCALAR_MODULUS.with(|v| v.set(7));
    PriceGuard(PRICE_ACTIVE.with(|v| v.replace(true)))
}
fn capture(
    s: &Scholten,
    f: impl FnOnce() -> Option<Vec<[u64; 2]>>,
) -> (
    Option<Vec<[u64; 2]>>,
    serde_json::Value,
    meter::Snapshot,
    Counts,
) {
    reset_arithmetic_counts();
    meter::reset();
    s.e.f.reset();
    s.j.f.reset();
    let result = f();
    (
        result,
        serde_json::to_value(arithmetic_counts()).unwrap(),
        meter::snapshot(),
        group_counts(s),
    )
}
fn normalized(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
#[test]
fn baseline_helper_is_exact_source_bound_closure() {
    let bytes = include_bytes!("../provenance/frozen-native-solver.rs.txt");
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        "e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df"
    );
    let original = std::str::from_utf8(bytes).unwrap();
    let body = original
        .split("let points = measured(observer, &s, &mut phases, \"candidate_verification\", || {")
        .nth(1)
        .unwrap()
        .split("\n            });")
        .next()
        .unwrap();
    let helper = include_str!("frozen_helper.rs");
    let helper_body = helper
        .split_once('{')
        .unwrap()
        .1
        .rsplit_once('}')
        .unwrap()
        .0;
    assert_eq!(normalized(body), normalized(helper_body));
    let marker =
        "let points = measured(observer, &s, &mut phases, \"candidate_verification\", || {";
    let start = original.find(marker).unwrap();
    let end = start
        + marker.len()
        + original[start + marker.len()..]
            .find("\n            });")
            .unwrap()
        + "\n            });".len();
    let mut extracted = original.to_owned();
    extracted.replace_range(start..end,"let points = measured(observer, &s, &mut phases, \"candidate_verification\", || { verify_candidate(&s, d, lambda, h, p, &buckets) });");
    let fork = include_str!("native_solver.rs")
        .split("fn verify_candidate(")
        .next()
        .unwrap();
    assert_eq!(
        normalized(&extracted),
        normalized(fork),
        "fork runtime changed outside helper extraction"
    );
}
#[test]
fn rejected_counts_zero_one_two_and_over_three_have_exact_removed_ledger() {
    let p = 19;
    let s = Scholten::new(p);
    let _guard = enable(p);
    // Polynomial-only divisibility fixture, not claimed as a smooth Scholten curve.
    let d = Divisor {
        degree: 2,
        u: [1, 0, 1],
        v: [1, 0],
    };
    let h = [2, 1, 2, 2, 1, 1];
    for length in [0, 1, 2, 4, 5] {
        let mut buckets = vec![vec![]; p as usize];
        buckets[0] = vec![0; length];
        let old = capture(&s, || frozen_verify(&s, d, 0, &h, p, &buckets));
        let new = capture(&s, || verify_candidate(&s, d, 0, &h, p, &buckets));
        assert!(old.0.is_none() && new.0.is_none());
        assert_eq!(old.1["base_muls"], 24);
        assert_eq!(old.1["base_adds"], 30);
        assert_eq!(
            new.1,
            serde_json::json!({"base_adds":0,"base_muls":0,"base_inversions":0,"scalar_adds":0,"scalar_muls":0,"scalar_inversions":0})
        );
        let removed = old.2 - new.2;
        // Independently derived D4,L1,T17,V16; initializations18, copies4.
        assert_eq!(
            removed,
            meter::Snapshot {
                collection_entry_visits: 78,
                collection_elements_copied: 4,
                collection_elements_initialized: 18,
                collection_comparisons: 27,
                ..Default::default()
            }
        );
        assert_eq!(
            new.2,
            meter::Snapshot {
                collection_entry_visits: 1,
                collection_comparisons: 1,
                ..Default::default()
            }
        );
        assert_eq!(old.3, new.3);
    }
}
#[test]
fn smooth_curve_accepted_and_exceptional_paths_match_source_helper() {
    let p = 19;
    let s = Scholten::new(p);
    let _guard = enable(p);
    let ys = rational_y_by_x(&s.j.polynomial, p);
    let points: Vec<_> = ys
        .iter()
        .enumerate()
        .flat_map(|(x, ys)| ys.iter().map(move |&y| (x as u64, y)))
        .collect();
    let mut accepted = 0;
    let mut exceptional = 0;
    let mut ordinary = 0;
    let mut exceptional_accepted = 0;
    let mut exceptional_rejected = 0;
    // Bounded software fixture inventory over one tiny valid curve, no targets or solver.
    for &(x, y) in &points {
        for &(xx, yy) in &points {
            let d = s.j.add(s.j.point(x, y), s.j.point(xx, yy));
            if d.degree != 2 {
                continue;
            }
            let (candidates, _, _, _, _, _, _, exceptional_count, buckets) =
                point_sieve_candidates(d, &s.j.polynomial, p, &ys);
            assert!(
                exceptional_count <= 2,
                "smooth quintic has at most two exceptional lambdas"
            );
            let (q0, _) = complement(d, 0, &s.j.polynomial, p);
            let mut exceptional_lambdas = std::collections::BTreeSet::new();
            let mut ordinary_points = vec![0usize; p as usize];
            for (x, roots) in ys.iter().enumerate() {
                let ux = peval(&d.u, x as u64, p);
                let vx = peval(&d.v, x as u64, p);
                if ux == 0 {
                    let qx = peval(&q0, x as u64, p);
                    let slope = mul(2, vx, p);
                    if slope != 0 {
                        exceptional_lambdas.insert(mul(sub(0, qx, p), pow(slope, p - 2, p), p));
                    } else {
                        assert_ne!(qx, 0, "all-lambda condition contradicts smooth fixture");
                    }
                } else {
                    let inv = pow(ux, p - 2, p);
                    for &y in roots {
                        ordinary_points[mul(sub(y, vx, p), inv, p) as usize] += 1;
                    }
                }
            }
            assert_eq!(exceptional_lambdas.len(), exceptional_count);
            for lambda in candidates {
                let exceptional_origin = exceptional_lambdas.contains(&lambda);
                let ordinary_origin = ordinary_points[lambda as usize] >= 3;
                assert!(exceptional_origin || ordinary_origin);
                assert!(
                    !(exceptional_origin && ordinary_origin),
                    "smooth cubic degree bounds disjoint origins"
                );
                exceptional += usize::from(exceptional_origin);
                ordinary += usize::from(ordinary_origin);
                let old = capture(&s, || {
                    frozen_verify(&s, d, lambda, &s.j.polynomial, p, &buckets)
                });
                let new = capture(&s, || {
                    verify_candidate(&s, d, lambda, &s.j.polynomial, p, &buckets)
                });
                assert_eq!(old.0, new.0);
                if let Some(old_points) = &old.0 {
                    accepted += 1;
                    exceptional_accepted += usize::from(exceptional_origin);
                    assert_eq!(old.1, new.1);
                    assert_eq!(old.2, new.2);
                    assert_eq!(old.3, new.3);
                    let source = LinearSolver::new(p, 7, &ys);
                    let mut old_graph = GraphSolver::new(&source, 0, 2);
                    let mut new_graph = GraphSolver::new(&source, 0, 2);
                    assert_eq!(
                        old_graph.relation(old_points, 1, 1),
                        new_graph.relation(new.0.as_ref().unwrap(), 1, 1)
                    );
                    assert_eq!(old_graph.failure, new_graph.failure);
                    assert_eq!(
                        old_graph.collector.pending_constraint(),
                        new_graph.collector.pending_constraint()
                    );
                } else {
                    assert!(exceptional_origin);
                    exceptional_rejected += 1;
                }
            }
        }
    }
    println!("software fixture counts: accepted={accepted}, ordinary-origin={ordinary}, exceptional-only-origin={exceptional}, exceptional-accepted={exceptional_accepted}, exceptional-rejected={exceptional_rejected}");
    assert!(
        accepted > 0
            && ordinary > 0
            && exceptional > 0
            && exceptional_accepted > 0
            && exceptional_rejected > 0
    );
}
#[test]
fn singular_all_lambda_branch_is_explicitly_outside_smooth_curve_domain() {
    let p = 19;
    let s = Scholten::new(p);
    let _guard = enable(p);
    // u=x², v=0, h=x³(x²+1), hence h(0)=h'(0)=0: singular polynomial only.
    let d = Divisor {
        degree: 2,
        u: [0, 0, 1],
        v: [0, 0],
    };
    let h = [0, 0, 0, 1, 0, 1];
    let ys = rational_y_by_x(&h, p);
    let (candidates, _, _, _, _, _, _, all, buckets) = point_sieve_candidates(d, &h, p, &ys);
    assert_eq!(all, p as usize);
    assert_eq!(candidates.len(), p as usize);
    let mut rejected = 0;
    for lambda in candidates {
        if buckets[lambda as usize].len() != 3 {
            assert!(frozen_verify(&s, d, lambda, &h, p, &buckets).is_none());
            assert!(verify_candidate(&s, d, lambda, &h, p, &buckets).is_none());
            rejected += 1;
        }
    }
    assert!(rejected > 0);
}
#[test]
fn rejection_keeps_phase_partition_and_graph_terminal_cap_behavior() {
    let p = 19;
    let s = Scholten::new(p);
    let _guard = enable(p);
    let d = Divisor {
        degree: 2,
        u: [1, 0, 1],
        v: [1, 0],
    };
    let h = [2, 1, 2, 2, 1, 1];
    let buckets = vec![vec![]; p as usize];
    let mut phases = BTreeMap::new();
    reset_arithmetic_counts();
    meter::reset();
    assert!(
        measured(None, &s, &mut phases, "candidate_verification", || {
            verify_candidate(&s, d, 0, &h, p, &buckets)
        })
        .is_none()
    );
    assert_eq!(phases["candidate_verification"].symbolic, meter::snapshot());
    assert_eq!(
        serde_json::to_value(phases["candidate_verification"].arithmetic).unwrap(),
        serde_json::to_value(arithmetic_counts()).unwrap()
    );
    let source = LinearSolver::new(p, 7, &vec![vec![]; p as usize]);
    let mut left = GraphSolver::new(&source, 0, 2);
    let mut right = GraphSolver::new(&source, 0, 2);
    // Neither rejected helper reaches graph insertion; next identical synthetic row hits same cap.
    assert_eq!(left.relation(&[], 1, 1), right.relation(&[], 1, 1));
    assert_eq!(left.failure, right.failure);
    assert!(left.failure.is_some());
    assert_eq!(
        left.collector.pending_constraint(),
        right.collector.pending_constraint()
    );
}
