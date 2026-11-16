//! Reuse the repository's unchanged identity issuer/checker and SHA-256.
//! These evaluated records are not formal proofs over Z or signed claims.
#[path = "../../../../src/hash/sha256.rs"]
#[allow(clippy::chunks_exact_to_as_chunks)] // Preserve the frozen imported SHA source.
pub mod sha256_native;
mod hash {
    pub use crate::sha256_native as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
#[allow(dead_code)]
mod identity;

use identity::{Expr, IdentityStatement};

fn power(x: Expr, n: usize) -> Expr {
    Expr::Mul(vec![x; n])
}
fn duplication() -> IdentityStatement {
    let x = Expr::var("x");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let rhs_curve = Expr::Add(vec![
        power(x.clone(), 3),
        Expr::Mul(vec![a.clone(), x.clone()]),
        b.clone(),
    ]);
    let slope = Expr::Add(vec![
        Expr::Mul(vec![Expr::Const(3), power(x.clone(), 2)]),
        a.clone(),
    ]);
    IdentityStatement {
        name: "Cleared duplication ordinate numerator h4".into(),
        source: "QUARTIC_GLUING_PROOF.md;short-Weierstrass doubling and Galbraithch26 Miller recurrence;Lean QuarticGluing.duplication_ordinate_numerator".into(),
        variables: vec!["x".into(), "a".into(), "b".into()],
        degree_bound: 6,
        lhs: Expr::Add(vec![
            Expr::Mul(vec![Expr::Const(-8), power(rhs_curve.clone(), 2)]),
            Expr::Mul(vec![Expr::Const(12), x.clone(), rhs_curve, slope.clone()]),
            Expr::Neg(Box::new(power(slope, 3))),
        ]),
        rhs: Expr::Add(vec![
            power(x.clone(), 6),
            Expr::Mul(vec![Expr::Const(5), a.clone(), power(x.clone(), 4)]),
            Expr::Mul(vec![Expr::Const(20), b.clone(), power(x.clone(), 3)]),
            Expr::Mul(vec![Expr::Const(-5), power(a.clone(), 2), power(x.clone(), 2)]),
            Expr::Mul(vec![Expr::Const(-4), a.clone(), b.clone(), x]),
            Expr::Mul(vec![Expr::Const(-8), power(b, 2)]),
            Expr::Neg(Box::new(power(a, 3))),
        ]),
    }
}

fn main() {
    let cert = identity::issue(
        duplication(),
        0x384c,
        32,
        "Same producer issues;existing repository checker code plus domain-separated64extra points;scientific standing unsigned",
        Some(identity::ProofRef {
            kind: "lean4;pointer only;requires separate proof gate".into(),
            path: "research/chained_jacobian_transfers_20261110/lean/QuarticGluing.lean".into(),
            sha256: hex::encode(sha256_native::sha256(include_bytes!("../../lean/QuarticGluing.lean"))),
        }),
    )
    .expect("the issuer must reject any false identity");
    let verdict = identity::check_with_extra(&cert, Some((0x384d, 64))).unwrap();
    assert!(matches!(verdict, identity::Verdict::Accept { .. }));
    if let Some(path) = std::env::args_os().nth(1) {
        std::fs::write(
            path,
            format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()),
        )
        .unwrap();
    }
    println!("PASS {}:32issuer+64domain-separatedchecker points over fixed2^61-1;existing unchanged checker and SHA;not a signed or all-characteristics proof", cert.id);
}

#[test]
fn mutated_duplication_is_rejected() {
    let mut statement = duplication();
    statement.rhs = Expr::Add(vec![statement.rhs, Expr::Const(1)]);
    assert!(matches!(
        identity::issue(statement, 0x384c, 32, "negative control", None),
        Err(identity::IssueError::False(_))
    ));
}
