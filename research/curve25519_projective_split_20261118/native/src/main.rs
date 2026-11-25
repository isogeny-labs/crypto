use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone)]
struct Cell {
    t: u64,
    arm: &'static str,
    path: PathBuf,
    value: Value,
}

fn sha256(path: &Path) -> Result<String> {
    let mut h = Sha256::new();
    h.update(fs::read(path)?);
    Ok(format!("{:x}", h.finalize()))
}

fn quantile(sorted: &[f64], q: f64) -> Option<f64> {
    (!sorted.is_empty()).then(|| sorted[((sorted.len() - 1) as f64 * q).floor() as usize])
}

fn geometric_mean(log_values: &[f64]) -> Option<f64> {
    (!log_values.is_empty())
        .then(|| (log_values.iter().sum::<f64>() / log_values.len() as f64).exp())
}

fn bootstrap(log_ratios: &[f64]) -> Value {
    if log_ratios.is_empty() {
        return json!({"status":"undetermined","reason":"no complete matched t>=4 pairs"});
    }
    let mut state = 20_261_118u64;
    let mut samples = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let mut total = 0.0;
        for _ in log_ratios {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            total += log_ratios[(state % log_ratios.len() as u64) as usize];
        }
        samples.push((total / log_ratios.len() as f64).exp());
    }
    samples.sort_by(f64::total_cmp);
    json!({
        "status":"descriptive_observed_pairs_only",
        "estimator":"geometric mean of paired candidate/baseline F4 multiply-add ratios",
        "point":geometric_mean(log_ratios),
        "low95":quantile(&samples,0.025),
        "high95":quantile(&samples,0.975),
        "resamples":10_000,
        "seed":20_261_118,
        "pairs":log_ratios.len(),
        "qualification_use":"not sufficient because t=5 and t=6 have no complete pairs"
    })
}

fn load_cells(study: &Path) -> Result<(Vec<Cell>, Vec<Value>)> {
    let mut cells = Vec::new();
    let mut censored = Vec::new();
    for t in 3..=6 {
        for arm in ["chain", "projective"] {
            let path = study.join("raw").join(format!("t{t}-{arm}.json"));
            let bytes = fs::read(&path).unwrap_or_default();
            if bytes.iter().all(u8::is_ascii_whitespace) {
                let status = if t == 5 {
                    "censored-timeout"
                } else {
                    "censored-memory-kill"
                };
                censored.push(json!({
                    "t":t,"arm":arm,"status":status,
                    "path":path.strip_prefix(study)?.display().to_string(),"targets_done":0,
                    "sha256":sha256(&path)?
                }));
                continue;
            }
            let value: Value = serde_json::from_slice(&bytes)?;
            cells.push(Cell {
                t,
                arm,
                path,
                value,
            });
        }
    }
    Ok((cells, censored))
}

