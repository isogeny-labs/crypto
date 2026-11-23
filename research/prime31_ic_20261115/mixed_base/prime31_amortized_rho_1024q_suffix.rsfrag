
// Source-exact benchmark suffix. This file is APPENDED to the immutable,
// already-merged original prime31_aps_one_relation.rs by CI. All existing
// rho/Jacobian/EC routines remain byte-for-byte unchanged.
// New logic only selects the SAME public Q as the index calculus baseline.
// Frozen comparison: MIXED_AMORTIZED_MATCHED_RHO_PROTOCOL_20261117.md.
fn main() -> Result<(), String> {
    const TARGETS: usize = 1024;
    const RHO_SEED: u64 = 0x5248_4f5f_414d_4f52;
    const FOLD_SEED_XOR: u64 = 0x464f_4c44_5f33_3231;
    const STEP_CAP: u64 = 2_000_000;
    const WATCHDOG: usize = 512;
    let input = std::env::args().nth(1)
        .ok_or("usage: prime31_amortized_rho_compare <amortized_input.json>")?;
    let bytes = std::fs::read_to_string(&input)
        .map_err(|e| format!("cannot read frozen IC input: {e}"))?;
    let ic: serde_json::Value = serde_json::from_str(&bytes)
        .map_err(|e| format!("invalid frozen IC input: {e}"))?;
    if ic["schema"] != "prime31.mixed_4p4_amortized_complete_ic.v1"
        || ic["target_count"] != TARGETS
        || ic["completed_unknown_target_logs"] != TARGETS
        || ic["setup_rank"] != 61
        || ic["base_log_point_replays"] != 61
        || ic["has_same_run_optimized_rho"] != false
        || ic["is_nongeneric_s9_extraction"] != false
    {
        return Err("refuse mismatched or incomplete index-calculus source".into());
    }
    let targets = ic["targets"].as_array()
        .ok_or("missing IC public targets")?;
    if targets.len() != TARGETS {
        return Err("source target count differs from frozen protocol".into());
    }
    let ic_setup: u128 = ic["setup_ns"].as_str()
        .ok_or("missing charged IC setup")?
        .parse().map_err(|_| "bad IC setup time")?;
    let ic_desc: u128 = ic["descent_total_ns"].as_str()
        .ok_or("missing charged IC descents")?
        .parse().map_err(|_| "bad IC descent time")?;
    let mut all = Vec::with_capacity(TARGETS);
    let mut total_ns = [0u128; 2];
    let mut completed = [0usize; 2];
    let mut aa_ns = 0u128;
    let mut aa_completed = 0usize;
    let mut wins = [0usize; 2];

    // The original source performs the whole rho cold solve and its own
    // original-curve scalar replay IN the time returned from the routine.
    // The caller-held scalar is not passed to this solver.
    for (i, entry) in targets.iter().enumerate() {
        if entry["solver"]["case"] != i || entry["caller_check"] != true
            || entry["solver"]["status"] != "complete"
        {
            return Err(format!("incomplete input target index {i}"));
        }
        let k = entry["caller_only_scalar"].as_u64()
            .ok_or_else(|| format!("missing caller scalar {i}"))?;
        let q: (u64,u64) = serde_json::from_value(
            entry["solver"]["public_q"].clone()
        ).map_err(|e| format!("bad public Q {i}: {e}"))?;
        let seed = RHO_SEED ^ (i as u64)
            .wrapping_mul(0xd1b5_4a32_d192_ed03);
        let order = if i % 2 == 0 { [0usize,1usize] } else { [1usize,0usize] };
        let mut results: [serde_json::Value;2] =
            std::array::from_fn(|_| serde_json::Value::Null);
        for arm in order {
            let folded = arm == 1;
            let label_seed = if folded { seed ^ FOLD_SEED_XOR } else { seed };
            let res = neg_rho_cold_policy(
                q, label_seed, 32, 4, STEP_CAP, WATCHDOG, folded
            )?;
            let elapsed = res["cold_wall_ns"].as_u64()
                .ok_or("rho missing cold wall")? as u128;
            total_ns[arm] += elapsed;
            if res["complete_target_scalar_recovered"] == true {
                let got = res["log"].as_u64()
                    .ok_or("completed rho missing discrete logarithm")?;
                if got != k {
                    return Err(format!("rho wrong complete logarithm target {i}, arm {arm}"));
                }
                let mut verifier = Arithmetic::default();
                if verifier.scalar(PRIME31, got, Some(PRIME31.g)) != Some(q) {
                    return Err(format!("rho original curve point replay failure {i}, arm {arm}"));
                }
                completed[arm] += 1;
            }
            if res["public_Q"] != serde_json::json!(q) {
                return Err(format!("rho solver returned altered public Q {i}, arm {arm}"));
            }
            results[arm] = res;
        }
        if results[0]["complete_target_scalar_recovered"] == true
            && results[1]["complete_target_scalar_recovered"] == true
        {
            let ic_ns: u128 = entry["solver"]["descent_ns"].as_str()
                .ok_or("missing IC descent cost")?
                .parse().map_err(|_| "bad IC descent wall")?;
            for arm in 0..2 {
                let n = results[arm]["cold_wall_ns"].as_u64().unwrap() as u128;
                if ic_ns < n {
                    wins[arm] += 1;
                }
            }
        }
        let aa = if i < 64 {
            let res = neg_rho_cold_policy(
                q, seed, 32, 4, STEP_CAP, WATCHDOG, false
            )?;
            aa_ns += res["cold_wall_ns"].as_u64()
                .ok_or("rho A/A missing cold wall")? as u128;
            if res["complete_target_scalar_recovered"] == true {
                let answer = res["log"].as_u64().ok_or("AA missing log")?;
                if answer != k {
                    return Err(format!("A/A rho wrong scalar at target {i}"));
                }
                aa_completed += 1;
            }
            Some(res)
        } else { None };
        all.push(serde_json::json!({
            "case":i, "public_Q":q,
            "caller_only_k_for_audit":k,
            "execution_order":order,
            "native_unfolded_rho32":results[0],
            "native_folded_negation_rho32":results[1],
            "rho32_AA_first64":aa,
            "IC_descent_ns":entry["solver"]["descent_ns"],
            "same_public_Q_verified":true
        }));
    }
    let combined_ic = ic_setup + ic_desc;
    let counters = serde_json::json!({
        "schema":"prime31.mixed_ic_vs_exact_native_rho_1024q.v1",
        "standing":"[pcc:unverified]",
        "source":"byte-identical original prime31_aps_one_relation.rs rho methods plus wrapper",
        "curve":PRIME31,"targets":TARGETS,
        "IC_completed":TARGETS,"IC_setup_ns":ic_setup.to_string(),
        "IC_descents_ns":ic_desc.to_string(),
        "IC_fully_charged_batch_ns":combined_ic.to_string(),
        "unfolded_rho32_completed":completed[0],
        "folded_rho32_completed":completed[1],
        "unfolded_rho32_cold_total_ns":total_ns[0].to_string(),
        "folded_rho32_cold_total_ns":total_ns[1].to_string(),
        "unfolded_rho32_AA_first64_completed":aa_completed,
        "unfolded_rho32_AA_first64_total_ns":aa_ns.to_string(),
        "paired_IC_descent_only_wins_vs_rho":wins,
        "ratio_IC_full_batch_over_rho32_unfolded":combined_ic as f64/total_ns[0] as f64,
        "ratio_IC_full_batch_over_rho32_folded":combined_ic as f64/total_ns[1] as f64,
        "fully_complete_comparison":completed[0] == TARGETS && completed[1] == TARGETS,
        "cross_arm_method_source_is_identical_public_Q":true,
        "benchmarks_optimized_external_WZ_or_precomputed_BSGS":false,
        "independent_signed_PCC":false,
        "rho_seed_base":format!("{RHO_SEED:#x}"),
        "rho_step_cap":STEP_CAP,"rho_watchdog":WATCHDOG,
        "cases":all
    });
    println!("{}", counters);
    Ok(())
}
