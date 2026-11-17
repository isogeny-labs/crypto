//! Opt-in independent re-audit of retained, sealed producer software fixtures.
//! This test generates no data and launches no producer or cloud process.
use ml_dataset_independent_consumer::{audit, common};
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, path::Path};

const PRODUCER: &str = "d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83";
const PRODUCER_BINARY: &str = "c01e024156067799356f590a0c3ac749da509305bd569e26c9bf084d5402d04a";

fn replace(cell: &Path, name: &str, value: &Value) {
    let raw = serde_json::to_vec(value).unwrap();
    fs::write(cell.join(name), &raw).unwrap();
    let path = cell.join("manifest.json");
    let mut manifest = common::load(&path).unwrap();
    let artifact = manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["path"] == name)
        .unwrap();
    artifact["bytes"] = json!(raw.len());
    artifact["sha256"] = json!(common::digest(&raw));
    fs::write(path, serde_json::to_vec(&manifest).unwrap()).unwrap();
}

#[test]
#[ignore = "requires retained source-bound software fixtures, no generation"]
fn sealed_roots_bind_and_rehashed_substitutions_fail() {
    let sources =
        std::env::var("ML_DATASET_ROOT_FIXTURES").expect("colon-separated sealed run directories");
    let output = std::env::var("ML_DATASET_ROOT_REPORT").expect("fresh retained report path");
    assert!(!Path::new(&output).exists());
    let mut coverage = BTreeSet::new();
    let mut versions = BTreeSet::new();
    let mut completed = 0u64;
    let mut censored_calls = 0;
    let mut censored_cells = 0;
    let mut total = 0;
    let mut reports = Vec::new();
    let mut tamper_families = BTreeSet::new();
    for source in sources.split(':').map(Path::new) {
        let run = common::load(&source.join("run.json")).unwrap();
        assert_eq!(run["source_commit"], PRODUCER);
        assert_eq!(run["binary_sha256"], PRODUCER_BINARY);
        let run_sha = common::hash_file(&source.join("run.json")).unwrap();
        let (report, rows) = audit::audit(source, Some(&run_sha), true).unwrap();
        total += rows.len();
        completed += report["completed_distinct_root_sets_checked"]
            .as_u64()
            .unwrap();
        for row in &rows {
            let check = &row["root_certificate_arithmetic"];
            if check["status"] != "checked" {
                continue;
            }
            let config = &row["config"];
            versions.insert(row["factor_base"]["schema"].as_str().unwrap().to_owned());
            coverage.insert((
                config["family"].as_str().unwrap().to_owned(),
                config["n"].as_u64().unwrap() % 2,
                config["twist"].as_bool().unwrap(),
            ));
            censored_cells += usize::from(check["cell_censored"] == true);
            censored_calls += check["calls"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| c["censored"] == true)
                .count();
            for key in [
                "execution_trace_complete_verified",
                "all_event_identities_verified",
                "total_operation_count_verified",
                "unsat_verified",
                "fall_degree_verified",
            ] {
                assert_eq!(check[key], false);
            }
            if check["completed_root_sets_checked"].as_u64().unwrap() == 0 {
                continue;
            }
            let family = config["family"].as_str().unwrap().to_owned();
            if !tamper_families.insert(family.clone()) {
                continue;
            }
            let temp = tempfile::tempdir().unwrap();
            let negative = temp.path().join("negative");
            fs::create_dir(&negative).unwrap();
            for name in ["plan.json", "run.json", "protocol.json"] {
                fs::copy(source.join(name), negative.join(name)).unwrap();
            }
            let index = row["index"].as_u64().unwrap();
            let relative = format!("cells/{index:012}");
            let original = source.join(&relative);
            let cell = negative.join(&relative);
            fs::create_dir_all(&cell).unwrap();
            let manifest = common::load(&original.join("manifest.json")).unwrap();
            for a in manifest["artifacts"].as_array().unwrap() {
                let name = a["path"].as_str().unwrap();
                let destination = cell.join(name);
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
                fs::copy(original.join(name), destination).unwrap();
            }
            fs::copy(original.join("manifest.json"), cell.join("manifest.json")).unwrap();
            let name = if family == "prime" {
                "prime-root-certificates.json"
            } else {
                "binary-root-certificates.json"
            };
            let cert = common::load(&cell.join(name)).unwrap();
            let call = if family == "prime" {
                "root-call-0000.json"
            } else {
                "binary-root-call-0000.json"
            };
            // Rehash both redundant copies: the algebra, rather than stale hashes,
            // must reject these root and coefficient substitutions.
            for mutation in ["roots", "polynomial", "fixed_x", "work_before"] {
                let mut bad = cert.clone();
                let q = &mut bad["queries"][0];
                match mutation {
                    "roots" => {
                        // Keep every element canonical: omit a genuine root,
                        // or falsely add zero when the correct list is empty.
                        let mut altered = q["report"]["roots"].as_array().unwrap().clone();
                        if altered.is_empty() {
                            altered.push(json!(0));
                        } else {
                            altered.pop();
                        }
                        q["report"]["roots"] = json!(altered);
                        q["report"]["events"]
                            .as_array_mut()
                            .unwrap()
                            .last_mut()
                            .unwrap()["roots"] = json!(altered);
                    }
                    "polynomial" => {
                        q["report"]["polynomial"] = json!([1]);
                        q["report"]["events"][0]["polynomial"] = json!([1]);
                    }
                    "fixed_x" => q["fixed_x"] = json!("00"),
                    _ => q["work_before"] = json!(1),
                }
                replace(&cell, call, &bad["queries"][0]);
                replace(&cell, name, &bad);
                assert!(
                    audit::audit(&negative, None, true).is_err(),
                    "accepted rehashed {family}/{mutation}"
                );
            }
            replace(&cell, call, &cert["queries"][0]);
            replace(&cell, name, &cert);
            audit::audit(&negative, None, true).unwrap();
            let (baseline, rows) = audit::audit(&negative, None, false).unwrap();
            assert_eq!(baseline["completed_distinct_root_sets_checked"], 0);
            assert_eq!(
                rows[0]["root_certificate_arithmetic"]["all_supplied_root_sets_verified"],
                false
            );
        }
        reports.push(json!({"run_sha256":run_sha,"report":report}));
    }
    for family in ["binary", "koblitz"] {
        for parity in [0, 1] {
            for twist in [false, true] {
                assert!(
                    coverage.contains(&(family.into(), parity, twist)),
                    "missing {family}/{parity}/{twist}"
                );
            }
        }
    }
    assert_eq!(versions.len(), 2, "both legacy/v2 factor bases required");
    assert!(completed > 0 && censored_calls > 0 && censored_cells > 0);
    assert_eq!(tamper_families.len(), 3);
    fs::write(output,serde_json::to_vec_pretty(&json!({"schema":"ml-dataset.independent-root-software-validation/v1",
        "producer_source_commit":PRODUCER,"producer_binary_sha256":PRODUCER_BINARY,"checked_cells":total,
        "completed_root_sets_checked":completed,"censored_cells":censored_cells,"censored_calls":censored_calls,
        "coverage":coverage,"factor_base_versions":versions,"rehashed_tamper_families":tamper_families,
        "scientific_observation":false,"execution_trace_complete_verified":false,"total_operation_count_verified":false,
        "unsat_verified":false,"runs":reports})).unwrap()).unwrap();
}
