//! Native analysis of original Runpod50 records; no scientific worker.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn price(w: &Value) -> Result<u128> {
    w["work_native"]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .try_fold(0u128, |sum, p| {
            sum.checked_add(
                p["diagnostic_priced_bitops"]
                    .as_str()
                    .ok_or("subtotal string")?
                    .parse()?,
            )
            .ok_or_else(|| "subtotal overflow".into())
        })
}
fn subtotal(v: &Value) -> Result<u128> {
    Ok(v["observed_incomplete_bitops"]
        .as_str()
        .ok_or("subtotal")?
        .parse()?)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: STUDY COLLECTED_STAGE NEW_OUTPUT".into());
    }
    let stage = Path::new(&a[0]);
    let run = stage.join("returned/task/run");
    let read = |p: &Path| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) };
    let deletion = read(&stage.join("deletion.json"))?;
    if deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
        || fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "0"
    {
        return Err("original qualification/owned cleanup".into());
    }
    let qb = fs::read(run.join("summary.json"))?;
    let q: Value = serde_json::from_slice(&qb)?;
    if q["schema"] != "cover.deferred-pair.qualified/v1"
        || q["completed"] != 8
        || q["excluded"] != 0
        || q["automatic_retries"] != 0
        || q["complete_cost"] != false
        || q["rho_parity"] != false
    {
        return Err("frozen original summary".into());
    }
    let mut outcomes = vec![];
    for (role, method) in [
        ("original-ic", "ic"),
        ("quadratic-table", "extension-sparse-remainders"),
        ("pair-discriminant", "discriminant-pairs"),
        ("pair-deferred", "deferred-pairs"),
        ("rho-plain-b1", "rho"),
        ("rho-neg-b1", "rho"),
        ("rho-plain-b16", "rho"),
        ("rho-neg-b16", "rho"),
    ] {
        if fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err("worker exit".into());
        }
        let bytes = fs::read(run.join(format!("{role}.raw.json")))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "11c9a0312f0b0a8ae85be325e72cbf9602d9d777"
            || w["method"] != method
            || w["seed"] != 389
            || w["public"]["p"] != 1009
            || w["actual_scalar"] != 242624768583643540u64
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("original source/method/endpoint".into());
        }
        let c = &w["work"]["collection"];
        let phases:Vec<_>=w["work_native"]["phases"].as_array().ok_or("phases")?.iter()
            .map(|p|json!({"phase":p["phase"],"observed_incomplete_bitops":p["diagnostic_priced_bitops"]})).collect();
        let ranks:Vec<_>=c["rank_checks"].as_array().map_or(&[][..],Vec::as_slice).iter()
            .map(|r|json!({"b_count":r["b_count"],"rows":r["rows"],"rank_certificate":r["result"]["rank_certificate"],
                "observed_incomplete_bitops":r["native"]["observed_incomplete_subtotal"],"inner_wall_ns":r["native"]["inner_wall_ns"]})).collect();
        let time = fs::read_to_string(run.join(format!("{role}.time.txt")))?;
        let real = time
            .lines()
            .find_map(|l| l.strip_prefix("real_s="))
            .ok_or("process real")?
            .parse::<f64>()?;
        if !real.is_finite() || real <= 0. {
            return Err("process real domain".into());
        }
        outcomes.push(json!({"mode":role,"original_sha256":hash(&bytes),"actual_scalar":w["actual_scalar"],
            "producer_verified":true,"capped":false,"inner_wall_ns_practicality_only":w["inner_wall_ns"],
            "process_real_seconds_practicality_only":real,"observed_incomplete_bitops":price(&w)?.to_string(),
            "phases":phases,"counts":c["counts"],"graph_stats":c["graph_stats"],"stop":c["stop"],
            "sources":c["sources"].as_array().map(Vec::len),"projections":c["projections"].as_array().map(Vec::len),
            "rank_checks":ranks,"slow_split_records":c["splits"].as_array().map(Vec::len),
            "screen_calls":c["discriminant_screen"]["calls"],"early_refusals":c["discriminant_screen"]["refusals"].as_array().map(Vec::len),
            "descent_residuals":w["work"]["descent_residuals"],"accepted_descents":w["work"]["descents"].as_array().map(Vec::len),
            "rho_options":w["rho_options"],"rho_steps":w["work"]["run"]["steps"]}));
    }
    let old = subtotal(&outcomes[2])?;
    let new = subtotal(&outcomes[3])?;
    let delta = i128::try_from(old)?
        .checked_sub(i128::try_from(new)?)
        .ok_or("delta overflow")?;
    let mut phase_deltas = vec![];
    for (old, new) in outcomes[2]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .zip(outcomes[3]["phases"].as_array().ok_or("phases")?)
    {
        if old["phase"] != new["phase"] {
            return Err("phase alignment".into());
        }
        let before: i128 = old["observed_incomplete_bitops"]
            .as_str()
            .ok_or("phase subtotal")?
            .parse()?;
        let after: i128 = new["observed_incomplete_bitops"]
            .as_str()
            .ok_or("phase subtotal")?
            .parse()?;
        phase_deltas.push(json!({"phase":old["phase"],"grounded_observed_bitops":before.to_string(),"deferred_observed_bitops":after.to_string(),"observed_saving":(before-after).to_string()}));
    }
    let mut cheapest = 4;
    let mut fastest = 4;
    for i in 5..8 {
        if subtotal(&outcomes[i])? < subtotal(&outcomes[cheapest])? {
            cheapest = i;
        }
        if outcomes[i]["inner_wall_ns_practicality_only"]
            .as_u64()
            .ok_or("rho timer")?
            < outcomes[fastest]["inner_wall_ns_practicality_only"]
                .as_u64()
                .ok_or("rho timer")?
        {
            fastest = i;
        }
    }
    let output = json!({"schema":"cover.deferred-pair.run-study/v1","source_revision":q["source_revision"],
        "original_qualified_summary_sha256":hash(&qb),"original_qualifier_exit_code":0,"outcomes":outcomes,
        "deferred_observed_saving_vs_grounded":delta.to_string(),"deferred_observed_saving_percent_vs_grounded":100.*delta as f64/old as f64,
        "phase_comparison":phase_deltas,"cheapest_observed_rho_mode":outcomes[cheapest]["mode"],
        "fastest_observed_rho_mode":outcomes[fastest]["mode"],"matched_original_proof":q["matched_pair"],
        "postexecution_analysis_only":true,"planned":8,"completed":8,"excluded":0,"automatic_retries":0,
        "sample_size_per_arm":1,"complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,
        "signed_independent_verdict":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
