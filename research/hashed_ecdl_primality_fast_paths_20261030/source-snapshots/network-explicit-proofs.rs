//! Bounded public setup/construction control; no search or timing experiment.
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::primality::PrimeCertificate;
use hashed_ecdl_tensor::reduction::{decomposition_network, FactorBase, HashSeed};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use serde_json::{json, Value};

fn integer(input: &Value, key: &str) -> BigUint {
    input[key].as_str().unwrap().parse().unwrap()
}

fn point(input: &Value, key: &str) -> Point {
    Point::Affine {
        x: input[key][0].as_str().unwrap().parse().unwrap(),
        y: input[key][1].as_str().unwrap().parse().unwrap(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Value = serde_json::from_str(include_str!(
        "../../../../crates/hashed-ecdl-tensor/examples/eccp131-certified-hash.json"
    ))?;
    let proofs: Vec<PrimeCertificate> =
        serde_json::from_value(input["prime_certificates"].clone())?;
    let mut meter = Meter::new(Limits {
        steps: 2_000_000,
        vector: 2_000_000,
        bits: 4096,
    });
    let curve = Curve::with_cofactor_and_certificates(
        integer(&input, "p"),
        integer(&input, "a"),
        integer(&input, "b"),
        integer(&input, "r"),
        integer(&input, "cofactor"),
        point(&input, "generator"),
        &proofs,
        &mut meter,
    )?;
    let s = integer(&input, "hash_prime");
    let mut bases = Vec::new();
    for (alpha_divisor, beta_divisor) in [(3u32, 7u32), (5, 11)] {
        bases.push(FactorBase::new(
            &curve,
            s.clone(),
            BigUint::from(256u32),
            HashSeed {
                alpha: &s / alpha_divisor,
                beta: &s / beta_divisor,
            },
            &mut meter,
        )?);
    }
    let setup_steps = meter.used;
    let network = decomposition_network(&curve, &bases, &point(&input, "target"), &mut meter)?;
    assert!(!network.interface().is_empty());
    println!(
        "{}",
        json!({
            "status": "constructed", "field_bits": curve.modulus().bits(),
            "hash_bits": s.bits(), "slots": bases.len(), "threshold": 256,
            "edges": network.edge_count(), "vertices": network.vertex_count(),
            "maximum_degree": network.maximum_degree(), "interface_bits": network.interface().len(),
            "setup_charged_steps": setup_steps, "total_charged_steps": meter.used,
            "step_limit": meter.limits.steps, "vector_limit": meter.limits.vector,
            "unit": "uncalibrated development steps", "challenge_solved": false
        })
    );
    Ok(())
}
