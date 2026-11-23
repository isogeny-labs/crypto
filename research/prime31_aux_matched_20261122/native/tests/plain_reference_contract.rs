//! Fixed arithmetic and transcript contracts only. No relation solver, rho
//! search, timing sample, or comparison campaign is executed by these tests.

use prime31_aux_matched::audit::audit_jsonl;
use prime31_aux_matched::bases::build_plain_control;
use prime31_aux_matched::collector::{CollectionLimits, PublicTarget, QueryStream};
use prime31_aux_matched::kernel_fibers::{
    build_fiber_with_factorization, KernelChoice, KernelFactorization,
};
use prime31_aux_matched::oracle::{DomainRoots, OracleCounters, OracleLimits, Presentation};
use prime31_aux_matched::plain_reference::{frozen_plain_reference, PlainReferenceBinding};
use prime31_aux_matched::replay::Point;
use prime31_aux_matched::target_transport::{prepare, TargetArm, N, P};
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
fn target(arm: TargetArm) -> PublicTarget {
    if arm == TargetArm::Original {
        source()
    } else {
        PublicTarget {
            p: P,
            a: 323_891_735,
            b: 1_985_248_359,
            order: N,
            generator: Point::affine(1_767_020_261, 634_125_456),
            target: Point::affine(1_767_020_261, 634_125_456),
        }
    }
}
fn limits(queries: u64) -> CollectionLimits {
    CollectionLimits {
        max_queries: queries,
        max_seconds: 110,
        oracle: OracleLimits {
            max_degree: 1024,
            max_pairs_per_step: 32,
            timeout_ms: 2000,
            max_solutions: 4096,
            max_signed_lift_attempts: 1_000_000,
            stop_after_first: true,
            max_variables: 48,
            max_input_terms: 100_000,
            domain_roots: DomainRoots::Auto,
            sibling_batch: true,
        },
    }
}
fn cell(binding: &PlainReferenceBinding) -> Value {
    json!({
        "schema":"prime31-isogeny-choice.cell/v2", "cell_id":"fixed-plain-reference-contract",
        "method":"ic", "stage":"screening", "source_target":source(),
        "kernel":binding.kernel, "factorization":binding.factorization,
        "coset_scalar":binding.coset_scalar, "target_arm":binding.target_arm,
        "arity":3, "presentation":"dense", "plain_control":true,
        "plain_seed":1, "max_plain_x_trials":1_000_000, "plain_reference":binding,
        "query_seed":4188, "collection":limits(64), "execution_seconds":120,
        "memory_mib":4096, "round":0, "rho_max_steps":2_000_000, "rho_seed":4188
    })
}
fn reference_records(binding: &PlainReferenceBinding) -> Vec<Value> {
    vec![
        json!({"event":"choice_start","cell":cell(binding)}),
        json!({"event":"choice_plain_reference","reference":binding,
            "study_reference_preparation_seconds_diagnostic":null,
            "study_reference_preparation_cost":null,
            "preparation_cost_status":"historical_matching_work_unmeasured_not_zero"}),
    ]
}
fn audit(records: &[Value]) -> Result<Value, String> {
    let mut bytes = Vec::new();
    for record in records {
        serde_json::to_writer(&mut bytes, record).unwrap();
        bytes.push(b'\n');
    }
    audit_jsonl(Cursor::new(bytes))
}
fn plain_records() -> &'static Vec<Value> {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let reference = frozen_plain_reference(
            KernelChoice::AminusH2,
            1,
            KernelFactorization::FullTwoTorsionFirst,
            TargetArm::IsogenousNative,
        )
        .unwrap();
        let mut records = reference_records(&reference);
        let public = target(reference.target_arm);
        // No auxiliary builder is used to prepare this plain-control fixture.
        let plain =
            build_plain_control(&public.curve(), N, reference.columns, 1, 1_000_000).unwrap();
        records.push(
            json!({"event":"choice_plain","base":plain,"public_target":public,
            "seed":1,"max_x_trials":1_000_000,"reference_columns":reference.columns,
            "reference_setup_seconds_diagnostic":null}),
        );
        records
    })
}
fn collection_start(cell: &Value, columns: usize) -> Value {
    let arm: TargetArm = serde_json::from_value(cell["target_arm"].clone()).unwrap();
    let mut record = json!({"event":"collection_start","public_target":target(arm),
        "query_seed":cell["query_seed"],"arity":3,"presentation":Presentation::Dense,
        "limits":cell["collection"],"columns":columns,
        "secret_given_to_collector":false,"planted_query_selection":false});
    if cell["oracle_backend"] == "pair_table" {
        record["oracle_backend"] = json!("pair_table");
    }
    record
}

