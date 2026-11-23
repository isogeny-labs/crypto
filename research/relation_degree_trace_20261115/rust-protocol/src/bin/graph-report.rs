//! Recorded graph/order trace. No new targets, timings or selection changes.
use certified_relations::{degree, System};
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn num(v: &Value) -> Result<f64> {
    v.as_f64().ok_or_else(|| "missing number".into())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: graph-report INPUTS RESULTS NEW_REPORT".into());
    }
    let inputs = Path::new(&a[1]);
    let results = Path::new(&a[2]);
    let out = Path::new(&a[3]);
    fs::create_dir(out)?;
    let summary = read(&results.join("summary.json"))?;
    let cases = summary["cases"].as_array().ok_or("cases")?;
    let mut md=String::from("# Graph/order screen: no eligible follow-up\n\nUnsigned producer L0 single-call screen, PCC-20261116-c431fc. All eighteen cases and all three native arms completed and matched three mature references. No family met both paid ratios>=10; no follow-up, confidence interval or confirmed 10x/15x result is claimed. [pcc:unverified]\n\n| Configuration | Graph width | Observed F4 degree | F4 max columns | Compact root | Local proof terms | F4/compact paid | Expanded/compact paid |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut traces = Vec::new();
    let mut bases = BTreeMap::<usize, Value>::new();
    let mut rows = Vec::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("id")?;
        if case["status"] != "screen_complete" {
            md += &format!("| {id} | — | — | — | — | — | — | {} |\n", case["status"]);
            continue;
        }
        let n = case["variables"].as_u64().ok_or("n")? as usize;
        let f4 = read(&results.join(id).join("screen-f4.call.json"))?;
        let compact = read(&results.join(id).join("screen-compact.call.json"))?;
        let reference = read(&results.join(id).join("references.json"))?;
        let same = bases.get(&n).is_none_or(|b| *b == reference["basis"]);
        bases.entry(n).or_insert_with(|| reference["basis"].clone());
        let s: System = serde_json::from_slice(&fs::read(inputs.join(id).join("system.json"))?)?;
        let f = s.decode()?;
        let top_axes_zero = f.iter().all(|poly| {
            let d = degree(poly);
            poly.keys()
                .filter(|m| m.iter().sum::<usize>() == d)
                .all(|m| m.iter().filter(|e| **e != 0).count() >= 2)
        });
        let observed = f4["worker_output"]["observed_solving_degree"].clone();
        let matrix = f4["worker_output"]["f4_observations"].clone();
        let info = compact["worker_output"]["quotient_info"].clone();
        let comparisons = case["comparisons"].as_array().ok_or("comparisons")?;
        let ratio_f = num(&comparisons[0]["full_cost_single_call_ratio"])?;
        let ratio_e = num(&comparisons[1]["full_cost_single_call_ratio"])?;
        md += &format!(
            "| {id} | {} | {observed} | {} | {} | {} | {ratio_f:.3}x | {ratio_e:.3}x |\n",
            case["induced_width_x0_first"],
            matrix["max_columns"],
            info["root_variable"],
            info["stored_proof_terms"]
        );
        traces.push(json!({"id":id,"variables":n,"graph_width_x0_first":case["induced_width_x0_first"],"observed_f4_productive_degree":observed,"f4_matrix":matrix,"compact_certificate":info,"reference_basis_equal_within_dimension":same,"supplied_top_forms_vanish_on_every_coordinate_axis":top_axes_zero,"comparisons":comparisons,"scope":"single call; no confidence or intrinsic-regularity inference"}));
        rows.push((
            format!("{id} [w{},d{observed}]", case["induced_width_x0_first"]),
            ratio_f,
            ratio_e,
        ));
    }
    let same = traces
        .iter()
        .all(|r| r["reference_basis_equal_within_dimension"] == true);
    let axis = traces
        .iter()
        .all(|r| r["supplied_top_forms_vanish_on_every_coordinate_axis"] == true);
    md+=&format!("\nExact mature reference polynomial arrays equal within each variable count: **{same}**. Supplied homogeneous top forms vanish on every coordinate axis in all recorded cases: **{axis}**. These are producer checks on the recorded inputs/results. [pcc:unverified]\n");
    md+="\nThe first-zero-Hilbert-function statistic for these supplied top forms is unavailable: restriction to a coordinate axis leaves nonzero pure powers in every degree. This is not a measurement of intrinsic regularity. Graph width is computed by an x0-first graph elimination, whereas the solver uses grevlex. Neither graph width nor certificate product degree is an F4 solving degree. Only the complete worker's observed productive degree is labelled as such. [pcc:unverified]\n\nThe eight-variable path's paid expanded/compact ratio exceeded15 in its one screening call, while its F4/compact ratio was below1. The prospective rule requires both controls; no family is selected. Do not turn a one-call point into a confidence claim, replace the null selection or reuse original conditional fresh targets. [pcc:unverified]\n\n![Single-call paid ratios](screen.svg){width=85%}\n\nEach point is one planned call, not a median or interval. Blue=F4/compact, orange=expanded/compact. Larger than1 favors compact; lines mark1/10/15. All shared reference generation, preparation and per-call checking are charged. Read raw traces.json for matrix/frontier and certificate diagnostics; no degree-of-regularity improvement is claimed. [pcc:unverified]\n";
    fs::write(out.join("report.md"), &md)?;
    fs::write(
        out.join("traces.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"graph-order-trace/v1","selected_followup_family":summary["selected_followup_family"],"same_reference_basis_per_dimension":same,"top_forms_axis_obstruction":axis,"cases":traces,"standing":"unsigned derived producer trace; no regularity or confirmed-performance verdict"}),
        )?,
    )?;
    let path = out.join("screen.svg");
    let root = SVGBackend::new(&path, (1250, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Paid single-call ratios: no intervals or eligible family",
            ("sans-serif", 23),
        )
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(300)
        .build_cartesian_2d((0.5f64..32f64).log_scale(), 0usize..rows.len() + 1)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_desc("Control / compact; blue=F4, orange=expanded; w=graph width, d=observed F4 degree")
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
            BLACK.mix(0.3),
        )))?;
    }
    for (i, (_, f, e)) in rows.iter().enumerate() {
        let y = rows.len() - i;
        chart.draw_series(std::iter::once(Circle::new((*f, y), 5, BLUE.filled())))?;
        chart.draw_series(std::iter::once(Circle::new(
            (*e, y),
            5,
            RGBColor(220, 110, 15).filled(),
        )))?;
    }
    root.present()?;
    println!("{md}");
    Ok(())
}
