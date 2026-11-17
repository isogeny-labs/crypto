//! Evaluated denominator-two subset/barycentric identity; see LINEAR-PROTOCOL.md.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr as E, IdentityStatement, Verdict,
};
fn x(i: usize) -> E {
    E::var(&format!("x{i}"))
}
fn w(i: usize) -> E {
    E::var(&format!("w{i}"))
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn sq(a: E) -> E {
    E::times(a.clone(), a)
}
fn poly(m: usize, skip: usize) -> E {
    E::product(
        (0..m)
            .filter(|&i| skip & (1 << i) == 0)
            .map(|i| sub(E::var("z"), x(i)))
            .collect(),
    )
}
fn main() {
    let mut certs = Vec::new();
    for g in 2..=4 {
        let m = g + 4;
        let s0 = E::sum((0..m).map(w).collect());
        let s1 = E::sum((0..m).map(|i| E::times(w(i), x(i))).collect());
        let h = E::sum((0..m).map(|i| E::times(w(i), poly(m, 1 << i))).collect());
        let b = sub(E::times(s0.clone(), E::var("z")), s1);
        let mut lhs = Vec::new();
        for i in 0..m {
            for j in 0..i {
                lhs.push(E::product(vec![
                    w(i),
                    w(j),
                    sq(sub(x(i), x(j))),
                    poly(m, (1 << i) | (1 << j)),
                ]));
            }
        }
        let statement=IdentityStatement{name:format!("d2 barycentric numerator identity genus{g}"),source:"D'Andrea/Krick/Szanto arXiv:1211.6895, subset specialization; LINEAR-PROTOCOL.md. Arbitrary signed weights; denominators absorbed into wi.".into(),variables:vec!["z".to_string()].into_iter().chain((0..m).map(|i|format!("x{i}"))).chain((0..m).map(|i|format!("w{i}"))).collect(),degree_bound:(m+2)as u32,lhs:E::sum(lhs),rhs:sub(E::times(b,h),E::times(sq(s0),poly(m,0)))};
        let cert=issue(statement,2026111508,32,"Producer-issued/rechecked at16additional points; no formal proof pointer or independent issuer.",None).unwrap();
        assert!(matches!(
            check_with_extra(&cert, Some((2026111509, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
        certs.push(cert);
    }
    println!("{}", serde_json::to_string(&certs).unwrap());
}
