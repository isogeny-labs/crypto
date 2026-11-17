use super::*;
use serde_json::{json, Value};
use std::fs;

fn fixture() -> Value {
    let params = json!({"encoding":"partitioned3","roots":"exact","stop":"exhaustive","labels":"random","subset_size":"1","attempts":"1","degree":"24","budget_ms":"1000","sampling_draws":"1000","orbit":"none","beta":"0","lambda":"0","prune":"false"});
    let names = [
        "setup",
        "target_query",
        "target_PDP",
        "target_relation_check",
        "target_descent",
        "target_recovery_check",
    ];
    let phases:Vec<_>=names.iter().enumerate().map(|(i,name)|json!({"name":name,"wall_ns":(i+1)*10,"adds":0,"doubles":0,"scalar_mults":0,"gae":0.0,"native":{}})).collect();
    let online = names
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, n)| (n.to_string(), json!((i + 2) * 10)))
        .collect::<serde_json::Map<_, _>>();
    json!({"schema":"ecbench.record/v1","record_id":"","session_id":"unit-session","seq":0,"round":0,"warmup":false,"arm":"fixture-reference","role":"reference",
        "method":{"id":"ic.aps_v1","method_id":"unit-method","params":params},
        "workload":{"workload_id":"unit-workload","curve_spec":{"kind":"prime_explicit","p":13,"a":7,"b":6,"group_order":11,"r":11,"gx":1,"gy":1},"curve":{"r":11,"generator":["1","1"]},"target":["1","1"]},
        "algorithm_seed":"7","outcome":{"status":"verified","recovered":"1"},"binary_sha256":"0000000000000000000000000000000000000000000000000000000000000000",
        "phases":phases,"online":{"wall_ns":200,"phases_ns":online},"time":{"solve_wall_ns":210,"method_call_wall_ns":220,"user_ns":20,"sys_ns":10,"max_rss_kib":1},"isolation":{"level":"L0"},
        "counters":{"aps_attempts":1,"aps_failed_attempts":0,"aps_deadline_attempts":0,"aps_f4_multiplications_uncharged":0,"aps_root_multiplications_uncharged":0,"aps_unreported_algebra_attempts":0,"aps_legacy_root_count_unknown_attempts":0},"detail":{"schema":"ecbench.aps/v1","outcome":"verified","target_error":null,"native_work_accounting_complete":false,"costs_attested":false,"complete_sets_independently_audited":false,
        "attempts":[{"index":0,"seed":7,"base":{"parts":[[1],[10],[6]],"points":{"1":{"x":1,"y":1,"u":1,"v":0},"10":{"x":10,"y":6,"u":2,"v":0},"6":{"x":6,"y":2,"u":0,"v":3}},"draws":3},"system":null,"system_sha256":null,
            "algebra":{"coordinate_sets":[[1,6,10]],"complete":true,"timed_out":false,"f4_field_multiplications":0,"roots":{"root_field_multiplications":0}},"visited_coordinate_sets":[[1,6,10]],"recovery":{"logarithm":1,"coordinates":[1,6,10],"signs":[1,-1,1],"a":3,"b":8},"independent_witness_verified":true,"outcome":"verified","error":null,"error_stage":null,"deadline_exhausted":false}]}})
}
fn seal(mut r: Value) -> String {
    r["record_id"] = json!("");
    let s = serde_json::to_string(&r).unwrap();
    r["record_id"] = json!(format!("ECR1h{}", &hash(s.as_bytes())[..12]));
    serde_json::to_string(&r).unwrap()
}
fn math(r: &Value) -> Result<Value> {
    audit::mathematical(r, audit::MAX_ORACLE_CANDIDATES)
}
fn no_witness(r: &mut Value) {
    r["outcome"] = json!({"status":"exhausted","recovered":null});
    r["detail"]["outcome"] = json!("attempts_exhausted");
    let a = &mut r["detail"]["attempts"][0];
    a["recovery"] = Value::Null;
    a["independent_witness_verified"] = json!(false);
    a["outcome"] = json!("no_informative_relation");
}

