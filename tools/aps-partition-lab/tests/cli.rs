use std::{fs, process::Command};

#[test]
fn cli_refuses_replacement_and_keeps_unqualified_structural_output() {
    let root = std::env::temp_dir().join(format!("aps-partition-lab-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let out = root.join("report.json");
    let call = || {
        Command::new(env!("CARGO_BIN_EXE_aps-partition-lab"))
            .args([
                "structural",
                "--source-revision",
                &"a".repeat(40),
                "--source-manifest-sha256",
                &"b".repeat(64),
                "--out",
            ])
            .arg(&out)
            .output()
            .unwrap()
    };
    let first = call();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let bytes = fs::read(&out).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["performance_trial"], false);
    assert_eq!(value["production_aps_sampler_tested"], false);
    assert_eq!(value["known_label_group_control_tested"], true);
    assert_eq!(value["f4_trace_reuse_tested"], false);
    assert_eq!(value["execution_attested"], false);
    assert_eq!(value["coloring"].as_array().unwrap().len(), 12);
    assert!(!call().status.success());
    assert_eq!(fs::read(&out).unwrap(), bytes);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn cli_requires_literal_source_pins_and_explicit_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_aps-partition-lab"))
        .args(["structural", "--source-revision", "not-a-commit"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("usage:"));
}
