//! EXP-R31-WZ-FRONTIER: reuse the repo's exact tuned negation rho on the
//! identical PUBLIC prime31 targets solved by the existing native Rust APS
//! pipeline. Caller-only known k is never passed to the solver.
//!
//! cargo +1.100.0 run --release --example prime31_wz_rho_frontier -- aps.json
//! Reads APS JSON; prints full per-target/cold-attempt JSON to stdout.
//! Correctness and timing status are producer development [pcc:unverified].
//!
//! See research/prime31_ic_20261115/native/WZ_FRONTIER_PROTOCOL_20261116.md
use crypto_lib::cryptanalysis::ic_boundary::{
    rho_reference_negation, rho_walk_with_policy, CountedGroup, GroupOps,
    NegationClasses, PrimeCurve, PrimePoint, RhoWalk, WalkPolicy,
};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const GX: u64 = 625_037_715;
const GY: u64 = 2_100_627_251;
const STEP_CAP: u64 = 2_000_000;
const MAX_ATTEMPTS: u64 = 3;
const SEED: u64 = 0x575a_4652_4f4e_5431;

fn curve() -> PrimeCurve {
    PrimeCurve { p: P, a: A, b: B }
}

fn get_u64(v: &Value, key: &str) -> Result<u64, String> {
    v.get(key).and_then(Value::as_u64).ok_or_else(|| format!("missing {key}"))
}
fn get_point(v: &Value) -> Result<PrimePoint, String> {
    let q = v.as_array().ok_or("missing public coordinate array")?;
    if q.len() != 2 { return Err("public Q must have exactly two coordinates".into()); }
    let x = q[0].as_u64().ok_or("invalid x")?;
    let y = q[1].as_u64().ok_or("invalid y")?;
    if x >= P || y >= P { return Err("noncanonical public point".into()); }
    Ok(PrimePoint::affine(x, y))
}

// A mode is either legacy WZ/short-window (0) or the *same library walk*
// with its published Brent-cycle policy (1).  No caller-known scalar here.
fn run_arm(c: &PrimeCurve, g: PrimePoint, q: PrimePoint,
           case: u64, mode: u8, arm_label: &str) -> Result<Value, String> {
    let cold = Instant::now();
    let mut attempts = Vec::new();
    let mut recovered: Option<u64> = None;
    let mut charged_steps = 0_u64;
    let mut charged_walks = 0_u64;
    let mut charged_gae = 0.0_f64;
    for repetition in 0..MAX_ATTEMPTS {
        let seed = SEED ^ case.wrapping_mul(0xd1b5_4a32_d192_ed03)
            ^ repetition.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        let rho = if mode == 0 {
            rho_reference_negation(c, g, q, N, seed, STEP_CAP)
        } else {
            rho_walk_with_policy(
                c, &NegationClasses { r: N }, g, q, N, seed, STEP_CAP,
                RhoWalk::negation(), WalkPolicy::v2(),
            )
        };
        charged_steps = charged_steps.saturating_add(rho.steps);
        charged_walks = charged_walks.saturating_add(rho.walks);
        charged_gae += rho.gae;

        let candidate = rho.recovered;
        if let Some(d) = candidate {
            // rho_reference_negation / rho_walk_with_policy ALREADY charge
            // the in-solver scalar verification in rho.wall_ns and rho.gae.
            if !rho.verified {
                return Err(format!("unchecked rho collision arm={arm_label} case={case} seed={seed}"));
            }
            recovered = Some(d);
        }
        // Retain capped/failed attempts as well as the successful one.
        attempts.push(json!({
            "seed": seed, "attempt_index": repetition,
            "cap_walk_steps": STEP_CAP, "full_rho": rho
        }));
        if recovered.is_some() { break; }
    }
    // End the fully cold SOLVER call before the separately charged caller-side
    // independent replay; otherwise WZ pays a second verifier while APS does
    // not. The WZ library has already paid for its own scalar verification.
    let solver_cold_wall_ns = cold.elapsed().as_nanos().to_string();
    let mut replay_ops = GroupOps::default();
    if let Some(d) = recovered {
        if c.mul(&mut replay_ops, g, d) != q {
            return Err(format!("external rho replay failed arm={arm_label} case={case}"));
        }
    }
    Ok(json!({
        "arm": arm_label, "complete": recovered.is_some(),
        "log": recovered, "cold_wall_ns": solver_cold_wall_ns,
        "caller_replay_group_ops_outside_cold": replay_ops,
        "total_attempts": attempts.len(), "charged_steps": charged_steps,
        "charged_walks": charged_walks, "charged_gae": charged_gae,
        "attempts": attempts
    }))
}

