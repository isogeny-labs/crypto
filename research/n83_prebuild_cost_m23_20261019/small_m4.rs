//! Frozen small-base m=4 gate; the parent owns the byte-pinned base importer.
use super::*;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crypto_lib::cryptanalysis::sat::{RootPropagationResult, SolveResult, SolverProgress};
use std::fs::File;
use std::sync::{mpsc, Arc};

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
    run_inner(mode, arm, root, corpus, target_index, output, None)
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
    let enabled = match cache {
        "off" => false,
        "on" => true,
        _ => return Err("cache setting must be off or on".into()),
    };
    run_inner(mode, arm, root, corpus, target_index, output, Some(enabled))
}

fn run_inner(
    mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    output: &Path,
    cache: Option<bool>,
) -> Result<()> {
    if !matches!(mode, "preflight" | "positive" | "natural")
        || !matches!(arm, "K2048" | "D12")
        || target_index > usize::from(cache.is_some())
    {
        return Err("small m4 request outside frozen protocol".into());
    }
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
        identity["experiment_policy"] = json!("small-m4-xor-cache/v1");
        identity["xor_decided_cache"] = json!(enabled);
        identity["conflict_budget"] = json!(conflict_budget);
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
    let mut cap = identity.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1790));
        cap["status"] = json!("UNKNOWN_budget");
        cap["elapsed_ms"] = json!(started.elapsed().as_secs_f64() * 1000.0);
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
    enc.constrain_frobenius_orbit_summands(&representatives, 300000, 8000000)
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
