//! E_{a,1}: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n} (ECC2K-130 is a = 0). Affine group law, canonical point keys.

use crate::field::Fq;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pt {
    Inf,
    Aff(u32, u32),
}

pub struct Curve<'a> {
    pub f: &'a Fq,
    pub a: u32,
    pub b: u32,
}

impl<'a> Curve<'a> {
    pub fn koblitz(f: &'a Fq, a: u32) -> Curve<'a> {
        assert!(a <= 1);
        Curve { f, a, b: 1 }
    }
    pub fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(x, x ^ y),
        }
    }
    pub fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::Aff(x, y) => {
                let f = self.f;
                let lhs = f.sq(y) ^ f.mul(x, y);
                let rhs = f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b;
                lhs == rhs
            }
        }
    }
    #[inline]
    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::Aff(x1, y1), Pt::Aff(x2, y2)) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == 0 {
                        return Pt::Inf;
                    }
                    let l = x1 ^ f.mul(y1, f.inv(x1));
                    let x3 = f.sq(l) ^ l ^ self.a;
                    let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                    Pt::Aff(x3, y3)
                } else {
                    let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                    let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ self.a;
                    let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                    Pt::Aff(x3, y3)
                }
            }
        }
    }
    pub fn mul(&self, p: Pt, mut k: u64) -> Pt {
        let (mut r, mut b) = (Pt::Inf, p);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    /// A y with (x,y) on the curve, or None. For x = 0 the 2-torsion point T = (0, sqrt b).
    pub fn lift_x(&self, x: u32) -> Option<u32> {
        let f = self.f;
        if x == 0 {
            return Some(f.sqrt(self.b));
        }
        let c = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        if f.tr(c) == 1 {
            return None;
        }
        Some(f.mul(x, f.ht(c)))
    }
    /// #E(F_q) by trace counting (O, the point with x = 0, and two points per good x != 0).
    pub fn count(&self) -> u32 {
        let f = self.f;
        let mut c = 2u32;
        for x in 1..f.q {
            if f.tr(x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)))) == 0 {
                c += 2;
            }
        }
        c
    }
    /// The 2-torsion point T = (0, sqrt b).
    pub fn t2(&self) -> Pt {
        Pt::Aff(0, self.f.sqrt(self.b))
    }
    /// iota(x) = sqrt(b)/x, the x-map of P -> P + T (x != 0).
    pub fn iota_x(&self, x: u32) -> u32 {
        self.f.mul(self.f.sqrt(self.b), self.f.inv(x))
    }
    /// Frobenius endomorphism sigma(P) = (x^2, y^2) (valid for b in F_2 as a map E -> E).
    pub fn frob_pt(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(self.f.sq(x), self.f.sq(y)),
        }
    }
    /// Size of the key space of `key`.
    pub fn key_space(&self) -> usize {
        2 * self.f.q as usize + 1
    }
    /// Canonical dense key: 2x + [y > x^y] for affine points (P and -P get different keys), 2q for O.
    #[inline]
    pub fn key(&self, p: Pt) -> usize {
        match p {
            Pt::Inf => 2 * self.f.q as usize,
            Pt::Aff(x, y) => 2 * x as usize + (y > (x ^ y)) as usize,
        }
    }
    /// All points with x in `xs` (x = 0 gives the single point T; any other good x gives P and -P).
    pub fn points_for_xs(&self, xs: &[u32]) -> Vec<Pt> {
        let mut v = Vec::new();
        for &x in xs {
            if let Some(y) = self.lift_x(x) {
                v.push(Pt::Aff(x, y));
                if x != 0 {
                    v.push(Pt::Aff(x, x ^ y));
                }
            }
        }
        v
    }
}
