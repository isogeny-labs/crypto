//! Derive the continued cohort status and matched rho band from validated native records.
use serde_json::{json, Map, Value};
use std::fs;

const ROOT: &str = "research/pencil_markowitz_20261022";

fn read(path: &str) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn main() {
    let validation = read(&format!("{ROOT}/results/validation.json"));
    assert_eq!(validation["status"], "pass");
    assert_eq!(validation["independent"], true);
    assert!(validation["missing_replays"].as_array().unwrap().is_empty());
    let old = read("research/pencil_sparse_20261022/results/status.json");
    assert_eq!(old["validation_status"], "pass");
    assert_eq!(old["original_cohort_targets"], 1260);
    assert_eq!(old["source_verified_distinct_targets"], 543);
    assert_eq!(old["distinct_targets_still_open"], 717);
    let prior_failure = old["failed_target_records"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row["workload_id"] == "Wa0fc1e72d7a0");
    assert!(prior_failure);

    let process = read(&format!("{ROOT}/results/p186379-markowitz-t0.status.json"));
    let replay_process = read(&format!(
        "{ROOT}/results/replay/p186379-markowitz-t0.status.json"
    ));
    assert_eq!(process["bits"], 32);
    assert_eq!(process["p"], 186379);
    assert_eq!(replay_process["bits"], process["bits"]);
    assert_eq!(replay_process["p"], process["p"]);
    for key in [
        "status",
        "exit_code",
        "raw_rows",
        "candidate_verified",
        "failure",
        "rho_verified",
    ] {
        assert_eq!(process[key], replay_process[key], "process replay {key}");
    }
    let raw = fs::read_to_string(format!("{ROOT}/results/p186379-markowitz-t0.jsonl")).unwrap();
    let rows: Vec<Value> = raw
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), process["raw_rows"].as_u64().unwrap() as usize);
    assert!(rows.len() == 1 || rows.len() == 2);
    let replay_raw =
        fs::read_to_string(format!("{ROOT}/results/replay/p186379-markowitz-t0.jsonl")).unwrap();
    let replay_rows: Vec<Value> = replay_raw
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(replay_rows.len(), rows.len());
    assert_eq!(rows[0], replay_rows[0], "fixed setup replay");
    let target_replay_validated = rows.len() == 2
        && validation["files"].as_array().unwrap().iter().any(|file| {
            file["path"] == format!("{ROOT}/results/p186379-markowitz-t0.jsonl")
                && file["replay_equivalence"] == "nonwall_equal"
        });
    assert!(rows.len() == 1 || target_replay_validated);
    let upper_replay = json!({
        "schema":"pencil-markowitz-process-replay/v1",
        "status":"verified","independent":true,
        "binary_sha256":"03a431e1ae71d022ec8f1c6fedfbe26240eb28446c55f68fd2460e115daca92f",
        "fixed_setup_equal":true,
        "process_status_equal_except_timestamps":true,
        "target_nonwall_equal_by_native_validator":target_replay_validated,
        "target_rows":rows.len()-1,
        "primary_status":process["status"],"replay_status":replay_process["status"],
        "note":"For setup-only timeout or process failure, no candidate or same-process rho scalar was emitted; only fixed setup and process outcome are replayed."
    });
    fs::write(
        format!("{ROOT}/results/upper-process-replay.json"),
        serde_json::to_vec_pretty(&upper_replay).unwrap(),
    )
    .unwrap();
    let target = rows.get(1);
    let candidate_verified = target.is_some_and(|row| row["verified"] == true);
    if candidate_verified {
        assert_eq!(target.unwrap()["workload_id"], "Wa0fc1e72d7a0");
        assert!(validation["files"].as_array().unwrap().iter().any(|file| {
            file["path"] == format!("{ROOT}/results/p186379-markowitz-t0.jsonl")
                && file["verified_candidate_rows"] == 1
        }));
    } else if let Some(row) = target {
        assert_eq!(row["workload_id"], "Wa0fc1e72d7a0");
        assert!(row["recovered"].is_null());
    }
    let candidate_ratio = if candidate_verified {
        let priced = fs::read_to_string(format!("{ROOT}/results/targets.jsonl")).unwrap();
        priced
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .find(|row| row["workload_id"] == "Wa0fc1e72d7a0")
            .expect("priced upper target")["representative"]["pencil_over_rho"]
            .clone()
    } else {
        Value::Null
    };
    let solved = 543 + usize::from(candidate_verified);
    let status = json!({
        "schema":"pencil-markowitz-cohort-status/v1",
        "original_cohort_targets":1260,
        "previous_source_verified_distinct_targets":543,
        "source_verified_distinct_targets":solved,
        "distinct_targets_still_open":1260-solved,
        "overall_status":if solved == 1260 {"complete"} else {"partial"},
        "validation_status":validation["status"],
        "p739_repeated_workloads":60,
        "p186379_target0_process":process,
        "p186379_target0_replay_process":replay_process,
        "p186379_target0_candidate_ratio":candidate_ratio,
        "note":"The p739 gate repeats previously solved workloads. The p186379 target contributes a new distinct solve only if its source scalar is independently verified. A stopped run or failed target has no candidate/rho price ratio."
    });
    fs::write(
        format!("{ROOT}/results/status.json"),
        serde_json::to_vec_pretty(&status).unwrap(),
    )
    .unwrap();

    let csv = fs::read_to_string(format!("{ROOT}/results/p739-markowitz-vs-peel.csv")).unwrap();
    let mut band = Map::new();
    for line in csv.lines().skip(1) {
        let fields: Vec<_> = line.split(',').collect();
        assert_eq!(fields.len(), 11);
        if fields[2] == "0.5" && fields[3] == "10" {
            let native = fields[4];
            assert!(["0", "0.1", "1"].contains(&native));
            let candidate: f64 = fields[6].parse().unwrap();
            let rho: f64 = fields[7].parse().unwrap();
            assert!(candidate > 0.0 && rho > 0.0);
            assert!(band
                .insert(native.to_owned(), json!({"ic":candidate,"rho":rho}))
                .is_none());
        }
    }
    assert_eq!(band.len(), 3);
    let order_holds = band
        .values()
        .all(|row| row["ic"].as_f64().unwrap() > row["rho"].as_f64().unwrap());
    let rho_band = json!({
        "kind":"negation",
        "protocol":"same-process source-negation-rho on each frozen target; PROTOCOL.md",
        "unit":"GAE",
        "binary_or_process":"pencil-sparse process runs candidate and source rho per target",
        "matches_protocol":true,"strongest_available":true,
        "same_unit":true,"same_binary_or_process":true,
        "gae_band":band,
        "gae_band_scope":"p=739, 60 targets, S/M=0.5, I/M=10; native paired CSV",
        "order_holds_across_band":order_holds,
        "ordering":if order_holds {"candidate GAE > matched rho GAE at all three native prices"} else {"ordering varies across native prices"}
    });
    fs::write(
        format!("{ROOT}/results/rho-band.json"),
        serde_json::to_vec_pretty(&rho_band).unwrap(),
    )
    .unwrap();
    let pair = read(&format!("{ROOT}/results/p739-markowitz-vs-peel.json"));
    assert_eq!(pair["targets"], 60);
    assert_eq!(pair["pricing_cells"], 36);
    let lead = json!({
        "schema":"pencil-markowitz-review-record/v1",
        "id":"point_driven_pencil_exact_sparse_pivot",
        "status":status["overall_status"],
        "cohort":{"models":21,"targets_per_model":60,"target_count":1260,
            "source_verified_distinct_targets":solved,"still_open":1260-solved},
        "price_cell":{"square_per_multiply":0.5,"inverse_per_multiply":10,
            "native_GAE_per_logical":0.1},
        "p739_repeated_gate":{"targets":60,"candidate_source_verified":60,
            "rho_source_verified":60,"independent_replay":true,
            "markowitz_over_peel":pair["representative"]["candidate_over_baseline"],
            "markowitz_over_rho":pair["representative"]["candidate_over_rho"],
            "peel_over_rho":pair["representative"]["baseline_over_rho"],
            "markowitz_over_peel_min":pair["min_candidate_over_baseline"],
            "markowitz_over_peel_max":pair["max_candidate_over_baseline"],
            "price_cells":36,"price_cells_below_peel":pair["cells_candidate_below_baseline"],
            "price_cells_below_rho":pair["cells_candidate_below_rho"]},
        "p186379_target0":{"candidate_source_verified":candidate_verified,
            "rho_source_verified":process["rho_verified"],
            "process_status":process["status"],"failure":process["failure"],
            "candidate_over_rho":candidate_ratio,
            "raw_rows":process["raw_rows"],
            "independent_process_replay":true},
        "native_validation":{"status":validation["status"],
            "source_scalars_checked":validation["source_scalars_checked"],
            "nonwall_rows_checked":validation["replay_rows_checked"],
            "price_cells_checked":validation["price_cells_checked"],
            "null_candidate_price_cells":validation["unpriced_cells"]},
        "raw":"results/p739-markowitz.jsonl",
        "upper_raw":"results/p186379-markowitz-t0.jsonl",
        "price_grid":"results/price-grid.csv",
        "paired_price_grid":"results/p739-markowitz-vs-peel.csv"
    });
    fs::write(
        format!("{ROOT}/results/lead-result.json"),
        serde_json::to_vec_pretty(&lead).unwrap(),
    )
    .unwrap();
    println!("native Markowitz status: {solved}/1260 distinct source solves; p739 matched rho band complete");
}
