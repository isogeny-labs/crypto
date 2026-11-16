//! Synthetic contract fixtures, not admitted experiment observations.
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    for name in [
        "request.json",
        "protocol.json",
        "plan.json",
        "records.jsonl",
        "history.json",
    ] {
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/cli")
                .join(name),
            root.path().join(name),
        )
        .unwrap();
    }
    root
}
fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_aps-measurement-export"))
        .args(args)
        .output()
        .unwrap()
}
fn build(root: &tempfile::TempDir) -> std::path::PathBuf {
    let out = root.path().join("export.json");
    let result = cli(&[
        "build",
        root.path().to_str().unwrap(),
        "request.json",
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    out
}
#[test]
fn native_cli_build_and_independent_check_close_the_file_pipeline() {
    let root = fixture();
    let out = build(&root);
    let result = cli(&["check", root.path().to_str().unwrap(), "export.json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let e: Value = serde_json::from_slice(&fs::read(out).unwrap()).unwrap();
    assert_eq!(e["runs"].as_array().unwrap().len(), 3);
    assert_eq!(e["backend_record"]["wall_qualified"], false);
}
#[test]
fn native_cli_refuses_to_overwrite_an_existing_export() {
    let root = fixture();
    let out = build(&root);
    let original = fs::read(&out).unwrap();
    let result = cli(&[
        "build",
        root.path().to_str().unwrap(),
        "request.json",
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(!result.status.success());
    assert_eq!(fs::read(&out).unwrap(), original);
}
#[test]
fn native_cli_refuses_fabricated_wall_qualification() {
    let root = fixture();
    let out = build(&root);
    let mut e: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    e["runs"][0]["measurements"]["wall_valid"] = Value::Bool(true);
    fs::write(
        root.path().join("forged.json"),
        serde_json::to_vec(&e).unwrap(),
    )
    .unwrap();
    assert!(
        !cli(&["check", root.path().to_str().unwrap(), "forged.json"])
            .status
            .success()
    );
}
#[test]
fn native_cli_refuses_changed_execution_history() {
    let root = fixture();
    build(&root);
    let p = root.path().join("history.json");
    let mut h: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    h["events"][0].as_object_mut().unwrap().remove("attempts");
    fs::write(p, serde_json::to_vec(&h).unwrap()).unwrap();
    assert!(
        !cli(&["check", root.path().to_str().unwrap(), "export.json"])
            .status
            .success()
    );
    assert!(!cli(&[
        "build",
        root.path().to_str().unwrap(),
        "request.json",
        "--out",
        root.path().join("new.json").to_str().unwrap()
    ])
    .status
    .success());
}
#[test]
fn native_cli_audits_the_literal_record_and_rejects_an_integrity_change() {
    let root = fixture();
    let raw = fs::read_to_string(root.path().join("records.jsonl")).unwrap();
    let line = raw.lines().next().unwrap();
    fs::write(root.path().join("record.json"), line).unwrap();
    let result = cli(&["audit-record", root.path().to_str().unwrap(), "record.json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["mathematical"]["in_budget_witnesses_checked"], 1);
    fs::write(
        root.path().join("record.json"),
        line.replace("unit-session", "changed-session"),
    )
    .unwrap();
    assert!(
        !cli(&["audit-record", root.path().to_str().unwrap(), "record.json"])
            .status
            .success()
    );
}