#[test]
fn independent_ordinary_residue_witness_and_full_set() {
    let r = fixture();
    assert_eq!(math(&r).unwrap()["witnesses_checked"], 1);
    assert_eq!(math(&r).unwrap()["complete_attempts_checked"], 1);
    assert_eq!(audit::metrics(&r).unwrap()["cold_ns"], 210);
}
#[test]
fn resealing_a_false_scalar_does_not_fool_the_mathematical_reader() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["recovery"]["logarithm"] = json!(2);
    let r = audit::record_line(&seal(r)).unwrap();
    assert!(math(&r).is_err());
}
#[test]
fn composite_field_and_subgroup_are_rejected() {
    for key in ["p", "r"] {
        let mut r = fixture();
        r["workload"]["curve_spec"][key] = json!(15);
        assert!(math(&r).is_err());
    }
}
#[test]
fn corrupted_label_and_ordinate_are_rejected() {
    for (key, n) in [("u", 2), ("v", 1), ("y", 2)] {
        let mut r = fixture();
        r["detail"]["attempts"][0]["base"]["points"]["1"][key] = json!(n);
        assert!(math(&r).is_err());
    }
}
#[test]
fn wrong_scalar_coefficient_sign_and_zero_b_are_rejected() {
    for (key, v) in [
        ("a", json!(4)),
        ("b", json!(0)),
        ("signs", json!([1, 1, 1])),
        ("coordinates", json!([1, 10, 6])),
    ] {
        let mut r = fixture();
        r["detail"]["attempts"][0]["recovery"][key] = v;
        assert!(math(&r).is_err());
    }
}
#[test]
fn a_correct_public_scalar_does_not_rescue_uninformative_labels() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["base"]["points"]["6"]["u"] = json!(3);
    r["detail"]["attempts"][0]["base"]["points"]["6"]["v"] = json!(0);
    r["detail"]["attempts"][0]["recovery"]["a"] = json!(0);
    r["detail"]["attempts"][0]["recovery"]["b"] = json!(0);
    assert!(math(&r).is_err());
}
#[test]
fn omitted_complete_root_is_detected_by_the_point_oracle() {
    let mut r = fixture();
    no_witness(&mut r);
    r["detail"]["attempts"][0]["algebra"]["coordinate_sets"] = json!([]);
    r["detail"]["attempts"][0]["visited_coordinate_sets"] = json!([]);
    assert!(math(&r).is_err());
}
#[test]
fn partial_root_sets_are_retained_without_a_completeness_claim() {
    let mut r = fixture();
    no_witness(&mut r);
    let a = &mut r["detail"]["attempts"][0];
    a["algebra"]["complete"] = json!(false);
    a["algebra"]["coordinate_sets"] = json!([]);
    a["visited_coordinate_sets"] = json!([]);
    a["outcome"] = json!("incomplete");
    let a = math(&r).unwrap();
    assert_eq!(a["complete_attempts_checked"], 0);
    assert_eq!(a["partial_attempts_retained"], 1);
}
#[test]
fn visitor_and_algebra_set_disagreement_is_rejected() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["visited_coordinate_sets"] = json!([]);
    assert!(math(&r).is_err());
}
#[test]
fn duplicate_sets_and_partition_members_are_rejected() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["visited_coordinate_sets"] = json!([[1, 6, 10], [1, 6, 10]]);
    assert!(math(&r).is_err());
    let mut r = fixture();
    r["detail"]["attempts"][0]["base"]["parts"] = json!([[1], [1], [6]]);
    assert!(math(&r).is_err());
}
#[test]
fn unsupported_expansion_does_not_panic() {
    let mut r = fixture();
    r["method"]["params"]["encoding"] = json!("expanded4");
    assert!(math(&r).is_err());
}
#[test]
fn oracle_budget_is_a_hard_refusal() {
    assert!(audit::mathematical(&fixture(), 1).is_err());
    assert!(audit::mathematical(&fixture(), 0).is_err());
    assert!(audit::mathematical(&fixture(), audit::MAX_ORACLE_CANDIDATES + 1).is_err());
}
#[test]
fn late_witness_cannot_become_a_success() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["deadline_exhausted"] = json!(true);
    assert!(math(&r).is_err());
    r["detail"]["attempts"][0]["outcome"] = json!("verified_after_deadline");
    r["detail"]["outcome"] = json!("verified_after_deadline");
    r["outcome"] = json!({"status":"exhausted","recovered":null});
    assert_eq!(math(&r).unwrap()["witnesses_checked"], 1);
}
#[test]
fn attempts_cannot_continue_after_a_witness() {
    let mut r = fixture();
    r["method"]["params"]["attempts"] = json!("2");
    let mut a = r["detail"]["attempts"][0].clone();
    a["index"] = json!(1);
    a["seed"] = json!(8);
    r["detail"]["attempts"].as_array_mut().unwrap().push(a);
    r["counters"]["aps_attempts"] = json!(2);
    assert!(math(&r).is_err());
}
#[test]
fn pre_transcript_sampling_failure_is_retained() {
    let mut r = fixture();
    no_witness(&mut r);
    r["detail"]["attempts"][0] = json!({"index":0,"seed":7,"base":null,"system":null,"recovery":null,"error":"cap","error_stage":"sampling","independent_witness_verified":false,"deadline_exhausted":false});
    assert_eq!(math(&r).unwrap()["partial_attempts_retained"], 1);
}
#[test]
fn nonconsecutive_seed_and_counter_are_rejected() {
    for key in ["index", "seed"] {
        let mut r = fixture();
        r["detail"]["attempts"][0][key] = json!(9);
        assert!(math(&r).is_err());
    }
    let mut r = fixture();
    r["counters"]["aps_attempts"] = json!(0);
    assert!(math(&r).is_err());
}
#[test]
fn target_error_has_no_invented_attempt() {
    let mut r = fixture();
    r["outcome"] = json!({"status":"error","recovered":null});
    r["detail"]["target_error"] = json!("invalid input");
    r["detail"]["attempts"] = json!([]);
    r["counters"]["aps_attempts"] = json!(0);
    assert_eq!(math(&r).unwrap()["witnesses_checked"], 0);
    r["outcome"]["status"] = json!("verified");
    assert!(math(&r).is_err());
}
#[test]
fn pruning_requires_orbit_certification() {
    let mut r = fixture();
    r["method"]["params"]["prune"] = json!("true");
    assert!(math(&r).is_err());
}
#[test]
fn equation_binding_is_checked() {
    let mut r = fixture();
    let a = &mut r["detail"]["attempts"][0];
    a["system"] = json!({"p":13,"variables":["x0"],"equations":[]});
    a["system_sha256"] = json!("false");
    assert!(math(&r).is_err());
}
#[test]
fn clock_duplicates_omissions_and_overflow_are_rejected() {
    let mut r = fixture();
    let p = r["phases"][0].clone();
    r["phases"].as_array_mut().unwrap().push(p);
    assert!(audit::metrics(&r).is_err());
    let mut r = fixture();
    r["phases"].as_array_mut().unwrap().pop();
    assert!(audit::metrics(&r).is_err());
    let mut r = fixture();
    r["phases"][0]["wall_ns"] = json!(u64::MAX);
    assert!(audit::metrics(&r).is_err());
}
#[test]
fn exclusive_totals_and_enclosing_clock_must_match() {
    for (key, v) in [("solve_wall_ns", 211), ("method_call_wall_ns", 100)] {
        let mut r = fixture();
        r["time"][key] = json!(v);
        assert!(audit::metrics(&r).is_err());
    }
    let mut r = fixture();
    r["online"]["phases_ns"]["target_PDP"] = json!(31);
    assert!(audit::metrics(&r).is_err());
}
#[test]
fn clock_completeness_does_not_attest_pricing() {
    let mut r = fixture();
    r["detail"]["costs_attested"] = json!(true);
    assert!(audit::metrics(&r).is_err());
}
#[test]
fn literal_seal_rejects_tampering_and_format_rewrites() {
    let line = seal(fixture());
    assert!(audit::record_line(&line).is_ok());
    assert!(audit::record_line(&line.replace("unit-session", "other-session")).is_err());
    let v: Value = serde_json::from_str(&line).unwrap();
    assert!(audit::record_line(&serde_json::to_string_pretty(&v).unwrap()).is_err());
}

