// Included inside primary_adapter.rs so the mixed worker reuses checked imports,
// the exact group-lift verifier, solver progress and immutable receipt helpers.

const MIXED_OVERLAP_SHA256: &str =
    "032c5cbbf1a55b667ca885a9b7cd766fa86d2926ad1d5762eebf1843bc036b93";
const MIXED_OVERLAP_AUDIT_SHA256: &str =
    "1913836ff201eb234127a141ed4512beb31143121b1aa8a9b650e3059f68087d";
type MixedLoaded = (PrimaryBase, Value, String, &'static str, &'static str);

fn mixed_slot(shape: &str, slot: usize) -> Result<char> {
    if !["UUUU", "UUDD", "DDUU", "DDDD"].contains(&shape) || slot >= 4 {
        return Err("mixed shape or slot outside frozen protocol".into());
    }
    Ok(shape.as_bytes()[slot] as char)
}

fn mixed_base<'a>(
    u: &'a Option<MixedLoaded>,
    d: &'a Option<MixedLoaded>,
    arm: char,
) -> Result<&'a PrimaryBase> {
    match arm {
        'U' => Ok(&u.as_ref().ok_or("U base absent")?.0),
        'D' => Ok(&d.as_ref().ok_or("D base absent")?.0),
        _ => Err("mixed arm outside frozen protocol".into()),
    }
}

fn mixed_inputs(
    shape: &str,
    u_root: &Path,
    d_root: &Path,
    evidence_root: &Path,
) -> Result<(Option<MixedLoaded>, Option<MixedLoaded>)> {
    mixed_slot(shape, 0)?;
    if file_sha256(&evidence_root.join("overlap-v1/overlap.json"))? != MIXED_OVERLAP_SHA256
        || file_sha256(&evidence_root.join("overlap-v1/audit.json"))? != MIXED_OVERLAP_AUDIT_SHA256
    {
        return Err("mixed overlap or independent audit receipt differs".into());
    }
    let overlap: Value =
        serde_json::from_slice(&fs::read(evidence_root.join("overlap-v1/overlap.json"))?)?;
    let audit: Value =
        serde_json::from_slice(&fs::read(evidence_root.join("overlap-v1/audit.json"))?)?;
    if overlap["shared_orbits"] != 0
        || overlap["union_orbits"] != 33_254
        || overlap["candidate_id"] != Value::Null
        || audit["status"] != "PASS"
        || audit["shared_orbits_by_full_point_digest"] != 0
    {
        return Err("mixed overlap evidence failed admission".into());
    }
    let u = if shape.contains('U') {
        Some(load_paired_base("U", u_root, evidence_root)?)
    } else {
        None
    };
    let d = if shape.contains('D') {
        Some(load_paired_base("D", d_root, evidence_root)?)
    } else {
        None
    };
    if let (Some(uu), Some(dd)) = (&u, &d) {
        let a = &uu.0.curve;
        let b = &dd.0.curve;
        if a.n != b.n
            || a.subgroup_order != b.subgroup_order
            || a.curve.irreducible.degree != b.curve.irreducible.degree
            || a.curve.irreducible.low_terms != b.curve.irreducible.low_terms
            || a.curve.a != b.curve.a
            || a.curve.b != b.curve.b
            || a.generator() != b.generator()
        {
            return Err("mixed bases have different curve identities".into());
        }
    }
    Ok((u, d))
}

fn mixed_source() -> Result<(String, &'static str)> {
    let (commit, mode) = frozen_chain_source()?;
    let root = std::env::var_os("ICV1_FROZEN_SOURCE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let path = root.join("research/n83_d18_subgroup_orbit_census_20261018/mixed-v1/worker.rs");
    if blake3::hash(&fs::read(path)?) != blake3::hash(MIXED_WORKER_SOURCE) {
        return Err("compiled mixed worker differs from frozen source".into());
    }
    Ok((commit, mode))
}

fn mixed_representatives(base: &PrimaryBase) -> Result<Vec<BigUint>> {
    base.factor_base
        .signed_orbits
        .iter()
        .map(|orbit| match &base.factor_base.points[orbit[0]] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("mixed orbit representative is infinity".into()),
        })
        .collect()
}

