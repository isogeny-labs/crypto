//! Print an exact finite ledger for the hashed-factor-base ECDL reduction.
//!
//! The default is the Curve25519 field and prime subgroup.  The output prices
//! setup, bounded relation attempts, contraction self-reduction calls, and
//! dense elimination.  Exponent surrogates omit theorem constants.

use hashed_ecdl_tensor::admission::reduction::audit_reduction_cost;
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use num_traits::One;
use serde_json::json;

const CURVE25519_SUBGROUP_TAIL: &str = "27742317777372353535851937790883648493";

fn decimal(value: &str, name: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.as_bytes(), 10).ok_or_else(|| format!("invalid {name}"))
}

fn curve25519_order() -> BigUint {
    (BigUint::one() << 252usize)
        + BigUint::parse_bytes(CURVE25519_SUBGROUP_TAIL.as_bytes(), 10)
            .expect("valid subgroup tail")
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let (field_modulus, subgroup_order) = match args.as_slice() {
        [] => (
            (BigUint::one() << 255usize) - BigUint::from(19u8),
            curve25519_order(),
        ),
        [field, order] => (
            decimal(field, "field modulus")?,
            decimal(order, "subgroup order")?,
        ),
        _ => return Err("usage: tensor-reduction-audit [FIELD_MODULUS SUBGROUP_ORDER]".into()),
    };
    let mut meter = Meter::new(Limits::default());
    let audit = audit_reduction_cost(&field_modulus, &subgroup_order, &mut meter)
        .map_err(|error| error.to_string())?;
    let report = json!({
        "schema": "hashed-ecdl-reduction-audit/v1",
        "scope": "exact finite reduction ledger and unit-constant exponent surrogates; no practical attack or external-theorem verification",
        "input": {
            "field_modulus": field_modulus.to_string(),
            "subgroup_order": subgroup_order.to_string(),
            "field_bits": audit.field_bits,
            "subgroup_bits": audit.subgroup_bits
        },
        "parameters": {
            "log2_field_bits_ceiling": audit.log_field_bits,
            "slots": audit.slots,
            "target_membership_mass": audit.target_membership_mass.to_string(),
            "threshold_upper_bound_using_4p": audit.threshold_upper_bound.to_string()
        },
        "finite_work_bounds": {
            "columns": audit.columns_upper_bound.to_string(),
            "factor_base_enumeration_candidates": audit.enumeration_candidates_upper_bound.to_string(),
            "relation_attempts_per_trial": audit.relation_attempts_upper_bound.to_string(),
            "interface_bits_per_query": audit.interface_bits_upper_bound.to_string(),
            "contraction_calls_per_trial": audit.contraction_calls_per_trial_upper_bound.to_string(),
            "expected_contraction_calls": audit.expected_contraction_calls_upper_bound.to_string(),
            "expected_contraction_calls_bits": audit.expected_contraction_calls_bits,
            "dense_elimination_field_operations": audit.dense_elimination_field_operations_bound.to_string(),
            "dense_elimination_bit_operations": audit.dense_elimination_bit_operations_bound.to_string(),
            "dense_elimination_work_bits": audit.dense_elimination_work_bits
        },
        "success_lower_bound": {
            "numerator": audit.trial_success_lower_numerator.to_string(),
            "denominator": audit.trial_success_lower_denominator.to_string(),
            "exceeds_one_sixth": audit.trial_success_exceeds_one_sixth,
            "expected_trials_upper_bound": 6,
            "native_policy": "configured finite trial cap returns a limit error; theorem schedule retries fresh trials"
        },
        "unit_constant_exponent_surrogates": {
            "one_contraction": audit.contraction_shape_exponent_surrogate.to_string(),
            "relation_phase": audit.relation_phase_exponent_surrogate.to_string(),
            "linear_algebra": audit.linear_algebra_exponent_surrogate.to_string()
        }
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("tensor-reduction-audit: {error}");
        std::process::exit(2);
    }
}