fn write(dir: &std::path::Path, name: &str, v: &Value) {
    fs::write(dir.join(name), serde_json::to_vec(v).unwrap()).unwrap();
}
fn tree() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let r = fixture();
    let ids = [
        ("fixture-reference", "reference"),
        ("fixture-candidate", "candidate"),
        ("fixture-aa", "control"),
    ];
    let arms: Vec<_> = ids
        .iter()
        .map(|(id, role)| json!({"name":id,"role":role,"method":r["method"]}))
        .collect();
    let execs: Vec<_> = (0..ids.len())
        .map(|i| json!({"seq":i,"round":0,"warmup":false,"arm":i,"workload":0,"algorithm_seed":7}))
        .collect();
    let plan = json!({"schema":"ecbench.plan/v1","arms":arms,"workloads":[r["workload"]],"executions":execs});
    write(d.path(), "plan.json", &plan);
    let lines: Vec<_> = ids
        .iter()
        .enumerate()
        .map(|(i, (id, role))| {
            let mut r = r.clone();
            r["seq"] = json!(i);
            r["arm"] = json!(id);
            r["role"] = json!(role);
            seal(r)
        })
        .collect();
    fs::write(
        d.path().join("records.jsonl"),
        format!("{}\n", lines.join("\n")),
    )
    .unwrap();
    let events:Vec<_>=(0..ids.len()).map(|i|json!({"seq":i,"attempts":1,"resumed":0,"checkpoint":null,"status":"succeeded","started_at":null,"finished_at":null})).collect();
    write(
        d.path(),
        "history.json",
        &json!({"schema":"aps.execution-history/v1","events":events}),
    );
    let mut p: Value =
        serde_json::from_str(include_str!("../tests/fixtures/synthetic-protocol.json")).unwrap();
    for a in p["arms"].as_array_mut().unwrap() {
        a["instrument"]["binary_sha256"] = json!("0".repeat(64));
        a["instrument"]["commit"] = json!("0".repeat(40));
        a["parameters"] = json!({"method":"ic.aps_v1","purpose":"synthetic exporter contract fixture, not an experimental control"});
    }
    p["freeze"] = json!({"status":"frozen","protocol_sha256":null,"commit":"0".repeat(40),"frozen_at":"2026-11-04T00:00:00Z","filed_before_affected_data":true});
    p["analysis_plan"]["script"] = Value::Null;
    p["analysis_plan"]["blinded"] = json!(false);
    p["extensions"]["aps_export"] =
        json!({"plan_sha256":hash(&fs::read(d.path().join("plan.json")).unwrap())});
    p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
    write(d.path(), "protocol.json", &p);
    write(
        d.path(),
        "request.json",
        &json!({"schema":"aps.export-request/v1","export_id":"unit-export","created":"2026-11-04T00:00:00Z","protocol":"protocol.json","plan":"plan.json","records":"records.jsonl","history":"history.json","source":{"repo":p["repository"],"commit":"0".repeat(40),"dirty":null},"producer":{"runtime":"unit-test","model":null,"session":null,"role":"fixture"}}),
    );
    d
}
fn build(d: &tempfile::TempDir) -> Result<Value> {
    package::build(&EvidenceRoot::new(d.path()).unwrap(), "request.json")
}
#[test]
fn native_package_roundtrip_reconstructs_from_pinned_inputs() {
    let d = tree();
    let e = build(&d).unwrap();
    package::check(&EvidenceRoot::new(d.path()).unwrap(), &e).unwrap();
    assert_eq!(e["flow"]["completed"], 3);
    assert_eq!(e["runs"][0]["measurements"]["wall_valid"], false);
    assert_eq!(e["backend_record"]["execution_attested"], false);
    // Optional retention of synthetic fixtures for the separate CLI contracts.
    // All correctness assertions above still run when retention is disabled.
    if let Some(out) = std::env::var_os("APS_EXPORT_TEST_FIXTURES") {
        fs::create_dir(&out).unwrap();
        for name in [
            "request.json",
            "protocol.json",
            "plan.json",
            "records.jsonl",
            "history.json",
        ] {
            fs::copy(d.path().join(name), std::path::Path::new(&out).join(name)).unwrap();
        }
    }
}
#[test]
fn reshaped_export_and_changed_raw_input_are_rejected() {
    let d = tree();
    let e = build(&d).unwrap();
    let mut bad = e.clone();
    bad["runs"][0]["measurements"]["wall_valid"] = json!(true);
    assert!(package::check(&EvidenceRoot::new(d.path()).unwrap(), &bad).is_err());
    fs::write(d.path().join("records.jsonl"), "changed").unwrap();
    assert!(package::check(&EvidenceRoot::new(d.path()).unwrap(), &e).is_err());
}
#[test]
fn unknown_restore_history_cannot_be_invented() {
    let d = tree();
    let mut h: Value =
        serde_json::from_slice(&fs::read(d.path().join("history.json")).unwrap()).unwrap();
    h["events"][0].as_object_mut().unwrap().remove("resumed");
    write(d.path(), "history.json", &h);
    assert!(build(&d).is_err());
    h["events"][0]["resumed"] = json!(1);
    write(d.path(), "history.json", &h);
    assert!(build(&d).is_err());
}
#[test]
fn duplicate_raw_records_and_history_events_are_rejected() {
    let d = tree();
    let line = fs::read(d.path().join("records.jsonl")).unwrap();
    let mut twice = line.clone();
    twice.extend(line);
    fs::write(d.path().join("records.jsonl"), twice).unwrap();
    assert!(build(&d).is_err());
    let d = tree();
    let mut h: Value =
        serde_json::from_slice(&fs::read(d.path().join("history.json")).unwrap()).unwrap();
    let ev = h["events"][0].clone();
    h["events"].as_array_mut().unwrap().push(ev);
    write(d.path(), "history.json", &h);
    assert!(build(&d).is_err());
}
#[test]
fn wrong_planned_seed_and_changed_protocol_are_rejected() {
    let d = tree();
    let mut p: Value =
        serde_json::from_slice(&fs::read(d.path().join("plan.json")).unwrap()).unwrap();
    p["executions"][0]["algorithm_seed"] = json!(8);
    write(d.path(), "plan.json", &p);
    assert!(build(&d).is_err());
    let d = tree();
    let mut p: Value =
        serde_json::from_slice(&fs::read(d.path().join("protocol.json")).unwrap()).unwrap();
    p["question"] = json!("modified after freeze");
    write(d.path(), "protocol.json", &p);
    assert!(build(&d).is_err());
}
#[test]
fn unknown_counts_are_not_exported_as_complete_s() {
    let d = tree();
    let e = build(&d).unwrap();
    assert_eq!(e["runs"][0]["measurements"]["counted"], json!({}));
    assert_eq!(
        e["backend_record"]["native_work_accounting_complete"],
        false
    );
    assert_eq!(e["host"]["path"], Value::Null);
}
#[test]
fn evidence_parent_escape_and_overwrite_are_rejected() {
    let d = tree();
    let root = EvidenceRoot::new(d.path()).unwrap();
    assert!(root.resolve("../request.json").is_err());
    assert!(root.resolve("/etc/passwd").is_err());
    write_new(&d.path().join("new.json"), &json!({})).unwrap();
    assert!(write_new(&d.path().join("new.json"), &json!({})).is_err());
}
#[cfg(unix)]
#[test]
fn evidence_symlink_escape_is_rejected() {
    let d = tree();
    std::os::unix::fs::symlink("/etc/passwd", d.path().join("escape")).unwrap();
    assert!(EvidenceRoot::new(d.path())
        .unwrap()
        .resolve("escape")
        .is_err());
}
#[test]
fn real_nested_schema_validation_is_applied() {
    let d = tree();
    let mut e = build(&d).unwrap();
    e["runs"][0]["measurements"]["wall_valid"] = json!("yes");
    assert!(validate_schema(&e, false).is_err());
}

