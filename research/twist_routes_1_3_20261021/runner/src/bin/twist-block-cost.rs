//! Small whole-worker Callgrind probe; separate from complete IC costing.
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
fn load(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn path(root: &Path, id: &str, suffix: &str) -> PathBuf {
    root.join("cells").join(format!("{id}.{suffix}"))
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    if a.len() != 5 && !(a.len() == 6 && a[5] == "--batch") {
        return Err("usage: cost MATCHED_INPUTS PANEL WORKER FRESH_OUTPUT [--batch]".into());
    }
    let inputs = Path::new(&a[1]);
    let panel = Path::new(&a[2]);
    let worker = Path::new(&a[3]);
    let out = Path::new(&a[4]);
    if a.len() == 6 {
        return batch_probe(inputs, panel, worker, out);
    }
    if out.exists() {
        return Err("fresh cost output required".into());
    }
    fs::create_dir_all(out)?;
    let all = load(inputs.join("JOBS.json"))?;
    let source_panel = load(panel.join("PANEL.json"))?;
    let fixed = "icv1-fp6-tm7-fc20da94-b8-m2-s0";
    let mut jobs = all
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["workload"] == fixed)
        .cloned()
        .collect::<Vec<_>>();
    jobs.sort_by_key(|j| {
        (
            j["arm"].as_str().unwrap().to_string(),
            j["branch"].as_u64().unwrap(),
        )
    });
    let direct = jobs
        .iter()
        .find(|j| j["arm"] == "direct-chain")
        .unwrap()
        .clone();
    let mut schedule = vec![(
        "direct-aa".to_string(),
        "native".to_string(),
        direct.clone(),
    )];
    for j in &jobs {
        schedule.push((
            j["arm"].as_str().unwrap().into(),
            "native".into(),
            j.clone(),
        ));
        schedule.push((
            j["arm"].as_str().unwrap().into(),
            "msolve".into(),
            j.clone(),
        ));
    }
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    let vg = PathBuf::from(format!("{vgroot}/bin/valgrind"));
    let lib = format!("{vgroot}/libexec/valgrind");
    let cas = "/workspace/shared/twist-tools-20261023/cas/root/usr";
    let started = Instant::now();
    let mut records = vec![];
    for (index, (arm, engine, job)) in schedule.iter().enumerate() {
        let id = job["id"].as_str().unwrap();
        let prior = source_panel
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["workload"] == fixed && r["arm"] == job["arm"])
            .unwrap();
        let prior_branch = prior["branches"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["job"] == id)
            .ok_or("missing prior branch")?;
        assert_eq!(
            prior_branch["input_sha256"],
            sha(path(panel, id, "input.json"))?
        );
        if started.elapsed().as_secs() >= 900
            || prior["all_branches_independently_certified"] != true
        {
            records.push(json!({"arm":arm,"engine":engine,"job":id,"status":"not-started-budget-or-prior-certificate"}));
            continue;
        }
        let stem = out.join(format!("process-{index:03}"));
        let trace = stem.with_extension("callgrind");
        let result = stem.with_extension(if engine == "native" {
            "native.json"
        } else {
            "msolve.gb"
        });
        let (program, args) = if engine == "native" {
            (
                worker.to_path_buf(),
                vec![
                    "followup".into(),
                    "block-worker".into(),
                    path(panel, id, "input.json").display().to_string(),
                    result.display().to_string(),
                    "250000".into(),
                ],
            )
        } else {
            (
                PathBuf::from(format!("{cas}/bin/msolve")),
                vec![
                    "-f".into(),
                    path(panel, id, "certificate.ms").display().to_string(),
                    "-o".into(),
                    result.display().to_string(),
                    "-g".into(),
                    "2".into(),
                    "-q".into(),
                    "0".into(),
                    "-t".into(),
                    "1".into(),
                    "-v".into(),
                    "0".into(),
                ],
            )
        };
        let mut command = Command::new("/bin/sh");
        command
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; exec \"$@\"",
                "block-cost",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                "300s",
            ])
            .arg(&vg)
            .args([
                "--tool=callgrind",
                "--cache-sim=no",
                "--branch-sim=no",
                "--error-exitcode=99",
                &format!("--callgrind-out-file={}", trace.display()),
            ])
            .arg(&program)
            .args(&args)
            .env("VALGRIND_LIB", &lib)
            .env("F4_FP_GM", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .env(
                "LD_LIBRARY_PATH",
                format!("{cas}/lib/x86_64-linux-gnu:{cas}/lib/x86_64-linux-gnu/singular"),
            )
            .stdout(Stdio::from(fs::File::create(
                stem.with_extension("stdout"),
            )?))
            .stderr(Stdio::from(fs::File::create(
                stem.with_extension("stderr"),
            )?));
        let status = command.status()?;
        let text = fs::read_to_string(&trace).unwrap_or_default();
        let event = text.lines().find_map(|s| s.strip_prefix("events: "));
        let count = if event == Some("Ir") {
            text.lines()
                .rev()
                .find_map(|s| s.strip_prefix("totals: "))
                .or_else(|| text.lines().rev().find_map(|s| s.strip_prefix("summary: ")))
                .and_then(|s| s.trim().parse::<u64>().ok())
        } else {
            None
        };
        let verified = if status.success() && result.exists() {
            if engine == "native" {
                let n = load(&result)?;
                let old = load(path(panel, id, "native.json"))?;
                n["basis"] == old["basis"]
                    && n["field_multiplications_row_reduction"]
                        == old["field_multiplications_row_reduction"]
                    && n["timed_out"] == false
                    && n["pairs_above_bound"] == 0
            } else {
                let old = path(panel, id, "certificate.msolve.gb");
                prior_branch["certificate"]["msolve_basis_independently_certified"] == true
                    && old.exists()
                    && sha(&result)? == sha(old)?
            }
        } else {
            false
        };
        let accepted = status.success() && verified && count.is_some();
        records.push(json!({"arm":arm,"engine":engine,"job":id,"branch":job["branch"],"exit_code":status.code(),"signal":status.signal(),"status":if accepted{"completed-and-output-matched"}else{"incomplete"},"guest_instructions_Ir":count,"basis_and_native_counters_match_prior_certificate":verified,"trace_sha256":if trace.exists(){Some(sha(&trace)?)}else{None},"output_sha256":if result.exists(){Some(sha(&result)?)}else{None},"stdout_sha256":sha(stem.with_extension("stdout"))?,"stderr_sha256":sha(stem.with_extension("stderr"))?,"program":program,"argv":args}));
        fs::write(
            out.join("RECORDS.json"),
            serde_json::to_string_pretty(&records)? + "\n",
        )?;
        println!(
            "{}",
            json!({"process":index+1,"arm":arm,"engine":engine,"completed":accepted})
        );
    }
    fs::write(
        out.join("RECORDS.json"),
        serde_json::to_string_pretty(&records)? + "\n",
    )?;
    let mut sums = BTreeMap::<(String, String), (u64, u64, u64)>::new();
    for r in &records {
        let e = sums
            .entry((
                r["arm"].as_str().unwrap().into(),
                r["engine"].as_str().unwrap().into(),
            ))
            .or_default();
        e.0 += 1;
        if r["status"] == "completed-and-output-matched" {
            e.1 += 1;
            e.2 += r["guest_instructions_Ir"].as_u64().unwrap();
        }
    }
    let by_arm=sums.iter().map(|((arm,engine),(requested,complete,instructions))|json!({"arm":arm,"engine":engine,"requested_branches":requested,"completed_branches":complete,"all_branches_completed":requested==complete,"guest_instructions_Ir":if requested==complete{Some(*instructions)}else{None}})).collect::<Vec<_>>();
    let aa = sums
        .get(&("direct-aa".into(), "native".into()))
        .filter(|x| x.0 == x.1)
        .map(|x| x.2);
    let reference = sums
        .get(&("direct-chain".into(), "native".into()))
        .filter(|x| x.0 == x.1)
        .map(|x| x.2);
    let spread = aa
        .zip(reference)
        .map(|(a, b)| a.abs_diff(b) as f64 / ((a as f64 + b as f64) / 2.0));
    fs::write(
        out.join("COST.json"),
        serde_json::to_string_pretty(
            &json!({"schema":"twist.block-worker-instruction-probe/v1","fixed_workload":fixed,"unit":"Callgrind simulated guest instructions Ir","scope":"serialized ideal to serialized basis in each worker process, summed across all branches","by_arm":by_arm,"AA_relative_spread":spread,"AA_within_preregistered_5_percent":spread.is_some_and(|s|s<=0.05),"worker_binary_sha256":sha(worker)?,"valgrind_launcher_sha256":sha(&vg)?,"callgrind_tool_sha256":sha(Path::new(&lib).join("callgrind-amd64-linux"))?,"source_panel_sha256":sha(panel.join("PANEL.json"))?,"construction_parent_orchestration_certification_lifting_and_DLP_costs_missing":true,"wall_time_admitted":false,"complete_cost_or_speedup_established":false,"new_targets_generated":false}),
        )? + "\n",
    )?;
    Ok(())
}

