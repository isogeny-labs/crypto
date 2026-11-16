//! Recorded complete/failed pipeline costs, without inventing missing ratios.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: pipeline-report RESULTS NEW_REPORT".into());
    }
    let input = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    let summary: Value = serde_json::from_slice(&fs::read(input.join("summary.json"))?)?;
    let cases = summary["cases"].as_array().ok_or("cases")?;
    fs::create_dir(out)?;
    let first = cases
        .first()
        .and_then(|c| c["samples"].as_array())
        .ok_or("samples")?;
    let names: Vec<_> = first
        .iter()
        .map(|s| s["pipeline"].as_str().map(str::to_owned).ok_or("pipeline"))
        .collect::<std::result::Result<_, _>>()?;
    let mut md=format!("# Full pipeline replay screen\n\n{} declared cases; follow-up selection {}. This is unsigned L0 single-call evidence, PCC-20261116-b4b6bd. No complete-roster ratios or confirmed15x gain. All completed answers match; failed controls are retained. [pcc:unverified]\n\n",cases.len(),summary["selected_followup_family"]);
    md+="Costs below are paid milliseconds for one planned pipeline call, including its actual construction and validation algorithm. Regeneration charges all three CAS routes; replay charges complete witness construction/import/export, launch, arithmetic checks and consumption. No old clock is changed. Failures have no numeric win. [pcc:unverified]\n\n| Case | R3 | F4+R3 | E+R3 | C+R3 | E replay | Std lift | Slim lift | Msolve+lift | C replay |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n";
    let mut traces = Vec::new();
    let mut points = Vec::new();
    let mut failed = 0usize;
    let mut complete = 0usize;
    for (index, case) in cases.iter().enumerate() {
        let id = case["id"].as_str().ok_or("case id")?;
        let samples = case["samples"].as_array().ok_or("case samples")?;
        md += &format!("| {id} |");
        for (arm, sample) in samples.iter().enumerate() {
            if sample["status"] == "complete" {
                let ns = sample["paid_wall_ns"].as_u64().ok_or("paid time")?;
                if ns == 0 {
                    return Err("zero cost".into());
                }
                let ms = ns as f64 / 1e6;
                md += &format!(" {ms:.2} |");
                points.push((arm, ms, index + 1));
                complete += 1;
            } else {
                md += " failed |";
                failed += 1;
            }
        }
        md.push('\n');
        let producer = input.join(id).join("liftstd-std/producer.stdout");
        let text = fs::read_to_string(producer)?;
        let mut inside = false;
        let mut terms = 0usize;
        for line in text.lines().map(str::trim) {
            if line.starts_with("PROOF:") {
                inside = true;
            } else if line == "ENDPROOF" {
                inside = false;
            } else if inside && line.starts_with("TERM:") {
                terms += 1;
            }
        }
        if !text.trim_end().ends_with("CERTDONE") {
            return Err("incomplete producer profile".into());
        }
        let mut stderr = fs::read_to_string(input.join(id).join("liftstd-std/import.stderr"))?;
        if stderr.is_empty() {
            let process: Value = serde_json::from_slice(&fs::read(
                input.join(id).join("liftstd-std/import.process.json"),
            )?)?;
            let bytes = fs::metadata(input.join(id).join("liftstd-std/witness.json"))?.len();
            stderr = format!(
                "terminated import; exit_code={}; witness bytes={bytes}",
                process["exit_code"]
            );
        }
        traces.push(json!({"id":id,"std_serialized_cofactor_term_records":terms,"std_import_diagnostic":stderr,"case":case}));
    }
    md+=&format!("\n{complete} complete outputs and{failed} failed pipeline attempts retained. R3=three-route regeneration, E=expanded constructor, C=compact constructor. These are single calls, not means or intervals. [pcc:unverified]\n\n| Case | Std cofactor TERM records | Import diagnostic |\n|---|---:|---|\n");
    for trace in &traces {
        md += &format!(
            "| {} | {} | {} |\n",
            trace["id"].as_str().unwrap(),
            trace["std_serialized_cofactor_term_records"],
            trace["std_import_diagnostic"]
                .as_str()
                .unwrap()
                .trim()
                .replace('|', "/")
        );
    }
    md+="\nRequired Std controls did not complete: four explicit100000-term cofactor-budget rejections and two terminated imports with16MiB witness files (see CAUSE_ERRATA.md for PCC-b4b6bd's initial cause correction). No cap was raised, no call replaced, and no incomplete control was made a numeric ratio. All per-child failure costs and raw matrices remain in the archive. The registered complete-roster gate therefore selects none. [pcc:unverified]\n\n![Complete pipeline costs](costs.svg){width=80%}\n\nFailed routes have no point. This plot compares absolute paid costs only. It establishes neither a solving-degree/regularity reduction nor the requested15x gain. Future algorithm or fallback changes require new prospective scope; this record remains frozen. [pcc:unverified]\n";
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("traces.json"),
        serde_json::to_vec_pretty(
            &json!({"summary":summary,"complete_outputs":complete,"failed_attempts":failed,"cases":traces,"standing":"unsigned one-call producer trace; no ratios"}),
        )?,
    )?;
    let min = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) * 0.8;
    let max = points.iter().map(|p| p.1).fold(1., f64::max) * 1.2;
    let path = out.join("costs.svg");
    let root = SVGBackend::new(&path, (1350, 900)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Paid pipeline costs: failed routes omitted, no ratios",
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(240)
        .build_cartesian_2d((min..max).log_scale(), 0usize..cases.len() + 1)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .y_labels(cases.len() + 1)
        .y_label_formatter(&|i| {
            if *i == 0 {
                String::new()
            } else {
                cases
                    .get(i - 1)
                    .and_then(|c| c["id"].as_str())
                    .unwrap_or("")
                    .into()
            }
        })
        .x_desc("Paid milliseconds, logarithmic scale; one call, no confidence intervals")
        .draw()?;
    for (arm, name) in names.iter().enumerate() {
        chart
            .draw_series(
                points
                    .iter()
                    .filter(|p| p.0 == arm)
                    .map(|p| Circle::new((p.1, p.2), 5, Palette99::pick(arm).filled())),
            )?
            .label(name.clone())
            .legend(move |(x, y)| Circle::new((x, y), 4, Palette99::pick(arm).filled()));
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .background_style(WHITE.mix(0.9))
        .border_style(BLACK.mix(0.3))
        .draw()?;
    root.present()?;
    Ok(())
}
