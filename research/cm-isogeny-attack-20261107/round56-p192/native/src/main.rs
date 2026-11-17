//! Cyclic marked trace from public points, with independent Frobenius-map checks.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct F<const P: i64, const D: i64>(i64, i64);
impl<const P: i64, const D: i64> F<P, D> {
    fn new(a: i64, b: i64) -> Self {
        Self(a.rem_euclid(P), b.rem_euclid(P))
    }
    fn c(a: i64) -> Self {
        Self::new(a, 0)
    }
    fn pow(mut self, mut n: u64) -> Self {
        let mut z = Self::c(1);
        while n > 0 {
            if n & 1 == 1 {
                z = z * self
            }
            self = self * self;
            n >>= 1;
        }
        z
    }
    fn inv(self) -> Self {
        assert_ne!(self, Self::c(0));
        self.pow((P * P - 2) as u64)
    }
    fn code(self) -> Value {
        json!([self.0, self.1])
    }
}
impl<const P: i64, const D: i64> std::ops::Add for F<P, D> {
    type Output = Self;
    fn add(self, y: Self) -> Self {
        Self::new(self.0 + y.0, self.1 + y.1)
    }
}
impl<const P: i64, const D: i64> std::ops::Neg for F<P, D> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0, -self.1)
    }
}
impl<const P: i64, const D: i64> std::ops::Sub for F<P, D> {
    type Output = Self;
    fn sub(self, y: Self) -> Self {
        self + -y
    }
}
impl<const P: i64, const D: i64> std::ops::Mul for F<P, D> {
    type Output = Self;
    fn mul(self, y: Self) -> Self {
        Self::new(self.0 * y.0 + D * self.1 * y.1, self.0 * y.1 + self.1 * y.0)
    }
}
impl<const P: i64, const D: i64> std::ops::Div for F<P, D> {
    type Output = Self;
    fn div(self, y: Self) -> Self {
        self * y.inv()
    }
}
type Pt<const P: i64, const D: i64> = Option<(F<P, D>, F<P, D>)>;
fn pc<const P: i64, const D: i64>(p: Pt<P, D>) -> Value {
    p.map(|(x, y)| json!([x.code(), y.code()]))
        .unwrap_or(Value::Null)
}
fn neg<const P: i64, const D: i64>(p: Pt<P, D>) -> Pt<P, D> {
    p.map(|(x, y)| (x, -y))
}
fn add<const P: i64, const D: i64>(p: Pt<P, D>, q: Pt<P, D>, a: i64) -> Pt<P, D> {
    match (p, q) {
        (None, _) => q,
        (_, None) => p,
        (Some((x, y)), Some((u, v))) => {
            if x == u && y + v == F::c(0) {
                return None;
            }
            let s = if p == q {
                (F::c(3) * x * x + F::c(a)) / (F::c(2) * y)
            } else {
                (v - y) / (u - x)
            };
            let z = s * s - x - u;
            Some((z, s * (x - z) - y))
        }
    }
}
fn mul<const P: i64, const D: i64>(mut n: u64, mut p: Pt<P, D>, a: i64) -> Pt<P, D> {
    let mut z = None;
    while n > 0 {
        if n & 1 == 1 {
            z = add(z, p, a)
        }
        p = add(p, p, a);
        n >>= 1;
    }
    z
}
fn ev<const P: i64, const D: i64>(a: &[F<P, D>], x: F<P, D>) -> F<P, D> {
    a.iter().rev().fold(F::c(0), |v, &c| v * x + c)
}
fn pm<const P: i64, const D: i64>(a: &[F<P, D>], b: &[F<P, D>]) -> Vec<F<P, D>> {
    let mut c = vec![F::c(0); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = c[i + j] + x * y
        }
    }
    c
}
fn pa<const P: i64, const D: i64>(a: &mut Vec<F<P, D>>, b: &[F<P, D>], scale: F<P, D>) {
    a.resize(a.len().max(b.len()), F::c(0));
    for (i, &v) in b.iter().enumerate() {
        a[i] = a[i] + v * scale
    }
}
fn rd<const P: i64, const D: i64>(a: &[F<P, D>], z: F<P, D>) -> Vec<F<P, D>> {
    let mut q = vec![F::c(0); a.len() - 1];
    q[a.len() - 2] = *a.last().unwrap();
    for i in (1..a.len() - 1).rev() {
        q[i - 1] = a[i] + z * q[i]
    }
    assert_eq!(a[0] + z * q[0], F::c(0));
    q
}
fn der<const P: i64, const D: i64>(a: &[F<P, D>]) -> Vec<F<P, D>> {
    (1..a.len()).map(|i| F::c(i as i64) * a[i]).collect()
}
fn deg<const P: i64, const D: i64>(a: &[F<P, D>]) -> usize {
    a.iter().rposition(|&v| v != F::c(0)).unwrap()
}
fn polypow<const P: i64, const D: i64>(a: &[F<P, D>], n: u64) -> Vec<F<P, D>> {
    let mut z = vec![F::c(1)];
    for _ in 0..n {
        z = pm(&z, a)
    }
    z
}
fn num<const P: i64, const D: i64>(
    graph: &[(Pt<P, D>, Pt<P, D>)],
    shift: Pt<P, D>,
    den: &[F<P, D>],
    a: i64,
    b: i64,
) -> (Vec<F<P, D>>, Vec<F<P, D>>) {
    let mut aa = vec![F::c(0)];
    aa.extend(den.iter().map(|&z| z * shift.unwrap().0));
    let mut bb = vec![F::c(0)];
    let mut seen = vec![];
    for &(u, v) in graph {
        if let Some((x, y)) = u {
            if seen.contains(&x) {
                continue;
            }
            seen.push(x);
            let vm = graph.iter().find(|(z, _)| *z == neg(u)).unwrap().1;
            let up = add(shift, v, a).unwrap().0;
            let um = add(shift, vm, a).unwrap().0;
            let q = rd(&rd(den, x), x);
            pa(
                &mut aa,
                &pm(&q, &[F::c(a) * x + F::c(2 * b), F::c(a) + x * x, x]),
                up + um,
            );
            pa(&mut bb, &q, (um - up) * F::c(2) * y);
        }
    }
    (aa, bb)
}
fn lift<const P: i64, const D: i64>(
    z: F<P, D>,
    shift: Pt<P, D>,
    r: u64,
    a: i64,
    b: i64,
) -> Pt<P, D> {
    let rhs = z * z * z + F::c(a) * z + F::c(b);
    let mut candidates = vec![];
    for i in 0..P {
        for j in 0..P {
            let y = F::new(i, j);
            if y * y == rhs {
                let out = add(Some((z, y)), neg(shift), a);
                if let Some((xx, yy)) = out {
                    if xx.1 == 0 && yy.1 == 0 && mul(r, out, a).is_none() {
                        candidates.push(out)
                    }
                }
            }
        }
    }
    assert_eq!(candidates.len(), 1);
    candidates[0]
}
fn control<const P: i64, const D: i64>(a: i64, b: i64, r: u64, g: Pt<P, D>, s: Pt<P, D>) -> Value {
    assert_eq!(F::<P, D>::c(D).pow(((P - 1) / 2) as u64), F::c(-1));
    let on = |p: Pt<P, D>| {
        p.map(|(x, y)| y * y == x * x * x + F::c(a) * x + F::c(b))
            .unwrap_or(true)
    };
    assert!(on(g) && on(s));
    assert!(mul(r, g, a).is_none());
    assert_eq!(s.map(|(x, y)| (x.pow(P as u64), y.pow(P as u64))), neg(s));
    assert_eq!(mul(17, s, a), None);
    let shift = mul(6, s, a);
    assert!(mul(2, shift, a).unwrap().1 .1 != 0);
    let kernel: Vec<_> = (0..r).map(|j| mul(j, g, a)).collect();
    let mut f = vec![F::c(1)];
    let mut used = vec![];
    for &(x, _) in kernel.iter().flatten() {
        if !used.contains(&x) {
            used.push(x);
            f = pm(&f, &[-x, F::c(1)])
        }
    }
    assert_eq!(deg(&f), (r as usize - 1) / 2);
    let den = pm(&f, &f);
    let dp = der(&f);
    // Ordinary normalized Velu numerator; it must equal x((1-pi)P) times f(X)^2.
    let mut velu = vec![F::c(0)];
    velu.extend(den.iter().copied());
    for &(x, y) in kernel.iter().flatten() {
        let q = rd(&rd(&den, x), x);
        pa(
            &mut velu,
            &pm(&q, &[F::c(a) * x + F::c(2 * b), F::c(a) + x * x, x]),
            F::c(1),
        );
        pa(&mut velu, &pm(&q, &[x * x, -F::c(2) * x, F::c(1)]), -x);
        assert_ne!(y, F::c(0));
    }
    let mut rawden = vec![F::c(0); P as usize + 1];
    rawden[1] = F::c(-1);
    rawden[P as usize] = F::c(1);
    let rawden = pm(&rawden, &rawden);
    let curve = vec![F::c(b), F::c(a), F::c(0), F::c(1)];
    let mut yp = polypow(&curve, ((P - 1) / 2) as u64);
    yp[0] = yp[0] + F::c(1);
    let mut frobnum = pm(&curve, &pm(&yp, &yp));
    let mut xx = vec![F::c(0); P as usize + 1];
    xx[1] = F::c(1);
    xx[P as usize] = F::c(1);
    pa(&mut frobnum, &pm(&xx, &rawden), F::c(-1));
    let left = pm(&velu, &rawden);
    let right = pm(&frobnum, &den);
    let mut delta = left;
    pa(&mut delta, &right, F::c(-1));
    assert!(delta.iter().all(|&z| z == F::c(0)));
    let mut twistfactor = vec![F::c(1)];
    for i in 0..P {
        if !used.contains(&F::c(i)) {
            twistfactor = pm(&twistfactor, &[-F::c(i), F::c(1)]);
        }
    }
    let twistden = pm(&twistfactor, &twistfactor);
    assert_eq!(pm(&den, &twistden), rawden);
    let mut targets = vec![];
    let mut total_actions = 0;
    let mut direct = 0;
    let mut rejected = 0;
    for j in 1..r {
        let q = mul(j, g, a);
        let graph: Vec<_> = (0..r).map(|k| (kernel[k as usize], mul(k, q, a))).collect();
        let (aa, bb) = num(&graph, shift, &den, a, b);
        let (rawa, rawb) = num(&graph, shift, &rawden, a, b);
        assert!(deg(&aa) <= r as usize && deg(&bb) <= r as usize - 3);
        assert_eq!(pm(&aa, &twistden), rawa);
        assert_eq!(pm(&bb, &twistden), rawb);
        let mut probes = vec![];
        for n in 1..=3 {
            let pp = mul(n, s, a);
            let (px, py) = pp.unwrap();
            let trace = graph.iter().fold(F::c(0), |z, &(u, v)| {
                z + add(pp, u, a).unwrap().0 * add(shift, v, a).unwrap().0
            });
            assert_eq!(ev(&aa, px) + py * ev(&bb, px), ev(&den, px) * trace);
            assert_eq!(ev(&rawa, px) + py * ev(&rawb, px), ev(&rawden, px) * trace);
            let pi = pp.map(|(x, y)| (x.pow(P as u64), y.pow(P as u64)));
            let xx = add(pp, neg(pi), a).unwrap().0;
            assert_eq!(ev(&velu, px) / ev(&den, px), xx);
            direct += 1;
            probes
                .push(json!({"P":pc(pp),"trace":trace.code(),"x_frobenius_difference":xx.code()}));
        }
        let mut actions = vec![];
        for &u in kernel.iter().skip(1) {
            let (x, y) = u.unwrap();
            let norm = F::c(4) * y * y * ev(&dp, x).pow(2);
            assert_eq!(ev(&velu, x), norm);
            let value = (ev(&aa, x) - y * ev(&bb, x)) / norm;
            let raw = (ev(&rawa, x) - y * ev(&rawb, x)) / (F::c(4) * y * y);
            assert_eq!(value, raw);
            let out = lift(value, shift, r, a, b);
            assert_eq!(out, mul(j, u, a));
            let wrong = add(out, g, a);
            assert_ne!(add(shift, wrong, a).unwrap().0, value);
            rejected += 1;
            total_actions += 1;
            actions.push(json!({"U":pc(u),"image":pc(out),"pole_value":value.code(),"raw_normalization":(F::c(4)*y*y).code()}));
        }
        targets.push(json!({"Q":pc(q),"A":aa.iter().map(|v|v.code()).collect::<Vec<_>>(),"B":bb.iter().map(|v|v.code()).collect::<Vec<_>>(),"A_degree":deg(&aa),"B_degree":deg(&bb),"probes":probes,"actions":actions}));
    }
    json!({"p":P,"nonsquare":D,"a":a,"b":b,"r":r,"G":pc(g),"S":pc(s),"shift":pc(shift),"kernel_polynomial":f.iter().map(|v|v.code()).collect::<Vec<_>>(),"velu_numerator":velu.iter().map(|v|v.code()).collect::<Vec<_>>(),"rational_map_identity":"full polynomial cross multiplication: Velu == 1-Frobenius","direct_traces":direct,"marked_actions":total_actions,"wrong_outputs_rejected":rejected,"targets":targets})
}
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("usage: cyclic-trace-frobenius ROUND56_DIRECTORY");
    let root = Path::new(&root);
    let c11 = control::<11, 2>(
        2,
        7,
        7,
        Some((F::c(6), F::c(2))),
        Some((F::c(0), F::new(0, 3))),
    );
    let c17 = control::<17, 3>(
        2,
        2,
        19,
        Some((F::c(5), F::c(1))),
        Some((F::c(1), F::new(0, 8))),
    );
    let input: Value =
        serde_json::from_str(&fs::read_to_string(root.join("../round52-p192/input.json")).unwrap())
            .unwrap();
    let p: BigUint = input["p"].to_string().parse().unwrap();
    let a: BigUint = input["a"].to_string().parse().unwrap();
    let b: BigUint = input["b"].to_string().parse().unwrap();
    let mut cases = vec![];
    let old: Value = serde_json::from_str(
        &fs::read_to_string(root.join("../round55-p192/P192-certificate.json")).unwrap(),
    )
    .unwrap();
    for c in old["normalization_cases"].as_array().unwrap() {
        let x: BigUint = c["point"][0].to_string().parse().unwrap();
        let y: BigUint = c["point"][1].to_string().parse().unwrap();
        assert_eq!(&y * &y % &p, (&x * &x * &x + &a * &x + &b) % &p);
        assert_eq!(x.modpow(&p, &p), x);
        assert_eq!(y.modpow(&p, &p), y);
        let n = BigUint::from(4u32) * &y * &y % &p;
        assert!(!n.is_zero());
        cases.push(json!({"point":c["point"],"raw_norm":n.to_string(),"phi_linear_coefficient":"-1 for pi-1; +1 for 1-pi","frobenius_local_order":p.to_string(),"scope":"Laurent leading coefficient only; no P192 cyclic sum"}));
    }
    assert!(p > BigUint::from(3u32));
    assert_eq!((&p - BigUint::one()) % BigUint::from(2u32), BigUint::zero());
    println!("{}",serde_json::to_string(&json!({"all_checks_pass":true,"controls":[c11,c17],"P192":cases,"standing":"unsigned producer evidence","construction_input":"public p,a,b,r,G,Q and chosen quadratic shift; no target scalar"})).unwrap());
}
