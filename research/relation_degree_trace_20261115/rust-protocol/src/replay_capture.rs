//! New full-pipeline screen, separate from frozen old numerical scopes.
use super::*;
const PIPELINES: [&str; 9] = [
    "regeneration-external",
    "f4-regeneration",
    "expanded-regeneration",
    "compact-regeneration",
    "expanded-replay",
    "liftstd-std",
    "liftstd-slimgb",
    "msolve-lift",
    "compact-replay",
];
fn checked(
    r: &Runner,
    exe: &Path,
    args: &[String],
    input: Option<&Path>,
    prefix: &Path,
) -> Result<Vec<u8>> {
    let (status, bytes, _) = r.process(exe, args, input, prefix)?;
    if !status.success() || !fs::read(prefix.with_extension("stderr"))?.is_empty() {
        return Err(format!("pipeline child failed/diagnostics: {}", prefix.display()).into());
    }
    Ok(bytes)
}
pub(super) fn replay(
    r: &Runner,
    cli: &Path,
    system: &System,
    input: &Path,
    out: &Path,
    kind: &str,
    reference: Option<&Basis>,
) -> Result<Value> {
    let start = Instant::now();
    fs::create_dir(out)?;
    let source = input.join("system.json");
    let wire = out.join("witness.json");
    let script = out.join("check.sing");
    let source_arg = source.display().to_string();
    let wire_arg = wire.display().to_string();
    match kind {
        "expanded-replay" | "compact-replay" => {
            checked(
                r,
                cli,
                &[
                    "produce".into(),
                    source_arg.clone(),
                    if kind == "compact-replay" {
                        "compact"
                    } else {
                        "expanded"
                    }
                    .into(),
                    wire_arg.clone(),
                ],
                None,
                &out.join("produce"),
            )?;
        }
        "liftstd-std" | "liftstd-slimgb" | "msolve-lift" => {
            let producer = out.join("producer.sing");
            if kind == "msolve-lift" {
                let ms = out.join("input.ms");
                let basis = out.join("msolve-basis.txt");
                checked(
                    r,
                    cli,
                    &[
                        "program".into(),
                        source_arg.clone(),
                        "msolve".into(),
                        ms.display().to_string(),
                    ],
                    None,
                    &out.join("msolve-input"),
                )?;
                checked(
                    r,
                    &r.msolve,
                    &[
                        "-g".into(),
                        "2".into(),
                        "-t".into(),
                        "1".into(),
                        "-f".into(),
                        ms.display().to_string(),
                        "-o".into(),
                        basis.display().to_string(),
                    ],
                    None,
                    &out.join("msolve"),
                )?;
                checked(
                    r,
                    cli,
                    &[
                        "program-lift".into(),
                        source_arg.clone(),
                        basis.display().to_string(),
                        producer.display().to_string(),
                    ],
                    None,
                    &out.join("producer-program"),
                )?;
            } else {
                checked(
                    r,
                    cli,
                    &[
                        "program".into(),
                        source_arg.clone(),
                        kind.strip_prefix("liftstd-").unwrap().into(),
                        producer.display().to_string(),
                    ],
                    None,
                    &out.join("producer-program"),
                )?;
            }
            let prefix = out.join("producer");
            checked(
                r,
                &r.singular,
                &["-q".into(), producer.display().to_string()],
                None,
                &prefix,
            )?;
            checked(
                r,
                cli,
                &[
                    "import".into(),
                    source_arg.clone(),
                    prefix.with_extension("stdout").display().to_string(),
                    wire_arg.clone(),
                ],
                None,
                &out.join("import"),
            )?;
        }
        _ => return Err("unknown replay pipeline".into()),
    }
    checked(
        r,
        cli,
        &[
            "script".into(),
            source_arg,
            wire_arg,
            script.display().to_string(),
        ],
        None,
        &out.join("export"),
    )?;
    let bytes = checked(
        r,
        &r.singular,
        &["-q".into(), script.display().to_string()],
        None,
        &out.join("check"),
    )?;
    if std::str::from_utf8(&bytes)?.trim() != "CERTIFICATE_REPLAY_OK" {
        return Err("replay acceptance marker missing or polluted".into());
    }
    let witness: certified_relations::replay::Witness = serde_json::from_slice(&fs::read(&wire)?)?;
    let rows: Vec<Vec<_>> = witness
        .basis
        .iter()
        .map(|row| {
            row.polynomial
                .iter()
                .map(|t| (t.exponents.clone(), t.coefficient))
                .collect()
        })
        .collect();
    let basis = native_basis(&json!(rows), system.variables.len())?;
    if reference.is_some_and(|g| *g != basis) {
        return Err("replay complete basis differs from regeneration".into());
    }
    let basis_sha256 = basis_hash(&basis);
    let elapsed = ns(start).max(1);
    Ok(
        json!({"pipeline":kind,"status":"complete","pipeline_wall_ns":elapsed,"basis_sha256":basis_sha256,"witness_sha256":file_hash(&wire)?,"checker_script_sha256":file_hash(&script)?,"validation":"exact witness arithmetic, strict S-pair representations and reducedness; unsigned","regeneration_basis_equal":reference.map(|g|*g==basis)}),
    )
}
fn comparisons(samples: &[Value]) -> Option<Vec<Value>> {
    if samples.len() != PIPELINES.len() {
        return None;
    }
    let mut costs = Vec::new();
    let digest = samples[0]["basis_sha256"].as_str()?;
    if digest.len() != 64 {
        return None;
    }
    for (sample, name) in samples.iter().zip(PIPELINES) {
        let cost = sample["paid_wall_ns"].as_u64()?;
        if sample["pipeline"] != name
            || sample["status"] != "complete"
            || sample["basis_sha256"] != digest
            || cost == 0
        {
            return None;
        }
        costs.push(cost);
    }
    let candidate = *costs.last()? as f64;
    Some(costs[..costs.len()-1].iter().zip(PIPELINES).map(|(cost,name)|json!({"control":name,"full_cost_single_call_ratio":*cost as f64/candidate})).collect())
}
fn selected(cases: &[Value]) -> Value {
    let mut best = 10.;
    let mut chosen = Value::Null;
    for case in cases {
        let Some(samples) = case["samples"].as_array() else {
            continue;
        };
        let Some(rows) = comparisons(samples) else {
            continue;
        };
        let score = rows
            .iter()
            .filter_map(|r| r["full_cost_single_call_ratio"].as_f64())
            .fold(f64::INFINITY, f64::min);
        if score >= 10. && (chosen.is_null() || score > best) {
            chosen = case["id"].clone();
            best = score;
        }
    }
    chosen
}
pub(super) fn run(
    prepared: &Path,
    out: &Path,
    worker: PathBuf,
    cli: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
) -> Result<()> {
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    if !qualification && manifest["mode"] != "replay-screen" {
        return Err("replay pipeline mode required".into());
    }
    let roster = manifest["cases"].as_array().ok_or("missing cases")?;
    if (!qualification && roster.len() != 6)
        || (qualification && (roster.len() != 1 || roster[0]["id"] != "n3-s7"))
    {
        return Err("unexpected replay roster".into());
    }
    let mut identities = serde_json::Map::new();
    for (name, path) in [
        ("adapter", std::env::current_exe()?),
        ("worker", worker.clone()),
        ("cli", cli.clone()),
        ("singular", singular.clone()),
        ("msolve", msolve.clone()),
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
        &json!({"identities":identities,"mode":manifest["mode"],"manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"cpu":cpu,"isolation":"L0","checkpoint":"waived by user","scope":if qualification{"correctness only"}else{"one-call pipeline screen; not confirmation"}}),
    )?;
    let mut r = Runner {
        worker,
        singular,
        msolve,
        cpu,
        worker_hash: identities["worker"]["sha256"]
            .as_str()
            .ok_or("worker hash")?
            .into(),
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut cases = Vec::new();
    for item in roster {
        let id = item["id"].as_str().ok_or("missing id")?;
        let input = prepared.join(id);
        let dir = out.join(id);
        fs::create_dir(&dir)?;
        r.case = Instant::now();
        let result = (|| -> Result<Value> {
            let bytes = fs::read(input.join("system.json"))?;
            let input_hash = hash(&bytes);
            if item["input_sha256"] != input_hash {
                return Err("changed replay input".into());
            }
            let system: System = serde_json::from_slice(&bytes)?;
            let n = system.variables.len();
            let prep = item["preparation_ns"].as_u64().ok_or("preparation cost")?;
            let regen = r.references(&input, &dir, &system, prep);
            let mut reference = None;
            let mut setup = None;
            let mut samples = Vec::new();
            match regen {
                Ok((basis, cost)) => {
                    samples.push(json!({"pipeline":PIPELINES[0],"status":"complete","paid_wall_ns":cost,"basis_sha256":basis_hash(&basis),"preparation_ns":prep,"validation":"three complete mature routes agree"}));
                    reference = Some(basis);
                    setup = Some(cost);
                }
                Err(e) => samples.push(
                    json!({"pipeline":PIPELINES[0],"status":"failed","reason":e.to_string()}),
                ),
            }
            let empty = Basis::new();
            for (kind, arm) in PIPELINES[1..4].iter().zip(ARMS) {
                let value = match r.sample(
                    arm,
                    &input,
                    &dir.join(kind),
                    n,
                    reference.as_ref().unwrap_or(&empty),
                    &input_hash,
                ) {
                    Ok(sample) => {
                        let cost = setup
                            .ok_or("missing reference cost")?
                            .checked_add(sample.worker_wall_ns)
                            .and_then(|c| c.checked_add(sample.validation_wall_ns))
                            .ok_or("cost overflow")?;
                        json!({"pipeline":kind,"status":"complete","paid_wall_ns":cost,"basis_sha256":sample.basis_sha256,"sample":sample,"regeneration_setup_ns":setup})
                    }
                    Err(e) => json!({"pipeline":kind,"status":"failed","reason":e.to_string()}),
                };
                samples.push(value);
            }
            for kind in &PIPELINES[4..] {
                let value = match replay(
                    &r,
                    &cli,
                    &system,
                    &input,
                    &dir.join(kind),
                    kind,
                    reference.as_ref(),
                ) {
                    Ok(mut sample) => {
                        sample["paid_wall_ns"] = json!(sample["pipeline_wall_ns"]
                            .as_u64()
                            .ok_or("pipeline cost")?
                            .checked_add(prep)
                            .ok_or("cost overflow")?);
                        sample["preparation_ns"] = json!(prep);
                        sample
                    }
                    Err(e) => json!({"pipeline":kind,"status":"failed","reason":e.to_string()}),
                };
                samples.push(value);
            }
            let equality = Instant::now();
            let same = samples
                .iter()
                .filter(|s| s["status"] == "complete")
                .all(|s| {
                    reference
                        .as_ref()
                        .is_some_and(|b| s["basis_sha256"] == basis_hash(b))
                });
            let equality_ns = ns(equality).max(1);
            for sample in &mut samples {
                if let Some(cost) = sample["paid_wall_ns"].as_u64() {
                    sample["paid_wall_ns"] = json!(cost
                        .checked_add(equality_ns)
                        .ok_or("equality cost overflow")?);
                    sample["cross_pipeline_equality_ns"] = json!(equality_ns);
                }
            }
            let rows = if qualification {
                None
            } else {
                comparisons(&samples)
            };
            Ok(
                json!({"id":id,"variables":n,"input_sha256":input_hash,"samples":samples,"all_complete_bases_equal":same,"comparisons":rows,"standing":if qualification{"correctness; no numerical selection"}else{"one planned call per pipeline; no confidence"}}),
            )
        })();
        let value = match result {
            Ok(v) => v,
            Err(e) => json!({"id":id,"status":"case_failed","reason":e.to_string()}),
        };
        save(&dir.join("outcome.json"), &value)?;
        eprintln!(
            "{id}: retained {} pipeline outcomes",
            value["samples"].as_array().map_or(0, Vec::len)
        );
        cases.push(value);
    }
    let mut unchanged = true;
    for pin in identities.values() {
        unchanged &= file_hash(Path::new(pin["path"].as_str().ok_or("identity path")?))?
            == pin["sha256"].as_str().ok_or("identity hash")?;
    }
    let family = if qualification {
        Value::Null
    } else {
        selected(&cases)
    };
    save(
        &out.join("summary.json"),
        &json!({"schema":"replay-pipeline-screen/v1","mode":manifest["mode"],"identities_unchanged":unchanged,"selected_followup_family":family,"cases":cases,"scope":"unsigned L0; validation algorithm changes explicitly; no old clocks modified"}),
    )?;
    if !unchanged {
        return Err("pipeline executable/library identity changed".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn complete() -> Vec<Value> {
        PIPELINES.iter().enumerate().map(|(i,p)|json!({"pipeline":p,"status":"complete","paid_wall_ns":if i==8{1}else{20},"basis_sha256":"a".repeat(64)})).collect()
    }
    #[test]
    fn all_eight_controls_must_be_complete_paid_and_matched() {
        let good = complete();
        assert_eq!(comparisons(&good).unwrap().len(), 8);
        let mut bad = good.clone();
        bad[6]["status"] = json!("failed");
        assert!(comparisons(&bad).is_none());
        let mut bad = good.clone();
        bad[5]["basis_sha256"] = json!("b".repeat(64));
        assert!(comparisons(&bad).is_none());
        let mut bad = good;
        bad[3]["paid_wall_ns"] = json!(0);
        assert!(comparisons(&bad).is_none());
    }
    #[test]
    fn cannot_promote_a_fast_candidate_with_a_missing_mature_control() {
        let mut samples = complete();
        samples.pop();
        assert!(comparisons(&samples).is_none());
        let good = json!({"id":"first","samples":complete()});
        let tied = json!({"id":"second","samples":complete()});
        assert_eq!(selected(&[good, tied]), "first");
        let mut samples = complete();
        samples[7]["paid_wall_ns"] = json!(5);
        assert!(selected(&[json!({"id":"miss","samples":samples})]).is_null());
    }
}
