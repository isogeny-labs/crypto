//! Rust-only prime31 index-calculus frontier admission audit.
//!
//! This tool refuses to equate a capped algebraic run with a negative target,
//! a stage time with complete solver cost, or generic-equivalent root indexing
//! with a non-generic PKM/APS advancement. It reads frozen source-bound
//! development evidence, not a new execution or scientific PCC verdict.
//!
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_ic_frontier_audit
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_ic_frontier_audit
use serde_json::{json, Value};

fn document() -> Result<Value, String> {
    let m8: Value = serde_json::from_str(include_str!("../M8_NATURAL_F4_16_CI_EVIDENCE_20261115.json"))
        .map_err(|e| e.to_string())?;
    let m4: Value = serde_json::from_str(include_str!("../M4_NO_DUPLICATE_NORM_CI_20261115.json"))
        .map_err(|e| e.to_string())?;
    let get = |v: &Value, key: &str| -> Result<u64, String> {
        v.get(key).and_then(Value::as_u64).ok_or(format!("missing {key}"))
    };
    if get(&m8, "field_prime")? != 2_147_483_647
        || get(&m8, "curve_order")? != 2_147_446_831
        || get(&m8, "registered_public_mask_count")? != 16
    { return Err("wrong prime31 or panel fixture".into()); }
    let probes = m8.pointer("/original_ci_json/probes")
        .and_then(Value::as_array).ok_or("missing m8 probes")?;
    if probes.len() != 16 { return Err("not the frozen 16-query panel".into()); }
    let mut statuses = std::collections::BTreeMap::<String,u64>::new();
    for probe in probes {
        let f4 = probe.get("f4").ok_or("missing F4 row")?;
        let verdict = f4.get("verdict").and_then(Value::as_str)
            .ok_or("missing F4 verdict")?;
        if f4.get("oversize").and_then(Value::as_bool) == Some(true)
            && verdict != "undetermined" {
            return Err("censored F4 run misclassified as complete".into());
        }
        if f4.get("root_extraction").and_then(Value::as_str) != Some("not_implemented")
            && verdict == "basis_computed_without_root_extraction" {
            return Err("unexpected root extraction state".into());
        }
        *statuses.entry(verdict.to_string()).or_default() += 1;
    }
    if statuses.get("undetermined") != Some(&16)
        || get(&m8, "completed_relations_with_extracted_roots")? != 0
        || get(&m8, "completed_refutations")? != 0 {
        return Err("m8 source evidence changed; inspect before frontier classification".into());
    }
    let rows = m4.pointer("/data/rows").and_then(Value::as_array)
        .ok_or("missing m4 result rows")?;
    if rows.len() != 6 { return Err("m4 frozen paired panel size changed".into()); }
    let mut m4_sizes = Vec::new();
    for size in [4_u64,8,16] {
        let selected: Vec<_> = rows.iter().filter(|row|
            row.get("base_size").and_then(Value::as_u64) == Some(size)
        ).collect();
        if selected.len() != 2 { return Err(format!("m4 missing B={size} controls")); }
        let pos = selected.iter().find(|row|row["target_kind"]=="planted-positive")
            .ok_or("m4 planted control missing")?;
        let neg = selected.iter().find(|row|row["target_kind"]=="fixed-public")
            .ok_or("m4 public control missing")?;
        if pos["projectively_normalized_signed_rows"].as_u64()!=Some(1)
           || neg["projectively_normalized_signed_rows"].as_u64()!=Some(0)
           || pos["complete_ecdlp_recovery"].as_bool()!=Some(false) {
           return Err("m4 control classifications changed".into());
        }
        m4_sizes.push(json!({
            "factor_base":size,
            "planted_target_eliminant_ns":pos["target_dependent_eliminant_ns"],
            "natural_fixed_target_eliminant_ns":neg["target_dependent_eliminant_ns"],
            "planted_useful_rows":pos["projectively_normalized_signed_rows"],
            "fixed_public_rows":neg["projectively_normalized_signed_rows"],
            "full_cold_cost_available":false
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "tool":"Rust-only source-bound IC frontier audit, not a benchmark execution",
        "curve":{"p":2_147_483_647_u64,"n":2_147_446_831_u64,"cofactor":1},
        "frontier_order":[
            "PKM 2016: compositional rational-map factor-base algebraic IC",
            "APS 2018: one-relation prime-field summation-polynomial method",
            "native dense F4 vs tower F4 same-ideal oracle (#4275)",
            "certified quotient/subresultant actual-root worker (#4228/#4349)",
            "factorized S4 m4 and genuine target-dependent m8 (#4188)"
        ],
        "generic_controls":"BSGS and optimized rho reserved for complete-cold security cost; not the index-calculus frontier denominator",
        "m8_natural_f4":{
            "natural_targets":probes.len(),
            "verdicts":statuses,
            "signed_roots_extracted":0,
            "complete_non_generic_ic_solver":false,
            "stage_time_is_not_full_cost":true,
            "failure_mode":"oversize at degree 6; no target proved negative"
        },
        "m4_algebraic_component":{
            "paired_control_rows":m4_sizes,
            "planted_results_not_natural_relation_yield":true
        },
        "admission_rules":[
            "Use identical curve, base, target encodings and same admitted ideal",
            "Charge factor-base, preprocessing, failures, root extraction, verification and useful-rank increment",
            "Censored F4 is undetermined, never zero useful relations",
            "Require complete roots and exact signed point identity for every accepted row",
            "Require same-host complete-call results against strongest measured non-generic IC arm",
            "After full unknown-log recovery compare cold rho separately; independent PCC remains mandatory"
        ],
        "next_experiment":"Rust m8 fixed S3 certified quotient, then target-dependent S4 root extraction on frozen natural panel; matched PKM/APS controls",
    }))
}
fn main() {
    match document() {
        Ok(result) => println!("{}",serde_json::to_string_pretty(&result).unwrap()),
        Err(err) => {eprintln!("frontier audit failed: {err}");std::process::exit(1);}
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn source_frontier_audit() { super::document().unwrap(); }
}
