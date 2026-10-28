//! Record the frozen capped process outcome in native code.
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "cap-process-status EXIT_CODE STEM");
    let exit_code: i32 = args[0].parse().unwrap();
    let stem = &args[1];
    let rows: Vec<Value> = BufReader::new(File::open(format!("{stem}.jsonl")).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect();
    assert!(!rows.is_empty());
    assert_eq!(rows[0]["schema"], "pencil-cohort-setup/v1");
    assert_eq!(rows[0]["bits"], 32);
    assert_eq!(rows[0]["p"], 186379);
    let target = rows.get(1);
    let status = if exit_code == 0 && target.is_some() {
        "complete"
    } else if exit_code == 124 {
        "timeout"
    } else {
        "process_failure"
    };
    let process = json!({
        "schema":"pencil-sparse-process/v1",
        "bits":32,"p":186379,"target_index":0,
        "status":status,"exit_code":exit_code,
        "started_utc":fs::read_to_string(format!("{stem}.started")).unwrap().trim(),
        "ended_utc":fs::read_to_string(format!("{stem}.ended")).unwrap().trim(),
        "raw_rows":rows.len(),
        "candidate_verified":target.map(|row| row["verified"].clone()).unwrap_or(Value::Null),
        "failure":target.map(|row| row["failure"].clone()).unwrap_or_else(|| json!(status)),
        "rho_verified":target.map(|row| row["rho"]["verified"].clone()).unwrap_or(Value::Null)
    });
    fs::write(
        format!("{stem}.status.json"),
        serde_json::to_vec_pretty(&process).unwrap(),
    )
    .unwrap();
}
