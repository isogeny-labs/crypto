//! Post-call presentation projection. Original run summary remains byte-exact.
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(
            "usage: production-quotient-compact ORIGINAL-SUMMARY.json NEW-PRESENTATION.json".into(),
        );
    }
    let s: Value = serde_json::from_reader(File::open(&args[1])?)?;
    if s["schema"] != "cover.production-quotient-analysis/v1"
        || s["comparative_eligibility"] != false
    {
        return Err("presentation scope".into());
    }
    let mut rows = Vec::new();
    for row in s["outcomes"].as_array().ok_or("outcomes")? {
        let mut out = serde_json::Map::new();
        for key in [
            "arm",
            "status",
            "exit_code",
            "verified",
            "actual_scalar",
            "full_base",
            "small_base",
            "b_count",
            "source_rows",
            "descent_residuals",
            "accepted_descents",
            "recorded_inner_wall_ns_practicality_only",
            "ranking_eligible",
            "pricing_band",
            "total_bitops",
        ] {
            out.insert(key.into(), row[key].clone());
        }
        out.insert(
            "observed_primitive_subtotal_bitops".into(),
            row["accounting"]["observed_primitive_subtotal_bitops"].clone(),
        );
        out.insert("phases".into(),json!(row["accounting"]["phases"].as_array().map(|phases|phases.iter().map(|p|json!({"phase":p["phase"],"diagnostic_priced_bitops":p["diagnostic_priced_bitops"]})).collect::<Vec<_>>())));
        out.insert(
            "all_prefix_bounds_checked".into(),
            row["gate_proof"]["all_prefix_bounds_checked"].clone(),
        );
        rows.push(Value::Object(out));
    }
    let mut out = s.as_object().ok_or("summary object")?.clone();
    out.insert(
        "schema".into(),
        json!("cover.production-quotient-presentation/v1"),
    );
    out.insert("outcomes".into(), json!(rows));
    out.insert(
        "scope".into(),
        json!("post-call field projection only; original summary retained separately"),
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut file, &Value::Object(out))?;
    writeln!(file)?;
    Ok(())
}
