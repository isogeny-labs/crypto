use super::test_support::*;
use serde_json::{json, Value};
use std::fs;

fn boundaries(v: &Value) {
    for name in [
        "cell_complete",
        "solver_state_restorable",
        "solver_resumed",
        "scientific_result_verified",
        "complete_execution_verified",
        "hardware_independently_verified",
        "platform_admitted",
        "training_cost_eligible",
        "clock_valid_for_comparison",
        "whole_worker_coverage_verified",
        "worker_publication_verified",
        "fresh_remote_readback_verified",
    ] {
        assert_eq!(v[name], false, "{name}");
    }
    assert!(v["whole_worker_instruction_count"].is_null());
    assert_eq!(v["public_rows"], 0);
    assert_eq!(v["partial_snapshot_chain_verified"], true);
}
#[test]
fn p01_original_producer_policy_only_relocated() {
    let p = retained_producer();
    let result = p.restore().unwrap();
    assert_eq!(result["artifact_count"], 7);
    assert_eq!(result["artifact_bytes"], 6564);
    assert_eq!(result["capture_validation_state"], "not_present");
    boundaries(&result);
    boundaries(&p.audit().unwrap());
    assert_eq!(
        fs::read(p.out.join("partial-consumer/anchor.json")).unwrap(),
        fs::read(&p.anchor).unwrap()
    );
    p.retain(
        "P01-retained-producer",
        "original seven context files/6564 bytes; relocated mirror",
        &result,
    );
}
#[test]
fn p02_legacy_actual_context_and_opaque_role() {
    let mut f = Fixture::new(false);
    f.add_context(0, "host.json", b"opaque legacy diagnostic".to_vec());
    f.snapshots[0]["context"][5]["role"] = json!("custom legacy role");
    let p = f.publish();
    let report = p.restore().unwrap();
    assert_eq!(report["artifact_count"], 6);
    assert_eq!(report["checked_phase_contract_verified"], false);
    boundaries(&report);
    boundaries(&p.audit().unwrap());
    p.retain(
        "P02-legacy-opaque",
        "actual context and opaque-role diagnostics",
        &report,
    );
}
#[test]
fn p03_p10_split_capture_binary_payloads_and_full_ancestry() {
    let mut f = Fixture::new(false);
    f.capture(
        0,
        vec![json!({"kind":"checkpoint","opaque":1})],
        vec![],
        None,
    );
    let i = f.push_snapshot(false);
    f.capture(
        i,
        vec![],
        vec![
            ("payloads/empty.bin", vec![]),
            ("payloads/binary.bin", vec![0, 255, 128]),
        ],
        None,
    );
    f.push_snapshot(false);
    let p = f.publish();
    let report = p.restore().unwrap();
    assert_eq!(report["snapshot_count"], 3);
    assert_eq!(report["cursor"]["event_count"], 3);
    assert_eq!(report["cursor"]["next_chunk"], 2);
    assert_eq!(report["cursor"]["payload_count"], 2);
    assert_eq!(report["cursor"]["payload_bytes"], 3);
    assert_eq!(
        fs::read(p.artifact_path("payloads/binary.bin")).unwrap(),
        vec![0, 255, 128]
    );
    boundaries(&report);
    boundaries(&p.audit().unwrap());
    p.retain(
        "P03-P10-capture-payloads",
        "three snapshots, two segments, empty and nonUTF8 payloads",
        &report,
    );
}
#[test]
fn p09_p11_empty_capture_sticky_stop_and_absent_delta() {
    let mut f = Fixture::new(false);
    f.capture(0, vec![], vec![], None);
    let i = f.push_snapshot(false);
    f.capture(i, vec![], vec![], Some("event_limit"));
    let i = f.push_snapshot(false);
    f.capture(i, vec![], vec![], Some("event_limit"));
    f.push_snapshot(false);
    let p = f.publish();
    let result = p.restore().unwrap();
    assert_eq!(result["capture_validation_state"], "verified_empty_prefix");
    assert_eq!(result["reported_stop"], "event_limit");
    boundaries(&result);
    p.retain(
        "P09-P11-empty-stopped",
        "explicit empty capture and unchanged sticky stop",
        &result,
    );
}
#[test]
fn p04_phase_prefixes_and_observation_only_terminal() {
    let mut f = Fixture::new(true);
    let i = f.push_snapshot(true);
    f.add_saved_phase(i, "telemetry/curve-construction-start.json");
    let i = f.push_snapshot(true);
    f.add_saved_phase(i, "telemetry/curve-construction.json");
    let i = f.push_snapshot(true);
    f.add_saved_phase(i, "telemetry/known-answer-replay-start.json");
    let i = f.push_snapshot(true);
    f.snapshots[i]["observation"]["status"] = json!("interrupted");
    let p = f.publish();
    let result = p.restore().unwrap();
    assert_eq!(result["snapshot_count"], 5);
    assert_eq!(result["checked_phase_contract_verified"], true);
    boundaries(&result);
    boundaries(&p.audit().unwrap());
    p.retain(
        "P04-phase-prefixes",
        "policy/start/end/next-start/observation prefixes",
        &result,
    );
}
#[test]
fn p06_complete_recorded_mock_prefix_stays_partial() {
    let mut f = Fixture::new(true);
    for phase in [
        "curve-construction",
        "known-answer-replay",
        "factor-base-replay",
        "solver",
    ] {
        f.add_saved_phase(0, &format!("telemetry/{phase}-start.json"));
        f.add_saved_phase(0, &format!("telemetry/{phase}.json"));
    }
    let p = f.publish();
    let result = p.restore().unwrap();
    assert_eq!(result["context_count"], 14);
    boundaries(&result);
    boundaries(&p.audit().unwrap());
    p.retain(
        "P06-full-recorded-prefix",
        "four recorded phases remain partial",
        &result,
    );
}
#[test]
fn p16_prior_report_never_supplies_proof() {
    let p = Fixture::new(false).publish();
    p.restore().unwrap();
    fs::write(
        p.out.join("partial-consumer/report.json"),
        b"{\"solver_resumed\":true}",
    )
    .unwrap();
    let result = p.audit().unwrap();
    boundaries(&result);
    p.retain(
        "P16-untrusted-prior-report",
        "fresh proof ignores stored report claims",
        &result,
    );
}

