//! Read-only presentation of the frozen native-identity comparison.
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
        return Err("usage:identity-visual RAW AUDIT NEW_OUT".into());
    }
    let raw = Path::new(&a[1]);
    let audit = read(Path::new(&a[2]))?;
    if audit["checked_answers"] != 816 || audit["costs_orders_and_statistics_recomputed"] != true {
        return Err("complete audited816-answer study required".into());
    }
    let cases = audit["cases"]
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("3cases")?;
    let out = Path::new(&a[3]);
    fs::create_dir(out)?;
    let mut md=String::from("# Per-phase medians\n\nEach entry is the median of25 main calls in milliseconds. Phase medians do not add to the median work cost. The frontier's constructor and full replay are inside its native phase; extra preprocessing is not its total construction cost. Ratios use paired block observations in the main report, not ratios of these medians. [pcc:unverified]\n\n|n|Arm|Extra preprocessing|Own preparation|Exact identity check|Native solve/check|Work|Full paid|\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    let mut neutral=String::from("# Neutral controls\n\nFixed21 A/A pairs. Both ordinary95 intervals and the prospective simultaneous screens are shown. No exact finite-sample bootstrap coverage assertion. [pcc:unverified]\n");
    let mut comparisons=String::from("# All registered comparisons\n\nMedian paired ratios and adjusted lower1/720 bounds,25 main blocks each. Full precision remains in report-01/audit.json. [pcc:unverified]\n\n|n|Comparison|Work median|Work lower|Paid median|Paid lower|\n|---|---|---:|---:|---:|---:|\n");
    for c in cases {
        neutral+=&format!("\n## n={}\n\nOrdinary95 all-pass={}, simultaneous all-pass={}.\n\n|Arm/scope|Ordinary low|Ordinary high|Simult. low|Simult. high|\n|---|---:|---:|---:|---:|\n",c["n"],c["aa_statistics"]["ordinary_95_admitted"],c["aa_admitted"]);
        for (k, name) in [
            "prefix-S work",
            "prefix-S paid",
            "prefix-R work",
            "prefix-R paid",
            "frontier-S work",
            "frontier-S paid",
            "frontier-R work",
            "frontier-R paid",
        ]
        .iter()
        .enumerate()
        {
            let o = &c["aa_statistics"]["ordinary95"][k]["percentile_95"];
            let a = &c["aa_statistics"]["simultaneous_screens"][k];
            neutral += &format!(
                "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                o[0].as_f64().ok_or("neutral")?,
                o[1].as_f64().ok_or("neutral")?,
                a[0].as_f64().ok_or("neutral")?,
                a[1].as_f64().ok_or("neutral")?
            );
        }
        for (j, name) in [
            "prefix-S / prefix-R",
            "frontier-R / prefix-R",
            "frontier-S / frontier-R",
        ]
        .iter()
        .enumerate()
        {
            let w = &c["result"]["statistics"][2 * j];
            let p = &c["result"]["statistics"][2 * j + 1];
            comparisons += &format!(
                "|{}|{name}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                c["n"],
                w["median_paired_ratio"].as_f64().ok_or("median")?,
                w["adjusted_lower_1_over_720"].as_f64().ok_or("lower")?,
                p["median_paired_ratio"].as_f64().ok_or("median")?,
                p["adjusted_lower_1_over_720"].as_f64().ok_or("lower")?
            );
        }
    }
    fs::write(out.join("neutral.md"), neutral)?;
    fs::write(out.join("comparisons.md"), comparisons)?;
    let mut rows = vec![];
    for c in cases {
        let n = c["n"].as_u64().ok_or("n")?;
        let v = read(&raw.join(format!("n{n}/outcome.json")))?;
        let blocks = v["main"]
            .as_array()
            .filter(|v| v.len() == 25)
            .ok_or("main")?;
        for (i, name) in [
            "prefix-singular",
            "prefix-rust",
            "frontier-singular",
            "frontier-rust",
        ]
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
        let mut max_ratio = 1.0f64;
        let mut min_ratio = 1.0f64;
        for c in cases {
            for j in 0..3 {
                let st = &c["result"]["statistics"][2 * j + scope];
                max_ratio = max_ratio.max(st["median_paired_ratio"].as_f64().ok_or("median")?);
                min_ratio = min_ratio.min(st["adjusted_lower_1_over_720"].as_f64().ok_or("lower")?);
            }
        }
        let mut chart = ChartBuilder::on(area)
            .caption(
                format!(
                    "{}: median (blue), adjusted lower1/720 (red); fixed development study",
                    if scope == 0 { "Work" } else { "Full paid" }
                ),
                ("sans-serif", 22),
            )
            .margin(20)
            .x_label_area_size(50)
            .y_label_area_size(65)
            .build_cartesian_2d(0usize..9usize, (min_ratio * 0.95)..(max_ratio * 1.08))?;
        chart
            .configure_mesh()
            .x_desc("0..2 n3;3..5 n4;6..8 n6; prefix-S/R, frontier-R/prefix-R, frontier-S/R")
            .y_desc("Matched control / Rust candidate")
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
