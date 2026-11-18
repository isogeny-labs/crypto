//! Exact structural audit for the projector-only n=83 image router.
//!
//! This binary records finite group counts and checked cover identities.  It
//! is not a timing benchmark and does not claim to classify routers that add
//! a source-label, descent, or Prym-cancellation oracle.

use clap::Parser;
use crypto_lib::binary_ecc::curve::{point_add, scalar_mul, BinaryCurve};
use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor, OrdinaryBinaryCover,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const SUBGROUP_ORDER: &str = "2417851639230796216685689";
const P_X: &str = "477f77103dfad59850800";
const P_Y: &str = "2fa5e737d542c4e4fd5c3";

#[derive(Parser)]
#[command(
    name = "n83-adaptive-image-router-audit",
    about = "Certify the coset collapse and kernel ambiguity of projector-only routing"
)]
struct Args {
    /// Write the deterministic JSON certificate here; omit for stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn f8_modulus() -> IrreduciblePoly {
    IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    }
}

fn n83_modulus() -> IrreduciblePoly {
    IrreduciblePoly {
        degree: 83,
        low_terms: vec![0, 1, 2, 45],
    }
}

fn elt(value: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(value), 3)
}

fn fixed_f8_cover() -> Result<OrdinaryBinaryCover, String> {
    OrdinaryBinaryCover::new(3, f8_modulus(), elt(1), elt(3)).map_err(|error| error.to_string())
}

fn target_curve(
    m: u32,
    irreducible: IrreduciblePoly,
    a: F2mElement,
    b: F2mElement,
    generator: BinaryPoint,
    order: BigUint,
    cofactor: BigUint,
) -> BinaryCurve {
    BinaryCurve {
        m,
        irreducible,
        a,
        b,
        generator,
        order,
        cofactor,
    }
}

fn multiples(
    base: &MumfordDivisor,
    count: usize,
    cover: &OrdinaryBinaryCover,
) -> Result<Vec<MumfordDivisor>, String> {
    let mut result = Vec::with_capacity(count);
    let mut current = MumfordDivisor::identity(cover.field_degree());
    for _ in 0..count {
        result.push(current.clone());
        current = current
            .try_add(base, cover.source_curve())
            .map_err(|error| error.to_string())?;
    }
    Ok(result)
}

fn polynomial_json(polynomial: &F2mPoly) -> Value {
    json!({
        "degree": polynomial.degree(),
        "coefficients_low_to_high": polynomial
            .coeffs
            .iter()
            .map(|coefficient| format!("0x{:x}", coefficient.to_biguint()))
            .collect::<Vec<_>>(),
    })
}

fn point_json(point: &BinaryPoint) -> Value {
    match point {
        BinaryPoint::Infinity => json!({"infinity": true}),
        BinaryPoint::Affine { x, y } => json!({
            "infinity": false,
            "x": format!("0x{:x}", x.to_biguint()),
            "y": format!("0x{:x}", y.to_biguint()),
        }),
    }
}