fn mixed_constructed_indices(
    shape: &str,
    u: &Option<MixedLoaded>,
    d: &Option<MixedLoaded>,
) -> Result<[usize; 4]> {
    let mut u_col = 0;
    let mut d_col = 0;
    let mut indices = [0; 4];
    for (slot, index) in indices.iter_mut().enumerate() {
        let arm = mixed_slot(shape, slot)?;
        let base = mixed_base(u, d, arm)?;
        let col = if arm == 'U' {
            let result = u_col;
            u_col += 1;
            result
        } else {
            let result = d_col;
            d_col += 1;
            result
        };
        *index = *base
            .factor_base
            .signed_orbits
            .get(col)
            .and_then(|orbit| orbit.first())
            .ok_or("mixed positive representative absent")?;
    }
    let first = mixed_base(u, d, mixed_slot(shape, 0)?)?;
    let x = |index: usize| -> Result<BigUint> {
        match &first.factor_base.points[index] {
            BinaryPoint::Affine { x, .. } => Ok(x.to_biguint()),
            BinaryPoint::Infinity => Err("mixed positive first pair has infinity".into()),
        }
    };
    if x(indices[0])? > x(indices[1])? {
        indices.swap(0, 1);
    }
    Ok(indices)
}

fn mixed_target_for_indices(
    shape: &str,
    u: &Option<MixedLoaded>,
    d: &Option<MixedLoaded>,
    indices: &[usize; 4],
) -> Result<(BinaryPoint, Vec<F2mElement>, Vec<BinaryPoint>)> {
    let curve = &mixed_base(u, d, mixed_slot(shape, 0)?)?.curve;
    let mut sum = BinaryPoint::Infinity;
    let mut xs = Vec::with_capacity(4);
    let mut partial = Vec::with_capacity(4);
    for (slot, &index) in indices.iter().enumerate() {
        let base = mixed_base(u, d, mixed_slot(shape, slot)?)?;
        let point = base
            .factor_base
            .points
            .get(index)
            .ok_or("mixed positive point index outside base")?;
        let BinaryPoint::Affine { x, .. } = point else {
            return Err("mixed positive point is infinity".into());
        };
        xs.push(x.clone());
        sum = curve.add(&sum, point);
        partial.push(sum.clone());
    }
    if partial
        .iter()
        .any(|point| !matches!(point, BinaryPoint::Affine { .. }))
        || xs
            .iter()
            .map(F2mElement::to_biguint)
            .collect::<HashSet<_>>()
            .len()
            != 4
    {
        return Err("mixed positive partials are not affine and distinct".into());
    }
    Ok((sum, xs, partial))
}

fn checked_mixed_row(
    shape: &str,
    u: &Option<MixedLoaded>,
    d: &Option<MixedLoaded>,
    indices: &[usize; 4],
    target: &BinaryPoint,
) -> Result<Vec<BigUint>> {
    let curve = &mixed_base(u, d, mixed_slot(shape, 0)?)?.curve;
    let mut u_indices = Vec::new();
    let mut d_indices = Vec::new();
    let mut u_sum = BinaryPoint::Infinity;
    let mut d_sum = BinaryPoint::Infinity;
    for (slot, &index) in indices.iter().enumerate() {
        let arm = mixed_slot(shape, slot)?;
        let base = mixed_base(u, d, arm)?;
        let point = base
            .factor_base
            .points
            .get(index)
            .ok_or("mixed relation point index outside slot base")?;
        if arm == 'U' {
            u_indices.push(index);
            u_sum = curve.add(&u_sum, point);
        } else {
            d_indices.push(index);
            d_sum = curve.add(&d_sum, point);
        }
    }
    if curve.add(&u_sum, &d_sum) != *target {
        return Err("mixed four-point group sum differs from target".into());
    }
    let mut row = Vec::with_capacity(if u.is_some() && d.is_some() {
        33_254
    } else {
        16_627
    });
    for (loaded, local_indices, partial) in [(u, &u_indices, &u_sum), (d, &d_indices, &d_sum)] {
        if let Some(base) = loaded {
            row.extend(checked_primary_relation_row(
                curve,
                &base.0.factor_base,
                local_indices,
                partial,
            )?);
        }
    }
    let mut replay = BinaryPoint::Infinity;
    let mut offset = 0;
    for loaded in [u, d].into_iter().flatten() {
        let fb = &loaded.0.factor_base;
        for (column, coefficient) in row[offset..offset + fb.unknowns()].iter().enumerate() {
            if !coefficient.is_zero() {
                let rep = &fb.points[fb.signed_orbits[column][0]];
                replay = curve.add(&replay, &curve.mul(rep, coefficient));
            }
        }
        offset += fb.unknowns();
    }
    if offset != row.len() || replay != *target {
        return Err("mixed concatenated row failed independent group replay".into());
    }
    Ok(row)
}

