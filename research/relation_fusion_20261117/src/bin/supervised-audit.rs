//! Saved functional-proof/transport audit. Does not replay process clocks or admit speedup.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../packed_roster.rs");
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file type/size".into());
    }
    Ok(fs::read(p)?)
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn process_ok(v: &Value, native: bool, worker_hash: &Value, success: bool) -> bool {
    v["native"] == native
        && v["worker_sha256"] == *worker_hash
        && v["success"] == success
        && v["timed_out"] == false
        && v["resource_envelope"]
            == json!({"cpu":0,"threads":1,"address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30})
        && if success {
            v["exit"] == 0
        } else {
            v["exit"].as_i64().is_some_and(|x| x != 0)
        }
}
fn check(input: &[u8], proof: &[u8], dag: bool) -> Result<relation_replay_checker::Checked> {
    Ok(if dag {
        relation_replay_checker::packed::verify_dag_json(input, proof)?
    } else {
        relation_replay_checker::packed::verify_json(input, proof)?
    })
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage:supervised-audit RAW PINS200 PROBE_SHA NEW_REPORT".into());
    }
    let raw = Path::new(&a[1]);
    let pins = value(Path::new(&a[2]))?;
    let summary = value(&raw.join("summary.json"))?;
    let rows = summary["cases"].as_array().ok_or("cases")?;
    require(
        summary["schema"] == "native-supervisor-functional-qualification/v1"
            && rows.len() == 216
            && summary["planned_proof_cases"] == 216
            && summary["all_complete"] == true,
        "complete schema/roster",
    )?;
    require(
        value(&raw.join("workers-before.json"))? == pins["worker_sha256"]
            && value(&raw.join("workers-after.json"))? == pins["worker_sha256"],
        "worker pins",
    )?;
    let mut seen = std::collections::BTreeSet::new();
    let mut bases = std::collections::BTreeMap::new();
    let mut results = Vec::new();
    let mut mutations = Vec::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for arm in roster("AB") {
                let mut identity = None;
                for native in [false, true] {
                    let id = format!(
                        "p{p}-n{n}-{}-{}",
                        arm.name,
                        if native { "native" } else { "wrapped" }
                    );
                    let found: Vec<_> = rows.iter().filter(|r| r["id"] == id).collect();
                    require(
                        found.len() == 1 && seen.insert(id.clone()),
                        "duplicate/missing cell",
                    )?;
                    let row = found[0];
                    let dir = raw.join(&id);
                    require(
                        row["status"] == "complete"
                            && row["prime"] == p
                            && row["n"] == n
                            && row["seed"] == 7
                            && row["graph"] == "LAST-star"
                            && row["native"] == native
                            && row["arm"] == arm.name
                            && row["kind"] == arm.kind
                            && row["kernel"] == arm.kernel
                            && row.get("observed_solving_degree") == Some(&Value::Null)
                            && row["performance_claim"] == false
                            && value(&dir.join("outcome.json"))? == *row,
                        "case identity/raw binding",
                    )?;
                    let process = value(&dir.join("process.json"))?;
                    require(
                        process_ok(&process, native, &pins["worker_sha256"][arm.worker], true),
                        "process metadata",
                    )?;
                    let original = read(&dir.join("proof/original.json"))?;
                    let proof = read(&dir.join("proof/witness.json"))?;
                    require(
                        row["input_sha256"] == sha(&original)
                            && pins["input_sha256"][format!("p{p}-n{n}")] == sha(&original)
                            && row["identity"]["wire_sha256"] == sha(&proof),
                        "original/wire binding",
                    )?;
                    let checked = check(&original, &proof, arm.kind.ends_with("-dag"))?;
                    let worker = value(&dir.join("call.stdout"))?;
                    let basis = serde_json::to_value(&checked.basis)?;
                    let stats = serde_json::to_value(&checked.stats)?;
                    let now = json!({"wire_sha256":sha(&proof),"basis":basis,"stats":stats,"constructor":worker["constructor"]});
                    require(
                        row["identity"] == now
                            && worker["status"] == "complete"
                            && worker["prime"] == p
                            && worker["variables"] == n
                            && worker["checker"] == "packed"
                            && worker["kernel"] == arm.kernel
                            && worker["basis"] == basis
                            && worker["consumer_stats"] == stats
                            && worker["S_pair_and_input_containment_checks"] == true
                            && read(&dir.join("call.stderr"))?.is_empty(),
                        "worker/complete-proof metadata",
                    )?;
                    if let Some(old) = &identity {
                        require(
                            *old == now,
                            "launcher pair proof bytes/basis/stats/constructor",
                        )?;
                    } else {
                        identity = Some(now);
                    }
                    if let Some(old) = bases.insert((p, n), basis.clone()) {
                        require(old == basis, "cross-arm basis")?;
                    }
                    if mutations.is_empty() {
                        let mut bad: Value = serde_json::from_slice(&proof)?;
                        let coefficient = bad["basis"][0]["polynomial"][0]["coefficient"]
                            .as_u64()
                            .ok_or("coefficient")?;
                        bad["basis"][0]["polynomial"][0]["coefficient"] =
                            json!(if coefficient == p - 1 {
                                1
                            } else {
                                coefficient + 1
                            });
                        let bad = serde_json::to_vec(&bad)?;
                        require(
                            check(&original, &bad, arm.kind.ends_with("-dag")).is_err(),
                            "coefficient sabotage accepted",
                        )?;
                        mutations.push("basis coefficient");
                        let mut bad = process.clone();
                        bad["native"] = json!(!native);
                        require(
                            !process_ok(&bad, native, &pins["worker_sha256"][arm.worker], true),
                            "launcher metadata sabotage accepted",
                        )?;
                        mutations.push("launcher flag");
                        let mut bad = process.clone();
                        bad["resource_envelope"]["address_space_bytes"] = json!(4294967296u64);
                        require(
                            !process_ok(&bad, native, &pins["worker_sha256"][arm.worker], true),
                            "resource metadata sabotage accepted",
                        )?;
                        mutations.push("resource envelope");
                    }
                    results.push(json!({"id":id,"complete_proof_replayed":true,"process_metadata_bound":true,"proof_sha256":sha(&proof)}));
                }
            }
        }
    }
    let mut transports = Vec::new();
    for native in [false, true] {
        for (mode, success) in [("invalid-mode", false), ("limits", true)] {
            let id = format!(
                "transport-{}-{mode}",
                if native { "native" } else { "wrapped" }
            );
            let dir = raw.join(&id);
            let receipt = value(&dir.join("process.json"))?;
            require(
                process_ok(&receipt, native, &json!(a[3]), success),
                "probe transport receipt",
            )?;
            if success {
                let text = String::from_utf8(read(&dir.join("call.stdout"))?)?;
                let v: Vec<_> = text.split_whitespace().collect();
                require(
                    v.len() == 7
                        && v[..6]
                            == [
                                "2147483648",
                                "2147483648",
                                "16777216",
                                "16777216",
                                "0",
                                "true",
                            ],
                    "OS probe envelope",
                )?;
            }
            transports.push(json!({"id":id,"expected_success":success,"checked":true}));
        }
        let dir = raw.join(format!(
            "collision-{}",
            if native { "native" } else { "wrapped" }
        ));
        require(
            process_ok(
                &value(&dir.join("process.json"))?,
                native,
                &pins["worker_sha256"][2],
                false,
            ),
            "collision receipt",
        )?;
        require(
            read(&dir.join("proof/sentinel"))? == b"preserve-existing-proof-owner"
                && String::from_utf8(read(&dir.join("call.stderr"))?)?
                    .to_lowercase()
                    .contains("exist"),
            "collision ownership",
        )?;
        transports
            .push(json!({"native":native,"collision_rejected":true,"sentinel_preserved":true}));
    }
    let report = json!({"schema":"native-supervised-artifact-audit/v1","complete_proof_replays":results.len(),"paired_launcher_equalities":108,"cases":results,"transport_controls":transports,"sabotage_classes_rejected":mutations,"OS_limits_independently_reexecuted":false,"timing_claim":false,"signed_execution_verdict":false});
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(&a[4])?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!("216completeproofs,108launcheridentities,6transportcontrols,3sabotages; no timing admission");
    Ok(())
}