fn fixed_f8_audit() -> Result<Value, String> {
    let cover = fixed_f8_cover()?;
    cover
        .verify_certificate()
        .map_err(|error| error.to_string())?;

    let a = MumfordDivisor::try_from_point(cover.source_curve(), &elt(2), &elt(3))
        .map_err(|error| error.to_string())?;
    let b = MumfordDivisor::try_from_point(cover.source_curve(), &elt(3), &elt(1))
        .map_err(|error| error.to_string())?;
    let identity = MumfordDivisor::identity(3);
    let a9 = a
        .try_scalar_mul(&BigUint::from(9u32), cover.source_curve())
        .map_err(|error| error.to_string())?;
    let a27 = a
        .try_scalar_mul(&BigUint::from(27u32), cover.source_curve())
        .map_err(|error| error.to_string())?;
    let b18 = b
        .try_scalar_mul(&BigUint::from(18u32), cover.source_curve())
        .map_err(|error| error.to_string())?;
    let b27 = b
        .try_scalar_mul(&BigUint::from(27u32), cover.source_curve())
        .map_err(|error| error.to_string())?;
    let b54 = b
        .try_scalar_mul(&BigUint::from(54u32), cover.source_curve())
        .map_err(|error| error.to_string())?;
    if a27 != identity
        || a9 == identity
        || b54 != identity
        || b27 == identity
        || b18 == identity
        || a9 != b18
    {
        return Err("fixed GF(8) Jacobian presentation checks failed".into());
    }

    let a_multiples = multiples(&a, 9, &cover)?;
    let b_multiples = multiples(&b, 54, &cover)?;
    let mut divisors = Vec::with_capacity(486);
    for a_multiple in &a_multiples {
        for b_multiple in &b_multiples {
            let divisor = a_multiple
                .try_add(b_multiple, cover.source_curve())
                .map_err(|error| error.to_string())?;
            if divisors.contains(&divisor) {
                return Err("fixed GF(8) presentation produced a duplicate class".into());
            }
            divisors.push(divisor);
        }
    }
    if divisors.len() != 486 {
        return Err("fixed GF(8) presentation did not produce 486 classes".into());
    }

    let q = BinaryPoint::Affine {
        x: elt(4),
        y: elt(4),
    };
    let elliptic = target_curve(
        3,
        f8_modulus(),
        elt(1),
        elt(3),
        q.clone(),
        BigUint::from(6u32),
        BigUint::from(1u32),
    );
    if scalar_mul(&elliptic, &q, &BigUint::from(6u32)) != BinaryPoint::Infinity
        || scalar_mul(&elliptic, &q, &BigUint::from(2u32)) == BinaryPoint::Infinity
        || scalar_mul(&elliptic, &q, &BigUint::from(3u32)) == BinaryPoint::Infinity
    {
        return Err("fixed GF(8) target generator does not have order six".into());
    }
    let target_points: Vec<_> = (0u32..6)
        .map(|scalar| scalar_mul(&elliptic, &q, &BigUint::from(scalar)))
        .collect();

    let mut images = Vec::with_capacity(divisors.len());
    let mut fiber_sizes = [0usize; 6];
    for divisor in &divisors {
        let image = cover
            .pushforward_arbitrary_jacobian_class(divisor)
            .map_err(|error| error.to_string())?;
        let label = target_points
            .iter()
            .position(|point| point == &image)
            .ok_or_else(|| "norm image is outside the six-point target".to_string())?;
        fiber_sizes[label] += 1;
        images.push(image);
    }
    if fiber_sizes != [81; 6] {
        return Err(format!("unexpected norm fiber sizes: {fiber_sizes:?}"));
    }

    let mut projected_zero_pairs = 0usize;
    let mut exact_zero_pairs = 0usize;
    let mut exact_pairs_not_projected_zero = 0usize;
    for (left_index, left) in divisors.iter().enumerate() {
        for (right_index, right) in divisors.iter().enumerate() {
            let projected_sum = point_add(&elliptic, &images[left_index], &images[right_index]);
            let exact_sum = left
                .try_add(right, cover.source_curve())
                .map_err(|error| error.to_string())?;

            // The presentation window is 0 <= i < 9, 0 <= j < 54 with
            // 9A = 18B.  Reduce the coordinate sum back into that window,
            // then independently compare both the Jacobian sum and its norm.
            let mut sum_i = left_index / 54 + right_index / 54;
            let mut sum_j = left_index % 54 + right_index % 54;
            if sum_i >= 9 {
                sum_i -= 9;
                sum_j += 18;
            }
            sum_j %= 54;
            let sum_index = sum_i * 54 + sum_j;
            if exact_sum != divisors[sum_index] || projected_sum != images[sum_index] {
                return Err(format!(
                    "arbitrary-class homomorphism failed for pair ({left_index}, {right_index})"
                ));
            }

            let projected_zero = projected_sum == BinaryPoint::Infinity;
            let exact_zero = exact_sum.is_identity();
            projected_zero_pairs += usize::from(projected_zero);
            exact_zero_pairs += usize::from(exact_zero);
            exact_pairs_not_projected_zero += usize::from(exact_zero && !projected_zero);
        }
    }

    let kernel_witness_index = divisors
        .iter()
        .zip(&images)
        .position(|(divisor, image)| !divisor.is_identity() && *image == BinaryPoint::Infinity)
        .ok_or_else(|| "no nonzero norm-kernel witness found".to_string())?;
    let kernel_witness = &divisors[kernel_witness_index];
    let expected_projected_zero_pairs = divisors.len() * fiber_sizes[0];
    let expected_exact_zero_pairs = divisors.len();
    if projected_zero_pairs != expected_projected_zero_pairs
        || exact_zero_pairs != expected_exact_zero_pairs
        || exact_pairs_not_projected_zero != 0
    {
        return Err(format!(
            "pair counts differ from group predictions: projected={projected_zero_pairs}, exact={exact_zero_pairs}, bad={exact_pairs_not_projected_zero}"
        ));
    }

    let false_projected_pairs = projected_zero_pairs - exact_zero_pairs;
    let kernel_ambiguity_factor = projected_zero_pairs / exact_zero_pairs;
    Ok(json!({
        "field": "GF(8)",
        "cover_parameters": {"a": "0x1", "b": "0x3"},
        "jacobian_presentation": {
            "classes": divisors.len(),
            "generator_A_order": 27,
            "generator_B_order": 54,
            "relation": "[9]A=[18]B",
            "presentation_window": "0<=i<9, 0<=j<54",
            "all_classes_distinct": true,
        },
        "norm": {
            "target_points": target_points.len(),
            "fiber_sizes_by_multiple_of_Q": fiber_sizes,
            "kernel_size": fiber_sizes[0],
            "all_fibers_equal": true,
        },
        "ordered_pair_control": {
            "population": divisors.len() * divisors.len(),
            "arbitrary_class_homomorphism_pairs_checked": divisors.len() * divisors.len(),
            "arbitrary_class_homomorphism_failures": 0,
            "projected_zero_pairs": projected_zero_pairs,
            "exact_jacobian_zero_pairs": exact_zero_pairs,
            "false_projected_zero_pairs": false_projected_pairs,
            "exact_pairs_not_projected_zero": exact_pairs_not_projected_zero,
            "kernel_ambiguity_factor": kernel_ambiguity_factor,
            "projected_zero_pair_lifts_exactly_probability": {
                "numerator": exact_zero_pairs,
                "denominator": projected_zero_pairs,
                "reduced": "1/81",
            },
        },
        "negative_control": {
            "description": "nonzero kernel class paired with the identity",
            "projected_sum_is_zero": true,
            "jacobian_sum_is_zero": false,
            "kernel_witness": {
                "u": polynomial_json(&kernel_witness.u),
                "v": polynomial_json(&kernel_witness.v),
            },
        },
        "checks_passed": true,
    }))
}

