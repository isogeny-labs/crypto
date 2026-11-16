//! Presentation of qualified component counts; no complete-cost ratio.
use plotters::prelude::*;
use serde_json::Value;
use std::{error::Error, fs::File, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: structural-gate-render SUMMARY.json NEW-OUTPUT.svg".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("presentation output already exists".into());
    }
    let s: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if s["schema"] != "cover.two-lp-structural-gate-analysis/v1"
        || s["comparative_eligibility"] != false
        || s["gate_proof"]["phase_conservation"]["all_phase_counts_conserved"] != true
    {
        return Err("qualified presentation scope".into());
    }
    let outcomes = s["outcomes"].as_array().ok_or("outcomes")?;
    if outcomes.len() != 2 {
        return Err("two arms required".into());
    }
    let mut values = Vec::new();
    for arm in outcomes {
        let phase = arm["accounting"]["phases"]
            .as_array()
            .ok_or("phases")?
            .iter()
            .find(|r| r["phase"] == "linear_algebra")
            .ok_or("LA")?;
        let v: u128 = phase["diagnostic_priced_bitops"]
            .as_str()
            .ok_or("subtotal")?
            .parse()?;
        values.push(v);
    }
    let maximum = *values.iter().max().ok_or("empty")?;
    if maximum == 0 {
        return Err("zero LA observations".into());
    }
    let root = SVGBackend::new(&args[2], (1080, 245)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Linear algebra: observed primitive subtotal, incomplete",
        (20, 28),
        ("sans-serif", 23).into_font(),
    ))?;
    for (i, (arm, value)) in outcomes.iter().zip(&values).enumerate() {
        let y = 60 + 60 * i as i32;
        let width = (*value as f64 / maximum as f64 * 550.0).round() as i32;
        let label = arm["arm"].as_str().ok_or("arm")?;
        root.draw(&Text::new(
            label,
            (20, y + 24),
            ("sans-serif", 18).into_font(),
        ))?;
        root.draw(&Rectangle::new(
            [(205, y), (205 + width.max(1), y + 32)],
            RGBColor(50 + 45 * i as u8, 110, 150).filled(),
        ))?;
        root.draw(&Text::new(
            value.to_string(),
            (785, y + 24),
            ("monospace", 17).into_font(),
        ))?;
    }
    let caption = format!(
        "Exact checkpoints: {} to {}. Known extra gate word events: {}.",
        s["gate_proof"]["baseline_exact_checks"],
        s["gate_proof"]["candidate_exact_checks"],
        s["gate_proof"]["charged_gate_word_events_lower_bound"]
    );
    root.draw(&Text::new(
        caption,
        (20, 202),
        ("sans-serif", 16).into_font(),
    ))?;
    root.draw(&Text::new("Remaining native DSU work is uncounted; totals, pricing bands and cost ratios stay unknown.",(20,229),("sans-serif",15).into_font()))?;
    root.present()?;
    Ok(())
}
