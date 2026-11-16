//! E: y^2 + xy = x^3 + a x^2 + b over F_{Q^4}, a, b in F_16. Affine group law
//! as in research/lift_dense_factor_base_20261027/src/main.rs (commit b3cb8b1ea).
use crate::gf::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pt {
    pub x: E4,
    pub y: E4,
    pub inf: bool,
}
pub const O: Pt = Pt { x: ZERO, y: ZERO, inf: true };

pub struct Curve<'a> {
    pub f: &'a Fq4,
    pub a: E4,
    pub b: E4,
}

impl<'a> Curve<'a> {
    pub fn on(&self, p: &Pt) -> bool {
        if p.inf {
            return true;
        }
        let f = self.f;
        let x2 = f.sq(&p.x);
        let lhs = add(&f.sq(&p.y), &f.mul(&p.x, &p.y));
        let rhs = add(&add(&f.mul(&x2, &p.x), &f.mul(&self.a, &x2)), &self.b);
        lhs == rhs
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        if p.inf {
            *p
        } else {
            Pt { x: p.x, y: add(&p.y, &p.x), inf: false }
        }
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let f = self.f;
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        if p.x == q.x {
            if p.y == add(&q.y, &q.x) {
                return O;
            }
            // doubling (p == q), x != 0
            let lam = add(&p.x, &f.mul(&p.y, &f.inv(&p.x)));
            let x3 = add(&add(&f.sq(&lam), &lam), &self.a);
            let y3 = add(&f.sq(&p.x), &f.mul(&add(&lam, &ONE), &x3));
            return Pt { x: x3, y: y3, inf: false };
        }
        let lam = f.mul(&add(&p.y, &q.y), &f.inv(&add(&p.x, &q.x)));
        let x3 = add(&add(&add(&f.sq(&lam), &lam), &add(&p.x, &q.x)), &self.a);
        let y3 = add(&add(&f.mul(&lam, &add(&p.x, &x3)), &x3), &p.y);
        Pt { x: x3, y: y3, inf: false }
    }
    /// Tr(x + a + b/x^2): 0 iff x != 0 lifts to E(F_{Q^4}).
    pub fn lift_w(&self, x: &E4) -> E4 {
        let f = self.f;
        let ix = f.inv(x);
        add(&add(x, &self.a), &f.mul(&self.b, &f.sq(&ix)))
    }
    pub fn lift(&self, x: &E4) -> Option<Pt> {
        let z = self.f.artin_schreier(&self.lift_w(x))?;
        let p = Pt { x: *x, y: self.f.mul(x, &z), inf: false };
        debug_assert!(self.on(&p));
        Some(p)
    }
    pub fn lifts(&self, x: &E4) -> bool {
        self.f.tr_abs(&self.lift_w(x)) == 0
    }
}

/// #E(F_16) by brute force, with a, b in F_16.
pub fn order_f16(f: &Fq4, a: &E4, b: &E4) -> u64 {
    let mut n = 1u64; // O
    for xv in 0..16u8 {
        for yv in 0..16u8 {
            let (x, y) = (f.f16(xv), f.f16(yv));
            let x2 = f.sq(&x);
            let lhs = add(&f.sq(&y), &f.mul(&x, &y));
            let rhs = add(&add(&f.mul(&x2, &x), &f.mul(a, &x2)), b);
            if lhs == rhs {
                n += 1;
            }
        }
    }
    n
}

/// #E(F_{16^k}) from the Frobenius trace over F_16: 16^k + 1 - t_k.
pub fn order_ext(n16: u64, k: u32) -> u64 {
    let t = 17i128 - n16 as i128;
    let (mut t0, mut t1) = (2i128, t);
    for _ in 1..k {
        let t2 = t * t1 - 16 * t0;
        t0 = t1;
        t1 = t2;
    }
    let tk = if k == 0 { 2 } else { t1 };
    (16i128.pow(k) + 1 - tk) as u64
}
