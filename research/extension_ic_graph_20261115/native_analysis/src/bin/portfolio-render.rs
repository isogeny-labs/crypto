//! Post-call presentation only. Does not change the immutable qualifier or raw outputs.
use plotters::prelude::*;
use serde_json::Value;
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: portfolio-render SUMMARY.json NEW-OUTPUT.svg".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("presentation output already exists".into());
    }
    let summary: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if summary["schema"] != "cover.ic-graph-rho-portfolio-analysis/v1"
        || summary["complete_cost"] != false
        || summary["comparative_eligibility"] != false
        || !summary["cost_ratios"].is_null()
    {
        return Err("presentation scope".into());
    }
    let rows = summary["outcomes"].as_array().ok_or("outcomes")?;
    if rows.len() != 6 {
        return Err("six arms required".into());
    }
    let costs: Vec<u128> = rows
        .iter()
        .map(|r| {
            r["accounting"]["observed_primitive_subtotal_bitops"]
                .as_str()
                .ok_or("subtotal")?
                .parse()
                .map_err(|_| "integer subtotal")
        })
        .collect::<Result<_, _>>()?;
    let largest = *costs.iter().max().ok_or("empty")?;
    if largest == 0 {
        return Err("zero costs".into());
    }
    let root = SVGBackend::new(&args[2], (1120, 370)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Observed primitive subtotals — incomplete and unrankable",
        (20, 28),
        ("sans-serif", 23).into_font().color(&BLACK),
    ))?;
    root.draw(&Text::new(
        "Existing bitop/v1 prices; missing operation counts prevent a total or parity ratio",
        (20, 52),
        ("sans-serif", 15).into_font().color(&RGBColor(80, 80, 80)),
    ))?;
    for (i, (row, cost)) in rows.iter().zip(&costs).enumerate() {
        let y = 72 + i as i32 * 42;
        let arm = row["arm"].as_str().ok_or("arm")?;
        let length = (*cost as f64 / largest as f64 * 590.0).round() as i32;
        let color = if i < 2 {
            RGBColor(33, 116, 154)
        } else {
            RGBColor(126, 111, 157)
        };
        root.draw(&Text::new(
            arm,
            (20, y + 20),
            ("sans-serif", 18).into_font(),
        ))?;
        root.draw(&Rectangle::new(
            [(205, y), (205 + length.max(1), y + 25)],
            color.filled(),
        ))?;
        root.draw(&Text::new(
            cost.to_string(),
            (815, y + 20),
            ("monospace", 17).into_font(),
        ))?;
    }
    root.draw(&Text::new(
        "Bar lengths show recorded subtotals only. Each arm still has uncounted native work.",
        (20, 350),
        ("sans-serif", 15).into_font().color(&RGBColor(80, 80, 80)),
    ))?;
    root.present()?;
    drop(root);
    if fs::metadata(&args[2])?.len() > 1024 * 1024 {
        return Err("figure unexpectedly large".into());
    }
    Ok(())
}
