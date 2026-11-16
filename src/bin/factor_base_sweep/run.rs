use super::forecast;
use super::model::{digest, read, write_new, Candidate, Plan, Request, Result};
use crypto_lib::cryptanalysis::dreg_forecast::MIN_SOLVES;
use crypto_lib::cryptanalysis::dreg_forecast::{json_approx_eq, Advice};
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
        // One write per event: the record and its newline go out together, so a supervisor
        // killed between events never leaves a partial final line (which `open` refuses).
        let mut line = serde_json::to_vec(&record).map_err(|e| e.to_string())?;
        line.push(b'\n');
        self.file
            .write_all(&line)
            .and_then(|_| self.file.flush())
            .map_err(|e| e.to_string())?;
        if self.json {
            eprintln!("{record}");
        } else {
            let id = candidate.unwrap_or("run");
            match event {
                "solver"=>eprintln!("Info:Solver:{id}: {} vars={} step={} D={} matrix={}x{} productive_D={} field_muls={} elapsed_ms={}",data["event"],data["variables"],data["step"],data["degree"],data["rows"],data["columns"],data["productive_degree"],data["field_multiplications"],data["elapsed_ms"]),
                "trial"=>eprintln!("Info:{}:{id}: {} {}/{} status={} verified={} refuted={} exhausted={} rank={}/{} {:.3} verified/s rolling={:.3}/s independent={}/s phase_ETA~{}s rank_ETA~{}s provisional{}{}",if data["phase"]=="train"{"Relation collection"}else{"Decomposition"},data["phase"],data["trial"],data["total"],data["status"],data["verified"],data["refuted"],data["exhausted"],data["independent_rank"],data["columns"],data["verified_decompositions_per_second"].as_f64().unwrap_or(0.0),data["rolling_verified_per_second"].as_f64().unwrap_or(0.0),optional_number(&data["independent_relations_per_second"]),optional_number(&data["eta_seconds_provisional"]),optional_number(&data["rank_completion_eta_seconds_provisional"]),data.get("observed_rate_ratio_to_baseline").map(|v|format!(" observed baseline relation-stage ratio={}x",v["ratio"])).unwrap_or_default(),data.get("observed_rate_ratio_to_previous").map(|v|format!(" previous={}x (provisional; not end-to-end)",v["ratio"])).unwrap_or_default()),
                "forecast"=>eprintln!("Info:Regularity forecast:{id}: solves={} relations={} D_reg~{} band95=[{},{}] P(D>{})={:.2} semi_regular={} yield={:.3} E[work/solve]~{} remaining_work~{} hope(x1,x10,x100)={} advice={} (advisory forecast; not certified; the candidate keeps its budget)",data["solves"],data["relations"],text(&data["predicted_degree_of_regularity"]),text(&data["band_95"][0]),text(&data["band_95"][1]),data["degree_bound"],data["p_exceeds_bound"].as_f64().unwrap_or(1.0),data["semi_regular_degree"],data["relation_yield"].as_f64().unwrap_or(0.0),optional_sci(&data["expected_work_per_solve"]),optional_sci(&data["projected_remaining_work"]),data["hope_ratio_band"].as_array().map(|b|b.iter().map(|h|optional_sci(&h["rho_floor_over_projected_relation_phase"])).collect::<Vec<_>>().join(",")).unwrap_or_else(||"unknown".into()),text(&data["advice"])),
                _=>eprintln!("Info:{event}:{id}: {data}"),
            }
        }
        Ok(())
    }
}
fn text(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), str::to_owned)
}
fn optional_sci(v: &Value) -> String {
    v.as_f64()
        .map(|x| format!("{x:.3e}"))
        .unwrap_or_else(|| "unknown".into())
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
        // The supervisor recomputes the forecast from the attempt's records,
        // for interrupted attempts too, and checks a completed worker's copy.
        match forecast::from_attempt(&attempt) {
            Ok(Some((replayed, view))) => {
                item["forecast"] = view;
                if done["outcome"] == "completed" {
                    let result: Value = read(&attempt.join("result.json"))?;
                    item["forecast_replay_matches_worker"] = json!(json_approx_eq(
                        &serde_json::to_value(&replayed).map_err(|e| e.to_string())?,
                        &result["degree_of_regularity_forecast"],
                        1e-9
                    ));
                }
            }
            Ok(None) => item["forecast"] = Value::Null,
            Err(e) => item["forecast_error"] = json!(e),
        }
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
    let triage = ranking(&results);
    let mut summary = json!({"schema":"factor_base_sweep.summary/v1","candidates":results,"triage":triage,"comparison":"diagnostic observations only; no total-cost winner without complete symmetric native pricing","full_pipeline_speedup":null,"certified_degree_of_regularity":null});
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

fn advice_of(view: &Value) -> Option<Advice> {
    serde_json::from_value(view["advice"].clone()).ok()
}
fn degree_key(view: &Value) -> u64 {
    let bound = view["degree_bound"].as_u64().unwrap_or(0);
    match view["predicted_degree_of_regularity"].as_str() {
        Some(s) if s.starts_with('>') => bound + 1,
        Some(s) => s.parse().unwrap_or(bound + 1),
        None => bound + 2,
    }
}
/// Advisory ranking of candidate items that carry a compact forecast:
/// promising first, then by predicted degree, then by projected work.
pub fn ranking(items: &[Value]) -> Value {
    let mut rows: Vec<Value> = items
        .iter()
        .filter(|item| item["forecast"].is_object())
        .map(|item| {
            let f = &item["forecast"];
            json!({"id":item["id"],"label":item["label"],"outcome":item["outcome"],"attempt":item["attempt"],"advice":f["advice"],"advice_reasons":f["advice_reasons"],"predicted_degree_of_regularity":f["predicted_degree_of_regularity"],"band_95":f["band_95"],"p_exceeds_bound":f["p_exceeds_bound"],"semi_regular_degree":f["semi_regular_degree"],"structural_gain":f["structural_gain"],"solves":f["solves"],"relations":f["relations"],"relation_yield":f["relation_yield"],"projected_relation_phase_work":f["projected_relation_phase_work"],"projected_linear_algebra_work":f["projected_linear_algebra_work"],"projected_relation_phase_work_with_linear_algebra":f["projected_relation_phase_work_with_linear_algebra"],"hope_ratio_band":f["hope_ratio_band"],"hope_ratio_band_with_linear_algebra":f["hope_ratio_band_with_linear_algebra"],"records_read":f["records_read"],"records_unreadable":f["records_unreadable"]})
        })
        .collect();
    rows.sort_by(|a, b| {
        let rank = |v: &Value| advice_of(v).map_or(u8::MAX, forecast::advice_rank);
        // The combined projection ranks when linear algebra was observed.
        let work = |v: &Value| {
            v["projected_relation_phase_work_with_linear_algebra"]
                .as_f64()
                .or_else(|| v["projected_relation_phase_work"].as_f64())
                .unwrap_or(f64::INFINITY)
        };
        rank(a)
            .cmp(&rank(b))
            .then(degree_key(a).cmp(&degree_key(b)))
            .then(work(a).total_cmp(&work(b)))
            .then_with(|| a["id"].as_str().cmp(&b["id"].as_str()))
    });
    json!({"schema":"factor_base_sweep.triage/v1","rows":rows,"order":"advice (promising, undecided, doubtful, hopeless at bound), then predicted degree of regularity, then projected relation-phase work (with priced linear algebra where observed)","advisory":"forecasts of the observed solving-degree distribution from the solves so far; not certified degrees of regularity; no candidate was stopped, skipped or shortened on this basis","certified_degree_of_regularity":null,"full_pipeline_speedup":null})
}

/// Rank every attempt of a run directory from its records on disk.  Takes
/// no lock, so it can read beside a live sweep; records still being
/// written are counted as unreadable, never guessed.
pub fn triage(dir: &Path) -> Result<Value> {
    let manifest: Value = read(&dir.join("manifest.json"))?;
    if manifest["schema"] != "factor_base_sweep.run/v1" {
        return Err("invalid run manifest".into());
    }
    let plan: Plan = serde_json::from_value(manifest["plan"].clone()).map_err(|e| e.to_string())?;
    plan.validate()?;
    let mut items = Vec::new();
    for c in &plan.candidates {
        let id = c.id()?;
        let root = dir.join("candidates").join(&id);
        if !root.is_dir() {
            continue;
        }
        let mut attempts: Vec<_> = fs::read_dir(&root)
            .map_err(|e| e.to_string())?
            .map(|v| v.map(|x| x.path()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        attempts.sort();
        for attempt in attempts {
            if !attempt.join("request.json").is_file() {
                continue;
            }
            let outcome = read::<Value>(&attempt.join("complete.json"))
                .map(|done| done["outcome"].clone())
                .unwrap_or_else(|_| json!("in_progress_or_interrupted"));
            let mut item = json!({"id":id,"label":c.label,"outcome":outcome,"attempt":attempt});
            match forecast::from_attempt(&attempt) {
                Ok(Some((_, view))) => item["forecast"] = view,
                Ok(None) => item["forecast"] = Value::Null,
                Err(e) => item["forecast_error"] = json!(e),
            }
            items.push(item);
        }
    }
    let mut out = ranking(&items);
    out["run"] = json!(dir);
    out["attempts_without_forecast"] = json!(items
        .iter()
        .filter(|i| !i["forecast"].is_object())
        .map(|i| json!({"id":i["id"],"label":i["label"],"attempt":i["attempt"],"outcome":i["outcome"],"error":i["forecast_error"]}))
        .collect::<Vec<_>>());
    Ok(out)
}

/// The label of a candidate without the `-dN-seedS` / `-seedS` suffix the
/// grid and `refine` append.
fn base_label(label: &str) -> &str {
    let mut base = label;
    if let Some(i) = base.rfind("-seed") {
        if base.len() > i + 5 && base[i + 5..].chars().all(|c| c.is_ascii_digit()) {
            base = &base[..i];
        }
    }
    if let Some(i) = base.rfind("-d") {
        if base.len() > i + 2 && base[i + 2..].chars().all(|c| c.is_ascii_digit()) {
            base = &base[..i];
        }
    }
    base
}

/// The next plan's candidates from a plan and its triage rows (one row per
/// attempt).  A candidate is dropped when its latest attempt is
/// `inapplicable`, or when its forecast has at least [`MIN_SOLVES`] solves
/// and an advice named in `drop`; every other candidate is kept and, for
/// binary shapes, expanded over `seed_count` consecutive seeds and the
/// `dimensions` given (its own dimension when none).  Pure: nothing runs.
pub fn refine_candidates(
    plan: &Plan,
    rows: &[Value],
    drop: &[String],
    seed_count: u32,
    dimensions: &[u32],
) -> Result<(Vec<Candidate>, Value)> {
    let mut latest: BTreeMap<String, &Value> = BTreeMap::new();
    for row in rows {
        if let Some(id) = row["id"].as_str() {
            let newer = latest
                .get(id)
                .is_none_or(|old| old["attempt"].as_str() <= row["attempt"].as_str());
            if newer {
                latest.insert(id.to_string(), row);
            }
        }
    }
    let mut kept = Vec::new();
    let mut dropped = Vec::new();
    let mut next = Vec::new();
    let mut ids = std::collections::BTreeSet::new();
    for c in &plan.candidates {
        let id = c.id()?;
        let row = latest.get(&id);
        let advice = row.and_then(|r| r["advice"].as_str()).map(str::to_owned);
        let solves = row.and_then(|r| r["solves"].as_u64()).unwrap_or(0);
        let outcome = row
            .and_then(|r| r["outcome"].as_str())
            .unwrap_or("not attempted")
            .to_string();
        let reasons = row
            .map(|r| r["advice_reasons"].clone())
            .unwrap_or(Value::Null);
        let verdict = json!({"id":id,"label":c.label,"outcome":outcome,"advice":advice,"solves":solves,"advice_reasons":reasons});
        if outcome == "inapplicable" {
            dropped.push(json!({"why":"inapplicable on this curve","candidate":verdict}));
            continue;
        }
        if let Some(a) = &advice {
            if solves as usize >= MIN_SOLVES && drop.iter().any(|d| d == a) {
                dropped.push(json!({"why":format!("advice `{a}` after {solves} solves"),"candidate":verdict}));
                continue;
            }
        }
        kept.push(verdict);
        let base = base_label(&c.label).to_string();
        if c.shape.is_some() {
            let dims: Vec<u32> = if dimensions.is_empty() {
                vec![c.dimension]
            } else {
                dimensions.to_vec()
            };
            for offset in 0..seed_count.max(1) {
                let seed = c
                    .seed
                    .checked_add(u64::from(offset))
                    .ok_or("seed range overflows")?;
                for &dimension in &dims {
                    let mut n = c.clone();
                    n.seed = seed;
                    n.dimension = dimension;
                    n.label = format!("{base}-d{dimension}-seed{seed}");
                    if ids.insert(n.id()?) {
                        next.push(n);
                    }
                }
            }
        } else {
            let isogeny = c
                .factor_base
                .as_deref()
                .is_some_and(|s| s.starts_with("prime-isogeny:"));
            let seeds = if isogeny { seed_count.max(1) } else { 1 };
            for offset in 0..seeds {
                let seed = c
                    .seed
                    .checked_add(u64::from(offset))
                    .ok_or("seed range overflows")?;
                let mut n = c.clone();
                n.seed = seed;
                if isogeny {
                    n.factor_base = Some(format!("prime-isogeny:t=2,seed={seed}"));
                    n.label = format!("{base}-seed{seed}");
                }
                if ids.insert(n.id()?) {
                    next.push(n);
                }
            }
        }
    }
    if next.is_empty() {
        return Err(format!(
            "refine dropped every candidate ({} dropped); widen what is kept, for example --drop hopeless_at_bound",
            dropped.len()
        ));
    }
    let provenance = json!({"schema":"factor_base_sweep.refine/v1","drop":drop,"seed_count":seed_count.max(1),"dimensions":dimensions,"kept":kept,"dropped":dropped,"next_candidates":next.len(),"rule":"a candidate is dropped when its latest attempt is inapplicable or when its forecast has at least MIN_SOLVES solves and an advice in `drop`; kept candidates are expanded over consecutive seeds and the given dimensions; budgets are inherited unless tuning flags change them","advisory":"the forecast is advisory; this plan is a proposal for the next sweep, not a result, and nothing was run to produce it"});
    Ok((next, provenance))
}

/// `refine_candidates` on a run directory: its frozen plan and the live
/// triage of its attempts.  Launches nothing and takes no lock.
pub fn refine(
    dir: &Path,
    drop: &[String],
    seed_count: u32,
    dimensions: &[u32],
) -> Result<(Plan, Value)> {
    let manifest: Value = read(&dir.join("manifest.json"))?;
    let mut plan: Plan =
        serde_json::from_value(manifest["plan"].clone()).map_err(|e| e.to_string())?;
    plan.validate()?;
    let ranking = triage(dir)?;
    // Attempts without a forecast (no inventory, inapplicable, failed before
    // a solve) still carry their outcome, which decides inapplicability.
    let mut rows = ranking["rows"].as_array().cloned().unwrap_or_default();
    rows.extend(
        ranking["attempts_without_forecast"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    let (candidates, mut provenance) =
        refine_candidates(&plan, &rows, drop, seed_count, dimensions)?;
    plan.candidates = candidates;
    plan.validate()?;
    provenance["run"] = json!(dir);
    provenance["source_plan_sha256"] = json!(sha256_hex(
        &serde_json::to_vec(&manifest["plan"]).map_err(|e| e.to_string())?
    ));
    provenance["triage_rows"] = json!(rows.len());
    Ok((plan, provenance))
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
    fn triage_ranking_orders_promising_low_degree_cheap_first() {
        let item = |id: &str, advice: &str, degree: &str, work: f64| json!({"id":id,"label":id,"outcome":"completed","attempt":"a","forecast":{"advice":advice,"degree_bound":8,"predicted_degree_of_regularity":degree,"projected_relation_phase_work":work}});
        let items = vec![
            item("hopeless", "hopeless_at_bound", ">8", 1.0),
            item("doubtful", "doubtful", "3", 1.0),
            item("promising-slow", "promising", "4", 9.0),
            item("promising-fast", "promising", "4", 2.0),
            item("promising-low", "promising", "3", 50.0),
            item("undecided", "undecided", "5", 1.0),
            json!({"id":"no-forecast","forecast":null}),
        ];
        let out = ranking(&items);
        let ids: Vec<_> = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            ids,
            [
                "promising-low",
                "promising-fast",
                "promising-slow",
                "undecided",
                "doubtful",
                "hopeless"
            ]
        );
        assert!(out["certified_degree_of_regularity"].is_null());
        assert!(out["advisory"]
            .as_str()
            .unwrap()
            .contains("no candidate was stopped"));
    }
    #[test]
    fn refine_drops_decided_bad_advice_keeps_the_rest_and_expands_seeds() {
        let plan = Plan::grid("binary", 1, 1, &[3]).unwrap();
        let ids: Vec<String> = plan.candidates.iter().map(|c| c.id().unwrap()).collect();
        let row = |id: &str, advice: &str, solves: u64, outcome: &str| json!({"id":id,"attempt":"a/attempt-0001","outcome":outcome,"advice":advice,"solves":solves,"advice_reasons":["r"]});
        let rows = vec![
            row(&ids[0], "hopeless_at_bound", 8, "completed"),
            row(&ids[1], "doubtful", 2, "completed"), // too few solves: kept
            row(&ids[2], "promising", 8, "completed"),
            row(&ids[3], "doubtful", 8, "inapplicable"),
            // A later attempt supersedes an earlier one for the same id.
            json!({"id":ids[4],"attempt":"a/attempt-0001","outcome":"completed","advice":"doubtful","solves":9}),
            json!({"id":ids[4],"attempt":"a/attempt-0002","outcome":"completed","advice":"undecided","solves":9}),
        ];
        let drop = vec!["hopeless_at_bound".to_string(), "doubtful".to_string()];
        let (next, prov) = refine_candidates(&plan, &rows, &drop, 2, &[3, 4]).unwrap();
        let labels: Vec<&str> = next.iter().map(|c| c.label.as_str()).collect();
        let starts = |k: usize| {
            let base = format!("{}-", base_label(&plan.candidates[k].label));
            labels.iter().any(|l| l.starts_with(&base))
        };
        assert!(!starts(0), "{labels:?}");
        assert!(starts(1));
        assert!(starts(2));
        assert!(!starts(3));
        assert!(starts(4));
        // Every kept shape candidate: 2 seeds × 2 dimensions.
        let kept = prov["kept"].as_array().unwrap().len();
        assert_eq!(prov["dropped"].as_array().unwrap().len(), 2);
        assert_eq!(next.len(), kept * 4);
        let expanded = format!("{}-d4-seed2", base_label(&plan.candidates[2].label));
        assert!(labels.contains(&expanded.as_str()), "{labels:?}");
        let mut plan2 = plan.clone();
        plan2.candidates = next;
        plan2.validate().unwrap();
        assert_eq!(base_label("binary-gp-d3-seed1"), "binary-gp");
        assert_eq!(base_label("prime-kummer:t=2-seed7"), "prime-kummer:t=2");
        assert_eq!(base_label("plain"), "plain");
        // Dropping everything is an error, not an empty plan.
        let all = vec![row(&ids[0], "doubtful", 8, "completed")];
        let mut one = plan.clone();
        one.candidates.truncate(1);
        assert!(refine_candidates(&one, &all, &drop, 1, &[]).is_err());
    }

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
