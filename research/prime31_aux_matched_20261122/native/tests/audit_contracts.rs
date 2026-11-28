use prime31_aux_matched::audit::audit_jsonl;
use prime31_aux_matched::bases::build_plain_control;
use prime31_aux_matched::collector::{collect, CollectionLimits, PublicTarget};
use prime31_aux_matched::oracle::{OracleLimits, Presentation};
use prime31_aux_matched::replay::{Curve, Point};
use serde_json::{json, Value};
use std::io::Cursor;
use std::sync::OnceLock;

// Fixed correctness fixture only. The public target is prepared here; the
// collector receives no scalar, and its query sequence is never filtered.
fn fixture(max_queries: u64) -> Vec<Value> {
    let curve = Curve { p: 127, a: 1, b: 7 };
    let generator = Point::affine(1, 3);
    let target = PublicTarget {
        p: 127,
        a: 1,
        b: 7,
        order: 109,
        generator,
        target: curve.scalar(generator, 37).unwrap(),
    };
    let base = build_plain_control(&target.curve(), 109, 1, 1, 127).unwrap();
    assert_eq!(
        (base.canonical_points[0].x, base.canonical_points[0].y),
        (1, 3)
    );
    let fb = base.factor_base().unwrap();
    let mut records = vec![
        json!({"event":"cell_start","cell":{
            "cell_id":"offline-audit-fixed-toy", "public_target":target,
            "arity":2,"presentation":"dense","query_seed":2
        }}),
        json!({"event":"selected_base","base":base,
            "reference_columns":1,"matched_columns":1}),
    ];
    let limits = CollectionLimits {
        max_queries,
        max_seconds: 60,
        oracle: OracleLimits {
            timeout_ms: 1000,
            max_degree: 8,
            stop_after_first: true,
            ..Default::default()
        },
    };
    let summary = collect(
        &target,
        &fb,
        &base.v,
        None,
        2,
        Presentation::Dense,
        2,
        &limits,
        &mut |value| {
            records.push(value.clone());
            Ok(())
        },
    )
    .unwrap();
    records.push(json!({"event":"cell_summary","status":summary.status,
        "recovered":summary.recovery_replayed,"collection":summary}));
    records
}

fn recovered_fixture() -> &'static Vec<Value> {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| fixture(80))
}

fn encode(records: &[Value]) -> String {
    let mut text = String::new();
    for record in records {
        text.push_str(&serde_json::to_string(record).unwrap());
        text.push('\n');
    }
    text
}

fn audit(records: &[Value]) -> Result<Value, String> {
    audit_jsonl(Cursor::new(encode(records)))
}

#[test]
fn offline_replay_verifies_natural_rows_and_useful_duplicate_dependency() {
    let records = recovered_fixture();
    let result = audit(records).unwrap();
    assert_eq!(result["audit_valid"], true);
    assert_eq!(result["complete"], true);
    assert_eq!(result["stream_complete"], true);
    assert_eq!(result["recovered_scalar"], 37);
    assert_eq!(result["rank_a"], 1);
    assert_eq!(result["rank_augmented"], 2);
    assert_eq!(result["accepted_rows_replayed"], 2);
    assert_eq!(result["duplicate_coefficient_rows"], 1);
    assert_eq!(result["useful_dependencies"], 1);
    assert_eq!(result["queries_replayed"], 55);
    assert_eq!(result["positive_queries"], 2);
    assert_eq!(result["negative_exhaustion_independently_replayed"], false);
    assert_eq!(result["performance_independently_validated"], false);
    let rows: Vec<_> = records
        .iter()
        .filter(|v| v["event"] == "relation")
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["row"], json!([-2]));
    assert_eq!(rows[1]["row"], rows[0]["row"]);
    assert_ne!(rows[0]["beta"], rows[1]["beta"]);
    assert_eq!(rows[0]["candidate"], Value::Null);
    assert_eq!(rows[1]["candidate"], 37);
}

