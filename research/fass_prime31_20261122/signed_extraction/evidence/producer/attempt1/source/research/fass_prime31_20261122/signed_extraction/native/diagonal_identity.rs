//! Fixed finite evaluation of the S3 diagonal coefficient identity.
//!
//! This uses the unchanged repository identity-certificate implementation.
//! Public deterministic evaluation points are a development check, not a
//! symbolic proof or an adversarial false-acceptance probability estimate.

#[path = "../../../../src/hash/sha256.rs"]
#[allow(dead_code)]
mod source_sha256;
mod hash {
    pub mod sha256 {
        pub use crate::source_sha256::sha256;
    }
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
#[allow(dead_code)]
mod identity_certificate;

use identity_certificate::{Expr, IdentityCertificate, IdentityStatement, Verdict};
use serde_json::json;
use std::{env, fs};

const PRODUCER_SEED: u64 = 0x5349_474e_4449_4147;
const CHECKER_SEED: u64 = 0x5349_474e_4348_454b;
const POINTS: u32 = 32;
const INDEPENDENCE: &str = "Issued by the signed-extraction continuation; separate-role checking is recorded in its own receipt. Fixed public deterministic points; no formal proof.";

fn variable(name: &str) -> Expr {
    Expr::var(name)
}
fn square(value: Expr) -> Expr {
    Expr::times(value.clone(), value)
}
fn scaled(coefficient: i64, value: Expr) -> Expr {
    Expr::times(Expr::Const(coefficient), value)
}
fn subtract(left: Expr, right: Expr) -> Expr {
    Expr::plus(left, Expr::minus(right))
}

/// Independent expressions for the classical S3 specialization and its
/// cubic/derivative decomposition. `factor=4` is the declared identity;
/// `factor=5` is the fixed negative coefficient control.
fn statement(factor: i64) -> IdentityStatement {
    let a = variable("a");
    let z = variable("z");
    let curve_a = variable("A");
    let curve_b = variable("B");
    let az = Expr::times(a.clone(), z.clone());
    let sum = Expr::plus(a.clone(), z.clone());
    let difference = subtract(a.clone(), z.clone());
    let lhs = Expr::sum(vec![
        Expr::times(square(difference), square(a.clone())),
        scaled(
            -2,
            Expr::times(
                Expr::plus(
                    Expr::times(sum.clone(), Expr::plus(az.clone(), curve_a.clone())),
                    scaled(2, curve_b.clone()),
                ),
                a.clone(),
            ),
        ),
        square(subtract(az, curve_a.clone())),
        scaled(-4, Expr::times(curve_b.clone(), sum)),
    ]);
    let cubic = Expr::sum(vec![
        Expr::product(vec![a.clone(), a.clone(), a.clone()]),
        Expr::times(curve_a.clone(), a.clone()),
        curve_b,
    ]);
    let derivative = Expr::plus(scaled(3, square(a.clone())), curve_a);
    let rhs = Expr::plus(
        square(derivative),
        scaled(-factor, Expr::times(cubic, Expr::plus(z, scaled(2, a)))),
    );
    IdentityStatement {
        name: "S3 diagonal cubic and derivative identity".to_owned(),
        source: "research/fass_prime31_20261122/signed_extraction/THEORY.md section 5; classical short-Weierstrass S3 specialization".to_owned(),
        variables: ["A", "B", "a", "z"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs,
        rhs,
    }
}

fn check_file(path: &str) {
    let bytes = fs::read(path).expect("read retained identity certificate");
    assert!(bytes.len() <= 65536, "identity certificate byte cap");
    let certificate: IdentityCertificate =
        serde_json::from_slice(&bytes).expect("identity certificate JSON");
    let original_value: serde_json::Value =
        serde_json::from_slice(&bytes).expect("identity certificate JSON value");
    assert_eq!(
        serde_json::to_value(&certificate).unwrap(),
        original_value,
        "unknown or omitted identity-certificate fields"
    );
    assert_eq!(
        certificate.statement,
        statement(4),
        "fixed statement binding"
    );
    assert_eq!(certificate.points, POINTS, "fixed point count");
    assert_eq!(certificate.seed, PRODUCER_SEED, "fixed producer seed");
    assert_eq!(
        certificate.independence, INDEPENDENCE,
        "fixed provenance label"
    );
    assert!(
        certificate.proof.is_none(),
        "no formal proof pointer is admitted"
    );
    let verdict =
        identity_certificate::check_with_extra(&certificate, Some((CHECKER_SEED, POINTS)))
            .expect("certificate structural validation");
    assert!(
        matches!(
            verdict,
            Verdict::Accept {
                points: 32,
                extra_points: 32,
                ..
            }
        ),
        "fixed identity and extra evaluation points must agree"
    );

    // Rebind the mutated statement ID, so this control exercises arithmetic
    // disagreement rather than relying only on a stale identifier.
    let mut changed_coefficient = certificate.clone();
    changed_coefficient.statement = statement(5);
    changed_coefficient.id = identity_certificate::statement_id(&changed_coefficient.statement);
    assert!(
        matches!(
            identity_certificate::check_with_extra(
                &changed_coefficient,
                Some((CHECKER_SEED, POINTS))
            ),
            Ok(Verdict::Reject(_))
        ),
        "changed coefficient must fail an actual evaluation"
    );

    let mut changed_digest = certificate.clone();
    let first = if changed_digest.value_digest.starts_with('0') {
        "1"
    } else {
        "0"
    };
    changed_digest.value_digest.replace_range(0..1, first);
    assert!(
        matches!(
            identity_certificate::check_with_extra(&changed_digest, Some((CHECKER_SEED, POINTS))),
            Ok(Verdict::DigestMismatch { .. })
        ),
        "changed value digest must fail the digest check"
    );
    println!(
        "{}",
        json!({
            "schema": "fass-signed-diagonal-check/v1",
            "identity_id": certificate.id,
            "certificate_sha256": hex::encode(source_sha256::sha256(&bytes)),
            "certificate_bytes": bytes.len(),
            "modulus": identity_certificate::MODULUS,
            "certificate_points": POINTS,
            "extra_points": POINTS,
            "extra_seed": CHECKER_SEED,
            "statement_binding": true,
            "coefficient_mutation_rejected": true,
            "digest_mutation_rejected": true,
            "formal_proof": false,
            "adversarial_false_acceptance_probability_claim": false,
            "standing": "unsigned fixed finite evaluation check"
        })
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => {
            let certificate = identity_certificate::issue(
                statement(4),
                PRODUCER_SEED,
                POINTS,
                INDEPENDENCE,
                None,
            )
            .expect("fixed diagonal identity evaluation");
            println!("{}", serde_json::to_string(&certificate).unwrap());
        }
        [_, flag, path] if flag == "--check" => check_file(path),
        _ => panic!("usage: fass-signed-diagonal-identity [--check CERTIFICATE_JSON]"),
    }
}
