//! Supplemental serialization of already kernel-proved polynomial identities.
//! Imports the unchanged certificate checker, not the new gcd/map candidate.
use p256_genus_check::hash::sha256::sha256;
use p256_genus_check::identity_certificate::{
    check_with_extra, issue, Expr as E, IdentityStatement, ProofRef, Verdict,
};

fn square(x: E) -> E {
    E::product(vec![x.clone(), x])
}
fn difference(a: E, b: E) -> E {
    E::sum(vec![a, E::minus(b)])
}
fn norm(t: E, u: E, a: E, b: E) -> E {
    E::sum(vec![
        square(a.clone()),
        E::product(vec![t, a, b.clone()]),
        E::product(vec![u, square(b)]),
    ])
}
fn main() {
    let source = std::env::args()
        .nth(1)
        .expect("pass the pinned shared proof source path");
    let bytes = std::fs::read(source).expect("read proof source");
    let proof = ProofRef {
        kind:"lean4".into(),
        path:"https://github.com/aburan28/crypto-lean/blob/f74b0e8bd3815fdf2adae5b180431fc94d2be107/CryptoLean/Curve/QuadraticNormTransport.lean".into(),
        sha256:sha256(&bytes).iter().map(|v|format!("{v:02x}")).collect(),
    };
    let (t, u, a, b, l) = (
        E::var("t"),
        E::var("u"),
        E::var("a"),
        E::var("b"),
        E::var("ell"),
    );
    let first=IdentityStatement {
        name:"quadraticNorm reciprocal-root factorization".into(),
        source:"CryptoLean.quadraticNorm_reciprocal_factorization at f74b0e8; generic integer proof already kernel-gated".into(),
        variables:vec!["t".into(),"u".into(),"a".into(),"b".into(),"ell".into()],
        degree_bound:5,
        lhs:norm(t.clone(),u.clone(),a.clone(),b.clone()),
        rhs:E::sum(vec![
            E::product(vec![square(a.clone()),E::sum(vec![E::Const(1),E::minus(E::product(vec![t.clone(),l.clone()])),E::product(vec![u.clone(),square(l.clone())])])]),
            E::product(vec![E::sum(vec![b.clone(),E::product(vec![l.clone(),a.clone()])]),E::sum(vec![E::product(vec![u.clone(),b.clone()]),E::product(vec![difference(t,E::product(vec![u,l])),a.clone()])])]),
        ]),
    };
    let second = IdentityStatement {
        name: "Eisenstein norm as three squares".into(),
        source: "CryptoLean.eisenstein_norm_doubled at f74b0e8; kernel-gated over integers".into(),
        variables: vec!["a".into(), "b".into()],
        degree_bound: 2,
        lhs: E::product(vec![
            E::Const(2),
            norm(E::Const(-1), E::Const(1), a.clone(), b.clone()),
        ]),
        rhs: E::sum(vec![
            square(difference(a.clone(), b.clone())),
            square(a.clone()),
            square(b.clone()),
        ]),
    };
    let third = IdentityStatement {
        name: "Eisenstein rotation preserves norm".into(),
        source: "CryptoLean.eisenstein_norm_rotate at f74b0e8; kernel-gated over integers".into(),
        variables: vec!["a".into(), "b".into()],
        degree_bound: 2,
        lhs: norm(
            E::Const(-1),
            E::Const(1),
            E::minus(b.clone()),
            difference(a.clone(), b.clone()),
        ),
        rhs: norm(E::Const(-1), E::Const(1), a, b),
    };
    let mut certificates = vec![];
    for statement in [first, second, third] {
        let cert=issue(statement,2026111517,32,
            "Supplemental serialization/replay with the unchanged existing checker after the Lean kernel proof; same producer, no independent signer and no candidate qualification",Some(proof.clone())).expect("issue proved identity");
        assert!(matches!(
            check_with_extra(&cert, Some((2026111518, 64))).expect("replay"),
            Verdict::Accept { .. }
        ));
        certificates.push(cert);
    }
    println!("{}", serde_json::to_string_pretty(&certificates).unwrap());
}
