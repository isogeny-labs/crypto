//! Render completed and censored native records without promoting failed controls.
use serde_json::{json, Value};
use std::{error::Error, fs, path::PathBuf};
fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    Some(if values.len() % 2 == 1 {
        values[values.len() / 2]
    } else {
        (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.
    })
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut a = std::env::args().skip(1);
    let root = PathBuf::from(a.next().ok_or("study root required")?);
    let out = PathBuf::from(a.next().ok_or("fresh output required")?);
    if out.exists() {
        return Err("output exists".into());
    }
    fs::create_dir_all(&out)?;
    let manifest: Value = serde_json::from_str(&fs::read_to_string(
        root.join("speed-fixtures/manifest.json"),
    )?)?;
    let runs = [
        "local-run-20261115T0521",
        "projection-run-20261115T0537",
        "engine-run-20261115T0544",
        "combined-run-20261115T0546","criteria-raw-run-20261115T0612","criteria-projection-run-20261115T0613","confirmation-run-20261115",
    ];
    let mut records = Vec::new();
    let mut md=String::from("# Local component observations\n\nProducer-only, exploratory L0 measurements; no signed independent validation. Ratios compare the same native full-basis computation and include candidate construction/checking. Missing or failed-control cases are never wins. `[pcc:unverified]`\n\n");
    for run in runs {
        if !root.join(run).exists() {
            continue;
        }
        md+=&format!("## {run}\n\n| Case | Median ratio | 95% paired bootstrap | A/A admitted | Samples complete | Baseline ms | Candidate ms | Tenfold criterion |\n|---|---:|---|---|---|---:|---:|---|\n");
        let custom_manifest=root.join(run).join("fixture-manifest.json");
        let case_manifest:Value=if custom_manifest.exists(){serde_json::from_str(&fs::read_to_string(custom_manifest)?)?}else{manifest.clone()};
        for c in case_manifest["cases"].as_array().ok_or("bad manifest")? {
            let id = c["id"].as_str().ok_or("bad ID")?;
            let path = root.join(run).join(id).join("summary.json");
            if !path.exists() {
                records.push(
                    json!({"run":run,"id":id,"status":"missing_or_interrupted","tenfold":false}),
                );
                md += &format!("| {id} | — | — | — | no | — | — | no |\n");
                continue;
            }
            let s: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
            let stats = &s["native_checked_wall_ratio"];
            let mut b = Vec::new();
            let mut candidate = Vec::new();
            for sample in s["samples"].as_array().ok_or("missing samples")? {
                let replay = sample["equality_replay_ns_charged_to_each"]
                    .as_u64()
                    .unwrap_or(0) as f64;
                if let Some(v) = sample["arms"]["baseline"]["charged_ns"].as_u64() {
                    b.push((v as f64 + replay) / 1e6);
                }
                if let Some(v) = sample["arms"]["candidate"]["charged_ns"].as_u64() {
                    candidate.push((v as f64 + replay) / 1e6);
                }
            }
            let bm = median(&mut b);
            let cm = median(&mut candidate);
            let ratio = stats["median"].as_f64();
            let low = stats["bootstrap_95"][0].as_f64();
            let high = stats["bootstrap_95"][1].as_f64();
            let complete = s["complete"] == true;
            let control = s["aa_admitted"] == true;
            let tenfold = complete && control && low.is_some_and(|x| x > 10.);
            let f = |v: Option<f64>| v.map(|x| format!("{x:.4}")).unwrap_or_else(|| "—".into());
            md += &format!(
                "| {id} | {} | [{}, {}] | {control} | {complete} | {} | {} | {tenfold} |\n",
                f(ratio),
                f(low),
                f(high),
                f(bm),
                f(cm)
            );
            records.push(json!({"run":run,"id":id,"ratio":ratio,"low":low,"high":high,"baseline_median_ms":bm,"candidate_median_ms":cm,"samples_complete":complete,"aa_admitted":control,"tenfold":tenfold,"status":"observed"}));
        }
        md += "\n";
    }
    md+="These records do not establish a full ECDLP or fastest-solver result. External CPU and whole-process diagnostics must remain separate from native solver-only wall ratios. Interruption notes and every failed attempt remain in their source run directories. Confirmation on fresh cases is separate from these exploratory panels.\n";
    fs::write(out.join("REPORT.md"), md)?;
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(&json!({"records":records}))? + "\n",
    )?;
    let selected: Vec<_> = records
        .iter()
        .filter(|r| r["run"] == "criteria-projection-run-20261115T0613")
        .collect();
    let top = selected
        .iter()
        .filter_map(|r| r["high"].as_f64())
        .fold(10f64, f64::max)
        * 1.1;
    let width = 1100;
    let height = 820;
    let mut svg=format!("<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}'><rect width='100%' height='100%' fill='white'/><g font-family='sans-serif' font-size='12'><text x='15' y='20'>Final projected-criterion arm: matched native wall ratio (producer L0 observation)</text>");
    for (i, r) in selected.iter().enumerate() {
        let y = 55 + i * 28;
        svg += &format!("<text x='10' y='{y}'>{}</text>", r["id"].as_str().unwrap());
        if let (Some(m), Some(l), Some(h)) =
            (r["ratio"].as_f64(), r["low"].as_f64(), r["high"].as_f64())
        {
            let x = |v: f64| 330. + v / top * 700.;
            let color = if r["aa_admitted"] == true {
                "#26745a"
            } else {
                "#999999"
            };
            svg+=&format!("<line x1='{}' x2='{}' y1='{y}' y2='{y}' stroke='{color}'/><circle cx='{}' cy='{y}' r='4' fill='{color}'/>",x(l),x(h),x(m));
        } else {
            svg += &format!("<text x='340' y='{y}' fill='#777777'>missing / incomplete</text>");
        }
    }
    for (v, label) in [(1., "parity"), (10., "tenfold threshold")] {
        let x = 330. + v / top * 700.;
        svg+=&format!("<line x1='{x}' x2='{x}' y1='30' y2='735' stroke='#bb6666' stroke-dasharray='4 4'/><text x='{x}' y='765'>{label}</text>");
    }
    svg+="<text x='10' y='750'>Grey = failed A/A control. Bars = paired bootstrap 95% intervals. Larger ratios are faster.</text></g></svg>";
    fs::write(out.join("combined-ratios.svg"), svg)?;
    Ok(())
}
