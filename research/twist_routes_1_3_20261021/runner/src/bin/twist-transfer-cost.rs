//! Given-map cold transfer cost with identical generic BSGS source control.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn bytes(p: &Path) -> u64 {
    fs::read_dir(p)
        .unwrap()
        .map(|e| {
            let p = e.unwrap().path();
            if p.is_dir() {
                bytes(&p)
            } else {
                fs::metadata(p).unwrap().len()
            }
        })
        .sum()
}
fn ir(s: &str) -> Option<u64> {
    if s.lines()
        .find_map(|s| s.strip_prefix("events:"))
        .map(str::trim)
        != Some("Ir")
    {
        return None;
    }
    s.lines()
        .rev()
        .find_map(|s| s.strip_prefix("totals:"))
        .or_else(|| s.lines().rev().find_map(|s| s.strip_prefix("summary:")))
        .and_then(|s| s.trim().parse().ok())
}
fn plan(fixtures: &Path, prior_file: &Path) -> Result<Value> {
    let fixtures = read(fixtures)?;
    let prior = read(prior_file)?;
    let mut cells = vec![];
    for row in prior["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["fixture"].is_string())
    {
        let fixture = fixtures["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == row["fixture"])
            .ok_or("fixture")?;
        let cf = PathBuf::from(row["constructor_input"].as_str().unwrap());
        let constructor = read(&cf)?;
        for control in row["toy_destination_BSGS_controls"].as_array().unwrap() {
            for arm in ["source-aa", "source-bsgs", "transfer-aa", "transfer-bsgs"] {
                cells.push(json!({"index":cells.len(),"fixture":row["fixture"],"label":control["label"],"arm":arm,"expected_scalar":control["recovered_scalar"],"constructor_path":cf,"constructor_sha256":sha(&cf)?,"task":{"fixture":fixture,"target":control["public_source_target"],"constructor":constructor["rows"][0],"label":control["label"],"arm":if arm.starts_with("source"){"source-bsgs"}else{"transfer-bsgs"}}}));
            }
        }
    }
    if cells.len() != 96 {
        return Err("expected 24 targets and 96 processes".into());
    }
    Ok(
        json!({"schema":"twist.transfer-cost-plan/v1","prior_source":prior_file,"prior_sha256":sha(prior_file)?,"requested":96,"distinct_targets":24,"cells":cells,"new_targets_opened":false}),
    )
}
fn run(plan_file: &Path, worker: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh output required".into());
    }
    let plan = read(plan_file)?;
    if sha(Path::new(plan["prior_source"].as_str().unwrap()))? != plan["prior_sha256"] {
        return Err("old transfer evidence changed".into());
    }
    fs::create_dir_all(out)?;
    let start = Instant::now();
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\ngood=0;bad=0;\n");
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    let vg = format!("{vgroot}/bin/valgrind");
    let lib = format!("{vgroot}/libexec/valgrind");
    for cell in plan["cells"].as_array().unwrap() {
        let mut row = cell.clone();
        if start.elapsed().as_secs() >= 2400 || bytes(out) >= 1_073_741_824 {
            row["status"] = json!("not-started-global-cap");
            rows.push(row);
            continue;
        }
        if sha(Path::new(cell["constructor_path"].as_str().unwrap()))? != cell["constructor_sha256"]
        {
            return Err("public constructor changed".into());
        }
        let dir = out.join(format!("cell-{:03}", cell["index"].as_u64().unwrap()));
        fs::create_dir(&dir)?;
        let input = dir.join("task.json");
        write(&input, &cell["task"])?;
        let output = dir.join("worker.json");
        let trace = dir.join("worker.callgrind");
        let args = vec![
            "followup".into(),
            "transfer-cost-worker".into(),
            input.display().to_string(),
            output.display().to_string(),
        ];
        let status = Command::new("/bin/sh")
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; ulimit -f 65536; exec \"$@\"",
                "transfer-cost",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                "60s",
            ])
            .arg(&vg)
            .args([
                "--tool=callgrind",
                "--cache-sim=no",
                "--branch-sim=no",
                "--error-exitcode=99",
                &format!("--callgrind-out-file={}", trace.display()),
            ])
            .arg(worker)
            .args(&args)
            .env("VALGRIND_LIB", &lib)
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .stdout(Stdio::from(fs::File::create(dir.join("worker.stdout"))?))
            .stderr(Stdio::from(fs::File::create(dir.join("worker.stderr"))?))
            .status()?;
        let result = read(&output).ok();
        let count = fs::read_to_string(&trace).ok().and_then(|s| ir(&s));
        let accepted = status.success()
            && count.is_some()
            && result.as_ref().is_some_and(|r| {
                r["source_scalar_verified"] == true
                    && r["source_scalar"] == cell["expected_scalar"]
                    && r["target"] == cell["task"]["target"]
            });
        if accepted {
            let r = result.as_ref().unwrap();
            let f = &cell["task"]["fixture"];
            let p = f["p"].as_u64().unwrap();
            let g: Vec<u64> = serde_json::from_value(f["generator"].clone())?;
            let t: Vec<Vec<u64>> = serde_json::from_value(cell["task"]["target"].clone())?;
            let scalar = r["source_scalar"].as_u64().unwrap();
            gp.push_str(&format!("E=ellinit([Mod({},{}),Mod({}, {})]);G=[Mod({},{}),Mod({}, {})];T=[Mod({},{}),Mod({}, {})];if(ellmul(E,G,{})!=T,error(\"source recovery\"));if(ellmul(E,G,{})==T,error(\"wrong scalar accepted\"));good++;bad++;\n",f["a"],p,f["b"],p,g[0],p,g[1],p,t[0][0],p,t[1][0],p,scalar,scalar+1));
        }
        row["status"] = json!(if accepted {
            "known-answer-matched-awaiting-independent-replay"
        } else {
            "incomplete-or-mismatch"
        });
        row["accepted_by_parent"] = json!(accepted);
        row["guest_instructions_Ir"] = json!(count);
        row["exit_code"] = json!(status.code());
        row["signal"] = json!(status.signal());
        row["task_sha256"] = json!(sha(&input)?);
        row["output_sha256"] = json!(if output.exists() {
            Some(sha(&output)?)
        } else {
            None
        });
        row["trace_sha256"] = json!(if trace.exists() {
            Some(sha(&trace)?)
        } else {
            None
        });
        row["stdout_sha256"] = json!(sha(dir.join("worker.stdout"))?);
        row["stderr_sha256"] = json!(sha(dir.join("worker.stderr"))?);
        row["argv"] = json!(args);
        println!(
            "{}",
            json!({"cell":cell["index"].as_u64().unwrap()+1,"fixture":cell["fixture"],"label":cell["label"],"arm":cell["arm"],"completed":accepted})
        );
        rows.push(row);
        write(out.join("RECORDS.json"), &json!(rows))?;
    }
    gp.push_str("if(good==0||bad==0,error(\"missing controls\"));print(\"ALL_TRANSFER_COST_REPLAY_OK \",good,\" \",bad);quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    write(out.join("RECORDS.json"), &json!(rows))?;
    write(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.transfer-cost/v1","requested":96,"recorded":rows.len(),"completed_parent_matches":rows.iter().filter(|r|r["accepted_by_parent"]==true).count(),"worker_sha256":sha(worker)?,"plan_sha256":sha(plan_file)?,"unit":"Callgrind simulated guest instructions Ir","given_map_interval_only":true,"map_discovery_and_constructor_search_priced":false,"GP_replay_executed":false,"full_IC_cost_or_fastest_claim":false,"wall_time_admitted":false}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("plan") if a.len() == 5 => {
            if Path::new(&a[4]).exists() {
                return Err("fresh plan required".into());
            }
            write(&a[4], &plan(Path::new(&a[2]), Path::new(&a[3]))?)
        }
        Some("run") if a.len() == 5 => run(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4])),
        _ => {
            Err("usage: plan FIXTURES PRIOR_TRANSFER FRESH_PLAN | run PLAN WORKER FRESH_OUT".into())
        }
    }
}
