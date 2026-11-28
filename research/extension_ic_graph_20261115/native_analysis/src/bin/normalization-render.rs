use plotters::prelude::*;
use serde_json::Value;
use std::{error::Error, fs::File, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: normalization-render SUMMARY.json NEW-OUTPUT.svg".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("presentation already exists".into());
    }
    let s: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if s["schema"] != "cover.rref-normalization-analysis/v1"
        || s["new_full_dlp_trial"] != false
        || s["every_insert_state_independently_replayed"] != true
    {
        return Err("qualified component scope".into());
    }
    let arms = s["outcomes"].as_array().ok_or("arms")?;
    if arms.len() != 2 {
        return Err("paired arms".into());
    }
    let maximum = arms[0]["counts"]["normalization_products"]
        .as_u64()
        .ok_or("products")?;
    if maximum == 0 {
        return Err("zero baseline".into());
    }
    let root = SVGBackend::new(&args[2], (1120, 245)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Pivot-normalization products on the saved matrix",
        (20, 30),
        ("sans-serif", 24).into_font(),
    ))?;
    for (i, arm) in arms.iter().enumerate() {
        let n = arm["counts"]["normalization_products"]
            .as_u64()
            .ok_or("products")?;
        let y = 62 + 60 * i as i32;
        let width = (n as f64 / maximum as f64 * 530.0).round() as i32;
        root.draw(&Text::new(
            arm["mode"].as_str().ok_or("mode")?,
            (20, y + 23),
            ("sans-serif", 17).into_font(),
        ))?;
        root.draw(&Rectangle::new(
            [(250, y), (250 + width.max(1), y + 32)],
            RGBColor(60 + 40 * i as u8, 115, 150).filled(),
        ))?;
        root.draw(&Text::new(
            n.to_string(),
            (810, y + 23),
            ("monospace", 18).into_font(),
        ))?;
    }
    root.draw(&Text::new(
        format!(
            "Avoided zero products: {}. Added zero tests: {}.",
            s["zero_products_avoided"], s["new_zero_tests"]
        ),
        (20, 202),
        ("sans-serif", 17).into_font(),
    ))?;
    root.draw(&Text::new("Normalization component only; native coverage incomplete, no pipeline repricing or cost ratio.",(20,231),("sans-serif",15).into_font()))?;
    root.present()?;
    Ok(())
}