fn batch_probe(inputs: &Path, panel: &Path, worker: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh batch cost output required".into());
    }
    fs::create_dir_all(out)?;
    let all = load(inputs.join("JOBS.json"))?;
    let parent = load(panel.join("PANEL.json"))?;
    let fixed = "icv1-fp6-tm7-fc20da94-b8-m2-s0";
    let mut groups = BTreeMap::<String, Vec<Value>>::new();
    for job in all
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["workload"] == fixed)
    {
        let id = job["id"].as_str().unwrap();
        assert_eq!(*job, load(path(panel, id, "input.json"))?);
        groups
            .entry(job["arm"].as_str().unwrap().into())
            .or_default()
            .push(job.clone());
    }
    for jobs in groups.values_mut() {
        jobs.sort_by_key(|j| j["branch"].as_u64().unwrap());
    }
    let mut schedule = vec![("direct-aa".to_string(), groups["direct-chain"].clone())];
    schedule.extend(groups.iter().map(|(a, j)| (a.clone(), j.clone())));
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    let vg = PathBuf::from(format!("{vgroot}/bin/valgrind"));
    let lib = format!("{vgroot}/libexec/valgrind");
    let cas = "/workspace/shared/twist-tools-20261023/cas/root/usr";
    let started = Instant::now();
    let mut records = vec![];
    let mut by_arm = vec![];
    for (index, (arm, jobs)) in schedule.iter().enumerate() {
        let source_arm = if arm == "direct-aa" {
            "direct-chain"
        } else {
            arm.as_str()
        };
        let prior = parent
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["workload"] == fixed && r["arm"] == source_arm)
            .unwrap();
        if started.elapsed().as_secs() >= 900
            || prior["all_branches_independently_certified"] != true
        {
            records.push(json!({"arm":arm,"engine":"native","branch_count":jobs.len(),"status":"not-started-budget-or-prior-certificate"}));
            by_arm.push(json!({"arm":arm,"engine":"native","requested_branches":jobs.len(),"completed_branches":0,"all_branches_completed":false,"guest_instructions_Ir":null,"processes":0}));
            continue;
        }
        let capsule = out.join(format!("capsule-{source_arm}.json"));
        if !capsule.exists() {
            fs::write(&capsule, serde_json::to_string_pretty(jobs)? + "\n")?;
        }
        let stem = out.join(format!("process-{index:03}"));
        let trace = stem.with_extension("callgrind");
        let result = stem.with_extension("native.json");
        let args = vec![
            "followup".into(),
            "block-batch-worker".into(),
            capsule.display().to_string(),
            result.display().to_string(),
            "250000".into(),
        ];
        let status = Command::new("/bin/sh")
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; exec \"$@\"",
                "block-batch-cost",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                "300s",
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
            .env("F4_FP_GM", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .env(
                "LD_LIBRARY_PATH",
                format!("{cas}/lib/x86_64-linux-gnu:{cas}/lib/x86_64-linux-gnu/singular"),
            )
            .stdout(Stdio::from(fs::File::create(
                stem.with_extension("stdout"),
            )?))
            .stderr(Stdio::from(fs::File::create(
                stem.with_extension("stderr"),
            )?))
            .status()?;
        let text = fs::read_to_string(&trace).unwrap_or_default();
        let count = if text
            .lines()
            .find_map(|s| s.strip_prefix("events:"))
            .map(str::trim)
            == Some("Ir")
        {
            text.lines()
                .rev()
                .find_map(|s| s.strip_prefix("totals:"))
                .or_else(|| text.lines().rev().find_map(|s| s.strip_prefix("summary:")))
                .and_then(|s| s.trim().parse::<u64>().ok())
        } else {
            None
        };
        let values = load(&result).ok();
        let mut verified = status.success()
            && values
                .as_ref()
                .is_some_and(|v| v.as_array().is_some_and(|v| v.len() == jobs.len()));
        if verified {
            for (new, job) in values
                .as_ref()
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .zip(jobs)
            {
                let old = load(path(panel, job["id"].as_str().unwrap(), "native.json"))?;
                verified &= new["job"] == job["id"]
                    && new["basis"] == old["basis"]
                    && new["field_multiplications_row_reduction"]
                        == old["field_multiplications_row_reduction"]
                    && new["timed_out"] == false
                    && new["pairs_above_bound"] == 0;
            }
        }
        let complete = verified && count.is_some();
        records.push(json!({"arm":arm,"engine":"native","branch_count":jobs.len(),"jobs":jobs.iter().map(|j|&j["id"]).collect::<Vec<_>>(),"status":if complete{"completed-and-output-matched"}else{"incomplete"},"exit_code":status.code(),"signal":status.signal(),"guest_instructions_Ir":count,"basis_and_native_counters_match_prior_certificate":verified,"trace_sha256":if trace.exists(){Some(sha(trace)?)}else{None},"output_sha256":if result.exists(){Some(sha(&result)?)}else{None},"capsule_sha256":sha(capsule)?,"stdout_sha256":sha(stem.with_extension("stdout"))?,"stderr_sha256":sha(stem.with_extension("stderr"))?,"argv":args}));
        by_arm.push(json!({"arm":arm,"engine":"native","requested_branches":jobs.len(),"completed_branches":if complete{jobs.len()}else{0},"all_branches_completed":complete,"guest_instructions_Ir":if complete{count}else{None},"processes":1}));
        fs::write(
            out.join("RECORDS.json"),
            serde_json::to_string_pretty(&records)? + "\n",
        )?;
        println!(
            "{}",
            json!({"arm":arm,"branches":jobs.len(),"completed":complete})
        );
    }
    fs::write(
        out.join("RECORDS.json"),
        serde_json::to_string_pretty(&records)? + "\n",
    )?;
    let aa =
        by_arm.iter().find(|a| a["arm"] == "direct-aa").unwrap()["guest_instructions_Ir"].as_u64();
    let direct = by_arm.iter().find(|a| a["arm"] == "direct-chain").unwrap()
        ["guest_instructions_Ir"]
        .as_u64();
    let spread = aa
        .zip(direct)
        .map(|(a, b)| a.abs_diff(b) as f64 / ((a as f64 + b as f64) / 2.0));
    fs::write(
        out.join("COST.json"),
        serde_json::to_string_pretty(
            &json!({"schema":"twist.block-batched-instruction-probe/v1","fixed_workload":fixed,"unit":"Callgrind simulated guest instructions Ir","scope":"one worker process per candidate, serialized batch of ideals to batch of bases","by_arm":by_arm,"AA_relative_spread":spread,"AA_within_preregistered_5_percent":spread.is_some_and(|s|s<=0.05),"worker_binary_sha256":sha(worker)?,"valgrind_launcher_sha256":sha(vg)?,"source_panel_sha256":sha(panel.join("PANEL.json"))?,"construction_parent_orchestration_certification_lifting_and_DLP_costs_missing":true,"wall_time_admitted":false,"complete_cost_or_speedup_established":false,"new_targets_generated":false}),
        )? + "\n",
    )?;
    Ok(())
}
