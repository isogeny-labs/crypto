use super::*;

pub(super) fn fixture() -> Request {
    Request {
        schema: "pkm.oracle-request/v1".into(),
        p: 17,
        a: 2,
        b: 2,
        t: 3,
        g: 3,
        zeta: 9,
        m: 4,
        target: Point { x: 5, y: 1 },
        arm: Arm::SymmetricS5,
        limits: Limits {
            solver_millis: 15_000,
            max_assignments: 50_000,
            max_witnesses: 10_000,
        },
    }
}

#[test]
fn complete_m4_sets_match_independent_affine_control_on_all_finite_targets() {
    // Fixed public development fixtures, not a measured or blind panel. The
    // second curve includes y=0 in this base; duplicate sign lifts must collapse.
    let mut queries = 0;
    let mut successes = 0;
    for (a, b) in [(2, 2), (0, 7)] {
        let mut r = fixture();
        r.a = a;
        r.b = b;
        for x in 0..r.p {
            for y in 0..r.p {
                let q = Point { x, y };
                if !reference::Curve::new(&r).on_curve(q) {
                    continue;
                }
                r.target = q;
                let result = run(&r).unwrap();
                assert!(
                    result.complete,
                    "a={a} b={b} Q={q:?} status={} visitor={:?}",
                    result.status, result.visitor
                );
                let cert = replay(&r, &serde_json::to_value(&result).unwrap()).unwrap();
                assert_eq!(cert.complete_set_match, Some(true));
                assert!(!result.comparison_admissible);
                assert_eq!(
                    result.ledger.signed_tuples * r.m as u64,
                    result.ledger.point_addition_calls
                );
                queries += 1;
                successes += usize::from(!result.witnesses.is_empty());
            }
        }
    }
    assert_eq!(queries, 35); // #E_2,2(F17)=19 and #E_0,7(F17)=18, excluding O.
    assert!(successes > 0);
}

#[test]
fn all_presentations_agree_on_supported_two_sum_sets() {
    let mut r = fixture();
    r.m = 2;
    r.t = 2;
    r.g = 1;
    r.zeta = 4;
    r.target = Point { x: 6, y: 3 };
    let mut sets = Vec::new();
    for arm in [Arm::PlainChain, Arm::PlainS5, Arm::SymmetricS5] {
        r.arm = arm;
        let result = run(&r).unwrap();
        assert!(result.complete);
        assert!(
            !result.witnesses.is_empty(),
            "positive presentation control must extract a witness"
        );
        replay(&r, &serde_json::to_value(&result).unwrap()).unwrap();
        sets.push(result.witnesses);
    }
    assert_eq!(sets[0], sets[1]);
    assert_eq!(sets[1], sets[2]);
}

#[test]
fn budget_and_output_caps_remain_incomplete() {
    let mut r = fixture();
    r.limits.solver_millis = 0;
    let zero = run(&r).unwrap();
    assert!(!zero.complete);
    assert_eq!(zero.status, "unknown");
    assert!(zero.visitor.is_none());
    let cert = replay(&r, &serde_json::to_value(&zero).unwrap()).unwrap();
    assert_eq!(cert.complete_set_match, None);
    assert!(cert.expected_witnesses > 1);
    r.limits.solver_millis = 15_000;
    r.limits.max_witnesses = 1;
    let capped = run(&r).unwrap();
    assert!(!capped.complete);
    assert_eq!(capped.witnesses.len(), 1);
    assert_eq!(capped.cutoff, Some("witness_limit"));
    assert_eq!(
        replay(&r, &serde_json::to_value(&capped).unwrap())
            .unwrap()
            .complete_set_match,
        None
    );
}

#[test]
fn tiny_control_rejects_lost_duplicate_and_corrupt_witnesses() {
    let r = fixture();
    let (all, _) = reference::all(&r).unwrap();
    let all: Vec<_> = all.into_iter().collect();
    assert!(all.len() > 1);
    assert!(reference::check(&r, &all[1..], true).is_err());
    assert_eq!(
        reference::check(&r, &all[1..], false)
            .unwrap()
            .complete_set_match,
        None
    );
    let mut duplicate = all.clone();
    duplicate.push(all[0].clone());
    assert!(reference::check(&r, &duplicate, true).is_err());
    let mut corrupt = all.clone();
    corrupt[0][0].y = r.p;
    assert!(reference::check(&r, &corrupt, false).is_err());
}

#[test]
fn replay_rejects_binding_completion_and_ledger_tampering() {
    let mut r = fixture();
    r.limits.solver_millis = 0;
    let record = serde_json::to_value(run(&r).unwrap()).unwrap();
    let mut bad = record.clone();
    bad["ledger"]["unattributed_ns"] = serde_json::json!(0);
    bad["ledger"]["total_ns"] = serde_json::json!(1);
    assert!(replay(&r, &bad).is_err());
    let mut bad = record.clone();
    bad["complete"] = serde_json::json!(true);
    assert!(replay(&r, &bad).is_err());
    let mut bad = record.clone();
    bad["comparison_admissible"] = serde_json::json!(true);
    assert!(replay(&r, &bad).is_err());
    let mut bad = record;
    bad["request"]["g"] = serde_json::json!(1);
    assert!(replay(&r, &bad).is_err());
}

#[test]
fn malformed_public_inputs_fail_before_polynomial_construction() {
    let mut r = fixture();
    r.target.y = r.p;
    assert!(run(&r).is_err());
    r = fixture();
    r.zeta = 1;
    assert!(run(&r).is_err());
    r = fixture();
    r.a = 0;
    r.b = 0;
    assert!(run(&r).is_err());
    r = fixture();
    r.limits.max_witnesses = 0;
    assert!(run(&r).is_err());
}

#[test]
fn certified_miss_does_not_depend_on_a_planted_target() {
    let mut r = fixture();
    r.t = 1;
    r.g = 1;
    r.zeta = 16;
    let result = run(&r).unwrap();
    assert!(result.complete);
    assert!(result.witnesses.is_empty());
    assert_eq!(result.status, "complete_miss");
    assert_eq!(
        replay(&r, &serde_json::to_value(&result).unwrap())
            .unwrap()
            .complete_set_match,
        Some(true)
    );
}

#[test]
fn independent_affine_reference_matches_known_group_equations() {
    let r = fixture();
    let mut curve = reference::Curve::new(&r);
    let p = Point { x: 5, y: 1 };
    assert_eq!(curve.add(Some(p), Some(p)), Some(Point { x: 6, y: 3 }));
    assert_eq!(curve.add(Some(p), Some(Point { x: 5, y: 16 })), None);
    let mut q = None;
    for _ in 0..19 {
        q = curve.add(q, Some(p));
    }
    assert_eq!(q, None);
}

#[test]
fn canonical_export_is_deterministic_and_uses_full_field_domains() {
    let mut r = fixture();
    r.m = 2;
    r.t = 2;
    r.g = 1;
    r.zeta = 4;
    let (first, n) = system(&r);
    let (second, n2) = system(&r);
    assert_eq!(n, 4);
    assert_eq!(n, n2);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    let result = run(&r).unwrap();
    assert!(result.visitor.is_some());
    assert!(result.system_sha256.is_some());
    assert!(!result.unpriced_work.is_empty());
}
