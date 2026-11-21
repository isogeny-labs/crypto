//! Frozen screen report; no confirmation inference from single calls.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const SHORT: [&str; 19] = [
    "R3", "F4+R3", "E+R3", "C+R3", "Legacy", "Prior", "00", "10", "01", "Full", "Eopt", "Std",
    "Slim", "MS", "Old", "M31", "New2", "Final", "Rust",
];
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: native-export-report RESULTS NEW_REPORT RESULT_PCC_ID".into());
    }
    let input = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    let summary: Value = serde_json::from_slice(&fs::read(input.join("summary.json"))?)?;
    if summary["schema"] != "native-export-paid-screen/v1"
        || summary["identities_unchanged"] != true
    {
        return Err("Native export transcript/schema/identity required".into());
    }
    let cases = summary["cases"].as_array().ok_or("cases")?;
    fs::create_dir(out)?;
    let samples = cases
        .first()
        .and_then(|c| c["samples"].as_array())
        .ok_or("samples")?;
    let names: Vec<_> = samples
        .iter()
        .map(|s| s["pipeline"].as_str().ok_or("name"))
        .collect::<std::result::Result<_, _>>()?;
    if names.len() != 19 {
        return Err("nineteen workflows required".into());
    }
    let mut md=format!("# Native completion export complete paid screen\n\n{} fixed development cases; prospective follow-up {}. Unsigned L0 single calls, {}. No paired confidence interval or confirmed15x. New2/Final/Rust are matched newly compiled two/one/no-export variants; M31 retains the previous frozen candidate. All retain full native checks and complete root replay. The original measured preparation is charged on these known fixtures; no fresh generation occurred. The two primary denominators are named old fully validated pipelines, not raw kernels or the fastest available control. [pcc:unverified]\n\nAll construction, preparation, transfer, exact checking and output consumption are paid. An inner failure and its fresh3CAS recovery remain charged; no sample is replaced. Every denominator remains in raw comparisons. [pcc:unverified]\n\n| Case |",cases.len(),summary["selected_followup_family"],a[3]);
    for name in SHORT {
        md += &format!(" {name} |");
    }
    md += "\n|---|";
    for _ in SHORT {
        md += "---:|";
    }
    md.push('\n');
    let (mut complete, mut failed, mut recovered) = (0usize, 0usize, 0usize);
    let mut points = Vec::new();
    let mut traces = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        let id = case["id"].as_str().ok_or("id")?;
        let samples = case["samples"].as_array().ok_or("samples")?;
        if samples.len() != 19 {
            return Err("missing workflow".into());
        }
        md += &format!("| {id} |");
        for (arm, s) in samples.iter().enumerate() {
            if s["pipeline"] != names[arm] {
                return Err("changed workflow order".into());
            }
            if s["status"] == "complete" {
                if s["basis_sha256"] != samples[0]["basis_sha256"] {
                    return Err("complete output mismatch".into());
                }
                let ns = s["paid_wall_ns"]
                    .as_u64()
                    .filter(|v| *v > 0)
                    .ok_or("paid cost")?;
                let ms = ns as f64 / 1e6;
                md += &format!(
                    " {ms:.2}{} |",
                    if s["used_recovery"] == true { " R" } else { "" }
                );
                points.push((arm, ms, index + 1));
                complete += 1;
            } else {
                md += " failed |";
                failed += 1;
            }
            if s["used_recovery"] == true {
                recovered += 1;
                let sum = s["first_attempt"]["charged_work_ns"]
                    .as_u64()
                    .ok_or("failed cost")?
                    .checked_add(s["recovery_wall_ns"].as_u64().ok_or("recovery cost")?)
                    .and_then(|x| x.checked_add(s["preparation_ns"].as_u64()?))
                    .and_then(|x| x.checked_add(s["cross_pipeline_equality_ns"].as_u64()?))
                    .ok_or("recovery sum")?;
                if s["paid_wall_ns"] != sum {
                    return Err("incorrect charged recovery sum".into());
                }
            }
        }
        md.push('\n');
        let worker: Value =
            serde_json::from_slice(&fs::read(input.join(id).join("f4-regeneration.call.json"))?)?;
        let candidate = &samples[18];
        let stderr = fs::read_to_string(input.join(id).join("rust-std/call.stderr"))?;
        traces.push(json!({"id":id,"f4_observed_solving_degree":worker["worker_output"]["observed_solving_degree"],"f4_observations":worker["worker_output"]["f4_observations"],"constructor":candidate["answer"]["constructor"],"consumer_stats":candidate["answer"]["consumer"]["stats"],"std_diagnostic":stderr,"case":case}));
    }
    md+=&format!("\n{complete} complete matching outer answers,{failed} failed outer workflows,{recovered} paid recoveries. R denotes recovery. Rust checks every charged recovery sum exactly. Counts are outcomes, not independent timing samples. [pcc:unverified]\n\nWorkflow key:\n\n");
    for (i, name) in names.iter().enumerate() {
        md += &format!("- {}: {}\n", SHORT[i], name);
    }
    md += "\n## All paid single-call ratios (control / Rust)\n\n| Case |";
    for name in &SHORT[..18] {
        md += &format!(" {name} |");
    }
    md += "\n|---|";
    for _ in 0..18 {
        md += "---:|";
    }
    md.push('\n');
    for case in cases {
        md += &format!("| {} |", case["id"].as_str().unwrap());
        if let Some(rows) = case["comparisons"].as_array() {
            if rows.len() != 18 {
                return Err("comparison roster".into());
            }
            for (index, row) in rows.iter().enumerate() {
                let denominator = case["samples"][18]["paid_wall_ns"]
                    .as_u64()
                    .filter(|x| *x > 0)
                    .ok_or("candidate cost")?;
                let numerator = case["samples"][index]["paid_wall_ns"]
                    .as_u64()
                    .filter(|x| *x > 0)
                    .ok_or("control cost")?;
                let computed = numerator as f64 / denominator as f64;
                let recorded = row["full_cost_single_call_ratio"]
                    .as_f64()
                    .ok_or("recorded ratio")?;
                if row["control"] != names[index]
                    || (computed - recorded).abs() > 4. * f64::EPSILON * computed.abs().max(1.)
                {
                    return Err("paid ratio recomputation disagrees".into());
                }
                md += &format!(
                    " {:.3} |",
                    row["full_cost_single_call_ratio"].as_f64().ok_or("ratio")?
                );
            }
        } else {
            for _ in 0..18 {
                md += " unavailable |";
            }
        }
        md.push('\n');
    }
    md+=&format!("\nThe unchanged prospective gate selects {}. Selection requires candidate proof fast path, lower cost than all eighteen controls including old M31 and matched both/final exports and both primary paid ratios>=15. This is admission for a separately frozen paired test, not confirmation. Gains against prior replay and optimized expanded remain distinct and can be much smaller than primary ratios. [pcc:unverified]\n\n## Degree and certificate trace\n\n| Case | Observed F4 degree | Output degree | Root dimension | Resultants | Local product bound | Original product bound | Coprime / all pairs |\n|---|---:|---:|---:|---:|---:|---:|---:|\n",summary["selected_followup_family"]);
    for t in &traces {
        let c = &t["constructor"]["info"];
        let s = &t["consumer_stats"];
        md += &format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} / {} |\n",
            t["id"].as_str().unwrap(),
            t["f4_observed_solving_degree"],
            c["basis_polynomial_degree"],
            c["quotient_dimension"],
            c["projection_work"]["edge_resultants"],
            c["max_local_certificate_product_degree"],
            c["expanded_original_product_degree_upper_bound"],
            s["product_pairs"],
            s["pairs"]
        );
    }
    md+="\nObserved F4 degree, native certificate degree and quotient dimension are different diagnostics. No intrinsic regularity reduction follows. Complete F4 observations, exact consumer counters and failure diagnostics remain in traces.json and the raw archives. [pcc:unverified]\n\n![Complete paid costs](costs.svg){width=85%}\n";
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("traces.json"),
        serde_json::to_vec_pretty(
            &json!({"complete_outputs":complete,"failed_outer_workflows":failed,"recovered_workflows":recovered,"selected_followup_family":summary["selected_followup_family"],"all_reported_ratios_recomputed":true,"cases":traces,"standing":"unsigned one-call L0; no confirmation"}),
        )?,
    )?;
    let min = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) * 0.8;
    let max = points.iter().map(|p| p.1).fold(1., f64::max) * 1.2;
    let path = out.join("costs.svg");
    let root = SVGBackend::new(&path, (1500, 1170)).into_drawing_area();
    root.fill(&WHITE)?;
    let (plot, legend) = root.split_vertically(800);
    let mut chart = ChartBuilder::on(&plot)
        .caption(
            "Native completion costs: all prior and matched controls",
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
            cases
                .get(i.wrapping_sub(1))
                .and_then(|c| c["id"].as_str())
                .unwrap_or("")
                .into()
        })
        .x_desc("Paid milliseconds, log scale; one call, no confidence interval")
        .draw()?;
    for (arm, name) in names.iter().enumerate() {
        chart.draw_series(
            points
                .iter()
                .filter(|p| p.0 == arm)
                .map(|p| Circle::new((p.1, p.2), 5, Palette99::pick(arm).filled())),
        )?;
        let x = 20 + (arm % 3) as i32 * 495;
        let y = 25 + (arm / 3) as i32 * 40;
        legend.draw(&Circle::new((x, y), 5, Palette99::pick(arm).filled()))?;
        legend.draw(&Text::new(
            format!("{}: {name}", SHORT[arm]),
            (x + 14, y + 5),
            ("sans-serif", 16).into_font(),
        ))?;
    }
    root.present()?;
    Ok(())
}
