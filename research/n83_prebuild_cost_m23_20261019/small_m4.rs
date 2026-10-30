//! Explicit frozen m=4 policies; the parent owns the byte-pinned base importer.
use super::*;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    ChainedS3Encoding, FrobeniusOrbitRootEncoding,
};
use crypto_lib::cryptanalysis::sat::{RootPropagationResult, SolveResult, SolverProgress};
use std::fs::File;
use std::sync::{mpsc, Arc};

#[derive(Clone, Copy)]
enum Policy {
    Legacy,
    SmallCache(bool),
    Full(bool),
    SharedRoot(FrobeniusOrbitRootEncoding),
}
impl Policy {
    fn check(self, mode: &str, arm: &str, target: usize) -> Result<()> {
        let valid = matches!(mode, "preflight" | "positive" | "natural")
            && match self {
                Self::Legacy => matches!(arm, "K2048" | "D12") && target == 0,
                Self::SmallCache(_) => matches!(arm, "K2048" | "D12") && target <= 1,
                Self::Full(cache) => {
                    matches!(arm, "U" | "D")
                        && target < 8
                        && (mode != "positive" || target == 0)
                        && (cache || mode == "positive")
                }
                Self::SharedRoot(_) => {
                    matches!(arm, "U" | "D")
                        && target < 8
                        && (mode != "positive" || target == 0)
                        && (mode != "natural" || arm == "D")
                }
            };
        if valid {
            Ok(())
        } else {
            Err("m4 request outside frozen protocol".into())
        }
    }
    fn cache(self) -> Option<bool> {
        match self {
            Self::Legacy => None,
            Self::SmallCache(b) | Self::Full(b) => Some(b),
            Self::SharedRoot(_) => Some(true),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Legacy => "small-m4/v1",
            Self::SmallCache(_) => "small-m4-xor-cache/v1",
            Self::Full(_) => "full-m4-common-public/v1",
            Self::SharedRoot(_) => "d18-shared-root-m4/v1",
        }
    }
    fn root_encoding(self) -> FrobeniusOrbitRootEncoding {
        match self {
            Self::SharedRoot(encoding) => encoding,
            _ => FrobeniusOrbitRootEncoding::PerRepresentative,
        }
    }
}

fn root_setting(setting: &str) -> Result<FrobeniusOrbitRootEncoding> {
    match setting {
        "per-representative" => Ok(FrobeniusOrbitRootEncoding::PerRepresentative),
        "shared-bits" => Ok(FrobeniusOrbitRootEncoding::SharedBits),
        _ => Err("root encoding must be per-representative or shared-bits".into()),
    }
}
fn cache_setting(cache: &str) -> Result<bool> {
    match cache {
        "off" => Ok(false),
        "on" => Ok(true),
        _ => Err("cache setting must be off or on".into()),
    }
}