#[test]
fn missing_planned_slots_are_retained_without_invented_attempts() {
    let d = tree();
    let raw = fs::read_to_string(d.path().join("records.jsonl")).unwrap();
    fs::write(
        d.path().join("records.jsonl"),
        format!("{}\n", raw.lines().take(2).collect::<Vec<_>>().join("\n")),
    )
    .unwrap();
    let mut h: Value =
        serde_json::from_slice(&fs::read(d.path().join("history.json")).unwrap()).unwrap();
    h["events"].as_array_mut().unwrap().pop();
    write(d.path(), "history.json", &h);
    let e = build(&d).unwrap();
    assert_eq!(e["backend_record"]["planned_execution_slots"], 3);
    assert_eq!(e["backend_record"]["missing_execution_slots"][0]["seq"], 2);
    assert_eq!(e["flow"]["attempted"], 2);
    assert_eq!(e["flow"]["excluded"], json!([]));
    assert!(package::check(&EvidenceRoot::new(d.path()).unwrap(), &e).is_err());
}

#[test]
fn recordless_infrastructure_failure_remains_an_observed_outcome() {
    let d = tree();
    let raw = fs::read_to_string(d.path().join("records.jsonl")).unwrap();
    fs::write(
        d.path().join("records.jsonl"),
        format!("{}\n", raw.lines().take(2).collect::<Vec<_>>().join("\n")),
    )
    .unwrap();
    let mut h: Value =
        serde_json::from_slice(&fs::read(d.path().join("history.json")).unwrap()).unwrap();
    h["events"][2]["status"] = json!("infra_error");
    write(d.path(), "history.json", &h);
    let e = build(&d).unwrap();
    package::check(&EvidenceRoot::new(d.path()).unwrap(), &e).unwrap();
    assert_eq!(e["flow"]["attempted"], 3);
    assert_eq!(e["flow"]["completed"], 2);
    assert_eq!(e["flow"]["not_completed"], 1);
    assert_eq!(e["runs"][2]["measurements"]["wall_seconds"], Value::Null);
    assert_eq!(e["runs"][2]["verification"]["status"], "no_claim");
}

