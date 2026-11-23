//! Independent-audit contracts. Setup uses fixed public arithmetic fixtures.
//! Any relation transcript below is constructed deliberately for a unit test;
//! no oracle, public-query campaign, rho search, or timing experiment runs here.
#[test]
fn matched_plain_requires_its_checked_reference_and_exact_column_count() {
    use prime31_aux_matched::bases::build_plain_control;
    let (source, fiber) = unit_source_and_fiber();
    let (mut records, prepared) = setup(fiber, source, TargetArm::IsogenousNative);
    records[0]["cell"]["plain_control"] = json!(true);
    records[0]["cell"]["plain_seed"] = json!(1);
    records[0]["cell"]["max_plain_x_trials"] = json!(1_000_000);
    records[0]["cell"]["presentation"] = json!("dense");
    let base = build_plain_control(
        &prepared.public_target.curve(),
        N,
        prepared.canonical_points.len(),
        1,
        1_000_000,
    )
    .unwrap();
    records.push(
        json!({"event":"choice_plain","base":base,"seed":1,"max_x_trials":1_000_000,
        "reference_columns":prepared.canonical_points.len()}),
    );
    let result = audit(&records).unwrap();
    assert_eq!(result["choice_prepared_independently_verified"], true);
    assert_eq!(result["choice_plain_independently_verified"], true);
    assert_eq!(result["complete"], false);
    let missing_plain = audit(&records[..3]).unwrap();
    assert_eq!(missing_plain["choice_plain_independently_verified"], false);
    for (index, pointer, replacement) in [
        (0, "/cell/plain_seed", json!(2)),
        (0, "/cell/max_plain_x_trials", json!(999_999)),
        (3, "/base/seed", json!(2)),
        (3, "/reference_columns", json!(1)),
        (3, "/max_x_trials", json!(999_999)),
    ] {
        let mut changed = records.clone();
        *changed[index].pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&changed).is_err(),
            "accepted matched-plain tamper {pointer}"
        );
    }
    let mut wrong_count = records.clone();
    let different = build_plain_control(
        &prepared.public_target.curve(),
        N,
        prepared.canonical_points.len() - 1,
        1,
        1_000_000,
    )
    .unwrap();
    wrong_count[3]["base"] = serde_json::to_value(different).unwrap();
    assert!(audit(&wrong_count).is_err());
    let mut no_reference = records.clone();
    no_reference.remove(2);
    assert!(audit(&no_reference).is_err());
}
use prime31_aux_matched::audit::audit_jsonl;
use prime31_aux_matched::bases::{
    independent_add, independent_scalar, CurveDefinition, PointDefinition,
};
use prime31_aux_matched::collector::{CollectionLimits, PublicTarget, QueryStream};
use prime31_aux_matched::kernel_fibers::{build_fiber, KernelChoice, KernelFiber};
use prime31_aux_matched::oracle::{
    OracleCounters, OracleLimits, OracleResult, OracleStatus, Presentation, SignedRelation,
};
use prime31_aux_matched::replay::Point;
use prime31_aux_matched::target_transport::{prepare, PreparedBase, TargetArm, N, P};
use prime31_aux_matched::GroupOps;
use serde_json::{json, Value};
use std::io::Cursor;
use std::sync::OnceLock;

