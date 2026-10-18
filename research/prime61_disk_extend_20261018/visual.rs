//! Recorded native counts and character signs only; no experiment reruns.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn status(path: impl AsRef<Path>) -> Value {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|s| serde_json::from_str::<Value>(s).ok())
        .find(|v| v["schema"] == "isogeny-disk-status/v1")
        .unwrap()
}
fn genus_svg(g: &Value) -> String {
    let mut s=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='880' height='630' viewBox='0 0 880 630'><rect width='880' height='630' fill='white'/><g font-family='sans-serif' fill='#162b40'><text x='30' y='30' font-size='22' font-weight='bold'>Selected-prime genus-character audit</text>");
    s+=&format!("<text x='30' y='55' font-size='14'>Exact finite calculation: rank {}; {} of 16 even-parity patterns; run 2026-10-18</text>",g["rank"],g["exhaustive_span_size"]);
    s += "<text x='35' y='100' font-size='14'>Degree</text>";
    for (i, p) in g["prime_discriminants"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        s += &format!(
            "<text x='{}' y='100' text-anchor='middle' font-size='14'>{p}</text>",
            170 + i * 135
        );
    }
    for (r, row) in g["rows"].as_array().unwrap().iter().enumerate() {
        let y = 115 + r * 38;
        s += &format!(
            "<text x='62' y='{}' text-anchor='middle' font-size='15'>{}</text>",
            y + 23,
            row["prime"]
        );
        for (i, c) in row["characters"].as_array().unwrap().iter().enumerate() {
            let (color, label) = if c.as_i64().unwrap() == 1 {
                ("#dce9f5", "+1")
            } else {
                ("#f1d7be", "-1")
            };
            s+=&format!("<rect x='{}' y='{y}' width='116' height='32' rx='3' fill='{color}'/><text x='{}' y='{}' text-anchor='middle' font-size='15'>{label}</text>",112+i*135,170+i*135,y+23);
        }
    }
    s+="<rect x='30' y='440' width='820' height='64' rx='6' fill='#e8eff5'/><text x='46' y='465' font-size='15'>Derived using standard genus theory and the prior exact order certificate:</text><text x='46' y='490' font-size='17' font-weight='bold'>lcm(16, 68,162,026) = 545,296,208 divides the generated subgroup size.</text><rect x='30' y='519' width='820' height='61' rx='6' fill='#f7f3e8' stroke='#8d7951' stroke-dasharray='6 4'/><text x='46' y='545' font-size='15'>If the whole class number is 545,296,208, full generation follows.</text><text x='46' y='567' font-size='14'>That class number remains conditional; class-group and CM bridges await Lean formalization.</text><text x='30' y='604' font-size='12'>Sources: results/genus-span.json, CLASS-COVERAGE.md, proof-map.json; finite Lean audit: 11 theorems.</text><text x='30' y='622' font-size='12'>This diagram describes a quotient calculation and a conditional implication; enumeration remains incomplete.</text></g></svg>";
    s
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert!(
        a.len() == 4 || a.len() == 5,
        "RUN_DIR REPORT_DIR GENUS_JSON [--genus-only]"
    );
    let runs = Path::new(&a[1]);
    let reports = Path::new(&a[2]);
    fs::create_dir_all(reports).unwrap();
    let g: Value = serde_json::from_slice(&fs::read(&a[3]).unwrap()).unwrap();
    fs::write(reports.join("genus.svg"), genus_svg(&g)).unwrap();
    if a.len() == 5 {
        assert_eq!(a[4], "--genus-only");
        return;
    }
    let phases: Vec<_> = ["restore-verify", "verify-512", "verify-1024"]
        .iter()
        .map(|n| status(runs.join(format!("{n}.stdout"))))
        .collect();
    let mut peak = 0;
    for entry in fs::read_dir(runs).unwrap() {
        let path = entry.unwrap().path();
        if !matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("stdout" | "completed")
        ) {
            continue;
        }
        for line in fs::read_to_string(path).unwrap().lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                if v["schema"] == "isogeny-disk-status/v1" {
                    peak = peak.max(v["process_peak_rss_kib"].as_u64().unwrap());
                }
            }
        }
    }
    let mut delta = serde_json::Map::new();
    for key in [
        "discovered",
        "expanded",
        "edges",
        "measured_curves",
        "runs",
        "target_trials",
        "relations",
        "verified_solves",
    ] {
        delta.insert(
            key.into(),
            json!(phases[2][key].as_u64().unwrap() - phases[0][key].as_u64().unwrap()),
        );
    }
    let summary = json!({"schema":"prime61.extension-summary/v1","phases":phases,"new_since_checkpoint":delta,
        "worker_peak_rss_kib_reported":peak,"genus_rank":g["rank"],"genus_span_size":g["exhaustive_span_size"],
        "full_class_certified":false,"full_cost":null,"speedup":null});
    fs::write(
        reports.join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    let maximum = phases
        .iter()
        .map(|v| v["discovered"].as_u64().unwrap())
        .max()
        .unwrap() as f64;
    let mut s=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='860' height='470' viewBox='0 0 860 470'><rect width='860' height='470' fill='white'/><g font-family='sans-serif' fill='#162b40'><text x='30' y='30' font-size='22' font-weight='bold'>Prime61 disk-backed extension</text><text x='30' y='56' font-size='14'>Cumulative populations; 12 fixed streams per measured curve; run 2026-10-18</text><rect x='460' y='78' width='12' height='12' fill='#235c91'/><text x='482' y='89' font-size='13'>Discovered</text><rect x='620' y='78' width='12' height='12' fill='#b45820'/><text x='642' y='89' font-size='13'>Measured</text>");
    for i in 0..=4 {
        let y = 355.0 - i as f64 * 57.5;
        s+=&format!("<path d='M75 {y} H820' stroke='#dae3eb'/><text x='65' y='{}' text-anchor='end' font-size='12'>{:.0}</text>",y+4.0,maximum*i as f64/4.0);
    }
    for (i, v) in phases.iter().enumerate() {
        let x = 200.0 + i as f64 * 245.0;
        for (offset, key, color) in [
            (-40.0, "discovered", "#235c91"),
            (7.0, "measured_curves", "#b45820"),
        ] {
            let n = v[key].as_u64().unwrap();
            let h = n as f64 / maximum * 230.0;
            s+=&format!("<rect x='{}' y='{}' width='38' height='{h}' fill='{color}'/><text x='{}' y='{}' text-anchor='middle' font-size='14'>{n}</text>",x+offset,355.0-h,x+offset+19.0,347.0-h);
        }
        s += &format!(
            "<text x='{x}' y='384' text-anchor='middle' font-size='15'>{}</text>",
            v["expanded"]
        );
    }
    s+="<text x='440' y='410' text-anchor='middle' font-size='14'>Expanded source curves (256 is inherited)</text><text x='30' y='440' font-size='12'>Source: results/runs/{restore-verify,verify-512,verify-1024}.stdout; exact counts, no statistical interval.</text><text x='30' y='460' font-size='12'>One transported workload per measured curve. Full horizontal-class coverage remains open.</text></g></svg>";
    fs::write(reports.join("progress.svg"), s).unwrap();
}
