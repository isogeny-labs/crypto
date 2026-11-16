//! Read-only recorded costs, exact recovery accounting and degree diagnostics.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: sparse-report RESULTS NEW_REPORT".into());
    }
    let input = Path::new(&a[1]);
    let out = Path::new(&a[2]);
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
    let mut md=format!("# Sparse paid pipeline screen\n\n{} fixed development cases; follow-up selection {}. Source/protocol/qualification published before the registered run. Unsigned local L0 single-call evidence, PCC-20261116-f8e097; no confidence interval or confirmed15x result. [pcc:unverified]\n\nEvery outer workflow returns the same complete validated reduced grevlex basis. Paid costs include construction, preparation, proof production/serialization/export, process launch, checking and consumption. Regeneration charges all three mature routes. A failed proof followed by fresh regeneration pays both attempts; no old record or failure is reclassified. [pcc:unverified]\n\n## Paid milliseconds\n\n| Case |",cases.len(),summary["selected_followup_family"]);
    for name in &names {
        md += &format!(" {name} |");
    }
    md += "\n|---|";
    for _ in &names {
        md += "---:|";
    }
    md.push('\n');
    let (mut complete, mut failed, mut recovered) = (0usize, 0usize, 0usize);
    let mut points = Vec::new();
    let mut traces = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        let id = case["id"].as_str().ok_or("id")?;
        let samples = case["samples"].as_array().ok_or("samples")?;
        if samples.len() != names.len() {
            return Err("missing workflow roster".into());
        }
        md += &format!("| {id} |");
        for (arm, s) in samples.iter().enumerate() {
            if s["status"] == "complete" {
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
                let first = s["first_attempt"]["charged_work_ns"]
                    .as_u64()
                    .ok_or("failed work")?;
                let recovery = s["recovery_wall_ns"].as_u64().ok_or("recovery work")?;
                let prep = s["preparation_ns"].as_u64().ok_or("prep")?;
                let equal = s["cross_pipeline_equality_ns"].as_u64().ok_or("equality")?;
                let sum = first
                    .checked_add(recovery)
                    .and_then(|x| x.checked_add(prep))
                    .and_then(|x| x.checked_add(equal))
                    .ok_or("overflow")?;
                if s["paid_wall_ns"] != sum {
                    return Err("recovery accounting mismatch".into());
                }
            }
        }
        md.push('\n');
        let worker: Value =
            serde_json::from_slice(&fs::read(input.join(id).join("f4-regeneration.call.json"))?)?;
        let candidate = samples.last().ok_or("candidate")?;
        let stderr = fs::read_to_string(input.join(id).join("fused-std/call.stderr"))?;
        traces.push(json!({"id":id,"f4_observed_solving_degree":worker["worker_output"]["observed_solving_degree"],"f4_observations":worker["worker_output"]["f4_observations"],"sparse_constructor":candidate["answer"]["constructor"],"explicit_cofactor_terms":candidate["answer"]["explicit_cofactor_terms"],"coprime_pairs":candidate["answer"]["coprime_pairs"],"total_pairs":candidate["answer"]["total_pairs"],"std_first_attempt_stderr":stderr,"case":case}));
    }
    md+=&format!("\n{complete} complete outer answers,{failed} failed outer workflows,{recovered} workflows using paid recovery. R marks recovery in the table; inner failures remain failures. The Rust report checks paid recovery sums exactly. These are one-call observations, not paired estimates. [pcc:unverified]\n\n## Ratios against the frozen candidate\n\n| Case | F4+regeneration / candidate | Expanded+regeneration / candidate | Legacy replay / candidate | Fused expanded / candidate | Full-projection ablation / candidate |\n|---|---:|---:|---:|---:|---:|\n");
    for case in cases {
        let rows = case["comparisons"]
            .as_array()
            .ok_or("complete-roster comparisons")?;
        md += &format!("| {} |", case["id"].as_str().unwrap());
        for name in [
            "f4-regeneration",
            "expanded-regeneration",
            "legacy-compact-replay",
            "fused-expanded",
            "fused-compact-ablation",
        ] {
            let row = rows
                .iter()
                .find(|r| r["control"] == name)
                .ok_or("comparison")?;
            md += &format!(
                " {:.3} |",
                row["full_cost_single_call_ratio"].as_f64().ok_or("ratio")?
            );
        }
        md.push('\n');
    }
    md+="\nAll nine required controls and the ablation remain in the complete raw comparison arrays. No case satisfies the prospectively fixed follow-up gate. In particular, no case reaches10 against both primary paid baselines; the sparse candidate also loses to its full-projection ablation on some calls. Neither a favorable single F4 comparison nor large ratios against expensive mature certificates establish15x overall. [pcc:unverified]\n\n## Degree and certificate trace\n\n| Case | F4 observed solving degree | Root quotient dimension | Output degree | Local certificate product degree | Original-product upper bound | Root resultants | Coprime / total pairs |\n|---|---:|---:|---:|---:|---:|---:|---:|\n";
    for t in &traces {
        let c = &t["sparse_constructor"]["info"];
        md += &format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} / {} |\n",
            t["id"].as_str().unwrap(),
            t["f4_observed_solving_degree"],
            c["quotient_dimension"],
            c["basis_polynomial_degree"],
            c["max_local_certificate_product_degree"],
            c["expanded_original_product_degree_upper_bound"],
            c["projection_work"]["edge_resultants"],
            t["coprime_pairs"],
            t["total_pairs"]
        );
    }
    md+="\nThe constructor degree bounds and the observed F4 solving degree describe different computations. They establish no intrinsic degree-of-regularity reduction. The full F4 trace is retained beside each certificate trace. Root-subset work fell structurally, but its incremental time effect is mixed at this single-call scope. [pcc:unverified]\n\n## Retained first-attempt diagnostics\n\n";
    for t in &traces {
        let e = t["std_first_attempt_stderr"].as_str().unwrap();
        if !e.is_empty() {
            md += &format!(
                "- {}: `{}`\n",
                t["id"].as_str().unwrap(),
                e.trim().replace('`', "'")
            );
        }
    }
    md+="\n![Paid costs with recovery included](costs.svg){width=85%}\n\nNext discriminating step: measure internal construction, witness completion, export, serialization and arithmetic-checker phases prospectively; keep current outcomes frozen. No conditional fresh-family target was generated. [pcc:unverified]\n";
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("traces.json"),
        serde_json::to_vec_pretty(
            &json!({"complete_outputs":complete,"failed_outer_workflows":failed,"recovered_workflows":recovered,"selected_followup_family":summary["selected_followup_family"],"cases":traces,"standing":"unsigned one-call L0; no confirmation"}),
        )?,
    )?;
    let min = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) * 0.8;
    let max = points.iter().map(|p| p.1).fold(1., f64::max) * 1.2;
    let path = out.join("costs.svg");
    let root = SVGBackend::new(&path, (1400, 950)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Sparse paid workflows: recovery included",
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
        .x_desc("Paid milliseconds, logarithmic scale; one call, no confidence interval")
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
