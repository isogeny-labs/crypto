use super::model::{digest, read, write_new, Candidate, Plan, Request, Result};
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Component, Path},
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub struct Log {
    file: File,
    sequence: u64,
    json: bool,
}
fn lock(dir: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(dir.join("run.lock"))
        .map_err(|e| e.to_string())?;
    file.try_lock()
        .map_err(|e| format!("another process holds the sweep run lock: {e}"))?;
    Ok(file)
}
impl Log {
    fn open(dir: &Path, json_mode: bool) -> Result<Self> {
        let path = dir.join("events.jsonl");
        let mut seq = 0;
        if path.exists() {
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            if !bytes.is_empty() && !bytes.ends_with(b"\n") {
                return Err("event log has an interrupted final line; preserving bytes for explicit recovery".into());
            }
        }
        if path.exists() {
            for line in BufReader::new(File::open(&path).map_err(|e| e.to_string())?).lines() {
                let line = line.map_err(|e| e.to_string())?;
                let v: Value = serde_json::from_str(&line)
                    .map_err(|e| format!("event log corrupt; preserving bytes: {e}"))?;
                seq += 1;
                if v["sequence"].as_u64() != Some(seq) {
                    return Err("event sequence is corrupt".into());
                }
            }
        }
        Ok(Self {
            file: OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| e.to_string())?,
            sequence: seq,
            json: json_mode,
        })
    }
    pub fn event(&mut self, candidate: Option<&str>, event: &str, data: Value) -> Result<()> {
        self.sequence += 1;
        let record = json!({"schema":"factor_base_sweep.event/v1","sequence":self.sequence,"unix_ms":SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_millis(),"candidate":candidate,"event":event,"data":data});
        serde_json::to_writer(&mut self.file, &record).map_err(|e| e.to_string())?;
        writeln!(self.file)
            .and_then(|_| self.file.flush())
            .map_err(|e| e.to_string())?;
        if self.json {
            eprintln!("{record}");
        } else {
            let id = candidate.unwrap_or("run");
            match event {
                "solver"=>eprintln!("Info:Solver:{id}: {} vars={} step={} D={} matrix={}x{} productive_D={} field_muls={} elapsed_ms={}",data["event"],data["variables"],data["step"],data["degree"],data["rows"],data["columns"],data["productive_degree"],data["field_multiplications"],data["elapsed_ms"]),
                "trial"=>eprintln!("Info:{}:{id}: {} {}/{} status={} verified={} refuted={} exhausted={} rank={}/{} {:.3} verified/s rolling={:.3}/s independent={}/s phase_ETA~{}s rank_ETA~{}s provisional{}{}",if data["phase"]=="train"{"Relation collection"}else{"Decomposition"},data["phase"],data["trial"],data["total"],data["status"],data["verified"],data["refuted"],data["exhausted"],data["independent_rank"],data["columns"],data["verified_decompositions_per_second"].as_f64().unwrap_or(0.0),data["rolling_verified_per_second"].as_f64().unwrap_or(0.0),optional_number(&data["independent_relations_per_second"]),optional_number(&data["eta_seconds_provisional"]),optional_number(&data["rank_completion_eta_seconds_provisional"]),data.get("observed_rate_ratio_to_baseline").map(|v|format!(" observed baseline relation-stage ratio={}x",v["ratio"])).unwrap_or_default(),data.get("observed_rate_ratio_to_previous").map(|v|format!(" previous={}x (provisional; not end-to-end)",v["ratio"])).unwrap_or_default()),
                _=>eprintln!("Info:{event}:{id}: {data}"),
            }
        }
        Ok(())
    }
}
fn optional_number(v: &Value) -> String {
    v.as_f64()
        .map(|x| format!("{x:.3}"))
        .unwrap_or_else(|| "unknown".into())
}

