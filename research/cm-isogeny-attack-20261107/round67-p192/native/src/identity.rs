use serde_json::{json, Value};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Polynomial(BTreeMap<[u8; 7], i128>);
impl Polynomial {
    fn c(c: i128) -> Self {
        Self(if c == 0 {
            BTreeMap::new()
        } else {
            BTreeMap::from([([0; 7], c)])
        })
    }
    fn x(i: usize) -> Self {
        let mut e = [0; 7];
        e[i] = 1;
        Self(BTreeMap::from([(e, 1)]))
    }
    fn add(&self, b: &Self) -> Self {
        let mut m = self.0.clone();
        for (e, c) in &b.0 {
            let v = m.entry(*e).or_default();
            *v = v.checked_add(*c).unwrap()
        }
        m.retain(|_, c| *c != 0);
        Self(m)
    }
    fn scale(&self, c: i128) -> Self {
        let mut m = self.0.clone();
        for v in m.values_mut() {
            *v = v.checked_mul(c).unwrap()
        }
        m.retain(|_, c| *c != 0);
        Self(m)
    }
    fn sub(&self, b: &Self) -> Self {
        self.add(&b.scale(-1))
    }
    fn mul(&self, b: &Self) -> Self {
        let mut m = BTreeMap::<[u8; 7], i128>::new();
        for (e, c) in &self.0 {
            for (f, d) in &b.0 {
                let a = std::array::from_fn(|i| e[i].checked_add(f[i]).unwrap());
                let v = m.entry(a).or_default();
                *v = v.checked_add(c.checked_mul(*d).unwrap()).unwrap()
            }
        }
        m.retain(|_, c| *c != 0);
        Self(m)
    }
    fn sq(&self) -> Self {
        self.mul(self)
    }
    fn code(&self) -> Value {
        json!(self
            .0
            .iter()
            .map(|(e, c)| json!({"exponents":e,"coefficient":c}))
            .collect::<Vec<_>>())
    }
}
fn f(x: &Polynomial, a: &Polynomial, b: &Polynomial) -> Polynomial {
    x.sq().mul(x).add(&a.mul(x)).add(b)
}
fn n(x: &Polynomial, t: &Polynomial, a: &Polynomial, b: &Polynomial) -> Polynomial {
    t.mul(&x.sq())
        .add(&a.add(&t.sq()).mul(x))
        .add(&a.mul(t))
        .add(&b.scale(2))
}
fn q(x: &Polynomial, t: &Polynomial, a: &Polynomial, b: &Polynomial) -> Polynomial {
    x.mul(t).sub(a).sq().sub(&b.mul(&x.add(t)).scale(4))
}
fn main() {
    let [x, t, z, v, a, b, c] = std::array::from_fn(Polynomial::x);
    let dg = x.sub(&t).sq();
    let dq = z.sub(&v).sq();
    let d = dg.mul(&dq);
    let ng = n(&x, &t, &a, &b);
    let nq = n(&z, &v, &a, &b);
    let left = ng.sq().sub(&f(&x, &a, &b).mul(&f(&t, &a, &b)).scale(4));
    let right = dg.mul(&q(&x, &t, &a, &b));
    assert_eq!(left, right);
    let l = ng.mul(&dq).sub(&nq.mul(&dg));
    let e0 = l
        .sq()
        .sub(&f(&x, &a, &b).mul(&f(&t, &a, &b)).mul(&dq.sq()).scale(4))
        .sub(&f(&z, &a, &b).mul(&f(&v, &a, &b)).mul(&dg.sq()).scale(4))
        .add(&c.mul(&d).scale(8));
    let m0 = q(&x, &t, &a, &b)
        .mul(&dq)
        .add(&q(&z, &v, &a, &b).mul(&dg))
        .sub(&ng.mul(&nq).scale(2))
        .add(&c.scale(8));
    let dm0 = d.mul(&m0);
    assert_eq!(e0, dm0);
    let e1 = l.mul(&d).scale(2);
    let m1 = l.scale(2);
    assert_eq!(e1, d.mul(&m1));
    let wrong = m0.sub(&c.scale(16));
    let bad = e0.sub(&d.mul(&wrong));
    assert!(!bad.0.is_empty());
    let out = json!({"all_checks_pass":true,"ring":"Z[x,t,z,v,a,b,c],c=y*w*gamma; checked i128 coefficient arithmetic","variables":["x","t","z","v","a","b","c"],"biquadratic_left":left.code(),"biquadratic_right":right.code(),"expanded_E0":e0.code(),"D_times_M0":dm0.code(),"E1":e1.code(),"D_times_M1":d.mul(&m1).code(),"wrong_gamma_sign_residual":bad.code(),"correct_residual_terms":0,"wrong_residual_terms":bad.0.len()});
    let args: Vec<_> = std::env::args().collect();
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let old: Value = serde_json::from_slice(&std::fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, old, "changed identity certificate")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap())
}
