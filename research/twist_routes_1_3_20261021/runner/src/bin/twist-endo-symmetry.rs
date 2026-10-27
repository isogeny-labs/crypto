//! Bounded exact twist-torsion -> isogeny -> relation transport experiment.
//! No timing, novelty, cryptographic-size or solver-advantage claim.
#![allow(dead_code)]
#[path = "../registry.rs"]
mod registry;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod toy_field;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, error::Error, fs, path::Path};
use toy_field::{inv, Quad, P, Q};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Poly = Vec<u64>;
#[derive(Clone, Copy)]
struct Ring(u64);
impl Ring {
    fn trim(self, mut a: Poly) -> Poly {
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    }
    fn add(self, a: &[u64], b: &[u64]) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| (a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0)) % self.0)
                .collect(),
        )
    }
    fn scale(self, a: &[u64], s: u64) -> Poly {
        self.trim(a.iter().map(|x| x * s % self.0).collect())
    }
    fn sub(self, a: &[u64], b: &[u64]) -> Poly {
        self.add(a, &self.scale(b, self.0 - 1))
    }
    fn mul(self, a: &[u64], b: &[u64]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = (out[i + j] + x * y) % self.0;
            }
        }
        self.trim(out)
    }
    fn pow(self, a: &[u64], n: usize) -> Poly {
        (0..n).fold(vec![1], |v, _| self.mul(&v, a))
    }
    fn dr(self, a: &[u64], b: &[u64]) -> (Poly, Poly) {
        assert!(!b.is_empty());
        let mut rem = self.trim(a.to_vec());
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        let bi = inv(*b.last().unwrap(), self.0);
        while rem.len() >= b.len() {
            let d = rem.len() - b.len();
            let c = rem[rem.len() - 1] * bi % self.0;
            q[d] = c;
            for (i, x) in b.iter().enumerate() {
                rem[i + d] = (rem[i + d] + self.0 - c * x % self.0) % self.0;
            }
            rem = self.trim(rem);
        }
        (self.trim(q), rem)
    }
    fn exact(self, a: &[u64], b: &[u64]) -> Poly {
        let (q, r) = self.dr(a, b);
        assert!(r.is_empty());
        q
    }
    fn gcd(self, a: &[u64], b: &[u64]) -> Poly {
        let (mut a, mut b) = (a.to_vec(), b.to_vec());
        while !b.is_empty() {
            let r = self.dr(&a, &b).1;
            a = b;
            b = r;
        }
        if a.is_empty() {
            a
        } else {
            self.scale(&a, inv(*a.last().unwrap(), self.0))
        }
    }
    fn der(self, a: &[u64]) -> Poly {
        self.trim(
            a.iter()
                .enumerate()
                .skip(1)
                .map(|(i, x)| x * i as u64 % self.0)
                .collect(),
        )
    }
    fn eval(self, a: &[u64], x: u64) -> u64 {
        a.iter().rev().fold(0, |v, c| (v * x + c) % self.0)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Rat {
    n: Poly,
    d: Poly,
}
impl Rat {
    fn new(r: Ring, n: Poly, d: Poly) -> Self {
        assert!(!d.is_empty());
        let g = r.gcd(&n, &d);
        let n = r.exact(&n, &g);
        let d = r.exact(&d, &g);
        let s = inv(*d.last().unwrap(), r.0);
        Self {
            n: r.scale(&n, s),
            d: r.scale(&d, s),
        }
    }
    fn poly(a: Poly) -> Self {
        Self { n: a, d: vec![1] }
    }
    fn add(&self, r: Ring, b: &Self) -> Self {
        Self::new(
            r,
            r.add(&r.mul(&self.n, &b.d), &r.mul(&b.n, &self.d)),
            r.mul(&self.d, &b.d),
        )
    }
    fn sub(&self, r: Ring, b: &Self) -> Self {
        self.add(r, &b.scale(r, r.0 - 1))
    }
    fn mul(&self, r: Ring, b: &Self) -> Self {
        Self::new(r, r.mul(&self.n, &b.n), r.mul(&self.d, &b.d))
    }
    fn div(&self, r: Ring, b: &Self) -> Self {
        assert!(!b.n.is_empty());
        Self::new(r, r.mul(&self.n, &b.d), r.mul(&self.d, &b.n))
    }
    fn scale(&self, r: Ring, s: u64) -> Self {
        Self::new(r, r.scale(&self.n, s), self.d.clone())
    }
    fn pow(&self, r: Ring, n: usize) -> Self {
        Self::new(r, r.pow(&self.n, n), r.pow(&self.d, n))
    }
}
fn compose(r: Ring, a: &[u64], x: &Rat) -> Rat {
    a.iter().rev().fold(Rat::poly(vec![]), |v, c| {
        v.mul(r, x).add(r, &Rat::poly(r.trim(vec![*c])))
    })
}
// Generic point is (X(x), y*S(x)) in Fp(x)[y]/(y²-f(x)).
fn generic_add(
    r: Ring,
    a: u64,
    b: u64,
    p: &(Rat, Rat),
    q: &(Rat, Rat),
    double: bool,
) -> (Rat, Rat) {
    let f = Rat::poly(vec![b, a, 0, 1]);
    let l = if double {
        p.0.pow(r, 2)
            .scale(r, 3)
            .add(r, &Rat::poly(vec![a]))
            .div(r, &f.mul(r, &p.1).scale(r, 2))
    } else {
        q.1.sub(r, &p.1).div(r, &q.0.sub(r, &p.0))
    };
    let x = f.mul(r, &l.pow(r, 2)).sub(r, &p.0).sub(r, &q.0);
    let s = l.mul(r, &p.0.sub(r, &x)).sub(r, &p.1);
    (x, s)
}
fn generic_seven(r: Ring, a: u64, b: u64) -> (Rat, Rat) {
    let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
    let p2 = generic_add(r, a, b, &p, &p, true);
    let p3 = generic_add(r, a, b, &p2, &p, false);
    let p6 = generic_add(r, a, b, &p3, &p3, true);
    generic_add(r, a, b, &p6, &p, false)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Velu {
    h: Poly,
    n: Poly,
    m: Poly,
    a: u64,
    b: u64,
    degree: u64,
}
impl Velu {
    fn roots(r: Ring, a: u64, b: u64, roots: &[u64]) -> Self {
        let h = roots
            .iter()
            .fold(vec![1], |h, t| r.mul(&h, &[(r.0 - t) % r.0, 1]));
        let mut n = r.mul(&[0, 1], &r.pow(&h, 2));
        let (mut ts, mut ws) = (0, 0);
        for t in roots {
            let q = r.exact(&h, &[(r.0 - t) % r.0, 1]);
            let tt = (3 * t * t + a) % r.0;
            let f = (t * t % r.0 * t + a * t + b) % r.0;
            n = r.add(
                &n,
                &r.add(
                    &r.scale(&r.mul(&q, &h), 2 * tt % r.0),
                    &r.scale(&r.pow(&q, 2), 4 * f % r.0),
                ),
            );
            ts = (ts + tt) % r.0;
            ws = (ws + 5 * t * t % r.0 * t + 3 * a * t + 2 * b) % r.0;
        }
        let m = r.sub(&r.mul(&r.der(&n), &h), &r.scale(&r.mul(&n, &r.der(&h)), 2));
        Self {
            h,
            n,
            m,
            a: (a + r.0 - 10 * ts % r.0) % r.0,
            b: (b + r.0 - 14 * ws % r.0) % r.0,
            degree: 2 * roots.len() as u64 + 1,
        }
    }
    fn certify(&self, r: Ring, a: u64, b: u64) -> bool {
        self.n.len() == self.degree as usize + 1
            && self.h.len() == (self.degree as usize + 1) / 2
            && r.gcd(&self.n, &self.h) == vec![1]
            && r.gcd(&self.h, &r.der(&self.h)) == vec![1]
            && r.mul(&[b, a, 0, 1], &r.pow(&self.m, 2))
                == r.add(
                    &r.add(
                        &r.pow(&self.n, 3),
                        &r.scale(&r.mul(&self.n, &r.pow(&self.h, 4)), self.a),
                    ),
                    &r.scale(&r.pow(&self.h, 6), self.b),
                )
    }
    fn eval(&self, e: Quad, p: P) -> P {
        let Some((x, y)) = p else { return None };
        let ev = |a: &[u64]| {
            a.iter()
                .rev()
                .fold(Q(0, 0), |v, c| e.add(e.mul(v, x), Q(*c, 0)))
        };
        let h = ev(&self.h);
        if h == Q(0, 0) {
            return None;
        }
        let hi = e.inv(h);
        let hi2 = e.mul(hi, hi);
        Some((
            e.mul(ev(&self.n), hi2),
            e.mul(y, e.mul(ev(&self.m), e.mul(hi2, hi))),
        ))
    }
    fn rat(&self, r: Ring, x: &Rat, s: &Rat) -> (Rat, Rat) {
        let h = compose(r, &self.h, x);
        (
            compose(r, &self.n, x).div(r, &h.pow(r, 2)),
            s.mul(r, &compose(r, &self.m, x).div(r, &h.pow(r, 3))),
        )
    }
}
fn scale_point(e: Quad, p: P, u: u64) -> P {
    p.map(|(x, y)| (e.scale(x, u * u % e.p), e.scale(y, u * u % e.p * u % e.p)))
}
fn rational(e: Quad) -> Vec<P> {
    let mut out = vec![None];
    for x in 0..e.p {
        if let Some(y) = toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p) {
            out.push(Some((Q(x, 0), Q(y, 0))));
            if y != 0 {
                out.push(Some((Q(x, 0), Q(e.p - y, 0))));
            }
        }
    }
    out.sort();
    out
}
fn canon(e: Quad, x: u64) -> P {
    toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p)
        .map(|y| Some((Q(x, 0), Q(y.min(e.p - y), 0))))
        .unwrap()
}
fn pair(p: P) -> Vec<u64> {
    p.map(|(x, y)| {
        assert_eq!((x.1, y.1), (0, 0));
        vec![x.0, y.0]
    })
    .unwrap_or(vec![0])
}
fn embed(e: Quad, p: P) -> P {
    p.map(|(x, y)| {
        (
            Q(x.0 * inv(e.d, e.p) % e.p, 0),
            Q(0, y.0 * inv(e.d * e.d % e.p, e.p) % e.p),
        )
    })
}
fn roots(points: &BTreeSet<P>) -> Vec<u64> {
    points
        .iter()
        .filter_map(|p| {
            p.map(|(x, _)| {
                assert_eq!(x.1, 0);
                x.0
            })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
fn f3(e: Quad, x: u64, y: u64, z: u64) -> u64 {
    let a = (x + e.p - y) % e.p;
    let c = (x + y) % e.p;
    let v = (x * y + e.p - e.a) % e.p;
    (a * a % e.p * z % e.p * z + e.p
        - 2 * ((c * (x * y % e.p) + e.a * c + 2 * e.b) % e.p) * z % e.p
        + v * v % e.p
        + e.p
        - 4 * e.b % e.p * c % e.p)
        % e.p
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Witness {
    indices: Vec<usize>,
    mask: u64,
}
fn witness_sum(e: Quad, base: &[P], w: &Witness) -> P {
    w.indices.iter().enumerate().fold(None, |s, (i, j)| {
        e.pa(
            s,
            if w.mask & (1 << i) == 0 {
                base[*j]
            } else {
                e.pn(base[*j])
            },
        )
    })
}
fn collect(e: Quad, base: &[P], target: P, m: usize) -> Vec<Witness> {
    let mut out = vec![];
    for i in 0..base.len() {
        for j in i..base.len() {
            for k in 0..if m == 3 { base.len() } else { 1 } {
                for mask in 0..1 << m {
                    let w = Witness {
                        indices: if m == 3 { vec![i, j, k] } else { vec![i, j] },
                        mask,
                    };
                    if witness_sum(e, base, &w) == target {
                        out.push(w);
                    }
                }
            }
        }
    }
    out
}
fn solve(mut rows: Vec<Vec<u64>>, n: usize, p: u64) -> (usize, Option<Vec<u64>>) {
    let mut pivot = vec![];
    let mut rank = 0;
    for c in 0..n {
        let Some(k) = (rank..rows.len()).find(|k| rows[*k][c] != 0) else {
            continue;
        };
        rows.swap(k, rank);
        let s = inv(rows[rank][c], p);
        for x in &mut rows[rank] {
            *x = *x * s % p;
        }
        for i in 0..rows.len() {
            if i != rank {
                let s = rows[i][c];
                for j in c..=n {
                    rows[i][j] = (rows[i][j] + p - s * rows[rank][j] % p) % p;
                }
            }
        }
        pivot.push(c);
        rank += 1;
    }
    assert!(
        rows.iter()
            .all(|r| r[..n].iter().any(|x| *x != 0) || r[n] == 0),
        "inconsistent relation system"
    );
    if rank != n {
        return (rank, None);
    }
    let mut x = vec![0; n];
    for (i, c) in pivot.iter().enumerate() {
        x[*c] = rows[i][n];
    }
    (rank, Some(x))
}
fn row(w: &Witness, j: u64, b: usize) -> Vec<u64> {
    let mut r = vec![0; b + 2];
    for (i, k) in w.indices.iter().enumerate() {
        r[*k] = (r[*k] + if w.mask & (1 << i) == 0 { 1 } else { 822 }) % 823;
    }
    r[b] = 822;
    r[b + 1] = j;
    r
}
fn ppoly(a: &[u64]) -> String {
    if a.is_empty() {
        "Mod(0,p)".into()
    } else {
        format!("Polrev({})*Mod(1,p)", serde_json::to_string(a).unwrap())
    }
}
fn prat(a: &Rat) -> String {
    format!("({})/({})", ppoly(&a.n), ppoly(&a.d))
}
fn gp_point(p: P) -> String {
    serde_json::to_string(&pair(p)).unwrap()
}
fn gp_map(m: &Velu) -> String {
    format!(
        "[{},{},{}]",
        ppoly(&m.n),
        format!("y*({})", ppoly(&m.m)),
        ppoly(&m.h)
    )
}
fn order(e: Quad, p: P, n: u64) -> u64 {
    (1..=n).find(|k| e.pm(p, *k as u128).is_none()).unwrap()
}

include!("../endo_experiment_20261027.rs");
