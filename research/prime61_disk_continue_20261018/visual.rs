//! Render native status receipts; never rerun arithmetic or manufacture timings.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn status(path: impl AsRef<Path>) -> Value {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find(|v| v["schema"] == "isogeny-disk-status/v1")
        .unwrap()
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    assert_eq!(a.len(), 3, "RUN_DIRECTORY REPORT_DIRECTORY");
    let run = Path::new(&a[1]);
    let report = Path::new(&a[2]);
    fs::create_dir_all(report).unwrap();
    let phases: Vec<Value> = ["restore-verify", "verify-64", "verify-128", "verify-256"]
        .iter()
        .map(|p| status(run.join(format!("{p}.stdout"))))
        .collect();
    let mut peak = 0;
    for file in fs::read_dir(run).unwrap() {
        let path = file.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("stdout") {
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
    let before = &phases[0];
    let after = phases.last().unwrap();
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
            json!(after[key].as_u64().unwrap() - before[key].as_u64().unwrap()),
        );
    }
    let summary = json!({"schema":"prime61.continuation-summary/v1", "phases":phases, "new_since_checkpoint":delta,
        "worker_peak_rss_kib_all_phases":peak, "full_cost":null, "speedup":null,
        "scope":"cumulative prefix; same public fixed target transported along first BFS parents; no full class certificate"});
    fs::write(
        report.join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    let maximum = phases
        .iter()
        .map(|v| v["discovered"].as_u64().unwrap())
        .max()
        .unwrap() as f64;
    let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='840' height='470' viewBox='0 0 840 470'><rect width='840' height='470' fill='white'/><g font-family='sans-serif' fill='#152539'><text x='35' y='32' font-size='21' font-weight='bold'>Prime61 restored-checkpoint continuation</text><text x='35' y='58' font-size='13'>Cumulative curves; 12 streams per measured curve; run 2026-10-18</text><rect x='470' y='78' width='12' height='12' fill='#235c91'/><text x='490' y='89' font-size='13'>Discovered</text><rect x='615' y='78' width='12' height='12' fill='#b45820'/><text x='635' y='89' font-size='13'>Measured</text>");
    for step in 0..=4 {
        let value = maximum * step as f64 / 4.0;
        let y = 355.0 - step as f64 * 57.5;
        svg += &format!("<path d='M75 {y} H800' stroke='#d9e2eb'/><text x='63' y='{}' text-anchor='end' font-size='12'>{value:.0}</text>", y+4.0);
    }
    for (i, v) in phases.iter().enumerate() {
        let x = 145.0 + i as f64 * 180.0;
        for (offset, key, color) in [
            (-35.0, "discovered", "#235c91"),
            (5.0, "measured_curves", "#b45820"),
        ] {
            let n = v[key].as_u64().unwrap();
            let h = n as f64 / maximum * 230.0;
            svg += &format!("<rect x='{}' y='{}' width='34' height='{h}' fill='{color}'/><text x='{}' y='{}' text-anchor='middle' font-size='13'>{n}</text>",x+offset,355.0-h,x+offset+17.0,347.0-h);
        }
        svg += &format!(
            "<text x='{x}' y='383' text-anchor='middle' font-size='14'>{}</text>",
            v["expanded"]
        );
    }
    svg += "<text x='430' y='410' text-anchor='middle' font-size='14'>Expanded source curves (32 is inherited)</text><text x='35' y='438' font-size='12'>Source: results/runs/{restore-verify,verify-64,verify-128,verify-256}.stdout</text><text x='35' y='457' font-size='12'>Exact population counts; no statistical interval. Selected-prime frontier remains open.</text></g></svg>";
    fs::write(report.join("progress.svg"), svg).unwrap();
}
