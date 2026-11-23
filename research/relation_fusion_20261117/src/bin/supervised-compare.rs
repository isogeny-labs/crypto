//! Preparatory matched-launcher collector. No comparison has been run.
//! Use only after the canonical protocol/export is qualified and source/pins published.
use relation_native_supervisor::{run, Limits};
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
include!("../supervised_roster.rs");
fn main() -> Result<()> {
    let global = Instant::now();
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--print-roster" {
        let stages = ["AA", "AB"].into_iter().map(|stage| -> Result<Value> {
            let arms = supervised_roster(stage)?.into_iter().map(|a| {
                json!({"arm":a.name,"kind":a.kind,"kernel":a.kernel,"worker":a.worker,"native":a.native})
            }).collect::<Vec<_>>();
            Ok(json!({"stage":stage,"arms":arms}))
        }).collect::<Result<Vec<_>>>()?;
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schema":"supervised-match-design-draft/v1",
                "canonical_protocol_validation":"pending",
                "primary":PRIMARY,"rounds":ROUNDS,"planned_calls":PLANNED_CALLS,
                "primes":[2147483647u64,9007199254740881u64],"n":[3,4,6],
                "seed":7,"graph":"LAST-star","stages":stages,
                "performance_observations_collected":0
            }))?
        );
        return Ok(());
    }
    if args.len() != 7 {
        return Err(
            "usage:supervised-compare NEW_OUT FROZEN85 FROZEN96 CURRENT_WORKER CORPUS66 PINS"
                .into(),
        );
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
    let pin_bytes = fs::read(&args[6])?;
    let pins: Value = serde_json::from_slice(&pin_bytes)?;
    if pins["schema"] != "supervised-match-pins/v1"
        || pins["worker_sha256"] != serde_json::to_value(&hashes)?
        || pins["collector_sha256"] != hash(&std::env::current_exe()?)?
    {
        return Err("prospective worker/collector pins".into());
    }
    save(&out.join("prospective-pins.json"), &pin_bytes)?;
    save(
        &out.join("workers-before.json"),
        &serde_json::to_vec_pretty(&hashes)?,
    )?;
    let mut outcomes = vec![];
    let mut bases = std::collections::BTreeMap::new();
    let mut md=String::from("# Repeated matched-launcher comparison: raw calls\n\nEach row is one planned call; times in milliseconds. Separate Rust analysis computes matched statistics from all raw calls. Parent pays raw capture,process/transport/proof writes and uniform complete packed readback; phase clocks are descriptive. [pcc:unverified]\n\n|Prime|n|Kernel|Checker|Construct+wire|Replay|Worker+proof write|Parent|\n|---|---|---|---|---:|---:|---:|---:|\n");
    for stage in ["AA", "AB"] {
        for round in 0..ROUNDS {
            for p in [2147483647u64, 9007199254740881] {
                for n in [3, 4, 6] {
                    let case = Instant::now();
                    let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
                    let input_hash = hash(&input)?;
                    if pins["input_sha256"][format!("p{p}-n{n}")] != input_hash {
                        return Err("prospective input pin".into());
                    }
                    let modes = supervised_roster(stage)?;
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
                            let mut command = if mode.native {
                                Command::new(driver)
                            } else {
                                let mut command = Command::new("taskset");
                                command
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
                                    .arg(driver);
                                command
                            };
                            command
                                .args([kind, checker, kernel])
                                .arg(dir.join("proof"))
                                .env("RAYON_NUM_THREADS", "1")
                                .env("OMP_NUM_THREADS", "1")
                                .env("OPENBLAS_NUM_THREADS", "1")
                                .env("RUST_TEST_THREADS", "1")
                                .stdin(Stdio::from(fs::File::open(&input)?))
                                .stdout(Stdio::from(stdout))
                                .stderr(Stdio::from(stderr));
                            let (status, timed_out) = if mode.native {
                                let result = run(&mut command, Limits::default())?;
                                (result.status, result.timed_out)
                            } else {
                                // Every nonzero/signal result is retained and rejects admission.
                                (command.status()?, false)
                            };
                            let parent_ns = u64::try_from(start.elapsed().as_nanos())?;
                            save(
                                &dir.join("process.json"),
                                &serde_json::to_vec_pretty(
                                    &json!({"exit":status.code(),"parent_ns":parent_ns,"driver_sha256":driver_hash,"input_sha256":input_hash,"native":mode.native,"timed_out":timed_out,"resource_envelope":{"cpu":0,"threads":1,"address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30}}),
                                )?,
                            )?;
                            if !status.success()
                                || timed_out
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
                            let original = fs::read(dir.join("proof/original.json"))?;
                            let wire = fs::read(dir.join("proof/witness.json"))?;
                            if original != fs::read(&input)? {
                                return Err("original changed".into());
                            }
                            let checked = if kind.ends_with("-dag") {
                                relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                            } else {
                                relation_replay_checker::packed::verify_json(&original, &wire)?
                            };
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
                            let total_ns = u64::try_from(total.elapsed().as_nanos())?;
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
                                json!({"stage":stage,"round":round,"arm":arm,"native":mode.native,"seed":7,"graph":"LAST-star","observed_solving_degree":null,"basis_sha256":basis_sha256,"prime":p,"n":n,"kernel":kernel,"checker":checker,"status":"complete","kind":kind,"parent_ns":total_ns,"child_process_ns":parent_ns,"construct_ns":construct,"replay_ns":replay,"worker_ns":worker,"proof_sha256":proof_sha256}),
                            )
                        })();
                        let outcome = match attempt {
                            Ok(v) => v,
                            Err(e) => {
                                json!({"stage":stage,"round":round,"arm":arm,"native":mode.native,"seed":7,"graph":"LAST-star","observed_solving_degree":null,"prime":p,"n":n,"kind":kind,"kernel":kernel,"checker":checker,"status":"failed","reason":e.to_string()})
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
    if after != hashes || outcomes.len() != PLANNED_CALLS {
        return Err("worker pin/roster".into());
    }
    let all = outcomes.iter().all(|v| v["status"] == "complete");
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"fusion-supervised-match/v1","planned":PLANNED_CALLS,"primary":PRIMARY,"prospective_pins_sha256":format!("{:x}",Sha256::digest(&pin_bytes)),"outcomes":outcomes,"all_correct":all,"ratios_or_admission_computed":false,"goal_15x":"active; not established"}),
        )?,
    )?;
    save(&out.join("report.md"), md.as_bytes())
}
