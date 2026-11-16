#[path = "support/mod.rs"]
mod support;
use std::io::Write;
use std::process::{Command, Stdio};
fn call(bytes: &[u8]) -> (bool, serde_json::Value) {
    let binary = std::env::var_os("COMPACT_ANALYSIS_BINARY")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_compact-speed-analysis").into());
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.stderr.is_empty());
    (
        result.status.success(),
        serde_json::from_slice(&result.stdout).unwrap(),
    )
}
#[test]
fn synthetic_calculation_has_no_execution_or_family_authority() {
    let (ok, result) = call(&serde_json::to_vec(&support::synthetic()).unwrap());
    assert!(ok);
    assert_eq!(result["numerical_rule_passes"], true);
    assert_eq!(result["execution_admission_verified"], false);
    assert_eq!(result["family_confirmed"], false);
}
#[test]
fn complete_losing_case_is_reported_as_calculated_not_a_process_failure() {
    let mut case = support::synthetic();
    case.setup_wall_ns = 1_000_000;
    let (ok, result) = call(&serde_json::to_vec(&case).unwrap());
    assert!(ok);
    assert_eq!(result["numerical_rule_passes"], false);
}
#[test]
fn incomplete_and_unknown_fields_retain_failure_output() {
    let mut case = support::synthetic();
    case.blocks.pop();
    let (ok, result) = call(&serde_json::to_vec(&case).unwrap());
    assert!(!ok);
    assert!(result["comparisons"].is_null());
    let (ok, result) = call(br#"{"verified":true}"#);
    assert!(!ok);
    assert_eq!(result["status"], "input_error");
}
