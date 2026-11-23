//! Frozen training-grid scheduling and raw Callgrind receipts; no kernel changes.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
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
fn leaf(p: &Path, id: &str, suffix: &str) -> PathBuf {
    p.join("cells").join(format!("{id}.{suffix}"))
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
        .find_map(|l| l.strip_prefix("events:"))
        .map(str::trim)
        != Some("Ir")
    {
        return None;
    }
    s.lines()
        .rev()
        .find_map(|l| l.strip_prefix("totals:"))
        .or_else(|| s.lines().rev().find_map(|l| l.strip_prefix("summary:")))
        .and_then(|v| v.trim().parse().ok())
}
const ARMS: [&str; 8] = [
    "direct-aa",
    "direct-chain",
    "ordinary-prefix-order",
    "frobenius-block",
    "hybrid-eight",
    "first-pair-invariant",
    "factored-control",
    "bits-control",
];
fn make_plan(inputs: &Path, panel: &Path) -> Result<Value> {
    let workloads = read(inputs.join("WORKLOADS.json"))?;
    let jobs = read(inputs.join("JOBS.json"))?;
    let mut groups = BTreeMap::<(String, String), Vec<String>>::new();
    for j in jobs.as_array().ok_or("jobs array")? {
        groups
            .entry((
                j["workload"].as_str().ok_or("workload")?.into(),
                j["arm"].as_str().ok_or("arm")?.into(),
            ))
            .or_default()
            .push(j["id"].as_str().ok_or("job id")?.into());
    }
    let mut cells = vec![];
    for w in workloads
        .as_array()
        .ok_or("workload array")?
        .iter()
        .filter(|w| w["arity"].as_u64().is_some_and(|m| m <= 3))
    {
        for arm in ARMS {
            let source = if arm == "direct-aa" {
                "direct-chain"
            } else {
                arm
            };
            let ids = groups
                .get(&(w["workload"].as_str().unwrap().into(), source.into()))
                .ok_or("missing candidate")?;
            cells.push(json!({"index":cells.len(),"workload":w["workload"],"fixture":w["fixture"],"p":w["p"],"B":w["B"],"arity":w["arity"],"arm":arm,"source_arm":source,"jobs":ids}));
        }
    }
    if cells.len() != 480 {
        return Err("expected 480 frozen cells".into());
    }
    Ok(
        json!({"schema":"twist.block-sweep-plan/v1","inputs":inputs,"panel":panel,"catalog_sha256":sha(inputs.join("JOBS.json"))?,"workloads_sha256":sha(inputs.join("WORKLOADS.json"))?,"parent_panel_sha256":sha(panel.join("PANEL.json"))?,"requested_cells":cells.len(),"cells":cells,"target_label":0,"new_targets_generated":false}),
    )
}
fn run(plan_file: &Path, worker: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh sweep output required".into());
    }
    let plan = read(plan_file)?;
    let inputs = Path::new(plan["inputs"].as_str().ok_or("input path")?);
    let panel = Path::new(plan["panel"].as_str().ok_or("panel path")?);
    if plan["catalog_sha256"] != sha(inputs.join("JOBS.json"))?
        || plan["parent_panel_sha256"] != sha(panel.join("PANEL.json"))?
    {
        return Err("frozen parent changed".into());
    }
    let catalog = read(inputs.join("JOBS.json"))?;
    let jobs = catalog
        .as_array()
        .unwrap()
        .iter()
        .map(|j| (j["id"].as_str().unwrap().to_string(), j.clone()))
        .collect::<BTreeMap<_, _>>();
    let parent = read(panel.join("PANEL.json"))?;
    fs::create_dir_all(out)?;
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    let vg = format!("{vgroot}/bin/valgrind");
    let vg_lib = format!("{vgroot}/libexec/valgrind");
    let started = Instant::now();
    let mut records = vec![];
    for cell in plan["cells"].as_array().ok_or("cells")? {
        let index = cell["index"].as_u64().unwrap();
        if started.elapsed().as_secs() >= 7200 || bytes(out) >= 1_073_741_824 {
            let mut row = cell.clone();
            row["status"] = json!("not-started-global-time-or-output-cap");
            records.push(row);
            continue;
        }
        let payload = cell["jobs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| jobs[id.as_str().unwrap()].clone())
            .collect::<Vec<_>>();
        let prior = parent
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["workload"] == cell["workload"] && r["arm"] == cell["source_arm"])
            .ok_or("missing source candidate")?;
        let dir = out.join(format!("cell-{index:03}"));
        fs::create_dir(&dir)?;
        let capsule = dir.join("capsule.json");
        write(&capsule, &json!(payload))?;
        let output = dir.join("worker.json");
        let trace = dir.join("worker.callgrind");
        let argv = vec![
            "followup".into(),
            "block-batch-worker".into(),
            capsule.display().to_string(),
            output.display().to_string(),
            "10000".into(),
        ];
        let status = Command::new("/bin/sh")
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; ulimit -f 65536; exec \"$@\"",
                "sweep-worker",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                "12s",
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
            .args(&argv)
            .env("VALGRIND_LIB", &vg_lib)
            .env("F4_FP_GM", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .stdout(Stdio::from(fs::File::create(dir.join("worker.stdout"))?))
            .stderr(Stdio::from(fs::File::create(dir.join("worker.stderr"))?))
            .status()?;
        let new = read(&output).ok();
        let count = fs::read_to_string(&trace).ok().and_then(|s| ir(&s));
        let mut finished = status.success()
            && new
                .as_ref()
                .is_some_and(|n| n.as_array().is_some_and(|a| a.len() == payload.len()));
        let mut matched = finished;
        let mut branches = vec![];
        for (i, j) in payload.iter().enumerate() {
            let id = j["id"].as_str().unwrap();
            let old_input = leaf(panel, id, "input.json");
            let input_equal = read(&old_input).is_ok_and(|old| old == *j);
            let old_branch = prior["branches"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["job"] == id);
            let old_result = leaf(panel, id, "native.json");
            let old = read(&old_result).ok();
            let n = new
                .as_ref()
                .and_then(|n| n.as_array())
                .and_then(|a| a.get(i));
            let branch_finished = n.is_some_and(|n| {
                n["job"] == j["id"] && n["timed_out"] == false && n["pairs_above_bound"] == 0
            });
            let certified = old_branch.is_some_and(|b| {
                b["certificate"]["accepted"] == true
                    && b["input_sha256"] == sha(&old_input).unwrap_or_default()
                    && b["result_sha256"] == sha(&old_result).unwrap_or_default()
            });
            let equal = n.zip(old.as_ref()).is_some_and(|(n, o)| {
                n["basis"] == o["basis"]
                    && n["field_multiplications_row_reduction"]
                        == o["field_multiplications_row_reduction"]
            });
            finished &= branch_finished;
            matched &= input_equal && certified && equal && branch_finished;
            branches.push(json!({"job":id,"input_equal":input_equal,"old_independent_certificate_accepted":certified,"basis_and_row_counter_equal":equal,"branch_finished":branch_finished,"prior_input_sha256":if old_input.exists(){Some(sha(&old_input)?)}else{None},"prior_result_sha256":if old_result.exists(){Some(sha(&old_result)?)}else{None}}));
        }
        let accepted = matched && count.is_some();
        let mut row = cell.clone();
        row["status"] = json!(if accepted {
            "completed-and-certified-output-matched"
        } else if finished {
            "finished-without-matched-completeness-certificate"
        } else {
            "incomplete"
        });
        row["worker_finished"] = json!(finished);
        row["accepted"] = json!(accepted);
        row["exit_code"] = json!(status.code());
        row["signal"] = json!(status.signal());
        row["guest_instructions_Ir"] = json!(count);
        row["branches"] = json!(branches);
        row["capsule_sha256"] = json!(sha(&capsule)?);
        row["trace_sha256"] = json!(if trace.exists() {
            Some(sha(&trace)?)
        } else {
            None
        });
        row["output_sha256"] = json!(if output.exists() {
            Some(sha(&output)?)
        } else {
            None
        });
        row["stdout_sha256"] = json!(sha(dir.join("worker.stdout"))?);
        row["stderr_sha256"] = json!(sha(dir.join("worker.stderr"))?);
        row["argv"] = json!(argv);
        println!(
            "{}",
            json!({"cell":index+1,"workload":cell["workload"],"arm":cell["arm"],"status":row["status"]})
        );
        records.push(row);
        write(out.join("RECORDS.json"), &json!(records))?;
    }
    write(out.join("RECORDS.json"), &json!(records))?;
    write(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.block-sweep/v1","requested":480,"recorded":records.len(),"accepted":records.iter().filter(|r|r["accepted"]==true).count(),"unit":"Callgrind simulated guest instructions Ir","plan_sha256":sha(plan_file)?,"worker_sha256":sha(worker)?,"valgrind_sha256":sha(&vg)?,"callgrind_tool_sha256":sha(Path::new(&vg_lib).join("callgrind-amd64-linux"))?,"raw_bytes":bytes(out),"construction_lifting_and_complete_IC_missing":true,"wall_time_admitted":false,"speedup_established":false}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("plan") if a.len() == 5 => {
            if Path::new(&a[4]).exists() {
                return Err("fresh plan required".into());
            }
            write(&a[4], &make_plan(Path::new(&a[2]), Path::new(&a[3]))?)
        }
        Some("run") if a.len() == 5 => run(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4])),
        _ => Err(
            "usage: sweep plan INPUTS PANEL FRESH_PLAN | run PLAN PINNED_WORKER FRESH_OUT".into(),
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trace_requires_one_instruction_event_and_valid_total() {
        assert_eq!(ir("events: Ir\nsummary: 8\ntotals: 12\n"), Some(12));
        assert_eq!(ir("events: Ir Dr\ntotals: 12 5\n"), None);
        assert_eq!(ir("events: Ir\ntotals: malformed\n"), None);
    }
}
