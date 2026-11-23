//! Independent, fixed software fixtures. No producer execution or PMU access.
mod support;
use ml_dataset_independent_consumer::{audit, common};
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

const POLICY: &[u8] =
    include_bytes!("../../../ml_dataset_consumer_checked_phase_20261029/measurement-policy.json");
const PHASES: [&str; 4] = [
    "curve-construction",
    "known-answer-replay",
    "factor-base-replay",
    "solver",
];
fn cell(root: &Path) -> std::path::PathBuf {
    root.join("cells/000000000000")
}
fn put(root: &Path, name: &str, raw: &[u8], role: &str) {
    let path = cell(root);
    let mut m = common::load(&path.join("manifest.json")).unwrap();
    let members = m["artifacts"].as_array_mut().unwrap();
    members.retain(|a| a["path"] != name);
    support::add(&path, members, name, raw, role).unwrap();
    fs::write(path.join("manifest.json"), serde_json::to_vec(&m).unwrap()).unwrap();
}
fn put_json(root: &Path, name: &str, value: &Value, role: &str) {
    put(root, name, &serde_json::to_vec(value).unwrap(), role);
}
fn remove(root: &Path, name: &str) {
    let path = cell(root);
    let mut m = common::load(&path.join("manifest.json")).unwrap();
    m["artifacts"]
        .as_array_mut()
        .unwrap()
        .retain(|a| a["path"] != name);
    fs::remove_file(path.join(name)).unwrap();
    fs::write(path.join("manifest.json"), serde_json::to_vec(&m).unwrap()).unwrap();
}
fn setup(root: &Path, protocol_index: usize) -> Value {
    let vectors: Value = serde_json::from_slice(include_bytes!(
        "../../../ml_dataset_consumer_checked_phase_20261029/protocol-canonical-vectors.json"
    ))
    .unwrap();
    let raw = vectors["positive"][protocol_index]["input_protocol_utf8"]
        .as_str()
        .unwrap()
        .as_bytes();
    setup_with_protocol(root, raw)
}
fn setup_with_protocol(root: &Path, raw: &[u8]) -> Value {
    support::fixture(root, "2").unwrap();
    put(root, "protocol.json", raw, "restricted-provenance");
    fs::write(root.join("protocol.json"), raw).unwrap();
    let mut plan = common::load(&root.join("plan.json")).unwrap();
    plan["protocol_sha256"] = json!(common::digest(raw));
    let plan_raw = serde_json::to_vec(&plan).unwrap();
    let plan_sha = common::digest(&plan_raw);
    put(root, "plan.json", &plan_raw, "restricted-provenance");
    fs::write(root.join("plan.json"), &plan_raw).unwrap();
    let mut run = common::load(&root.join("run.json")).unwrap();
    run["plan_sha256"] = json!(plan_sha);
    put_json(root, "run.json", &run, "restricted-provenance");
    fs::write(root.join("run.json"), serde_json::to_vec(&run).unwrap()).unwrap();
    put_json(
        root,
        "attempt.json",
        &json!({"schema":"crypto.ml-dataset.attempt/v1","plan_sha256":plan_sha,"cell_index":0}),
        "process-provenance",
    );
    let mut m = common::load(&cell(root).join("manifest.json")).unwrap();
    m["plan_sha256"] = json!(plan_sha);
    fs::write(
        cell(root).join("manifest.json"),
        serde_json::to_vec(&m).unwrap(),
    )
    .unwrap();
    put(
        root,
        "measurement-policy.json",
        POLICY,
        "restricted-measurement-policy",
    );
    binding(root)
}