#[test]
fn tampered_points_labels_rows_ranks_and_scalar_are_rejected() {
    let original = recovered_fixture();
    let relation = original
        .iter()
        .position(|v| v["event"] == "relation")
        .unwrap();
    let final_relation = original
        .iter()
        .rposition(|v| v["event"] == "relation")
        .unwrap();
    let query = original
        .iter()
        .position(|v| v["event"] == "query_start")
        .unwrap();
    let summary = original
        .iter()
        .position(|v| v["event"] == "collection_summary")
        .unwrap();
    let positive_oracle = original
        .iter()
        .position(|v| v["event"] == "query_result" && v["oracle"]["positive"] == true)
        .unwrap();

    let mutations = [
        (relation, "/row/0", json!(2)),
        (relation, "/alpha", json!(0)),
        (relation, "/rank_a", json!(0)),
        (relation, "/signed_points/0/y", json!(3)),
        (final_relation, "/candidate", json!(38)),
        (query, "/alpha", json!(0)),
        (query, "/target/x", json!(127)),
        (query, "/random_words", json!(99)),
        (summary, "/summary/recovered_scalar", json!(38)),
        (summary, "/summary/rank_augmented", json!(1)),
        (summary, "/summary/duplicate_coefficient_rows", json!(0)),
        (positive_oracle, "/oracle/relations/0/indices/0", json!(0)),
        (1, "/base/canonical_points/0/y", json!(124)),
    ];
    for (event, pointer, replacement) in mutations {
        let mut altered = original.clone();
        *altered[event].pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&altered).is_err(),
            "accepted tamper at event {event} {pointer}"
        );
    }
}

#[test]
fn interrupted_and_capped_streams_remain_valid_partial_audits() {
    let records = recovered_fixture();
    let result = audit(&records[..records.len() - 1]).unwrap();
    assert_eq!(result["complete"], false);
    assert_eq!(result["stream_complete"], false);
    assert_eq!(result["recovered_scalar"], 37);
    assert_eq!(result["public_scalar_equation_verified"], true);

    let first_start = records
        .iter()
        .position(|v| v["event"] == "query_start")
        .unwrap();
    let prefix = &records[..=first_start];
    let result = audit(prefix).unwrap();
    assert_eq!(result["complete"], false);
    assert_eq!(result["queries_replayed"], 1);
    assert_eq!(result["pending_query"], true);
    let mut truncated_json = encode(prefix);
    truncated_json.push_str("{\"event\":\"query_result\",\"oracle\":");
    let result = audit_jsonl(Cursor::new(truncated_json)).unwrap();
    assert_eq!(result["trailing_partial_json"], true);
    assert_eq!(result["complete"], false);

    let capped = fixture(1);
    let result = audit(&capped).unwrap();
    assert_eq!(result["audit_valid"], true);
    assert_eq!(result["stream_complete"], true);
    assert_eq!(result["complete"], false);
    assert_eq!(result["recovered_scalar"], Value::Null);
    assert_eq!(result["queries_replayed"], 1);
    assert_eq!(result["reported_complete_negative_queries"], 1);
}

#[test]
fn event_order_and_unresolved_query_summary_tampering_are_rejected() {
    let original = recovered_fixture();
    let start = original
        .iter()
        .position(|v| v["event"] == "query_start")
        .unwrap();
    let result = original
        .iter()
        .position(|v| v["event"] == "query_result")
        .unwrap();
    let mut omitted = original.clone();
    omitted.remove(result);
    assert!(audit(&omitted).is_err());
    let mut omitted = original.clone();
    omitted.remove(start);
    assert!(audit(&omitted).is_err());

    let mut repeated = original.clone();
    repeated.push(original[0].clone());
    assert!(audit(&repeated).is_err());
    let mut malformed = encode(&original[..=start]);
    malformed.push_str("{\"event\":\n");
    assert!(audit_jsonl(Cursor::new(malformed)).is_err());
    assert!(audit_jsonl(Cursor::new("\n")).is_err());
}

#[test]
fn frozen_reference_base_and_collection_caps_are_bound_to_the_cell() {
    let mut records = recovered_fixture().clone();
    let limits = records
        .iter()
        .find(|v| v["event"] == "collection_start")
        .unwrap()["limits"]
        .clone();
    let cell = records[0]["cell"].as_object_mut().unwrap();
    cell.insert("reference_family".into(), json!("plain"));
    cell.insert("depth".into(), json!(0));
    cell.insert("base_seed".into(), json!(1));
    cell.insert("plain_control".into(), json!(false));
    cell.insert("collection".into(), limits);
    let reference = records[1]["base"].clone();
    records.insert(
        1,
        json!({"event":"reference_build_result","base":reference}),
    );
    assert_eq!(audit(&records).unwrap()["complete"], true);
    for (index, pointer, replacement) in [
        (1, "/base/seed", json!(2)),
        (2, "/base/seed", json!(2)),
        (0, "/cell/reference_family", json!("ss_j1728")),
        (0, "/cell/collection/max_queries", json!(81)),
        (0, "/cell/collection/oracle/max_degree", json!(20)),
    ] {
        let mut altered = records.clone();
        *altered[index].pointer_mut(pointer).unwrap() = replacement;
        assert!(
            audit(&altered).is_err(),
            "accepted frozen-input tamper {pointer}"
        );
    }
    records.remove(1);
    assert!(
        audit(&records).is_err(),
        "selected arm hid its reference construction"
    );
}
