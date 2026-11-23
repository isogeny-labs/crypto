use num_bigint::BigInt;
use std::collections::BTreeMap;
use torus_s4_tree::{identity::Expr, s3_statement};
type Polynomial = BTreeMap<[u8; 4], BigInt>;
fn plus(mut a: Polynomial, b: Polynomial) -> Polynomial {
    for (m, c) in b {
        let v = a.entry(m).or_default();
        *v += c;
    }
    a.retain(|_, c| *c != BigInt::from(0));
    a
}
fn times(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut r = Polynomial::new();
    for (m, c) in a {
        for (n, d) in &b {
            let mut k = m;
            for i in 0..4 {
                k[i] += n[i];
            }
            *r.entry(k).or_default() += &c * d;
        }
    }
    r.retain(|_, c| *c != BigInt::from(0));
    r
}
fn expand(e: &Expr) -> Polynomial {
    match e {
        Expr::Const(n) => {
            if *n == 0 {
                Polynomial::new()
            } else {
                BTreeMap::from([([0; 4], (*n).into())])
            }
        }
        Expr::Var(v) => {
            let i = ["A", "x", "y", "z"].iter().position(|n| n == v).unwrap();
            let mut m = [0; 4];
            m[i] = 1;
            BTreeMap::from([(m, 1.into())])
        }
        Expr::Neg(a) => expand(a).into_iter().map(|(m, c)| (m, -c)).collect(),
        Expr::Add(v) => v.iter().fold(Polynomial::new(), |a, b| plus(a, expand(b))),
        Expr::Mul(v) => v.iter().fold(BTreeMap::from([([0; 4], 1.into())]), |a, b| {
            times(a, expand(b))
        }),
    }
}
#[test]
fn exact_integer_coefficients_match_and_mutant_fails() {
    let s = s3_statement();
    assert_eq!(expand(&s.lhs), expand(&s.rhs));
    let bad = Expr::plus(s.rhs, Expr::Const(1));
    assert_ne!(expand(&s.lhs), expand(&bad));
}

#[test]
fn both_genus_two_elliptic_map_identities_are_exact() {
    for plus in [false, true] {
        let s = torus_s4_tree::elliptic_map_statement(plus);
        assert_eq!(expand(&s.lhs), expand(&s.rhs));
    }
}