fn n83_projector_replay() -> Result<Value, String> {
    let m = 83;
    let irreducible = n83_modulus();
    let order = BigUint::parse_bytes(SUBGROUP_ORDER.as_bytes(), 10)
        .ok_or_else(|| "invalid n=83 subgroup order".to_string())?;
    let point = BinaryPoint::Affine {
        x: F2mElement::from_hex(P_X, m),
        y: F2mElement::from_hex(P_Y, m),
    };
    let elliptic = target_curve(
        m,
        irreducible.clone(),
        F2mElement::zero(m),
        F2mElement::one(m),
        point.clone(),
        order.clone(),
        BigUint::from(4u32),
    );
    if !elliptic.is_on_curve(&point)
        || point == BinaryPoint::Infinity
        || scalar_mul(&elliptic, &point, &order) != BinaryPoint::Infinity
    {
        return Err("pinned n=83 point is not a nonzero order-r candidate".into());
    }

    let cover = OrdinaryBinaryCover::new(m, irreducible, F2mElement::zero(m), F2mElement::one(m))
        .map_err(|error| error.to_string())?;
    cover
        .verify_certificate()
        .map_err(|error| error.to_string())?;
    let pullback = cover
        .pullback_point_class(&point)
        .map_err(|error| error.to_string())?;
    let pushforward = cover
        .pushforward_arbitrary_jacobian_class(&pullback)
        .map_err(|error| error.to_string())?;
    let three_times_point = scalar_mul(&elliptic, &point, &BigUint::from(3u32));
    if pushforward != three_times_point {
        return Err("n=83 general norm did not replay phi_* phi^*=[3]".into());
    }

    let projector = cover
        .torsion_projector_certificate(&pullback, &order)
        .map_err(|error| error.to_string())?;
    if projector.normalized_projection != pullback
        || !projector.complementary_component.is_identity()
        || !projector.projection_is_idempotent
        || !projector.projected_component_is_in_image
        || !projector.complementary_component_is_in_pushforward_kernel
        || !projector.input_is_in_image
        || projector.input_is_in_pushforward_kernel
        || !projector.decomposition_holds
    {
        return Err("n=83 normalized projector replay failed".into());
    }

    Ok(json!({
        "field": "GF(2^83)",
        "elliptic_curve": "y^2+xy=x^3+1",
        "cover_curve": "v^2+u^2v=u^7+u",
        "map": "phi(u,v)=(u^3,uv+1)",
        "cover_degree": 3,
        "subgroup_order_r": order.to_string(),
        "gcd_cover_degree_r": 1,
        "pinned_point": point_json(&point),
        "r_times_pinned_point_is_zero": true,
        "general_norm_of_pullback_equals_three_times_point": true,
        "degree_inverse_mod_r": projector.degree_inverse_mod_order.to_string(),
        "normalized_projector_is_idempotent": projector.projection_is_idempotent,
        "pullback_is_fixed_by_normalized_projector": true,
        "complementary_component_is_zero": true,
        "checks_passed": true,
    }))
}

