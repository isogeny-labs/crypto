//! Fail-closed aggregation and 36-cell pricing for the replication cohort.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const ROOT: &str = "research/factor_base_replication_20261024/results";
const RAW: &str = "research/factor_base_replication_20261024/results/raw";
const PROTOCOL_SHA256: &str = "ace04952556bfe1cb9b1a0c8cb5dec6715040f55dd376d1e3ca3d7b659738997";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [(u64, u64); 5] = [(1, 16), (2, 16), (4, 16), (1, 8), (1, 32)];

fn rows(path: &Path) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn number(v: &Value, key: &str) -> f64 {
    v[key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing numeric field {key}")) as f64
}
fn logical(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(key, _)| key.ends_with("_uncharged"))
        .map(|(_, value)| value.as_u64().unwrap() as f64)
        .sum()
}
fn price(row: &Value, square: f64, inversion: f64, native: f64) -> f64 {
    let counts = &row["counts"];
    let unit = &row["source_add"];
    let arithmetic =
        number(counts, "m") + square * number(counts, "s") + inversion * number(counts, "i");
    let divisor = number(unit, "m") + square * number(unit, "s") + inversion * number(unit, "i");
    assert!(divisor > 0.0);
    arithmetic / divisor
        + native
            * (logical(&row["native"]) + number(counts, "field_adds") + number(counts, "bit_ops"))
}
fn arm_name(d: u64, j: u64) -> String {
    format!("fb{d}-j{j}")
}
fn checked_row(row: &Value, bits: u64) {
    assert_eq!(row["schema"], "factor-base-graph-replication-record/v1");
    assert_eq!(row["protocol_sha256"], PROTOCOL_SHA256);
    assert_eq!(row["bits"], bits);
    assert_eq!(row["verified"], true);
    assert_eq!(row["exhausted"], false);
    assert_eq!(row["recovered"], row["planted"]);
    assert_eq!(row["rho"]["verified"], true);
    assert_eq!(row["rho"]["exhausted"], false);
    assert_eq!(row["rho"]["recovered"], row["planted"]);
    assert_eq!(row["rho"]["source_add"], row["source_add"]);
    let digest = row["workload_sha256"].as_str().unwrap();
    assert_eq!(digest.len(), 64);
    assert_eq!(row["workload_id"], format!("W{}", &digest[..12]));
}

