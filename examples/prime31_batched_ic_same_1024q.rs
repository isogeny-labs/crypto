//! Fully charged 1024-Q prime31 batched pair-table index-calculus comparison.
//! Uses the existing native-width prime_fast implementation and the **same**
//! 1024 public Q as the shared-advice BSGS run from PR #4532.
//! The underlying S3 pair table is generic-MITM equivalent: NO non-generic claim.
//! protocol: research/prime31_ic_20261115/batched_frontier/PROTOCOL_20261117.md
//! cargo run --release --example prime31_batched_ic_same_1024q -- input.json

use crypto_lib::cryptanalysis::prime_fast::{ic_multi_solve_ex, FastCurve};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const N: u64 = 2_147_446_831;
const GX: u64 = 625_037_715;
const GY: u64 = 2_100_627_251;
const CASES: usize = 1024;
const FB_SIZE: usize = 1920;
const EXTRA: usize = 32;
const MAX_WALK: u64 = 500_000;
const SEED: u64 = 20_261_117;

fn sample_vm_kib(key: &str) -> Option<u64> {
    fs::read_to_string("/proc/self/status").ok()?
        .lines().find(|l| l.starts_with(key))?
        .split_whitespace().nth(1)?.parse().ok()
}

fn run(input: &str) -> Result<Value, String> {
    let d: Value = serde_json::from_str(input).map_err(|e| e.to_string())?;
    if d["schema"] != "prime31.mixed_4p4_amortized_complete_ic.v1"
        || d["target_count"] != CASES
        || d["completed_unknown_target_logs"] != CASES
        || d["base_log_point_replays"] != 61
        || d["is_nongeneric_s9_extraction"] != false
    {
        return Err("input is not the complete frozen prime31 1024-Q corpus".into());
    }
    let rows = d["targets"].as_array().ok_or("missing target rows")?;
    if rows.len() != CASES {
        return Err("source corpus does not have 1024 targets".into());
    }

    let c = FastCurve::new("prime31-existing-ordinary", P, A, B, N);
    let g = c.point(GX, GY);
    if !c.is_on_curve(g) || c.mul(Some(g), N).is_some() {
        return Err("original ordinary prime31 generator/order validation failed".into());
    }
    let mut qs = Vec::with_capacity(CASES);
    let mut caller_only_k = Vec::with_capacity(CASES);
    let mut distinct = HashSet::with_capacity(CASES);
    for (i, row) in rows.iter().enumerate() {
        if row["solver"]["case"].as_u64() != Some(i as u64)
            || row["solver"]["status"] != "complete"
            || row["caller_check"] != true
        {
            return Err(format!("source row {i} not complete"));
        }
        let public: [u64; 2] = serde_json::from_value(
            row["solver"]["public_q"].clone()
        ).map_err(|e| format!("bad source Q row {i}: {e}"))?;
        let known = row["caller_only_scalar"].as_u64()
            .ok_or_else(|| format!("source known-answer absent row {i}"))?;
        let q = c.point(public[0], public[1]);
        if !c.is_on_curve(q) || known >= N || known == 0
            || c.mul(Some(g), known) != Some(q)
        {
            return Err(format!("source Q or caller-held k invalid row {i}"));
        }
        if !distinct.insert(public) {
            return Err(format!("duplicate source Q row {i}"));
        }
        qs.push(q);
        caller_only_k.push(known);
    }
    // PRE-SOLVE caller-only k is not passed into the solver.
    let start = Instant::now();
    let answer = ic_multi_solve_ex(
        &c, g, &qs, FB_SIZE, EXTRA, MAX_WALK, SEED, true, true,
    );
    let whole_ns = start.elapsed().as_nanos();
    let Some((guesses, report)) = answer else {
        return Ok(json!({
            "schema": "prime31.batched_ic_shared_frontier.v1",
            "status":"setup_or_relation_collection_failed",
            "p":P, "n":N, "fb":FB_SIZE, "extra":EXTRA, "max_walk":MAX_WALK,
            "target_count":CASES,
            "complete_logs":0,
            "all_failures_charged_ns":whole_ns.to_string(),
            "candidate_is_nongeneric":false,
            "pcc":"unverified"
        }));
    };
    if guesses.len() != CASES || report.targets_walked.len() != CASES {
        return Err("incomplete per-target report".into());
    }
    let mut cases = Vec::with_capacity(CASES);
    let mut completed = 0usize;
    let mut total_walked = 0u128;
    let mut max_walked = 0u64;
    for (i, (guess, &known)) in guesses.iter().zip(caller_only_k.iter()).enumerate() {
        let replay = guess.map(|k| k < N && c.mul(Some(g), k) == Some(qs[i]) && k == known)
            .unwrap_or(false);
        if guess.is_some() && !replay {
            return Err(format!("incorrect returned log or replay row {i}"));
        }
        completed += usize::from(replay);
        let walked = report.targets_walked[i];
        total_walked += walked as u128;
        max_walked = max_walked.max(walked);
        let (x,y) = c.canonical(qs[i]);
        cases.push(json!({
            "case":i,
            "public_Q":[x,y],
            "returned_log":guess,
            "caller_only_k_for_independent_replay":known,
            "verified":replay,
            "targets_walked":walked
        }));
    }
    Ok(json!({
        "schema":"prime31.batched_ic_shared_frontier.v1",
        "status":if completed == CASES {"complete"} else {"incomplete"},
        "source_schema":"prime31.mixed_4p4_amortized_complete_ic.v1",
        "p":P,"a":A,"b":B,"n":N,"G":[GX,GY],"cofactor":1,
        "fb":FB_SIZE,"extra":EXTRA,"max_walk":MAX_WALK,"seed":SEED,
        "target_count":CASES,"complete_logs":completed,
        "factor_base_determined_logs":report.determined_logs,
        "charged_setup_ms":report.setup_ms,
        "charged_total_ms":report.total_ms,
        "outer_timer_ns":whole_ns.to_string(),
        "total_walks":total_walked.to_string(),
        "max_walks_per_target":max_walked,
        "vm_hwm_kib":sample_vm_kib("VmHWM:"),
        "vm_rss_after_kib":sample_vm_kib("VmRSS:"),
        "cases":cases,
        "algorithm":"reused prime_fast::ic_multi_solve_ex table=true batched=true; S3 pair lookup",
        "candidate_is_nongeneric":false,
        "generic_mitm_equivalent_detector":true,
        "all_phase_costs_charged":true,
        "pcc":"unverified",
    }))
}
fn main() -> Result<(), String> {
    let path = std::env::args().nth(1)
        .ok_or("usage: prime31_batched_ic_same_1024q <original_1024_q.json>")?;
    let input = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let result = run(&input)?;
    println!("{}", result);
    if result["status"] != "complete" {
        return Err("batched 1024-Q candidate did not complete; failure preserved in JSON".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_prime31_curve_and_group_checks() {
        let c = FastCurve::new("prime31-existing-ordinary", P, A, B, N);
        let g = c.point(GX,GY);
        assert!(c.is_on_curve(g));
        assert_eq!(c.mul(Some(g), N), None);
        for k in [1u64,2,3,7,17,1024,N-1] {
            let q = c.mul(Some(g),k).unwrap();
            assert!(c.is_on_curve(q));
            assert_eq!(c.add(Some(q), c.mul(Some(g),N-k)),None);
        }
    }
}
