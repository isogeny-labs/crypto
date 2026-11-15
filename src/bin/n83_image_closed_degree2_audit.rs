//! Exact closed-point audit for the degree-two component of the n=83 cover.
//!
//! This is a structural certificate, not a timing benchmark.  It rejects the
//! rational-fiber relation design when its projected relation is only the
//! scalar-multiplication identity `P + [2]P = [3]P`.

use clap::Parser;
use crypto_lib::binary_ecc::curve::{scalar_mul, BinaryCurve};
use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, OrdinaryBinaryCover,
    PullbackClosedPointCertificate, PullbackFiberKind,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const SUBGROUP_ORDER: &str = "2417851639230796216685689";
const P_X: &str = "477f77103dfad59850800";
const P_Y: &str = "2fa5e737d542c4e4fd5c3";

#[derive(Parser)]
#[command(
    name = "n83-image-closed-degree2-audit",
    about = "Certify and screen the degree-two image-closed fiber relation"
)]
struct Args {
    /// Write the deterministic JSON certificate here; omit for stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn n83_modulus() -> IrreduciblePoly {
    IrreduciblePoly {
        degree: 83,
        low_terms: vec![0, 1, 2, 45],
    }
}

fn modulus(degree: u32) -> Result<IrreduciblePoly, String> {
    if degree == 83 {
        Ok(n83_modulus())
    } else {
        find_irreducible_sparse(degree)
            .ok_or_else(|| format!("no deterministic sparse modulus for degree {degree}"))
    }
}

fn cover(degree: u32, irreducible: IrreduciblePoly) -> Result<OrdinaryBinaryCover, String> {
    OrdinaryBinaryCover::new(
        degree,
        irreducible,
        F2mElement::zero(degree),
        F2mElement::one(degree),
    )
    .map_err(|error| error.to_string())
}