fn synthetic_pair_setup(columns: usize) -> Value {
    // Deliberately fabricated report metadata: the witness auditor checks its
    // shape and binding, not table contents, operation counts, or performance.
    let attempts = columns * columns;
    json!({"event":"choice_pair_setup","setup":{
        "backend":"pair_table","mode":"batched","columns":columns,
        "signed_points":2*columns,"abscissae":columns,"table_kind":"negation_folded",
        "table_entries":1,"insert_attempts":attempts,"index_lookups":columns,
        "requested_hash_entry_capacity":attempts+1,
        "logical_entry_payload_bytes":16,"requested_payload_bytes":16*(attempts+1),
        "first_witness_policy":"cancellation_first_then_signed_index_order"
    }})
}

fn pair_collection_records() -> Vec<Value> {
    let mut records = plain_records().clone();
    records[0]["cell"]["oracle_backend"] = json!("pair_table");
    records.push(synthetic_pair_setup(533));
    records.push(collection_start(&records[0]["cell"], 533));
    records
}

#[test]
fn frozen_counts_bind_the_complete_coset_and_each_target_arm() {
    for (kernel, original, native) in [
        (KernelChoice::AplusCyclic, 518, 506),
        (KernelChoice::AminusH0, 508, 508),
        (KernelChoice::AminusH1, 512, 519),
        (KernelChoice::AminusH2, 515, 533),
    ] {
        for arm in [
            TargetArm::Original,
            TargetArm::IsogenousNative,
            TargetArm::IsogenousTransported,
        ] {
            let reference =
                frozen_plain_reference(kernel, 1, KernelFactorization::Standard, arm).unwrap();
            assert_eq!(
                reference.columns,
                if arm == TargetArm::IsogenousNative {
                    native
                } else {
                    original
                }
            );
            let result = audit(&reference_records(&reference)).unwrap();
            assert_eq!(
                result["choice_plain_reference_independently_verified"],
                true
            );
            assert_eq!(result["choice_kernel_independently_verified"], false);
            assert_eq!(result["choice_prepared_independently_verified"], false);
            assert_eq!(result["choice_plain_independently_verified"], false);
            assert_eq!(result["complete"], false);
        }
    }
    assert!(frozen_plain_reference(
        KernelChoice::AminusH2,
        3,
        KernelFactorization::Standard,
        TargetArm::Original
    )
    .is_err());
    assert!(frozen_plain_reference(
        KernelChoice::AminusH0,
        1,
        KernelFactorization::FullTwoTorsionFirst,
        TargetArm::Original
    )
    .is_err());
}

#[test]
fn plain_candidate_uses_its_own_base_without_rebuilding_the_reference() {
    let mut records = plain_records().clone();
    let result = audit(&records).unwrap();
    assert_eq!(
        result["choice_plain_reference_independently_verified"],
        true
    );
    assert_eq!(result["choice_plain_independently_verified"], true);
    assert_eq!(result["choice_kernel_independently_verified"], false);
    assert_eq!(result["choice_prepared_independently_verified"], false);
    assert_eq!(result["performance_independently_validated"], false);
    records.push(collection_start(&records[0]["cell"], 533));
    assert_eq!(audit(&records).unwrap()["complete"], false);
}

