//! Explicitly synthetic analysis fixtures, reusing saved complete proof bytes.
//! No worker, launcher, host monitor or performance clock is executed here.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../supervised_roster.rs");
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(p: &Path, b: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn clone_synthetic(source: &Path, out: &Path) -> Result<()> {
    fs::create_dir(out)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let target = out.join(entry.file_name());
        if kind.is_dir() {
            clone_synthetic(&entry.path(), &target)?;
        } else if kind.is_file() {
            fs::hard_link(entry.path(), target)?;
        } else {
            return Err("synthetic fixture contains a nonregular entry".into());
        }
    }
    Ok(())
}
fn replace_copy(path: &Path, value: &Value) -> Result<()> {
    // Atomic replacement changes only the cloned path, never the shared inode
    // or source fixture. Only explicitly synthetic copies reach this helper.
    let temporary = path.with_extension("canary-tmp");
    save(&temporary, &serde_json::to_vec_pretty(value)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 5 && a[1] == "--mutate" {
        if !["clock", "duplicate"].contains(&a[4].as_str()) {
            return Err("unknown synthetic sabotage".into());
        }
        let source = Path::new(&a[2]);
        let out = Path::new(&a[3]);
        let mut summary = value(&source.join("summary.json"))?;
        if summary["synthetic_fixture"] != true {
            return Err("refusing to mutate a performance/evidence cohort".into());
        }
        clone_synthetic(source, out)?;
        if a[4] == "clock" {
            let row = &summary["outcomes"][0];
            let dir = out.join(format!(
                "{}-r{}-p{}-n{}-{}",
                row["stage"].as_str().ok_or("stage")?,
                row["round"],
                row["prime"],
                row["n"],
                row["arm"].as_str().ok_or("arm")?
            ));
            let mut process = value(&dir.join("process.json"))?;
            process["parent_ns"] = json!(row["child_process_ns"].as_u64().ok_or("clock")? + 1);
            replace_copy(&dir.join("process.json"), &process)?;
        } else {
            summary["outcomes"][1] = summary["outcomes"][0].clone();
            replace_copy(&out.join("summary.json"), &summary)?;
        }
        println!(
            "{}",
            json!({"synthetic_fixture":true,"sabotage":a[4],
            "source_not_modified":true,"performance_observations_collected":0})
        );
        return Ok(());
    }
    if a.len() != 5 || !["neutral", "hypothetical20"].contains(&a[4].as_str()) {
        return Err(
            "usage:supervised-fixture RAW240 PINS200 NEW_OUT neutral|hypothetical20".into(),
        );
    }
    let source = Path::new(&a[1]);
    let out = Path::new(&a[3]);
    fs::create_dir(out)?;
    let original_pins = value(Path::new(&a[2]))?;
    let pins = json!({"schema":"supervised-match-pins/v1","synthetic_fixture":true,
        "scope":"analysis qualification only; no performance observations",
        "worker_sha256":original_pins["worker_sha256"],
        "input_sha256":original_pins["input_sha256"]});
    let pin_bytes = serde_json::to_vec_pretty(&pins)?;
    save(&out.join("prospective-pins.json"), &pin_bytes)?;
    for n in ["workers-before.json", "workers-after.json"] {
        save(
            &out.join(n),
            &serde_json::to_vec_pretty(&pins["worker_sha256"])?,
        )?;
    }
    let mut outcomes = Vec::new();
    for stage in ["AA", "AB"] {
        for round in 0..ROUNDS {
            for p in [2147483647u64, 9007199254740881] {
                for n in [3, 4, 6] {
                    for arm in supervised_roster(stage)? {
                        let base = arm.name.strip_suffix("-copy").unwrap_or(&arm.name);
                        let from = source.join(format!("p{p}-n{n}-{base}"));
                        let dir = out.join(format!("{stage}-r{round}-p{p}-n{n}-{}", arm.name));
                        fs::create_dir(&dir)?;
                        fs::create_dir(dir.join("proof"))?;
                        let original = fs::read(from.join("proof/original.json"))?;
                        let wire = fs::read(from.join("proof/witness.json"))?;
                        let input_sha = sha(&original);
                        if pins["input_sha256"][format!("p{p}-n{n}")] != input_sha {
                            return Err("fixture source input differs from fixed original".into());
                        }
                        // Independently check every reused proof before copying.
                        let checked = if arm.kind.ends_with("-dag") {
                            relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                        } else {
                            relation_replay_checker::packed::verify_json(&original, &wire)?
                        };
                        let mut worker = value(&from.join("call.stdout"))?;
                        if worker["basis"] != serde_json::to_value(&checked.basis)?
                            || worker["consumer_stats"] != serde_json::to_value(&checked.stats)?
                        {
                            return Err("fixture source proof/basis/stats".into());
                        }
                        let control = supervised_controls().iter().any(|c| c.name == base);
                        let parent = if a[4] == "hypothetical20" && control {
                            20000u64
                        } else if control && base == "current-dag-487-native" {
                            800
                        } else if control && arm.native {
                            1600
                        } else if control {
                            2000
                        } else {
                            1000
                        };
                        let child = parent - 100;
                        let (construct, replay, whole) = (200u64, 100u64, 400u64);
                        worker["synthetic_fixture"] = json!(true);
                        worker["phases_diagnostic_ns"] = json!({
                            "construct_and_wire":construct.to_string(),
                            "independent_replay":replay.to_string(),
                            "full_worker_including_proof_write":whole.to_string()});
                        save(&dir.join("call.stdout"), &serde_json::to_vec(&worker)?)?;
                        save(&dir.join("call.stderr"), b"")?;
                        save(&dir.join("proof/original.json"), &original)?;
                        save(&dir.join("proof/witness.json"), &wire)?;
                        save(
                            &dir.join("process.json"),
                            &serde_json::to_vec_pretty(&json!({
                            "synthetic_fixture":true,"exit":0,"parent_ns":child,
                            "driver_sha256":pins["worker_sha256"][arm.worker],
                            "input_sha256":input_sha,"native":arm.native,"timed_out":false,
                            "resource_envelope":{"cpu":0,"threads":1,
                                "address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30}}))?,
                        )?;
                        let row = json!({"synthetic_fixture":true,"stage":stage,"round":round,
                            "arm":arm.name,"native":arm.native,"seed":7,"graph":"LAST-star",
                            "observed_solving_degree":null,"prime":p,"n":n,"kernel":arm.kernel,
                            "checker":"packed","status":"complete","kind":arm.kind,
                            "basis_sha256":sha(&serde_json::to_vec(&checked.basis)?),
                            "proof_sha256":sha(&wire),"parent_ns":parent,"child_process_ns":child,
                            "construct_ns":construct,"replay_ns":replay,"worker_ns":whole});
                        save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                        outcomes.push(row);
                    }
                }
            }
        }
    }
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(&json!({
        "schema":"fusion-supervised-match/v1","synthetic_fixture":true,
        "planned":PLANNED_CALLS,"primary":PRIMARY,"outcomes":outcomes,"all_correct":true,
        "prospective_pins_sha256":sha(&pin_bytes),"performance_observations_collected":0}))?,
    )?;
    save(
        &out.join("monitor.jsonl"),
        b"{\"synthetic_fixture\":true,\"exit_status\":0,\"contended_samples\":0}\n",
    )?;
    println!(
        "{}",
        json!({"synthetic_fixture":true,"proofs_checked_and_copied":PLANNED_CALLS,
        "scenario":a[4],"performance_observations_collected":0})
    );
    Ok(())
}
