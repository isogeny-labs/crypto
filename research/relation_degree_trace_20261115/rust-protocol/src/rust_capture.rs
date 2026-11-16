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
fn comparisons(samples: &[Value]) -> Option<Vec<Value>> {
    if samples.len() != ROUTES.len() {
        return None;
    }
    let digest = samples[0]["basis_sha256"].as_str()?;
    if digest.len() != 64 {
        return None;
    }
    let mut costs = Vec::new();
    for (s, name) in samples.iter().zip(ROUTES) {
        let cost = s["paid_wall_ns"].as_u64()?;
        if s["pipeline"] != name
            || s["status"] != "complete"
            || s["basis_sha256"] != digest
            || cost == 0
        {
            return None;
        }
        costs.push(cost);
    }
    let candidate = *costs.last()? as f64;
    Some(costs[..14].iter().zip(ROUTES).map(|(c,name)|json!({"control":name,"full_cost_single_call_ratio":*c as f64/candidate})).collect())
}
fn selected(cases: &[Value]) -> Value {
    let mut chosen = Value::Null;
    let mut best = 15.;
    for case in cases {
        let Some(samples) = case["samples"].as_array() else {
            continue;
        };
        let Some(rows) = comparisons(samples) else {
            continue;
        };
        let candidate = &samples[14];
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
    let config_bytes = fs::read(config)?;
    let paths: Paths = serde_json::from_slice(&config_bytes)?;
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
    fs::create_dir(out)?;
    write_new(&out.join("config.json"), &config_bytes)?;
    save(
        &out.join("pre-call.json"),
        &json!({"identities":identities,"manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"config_sha256":hash(&config_bytes),"mode":manifest["mode"],"cpu":paths.cpu,"isolation":"L0","checkpoint":"waived by user","protocol":"RUST_CONSUMER_PAID_SCREEN_PROTOCOL.md"}),
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
            for (kind, (exe, producer)) in ROUTES[5..].iter().zip(paths.replay_routes()) {
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
            let rows = if qualification {
                None
            } else {
                comparisons(&samples)
            };
            Ok(
                json!({"id":id,"variables":n,"input_sha256":input_hash,"samples":samples,"all_complete_bases_equal":equal,"comparisons":rows,"scope":if qualification{"correctness only; no ratios or selection"}else{"one call per workflow, no confidence"}}),
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
    let family = if qualification {
        Value::Null
    } else {
        selected(&cases)
    };
    save(
        &out.join("summary.json"),
        &json!({"schema":"rust-consumer-paid-screen/v1","mode":manifest["mode"],"identities_unchanged":unchanged,"selected_followup_family":family,"cases":cases,"scope":"unsigned L0; actual-root2x2 cells, all controls and paid recovery"}),
    )?;
    if !unchanged {
        return Err("changed executable identities".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
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