fn inventory(dir: &Path) -> Result<BTreeMap<String, Value>> {
    let mut out = BTreeMap::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "non-UTF8 artifact")?;
        if name == "complete.json" {
            continue;
        }
        let meta = entry.file_type().map_err(|e| e.to_string())?;
        if !meta.is_file() {
            return Err("attempt contains a non-regular artifact".into());
        }
        out.insert(name,json!({"sha256":digest(&entry.path())?,"bytes":entry.metadata().map_err(|e|e.to_string())?.len()}));
    }
    Ok(out)
}
fn verify_attempt(dir: &Path, candidate: &Candidate, plan: &Plan) -> Result<Value> {
    let done: Value = read(&dir.join("complete.json"))?;
    let files = done["files"]
        .as_object()
        .ok_or("invalid artifact manifest")?;
    if done["schema"] != "factor_base_sweep.attempt/v1"
        || !matches!(
            done["outcome"].as_str(),
            Some("completed" | "failed" | "timed_out" | "matrix_budget_exhausted" | "inapplicable")
        )
    {
        return Err("invalid attempt schema/outcome".into());
    }
    if done["outcome"] == "completed"
        && (!files.contains_key("result.json") || done["exit_code"] != 0)
    {
        return Err("completed attempt lacks a successful result".into());
    }
    for name in files.keys() {
        if Path::new(name).components().count() != 1
            || !matches!(
                Path::new(name).components().next(),
                Some(Component::Normal(_))
            )
        {
            return Err("unsafe artifact member".into());
        }
    }
    if serde_json::to_value(inventory(dir)?).map_err(|e| e.to_string())? != done["files"] {
        return Err(format!("{}: artifact hash/member mismatch", dir.display()));
    }
    let req: Request = read(&dir.join("request.json"))?;
    if req.candidate != *candidate || req.plan != *plan {
        return Err("attempt is bound to a different candidate or plan".into());
    }
    Ok(done)
}

#[derive(Clone)]
struct Observation {
    id: String,
    rates: BTreeMap<u64, f64>,
    from_previous_segment: bool,
}
fn compare(data: &mut Value, reference: &Observation, key: &str) {
    let trial = data["trial"].as_u64().unwrap_or(0);
    if let Some(&base) = reference
        .rates
        .get(&trial)
        .filter(|rate| **rate > 0.0 && trial >= 4)
    {
        if let Some(rate) = data["verified_decompositions_per_second"].as_f64() {
            data[key] = json!({"candidate":reference.id,"ratio":rate/base,"reference_verified_per_second":base,"matched_probe_count":trial,"reference_from_previous_segment":reference.from_previous_segment});
            data["comparison_scope"]=json!("same curve, exact seeded training prefix, solver and limits; provisional wall telemetry including census/setup/logging, not an accepted speedup; previous segments may have different host/load conditions");
        }
    }
}

struct ManagedChild(std::sync::Arc<std::sync::Mutex<Child>>);
impl ManagedChild {
    fn try_wait(&self) -> Result<Option<std::process::ExitStatus>> {
        self.0
            .lock()
            .map_err(|_| "worker lock poisoned")?
            .try_wait()
            .map_err(|e| e.to_string())
    }
    fn kill(&self) -> Result<()> {
        self.0
            .lock()
            .map_err(|_| "worker lock poisoned")?
            .kill()
            .map_err(|e| e.to_string())
    }
    fn wait(&self) -> Result<std::process::ExitStatus> {
        self.0
            .lock()
            .map_err(|_| "worker lock poisoned")?
            .wait()
            .map_err(|e| e.to_string())
    }
}
impl Drop for ManagedChild {
    fn drop(&mut self) {
        if let Ok(mut c) = self.0.lock() {
            if c.try_wait().ok().flatten().is_none() {
                let _ = c.kill();
                let _ = c.wait();
            }
        }
    }
}

