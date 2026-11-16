//! Evaluated scalar norm-factor identity; not a proof of divisor bookkeeping.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr as E, IdentityStatement, Verdict,
};
fn sq(x: E) -> E {
    E::times(x.clone(), x)
}
fn main() {
    let (a, b, f, h) = (E::var("a"), E::var("b"), E::var("f"), E::var("h"));
    let statement=IdentityStatement{
        name:"vertical common-factor norm removal".into(),
        source:"Elementary norm factorization; PRIMITIVE-PROTOCOL.md. Evaluated scalar polynomial identity only; divisor/anchor bookkeeping is checked separately.".into(),
        variables:vec!["a".into(),"b".into(),"f".into(),"h".into()],degree_bound:5,
        lhs:E::plus(sq(E::times(h.clone(),a.clone())),E::minus(E::times(f.clone(),sq(E::times(h.clone(),b.clone()))))),
        rhs:E::times(sq(h),E::plus(sq(a),E::minus(E::times(f,sq(b))))),
    };
    let cert=issue(statement,2026111506,32,"Producer-issued and rechecked at16additional derived points; not independent authorship or a formal proof.",None).unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((2026111507, 16))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