fn source() -> PublicTarget {
    PublicTarget {
        p: P,
        a: 847_514_846,
        b: 2_087_444_724,
        order: N,
        generator: Point::affine(625_037_715, 2_100_627_251),
        target: Point::affine(625_037_715, 2_100_627_251),
    }
}
fn pd(p: Point) -> PointDefinition {
    PointDefinition {
        x: p.x,
        y: p.y,
        infinity: p.infinity,
    }
}
fn rp(p: PointDefinition) -> Point {
    Point {
        x: p.x,
        y: p.y,
        infinity: p.infinity,
    }
}
fn power(mut x: u64, mut k: u64, p: u64) -> u64 {
    let mut out = 1;
    while k != 0 {
        if k & 1 != 0 {
            out = (out as u128 * x as u128 % p as u128) as u64;
        }
        x = (x as u128 * x as u128 % p as u128) as u64;
        k >>= 1;
    }
    out
}
fn limits() -> CollectionLimits {
    CollectionLimits {
        max_queries: 1,
        max_seconds: 60,
        oracle: OracleLimits {
            max_degree: 1024,
            max_variables: 64,
            timeout_ms: 1000,
            ..Default::default()
        },
    }
}
fn cell(source: &PublicTarget, fiber: &KernelFiber, arm: TargetArm) -> Value {
    json!({
        "schema":"prime31-isogeny-choice.cell/v1","cell_id":"fixed-choice-audit-contract",
        "method":"ic","source_target":source,"kernel":fiber.choice,
        "coset_scalar":fiber.coset_scalar,"target_arm":arm,"arity":2,
        "presentation":if arm == TargetArm::IsogenousTransported { Presentation::Dense } else { Presentation::Tower4 },
        "query_seed":17,"collection":limits()
    })
}
fn encode(records: &[Value]) -> String {
    records
        .iter()
        .map(|v| format!("{}\n", serde_json::to_string(v).unwrap()))
        .collect()
}
fn audit(records: &[Value]) -> Result<Value, String> {
    audit_jsonl(Cursor::new(encode(records)))
}
fn setup(fiber: &KernelFiber, source: &PublicTarget, arm: TargetArm) -> (Vec<Value>, PreparedBase) {
    let prepared = prepare(fiber, source, arm).unwrap();
    (
        vec![
            json!({"event":"choice_start","cell":cell(source, fiber, arm)}),
            json!({"event":"choice_kernel","fiber":fiber}),
            json!({"event":"choice_prepared","base":prepared}),
        ],
        prepared,
    )
}
fn unit_source_and_fiber() -> &'static (PublicTarget, KernelFiber) {
    static DATA: OnceLock<(PublicTarget, KernelFiber)> = OnceLock::new();
    DATA.get_or_init(|| {
        // A non-default coset and a noncyclic kernel exercise both contracts.
        let fiber = build_fiber(KernelChoice::AminusH2, 3, &mut GroupOps::default()).unwrap();
        let mut source = source();
        let base = prepare(&fiber, &source, TargetArm::Original).unwrap();
        let e = CurveDefinition {
            p: source.p,
            a: source.a,
            b: source.b,
        };
        let mut stream = QueryStream::new(17);
        let alpha = stream.scalar(N);
        let beta = stream.scalar(N);
        assert_ne!(beta, 0);
        let twice = independent_add(&e, base.canonical_points[0], base.canonical_points[0]);
        let ag = independent_scalar(&e, pd(source.generator), alpha);
        let neg_ag = PointDefinition {
            x: ag.x,
            y: P - ag.y,
            infinity: false,
        };
        let difference = independent_add(&e, twice, neg_ag);
        source.target = rp(independent_scalar(&e, difference, power(beta, N - 2, N)));
        assert!(!source.target.infinity);
        (source, fiber)
    })
}
fn positive_records(arm: TargetArm) -> Vec<Value> {
    assert_ne!(arm, TargetArm::IsogenousNative);
    let (source, fiber) = unit_source_and_fiber();
    let (mut records, base) = setup(fiber, source, arm);
    let presentation = if arm == TargetArm::Original {
        Presentation::Tower4
    } else {
        Presentation::Dense
    };
    let mut stream = QueryStream::new(17);
    let alpha = stream.scalar(N);
    let beta = stream.scalar(N);
    let (column, sign) = if arm == TargetArm::Original {
        (0, 1)
    } else {
        (
            base.column_transport[0].destination_column,
            base.column_transport[0].sign,
        )
    };
    let mut q = base.canonical_points[column];
    if sign < 0 {
        q.y = P - q.y;
    }
    let target = independent_add(
        &base.curve,
        independent_scalar(&base.curve, pd(base.public_target.generator), alpha),
        independent_scalar(&base.curve, pd(base.public_target.target), beta),
    );
    assert_eq!(target, independent_add(&base.curve, q, q));
    let index = 2 * column + usize::from(sign < 0);
    let mut row = vec![0; base.canonical_points.len()];
    row[column] = 2 * sign;
    let oracle = OracleResult {
        backend_work: None,
        status: OracleStatus::FoundFirst,
        complete: false,
        positive: true,
        presentation,
        summands: 2,
        relations: vec![SignedRelation {
            indices: vec![index, index],
            row: row.clone(),
        }],
        charts: vec![],
        counters: OracleCounters::default(),
        elapsed_ns: 0,
        polynomial_build_ns: 0,
        reason: None,
        accounting: "Synthetic unit-test witness; no oracle execution.".into(),
    };
    records.extend([
        json!({"event":"collection_start","public_target":base.public_target,
            "query_seed":17,"arity":2,"presentation":presentation,"limits":limits(),
            "columns":row.len(),"secret_given_to_collector":false,"planted_query_selection":false}),
        json!({"event":"query_start","query":1,"alpha":alpha,"beta":beta,
            "random_words":stream.words,"target":target}),
        json!({"event":"query_result","query":1,"oracle":oracle}),
        json!({"event":"relation","query":1,"alpha":alpha,"beta":beta,"row":row,
            "signed_points":[q,q],"rank_a":1,"rank_augmented":1,"candidate":null,"independent_replay":true}),
    ]);
    let summary = json!({
        "status":"query_limit","queries":1,"identity_queries":0,"positive_queries":1,
        "complete_negative_queries":0,"unknown_queries":0,"accepted_rows":1,
        "duplicate_coefficient_rows":0,"rank_a":1,"rank_augmented":1,"useful_dependencies":0,
        "independent_replay_rows":1,"recovered_scalar":null,"recovery_replayed":false
    });
    records.push(json!({"event":"collection_summary","summary":summary}));
    records.push(
        json!({"event":"cell_summary","status":"query_limit","recovered":false,
                       "collection":summary,"original_scalar_replay":false}),
    );
    records
}
fn transported_records() -> &'static Vec<Value> {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| positive_records(TargetArm::IsogenousTransported))
}

