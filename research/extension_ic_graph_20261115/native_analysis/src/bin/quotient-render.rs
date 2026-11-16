//! Plot qualified saved-matrix component subtotals, without pipeline composition.
use plotters::prelude::*;
use serde_json::Value;
use std::{error::Error, fs::File, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || Path::new(&args[2]).exists() {
        return Err("usage: quotient-render SUMMARY.json NEW-FIGURE.svg".into());
    }
    let s: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if s["schema"] != "cover.homogeneous-quotient-analysis/v1"
        || s["comparative_eligibility"] != false
    {
        return Err("qualified presentation scope".into());
    }
    let outcomes = s["outcomes"].as_array().ok_or("outcomes")?;
    let mut maximum = 1u128;
    for row in outcomes {
        for v in row["prices"].as_array().ok_or("prices")? {
            maximum = maximum.max(
                v["observed_incomplete_bitops"]
                    .as_str()
                    .ok_or("subtotal")?
                    .parse()?,
            );
        }
    }
    let root = SVGBackend::new(&args[2], (1120, 560)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Checkpoint + affine solve: observed subtotals (incomplete)",
        (20, 28),
        ("sans-serif", 23).into_font(),
    ))?;
    for (i, row) in outcomes.iter().enumerate() {
        let y = 55 + 68 * i as i32;
        root.draw(&Text::new(
            row["arm"].as_str().ok_or("arm")?,
            (20, y + 26),
            ("sans-serif", 17).into_font(),
        ))?;
        for (j, price) in row["prices"].as_array().ok_or("prices")?.iter().enumerate() {
            let value: u128 = price["observed_incomplete_bitops"]
                .as_str()
                .ok_or("subtotal")?
                .parse()?;
            let width = (value as f64 / maximum as f64 * 640.0).round() as i32;
            let top = y + 26 * j as i32;
            let color = if j == 0 {
                RGBColor(70, 130, 180)
            } else {
                RGBColor(80, 150, 105)
            };
            root.draw(&Rectangle::new(
                [(155, top), (155 + width, top + 21)],
                color.filled(),
            ))?;
            root.draw(&Text::new(
                value.to_string(),
                (825, top + 17),
                ("monospace", 16).into_font(),
            ))?;
        }
    }
    root.draw(&Text::new(
        "Blue: original full affine RREF. Green: kernel substitution + two-column solve.",
        (20, 490),
        ("sans-serif", 16).into_font(),
    ))?;
    root.draw(&Text::new(
        "Both pay the homogeneous checkpoint. Six recorded matrices / no new DLP sample.",
        (20, 518),
        ("sans-serif", 16).into_font(),
    ))?;
    root.draw(&Text::new("New state events are charged; other native gaps remain. No complete-cost or pipeline ratio.",(20,546),("sans-serif",15).into_font()))?;
    root.present()?;
    Ok(())
}
