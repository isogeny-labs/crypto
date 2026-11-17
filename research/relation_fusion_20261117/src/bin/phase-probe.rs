//! Fixed one-call phase diagnosis, not a performance confirmation or selector.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(p: &Path, b: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn hash(p: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(p)?)))
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage:phase-probe NEW_OUT FROZEN_WORKER CORPUS66".into());
    }
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    let driver = Path::new(&args[2]);
    let corpus = Path::new(&args[3]);
    let driver_hash = hash(driver)?;
    save(&out.join("driver-before.sha256"), driver_hash.as_bytes())?;
    let global = Instant::now();
    let mut outcomes = vec![];
    let mut md=String::from("# Fixed one-call phase diagnosis\n\nEach row is one planned call; times in milliseconds, no ratios or performance admission. Parent pays process/transport/proof writes; phase clocks are descriptive. [pcc:unverified]\n\n|Prime|n|Kernel|Checker|Construct+wire|Replay|Worker+proof write|Parent|\n|---|---|---|---|---:|---:|---:|---:|\n");
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            let case = Instant::now();
            let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
            let input_hash = hash(&input)?;
            for (kernel, checker) in [
                ("legacy", "legacy"),
                ("frontier", "legacy"),
                ("legacy", "packed"),
                ("frontier", "packed"),
            ] {
                if p != 2147483647 && checker == "legacy" {
                    continue;
                }
                let dir = out.join(format!("p{p}-n{n}-{kernel}-{checker}"));
                fs::create_dir(&dir)?;
                let attempt = (|| -> Result<Value> {
                    if global.elapsed() > Duration::from_secs(600)
                        || case.elapsed() > Duration::from_secs(180)
                    {
                        return Err("global/case deadline".into());
                    }
                    let stdout = fs::File::options()
                        .write(true)
                        .create_new(true)
                        .open(dir.join("call.stdout"))?;
                    let stderr = fs::File::options()
                        .write(true)
                        .create_new(true)
                        .open(dir.join("call.stderr"))?;
                    let start = Instant::now();
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
                        .arg(driver)
                        .args(["compact-sparse", checker, kernel])
                        .arg(dir.join("proof"))
                        .env("RAYON_NUM_THREADS", "1")
                        .env("OMP_NUM_THREADS", "1")
                        .env("OPENBLAS_NUM_THREADS", "1")
                        .stdin(Stdio::from(fs::File::open(&input)?))
                        .stdout(Stdio::from(stdout))
                        .stderr(Stdio::from(stderr))
                        .status()?;
                    let parent_ns = u64::try_from(start.elapsed().as_nanos())?;
                    save(
                        &dir.join("process.json"),
                        &serde_json::to_vec_pretty(
                            &json!({"exit":status.code(),"parent_ns":parent_ns,"driver_sha256":driver_hash,"input_sha256":input_hash}),
                        )?,
                    )?;
                    if !status.success()
                        || fs::metadata(dir.join("call.stdout"))?.len() > 16 << 20
                        || !fs::read(dir.join("call.stderr"))?.is_empty()
                    {
                        return Err("child failure/diagnostics".into());
                    }
                    let v: Value = serde_json::from_slice(&fs::read(dir.join("call.stdout"))?)?;
                    if v["status"] != "complete"
                        || v["prime"] != p
                        || v["variables"] != n
                        || v["checker"] != checker
                        || v["kernel"] != kernel
                        || v["S_pair_and_input_containment_checks"] != true
                    {
                        return Err("worker identity/status".into());
                    }
                    let original = fs::read(dir.join("proof/original.json"))?;
                    let wire = fs::read(dir.join("proof/witness.json"))?;
                    if original != fs::read(&input)? {
                        return Err("original changed".into());
                    }
                    let checked = relation_replay_checker::packed::verify_json(&original, &wire)?;
                    if v["basis"] != serde_json::to_value(&checked.basis)?
                        || v["consumer_stats"] != serde_json::to_value(&checked.stats)?
                    {
                        return Err("proof readback/basis/stats".into());
                    }
                    if p == 2147483647 {
                        let old = relation_replay_checker::verify_json(&original, &wire)?;
                        if old.basis != checked.basis
                            || serde_json::to_value(old.stats)?
                                != serde_json::to_value(checked.stats)?
                        {
                            return Err("legacy readback differs".into());
                        }
                    }
                    let phases = &v["phases_diagnostic_ns"];
                    let construct = phases["construct_and_wire"]
                        .as_str()
                        .ok_or("phase")?
                        .parse::<u64>()?;
                    let replay = phases["independent_replay"]
                        .as_str()
                        .ok_or("phase")?
                        .parse::<u64>()?;
                    let worker = phases["full_worker_including_proof_write"]
                        .as_str()
                        .ok_or("phase")?
                        .parse::<u64>()?;
                    if construct.checked_add(replay).is_none_or(|sum| sum > worker)
                        || worker > parent_ns
                    {
                        return Err("phase containment".into());
                    }
                    md += &format!(
                        "|{p}|{n}|{kernel}|{checker}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                        construct as f64 / 1e6,
                        replay as f64 / 1e6,
                        worker as f64 / 1e6,
                        parent_ns as f64 / 1e6
                    );
                    Ok(
                        json!({"prime":p,"n":n,"kernel":kernel,"checker":checker,"status":"complete","parent_ns":parent_ns,"construct_ns":construct,"replay_ns":replay,"worker_ns":worker,"proof_sha256":hash(&dir.join("proof/witness.json"))?}),
                    )
                })();
                let outcome = match attempt {
                    Ok(v) => v,
                    Err(e) => {
                        json!({"prime":p,"n":n,"kernel":kernel,"checker":checker,"status":"failed","reason":e.to_string()})
                    }
                };
                save(
                    &dir.join("outcome.json"),
                    &serde_json::to_vec_pretty(&outcome)?,
                )?;
                outcomes.push(outcome);
            }
            if input_hash != hash(&input)? {
                return Err("input pin changed".into());
            }
        }
    }
    let after = hash(driver)?;
    save(&out.join("driver-after.sha256"), after.as_bytes())?;
    if after != driver_hash || outcomes.len() != 18 {
        return Err("driver pin/roster".into());
    }
    let all = outcomes.iter().all(|v| v["status"] == "complete");
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"fusion-phase-diagnosis/v1","planned":18,"outcomes":outcomes,"all_correct":all,"ratios_or_admission_computed":false,"goal_15x":"active; not established"}),
        )?,
    )?;
    save(&out.join("report.md"), md.as_bytes())
}
