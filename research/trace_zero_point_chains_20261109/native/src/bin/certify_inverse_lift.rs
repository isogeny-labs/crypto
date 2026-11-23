//! Integral norm expansion used in the known inverse/halving derivation.
use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let u = Expr::var("u");
    let s = Expr::var("s");
    let u2 = Expr::times(u.clone(), u.clone());
    let su = Expr::times(s.clone(), u);
    let statement = IdentityStatement {
        name: "Integral norm expansion underlying the binary inverse halving lift".into(),
        source: "Elementary expansion; known half-point validation algebra in US7257709, condition iii; see trace_zero_point_chains_20261109/README.md".into(),
        variables: vec!["u".into(), "s".into()],
        degree_bound: 4,
        lhs: Expr::times(
            Expr::sum(vec![u2.clone(), su.clone(), Expr::Const(1)]),
            Expr::sum(vec![u2.clone(), Expr::minus(su), Expr::Const(1)]),
        ),
        rhs: Expr::sum(vec![
            Expr::times(u2.clone(), u2.clone()),
            Expr::times(
                Expr::plus(Expr::Const(2), Expr::minus(Expr::times(s.clone(), s))),
                u2,
            ),
            Expr::Const(1),
        ]),
    };
    let certificate = issue(
        statement,
        2026110902,
        32,
        "Local producer issue/replay; unsigned, not a formal characteristic-two subgroup proof",
        None,
    )
    .unwrap();
    let verdict = check_with_extra(&certificate, Some((0x696e7665727365, 32))).unwrap();
    assert!(matches!(verdict, Verdict::Accept { .. }));
    eprintln!(
        "{} {verdict:?}; evaluated norm expansion only, not the subgroup criterion",
        certificate.id
    );
    println!("{}", serde_json::to_string(&certificate).unwrap());
}