#[test]
fn all_four_kernel_inventories_are_replayed_independently() {
    for choice in [
        KernelChoice::AplusCyclic,
        KernelChoice::AminusH0,
        KernelChoice::AminusH1,
        KernelChoice::AminusH2,
    ] {
        let fiber = build_fiber(choice, 1, &mut GroupOps::default()).unwrap();
        let (records, _) = setup(&fiber, &source(), TargetArm::Original);
        let result = audit(&records).unwrap();
        assert_eq!(result["choice_kernel_independently_verified"], true);
        assert_eq!(result["choice_prepared_independently_verified"], true);
        assert_eq!(result["stream_complete"], false);
        assert_eq!(result["complete"], false);
    }
}
#[test]
fn source_and_transported_signed_rows_use_the_existing_query_and_rank_auditor() {
    for arm in [TargetArm::Original, TargetArm::IsogenousTransported] {
        let records = positive_records(arm);
        let result = audit(&records).unwrap();
        assert_eq!(result["audit_valid"], true);
        assert_eq!(result["choice_prepared_independently_verified"], true);
        assert_eq!(result["stream_complete"], true);
        assert_eq!(result["complete"], false);
        assert_eq!(result["accepted_rows_replayed"], 1);
        assert_eq!(result["rank_a"], 1);
        assert_eq!(result["rank_augmented"], 1);
        assert_eq!(result["performance_independently_validated"], false);
    }
    let (source, fiber) = unit_source_and_fiber();
    let (records, _) = setup(fiber, source, TargetArm::IsogenousNative);
    assert_eq!(
        audit(&records).unwrap()["choice_prepared_independently_verified"],
        true
    );
}

