//! Analysis of retained Runpod49 originals; never launches a scientific worker.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn price(w: &Value) -> Result<u128> {
    w["work_native"]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .try_fold(0u128, |s, p| {
            s.checked_add(
                p["diagnostic_priced_bitops"]
                    .as_str()
                    .ok_or("subtotal")?
                    .parse()?,
            )
            .ok_or_else(|| "price overflow".into())
        })
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: STUDY COLLECTED_STAGE NEW_OUTPUT".into());
    }
    let stage = Path::new(&a[0]);
    let run = stage.join("returned/task/run");
    let deletion: Value = serde_json::from_slice(&fs::read(stage.join("deletion.json"))?)?;
    if deletion["delete_http"] != 204 || deletion["independent_get_http"] != 404 {
        return Err("owned runner cleanup".into());
    }
    let original_failure = fs::read_to_string(run.join("analysis-original.log"))?;
    if fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "1"
        || original_failure.trim() != "Error: \"paired exact geometry rank_checks\""
    {
        return Err("original qualification outcome".into());
    }
    let mut outcomes = Vec::new();
    let mut ranks = Vec::new();
    for role in [
        "original-ic",
        "quadratic-table",
        "pair-unscreened",
        "pair-discriminant",
    ] {
        if fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err("worker process status".into());
        }
        let bytes = fs::read(run.join(format!("{role}.raw.json")))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "bd69a76db876fa1da120de0edf1033489327d937"
            || w["seed"] != 389
            || w["public"]["p"] != 1009
            || w["actual_scalar"] != 242624768583643540u64
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("worker source/target/endpoint".into());
        }
        let c = &w["work"]["collection"];
        if role.starts_with("pair-") {
            ranks.push(c["rank_checks"].clone());
        }
        outcomes.push(json!({"mode":role,"original_sha256":format!("{:x}",Sha256::digest(&bytes)),
            "actual_scalar":w["actual_scalar"],"producer_verified":w["verified"],"capped":w["capped"],
            "inner_wall_ns_practicality_only":w["inner_wall_ns"],"observed_incomplete_bitops":price(&w)?.to_string(),
            "phases":w["work_native"]["phases"],"counts":c["counts"],"stop":c["stop"],
            "sources":c["sources"].as_array().map(Vec::len),"projections":c["projections"].as_array().map(Vec::len),
            "graph_stats":c["graph_stats"],"rank_checks":c["rank_checks"],
            "slow_split_records":c["splits"].as_array().map(Vec::len),
            "screen_calls":c["discriminant_screen"]["calls"],
            "early_refusals":c["discriminant_screen"]["refusals"].as_array().map(Vec::len),
            "descent_residuals":w["work"]["descent_residuals"]}));
    }
    let mut timing = Vec::new();
    for r in &mut ranks {
        let mut times = Vec::new();
        for checkpoint in r.as_array_mut().ok_or("checkpoint array")? {
            let t = checkpoint["native"]
                .as_object_mut()
                .ok_or("native object")?
                .remove("inner_wall_ns")
                .ok_or("checkpoint timer")?;
            let _: u128 = t.as_str().ok_or("timer string")?.parse()?;
            times.push(t);
        }
        timing.push(times);
    }
    if ranks[0] != ranks[1] {
        return Err("additional rank differences beyond inner_wall_ns".into());
    }
    let output = json!({"schema":"cover.discriminant-pair.run-study/v1",
        "source_revision":"bd69a76db876fa1da120de0edf1033489327d937",
        "original_qualifier_exit_code":1,"original_qualifier_failure":original_failure.trim(),
        "outcomes":outcomes,"checkpoint_inner_wall_ns":timing,
        "checkpoint_records_equal_except_inner_wall_ns":true,
        "postexecution_analysis_only":true,"original_failure_preserved":true,
        "complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,
        "signed_independent_verdict":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
