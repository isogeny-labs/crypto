//! Sparse multivariate polynomials over F_p (up to 4 variables) and the
//! Semaev summation polynomials S_3 and S_4 = Res_X(S_3(x1,x2,X), S_3(x3,x4,X)).
use crate::fp::*;
use std::collections::HashMap;

pub type Exp = [u8; 4];

#[derive(Clone, Debug)]
pub struct MP { pub p: u64, pub t: HashMap<Exp, u64> }

impl MP {
    pub fn zero(p: u64) -> MP { MP { p, t: HashMap::new() } }
    pub fn constant(c: u64, p: u64) -> MP {
        let mut m = MP::zero(p);
        if c % p != 0 { m.t.insert([0; 4], c % p); }
        m
    }
    pub fn var(i: usize, p: u64) -> MP {
        let mut e = [0u8; 4];
        e[i] = 1;
        let mut m = MP::zero(p);
        m.t.insert(e, 1);
        m
    }
    fn addterm(&mut self, e: Exp, c: u64) {
        let p = self.p;
        let v = self.t.entry(e).or_insert(0);
        *v = addm(*v, c, p);
        if *v == 0 { self.t.remove(&e); }
    }
    pub fn add(&self, o: &MP) -> MP {
        let mut r = self.clone();
        for (e, c) in &o.t { r.addterm(*e, *c); }
        r
    }
    pub fn sub(&self, o: &MP) -> MP {
        let mut r = self.clone();
        for (e, c) in &o.t { r.addterm(*e, subm(0, *c, self.p)); }
        r
    }
    pub fn scale(&self, k: u64) -> MP {
        let mut r = MP::zero(self.p);
        for (e, c) in &self.t { r.addterm(*e, mulm(*c, k % self.p, self.p)); }
        r
    }
    pub fn mul(&self, o: &MP) -> MP {
        let mut r = MP::zero(self.p);
        for (e1, c1) in &self.t {
            for (e2, c2) in &o.t {
                let e = [e1[0] + e2[0], e1[1] + e2[1], e1[2] + e2[2], e1[3] + e2[3]];
                r.addterm(e, mulm(*c1, *c2, self.p));
            }
        }
        r
    }
    pub fn eval(&self, x: &[u64; 4]) -> u64 {
        let p = self.p;
        let mut s = 0;
        for (e, c) in &self.t {
            let mut v = *c;
            for i in 0..4 { v = mulm(v, powm(x[i], e[i] as u64, p), p); }
            s = addm(s, v, p);
        }
        s
    }
    /// Substitute variable i := val.
    pub fn subst(&self, i: usize, val: u64) -> MP {
        let mut r = MP::zero(self.p);
        for (e, c) in &self.t {
            let mut e2 = *e;
            let k = e2[i];
            e2[i] = 0;
            r.addterm(e2, mulm(*c, powm(val, k as u64, self.p), self.p));
        }
        r
    }
}

/// S_3(x_i, x_j, x_k) for y^2 = x^3 + a x + b.
pub fn s3_vars(i: usize, j: usize, k: usize, a: u64, b: u64, p: u64) -> MP {
    let (xi, xj, xk) = (MP::var(i, p), MP::var(j, p), MP::var(k, p));
    let (ca, cb) = (MP::constant(a, p), MP::constant(b, p));
    let d = xi.sub(&xj);
    let t1 = d.mul(&d).mul(&xk).mul(&xk);
    let s = xi.add(&xj);
    let pr = xi.mul(&xj);
    let t2 = s.mul(&pr.add(&ca)).add(&cb.scale(2)).mul(&xk).scale(2);
    let q = pr.sub(&ca);
    let t3 = q.mul(&q).sub(&cb.mul(&s).scale(4));
    t1.sub(&t2).add(&t3)
}

/// Coefficients (A, B, C) of S_3(x_i, x_j, X) as a quadratic in X.
fn s3_quadratic(i: usize, j: usize, a: u64, b: u64, p: u64) -> (MP, MP, MP) {
    let (xi, xj) = (MP::var(i, p), MP::var(j, p));
    let (ca, cb) = (MP::constant(a, p), MP::constant(b, p));
    let d = xi.sub(&xj);
    let aa = d.mul(&d);
    let s = xi.add(&xj);
    let pr = xi.mul(&xj);
    let bb = s.mul(&pr.add(&ca)).add(&cb.scale(2)).scale(subm(0, 2, p));
    let q = pr.sub(&ca);
    let cc = q.mul(&q).sub(&cb.mul(&s).scale(4));
    (aa, bb, cc)
}

/// S_4(x0,x1,x2,x3) = Res_X(S_3(x0,x1,X), S_3(x2,x3,X)).
pub fn s4(a: u64, b: u64, p: u64) -> MP {
    let (a1, b1, c1) = s3_quadratic(0, 1, a, b, p);
    let (d1, e1, f1) = s3_quadratic(2, 3, a, b, p);
    let u = a1.mul(&f1).sub(&c1.mul(&d1));
    let v = a1.mul(&e1).sub(&b1.mul(&d1));
    let w = b1.mul(&f1).sub(&c1.mul(&e1));
    u.mul(&u).sub(&v.mul(&w))
}