#[test]
fn a_self_consistent_metadata_lie_cannot_fake_cardinality_matching() {
    let original = plain_records();
    for (field, replacement) in [
        ("columns", json!(1)),
        ("fixture_sha256", json!("0".repeat(64))),
        ("coset_scalar", json!(3)),
        ("kernel", json!("aminus_h0")),
        ("target_arm", json!("original")),
    ] {
        let mut changed = original[..2].to_vec();
        changed[0]["cell"]["plain_reference"][field] = replacement.clone();
        changed[1]["reference"][field] = replacement.clone();
        if matches!(field, "kernel" | "coset_scalar" | "target_arm") {
            changed[0]["cell"][field] = replacement;
        }
        assert!(
            audit(&changed).is_err(),
            "accepted self-consistent reference tamper: {field}"
        );
    }
    let mut swapped_route = original[..2].to_vec();
    swapped_route[1]["reference"]["factorization"] = json!("standard");
    assert!(audit(&swapped_route).is_err());
    let mut changed_point = original.clone();
    changed_point[2]["base"]["canonical_points"][0]["y"] = json!(0);
    assert!(audit(&changed_point).is_err());
}

#[test]
fn v2_plain_reference_order_is_distinct_from_legacy_construction_receipts() {
    let original = plain_records();
    for event in ["choice_kernel", "choice_prepared"] {
        let mut mixed = original[..1].to_vec();
        mixed.push(json!({"event":event,"fiber":null,"base":null}));
        assert!(audit(&mixed).is_err());
    }
    let mut absent = original.clone();
    absent.remove(1);
    assert!(audit(&absent).is_err());
    let mut repeated = original.clone();
    repeated.insert(2, original[1].clone());
    assert!(audit(&repeated).is_err());
    let mut legacy = original.clone();
    legacy[0]["cell"]["schema"] = json!("prime31-isogeny-choice.cell/v1");
    assert!(audit(&legacy).is_err());
}

#[test]
fn screening_and_recovery_attempts_bind_the_first_hit_strategy_and_caps() {
    let start = plain_records()[0].clone();
    assert_eq!(audit(&[start.clone()]).unwrap()["complete"], false);
    let mut recovery = start.clone();
    recovery["cell"]["stage"] = json!("complete_recovery");
    recovery["cell"]["collection"]["max_queries"] = json!(65_536);
    assert_eq!(audit(&[recovery]).unwrap()["complete"], false);
    for (pointer, replacement) in [
        ("/cell/stage", Value::Null),
        ("/cell/stage", json!("complete_recovery")),
        ("/cell/collection/max_queries", json!(65_536)),
        ("/cell/collection/oracle/stop_after_first", json!(false)),
        ("/cell/collection/max_seconds", json!(111)),
        ("/cell/execution_seconds", json!(121)),
        ("/cell/memory_mib", json!(4097)),
    ] {
        let mut changed = start.clone();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&[changed]).is_err(),
            "accepted stage/cap tamper: {pointer}"
        );
    }
}

#[test]
fn kernel_factorization_is_bound_to_the_actual_coordinate_prefix() {
    for factorization in [
        KernelFactorization::Standard,
        KernelFactorization::FullTwoTorsionFirst,
    ] {
        let fiber = build_fiber_with_factorization(
            KernelChoice::AminusH2,
            1,
            factorization,
            &mut GroupOps::default(),
        )
        .unwrap();
        let prepared = prepare(&fiber, &source(), TargetArm::Original).unwrap();
        let reference = frozen_plain_reference(
            KernelChoice::AminusH2,
            1,
            factorization,
            TargetArm::Original,
        )
        .unwrap();
        let mut input = cell(&reference);
        input["plain_control"] = json!(false);
        input["plain_reference"] = Value::Null;
        input["presentation"] = json!("tower4");
        let records = vec![
            json!({"event":"choice_start","cell":input}),
            json!({"event":"choice_kernel","fiber":fiber}),
            json!({"event":"choice_prepared","base":prepared}),
        ];
        assert_eq!(
            audit(&records).unwrap()["choice_kernel_independently_verified"],
            true
        );
        let other = if factorization == KernelFactorization::Standard {
            "full_two_torsion_first"
        } else {
            "standard"
        };
        let mut false_label = records.clone();
        false_label[0]["cell"]["factorization"] = json!(other);
        false_label[1]["fiber"]["factorization"] = json!(other);
        assert!(
            audit(&false_label).is_err(),
            "accepted a mutually consistent false route label"
        );
    }
}