fn audit() -> Result<Value, String> {
    let fixed_f8 = fixed_f8_audit()?;
    let n83 = n83_projector_replay()?;
    Ok(json!({
        "schema": "crypto.n83-adaptive-image-router-audit/v1",
        "scope": "projector-only routing; no additional label, descent, or Prym-cancellation oracle",
        "status": "rejected_as_new_relation_mechanism",
        "fixed_f8_exhaustive_control": fixed_f8,
        "n83_projector_replay": n83,
        "derived_identities": {
            "kernel_coset_collapse": "K in ker(phi_*) implies p(D+K)=p(D)",
            "forward_relation_implication": "sum(c_i D_i)=0 implies sum(c_i phi_*(D_i))=0",
            "converse_boundary": "sum(c_i phi_*(D_i))=0 only implies sum(c_i D_i) lies in ker(phi_*)",
            "image_injectivity": "phi^* is injective on r-torsion because phi_*phi^*=[3] and gcd(3,r)=1",
        },
        "decision": {
            "projector_only_router_supplies_independent_rank": false,
            "reason": "projection retains only the elliptic norm label and discards the kernel/Prym residue; image-closed relations are the source elliptic relations",
            "timing_experiment_admitted": false,
            "speedup_claim": false,
        },
        "open_obligations": [
            "a broader growing-degree/adaptive design remains unclassified if it supplies an additional non-generic label/descent or Prym-cancellation oracle",
            "such a design must define its non-reduced or Riemann--Roch representation because reduced genus-three Mumford support has degree at most three",
            "any surviving design must cost relation search, projected independent rank, matrix and memory, blind individual logarithms, and finite crossover in the actual order r"
        ],
    }))
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let certificate = audit()?;
    let encoded = serde_json::to_string_pretty(&certificate).map_err(|error| error.to_string())?;
    if let Some(path) = args.output {
        fs::write(path, format!("{encoded}\n")).map_err(|error| error.to_string())?;
    } else {
        println!("{encoded}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_router_audit_passes() {
        let certificate = audit().expect("router audit must pass");
        assert_eq!(
            certificate["fixed_f8_exhaustive_control"]["ordered_pair_control"]
                ["kernel_ambiguity_factor"],
            81
        );
        assert_eq!(
            certificate["fixed_f8_exhaustive_control"]["ordered_pair_control"]
                ["arbitrary_class_homomorphism_failures"],
            0
        );
        assert_eq!(
            certificate["decision"]["projector_only_router_supplies_independent_rank"],
            false
        );
    }
}
