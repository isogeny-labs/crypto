//! Actual-root paid consumer screen; all prior records/protocols remain frozen.
use super::sparse_capture::{failed_native_work, fused_sample, paid_sum, Context};
use super::*;
const ROUTES: [&str; 15] = [
    "regeneration-external",
    "f4-regeneration",
    "expanded-regeneration",
    "compact-regeneration",
    "legacy-compact-replay",
    "prior-fused-sparse",
    "root-cas-baseline",
    "root-consumer-only",
    "root-native-only",
    "root-full-projection",
    "root-expanded",
    "rust-std",
    "rust-slimgb",
    "rust-msolve-lift",
    "root-sparse-both",
];
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Paths {
    worker: PathBuf,
    legacy_cli: PathBuf,
    prior_fused: PathBuf,
    root_baseline: PathBuf,
    root_consumer: PathBuf,
    root_native: PathBuf,
    root_both: PathBuf,
    #[serde(default)]
    m31: Option<PathBuf>,
    #[serde(default)]
    export_both: Option<PathBuf>,
    #[serde(default)]
    export_final: Option<PathBuf>,
    #[serde(default)]
    export_rust_only: Option<PathBuf>,
    #[serde(default)]
    kernel_legacy: Option<PathBuf>,
    #[serde(default)]
    kernel_ordered: Option<PathBuf>,
    #[serde(default)]
    kernel_cached: Option<PathBuf>,
    #[serde(default)]
    kernel_both: Option<PathBuf>,
    #[serde(default)]
    update_baseline: Option<PathBuf>,
    #[serde(default)]
    update_in_place: Option<PathBuf>,
    #[serde(default)]
    update_fastsum: Option<PathBuf>,
    #[serde(default)]
    lead_baseline: Option<PathBuf>,
    #[serde(default)]
    lead_cached: Option<PathBuf>,
    #[serde(default)]
    fused_baseline: Option<PathBuf>,
    #[serde(default)]
    fused_direct: Option<PathBuf>,
    #[serde(default)]
    frontier_baseline: Option<PathBuf>,
    #[serde(default)]
    frontier_direct: Option<PathBuf>,
    mature: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
}
impl Paths {
    fn replay_routes(&self) -> [(&Path, &str); 10] {
        [
            (&self.prior_fused, "compact-sparse"),
            (&self.root_baseline, "compact-sparse"),
            (&self.root_consumer, "compact-sparse"),
            (&self.root_native, "compact-sparse"),
            (&self.root_both, "compact"),
            (&self.root_both, "expanded"),
            (&self.mature, "std"),
            (&self.mature, "slimgb"),
            (&self.mature, "msolve-lift"),
            (&self.root_both, "compact-sparse"),
        ]
    }
}
fn comparisons_for(samples: &[Value], routes: &[&str]) -> Option<Vec<Value>> {
    comparisons_at(samples, routes, routes.len().checked_sub(1)?)
}
fn comparisons_at(samples: &[Value], routes: &[&str], at: usize) -> Option<Vec<Value>> {
    if samples.len() != routes.len() {
        return None;
    }
    let digest = samples[0]["basis_sha256"].as_str()?;
    if digest.len() != 64 {
        return None;
    }
    let mut costs = Vec::new();
    for (s, name) in samples.iter().zip(routes) {
        let cost = s["paid_wall_ns"].as_u64()?;
        if s["pipeline"] != *name
            || s["status"] != "complete"
            || s["basis_sha256"] != digest
            || cost == 0
        {
            return None;
        }
        costs.push(cost);
    }
    let candidate = *costs.get(at)? as f64;
    Some(costs.iter().zip(routes).enumerate().filter(|(i,_)|*i!=at).map(|(_, (c,name))|json!({"control":name,"full_cost_single_call_ratio":*c as f64/candidate})).collect())
}
fn kernel_candidate(samples: &[Value]) -> Option<usize> {
    let eligible = match samples.len() {
        23 => 20..23,
        26 => 24..26,
        28 => 27..28,
        30 => 29..30,
        32 => 31..32,
        _ => return None,
    };
    eligible
        .map(|i| Some((samples[i]["paid_wall_ns"].as_u64().filter(|n| *n > 0)?, i)))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .min()
        .map(|(_, i)| i)
}
fn selected_kernel(cases: &[Value], routes: &[&str]) -> Value {
    if ![23, 26, 28, 30, 32].contains(&routes.len()) {
        return Value::Null;
    }
    let mut chosen = Value::Null;
    let mut best = 15.;
    for case in cases {
        if [28,30,32].contains(&routes.len())
            && (case["id"] != "star-last-n16-s7"
                || case["input_sha256"]
                    != "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3")
        {
            continue;
        }
        let Some(samples) = case["samples"].as_array() else {
            continue;
        };
        let Some(at) = kernel_candidate(samples) else {
            continue;
        };
        let Some(rows) = comparisons_at(samples, routes, at) else {
            continue;
        };
        if [
            (19, "legacy"),
            (20, "ordered"),
            (21, "cached"),
            (22, "both"),
            (23, "both"),
            (24, "both_updates"),
            (25, "both_updates_sum"),
            (26, "both_updates"),
            (27, "both_updates_lead"),
            (28, "both_updates"),
            (29, "both_updates_fused"),
            (30, "both_updates_fused"),
            (31, "both_updates_frontier"),
        ]
        .into_iter()
        .filter(|(i, _)| *i < routes.len())
        .any(|(i, mode)| {
            samples[i]["answer"]["kernel_mode"] != mode
                || samples[i]["answer"]["completion_exports"] != 0
                || samples[i]["answer"]["root_api"] != true
                || samples[i]["answer"]["native_product_criterion"] != true
        }) {
            continue;
        }
        if [16, 17, 18]
            .into_iter()
            .zip([2, 1, 0])
            .any(|(i, count)| samples[i]["answer"]["completion_exports"] != count)
        {
            continue;
        }
        let candidate = &samples[at];
        if candidate["used_recovery"] != false
            || candidate["answer"]["constructor"]["subset_succeeded"] != true
            || candidate["answer"]["consumer"]["kind"] != "separate-rust-arithmetic"
        {
            continue;
        }
        let ratios: Vec<_> = rows
            .iter()
            .filter_map(|v| v["full_cost_single_call_ratio"].as_f64())
            .collect();
        if ratios.len() != routes.len() - 1 {
            continue;
        }
        let score = ratios[1].min(ratios[2]);
        if score < 15. || ratios.iter().any(|r| *r <= 1.) {
            continue;
        }
        if chosen.is_null() || score > best {
            best = score;
            chosen = json!({"id":case["id"],"pipeline":routes[at],"kernel":candidate["answer"]["kernel_mode"],"candidate_index":at});
        }
    }
    chosen
}
fn selected_for(cases: &[Value], routes: &[&str]) -> Value {
    let mut chosen = Value::Null;
    let mut best = 15.;
    for case in cases {
        let Some(samples) = case["samples"].as_array() else {
            continue;
        };
        let Some(rows) = comparisons_for(samples, routes) else {
            continue;
        };
        let candidate = &samples[routes.len() - 1];
        if routes.last() == Some(&"root-export-rust-only")
            && [16, 17, 18].into_iter().zip([2, 1, 0]).any(|(i, count)| {
                samples[i]["answer"]["completion_exports"] != count
                    || samples[i]["answer"]["root_api"] != true
                    || samples[i]["answer"]["native_product_criterion"] != true
            })
        {
            continue;
        }
        if candidate["used_recovery"] != false
            || candidate["answer"]["constructor"]["subset_succeeded"] != true
            || candidate["answer"]["root_api"] != true
            || candidate["answer"]["native_product_criterion"] != true
            || candidate["answer"]["consumer"]["kind"] != "separate-rust-arithmetic"
        {
            continue;
        }
        let ratios: Vec<_> = rows
            .iter()
            .map(|r| r["full_cost_single_call_ratio"].as_f64().unwrap())
            .collect();
        let score = ratios[1].min(ratios[2]);
        if score < 15. || ratios.iter().any(|r| *r <= 1.) {
            continue;
        }
        if chosen.is_null() || score > best {
            chosen = case["id"].clone();
            best = score;
        }
    }
    chosen
}
pub(super) fn run(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 0)
}
pub(super) fn run_m31(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 1)
}
pub(super) fn run_export(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 2)
}
pub(super) fn run_kernels(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 3)
}
pub(super) fn run_updates(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 4)
}
pub(super) fn run_leads(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 5)
}
pub(super) fn run_fused(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 6)
}
pub(super) fn run_frontier(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, 7)
}
fn run_impl(prepared: &Path, out: &Path, config: &Path, mode: usize) -> Result<()> {
    let config_bytes = fs::read(config)?;
    let paths: Paths = serde_json::from_slice(&config_bytes)?;
    let m31 = mode >= 1;
    let exports = mode >= 2;
    let kernels = mode >= 3;
    let updates = mode >= 4;
    let leads = mode >= 5;
    let fused = mode >= 6;
    let frontier = mode == 7;
    if [&paths.frontier_baseline, &paths.frontier_direct].into_iter().any(|p|p.is_some()!=frontier) {
        return Err("exact two frontier paths required in32 mode".into());
    }
    if [&paths.fused_baseline, &paths.fused_direct].into_iter().any(|p|p.is_some()!=fused) {
        return Err("exact two fused paths required in30 mode".into());
    }
    if [&paths.lead_baseline, &paths.lead_cached]
        .into_iter()
        .any(|p| p.is_some() != leads)
    {
        return Err("exact two lead paths required in28 mode".into());
    }
    if [
        &paths.update_baseline,
        &paths.update_in_place,
        &paths.update_fastsum,
    ]
    .into_iter()
    .any(|p| p.is_some() != updates)
    {
        return Err("exact three update paths required in26 mode".into());
    }
    if m31 != paths.m31.is_some()
        || [
            &paths.export_both,
            &paths.export_final,
            &paths.export_rust_only,
        ]
        .into_iter()
        .any(|p| p.is_some() != exports)
    {
        return Err("exact optional paths required for15/16/19-workflow modes".into());
    }
    if [
        &paths.kernel_legacy,
        &paths.kernel_ordered,
        &paths.kernel_cached,
        &paths.kernel_both,
    ]
    .into_iter()
    .any(|p| p.is_some() != kernels)
    {
        return Err("exact four optional kernel paths required in23 mode".into());
    }
    let mut routes = ROUTES.to_vec();
    let mut replay_routes = paths.replay_routes().to_vec();
    if let Some(exe) = &paths.m31 {
        routes.push("root-sparse-m31");
        replay_routes.push((exe.as_path(), "compact-sparse"));
    }
    for (name, exe) in [
        ("root-export-both", &paths.export_both),
        ("root-export-final", &paths.export_final),
        ("root-export-rust-only", &paths.export_rust_only),
        ("root-kernel-legacy", &paths.kernel_legacy),
        ("root-kernel-ordered", &paths.kernel_ordered),
        ("root-kernel-cached", &paths.kernel_cached),
        ("root-kernel-both", &paths.kernel_both),
        ("root-update-baseline", &paths.update_baseline),
        ("root-update-in-place", &paths.update_in_place),
        ("root-update-fastsum", &paths.update_fastsum),
        ("root-lead-baseline", &paths.lead_baseline),
        ("root-lead-cached", &paths.lead_cached),
        ("root-fused-baseline", &paths.fused_baseline),
        ("root-fused-direct", &paths.fused_direct),
        ("root-frontier-baseline", &paths.frontier_baseline),
        ("root-frontier-direct", &paths.frontier_direct),
    ] {
        if let Some(exe) = exe {
            routes.push(name);
            replay_routes.push((exe.as_path(), "compact-sparse"));
        }
    }
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    if !qualification && manifest["mode"] != "rust-consumer-screen" {
        return Err("Rust consumer manifest required".into());
    }
    let roster = manifest["cases"].as_array().ok_or("roster")?;
    if (qualification && (roster.len() != 1 || roster[0]["id"] != "n3-s7"))
        || (!qualification && roster.len() != 6)
    {
        return Err("Rust consumer roster shape".into());
    }
    let mut identities = serde_json::Map::new();
    for (name, path) in [
        ("adapter", std::env::current_exe()?),
        ("worker", paths.worker.clone()),
        ("legacy_cli", paths.legacy_cli.clone()),
        ("prior_fused", paths.prior_fused.clone()),
        ("root_baseline", paths.root_baseline.clone()),
        ("root_consumer", paths.root_consumer.clone()),
        ("root_native", paths.root_native.clone()),
        ("root_both", paths.root_both.clone()),
        ("mature", paths.mature.clone()),
        ("singular", paths.singular.clone()),
        ("msolve", paths.msolve.clone()),
        (
            "standard_library",
            PathBuf::from("/workspace/native-cas/share/singular/LIB/standard.lib"),
        ),
    ] {
        identities.insert(name.into(), json!({"path":path,"sha256":file_hash(&path)?}));
    }
    if let Some(path) = &paths.m31 {
        identities.insert("m31".into(), json!({"path":path,"sha256":file_hash(path)?}));
    }
    for (name, path) in [
        ("export_both", &paths.export_both),
        ("export_final", &paths.export_final),
        ("export_rust_only", &paths.export_rust_only),
        ("kernel_legacy", &paths.kernel_legacy),
        ("kernel_ordered", &paths.kernel_ordered),
        ("kernel_cached", &paths.kernel_cached),
        ("kernel_both", &paths.kernel_both),
        ("update_baseline", &paths.update_baseline),
        ("update_in_place", &paths.update_in_place),
        ("update_fastsum", &paths.update_fastsum),
        ("lead_baseline", &paths.lead_baseline),
        ("lead_cached", &paths.lead_cached),
        ("fused_baseline", &paths.fused_baseline),
        ("fused_direct", &paths.fused_direct),
        ("frontier_baseline", &paths.frontier_baseline),
        ("frontier_direct", &paths.frontier_direct),
    ] {
        if let Some(path) = path {
            identities.insert(name.into(), json!({"path":path,"sha256":file_hash(path)?}));
        }
    }
    fs::create_dir(out)?;
    write_new(&out.join("config.json"), &config_bytes)?;
    save(
        &out.join("pre-call.json"),
        &json!({"identities":identities,"manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"config_sha256":hash(&config_bytes),"mode":manifest["mode"],"cpu":paths.cpu,"isolation":"L0","checkpoint":"waived by user","protocol":if frontier{"FRONTIER_PAID_SCREEN_PROTOCOL.md"}else if fused{"FUSED_PAID_SCREEN_PROTOCOL.md"}else if leads{"LEAD_PAID_SCREEN_PROTOCOL.md"}else if updates{"UPDATE_PAID_SCREEN_PROTOCOL.md"}else if kernels{"EXACT_KERNEL_PAID_SCREEN_PROTOCOL.md"}else if exports{"NATIVE_EXPORT_PAID_SCREEN_PROTOCOL.md"}else if m31{"M31_PAID_SCREEN_PROTOCOL.md"}else{"RUST_CONSUMER_PAID_SCREEN_PROTOCOL.md"}}),
    )?;
    let mut r = Runner {
        worker: paths.worker.clone(),
        singular: paths.singular.clone(),
        msolve: paths.msolve.clone(),
        cpu: paths.cpu.clone(),
        worker_hash: identities["worker"]["sha256"]
            .as_str()
            .ok_or("worker pin")?
            .into(),
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut cases = Vec::new();
    for item in roster {
        let id = item["id"].as_str().ok_or("id")?;
        let input = prepared.join(id);
        let dir = out.join(id);
        fs::create_dir(&dir)?;
        r.case = Instant::now();
        let result = (|| -> Result<Value> {
            let bytes = fs::read(input.join("system.json"))?;
            let input_hash = hash(&bytes);
            if item["input_sha256"] != input_hash {
                return Err("changed Rust consumer input".into());
            }
            let system: System = serde_json::from_slice(&bytes)?;
            let n = system.variables.len();
            let prep = item["preparation_ns"]
                .as_u64()
                .filter(|v| *v > 0)
                .ok_or("preparation cost")?;
            let mut reference = None;
            let mut setup = None;
            let mut samples = Vec::new();
            match r.references(&input, &dir, &system, prep) {
                Ok((basis, cost)) => {
                    samples.push(json!({"pipeline":ROUTES[0],"status":"complete","used_recovery":false,"paid_wall_ns":cost,"basis_sha256":basis_hash(&basis),"preparation_ns":prep}));
                    reference = Some(basis);
                    setup = Some(cost);
                }
                Err(e) => samples
                    .push(json!({"pipeline":ROUTES[0],"status":"failed","reason":e.to_string()})),
            }
            let context = Context {
                r: &r,
                input: &input,
                system: &system,
                prep,
                reference: reference.as_ref(),
            };
            let empty = Basis::new();
            for (kind, arm) in ROUTES[1..4].iter().zip(ARMS) {
                let start = Instant::now();
                let result = r.sample(
                    arm,
                    &input,
                    &dir.join(kind),
                    n,
                    reference.as_ref().unwrap_or(&empty),
                    &input_hash,
                );
                let elapsed = ns(start).max(1);
                let paid = paid_sum(elapsed, setup.unwrap_or(0), 0)?;
                let result=result.map(|sample|json!({"status":"complete","basis_sha256":sample.basis_sha256,"sample":sample,"regeneration_setup_ns":setup}));
                samples.push(context.finish(
                    kind,
                    result,
                    failed_native_work(elapsed, setup, prep)?,
                    paid,
                    &dir,
                ));
            }
            let start = Instant::now();
            let result = replay_capture::replay(
                &r,
                &paths.legacy_cli,
                &system,
                &input,
                &dir.join(ROUTES[4]),
                "compact-replay",
                reference.as_ref(),
            );
            let elapsed = ns(start).max(1);
            samples.push(context.finish(
                ROUTES[4],
                result,
                elapsed,
                paid_sum(elapsed, 0, prep)?,
                &dir,
            ));
            for (kind, (exe, producer)) in routes[5..].iter().zip(&replay_routes) {
                let start = Instant::now();
                let result = fused_sample(
                    &r,
                    exe,
                    &input,
                    &dir.join(kind),
                    producer,
                    n,
                    reference.as_ref(),
                );
                let elapsed = ns(start).max(1);
                samples.push(context.finish(
                    kind,
                    result,
                    elapsed,
                    paid_sum(elapsed, 0, prep)?,
                    &dir,
                ));
            }
            let start = Instant::now();
            let digest = samples[0]["basis_sha256"].as_str().unwrap_or("");
            let equal = samples
                .iter()
                .all(|s| s["status"] == "complete" && s["basis_sha256"] == digest);
            let equality_ns = ns(start).max(1);
            for sample in &mut samples {
                if let Some(cost) = sample["paid_wall_ns"].as_u64() {
                    sample["paid_wall_ns"] = json!(paid_sum(cost, equality_ns, 0)?);
                    sample["cross_pipeline_equality_ns"] = json!(equality_ns);
                }
            }
            let at = if kernels && !qualification {
                kernel_candidate(&samples).unwrap_or(routes.len() - 1)
            } else {
                routes.len() - 1
            };
            let rows = if qualification {
                None
            } else {
                comparisons_at(&samples, &routes, at)
            };
            Ok(
                json!({"id":id,"variables":n,"input_sha256":input_hash,"samples":samples,"all_complete_bases_equal":equal,"comparisons":rows,"comparison_candidate_index":if qualification{Value::Null}else{json!(at)},"comparison_candidate":if qualification{Value::Null}else{json!(routes[at])},"scope":if qualification{"correctness only; no ratios or selection"}else{"one call per workflow, no confidence"}}),
            )
        })();
        let value = result
            .unwrap_or_else(|e| json!({"id":id,"status":"case_failed","reason":e.to_string()}));
        save(&dir.join("outcome.json"), &value)?;
        eprintln!("{id}: actual-root workflow outcomes retained");
        cases.push(value);
    }
    let mut unchanged = true;
    for pin in identities.values() {
        unchanged &= file_hash(Path::new(pin["path"].as_str().ok_or("pin path")?))?
            == pin["sha256"].as_str().ok_or("pin hash")?;
    }
    let kernel_selection = if !qualification && kernels {
        selected_kernel(&cases, &routes)
    } else {
        Value::Null
    };
    let family = if qualification {
        Value::Null
    } else if kernels {
        kernel_selection["id"].clone()
    } else {
        selected_for(&cases, &routes)
    };
    save(
        &out.join("summary.json"),
        &json!({"schema":if frontier{"exact-frontier-paid-screen/v1"}else if fused{"exact-fused-paid-screen/v1"}else if leads{"exact-lead-paid-screen/v1"}else if updates{"exact-update-paid-screen/v1"}else if kernels{"exact-kernel-paid-screen/v1"}else if exports{"native-export-paid-screen/v1"}else if m31{"m31-paid-screen/v1"}else{"rust-consumer-paid-screen/v1"},"mode":manifest["mode"],"identities_unchanged":unchanged,"selected_followup_family":family,"selected_kernel":kernel_selection,"cases":cases,"scope":"unsigned L0; actual-root2x2 cells, all controls and paid recovery"}),
    )?;
    if !unchanged {
        return Err("changed executable identities".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn kernel_fixture() -> (Vec<&'static str>, Vec<Value>) {
        let mut routes = ROUTES.to_vec();
        routes.extend([
            "root-sparse-m31",
            "root-export-both",
            "root-export-final",
            "root-export-rust-only",
            "root-kernel-legacy",
            "root-kernel-ordered",
            "root-kernel-cached",
            "root-kernel-both",
        ]);
        let mut rows = complete();
        rows[14]["paid_wall_ns"] = json!(20);
        for i in 15..23 {
            let mut row = rows[14].clone();
            row["pipeline"] = json!(routes[i]);
            row["paid_wall_ns"] = json!(match i {
                19 => 4,
                20 => 2,
                21 => 3,
                22 => 1,
                _ => 20,
            });
            if (16..19).contains(&i) {
                row["answer"]["completion_exports"] = json!(18 - i);
            }
            if i >= 19 {
                row["answer"]["completion_exports"] = json!(0);
                row["answer"]["kernel_mode"] =
                    json!(["legacy", "ordered", "cached", "both"][i - 19]);
            }
            rows.push(row);
        }
        (routes, rows)
    }
    #[test]
    fn kernel_selection_can_choose_ordered_or_both_but_never_ties_or_legacy() {
        let (routes, rows) = kernel_fixture();
        let result = selected_kernel(&[json!({"id":"case","samples":rows})], &routes);
        assert_eq!(result["kernel"], "both");
        let mut changed = rows.clone();
        changed[20]["paid_wall_ns"] = json!(1);
        changed[22]["paid_wall_ns"] = json!(2);
        assert_eq!(
            selected_kernel(&[json!({"id":"ordered","samples":changed})], &routes)["kernel"],
            "ordered"
        );
        let cmp = comparisons_at(&changed, &routes, 20).unwrap();
        assert_eq!(cmp.len(), 22);
        assert!(cmp.iter().all(|c| c["control"] != "root-kernel-ordered"));
        changed[22]["paid_wall_ns"] = json!(1);
        assert!(selected_kernel(&[json!({"id":"tie","samples":changed})], &routes).is_null());
        let mut changed = rows;
        changed[19]["paid_wall_ns"] = json!(1);
        assert!(
            selected_kernel(&[json!({"id":"legacy-cheaper","samples":changed})], &routes).is_null()
        );
    }
    #[test]
    fn kernel_selection_rejects_missing_control_wrong_mode_recovery_and_under15() {
        let (routes, rows) = kernel_fixture();
        for i in 0..23 {
            let mut bad = rows.clone();
            bad[i]["status"] = json!("failed");
            assert!(selected_kernel(&[json!({"id":"missing","samples":bad})], &routes).is_null());
        }
        for i in 19..23 {
            let mut bad = rows.clone();
            bad[i]["answer"]["kernel_mode"] = Value::Null;
            assert!(
                selected_kernel(&[json!({"id":"wrong-mode","samples":bad})], &routes).is_null()
            );
        }
        let mut bad = rows.clone();
        bad[22]["used_recovery"] = json!(true);
        assert!(selected_kernel(&[json!({"id":"recovery","samples":bad})], &routes).is_null());
        let mut bad = rows.clone();
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(selected_kernel(&[json!({"id":"under","samples":bad})], &routes).is_null());
        let mut bad = rows;
        bad[22]["answer"]["constructor"]["subset_succeeded"] = json!(false);
        assert!(selected_kernel(&[json!({"id":"fallback","samples":bad})], &routes).is_null());
    }

    fn update_fixture() -> (Vec<&'static str>, Vec<Value>) {
        let (mut routes, mut rows) = kernel_fixture();
        for row in &mut rows {
            row["paid_wall_ns"] = json!(10);
        }
        rows[1]["paid_wall_ns"] = json!(40);
        rows[2]["paid_wall_ns"] = json!(40);
        for (name, mode, cost) in [
            ("root-update-baseline", "both", 3),
            ("root-update-in-place", "both_updates", 2),
            ("root-update-fastsum", "both_updates_sum", 1),
        ] {
            routes.push(name);
            let mut row = rows[22].clone();
            row["pipeline"] = json!(name);
            row["answer"]["kernel_mode"] = json!(mode);
            row["paid_wall_ns"] = json!(cost);
            rows.push(row);
        }
        (routes, rows)
    }
    #[test]
    fn update_selection_can_choose_each_ablation_without_ties_or_matched_control() {
        let (routes, rows) = update_fixture();
        let select = |r: Vec<Value>| selected_kernel(&[json!({"id":"case","samples":r})], &routes);
        assert_eq!(select(rows.clone())["kernel"], "both_updates_sum");
        let mut changed = rows.clone();
        changed[24]["paid_wall_ns"] = json!(1);
        changed[25]["paid_wall_ns"] = json!(2);
        assert_eq!(select(changed.clone())["kernel"], "both_updates");
        assert_eq!(comparisons_at(&changed, &routes, 24).unwrap().len(), 25);
        changed[25]["paid_wall_ns"] = json!(1);
        assert!(select(changed).is_null());
        for i in [22, 23] {
            let mut bad = rows.clone();
            bad[i]["paid_wall_ns"] = json!(1);
            assert!(select(bad).is_null());
        }
    }
    #[test]
    fn update_selection_requires_all26_outputs_modes_and_full_fast_path() {
        let (routes, rows) = update_fixture();
        let select = |r: Vec<Value>| selected_kernel(&[json!({"id":"case","samples":r})], &routes);
        for i in 0..26 {
            let mut bad = rows.clone();
            bad[i]["status"] = json!("failed");
            assert!(select(bad).is_null());
        }
        for i in 19..26 {
            let mut bad = rows.clone();
            bad[i]["answer"]["kernel_mode"] = Value::Null;
            assert!(select(bad).is_null());
        }
        for key in ["root_api", "native_product_criterion"] {
            let mut bad = rows.clone();
            bad[25]["answer"][key] = json!(false);
            assert!(select(bad).is_null());
        }
        let mut bad = rows.clone();
        bad[25]["used_recovery"] = json!(true);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[25]["answer"]["constructor"]["subset_succeeded"] = json!(false);
        assert!(select(bad).is_null());
        let mut bad = rows;
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(select(bad).is_null());
    }

    fn lead_fixture() -> (Vec<&'static str>, Vec<Value>) {
        let (mut routes, mut rows) = update_fixture();
        for row in &mut rows {
            row["paid_wall_ns"] = json!(20);
        }
        rows[1]["paid_wall_ns"] = json!(40);
        rows[2]["paid_wall_ns"] = json!(40);
        for (name, mode, cost) in [
            ("root-lead-baseline", "both_updates", 3),
            ("root-lead-cached", "both_updates_lead", 1),
        ] {
            routes.push(name);
            let mut row = rows[24].clone();
            row["pipeline"] = json!(name);
            row["answer"]["kernel_mode"] = json!(mode);
            row["paid_wall_ns"] = json!(cost);
            rows.push(row);
        }
        (routes, rows)
    }
    fn select_lead(rows: Vec<Value>, routes: &[&str], id: &str, hash: &str) -> Value {
        selected_kernel(
            &[json!({"id":id,"input_sha256":hash,"samples":rows})],
            routes,
        )
    }
    #[test]
    fn lead_selection_is_fixed_candidate_and_fixed_last_hash() {
        let (routes, rows) = lead_fixture();
        let hash = "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3";
        let s = select_lead(rows.clone(), &routes, "star-last-n16-s7", hash);
        assert_eq!(s["candidate_index"], 27);
        assert_eq!(s["kernel"], "both_updates_lead");
        assert!(select_lead(rows.clone(), &routes, "star-first-n16-s7", hash).is_null());
        assert!(select_lead(rows.clone(), &routes, "star-last-n16-s7", "wrong").is_null());
        assert_eq!(comparisons_at(&rows, &routes, 27).unwrap().len(), 27);
        let mut tie = rows;
        tie[26]["paid_wall_ns"] = json!(1);
        assert!(select_lead(tie, &routes, "star-last-n16-s7", hash).is_null());
    }
    #[test]
    fn lead_selection_keeps_all28_checks_and_old_controls() {
        let (routes, rows) = lead_fixture();
        let select = |r: Vec<Value>| {
            select_lead(
                r,
                &routes,
                "star-last-n16-s7",
                "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3",
            )
        };
        for i in 0..28 {
            let mut bad = rows.clone();
            bad[i]["status"] = json!("failed");
            assert!(select(bad).is_null());
        }
        for i in 19..28 {
            let mut bad = rows.clone();
            bad[i]["answer"]["kernel_mode"] = Value::Null;
            assert!(select(bad).is_null());
        }
        let mut bad = rows.clone();
        bad[24]["paid_wall_ns"] = json!(1);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[27]["used_recovery"] = json!(true);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[27]["answer"]["constructor"]["subset_succeeded"] = json!(false);
        assert!(select(bad).is_null());
        let mut bad = rows;
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(select(bad).is_null());
    }
    fn fused_fixture() -> (Vec<&'static str>, Vec<Value>) {
        let (mut routes, mut rows) = lead_fixture();
        for row in &mut rows {
            row["paid_wall_ns"] = json!(20);
        }
        rows[1]["paid_wall_ns"] = json!(40);
        rows[2]["paid_wall_ns"] = json!(40);
        for (name, mode, cost) in [
            ("root-fused-baseline", "both_updates", 3),
            ("root-fused-direct", "both_updates_fused", 1),
        ] {
            routes.push(name);
            let mut row = rows[24].clone();
            row["pipeline"] = json!(name);
            row["answer"]["kernel_mode"] = json!(mode);
            row["paid_wall_ns"] = json!(cost);
            rows.push(row);
        }
        (routes, rows)
    }
    fn select_fused(rows: Vec<Value>, routes: &[&str], id: &str, hash: &str) -> Value {
        selected_kernel(
            &[json!({"id":id,"input_sha256":hash,"samples":rows})],
            routes,
        )
    }
    #[test]
    fn fused_selection_is_fixed_candidate_and_fixed_last_hash() {
        let (routes, rows) = fused_fixture();
        let hash = "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3";
        let s = select_fused(rows.clone(), &routes, "star-last-n16-s7", hash);
        assert_eq!(s["candidate_index"], 29);
        assert_eq!(s["kernel"], "both_updates_fused");
        assert!(select_fused(rows.clone(), &routes, "star-first-n16-s7", hash).is_null());
        assert!(select_fused(rows.clone(), &routes, "star-last-n16-s7", "wrong").is_null());
        assert_eq!(comparisons_at(&rows, &routes, 29).unwrap().len(), 29);
        let mut tie = rows;
        tie[28]["paid_wall_ns"] = json!(1);
        assert!(select_fused(tie, &routes, "star-last-n16-s7", hash).is_null());
    }
    #[test]
    fn fused_selection_keeps_all30_checks_and_old_controls() {
        let (routes, rows) = fused_fixture();
        let select = |r: Vec<Value>| {
            select_fused(
                r,
                &routes,
                "star-last-n16-s7",
                "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3",
            )
        };
        for i in 0..30 {
            let mut bad = rows.clone();
            bad[i]["status"] = json!("failed");
            assert!(select(bad).is_null());
        }
        for i in 19..30 {
            let mut bad = rows.clone();
            bad[i]["answer"]["kernel_mode"] = Value::Null;
            assert!(select(bad).is_null());
        }
        let mut bad = rows.clone();
        bad[24]["paid_wall_ns"] = json!(1);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[29]["used_recovery"] = json!(true);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[29]["answer"]["constructor"]["subset_succeeded"] = json!(false);
        assert!(select(bad).is_null());
        let mut bad = rows;
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(select(bad).is_null());
    }
    fn frontier_fixture() -> (Vec<&'static str>, Vec<Value>) {
        let (mut routes, mut rows) = fused_fixture();
        for row in &mut rows {
            row["paid_wall_ns"] = json!(20);
        }
        rows[1]["paid_wall_ns"] = json!(40);
        rows[2]["paid_wall_ns"] = json!(40);
        for (name, mode, cost) in [
            ("root-frontier-baseline", "both_updates_fused", 3),
            ("root-frontier-direct", "both_updates_frontier", 1),
        ] {
            routes.push(name);
            let mut row = rows[24].clone();
            row["pipeline"] = json!(name);
            row["answer"]["kernel_mode"] = json!(mode);
            row["paid_wall_ns"] = json!(cost);
            rows.push(row);
        }
        (routes, rows)
    }
    fn select_frontier(rows: Vec<Value>, routes: &[&str], id: &str, hash: &str) -> Value {
        selected_kernel(
            &[json!({"id":id,"input_sha256":hash,"samples":rows})],
            routes,
        )
    }
    #[test]
    fn frontier_selection_is_fixed_candidate_and_fixed_last_hash() {
        let (routes, rows) = frontier_fixture();
        let hash = "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3";
        let s = select_frontier(rows.clone(), &routes, "star-last-n16-s7", hash);
        assert_eq!(s["candidate_index"], 31);
        assert_eq!(s["kernel"], "both_updates_frontier");
        assert!(select_frontier(rows.clone(), &routes, "star-first-n16-s7", hash).is_null());
        assert!(select_frontier(rows.clone(), &routes, "star-last-n16-s7", "wrong").is_null());
        assert_eq!(comparisons_at(&rows, &routes, 31).unwrap().len(), 31);
        let mut tie = rows;
        tie[30]["paid_wall_ns"] = json!(1);
        assert!(select_frontier(tie, &routes, "star-last-n16-s7", hash).is_null());
    }
    #[test]
    fn frontier_selection_keeps_all32_checks_and_old_controls() {
        let (routes, rows) = frontier_fixture();
        let select = |r: Vec<Value>| {
            select_frontier(
                r,
                &routes,
                "star-last-n16-s7",
                "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3",
            )
        };
        for i in 0..32 {
            let mut bad = rows.clone();
            bad[i]["status"] = json!("failed");
            assert!(select(bad).is_null());
        }
        for i in 19..32 {
            let mut bad = rows.clone();
            bad[i]["answer"]["kernel_mode"] = Value::Null;
            assert!(select(bad).is_null());
        }
        let mut bad = rows.clone();
        bad[24]["paid_wall_ns"] = json!(1);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[31]["used_recovery"] = json!(true);
        assert!(select(bad).is_null());
        let mut bad = rows.clone();
        bad[31]["answer"]["constructor"]["subset_succeeded"] = json!(false);
        assert!(select(bad).is_null());
        let mut bad = rows;
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(select(bad).is_null());
    }
    fn comparisons(samples: &[Value]) -> Option<Vec<Value>> {
        comparisons_for(samples, &ROUTES)
    }
    fn selected(cases: &[Value]) -> Value {
        selected_for(cases, &ROUTES)
    }
    #[test]
    fn m31_admission_keeps_every_old_control_and_the_prior_candidate() {
        let mut routes = ROUTES.to_vec();
        routes.push("root-sparse-m31");
        let mut rows = complete();
        rows[14]["paid_wall_ns"] = json!(20);
        let mut candidate = rows[14].clone();
        candidate["pipeline"] = json!(routes[15]);
        candidate["paid_wall_ns"] = json!(1);
        rows.push(candidate);
        assert_eq!(comparisons_for(&rows, &routes).unwrap().len(), 15);
        assert_eq!(
            selected_for(&[json!({"id":"new","samples":rows})], &routes),
            json!("new")
        );
        let mut bad = rows.clone();
        bad[14]["paid_wall_ns"] = json!(1);
        assert!(selected_for(&[json!({"id":"tie-prior","samples":bad})], &routes).is_null());
        let mut bad = rows.clone();
        bad[13]["status"] = json!("failed");
        assert!(comparisons_for(&bad, &routes).is_none());
        let mut bad = rows.clone();
        bad[0]["basis_sha256"] = json!("b".repeat(64));
        assert!(comparisons_for(&bad, &routes).is_none());
        let mut bad = rows.clone();
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(selected_for(&[json!({"id":"under15","samples":bad})], &routes).is_null());
        assert!(comparisons_for(&rows[..15], &routes).is_none());
        assert!(comparisons_for(&rows, &ROUTES).is_none());
    }
    #[test]
    fn export_admission_keeps_m31_and_requires_all_three_actual_modes() {
        let mut routes = ROUTES.to_vec();
        routes.extend([
            "root-sparse-m31",
            "root-export-both",
            "root-export-final",
            "root-export-rust-only",
        ]);
        let mut rows = complete();
        rows[14]["paid_wall_ns"] = json!(20);
        for (i, name) in routes.iter().enumerate().skip(15) {
            let mut row = rows[14].clone();
            row["pipeline"] = json!(name);
            row["paid_wall_ns"] = json!(if i == 18 { 1 } else { 20 });
            if i >= 16 {
                row["answer"]["completion_exports"] = json!(18 - i);
            }
            rows.push(row);
        }
        assert_eq!(comparisons_for(&rows, &routes).unwrap().len(), 18);
        assert_eq!(
            selected_for(&[json!({"id":"export","samples":rows})], &routes),
            json!("export")
        );
        for (index, key) in [
            (16, "completion_exports"),
            (17, "completion_exports"),
            (18, "completion_exports"),
            (18, "root_api"),
        ] {
            let mut bad = rows.clone();
            bad[index]["answer"][key] = Value::Null;
            assert!(selected_for(&[json!({"id":"wrong-mode","samples":bad})], &routes).is_null());
        }
        let mut bad = rows.clone();
        bad[15]["paid_wall_ns"] = json!(1);
        assert!(selected_for(&[json!({"id":"tie-m31","samples":bad})], &routes).is_null());
        assert!(comparisons_for(&rows[..18], &routes).is_none());
    }
    fn complete() -> Vec<Value> {
        ROUTES.iter().enumerate().map(|(i,p)|json!({"pipeline":p,"status":"complete","paid_wall_ns":if i==14{1}else{20},"basis_sha256":"a".repeat(64),"used_recovery":false,"answer":{"root_api":true,"native_product_criterion":true,"consumer":{"kind":"separate-rust-arithmetic"},"constructor":{"subset_succeeded":true}}})).collect()
    }
    #[test]
    fn every_control_exactly_matched_and_actual_root_fastpath_required() {
        let good = complete();
        assert_eq!(comparisons(&good).unwrap().len(), 14);
        assert_eq!(
            selected(&[
                json!({"id":"first","samples":good.clone()}),
                json!({"id":"second","samples":good.clone()})
            ]),
            "first"
        );
        for index in 0..15 {
            let mut bad = good.clone();
            bad[index]["status"] = json!("failed");
            assert!(comparisons(&bad).is_none());
        }
        let mut bad = good.clone();
        bad[11]["basis_sha256"] = json!("b".repeat(64));
        assert!(comparisons(&bad).is_none());
        for key in ["root_api", "native_product_criterion"] {
            let mut bad = good.clone();
            bad[14]["answer"][key] = json!(false);
            assert!(selected(&[json!({"id":"fake","samples":bad})]).is_null());
        }
        let mut bad = good.clone();
        bad[14]["used_recovery"] = json!(true);
        assert!(selected(&[json!({"id":"recovery","samples":bad})]).is_null());
        let mut bad = good.clone();
        bad[2]["paid_wall_ns"] = json!(14);
        assert!(selected(&[json!({"id":"under15","samples":bad})]).is_null());
        let mut bad = good;
        bad[9]["paid_wall_ns"] = json!(1);
        assert!(selected(&[json!({"id":"loses-ablation","samples":bad})]).is_null());
    }
    #[test]
    fn route_matrix_keeps_native_root_cells_and_direct_mature_controls() {
        let paths = Paths {
            worker: "w".into(),
            legacy_cli: "l".into(),
            prior_fused: "old".into(),
            root_baseline: "00".into(),
            root_consumer: "10".into(),
            root_native: "01".into(),
            root_both: "11".into(),
            m31: None,
            export_both: None,
            export_final: None,
            export_rust_only: None,
            kernel_legacy: None,
            kernel_ordered: None,
            kernel_cached: None,
            kernel_both: None,
            update_baseline: None,
            update_in_place: None,
            update_fastsum: None,
            lead_baseline: None,
            lead_cached: None,
            fused_baseline: None,
            fused_direct: None,
            frontier_baseline: None,
            frontier_direct: None,
            mature: "m".into(),
            singular: "s".into(),
            msolve: "ms".into(),
            cpu: "0".into(),
        };
        let p = paths.replay_routes();
        assert_eq!(p[1], (Path::new("00"), "compact-sparse"));
        assert_eq!(p[2], (Path::new("10"), "compact-sparse"));
        assert_eq!(p[3], (Path::new("01"), "compact-sparse"));
        assert_eq!(p[9], (Path::new("11"), "compact-sparse"));
        for (i, kind) in ["std", "slimgb", "msolve-lift"].into_iter().enumerate() {
            assert_eq!(p[6 + i], (Path::new("m"), kind));
        }
    }
}