fn main() {
    fs::create_dir_all(ROOT).unwrap();
    let mut all: BTreeMap<String, Vec<(u64, Value)>> = BTreeMap::new();
    let mut rho_by_workload: BTreeMap<String, Value> = BTreeMap::new();
    let mut canonical_ids = BTreeSet::new();
    let mut raw_files = Vec::new();
    for (denominator, jumps) in ARMS {
        let arm = arm_name(denominator, jumps);
        let mut arm_ids = BTreeSet::new();
        for bits in BITS {
            let path = format!("{RAW}/{arm}-bits{bits}.jsonl");
            let file_rows = rows(Path::new(&path));
            assert_eq!(file_rows.len(), 180, "{path}");
            raw_files.push(path.clone());
            for row in file_rows {
                checked_row(&row, bits);
                let id = row["workload_id"].as_str().unwrap().to_owned();
                assert!(arm_ids.insert(id.clone()), "duplicate {id} in {arm}");
                if let Some(rho) = rho_by_workload.get(&id) {
                    assert_eq!(rho, &row["rho"], "rho drift for {id}");
                } else {
                    rho_by_workload.insert(id, row["rho"].clone());
                }
                all.entry(arm.clone()).or_default().push((bits, row));
            }
        }
        assert_eq!(arm_ids.len(), 1260);
        if canonical_ids.is_empty() {
            canonical_ids = arm_ids;
        } else {
            assert_eq!(canonical_ids, arm_ids, "target-set drift in {arm}");
        }
    }
    assert_eq!(canonical_ids.len(), 1260);
    assert_eq!(rho_by_workload.len(), 1260);

    let mut targets = File::create(format!("{ROOT}/targets.jsonl")).unwrap();
    let mut grid = File::create(format!("{ROOT}/price-grid.csv")).unwrap();
    let mut graph = File::create(format!("{ROOT}/graph-data.csv")).unwrap();
    writeln!(grid, "arm,slice,targets,verified,failures,S_per_M,I_per_M,native_GAE_per_logical,candidate_GAE,rho_GAE,candidate_over_rho,processed_divisors,rejected_large_large").unwrap();
    writeln!(
        graph,
        "arm,slice,targets,candidate_over_rho,processed_divisors,candidate_GAE,rho_GAE"
    )
    .unwrap();
    let mut summaries = Vec::new();
    for (arm, records) in &all {
        assert_eq!(records.len(), 1260);
        for (bits, row) in records {
            let candidate = price(row, 0.5, 10.0, 0.1);
            let rho = price(&row["rho"], 0.5, 10.0, 0.1);
            writeln!(
                targets,
                "{}",
                json!({
                    "schema":"factor-base-graph-replication-target/v1", "arm":arm,
                    "workload_id":row["workload_id"], "workload_sha256":row["workload_sha256"],
                    "bits":bits, "curve_p":row["p"], "target_index":row["target_index"],
                    "algorithm_seed":row["algorithm_seed"], "planted":row["planted"],
                    "verified":true, "rho_verified":true,
                    "candidate_GAE":candidate, "rho_GAE":rho,
                    "candidate_over_rho":candidate/rho,
                    "processed_divisors":row["native"]["processed_divisors"],
                    "rejected_large_large":row["native"]["rejected_large_large_relations"],
                    "raw_record":format!("results/raw/{arm}-bits{bits}.jsonl")
                })
            )
            .unwrap();
        }
        for slice in std::iter::once(None).chain(BITS.map(Some)) {
            let selected: Vec<_> = records
                .iter()
                .filter(|(bits, _)| slice.is_none_or(|wanted| wanted == *bits))
                .collect();
            let label = slice.map_or_else(|| "all".to_owned(), |value| value.to_string());
            let processed: u64 = selected
                .iter()
                .map(|(_, row)| row["native"]["processed_divisors"].as_u64().unwrap())
                .sum();
            let rejected: u64 = selected
                .iter()
                .map(|(_, row)| {
                    row["native"]["rejected_large_large_relations"]
                        .as_u64()
                        .unwrap()
                })
                .sum();
            let mut ratios = Vec::new();
            let mut representative = None;
            let mut wins = 0u64;
            let mut losses = 0u64;
            let mut ties = 0u64;
            for square in [0.0, 0.5, 1.0] {
                for inversion in [1.0, 10.0, 100.0, 1000.0] {
                    for native in [0.0, 0.1, 1.0] {
                        let candidate: f64 = selected
                            .iter()
                            .map(|(_, row)| price(row, square, inversion, native))
                            .sum();
                        let rho: f64 = selected
                            .iter()
                            .map(|(_, row)| price(&row["rho"], square, inversion, native))
                            .sum();
                        let ratio = candidate / rho;
                        if ratio < 1.0 {
                            wins += 1;
                        } else if ratio > 1.0 {
                            losses += 1;
                        } else {
                            ties += 1;
                        }
                        ratios.push(ratio);
                        writeln!(grid, "{arm},{label},{},{},0,{square},{inversion},{native},{candidate},{rho},{ratio},{processed},{rejected}", selected.len(), selected.len()).unwrap();
                        if square == 0.5 && inversion == 10.0 && native == 0.1 {
                            representative = Some((candidate, rho, ratio));
                        }
                    }
                }
            }
            assert_eq!(wins + losses + ties, 36);
            ratios.sort_by(f64::total_cmp);
            let (candidate, rho, ratio) = representative.unwrap();
            writeln!(
                graph,
                "{arm},{label},{},{ratio},{processed},{candidate},{rho}",
                selected.len()
            )
            .unwrap();
            summaries.push(json!({
                "arm":arm, "slice":label, "targets":selected.len(), "verified":selected.len(),
                "failures":0, "processed_divisors":processed, "rejected_large_large":rejected,
                "candidate_over_rho_band":[ratios[0], ratios[ratios.len()-1]],
                "candidate_win_cells_vs_rho":wins, "candidate_loss_cells_vs_rho":losses,
                "candidate_tie_cells_vs_rho":ties,
                "representative":{"candidate_GAE":candidate,"rho_GAE":rho,"candidate_over_rho":ratio}
            }));
        }
    }
    let original: Value = serde_json::from_str(
        &fs::read_to_string("research/four_followups_20261021/results/summary.json").unwrap(),
    )
    .unwrap();
    let original_all: BTreeMap<String, Value> = original["summary"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["slice"] == "all")
        .map(|row| (row["arm"].as_str().unwrap().to_owned(), row.clone()))
        .collect();
    let replication_all: BTreeMap<String, Value> = summaries
        .iter()
        .filter(|row| row["slice"] == "all")
        .map(|row| (row["arm"].as_str().unwrap().to_owned(), row.clone()))
        .collect();
    let mut comparison = File::create(format!("{ROOT}/original-vs-replication.csv")).unwrap();
    writeln!(comparison, "arm,original_targets,replication_targets,original_candidate_over_rho,replication_candidate_over_rho,ratio_delta,original_processed_divisors,replication_processed_divisors,processed_divisors_delta").unwrap();
    for arm in replication_all.keys() {
        let before = &original_all[arm];
        let after = &replication_all[arm];
        let before_ratio = before["representative"]["candidate_over_rho"]
            .as_f64()
            .unwrap();
        let after_ratio = after["representative"]["candidate_over_rho"]
            .as_f64()
            .unwrap();
        let before_processed = before["processed_divisors"].as_u64().unwrap();
        let after_processed = after["processed_divisors"].as_u64().unwrap();
        writeln!(
            comparison,
            "{arm},{},{},{before_ratio},{after_ratio},{},{before_processed},{after_processed},{}",
            before["targets"].as_u64().unwrap(),
            after["targets"].as_u64().unwrap(),
            after_ratio - before_ratio,
            after_processed as i128 - before_processed as i128
        )
        .unwrap();
    }
    fs::write(
        format!("{ROOT}/summary.json"),
        serde_json::to_string_pretty(&json!({
            "schema":"factor-base-graph-replication-analysis/v1",
            "protocol_sha256":PROTOCOL_SHA256,
            "targets":1260, "arm_target_records":6300, "verified_candidate_records":6300,
            "verified_rho_records":6300, "pricing_cells_per_slice":36,
            "raw_files":raw_files, "summary":summaries
        }))
        .unwrap()
            + "\n",
    )
    .unwrap();
}