#[test]
fn huge_integer_protocol_reaches_actual_audit_and_nonfinite_float_rejects() {
    for value in ["9".repeat(1000), "1e400".into()] {
        let body = format!("{{\"huge\":{value},\"schema\":\"experiment.protocol/v1\"}}");
        let protocol=format!("{{\"schema\":\"experiment.protocol/v1\",\"huge\":{value},\"freeze\":{{\"status\":\"frozen\",\"filed_before_affected_data\":true,\"frozen_at\":\"software-fixture\",\"protocol_sha256\":\"{}\"}}}}",common::digest(body.as_bytes()));
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup_with_protocol(&root, protocol.as_bytes());
        add_phase(&root, &b, 0, false);
        let output = Command::new(env!("CARGO_BIN_EXE_dataset-consumer"))
            .args(["audit", "--run"])
            .arg(&root)
            .output()
            .unwrap();
        if value == "1e400" {
            assert!(!output.status.success());
            assert!(String::from_utf8_lossy(&output.stderr).contains("nonfinite_float"));
        } else {
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let (_, rows) = audit::audit(&root, None, false).unwrap();
            assert!(
                rows[0]["checked_phase_contract"]["modeled_storage_reserved"]
                    .as_u64()
                    .unwrap()
                    > 0
            );
        }
    }
}
fn binding(root: &Path) -> Value {
    let path = cell(root);
    let run = common::load(&path.join("run.json")).unwrap();
    json!({"schema":"crypto.ml-dataset.attempt-binding/v1","run_sha256":common::hash_file(&path.join("run.json")).unwrap(),
        "plan_sha256":common::hash_file(&path.join("plan.json")).unwrap(),"protocol_sha256":common::hash_file(&path.join("protocol.json")).unwrap(),
        "config_sha256":common::hash_file(&path.join("config.json")).unwrap(),"attempt_sha256":common::hash_file(&path.join("attempt.json")).unwrap(),
        "source_commit":run["source_commit"],"binary_sha256":run["binary_sha256"],"cell_index":0})
}
fn request(event: &str) -> Value {
    let config = u64::from(event == "instructions");
    let mut attr = vec![0u8; 112];
    attr[4..8].copy_from_slice(&112u32.to_le_bytes());
    for (off, n) in [(8, config), (32, 7), (40, 97)] {
        attr[off..off + 8].copy_from_slice(&n.to_le_bytes());
    }
    json!({"event":event,"perf_type":0,"config":config,"attr_size":112,"read_format":7,"attr_flags":97,"attr_bytes_native_endian":attr,"byte_order":"little","pid":0,"cpu":-1,"group_fd":-1,"syscall_flags":8})
}
fn unavailable() -> Value {
    let events:Vec<_>=["instructions","cpu_cycles"].into_iter().map(|event|json!({"request":request(event),"kernel_event_id":null,"identity_error":null,
        "lifecycle":if event=="instructions"{json!([{"operation":"open","result":{"Err":{"errno":13,"message":"permission denied"}}}])}else{json!([])},
        "status":if event=="instructions"{"permission_denied"}else{"not_attempted"},"raw":null,"tuple_assessment":null,"read_bytes":null,"cleanup_succeeded":null})).collect();
    json!({"collector_version":"checked-native-pmu/v1","policy":{"scope":"calling_thread","cycles":"optional_with_standalone_fallback"},"owner_pid":71,"owner_tid":73,
        "context_checks":[{"stage":"start","current":{"pid":71,"tid":73},"status":"matched"},{"stage":"finish","current":{"pid":71,"tid":73},"status":"matched"}],
        "os":"linux","architecture":"x86_64","attempts":[{"mode":"instructions_and_cycles_group","events":events,"start_succeeded":false}],"selected_attempt":null,
        "whole_worker_coverage_verified":false,"descendant_lifetimes_verified":false,"complete_execution_verified":false,"hardware_independently_verified":false,"platform_admitted":false,"clock_valid_for_comparison":false})
}
fn add_phase(root: &Path, b: &Value, ordinal: usize, end: bool) {
    let phase = PHASES[ordinal];
    let start = json!({"schema":"crypto.ml-dataset.phase-start/v2","phase":phase,"ordinal":ordinal,"binding":b,"policy_sha256":common::digest(POLICY),"recording_pid":71,"usage_before":null,"counter_boundary":"durable_start_before_open"});
    let start_raw = serde_json::to_vec(&start).unwrap();
    put(
        root,
        &format!("telemetry/{phase}-start.json"),
        &start_raw,
        "phase-start",
    );
    if end {
        let m = json!({"schema":"crypto.ml-dataset.phase-metrics/v2","phase":phase,"ordinal":ordinal,"binding":b,"policy_sha256":common::digest(POLICY),"start_sha256":common::digest(&start_raw),"returned_ok":true,"returned_error":null,"counter_start_succeeded":false,"collection":unavailable(),"observations":[],"usage_after":null,"elapsed_ns_diagnostic":"0","training_cost_eligible":false,"whole_worker_retired_instructions":null,"whole_worker_coverage_verified":false,"independently_verified":false,"platform_admitted":false,"clock_valid_for_comparison":false,"interrupted_phase_readback":false,"consumer_v2_admitted":false});
        put_json(
            root,
            &format!("telemetry/{phase}.json"),
            &m,
            "hardware-and-resource-telemetry",
        );
    }
}
fn export(root: &Path, out: &Path) -> Value {
    let (_, rows) = audit::audit(root, None, true).unwrap();
    audit::export_rows(&rows, out).unwrap();
    common::load(out).unwrap()
}

