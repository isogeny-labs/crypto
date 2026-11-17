use super::*;
use std::collections::BTreeMap;
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/identity_certificate.rs"]
mod certificate;
type Poly = BTreeMap<(u32, u32), i128>;
fn clean(mut p: Poly) -> Poly {
    p.retain(|_, c| *c != 0);
    p
}
fn add(mut a: Poly, b: &Poly) -> Poly {
    for (&m, &c) in b {
        *a.entry(m).or_default() += c;
    }
    clean(a)
}
fn scale(a: &Poly, c: i128) -> Poly {
    clean(a.iter().map(|(&m, &v)| (m, c * v)).collect())
}
fn mul(a: &Poly, b: &Poly) -> Poly {
    let mut c = Poly::new();
    for (&(i, j), &x) in a {
        for (&(k, l), &y) in b {
            *c.entry((i + k, j + l)).or_default() += x * y;
        }
    }
    clean(c)
}
fn pow(a: &Poly, k: usize) -> Poly {
    (0..k).fold(Poly::from([((0, 0), 1)]), |r, _| mul(&r, a))
}
fn expr(a: &Poly) -> certificate::Expr {
    use certificate::Expr;
    Expr::sum(
        a.iter()
            .map(|(&(i, t), &c)| {
                let mut factors = vec![Expr::Const(c.try_into().unwrap())];
                factors.extend((0..i).map(|_| Expr::var("i")));
                factors.extend((0..t).map(|_| Expr::var("t")));
                Expr::product(factors)
            })
            .collect(),
    )
}
pub fn generate(dir: &str) {
    fs::create_dir_all(dir).unwrap();
    let dir = Path::new(dir);
    let a = Poly::from([((1, 0), 1), ((0, 1), 1), ((1, 1), 1)]);
    let b = Poly::from([((1, 0), -1), ((0, 1), 1), ((1, 1), -1)]);
    let r = Poly::from([((0, 0), 33), ((1, 0), 56)]);
    let s = Poly::from([((0, 0), 159), ((1, 0), 56)]);
    let rp = Poly::from([((0, 0), 33), ((1, 0), -56)]);
    let sp = Poly::from([((0, 0), 159), ((1, 0), -56)]);
    let num = add(
        add(
            add(
                mul(&r, &pow(&a, 6)),
                &mul(&s, &mul(&pow(&a, 4), &pow(&b, 2))),
            ),
            &mul(&sp, &mul(&pow(&a, 2), &pow(&b, 4))),
        ),
        &mul(&rp, &pow(&b, 6)),
    );
    let ideal = Poly::from([((2, 0), 1), ((0, 0), 1)]);
    let mut remainder = num.clone();
    let mut quotient = Poly::new();
    while let Some((&(i, t), &c)) = remainder.iter().rev().find(|&(&(i, _), _)| i >= 2) {
        let monomial = Poly::from([((i - 2, t), c)]);
        quotient = add(quotient, &monomial);
        remainder = add(remainder, &scale(&mul(&ideal, &monomial), -1));
    }
    assert!(remainder.keys().all(|&(i, t)| i == 0 && t <= 5));
    assert_eq!(num, add(remainder.clone(), &mul(&ideal, &quotient)));
    let statement=certificate::IdentityStatement{name:"Scholten one-infinity cover polynomial, with explicit quotient by i^2+1".into(),source:"Bernstein-Lange 2014 sections 2-3; exact integer expansion in native/certificates.rs; TRANSFER.md".into(),variables:vec!["i".into(),"t".into()],degree_bound:13,lhs:expr(&remainder),rhs:certificate::Expr::plus(expr(&num),certificate::Expr::minus(expr(&mul(&ideal,&quotient))))};
    let cert=certificate::issue(statement,202610181260,64,"Statement and quotient expanded over Z; checked with the repository's separate expression evaluator and extra checker seed",None).unwrap();
    let verdict = certificate::check_with_extra(&cert, Some((20261019999, 64))).unwrap();
    assert!(matches!(verdict, certificate::Verdict::Accept { .. }));
    write(
        dir.join("cover-identity-v1.json"),
        &serde_json::to_value(&cert).unwrap(),
    );
    let numerator: Vec<_> = (0..=5)
        .map(|t| *remainder.get(&(0, t)).unwrap_or(&0) as i64)
        .collect();
    write(
        dir.join("cover-identity-receipt.json"),
        &json!({"certificate_id":cert.id,"verdict":format!("{verdict:?}"),"exact_integer_remainder_verified":true,"quintic_numerator":numerator,"denominator":-384,"formal_proof":false,"meaning":"Exact polynomial identity with a quotient witness; not a Lean proof of the geometric homomorphism"}),
    );
    let mut orders = vec![];
    for p in [19, 23, 31, 43, 59, 67, 83, 103] {
        let s = Scholten::new(p);
        let n1 = 1
            + (0..p)
                .map(|x| {
                    let f = &s.j.f;
                    let y =
                        s.j.polynomial
                            .iter()
                            .rev()
                            .fold(0, |a, &b| f.add(f.mul(a, x), b));
                    match f.sqrt(y) {
                        None => 0,
                        Some(0) => 1,
                        Some(_) => 2,
                    }
                })
                .sum::<u64>();
        let n2 = 1
            + (0..p * p)
                .map(|x| {
                    let f = &s.j2.f;
                    let y =
                        s.j2.polynomial
                            .iter()
                            .rev()
                            .fold(0, |a, &b| f.add(f.mul(a, x), b));
                    match f.sqrt(y) {
                        None => 0,
                        Some(0) => 1,
                        Some(_) => 2,
                    }
                })
                .sum::<u64>();
        let j = (n1 * n1 + n2) / 2 - p;
        let e = exhaustive_order(&s.e);
        assert_eq!(j, e);
        orders.push(json!({"p":p,"C_Fp":n1,"C_Fp2":n2,"Jac_Fp":j,"E_Fp2":e,"equal":true}));
    }
    write(
        dir.join("small-independent-zeta-orders.json"),
        &json!(orders),
    );
}