fn run(input: &Value) -> Result<Value, String> {
    let c = curve();
    let g = PrimePoint::affine(GX, GY);
    if !c.is_on_curve(g) { return Err("off-curve generator".into()); }
    let mut group_ops = GroupOps::default();
    if !c.mul(&mut group_ops, g, N).infinity {
        return Err("generator order certificate failed".into());
    }
    if get_u64(input, "p")? != P || get_u64(input, "n")? != N
        || get_u64(input, "cofactor")? != 1 {
        return Err("APS fixture curve/order mismatch".into());
    }
    let inputs = input["cases"].as_array().ok_or("missing APS cases")?;
    if inputs.len() != 24 || get_u64(input,"cases_count")? != 24 {
        return Err("require exact 24-Q composed APS cohort".into());
    }
    let mut seen = HashSet::new();
    let mut cases = Vec::with_capacity(inputs.len());
    let mut total_ns = [0u128;3];
    let mut completes = [0u64;3];
    let mut aps_total = 0u128;
    let mut paired_wz_over_aps = [0u64;3];

    for (i, case) in inputs.iter().enumerate() {
        if get_u64(case,"case")? != i as u64 { return Err("case order mismatch".into()); }
        let q = get_point(&case["public_Q"])?;
        let caller_k = get_u64(case,"caller_known_scalar_verifier_only")?;
        if caller_k <= 1 || caller_k >= N || !seen.insert(caller_k) {
            return Err("duplicate or invalid caller-side scalar".into());
        }
        if !c.is_on_curve(q) {
            return Err(format!("off-curve public Q case={i}"));
        }
        let mut caller_ops = GroupOps::default();
        if c.mul(&mut caller_ops,g,caller_k) != q {
            return Err(format!("caller fixture scalar mismatch case={i}"));
        }
        let aps = case.get("APS_window4_public_y").ok_or("missing APS composed arm")?;
        if aps["complete_target_scalar_recovered"] != true {
            return Err(format!("APS did not complete case={i}"));
        }
        let aps_ns = get_u64(aps,"cold_wall_ns")?;
        let aps_log = aps["attempts"].as_array()
            .and_then(|a|a.last())
            .and_then(|a|a["solution"]["log"].as_u64())
            .ok_or("missing APS scalar")?;
        if aps_log != caller_k { return Err(format!("wrong APS scalar case={i}")); }
        aps_total += aps_ns as u128;

        let mut arms: [Value;3] = std::array::from_fn(|_|Value::Null);
        let mut order = [0usize,1,2];
        order.rotate_left(i%3);
        for index in order {
            let label = match index {
                0 => "wz_negation_legacy", 1 => "wz_negation_brent_v2",
                _ => "wz_negation_brent_v2_AA"
            };
            arms[index] = run_arm(&c,g,q,i as u64,if index == 0 {0} else {1},label)?;
            let value = &arms[index];
            let ns = value["cold_wall_ns"].as_str()
                .ok_or("missing WZ cold wall")?.parse::<u128>()
                .map_err(|e|format!("WZ cold wall parse: {e}"))?;
            total_ns[index] += ns;
            if value["complete"] == true {
                let result = value["log"].as_u64().ok_or("complete without rho scalar")?;
                if result != caller_k {
                    return Err(format!("wrong recovered scalar case={i} arm={label}"));
                }
                completes[index] += 1;
                if ns < aps_ns as u128 {paired_wz_over_aps[index] += 1;}
            }
        }
        // The v2 A/A pair must reproduce the same solution and every
        // deterministic attempt count/trajectory, regardless of wall noise.
        if arms[1]["log"] != arms[2]["log"] ||
           arms[1]["total_attempts"] != arms[2]["total_attempts"] ||
           arms[1]["charged_steps"] != arms[2]["charged_steps"] {
            return Err(format!("v2 A/A trajectory mismatch case={i}"));
        }
        cases.push(json!({
            "case":i, "public_Q":[q.x,q.y],
            "caller_known_scalar_verifier_only":caller_k,
            "APS_composed_cold_ns":aps_ns,
            "chronological_WZ_order":order,
            "wz_negation_legacy":arms[0],
            "wz_negation_brent_v2":arms[1],
            "wz_negation_brent_v2_AA":arms[2]
        }));
    }
    let all_complete = completes.iter().all(|&v| v == 24);
    Ok(json!({
        "protocol":"WZ_FRONTIER_PROTOCOL_20261116.md",
        "standing":"[pcc:unverified]",
        "classification":"published APS S3 MITM-equivalent vs exact repo tuned WZ rho",
        "curve":{"p":P,"a":A,"b":B,"n":N,"cofactor":1,"generator":[GX,GY]},
        "APS_input_cases":inputs.len(),"APS_composed_total_cold_ns":aps_total.to_string(),
        "WZ_arm_names":["wz_negation_legacy","wz_negation_brent_v2",
                        "wz_negation_brent_v2_AA"],
        "WZ_complete_calls":completes,"all_WZ_complete":all_complete,
        "WZ_total_cold_ns":total_ns.map(|v| v.to_string()),
        "WZ_paired_wins_over_APS":paired_wz_over_aps,
        "WZ_over_APS_aggregate_ratio":if all_complete {
            Some(total_ns[0].min(total_ns[1]) as f64/aps_total as f64)
        }else{None},
        "WZ_AA_aggregate_drift": if total_ns[1] > 0 {
            Some((total_ns[1] as f64/total_ns[2] as f64-1.0).abs())
        }else{None},
        "same_host_sequential_runner":true,
        "scientific_limits":"sequential unisolated producer timing; historical APS-optimized Q cohort; different arithmetic backends; no external fastest rho or independently signed PCC",
        "cases":cases
    }))
}

fn main() -> Result<(),String> {
    let input_file = env::args().nth(1).ok_or("usage: prime31_wz_rho_frontier INPUT_APS_JSON")?;
    let raw = fs::read_to_string(input_file).map_err(|e|e.to_string())?;
    let input:Value = serde_json::from_str(&raw).map_err(|e|e.to_string())?;
    let output = run(&input)?;
    println!("{}",serde_json::to_string_pretty(&output).map_err(|e|e.to_string())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime31_curve_generator_and_short_log() {
        let c=curve();
        let g=PrimePoint::affine(GX,GY);
        assert!(c.is_on_curve(g));
        let mut ops=GroupOps::default();
        assert!(c.mul(&mut ops,g,N).infinity);
        for k in [1u64,2,17,31] {
            let q=c.mul(&mut ops,g,k);
            assert!(c.is_on_curve(q));
            assert_ne!(q,PrimePoint::INFINITY);
        }
    }
    #[test]
    fn fixture_validation_rejects_missing_cases() {
        assert!(run(&json!({"p":P,"n":N,"cofactor":1,"cases_count":0,"cases":[]})).is_err());
    }
}
