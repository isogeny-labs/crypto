//! Opt-in interoperability against a pinned native producer, without AWS.
//! These small software fixtures are not campaign observations or benchmarks.
#![cfg(unix)]

use ml_dataset_independent_consumer::{audit, common, transport};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PRODUCER_SHA: &str = "63c95dd788f267d1c4cc6323a33b67f607390dc669730c13e99eb156a0004c07";
const PRODUCER_COMMIT: &str = "d71ec2ef5ab9b7f2a038f61c941af10297a775ad";

fn run(binary: &Path, root: &Path, name: &str, args: &[&str]) {
    let stdout = root.join(format!("{name}.stdout.json"));
    let stderr = root.join(format!("{name}.stderr.log"));
    let mut child = Command::new(binary)
        .args(args)
        .current_dir(root)
        .env_remove("ISOLAB_RESULTS_URI")
        .stdin(Stdio::null())
        .stdout(File::create(&stdout).unwrap())
        .stderr(File::create(&stderr).unwrap())
        .process_group(0)
        .spawn()
        .unwrap();
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(
                status.success(),
                "producer {name} failed: {}",
                fs::read_to_string(&stderr).unwrap()
            );
            break;
        }
        let over_output = [&stdout, &stderr]
            .iter()
            .any(|p| fs::metadata(p).unwrap().len() > 8 * 1024 * 1024);
        if started.elapsed() > Duration::from_secs(600) || over_output {
            // The fresh process group includes any producer worker descendants.
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            let _ = child.wait();
            panic!("producer {name} exceeded fixture time/output bound");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn rewrite_member(cell: &Path, name: &str, value: &Value) {
    let bytes = serde_json::to_vec(value).unwrap();
    fs::write(cell.join(name), &bytes).unwrap();
    let mut manifest = common::load(&cell.join("manifest.json")).unwrap();
    let artifact = manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["path"] == name)
        .unwrap();
    artifact["bytes"] = json!(bytes.len());
    artifact["sha256"] = json!(common::digest(&bytes));
    fs::write(
        cell.join("manifest.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "requires pinned producer binary and a fresh retained fixture directory"]
fn pinned_producer_policy_cells_audit_replay_and_export() {
    let binary =
        PathBuf::from(std::env::var_os("ML_DATASET_INTEROP_PRODUCER").expect("producer path"));
    let root =
        PathBuf::from(std::env::var_os("ML_DATASET_INTEROP_DIR").expect("fresh evidence path"));
    let frozen_at =
        std::env::var("ML_DATASET_INTEROP_FROZEN_AT").expect("actual UTC freeze timestamp");
    assert_eq!(common::hash_file(&binary).unwrap(), PRODUCER_SHA);
    assert!(
        !root.exists(),
        "never overwrite or silently resume fixture output"
    );
    fs::create_dir_all(&root).unwrap();
    let policies = json!([
        {"kind":"legacy"},
        {"kind":"sampled_x_projected_control","candidate_limit":32},
        {"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":4},
        {"kind":"binary_frobenius_union_intersection","basis":[1,2],"candidate_limit":64},
        {"kind":"sampled_x_projected_control","candidate_limit":1},
        {"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":1}
    ]);
    fs::write(
        root.join("policies.json"),
        serde_json::to_vec_pretty(&policies).unwrap(),
    )
    .unwrap();
    let body = br#"{"claims":["software interoperability only"],"purpose":"Bounded local producer and independent consumer policy fixtures","schema":"experiment.protocol/v1","scientific_observation":false}"#;
    let mut protocol: Value = serde_json::from_slice(body).unwrap();
    protocol["freeze"] = json!({"status":"frozen","filed_before_affected_data":true,
        "frozen_at":frozen_at,"protocol_sha256":common::digest(body)});
    let bytes = serde_json::to_vec_pretty(&protocol).unwrap();
    fs::write(root.join("protocol.json"), &bytes).unwrap();
    let protocol_sha = common::digest(&bytes);
    run(
        &binary,
        &root,
        "plan",
        &[
            "dataset",
            "plan",
            "--out",
            "plan.json",
            "--n",
            "5",
            "--m",
            "2",
            "--families",
            "koblitz,binary,prime",
            "--solvers",
            "enumerate,root",
            "--curve-seeds",
            "1",
            "--generators",
            "1",
            "--keys",
            "1",
            "--seed",
            "1",
            "--factor-base-size",
            "2",
            "--factor-base-policies",
            "policies.json",
            "--degree-bound",
            "3",
            "--work-limit",
            "5000",
            "--protocol-sha256",
            &protocol_sha,
        ],
    );
    run(
        &binary,
        &root,
        "generate",
        &[
            "dataset",
            "generate",
            "--plan",
            "plan.json",
            "--protocol",
            "protocol.json",
            "--out",
            "run",
            "--max-cases",
            "72",
            "--seconds-per-case",
            "5",
            "--memory-mib",
            "512",
            "--trace-mib",
            "8",
        ],
    );
    let source = root.join("run");
    let (mut report, rows) = audit::audit(&source, None, true).unwrap();
    assert_eq!(report["checked_cells"], 72);
    assert!(
        report["positive_certificates_arithmetic_checked"]
            .as_u64()
            .unwrap()
            > 0
    );
    transport::replay_native(&source, &binary, &mut report, &rows).unwrap();
    let mut kinds = BTreeSet::new();
    let mut statuses = BTreeSet::new();
    let mut twists = BTreeSet::new();
    let mut root_records = 0;
    for row in &rows {
        kinds.insert(
            row["factor_base"]["policy"]["kind"]
                .as_str()
                .unwrap()
                .to_owned(),
        );
        statuses.insert(
            row["factor_base"]["construction_status"]
                .as_str()
                .unwrap()
                .to_owned(),
        );
        twists.insert(row["config"]["twist"].as_bool().unwrap());
        root_records += usize::from(
            row["artifact_roles"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r == "prime-root-certificates/v1"),
        );
        for check in row["positive_certificate_arithmetic"].as_array().unwrap() {
            for excluded in [
                "factor_base_construction",
                "subgroup_order",
                "unsat",
                "instructions",
                "fall_degree",
            ] {
                assert_eq!(check["checks"][excluded], false);
            }
        }
    }
    assert_eq!(kinds.len(), 4);
    assert_eq!(twists.len(), 2);
    for status in ["complete", "capped", "unsupported"] {
        assert!(statuses.contains(status));
    }
    assert!(root_records > 0);
    let export = root.join("static-view.jsonl");
    audit::export_rows(&rows, &export).unwrap();
    let mut excluded = 0;
    for line in fs::read_to_string(&export).unwrap().lines() {
        let record: Value = serde_json::from_str(line).unwrap();
        assert_eq!(record["schema"], "ml-dataset.static-routing-view/v2");
        assert_eq!(
            record["producer_training_export_equivalence_verified"],
            false
        );
        assert_eq!(
            record["labels"]["root_sets_independently_verified"],
            record["labels"]["root_certificate_arithmetic"]["all_supplied_root_sets_verified"]
                == true
        );
        assert_ne!(
            record["labels"]["root_certificate_arithmetic"]["execution_trace_complete_verified"],
            true
        );
        assert_ne!(
            record["labels"]["root_certificate_arithmetic"]["total_operation_count_verified"],
            true
        );
        let features = serde_json::to_string(&record["features"]).unwrap();
        for forbidden in [
            "sampling_seed",
            "construction_context",
            "construction_counters",
            "construction_candidates",
            "scalar",
            "outcome",
            "root_certificate_arithmetic",
        ] {
            assert!(!features.contains(forbidden), "leaked {forbidden}");
        }
        if record["feature_status"] == "unsupported_factor_base_policy" {
            excluded += 1;
            assert!(record["features"].is_null());
        } else {
            assert!(record["features"]["factor_base"]["policy"].is_object());
        }
    }
    assert!(excluded > 0);
    // Mutate a separate copy of one sealed cell; original producer bytes stay intact.
    let negative = root.join("negative-run");
    fs::create_dir_all(negative.join("cells/000000000000")).unwrap();
    for name in ["run.json", "plan.json", "protocol.json"] {
        fs::copy(source.join(name), negative.join(name)).unwrap();
    }
    let original_cell = source.join("cells/000000000000");
    let cell = negative.join("cells/000000000000");
    let original_manifest = common::load(&original_cell.join("manifest.json")).unwrap();
    for member in original_manifest["artifacts"].as_array().unwrap() {
        let name = member["path"].as_str().unwrap();
        let destination = cell.join(name);
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::copy(original_cell.join(name), destination).unwrap();
    }
    fs::copy(
        original_cell.join("manifest.json"),
        cell.join("manifest.json"),
    )
    .unwrap();
    let base = common::load(&cell.join("factor-base.json")).unwrap();
    for (key, value) in [
        (
            "policy",
            json!({"kind":"sampled_x_projected_control","candidate_limit":32}),
        ),
        (
            "construction_context",
            json!({"sampling_seed":"0","sampling_algorithm":"not-used-by-this-policy","seed_access":"restricted"}),
        ),
        ("construction_status", json!("unsupported")),
    ] {
        let mut bad = base.clone();
        bad[key] = value;
        rewrite_member(&cell, "factor-base.json", &bad);
        assert!(
            audit::audit(&negative, None, false).is_err(),
            "accepted rehashed {key} substitution"
        );
    }
    rewrite_member(&cell, "factor-base.json", &base);
    audit::audit(&negative, None, true).unwrap();
    assert_eq!(common::hash_file(&binary).unwrap(), PRODUCER_SHA);
    report["software_fixture"] = json!(true);
    report["scientific_observation"] = json!(false);
    report["producer_binary_sha256"] = json!(PRODUCER_SHA);
    report["producer_source_commit"] =
        common::load(&source.join("run.json")).unwrap()["source_commit"].clone();
    assert_eq!(report["producer_source_commit"], PRODUCER_COMMIT);
    report["root_artifact_cells_audited"] = json!(root_records);
    assert!(
        report["completed_distinct_root_sets_checked"]
            .as_u64()
            .unwrap()
            > 0
    );
    report["unsupported_feature_rows"] = json!(excluded);
    report["static_view_sha256"] = json!(common::hash_file(&export).unwrap());
    report["rehashed_policy_seed_status_substitutions_rejected"] = json!(true);
    fs::write(
        root.join("interop-report.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
}
