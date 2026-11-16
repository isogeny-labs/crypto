//! Exact n=83 Jacobian norm, projector, and Prym-factor audit.
//!
//! This is a structural certificate.  It does not run a full-size DLP and
//! makes no speedup claim.

use clap::Parser;
use crypto_lib::binary_ecc::curve::{scalar_mul, BinaryCurve};
use crypto_lib::binary_ecc::{
    BinaryCoverPoint, BinaryPoint, F2mElement, IrreduciblePoly, OrdinaryBinaryCover,
};
use crypto_lib::cryptanalysis::identity_certificate::{check, IdentityCertificate, Verdict};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::One;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const SUBGROUP_ORDER: &str = "2417851639230796216685689";
const P_X: &str = "477f77103dfad59850800";
const P_Y: &str = "2fa5e737d542c4e4fd5c3";

#[derive(Parser)]
#[command(
    name = "n83-jacobian-projector-audit",
    about = "Certify the general norm, image projector, and n=83 Prym factors"
)]
struct Args {
    /// Write the deterministic JSON certificate here; omit for stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn decimal(value: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.as_bytes(), 10).ok_or_else(|| format!("invalid integer {value}"))
}

fn f2_modulus(degree: u32) -> IrreduciblePoly {
    match degree {
        1 => IrreduciblePoly {
            degree,
            low_terms: vec![0],
        },
        2 => IrreduciblePoly {
            degree,
            low_terms: vec![0, 1],
        },
        3 => IrreduciblePoly {
            degree,
            low_terms: vec![0, 1],
        },
        _ => unreachable!("audit only uses GF(2), GF(4), and GF(8)"),
    }
}

