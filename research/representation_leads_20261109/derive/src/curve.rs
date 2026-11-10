//! Ordinary binary curve y^2 + xy = x^3 + a x^2 + b over GF(2^n).

use crate::gf::{Gf, Rng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    Inf,
    A(u64, u64),
}

pub struct Curve {
    pub f: Gf,
    pub a: u64,
    pub b: u64,
}

impl Curve {
    pub fn on(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::A(x, y) => {
                let f = &self.f;
                let x2 = f.sq(x);
                f.sq(y) ^ f.mul(x, y) == f.mul(x2, x) ^ f.mul(self.a, x2) ^ self.b
            }
        }
    }
    pub fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::A(x, y) => Pt::A(x, y ^ x),
        }
    }
    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                if x1 == x2 {
                    if y1 ^ y2 == x1 || x1 == 0 {
                        return Pt::Inf; // q = -p, or doubling a 2-torsion point
                    }
                    let l = x1 ^ f.mul(y1, f.inv(x1));
                    let x3 = f.sq(l) ^ l ^ self.a;
                    let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                    return Pt::A(x3, y3);
                }
                let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ self.a;
                let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                Pt::A(x3, y3)
            }
        }
    }
    /// x lies on E (rather than on the quadratic twist) iff Tr(x + a + b/x^2) = 0.
    pub fn x_on_curve(&self, x: u64) -> bool {
        let f = &self.f;
        x != 0 && f.trace(x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)))) == 0
    }
    pub fn lift(&self, x: u64) -> Option<Pt> {
        if !self.x_on_curve(x) {
            return None;
        }
        let f = &self.f;
        let z = f.half_trace(x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x))));
        Some(Pt::A(x, f.mul(x, z)))
    }
    pub fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            if let Some(p) = self.lift(rng.nonzero(&self.f)) {
                return if rng.next() & 1 == 1 { self.neg(p) } else { p };
            }
        }
    }
    /// Semaev S_3 for this curve family (depends only on b).
    pub fn s3(&self, x1: u64, x2: u64, x3: u64) -> u64 {
        let f = &self.f;
        let e2 = f.mul(x1, x2) ^ f.mul(x1, x3) ^ f.mul(x2, x3);
        f.sq(e2) ^ f.mul(f.mul(x1, x2), x3) ^ self.b
    }
}
