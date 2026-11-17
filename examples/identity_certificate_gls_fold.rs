//! Certify the polynomial identity behind GLS row covariance.
//! Reducing lambda^2 + 1 to zero gives the actual subgroup identity.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};

fn main() {
    let a = Expr::var("a");
    let b = Expr::var("b");
    let l = Expr::var("lambda");
    let correction = Expr::times(
        Expr::plus(Expr::times(l.clone(), l.clone()), Expr::Const(1)),
        b.clone(),
    );
    let statement = IdentityStatement {
        name: "GLS orbit row covariance with characteristic-polynomial remainder".into(),
        source: "research/twist_index_calculus_20261020/GLS_SYMMETRY_REPORT.md".into(),
        variables: vec!["a".into(), "b".into(), "lambda".into()],
        degree_bound: 3,
        lhs: Expr::sum(vec![
            Expr::minus(b.clone()),
            Expr::times(l.clone(), a.clone()),
            correction,
        ]),
        rhs: Expr::times(l.clone(), Expr::plus(a, Expr::times(l, b))),
    };
    let cert = issue(
        statement,
        0x676c_735f_666f_6c64,
        32,
        "native identity_certificate_gls_fold issuer; native checker with 32 independent extra points; subgroup lambda^2=-1 checked separately by the GLS verifier",
        None,
    )
    .expect("GLS row identity must hold");
    assert!(matches!(
        check_with_extra(&cert, Some((0x676c_735f_7665_7269, 32))).expect("well-formed"),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