fn small_point_counts(degree: u32) -> Result<(u64, u64), String> {
    let size = 1u32 << degree;
    let cover = OrdinaryBinaryCover::new(
        degree,
        f2_modulus(degree),
        F2mElement::zero(degree),
        F2mElement::one(degree),
    )
    .map_err(|error| error.to_string())?;
    let mut elliptic = 1u64;
    let mut genus_three = 1u64;
    for x in 0..size {
        for y in 0..size {
            let x = F2mElement::from_biguint(&BigUint::from(x), degree);
            let y = F2mElement::from_biguint(&BigUint::from(y), degree);
            if cover.target_is_on_curve(&BinaryPoint::Affine {
                x: x.clone(),
                y: y.clone(),
            }) {
                elliptic += 1;
            }
            if cover.source_is_on_curve(&BinaryCoverPoint::Affine { u: x, v: y }) {
                genus_three += 1;
            }
        }
    }
    Ok((elliptic, genus_three))
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

fn execute() -> Result<Value, String> {
    let identity: IdentityCertificate = serde_json::from_str(include_str!(
        "../../research/n83_jacobian_projector_20261031/prym-factorization.identity.json"
    ))
    .map_err(|error| error.to_string())?;
    if !matches!(
        check(&identity).map_err(|error| error.to_string())?,
        Verdict::Accept { .. }
    ) {
        return Err("Prym polynomial identity certificate did not replay".into());
    }

    let counts = [
        small_point_counts(1)?,
        small_point_counts(2)?,
        small_point_counts(3)?,
    ];
    if counts != [(4, 4), (8, 8), (4, 4)] {
        return Err(format!("unexpected GF(2^i) point counts: {counts:?}"));
    }

    let q = BigUint::one() << 83usize;
    let factor_trace = BigUint::one() << 42usize;
    if &factor_trace * &factor_trace != &q * 2u32 {
        return Err("Prym factor trace does not square to 2q".into());
    }
    let order_minus = &q + 1u32 - &factor_trace;
    let order_plus = &q + 1u32 + &factor_trace;
    let prym_order = &q * &q + 1u32;
    if &order_minus * &order_plus != prym_order {
        return Err("Prym elliptic-factor orders do not multiply to q^2+1".into());
    }

    let r = decimal(SUBGROUP_ORDER)?;
    let gcd_minus = r.gcd(&order_minus);
    let gcd_plus = r.gcd(&order_plus);
    let gcd_prym = r.gcd(&prym_order);
    if gcd_minus != BigUint::one() || gcd_plus != BigUint::one() || gcd_prym != BigUint::one() {
        return Err("target prime unexpectedly divides a Prym order factor".into());
    }

    let group_order = &r * 4u32;
    let source_trace = BigInt::from_biguint(Sign::Plus, &q + 1u32)
        - BigInt::from_biguint(Sign::Plus, group_order.clone());
    let expected_source_trace = BigInt::from(-6_151_469_093_347i64);
    if source_trace != expected_source_trace {
        return Err("pinned source trace does not match its exact group order".into());
    }
    let positive_factor_trace = BigInt::from_biguint(Sign::Plus, factor_trace.clone());
    let negative_factor_trace = -positive_factor_trace.clone();
    if source_trace == positive_factor_trace || source_trace == negative_factor_trace {
        return Err("source elliptic curve unexpectedly shares a Prym trace".into());
    }

    let irreducible = IrreduciblePoly {
        degree: 83,
        low_terms: vec![0, 1, 2, 45],
    };
    let p = BinaryPoint::Affine {
        x: F2mElement::from_hex(P_X, 83),
        y: F2mElement::from_hex(P_Y, 83),
    };
    let source = BinaryCurve {
        m: 83,
        irreducible: irreducible.clone(),
        a: F2mElement::zero(83),
        b: F2mElement::one(83),
        generator: p.clone(),
        order: r.clone(),
        cofactor: BigUint::from(4u32),
    };
    if scalar_mul(&source, &p, &r) != BinaryPoint::Infinity {
        return Err("pinned source point is not killed by r".into());
    }
    let cover =
        OrdinaryBinaryCover::new(83, irreducible, F2mElement::zero(83), F2mElement::one(83))
            .map_err(|error| error.to_string())?;
    let image = cover
        .pullback_point_class(&p)
        .map_err(|error| error.to_string())?;
    let expected_pushforward = scalar_mul(&source, &p, &BigUint::from(3u32));
    let actual_pushforward = cover
        .pushforward_arbitrary_jacobian_class(&image)
        .map_err(|error| error.to_string())?;
    if actual_pushforward != expected_pushforward {
        return Err("general norm failed phi_* phi^*=[3] on the pinned generator".into());
    }
    let endomorphism = cover
        .image_endomorphism_certificate(&image)
        .map_err(|error| error.to_string())?;
    let projector = cover
        .torsion_projector_certificate(&image, &r)
        .map_err(|error| error.to_string())?;
    if !endomorphism.quadratic_identity_holds
        || !projector.projection_is_idempotent
        || !projector.projected_component_is_in_image
        || !projector.complementary_component_is_in_pushforward_kernel
        || !projector.input_is_in_image
        || projector.input_is_in_pushforward_kernel
        || !projector.complementary_component.is_identity()
        || !projector.decomposition_holds
    {
        return Err("n=83 image-projector certificate failed".into());
    }

    Ok(json!({
        "schema": "crypto.n83-jacobian-projector-audit/v1",
        "status": "pass",
        "claim_scope": "exact_general_norm_and_projector_on_the_pinned_r_image; exact_Prym_order_factor_audit; no_DLP_speedup_claim",
        "polynomial_identity_certificate": {
            "id": identity.id,
            "native_replay": "accept"
        },
        "small_field_point_counts": [
            {"field": "GF(2)", "elliptic": counts[0].0, "genus_three": counts[0].1},
            {"field": "GF(4)", "elliptic": counts[1].0, "genus_three": counts[1].1},
            {"field": "GF(8)", "elliptic": counts[2].0, "genus_three": counts[2].1}
        ],
        "frobenius": {
            "base_elliptic_polynomial": "X^2 + X + 2",
            "base_genus_three_polynomial": "(X^2 + X + 2)(X^4 + 4)",
            "base_prym_polynomial": "X^4 + 4",
            "degree_83_prym_polynomial": "X^4 + q^2",
            "degree_83_factorization": "(X^2 - 2^42 X + q)(X^2 + 2^42 X + q)",
            "q": q.to_string(),
            "source_trace": source_trace.to_string(),
            "prym_elliptic_factor_traces": [
                factor_trace.to_string(),
                negative_factor_trace.to_string()
            ],
            "source_shares_prym_elliptic_isogeny_factor": false,
            "isogeny_test": "distinct elliptic Frobenius traces over the same finite field"
        },
        "orders": {
            "target_prime_r": r.to_string(),
            "prym_factor_trace_plus_order": order_minus.to_string(),
            "prym_factor_trace_minus_order": order_plus.to_string(),
            "prym_order": prym_order.to_string(),
            "gcd_r_factor_trace_plus": gcd_minus.to_string(),
            "gcd_r_factor_trace_minus": gcd_plus.to_string(),
            "gcd_r_prym": gcd_prym.to_string(),
            "target_prime_occurs_in_prym": false
        },
        "general_norm": {
            "input": "phi^*([P]-[O])",
            "actual_pushforward": point_json(&actual_pushforward),
            "expected": "[3]P",
            "phi_push_phi_pull_equals_three": true,
            "nonsplit_support_supported": true
        },
        "endomorphism": {
            "definition": "e=phi^* phi_*",
            "quadratic_identity": "e^2=[3]e",
            "quadratic_identity_holds": endomorphism.quadratic_identity_holds
        },
        "r_torsion_projector": {
            "definition": "p=[3^-1 mod r]e",
            "inverse_three_mod_r": projector.degree_inverse_mod_order.to_string(),
            "idempotent": projector.projection_is_idempotent,
            "input_is_in_image": projector.input_is_in_image,
            "input_is_in_kernel": projector.input_is_in_pushforward_kernel,
            "complement_is_zero_for_pinned_image_input": projector.complementary_component.is_identity(),
            "complement_is_in_pushforward_kernel": projector.complementary_component_is_in_pushforward_kernel,
            "decomposition_holds": projector.decomposition_holds
        },
        "algorithmic_verdict": {
            "standard_full_genus3_index_calculus": "screened_out",
            "prym_shared_r_route": "rejected",
            "reason": "gcd(r,#Prym(GF(2^83)))=1",
            "remaining_route": "projected_or_image_closed_relations_only",
            "required_cost_gate": "must beat r^1/2 and predict a finite crossover against source rho"
        }
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
                        eprintln!("n83 projector audit: create {}: {error}", parent.display());
                        std::process::exit(1);
                    }
                }
                if let Err(error) = fs::write(&path, bytes) {
                    eprintln!("n83 projector audit: write {}: {error}", path.display());
                    std::process::exit(1);
                }
            } else {
                print!("{}", String::from_utf8_lossy(&bytes));
            }
        }
        Err(error) => {
            eprintln!("n83 projector audit: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_projector_and_prym_audit_replays() {
        let receipt = execute().expect("exact n=83 audit");
        assert_eq!(receipt["status"], "pass");
        assert_eq!(receipt["orders"]["gcd_r_prym"], "1");
        assert_eq!(receipt["r_torsion_projector"]["idempotent"], true);
    }
}
