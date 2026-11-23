#[allow(dead_code)]
#[path = "../examples/moment_identity.rs"]
mod certificate;
use num_bigint::BigInt;
use torus_s4_tree::identity::{self, Expr};
fn evaluate(e: &Expr, n: i32, q: i32, t: i32) -> BigInt {
    match e {
        Expr::Const(c) => (*c).into(),
        Expr::Var(v) => match v.as_str() {
            "n" => n.into(),
            "q" => q.into(),
            "t" => t.into(),
            _ => panic!("unknown variable"),
        },
        Expr::Neg(x) => -evaluate(x, n, q, t),
        Expr::Add(xs) => xs.iter().map(|x| evaluate(x, n, q, t)).sum(),
        Expr::Mul(xs) => xs.iter().map(|x| evaluate(x, n, q, t)).product(),
    }
}
#[test]
fn native_certificate_checks_only_the_declared_algebra() {
    let s = certificate::statement();
    let cert = identity::issue(
        s.clone(),
        202611150121,
        32,
        "Development control only",
        None,
    )
    .unwrap();
    assert!(matches!(
        identity::check_with_extra(&cert, Some((202611150127, 32))).unwrap(),
        identity::Verdict::Accept { .. }
    ));
    let mut bad = s;
    bad.rhs = Expr::plus(bad.rhs, Expr::Const(1));
    assert!(identity::issue(bad, 202611150121, 32, "mutation", None).is_err());
}
#[test]
fn integer_unisolvent_grid_and_mutation() {
    let s = certificate::statement();
    // Per-variable degree bounds are n:4, q:2, t:1. This tests every
    // point of a corresponding integer interpolation grid, not probability.
    for n in 0..=4 {
        for q in 0..=2 {
            for t in 0..=1 {
                assert_eq!(evaluate(&s.lhs, n, q, t), evaluate(&s.rhs, n, q, t));
                assert_ne!(
                    evaluate(&s.lhs, n, q, t),
                    evaluate(&Expr::plus(s.rhs.clone(), Expr::Const(1)), n, q, t)
                );
            }
        }
    }
}
