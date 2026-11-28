//! Fixed-input transport/proof qualification. No comparative/worker clocks.
use relation_native_supervisor::{run, Limits};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../native_supervised_roster.rs");
fn save(p: &Path, b: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn validate(
    v: &Value,
    arm: &SupervisedArm,
    p: u64,
    n: usize,
    original: &[u8],
    wire: &[u8],
) -> Result<()> {
    if !worker_identity_valid(v, arm, p, n)
        || v["qualification_without_worker_clocks"] != true
        || v["performance_observations_collected"] != 0
    {
        return Err("worker identity/qualification scope".into());
    }
    for key in [
        "construct_and_wire",
        "independent_replay",
        "full_worker_including_proof_write",
    ] {
        if v["phases_diagnostic_ns"].get(key) != Some(&Value::Null) {
            return Err("qualification recorded a worker clock".into());
        }
    }
    let checked = if arm.kind.ends_with("-dag") {
        relation_replay_checker::packed::verify_dag_json(original, wire)?
    } else {
        relation_replay_checker::packed::verify_json(original, wire)?
    };
    if v["basis"] != serde_json::to_value(&checked.basis)?
        || v["consumer_stats"] != serde_json::to_value(&checked.stats)?
        || !root_native_basis_matches(v, arm, &checked, p, n)
    {
        return Err("complete proof/basis/receipt binding".into());
    }
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err(
            "usage:root-transport-qualify FROZEN_WORKER REFERENCE_INPUTS FOUR_WORKER_PINS NEW_OUT"
                .into(),
        );
    }
    let worker = Path::new(&args[1]);
    let corpus = Path::new(&args[2]);
    let out = Path::new(&args[4]);
    let pins: Value = serde_json::from_slice(&fs::read(&args[3])?)?;
    let worker_sha = sha(&fs::read(worker)?);
    if pins["worker_sha256"][3] != worker_sha {
        return Err("prospective root-worker pin".into());
    }
    fs::create_dir(out)?;
    let mut outcomes = Vec::new();
    let mut failures = Vec::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            let original = fs::read(corpus.join(format!("p{p}-n{n}-frontier/original.json")))?;
            if pins["input_sha256"][format!("p{p}-n{n}")] != sha(&original) {
                return Err("fixed input pin".into());
            }
            for arm in root_arms(false).into_iter().chain(root_arms(true)) {
                let dir = out.join(format!("p{p}-n{n}-{}", arm.name));
                fs::create_dir(&dir)?;
                let attempt = (|| -> Result<Value> {
                    let mut command = if arm.native {
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
                        .args([arm.kind, "packed", arm.kernel])
                        .arg(dir.join("proof"))
                        .arg("--qualify")
                        .env("RAYON_NUM_THREADS", "1")
                        .env("OMP_NUM_THREADS", "1")
                        .env("OPENBLAS_NUM_THREADS", "1")
                        .env("RUST_TEST_THREADS", "1")
                        .stdin(Stdio::from(fs::File::open(
                            corpus.join(format!("p{p}-n{n}-frontier/original.json")),
                        )?))
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
                    let (status, timed_out) = if arm.native {
                        let r = run(&mut command, Limits::default())?;
                        (r.status, r.timed_out)
                    } else {
                        (command.status()?, false)
                    };
                    save(
                        &dir.join("process.json"),
                        &serde_json::to_vec_pretty(
                            &json!({"exit":status.code(),"timed_out":timed_out,"native":arm.native,"worker_sha256":worker_sha,"qualification_only":true,"performance_observations_collected":0}),
                        )?,
                    )?;
                    if !status.success()
                        || timed_out
                        || !fs::read(dir.join("call.stderr"))?.is_empty()
                    {
                        return Err("child refusal/failure".into());
                    }
                    let v: Value = serde_json::from_slice(&fs::read(dir.join("call.stdout"))?)?;
                    if fs::read(dir.join("proof/original.json"))? != original {
                        return Err("transport changed fixed input".into());
                    }
                    let wire = fs::read(dir.join("proof/witness.json"))?;
                    validate(&v, &arm, p, n, &original, &wire)?;
                    let mut bad = v.clone();
                    bad["native_check_kind"] = json!("forged");
                    if validate(&bad, &arm, p, n, &original, &wire).is_ok() {
                        return Err("kind canary accepted".into());
                    }
                    let mut bad = v.clone();
                    bad["native_basis"][0][0][1] = json!(0);
                    if validate(&bad, &arm, p, n, &original, &wire).is_ok() {
                        return Err("native-basis canary accepted".into());
                    }
                    let mut bad = v.clone();
                    bad["consumer_stats"]["cofactor_terms"] = json!(usize::MAX);
                    if validate(&bad, &arm, p, n, &original, &wire).is_ok() {
                        return Err("stats canary accepted".into());
                    }
                    if validate(&v, &arm, p, n, &original, b"!").is_ok() {
                        return Err("proof canary accepted".into());
                    }
                    Ok(
                        json!({"prime":p,"n":n,"arm":arm.name,"kind":arm.kind,"kernel":arm.kernel,"native":arm.native,"status":"complete","proof_sha256":sha(&wire),"input_sha256":sha(&original),"canaries_rejected":4,"performance_observations_collected":0}),
                    )
                })();
                let row = match attempt {
                    Ok(v) => v,
                    Err(e) => {
                        failures.push(format!("{}: {e}", arm.name));
                        json!({"prime":p,"n":n,"arm":arm.name,"status":"failed","reason":e.to_string()})
                    }
                };
                save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                outcomes.push(row);
            }
        }
    }
    if sha(&fs::read(worker)?) != worker_sha {
        return Err("worker changed during qualification".into());
    }
    let complete = failures.is_empty() && outcomes.len() == 432;
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"root-worker-transport-qualification/v1","all_complete":complete,"planned":432,"outcomes":outcomes,"failures":failures,"worker_sha256":worker_sha,"performance_observations_collected":0,"15x_admitted":false,"signed_verdict":false}),
        )?,
    )?;
    if !complete {
        return Err("incomplete transport/proof qualification; every failure retained".into());
    }
    println!(
        "{}",
        json!({"complete_calls":432,"complete_parent_proof_replays":432,"canaries_rejected":1728,"performance_observations_collected":0,"15x_admitted":false})
    );
    Ok(())
}