fn check_full_pair(off: &Value, on: &Value) -> Result<()> {
    for (w, enabled) in [(off, false), (on, true)] {
        if w["schema"] != "n83.small-m4-worker/v1"
            || w["experiment_policy"] != "full-m4-common-public/v1"
            || w["status"] != "PASS_CONSTRUCTED_PENDING_REPLAY"
            || w["mode"] != "positive"
            || w["curve_id"] != CURVE
            || w["m"] != 4
            || w["target_index"] != 0
            || w["conflict_budget"] != 10000
            || w["xor_decided_cache"] != enabled
            || w["group_verified"] != true
            || w["rank"] != 1
            || w["columns"] != 16627
            || w["points"] != 2760082
            || w["public_corpus_sha256"] != PUBLIC_SHA256
            || !matches!(w["arm"].as_str(), Some("U" | "D"))
            || w["counters"]["phase"] != "Complete"
            || w["sat_model_blake3"].as_str().is_none_or(|s| s.len() != 64)
        {
            return Err("full-base control contract differs".into());
        }
        for k in ["candidate_id", "selected_best_total_runtime"] {
            if w.get(k) != Some(&Value::Null) {
                return Err("control selected candidate".into());
            }
        }
    }
    for k in [
        "arm",
        "source_commit",
        "binary_sha256",
        "source_module_blake3",
        "source_worker_blake3",
        "object_blake3",
        "point_set_blake3",
        "labeled_rows_blake3",
        "target",
        "sat_variables",
        "sat_clauses",
        "sat_xor_rows",
        "sat_model_blake3",
        "point_indices",
        "sparse_row",
        "sparse_row_blake3",
    ] {
        if off.get(k).is_none() || off[k] != on[k] {
            return Err(format!("full-base control differs: {k}").into());
        }
    }
    for k in [
        "completed_conflicts",
        "completed_decisions",
        "completed_restarts",
        "completed_xor_passes",
        "completed_xor_row_ops",
        "completed_xor_row_scans",
    ] {
        if off["counters"][k].as_u64().is_none() || off["counters"][k] != on["counters"][k] {
            return Err("full-base control search differs".into());
        }
    }
    let before = off["counters"]["completed_xor_repivots"]
        .as_u64()
        .ok_or("off visits")?;
    let after = on["counters"]["completed_xor_repivots"]
        .as_u64()
        .ok_or("on visits")?;
    let skipped = on["xor_decided_skips"].as_u64().ok_or("on skips")?;
    if off["xor_decided_skips"] != 0 || after.checked_add(skipped) != Some(before) {
        return Err("full-base cache conservation differs".into());
    }
    Ok(())
}

pub(super) fn full_control_pair(
    off_path: &Path,
    on_path: &Path,
    off_replay: &Path,
    on_replay: &Path,
    output: &Path,
) -> Result<()> {
    let started = Instant::now();
    let off_bytes = fs::read(off_path)?;
    let on_bytes = fs::read(on_path)?;
    let off: Value = serde_json::from_slice(&off_bytes)?;
    let on: Value = serde_json::from_slice(&on_bytes)?;
    check_full_pair(&off, &on)?;
    for (w, bytes, path) in [(&off, &off_bytes, off_replay), (&on, &on_bytes, on_replay)] {
        let r: Value = serde_json::from_slice(&fs::read(path)?)?;
        if r["schema"] != "n83.small-m4-general-replay/v1"
            || r["status"] != "PASS"
            || r["rank_lower_bound"] != 1
            || r["worker_blake3"] != blake3::hash(bytes).to_hex().as_str()
            || r["changed_target_rejected"] != true
            || r["changed_point_rejected"] != true
            || r["changed_coefficient_rejected"] != true
        {
            return Err("full-base control replay differs".into());
        }
        for k in [
            "arm",
            "curve_id",
            "mode",
            "object_blake3",
            "public_corpus_sha256",
            "point_indices",
        ] {
            if r[k] != w[k] {
                return Err("full-base replay identity differs".into());
            }
        }
    }
    write_new_json(
        output,
        &json!({"schema":"n83.full-m4-control-pair/v1","status":"PASS","arm":off["arm"],
        "off_worker_blake3":blake3::hash(&off_bytes).to_hex().to_string(),"on_worker_blake3":blake3::hash(&on_bytes).to_hex().to_string(),
        "binary_sha256":format!("{:x}",Sha256::digest(fs::read(std::env::current_exe()?)?)),
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,"matched_model_and_row":true,"matched_search_work":true,
        "row_visit_conservation":true,"candidate_id":null,"selected_best_total_runtime":null}),
    )
}