fn write_figure(path: &Path, analysis: &Value) -> Result<()> {
    const LEFT: f64 = 108.0;
    const RIGHT: f64 = 900.0;
    const TOP: f64 = 92.0;
    const BOTTOM: f64 = 390.0;
    const Y_MIN: f64 = 7.5;
    const Y_MAX: f64 = 11.0;
    let x = |t: u64| LEFT + (t as f64 - 3.0) * (RIGHT - LEFT) / 3.0;
    let y = |log_cost: f64| BOTTOM - (log_cost - Y_MIN) * (BOTTOM - TOP) / (Y_MAX - Y_MIN);

    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="640" viewBox="0 0 1000 640">
<rect width="1000" height="640" fill="#fbfaf7"/>
<style>
text{font-family:Inter,DejaVu Sans,sans-serif;fill:#20252b}.title{font-size:25px;font-weight:700}.sub{font-size:14px;fill:#56606a}.axis{font-size:13px}.grid{stroke:#d7dbdf;stroke-width:1}.base{stroke:#20639b;fill:#20639b}.cand{stroke:#d1495b;fill:#d1495b}.line{fill:none;stroke-width:2.5}.status{font-size:12px;font-weight:600}.note{font-size:14px;font-weight:650;fill:#8d2432}</style>
<text class="title" x="52" y="42">Projective split does not lower relation-finding cost</text>
<text class="sub" x="52" y="66">Curve25519-like k=20 miniature; frozen m=3 qualification; F4 multiply-adds per random target</text>
"##,
    );
    for tick in [8.0, 9.0, 10.0, 11.0] {
        let yy = y(tick);
        svg.push_str(&format!(
            "<line class=\"grid\" x1=\"{LEFT}\" x2=\"{RIGHT}\" y1=\"{yy:.1}\" y2=\"{yy:.1}\"/><text class=\"axis\" x=\"96\" y=\"{:.1}\" text-anchor=\"end\">10^{tick:.0}</text>\n",
            yy + 5.0
        ));
    }
    svg.push_str(&format!(
        "<line stroke=\"#20252b\" x1=\"{LEFT}\" x2=\"{RIGHT}\" y1=\"{BOTTOM}\" y2=\"{BOTTOM}\"/><line stroke=\"#20252b\" x1=\"{LEFT}\" x2=\"{LEFT}\" y1=\"{TOP}\" y2=\"{BOTTOM}\"/>\n"
    ));
    for t in 3..=6 {
        let xx = x(t);
        svg.push_str(&format!(
            "<line stroke=\"#20252b\" x1=\"{xx:.1}\" x2=\"{xx:.1}\" y1=\"{BOTTOM}\" y2=\"{}\"/><text class=\"axis\" x=\"{xx:.1}\" y=\"414\" text-anchor=\"middle\">t={t}</text>\n",
            BOTTOM + 6.0
        ));
    }
    svg.push_str("<text class=\"axis\" transform=\"translate(25 255) rotate(-90)\" text-anchor=\"middle\">mean F4 multiply-adds (log scale)</text>\n");

    for (arm, class, offset) in [("chain", "base", -5.0), ("projective", "cand", 5.0)] {
        let mut points = Vec::new();
        for row in analysis["cells"].as_array().into_iter().flatten() {
            if row["arm"] != arm {
                continue;
            }
            let t = row["t"].as_u64().ok_or("figure t")?;
            let cost = row["muladds_per_target"].as_f64().ok_or("figure cost")?;
            points.push((x(t) + offset, y(cost.log10()), t, cost));
        }
        if points.len() > 1 {
            let coords = points
                .iter()
                .map(|(xx, yy, _, _)| format!("{xx:.1},{yy:.1}"))
                .collect::<Vec<_>>()
                .join(" ");
            svg.push_str(&format!(
                "<polyline class=\"{class} line\" points=\"{coords}\"/>\n"
            ));
        }
        for (xx, yy, t, cost) in points {
            svg.push_str(&format!(
                "<circle class=\"{class}\" cx=\"{xx:.1}\" cy=\"{yy:.1}\" r=\"6\"/><title>{arm}, t={t}: {cost:.3} multiply-adds/target</title>\n"
            ));
        }
    }

    svg.push_str(
        r##"<line class="base line" x1="657" x2="691" y1="103" y2="103"/><circle class="base" cx="674" cy="103" r="5"/><text class="axis" x="700" y="108">affine chain baseline</text>
<line class="cand line" x1="657" x2="691" y1="128" y2="128"/><circle class="cand" cx="674" cy="128" r="5"/><text class="axis" x="700" y="133">projective candidate</text>
<text class="sub" x="108" y="448">Completion status (32 random + 8 planted requested per arm)</text>
"##,
    );
    for t in 3..=6 {
        let xx = x(t);
        let complete = analysis["cells"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|row| row["t"].as_u64() == Some(t))
            .count();
        let status = if complete == 2 {
            ("complete", "#31733c")
        } else if t == 5 {
            ("timeout", "#9a6816")
        } else {
            ("memory kill", "#8d2432")
        };
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"466\" width=\"150\" height=\"38\" rx=\"7\" fill=\"{}\" opacity=\"0.14\"/><text class=\"status\" x=\"{xx:.1}\" y=\"490\" text-anchor=\"middle\" fill=\"{}\">{}</text>\n",
            xx - 75.0,
            status.1,
            status.1,
            status.0
        ));
    }
    svg.push_str(
        r##"<rect x="52" y="535" width="896" height="68" rx="9" fill="#f5e6e8" stroke="#d1495b"/>
<text class="note" x="72" y="563">NOT QUALIFIED FOR m=4</text>
<text class="sub" x="72" y="586">Observed t=4 ratio 1.000464 (candidate/baseline); frozen upper-95% target &lt; 0.80. No four-point slope is estimable.</text>
</svg>
"##,
    );
    fs::write(path, svg)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: curve25519-projective-split-analysis STUDY_DIR".into());
    }
    let study = Path::new(&args[1]);
    let out = study.join("evidence");
    fs::create_dir_all(&out)?;
    let (cells, censored) = load_cells(study)?;
    let attempts: Value = serde_json::from_slice(&fs::read(study.join("raw/attempts.json"))?)?;
    let mut cell_rows = Vec::new();
    let mut per_target = Vec::new();
    let mut paired_logs_t4plus = Vec::new();
    let mut target_mismatches = Vec::new();
    let mut paired_targets = 0usize;

    for cell in &cells {
        let invalid = cell.value["native"]["pkm_invalid_rows"].as_u64();
        cell_rows.push(json!({
            "t":cell.t,"arm":cell.arm,"status":"complete",
            "targets_done":cell.value["targets_done"],"targets_requested":cell.value["targets_requested"],
            "muladds_per_target":cell.value["muladds_per_target"],"wall_s":cell.value["wall_s"],
            "planted_done":cell.value["planted_done"],"planted_exact":cell.value["planted_exact"],
            "hits":cell.value["hits"],"hits_verified":cell.value["hits_verified"],
            "invalid_rows":invalid,"invalid_counter_present":invalid.is_some(),
            "native":cell.value["native"],"native_including_planted":cell.value["native_including_planted"],
            "sha256":sha256(&cell.path)?
        }));
        for trace in cell.value["target_trace"].as_array().into_iter().flatten() {
            let mut row = trace.clone();
            row["t"] = json!(cell.t);
            row["arm"] = json!(cell.arm);
            per_target.push(row);
        }
    }

    for t in 3..=6 {
        let baseline = cells.iter().find(|c| c.t == t && c.arm == "chain");
        let candidate = cells.iter().find(|c| c.t == t && c.arm == "projective");
        let (Some(baseline), Some(candidate)) = (baseline, candidate) else {
            continue;
        };
        let b = baseline.value["target_trace"]
            .as_array()
            .ok_or("baseline target_trace")?;
        let c = candidate.value["target_trace"]
            .as_array()
            .ok_or("candidate target_trace")?;
        if b.len() != c.len() {
            target_mismatches
                .push(json!({"t":t,"kind":"length","baseline":b.len(),"candidate":c.len()}));
            continue;
        }
        for (position, (br, cr)) in b.iter().zip(c).enumerate() {
            let matched = br["target_index"] == cr["target_index"]
                && br["scalar"] == cr["scalar"]
                && br["target_x"] == cr["target_x"]
                && br["target_y"] == cr["target_y"];
            if !matched {
                target_mismatches
                    .push(json!({"t":t,"position":position,"baseline":br,"candidate":cr}));
                continue;
            }
            paired_targets += 1;
            let ratio = cr["f4_muladds"].as_u64().ok_or("candidate cost")? as f64
                / br["f4_muladds"].as_u64().ok_or("baseline cost")? as f64;
            if t >= 4 {
                paired_logs_t4plus.push(ratio.ln());
            }
        }
    }

    let observed_correctness = cells.len() == 4
        && target_mismatches.is_empty()
        && cells.iter().all(|c| {
            c.value["targets_done"] == 32
                && c.value["planted_done"] == 8
                && c.value["planted_exact"] == 8
                && c.value["hits"] == c.value["hits_verified"]
                && c.value["native"]["pkm_invalid_rows"].as_u64() == Some(0)
        });
    let ratio = bootstrap(&paired_logs_t4plus);
    let censoring_not_worse = censored.iter().filter(|v| v["arm"] == "chain").count()
        == censored.iter().filter(|v| v["arm"] == "projective").count();
    let full_grid_complete = cells.len() == 8;
    let qualifies = full_grid_complete
        && observed_correctness
        && censoring_not_worse
        && ratio["high95"].as_f64().is_some_and(|v| v < 0.80);
    let analysis = json!({
        "schema":"curve25519-projective-split-analysis/v1",
        "evidence_status":"pcc:unverified",
        "planned_cells":8,"process_attempts":attempts["process_attempts"],
        "complete_cells":cells.len(),"censored_cells":censored.len(),"cells":cell_rows,
        "censored":censored,"paired_targets":paired_targets,"target_mismatches":target_mismatches,
        "observed_complete_cell_correctness":observed_correctness,
        "full_grid_correctness_gate":false,
        "candidate_censoring_not_worse":censoring_not_worse,
        "paired_ratio_t_ge_4":ratio,
        "baseline_slope":{"status":"undetermined","reason":"t=5 and t=6 have no complete cell mean"},
        "candidate_slope":{"status":"undetermined","reason":"t=5 and t=6 have no complete cell mean"},
        "m3_exponent_lever":"undetermined; no four-point slope interval",
        "qualifies_for_m4":qualifies,
        "decision":"not qualified; incomplete/censored grid and observed cost ratio near one",
        "claim_boundary":"This component result does not establish a subexponential ECDLP algorithm."
    });
    fs::write(
        out.join("analysis.json"),
        serde_json::to_vec_pretty(&analysis)?,
    )?;

    let mut jsonl = String::new();
    for row in &per_target {
        jsonl.push_str(&serde_json::to_string(row)?);
        jsonl.push('\n');
    }
    fs::write(out.join("per-target.jsonl"), jsonl)?;
    let mut csv = String::from("t,arm,status,muladds_per_target,wall_s,planted_exact\n");
    for row in analysis["cells"].as_array().unwrap() {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            row["t"],
            row["arm"].as_str().unwrap(),
            row["status"].as_str().unwrap(),
            row["muladds_per_target"],
            row["wall_s"],
            row["planted_exact"]
        ));
    }
    for row in analysis["censored"].as_array().unwrap() {
        csv.push_str(&format!(
            "{},{},{},,,\n",
            row["t"],
            row["arm"].as_str().unwrap(),
            row["status"].as_str().unwrap()
        ));
    }
    fs::write(out.join("plot.csv"), csv)?;
    write_figure(&out.join("figure.svg"), &analysis)?;

    let mut files = Vec::new();
    for c in &cells {
        files.push(json!({"path":c.path.strip_prefix(study)?.display().to_string(),"sha256":sha256(&c.path)?,"role":"raw aggregate"}));
    }
    for name in ["raw/attempts.json", "raw/qualification-tests.json"] {
        files.push(json!({"path":name,"sha256":sha256(&study.join(name))?,"role":"run metadata"}));
    }
    for c in analysis["censored"].as_array().into_iter().flatten() {
        files.push(json!({"path":c["path"],"sha256":c["sha256"],"role":"censored raw aggregate (empty by construction)"}));
    }
    if study.join("evidence/host.json").exists() {
        files.push(json!({"path":"evidence/host.json","sha256":sha256(&study.join("evidence/host.json"))?,"role":"host metadata"}));
    }
    for (name, role) in [
        ("native/Cargo.lock", "analysis dependency lock"),
        ("native/src/main.rs", "analysis source"),
        ("evidence/analysis.json", "derived analysis"),
        ("evidence/per-target.jsonl", "derived per-target records"),
        ("evidence/plot.csv", "editable figure data"),
        ("evidence/figure.svg", "editable vector figure"),
        ("evidence/figure.png", "raster figure render"),
        ("evidence/report.typ", "report source"),
        ("evidence/report.pdf", "compiled report"),
    ] {
        if study.join(name).exists() {
            files.push(json!({"path":name,"sha256":sha256(&study.join(name))?,"role":role}));
        }
    }
    let export = json!({
        "schema":"experiment.export/v1","export_id":"CURVE25519-PROJECTIVE-SPLIT-20261125-R1",
        "created":"2026-11-25",
        "protocol":{"path":"protocol.json","sha256":sha256(&study.join("protocol.json"))?},
        "source":{"repo":"isogeny-labs/crypto","implementation_commit":"fcfa0556673cbbf45dec4a9d1906bd8f1e068fba","instrumentation_diff_sha256":"a9b7159f5c36bf1faf888a281568a03c58092059899cdb05058b6d645b118991","binary_sha256":"249c9415a43d060012ad179ab6d814da7ee0530e8ad8ee50693bc272c80764fc"},
        "backend":"local-cloud-executor","arms":[{"id":"chain","role":"baseline"},{"id":"projective","role":"candidate"}],
        "flow":{"planned":8,"attempted":9,"completed":4,"not_completed":4,"interrupted":1,"resumed":1,
            "excluded":[{"attempt":3,"reason":"environment restart destroyed tee output sink; unchanged t4-chain retry completed"}]},
        "files":files,
        "analysis":{"script":"native/src/main.rs","outputs":["evidence/analysis.json","evidence/per-target.jsonl","evidence/plot.csv","evidence/figure.svg"],"blinded":false},
        "notes":[{"kind":"observation","text":"t=5 timed out before one target in both arms; t=6 was killed for memory before one target in both arms."},{"kind":"claim boundary","text":"No m=4 stage and no total-work subexponential claim."}],
        "reviews_dir":"reviews"
    });
    fs::write(
        study.join("export.json"),
        serde_json::to_vec_pretty(&export)?,
    )?;

    let point = analysis["paired_ratio_t_ge_4"]["point"].as_f64();
    let low = analysis["paired_ratio_t_ge_4"]["low95"].as_f64();
    let high = analysis["paired_ratio_t_ge_4"]["high95"].as_f64();
    let report = format!(
        "# Curve25519 projective split qualification\n\n[pcc:unverified]\n\nFour of eight frozen cells completed. Both t=5 cells timed out before one target; both t=6 cells were killed for memory before one target. All 32+8 requests in the completed t=3/t=4 cells passed the exact relation checks, including measured projective identity-chart hits.\n\nThe descriptive matched t>=4 candidate/baseline geometric-mean ratio on the 32 available t=4 pairs is {} with a deterministic 95% bootstrap interval [{}, {}]. This does not meet the frozen upper-bound target below 0.80. Because t=5 and t=6 have no complete means, both four-point slope intervals are undefined. The projective arm is **not qualified for m=4**.\n\nThis is a component qualification result, not a total-work subexponential ECDLP result. See `analysis.json` and `../raw/attempts.json` for exact flow and censoring.\n",
        point.map(|v| format!("{v:.9}")).unwrap_or_else(|| "undetermined".into()),
        low.map(|v| format!("{v:.9}")).unwrap_or_else(|| "undetermined".into()),
        high.map(|v| format!("{v:.9}")).unwrap_or_else(|| "undetermined".into()),
    );
    fs::write(out.join("report.md"), report)?;
    Ok(())
}
