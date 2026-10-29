//! Native receipt aggregation and recorded-value cross-checks; no new measurements.
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn main() {
    let root_arg = std::env::args().nth(1).expect("study directory");
    let root = Path::new(&root_arg);
    let mut reports: Vec<_> = fs::read_dir(root.join("reports"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.join("summary.json").exists())
        .collect();
    reports.sort();
    let (mut records, mut verified, mut errors, mut replays) = (0u64, 0u64, 0u64, 0usize);
    let mut panels = Vec::new();
    let mut hashes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for report in reports {
        let audit = read(report.join("audit.json"));
        let summary = read(report.join("summary.json"));
        let session_name = Path::new(summary["session"].as_str().unwrap())
            .file_name()
            .unwrap();
        let session = read(
            root.join("sessions")
                .join(session_name)
                .join("session.json"),
        );
        assert_eq!(audit["ok"], true);
        assert!(audit["problems"].as_array().unwrap().is_empty());
        assert!(audit["checks"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v == true));
        let replay_rows = audit["replays"].as_array().unwrap();
        assert!(replay_rows.iter().all(|r| r["reproduced"] == true));
        let panel_records = audit["records"].as_u64().unwrap();
        let panel_verified = audit["verified_records"].as_u64().unwrap();
        let panel_errors = session["status_counts"]["error"].as_u64().unwrap_or(0);
        assert_eq!(panel_records, panel_verified + panel_errors);
        records += panel_records;
        verified += panel_verified;
        errors += panel_errors;
        replays += replay_rows.len();
        let mut pairs: BTreeMap<(String, String), BTreeMap<String, Vec<f64>>> = BTreeMap::new();
        for row in summary["workloads"].as_array().unwrap() {
            let arm = row["arm"].as_str().unwrap();
            let curve = row["curve"]["slug"].as_str().unwrap();
            let key = (
                curve.to_owned(),
                row["workload_id"].as_str().unwrap().to_owned(),
            );
            if let Some(values) = row["priced_gae_totals"].as_array() {
                if let Some(costs) = values.iter().map(Value::as_f64).collect::<Option<Vec<_>>>() {
                    pairs.entry(key).or_default().insert(arm.to_owned(), costs);
                }
            }
            if matches!(arm, "known-log" | "transported") {
                let label = format!("{}/{}", report.file_name().unwrap().to_string_lossy(), arm);
                hashes.entry(label).or_default().extend(
                    row["factor_base_point_hashes"]
                        .as_object()
                        .unwrap()
                        .keys()
                        .cloned(),
                );
            }
        }
        let mut exceptions: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for ((curve, workload), arms) in &pairs {
            if let (Some(candidate), Some(reference)) =
                (arms.get("known-log"), arms.get("abscissa"))
            {
                if !candidate.iter().zip(reference).all(|(c, r)| c < r) {
                    exceptions
                        .entry(curve.clone())
                        .or_default()
                        .push(workload.clone());
                }
            }
            if let (Some(a), Some(b)) = (arms.get("known-log"), arms.get("known-log-aa")) {
                assert_eq!(a, b, "A/A counted costs differ");
            }
        }
        panels.push(
            json!({"report":report.file_name().unwrap().to_string_lossy(),
            "records":panel_records,"verified":panel_verified,"errors":panel_errors,
            "measured_replays":replay_rows.len(),"all_replays_identical":true,
            "candidate_not_cheaper_at_every_weight_by_workload":exceptions}),
        );
    }
    let anchor = &hashes["run-20261021T0636Z-generator-1/known-log"];
    assert_eq!(anchor.len(), 1);
    assert_eq!(
        anchor,
        &hashes["run-20261021T0636Z-generator-2/transported"]
    );
    assert_eq!(
        anchor,
        &hashes["run-20261021T0636Z-generator-7/transported"]
    );
    println!("{}", serde_json::to_string_pretty(&json!({
        "schema":"known-log-study.audit-totals/v1", "records":records,
        "verified":verified,"errors":errors,"measured_replays":replays,
        "all_replays_identical":true,"known_log_aa_counted_costs_identical":true,
        "transported_point_sets_identical":true,"factor_base_point_hashes":hashes,
        "panels":panels,"independent_validation":false,
        "scope":"Same-host receipt cross-check, including warmups, controls and corrected rerun; not distinct target counts."
    })).unwrap());
}
