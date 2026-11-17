//! Prospective169: bounded child instruction attribution, never runtime admission.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = fs::File::options().write(true).create_new(true).open(p)?;
    f.write_all(b)?;
    Ok(())
}
fn hash(p: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(p)?)))
}
fn load(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 << 20 {
        return Err("artifact type/size".into());
    }
    let b = fs::read(p)?;
    if b.len() > 16 << 20 {
        return Err("artifact grew".into());
    }
    Ok(b)
}
fn pin(pins: &Value, p: &Path) -> Result<()> {
    let wanted = pins["files"]
        .as_array()
        .ok_or("pin files")?
        .iter()
        .find(|r| r["path"].as_str() == p.to_str())
        .ok_or("missing pin")?;
    if wanted["sha256"] != hash(p)? {
        return Err(format!("pin changed:{}", p.display()).into());
    }
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 7 {
        return Err(
            "usage:profile-matrix NEW_OUT FROZEN_CONTROL FROZEN_DAG CORPUS PROFILE_TOOL PINS"
                .into(),
        );
    }
    let out = Path::new(&a[1]);
    let control = Path::new(&a[2]);
    let dag = Path::new(&a[3]);
    let corpus = Path::new(&a[4]);
    let tool = Path::new(&a[5]);
    let pins: Value = serde_json::from_slice(&fs::read(&a[6])?)?;
    fs::create_dir(out)?;
    let global = Instant::now();
    let mut results = Vec::new();
    let mut bases = BTreeMap::new();
    let mut count = 0;
    for row in pins["files"].as_array().ok_or("files")? {
        pin(&pins, Path::new(row["path"].as_str().ok_or("pin path")?))?;
    }
    save(
        &out.join("pins-before.json"),
        &serde_json::to_vec_pretty(&pins)?,
    )?;
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            let case = Instant::now();
            let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
            for (kind, worker) in [("compact-sparse", control), ("compact-dag", dag)] {
                for kernel in ["frontier", "packed-products", "packed-batch"] {
                    let id = format!("p{p}-n{n}-{kind}-{kernel}");
                    let dir = out.join(&id);
                    fs::create_dir(&dir)?;
                    if global.elapsed() > Duration::from_secs(600)
                        || case.elapsed() > Duration::from_secs(180)
                    {
                        let row = json!({"case":id,"status":"not_run","reason":"original global/case deadline"});
                        save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                        results.push(row);
                        continue;
                    }
                    pin(&pins, &input)?;
                    pin(&pins, worker)?;
                    pin(&pins, tool)?;
                    let attempt = (|| -> Result<Value> {
                        let status = Command::new("taskset")
                            .args([
                                "-c",
                                "0",
                                "timeout",
                                "--kill-after=1s",
                                "30s",
                                "prlimit",
                                "--as=2147483648",
                                "--fsize=16777216",
                                "--",
                            ])
                            .arg(tool)
                            .args(["cpu", "--mode", "callgrind", "--out"])
                            .arg(dir.join("profile"))
                            .args(["--label", &id, "--"])
                            .arg(worker)
                            .args([kind, "packed", kernel])
                            .arg(dir.join("proof"))
                            .env("RAYON_NUM_THREADS", "1")
                            .env("OMP_NUM_THREADS", "1")
                            .env("OPENBLAS_NUM_THREADS", "1")
                            .stdin(Stdio::from(fs::File::open(&input)?))
                            .stdout(Stdio::from(
                                fs::File::options()
                                    .write(true)
                                    .create_new(true)
                                    .open(dir.join("tool.stdout"))?,
                            ))
                            .stderr(Stdio::from(
                                fs::File::options()
                                    .write(true)
                                    .create_new(true)
                                    .open(dir.join("tool.stderr"))?,
                            ))
                            .status()?;
                        save(
                            &dir.join("process.json"),
                            &serde_json::to_vec_pretty(
                                &json!({"exit":status.code(),"worker_sha256":hash(worker)?,"input_sha256":hash(&input)?,"profile_wall_time_is_not_a_measurement":true}),
                            )?,
                        )?;
                        if !status.success() {
                            return Err(format!("profile exit {:?}", status.code()).into());
                        }
                        let m: Value =
                            serde_json::from_slice(&load(&dir.join("profile/manifest.json"))?)?;
                        if m["status"] != "ok"
                            || m["hotspots_method"] != "callgrind"
                            || m["binary"]["sha256"] != hash(worker)?
                        {
                            return Err("profile identity/status".into());
                        }
                        let v: Value =
                            serde_json::from_slice(&load(&dir.join("profile/command.stdout"))?)?;
                        if v["status"] != "complete"
                            || v["prime"] != p
                            || v["variables"] != n
                            || v["kernel"] != kernel
                            || v["checker"] != "packed"
                            || v["S_pair_and_input_containment_checks"] != true
                        {
                            return Err("worker identity/status".into());
                        }
                        let original = load(&dir.join("proof/original.json"))?;
                        let wire = load(&dir.join("proof/witness.json"))?;
                        if original != fs::read(&input)? {
                            return Err("input changed".into());
                        }
                        let checked = if kind == "compact-dag" {
                            relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                        } else {
                            relation_replay_checker::packed::verify_json(&original, &wire)?
                        };
                        let basis = serde_json::to_value(&checked.basis)?;
                        if v["basis"] != basis
                            || v["consumer_stats"] != serde_json::to_value(&checked.stats)?
                        {
                            return Err("saved proof basis/stats mismatch".into());
                        }
                        if let Some(old) = bases.get(&(p, n)) {
                            if old != &basis {
                                return Err("cross-arm basis mismatch".into());
                            }
                        } else {
                            bases.insert((p, n), basis);
                        }
                        let hot: Value =
                            serde_json::from_slice(&load(&dir.join("profile/hotspots.json"))?)?;
                        if hot["method"] != "callgrind"
                            || hot["total_ir"].as_u64().unwrap_or(0) == 0
                        {
                            return Err("missing positive deterministic instruction count".into());
                        }
                        Ok(
                            json!({"case":id,"prime":p,"n":n,"kind":kind,"kernel":kernel,"status":"complete","complete_second_replay":true,"proof_sha256":hash(&dir.join("proof/witness.json"))?,"hotspots":hot,"timing_admission":false}),
                        )
                    })();
                    let row = match attempt {
                        Ok(v) => v,
                        Err(e) => {
                            json!({"case":id,"prime":p,"n":n,"kind":kind,"kernel":kernel,"status":"failed","reason":e.to_string(),"timing_admission":false})
                        }
                    };
                    save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                    results.push(row);
                    count += 1;
                    pin(&pins, &input)?;
                    pin(&pins, worker)?;
                }
            }
        }
    }
    for row in pins["files"].as_array().ok_or("files")? {
        pin(&pins, Path::new(row["path"].as_str().ok_or("path")?))?;
    }
    save(
        &out.join("pins-after.json"),
        &serde_json::to_vec_pretty(&pins)?,
    )?;
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"relation-profile-matrix/v1","attempted":count,"outcomes":results,"performance_measurement":false,"speedup_claim":false,"independent_host":false}),
        )?,
    )?;
    Ok(())
}