#[test]
fn local_content_addressed_restore_preserves_checked_members() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    let b = setup(&root, 0);
    add_phase(&root, &b, 0, true);
    let receipt =
        support::receipt(&root, "s3://setec-astronomy-artifacts/software-fixture").unwrap();
    let objects = temp.path().join("objects");
    fs::create_dir(&objects).unwrap();
    for obj in receipt["objects"].as_array().unwrap() {
        let a = &obj["artifact"];
        fs::copy(
            cell(&root).join(a["path"].as_str().unwrap()),
            objects.join(a["sha256"].as_str().unwrap()),
        )
        .unwrap();
    }
    let mock = temp.path().join("mock-bin");
    fs::create_dir(&mock).unwrap();
    let script = mock.join("aws");
    fs::write(&script,b"#!/bin/sh\nset -eu\nkey=$8\nfile=\"$MOCK_OBJECTS/${key##*/}\"\ncase \"$4\" in\nhead-object) size=$(stat -c%s \"$file\"); printf '{\"ContentLength\":%s,\"ETag\":\"mock\"}' \"$size\";;\nget-object) cp \"$file\" \"${11}\"; printf '{}';;\n*) exit 99;;\nesac\n").unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
    let raw = serde_json::to_vec(&receipt).unwrap();
    let receipt_file = temp.path().join("receipt.json");
    fs::write(&receipt_file, &raw).unwrap();
    let restored = temp.path().join("restored");
    let output = Command::new(env!("CARGO_BIN_EXE_dataset-consumer"))
        .args(["fetch", "--receipt"])
        .arg(&receipt_file)
        .arg("--expected-sha256")
        .arg(common::digest(&raw))
        .arg("--out")
        .arg(&restored)
        .env("MOCK_OBJECTS", &objects)
        .env("PATH", format!("{}:/usr/bin:/bin", mock.display()))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (_, rows) = audit::audit(&restored, None, false).unwrap();
    assert_eq!(rows[0]["checked_phase_contract"]["supplied_ends"], 1);
    for name in [
        "measurement-policy.json",
        "telemetry/curve-construction-start.json",
        "telemetry/curve-construction.json",
    ] {
        assert_eq!(
            fs::read(cell(&root).join(name)).unwrap(),
            fs::read(cell(&restored).join(name)).unwrap()
        );
    }
}