#[test]
fn a_late_witness_cannot_be_promoted_by_resealing_the_record() {
    let mut r = fixture();
    r["detail"]["attempts"][0]["outcome"] = json!("verified_after_deadline");
    r["detail"]["attempts"][0]["deadline_exhausted"] = json!(true);
    let r = audit::record_line(&seal(r)).unwrap();
    assert!(math(&r).is_err());
}

#[test]
fn draft_and_post_data_protocols_are_not_accepted() {
    for key in ["status", "filed_before_affected_data"] {
        let d = tree();
        let mut p: Value =
            serde_json::from_slice(&fs::read(d.path().join("protocol.json")).unwrap()).unwrap();
        p["freeze"][key] = if key == "status" {
            json!("draft")
        } else {
            json!(false)
        };
        write(d.path(), "protocol.json", &p);
        assert!(build(&d).is_err());
    }
}

#[test]
fn a_resealed_record_cannot_change_its_frozen_execution_cell() {
    let d = tree();
    let raw = fs::read_to_string(d.path().join("records.jsonl")).unwrap();
    let mut lines: Vec<_> = raw.lines().map(str::to_owned).collect();
    let mut r: Value = serde_json::from_str(&lines[0]).unwrap();
    r["algorithm_seed"] = json!("8");
    lines[0] = seal(r);
    fs::write(
        d.path().join("records.jsonl"),
        format!("{}\n", lines.join("\n")),
    )
    .unwrap();
    assert!(build(&d).is_err());
}

