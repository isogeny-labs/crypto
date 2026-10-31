//! Fail-closed selection over a frozen K2816 scalar/batch index-stage panel.
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const SOURCE_SHA256: &str = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507";
const BASE_HASH: &str = "9208460bf63be634f0aee6091abaaed855e05004fc2894e28a9c1fd394c35ec4";
const ARMS: [(&str, bool); 4] = [
    ("1-scalar", false),
    ("2-batch", true),
    ("3-batch", true),
    ("4-scalar", false),
];

fn positive(row: &Value, key: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let value = row[key].as_f64().ok_or("missing timing")?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("invalid timing {key}").into());
    }
    Ok(value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::args()
        .nth(1)
        .ok_or("usage: n83_root_batch_highk_analyze DIR")?;
    let root = Path::new(&root);
    let hashes = fs::read_to_string(root.join("sha256.txt"))?;
    if !hashes.contains(SOURCE_SHA256) {
        return Err("frozen source digest absent".into());
    }
    let mut records = Vec::new();
    for (arm, batch) in ARMS {
        let dir = root.join(arm);
        if fs::read_to_string(dir.join("exit_status.txt"))?.trim() != "0" {
            return Err(format!("{arm} did not exit successfully").into());
        }
        let row: Value = serde_json::from_slice(&fs::read(dir.join("stdout.jsonl"))?)?;
        if row["kind"] != "compact_orbit_index_stage"
            || row["status"] != "STAGE_DIAGNOSTIC_ONLY"
            || row["n"] != 83
            || row["batch_roots"] != batch
            || row["base_hash"] != BASE_HASH
            || row["orbit_columns"] != 2816
            || row["factor_base_points"] != 467456
        {
            return Err(format!("{arm} has invalid identity or counts").into());
        }
        for key in [
            "index_build_ms",
            "setup_total_ms",
            "base_load_ms",
            "normal_basis_and_selftest_ms",
        ] {
            positive(&row, key)?;
        }
        for key in ["state_digest_blake3", "table_digest_blake3"] {
            if row[key].as_str().is_none_or(|value| value.len() != 64) {
                return Err(format!("{arm} has no full {key}").into());
            }
        }
        records.push(row);
    }
    for key in [
        "regular_states",
        "root_table_entries",
        "state_digest_blake3",
        "table_digest_blake3",
    ] {
        if records.iter().any(|row| row[key] != records[0][key]) {
            return Err(format!("{key} differs across arms").into());
        }
    }
    let index_ms: Vec<f64> = records
        .iter()
        .map(|row| positive(row, "index_build_ms"))
        .collect::<Result<_, _>>()?;
    let setup_ms: Vec<f64> = records
        .iter()
        .map(|row| positive(row, "setup_total_ms"))
        .collect::<Result<_, _>>()?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"n83.root-batch-k2816-stage-panel/v1",
            "status":"CLOUD_STAGE_PANEL_COMPLETE",
            "candidate_id":null,
            "source_sha256":SOURCE_SHA256,
            "base_hash":BASE_HASH,
            "orbit_columns":2816,
            "factor_base_points":467456,
            "regular_states":records[0]["regular_states"],
            "root_table_entries":records[0]["root_table_entries"],
            "state_digest_blake3":records[0]["state_digest_blake3"],
            "table_digest_blake3":records[0]["table_digest_blake3"],
            "index_build_ms":index_ms,
            "setup_total_ms":setup_ms,
            "paired_scalar_over_batch_index_ratio":[index_ms[0]/index_ms[1],index_ms[3]/index_ms[2]],
            "mean_scalar_index_ms":(index_ms[0]+index_ms[3])/2.0,
            "mean_batch_index_ms":(index_ms[1]+index_ms[2])/2.0,
            "all_counts_and_digests_identical":true,
            "measurement_scope":"index_stage_only",
        }))?
    );
    Ok(())
}
