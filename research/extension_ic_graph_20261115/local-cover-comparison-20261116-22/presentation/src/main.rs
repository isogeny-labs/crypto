//! Presentation only: consume the unchanged frozen analyzer summary.
use plotters::prelude::*;
use serde_json::Value;
use std::{error::Error, fs::File};
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: presentation SUMMARY OUTPUT.svg".into());
    }
    let s: Value = serde_json::from_reader(File::open(&a[1])?)?;
    let r = &s["strata"][0];
    let names = ["graph_over_ic", "graph_over_rho", "ic_over_rho"];
    let labels = ["Graph / IC", "Graph / rho", "IC / rho"];
    let root = SVGBackend::new(&a[2], (1100, 420)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut c = ChartBuilder::on(&root)
        .caption(
            "Partial paired wall ratios: incomplete, no usable intervals",
            ("sans-serif", 25),
        )
        .margin(30)
        .x_label_area_size(65)
        .y_label_area_size(155)
        .build_cartesian_2d((0.04f64..1.6f64).log_scale(), -1i32..3i32)?;
    c.configure_mesh()
        .disable_mesh()
        .x_labels(6)
        .y_labels(5)
        .x_desc("Cold process wall ratio (log scale)")
        .x_label_formatter(&|v| format!("{v:.2}"))
        .y_label_formatter(&|v| {
            usize::try_from(*v)
                .ok()
                .and_then(|i| labels.get(i))
                .copied()
                .unwrap_or("")
                .into()
        })
        .label_style(("sans-serif", 20))
        .axis_desc_style(("sans-serif", 20))
        .draw()?;
    c.draw_series(LineSeries::new([(1.0, -1), (1.0, 3)], BLACK.mix(0.45)))?;
    for (i, name) in names.iter().enumerate() {
        if let Some(x) = r["ratios"][*name]["outer"]["geometric_mean"].as_f64() {
            if !x.is_finite() || x <= 0.0 {
                return Err("invalid recorded ratio".into());
            }
            c.draw_series(std::iter::once(Circle::new(
                (x, i as i32),
                8,
                RGBColor(130, 130, 130).filled(),
            )))?;
            c.draw_series(std::iter::once(Text::new(
                format!("{x:.5}"),
                (x * 1.08, i as i32),
                ("sans-serif", 19),
            )))?;
        }
    }
    root.present()?;
    Ok(())
}