#[test]
fn p05_recorded_operation_error_is_valid_but_blocks_later_phases() {
    let mut f = Fixture::new(true);
    f.add_saved_phase(0, "telemetry/curve-construction-start.json");
    let mut end: Value =
        serde_json::from_slice(&saved("telemetry/curve-construction.json")).unwrap();
    end["returned_ok"] = json!(false);
    end["returned_error"] = json!({"message_preview":"é","message_bytes":2,"message_sha256":hash("é".as_bytes()),"truncated":false});
    f.add_context(0, "telemetry/curve-construction.json", wire(&end));
    let i = f.push_snapshot(true);
    f.snapshots[i]["observation"]["status"] = json!("exited");
    f.snapshots[i]["observation"]["exit_code"] = json!(1);
    let p = f.publish();
    let report = p.restore().unwrap();
    assert_eq!(
        report["checked_phase_contract"]["prefix_state"],
        "operation_error"
    );
    boundaries(&report);
    p.retain(
        "P05-operation-error",
        "checked operation error, no later phase",
        &report,
    );
}
#[test]
fn p07_fallback_reuses_closed_descriptor_and_zero_kernel_id() {
    let mut f = Fixture::new(true);
    f.add_saved_phase(0, "telemetry/curve-construction-start.json");
    let mut end: Value =
        serde_json::from_slice(&saved("telemetry/curve-construction.json")).unwrap();
    let mut fallback = end["collection"]["attempts"][0]["events"][0].clone();
    fallback["kernel_event_id"] = json!(0);
    fallback["raw"]["event_id"] = json!(0);
    let leader = &mut end["collection"]["attempts"][0]["events"][0];
    leader["lifecycle"]
        .as_array_mut()
        .unwrap()
        .retain(|c| matches!(c["operation"].as_str(), Some("open" | "event_id" | "close")));
    leader["status"] = json!("cancelled_before_start");
    for key in ["raw", "read_bytes", "tuple_assessment"] {
        leader[key] = Value::Null;
    }
    let member = &mut end["collection"]["attempts"][0]["events"][1];
    member["lifecycle"] =
        json!([{"operation":"open","result":{"Err":{"errno":13,"message":"permission denied"}}}]);
    member["status"] = json!("permission_denied");
    for key in [
        "raw",
        "tuple_assessment",
        "read_bytes",
        "cleanup_succeeded",
        "kernel_event_id",
    ] {
        member[key] = Value::Null;
    }
    end["collection"]["attempts"][0]["start_succeeded"] = json!(false);
    end["collection"]["selected_attempt"] = json!(1);
    end["collection"]["attempts"].as_array_mut().unwrap().push(json!({"mode":"standalone_instructions_fallback","events":[fallback],"start_succeeded":true}));
    end["observations"] = json!([{"event":"instructions","attempt_index":1,"event_index":0,"status":"scoped_raw","raw":0,"estimate":null,"lifecycle_clean":true}]);
    f.add_context(0, "telemetry/curve-construction.json", wire(&end));
    let p = f.publish();
    let report = p.restore().unwrap();
    boundaries(&report);
    p.retain(
        "P07-fallback-zero-id",
        "permitted fallback with closed FD reuse and zero kernel ID",
        &report,
    );
}
fn unavailable(mut end: Value) -> Value {
    end["counter_start_succeeded"] = json!(false);
    end["observations"] = json!([]);
    let c = &mut end["collection"];
    c["os"] = json!("other");
    c["architecture"] = json!("powerpc64");
    c["owner_tid"] = Value::Null;
    c["selected_attempt"] = Value::Null;
    c["attempts"][0]["start_succeeded"] = json!(false);
    for check in c["context_checks"].as_array_mut().unwrap() {
        check["current"]["tid"] = Value::Null;
        check["status"] = json!("unavailable");
    }
    for event in c["attempts"][0]["events"].as_array_mut().unwrap() {
        let config = event["request"]["config"].as_u64().unwrap();
        let mut attrs = vec![0u8; 112];
        attrs[4..8].copy_from_slice(&112u32.to_be_bytes());
        for (offset, value) in [(8, config), (32, 7), (40, 97)] {
            attrs[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
        }
        event["request"]["attr_bytes_native_endian"] = json!(attrs);
        event["request"]["byte_order"] = json!("big");
        event["request"]["attr_flags"] = json!(97);
        event["request"]["group_fd"] = json!(-1);
        event["lifecycle"] = json!([]);
        event["status"] = json!("unsupported_platform");
        for key in [
            "kernel_event_id",
            "raw",
            "tuple_assessment",
            "read_bytes",
            "cleanup_succeeded",
        ] {
            event[key] = Value::Null;
        }
    }
    end
}
#[test]
fn p08_unsupported_big_endian_record_is_explicitly_unavailable() {
    let mut f = Fixture::new(true);
    f.add_saved_phase(0, "telemetry/curve-construction-start.json");
    let end =
        unavailable(serde_json::from_slice(&saved("telemetry/curve-construction.json")).unwrap());
    f.add_context(0, "telemetry/curve-construction.json", wire(&end));
    let p = f.publish();
    let report = p.restore().unwrap();
    boundaries(&report);
    p.retain(
        "P08-unsupported-big-endian",
        "unsupported diagnostic with no native calls/TIDs/projection",
        &report,
    );
}
#[test]
fn p12_omitted_options_and_reordered_wire_have_equivalent_semantics() {
    let f = Fixture::new(false);
    let original = f.publish();
    let report = original.restore().unwrap();
    let changed = f.publish_with(|_, stage, v| match stage {
        Stage::Manifest => {
            let m = v.as_object_mut().unwrap();
            m.remove("delta");
            m.remove("previous");
            for k in ["exit_code", "signal", "reason"] {
                m.get_mut("observation")
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove(k);
            }
            m.get_mut("coverage")
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove("stop");
        }
        Stage::Receipt => {
            v.as_object_mut().unwrap().remove("previous");
        }
        _ => (),
    });
    assert_ne!(changed.pin, original.pin);
    let observed = changed.restore().unwrap();
    assert_eq!(report["binding"], observed["binding"]);
    assert_eq!(observed["capture_validation_state"], "not_present");
    boundaries(&observed);
    changed.retain(
        "P12-omitted-null",
        "omitted nullable wire values retain equivalent semantics",
        &observed,
    );
    let reordered = f.publish_raw(|_, _, raw| {
        let value: Value = serde_json::from_slice(raw).unwrap();
        let map = value
            .as_object()
            .unwrap()
            .iter()
            .rev()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        *raw = wire(&Value::Object(map));
    });
    assert_ne!(reordered.pin, original.pin);
    let observed = reordered.restore().unwrap();
    boundaries(&observed);
    reordered.retain(
        "P12-reordered-metadata",
        "metadata order changes raw pins but preserves semantics",
        &observed,
    );
}
#[test]
fn p06_all_recorded_phases_can_have_unavailable_counters() {
    let mut f = Fixture::new(true);
    for phase in [
        "curve-construction",
        "known-answer-replay",
        "factor-base-replay",
        "solver",
    ] {
        f.add_saved_phase(0, &format!("telemetry/{phase}-start.json"));
        let name = format!("telemetry/{phase}.json");
        let end = unavailable(serde_json::from_slice(&saved(&name)).unwrap());
        f.add_context(0, &name, wire(&end));
    }
    let p = f.publish();
    let report = p.restore().unwrap();
    assert_eq!(report["checked_phase_contract"]["supplied_ends"], 4);
    boundaries(&report);
    p.retain(
        "P06-unavailable-four-phases",
        "complete recorded prefix with unavailable counters remains partial",
        &report,
    );
}
#[test]
fn p13_thousand_digit_protocol_preserves_raw_and_canonical_domains() {
    let mut f = Fixture::new(false);
    let integer = "7".repeat(1000);
    let body = format!("{{\"n\":{integer},\"schema\":\"experiment.protocol/v1\"}}");
    let freeze = hash(body.as_bytes());
    let protocol=format!("{{\"schema\":\"experiment.protocol/v1\",\"n\":{integer},\"freeze\":{{\"status\":\"frozen\",\"frozen_at\":\"2026-10-29T00:00:00Z\",\"filed_before_affected_data\":true,\"protocol_sha256\":\"{freeze}\"}}}}\n").into_bytes();
    let mut plan: Value = serde_json::from_slice(&saved("plan.json")).unwrap();
    plan["protocol_sha256"] = json!(hash(&protocol));
    // Independent literal v1 field order from the frozen producer Plan wire.
    let keys = [
        "schema",
        "field_sizes",
        "arities",
        "families",
        "solvers",
        "curve_seeds",
        "generators",
        "keys",
        "seed",
        "factor_base_size",
        "degree_bound",
        "work_limit",
        "twists",
        "protocol_sha256",
        "split_rule",
    ];
    let mut canonical = serde_json::Map::new();
    for key in keys {
        canonical.insert(key.into(), plan[key].clone());
    }
    let canonical_sha = hash(&serde_json::to_vec(&Value::Object(canonical)).unwrap());
    let raw_plan = wire(&plan);
    assert_ne!(hash(&raw_plan), canonical_sha);
    for name in ["run.json", "attempt.json"] {
        let mut value: Value = serde_json::from_slice(&saved(name)).unwrap();
        value["plan_sha256"] = json!(canonical_sha);
        f.replace_artifact(&format!("context/{name}"), wire(&value));
    }
    f.replace_artifact("context/protocol.json", protocol);
    f.replace_artifact("context/plan.json", raw_plan);
    f.refresh_binding_raw();
    let p = f.publish();
    let report = p.restore().unwrap();
    boundaries(&report);
    p.retain(
        "P13-large-integer",
        "1000-digit integer and separate raw/canonical commitments",
        &report,
    );
}
