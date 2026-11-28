//! Finite, create-new native orchestration; every attempted call is retained.
#![allow(dead_code, clippy::duplicate_mod)]
#[path = "qualify.rs"]
mod qualification;
use qualification::{collection_check, driver, remainder};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};

fn read(p: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn put(w: &mut impl Write, v: &Value) -> std::io::Result<()> {
    serde_json::to_writer(&mut *w, v)?;
    w.write_all(b"\n")?;
    w.flush()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 6 {
        return Err(
            "usage: RUN WORKER NEW_DIR PUBLIC_JSON PRIVATE_FIXTURE SOURCE_SHA ISOLATION_SCRIPT"
                .into(),
        );
    }
    let worker = fs::canonicalize(&a[0])?;
    let dir = PathBuf::from(&a[1]);
    fs::create_dir(&dir)?;
    let public_path = fs::canonicalize(&a[2])?;
    let bytes = fs::read(&public_path)?;
    let public: Value = serde_json::from_slice(&bytes)?;
    let fixture = read(Path::new(&a[3]))?;
    if fixture["public"] != public || a[4].len() != 40 {
        return Err("fixture/source registration".into());
    }
    let answer = fixture["expected_scalar"].as_u64().ok_or("owner answer")?;
    let mut log = BufWriter::new(
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(dir.join("runs.jsonl"))?,
    );
    let mut plan = Vec::new();
    let arms = ["baseline", "unit", "rho"];
    for arm in arms {
        plan.push(("warmup", 0, arm));
    }
    for round in 0..2 {
        for arm in arms {
            plan.push(("control", round, arm));
        }
    }
    for round in 0..5 {
        let mut order = if round % 2 == 0 {
            vec!["baseline", "unit"]
        } else {
            vec!["unit", "baseline"]
        };
        order.insert(round % 3, "rho");
        for arm in order {
            plan.push(("measurement", round, arm));
        }
    }
    put(
        &mut log,
        &json!({"type":"plan","schema":"nagao.e2e.run/v1","source_revision":a[4],"input_sha256":qualification::digest(&bytes),"planned":plan,"complete_worker_process_wall":true,"complete_operation_counts":false}),
    )?;
    let mut signatures = BTreeMap::new();
    let mut first_ic: BTreeMap<&str, Value> = BTreeMap::new();
    let mut completed = 0;
    let mut invalid = 0;
    let started = Instant::now();
    for (phase, round, arm) in plan {
        let key = format!("{phase}-{round}-{arm}");
        let raw = dir.join(format!("{key}.raw.json"));
        let stdout = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(dir.join(format!("{key}.stdout")))?;
        let stderr = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(dir.join(format!("{key}.stderr")))?;
        let isolation = dir.join(format!("{key}.isolation.jsonl"));
        let method = match arm {
            "baseline" => "extension-remainders",
            "unit" => "extension-unit",
            _ => "rho",
        };
        put(
            &mut log,
            &json!({"type":"start","phase":phase,"round":round,"arm":arm,"raw":raw,"elapsed_seconds":started.elapsed().as_secs_f64()}),
        )?;
        let status = Command::new("python3")
            .arg(&a[5])
            .args(["run", "--cpus", "0", "--label", &key, "--out"])
            .arg(&isolation)
            .args(["--", "timeout", "300"])
            .arg(&worker)
            .args(["--seed", "401", "--method", method, "--fixture"])
            .arg(&public_path)
            .args([
                "--source-revision",
                &a[4],
                "--small-percent",
                "100",
                "--b-count",
                "1018081",
                "--max-descent-residuals",
                "8192",
                "--max-accepted-descents",
                "32",
                "--rho-batch",
                "16",
                "--rho-plain",
                "--out",
            ])
            .arg(&raw)
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .status();
        let mut errors = Vec::new();
        let code = status.as_ref().ok().and_then(|s| s.code());
        if code != Some(0) {
            errors.push(format!("worker/wrapper exit: {code:?}"));
        }
        let iso = fs::read_to_string(&isolation).ok().and_then(|s| {
            s.lines()
                .last()
                .and_then(|x| serde_json::from_str::<Value>(x).ok())
        });
        let wall = iso.as_ref().and_then(|v| v["run"]["wall_seconds"].as_f64());
        let admitted = iso
            .as_ref()
            .is_some_and(|v| v["run"]["contended"] == false && v["run"]["exit_status"] == 0);
        if !admitted {
            errors.push("isolation refusal/contention/failure".into());
        }
        let mut audit = Value::Null;
        let mut result = Value::Null;
        if let Ok(w) = read(&raw) {
            if w["source_revision"] != a[4] {
                errors.push("worker source binding".into());
            }
            match qualification::qualify(&w, &public, answer, &qualification::digest(&bytes)) {
                Ok(q) => audit = q,
                Err(e) => errors.push(format!("qualification: {e}")),
            };
            let signature = qualification::signature(&w);
            if let Some(old) = signatures.get(arm) {
                if old != &signature {
                    errors.push("within-arm repeat differs".into());
                }
            } else {
                signatures.insert(arm, signature);
            }
            if arm != "rho" {
                first_ic.entry(arm).or_insert_with(|| w.clone());
                if first_ic.len() == 2 {
                    if let Err(e) = qualification::pair(&first_ic["baseline"], &first_ic["unit"]) {
                        errors.push(e);
                    }
                }
            }
            result = json!({"scalar":w["actual_scalar"],"verified":w["verified"],"residuals":w["work"]["descent_residuals"],"collection":w["work"]["collection"]["counts"],"phases":w["work_native"]["phases"]});
        } else {
            errors.push("missing/invalid raw worker report".into());
        }
        if errors.is_empty() {
            completed += 1;
        } else {
            invalid += 1;
        }
        let row = json!({"type":"result","phase":phase,"round":round,"arm":arm,"exit_code":code,"wall_seconds":wall,"isolation_admitted":admitted,"raw":raw,"raw_sha256":fs::read(&raw).ok().map(|b|qualification::digest(&b)),"valid":errors.is_empty(),"errors":errors,"result":result,"independent_audit":audit});
        put(&mut log, &row)?;
        println!(
            "{}",
            json!({"phase":phase,"round":round,"arm":arm,"wall_seconds":wall,"valid":row["valid"],"errors":row["errors"]})
        );
        std::io::stdout().flush()?;
    }
    put(
        &mut log,
        &json!({"type":"complete","planned":24,"valid":completed,"invalid":invalid,"outer_seconds":started.elapsed().as_secs_f64(),"automatic_retries":0}),
    )?;
    if invalid != 0 {
        return Err("one or more planned runs failed; all retained".into());
    }
    Ok(())
}