#[test]
fn unsealed_context_is_not_a_checked_cell() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    let b = setup(&root, 0);
    add_phase(&root, &b, 0, false);
    fs::remove_file(cell(&root).join("manifest.json")).unwrap();
    let (report, rows) = audit::audit(&root, None, false).unwrap();
    assert!(rows.is_empty());
    assert_eq!(report["incomplete_attempts"], json!(["000000000000"]));
}

#[test]
fn all_frozen_protocols_bind_policy_only_and_preserve_public_features() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("legacy");
    support::fixture(&base, "2").unwrap();
    let original = export(&base, &temp.path().join("original.json"));
    assert!(original["labels"].get("checked_phase_contract").is_none());
    for index in 0..14 {
        let root = temp.path().join(index.to_string());
        setup(&root, index);
        let result = export(&root, &temp.path().join(format!("{index}.json")));
        assert_eq!(
            serde_json::to_vec(&result["features"]).unwrap(),
            serde_json::to_vec(&original["features"]).unwrap()
        );
        assert_eq!(
            result["labels"]["checked_phase_contract"]["prefix_state"],
            "policy_only"
        );
        assert_eq!(
            result["labels"]["checked_phase_contract"]["training_cost_eligible"],
            false
        );
    }
}

#[test]
fn raw_binding_distinguishes_whitespace_from_canonical_plan() {
    for name in ["run.json", "plan.json", "config.json", "attempt.json"] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup(&root, 0);
        add_phase(&root, &b, 0, false);
        let mut raw = vec![b' '];
        raw.extend(fs::read(cell(&root).join(name)).unwrap());
        put(
            &root,
            name,
            &raw,
            if name == "attempt.json" {
                "process-provenance"
            } else {
                "restricted-provenance"
            },
        );
        let e = audit::audit(&root, None, false).unwrap_err();
        assert!(e.to_string().contains("binding"), "{name}: {e:#}");
        let rebound = binding(&root);
        add_phase(&root, &rebound, 0, false);
        let (_, rows) = audit::audit(&root, None, false).unwrap();
        assert_eq!(rows[0]["checked_phase_contract"]["binding"], rebound);
    }
}

#[test]
fn attempt_index_canonical_digest_and_source_binary_transplants_reject() {
    for (file, key, value) in [
        ("attempt.json", "cell_index", json!(1)),
        ("attempt.json", "plan_sha256", json!("0".repeat(64))),
        ("run.json", "source_commit", json!("c".repeat(40))),
        ("run.json", "binary_sha256", json!("d".repeat(64))),
        ("run.json", "shard_count", json!(2)),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup(&root, 0);
        add_phase(&root, &b, 0, false);
        let mut input = common::load(&cell(&root).join(file)).unwrap();
        input[key] = value;
        put_json(&root, file, &input, "restricted-provenance");
        if file == "run.json" {
            fs::write(root.join(file), serde_json::to_vec(&input).unwrap()).unwrap();
        }
        assert!(audit::audit(&root, None, false).is_err(), "{file}/{key}");
    }
}

