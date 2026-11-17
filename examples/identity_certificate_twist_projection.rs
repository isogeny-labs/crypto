//! Certify the additive identity behind the quadratic-twist projection.
//!
//! The variables are formal abelian-group elements: F and its Frobenius image.
//! This certifies their integer coefficient identity, not the Frobenius
//! eigenspace property or an elliptic-curve group-law implementation.

use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};

fn main() {
    let f = Expr::var("F");
    let pi_f = Expr::var("pi_F");
    let statement = IdentityStatement {
        name: "quadratic-twist Frobenius projection numerator".into(),
        source: "research/twist_index_calculus_20261020/MIXED_REPORT.md".into(),
        variables: vec!["F".into(), "pi_F".into()],
        degree_bound: 1,
        lhs: Expr::times(Expr::Const(2), f.clone()),
        rhs: Expr::sum(vec![
            Expr::plus(f.clone(), pi_f.clone()),
            Expr::plus(f, Expr::minus(pi_f)),
        ]),
    };
    let cert = issue(
        statement,
        0x7477_6973_745f_6963,
        32,
        "issued by identity_certificate_twist_projection.rs; checked by the native certificate checker at 32 additional domain-separated points",
        None,
    )
    .expect("formal projection identity must hold");
    assert!(matches!(
        check_with_extra(&cert, Some((0x7665_7269_6679_5f31, 32))).expect("well-formed"),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
