//! Analysis-only fixture. No instrument or solver is invoked.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, process::Command};
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
#[test]
fn explicit_gate_cli_and_same_count_membership_rejection() {
    let dir = std::env::temp_dir().join(format!("pencil-gate-cli-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let plan = dir.join("gate-p739.json");
    let bytes = include_bytes!("fixtures/gate-p739.json");
    fs::write(&plan, bytes).unwrap();
    let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":60,"planned_executions":1080,"measured_executions":900,"warmup_executions":180,"solver_executions_performed":0,"binary_sha256":"b".repeat(64),"files":{"gate-p739.json":sha(bytes)}});
    let rb = serde_json::to_vec(&receipt).unwrap();
    let rp = dir.join("receipt.json");
    fs::write(&rp, &rb).unwrap();
    let mut manifest = json!({"schema":"pencil-price-cohort-input/v1","analysis_scope":pencil_accounting_price_draft::aggregate::p739_gate60_scope(),"prepared_plan_validation":{"path":rp,"sha256":sha(&rb)},"binary_sha256":"b".repeat(64),"protocol_sha256":"a".repeat(64),"coverage_review":null,"plans":[{"name":"gate-p739.json","path":plan,"sha256":sha(bytes),"session_id":"software-only","records":null}]});
    let mp = dir.join("manifest.json");
    fs::write(&mp, serde_json::to_vec(&manifest).unwrap()).unwrap();
    let binary = env!("CARGO_BIN_EXE_analyze_cohort");
    let out = Command::new(binary)
        .arg(&mp)
        .args(["--p739-gate60", "--summary"])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["expected_measured_pairs"], 300);
    assert_eq!(v["missing_executions_count"], 1080);
    assert!(!Command::new(binary)
        .arg(&mp)
        .output()
        .unwrap()
        .status
        .success());
    let mut edited: Value = serde_json::from_slice(bytes).unwrap();
    edited["workloads"][0]["workload_id"] = json!("different-public-target");
    let replacement = serde_json::to_vec(&edited).unwrap();
    fs::write(&plan, &replacement).unwrap();
    manifest["plans"][0]["sha256"] = json!(sha(&replacement));
    assert!(pencil_accounting_price_draft::aggregate::aggregate_p739_gate60(&manifest).is_err());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn explicit_upper_gate_cli_is_distinct_and_flags_are_exclusive() {
    let dir = std::env::temp_dir().join(format!("pencil-upper-gate-cli-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let plan = dir.join("gate-p186379-target0.json");
    let bytes = include_bytes!("fixtures/gate-p186379-target0.json");
    fs::write(&plan, bytes).unwrap();
    let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":1,"planned_executions":18,"measured_executions":15,"warmup_executions":3,"solver_executions_performed":0,"binary_sha256":"b".repeat(64),"files":{"gate-p186379-target0.json":sha(bytes)}});
    let rb = serde_json::to_vec(&receipt).unwrap();
    let rp = dir.join("receipt.json");
    fs::write(&rp, &rb).unwrap();
    let manifest = json!({"schema":"pencil-price-cohort-input/v1","analysis_scope":pencil_accounting_price_draft::aggregate::p186379_target0_scope(),"prepared_plan_validation":{"path":rp,"sha256":sha(&rb)},"binary_sha256":"b".repeat(64),"protocol_sha256":"a".repeat(64),"coverage_review":null,"plans":[{"name":"gate-p186379-target0.json","path":plan,"sha256":sha(bytes),"session_id":"software-only","records":null}]});
    let mp = dir.join("manifest.json");
    fs::write(&mp, serde_json::to_vec(&manifest).unwrap()).unwrap();
    let binary = env!("CARGO_BIN_EXE_analyze_cohort");
    let out = Command::new(binary)
        .arg(&mp)
        .args(["--p186379-target0", "--summary"])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["expected_workloads"], 1);
    assert_eq!(v["expected_measured_pairs"], 5);
    assert_eq!(v["missing_executions_count"], 18);
    assert!(!Command::new(binary)
        .arg(&mp)
        .args(["--p186379-target0", "--p739-gate60"])
        .output()
        .unwrap()
        .status
        .success());
    fs::remove_dir_all(dir).unwrap();
}
