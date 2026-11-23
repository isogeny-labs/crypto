//! Postexecution analysis of retained Runpod48 data. No scientific worker.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn n(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "unsigned integer".into())
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn bump(h: &mut BTreeMap<String, u64>, key: String) {
    *h.entry(key).or_default() += 1;
}
fn price(w: &Value) -> Result<u128> {
    let mut sum = 0u128;
    for p in w["work_native"]["phases"].as_array().ok_or("phases")? {
        sum = sum
            .checked_add(
                p["diagnostic_priced_bitops"]
                    .as_str()
                    .ok_or("price")?
                    .parse()?,
            )
            .ok_or("price overflow")?;
    }
    Ok(sum)
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
    let mut outcomes = Vec::new();
    let mut candidate = None;
    for role in ["original-ic", "quadratic-table", "grounded-pairs"] {
        if fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err("original worker status".into());
        }
        let bytes = fs::read(run.join(format!("{role}.raw.json")))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "f58a7eb96be9fe47c40f5ef5dcaabe74e2392854"
            || w["seed"] != 389
            || w["public"]["p"] != 1009
        {
            return Err("original identity".into());
        }
        outcomes.push(json!({"mode":role,"original_sha256":hash(&bytes),"actual_scalar":w["actual_scalar"],"verified":w["verified"],"capped":w["capped"],"inner_wall_ns_practicality_only":w["inner_wall_ns"],"observed_incomplete_bitops":price(&w)?.to_string(),"phases":w["work_native"]["phases"],"counts":w["work"]["collection"]["counts"],"stop":w["work"]["collection"]["stop"],"descent_residuals":w["work"]["descent_residuals"]}));
        if role == "grounded-pairs" {
            candidate = Some(w);
        }
    }
    let w = candidate.ok_or("candidate")?;
    let c = &w["work"]["collection"];
    if w["verified"] != false
        || w["capped"] != true
        || !w["actual_scalar"].is_null()
        || c["stop"] != "split trace resource cap"
    {
        return Err("capped endpoint".into());
    }
    let splits = c["splits"].as_array().ok_or("quartic histories")?;
    if splits.len() != 262144
        || n(&c["counts"]["candidates"])?
            != n(&c["counts"]["refused"])?
                + n(&c["counts"]["duplicates"])?
                + c["sources"].as_array().ok_or("sources")?.len() as u64
                + 1
    {
        return Err("capped flow".into());
    }
    let (mut refusals, mut attempts, mut outcomes_hist, mut squarefree, mut linear) = (
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let (mut successful_factors, mut recorded_attempts, mut split_candidates) = (0u64, 0u64, 0u64);
    for s in splits {
        if n(&s["candidate_index"])? <= split_candidates {
            return Err("quartic candidate order".into());
        }
        split_candidates = n(&s["candidate_index"])?;
        let r = &s["report"];
        let count = r["attempts"].as_array().ok_or("attempt history")?.len() as u64;
        recorded_attempts += count;
        bump(&mut attempts, count.to_string());
        bump(
            &mut refusals,
            r["refusal"]
                .as_str()
                .unwrap_or("admitted two-factor split")
                .to_string(),
        );
        bump(
            &mut squarefree,
            r["squarefree_gcd"]
                .as_array()
                .ok_or("squarefree prefix")?
                .len()
                .to_string(),
        );
        bump(
            &mut linear,
            r["linear_gcd"]
                .as_array()
                .map_or_else(|| "not reached".into(), |x| x.len().to_string()),
        );
        if r["factors"].is_array() {
            successful_factors += 1;
        }
    }
    for source in c["sources"].as_array().ok_or("source history")? {
        bump(
            &mut outcomes_hist,
            source["graph"]["kind"]
                .as_str()
                .ok_or("graph outcome")?
                .into(),
        );
    }
    let raw_qualifier = fs::read(run.join("analysis-original.log"))?;
    if fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "1"
        || !String::from_utf8_lossy(&raw_qualifier).contains("worker identity grounded-pairs")
    {
        return Err("frozen failed qualifier".into());
    }
    let output = json!({"schema":"cover.quadratic-pair.cap-study/v1","source_revision":w["source_revision"],"original_qualifier_exit_code":1,"original_qualifier_sha256":hash(&raw_qualifier),"original_qualifier_failure":"worker identity grounded-pairs","outcomes":outcomes,"candidate":{"counts":c["counts"],"graph_stats":c["graph_stats"],"projected_rows":c["projections"].as_array().ok_or("projections")?.len(),"sparse_checkpoints":c["rank_checks"].as_array().ok_or("rank checks")?.len(),"quartic_histories":splits.len(),"quartic_refusals":refusals,"attempt_count_histogram":attempts,"recorded_random_polynomial_attempts":recorded_attempts,"successful_factorizations":successful_factors,"squarefree_gcd_length_histogram":squarefree,"linear_gcd_length_histogram":linear,"graph_outcomes":outcomes_hist},"no_scientific_worker_invoked":true,"original_frozen_qualifier_failure_preserved":true,"postexecution_analysis_only":true,"independent_component_replay_pending":true,"complete_cost":false,"total_bitops":null,"ratios":null,"rho_parity":false,"signed_execution_verdict":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
