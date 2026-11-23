//! Adversarial controls for the v2 exporter. These are source artifacts only
//! in this packet; the required Rust execution remains separately admitted.
use super::*;

fn pin(tag: &str) -> FilePin {
    FilePin {
        path: format!("/tmp/{tag}"),
        sha256: "a".repeat(64),
        bytes: 1,
    }
}

fn slot(session: &str, sequence: u64, workload: &str) -> Slot {
    Slot {
        plan_name: "plan-16.json".into(),
        plan_sha256: PLAN_INVENTORY[0].1.into(),
        session_id: session.into(),
        sequence,
        round: 4,
        warmup: false,
        arm: "pencil".into(),
        role: "candidate".into(),
        algorithm_seed: 17,
        workload: json!({
            "workload_id": workload,
            "workload_sha256": "b".repeat(64),
            "curve_spec": {"p": 739, "r": 739}
        }),
        method: json!({"id":"ic.three_leads_genus2_pencil_symbolic_v2"}),
    }
}

fn attempt(session: &str, sequence: u64, attempt: u64) -> AttemptEvidence {
    AttemptEvidence {
        session_id: session.into(),
        sequence,
        attempt,
        retry_of: None,
        context: pin(&format!("context-{sequence}-{attempt}")),
        ordinary_record: None,
        progress_files: vec![],
        progress_totals: None,
        diagnostics_attachment: None,
        relation: None,
        resources: None,
        session_snapshot: None,
    }
}

#[test]
fn partial_rows_do_not_change_the_frozen_expected_set() {
    let mut expected = BTreeMap::new();
    expected.insert(
        SlotKey {
            session_id: "s".into(),
            sequence: 0,
        },
        slot("s", 0, "w0"),
    );
    expected.insert(
        SlotKey {
            session_id: "s".into(),
            sequence: 1,
        },
        slot("s", 1, "w1"),
    );
    assert!(validate_retry_links(&expected, &[attempt("s",0,0)]).is_ok());
    assert_eq!(expected.len(),2); // Enrollment is immutable; missing row remains a separate slot.
}

#[test]
fn retry_requires_present_same_slot_attempt_zero_parent() {
    let mut expected = BTreeMap::new();
    expected.insert(
        SlotKey {
            session_id: "s".into(),
            sequence: 0,
        },
        slot("s", 0, "w0"),
    );
    let primary = attempt("s", 0, 0);
    let mut orphan = attempt("s", 0, 1);
    assert_eq!(
        validate_retry_links(&expected, &[primary.clone(), orphan.clone()]).unwrap_err(),
        "retry lacks explicit primary parent"
    );
    orphan.retry_of = Some(AttemptRef {
        session_id: "s".into(),
        sequence: 9,
        attempt: 0,
    });
    assert_eq!(
        validate_retry_links(&expected, &[primary.clone(), orphan.clone()]).unwrap_err(),
        "retry parent crosses slot or is not attempt zero"
    );
    orphan.retry_of = Some(AttemptRef {
        session_id: "s".into(),
        sequence: 0,
        attempt: 0,
    });
    orphan.session_id = "retry-s".into();
    assert!(validate_retry_links(&expected, &[primary, orphan]).is_ok());
}

#[test]
fn same_seed_different_workload_or_plan_never_forms_one_triplet() {
    let a = triplet_key(&slot("s", 0, "w0")).unwrap();
    let mut other_workload = slot("s", 1, "w1");
    other_workload.algorithm_seed = 17;
    let b = triplet_key(&other_workload).unwrap();
    let mut other_plan = slot("s2", 0, "w0");
    other_plan.algorithm_seed = 17;
    other_plan.plan_name = "plan-20.json".into();
    other_plan.plan_sha256 = PLAN_INVENTORY[1].1.into();
    let c = triplet_key(&other_plan).unwrap();
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(pair_id(&a).unwrap(), pair_id(&b).unwrap());
    assert_ne!(pair_id(&a).unwrap(), pair_id(&c).unwrap());
}

