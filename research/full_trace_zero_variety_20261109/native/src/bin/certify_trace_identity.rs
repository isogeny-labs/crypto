//! Certificate for the integral telescoping identity underlying trace descent.
use full_trace_zero_backend::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
fn main() {
    let power = |n| Expr::product((0..n).map(|_| Expr::var("t")).collect());
    let statement = IdentityStatement {
        name: "Degree-131 trace telescoping identity".into(),
        source: "Geometric sum identity; research/full_trace_zero_variety_20261109/README.md"
            .into(),
        variables: vec!["t".into()],
        degree_bound: 131,
        lhs: Expr::times(
            Expr::sum((0..131).map(power).collect()),
            Expr::plus(Expr::var("t"), Expr::Const(-1)),
        ),
        rhs: Expr::plus(power(131), Expr::Const(-1)),
    };
    let cert = issue(
        statement,
        20261109,
        32,
        "Local producer replay only; no independent checker or formal proof",
        None,
    )
    .unwrap();
    let verdict = check_with_extra(&cert, Some((0x7472616365, 32))).unwrap();
    assert!(matches!(verdict, Verdict::Accept { .. }));
    eprintln!(
        "{} {:?}; modulus 2^61-1, not a characteristic-two point-model certificate",
        cert.id, verdict
    );
    println!("{}", serde_json::to_string(&cert).unwrap());
}
