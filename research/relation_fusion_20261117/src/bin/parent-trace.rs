//! Descriptive exclusive parent phase trace. Not a speedup-admission experiment.
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
include!("../packed_roster.rs");
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        return Err("usage:parent-trace NEW_OUT FROZEN85 FROZEN96 CURRENT_WORKER CORPUS66".into());
    }
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    let drivers = [
        Path::new(&args[2]),
        Path::new(&args[3]),
        Path::new(&args[4]),
    ];
    let hashes = drivers
        .iter()
        .map(|p| hash(p))
        .collect::<Result<Vec<_>>>()?;
    let corpus = Path::new(&args[5]);
    save(
        &out.join("workers-before.json"),
        &serde_json::to_vec_pretty(&hashes)?,
    )?;
    let global = Instant::now();
    let mut outcomes = vec![];
    let mut bases = std::collections::BTreeMap::new();
    let mut md=String::from("# Descriptive parent trace: raw calls\n\nEach row is one planned call; times in milliseconds. Exclusive parent phases are descriptive and cannot admit speedup. Parent pays raw capture,process/transport/proof writes and uniform complete packed readback; phase clocks are descriptive. [pcc:unverified]\n\n|Prime|n|Kernel|Checker|Construct+wire|Replay|Worker+proof write|Parent|\n|---|---|---|---|---:|---:|---:|---:|\n");
    for stage in ["TRACE"] {
        for round in 0..3 {
            for p in [2147483647u64, 9007199254740881] {
                for n in [3, 4, 6] {
                    let case = Instant::now();
                    let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
                    let input_hash = hash(&input)?;
                    let modes = roster("AB");
                    for index in 0..modes.len() {
                        let mode = &modes[(index + round) % modes.len()];
                        let arm = mode.name.as_str();
                        let kind = mode.kind;
                        let kernel = mode.kernel;
                        let driver = drivers[mode.worker];
                        let driver_hash = &hashes[mode.worker];
                        let checker = "packed";
                        let dir = out.join(format!("{stage}-r{round}-p{p}-n{n}-{arm}"));
                        fs::create_dir(&dir)?;
                        let attempt = (|| -> Result<Value> {
                            let total = Instant::now();
                            let capture_start = Instant::now();
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
                            let stdin = fs::File::open(&input)?;
                            let capture_ns = u64::try_from(capture_start.elapsed().as_nanos())?;
                            let start = Instant::now();
                            let mut child = Command::new("taskset")
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
                                .args([kind, checker, kernel])
                                .arg(dir.join("proof"))
                                .env("RAYON_NUM_THREADS", "1")
                                .env("OMP_NUM_THREADS", "1")
                                .env("OPENBLAS_NUM_THREADS", "1")
                                .stdin(Stdio::from(stdin))
                                .stdout(Stdio::from(stdout))
                                .stderr(Stdio::from(stderr))
                                .spawn()?;
                            let spawn_ns = u64::try_from(start.elapsed().as_nanos())?;
                            let wait_start = Instant::now();
                            let status = child.wait()?;
                            let wait_ns = u64::try_from(wait_start.elapsed().as_nanos())?;
                            let parent_ns = u64::try_from(start.elapsed().as_nanos())?;
                            let receipt_start = Instant::now();
                            save(
                                &dir.join("process.json"),
                                &serde_json::to_vec_pretty(
                                    &json!({"exit":status.code(),"parent_ns":parent_ns,"driver_sha256":driver_hash,"input_sha256":input_hash}),
                                )?,
                            )?;
                            let receipt_ns = u64::try_from(receipt_start.elapsed().as_nanos())?;
                            let parse_start = Instant::now();
                            if !status.success()
                                || fs::metadata(dir.join("call.stdout"))?.len() > 16 << 20
                                || !fs::read(dir.join("call.stderr"))?.is_empty()
                            {
                                return Err("child failure/diagnostics".into());
                            }
                            let v: Value =
                                serde_json::from_slice(&fs::read(dir.join("call.stdout"))?)?;
                            if v["status"] != "complete"
                                || v["prime"] != p
                                || v["variables"] != n
                                || v["checker"] != checker
                                || v["kernel"] != kernel
                                || v["S_pair_and_input_containment_checks"] != true
                            {
                                return Err("worker identity/status".into());
                            }
                            let parse_ns = u64::try_from(parse_start.elapsed().as_nanos())?;
                            let readback_start = Instant::now();
                            let original = fs::read(dir.join("proof/original.json"))?;
                            let wire = fs::read(dir.join("proof/witness.json"))?;
                            if original != fs::read(&input)? {
                                return Err("original changed".into());
                            }
                            let readback_ns = u64::try_from(readback_start.elapsed().as_nanos())?;
                            let replay_start = Instant::now();
                            let checked = if kind.ends_with("-dag") {
                                relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                            } else {
                                relation_replay_checker::packed::verify_json(&original, &wire)?
                            };
                            let controller_replay_ns =
                                u64::try_from(replay_start.elapsed().as_nanos())?;
                            let binding_start = Instant::now();
                            if v["basis"] != serde_json::to_value(&checked.basis)?
                                || v["consumer_stats"] != serde_json::to_value(&checked.stats)?
                            {
                                return Err("proof readback/basis/stats".into());
                            }
                            let basis = serde_json::to_value(&checked.basis)?;
                            if let Some(expected) = bases.get(&(p, n)) {
                                if expected != &basis {
                                    return Err("cross-arm complete-basis mismatch".into());
                                }
                            } else {
                                bases.insert((p, n), basis.clone());
                            }
                            let proof_sha256 = hash(&dir.join("proof/witness.json"))?;
                            let basis_sha256 =
                                format!("{:x}", Sha256::digest(serde_json::to_vec(&basis)?));
                            let binding_ns = u64::try_from(binding_start.elapsed().as_nanos())?;
                            let total_ns = u64::try_from(total.elapsed().as_nanos())?;
                            let partition = std::collections::BTreeMap::from([
                                ("capture_input_and_deadline_gate", capture_ns),
                                ("spawn_call", spawn_ns),
                                ("wait_for_child", wait_ns),
                                ("process_receipt_write", receipt_ns),
                                ("output_parse_and_child_gates", parse_ns),
                                ("artifact_readback_and_input_binding", readback_ns),
                                ("controller_whole_proof_replay", controller_replay_ns),
                                ("basis_stat_cross_arm_binding_and_digests", binding_ns),
                            ]);
                            let accounted = partition
                                .values()
                                .try_fold(0u64, |a, b| a.checked_add(*b))
                                .ok_or("phase sum overflow")?;
                            let residual = total_ns
                                .checked_sub(accounted)
                                .ok_or("phase partition exceeds parent")?;
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
                                "|{p}|{n}|{kind}/{kernel}|{checker}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                                construct as f64 / 1e6,
                                replay as f64 / 1e6,
                                worker as f64 / 1e6,
                                total_ns as f64 / 1e6
                            );
                            Ok(
                                json!({"stage":stage,"round":round,"arm":arm,"basis_sha256":basis_sha256,"prime":p,"n":n,"kernel":kernel,"checker":checker,"status":"complete","kind":kind,"parent_ns":total_ns,"child_process_ns":parent_ns,"construct_ns":construct,"replay_ns":replay,"worker_ns":worker,"proof_sha256":proof_sha256,"parent_exclusive_ns":partition,"residual_bookkeeping_ns":residual,"trace_only":true}),
                            )
                        })();
                        let outcome = match attempt {
                            Ok(v) => v,
                            Err(e) => {
                                json!({"stage":stage,"round":round,"arm":arm,"prime":p,"n":n,"kind":kind,"kernel":kernel,"checker":checker,"status":"failed","reason":e.to_string()})
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
        }
    }
    let after = drivers
        .iter()
        .map(|p| hash(p))
        .collect::<Result<Vec<_>>>()?;
    save(
        &out.join("workers-after.json"),
        &serde_json::to_vec_pretty(&after)?,
    )?;
    if after != hashes || outcomes.len() != 324 {
        return Err("worker pin/roster".into());
    }
    let all = outcomes.iter().all(|v| v["status"] == "complete");
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"fusion-parent-trace/v1","planned":324,"outcomes":outcomes,"all_correct":all,"ratios_or_admission_computed":false,"goal_15x":"active; not established"}),
        )?,
    )?;
    save(&out.join("report.md"), md.as_bytes())
}
