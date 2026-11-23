//! Render the recorded pilot; no measurements or target generation.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn number(v: &Value) -> Result<f64> {
    v.as_f64().ok_or_else(|| "missing numeric field".into())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: local-report RESULT_DIR NEW_REPORT_DIR".into());
    }
    let input = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    fs::create_dir(out)?;
    let summary: Value = serde_json::from_slice(&fs::read(input.join("summary.json"))?)?;
    let cases = summary["cases"].as_array().ok_or("missing cases")?;
    let mut md=String::from("# Local compact pilot: actual measured outcome\n\nUnsigned producer L0 observations, PCC-20261116-1075bb; no independent replay, L2 or fresh-family confirmation. The user's checkpoint waiver enabled this bounded local pilot. All three mature CAS routes agreed before native samples.\n\nThe full-cost 10x/15x target was **not met**. Compact beats the expanded-certificate constructor on the admitted cases, but optimized F4 remains faster. Worker-call intervals include launch/consumption; full cost also charges all shared reference setup and per-call checking to every arm. [pcc:unverified]\n\n| Case | F4 / compact worker | Expanded / compact worker | F4 / compact full | Expanded / compact full | Status |\n|---|---:|---:|---:|---:|---|\n");
    let mut rows = Vec::new();
    let mut bounds = Vec::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("missing id")?;
        let Some(comparisons) = case["comparisons"].as_array() else {
            md += &format!(
                "| {id} | — | — | — | — | {} |\n",
                case["reason"].as_str().unwrap_or("incomplete")
            );
            continue;
        };
        let mut vals = Vec::new();
        for c in comparisons {
            vals.push((
                number(&c["native"]["median_paired_ratio"])?,
                number(&c["full_cost"]["median_paired_ratio"])?,
            ));
            rows.push((
                format!("{id}/{}", c["control"].as_str().ok_or("control")?),
                c["control"] == "f4",
                c.clone(),
            ));
        }
        md += &format!(
            "| {id} | {:.3}x | {:.3}x | {:.3}x | {:.3}x | A/A admitted, 21 blocks |\n",
            vals[0].0, vals[1].0, vals[0].1, vals[1].1
        );
        let transcript: Value =
            serde_json::from_slice(&fs::read(input.join(id).join("transcript.json"))?)?;
        let setup = number(&transcript["setup_wall_ns"])?;
        let mut ceiling = 0f64;
        for block in transcript["blocks"].as_array().ok_or("blocks")? {
            for s in block.as_array().ok_or("block")? {
                if s["arm"] == "f4" {
                    ceiling = ceiling.max(
                        (setup + number(&s["worker_wall_ns"])? + number(&s["validation_wall_ns"])?)
                            / setup,
                    );
                }
            }
        }
        bounds.push(json!({"id":id,"maximum_measured_f4_full_cost_over_shared_setup":ceiling,"meaning":"conditional upper ceiling for any zero-compute/check candidate on these measured controls with the SAME paid setup; not a cross-host or different-input impossibility result"}));
    }
    md+="\nRatios are control/candidate, so values below one mean compact is slower. Four A/A-rejected cases remain failures with missing remainder, never replaced. Each successful case uses the frozen 21-block order, 100000 joint resamples, seed 20261115 and adjusted lower quantile 1/720. Every successful six-variable worker-call expanded/compact adjusted lower endpoint exceeds 10, but its full-cost endpoints do not. This component result does not meet the overall target. [pcc:unverified]\n\nShared reference setup is a large common cost. The following arithmetic ceilings hold only for these measured control samples with that same setup: even eliminating all candidate compute/check work leaves the paid setup. They do not constrain a different input family or validation design.\n\n| Case | Largest measured F4 full-cost / shared-setup ceiling |\n|---|---:|\n";
    for b in &bounds {
        md += &format!(
            "| {} | {:.3}x |\n",
            b["id"].as_str().unwrap(),
            number(&b["maximum_measured_f4_full_cost_over_shared_setup"])?
        );
    }
    md+="\nThe next search should target configurations where generic F4 costs dominate, while certified quotient construction remains small. Remeasuring the same small cycles or dropping validation cannot establish the registered full-cost target. No conditional fresh target was generated.\n\n![Measured paired ratios](ratios.svg)\n\nPoints are paired-ratio medians, bars are ordinary 95% bootstrap intervals, and short ticks mark adjusted lower endpoints. Threshold lines are 10 and 15. Raw inputs, commands, stdout/stderr, transcripts and failures are retained in the raw archive; executable and library hashes match before/after. Full raw schema and claims remain in the research directory and benchmark record.\n";
    fs::write(out.join("report.md"), &md)?;
    fs::write(
        out.join("conditional-ceilings.json"),
        serde_json::to_vec_pretty(&bounds)?,
    )?;
    let svg = out.join("ratios.svg");
    let root = SVGBackend::new(&svg, (1200, 760)).into_drawing_area();
    root.fill(&WHITE)?;
    let panels = root.split_evenly((1, 2));
    for (panel, scope, title) in [
        (
            panels[0].clone(),
            "native",
            "Worker call: launch through consumption",
        ),
        (
            panels[1].clone(),
            "full_cost",
            "Full cost: reference and checking paid",
        ),
    ] {
        let mut chart = ChartBuilder::on(&panel)
            .caption(title, ("sans-serif", 19))
            .margin(15)
            .x_label_area_size(55)
            .y_label_area_size(130)
            .build_cartesian_2d((0.3f64..32f64).log_scale(), 0usize..rows.len() + 1)?;
        chart
            .configure_mesh()
            .disable_y_mesh()
            .x_desc("Control / compact (higher is faster)")
            .y_labels(rows.len() + 1)
            .y_label_formatter(&|y| {
                if *y > 0 && *y <= rows.len() {
                    rows[rows.len() - *y].0.clone()
                } else {
                    String::new()
                }
            })
            .draw()?;
        for t in [1., 10., 15.] {
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(t, 0), (t, rows.len() + 1)],
                BLACK.mix(0.25),
            )))?;
        }
        for (i, (_, f4, c)) in rows.iter().enumerate() {
            let y = rows.len() - i;
            let stat = &c[scope];
            let m = number(&stat["median_paired_ratio"])?;
            let lo = number(&stat["percentile_95"][0])?;
            let hi = number(&stat["percentile_95"][1])?;
            let adj = number(&stat["adjusted_lower_1_over_720"])?;
            let color = if *f4 { BLUE } else { RGBColor(220, 110, 15) };
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(lo, y), (hi, y)],
                color.stroke_width(2),
            )))?;
            chart.draw_series(std::iter::once(Circle::new((m, y), 4, color.filled())))?;
            chart.draw_series(std::iter::once(Cross::new((adj, y), 5, color)))?;
        }
    }
    root.present()?;
    let embedded = fs::read_to_string(&svg)?;
    let html=format!("<!doctype html><meta charset=utf-8><title>Local compact pilot</title><style>@page{{size:landscape;margin:15mm}}body{{font:13px sans-serif}}pre{{white-space:pre-wrap}}svg{{max-width:100%;height:auto}}</style><h1>Local compact pilot: full-cost target not met</h1><p>Unsigned producer L0 result. The six-variable worker-call gain over expanded certificates exceeds 10x; the paid full-cost gain is below 3x and optimized F4 remains faster. Four failed A/A cases are retained. No formal confirmation.</p>{embedded}<pre>{}</pre>",md.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;"));
    fs::write(out.join("report.html"), html)?;
    println!("{md}");
    Ok(())
}
