//! DEVELOPMENT PUBLIC-INPUT CENSUS ONLY. No selector or relation census.
use serde_json::json;
use std::collections::BTreeMap;
use std::time::Instant;
use twist_relations::{core::Parameters, tr04_verify::model_info};

fn choose(n: u128, k: u128) -> u128 {
    if n < k {
        return 0;
    }
    (0..k).fold(1, |value, i| value * (n - i) / (i + 1))
}

fn main() -> Result<(), String> {
    let started = Instant::now();
    let calibration_path = std::env::args()
        .nth(1)
        .ok_or("Usage: tr04-preflight-estimate-helper CALIBRATION.json")?;
    let calibration: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&calibration_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let control_ns = calibration["control_audit_elapsed_ns"]
        .as_str()
        .ok_or("missing control duration")?
        .parse::<u128>()
        .map_err(|e| e.to_string())?;
    let max_selector_ns = calibration["methods"]
        .as_array()
        .ok_or("missing methods")?
        .iter()
        .map(|row| row["elapsed_ns"].as_str().unwrap().parse::<u128>().unwrap())
        .max()
        .ok_or("empty methods")?;
    let mut fields = Vec::new();
    let mut maximum_field_work_envelope = 0.0f64;
    for p in [7, 11, 19] {
        let mut singular = 0u64;
        let mut eligible_orders = BTreeMap::<u64, u64>::new();
        for a in 0..p {
            for b in 0..p {
                let info = model_info(Parameters { p, a, b })?;
                if !info.nonsingular {
                    singular += 1;
                }
                if info.eligible {
                    *eligible_orders
                        .entry(info.group_order.ok_or("missing order")?)
                        .or_default() += 1;
                }
            }
        }
        let eligible = eligible_orders.values().sum::<u64>();
        let mut base2_cache_ceiling = 0u128;
        let mut base3_cache_ceiling = 0u128;
        let mut base2_target_cache_ceiling = 0u128;
        let mut base3_target_cache_ceiling = 0u128;
        let mut selector_scale_sum = 0.0f64;
        for (&n, &models) in &eligible_orders {
            let canonical_xs = (n as u128 - 1) / 2;
            // At most one cache per canonical base and each prepaid flag.
            let two = models as u128 * choose(canonical_xs, 2) * 2;
            let three = models as u128 * choose(canonical_xs, 3) * 2;
            base2_cache_ceiling += two;
            base3_cache_ceiling += three;
            base2_target_cache_ceiling += two * (n as u128 - 1);
            base3_target_cache_ceiling += three * (n as u128 - 1);
            let progression_scale = (p * (p - 1) * 8) as f64 / (7.0 * 6.0 * 3.0);
            for b in [2, 3] {
                let states = (0..=b).map(|k| choose(canonical_xs, k)).sum::<u128>();
                let iid_scale = (8 * p as u128 * states) as f64 / (3.0 * 7.0 * 22.0);
                selector_scale_sum += models as f64 * iid_scale.max(progression_scale);
            }
        }
        // This is deliberately an explicit heuristic, not a proved wall-time
        // bound: target counts are linear and elimination width cubic.
        let cache_seconds = control_ns as f64 / 1e9
            * (base2_target_cache_ceiling as f64 / 12.0
                + base3_target_cache_ceiling as f64 / 12.0 * (4.0f64 / 3.0).powi(3));
        let cache_with_margin = 2.0 * cache_seconds;
        // Use every arm's maximum measured M3 selector duration; 8x margin
        // covers replay, table-width and other per-state implementation cost.
        let selectors_with_margin = max_selector_ns as f64 / 1e9 * selector_scale_sum * 7.0 * 8.0;
        let raw_field_work_estimate = cache_with_margin + selectors_with_margin;
        let field_work_envelope = (raw_field_work_estimate / 100.0).ceil() * 100.0;
        maximum_field_work_envelope = maximum_field_work_envelope.max(field_work_envelope);
        fields.push(json!({"prime":p,"model_dispositions":p*p,"singular":singular,
            "eligible_models":eligible,"other_nonsingular":p*p-singular-eligible,
            "eligible_order_model_counts":eligible_orders,
            "base2_cache_ceiling":base2_cache_ceiling.to_string(),"base3_cache_ceiling":base3_cache_ceiling.to_string(),
            "base2_target_cache_ceiling":base2_target_cache_ceiling.to_string(),"base3_target_cache_ceiling":base3_target_cache_ceiling.to_string(),
            "cache_seconds_heuristic_before_margin":cache_seconds,"cache_seconds_with_2x_margin":cache_with_margin,
            "selector_seconds_with_8x_margin":selectors_with_margin,"raw_field_work_estimate_seconds":raw_field_work_estimate,
            "field_work_planning_envelope_seconds":field_work_envelope}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"tr04.development-public-input-census/v1",
        "scope":"Public model orders and combinatorial cache ceilings only; no M8 selectors, rank distributions, paid-work comparison or primary relation census executed.",
        "elapsed_ns":started.elapsed().as_nanos().to_string(),"fields":fields,
        "calibration":{"path":calibration_path,"control_ns":control_ns.to_string(),"maximum_selector_ns":max_selector_ns.to_string(),"control_target_count":12,"control_dimension":3,"selector_reference_budget":3,"selector_reference_prime":7,"selector_reference_state_ceiling":22},
        "planning":{"fixed_seconds":219,"per_sample_seconds":maximum_field_work_envelope,"threads":1,"estimated_total_seconds":219.0+3.0*maximum_field_work_envelope,
        "fixed_derivation":"5 fresh standalone-crate builds * ceil(36.88s observed helper build) + 34s observed final native dev checks =219s. Five roles are hosted preflight, three fields, hosted aggregate; setup/router/checkpoint variability is not independently measured.",
        "field_work_derivation":"Per field: 2*control_seconds*(B2 target-cache ceiling/12+B3 target-cache ceiling/12*(4/3)^3) + 8*7*max_selector_seconds*sum_model_and_B max(8*p*sum_k0..B C((n-1)/2,k)/(3*7*22),p*(p-1)*8/(7*6*3)). Round up to100s; per_sample is maximum of the3 field envelopes, total=fixed+3*maximum.",
        "limitations":"Timing extrapolation and margins are heuristic, not proved runtime bounds or admitted performance. No full panel, M8 selector census, primary rank distribution or quality tuning was performed. New runner CPU/IO/network can differ; hard deadlines still retain failures."}})).map_err(|e|e.to_string())?);
    Ok(())
}
