//! Read-only presentation of the already audited fixed early-rank study.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: rank-visual AUDIT NEW_OUT".into());
    }
    let v: Value = serde_json::from_slice(&fs::read(&args[1])?)?;
    if v["mode"] != "development" || v["all_rank_decisions_and_raw_proofs_replayed"] != true {
        return Err("audited development data required".into());
    }
    let cells = v["cells"]
        .as_array()
        .filter(|a| a.len() == 18)
        .ok_or("18 fixed cells")?;
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    let mut md=String::from("# Exact exported certificate costs\n\nValues below describe exported rows; unused candidate discovery and rank selection are still fully paid. Product degree belongs to original-input membership identities, not F4 solving degree. All18 cells remain; single-call timing gains are unconfirmed. [pcc:unverified]\n\n|n|Class|Selected ids|Total polynomial terms|Total proof terms|Max certificate product degree|\n|---|---|---|---:|---:|---:|\n");
    let mut summaries = vec![];
    for c in cells {
        let m = &c["selection"]["selection"];
        let ids = m["selected_ids"].as_array().ok_or("ids")?;
        let candidates = m["candidates"].as_array().ok_or("candidates")?;
        let mut poly = 0u64;
        let mut proof = 0u64;
        let mut degree = None;
        for id in ids {
            let row = &candidates[id.as_u64().ok_or("id")? as usize];
            poly = poly
                .checked_add(row["poly_terms"].as_u64().ok_or("poly terms")?)
                .ok_or("term overflow")?;
            proof = proof
                .checked_add(row["proof_terms"].as_u64().ok_or("proof terms")?)
                .ok_or("proof overflow")?;
            let d = row["certificate_product_degree"].as_u64().ok_or("degree")?;
            degree = Some(degree.unwrap_or(0).max(d));
        }
        md += &format!(
            "|{}|{}|{}|{poly}|{proof}|{}|\n",
            c["cell"]["n"],
            c["cell"]["class"],
            json!(ids),
            json!(degree)
        );
        summaries.push(json!({"n":c["cell"]["n"],"class":c["cell"]["class"],"selected_ids":ids,"polynomial_terms":poly,"proof_terms":proof,"certificate_product_degree":degree}));
    }
    md+="\n![Actual F4 degree](degree.svg){width=90%}\n\n![All matrix dimensions](matrix.svg){width=90%}\n";
    fs::write(out.join("certificates.md"), md)?;
    fs::write(
        out.join("certificates.json"),
        serde_json::to_vec_pretty(&summaries)?,
    )?;
    let file = out.join("degree.svg");
    let root = SVGBackend::new(&file, (1500, 650)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "All18 cells: actual F4 degree (blue), original star reference degree5 (red)",
            ("sans-serif", 23),
        )
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0usize..18usize, 0u32..7u32)?;
    chart
        .configure_mesh()
        .x_desc("0..5 n3; 6..11 n4; 12..17 n6; fixed sixarm order")
        .y_desc("Observed degree")
        .draw()?;
    chart.draw_series(LineSeries::new([(0, 5), (18, 5)], RED))?;
    chart.draw_series(cells.iter().enumerate().filter_map(|(i, c)| {
        c["f4"]["observed_solving_degree"]
            .as_u64()
            .map(|d| Circle::new((i, d as u32), 5, BLUE.filled()))
    }))?;
    root.present()?;
    let file = out.join("matrix.svg");
    let root = SVGBackend::new(&file, (1500, 650)).into_drawing_area();
    root.fill(&WHITE)?;
    let max = cells
        .iter()
        .flat_map(|c| {
            [
                c["f4"]["f4_observations"]["max_rows"].as_u64(),
                c["f4"]["f4_observations"]["max_columns"].as_u64(),
            ]
        })
        .flatten()
        .max()
        .ok_or("matrix observations")?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "All18 cells: maximum F4 rows (blue) and columns (red)",
            ("sans-serif", 23),
        )
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0usize..18usize, 0u32..max as u32 + 10)?;
    chart
        .configure_mesh()
        .x_desc("0..5 n3; 6..11 n4; 12..17 n6; fixed sixarm order")
        .y_desc("Unchanged F4 worker maximum")
        .draw()?;
    for (key, color) in [("max_rows", BLUE), ("max_columns", RED)] {
        chart.draw_series(cells.iter().enumerate().filter_map(|(i, c)| {
            c["f4"]["f4_observations"][key]
                .as_u64()
                .map(|d| Circle::new((i, d as u32), 5, color.filled()))
        }))?;
    }
    root.present()?;
    Ok(())
}