fn checked_mixed_four_x_lift(
    shape: &str,
    u: &Option<MixedLoaded>,
    d: &Option<MixedLoaded>,
    xs: &[F2mElement],
    target: &BinaryPoint,
) -> Result<Option<([usize; 4], Vec<BigUint>)>> {
    if xs.len() != 4 {
        return Err("mixed four-summand model has wrong x count".into());
    }
    let mut choices: [Vec<usize>; 4] = std::array::from_fn(|_| Vec::new());
    for slot in 0..4 {
        let fb = &mixed_base(u, d, mixed_slot(shape, slot)?)?.factor_base;
        for (index, point) in fb.points.iter().enumerate() {
            if let BinaryPoint::Affine { x, .. } = point {
                if x == &xs[slot] {
                    choices[slot].push(index);
                    if choices[slot].len() > 2 {
                        return Err("mixed slot has more than two x lifts".into());
                    }
                }
            }
        }
    }
    if choices.iter().any(Vec::is_empty) {
        return Ok(None);
    }
    for &a in &choices[0] {
        for &b in &choices[1] {
            for &c in &choices[2] {
                for &d_idx in &choices[3] {
                    let indices = [a, b, c, d_idx];
                    let curve = &mixed_base(u, d, mixed_slot(shape, 0)?)?.curve;
                    let mut sum = BinaryPoint::Infinity;
                    for (slot, &index) in indices.iter().enumerate() {
                        let point = &mixed_base(u, d, mixed_slot(shape, slot)?)?
                            .factor_base
                            .points[index];
                        sum = curve.add(&sum, point);
                    }
                    if sum == *target {
                        let row = checked_mixed_row(shape, u, d, &indices, target)?;
                        return Ok(Some((indices, row)));
                    }
                }
            }
        }
    }
    Ok(None)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn mixed_orbit_cli(
    mode: &str,
    shape: &str,
    fixture_index: u64,
    u_root: &Path,
    d_root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    output: &Path,
) -> Result<Value> {
    if !["preflight", "positive", "natural"].contains(&mode) || fixture_index > 1 {
        return Err("mixed mode or fixture outside frozen protocol".into());
    }
    mixed_slot(shape, 0)?;
    if output.exists() {
        return Err("mixed output already exists".into());
    }
    let memory_limit_bytes = checked_cgroup_memory_limit(8192)?;
    let (source_commit, source_attestation_mode) = mixed_source()?;
    let binary_sha256 = file_sha256(&std::env::current_exe()?)?;
    let started = Instant::now();
    let cap_path = PathBuf::from(format!("{}.cap.json", output.display()));
    let cap_shape = shape.to_owned();
    let cap_mode = mode.to_owned();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(600));
        let memory_cgroup_peak_bytes = match cgroup_v2_file("memory.peak")
            .ok()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|value| value.trim().parse::<u64>().ok())
        {
            Some(value) => value,
            None => std::process::exit(125),
        };
        let cap = json!({
            "schema":"n83.mixed-orbit-m4-cap/v1", "status":"UNKNOWN_budget",
            "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
            "process_usage":process_usage(),
            "shape":cap_shape, "mode":cap_mode,
            "summands":4, "fixture_index":fixture_index,
            "budget_seconds":600,
            "elapsed_process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "candidate_id":Value::Null,
            "selected_best_total_runtime":Value::Null
        });
        if write_new_json(&cap_path, &cap).is_err() {
            std::process::exit(125);
        }
        std::process::exit(124);
    });
    let import_start = Instant::now();
    let (u, d) = mixed_inputs(shape, u_root, d_root, evidence_root)?;
    let import_ms = import_start.elapsed().as_secs_f64() * 1000.0;
    let curve = &mixed_base(&u, &d, mixed_slot(shape, 0)?)?.curve;
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256 {
        return Err("mixed public corpus differs".into());
    }
    let target_start = Instant::now();
    let (public, corpus_hash) = public_target_fixture(fixture_root, curve, fixture_index)?;
    let target_validation_ms = target_start.elapsed().as_secs_f64() * 1000.0;
    let expected_public = match fixture_index {
        0 => json!({
            "x_decimal":"1834913121178197242856238",
            "y_decimal":"8797964190732846213981377"
        }),
        1 => json!({
            "x_decimal":"5809312440517013698248638",
            "y_decimal":"3568442427236746854022671"
        }),
        _ => return Err("mixed public fixture outside frozen target pair".into()),
    };
    if planted_point_json(&public) != expected_public {
        return Err("mixed public fixture point differs".into());
    }
    let identities = json!({
        "U":u.as_ref().map(|loaded| json!({
            "object_blake3":loaded.3, "point_set_blake3":loaded.4,
            "manifest_blake3":loaded.2, "policy":loaded.1["policy"],
            "seed":loaded.1["seed"], "columns":loaded.0.factor_base.unknowns(),
            "points":loaded.0.factor_base.points.len(),
        })),
        "D":d.as_ref().map(|loaded| json!({
            "object_blake3":loaded.3, "point_set_blake3":loaded.4,
            "manifest_blake3":loaded.2, "policy":loaded.1["policy"],
            "seed":loaded.1["seed"], "columns":loaded.0.factor_base.unknowns(),
            "points":loaded.0.factor_base.points.len(),
        })),
    });
    for loaded in [u.as_ref(), d.as_ref()].into_iter().flatten() {
        if loaded.0.factor_base.unknowns() != 16_627
            || loaded.0.factor_base.points.len() != 2_760_082
        {
            return Err("mixed factor-base dimensions differ".into());
        }
    }
    let prepare_start = Instant::now();
    let u_reps = u
        .as_ref()
        .map(|base| mixed_representatives(&base.0))
        .transpose()?;
    let d_reps = d
        .as_ref()
        .map(|base| mixed_representatives(&base.0))
        .transpose()?;
    let domains: Vec<Vec<BigUint>> = (0..4)
        .map(|slot| match mixed_slot(shape, slot)? {
            'U' => Ok(u_reps.as_ref().ok_or("mixed U reps absent")?.clone()),
            'D' => Ok(d_reps.as_ref().ok_or("mixed D reps absent")?.clone()),
            _ => Err("mixed slot outside frozen domain".into()),
        })
        .collect::<Result<_>>()?;
    let positive_indices = if mode == "positive" {
        Some(mixed_constructed_indices(shape, &u, &d)?)
    } else {
        None
    };
    let mut positive_xs = None;
    let mut positive_row = None;
    let target = if let Some(indices) = positive_indices {
        let (target, xs, partial) = mixed_target_for_indices(shape, &u, &d, &indices)?;
        let row = checked_mixed_row(shape, &u, &d, &indices, &target)?;
        let mut rank = WideRankTracker::new(&curve.subgroup_order, row.len());
        let tamper = curve.add(&target, curve.generator());
        if rank.insert(row.clone()) != 1
            || checked_mixed_row(shape, &u, &d, &indices, &tamper).is_ok()
        {
            return Err("mixed positive group/rank/tamper preflight failed".into());
        }
        positive_xs = Some((xs, partial));
        positive_row = Some(row);
        target
    } else {
        public.clone()
    };
    let BinaryPoint::Affine { x: target_x, .. } = &target else {
        return Err("mixed target is infinity".into());
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    let preparation_ms = prepare_start.elapsed().as_secs_f64() * 1000.0;
    let first_loaded = if mixed_slot(shape, 0)? == 'U' {
        u.as_ref().ok_or("mixed first U absent")?
    } else {
        d.as_ref().ok_or("mixed first D absent")?
    };
    let mut phases = WitnessPhases::create(
        output,
        started,
        source_commit.clone(),
        "n83.mixed-orbit-m4-phase/v1",
        first_loaded.3,
        first_loaded.4,
    )?;
    phases.mark(
        "inputs_checked",
        json!({
            "shape":shape, "mode":mode, "fixture_index":fixture_index,
            "binary_sha256":binary_sha256, "identities":identities,
            "overlap_sha256":MIXED_OVERLAP_SHA256,
            "overlap_audit_sha256":MIXED_OVERLAP_AUDIT_SHA256,
            "target_blake3":target_hash, "public_corpus_blake3":corpus_hash,
            "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
            "domain_preparation_ms":preparation_ms,
        }),
    )?;
    let model_start = Instant::now();
    let mut enc = ChainedS3Encoding::build(
        4,
        &curve.curve.irreducible,
        &curve.curve.b,
        Some(target_x),
        0,
        300_000,
    )
    .map_err(|_| "mixed chained-S3 model build failed")?;
    enc.constrain_frobenius_orbit_domains(&domains, 300_000, 8_000_000)
        .map_err(|_| "mixed per-slot orbit-domain install failed")?;
    enc.order_first_pair(300_000)
        .map_err(|_| "mixed first-pair symmetry install failed")?;
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "model_domain_done",
        json!({
            "model_ms":model_ms, "variables":enc.solver.n_vars(),
            "clauses":enc.solver.n_clauses(), "xor_rows":enc.solver.n_xors(),
            "ordered_first_pair":true,
        }),
    )?;
    if mode == "preflight" {
        let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
            .trim()
            .parse()?;
        let receipt = json!({
            "schema":"n83.mixed-orbit-m4-worker/v1", "status":"PASS_PREFLIGHT",
            "shape":shape, "mode":mode, "summands":4, "fixture_index":fixture_index,
            "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
            "source_adapter_blake3":blake3::hash(MIXED_ADAPTER_SOURCE).to_hex().to_string(),
            "source_example_blake3":blake3::hash(MIXED_EXAMPLE_SOURCE).to_hex().to_string(),
            "source_mixed_worker_blake3":blake3::hash(MIXED_WORKER_SOURCE).to_hex().to_string(),
            "binary_sha256":binary_sha256, "identities":identities,
            "overlap_sha256":MIXED_OVERLAP_SHA256,
            "overlap_audit_sha256":MIXED_OVERLAP_AUDIT_SHA256,
            "public_corpus_blake3":corpus_hash, "public_point":planted_point_json(&public),
            "target_blake3":target_hash, "target":planted_point_json(&target),
            "columns":if u.is_some() && d.is_some() { 33_254 } else { 16_627 },
            "memory_cgroup_limit_bytes":memory_limit_bytes,
            "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
            "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
            "domain_preparation_ms":preparation_ms, "model_domain_ms":model_ms,
            "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
            "sat_xor_rows":enc.solver.n_xors(), "ordered_first_pair":true,
            "process_usage":process_usage(),
            "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
            "solver_stage_executed":false,
            "candidate_id":Value::Null,
            "total_index_calculus_runtime_ms":Value::Null,
            "selected_best_total_runtime":Value::Null,
        });
        write_new_json(output, &receipt)?;
        return Ok(receipt);
    }
    let pin_start = Instant::now();
    if let Some((xs, partial)) = &positive_xs {
        let mut pinned = xs.clone();
        for point in &partial[1..3] {
            let BinaryPoint::Affine { x, .. } = point else {
                return Err("mixed positive partial sum is infinity".into());
            };
            pinned.push(x.clone());
        }
        let pinned = [
            pinned[0].clone(),
            pinned[1].clone(),
            pinned[2].clone(),
            pinned[3].clone(),
            pinned[4].clone(),
            pinned[5].clone(),
            target_x.clone(),
        ];
        for (block, x) in pinned.iter().enumerate() {
            enc.pin_x_block(block, x)
                .map_err(|_| "mixed positive x pin failed")?;
        }
        let handles = enc
            .orbit_choice_wires()
            .ok_or("mixed positive selectors absent")?;
        if handles.len() != 4 {
            return Err("mixed selector handle count differs".into());
        }
        let mut pins = Vec::with_capacity(4);
        for (slot, x) in xs.iter().enumerate() {
            let mut sorted = domains[slot].clone();
            sorted.sort_unstable();
            let position = sorted
                .binary_search(&x.to_biguint())
                .map_err(|_| "mixed positive representative missing")?;
            let handle = &handles[slot];
            if handle.selectors.len() != sorted.len() || handle.phase.len() != 7 {
                return Err("mixed positive selector/phase width differs".into());
            }
            pins.push((handle.selectors[position], handle.phase.clone()));
        }
        for (selector, phases) in pins {
            if !enc.solver.add_clause(vec![selector as i32]) {
                return Err("mixed positive selector pin failed".into());
            }
            for wire in phases {
                if !enc.solver.add_clause(vec![-(wire as i32)]) {
                    return Err("mixed positive phase-zero pin failed".into());
                }
            }
        }
        phases.mark(
            "positive_choices_pinned",
            json!({"selectors":4,"phases":28}),
        )?;
    }
    let positive_pin_ms = if mode == "positive" {
        pin_start.elapsed().as_secs_f64() * 1000.0
    } else {
        0.0
    };
    enc.solver.conflict_budget = 100_000;
    enc.solver.deadline = Some(started + Duration::from_secs(600));
    phases.mark(
        "root_propagation_entered",
        json!({"stats":root_phase_stats(&enc.solver.stats)}),
    )?;
    let root_start = Instant::now();
    let root_result = enc.solver.propagate_root();
    let root_ms = root_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "root_propagation_returned",
        json!({
            "result":format!("{root_result:?}"), "root_ms":root_ms,
            "stats":root_phase_stats(&enc.solver.stats),
        }),
    )?;
    let search_start = Instant::now();
    let answer = if root_result == RootPropagationResult::Unsat {
        SolveResult::Unsat
    } else {
        phases.mark(
            "sat_entered",
            json!({"conflict_budget":100_000,"max_models":1}),
        )?;
        let progress = Arc::new(SolverProgress::default());
        enc.solver.set_progress_observer(Arc::clone(&progress));
        let sampler = ProgressSampler::start(
            output,
            started,
            "n83.mixed-orbit-m4-progress/v1",
            source_commit.clone(),
            first_loaded.3,
            first_loaded.4,
            progress,
        )?;
        let result = enc.solver.solve();
        sampler.finish()?;
        result
    };
    let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
    phases.mark(
        "sat_returned",
        json!({
            "answer":format!("{answer:?}"), "search_ms":search_ms,
            "conflicts":enc.solver.conflicts(),
        }),
    )?;
    let mut relation = Value::Null;
    let mut relation_check_ms = 0.0;
    let mut rank_ms = 0.0;
    let status = match answer {
        SolveResult::Unknown => "UNKNOWN_search_cap",
        SolveResult::Unsat if mode == "positive" => "INVALID_POSITIVE_CONTROL",
        SolveResult::Unsat => "UNVERIFIED_MODEL_UNSAT",
        SolveResult::Sat => {
            let checked = Instant::now();
            let xs = enc.decode_x_blocks();
            let result = if !enc.verify_model() || xs.len() != 7 || xs[6] != *target_x {
                "INVALID_MODEL"
            } else if let Some((indices, row)) =
                checked_mixed_four_x_lift(shape, &u, &d, &xs[..4], &target)?
            {
                let rank_start = Instant::now();
                let mut rank = WideRankTracker::new(&curve.subgroup_order, row.len());
                let rank_after_insert = rank.insert(row.clone());
                rank_ms = rank_start.elapsed().as_secs_f64() * 1000.0;
                let changed_target = curve.add(&target, curve.generator());
                let tamper_rejected =
                    checked_mixed_row(shape, &u, &d, &indices, &changed_target).is_err();
                let dense: Vec<_> = row.iter().map(ToString::to_string).collect();
                relation = json!({
                    "point_indices":indices,
                    "dense_decimal_blake3":blake3::hash(&serde_json::to_vec(&dense)?).to_hex().to_string(),
                    "rank_after_insert":rank_after_insert,
                    "group_verified":true, "tamper_rejected":tamper_rejected,
                    "matches_constructed_row":positive_row.as_ref().map(|prior| prior == &row),
                    "matches_constructed_indices":positive_indices.map(|prior| prior == indices),
                });
                phases.mark("relation_checked", relation.clone())?;
                if rank_after_insert != 1 || !tamper_rejected {
                    "INVALID_RELATION"
                } else if mode == "positive" && positive_row.as_ref() != Some(&row) {
                    "INVALID_POSITIVE_CONTROL"
                } else if mode == "positive" {
                    "PASS_CONSTRUCTED_CONTROL"
                } else {
                    "PASS_NATURAL_RELATION_PENDING_REPLAY"
                }
            } else {
                "INVALID_GROUP_LIFT"
            };
            relation_check_ms = checked.elapsed().as_secs_f64() * 1000.0 - rank_ms;
            result
        }
    };
    let memory_cgroup_peak_bytes: u64 = fs::read_to_string(cgroup_v2_file("memory.peak")?)?
        .trim()
        .parse()?;
    let receipt = json!({
        "schema":"n83.mixed-orbit-m4-worker/v1", "status":status,
        "shape":shape, "mode":mode, "summands":4, "fixture_index":fixture_index,
        "source_commit":source_commit, "source_attestation_mode":source_attestation_mode,
        "source_adapter_blake3":blake3::hash(MIXED_ADAPTER_SOURCE).to_hex().to_string(),
        "source_example_blake3":blake3::hash(MIXED_EXAMPLE_SOURCE).to_hex().to_string(),
        "source_mixed_worker_blake3":blake3::hash(MIXED_WORKER_SOURCE).to_hex().to_string(),
        "binary_sha256":binary_sha256, "identities":identities,
        "overlap_sha256":MIXED_OVERLAP_SHA256,
        "overlap_audit_sha256":MIXED_OVERLAP_AUDIT_SHA256,
        "public_corpus_blake3":corpus_hash, "target_blake3":target_hash,
        "target":planted_point_json(&target),
        "columns":if u.is_some() && d.is_some() { 33_254 } else { 16_627 },
        "base_import_ms":import_ms, "target_validation_ms":target_validation_ms,
        "domain_preparation_ms":preparation_ms, "model_domain_ms":model_ms,
        "positive_pin_ms":positive_pin_ms, "root_propagation_ms":root_ms,
        "search_ms":search_ms, "relation_check_ms":relation_check_ms,
        "rank_ms":rank_ms, "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "process_usage":process_usage(), "memory_cgroup_limit_bytes":memory_limit_bytes,
        "memory_cgroup_peak_bytes":memory_cgroup_peak_bytes,
        "sat_variables":enc.solver.n_vars(), "sat_clauses":enc.solver.n_clauses(),
        "sat_xor_rows":enc.solver.n_xors(), "ordered_first_pair":true,
        "relation":relation,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn mixed_relation_replay_cli(
    shape: &str,
    u_root: &Path,
    d_root: &Path,
    evidence_root: &Path,
    fixture_root: &Path,
    worker_path: &Path,
    output: &Path,
) -> Result<Value> {
    mixed_slot(shape, 0)?;
    if output.exists() {
        return Err("mixed replay output already exists".into());
    }
    let (source_commit, source_attestation_mode) = mixed_source()?;
    let started = Instant::now();
    let (u, d) = mixed_inputs(shape, u_root, d_root, evidence_root)?;
    let curve = &mixed_base(&u, &d, mixed_slot(shape, 0)?)?.curve;
    let worker: Value = serde_json::from_slice(&fs::read(worker_path)?)?;
    let mode = worker["mode"].as_str().ok_or("mixed worker mode absent")?;
    let fixture_index = worker["fixture_index"]
        .as_u64()
        .ok_or("mixed worker fixture index absent")?;
    if !["positive", "natural"].contains(&mode)
        || worker["schema"] != "n83.mixed-orbit-m4-worker/v1"
        || worker["summands"] != 4
        || worker["shape"] != shape
        || fixture_index > 1
        || worker["status"]
            != if mode == "positive" {
                "PASS_CONSTRUCTED_CONTROL"
            } else {
                "PASS_NATURAL_RELATION_PENDING_REPLAY"
            }
        || worker["source_commit"]
            .as_str()
            .is_none_or(|value| value.len() != 40)
        || worker["source_adapter_blake3"]
            != blake3::hash(MIXED_ADAPTER_SOURCE).to_hex().to_string()
        || worker["source_example_blake3"]
            != blake3::hash(MIXED_EXAMPLE_SOURCE).to_hex().to_string()
        || worker["source_mixed_worker_blake3"]
            != blake3::hash(MIXED_WORKER_SOURCE).to_hex().to_string()
        || worker["overlap_sha256"] != MIXED_OVERLAP_SHA256
        || worker["overlap_audit_sha256"] != MIXED_OVERLAP_AUDIT_SHA256
        || worker["candidate_id"] != Value::Null
        || worker["selected_best_total_runtime"] != Value::Null
    {
        return Err("mixed worker identity or success receipt differs".into());
    }
    for (arm, loaded) in [("U", &u), ("D", &d)] {
        let row = &worker["identities"][arm];
        if let Some(loaded) = loaded {
            if row["object_blake3"] != loaded.3
                || row["point_set_blake3"] != loaded.4
                || row["manifest_blake3"] != loaded.2
                || row["policy"] != loaded.1["policy"]
                || row["seed"] != loaded.1["seed"]
                || row["columns"] != loaded.0.factor_base.unknowns()
                || row["points"] != loaded.0.factor_base.points.len()
            {
                return Err("mixed worker factor-base identity differs".into());
            }
        } else if !row.is_null() {
            return Err("mixed worker includes an unused base".into());
        }
    }
    if file_sha256(&fixture_root.join("probe-corpus.json"))? != PAIRED_CORPUS_SHA256 {
        return Err("mixed replay public corpus differs".into());
    }
    let (public, corpus_hash) = public_target_fixture(fixture_root, curve, fixture_index)?;
    if worker["public_corpus_blake3"] != corpus_hash {
        return Err("mixed replay public corpus digest differs".into());
    }
    let target = if mode == "positive" {
        let indices = mixed_constructed_indices(shape, &u, &d)?;
        mixed_target_for_indices(shape, &u, &d, &indices)?.0
    } else {
        public
    };
    let target_hash = blake3::hash(&serde_json::to_vec(&planted_point_json(&target))?)
        .to_hex()
        .to_string();
    if worker["target"] != planted_point_json(&target) || worker["target_blake3"] != target_hash {
        return Err("mixed replay target differs".into());
    }
    let stored = worker["relation"]["point_indices"]
        .as_array()
        .ok_or("mixed point indices absent")?;
    if stored.len() != 4 {
        return Err("mixed point count differs".into());
    }
    let mut indices = [0usize; 4];
    for (slot, value) in stored.iter().enumerate() {
        indices[slot] = usize::try_from(value.as_u64().ok_or("mixed point index absent")?)?;
        let fb = &mixed_base(&u, &d, mixed_slot(shape, slot)?)?.factor_base;
        if indices[slot] >= fb.points.len() {
            return Err("mixed point index outside slot base".into());
        }
    }
    if mode == "positive" && indices != mixed_constructed_indices(shape, &u, &d)? {
        return Err("mixed positive indices differ from pinned representatives".into());
    }
    let checked = Instant::now();
    let dense = checked_mixed_row(shape, &u, &d, &indices, &target)?;
    let dense_strings: Vec<_> = dense.iter().map(ToString::to_string).collect();
    let dense_hash = blake3::hash(&serde_json::to_vec(&dense_strings)?)
        .to_hex()
        .to_string();
    let mut rank = WideRankTracker::new(&curve.subgroup_order, dense.len());
    let rank_after_insert = rank.insert(dense.clone());
    let mut changed_indices = indices;
    changed_indices[0] ^= 1;
    let changed_point_rejected =
        checked_mixed_row(shape, &u, &d, &changed_indices, &target).is_err();
    let mut changed_row = dense;
    changed_row[0] = (&changed_row[0] + BigUint::one()) % &curve.subgroup_order;
    let changed_strings: Vec<_> = changed_row.iter().map(ToString::to_string).collect();
    let changed_row_rejected = blake3::hash(&serde_json::to_vec(&changed_strings)?)
        .to_hex()
        .as_str()
        != worker["relation"]["dense_decimal_blake3"]
            .as_str()
            .ok_or("mixed dense row digest absent")?;
    if worker["relation"]["dense_decimal_blake3"] != dense_hash
        || worker["relation"]["rank_after_insert"] != 1
        || rank_after_insert != 1
        || !changed_point_rejected
        || !changed_row_rejected
    {
        return Err("mixed relation, rank or tamper replay failed".into());
    }
    let receipt = json!({
        "schema":"n83.mixed-orbit-m4-replay/v1", "status":"PASS",
        "shape":shape, "mode":mode, "summands":4, "fixture_index":fixture_index,
        "replay_source_commit":source_commit,
        "worker_source_commit":worker["source_commit"],
        "source_attestation_mode":source_attestation_mode,
        "worker_sha256":file_sha256(worker_path)?,
        "overlap_sha256":MIXED_OVERLAP_SHA256,
        "overlap_audit_sha256":MIXED_OVERLAP_AUDIT_SHA256,
        "target_blake3":target_hash, "point_indices":indices,
        "dense_decimal_blake3":dense_hash, "rank_after_insert":rank_after_insert,
        "changed_point_rejected":changed_point_rejected,
        "changed_row_rejected":changed_row_rejected,
        "relation_replay_ms":checked.elapsed().as_secs_f64()*1000.0,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "candidate_id":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    write_new_json(output, &receipt)?;
    Ok(receipt)
}

#[cfg(test)]
mod mixed_tests {
    use super::super::construct;
    use super::*;

    #[test]
    fn mixed_four_point_row_replays_across_both_column_blocks() {
        let deadline = Instant::now() + Duration::from_secs(60);
        let (u_bytes, u_row) = construct(0, "public_x_hash", 2, 17, deadline).unwrap();
        let (d_bytes, d_row) = construct(0, "public_x_hash", 2, 19, deadline).unwrap();
        let u = Some((
            from_record(&u_row, &u_bytes).unwrap(),
            Value::Null,
            String::new(),
            "",
            "",
        ));
        let d = Some((
            from_record(&d_row, &d_bytes).unwrap(),
            Value::Null,
            String::new(),
            "",
            "",
        ));
        for shape in ["UUDD", "DDUU"] {
            let indices = mixed_constructed_indices(shape, &u, &d).unwrap();
            let (target, xs, _) = mixed_target_for_indices(shape, &u, &d, &indices).unwrap();
            let row = checked_mixed_row(shape, &u, &d, &indices, &target).unwrap();
            assert_eq!(row.len(), 4);
            assert!(row[..2].iter().any(|v| !v.is_zero()));
            assert!(row[2..].iter().any(|v| !v.is_zero()));
            let (_, lifted) = checked_mixed_four_x_lift(shape, &u, &d, &xs, &target)
                .unwrap()
                .expect("constructed mixed relation has a group lift");
            assert_eq!(lifted, row);
            let curve = &u.as_ref().unwrap().0.curve;
            assert!(checked_mixed_row(
                shape,
                &u,
                &d,
                &indices,
                &curve.add(&target, curve.generator()),
            )
            .is_err());
        }
    }
}
