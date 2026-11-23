//! Actual flat worker dispatch/transport qualification, with no parent timing statistics.
use relation_native_supervisor::{run, Limits};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
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
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file shape/cap".into());
    }
    Ok(fs::read(p)?)
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn launch(
    worker: &Path,
    input: &Path,
    mode: &str,
    kernel: &str,
    dir: &Path,
    native: bool,
    worker_hash: &str,
) -> Result<bool> {
    let mut command = if native {
        Command::new(worker)
    } else {
        let mut c = Command::new("taskset");
        c.args([
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
        .arg(worker);
        c
    };
    command
        .args([mode, "packed", kernel])
        .arg(dir.join("proof"))
        .env("RAYON_NUM_THREADS", "1")
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .env("RUST_TEST_THREADS", "1")
        .stdin(Stdio::from(fs::File::open(input)?))
        .stdout(Stdio::from(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(dir.join("call.stdout"))?,
        ))
        .stderr(Stdio::from(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(dir.join("call.stderr"))?,
        ));
    let (status, timed_out) = if native {
        let r = run(&mut command, Limits::default())?;
        (r.status, r.timed_out)
    } else {
        (command.status()?, false)
    };
    save(
        &dir.join("process.json"),
        &serde_json::to_vec_pretty(&json!({
        "native":native,"exit":status.code(),"success":status.success(),"timed_out":timed_out,
        "worker_sha256":worker_hash,"resource_envelope":{"cpu":0,"threads":1,
            "address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30},
        "parent_performance_statistics":false}))?,
    )?;
    if timed_out || (!status.success() && status.code() != Some(1)) {
        return Err("unexpected process failure/timeout".into());
    }
    Ok(status.success())
}
fn main() -> Result<()> {
    let start = Instant::now();
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage:flat-worker-qualify WORKER CORPUS66 REFERENCE277 NEW_OUT".into());
    }
    let worker = Path::new(&a[1]);
    let corpus = Path::new(&a[2]);
    let reference = Path::new(&a[3]);
    let out = Path::new(&a[4]);
    fs::create_dir(out)?;
    let worker_hash = sha(&read(worker)?);
    let mut controls = Vec::new();
    for native in [false, true] {
        let input = corpus.join("p2147483647-n3-frontier/original.json");
        for collision in [false, true] {
            let dir = out.join(format!(
                "control-{}-{}",
                if native { "native" } else { "wrapped" },
                if collision {
                    "collision"
                } else {
                    "invalid-mode"
                }
            ));
            fs::create_dir(&dir)?;
            if collision {
                fs::create_dir(dir.join("proof"))?;
                save(
                    &dir.join("proof/sentinel"),
                    b"preserve-existing-proof-owner",
                )?;
            }
            let success = launch(
                worker,
                &input,
                if collision {
                    "flat-constructor-dag"
                } else {
                    "not-a-mode"
                },
                "packed-batch",
                &dir,
                native,
                &worker_hash,
            )?;
            if success {
                return Err("negative transport control unexpectedly passed".into());
            }
            if collision
                && (read(&dir.join("proof/sentinel"))? != b"preserve-existing-proof-owner"
                    || !String::from_utf8(read(&dir.join("call.stderr"))?)?
                        .to_lowercase()
                        .contains("exist"))
            {
                return Err("proof ownership control".into());
            }
            controls.push(json!({"native":native,"collision":collision,"expected_failure":true,"later_arms_remain_planned":true}));
        }
    }
    let mut rows = Vec::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for dag in [false, true] {
                for (name, kernel) in [
                    ("frontier103", "frontier"),
                    ("products231", "packed-products"),
                    ("batch487", "packed-batch"),
                ] {
                    let base = format!("p{p}-n{n}-{}-{name}", if dag { "dag" } else { "sparse" });
                    let expected = value(&reference.join(&base).join("outcome.json"))?;
                    let expected_wire = read(&reference.join(&base).join("flat-proof.json"))?;
                    let mut pair = None;
                    for native in [false, true] {
                        let id = format!("{base}-{}", if native { "native" } else { "wrapped" });
                        let dir = out.join(&id);
                        fs::create_dir(&dir)?;
                        let attempt = (|| -> Result<Value> {
                            if start.elapsed().as_secs() >= 600 {
                                return Err("qualification global deadline".into());
                            }
                            let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
                            let original = read(&input)?;
                            if !launch(
                                worker,
                                &input,
                                if dag {
                                    "flat-constructor-dag"
                                } else {
                                    "flat-constructor-sparse"
                                },
                                kernel,
                                &dir,
                                native,
                                &worker_hash,
                            )? {
                                return Err("positive worker call failed".into());
                            }
                            let saved = read(&dir.join("proof/original.json"))?;
                            let proof = read(&dir.join("proof/witness.json"))?;
                            if saved != original
                                || proof != expected_wire
                                || !read(&dir.join("call.stderr"))?.is_empty()
                            {
                                return Err("exact input/wire/diagnostics".into());
                            }
                            let checked = if dag {
                                relation_replay_checker::packed::verify_dag_json(&saved, &proof)?
                            } else {
                                relation_replay_checker::packed::verify_json(&saved, &proof)?
                            };
                            let v = value(&dir.join("call.stdout"))?;
                            let basis = serde_json::to_value(&checked.basis)?;
                            let stats = serde_json::to_value(&checked.stats)?;
                            if v["status"] != "complete"
                                || v["prime"] != p
                                || v["variables"] != n
                                || v["checker"] != "packed"
                                || v["kernel"] != kernel
                                || v["S_pair_and_input_containment_checks"] != true
                                || v["basis"] != basis
                                || v["consumer_stats"] != stats
                                || v["constructor"] != expected["metadata"]
                                || basis != expected["checked_basis"]
                                || stats != expected["checked_stats"]
                            {
                                return Err("worker metadata or complete replay binding".into());
                            }
                            let identity = json!({"wire_sha256":sha(&proof),"basis":basis,"stats":stats,"constructor":v["constructor"]});
                            if pair.as_ref().is_some_and(|old| old != &identity) {
                                return Err("launcher pair identity".into());
                            }
                            pair = Some(identity.clone());
                            Ok(
                                json!({"id":id,"prime":p,"n":n,"seed":7,"graph":"LAST-star","dag":dag,"kernel":kernel,"native":native,
                        "input_sha256":sha(&saved),"status":"complete","identity":identity,"observed_solving_degree":null,"timing_claim":false}),
                            )
                        })();
                        let row = match attempt {
                            Ok(v) => v,
                            Err(e) => {
                                json!({"id":id,"status":"failed","reason":e.to_string(),"timing_claim":false})
                            }
                        };
                        save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                        rows.push(row);
                    }
                }
            }
        }
    }
    let unchanged = worker_hash == sha(&read(worker)?);
    let complete = unchanged && rows.len() == 72 && rows.iter().all(|r| r["status"] == "complete");
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(&json!({"schema":"flat-worker-functional-qualification/v1",
        "planned":72,"cases":rows,"all_complete":complete,"worker_sha256":worker_hash,"worker_hash_unchanged":unchanged,
        "transport_controls":controls,"raw_worker_diagnostic_clocks_retained":true,"parent_performance_statistics_computed":false,"timing_claim":false}))?,
    )?;
    if !complete {
        return Err("incomplete functional qualification; all raw outcomes retained".into());
    }
    println!(
        "{}",
        json!({"complete_cases":72,"launcher_equalities":36,"negative_transport_controls":4,"parent_timing_claim":false})
    );
    Ok(())
}
