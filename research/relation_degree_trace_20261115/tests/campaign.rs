//! Regression for configurable baseline dispatch, including the A/A arm.
//! The fake worker returns fixed cost records; this is not a performance run.
#![cfg(unix)]
use serde_json::{json, Value};
use std::{fs, os::unix::fs::PermissionsExt, process::Command};
fn synthetic_campaign(label: &str, mismatch: bool) -> (std::path::PathBuf, Value) {
    let root = std::env::temp_dir().join(format!("degree-campaign-{label}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(&root).unwrap();
    fs::write(
        root.join("fixture.json"),
        include_str!("../speed-fixtures/p2147483647-cycle2-seed101/fixture.json"),
    )
    .unwrap();
    fs::write(root.join("manifest.json"),serde_json::to_vec(&json!({"cases":[{"id":"dispatch-test","fixture":"fixture.json","variables":2,"seed":101,"prime":2147483647}]})).unwrap()).unwrap();
    let fake = root.join("fake-worker");
    let script = r##"#!/bin/sh
while [ "$#" -gt 0 ]; do
  case "$1" in --arm) shift; arm="$1" ;; esac
  shift
done
basis=1
if [ "MISMATCH" = "true" ] && [ "$arm" = "quadratic-projection-criteria" ]; then basis=2; fi
printf '{"arm":"%s","basis":[%s],"counter_build":false,"charged_ns":1000}\n' "$arm" "$basis"
"##;
    fs::write(
        &fake,
        script.replace("MISMATCH", if mismatch { "true" } else { "false" }),
    )
    .unwrap();
    fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
    let out = root.join("out");
    let r = Command::new(env!("CARGO_BIN_EXE_campaign"))
        .args([
            "--fixtures",
            root.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
            "--compare-bin",
            fake.to_str().unwrap(),
            "--backend",
            "local",
            "--baseline-arm",
            "projection-criteria",
            "--candidate-arm",
            "quadratic-projection-criteria",
            "--protocol",
            "QPROJECTION_PROTOCOL.md",
        ])
        .output()
        .unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let summary: Value =
        serde_json::from_slice(&fs::read(out.join("summary.json")).unwrap()).unwrap();
    (root, summary)
}
#[test]
fn mismatched_complete_pair_stops_before_any_control() {
    let (root, summary) = synthetic_campaign("mismatch", true);
    let case = &summary["case_summaries"][0];
    assert_eq!(case["samples"].as_array().unwrap().len(), 1);
    assert!(case["samples"][0].get("aa").is_none());
    assert_eq!(case["complete"], false);
    assert_eq!(case["tenfold_native_criterion_met"], false);
    assert!(fs::read_dir(root.join("out/dispatch-test"))
        .unwrap()
        .all(|e| !e.unwrap().file_name().to_str().unwrap().starts_with("aa-")));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn selected_baseline_is_used_for_pairs_and_controls_and_protocol_is_preserved() {
    let (root, summary) = synthetic_campaign("dispatch", false);
    assert_eq!(summary["protocol"], "QPROJECTION_PROTOCOL.md");
    let case = &summary["case_summaries"][0];
    assert_eq!(case["baseline_arm"], "projection-criteria");
    let samples = case["samples"].as_array().unwrap();
    assert_eq!(samples.len(), 21);
    let mut controls = 0;
    for sample in samples {
        assert_eq!(sample["arms"]["baseline"]["arm"], "projection-criteria");
        assert_eq!(
            sample["arms"]["candidate"]["arm"],
            "quadratic-projection-criteria"
        );
        if sample.get("aa").is_some() {
            controls += 1;
            assert_eq!(sample["aa"]["first"]["arm"], "projection-criteria");
            assert_eq!(sample["aa"]["second"]["arm"], "projection-criteria");
        }
    }
    assert_eq!(controls, 7);
    let host: Value =
        serde_json::from_slice(&fs::read(root.join("out/host.json")).unwrap()).unwrap();
    assert_eq!(host["protocol"], "QPROJECTION_PROTOCOL.md");
    fs::remove_dir_all(root).unwrap();
}