#[test]
fn caller_eligibility_and_cost_injection_are_unknown_fields() {
    let value = json!({
        "session_id":"s",
        "sequence":0,
        "attempt":0,
        "retry_of":null,
        "context":pin("context"),
        "ordinary_record":null,
        "progress_files":[],
        "progress_totals":null,
        "diagnostics_attachment":null,
        "relation":null,
        "resources":null,
        "session_snapshot":null,
        "eligible_for_pair":true,
        "raw_costs":{"ratio":0}
    });
    let error = serde_json::from_value::<AttemptEvidence>(value).unwrap_err();
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn null_or_partial_nine_category_rows_are_not_admitted() {
    let null_cells = (0..36)
        .map(|_| json!({"candidate":null,"reference":null,"ratio":null}))
        .collect::<Vec<_>>();
    assert!(!mapped_price_is_complete(&json!({
        "coverage_blockers":["missing counter"],
        "cells":null_cells
    }))
    .unwrap());
    let mut cells = (0..36)
        .map(|_| json!({
            "candidate":{"numerator":"1","denominator":"1"},
            "reference":{"numerator":"1","denominator":"1"},
            "ratio":{"numerator":"1","denominator":"1"}
        }))
        .collect::<Vec<_>>();
    cells[7]["ratio"] = Value::Null;
    assert!(!mapped_price_is_complete(&json!({
        "coverage_blockers":[],
        "cells":cells
    }))
    .unwrap());
}

#[test]
fn changed_file_bytes_do_not_match_a_retained_pin() {
    let directory = std::env::temp_dir().join(format!(
        "retained-attempt-v2-control-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir(&directory).unwrap();
    let path = directory.join("evidence.json");
    fs::write(&path, b"{\"schema\":\"fixture/v1\"}").unwrap();
    let wrong = FilePin {
        path: path.to_str().unwrap().into(),
        sha256: "0".repeat(64),
        bytes: fs::metadata(&path).unwrap().len(),
    };
    assert!(checked_file(&wrong, MAX_JSON_BYTES).is_err());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn distinct_retry_resource_snapshot_uses_primary_plan_not_primary_session() {
    let scheduled=slot("primary",0,"w0"); let mut retry=attempt("retry-session",0,1);
    retry.retry_of=Some(AttemptRef{session_id:"primary".into(),sequence:0,attempt:0});
    let snapshot=json!({"schema":"ecbench.session/v1","session_id":"retry-session",
        "plan_sha256":scheduled.plan_sha256,"binary_sha256":"binary","status":"running",
        "executions_planned":3240,"records_written":1,"last_seq":0});
    assert!(validate_session_snapshot(&snapshot,&retry,&scheduled,"binary").is_ok());
    let mut tampered=snapshot.clone();tampered["plan_sha256"]=json!(PLAN_INVENTORY[1].1);
    assert!(validate_session_snapshot(&tampered,&retry,&scheduled,"binary").is_err());
    let mut wrong_session=snapshot;wrong_session["session_id"]=json!("primary");
    assert!(validate_session_snapshot(&wrong_session,&retry,&scheduled,"binary").is_err());
}
#[test]
fn partial_primary_and_retry_sessions_keep_expected_slots() {
    let mut slots=BTreeMap::new();
    for sequence in 0..3240 { slots.insert(SlotKey{session_id:"s".into(),sequence},slot("s",sequence,"w")); }
    let mut custody=BTreeMap::new();custody.insert("s".into(),(false,PLAN_INVENTORY[0].1.into()));
    custody.insert("retry".into(),(false,PLAN_INVENTORY[0].1.into()));
    let mut retry=attempt("retry",0,1);retry.retry_of=Some(AttemptRef{session_id:"s".into(),sequence:0,attempt:0});
    let rows=vec![attempt("s",0,0),retry];
    assert!(validate_retry_links(&slots,&rows).is_ok());
    assert!(validate_session_attempt_coverage(&slots,&rows,&custody).is_ok());
    assert_eq!(slots.len(),3240);
    let mut bad=rows.clone();bad[1].sequence=3240;bad[1].retry_of.as_mut().unwrap().sequence=3240;
    assert!(validate_retry_links(&slots,&bad).is_err());
}
#[test]
fn missing_custody_or_diagnostics_prevents_comparison_even_with_priced_data() {
    assert!(required_evidence_blockers(true,true).is_empty());
    assert_eq!(required_evidence_blockers(false,true),vec!["owner_custody_not_closed"]);
    assert_eq!(required_evidence_blockers(true,false),vec!["diagnostic_attachment_incomplete"]);
    assert_eq!(required_evidence_blockers(false,false).len(),2);
}
