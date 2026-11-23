use serde_json::{json, Value};
use std::{fs, process::Command};
#[test]
fn nonbijective_cli_fails_and_retains_diagnostics() {
    // Software fixture only: no mathematical solver or measured workload.
    let dir = std::env::temp_dir().join(format!("p739-enrollment-cli-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let workloads: Vec<_> = (0..60)
        .map(|i| json!({"workload_id":format!("fixture-{i}")}))
        .collect();
    let arms = json!([{"name":"candidate"},{"name":"rho"},{"name":"rho_AA"}]);
    let mut executions = Vec::new();
    for workload in 0..60 {
        for arm in 0..3 {
            for round in 0..6 {
                executions.push(json!({"seq":executions.len(),"workload":workload,"arm":arm,"round":round,"warmup":round==0,"algorithm_seed":u64::MAX-round}));
            }
        }
    }
    let gate = json!({"schema":"ecbench.plan/v1","workloads":workloads,"arms":arms,"executions":executions});
    let mut cohort = gate.clone();
    cohort["executions"].as_array_mut().unwrap().pop();
    for (name, value) in [("gate", &gate), ("cohort", &cohort), ("fresh", &gate)] {
        fs::write(dir.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_p739-enrollment-audit"))
        .arg(dir.join("gate"))
        .arg(dir.join("cohort"))
        .arg(dir.join("fresh"))
        .arg(dir.join("output"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("verification failed"));
    let report: Value =
        serde_json::from_slice(&fs::read(dir.join("output/audit.json")).unwrap()).unwrap();
    assert_eq!(report["gate_to_cohort16"]["bijection"], false);
    assert_eq!(report["gate_to_cohort16"]["missing"], 1);
    assert!(dir.join("output/report.pdf").is_file());
    fs::remove_dir_all(dir).unwrap();
}
