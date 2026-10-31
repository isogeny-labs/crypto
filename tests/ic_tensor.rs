//! `ic tensor`: the ecdl-tensor-ic crate through the research CLI.
use serde_json::Value;
use std::{path::PathBuf, process::Command};

fn ic_binary() -> PathBuf {
    option_env!("CARGO_BIN_EXE_ic")
        .or(option_env!("IC_TEST_BINARY"))
        .map(PathBuf::from)
        .expect("CLI contracts require Cargo's integration-test executable or IC_TEST_BINARY pointing to a freshly built ic; see docs/ic/cli-contract-tests.md")
}

fn ic(args: &[&str]) -> (bool, Value) {
    let output = Command::new(ic_binary())
        .args(args)
        .arg("--json")
        .output()
        .unwrap();
    let report = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&output.stderr)));
    (output.status.success(), report)
}

#[test]
fn scale_matches_the_go_table() {
    let (ok, report) = ic(&["tensor", "scale", "--bits", "64,256"]);
    assert!(ok, "{report}");
    assert_eq!(report["operation"], "tensor-scale");
    assert_eq!(report["status"], "complete");
    // Rows of `go run ./cmd/ecdlscale` on the carrier branch (commit 1c29f66).
    let lines: Vec<&str> = report["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["line"].as_str().unwrap())
        .collect();
    assert_eq!(
        lines,
        [
            "     64     8    8   16 |     35.2    218.2    217.1 |     352      15.6",
            "    256    16   16   32 |     31.4    198.4    195.9 |    1248      19.5",
        ]
    );
}

#[test]
fn scale_rejects_tiny_fields() {
    let (ok, report) = ic(&["tensor", "scale", "--bits", "3"]);
    assert!(!ok);
    assert_eq!(report["status"], "error");
}

#[test]
fn demo_recovers_the_secret() {
    let (ok, report) = ic(&["tensor", "demo", "--bits", "10", "--m", "2", "--seed", "7"]);
    assert!(ok, "{report}");
    assert_eq!(report["operation"], "tensor-demo");
    assert_eq!(report["result"]["verified"], true);
    assert_eq!(report["result"]["recovered"], report["result"]["expected"]);
    assert_eq!(report["network"]["is_graph"], true);
}
