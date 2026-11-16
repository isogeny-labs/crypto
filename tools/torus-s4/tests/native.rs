use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{fs, path::Path};
use torus_s4::{curve25519_prime, decimal, Case, Curve, Field, Poly, System};
fn fixture(name: &str) -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
    )
    .unwrap()
}
fn case(name: &str) -> Case {
    serde_json::from_str(&fixture(name)).unwrap()
}
fn actual_us() -> [BigUint; 3] {
    [
        "4527033373057041096665320307600593343270607561305067669696674248090335548871",
        "38407054660753056730010218173159084234842057283943310924705499020506171358627",
        "42980747477123847829319033766103824444397400740104332037350641329963907722263",
    ]
    .map(|x| decimal(x).unwrap())
}

#[test]
fn chebyshev_matches_linear_recurrence() {
    for p in [13u32, 29, 5309] {
        let f = Field::new(p.into()).unwrap();
        for x in 0..p.min(47) {
            let x = BigUint::from(x);
            let (mut a, mut b) = (BigUint::one(), x.clone());
            assert_eq!(f.chebyshev(0, &x), a);
            for n in 1..96 {
                assert_eq!(f.chebyshev(n, &x), b);
                let c = f.sub(&f.scale(&f.mul(&x, &b), 2), &a);
                a = b;
                b = c;
            }
        }
    }
}

// Parse the frozen historical serializer, WITHOUT executing its legacy commands.
// This checks every coefficient and the exact variable/equation ordering.
fn historical(text: &str, p: &BigUint) -> (Vec<String>, Vec<Poly>) {
    let vars = text
        .split(",(")
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();
    let body = text
        .split("ideal I=")
        .nth(1)
        .unwrap()
        .split(';')
        .next()
        .unwrap();
    let polys = body
        .split(',')
        .map(|line| {
            let mut q = Poly::small(p, 0);
            for term in line.trim().split('+') {
                let mut parts = term.split('*');
                let c = parts.next().unwrap().trim();
                let coefficient = if let Some(c) = c.strip_prefix('-') {
                    (p - decimal(c).unwrap() % p) % p
                } else {
                    decimal(c).unwrap() % p
                };
                let mut mono = vec![];
                for part in parts {
                    let (v, e) = part
                        .split_once('^')
                        .map(|(v, n)| (v, n.parse::<usize>().unwrap()))
                        .unwrap_or((part, 1));
                    let i = vars.iter().position(|x| x == v).unwrap();
                    mono.extend(std::iter::repeat_n(i, e));
                }
                mono.sort_unstable();
                let mut term = Poly::constant(p, coefficient);
                for i in mono {
                    term = &term * &Poly::var(p, i);
                }
                q = &q + &term;
            }
            q
        })
        .collect();
    (vars, polys)
}

#[test]
fn every_historical_coefficient_matches_native() {
    for (m, vars) in [(5, 23), (25, 38), (79, 47)] {
        let c = case(&format!("M{m}_planted.json"));
        let s = System::compile(&c).unwrap();
        let (v, e) = historical(&fixture(&format!("historical_M{m}.sing")), &s.p);
        assert_eq!(s.variables, v);
        assert_eq!(s.equations, e);
        assert_eq!(s.variables.len(), vars);
        assert_eq!(s.equations.len(), vars + 1);
        assert!(s.equations.iter().all(|e| e.degree() <= 2));
    }
}

#[test]
fn actual_field_witness_and_group_sum() {
    let c = case("actual_planted.json");
    assert_eq!(c.field().unwrap().p, curve25519_prime());
    let s = System::compile(&c).unwrap();
    assert_eq!(s.variables.len(), 395);
    assert_eq!(s.equations.len(), 396);
    let us = actual_us();
    let w = s.witness(&c, &us).unwrap();
    s.check(&w).unwrap();
    let curve = Curve::from_case(&c).unwrap();
    curve.verify_triple(&c, &us).unwrap();
    let prime = curve.times(&c.point().unwrap(), 8).unwrap().unwrap();
    assert_eq!(
        prime.0,
        decimal("24716044802460175362938714720543511367370248517208112821729288131297008555448")
            .unwrap()
    );
    assert_eq!(
        prime.1,
        decimal("28143606249318346099456356964299045610963165688037152668072856401530608941656")
            .unwrap()
    );
}

#[test]
fn mutations_are_rejected() {
    let c = case("actual_planted.json");
    let s = System::compile(&c).unwrap();
    let us = actual_us();
    let mut w = s.witness(&c, &us).unwrap();
    w[0] = (&w[0] + 1u32) % &s.p;
    assert!(s.check(&w).is_err());
    let mut bad = c.clone();
    bad.beta = serde_json::Number::from(1u32);
    assert!(Curve::from_case(&bad)
        .unwrap()
        .verify_triple(&bad, &us)
        .is_err());
    bad = c.clone();
    bad.target[1] = serde_json::Number::from(0u32);
    assert!(bad.validate().is_err());
    let mut altered = s.clone();
    let first = altered.equations[0].terms.values_mut().next().unwrap();
    *first = (&*first + 1u32) % &s.p;
    assert!(altered.check(&s.witness(&c, &us).unwrap()).is_err());
}

#[test]
fn m1_oracle_control_is_complete() {
    let c = case("M1_planted.json");
    let curve = Curve::from_case(&c).unwrap();
    let f = &curve.f;
    // C1(X)=17 gives (2u+A)^2 = 2A^2/18. Check both
    // roots, retain exactly the branch that lifts to the curve.
    let dd = f.mul(
        &f.scale(&f.square(&curve.a), 2),
        &f.inv(&18u32.into()).unwrap(),
    );
    let d = f.sqrt(&dd).unwrap().unwrap();
    let mut base = vec![];
    for d in [d.clone(), f.sub(&0u32.into(), &d)] {
        let u = f.mul(&f.sub(&d, &curve.a), &f.inv(&2u32.into()).unwrap());
        if f.sqrt(&curve.rhs(&u)).unwrap().is_some() {
            base.push(u);
        }
    }
    assert_eq!(base.len(), 1);
    let us = std::array::from_fn(|_| base[0].clone());
    let s = System::compile(&c).unwrap();
    assert_eq!(s.variables.len(), 17);
    s.check(&s.witness(&c, &us).unwrap()).unwrap();
    curve.verify_triple(&c, &us).unwrap();
    let no = case("M1_nonmember.json");
    assert!(curve.verify_triple(&no, &us).is_err());
    assert!(s.equations.iter().all(|e| e.degree() <= 2));
}

#[test]
fn exceptional_inputs_fail_explicitly() {
    let f = Field::new(curve25519_prime()).unwrap();
    assert!(f.inv(&BigUint::zero()).is_err());
    let mut c = case("actual_planted.json");
    c.m = serde_json::Number::from(0u32);
    assert!(System::compile(&c).is_err());
    c = case("actual_planted.json");
    c.target[0] = serde_json::Number::from_str(&(&f.p - 1u32).to_string()).unwrap();
    assert!(System::compile(&c).is_err());
}
use std::str::FromStr;
