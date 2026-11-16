//! Native reproduction of the stale c2pnb304w1 test expectation.
use crypto_lib::cryptanalysis::jacobian_transfer::{
    cost::CostModel, read_registry, screen, DEFAULT_FACTOR_BUDGET,
};

fn main() {
    let text = std::fs::read_to_string("docs/curves/registry.json").unwrap();
    let curves = read_registry(&text).unwrap();
    let curve = curves
        .iter()
        .find(|c| c.standard_names.iter().any(|s| s == "c2pnb304w1"))
        .unwrap();
    let current = CostModel::default();
    let mut old = current.clone();
    // Reproduce the documented pre-#1846 10*g^2 price, while retaining all
    // current exact geometry, subgroup checks and classification thresholds.
    old.jacobian_group_op_mults_measured = current
        .jacobian_group_op_mults_measured
        .iter()
        .map(|&(g, _)| (g, 10.0 * f64::from(g).powi(2)))
        .collect();
    old.jacobian_group_op_extrapolation_exponent = 2.0;
    let old_screen = screen(curve, &old, DEFAULT_FACTOR_BUDGET);
    let new_screen = screen(curve, &current, DEFAULT_FACTOR_BUDGET);
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "legacy_10g2_model_current_geometry": old_screen,
            "current_measured_cantor_model": new_screen,
            "only_changed_model_fields": ["jacobian_group_op_mults_measured", "jacobian_group_op_extrapolation_exponent"],
            "claim_ceiling": "Deterministic formula-model regression reproduction; no timing or security claim"
        }))
        .unwrap()
    );
}
