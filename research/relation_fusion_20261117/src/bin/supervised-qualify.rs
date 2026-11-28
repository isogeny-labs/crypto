//! Functional transport/full-proof qualification. No parent performance statistics.
use relation_native_supervisor::{run, Limits};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../packed_roster.rs");
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
        return Err("file type/size".into());
    }
    Ok(fs::read(p)?)
}
fn launch(
    worker: &Path,
    args: &[String],
    input: Option<&Path>,
    dir: &Path,
    native: bool,
) -> Result<(ExitStatus, bool)> {
    let mut c = if native {
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
    c.args(args)
        .env("RAYON_NUM_THREADS", "1")
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .env("RUST_TEST_THREADS", "1");
    c.stdin(if let Some(p) = input {
        Stdio::from(fs::File::open(p)?)
    } else {
        Stdio::null()
    });
    c.stdout(Stdio::from(
        fs::File::options()
            .write(true)
            .create_new(true)
            .open(dir.join("call.stdout"))?,
    ));
    c.stderr(Stdio::from(
        fs::File::options()
            .write(true)
            .create_new(true)
            .open(dir.join("call.stderr"))?,
    ));
    let (status, timed_out) = if native {
        let result = run(&mut c, Limits::default())?;
        (result.status, result.timed_out)
    } else {
        (c.status()?, false)
    };
    save(
        &dir.join("process.json"),
        &serde_json::to_vec_pretty(
            &json!({"exit":status.code(),"success":status.success(),"native":native,"timed_out":timed_out,"worker_sha256":sha(&read(worker)?),"resource_envelope":{"cpu":0,"threads":1,"address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30},"performance_claim":false}),
        )?,
    )?;
    Ok((status, timed_out))
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 8 {
        return Err(
            "usage:supervised-qualify NEW_OUT CORPUS FROZEN85 FROZEN96 FROZEN200 PROBE PINS200"
                .into(),
        );
    }
    let deadline = Instant::now();
    let out = Path::new(&a[1]);
    fs::create_dir(out)?;
    let corpus = Path::new(&a[2]);
    let workers = [Path::new(&a[3]), Path::new(&a[4]), Path::new(&a[5])];
    let probe = Path::new(&a[6]);
    let pins: Value = serde_json::from_slice(&read(Path::new(&a[7]))?)?;
    let hashes = workers
        .iter()
        .map(|p| read(p).map(|b| sha(&b)))
        .collect::<Result<Vec<_>>>()?;
    if json!(hashes) != pins["worker_sha256"] {
        return Err("worker pins".into());
    }
    save(
        &out.join("workers-before.json"),
        &serde_json::to_vec_pretty(&hashes)?,
    )?;
    let mut transport = Vec::new();
    for native in [false, true] {
        for (mode, expected) in [("invalid-mode", false), ("limits", true)] {
            let id = format!(
                "transport-{}-{mode}",
                if native { "native" } else { "wrapped" }
            );
            let dir = out.join(&id);
            fs::create_dir(&dir)?;
            let (status, timed) = launch(probe, &[mode.into()], None, &dir, native)?;
            if timed || status.success() != expected {
                return Err(format!("transport outcome {id}").into());
            }
            if expected {
                let text = String::from_utf8(read(&dir.join("call.stdout"))?)?;
                let v: Vec<_> = text.split_whitespace().collect();
                if v.len() != 7
                    || v[..6]
                        != [
                            "2147483648",
                            "2147483648",
                            "16777216",
                            "16777216",
                            "0",
                            "true",
                        ]
                {
                    return Err("transport OS envelope".into());
                }
            }
            transport.push(json!({"id":id,"expected_success":expected,"observed_success":status.success(),"later_arms_still_planned":true}));
        }
        let dir = out.join(format!(
            "collision-{}",
            if native { "native" } else { "wrapped" }
        ));
        fs::create_dir(&dir)?;
        let proof = dir.join("proof");
        fs::create_dir(&proof)?;
        save(&proof.join("sentinel"), b"preserve-existing-proof-owner")?;
        let input = corpus.join("p2147483647-n3-frontier/original.json");
        let args = vec![
            "packed-constructor-dag".into(),
            "packed".into(),
            "packed-batch".into(),
            proof.to_str().ok_or("path UTF8")?.into(),
        ];
        let (status, timed) = launch(workers[2], &args, Some(&input), &dir, native)?;
        let stderr = String::from_utf8(read(&dir.join("call.stderr"))?)?;
        if status.success()
            || timed
            || !stderr.to_lowercase().contains("exist")
            || read(&proof.join("sentinel"))? != b"preserve-existing-proof-owner"
        {
            return Err("collision transport contract".into());
        }
        transport.push(
            json!({"native":native,"expected_collision_rejected":true,"sentinel_preserved":true}),
        );
    }
    let mut cases = Vec::new();
    let mut bases = std::collections::BTreeMap::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for arm in roster("AB") {
                let input = corpus.join(format!("p{p}-n{n}-frontier/original.json"));
                let raw = read(&input)?;
                if pins["input_sha256"][format!("p{p}-n{n}")] != sha(&raw) {
                    return Err("fixed input pin".into());
                }
                let mut pair = None;
                for native in [false, true] {
                    let id = format!(
                        "p{p}-n{n}-{}-{}",
                        arm.name,
                        if native { "native" } else { "wrapped" }
                    );
                    let dir = out.join(&id);
                    fs::create_dir(&dir)?;
                    let result = (|| -> Result<Value> {
                        if deadline.elapsed() > Duration::from_secs(600) {
                            return Err("qualification global deadline".into());
                        }
                        let proof = dir.join("proof");
                        let args = vec![
                            arm.kind.into(),
                            "packed".into(),
                            arm.kernel.into(),
                            proof.to_str().ok_or("path UTF8")?.into(),
                        ];
                        let (status, timed) =
                            launch(workers[arm.worker], &args, Some(&input), &dir, native)?;
                        if !status.success() || timed || !read(&dir.join("call.stderr"))?.is_empty()
                        {
                            return Err("positive worker failure".into());
                        }
                        let stdout: Value =
                            serde_json::from_slice(&read(&dir.join("call.stdout"))?)?;
                        let original = read(&proof.join("original.json"))?;
                        let wire = read(&proof.join("witness.json"))?;
                        if original != raw {
                            return Err("original bytes changed".into());
                        }
                        let checked = if arm.kind.ends_with("-dag") {
                            relation_replay_checker::packed::verify_dag_json(&original, &wire)?
                        } else {
                            relation_replay_checker::packed::verify_json(&original, &wire)?
                        };
                        let basis = serde_json::to_value(&checked.basis)?;
                        let stats = serde_json::to_value(&checked.stats)?;
                        if stdout["status"] != "complete"
                            || stdout["prime"] != p
                            || stdout["variables"] != n
                            || stdout["checker"] != "packed"
                            || stdout["kernel"] != arm.kernel
                            || stdout["S_pair_and_input_containment_checks"] != true
                            || stdout["basis"] != basis
                            || stdout["consumer_stats"] != stats
                        {
                            return Err("worker/proof/basis/stat identity".into());
                        }
                        if let Some(expected) = bases.get(&(p, n)) {
                            if expected != &basis {
                                return Err("cross-arm basis".into());
                            }
                        } else {
                            bases.insert((p, n), basis.clone());
                        }
                        let identity = json!({"wire_sha256":sha(&wire),"basis":basis,"stats":stats,"constructor":stdout["constructor"]});
                        if let Some(expected) = &pair {
                            if expected != &identity {
                                return Err("native/wrapped proof or metadata differs".into());
                            }
                        } else {
                            pair = Some(identity.clone());
                        }
                        Ok(
                            json!({"id":id,"status":"complete","prime":p,"n":n,"seed":7,"graph":"LAST-star","arm":arm.name,"kind":arm.kind,"kernel":arm.kernel,"native":native,"input_sha256":sha(&raw),"identity":identity,"performance_claim":false,"observed_solving_degree":null}),
                        )
                    })();
                    let outcome = match result {
                        Ok(v) => v,
                        Err(e) => {
                            json!({"id":id,"status":"failed","native":native,"reason":e.to_string(),"performance_claim":false})
                        }
                    };
                    save(
                        &dir.join("outcome.json"),
                        &serde_json::to_vec_pretty(&outcome)?,
                    )?;
                    cases.push(outcome);
                }
            }
        }
    }
    let after = workers
        .iter()
        .map(|p| read(p).map(|b| sha(&b)))
        .collect::<Result<Vec<_>>>()?;
    save(
        &out.join("workers-after.json"),
        &serde_json::to_vec_pretty(&after)?,
    )?;
    let complete =
        cases.len() == 216 && cases.iter().all(|v| v["status"] == "complete") && after == hashes;
    let report = json!({"schema":"native-supervisor-functional-qualification/v1","planned_proof_cases":216,"cases":cases,"transport_controls":transport,"all_complete":complete,"worker_hashes_unchanged":after==hashes,"parent_performance_statistics_computed":false,"primary_15x_admitted":false});
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(&report)?,
    )?;
    println!(
        "{}",
        json!({"proof_cases":cases.len(),"all_complete":complete,"performance_claim":false})
    );
    if !complete {
        return Err("qualification incomplete".into());
    }
    Ok(())
}
