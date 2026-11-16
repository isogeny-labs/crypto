//! CLI contract checks on existing development fixtures, not a timing campaign.
use certified_relations::{workload, System};
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

fn call(engine: &str, system: &System, extra: &[&str]) -> (bool, Value) {
    let binary = std::env::var_os("RELATION_ROOT_WORKER_BINARY")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_native-root-worker").into());
    let mut child = Command::new(binary)
        .arg(engine)
        .args(extra)
        .env("RAYON_NUM_THREADS", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(system).unwrap())
        .unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.stderr.is_empty());
    (
        result.status.success(),
        serde_json::from_slice(&result.stdout).unwrap(),
    )
}
#[test]
fn actual_root_arms_agree_on_complete_development_basis() {
    let system = workload::generate(2147483647, 3, 7).unwrap().system;
    let (control_ok, control) = call("f4", &system, &[]);
    let (candidate_ok, candidate) = call("certified", &system, &[]);
    let (compact_ok, compact) = call("compact", &system, &[]);
    let (all_ok, all) = call("compact-all-vertices", &system, &[]);
    assert!(control_ok && candidate_ok && compact_ok && all_ok);
    assert_eq!(control["status"], "complete");
    assert_eq!(candidate["status"], "complete");
    assert!(control["observed_solving_degree"].is_u64());
    assert!(candidate["observed_solving_degree"].is_null());
    assert!(!control["basis"].as_array().unwrap().is_empty());
    assert_eq!(control["basis"], candidate["basis"]);
    assert_eq!(control["basis"], compact["basis"]);
    assert_eq!(compact["basis"], all["basis"]);
    assert_eq!(all["engine"], "compact-all-vertices");
    assert!(all["observed_solving_degree"].is_null());
    assert_eq!(
        compact["quotient_info"]["projection_work"]["edge_resultants"],
        2
    );
    assert_eq!(
        all["quotient_info"]["projection_work"]["edge_resultants"],
        3
    );
    assert_eq!(
        compact["quotient_info"]["projection_work"]["vertex_gcd_folds"],
        1
    );
    assert_eq!(
        all["quotient_info"]["projection_work"]["vertex_gcd_folds"],
        3
    );
    assert!(compact["observed_solving_degree"].is_null());
    assert!(compact["quotient_info"]["stored_proof_terms"].is_u64());
    assert!(candidate["quotient_info"].is_object());
    assert_eq!(candidate["requires_matched_external_basis_check"], true);
    // Clocks exist for later protocol use, but their values are not compared.
    assert!(control["clock_ns"]["input_parse_admission_conversion"].is_u64());
    assert!(candidate["clock_ns"]["construction_and_internal_checks"].is_u64());
}
#[test]
fn wide_control_is_explicitly_unsupported() {
    let system = workload::generate(9007199254740881, 3, 7).unwrap().system;
    let (ok, result) = call("f4", &system, &[]);
    assert!(!ok);
    assert_eq!(result["status"], "unsupported");
    assert!(result["basis"].is_null());
    assert!(result["observed_solving_degree"].is_null());
}
#[test]
fn capped_f4_cannot_supply_a_complete_basis_or_solving_degree() {
    let system = workload::generate(2147483647, 3, 7).unwrap().system;
    let (ok, result) = call("f4", &system, &["--degree-cap", "0"]);
    assert!(!ok);
    assert_eq!(result["status"], "incomplete");
    assert!(result["basis"].is_null());
    assert!(result["observed_solving_degree"].is_null());
    assert!(result["f4_observations"]["pairs_above_bound"]
        .as_u64()
        .is_some_and(|n| n > 0));
}
