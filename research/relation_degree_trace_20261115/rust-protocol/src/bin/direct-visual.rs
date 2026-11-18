//! Read-only presentation of the frozen direct-construction comparison.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn med(mut v: Vec<u64>) -> Result<u64> {
    if v.len() != 25 {
        return Err("25 observations required".into());
    }
    v.sort_unstable();
    Ok(v[12])
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage:direct-visual RAW AUDIT NEW_OUT".into());
    }
    let raw = Path::new(&a[1]);
    let audit = read(Path::new(&a[2]))?;
    if audit["checked_answers"] != 480 || audit["costs_orders_and_statistics_recomputed"] != true {
        return Err("complete audited480-answer study required".into());
    }
    let cases = audit["cases"]
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("3cases")?;
    let out = Path::new(&a[3]);
    fs::create_dir(out)?;
    let mut md=String::from("# Per-phase medians\n\nEach entry is the median of25 main calls in milliseconds. Phase medians do not add to the median work cost. The frontier's constructor and full replay are inside its native phase; extra preprocessing is not its total construction cost. Ratios use paired block observations in the main report, not ratios of these medians. [pcc:unverified]\n\n|n|Arm|Extra preprocessing|Own preparation|Fresh identity check|Native solve/check|Work|Full paid|\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    let mut rows = vec![];
    for c in cases {
        let n = c["n"].as_u64().ok_or("n")?;
        let v = read(&raw.join(format!("n{n}/outcome.json")))?;
        let blocks = v["main"]
            .as_array()
            .filter(|v| v.len() == 25)
            .ok_or("main")?;
        for (i, name) in ["rank", "direct-all", "direct-prefix", "frontier"]
            .iter()
            .enumerate()
        {
            let mut phases = serde_json::Map::new();
            for k in [
                "construction_ns",
                "preparation_ns",
                "identity_ns",
                "solve_ns",
            ] {
                phases.insert(
                    k.into(),
                    json!(med(blocks
                        .iter()
                        .map(|b| b["samples"][i]["phases"][k].as_u64().ok_or("phase"))
                        .collect::<std::result::Result<Vec<_>, _>>()?)?),
                );
            }
            let work = med(blocks
                .iter()
                .map(|b| b["samples"][i]["work_ns"].as_u64().ok_or("work"))
                .collect::<std::result::Result<Vec<_>, _>>()?)?;
            let paid = med(blocks
                .iter()
                .map(|b| b["samples"][i]["paid_ns"].as_u64().ok_or("paid"))
                .collect::<std::result::Result<Vec<_>, _>>()?)?;
            md += &format!(
                "|{n}|{name}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                phases["construction_ns"].as_u64().unwrap() as f64 / 1e6,
                phases["preparation_ns"].as_u64().unwrap() as f64 / 1e6,
                phases["identity_ns"].as_u64().unwrap() as f64 / 1e6,
                phases["solve_ns"].as_u64().unwrap() as f64 / 1e6,
                work as f64 / 1e6,
                paid as f64 / 1e6
            );
            rows.push(json!({"n":n,"arm":name,"phase_medians_ns":phases,"work_median_ns":work,"paid_median_ns":paid}));
        }
    }
    md += "\n![Every registered comparison](ratios.svg){width=90%}\n";
    fs::write(out.join("phases.md"), md)?;
    fs::write(out.join("phases.json"), serde_json::to_vec_pretty(&rows)?)?;
    let file = out.join("ratios.svg");
    let root = SVGBackend::new(&file, (1500, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    let areas = root.split_evenly((2, 1));
    for (scope, area) in areas.iter().enumerate() {
        let mut chart = ChartBuilder::on(area)
            .caption(
                format!(
                    "{}: median (blue), adjusted lower1/720 (red); n4/n6 A/A rejected",
                    if scope == 0 { "Work" } else { "Full paid" }
                ),
                ("sans-serif", 22),
            )
            .margin(20)
            .x_label_area_size(50)
            .y_label_area_size(65)
            .build_cartesian_2d(0usize..9usize, 0.85f64..1.45f64)?;
        chart
            .configure_mesh()
            .x_desc("0..2 n3;3..5 n4;6..8 n6;rank/direct-all/frontier")
            .y_desc("Control / direct-prefix")
            .draw()?;
        chart.draw_series(LineSeries::new([(0, 1.), (9, 1.)], BLACK))?;
        for (i, c) in cases.iter().enumerate() {
            for j in 0..3 {
                let s = &c["result"]["statistics"][2 * j + scope];
                let m = s["median_paired_ratio"].as_f64().ok_or("median")?;
                let l = s["adjusted_lower_1_over_720"].as_f64().ok_or("lower")?;
                chart.draw_series([Circle::new((i * 3 + j, m), 5, BLUE.filled())])?;
                chart.draw_series([TriangleMarker::new((i * 3 + j, l), 6, RED.filled())])?;
            }
        }
    }
    root.present()?;
    Ok(())
}
