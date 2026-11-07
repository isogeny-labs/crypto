mod support;
use ml_dataset_independent_consumer::{audit, common};
use std::{fs, process::Command};

fn rewrite_member(root: &std::path::Path, name: &str, value: serde_json::Value) {
    let cell = root.join("cells/000000000000");
    let raw = serde_json::to_vec(&value).unwrap();
    fs::write(cell.join(name), &raw).unwrap();
    let mut manifest = common::load(&cell.join("manifest.json")).unwrap();
    let artifact = manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["path"] == name)
        .unwrap();
    artifact["sha256"] = common::digest(&raw).into();
    artifact["bytes"] = raw.len().into();
    fs::write(
        cell.join("manifest.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
}

#[test]
fn provenance_transplants_and_false_completion_reject_even_with_valid_hashes() {
    for case in ["private_seed", "public_m", "empty_base"] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        support::fixture(&root, "2").unwrap();
        let name = match case {
            "private_seed" => "witness.json",
            "public_m" => "public.json",
            _ => "factor-base.json",
        };
        let mut value = common::load(&root.join("cells/000000000000").join(name)).unwrap();
        match case {
            "private_seed" => value["sampling_provenance"]["seed"] = 99.into(),
            "public_m" => value["m"] = 15.into(),
            _ => value["points"] = serde_json::json!([]),
        }
        rewrite_member(&root, name, value);
        assert!(audit::audit(&root, None, false).is_err(), "accepted {case}");
    }
}

#[test]
fn contradictory_outcomes_and_duplicate_members_reject() {
    for duplicate in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("run");
        support::fixture(&root, "2").unwrap();
        let path = root.join("cells/000000000000/manifest.json");
        let mut value = common::load(&path).unwrap();
        if duplicate {
            let first = value["artifacts"][0].clone();
            value["artifacts"].as_array_mut().unwrap().push(first);
        } else {
            value["outcome"]["exhaustive_no_decomposition"] = true.into();
        }
        fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(audit::audit(&root, None, false).is_err());
    }
}

#[test]
fn incomplete_attempts_and_empty_arithmetic_coverage_remain_explicit() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "2").unwrap();
    fs::create_dir(root.join("cells/000000000001")).unwrap();
    let path = root.join("cells/000000000000/manifest.json");
    let mut manifest = common::load(&path).unwrap();
    manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .retain(|a| a["path"] != "recovery.json");
    manifest["outcome"] = serde_json::json!({"status":"timeout","solver_completed":false});
    fs::remove_file(root.join("cells/000000000000/recovery.json")).unwrap();
    fs::write(path, serde_json::to_vec(&manifest).unwrap()).unwrap();
    let (report, rows) = audit::audit(&root, None, true).unwrap();
    assert_eq!(
        report["incomplete_attempts"],
        serde_json::json!(["000000000001"])
    );
    assert_eq!(report["positive_certificates_arithmetic_checked"], 0);
    assert_eq!(report["mathematics_independently_verified"], false);
    assert_eq!(rows[0]["outcome"]["status"], "timeout");
    assert!(audit::audit(&root, Some(&"0".repeat(64)), false).is_err());
}

#[test]
fn complete_native_arithmetic_export_keeps_labels_separate() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "2").unwrap();
    let (report, rows) = audit::audit(&root, None, true).unwrap();
    assert_eq!(report["checked_cells"], 1);
    assert_eq!(report["positive_certificates_arithmetic_checked"], 1);
    assert_eq!(report["mathematics_independently_verified"], false);
    let output = temp.path().join("routing.jsonl");
    audit::export_rows(&rows, &output).unwrap();
    let record = common::load(&output).unwrap();
    assert_eq!(
        record["labels"]["positive_certificate_arithmetic"][0]["checks"]["group_relation"],
        true
    );
    assert_eq!(record["labels"]["independently_verified"], false);
    let features = serde_json::to_string(&record["features"]).unwrap();
    for excluded in [
        "PRIVATE-SCALAR",
        "scalar",
        "sampling_provenance",
        "seed",
        "group_relation",
    ] {
        assert!(!features.contains(excluded), "leaked {excluded}");
    }
    assert!(audit::export_rows(&rows, &output).is_err());
}

#[test]
fn false_positive_rehashed_certificate_fails_arithmetic() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "3").unwrap();
    audit::audit(&root, None, false).unwrap();
    assert!(audit::audit(&root, None, true).is_err());
}

#[test]
fn cli_requires_preview_and_rejects_corruption() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "2").unwrap();
    let exe = env!("CARGO_BIN_EXE_dataset-consumer");
    let output = temp.path().join("routing.jsonl");
    let denied = Command::new(exe)
        .args(["export", "--run"])
        .arg(&root)
        .arg("--out")
        .arg(&output)
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert!(!output.exists());
    let allowed = Command::new(exe)
        .args([
            "export",
            "--arithmetic",
            "--allow-unreplayed-preview",
            "--run",
        ])
        .arg(&root)
        .arg("--out")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        allowed.status.success(),
        "{}",
        String::from_utf8_lossy(&allowed.stderr)
    );
    fs::write(root.join("cells/000000000000/stdout.log"), b"corrupt").unwrap();
    let corrupt = Command::new(exe)
        .args(["audit", "--run"])
        .arg(&root)
        .output()
        .unwrap();
    assert!(!corrupt.status.success());
}

#[test]
fn fixture_publication_has_exact_artifacts_and_explicit_scope() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("run");
    support::fixture(&root, "2").unwrap();
    let receipt =
        support::receipt(&root, "s3://setec-astronomy-artifacts/software-fixture").unwrap();
    assert_eq!(receipt["software_fixture"], true);
    assert_eq!(receipt["scientific_observation"], false);
    assert_eq!(receipt["objects"].as_array().unwrap().len(), 13);
}
