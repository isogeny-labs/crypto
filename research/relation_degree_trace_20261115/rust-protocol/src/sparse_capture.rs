//! Prospective sparse paid workflows; old screen and clocks remain frozen.
use super::*;
const ROUTES: [&str; 11] = [
    "regeneration-external",
    "f4-regeneration",
    "expanded-regeneration",
    "compact-regeneration",
    "legacy-compact-replay",
    "fused-expanded",
    "fused-std",
    "fused-slimgb",
    "fused-msolve-lift",
    "fused-compact-ablation",
    "fused-sparse",
];
pub(super) struct Paths {
    pub worker: PathBuf,
    pub legacy_cli: PathBuf,
    pub fused: PathBuf,
    pub singular: PathBuf,
    pub msolve: PathBuf,
    pub cpu: String,
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
    Some(costs[..10].iter().zip(ROUTES).map(|(c,name)|json!({"control":name,"full_cost_single_call_ratio":*c as f64/candidate})).collect())
}
fn selected(cases: &[Value]) -> Value {
    let mut chosen = Value::Null;
    let mut best = 10.;
    for case in cases {
        let Some(samples) = case["samples"].as_array() else {
            continue;
        };
        let Some(rows) = comparisons(samples) else {
            continue;
        };
        if samples[10]["used_recovery"] != false
            || samples[10]["answer"]["constructor"]["subset_succeeded"] != true
        {
            continue;
        }
        let ratios: Vec<_> = rows
            .iter()
            .map(|r| r["full_cost_single_call_ratio"].as_f64().unwrap())
            .collect();
        let score = ratios[1].min(ratios[2]);
        if ratios.iter().any(|r| *r <= 1.) || score < 10. {
            continue;
        }
        if chosen.is_null() || score > best {
            best = score;
            chosen = case["id"].clone();
        }
    }
    chosen
}
pub(super) fn fused_sample(
    r: &Runner,
    exe: &Path,
    input: &Path,
    out: &Path,
    kind: &str,
    n: usize,
    reference: Option<&Basis>,
) -> Result<Value> {
    let start = Instant::now();
    fs::create_dir(out)?;
    let (status, bytes, _) = r.process(
        exe,
        &[
            input.join("system.json").display().to_string(),
            kind.into(),
            r.singular.display().to_string(),
            r.msolve.display().to_string(),
            out.join("proof").display().to_string(),
        ],
        None,
        &out.join("call"),
    )?;
    if !status.success() || !fs::read(out.join("call.stderr"))?.is_empty() {
        return Err("fused invocation failed/diagnostics".into());
    }
    let answer: Value = serde_json::from_slice(&bytes)?;
    let basis = native_basis(&answer["basis"], n)?;
    if reference.is_some_and(|b| *b != basis) {
        return Err("fused basis mismatch".into());
    }
    let digest = basis_hash(&basis);
    Ok(
        json!({"status":"complete","basis_sha256":digest,"pipeline_wall_ns":ns(start).max(1),"answer":answer}),
    )
}
pub(super) fn paid_sum(attempt: u64, recovery: u64, prep: u64) -> Result<u64> {
    attempt
        .checked_add(recovery)
        .and_then(|x| x.checked_add(prep))
        .ok_or_else(|| "paid cost overflow".into())
}
pub(super) fn failed_native_work(elapsed: u64, setup: Option<u64>, prep: u64) -> Result<u64> {
    // Setup already contains preparation. Recovery adds preparation once,
    // while retaining the full failed validation setup as charged work.
    let validation = match setup {
        Some(cost) => cost
            .checked_sub(prep)
            .ok_or("setup cost below preparation")?,
        None => 0,
    };
    paid_sum(elapsed, validation, 0)
}
pub(super) struct Context<'a> {
    pub(super) r: &'a Runner,
    pub(super) input: &'a Path,
    pub(super) system: &'a System,
    pub(super) prep: u64,
    pub(super) reference: Option<&'a Basis>,
}
impl Context<'_> {
    pub(super) fn finish(
        &self,
        kind: &str,
        result: Result<Value>,
        elapsed: u64,
        success_paid: u64,
        out: &Path,
    ) -> Value {
        match result {
            Ok(mut value) => {
                value["pipeline"] = json!(kind);
                value["used_recovery"] = json!(false);
                value["paid_wall_ns"] = json!(success_paid);
                value
            }
            Err(e) => {
                let attempt =
                    json!({"status":"failed","reason":e.to_string(),"charged_work_ns":elapsed});
                let recovery = out.join(format!("recovery-{kind}"));
                let result = (|| -> Result<Value> {
                    let start = Instant::now();
                    fs::create_dir(&recovery)?;
                    let (basis, _) = self.r.references(self.input, &recovery, self.system, 0)?;
                    if self.reference.is_some_and(|b| *b != basis) {
                        return Err("fresh recovery basis mismatch".into());
                    }
                    let digest = basis_hash(&basis);
                    let recovery_ns = ns(start).max(1);
                    let paid = paid_sum(elapsed, recovery_ns, self.prep)?;
                    Ok(
                        json!({"pipeline":kind,"status":"complete","used_recovery":true,"first_attempt":attempt,"recovery_wall_ns":recovery_ns,"preparation_ns":self.prep,"paid_wall_ns":paid,"basis_sha256":digest,"validation":"fresh three-route regeneration after retained failed attempt"}),
                    )
                })();
                result.unwrap_or_else(|e|json!({"pipeline":kind,"status":"failed","used_recovery":true,"first_attempt":attempt,"recovery_reason":e.to_string()}))
            }
        }
    }
}
pub(super) fn run(prepared: &Path, out: &Path, paths: Paths) -> Result<()> {
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    if !qualification && manifest["mode"] != "sparse-screen" {
        return Err("sparse screen manifest required".into());
    }
    let roster = manifest["cases"].as_array().ok_or("roster")?;
    if (qualification && (roster.len() != 1 || roster[0]["id"] != "n3-s7"))
        || (!qualification && roster.len() != 6)
    {
        return Err("sparse roster shape".into());
    }
    let mut identities = serde_json::Map::new();
    for (name, path) in [
        ("adapter", std::env::current_exe()?),
        ("worker", paths.worker.clone()),
        ("legacy_cli", paths.legacy_cli.clone()),
        ("fused", paths.fused.clone()),
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
    save(
        &out.join("pre-call.json"),
        &json!({"identities":identities,"manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"mode":manifest["mode"],"cpu":paths.cpu,"isolation":"L0","checkpoint":"waived by user","protocol":"SPARSE_PIPELINE_SCREEN_PROTOCOL_02.md"}),
    )?;
    let mut r = Runner {
        worker: paths.worker,
        singular: paths.singular,
        msolve: paths.msolve,
        cpu: paths.cpu,
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
                return Err("changed sparse input".into());
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
            for (kind, producer) in ROUTES[5..].iter().zip([
                "expanded",
                "std",
                "slimgb",
                "msolve-lift",
                "compact",
                "compact-sparse",
            ]) {
                let start = Instant::now();
                let result = fused_sample(
                    &r,
                    &paths.fused,
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
                json!({"id":id,"variables":n,"input_sha256":input_hash,"samples":samples,"all_complete_bases_equal":equal,"comparisons":rows,"scope":if qualification{"correctness only; no ratios"}else{"one planned call per workflow; no confidence"}}),
            )
        })();
        let value = result
            .unwrap_or_else(|e| json!({"id":id,"status":"case_failed","reason":e.to_string()}));
        save(&dir.join("outcome.json"), &value)?;
        eprintln!("{id}: retained sparse workflow outcomes");
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
        &json!({"schema":"sparse-paid-screen/v1","mode":manifest["mode"],"identities_unchanged":unchanged,"selected_followup_family":family,"cases":cases,"scope":"unsigned L0; explicit paid recovery; old results unchanged"}),
    )?;
    if !unchanged {
        return Err("changed identities".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn complete() -> Vec<Value> {
        ROUTES.iter().enumerate().map(|(i,p)|json!({"pipeline":p,"status":"complete","paid_wall_ns":if i==10{1}else{20},"basis_sha256":"a".repeat(64),"used_recovery":false,"answer":{"constructor":{"subset_succeeded":true}}})).collect()
    }
    #[test]
    fn complete_roster_and_proof_fastpath_required() {
        let good = complete();
        assert_eq!(comparisons(&good).unwrap().len(), 10);
        let mut bad = good.clone();
        bad[6]["status"] = json!("failed");
        assert!(comparisons(&bad).is_none());
        let mut bad = good.clone();
        bad[5]["basis_sha256"] = json!("b".repeat(64));
        assert!(comparisons(&bad).is_none());
        let mut bad = good.clone();
        bad[4]["paid_wall_ns"] = json!(0);
        assert!(comparisons(&bad).is_none());
        assert_eq!(
            selected(&[
                json!({"id":"first","samples":good.clone()}),
                json!({"id":"second","samples":good.clone()})
            ]),
            "first"
        );
        let mut bad = good.clone();
        bad[10]["used_recovery"] = json!(true);
        assert!(selected(&[json!({"id":"recovered","samples":bad})]).is_null());
        let mut bad = good;
        bad[9]["paid_wall_ns"] = json!(1);
        assert!(selected(&[json!({"id":"tie-ablation","samples":bad})]).is_null());
    }
    #[test]
    fn failed_work_and_recovery_and_preparation_are_all_paid() {
        assert_eq!(paid_sum(30, 70, 5).unwrap(), 105);
        assert!(paid_sum(u64::MAX, 1, 0).is_err());
        assert_eq!(
            paid_sum(failed_native_work(150, Some(70), 5).unwrap(), 30, 5).unwrap(),
            250
        );
        assert!(failed_native_work(1, Some(2), 3).is_err());
    }
}
