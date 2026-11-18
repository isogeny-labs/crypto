//! Integral identity; the Frobenius characteristic equation is a separate input.
use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let t = Expr::var("t");
    let characteristic = Expr::sum(vec![
        Expr::times(t.clone(), t.clone()),
        t.clone(),
        Expr::Const(2),
    ]);
    let statement = IdentityStatement {
        name: "Integral norm-four factorization modulo the E0 Frobenius characteristic polynomial".into(),
        source: "Elementary polynomial expansion; E0/F2 has trace -1; trace_zero_point_chains_20261109/README.md".into(),
        variables: vec!["t".into()], degree_bound: 2,
        lhs: Expr::plus(Expr::times(
            Expr::plus(t.clone(), Expr::Const(-1)),
            Expr::plus(Expr::minus(t), Expr::Const(-2)),
        ), characteristic),
        rhs: Expr::Const(4),
    };
    let cert = issue(
        statement,
        2026110903,
        32,
        "Local producer issue/replay; unsigned, no formal Frobenius or subgroup proof",
        None,
    )
    .unwrap();
    let verdict = check_with_extra(&cert, Some((0x7461756e6f726d, 32))).unwrap();
    assert!(matches!(verdict, Verdict::Accept { .. }));
    eprintln!(
        "{} {verdict:?}; evaluated integral factorization only",
        cert.id
    );
    println!("{}", serde_json::to_string(&cert).unwrap());
}