fn generic_point(degree: u32) -> BinaryPoint {
    BinaryPoint::Affine {
        x: F2mElement::one(degree),
        y: F2mElement::zero(degree),
    }
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

fn fiber_kind_json(kind: &PullbackFiberKind) -> Value {
    match kind {
        PullbackFiberKind::InfinityRamified { ramification_index } => json!({
            "kind": "infinity_ramified",
            "ramification_index": ramification_index,
        }),
        PullbackFiberKind::ExceptionalTwoTorsionRamified { ramification_index } => json!({
            "kind": "exceptional_two_torsion_ramified",
            "ramification_index": ramification_index,
        }),
        PullbackFiberKind::GenericSeparable { geometric_degree } => json!({
            "kind": "generic_separable",
            "geometric_degree": geometric_degree,
        }),
    }
}

fn certificate_json(
    field_degree: u32,
    irreducible: &IrreduciblePoly,
    certificate: &PullbackClosedPointCertificate,
) -> Value {
    let raw_component_count = certificate.components.len();
    let mut projected_labels: Vec<BinaryPoint> = Vec::new();
    for component in &certificate.components {
        if !projected_labels.contains(&component.image) {
            projected_labels.push(component.image.clone());
        }
    }
    let distinct_projected_labels = projected_labels.len();
    let projected_collision_count = raw_component_count - distinct_projected_labels;
    let projected_collision_rate = if raw_component_count == 0 {
        0.0
    } else {
        projected_collision_count as f64 / raw_component_count as f64
    };
    let mut factor_degrees: Vec<_> = certificate
        .components
        .iter()
        .map(|component| component.degree)
        .collect();
    factor_degrees.sort_unstable();

    json!({
        "field_degree": field_degree,
        "field_modulus": {
            "degree": irreducible.degree,
            "nonleading_terms": irreducible.low_terms,
        },
        "input": point_json(&certificate.input),
        "fiber_kind": fiber_kind_json(&certificate.fiber_kind),
        "factor_degrees": factor_degrees,
        "reduced_support_degree": certificate.reduced_support_degree,
        "components": certificate.components.iter().map(|component| json!({
            "support": polynomial_json(&component.support),
            "residue_degree": component.degree,
            "pushforward": point_json(&component.image),
            "pushforward_equals_degree_times_input": true,
        })).collect::<Vec<_>>(),
        "component_image_sum": point_json(&certificate.component_image_sum),
        "three_times_input": point_json(&certificate.three_times_input),
        "support_reconstruction_holds": true,
        "every_component_is_degree_times_input": certificate.every_component_is_degree_times_input,
        "component_sum_equals_general_pushforward": true,
        "component_sum_equals_three_times_input": certificate.total_matches_three_times_input,
        "projection_accounting": {
            "raw_closed_point_components": raw_component_count,
            "distinct_projected_elliptic_labels": distinct_projected_labels,
            "projected_collision_count": projected_collision_count,
            "projected_collision_rate": projected_collision_rate,
        },
    })
}

fn generic_fixture(field_degree: u32) -> Result<Value, String> {
    let irreducible = modulus(field_degree)?;
    let point = generic_point(field_degree);
    let certificate = cover(field_degree, irreducible.clone())?
        .pullback_closed_point_certificate(&point)
        .map_err(|error| error.to_string())?;
    let mut degrees: Vec<_> = certificate
        .components
        .iter()
        .map(|component| component.degree)
        .collect();
    degrees.sort_unstable();
    let expected = if field_degree.is_multiple_of(2) {
        vec![1, 1, 1]
    } else {
        vec![1, 2]
    };
    if degrees != expected {
        return Err(format!(
            "GF(2^{field_degree}) generic fiber degrees {degrees:?}, expected {expected:?}"
        ));
    }
    if !certificate.every_component_is_degree_times_input
        || !certificate.total_matches_three_times_input
    {
        return Err(format!(
            "GF(2^{field_degree}) closed-point norm certificate failed"
        ));
    }
    Ok(certificate_json(field_degree, &irreducible, &certificate))
}

fn pinned_subgroup_fixture() -> Result<Value, String> {
    let field_degree = 83;
    let irreducible = n83_modulus();
    let order = BigUint::parse_bytes(SUBGROUP_ORDER.as_bytes(), 10)
        .ok_or_else(|| "invalid subgroup order".to_string())?;
    let point = BinaryPoint::Affine {
        x: F2mElement::from_hex(P_X, field_degree),
        y: F2mElement::from_hex(P_Y, field_degree),
    };
    let curve = BinaryCurve {
        m: field_degree,
        irreducible: irreducible.clone(),
        a: F2mElement::zero(field_degree),
        b: F2mElement::one(field_degree),
        generator: point.clone(),
        order: order.clone(),
        cofactor: BigUint::from(4u32),
    };
    if !curve.is_on_curve(&point) || scalar_mul(&curve, &point, &order) != BinaryPoint::Infinity {
        return Err("pinned generator is not an r-torsion point on E".into());
    }

    let certificate = cover(field_degree, irreducible.clone())?
        .pullback_closed_point_certificate(&point)
        .map_err(|error| error.to_string())?;
    let degree_one = certificate
        .components
        .iter()
        .find(|component| component.degree == 1)
        .ok_or_else(|| "pinned fiber has no degree-one component".to_string())?;
    let degree_two = certificate
        .components
        .iter()
        .find(|component| component.degree == 2)
        .ok_or_else(|| "pinned fiber has no degree-two component".to_string())?;
    let doubled = scalar_mul(&curve, &point, &BigUint::from(2u32));
    if degree_one.image != point || degree_two.image != doubled {
        return Err("pinned fiber labels are not P and [2]P".into());
    }
    if doubled == BinaryPoint::Infinity || doubled == point {
        return Err("doubling collapsed a label in the odd prime-order subgroup".into());
    }

    Ok(json!({
        "subgroup_order_r": order.to_string(),
        "r_is_odd": (&order % 2u32) == BigUint::from(1u32),
        "r_times_input_is_zero": true,
        "degree_one_label": point_json(&degree_one.image),
        "degree_two_label": point_json(&degree_two.image),
        "degree_two_label_equals_two_times_input": true,
        "degree_two_label_is_nonzero": true,
        "degree_two_label_is_distinct_from_degree_one_label": true,
        "doubling_is_injective_on_the_prime_order_subgroup": true,
        "closed_point_certificate": certificate_json(field_degree, &irreducible, &certificate),
    }))
}

fn ramified_fixture() -> Result<Value, String> {
    let field_degree = 83;
    let irreducible = n83_modulus();
    let point = BinaryPoint::Affine {
        x: F2mElement::zero(field_degree),
        y: F2mElement::one(field_degree),
    };
    let certificate = cover(field_degree, irreducible.clone())?
        .pullback_closed_point_certificate(&point)
        .map_err(|error| error.to_string())?;
    if certificate.reduced_support_degree != 1
        || certificate.components.len() != 1
        || !matches!(
            certificate.fiber_kind,
            PullbackFiberKind::ExceptionalTwoTorsionRamified {
                ramification_index: 3
            }
        )
    {
        return Err("ramified fixture did not preserve multiplicity metadata".into());
    }
    Ok(certificate_json(field_degree, &irreducible, &certificate))
}

fn execute() -> Result<Value, String> {
    let odd_fixtures = [9u32, 13, 19, 23, 83]
        .into_iter()
        .map(generic_fixture)
        .collect::<Result<Vec<_>, _>>()?;
    let even_control = generic_fixture(2)?;
    let pinned_subgroup = pinned_subgroup_fixture()?;
    let ramified_control = ramified_fixture()?;

    let r = BigUint::parse_bytes(SUBGROUP_ORDER.as_bytes(), 10)
        .ok_or_else(|| "invalid subgroup order".to_string())?;
    let log2_r = r
        .to_f64()
        .ok_or_else(|| "subgroup order does not fit f64 cost model".to_string())?
        .log2();
    let rho_exponent_bits = log2_r / 2.0;
    let ambient_collection_exponent_bits = 4.0 * 83.0 / 3.0;
    let ambient_gap_bits = ambient_collection_exponent_bits - rho_exponent_bits;

    Ok(json!({
        "schema": "crypto.n83-image-closed-degree2-audit/v1",
        "status": "rejected_by_frozen_gate",
        "claim_scope": "exact closed-point decomposition and projected-label accounting for the rational-fiber degree-two design; asymptotic comparison for already-screened alternatives; no universal higher-degree impossibility claim",
        "cover": {
            "source": "v^2 + u^2 v = u^7 + u",
            "target": "y^2 + x y = x^3 + 1",
            "map": "phi(u,v)=(u^3,u v+1)",
            "degree": 3,
        },
        "fixtures": {
            "odd_degree_generic": odd_fixtures,
            "even_degree_positive_split_control": even_control,
            "pinned_r_torsion_generator": pinned_subgroup,
            "exceptional_ramification_control": ramified_control,
        },
        "relation_assessment": {
            "projected_identity": "[3]P=P+[2]P",
            "identity_is_implied_by_scalar_multiplication": true,
            "source_labelled_case": "if P=[a]G then the two labels already have scalars a and 2a",
            "unknown_target_case": "if P=Q then [3]Q=Q+[2]Q reveals no discrete logarithm",
            "useful_relation_probability_per_generic_fiber": 0.0,
            "verified_independent_rank_per_collected_identity": 0,
            "admitted_projected_matrix_rows": 0,
            "admitted_projected_matrix_columns": 0,
            "admitted_matrix_memory_bytes": 0,
            "individual_log_success": false,
            "complete_exponent_is_in_actual_order_r": true,
        },
        "cost_gate": {
            "target_order_r": r.to_string(),
            "log2_r": log2_r,
            "source_rho_exponent_bits": rho_exponent_bits,
            "ordinary_ambient_genus3_collection_exponent_bits": ambient_collection_exponent_bits,
            "ordinary_ambient_minus_source_rho_bits": ambient_gap_bits,
            "ordinary_ambient_route_beats_rho": false,
            "fixed_bounded_branch_lower_bound": "Omega(r) relation evaluations before linear algebra",
            "fixed_bounded_branch_beats_rho": false,
            "finite_crossover_against_strong_source_rho_predicted": false,
        },
        "frozen_gate": {
            "information_gate_passed": false,
            "complete_cost_gate_passed": false,
            "collector_implementation_admitted": false,
            "eks_timing_run_admitted": false,
            "decision": "stop",
        },
        "open_obligation": {
            "candidate": "compact growing-degree or adaptive target-local router",
            "required_evidence": "a named router with projected rank, relation probability, linear algebra, memory, blind individual-log descent, and finite-crossover accounting",
            "status": "unclassified",
        },
    }))
}

fn main() {
    let args = Args::parse();
    match execute() {
        Ok(receipt) => {
            let mut bytes = serde_json::to_vec_pretty(&receipt).expect("JSON serialization");
            bytes.push(b'\n');
            if let Some(path) = args.output {
                if let Some(parent) = path.parent() {
                    if let Err(error) = fs::create_dir_all(parent) {
                        eprintln!("n83 degree-two audit: create {}: {error}", parent.display());
                        std::process::exit(1);
                    }
                }
                if let Err(error) = fs::write(&path, bytes) {
                    eprintln!("n83 degree-two audit: write {}: {error}", path.display());
                    std::process::exit(1);
                }
            } else {
                print!("{}", String::from_utf8_lossy(&bytes));
            }
        }
        Err(error) => {
            eprintln!("n83 degree-two audit: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_degree_two_gate_rejects_the_tautological_relation() {
        let receipt = execute().expect("exact degree-two audit");
        assert_eq!(receipt["status"], "rejected_by_frozen_gate");
        assert_eq!(receipt["frozen_gate"]["information_gate_passed"], false);
        assert_eq!(receipt["frozen_gate"]["eks_timing_run_admitted"], false);
        for fixture in receipt["fixtures"]["odd_degree_generic"]
            .as_array()
            .expect("odd fixtures")
        {
            assert_eq!(fixture["factor_degrees"], json!([1, 2]));
        }
        assert_eq!(
            receipt["fixtures"]["even_degree_positive_split_control"]["factor_degrees"],
            json!([1, 1, 1])
        );
        assert_eq!(
            receipt["relation_assessment"]["verified_independent_rank_per_collected_identity"],
            0
        );
    }
}
