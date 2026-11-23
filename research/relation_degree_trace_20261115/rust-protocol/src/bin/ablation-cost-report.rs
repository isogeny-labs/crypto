//! Descriptive formatting of already audited, fixed ablation data. No solver runs.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn ms(v: &Value) -> String {
    v.as_u64()
        .map(|n| format!("{:.6}", n as f64 / 1e6))
        .unwrap_or("null".into())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: ablation-cost-report RAW AUDIT NEW_OUT".into());
    }
    let raw = Path::new(&a[1]);
    let audit = read(Path::new(&a[2]))?;
    let out = Path::new(&a[3]);
    if audit["raw_identities_and_completed_bases_replayed"] != true
        || audit["mode"] != "development"
    {
        return Err("requires audited development data".into());
    }
    let cells = audit["cells"]
        .as_array()
        .filter(|c| c.len() == 18)
        .ok_or("18 cells required")?;
    let mut md=String::from("# Matrix, certificate and paid-cost detail\n\nRead-only Rust formatting of the audited18 fixed development observations. All times are single calls in milliseconds; no timing confidence or speed claim. F4 matrix nonzeros/fill are absent from this worker. The terminal affine diagnostic nonzeros and row reductions below belong to the separate closure algorithm. [pcc:unverified]\n\n| n | Class | Terminal degree | Diagnostic new row nnz | Diagnostic reductions | Projection terms | Projection product degree |\n|---|---|---:|---:|---:|---:|---:|\n");
    let mut costs=String::from("\n| n | Class | Generation | Full construction | Preparation | Identity check | All3 refs | Checked F4 | Fastest external | External ms | Separate diagnostic |\n|---|---|---:|---:|---:|---:|---:|---:|---|---:|---:|\n");
    let mut details = Vec::new();
    for c in cells {
        let v = &c["cell"];
        let n = v["n"].as_u64().ok_or("n")?;
        let class = v["class"].as_str().ok_or("class")?;
        let dir = raw.join(format!("n{n}-{class}"));
        let info = read(&dir.join("identities.json"))?;
        let selected = info["vertices"]["selected_vertices"].as_array();
        if selected.is_some_and(|s| s.len() > 1) {
            return Err("fixed star unexpectedly selected multiple projections".into());
        }
        let projection = selected
            .and_then(|s| s.first())
            .cloned()
            .unwrap_or(Value::Null);
        let last = c["affine_trace"]["trace"]
            .as_array()
            .and_then(|t| t.last())
            .ok_or("terminal trace event")?;
        md += &format!(
            "|{n}|{class}|{}|{}|{}|{}|{}|\n",
            last["degree"],
            last["new_row_nonzeros"],
            last["row_reductions"],
            projection["certificate_terms"],
            projection["certificate_product_degree"]
        );
        let refs = read(&dir.join("references.json"))?;
        let fastest = refs["routes"]
            .as_array()
            .ok_or("reference routes")?
            .iter()
            .min_by_key(|r| r["whole_route_ns"].as_u64().unwrap_or(u64::MAX))
            .ok_or("no routes")?;
        costs += &format!(
            "|{n}|{class}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            ms(&v["generation_ns"]),
            ms(&v["construction_ns"]),
            ms(&v["preparation_ns"]),
            ms(&v["identity_check_ns"]),
            ms(&v["reference_ns"]),
            ms(&v["checked_f4_ns"]),
            fastest["route"],
            ms(&fastest["whole_route_ns"]),
            ms(&v["separate_diagnostic_ns"])
        );
        details.push(json!({"n":n,"class":class,"projection":projection,"terminal_affine_event":last,"costs":v,"fastest_external":fastest}));
    }
    md += &costs;
    md+="\nReference clocks include process launch, basis parsing and route checking. The checked F4 clock includes process launch, the unchanged native solver/checks and full equality consumption. These whole-route clocks are descriptive and are not a controlled timing comparison against the fastest external route. Full paid costs also include all three reference routes. Oracle construction is the separately recorded reference fee already included in paid F4 cost. Diagnostic child time is recorded outside that paid cost and is never counted as an F4 speed ratio. Projection product degree measures original-input identity products, not solving degree. [pcc:unverified]\n\n![All matrix observations](matrices.svg){width=90%}\n";
    fs::create_dir(out)?;
    fs::write(out.join("detail.md"), md)?;
    fs::write(
        out.join("detail.json"),
        serde_json::to_vec_pretty(&json!({"source_audit":a[2],"cells":details}))?,
    )?;
    let file = out.join("matrices.svg");
    let plot = SVGBackend::new(&file, (1500, 700)).into_drawing_area();
    plot.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&plot)
        .caption(
            "All18 fixed cells: maximum F4 rows (blue) and columns (red)",
            ("sans-serif", 24),
        )
        .margin(25)
        .x_label_area_size(65)
        .y_label_area_size(65)
        .build_cartesian_2d(0usize..18usize, 0u32..500u32)?;
    chart
        .configure_mesh()
        .x_desc("0..5 n3; 6..11 n4; 12..17 n6; protocol class order")
        .y_desc("Maximum in unchanged F4 report")
        .draw()?;
    for (key, color) in [("max_rows", BLUE), ("max_columns", RED)] {
        chart.draw_series(cells.iter().enumerate().filter_map(|(i, c)| {
            c["f4"]["f4_observations"][key]
                .as_u64()
                .map(|n| Circle::new((i, n as u32), 5, color.filled()))
        }))?;
    }
    plot.present()?;
    Ok(())
}
