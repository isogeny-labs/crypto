//! Describe a recorded fixed screen, preserving failed rows and missing ratios.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: screen-report INPUTS RESULTS NEW_REPORT".into());
    }
    let input = Path::new(&args[1]);
    let results = Path::new(&args[2]);
    let output = Path::new(&args[3]);
    let manifest = read(&input.join("manifest.json"))?;
    let summary = read(&results.join("summary.json"))?;
    let cases = manifest["cases"].as_array().ok_or("missing roster")?;
    fs::create_dir(output)?;
    let mut md = format!("# Recorded {}\n\n{} declared cases. Selected follow-up: {}. Unsigned producer L0 single-call evidence; no interval, independent execution coverage or confirmed 10x/15x gain. All preparation, mature references and per-call checking are paid. Missing/failed arms have no numeric win. [pcc:unverified]\n\n", manifest["mode"], cases.len(), summary["selected_followup_family"]);
    md += "| Case | Outcome | F4 degree | F4 max columns | Compact proof terms | Paid F4/compact | Paid expanded/compact |\n|---|---|---:|---:|---:|---:|---:|\n";
    let mut traces = Vec::new();
    let mut plot_rows = Vec::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("case id")?;
        let dir = results.join(id);
        let outcome = read(&dir.join("outcome.json"))?;
        let mut calls = serde_json::Map::new();
        for arm in ["f4", "certified", "compact"] {
            let path = dir.join(format!("screen-{arm}.call.json"));
            if path.exists() {
                calls.insert(arm.into(), read(&path)?);
            }
        }
        let number = |control: &str| {
            if outcome["status"] != "screen_complete" {
                return None;
            }
            outcome["comparisons"]
                .as_array()?
                .iter()
                .find(|r| r["control"] == control)?["full_cost_single_call_ratio"]
                .as_f64()
        };
        let f = number("f4");
        let e = number("certified");
        if f.is_some_and(|v| !v.is_finite() || v <= 0.)
            || e.is_some_and(|v| !v.is_finite() || v <= 0.)
        {
            return Err("invalid recorded ratio".into());
        }
        let degree = calls
            .get("f4")
            .map(|v| &v["worker_output"]["observed_solving_degree"]);
        let columns = calls
            .get("f4")
            .map(|v| &v["worker_output"]["f4_observations"]["max_columns"]);
        let terms = calls
            .get("compact")
            .map(|v| &v["worker_output"]["quotient_info"]["stored_proof_terms"]);
        let display = |v: Option<&Value>| {
            v.filter(|v| !v.is_null())
                .map_or("—".into(), Value::to_string)
        };
        let ratio = |v: Option<f64>| v.map_or("—".into(), |r| format!("{r:.3}x"));
        md += &format!(
            "| {id} | {} | {} | {} | {} | {} | {} |\n",
            outcome["status"],
            display(degree),
            display(columns),
            display(terms),
            ratio(f),
            ratio(e)
        );
        traces.push(json!({"input":case,"outcome":outcome,"calls":calls}));
        plot_rows.push((id.to_owned(), f, e));
    }
    md += "\nF4 degree is the completed worker's observed productive degree, not intrinsic regularity. The native packed grevlex path is used only through15 variables;16 uses its existing vector fallback. No solver path was disabled or retuned. [pcc:unverified]\n\n![Paid single-call ratios](ratios.svg){width=80%}\n\nBlue=F4/compact; orange=expanded/compact. No point is drawn for an incomplete comparison. Each point is one planned call, with no confidence interval. The original null-selection screen and its frozen protocols remain intact. [pcc:unverified]\n";
    fs::write(output.join("report.md"), md)?;
    fs::write(
        output.join("traces.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"recorded-screen-trace/v1","manifest":manifest,"summary":summary,"cases":traces,"standing":"unsigned single-call producer evidence"}),
        )?,
    )?;
    let max_ratio = plot_rows
        .iter()
        .flat_map(|(_, f, e)| [*f, *e])
        .flatten()
        .fold(32., f64::max)
        * 1.1;
    let min_ratio = plot_rows
        .iter()
        .flat_map(|(_, f, e)| [*f, *e])
        .flatten()
        .fold(0.1, f64::min)
        * 0.9;
    let path = output.join("ratios.svg");
    let root = SVGBackend::new(&path, (1150, 650)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Paid single-call screen; no confidence intervals",
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(230)
        .build_cartesian_2d((min_ratio..max_ratio).log_scale(), 0usize..plot_rows.len() + 1)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .y_labels(plot_rows.len() + 1)
        .y_label_formatter(&|i| {
            plot_rows
                .get(i.saturating_sub(1))
                .filter(|_| *i > 0)
                .map_or(String::new(), |r| r.0.clone())
        })
        .x_desc("Control / compact; >1 favors compact; blue=F4, orange=expanded")
        .draw()?;
    for x in [1., 10., 15.] {
        chart.draw_series(LineSeries::new(
            [(x, 0), (x, plot_rows.len() + 1)],
            BLACK.mix(0.4),
        ))?;
    }
    for (index, (_, f, e)) in plot_rows.iter().enumerate() {
        for (ratio, color) in [(f, BLUE), (e, RGBColor(220, 110, 0))] {
            if let Some(ratio) = ratio {
                chart.draw_series(std::iter::once(Circle::new(
                    (*ratio, index + 1),
                    5,
                    color.filled(),
                )))?;
            }
        }
    }
    root.present()?;
    Ok(())
}