#[test]
fn counter_tokens_above_javascript_precision_remain_exact() {
    let d = tree();
    let raw = fs::read_to_string(d.path().join("records.jsonl")).unwrap();
    let mut lines: Vec<_> = raw.lines().map(str::to_owned).collect();
    let mut r: Value = serde_json::from_str(&lines[0]).unwrap();
    r["counters"]["large_diagnostic"] = json!("9007199254740993");
    lines[0] = seal(r);
    fs::write(
        d.path().join("records.jsonl"),
        format!("{}\n", lines.join("\n")),
    )
    .unwrap();
    let e = build(&d).unwrap();
    assert_eq!(
        e["runs"][0]["measurements"]["counters"]["large_diagnostic"],
        "9007199254740993"
    );
}

#[test]
fn resealed_diagnostic_totals_must_match_the_attempt_transcript() {
    let mut r = fixture();
    r["counters"]["aps_f4_multiplications_uncharged"] = json!(1);
    let r = audit::record_line(&seal(r)).unwrap();
    assert!(audit::metrics(&r).is_err());
    let mut r = fixture();
    r["detail"]["attempts"][0]["algebra"]["roots"] = Value::Null;
    assert!(audit::metrics(&r).is_err());
    r["counters"]["aps_legacy_root_count_unknown_attempts"] = json!(1);
    assert!(audit::metrics(&r).is_ok());
}

#[test]
fn a_refrozen_plan_cannot_duplicate_an_execution_cell() {
    let d = tree();
    let mut p: Value =
        serde_json::from_slice(&fs::read(d.path().join("plan.json")).unwrap()).unwrap();
    let mut extra = p["executions"][0].clone();
    extra["seq"] = json!(3);
    p["executions"].as_array_mut().unwrap().push(extra);
    write(d.path(), "plan.json", &p);
    let mut proto: Value =
        serde_json::from_slice(&fs::read(d.path().join("protocol.json")).unwrap()).unwrap();
    proto["extensions"]["aps_export"]["plan_sha256"] =
        json!(hash(&fs::read(d.path().join("plan.json")).unwrap()));
    proto["freeze"]["protocol_sha256"] = json!(protocol_hash(&proto).unwrap());
    write(d.path(), "protocol.json", &proto);
    assert!(build(&d).is_err());
}

#[test]
fn restore_evidence_cannot_disagree_with_the_resume_count() {
    let d = tree();
    let mut h: Value =
        serde_json::from_slice(&fs::read(d.path().join("history.json")).unwrap()).unwrap();
    h["events"][0]["checkpoint"] =
        json!({"policy":"criu","generations_written":1,"restored_from_generation":0});
    write(d.path(), "history.json", &h);
    assert!(build(&d).is_err());
    h["events"][0]["resumed"] = json!(1);
    write(d.path(), "history.json", &h);
    let e = build(&d).unwrap();
    assert_eq!(e["flow"]["resumed"], 1);
    assert_eq!(e["runs"][0]["measurements"]["wall_valid"], false);
}

#[test]
fn unsupported_configurations_are_rejected_even_without_attempts() {
    for (key, invalid) in [
        ("attempts", json!("0")),
        ("attempts", json!("129")),
        ("degree", json!("25")),
        ("budget_ms", json!("0")),
        ("budget_ms", json!("60001")),
        ("sampling_draws", json!("10001")),
        ("sampling_draws", json!(1)),
        ("beta", json!("1")),
        ("lambda", json!("1")),
        ("labels", json!("unknown")),
        ("roots", json!("unknown")),
        ("stop", json!("unknown")),
        ("orbit", json!("unknown")),
        ("prune", json!("unknown")),
    ] {
        let mut r = fixture();
        r["method"]["params"][key] = invalid;
        assert!(math(&r).is_err(), "accepted invalid {key}");
        no_witness(&mut r);
        r["detail"]["attempts"] = json!([]);
        r["detail"]["target_error"] = json!("retained conversion failure");
        r["counters"]["aps_attempts"] = json!(0);
        assert!(math(&r).is_err(), "ignored invalid {key} on target error");
    }
    let mut r = fixture();
    r["method"]["params"]["roots"] = json!("legacy");
    r["method"]["params"]["stop"] = json!("first_informative");
    assert!(math(&r).is_err());
    r = fixture();
    r["method"]["params"]["orbit"] = json!("cubic");
    r["method"]["params"]["encoding"] = json!("expanded4");
    assert!(math(&r).is_err());
}

