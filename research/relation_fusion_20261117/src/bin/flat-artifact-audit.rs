//! Saved complete-proof audit for the flat constructor and real worker qualifications.
//! No process re-execution, native-operation counter re-execution or timing admission.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file type/cap".into());
    }
    Ok(fs::read(p)?)
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn require(ok: bool, msg: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn checked(input: &[u8], wire: &[u8], dag: bool) -> Result<relation_replay_checker::Checked> {
    Ok(if dag {
        relation_replay_checker::packed::verify_dag_json(input, wire)?
    } else {
        relation_replay_checker::packed::verify_json(input, wire)?
    })
}
fn receipt(v: &Value, native: bool, worker: &Value, success: bool) -> bool {
    v["native"] == native
        && v["worker_sha256"] == *worker
        && v["timed_out"] == false
        && v["success"] == success
        && v["exit"] == if success { 0 } else { 1 }
        && v["resource_envelope"]
            == json!({"cpu":0,"threads":1,"address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30})
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage:flat-artifact-audit MATRIX277 WORKER281 PINS274 NEW_REPORT".into());
    }
    let direct = Path::new(&a[1]);
    let workers = Path::new(&a[2]);
    let pins = value(Path::new(&a[3]))?;
    let matrix = value(&direct.join("matrix.json"))?;
    let summary = value(&workers.join("summary.json"))?;
    let cases = matrix["cases"].as_array().ok_or("direct cases")?;
    let calls = summary["cases"].as_array().ok_or("worker cases")?;
    require(
        matrix["schema"] == "flat-constructor-correctness/v1"
            && matrix["case_count"] == 36
            && cases.len() == 36,
        "direct complete roster",
    )?;
    require(
        summary["schema"] == "flat-worker-functional-qualification/v1"
            && summary["planned"] == 72
            && calls.len() == 72
            && summary["all_complete"] == true
            && summary["worker_hash_unchanged"] == true,
        "worker complete roster",
    )?;
    let worker =
        pins["tool_sha256"]["/workspace/relation-native-qualifiers/flat-274/fused-native"].clone();
    require(worker.is_string(), "worker pin")?;
    require(summary["worker_sha256"] == worker, "summary worker binding")?;
    let mut direct_seen = BTreeSet::new();
    let mut worker_seen = BTreeSet::new();
    let mut replays = 0;
    let mut stored_mutations = 0;
    let mut sabotages = Vec::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for dag in [false, true] {
                for (name, kernel) in [
                    ("frontier103", "frontier"),
                    ("products231", "packed-products"),
                    ("batch487", "packed-batch"),
                ] {
                    let id = format!("p{p}-n{n}-{}-{name}", if dag { "dag" } else { "sparse" });
                    let found: Vec<_> = cases.iter().filter(|r| r["id"] == id).collect();
                    require(
                        found.len() == 1 && direct_seen.insert(id.clone()),
                        "direct cell coverage",
                    )?;
                    let row = found[0];
                    let dir = direct.join(&id);
                    require(
                        value(&dir.join("outcome.json"))? == *row,
                        "direct raw/summary",
                    )?;
                    let input = read(&dir.join("original.json"))?;
                    let flat = read(&dir.join("flat-proof.json"))?;
                    require(
                        row["prime"] == p
                            && row["n"] == n
                            && row["seed"] == 7
                            && row["graph"] == "LAST-star"
                            && row["dag"] == dag
                            && row["kernel"] == name
                            && row.get("observed_solving_degree") == Some(&Value::Null)
                            && row["timing_claim"] == false
                            && row["input_sha256"] == sha(&input)
                            && pins["input_sha256"][format!("p{p}-n{n}")] == sha(&input)
                            && row["proof_sha256"] == sha(&flat)
                            && row["proof_bytes"] == flat.len(),
                        "direct identity/digests",
                    )?;
                    let mut identity = None;
                    for file in ["legacy-proof.json", "tree-proof.json", "flat-proof.json"] {
                        let wire = read(&dir.join(file))?;
                        require(wire == flat, "three backend wire equality")?;
                        let c = checked(&input, &wire, dag)?;
                        replays += 1;
                        require(
                            serde_json::to_value(&c.basis)? == row["checked_basis"]
                                && serde_json::to_value(&c.stats)? == row["checked_stats"],
                            "direct independent basis/stats",
                        )?;
                        identity = Some(
                            json!({"wire_sha256":sha(&wire),"basis":c.basis,"stats":c.stats,"constructor":row["metadata"]}),
                        );
                    }
                    let expected_mutations = if dag {
                        vec![
                            "basis-coefficient",
                            "drop-input-obligation",
                            "drop-pair-obligation",
                            "v2-as-v1",
                        ]
                    } else {
                        vec![
                            "basis-coefficient",
                            "drop-input-obligation",
                            "drop-pair-obligation",
                        ]
                    };
                    require(
                        row["rejected_mutations"] == json!(expected_mutations),
                        "complete mutation roster",
                    )?;
                    for mutation in row["rejected_mutations"].as_array().ok_or("mutations")? {
                        let name = mutation.as_str().ok_or("mutation name")?;
                        require(
                            checked(
                                &input,
                                &read(&dir.join(format!("sabotage-{name}.json")))?,
                                dag,
                            )
                            .is_err(),
                            "stored mutation accepted",
                        )?;
                        stored_mutations += 1;
                    }
                    for native in [false, true] {
                        let call_id = format!("{id}-{}", if native { "native" } else { "wrapped" });
                        let found: Vec<_> = calls.iter().filter(|r| r["id"] == call_id).collect();
                        require(
                            found.len() == 1 && worker_seen.insert(call_id.clone()),
                            "worker cell coverage",
                        )?;
                        let row = found[0];
                        let dir = workers.join(&call_id);
                        require(
                            value(&dir.join("outcome.json"))? == *row,
                            "worker raw/summary",
                        )?;
                        let process = value(&dir.join("process.json"))?;
                        require(
                            receipt(&process, native, &worker, true),
                            "resource/process binding",
                        )?;
                        let original = read(&dir.join("proof/original.json"))?;
                        let wire = read(&dir.join("proof/witness.json"))?;
                        require(
                            original == input
                                && wire == flat
                                && read(&dir.join("call.stderr"))?.is_empty(),
                            "worker exact transport",
                        )?;
                        let c = checked(&original, &wire, dag)?;
                        replays += 1;
                        let output = value(&dir.join("call.stdout"))?;
                        require(
                            row["status"] == "complete"
                                && row["prime"] == p
                                && row["n"] == n
                                && row["seed"] == 7
                                && row["graph"] == "LAST-star"
                                && row["dag"] == dag
                                && row["kernel"] == kernel
                                && row["native"] == native
                                && row.get("observed_solving_degree") == Some(&Value::Null)
                                && row["timing_claim"] == false
                                && row["input_sha256"] == sha(&input)
                                && row["identity"] == *identity.as_ref().ok_or("identity")?
                                && output["status"] == "complete"
                                && output["prime"] == p
                                && output["variables"] == n
                                && output["checker"] == "packed"
                                && output["kernel"] == kernel
                                && output["S_pair_and_input_containment_checks"] == true
                                && output["basis"] == serde_json::to_value(&c.basis)?
                                && output["consumer_stats"] == serde_json::to_value(&c.stats)?
                                && output["constructor"] == row["identity"]["constructor"],
                            "worker replay/identity binding",
                        )?;
                        if sabotages.is_empty() {
                            let mut bad = process.clone();
                            bad["native"] = json!(!native);
                            require(
                                !receipt(&bad, native, &worker, true),
                                "launcher sabotage accepted",
                            )?;
                            sabotages.push("launcher flag");
                            let mut bad = process.clone();
                            bad["resource_envelope"]["address_space_bytes"] = json!(4294967296u64);
                            require(
                                !receipt(&bad, native, &worker, true),
                                "resource sabotage accepted",
                            )?;
                            sabotages.push("resource envelope");
                        }
                    }
                }
            }
        }
    }
    let mut expected_controls = Vec::new();
    for native in [false, true] {
        for collision in [false, true] {
            expected_controls.push(json!({"native":native,"collision":collision,"expected_failure":true,"later_arms_remain_planned":true}));
            let dir = workers.join(format!(
                "control-{}-{}",
                if native { "native" } else { "wrapped" },
                if collision {
                    "collision"
                } else {
                    "invalid-mode"
                }
            ));
            require(
                receipt(&value(&dir.join("process.json"))?, native, &worker, false),
                "negative transport receipt",
            )?;
            require(
                read(&dir.join("call.stdout"))?.is_empty()
                    && !read(&dir.join("call.stderr"))?.is_empty(),
                "negative transport diagnostics",
            )?;
            if collision {
                require(
                    read(&dir.join("proof/sentinel"))? == b"preserve-existing-proof-owner"
                        && String::from_utf8(read(&dir.join("call.stderr"))?)?
                            .to_lowercase()
                            .contains("exist"),
                    "negative ownership binding",
                )?;
            }
        }
    }
    require(
        summary["transport_controls"] == json!(expected_controls),
        "negative control summary binding",
    )?;
    let report = json!({"schema":"flat-artifact-audit/v1","direct_cases":36,"worker_cases":72,"full_independent_proof_replays":replays,
        "stored_polynomial_mutations_rejected":stored_mutations,"receipt_sabotage_classes_rejected":sabotages,"negative_transport_controls":4,
        "OS_limits_independently_reexecuted":false,"parent_clocks_or_native_counters_attested":false,"timing_claim":false,"15x_admitted":false,"signed_execution_verdict":false});
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(&a[4])?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!("{}", report);
    Ok(())
}