fn execute(
    exe: &Path,
    dir: &Path,
    request: &Request,
    log: &mut Log,
    id: &str,
    baseline: Option<&Observation>,
    previous: Option<&Observation>,
    heartbeat: Duration,
) -> Result<Value> {
    write_new(&dir.join("request.json"), request)?;
    let err = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("stderr.log"))
        .map_err(|e| e.to_string())?;
    let mut raw_stdout = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("worker-stdout.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut cmd = Command::new(exe);
    cmd.arg("worker")
        .arg("--attempt")
        .arg(dir)
        .env("RAYON_NUM_THREADS", request.plan.threads.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(err));
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::process::CommandExt;
        // A forcibly stopped supervisor must not leave an unobserved worker.
        let bytes = request.plan.memory_mib * 1024 * 1024;
        unsafe {
            cmd.pre_exec(move || {
                let limit = libc::rlimit {
                    rlim_cur: bytes as libc::rlim_t,
                    rlim_max: bytes as libc::rlim_t,
                };
                if libc::setrlimit(libc::RLIMIT_AS, &limit) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::getppid() == 1 {
                    return Err(std::io::Error::other("supervisor exited"));
                }
                Ok(())
            });
        }
    }
    let began = Instant::now();
    let mut spawned = cmd.spawn().map_err(|e| e.to_string())?;
    let pid = spawned.id();
    let stdout = spawned.stdout.take().ok_or("missing worker stdout")?;
    let child = ManagedChild(std::sync::Arc::new(std::sync::Mutex::new(spawned)));
    let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let expired_watch = expired.clone();
    let process = child.0.clone();
    let (cancel, cancellation) = mpsc::channel();
    let limit = Duration::from_secs(request.plan.candidate_seconds).saturating_sub(began.elapsed());
    let watchdog = std::thread::spawn(move || {
        if matches!(
            cancellation.recv_timeout(limit),
            Err(mpsc::RecvTimeoutError::Timeout)
        ) {
            if let Ok(mut child) = process.lock() {
                if child.try_wait().ok().flatten().is_none() {
                    expired_watch.store(true, std::sync::atomic::Ordering::Relaxed);
                    let _ = child.kill();
                }
            }
        }
    });
    let (tx, rx) = mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut stream = BufReader::new(stdout);
        loop {
            let mut bytes = Vec::new();
            match stream.read_until(b'\n', &mut bytes) {
                Ok(0) => break,
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                    break;
                }
                Ok(_) => {}
            }
            if let Err(e) = raw_stdout
                .write_all(&bytes)
                .and_then(|_| raw_stdout.flush())
            {
                let _ = tx.send(Err(e.to_string()));
                break;
            }
            if tx
                .send(String::from_utf8(bytes).map_err(|e| e.to_string()))
                .is_err()
            {
                break;
            }
        }
    });
    log.event(Some(id),"worker_started",json!({"pid":pid,"threads":request.plan.threads,"memory_mib":request.plan.memory_mib,"address_space_limit_enforced":cfg!(target_os="linux")}))?;
    let mut last_beat = Instant::now();
    let mut resource_cap = false;
    let mut fatal = None;
    let mut stream_closed = false;
    let mut last_activity = Value::Null;
    loop {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(line) => {
                let parsed = line.and_then(|line| {
                    serde_json::from_str::<Value>(&line)
                        .map_err(|e| format!("invalid worker event: {e}"))
                });
                let v = match parsed {
                    Ok(v) if v["event"].is_string() => v,
                    other => {
                        let message = match other {
                            Err(e) => e,
                            Ok(_) => "worker event missing type".into(),
                        };
                        fatal = Some(json!({"message":message,"raw_output":"worker-stdout.jsonl"}));
                        let _ = child.kill();
                        log.event(Some(id), "worker_protocol_error", fatal.clone().unwrap())?;
                        continue;
                    }
                };
                let event = v["event"].as_str().ok_or("worker event missing type")?;
                let mut data = v["data"].clone();
                if event == "fatal" {
                    fatal = Some(data.clone());
                }
                last_activity = json!({"event":event,"data":data});
                if event == "solver"
                    && data["event"] == "matrix"
                    && data["rows"]
                        .as_u64()
                        .unwrap_or(0)
                        .saturating_mul(data["columns"].as_u64().unwrap_or(0))
                        > request.plan.max_matrix_cells
                {
                    resource_cap = true;
                    let _ = child.kill();
                }
                if event == "trial" && data["phase"] == "train" {
                    if let Some(base) = baseline {
                        compare(&mut data, base, "observed_rate_ratio_to_baseline");
                    }
                    if let Some(prev) = previous {
                        compare(&mut data, prev, "observed_rate_ratio_to_previous");
                    }
                }
                log.event(Some(id), event, data)?;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => stream_closed = true,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        if last_beat.elapsed() >= heartbeat {
            log.event(Some(id),"heartbeat",json!({"elapsed_seconds":began.elapsed().as_secs_f64(),"budget_seconds":request.plan.candidate_seconds,"status":"worker active; remaining solver work unknown","last_activity":last_activity}))?;
            last_beat = Instant::now();
        }
        if stream_closed && child.try_wait()?.is_some() {
            break;
        }
        if stream_closed {
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    reader.join().map_err(|_| "worker reader panicked")?;
    let _ = cancel.send(());
    watchdog.join().map_err(|_| "worker watchdog panicked")?;
    let exit = child.wait()?;
    let timed_out = expired.load(std::sync::atomic::Ordering::Relaxed);
    let outcome = if resource_cap {
        "matrix_budget_exhausted"
    } else if timed_out {
        "timed_out"
    } else if exit.success() {
        "completed"
    } else if fatal
        .as_ref()
        .and_then(|v| v["message"].as_str())
        .is_some_and(|s| s.starts_with("inapplicable:"))
    {
        "inapplicable"
    } else {
        "failed"
    };
    if exit.success() {
        let _: Value = read(&dir.join("result.json"))?;
    }
    let complete = json!({"schema":"factor_base_sweep.attempt/v1","outcome":outcome,"exit_code":exit.code(),"fatal":fatal,"elapsed_seconds_operational":began.elapsed().as_secs_f64(),"files":inventory(dir)?});
    write_new(&dir.join("complete.json"), &complete)?;
    log.event(
        Some(id),
        "checkpoint",
        json!({"outcome":outcome,"path":dir,"artifacts_sealed":true}),
    )?;
    Ok(complete)
}

pub fn run(
    plan: Plan,
    dir: &Path,
    resume: bool,
    json_mode: bool,
    heartbeat: Duration,
) -> Result<Value> {
    plan.validate()?;
    #[cfg(target_os = "linux")]
    let exe = std::path::PathBuf::from("/proc/self/exe");
    #[cfg(not(target_os = "linux"))]
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let binary = digest(&exe)?;
    let solver_environment: BTreeMap<String, String> = std::env::vars_os()
        .filter(|(key, _)| key.to_string_lossy().starts_with("F4_"))
        .map(|(key, value)| {
            Ok((
                key.into_string()
                    .map_err(|_| "non-UTF8 F4 environment key")?,
                value
                    .into_string()
                    .map_err(|_| "non-UTF8 F4 environment value")?,
            ))
        })
        .collect::<Result<_>>()?;
    if !resume {
        if let Some(parent) = dir.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::create_dir(dir).map_err(|e| {
            format!(
                "{}: {e}; use a new directory or explicit --resume",
                dir.display()
            )
        })?;
    } else if !dir.is_dir() {
        return Err("resume directory does not exist".into());
    }
    let _lock = lock(dir)?;
    let manifest = json!({"schema":"factor_base_sweep.run/v1","plan":plan,"binary_sha256":binary,"solver_environment":solver_environment});
    if resume {
        let old: Value = read(&dir.join("manifest.json"))?;
        if old != manifest {
            return Err(
                "resume refuses a different plan, executable hash or F4 environment".into(),
            );
        }
    } else {
        write_new(&dir.join("manifest.json"), &manifest)?;
    }
    let mut log = Log::open(dir, json_mode)?;
    log.event(None,if resume{"resume"}else{"start"},json!({"candidates":plan.candidates.len(),"binary_sha256":binary,"plan_sha256":sha256_hex(&serde_json::to_vec(&plan).map_err(|e|e.to_string())?)}))?;
    fs::create_dir_all(dir.join("candidates")).map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    let mut baselines: BTreeMap<String, Observation> = BTreeMap::new();
    let mut previous: BTreeMap<String, Observation> = BTreeMap::new();
    for c in &plan.candidates {
        let id = c.id()?;
        let root = dir.join("candidates").join(&id);
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        let mut attempts = fs::read_dir(&root)
            .map_err(|e| e.to_string())?
            .map(|v| v.map(|x| x.path()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        attempts.sort();
        let mut completed = None;
        for attempt in &attempts {
            if attempt.join("complete.json").exists() {
                completed = Some((attempt.clone(), verify_attempt(attempt, c, &plan)?));
            }
        }
        let cached = completed.is_some();
        let (attempt, done) = if let Some((path, done)) = completed {
            log.event(
                Some(&id),
                "resume_skip",
                json!({"verified_outcome":done["outcome"]}),
            )?;
            (path, done)
        } else {
            let attempt = root.join(format!("attempt-{:04}", attempts.len() + 1));
            fs::create_dir(&attempt).map_err(|e| e.to_string())?;
            log.event(
                Some(&id),
                "candidate_begin",
                json!({"label":c.label,"configuration":c,"attempt":attempts.len()+1}),
            )?;
            let key = serde_json::to_string(&c.curve).map_err(|e| e.to_string())?;
            let done = execute(
                &exe,
                &attempt,
                &Request {
                    plan: plan.clone(),
                    candidate: c.clone(),
                },
                &mut log,
                &id,
                baselines.get(&key),
                previous.get(&key),
                heartbeat,
            )?;
            (attempt, done)
        };
        let mut item = json!({"id":id,"label":c.label,"outcome":done["outcome"],"attempt":attempt});
        if done["outcome"] == "completed" {
            let result: Value = read(&attempt.join("result.json"))?;
            let key = serde_json::to_string(&c.curve).map_err(|e| e.to_string())?;
            if let Some(trials) = result["trials"].as_array() {
                let observation = Observation {
                    id: id.clone(),
                    from_previous_segment: cached,
                    rates: trials
                        .iter()
                        .filter(|v| v["phase"] == "train")
                        .filter_map(|v| {
                            Some((
                                v["trial"].as_u64()? + 1,
                                v["observed_verified_per_second"].as_f64()?,
                            ))
                        })
                        .collect(),
                };
                baselines
                    .entry(key.clone())
                    .or_insert_with(|| observation.clone());
                previous.insert(key, observation);
            }
            item["measurement"] = result;
        }
        results.push(item);
    }
    let mut summary = json!({"schema":"factor_base_sweep.summary/v1","candidates":results,"comparison":"diagnostic observations only; no total-cost winner without complete symmetric native pricing","full_pipeline_speedup":null,"certified_degree_of_regularity":null});
    let mut index = 1;
    loop {
        let name = format!("summary-{index:04}.json");
        let path = dir.join(&name);
        if !path.exists() {
            summary["summary_file"] = json!(name);
            write_new(&path, &summary)?;
            break;
        }
        index += 1;
    }
    log.event(None,"complete",json!({"candidates":plan.candidates.len(),"summary":format!("summary-{index:04}.json"),"status":"bounded sweep finished; inspect per-candidate outcomes"}))?;
    Ok(summary)
}

pub fn verify(dir: &Path) -> Result<Value> {
    let manifest: Value = read(&dir.join("manifest.json"))?;
    if manifest["schema"] != "factor_base_sweep.run/v1" || !dir.join("events.jsonl").is_file() {
        return Err("invalid run manifest or missing event log".into());
    }
    let plan: Plan = serde_json::from_value(manifest["plan"].clone()).map_err(|e| e.to_string())?;
    plan.validate()?;
    let _lock = lock(dir)?;
    let _events = Log::open(dir, true)?;
    let mut count = 0;
    let mut interrupted = 0;
    let mut witnesses = 0;
    for c in &plan.candidates {
        let root = dir.join("candidates").join(c.id()?);
        if !root.exists() {
            continue;
        }
        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.join("complete.json").exists() {
                let done = verify_attempt(&path, c, &plan)?;
                if done["outcome"] == "completed" {
                    witnesses += super::audit::replay(
                        &path,
                        &Request {
                            plan: plan.clone(),
                            candidate: c.clone(),
                        },
                    )?;
                }
                count += 1;
            } else {
                interrupted += 1;
            }
        }
    }
    Ok(
        json!({"sealed_attempts_verified":count,"trial_point_witness_checks":witnesses,"interrupted_attempts_retained":interrupted,"verification":"hashes, native inventory reconstruction, signed lookup, systems and point-witness/relation replay; F4 is not rerun; regularity is not certified"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn provisional_comparison_requires_the_same_probe_prefix_and_positive_reference() {
        let reference = Observation {
            id: "baseline".into(),
            rates: [(4, 2.0), (8, 3.0)].into(),
            from_previous_segment: false,
        };
        let mut early = json!({"trial":3,"verified_decompositions_per_second":6.0});
        compare(&mut early, &reference, "comparison");
        assert!(early.get("comparison").is_none());
        let mut unmatched = json!({"trial":5,"verified_decompositions_per_second":6.0});
        compare(&mut unmatched, &reference, "comparison");
        assert!(unmatched.get("comparison").is_none());
        let mut matched = json!({"trial":4,"verified_decompositions_per_second":6.0});
        compare(&mut matched, &reference, "comparison");
        assert_eq!(matched["comparison"]["ratio"], 3.0);
        assert_eq!(matched["comparison"]["matched_probe_count"], 4);
        let zero = Observation {
            id: "no-successes".into(),
            rates: [(4, 0.0)].into(),
            from_previous_segment: true,
        };
        compare(&mut matched, &zero, "zero_reference");
        assert!(matched.get("zero_reference").is_none());
    }
}
