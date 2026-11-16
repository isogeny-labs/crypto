//! Render the exact census rows as a labeled SVG diagnostic plot.
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data: Value =
        serde_json::from_slice(&fs::read(root.join("results.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if data["schema"] != "n83.native-rank-query-census/v1"
        || data["rows"].as_array().map(Vec::len) != Some(390)
    {
        return Err("unexpected result schema/count".into());
    }
    let shapes = [
        (10624u64, 64u64, "K64 / B10624", "#aa336a"),
        (42496, 256, "K256 / B42496", "#1967a3"),
        (99600, 600, "K600 / B99600", "#13805e"),
        (5418738, 32643, "full D18 / B5418738", "#b27016"),
    ];
    let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='920' height='620' viewBox='0 0 920 620'>\n<rect width='920' height='620' fill='white'/><text x='70' y='35' font-size='22' font-family='sans-serif'>N83 primary: necessary uniform one-row query counts</text>\n<text x='70' y='58' font-size='13' font-family='sans-serif'>Exact source: N83-RQC-20261115-v1; y-axis shows log10(q_min), not measured runtime</text>\n");
    for tick in (0..=12).step_by(2) {
        let y = 500.0 - tick as f64 * 35.0;
        svg.push_str(&format!("<line x1='90' y1='{y}' x2='690' y2='{y}' stroke='#dddddd'/><text x='55' y='{}' font-size='13' font-family='sans-serif'>{tick}</text>\n",y+4.0));
    }
    svg.push_str("<line x1='90' y1='80' x2='90' y2='500' stroke='#222'/><line x1='90' y1='500' x2='690' y2='500' stroke='#222'/><text x='12' y='300' font-size='14' font-family='sans-serif' transform='rotate(-90 12 300)'>log10(q_min queries)</text>\n");
    for m in 4..=6 {
        let x = 140 + (m - 4) * 250;
        svg.push_str(&format!(
            "<text x='{x}' y='528' font-size='15' font-family='sans-serif'>m={m}</text>\n"
        ));
    }
    for (idx, (points, columns, label, color)) in shapes.iter().enumerate() {
        let mut coords = Vec::new();
        for m in 4..=6 {
            let matches: Vec<_> = data["rows"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| {
                    r["curve_arm"] == "a0"
                        && r["point_count_manifest_claim"] == *points
                        && r["orbit_columns_manifest_claim"] == *columns
                        && r["summands"] == m
                })
                .collect();
            if matches.is_empty() {
                return Err(format!("missing shape {label} m={m}"));
            }
            let first = matches[0]["minimum_queries_before_half_rank_probability_is_possible"]
                .as_str()
                .ok_or("missing q")?;
            if matches
                .iter()
                .any(|r| r["minimum_queries_before_half_rank_probability_is_possible"] != first)
            {
                return Err("same-size manifests disagree".into());
            }
            let q: f64 = first.parse().map_err(|e| format!("invalid q: {e}"))?;
            let x = 160.0 + (m - 4) as f64 * 250.0;
            let y = 500.0 - q.log10() * 35.0;
            coords.push((x, y));
            svg.push_str(&format!(
                "<circle cx='{x:.2}' cy='{y:.2}' r='5' fill='{color}'/>\n"
            ));
        }
        svg.push_str(&format!(
            "<polyline points='{}' fill='none' stroke='{color}' stroke-width='3'/>\n",
            coords
                .iter()
                .map(|(x, y)| format!("{x:.2},{y:.2}"))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        let ly = 110 + idx * 31;
        svg.push_str(&format!("<line x1='710' y1='{ly}' x2='735' y2='{ly}' stroke='{color}' stroke-width='3'/><text x='744' y='{}' font-size='13' font-family='sans-serif'>{label}</text>\n",ly+5));
    }
    svg.push_str("<text x='95' y='572' font-size='12' font-family='sans-serif'>One row/query, uniform subgroup target; manifest-claimed B and K. Large-prime and solver costs excluded.</text></svg>\n");
    let output = root.join("rank-query-plot.svg");
    if output.exists() {
        if fs::read(&output).map_err(|e| e.to_string())? != svg.as_bytes() {
            return Err("write-once plot differs".into());
        }
    } else {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&output)
            .map_err(|e| e.to_string())?;
        f.write_all(svg.as_bytes()).map_err(|e| e.to_string())?;
    }
    println!("{}", output.display());
    Ok(())
}
