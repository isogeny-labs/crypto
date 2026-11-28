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

fn halving() -> IdentityStatement {
    let x = Expr::var("x");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let target = Expr::var("X");
    let slope = Expr::Add(vec![
        Expr::Mul(vec![Expr::Const(3), power(x.clone(), 2)]),
        a.clone(),
    ]);
    let curve = Expr::Add(vec![
        power(x.clone(), 3),
        Expr::Mul(vec![a.clone(), x.clone()]),
        b.clone(),
    ]);
    IdentityStatement {
        name:"Cleared doubling abscissa quartic for complete halving".into(),
        source:"TWO_PRIMARY_QUOTIENT_PROOF.md;Galbraithch9eq9.2;Lean TwoPrimaryQuotient.halving_abscissa_identity".into(),
        variables:vec!["x".into(),"a".into(),"b".into(),"X".into()],degree_bound:4,
        lhs:Expr::Add(vec![power(slope,2),Expr::Mul(vec![Expr::Const(-4),curve,Expr::Add(vec![Expr::Mul(vec![Expr::Const(2),x.clone()]),target.clone()])])]),
        rhs:Expr::Add(vec![power(x.clone(),4),Expr::Mul(vec![Expr::Const(-4),target.clone(),power(x.clone(),3)]),Expr::Mul(vec![Expr::Const(-2),a.clone(),power(x.clone(),2)]),Expr::Mul(vec![Expr::Const(-1),Expr::Add(vec![Expr::Mul(vec![Expr::Const(8),b.clone()]),Expr::Mul(vec![Expr::Const(4),a.clone(),target.clone()])]),x]),power(a,2),Expr::Mul(vec![Expr::Const(-4),b,target])]),
    }
}

fn main() {
    let outputs = std::env::args_os().skip(1).collect::<Vec<_>>();
    let cases: [(IdentityStatement, &[u8], &str, u64); 2] = [
        (
            duplication(),
            include_bytes!("../../lean/QuarticGluing.lean"),
            "research/chained_jacobian_transfers_20261110/lean/QuarticGluing.lean",
            0x384c,
        ),
        (
            halving(),
            include_bytes!("../../lean/TwoPrimaryQuotient.lean"),
            "research/chained_jacobian_transfers_20261110/lean/TwoPrimaryQuotient.lean",
            0x394c,
        ),
    ];
    for (i, (statement, source, path, seed)) in cases.into_iter().enumerate() {
        let cert=identity::issue(statement,seed,32,"Same producer issues;existing repository checker code plus domain-separated64extra points;scientific standing unsigned",Some(identity::ProofRef {kind:"lean4;pointer only;requires separate proof gate".into(),path:path.into(),sha256:hex::encode(sha256_native::sha256(source))})).expect("the issuer must reject any false identity");
        let verdict = identity::check_with_extra(&cert, Some((seed + 1, 64))).unwrap();
        assert!(matches!(verdict, identity::Verdict::Accept { .. }));
        if let Some(path) = outputs.get(i) {
            std::fs::write(
                path,
                format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()),
            )
            .unwrap();
        }
        println!("PASS {}:32issuer+64domain-separatedchecker points over fixed2^61-1;existing unchanged checker and SHA;not a signed or all-characteristics proof",cert.id);
    }
}

#[test]
fn mutated_identities_are_rejected() {
    for mut statement in [duplication(), halving()] {
        statement.rhs = Expr::Add(vec![statement.rhs, Expr::Const(1)]);
        assert!(matches!(
            identity::issue(statement, 0x384c, 32, "negative control", None),
            Err(identity::IssueError::False(_))
        ));
    }
}
