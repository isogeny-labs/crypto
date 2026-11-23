//! Issue/replay v1 certificates for the denominator-cleared integer identities.
//! Reduction modulo 101 is a separate exact coefficient check in native tests.
use crypto_lib::cryptanalysis::identity_certificate::{
    check, issue, Expr as E, IdentityStatement, ProofRef, Verdict,
};

fn term(c: i64, n: usize) -> E {
    let mut v = vec![E::Const(c)];
    v.extend((0..n).map(|_| E::var("x")));
    E::product(v)
}
fn poly(c: &[i64]) -> E {
    E::sum(
        c.iter()
            .enumerate()
            .filter(|(_, v)| **v != 0)
            .map(|(i, v)| term(*v, i))
            .collect(),
    )
}
fn statements() -> Vec<IdentityStatement> {
    let src =
        "research/chained_jacobian_transfers_20261110/REPORT_20261111.md; Kunzweiler 2024 Prop.7–8";
    vec![
        IdentityStatement {
            name: "source quadratic splitting over Z before reduction mod 101".into(),
            source: src.into(),
            variables: vec!["x".into()],
            degree_bound: 6,
            lhs: E::product(vec![
                poly(&[1, 0, 1]),
                poly(&[-1, 10, 1]),
                poly(&[-1, -10, 1]),
            ]),
            rhs: poly(&[1, 0, -101, 0, -101, 0, 1]),
        },
        IdentityStatement {
            name: "raw Richelot target numerator before division by 40^3".into(),
            source: src.into(),
            variables: vec!["x".into()],
            degree_bound: 6,
            lhs: E::product(vec![
                poly(&[-20, 0, -20]),
                poly(&[-10, 4, 10]),
                poly(&[-10, -4, 10]),
            ]),
            rhs: poly(&[-2000, 0, 2320, 0, 2320, 0, -2000]),
        },
    ]
}
fn certificates() -> Vec<crypto_lib::cryptanalysis::identity_certificate::IdentityCertificate> {
    let proof_path = "research/chained_jacobian_transfers_20261110/lean/PolynomialIdentities.lean";
    let proof = ProofRef {
        kind: "lean4".into(),
        path: proof_path.into(),
        sha256: hex::encode(crypto_lib::hash::sha256::sha256(include_bytes!(
            "../research/chained_jacobian_transfers_20261110/lean/PolynomialIdentities.lean"
        ))),
    };
    statements()
        .into_iter()
        .map(|s| {
            let cert = issue(
                s,
                20261111,
                32,
                "producer issue/replay only; no independent signed verification",
                Some(proof.clone()),
            )
            .unwrap();
            assert!(matches!(check(&cert).unwrap(), Verdict::Accept { .. }));
            cert
        })
        .collect()
}
fn main() {
    println!("{}", serde_json::to_string_pretty(&certificates()).unwrap());
}
#[test]
fn cleared_integer_identities_replay() {
    assert_eq!(certificates().len(), 2);
}
