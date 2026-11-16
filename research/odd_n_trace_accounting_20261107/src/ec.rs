//! E: y^2 + x y = x^3 + a x^2 + b over GF(2^N), affine points.
use crate::gf::Gf;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pt {
    pub x: u32,
    pub y: u32,
    pub inf: bool,
}
pub const O: Pt = Pt { x: 0, y: 0, inf: true };

impl Pt {
    #[inline]
    pub fn key(&self) -> u64 {
        if self.inf { u64::MAX } else { ((self.x as u64) << 32) | self.y as u64 }
    }
}

pub struct Curve<'a> {
    pub f: &'a Gf,
    pub a: u32,
    pub b: u32,
}

impl<'a> Curve<'a> {
    pub fn on(&self, p: &Pt) -> bool {
        if p.inf {
            return true;
        }
        let f = self.f;
        let (x, y) = (p.x, p.y);
        let lhs = f.sq(y) ^ f.mul(x, y);
        let x2 = f.sq(x);
        let rhs = f.mul(x2, x) ^ f.mul(self.a, x2) ^ self.b;
        lhs == rhs
    }
    #[inline]
    pub fn neg(&self, p: &Pt) -> Pt {
        if p.inf { *p } else { Pt { x: p.x, y: p.x ^ p.y, inf: false } }
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        let f = self.f;
        if p.x == q.x {
            if p.y == q.y {
                return self.dbl(p);
            }
            return O; // q = -p
        }
        let l = f.mul(p.y ^ q.y, f.inv(p.x ^ q.x));
        let x3 = f.sq(l) ^ l ^ p.x ^ q.x ^ self.a;
        let y3 = f.mul(l, p.x ^ x3) ^ x3 ^ p.y;
        Pt { x: x3, y: y3, inf: false }
    }
    pub fn dbl(&self, p: &Pt) -> Pt {
        if p.inf || p.x == 0 {
            return O;
        }
        let f = self.f;
        let l = p.x ^ f.mul(p.y, f.inv(p.x));
        let x3 = f.sq(l) ^ l ^ self.a;
        let y3 = f.sq(p.x) ^ f.mul(l ^ 1, x3);
        Pt { x: x3, y: y3, inf: false }
    }
    #[inline]
    pub fn sub(&self, p: &Pt, q: &Pt) -> Pt {
        self.add(p, &self.neg(q))
    }
    pub fn mul(&self, k: u64, p: &Pt) -> Pt {
        let mut r = O;
        for i in (0..64).rev() {
            r = self.dbl(&r);
            if (k >> i) & 1 == 1 {
                r = self.add(&r, p);
            }
        }
        r
    }
    /// the point with y = x z, z^2 + z = x + a + b/x^2 (the "+" lift); None if x does not lift
    pub fn lift(&self, x: u32) -> Option<Pt> {
        let f = self.f;
        if x == 0 {
            return Some(Pt { x: 0, y: f.sqrt(self.b), inf: false });
        }
        let ix = f.inv(x);
        let c = x ^ self.a ^ f.mul(self.b, f.sq(ix));
        f.solve_as(c).map(|z| Pt { x, y: f.mul(x, z), inf: false })
    }
    /// does x != 0 lift (Tr(x + a + b/x^2) = 0)?
    pub fn lifts(&self, x: u32) -> bool {
        let f = self.f;
        let ix = f.inv(x);
        f.tr(x ^ self.a ^ f.mul(self.b, f.sq(ix))) == 0
    }
    /// #E(F_{2^N}) by counting abscissae
    pub fn order(&self) -> u64 {
        let f = self.f;
        let mut c = 2u64; // O and (0, sqrt b)
        for x in 1..=f.mask {
            if self.lifts(x) {
                c += 2;
            }
        }
        c
    }
    /// 2E membership by the trace criterion: P = O, or Tr(x_P) = Tr(a)
    #[inline]
    pub fn in_2e(&self, p: &Pt) -> bool {
        p.inf || self.f.tr(p.x) == self.f.tr(self.a)
    }
}
