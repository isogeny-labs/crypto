//! Reads precomputed frozen statistics; no fitting, resampling or workload selection.
use serde_json::{json, Value};
pub fn audit(s: &Value) -> Value {
    let mut groups = vec![];
    for g in s["groups"].as_array().unwrap() {
        let r = &g["paired_ratios"];
        let aa = &r["half/half-aa:query_muls"];
        let positive = &r["half-double/half:query_muls"];
        let aa_time = &r["half/half-aa:query_ns"];
        let positive_time = &r["half-double/half:query_ns"];
        let finite = g["group"].as_str().unwrap().ends_with(":finite");
        let counts = aa["n"].as_u64().unwrap_or(0) > 0
            && positive["n"].as_u64().unwrap_or(0) > 0
            && aa["min"] == 1.0
            && aa["max"] == 1.0
            && positive["min"] == 2.0
            && positive["max"] == 2.0;
        let separation = positive_time["median"]
            .as_f64()
            .zip(aa_time["max"].as_f64())
            .is_some_and(|(p, n)| p > 1.25 && p > n);
        let mut comparisons = vec![];
        for phase in ["query_ns", "cold_ns"] {
            let noise = &r[format!("half/half-aa:{phase}")];
            for arm in ["school", "newton", "lookup"] {
                let effect = &r[format!("{arm}/half:{phase}")];
                comparisons.push(json!({"reference":arm,"phase":phase,"precomputed_effect":effect,"precomputed_aa":noise,"note":"Read differences against the observed A/A range and descriptive interval; no statistical-significance or admitted timing conclusion."}));
            }
        }
        groups.push(json!({"group":g["group"],"finite":finite,"aa_query_count_exact":aa["min"]==1.0&&aa["max"]==1.0,"double_query_count_exact":positive["min"]==2.0&&positive["max"]==2.0,"count_controls_pass":counts,"positive_query_time_median":positive_time["median"],"aa_query_time_observed_range":[aa_time["min"],aa_time["max"]],"positive_time_above_frozen_1_25_and_observed_aa_max":separation,"local_query_timing_interpretable":counts&&separation,"admitted_L2_timing":false,"comparisons":comparisons}));
    }
    json!({"schema":"vieta-speed-controls/v1","source":"run/summary.json","calculation":"Only comparisons of the frozen native precomputed extrema/medians; no new statistics. Query positive median must exceed1.25 and the full observed A/A maximum. No timing admission follows.","groups":groups})
}
