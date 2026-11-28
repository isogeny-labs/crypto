//! Genus-2 Jacobian over F_{q^2}: y^2 = f(x), f monic quintic over F_{q^2}.
//! Cantor/Mumford arithmetic, point counting over F_{q^2}, group-order
//! search (BSGS in the Hasse-Weil window), factorisation.

use super::fq2::{Fld, Poly, E, ONE, ZERO};
use std::cell::Cell;
use std::collections::HashMap;

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Div {
    pub u: Poly,
    pub v: Poly,
}

pub type Key = [u32; 8];

pub struct Curve {
    pub fld: Fld,
    pub f: Poly,
    /// group additions (including doublings) performed
    pub adds: Cell<u64>,
}

impl Curve {
    pub fn new(fld: Fld, f: Poly) -> Curve {
        assert_eq!(f.len(), 6);
        assert_eq!(f[5], ONE);
        Curve {
            fld,
            f,
            adds: Cell::new(0),
        }
    }

    pub fn random_f(fld: &Fld, rng: &mut Rng) -> Poly {
        let q = fld.q as u64;
        let mut f: Poly = (0..5)
            .map(|_| E {
                a: rng.below(q) as u32,
                b: rng.below(q) as u32,
            })
            .collect();
        f.push(ONE);
        f
    }

    pub fn is_squarefree(&self) -> bool {
        let d = self.fld.pderiv(&self.f);
        let (g, _, _) = self.fld.pxgcd(&self.f, &d);
        g.len() == 1
    }

    pub fn identity(&self) -> Div {
        Div {
            u: vec![ONE],
            v: Vec::new(),
        }
    }
    pub fn is_identity(&self, d: &Div) -> bool {
        d.u.len() == 1
    }
    pub fn neg(&self, d: &Div) -> Div {
        Div {
            u: d.u.clone(),
            v: self.fld.pneg(&d.v),
        }
    }
    pub fn point(&self, x: E, y: E) -> Div {
        let mut v = vec![y];
        self.fld.trim(&mut v);
        Div {
            u: vec![self.fld.neg(x), ONE],
            v,
        }
    }
    /// Is (u, v) a valid reduced Mumford representation (u | f - v^2)?
    #[cfg(test)]
    pub fn is_valid(&self, d: &Div) -> bool {
        let fl = &self.fld;
        if d.u.is_empty() || d.u.len() > 3 || *d.u.last().unwrap() != ONE {
            return false;
        }
        if d.v.len() >= d.u.len() && !(d.u.len() == 1 && d.v.is_empty()) {
            return false;
        }
        let num = fl.psub(&self.f, &fl.pmul(&d.v, &d.v));
        fl.pdivrem(&num, &d.u).1.is_empty()
    }

    pub fn eval_f(&self, x: E) -> E {
        self.fld.peval(&self.f, x)
    }

    pub fn random_point(&self, rng: &mut Rng) -> Div {
        let q = self.fld.q as u64;
        loop {
            let x = E {
                a: rng.below(q) as u32,
                b: rng.below(q) as u32,
            };
            let fx = self.eval_f(x);
            if let Some(y) = self.fld.sqrt(fx) {
                let y = if rng.next() & 1 == 1 {
                    self.fld.neg(y)
                } else {
                    y
                };
                return self.point(x, y);
            }
        }
    }
    pub fn random_elt(&self, rng: &mut Rng) -> Div {
        let p1 = self.random_point(rng);
        let p2 = self.random_point(rng);
        self.add(&p1, &p2)
    }

    /// Canonical key of a reduced divisor (for hashing and comparison).
    pub fn key(&self, d: &Div) -> Key {
        let mut k = [0u32; 8];
        k[0] = d.u.len() as u32;
        for (i, c) in d.u.iter().take(2).enumerate() {
            k[1 + 2 * i] = c.a;
            k[2 + 2 * i] = c.b;
        }
        let v0 = d.v.first().copied().unwrap_or(ZERO);
        let v1 = d.v.get(1).copied().unwrap_or(ZERO);
        k[5] = v0.a;
        k[6] = v0.b;
        k[7] = v1.a ^ (v1.b << 16);
        k
    }

    pub fn add(&self, d1: &Div, d2: &Div) -> Div {
        self.adds.set(self.adds.get() + 1);
        let fl = &self.fld;
        let (dd1, e1, e2) = fl.pxgcd(&d1.u, &d2.u);
        let vs = fl.padd(&d1.v, &d2.v);
        let (d, c1, c2) = fl.pxgcd(&dd1, &vs);
        let s1 = fl.pmul(&c1, &e1);
        let s2 = fl.pmul(&c1, &e2);
        let s3 = c2;
        let u1u2 = fl.pmul(&d1.u, &d2.u);
        let dd = fl.pmul(&d, &d);
        let (mut u, _) = fl.pdivrem(&u1u2, &dd);
        let t1 = fl.pmul(&fl.pmul(&s1, &d1.u), &d2.v);
        let t2 = fl.pmul(&fl.pmul(&s2, &d2.u), &d1.v);
        let t3 = fl.pmul(&s3, &fl.padd(&fl.pmul(&d1.v, &d2.v), &self.f));
        let num = fl.padd(&fl.padd(&t1, &t2), &t3);
        let (v, _) = fl.pdivrem(&num, &d);
        let mut v = fl.pdivrem(&v, &u).1;
        while u.len() > 3 {
            let vv = fl.pmul(&v, &v);
            let num = fl.psub(&self.f, &vv);
            let (u2, _) = fl.pdivrem(&num, &u);
            let u2 = fl.pmonic(&u2);
            let nv = fl.pdivrem(&fl.pneg(&v), &u2).1;
            u = u2;
            v = nv;
        }
        let u = fl.pmonic(&u);
        let v = fl.pdivrem(&v, &u).1;
        Div { u, v }
    }

