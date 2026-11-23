//! Execute the whole numerical reduction on explicit finite test parameters.
//! This is a correctness demonstration, not a benchmark or an invocation of
//! the astronomical asymptotic parameter regime.

use isogeny_algos::bigint::Big;
use isogeny_algos::subnlogn::assembly::{Parameters, Plan};

fn demonstration() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = Parameters {
        radix_bits: 8,
        precision: 128,
        source_lengths: vec![5, 7],
        target_lengths: vec![8, 16],
        alpha: 4,
        chunk_bits: 1,
    };
    let mut plan = Plan::new(parameters)?;
    let input = Big::from_u128(u128::MAX);
    let product = plan.multiply(&input, &input)?;
    // Independent closed-form answer: (2^128-1)^2 = 2^256-2^129+1.
    assert_eq!(product.value.0, [1, 0, u64::MAX - 1, u64::MAX]);
    let execution = &product.execution;
    assert_eq!(execution.source_transforms, 3);
    assert_eq!(execution.source_volume_factors, 2);
    assert_eq!(execution.cyclic_convolutions, 3);
    assert!(execution.gaussian_expansions > 0);
    assert!(execution.gaussian_compressions > 0);
    assert!(execution.simultaneous_layer_calls > 0);
    let layer_nodes = execution
        .layer_receipts
        .iter()
        .fold(Big::zero(), |sum, receipt| sum.add(&receipt.network_nodes));
    let swap_nodes = execution
        .transform_swap_receipts
        .iter()
        .fold(Big::zero(), |sum, receipt| {
            sum.add(&receipt.work.recursive_nodes)
        });
    println!(
        concat!(
            "{{\"scope\":\"whole_numerical_flow_at_explicit_parameters\",",
            "\"published_asymptotic_regime\":false,",
            "\"known_answer_matches\":true,\"input_bits\":128,",
            "\"radix_bits\":8,\"precision\":128,",
            "\"source_shape\":[5,7],\"target_shape\":[8,16],",
            "\"source_transforms\":{},\"source_volume_factors\":{},",
            "\"gaussian_expansions\":{},\"gaussian_compressions\":{},",
            "\"polynomial_products\":{},\"simultaneous_layer_calls\":{},",
            "\"recursive_layer_nodes\":\"{}\",",
            "\"recursive_transform_swap_nodes\":\"{}\",",
            "\"coefficient_error_numerator\":\"{}\",",
            "\"coefficient_error_denominator\":\"{}\",",
            "\"product\":\"{}\"}}"
        ),
        execution.source_transforms,
        execution.source_volume_factors,
        execution.gaussian_expansions,
        execution.gaussian_compressions,
        execution.normalized_polynomial_products,
        execution.simultaneous_layer_calls,
        layer_nodes.to_dec(),
        swap_nodes.to_dec(),
        product.precision.final_coefficient_ulps.to_dec(),
        Big::from_u64(1).shl(product.precision.precision).to_dec(),
        product.value.to_dec()
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    demonstration()
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_answer_runs_the_whole_numerical_flow() {
        super::demonstration().unwrap();
    }
}