fn inclusive_tree() -> (tempfile::TempDir, Value) {
    let d = tempfile::tempdir().unwrap();
    let r = audit::record_line(&seal(fixture())).unwrap();
    let mut profiled = r.clone();
    profiled["time"]["user_ns"] = json!(21);
    fs::write(d.path().join("profiled.jsonl"), seal(profiled)).unwrap();
    let part = |name: &str, trigger: &str, ir: u64| {
        let bytes = format!(
            "desc: Trigger: Client Request: {trigger}\nevents: Ir\nsummary: {ir}\ntotals: {ir}\n"
        );
        fs::write(d.path().join(name), bytes.as_bytes()).unwrap();
        json!({"path":name,"sha256":hash(bytes.as_bytes())})
    };
    let method = vec![
        part("method.1", "ecbench_before_solve", 10),
        part("method.2", "ecbench_before_online", 2),
        part("method.3", "phase", 3),
        part("method.4", "ecbench_online", 5),
        part("method.5", "ecbench_solve", 7),
    ];
    let scalar = vec![
        part("scalar.1", "ecbench_before_solve", 4),
        part("scalar.2", "ecbench_solve", 11),
    ];
    let mut receipt = audit::scalar_record(&r).unwrap();
    receipt["binary_sha256"] = json!("1".repeat(64));
    write(d.path(), "scalar.json", &receipt);
    let packet = json!({"schema":"aps.inclusive-ir/v1","record_id":r["record_id"],"scope":"method_plus_independent_scalar_check", "thread_mode":"combined",
        "source_commit":"0".repeat(40),"protocol_sha256":"2".repeat(64),"binary_sha256":r["binary_sha256"],
        "profiled_record":"profiled.jsonl","trajectory_sha256":inclusive::trajectory(&r).unwrap(),"method_parts":method,"scalar_parts":scalar,
        "verifier_binary_sha256":"1".repeat(64),"verifier_receipt":"scalar.json"});
    write(d.path(), "inclusive.json", &packet);
    (d, r)
}

#[test]
fn inclusive_method_and_scalar_cost_have_one_closed_unit() {
    let (d, r) = inclusive_tree();
    let root = EvidenceRoot::new(d.path()).unwrap();
    let (v, pins) = inclusive::read(&root, "inclusive.json", &r).unwrap();
    assert_eq!(v["method_ir"], 17);
    assert_eq!(v["independent_scalar_check_ir"], 11);
    assert_eq!(v["total_ir"], 28);
    assert_eq!(v["method_online_ir"], 8);
    assert_eq!(v["wall_qualified"], false);
    assert_eq!(v["execution_attested"], false);
    assert_eq!(pins.len(), 10);
}
#[test]
fn profiling_cannot_change_a_search_trajectory() {
    let (d, r) = inclusive_tree();
    let mut replay: Value =
        serde_json::from_str(&fs::read_to_string(d.path().join("profiled.jsonl")).unwrap())
            .unwrap();
    replay["counters"]["aps_attempts"] = json!(2);
    fs::write(d.path().join("profiled.jsonl"), seal(replay)).unwrap();
    assert!(inclusive::read(&EvidenceRoot::new(d.path()).unwrap(), "inclusive.json", &r).is_err());
}
#[test]
fn missing_or_changed_ir_parts_cannot_be_filled_with_zero() {
    let (d, r) = inclusive_tree();
    let mut packet: Value =
        serde_json::from_slice(&fs::read(d.path().join("inclusive.json")).unwrap()).unwrap();
    packet["method_parts"].as_array_mut().unwrap().remove(2);
    write(d.path(), "inclusive.json", &packet);
    assert!(inclusive::read(&EvidenceRoot::new(d.path()).unwrap(), "inclusive.json", &r).is_err());
    let (d, r) = inclusive_tree();
    fs::write(d.path().join("scalar.2"), "changed").unwrap();
    assert!(inclusive::read(&EvidenceRoot::new(d.path()).unwrap(), "inclusive.json", &r).is_err());
}
#[test]
fn inclusive_counts_preserve_integers_above_binary64_precision() {
    let (d, r) = inclusive_tree();
    let mut p: Value =
        serde_json::from_slice(&fs::read(d.path().join("inclusive.json")).unwrap()).unwrap();
    let big = (1u64 << 53) + 100;
    let raw = format!("desc: Trigger: Client Request: ecbench_solve\nevents: Ir\nsummary: {big}\n");
    fs::write(d.path().join("method.5"), &raw).unwrap();
    p["method_parts"][4]["sha256"] = json!(hash(raw.as_bytes()));
    write(d.path(), "inclusive.json", &p);
    let (v, _) =
        inclusive::read(&EvidenceRoot::new(d.path()).unwrap(), "inclusive.json", &r).unwrap();
    assert_eq!(v["method_ir"], (big + 10).to_string());
    assert_eq!(v["total_ir"], (big + 21).to_string());
}
#[test]
fn a_required_but_missing_cost_preserves_rows_and_blocks_qualification() {
    let d = tree();
    let mut p: Value =
        serde_json::from_slice(&fs::read(d.path().join("protocol.json")).unwrap()).unwrap();
    p["extensions"]["aps_export"]["require_inclusive_ir"] = json!(true);
    p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
    write(d.path(), "protocol.json", &p);
    let e = build(&d).unwrap();
    assert_eq!(e["runs"].as_array().unwrap().len(), 3);
    assert_eq!(
        e["backend_record"]["missing_inclusive_ir_slots"],
        json!([0, 1, 2])
    );
    assert!(package::check(&EvidenceRoot::new(d.path()).unwrap(), &e).is_err());
}
#[test]
fn an_unknown_aps_version_cannot_escape_audit() {
    let mut r = fixture();
    r["detail"]["schema"] = json!("ecbench.aps/unknown");
    assert!(audit::metrics(&r).is_err());
    assert!(math(&r).is_err());
}

