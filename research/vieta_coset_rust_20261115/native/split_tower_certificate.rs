//! Exact symbolic example for the hand rational-tower argument, not a solver
//! benchmark and not a formal/independent proof of the general theorem.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use vieta_coset_lab::vieta_coset::{Arithmetic, Curve, Limits, WordField};
type Poly = BTreeMap<[usize; 2], u64>;
fn term(i: usize, j: usize, c: u64) -> Poly {
    if c.is_multiple_of(17) {
        BTreeMap::new()
    } else {
        BTreeMap::from([([i, j], c % 17)])
    }
}
fn add(a: &Poly, b: &Poly) -> Poly {
    let mut r = a.clone();
    for (e, c) in b {
        let v = r.entry(*e).or_default();
        *v = (*v + c) % 17;
    }
    r.retain(|_, c| *c != 0);
    r
}
fn scale(a: &Poly, c: u64) -> Poly {
    a.iter()
        .filter_map(|(e, v)| {
            let n = v * c % 17;
            (n != 0).then_some((*e, n))
        })
        .collect()
}
fn sub(a: &Poly, b: &Poly) -> Poly {
    add(a, &scale(b, 16))
}
fn mul(a: &Poly, b: &Poly) -> Poly {
    let mut r = Poly::new();
    for (e, x) in a {
        for (f, y) in b {
            let v = r.entry([e[0] + f[0], e[1] + f[1]]).or_default();
            *v = (*v + x * y) % 17;
        }
    }
    r.retain(|_, c| *c != 0);
    r
}
fn inv(a: u64) -> u64 {
    (1..17)
        .find(|b| a * b % 17 == 1)
        .expect("nonzero derivative/denominator")
}
fn minus(a: u64, b: u64) -> u64 {
    (a % 17 + 17 - b % 17) % 17
}
fn map(x: u64) -> u64 {
    (x * x + 1) * inv(2 * x % 17) % 17
}
fn terms(a: &Poly) -> Vec<[usize; 3]> {
    a.iter().map(|(e, c)| [e[0], e[1], *c as usize]).collect()
}
fn check() -> Value {
    let (x, y, one) = (term(1, 0, 1), term(0, 1, 1), term(0, 0, 1));
    let f0 = sub(&add(&mul(&x, &x), &one), &scale(&mul(&x, &y), 2));
    let f1 = sub(&add(&mul(&y, &y), &one), &scale(&y, 8));
    let paths = [(2, 14), (9, 14), (10, 11), (12, 11)];
    let mut partition = Poly::new();
    let mut leaves = vec![];
    for (a, b) in paths {
        assert_eq!(map(a), b);
        assert_eq!(map(b), 4);
        let gamma0 = inv(minus(2 * a, 2 * b));
        let gamma1 = inv(minus(2 * b, 8));
        let e0 = scale(&sub(&x, &term(0, 0, minus(2 * b, a))), gamma0);
        let e1 = scale(&sub(&y, &term(0, 0, minus(8, b))), gamma1);
        let e = mul(&e0, &e1);
        partition = add(&partition, &e);
        let l0 = mul(&sub(&x, &term(0, 0, a)), &e);
        let r0 = add(
            &scale(&mul(&e1, &f0), gamma0),
            &scale(&mul(&scale(&x, 2), &f1), gamma0 * gamma1 % 17),
        );
        let l1 = mul(&sub(&y, &term(0, 0, b)), &e);
        let r1 = scale(&mul(&e0, &f1), gamma1);
        assert_eq!(l0, r0);
        assert_eq!(l1, r1);
        assert!(l0.keys().chain(l1.keys()).all(|v| v[0] + v[1] <= 3));
        leaves.push(json!({"path":[a,b],"gamma":[gamma0,gamma1],"projector_terms":terms(&e),"coordinate_certificates_equal":true,"degree_cap":3}));
    }
    assert_eq!(partition, one);
    // Homogeneous composition L^2(X)-4, checked against its four linear factors.
    let u = add(&mul(&x, &x), &one);
    let w = scale(&x, 2);
    let h = sub(&add(&mul(&u, &u), &mul(&w, &w)), &scale(&mul(&u, &w), 8));
    let mut product = one.clone();
    for (a, _) in paths {
        product = mul(&product, &sub(&x, &term(0, 0, a)));
    }
    assert_eq!(h, product);
    // The separate prime-order toy curve supplies a non-summation relation.
    let a = Arithmetic::new(WordField(17), Limits::default()).unwrap();
    let curve = Curve { a: 2, b: 2 };
    let mut base = vec![];
    let mut affine = 0;
    for u in 0..17 {
        for v in 0..17 {
            if a.on_curve(&curve, &Some((u, v))) {
                affine += 1;
                if paths.iter().any(|(x, _)| *x == u) {
                    base.push((u, v));
                }
            }
        }
    }
    assert_eq!(affine + 1, 19);
    assert!((2..19).all(|d| 19 % d != 0));
    let target = Some((6, 14));
    assert!(a.on_curve(&curve, &target));
    let relations = a.lookup_scalar(&curve, &base, &target).unwrap();
    assert_eq!(relations, vec![((9, 1), (10, 6)), ((10, 6), (9, 1))]);
    let q = add(&sub(&mul(&x, &x), &scale(&x, 2)), &term(0, 0, 5));
    let quotient = add(&add(&mul(&x, &x), &scale(&x, 3)), &term(0, 0, 7));
    assert_eq!(mul(&q, &quotient), h);
    json!({"schema":"split-rational-tower-example/v1","p":17,"map":{"numerator":[1,0,1],"denominator":[0,2]},"terminal":4,"leaves":leaves,"partition_of_unity":true,"membership_polynomial_terms":terms(&h),"prime_curve":{"a":2,"b":2,"group_order":19,"order_check":"exhaustive affine enumeration plus identity; trial primality","factor_base":base,"target":[6,14],"ordered_relations":relations,"vieta_slope":5,"membership_quadratic_divisibility":true},"scope":"Producer exact symbolic example [pcc:unverified]; not a formal general proof, elliptic-isogeny assertion, performance measurement or security result."})
}
fn main() {
    println!("{}", check());
}