#[test]
fn pair_setup_requires_the_selected_base_and_frozen_batched_backend() {
    let original = pair_collection_records();
    let result = audit(&original).unwrap();
    assert_eq!(result["choice_pair_setup_checked"], true);
    assert_eq!(result["complete"], false);
    assert_eq!(result["performance_independently_validated"], false);
    let mut absent = original.clone();
    absent.remove(3);
    assert!(audit(&absent).is_err());
    let mut premature = original[..2].to_vec();
    premature.push(original[3].clone());
    assert!(audit(&premature).is_err());
    let mut repeated = original.clone();
    repeated.insert(4, original[3].clone());
    assert!(audit(&repeated).is_err());
    for (index, pointer, replacement) in [
        (0, "/cell/oracle_backend", json!("polynomial")),
        (0, "/cell/presentation", json!("tower4")),
        (3, "/setup/mode", json!("scalar")),
        (3, "/setup/columns", json!(532)),
        (3, "/setup/insert_attempts", json!(1)),
        (3, "/setup/table_entries", json!(0)),
        (3, "/setup/requested_payload_bytes", json!(0)),
        (4, "/oracle_backend", json!("polynomial")),
    ] {
        let mut changed = original.clone();
        *changed[index].pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&changed).is_err(),
            "accepted pair-setup tamper: {pointer}"
        );
    }
}

#[test]
fn pair_query_results_cannot_be_relabelled_as_polynomial_or_scalar_probes() {
    let mut records = pair_collection_records();
    let public = target(TargetArm::IsogenousNative);
    let mut stream = QueryStream::new(4188);
    let alpha = stream.scalar(N);
    let beta = stream.scalar(N);
    let replay = public.replay_curve();
    let query = replay
        .add(
            replay.scalar(public.generator, alpha).unwrap(),
            replay.scalar(public.target, beta).unwrap(),
        )
        .unwrap();
    records.push(
        json!({"event":"query_start","query":1,"alpha":alpha,"beta":beta,
        "target":query,"random_words":stream.words}),
    );
    records.push(json!({"event":"query_result","query":1,"oracle":{
        "status":"unknown_timeout","complete":false,"positive":false,
        "presentation":"dense","summands":3,"relations":[],"charts":[],
        "counters":OracleCounters::default(),"elapsed_ns":0,"polynomial_build_ns":0,
        "reason":"synthetic unit-test timeout; no pair query executed",
        "accounting":"No measured work in this constructed receipt.",
        "backend_work":{"backend":"pair_table","mode":"batched"}
    }}));
    let result = audit(&records).unwrap();
    assert_eq!(result["unknown_queries"], 1);
    assert_eq!(result["complete"], false);
    assert_eq!(result["negative_exhaustion_independently_replayed"], false);
    let last = records.len() - 1;
    for (pointer, replacement) in [
        ("/oracle/backend_work", Value::Null),
        ("/oracle/backend_work/backend", json!("polynomial")),
        ("/oracle/backend_work/mode", json!("scalar")),
        ("/oracle/counters/f4_runs", json!(1)),
    ] {
        let mut changed = records.clone();
        *changed[last].pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&changed).is_err(),
            "accepted pair-result tamper: {pointer}"
        );
    }
}
