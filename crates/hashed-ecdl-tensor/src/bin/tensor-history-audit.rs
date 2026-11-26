//! Audit the source precision absorptions over every potentially active depth.
//!
//! This is an exact integer schedule certificate.  It uses the source's fixed
//! root leaf cutoff and strict eight-fold child shrinkage, but it neither
//! allocates a graph nor prices the remaining ring/rational bit operations.

use hashed_ecdl_tensor::admission::dimensions::{
    audit_recursion_composition, audit_uniform_history, UNIFORM_HISTORY_LAMBDA_THRESHOLD,
};
use serde_json::json;

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let lambda = match args.as_slice() {
        [] => UNIFORM_HISTORY_LAMBDA_THRESHOLD,
        [value] => value
            .parse::<u64>()
            .map_err(|_| "invalid root log-size lambda".to_owned())?,
        _ => return Err("usage: tensor-history-audit [ROOT_LOG2_SIZE]".into()),
    };
    let audit = audit_uniform_history(lambda).map_err(|error| error.to_string())?;
    let depths: Vec<_> = audit
        .depths
        .iter()
        .map(|case| {
            json!({
                "depth": case.depth,
                "shrink_factor": case.shrink_factor.to_string(),
                "u": case.u,
                "prior_a": case.prior_a,
                "current_a": case.current_a,
                "inherited_absorption": {
                    "lhs": case.inherited_absorption_lhs.to_string(),
                    "rhs": format!("2^{}", case.inherited_absorption_rhs_exponent),
                    "holds": case.inherited_absorption_holds
                },
                "evaluation_absorption": {
                    "lhs": case.evaluation_absorption_lhs.to_string(),
                    "rhs": format!("2^{}", case.evaluation_absorption_rhs_exponent),
                    "holds": case.evaluation_absorption_holds
                }
            })
        })
        .collect();
    let composition = if audit.threshold_proof_applies && audit.all_absorption_checks_hold {
        let value = audit_recursion_composition(lambda).map_err(|error| error.to_string())?;
        Some(json!({
            "local_child_exponent_constant": value.local_child_exponent_constant,
            "path_sum_constant": value.path_sum_constant,
            "active_tree_exponent_constant": value.active_tree_exponent_constant,
            "leaf_exponent_constant": value.leaf_exponent_constant,
            "leaf_scale_absorbed": value.leaf_scale_absorbed,
            "packet_entry_exponent_constant": value.packet_entry_exponent_constant,
            "packet_entry_polynomial_factor": value.packet_entry_polynomial_factor.to_string(),
            "packet_entry_absorbed": value.packet_entry_absorbed,
            "total_root_u_exponent_constant": value.total_root_u_exponent_constant,
            "total_root_log_exponent_constant": value.total_root_log_exponent_constant,
            "all_checks_hold": value.all_composition_checks_hold
        }))
    } else {
        None
    };
    let report = json!({
        "schema": "hashed-ecdl-tensor-history-audit/v1",
        "scope": "exact source-schedule absorption checks; no graph extraction or complete bit-cost theorem",
        "lambda": audit.lambda,
        "leaf_divisor": audit.leaf_divisor,
        "root_u": audit.root_u,
        "threshold_proof": {
            "minimum_lambda": UNIFORM_HISTORY_LAMBDA_THRESHOLD,
            "applies": audit.threshold_proof_applies,
            "minimum_possible_active_u": audit.scale_floor
        },
        "constants": {
            "finite_history_precision": audit.inherited_precision_constant,
            "normalized_local_precision": audit.normalized_precision_constant
        },
        "all_absorption_checks_hold": audit.all_absorption_checks_hold,
        "possible_active_depths": depths,
        "recursion_composition": composition
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-history-audit: {error}");
        std::process::exit(2);
    }
}
