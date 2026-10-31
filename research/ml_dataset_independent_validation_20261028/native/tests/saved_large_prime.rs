//! Retained software acceptance only. This test never launches a producer.
use anyhow::{ensure, Context, Result};
use ml_dataset_independent_consumer::{
    audit, common,
    large_prime::schema::{Event, Transcript},
};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
const SOURCE: &str = "58bd5ab64424f7710540e0aea502225de6e6c438";
const BINARY: &str = "968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7";
fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let meta = entry.file_type()?;
        ensure!(!meta.is_symlink(), "unexpected fixture symlink");
        if meta.is_dir() {
            copy_tree(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn replace(cell: &Path, name: &str, v: &Value) -> Result<()> {
    let bytes = serde_json::to_vec(v)?;
    fs::write(cell.join(name), &bytes)?;
    let mut m = common::load(&cell.join("manifest.json"))?;
    let a = m["artifacts"]
        .as_array_mut()
        .context("artifacts")?
        .iter_mut()
        .find(|a| a["path"] == name)
        .context("artifact")?;
    a["bytes"] = json!(bytes.len());
    a["sha256"] = json!(common::digest(&bytes));
    fs::write(cell.join("manifest.json"), serde_json::to_vec(&m)?)?;
    Ok(())
}
fn write_transcript(cell: &Path, t: &Value) -> Result<()> {
    replace(cell, "generic-large-prime-transcript.json", t)?;
    for (i, b) in t["batches"].as_array().unwrap().iter().enumerate() {
        replace(
            cell,
            &format!("generic-large-prime-filter-batch-{i:04}.json"),
            b,
        )?;
    }
    Ok(())
}
fn reprice(t: &mut Value) -> Result<()> {
    let mut total = 0u64;
    for batch in t["batches"].as_array_mut().unwrap() {
        let mut bytes = 0u64;
        for event in batch["native"]["events"].as_array().unwrap() {
            let event: Event = serde_json::from_value(event.clone())?;
            bytes += serde_json::to_vec(&event)?.len() as u64;
        }
        batch["native"]["acknowledged_bytes"] = json!(bytes);
        total += bytes;
    }
    t["trace_totals"]["acknowledged_bytes"] = json!(total);
    Ok(())
}
fn feature_export(rows: &[Value], path: &Path) -> Result<Vec<Value>> {
    audit::export_rows(rows, path)?;
    fs::read_to_string(path)?
        .lines()
        .map(|line| Ok(serde_json::from_str::<Value>(line)?["features"].clone()))
        .collect()
}

#[test]
#[ignore = "requires already retained fixed producer panel; no generation"]
fn sealed_generic_filter_panel_and_rehashed_tampering() -> Result<()> {
    let panel = PathBuf::from(std::env::var("ML_GENERIC_FILTER_PANEL")?);
    let output = PathBuf::from(std::env::var("ML_GENERIC_FILTER_REPORT_DIR")?);
    ensure!(!output.exists(), "report directory must be fresh");
    fs::create_dir(&output)?;
    let mut entries = fs::read_dir(&panel)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort();
    let mut receipts = Vec::new();
    let mut errors = Vec::new();
    let mut coverage = BTreeSet::new();
    let mut completed = 0;
    let mut censored = 0;
    let mut recovered = 0;
    let mut source_combinations = 0;
    let mut multi_batch = 0;
    let mut no_recovery = 0;
    let mut legacy = 0;
    let mut first = None;
    for entry in entries {
        let receipt = entry.join("receipt.json");
        if !receipt.is_file() {
            continue;
        }
        let run = entry.join("run");
        if !run.is_dir() {
            receipts.push(json!({"name":entry.file_name().unwrap().to_str(),"state":"no_run","receipt":common::load(&receipt)?}));
            continue;
        }
        let binding = common::load(&run.join("run.json"))?;
        ensure!(
            binding["source_commit"] == SOURCE && binding["binary_sha256"] == BINARY,
            "source-bound producer fixture"
        );
        let sha = common::hash_file(&run.join("run.json"))?;
        match audit::audit(&run, Some(&sha), true) {
            Err(e) => {
                errors.push(json!({"run":run,"error":format!("{e:#}")}));
            }
            Ok((summary, rows)) => {
                let n = receipts.len();
                let before = feature_export(&rows, &output.join(format!("export-{n}.jsonl")))?;
                let mut altered = rows.clone();
                for row in &mut altered {
                    row.as_object_mut()
                        .unwrap()
                        .remove("large_prime_filter_arithmetic");
                    row["restricted_source_pivot_seed_counter_stop"] =
                        json!("MUST_NOT_BECOME_A_FEATURE");
                    row["outcome"]["reason"] = json!("MUST_NOT_BECOME_A_FEATURE");
                }
                let after =
                    feature_export(&altered, &output.join(format!("feature-control-{n}.jsonl")))?;
                ensure!(
                    before == after,
                    "restricted filter data leaked into features"
                );
                for row in &rows {
                    let config = &row["config"];
                    let cell = run.join(format!("cells/{:012}", row["index"].as_u64().unwrap()));
                    if let Some(check) = row.get("large_prime_filter_arithmetic") {
                        completed += check["completed_batches_checked"].as_u64().unwrap();
                        censored += check["censored_prefixes_checked"].as_u64().unwrap();
                        recovered += u64::from(check["positive_recovery_checked"] == true);
                        no_recovery += u64::from(check["positive_recovery_checked"] == false);
                        coverage.insert((
                            config["family"].as_str().unwrap().to_owned(),
                            config["n"].as_u64().unwrap() % 2,
                            config["twist"].as_bool().unwrap(),
                        ));
                        let t = common::load(&cell.join("generic-large-prime-transcript.json"))?;
                        let typed: Transcript = serde_json::from_value(t.clone())?;
                        multi_batch += u64::from(typed.batches.len() > 1);
                        source_combinations += typed
                            .batches
                            .iter()
                            .flat_map(|b| &b.native.rows)
                            .filter(|r| r.sources.len() > 1)
                            .count() as u64;
                        if first.is_none()
                            && typed.batches.len() == 1
                            && !typed.batches[0].native.events.is_empty()
                        {
                            first = Some((run.clone(), row["index"].as_u64().unwrap()));
                        }
                    } else if config["family"] == "koblitz"
                        && row["outcome"]["status"] == "verified"
                        && row["artifact_roles"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|r| r == "large-prime-elimination-transcript")
                        && cell.join("large-prime-transcript.json").is_file()
                    {
                        legacy += 1;
                    }
                }
                receipts.push(json!({"run":run,"summary":summary,"rows":rows}));
            }
        }
    }
    let summary = json!({"schema":"consumer-generic-filter-software-review/v1","producer_source":SOURCE,"producer_binary":BINARY,"runs_audited":receipts.len(),"completed_batches":completed,"censored_prefixes":censored,"positive_recoveries":recovered,"no_recovery_cells":no_recovery,"multi_batch_cells":multi_batch,"nontrivial_source_combinations":source_combinations,"legacy_koblitz_cells":legacy,"coverage":coverage,"errors":errors,"mathematics_globally_verified":false,"execution_complete":false,"total_cost_verified":false,"scientific_observation":false,"receipts":receipts});
    fs::write(
        output.join("acceptance.json"),
        serde_json::to_vec_pretty(&summary)?,
    )?;
    ensure!(errors.is_empty(), "retained independent audit failures");
    let (source, index) = first.context("no completed generic v2 fixture for tamper checks")?;
    let mut tampers = Vec::new();
    for kind in [
        "missing_batch",
        "wrong_role",
        "wrong_binding",
        "wrong_config",
        "normalized_rhs",
        "inverse",
        "matrix",
        "query_group",
        "auxiliary_pair",
        "overclaim",
    ] {
        let temp = tempfile::tempdir()?;
        let run = temp.path().join("run");
        copy_tree(&source, &run)?;
        let cell = run.join(format!("cells/{index:012}"));
        let mut t = common::load(&cell.join("generic-large-prime-transcript.json"))?;
        match kind {
            "missing_batch" => {
                let mut m = common::load(&cell.join("manifest.json"))?;
                m["artifacts"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|a| a["path"] != "generic-large-prime-filter-batch-0000.json");
                fs::write(cell.join("manifest.json"), serde_json::to_vec(&m)?)?;
            }
            "wrong_role" => {
                let mut m = common::load(&cell.join("manifest.json"))?;
                m["artifacts"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|a| a["path"] == "generic-large-prime-filter-batch-0000.json")
                    .unwrap()["role"] = json!("wrong");
                fs::write(cell.join("manifest.json"), serde_json::to_vec(&m)?)?;
            }
            "auxiliary_pair" => {
                let mut p = common::load(&cell.join("generic-large-prime-pairs.json"))?;
                p["pairs"][0]["sum"] = Value::Null;
                replace(&cell, "generic-large-prime-pairs.json", &p)?;
            }
            _ => {
                match kind {
                    "wrong_binding" => t["binding"]["public_sha256"] = json!("0".repeat(64)),
                    "wrong_config" => t["config"]["seed"] = json!(43),
                    "normalized_rhs" => {
                        let e = t["batches"][0]["native"]["events"]
                            .as_array_mut()
                            .unwrap()
                            .iter_mut()
                            .find(|e| e["transition"]["kind"] == "normalized")
                            .unwrap();
                        let v = e["transition"]["row"]["rhs"].as_u64().unwrap();
                        e["transition"]["row"]["rhs"] = json!(v + 1);
                        reprice(&mut t)?;
                    }
                    "inverse" => {
                        let e = t["batches"][0]["native"]["events"]
                            .as_array_mut()
                            .unwrap()
                            .iter_mut()
                            .find(|e| e["transition"]["kind"] == "reduction")
                            .unwrap();
                        e["transition"]["inverse"] = json!(0);
                        reprice(&mut t)?;
                    }
                    "matrix" => {
                        t["batches"][0]["terminal"]["legacy_batch"]["matrix"]["rank"] = json!(0)
                    }
                    "query_group" => t["attempts"][0]["target"] = Value::Null,
                    "overclaim" => t["execution_trace_complete_verified"] = json!(true),
                    _ => unreachable!(),
                }
                write_transcript(&cell, &t)?;
                if matches!(kind, "normalized_rhs" | "inverse") {
                    let mut m = common::load(&cell.join("manifest.json"))?;
                    m["outcome"]["counters"]["native_filter_trace"] = t["trace_totals"].clone();
                    fs::write(cell.join("manifest.json"), serde_json::to_vec(&m)?)?;
                }
            }
        }
        let error = audit::audit(&run, None, true)
            .err()
            .with_context(|| format!("rehashed {kind} mutation was accepted"))?;
        let record = json!({"kind":kind,"error":format!("{error:#}"),"manifest_sha256":common::hash_file(&cell.join("manifest.json"))?,"transcript_sha256":common::hash_file(&cell.join("generic-large-prime-transcript.json"))?});
        fs::write(
            output.join(format!("tamper-{kind}.json")),
            serde_json::to_vec_pretty(&record)?,
        )?;
        tampers.push(record);
    }
    fs::write(
        output.join("tamper-summary.json"),
        serde_json::to_vec_pretty(&tampers)?,
    )?;
    Ok(())
}