    pub fn mul(&self, d: &Div, mut n: u64) -> Div {
        let mut acc = self.identity();
        let mut base = d.clone();
        let mut first = true;
        while n > 0 {
            if n & 1 == 1 {
                acc = if first {
                    first = false;
                    base.clone()
                } else {
                    self.add(&acc, &base)
                };
            }
            n >>= 1;
            if n > 0 {
                base = self.add(&base, &base);
            }
        }
        acc
    }

    /// Number of points of C over F_{q^2} (including infinity).
    pub fn count_points_fq2(&self) -> u64 {
        let fl = &self.fld;
        let q = fl.q;
        let mut n = 1u64;
        for a in 0..q {
            for b in 0..q {
                let fx = self.eval_f(E { a, b });
                if fx == ZERO {
                    n += 1;
                } else if fl.is_square(fx) {
                    n += 2;
                }
            }
        }
        n
    }

    /// Group order of J(F_{q^2}) from N1 = #C(F_{q^2}) via BSGS in the
    /// Weil window.  Returns None when no unique candidate survives.
    pub fn find_order(&self, n1: u64, rng: &mut Rng) -> Option<u64> {
        let q = self.fld.q as i64;
        let big_q = q * q;
        let s1 = big_q + 1 - n1 as i64;
        let s1a = s1.abs();
        if s1a > 4 * q {
            return None;
        }
        let lo = 2 * q * (s1a - 2 * q);
        let hi = (s1a * s1a).div_euclid(4);
        let base = |p: i64| -> i64 { 1 - s1 + 2 * big_q + p - big_q * s1 + big_q * big_q };
        let nlo = base(lo);
        let w = (hi - lo) as u64;
        let m = ((w + 1) as f64).sqrt().ceil() as u64 + 1;
        // pick a first element with a long order (no early collision)
        let mut cands: Vec<u64> = Vec::new();
        for _attempt in 0..8 {
            let d = self.random_elt(rng);
            let mut table: HashMap<Key, u64> = HashMap::new();
            let mut cur = self.identity();
            let mut ok = true;
            for j in 0..m {
                let k = self.key(&cur);
                if table.insert(k, j).is_some() {
                    ok = false;
                    break;
                }
                cur = self.add(&cur, &d);
            }
            if !ok {
                continue;
            }
            // t0 = nlo * d ; need t*d = -t0 ; t = i*m + j
            let t0 = self.mul(&d, nlo as u64);
            let md = cur; // m*d
            let mut y = self.neg(&t0);
            let mdneg = self.neg(&md);
            for i in 0..=(w / m + 1) {
                if let Some(&j) = table.get(&self.key(&y)) {
                    let t = i * m + j;
                    if t <= w {
                        cands.push(t);
                    }
                }
                y = self.add(&y, &mdneg);
            }
            break;
        }
        if cands.is_empty() {
            return None;
        }
        for _ in 0..6 {
            let d = self.random_elt(rng);
            cands.retain(|&t| {
                let n = (nlo + t as i64) as u64;
                self.is_identity(&self.mul(&d, n))
            });
            if cands.len() == 1 {
                break;
            }
        }
        if cands.len() == 1 {
            Some((nlo + cands[0] as i64) as u64)
        } else {
            None
        }
    }
}

pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        if n.is_multiple_of(p) {
            let mut e = 0;
            while n.is_multiple_of(p) {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_curve(q: u32, seed: u64) -> (Curve, Rng) {
        let fld = Fld::new(q);
        let mut rng = Rng(seed);
        loop {
            let f = Curve::random_f(&fld, &mut rng);
            let c = Curve::new(fld.clone(), f);
            if c.is_squarefree() {
                return (c, rng);
            }
        }
    }

    #[test]
    fn group_law_axioms() {
        let (c, mut rng) = tiny_curve(11, 5);
        for _ in 0..40 {
            let a = c.random_elt(&mut rng);
            let b = c.random_elt(&mut rng);
            let d = c.random_elt(&mut rng);
            assert!(c.is_valid(&a));
            let ab = c.add(&a, &b);
            assert!(c.is_valid(&ab));
            assert_eq!(ab, c.add(&b, &a));
            assert_eq!(c.add(&ab, &d), c.add(&a, &c.add(&b, &d)));
            assert!(c.is_identity(&c.add(&a, &c.neg(&a))));
            assert_eq!(c.add(&a, &c.identity()), a);
            assert_eq!(c.mul(&a, 5), c.add(&c.mul(&a, 2), &c.mul(&a, 3)));
        }
    }

    #[test]
    fn order_search_kills_random_elements() {
        for &(q, seed) in &[(7u32, 1u64), (11, 2), (13, 3)] {
            let (c, mut rng) = tiny_curve(q, seed);
            let n1 = c.count_points_fq2();
            let n = c.find_order(n1, &mut rng).expect("order found");
            for _ in 0..10 {
                let d = c.random_elt(&mut rng);
                assert!(c.is_identity(&c.mul(&d, n)), "q={q} N={n}");
            }
            // Weil window sanity: q^4 scale
            let q4 = (q as f64).powi(4);
            assert!((n as f64) > 0.2 * q4 && (n as f64) < 4.0 * q4);
        }
    }

    #[test]
    fn factor_works() {
        assert_eq!(factor(360), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(factor(1_000_003), vec![(1_000_003, 1)]);
    }
}
