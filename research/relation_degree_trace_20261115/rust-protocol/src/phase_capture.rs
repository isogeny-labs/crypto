//! Known-input diagnostic phase costs, no speed ratios or family selection.
use super::*;
use plotters::prelude::*;
const PHASES: [&str; 5] = [
    "construction_and_completion",
    "wire_write",
    "script_export_write",
    "checker_process_consumption",
    "unattributed_parent",
];
fn accounted(answer: &Value, parent: u64) -> Result<Value> {
    let mut phases = answer["profile_ns"]
        .as_object()
        .ok_or("missing phase profile")?
        .clone();
    let mut total = 0u64;
    for phase in &PHASES[..4] {
        let cost = phases[*phase]
            .as_u64()
            .filter(|v| *v > 0)
            .ok_or("missing/zero phase")?;
        total = total.checked_add(cost).ok_or("phase sum overflow")?;
    }
    if phases.len() != 4 {
        return Err("unexpected phase keys".into());
    }
    phases.insert(
        PHASES[4].into(),
        json!(parent
            .checked_sub(total)
            .ok_or("phase sum exceeds parent")?),
    );
    Ok(Value::Object(phases))
}
pub(super) fn run(
    prepared: &Path,
    out: &Path,
    profile: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
) -> Result<()> {
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    let roster = manifest["cases"].as_array().ok_or("manifest cases")?;
    if (qualification && (roster.len() != 1 || roster[0]["id"] != "n3-s7"))
        || (!qualification && (manifest["mode"] != "sparse-screen" || roster.len() != 6))
    {
        return Err("unexpected profile roster".into());
    }
    let mut identities = serde_json::Map::new();
    for (name, path) in [
        ("adapter", std::env::current_exe()?),
        ("profile", profile.clone()),
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
        &json!({"identities":identities,"input_manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"cpu":cpu,"isolation":"L0","scope":"known-input phase diagnostics; generation excluded; no ratios"}),
    )?;
    let mut r = Runner {
        worker: profile.clone(),
        singular,
        msolve,
        cpu,
        worker_hash: identities["profile"]["sha256"]
            .as_str()
            .ok_or("profile pin")?
            .into(),
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut records = Vec::new();
    for item in roster {
        let id = item["id"].as_str().ok_or("id")?;
        let input = prepared.join(id).join("system.json");
        let bytes = fs::read(&input)?;
        if item["input_sha256"] != hash(&bytes) {
            return Err("changed profile input".into());
        }
        let system: System = serde_json::from_slice(&bytes)?;
        let dir = out.join(id);
        fs::create_dir(&dir)?;
        r.case = Instant::now();
        let mut reference = None;
        for kind in ["compact", "expanded", "compact-sparse"] {
            let dest = dir.join(kind);
            fs::create_dir(&dest)?;
            let start = Instant::now();
            let result = (|| -> Result<Value> {
                let (status, bytes, _) = r.process(
                    &profile,
                    &[
                        input.display().to_string(),
                        kind.into(),
                        r.singular.display().to_string(),
                        r.msolve.display().to_string(),
                        dest.join("proof").display().to_string(),
                    ],
                    None,
                    &dest.join("call"),
                )?;
                if !status.success() || !fs::read(dest.join("call.stderr"))?.is_empty() {
                    return Err("profile child failed/diagnostics".into());
                }
                let answer: Value = serde_json::from_slice(&bytes)?;
                let basis = native_basis(&answer["basis"], system.variables.len())?;
                if reference.as_ref().is_some_and(|b| *b != basis) {
                    return Err("complete profile basis mismatch".into());
                }
                let digest = basis_hash(&basis);
                if reference.is_none() {
                    reference = Some(basis);
                }
                let parent = ns(start).max(1);
                let phases = accounted(&answer, parent)?;
                Ok(
                    json!({"id":id,"kind":kind,"status":"complete","input_sha256":item["input_sha256"],"basis_sha256":digest,"parent_wall_ns":parent,"phases_ns":phases,"scope":"original known input, generation excluded; no ratio"}),
                )
            })();
            let value = result.unwrap_or_else(
                |e| json!({"id":id,"kind":kind,"status":"failed","reason":e.to_string()}),
            );
            save(&dest.join("outcome.json"), &value)?;
            records.push(value);
        }
        eprintln!("{id}: phase attempts retained");
    }
    let mut unchanged = true;
    for row in identities.values() {
        unchanged &= file_hash(Path::new(row["path"].as_str().ok_or("path")?))?
            == row["sha256"].as_str().ok_or("hash")?;
    }
    let all_complete = records.iter().all(|r| r["status"] == "complete");
    let summary = json!({"records":records,"all_complete":all_complete,"identities_unchanged":unchanged,"qualification":qualification,"scope":"known-input phase diagnosis; no ratios or numerical selection"});
    save(&out.join("summary.json"), &summary)?;
    if !qualification {
        report(out, &summary)?;
    }
    if !unchanged || !all_complete {
        return Err("profile incomplete/changed; retained all attempts".into());
    }
    Ok(())
}
pub(super) fn render(input: &Path, out: &Path) -> Result<()> {
    let summary: Value = serde_json::from_slice(&fs::read(input.join("summary.json"))?)?;
    fs::create_dir(out)?;
    report(out, &summary)
}
fn report(out: &Path, summary: &Value) -> Result<()> {
    let records = summary["records"].as_array().ok_or("records")?;
    let mut complete = 0usize;
    let mut md=String::from("# Replay phase diagnosis\n\nKnown original inputs, generation excluded. One planned call per mode/case; unsigned L0, no speed ratios, confidence intervals or numerical family selection. Every accepted call pays retained exact checking. [pcc:unverified]\n\n| Case / mode | Construction+completion ms | Wire ms | Export ms | Checker ms | Unattributed parent ms | Parent ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    let mut data = Vec::new();
    for r in records {
        if r["status"] != "complete" {
            md += &format!(
                "\nFailed attempt `{}` / `{}`: {}. [pcc:unverified]\n",
                r["id"], r["kind"], r["reason"]
            );
            continue;
        }
        complete += 1;
        md += &format!(
            "| {} / {} |",
            r["id"].as_str().unwrap(),
            r["kind"].as_str().unwrap()
        );
        let mut costs = Vec::new();
        for phase in PHASES {
            let ms = r["phases_ns"][phase].as_u64().ok_or("phase cost")? as f64 / 1e6;
            md += &format!(" {ms:.3} |");
            costs.push(ms);
        }
        md += &format!(
            " {:.3} |\n",
            r["parent_wall_ns"].as_u64().ok_or("parent")? as f64 / 1e6
        );
        data.push(costs);
    }
    md+="\nDisjoint phases sum to the measured parent interval. Construction includes native construction, membership rows, witness completion and its internal export validation. Unattributed parent includes launch, initial input parsing, output assembly, receipt writing and parent parsing/agreement/hash. Internal timer overhead remains diagnostic overhead. No phase by itself establishes a complete-workflow gain or regularity reduction. [pcc:unverified]\n";
    fs::write(out.join("report.md"), md)?;
    save(
        &out.join("counts.json"),
        &json!({"complete_observations":complete,"failed_observations":records.len()-complete}),
    )?;
    if data.len() != records.len() {
        return Ok(());
    }
    let maximum = data
        .iter()
        .map(|v| v.iter().sum::<f64>())
        .fold(0., f64::max)
        * 1.1;
    let path = out.join("phases.svg");
    let root = SVGBackend::new(&path, (1450, 1100)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Replay phase diagnosis: generation excluded",
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(340)
        .build_cartesian_2d(0f64..maximum, 0f64..records.len() as f64 + 3.)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .y_labels(records.len() + 1)
        .y_label_formatter(&|i| {
            records
                .get((*i as usize).wrapping_sub(1))
                .map(|r| {
                    format!(
                        "{} / {}",
                        r["id"].as_str().unwrap(),
                        r["kind"].as_str().unwrap()
                    )
                })
                .unwrap_or_default()
        })
        .x_desc("Milliseconds, one observation; all phases retained")
        .draw()?;
    for (phase, name) in PHASES.iter().enumerate() {
        chart
            .draw_series(data.iter().enumerate().map(|(i, c)| {
                let start = c[..phase].iter().sum::<f64>();
                Rectangle::new(
                    [(start, i as f64 + 0.7), (start + c[phase], i as f64 + 1.3)],
                    Palette99::pick(phase).filled(),
                )
            }))?
            .label(*name)
            .legend(move |(x, y)| {
                Rectangle::new(
                    [(x, y - 4), (x + 12, y + 4)],
                    Palette99::pick(phase).filled(),
                )
            });
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.mix(0.9))
        .border_style(BLACK.mix(0.3))
        .draw()?;
    root.present()?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diagnostic_costs_require_positive_complete_disjoint_phases() {
        let answer = json!({"profile_ns":{"construction_and_completion":1,"wire_write":2,"script_export_write":3,"checker_process_consumption":4}});
        assert_eq!(accounted(&answer, 15).unwrap()["unattributed_parent"], 5);
        assert!(accounted(&answer, 9).is_err());
        let mut bad = answer.clone();
        bad["profile_ns"]["wire_write"] = json!(0);
        assert!(accounted(&bad, 15).is_err());
        let mut bad = answer;
        bad["profile_ns"]["invented"] = json!(1);
        assert!(accounted(&bad, 15).is_err());
    }
}
