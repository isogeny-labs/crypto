//! Recorded phase subtotals only; no inferred complete price or ratios.
use plotters::prelude::*;
use serde_json::Value;
use std::{error::Error, fs::File, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || Path::new(&args[2]).exists() {
        return Err("usage: production-quotient-render SUMMARY.json NEW-FIGURE.svg".into());
    }
    let s: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if s["schema"] != "cover.production-quotient-analysis/v1"
        || s["comparative_eligibility"] != false
    {
        return Err("qualified scope".into());
    }
    let outcomes = s["outcomes"].as_array().ok_or("outcomes")?;
    let mut maximum = 1u128;
    for row in outcomes {
        if let Some(v) = row["accounting"]["observed_primitive_subtotal_bitops"].as_str() {
            maximum = maximum.max(v.parse()?);
        }
    }
    let root = SVGBackend::new(&args[2], (1120, 340)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Recorded primitive subtotals by phase (incomplete)",
        (20, 28),
        ("sans-serif", 23).into_font(),
    ))?;
    let colors = [
        RGBColor(70, 130, 180),
        RGBColor(220, 150, 60),
        RGBColor(110, 160, 110),
        RGBColor(150, 130, 170),
    ];
    for (i, row) in outcomes.iter().enumerate() {
        let y = 54 + 49 * i as i32;
        let name = row["arm"].as_str().ok_or("arm")?;
        root.draw(&Text::new(
            name,
            (20, y + 22),
            ("sans-serif", 18).into_font(),
        ))?;
        let mut x = 155;
        if let Some(phases) = row["accounting"]["phases"].as_array() {
            for phase in phases {
                let amount: u128 = phase["diagnostic_priced_bitops"]
                    .as_str()
                    .ok_or("price")?
                    .parse()?;
                let width = (amount as f64 / maximum as f64 * 680.0).round() as i32;
                let c = match phase["phase"].as_str() {
                    Some("relations") => 0,
                    Some("descent") => 1,
                    Some("linear_algebra") => 2,
                    _ => 3,
                };
                root.draw(&Rectangle::new(
                    [(x, y), (x + width, y + 30)],
                    colors[c].filled(),
                ))?;
                x += width;
            }
            root.draw(&Text::new(
                row["accounting"]["observed_primitive_subtotal_bitops"]
                    .as_str()
                    .ok_or("subtotal")?
                    .to_owned(),
                (855, y + 21),
                ("monospace", 16).into_font(),
            ))?;
        } else {
            root.draw(&Text::new(
                "execution failed; exact records retained",
                (155, y + 22),
                ("sans-serif", 17).into_font(),
            ))?;
        }
    }
    for (i, label) in ["relations", "descent", "linear algebra", "other phases"]
        .iter()
        .enumerate()
    {
        let x = 20 + 255 * i as i32;
        root.draw(&Rectangle::new(
            [(x, 235), (x + 20, 251)],
            colors[i].filled(),
        ))?;
        root.draw(&Text::new(
            *label,
            (x + 28, 250),
            ("sans-serif", 16).into_font(),
        ))?;
    }
    root.draw(&Text::new(
        "Trial39 / p1009 seed367 / one cold round, zero warmups or retries.",
        (20, 286),
        ("sans-serif", 16).into_font(),
    ))?;
    root.draw(&Text::new("Native gaps remain; complete costs, bands and ratios are unknown. Failures/caps are retained.",(20,314),("sans-serif",15).into_font()))?;
    root.present()?;
    Ok(())
}
