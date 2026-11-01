//! Independent certificate checker. It does not call the producer's reduction code.
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

type Key = [u8; 4];
#[derive(Clone, Debug, PartialEq, Eq)]
struct P {
    prime: u64,
    m: BTreeMap<Key, u64>,
}
fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn negmod(a: u64, p: u64) -> u64 {
    if a == 0 { 0 } else { p - a }
}
fn num(v: &Value, k: &str) -> u64 {
    v[k].as_u64().expect(k)
}
impl P {
    fn z(p: u64) -> Self {
        Self {
            prime: p,
            m: BTreeMap::new(),
        }
    }
    fn n(p: u64, n: u64) -> Self {
        let mut x = Self::z(p);
        if !n.is_multiple_of(p) {
            x.m.insert([0; 4], n % p);
        }
        x
    }
    fn var(p: u64, i: usize) -> Self {
        let mut x = Self::z(p);
        let mut e = [0; 4];
        e[i] = 1;
        x.m.insert(e, 1);
        x
    }
    fn from_json(v: &Value, p: u64) -> Self {
        let mut x = Self::z(p);
        for row in v.as_array().unwrap() {
            let a = row.as_array().unwrap();
            let e = std::array::from_fn(|i| a[i].as_u64().unwrap().try_into().unwrap());
            let c = a[4].as_u64().unwrap();
            assert!(c > 0 && c < p && x.m.insert(e, c).is_none());
        }
        x
    }
    fn put(&mut self, e: Key, c: u64) {
        let t = (self.m.get(&e).copied().unwrap_or(0) + c) % self.prime;
        if t == 0 {
            self.m.remove(&e);
        } else {
            self.m.insert(e, t);
        }
    }
    fn plus(&self, y: &Self) -> Self {
        assert_eq!(self.prime, y.prime);
        let mut z = self.clone();
        for (&e, &c) in &y.m {
            z.put(e, c)
        }
        z
    }
    fn times_n(&self, c: u64) -> Self {
        let mut z = Self::z(self.prime);
        for (&e, &a) in &self.m {
            z.put(e, mulmod(a, c, self.prime))
        }
        z
    }
    fn minus(&self, y: &Self) -> Self {
        self.plus(&y.times_n(self.prime - 1))
    }
    fn times(&self, y: &Self) -> Self {
        assert_eq!(self.prime, y.prime);
        let mut z = Self::z(self.prime);
        for (&e, &a) in &self.m {
            for (&f, &b) in &y.m {
                let g = std::array::from_fn(|i| e[i].checked_add(f[i]).unwrap());
                z.put(g, mulmod(a, b, self.prime));
            }
        }
        z
    }
    fn power(&self, n: u32) -> Self {
        let mut z = Self::n(self.prime, 1);
        for _ in 0..n {
            z = z.times(self);
        }
        z
    }
    fn is_zero(&self) -> bool {
        self.m.is_empty()
    }
}
fn field_curve(v: &Value, p: u64) -> Vec<u64> {
    let mut f = vec![0; 6];
    f[1] = 1;
    for root in [1, num(v, "lambda"), num(v, "mu"), num(v, "nu")] {
        let mut next = vec![0; 6];
        for i in 0..5 {
            next[i + 1] = (next[i + 1] + f[i]) % p;
            next[i] = (next[i] + negmod(mulmod(root, f[i], p), p)) % p;
        }
        f = next;
    }
    f
}
fn field_eval(f: &[u64], r: u64, p: u64) -> u64 {
    f.iter().rev().fold(0, |a, &b| (mulmod(a, r, p) + b) % p)
}
fn poly_curve_remainder(f: &[u64], a0: &P, a1: &P) -> (P, P, P) {
    let p = a0.prime;
    let mut constant = P::n(p, 1);
    let mut linear = P::z(p);
    let mut r0 = P::z(p);
    let mut r1 = P::z(p);
    let mut h = P::z(p);
    for (i, &coefficient) in f.iter().enumerate() {
        r0 = r0.plus(&constant.times_n(coefficient));
        r1 = r1.plus(&linear.times_n(coefficient));
        if i >= 1 {
            h = h.minus(&linear.times_n(f.get(i + 1).copied().unwrap_or(0)));
        }
        let next_constant = a0.times(&linear).times_n(p - 1);
        let next_linear = constant.minus(&a1.times(&linear));
        constant = next_constant;
        linear = next_linear;
    } // h=-sum_{n>=2}f_n L_{n-1}
    (r0, r1, h)
}
fn matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|j| v[i][j].as_u64().unwrap()))
}
fn apply(m: &[[u64; 4]; 4], v: &[P; 4]) -> [P; 4] {
    let p = v[0].prime;
    std::array::from_fn(|i| (0..4).fold(P::z(p), |s, j| s.plus(&v[j].times_n(m[i][j]))))
}
fn parse4(v: &Value, p: u64) -> [P; 4] {
    std::array::from_fn(|i| P::from_json(&v[i], p))
}
fn conversion(t: &Value, p: u64) -> [[u64; 4]; 4] {
    let l = num(t, "lambda");
    let mu = num(t, "mu");
    let nu = num(t, "nu");
    let d = ["a", "b", "c", "d"].map(|k| num(t, k));
    let c = [mu, mulmod(l, nu, p), nu, mulmod(l, mu, p)];
    let e = [(l + nu) % p, (1 + mu) % p, (l + mu) % p, (1 + nu) % p];
    std::array::from_fn(|i| {
        [
            mulmod(d[i], mulmod(c[i], e[i], p), p),
            negmod(mulmod(d[i], c[i], p), p),
            mulmod(d[i], e[i], p),
            negmod(d[i], p),
        ]
    })
}
fn product_x(x: &[P], y: &[P]) -> Vec<P> {
    let p = x[0].prime;
    let mut z = vec![P::z(p); x.len() + y.len() - 1];
    for (i, a) in x.iter().enumerate() {
        for (j, b) in y.iter().enumerate() {
            z[i + j] = z[i + j].plus(&a.times(b));
        }
    }
    z
}
fn check_row(row: &Value, theta: &Value, action: &Value) {
    let p = num(row, "p");
    let t = &theta["selected"]["constants"];
    let idx = num(row, "action_index") as usize;
    let r0 = num(&theta["selected"], "r0");
    let delta = num(&theta["selected"], "delta");
    let original_root = num(action, "root");
    let r = num(row, "root");
    assert_eq!(original_root, (r0 + mulmod(r, delta, p)) % p);
    let f = field_curve(t, p);
    assert_eq!(
        f,
        row["curve_coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(field_eval(&f, r, p), 0);
    let c = conversion(t, p);
    let m = matrix(&action["action"]["matrix"]);
    assert_eq!(c, matrix(&row["conversion_matrix"]));
    assert_eq!(m, matrix(&row["action_matrix"]));
    let a0 = P::var(p, 0);
    let a1 = P::var(p, 1);
    let b0 = P::var(p, 2);
    let b1 = P::var(p, 3);
    let one = P::n(p, 1);
    let (r0p, r1p, h0) = poly_curve_remainder(&f, &a0, &a1);
    let e0 = b0.power(2).minus(&a0.times(&b1.power(2))).minus(&r0p);
    let e1 = b0
        .times(&b1)
        .times_n(2)
        .minus(&a1.times(&b1.power(2)))
        .minus(&r1p);
    assert_eq!(e0, P::from_json(&row["mumford_generators"][0], p));
    assert_eq!(e1, P::from_json(&row["mumford_generators"][1], p));
    let q = [
        one.clone(),
        a1.times_n(p - 1),
        a0.times_n(p - 1),
        a0.times(&a1).plus(&b1.power(2)).plus(&h0),
    ];
    let source_w = parse4(&row["source_w"], p);
    assert_eq!(apply(&c, &source_w), apply(&m, &apply(&c, &q)));
    let u = a0.plus(&a1.times_n(r)).plus(&P::n(p, mulmod(r, r, p)));
    let v = b0.plus(&b1.times_n(r));
    let ln = [
        u.times(&b0).minus(&v.times(&a0)),
        u.times(&b1).minus(&v.times(&a1)),
        v.times_n(p - 1),
    ];
    let mut numerator = vec![P::z(p); 6];
    for (i, &coef) in f.iter().enumerate() {
        numerator[i] = u.power(2).times_n(coef);
    }
    let l2 = product_x(&ln, &ln);
    for (i, val) in l2.iter().enumerate() {
        numerator[i] = numerator[i].minus(val);
    }
    let den = [
        a0.times_n(negmod(r, p)),
        a0.minus(&a1.times_n(r)),
        a1.minus(&P::n(p, r)),
        one.clone(),
    ];
    let mut quotient = vec![P::z(p); 3];
    for k in (0..=2).rev() {
        quotient[k] = numerator[k + 3].clone();
        for j in 0..4 {
            numerator[k + j] = numerator[k + j].minus(&quotient[k].times(&den[j]));
        }
    }
    let d = num(row, "denominator_power") as u32;
    assert_eq!(d, 6);
    let qn = parse4(&row["translated_num"], p);
    assert_eq!(qn[0], u.power(d));
    assert_eq!(qn[1], quotient[1].times_n(p - 1).times(&u.power(d - 2)));
    assert_eq!(qn[2], quotient[0].times_n(p - 1).times(&u.power(d - 2)));
    let bnum = b1
        .times_n(p - 1)
        .times(&u.power(3))
        .plus(&v.times(&a1.times(&u.power(2)).minus(&quotient[1])));
    let mut hnum = P::z(p);
    let degree =
        h0.m.keys()
            .map(|e| u32::from(e[0]) + u32::from(e[1]))
            .max()
            .unwrap_or(0);
    assert_eq!(degree, 3);
    for (&e, &coefficient) in &h0.m {
        assert_eq!(e[2], 0);
        assert_eq!(e[3], 0);
        let term = quotient[0]
            .power(u32::from(e[0]))
            .times(&quotient[1].power(u32::from(e[1])))
            .times(&u.power(2 * (degree - u32::from(e[0]) - u32::from(e[1]))))
            .times_n(coefficient);
        hnum = hnum.plus(&term);
    }
    let expected_q3 = quotient[0]
        .times(&quotient[1])
        .times(&u.power(d - 4))
        .plus(&bnum.power(2).times(&u.power(d - 6)))
        .plus(&hnum.times(&u.power(d - 2 * degree)));
    assert_eq!(qn[3], expected_q3);
    let factor = num(row, "w0_factor");
    assert_ne!(factor, 0);
    assert_eq!(source_w[0], u.times_n(factor));
    assert_eq!(row["w0_equals_factor_u"], true);
    let source_k = apply(&c, &source_w);
    let translated_k = apply(&c, &qn);
    let certs = row["six_cross_product_certificates"].as_array().unwrap();
    assert_eq!(certs.len(), 6);
    let mut h = 0;
    for i in 0..4 {
        for j in i + 1..4 {
            let cert = &certs[h];
            let cross = translated_k[i]
                .times(&source_k[j])
                .minus(&translated_k[j].times(&source_k[i]));
            let declared = P::from_json(&cert["cross_product"], p);
            assert_eq!(cross, declared, "cross p={p} idx={idx} {i},{j}");
            let q0 = P::from_json(&cert["e0_quotient"], p);
            let q1 = P::from_json(&cert["e1_quotient"], p);
            let rem = P::from_json(&cert["remainder"], p);
            assert!(rem.is_zero(), "nonzero remainder p={p} idx={idx} {i},{j}");
            assert_eq!(
                declared,
                e0.times(&q0).plus(&e1.times(&q1)).plus(&rem),
                "ideal membership p={p} idx={idx} {i},{j}"
            );
            h += 1;
        }
    }
    let remainder_certs = row["quotient_remainder_certificates"].as_array().unwrap();
    assert_eq!(remainder_certs.len(), 3);
    for (i, residual) in numerator.iter().take(3).enumerate() {
        let cert = &remainder_certs[i];
        let declared = P::from_json(&cert["polynomial"], p);
        assert_eq!(*residual, declared);
        let q0 = P::from_json(&cert["e0_quotient"], p);
        let q1 = P::from_json(&cert["e1_quotient"], p);
        let remainder = P::from_json(&cert["remainder"], p);
        assert!(remainder.is_zero());
        assert_eq!(
            declared,
            e0.times(&q0).plus(&e1.times(&q1)).plus(&remainder)
        );
    }
}
fn main() {
    let base = "research/kummer_differential_collector_20261022/n16-action-certificate/results";
    let data: Value =
        serde_json::from_slice(&fs::read(format!("{base}/generic-ideal.json")).unwrap()).unwrap();
    assert_eq!(data["schema"], "kummer-n16-generic-ideal/v1");
    let theta: HashMap<u64, Value> =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl")
            .unwrap()
            .lines()
            .map(|s| {
                let v: Value = serde_json::from_str(s).unwrap();
                (num(&v, "p"), v)
            })
            .collect();
    let actions: HashMap<u64, Value> =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap()
            .lines()
            .map(|s| {
                let v: Value = serde_json::from_str(s).unwrap();
                (num(&v, "p"), v)
            })
            .collect();
    let rows = data["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 105);
    let mut seen = HashSet::new();
    for row in rows {
        let p = num(row, "p");
        let i = num(row, "action_index") as usize;
        assert!(seen.insert((p, i)));
        assert!(i < 5);
        check_row(row, &theta[&p], &actions[&p]["torsion_actions"][i]);
    }
    assert_eq!(seen.len(), 105);
    println!(
        "verified 105 generic model/root rows and 630 six-cross-product polynomial-ideal memberships"
    );
}
