//! Presentation only: plot unchanged recorded checkpoint ranks and qualified costs.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{File, OpenOptions},
};
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 5 {
        return Err("usage: RENDER FULL_RAW REMAINDER_RAW SUMMARY OUT_SVG NEW_PRESENTATION".into());
    }
    let raw: [Value; 2] = [
        serde_json::from_reader(File::open(&a[0])?)?,
        serde_json::from_reader(File::open(&a[1])?)?,
    ];
    let summary: Value = serde_json::from_reader(File::open(&a[2])?)?;
    let names = ["Full smooth", "Quadratic remainders"];
    let colors = [RGBColor(42, 90, 150), RGBColor(190, 75, 45)];
    let curves: Vec<Vec<(f64, f64)>> = raw
        .iter()
        .map(|r| {
            r["checkpoints"]
                .as_array()
                .ok_or("checkpoints")?
                .iter()
                .map(|c| {
                    Ok((
                        c["b_count"].as_u64().ok_or("B count")? as f64 / 1000.0,
                        c["rank"].as_u64().ok_or("rank")? as f64,
                    ))
                })
                .collect::<Result<_, Box<dyn Error>>>()
        })
        .collect::<Result<_, _>>()?;
    let max_b = raw
        .iter()
        .map(|r| r["counts"]["b_count"].as_u64().unwrap_or(0))
        .max()
        .ok_or("B endpoint")? as f64
        / 1000.0;
    let target = raw[0]["rank_target"].as_u64().ok_or("target")? as f64;
    let values: Vec<u128> = summary["outcomes"]
        .as_array()
        .ok_or("outcomes")?
        .iter()
        .map(|r| {
            r["observed_incomplete_bitops"]
                .as_str()
                .ok_or("cost")?
                .parse()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .collect::<Result<_, _>>()?;
    let root = SVGBackend::new(&a[3], (1120, 620)).into_drawing_area();
    root.fill(&WHITE)?;
    let areas = root.split_evenly((2, 1));
    let mut chart = ChartBuilder::on(&areas[0])
        .caption(
            "Actual homogeneous rank at complete batches",
            ("sans-serif", 22),
        )
        .margin(15)
        .x_label_area_size(40)
        .y_label_area_size(65)
        .build_cartesian_2d(0f64..max_b * 1.03, 0f64..target + 10.0)?;
    chart
        .configure_mesh()
        .x_labels(7)
        .y_labels(6)
        .max_light_lines(0)
        .x_desc("Enumerated B polynomials (thousands)")
        .y_desc("Rank over subgroup field")
        .draw()?;
    for i in 0..2 {
        chart
            .draw_series(LineSeries::new(
                curves[i].iter().copied(),
                colors[i].stroke_width(2),
            ))?
            .label(names[i])
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 25, y)], colors[i].stroke_width(2))
            });
    }
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.85))
        .border_style(BLACK)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;
    let max_cost = *values.iter().max().ok_or("costs")? as f64 / 1e12;
    let costs = &areas[1];
    costs.draw(&Text::new(
        "Observed incomplete primitive subtotal (complete costs unavailable)",
        (15, 28),
        ("sans-serif", 20),
    ))?;
    for i in 0..2 {
        let value = values[i] as f64 / 1e12;
        let y = 65 + 85 * i as i32;
        let end = 200 + (value / max_cost * 670.0).round() as i32;
        costs.draw(&Text::new(names[i], (12, y + 26), ("sans-serif", 17)))?;
        costs.draw(&Rectangle::new(
            [(200, y), (end, y + 42)],
            colors[i].mix(0.7).filled(),
        ))?;
        costs.draw(&Text::new(
            values[i].to_string(),
            (end + 10, y + 26),
            ("sans-serif", 16),
        ))?;
    }
    costs.draw(&Text::new(
        "Bar lengths: recorded primitive bitops. Exact labels; no complete-cost ratios.",
        (200, 250),
        ("sans-serif", 16),
    ))?;
    root.present()?;
    let projected = json!({"schema":"cover.extension-remainder.presentation/v1","source_revision":summary["source_revision"],"outcomes":summary["outcomes"],"common_complete_batch_prefixes":summary["common_complete_batch_prefixes"],"rank_curves":raw.iter().map(|r|r["checkpoints"].as_array().unwrap().iter().map(|c|json!({"b_count":c["b_count"],"rank":c["rank"]})).collect::<Vec<_>>()).collect::<Vec<_>>(),"complete_cost":false});
    serde_json::to_writer_pretty(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&a[4])?,
        &projected,
    )?;
    Ok(())
}