#[test]
fn kernel_map_coset_and_polynomial_tampering_is_rejected() {
    let original = transported_records();
    for (index, pointer, replacement) in [
        (0, "/cell/kernel", json!("aminus_h0")),
        (0, "/cell/coset_scalar", json!(5)),
        (1, "/fiber/coset_point/x", json!(0)),
        (1, "/fiber/kernel_generators/0/y", json!(0)),
        (1, "/fiber/kernel_points/1/x", json!(0)),
        (1, "/fiber/steps/9/tau", json!(0)),
        (1, "/fiber/tower/terminal", json!(0)),
        (1, "/fiber/certificate/numerator/0", json!(0)),
        (1, "/fiber/certificate/kernel_x_poles_checked", json!(0)),
        (1, "/fiber/endpoint/y", json!(0)),
    ] {
        let mut changed = original.clone();
        *changed[index].pointer_mut(pointer).unwrap() = replacement;
        assert!(audit(&changed).is_err(), "accepted tamper {pointer}");
    }
}
#[test]
fn destination_points_membership_and_signed_permutation_tampering_is_rejected() {
    let original = transported_records();
    for (index, pointer, replacement) in [
        (0, "/cell/target_arm", json!("original")),
        (0, "/cell/source_target/target/x", json!(0)),
        (2, "/base/arm", json!("isogenous_native")),
        (2, "/base/public_target/generator/y", json!(0)),
        (2, "/base/public_target/target/x", json!(0)),
        (2, "/base/canonical_points/0/y", json!(0)),
        (2, "/base/column_transport/0/sign", json!(0)),
        (2, "/base/column_transport/0/source_column", json!(1)),
        (2, "/base/duplicate_image_values", json!(999)),
        (4, "/alpha", json!(0)),
        (6, "/row/0", json!(99)),
        (6, "/rank_a", json!(0)),
    ] {
        let mut changed = original.clone();
        *changed[index].pointer_mut(pointer).unwrap() = replacement;
        assert!(audit(&changed).is_err(), "accepted tamper {pointer}");
    }
    let mut truncated = original.clone();
    truncated[2]["base"]["v"].as_array_mut().unwrap().pop();
    assert!(audit(&truncated).is_err());
    let mut dropped_column = original.clone();
    dropped_column[2]["base"]["canonical_points"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(audit(&dropped_column).is_err());
}
#[test]
fn choice_event_order_and_truncation_remain_fail_closed() {
    let original = transported_records();
    for end in [1, 2, 3, 5, original.len() - 1] {
        let result = audit(&original[..end]).unwrap();
        assert_eq!(result["stream_complete"], false);
        assert_eq!(result["complete"], false);
    }
    let mut truncated_line = encode(&original[..3]);
    truncated_line.push_str("{\"event\":\"collection_start\",");
    assert_eq!(
        audit_jsonl(Cursor::new(truncated_line)).unwrap()["trailing_partial_json"],
        true
    );
    let mut missing_kernel = original.clone();
    missing_kernel.remove(1);
    assert!(audit(&missing_kernel).is_err());
    let mut missing_base = original.clone();
    missing_base.remove(2);
    assert!(audit(&missing_base).is_err());
    let mut repeated = original.clone();
    repeated.insert(3, original[2].clone());
    assert!(audit(&repeated).is_err());
    let mut non_dense = original.clone();
    non_dense[0]["cell"]["presentation"] = json!("tower4");
    assert!(audit(&non_dense).is_err());
    let mut old_base_event = original.clone();
    old_base_event[2]["event"] = json!("selected_base");
    assert!(audit(&old_base_event).is_err());

    // A driver failure closes the receipt without asserting collection or
    // scalar success. It remains valid even when the last query was pending.
    for end in [1, 3, 5, original.len() - 1] {
        let mut failed = original[..end].to_vec();
        failed.push(json!({"event":"driver_error","error":"synthetic failure"}));
        let result = audit(&failed).unwrap();
        assert_eq!(result["stream_complete"], true);
        assert_eq!(result["terminal_record_present"], true);
        assert_eq!(result["complete"], false);
        assert_eq!(result["recovered_scalar"], Value::Null);
        failed.push(original.last().unwrap().clone());
        assert!(
            audit(&failed).is_err(),
            "accepted success after driver failure"
        );
    }
    assert!(audit(&[json!({"event":"driver_error","error":"no frozen cell"})]).is_err());
}
#[test]
fn rho_receipt_checks_the_frozen_mapped_target_and_original_scalar() {
    for arm in [TargetArm::Original, TargetArm::IsogenousNative] {
        let source = source();
        let mapped = if arm == TargetArm::Original {
            source.clone()
        } else {
            PublicTarget {
                p: P,
                a: 323_891_735,
                b: 1_985_248_359,
                order: N,
                generator: Point::affine(1_767_020_261, 634_125_456),
                target: Point::affine(1_767_020_261, 634_125_456),
            }
        };
        let start = json!({"event":"choice_start","cell":{
            "schema":"prime31-isogeny-choice.cell/v1","cell_id":"synthetic-rho-audit",
            "method":"rho_v6","source_target":source,"target_arm":arm,
            "kernel":"aplus_cyclic","coset_scalar":1,"presentation":"dense",
            "arity":2,"query_seed":17,"collection":limits()
        }});
        let records = vec![
            start,
            json!({"event":"rho_summary","result":{"recovered":1},
            "independent_replay":true,"public_target":mapped}),
        ];
        let result = audit(&records).unwrap();
        assert_eq!(result["complete"], true);
        assert_eq!(result["source_public_scalar_equation_verified"], true);
        assert_eq!(result["choice_kernel_independently_verified"], false);
        let mut wrong_target = records.clone();
        wrong_target[1]["public_target"]["generator"]["x"] = json!(0);
        assert!(audit(&wrong_target).is_err());
        let mut wrong_scalar = records.clone();
        wrong_scalar[1]["result"]["recovered"] = json!(2);
        assert!(audit(&wrong_scalar).is_err());
        let mut wrong_method = records.clone();
        wrong_method[0]["cell"]["method"] = json!("ic");
        assert!(audit(&wrong_method).is_err());
    }
}