#[test]
fn native_export_reconstructs_all_instruction_rows_and_pins_the_frozen_protocol() {
    let d = tree();
    let (sample, _) = inclusive_tree();
    let mut protocol: Value =
        serde_json::from_slice(&fs::read(d.path().join("protocol.json")).unwrap()).unwrap();
    protocol["extensions"]["aps_export"]["require_inclusive_ir"] = json!(true);
    protocol["freeze"]["protocol_sha256"] = json!(protocol_hash(&protocol).unwrap());
    write(d.path(), "protocol.json", &protocol);
    let records = fs::read_to_string(d.path().join("records.jsonl")).unwrap();
    let mut bindings = vec![];
    for (seq, line) in records.lines().enumerate() {
        let record = audit::record_line(line).unwrap();
        let dir = format!("ir-{seq}");
        fs::create_dir(d.path().join(&dir)).unwrap();
        let mut packet: Value =
            serde_json::from_slice(&fs::read(sample.path().join("inclusive.json")).unwrap())
                .unwrap();
        packet["record_id"] = record["record_id"].clone();
        packet["protocol_sha256"] = protocol["freeze"]["protocol_sha256"].clone();
        packet["trajectory_sha256"] = json!(inclusive::trajectory(&record).unwrap());
        for key in ["method_parts", "scalar_parts"] {
            for part in packet[key].as_array_mut().unwrap() {
                let old = text(&part["path"]).unwrap().to_string();
                let new = format!("{dir}/{old}");
                fs::copy(sample.path().join(&old), d.path().join(&new)).unwrap();
                part["path"] = json!(new);
            }
        }
        let mut profiled = record.clone();
        profiled["time"]["user_ns"] = json!(21);
        let profile_path = format!("{dir}/profiled.jsonl");
        fs::write(d.path().join(&profile_path), seal(profiled)).unwrap();
        packet["profiled_record"] = json!(profile_path);
        let mut receipt = audit::scalar_record(&record).unwrap();
        receipt["binary_sha256"] = packet["verifier_binary_sha256"].clone();
        let receipt_path = format!("{dir}/scalar.json");
        write(d.path(), &receipt_path, &receipt);
        packet["verifier_receipt"] = json!(receipt_path);
        let path = format!("{dir}/inclusive.json");
        write(d.path(), &path, &packet);
        bindings.push(json!({"seq":seq,"path":path}));
    }
    let mut request: Value =
        serde_json::from_slice(&fs::read(d.path().join("request.json")).unwrap()).unwrap();
    request["inclusive_profiles"] = json!(bindings);
    write(d.path(), "request.json", &request);
    let export = build(&d).unwrap();
    assert_eq!(export["runs"].as_array().unwrap().len(), 3);
    for row in export["runs"].as_array().unwrap() {
        assert_eq!(row["measurements"]["counted"]["total_ir"], 28);
    }
    let root = EvidenceRoot::new(d.path()).unwrap();
    package::check(&root, &export).unwrap();
    let mut bad: Value = root.json("ir-1/inclusive.json").unwrap();
    bad["protocol_sha256"] = json!("f".repeat(64));
    write(d.path(), "ir-1/inclusive.json", &bad);
    assert!(build(&d).is_err());
    bad["protocol_sha256"] = protocol["freeze"]["protocol_sha256"].clone();
    bad["method_parts"] = root.json("ir-0/inclusive.json").unwrap()["method_parts"].clone();
    write(d.path(), "ir-1/inclusive.json", &bad);
    assert!(
        build(&d).is_err(),
        "a second execution cannot reuse the first execution's raw frame paths"
    );
}

#[cfg(unix)]
#[test]
fn scalar_frame_cannot_alias_method_parts_through_local_symlinks() {
    let (d, r) = inclusive_tree();
    let mut packet: Value =
        serde_json::from_slice(&fs::read(d.path().join("inclusive.json")).unwrap()).unwrap();
    for (dst, src, index) in [("scalar.1", "method.1", 0), ("scalar.2", "method.5", 1)] {
        fs::remove_file(d.path().join(dst)).unwrap();
        std::os::unix::fs::symlink(src, d.path().join(dst)).unwrap();
        packet["scalar_parts"][index]["sha256"] =
            json!(hash(&fs::read(d.path().join(src)).unwrap()));
    }
    write(d.path(), "inclusive.json", &packet);
    assert!(inclusive::read(&EvidenceRoot::new(d.path()).unwrap(), "inclusive.json", &r).is_err());
}