#[test]
fn legacy_complete_and_start_only_measurements_have_exact_golden_bytes() {
    for ended in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        support::fixture(&root, "2").unwrap();
        let start = json!({"schema":"crypto.ml-dataset.phase-start/v1","phase":"solver","diagnostic":"legacy"});
        let raw = serde_json::to_vec(&start).unwrap();
        put(&root, "telemetry/solver-start.json", &raw, "phase-start");
        let mut expected = json!({"phase":"solver","start_sha256":common::digest(&raw),"start":start,"metrics_sha256":null,"producer_reported":null,"measurement_state":"interrupted_or_unfinished","independently_verified":false});
        if ended {
            let metrics = json!({"schema":"crypto.ml-dataset.phase-metrics/v1","phase":"solver","hardware":{"instructions":0,"instructions_time_enabled_ns":1,"instructions_time_running_ns":1,"instructions_scaled":0,"multiplexed":false},"instructions_status":"measured_unmultiplexed","retired_instructions_raw":0,"retired_instructions_exact_in_scope":0,"clock_valid_for_comparison":false,"counter_scope":"legacy thread","excludes":["children"],"returned_ok":true,"returned_error":null,"interrupted_phase_readback":false});
            let raw = serde_json::to_vec(&metrics).unwrap();
            put(
                &root,
                "telemetry/solver.json",
                &raw,
                "hardware-and-resource-telemetry",
            );
            expected["metrics_sha256"] = json!(common::digest(&raw));
            expected["producer_reported"] = metrics;
            expected["measurement_state"] = json!("reported_phase_readback");
        }
        let (_, rows) = audit::audit(&root, None, false).unwrap();
        assert!(rows[0].get("checked_phase_contract").is_none());
        assert_eq!(
            serde_json::to_vec(&rows[0]["phase_measurements"]).unwrap(),
            serde_json::to_vec(&json!([expected])).unwrap()
        );
    }
}
#[test]
fn sealed_start_only_and_unavailable_end_pass_cli_without_promoting_costs() {
    for end in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup(&root, 0);
        add_phase(&root, &b, 0, end);
        for arithmetic in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_dataset-consumer"));
            command.args(["audit", "--run"]).arg(&root);
            if arithmetic {
                command.arg("--arithmetic");
            }
            let output = command.output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let out = temp.path().join("export.json");
        let denied = Command::new(env!("CARGO_BIN_EXE_dataset-consumer"))
            .args(["export", "--run"])
            .arg(&root)
            .arg("--out")
            .arg(&out)
            .output()
            .unwrap();
        assert!(!denied.status.success());
        assert!(!out.exists());
        let accepted = Command::new(env!("CARGO_BIN_EXE_dataset-consumer"))
            .args(["export", "--run"])
            .arg(&root)
            .arg("--out")
            .arg(&out)
            .arg("--allow-unreplayed-preview")
            .output()
            .unwrap();
        assert!(
            accepted.status.success(),
            "{}",
            String::from_utf8_lossy(&accepted.stderr)
        );
        let record = common::load(&out).unwrap();
        assert_eq!(
            record["labels"]["checked_phase_contract"]["supplied_ends"],
            u64::from(end)
        );
        assert!(record["labels"]["whole_worker_instruction_count"].is_null());
        assert_eq!(record["labels"]["independently_verified"], false);
    }
}
#[test]
fn rehashed_semantic_tampering_rejects_without_arithmetic_gate() {
    for case in [
        "policy",
        "binding",
        "start_hash",
        "projection",
        "claim",
        "attr",
        "context",
        "native_order",
        "missing_nullable",
        "extra_field",
        "elapsed",
        "gap",
        "end_without_start",
        "missing_policy",
        "wrong_role",
        "wrong_encoding",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup(&root, 0);
        add_phase(&root, &b, 0, true);
        let name = "telemetry/curve-construction.json";
        let mut m = common::load(&cell(&root).join(name)).unwrap();
        match case {
            "policy" => {
                put(
                    &root,
                    "measurement-policy.json",
                    b"{}",
                    "restricted-measurement-policy",
                );
            }
            "binding" => {
                m["binding"]["attempt_sha256"] = json!("0".repeat(64));
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "start_hash" => {
                m["start_sha256"] = json!("0".repeat(64));
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "projection" => {
                m["observations"] = json!([{"event":"instructions","attempt_index":0,"event_index":0,"status":"scoped_raw","raw":1,"estimate":null,"lifecycle_clean":true}]);
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "claim" => {
                m["consumer_v2_admitted"] = json!(true);
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "attr" => {
                m["collection"]["attempts"][0]["events"][0]["request"]
                    ["attr_bytes_native_endian"][111] = json!(1);
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "context" => {
                m["collection"]["context_checks"][0]["current"]["pid"] = json!(72);
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "native_order" => {
                m["collection"]["attempts"][0]["events"][0]["lifecycle"][0]["operation"] =
                    json!("read");
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "missing_nullable" => {
                m.as_object_mut().unwrap().remove("usage_after");
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "extra_field" => {
                m["extra"] = json!(false);
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "elapsed" => {
                m["elapsed_ns_diagnostic"] = json!("00");
                put_json(&root, name, &m, "hardware-and-resource-telemetry");
            }
            "gap" => add_phase(&root, &b, 2, false),
            "end_without_start" => remove(&root, "telemetry/curve-construction-start.json"),
            "missing_policy" => remove(&root, "measurement-policy.json"),
            "wrong_role" => put_json(&root, name, &m, "diagnostic"),
            _ => {
                let path = cell(&root).join("manifest.json");
                let mut v = common::load(&path).unwrap();
                for a in v["artifacts"].as_array_mut().unwrap() {
                    if a["path"] == name {
                        a["encoding"] = json!("native");
                    }
                }
                fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
            }
        }
        for arithmetic in [false, true] {
            assert!(
                audit::audit(&root, None, arithmetic).is_err(),
                "accepted {case}, arithmetic={arithmetic}"
            );
        }
    }
}
#[test]
fn missing_policy_cannot_be_hidden_by_unknown_role_or_encoding() {
    for encoding in ["json", "native"] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        let b = setup(&root, 0);
        add_phase(&root, &b, 0, false);
        remove(&root, "measurement-policy.json");
        let p = cell(&root).join("manifest.json");
        let mut m = common::load(&p).unwrap();
        for a in m["artifacts"].as_array_mut().unwrap() {
            if a["path"] == "telemetry/curve-construction-start.json" {
                a["role"] = json!("diagnostic");
                a["encoding"] = json!(encoding);
            }
        }
        fs::write(p, serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(audit::audit(&root, None, false).is_err());
    }
}
#[test]
fn opaque_legacy_diagnostics_preserve_exact_export_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "2").unwrap();
    let original = export(&root, &temp.path().join("original.json"));
    // Each mutation changes content identity, but no labels or public features.
    for (index, raw) in [
        b"not JSON".to_vec(),
        b"{\"schema\":42}".to_vec(),
        vec![b'x'; 1024 * 1024],
    ]
    .into_iter()
    .enumerate()
    {
        put(
            &root,
            "telemetry/curve-construction-start.json",
            &raw,
            "opaque-diagnostic",
        );
        let result = export(&root, &temp.path().join(format!("after-{index}.json")));
        assert_eq!(
            serde_json::to_vec(&result["features"]).unwrap(),
            serde_json::to_vec(&original["features"]).unwrap()
        );
        assert_eq!(
            serde_json::to_vec(&result["labels"]).unwrap(),
            serde_json::to_vec(&original["labels"]).unwrap()
        );
    }
}
#[test]
fn valid_error_and_private_diagnostic_perturbations_do_not_change_features() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    let b = setup(&root, 0);
    for n in 0..4 {
        add_phase(&root, &b, n, true);
    }
    let original = export(&root, &temp.path().join("before.json"));
    let name = "telemetry/solver.json";
    let mut metrics = common::load(&cell(&root).join(name)).unwrap();
    metrics["returned_ok"] = json!(false);
    metrics["returned_error"] = json!({"message_preview":"diagnostic é","message_bytes":13,"message_sha256":common::digest("diagnostic é".as_bytes()),"truncated":false});
    metrics["elapsed_ns_diagnostic"] = json!("1000000");
    put_json(&root, name, &metrics, "hardware-and-resource-telemetry");
    let changed = export(&root, &temp.path().join("after.json"));
    assert_eq!(
        serde_json::to_vec(&original["features"]).unwrap(),
        serde_json::to_vec(&changed["features"]).unwrap()
    );
    assert_eq!(
        changed["labels"]["checked_phase_contract"]["prefix_state"],
        "operation_error"
    );
}