fn append(file: &mut File, record: &Value) -> Result<()> {
    serde_json::to_writer(&mut *file, record)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn sample(progress: &SolverProgress) -> Value {
    let s = progress.snapshot();
    json!({"phase":format!("{:?}",s.phase),"completed_decisions":s.decisions,
        "completed_conflicts":s.conflicts,"completed_restarts":s.restarts,
        "completed_xor_passes":s.xor_passes,"completed_xor_nanoseconds":s.xor_nanoseconds,
        "completed_xor_row_scans":s.xor_row_scans,"completed_xor_row_ops":s.xor_row_ops,
        "completed_xor_repivots":s.xor_repivots})
}

fn budget_receipt(mut identity: Value, started: Instant, memory_peak: &Path) -> Value {
    let memory = fs::read_to_string(memory_peak)
        .map_err(|e| format!("memory.peak read: {e}"))
        .and_then(|s| {
            s.trim()
                .parse::<u64>()
                .map_err(|e| format!("memory.peak parse: {e}"))
        });
    let (rss, cpu) = resource_usage();
    identity["status"] = json!("UNKNOWN_budget");
    identity["elapsed_ms"] = json!(started.elapsed().as_secs_f64() * 1000.0);
    identity["peak_rss_bytes"] = json!(rss);
    identity["cpu_seconds"] = json!(cpu);
    identity["resource_usage_error"] =
        json!((rss.is_none() || cpu.is_none()).then_some("getrusage value unavailable"));
    match memory {
        Ok(peak) => {
            identity["memory_cgroup_peak_bytes"] = json!(peak);
            identity["memory_cgroup_peak_error"] = Value::Null;
        }
        Err(error) => {
            identity["memory_cgroup_peak_bytes"] = Value::Null;
            identity["memory_cgroup_peak_error"] = json!(error);
        }
    }
    identity["resource_accounting_scope"] =
        json!("Watchdog readings before final receipt serialization; durable solver counters remain in the separate progress stream.");
    identity
}

fn cgroup_contract() -> Result<()> {
    // Docker uses a private cgroup namespace with its resource files at the root.
    if fs::read_to_string("/sys/fs/cgroup/memory.max")?.trim() != "4294967296"
        || fs::read_to_string("/sys/fs/cgroup/memory.swap.max")?.trim() != "0"
    {
        return Err("small m4 requires a hard 4 GiB zero-swap Docker cgroup".into());
    }
    Ok(())
}

fn lift(fast: &FastBinaryCurve128, base: &Base, xs: &[u128], target: Words) -> Option<Vec<usize>> {
    if xs.len() != 4 {
        return None;
    }
    let choices: Vec<Vec<usize>> = xs
        .iter()
        .map(|&x| {
            fast.points_with_x(1, x)
                .iter()
                .filter_map(|p| p.and_then(|p| base.index.get(&p).copied()))
                .collect()
        })
        .collect();
    for a in &choices[0] {
        for b in &choices[1] {
            for c in &choices[2] {
                for d in &choices[3] {
                    let mut indices = vec![*a, *b, *c, *d];
                    let sum = indices
                        .iter()
                        .fold(None, |sum, &i| fast.add(sum, Some(base.rows[i].point)));
                    if sum == Some(target) {
                        indices.sort_unstable();
                        return Some(indices);
                    }
                }
            }
        }
    }
    None
}

pub(super) fn run(
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
) -> Result<()> {
    run_inner(
        mode,
        arm,
        root,
        corpus,
        target_index,
        output,
        Policy::Legacy,
    )
}

pub(super) fn cache_run(
    cache: &str,
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
) -> Result<()> {
    run_inner(
        mode,
        arm,
        root,
        corpus,
        target_index,
        output,
        Policy::SmallCache(cache_setting(cache)?),
    )
}

pub(super) fn full_run(
    cache: &str,
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
) -> Result<()> {
    run_inner(
        mode,
        arm,
        root,
        corpus,
        target_index,
        output,
        Policy::Full(cache_setting(cache)?),
    )
}

pub(super) fn shared_root_run(
    encoding: &str,
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
) -> Result<()> {
    run_inner(
        mode,
        arm,
        root,
        corpus,
        target_index,
        output,
        Policy::SharedRoot(root_setting(encoding)?),
    )
}

fn run_inner(
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
    policy: Policy,
) -> Result<()> {
    policy.check(mode, arm, target_index)?;
    let cache = policy.cache();
    if output.exists() {
        return Err("output already exists".into());
    }
    cgroup_contract()?;
    let started = Instant::now();
    let source_commit = std::env::var("N83_SOURCE_COMMIT")?;
    if source_commit.len() != 40 || !source_commit.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("source commit required".into());
    }
    let mut identity = json!({"schema":"n83.small-m4-worker/v1","curve_id":CURVE,
        "arm":arm,"mode":mode,"m":4,"target_index":target_index,"source_commit":source_commit,
        "binary_sha256":format!("{:x}",Sha256::digest(fs::read(std::env::current_exe()?)?)),
        "source_module_blake3":blake3::hash(include_bytes!("small_m4.rs")).to_hex().to_string(),
        "source_worker_blake3":blake3::hash(include_bytes!("../../examples/n83_prebuild_cost_m23.rs")).to_hex().to_string(),
        "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null});
    let conflict_budget = if cache.is_some() { 10000 } else { 100000 };
    if let Some(enabled) = cache {
        identity["experiment_policy"] = json!(policy.name());
        identity["xor_decided_cache"] = json!(enabled);
        identity["conflict_budget"] = json!(conflict_budget);
    }
    if let Policy::SharedRoot(encoding) = policy {
        identity["root_encoding"] = json!(match encoding {
            FrobeniusOrbitRootEncoding::PerRepresentative => "per-representative",
            FrobeniusOrbitRootEncoding::SharedBits => "shared-bits",
        });
        identity["source_encoding_sha256"] = json!(format!(
            "{:x}",
            Sha256::digest(include_bytes!(
                "../../src/cryptanalysis/binary_semaev_chain_sat.rs"
            ))
        ));
    }
    let mut phases = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(format!("{}.phases.jsonl", output.display()))?;
    let mut phase = |name: &str, details: Value| -> Result<()> {
        let (rss, cpu) = resource_usage();
        append(
            &mut phases,
            &json!({"schema":"n83.small-m4-phase/v1","phase":name,"arm":arm,"mode":mode,
            "elapsed_ms":started.elapsed().as_secs_f64()*1000.0,"peak_rss_bytes":rss,"cpu_seconds":cpu,"details":details}),
        )
    };
    phase("import_entered", identity.clone())?;
    let cap_path = PathBuf::from(format!("{}.cap.json", output.display()));
    let cap = identity.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1790));
        let cap = budget_receipt(cap, started, Path::new("/sys/fs/cgroup/memory.peak"));
        std::process::exit(if write_new_json(&cap_path, &cap).is_ok() {
            124
        } else {
            125
        });
    });
    let base = load_base(arm, root)?;
    identity["object_blake3"] = json!(base.object_blake3);
    identity["point_set_blake3"] = json!(base.point_set_blake3);
    identity["labeled_rows_blake3"] = json!(labeled_rows_hash(&base));
    identity["columns"] = json!(base.rows.len() / 166);
    identity["points"] = json!(base.rows.len());
    identity["base_import_index_ms"] = json!(started.elapsed().as_secs_f64() * 1000.0);
    phase("corpus_validation_entered", identity.clone())?;
    let part = Instant::now();
    let public = public_targets(corpus)?;
    let (kc, fast) = curve()?;
    identity["public_corpus_sha256"] = json!(PUBLIC_SHA256);
    identity["public_validation_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
    let indices = vec![0usize, 166, 332, 498];
    let mut partial = Vec::new();
    let mut sum = None;
    for &i in &indices {
        sum = fast.add(sum, Some(base.rows[i].point));
        partial.push(sum);
    }
    let target = if mode == "positive" {
        sum.ok_or("positive infinity")?
    } else {
        public[target_index]
    };
    identity["target"] = json!([target.0.to_string(), target.1.to_string()]);
    if mode == "preflight" {
        identity["status"] = json!("PASS_PREFLIGHT");
        identity["process_wall_ms"] = json!(started.elapsed().as_secs_f64() * 1000.0);
        return write_new_json(output, &identity);
    }
    let part = Instant::now();
    let representatives: Vec<BigUint> = base
        .rows
        .iter()
        .step_by(166)
        .map(|r| BigUint::from(r.point.0))
        .collect();
    if matches!(policy, Policy::SharedRoot(_)) {
        let mut digest = blake3::Hasher::new();
        digest.update(b"n83.root-representatives/v1");
        for row in base.rows.iter().step_by(166) {
            digest.update(&row.point.0.to_le_bytes());
        }
        identity["root_representatives_blake3"] = json!(digest.finalize().to_hex().to_string());
        identity["root_shared_bit_count"] = json!((0..83u64)
            .filter(|&bit| representatives
                .iter()
                .all(|r| r.bit(bit) == representatives[0].bit(bit)))
            .count());
    }
    let target_x = fast.element(target.0);
    phase(
        "model_entered",
        json!({"max_variables":300000,"max_domain_clauses":8000000}),
    )?;
    let mut enc = ChainedS3Encoding::build(
        4,
        &kc.curve.irreducible,
        &kc.curve.b,
        Some(&target_x),
        0,
        300000,
    )
    .map_err(|_| "small m4 model build failed")?;
    enc.constrain_frobenius_orbit_summands_with_encoding(
        &representatives,
        300000,
        8000000,
        policy.root_encoding(),
    )
    .map_err(|_| "small m4 orbit-domain failed")?;
    identity["model_domain_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
    identity["sat_variables"] = json!(enc.solver.n_vars());
    identity["sat_clauses"] = json!(enc.solver.n_clauses());
    identity["sat_xor_rows"] = json!(enc.solver.n_xors());
    phase("model_done", identity.clone())?;
    let pin = Instant::now();
    if mode == "positive" {
        let mut blocks: Vec<_> = indices
            .iter()
            .map(|&i| fast.element(base.rows[i].point.0))
            .collect();
        for p in &partial[1..3] {
            blocks.push(fast.element(p.ok_or("positive nonaffine intermediate")?.0));
        }
        blocks.push(target_x.clone());
        for (block, x) in blocks.iter().enumerate() {
            enc.pin_x_block(block, x)
                .map_err(|_| "positive block pin")?;
        }
        let mut sorted = representatives.clone();
        sorted.sort_unstable();
        if sorted.windows(2).any(|p| p[0] == p[1]) {
            return Err("duplicate representative x".into());
        }
        let handles = enc.orbit_choice_wires().ok_or("missing choice wires")?;
        let mut choices = Vec::new();
        for (slot, &i) in indices.iter().enumerate() {
            let pos = sorted
                .binary_search(&BigUint::from(base.rows[i].point.0))
                .map_err(|_| "missing representative")?;
            let h = &handles[slot];
            if h.selectors.len() != sorted.len() || h.phase.len() != 7 {
                return Err("choice wire shape".into());
            }
            choices.push((h.selectors[pos], h.phase.clone()));
        }
        for (selector, phase) in choices {
            if !enc.solver.add_clause(vec![selector as i32]) {
                return Err("selector pin failed".into());
            }
            for wire in phase {
                if !enc.solver.add_clause(vec![-(wire as i32)]) {
                    return Err("phase pin failed".into());
                }
            }
        }
    }
    identity["positive_pin_ms"] = json!(if mode == "positive" {
        pin.elapsed().as_secs_f64() * 1000.0
    } else {
        0.0
    });
    enc.solver.conflict_budget = conflict_budget;
    enc.solver.set_xor_decided_cache(cache.unwrap_or(false));
    enc.solver.deadline = Some(started + Duration::from_secs(1790));
    let progress = Arc::new(SolverProgress::default());
    enc.solver.set_progress_observer(Arc::clone(&progress));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(format!("{}.progress.jsonl", output.display()))?;
    let (stop, receiver) = mpsc::channel();
    let observed = Arc::clone(&progress);
    let progress_identity = identity.clone();
    let observer = std::thread::spawn(move || -> std::io::Result<()> {
        let mut done = false;
        loop {
            let (rss, cpu) = resource_usage();
            let record = json!({"schema":"n83.small-m4-progress/v1","identity":progress_identity,
                "elapsed_ms":started.elapsed().as_secs_f64()*1000.0,"peak_rss_bytes":rss,
                "cpu_seconds":cpu,"counters":sample(&observed),"final_sample":done});
            serde_json::to_writer(&mut file, &record)?;
            file.write_all(b"\n")?;
            file.sync_all()?;
            if done {
                return Ok(());
            }
            done = !matches!(
                receiver.recv_timeout(Duration::from_secs(10)),
                Err(mpsc::RecvTimeoutError::Timeout)
            );
        }
    });
    phase("root_entered", sample(&progress))?;
    let part = Instant::now();
    let root_result = enc.solver.propagate_root();
    identity["root_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
    phase(
        "root_returned",
        json!({"result":format!("{root_result:?}"),"counters":sample(&progress)}),
    )?;
    phase("search_entered", sample(&progress))?;
    let part = Instant::now();
    let answer = if root_result == RootPropagationResult::Unsat {
        SolveResult::Unsat
    } else {
        enc.solver.solve()
    };
    identity["search_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
    let _ = stop.send(());
    observer.join().map_err(|_| "sampler panicked")??;
    phase(
        "search_returned",
        json!({"answer":format!("{answer:?}"),"counters":sample(&progress)}),
    )?;
    identity["counters"] = sample(&progress);
    if cache.is_some() {
        identity["xor_decided_skips"] = json!(enc.solver.stats.xor_decided_skips);
    }
    identity["status"] = json!(match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat => "UNVERIFIED_MODEL_UNSAT",
        SolveResult::Sat => "INVALID_MODEL",
    });
    if answer == SolveResult::Sat {
        let part = Instant::now();
        if cache.is_some() {
            identity["sat_model_blake3"] =
                json!(blake3::hash(&serde_json::to_vec(&enc.solver.model())?)
                    .to_hex()
                    .to_string());
        }
        let xs = enc.decode_x_blocks();
        if !enc.verify_model() || xs.len() != 7 || xs[6] != target_x {
            return Err("SAT model failed original-field verification".into());
        }
        let xs: Vec<_> = xs[..4].iter().map(|x| fast.word(x)).collect();
        let found =
            lift(&fast, &base, &xs, target).ok_or("SAT model failed stored-point group lift")?;
        if mode == "positive" && found != indices {
            return Err("positive indices differ".into());
        }
        let row = relation_row(&base.rows, &found, &kc.subgroup_order);
        let canonical: Vec<_> = row.iter().map(|(&c, v)| (c, v.to_string())).collect();
        identity["sparse_row_blake3"] = json!(blake3::hash(&serde_json::to_vec(&canonical)?)
            .to_hex()
            .to_string());
        identity["point_indices"] = json!(found);
        identity["sparse_row"] = json!(canonical);
        let changed = fast
            .add(Some(target), Some(words(kc.generator(), &fast)?))
            .ok_or("changed target infinity")?;
        let mut ignored = RankTracker::new(kc.subgroup_order.clone());
        if verify_relation(&fast, &base, changed, &found, &mut ignored).is_ok() {
            return Err("target tamper accepted".into());
        }
        let mut changed_indices = found.clone();
        changed_indices[0] ^= 1;
        if verify_relation(&fast, &base, target, &changed_indices, &mut ignored).is_ok() {
            return Err("row tamper accepted".into());
        }
        identity["changed_target_rejected"] = json!(true);
        identity["changed_row_rejected"] = json!(true);
        identity["group_verified"] = json!(true);
        identity["relation_check_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
        let part = Instant::now();
        let mut rank = RankTracker::new(kc.subgroup_order.clone());
        if !rank.insert(row) {
            return Err("returned row has rank zero".into());
        }
        identity["rank"] = json!(rank.rank());
        identity["rank_ms"] = json!(part.elapsed().as_secs_f64() * 1000.0);
        identity["status"] = json!(if mode == "positive" {
            "PASS_CONSTRUCTED_PENDING_REPLAY"
        } else {
            "PASS_NATURAL_PENDING_REPLAY"
        });
    } else if mode == "positive" {
        identity["status"] = json!("INVALID_POSITIVE_CONTROL");
    }
    identity["process_wall_ms"] = json!(started.elapsed().as_secs_f64() * 1000.0);
    identity["memory_cgroup_peak_bytes"] = json!(fs::read_to_string("/sys/fs/cgroup/memory.peak")?
        .trim()
        .parse::<u64>()?);
    let (rss, cpu) = resource_usage();
    identity["peak_rss_bytes"] = json!(rss);
    identity["cpu_seconds"] = json!(cpu);
    phase("worker_returned", identity.clone())?;
    write_new_json(output, &identity)
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    #[test]
    fn watchdog_preserves_censor_when_resource_read_or_parse_fails() {
        let dir = std::env::temp_dir().join(format!(
            "n83-cap-resource-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let peak = dir.join("memory.peak");
        let identity = json!({"arm":"D","mode":"natural","target_index":7,
            "candidate_id":null,"selected_best_total_runtime":null});
        for contents in [None, Some("invalid\n"), Some("1024000\n")] {
            if let Some(s) = contents {
                fs::write(&peak, s).unwrap();
            }
            let cap = budget_receipt(identity.clone(), Instant::now(), &peak);
            assert_eq!(cap["status"], "UNKNOWN_budget");
            assert_eq!(cap["target_index"], 7);
            assert!(cap["candidate_id"].is_null());
            assert!(cap.get("rank").is_none());
            assert!(cap.get("counters").is_none());
            if contents == Some("1024000\n") {
                assert_eq!(cap["memory_cgroup_peak_bytes"], 1024000);
                assert!(cap["memory_cgroup_peak_error"].is_null());
            } else {
                assert!(cap["memory_cgroup_peak_bytes"].is_null());
                assert!(cap["memory_cgroup_peak_error"].is_string());
            }
            let output = dir.join(format!("cap-{}.json", contents.unwrap_or("absent").trim()));
            write_new_json(&output, &cap).unwrap();
            let retained: Value = serde_json::from_slice(&fs::read(output).unwrap()).unwrap();
            assert_eq!(retained, cap);
        }
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn full_policy_does_not_expand_old_commands_or_allow_off_natural_queries() {
        for arm in ["U", "D"] {
            for target in 0..8 {
                assert!(Policy::Full(true).check("natural", arm, target).is_ok());
            }
            assert!(Policy::Full(true).check("natural", arm, 8).is_err());
            assert!(Policy::Full(false).check("natural", arm, 0).is_err());
            assert!(Policy::Full(false).check("positive", arm, 0).is_ok());
            assert!(Policy::Full(true).check("positive", arm, 1).is_err());
            assert!(Policy::SmallCache(true).check("natural", arm, 0).is_err());
            assert!(Policy::Legacy.check("natural", arm, 0).is_err());
        }
        assert!(Policy::Full(true).check("natural", "D12", 0).is_err());
        assert!(Policy::SmallCache(true).check("natural", "D12", 1).is_ok());
        assert!(Policy::SmallCache(true).check("natural", "D12", 2).is_err());
        assert!(Policy::Legacy.check("natural", "D12", 1).is_err());
        assert!(cache_setting("default").is_err());
    }

    #[test]
    fn shared_root_policy_requires_explicit_encoding_and_full_d_natural_inputs() {
        for name in ["per-representative", "shared-bits"] {
            let encoding = root_setting(name).unwrap();
            let policy = Policy::SharedRoot(encoding);
            assert_eq!(policy.cache(), Some(true));
            assert_eq!(policy.root_encoding(), encoding);
            for target in 0..8 {
                assert!(policy.check("natural", "D", target).is_ok());
                assert!(policy.check("natural", "U", target).is_err());
            }
            for arm in ["U", "D"] {
                assert!(policy.check("positive", arm, 0).is_ok());
                assert!(policy.check("positive", arm, 1).is_err());
            }
            assert!(policy.check("natural", "D", 8).is_err());
            assert!(policy.check("natural", "D12", 0).is_err());
            assert!(policy.check("natural", "K2048", 0).is_err());
        }
        assert!(root_setting("default").is_err());
        assert!(root_setting("on").is_err());
        assert_eq!(
            Policy::Full(true).root_encoding(),
            FrobeniusOrbitRootEncoding::PerRepresentative
        );
    }
    #[test]
    fn full_control_gate_requires_model_work_and_cache_conservation() {
        let mut off = json!({"schema":"n83.small-m4-worker/v1","experiment_policy":"full-m4-common-public/v1",
            "status":"PASS_CONSTRUCTED_PENDING_REPLAY","mode":"positive","curve_id":CURVE,"arm":"U","m":4,"target_index":0,
            "conflict_budget":10000,"xor_decided_cache":false,"group_verified":true,"rank":1,"columns":16627,"points":2760082,
            "public_corpus_sha256":PUBLIC_SHA256,"sat_model_blake3":"0".repeat(64),"candidate_id":null,"selected_best_total_runtime":null,
            "counters":{"phase":"Complete","completed_xor_repivots":100},"xor_decided_skips":0});
        for k in [
            "source_commit",
            "binary_sha256",
            "source_module_blake3",
            "source_worker_blake3",
            "object_blake3",
            "point_set_blake3",
            "labeled_rows_blake3",
            "target",
            "sat_variables",
            "sat_clauses",
            "sat_xor_rows",
            "point_indices",
            "sparse_row",
            "sparse_row_blake3",
        ] {
            off[k] = json!("fixture");
        }
        for k in [
            "completed_conflicts",
            "completed_decisions",
            "completed_restarts",
            "completed_xor_passes",
            "completed_xor_row_ops",
            "completed_xor_row_scans",
        ] {
            off["counters"][k] = json!(0);
        }
        let mut on = off.clone();
        on["xor_decided_cache"] = json!(true);
        on["counters"]["completed_xor_repivots"] = json!(20);
        on["xor_decided_skips"] = json!(80);
        assert!(check_full_pair(&off, &on).is_ok());
        let mut bad = on.clone();
        bad["sat_model_blake3"] = json!("1".repeat(64));
        assert!(check_full_pair(&off, &bad).is_err());
        let mut bad = on.clone();
        bad["xor_decided_skips"] = json!(79);
        assert!(check_full_pair(&off, &bad).is_err());
        let mut bad = on;
        bad["counters"]["completed_conflicts"] = json!(1);
        assert!(check_full_pair(&off, &bad).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lift_checks_signs_membership_and_exact_target() {
        let (kc, fast) = curve().unwrap();
        let g = words(kc.generator(), &fast).unwrap();
        let mut rows = Vec::new();
        for (column, k) in [1u8, 3, 9, 27].into_iter().enumerate() {
            let p = fast.scalar_mul(Some(g), &BigUint::from(k)).unwrap();
            rows.push(Row {
                point: p,
                column,
                coefficient: 1u8.into(),
            });
            rows.push(Row {
                point: FastBinaryCurve128::neg(Some(p)).unwrap(),
                column,
                coefficient: &kc.subgroup_order - 1u8,
            });
        }
        let index = rows.iter().enumerate().map(|(i, r)| (r.point, i)).collect();
        let base = Base {
            rows,
            index,
            object_blake3: String::new(),
            point_set_blake3: String::new(),
        };
        let wanted = [0usize, 3, 4, 7];
        let target = wanted
            .iter()
            .fold(None, |sum, &i| fast.add(sum, Some(base.rows[i].point)))
            .unwrap();
        let xs: Vec<_> = wanted.iter().map(|&i| base.rows[i].point.0).collect();
        assert_eq!(lift(&fast, &base, &xs, target), Some(wanted.to_vec()));
        let wrong = fast.scalar_mul(Some(g), &100u8.into()).unwrap();
        assert!(lift(&fast, &base, &xs, wrong).is_none());
        assert!(lift(&fast, &base, &xs[..3], target).is_none());
    }
}
