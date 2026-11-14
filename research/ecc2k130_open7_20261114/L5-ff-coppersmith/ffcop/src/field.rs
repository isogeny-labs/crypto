//! F_{2^n} = F_2[t]/(f) on P, and the Koblitz curve y^2 + xy = x^3 + a x^2 + 1 (a = 0 here).
use crate::gf2x::{Rng, P};

pub struct Fld { pub n: usize, pub f: P, pub trmask: P }

impl Fld {
    pub fn new(n: usize) -> Fld {
        let f = crate::gf2x::find_modulus(n);
        let mut fl = Fld { n, f, trmask: P::zero() };
        let mut m = P::zero();
        for i in 0..n {
            let x = P::mono(i);
            let mut acc = x.clone();
            let mut y = x;
            for _ in 1..n { y = fl.sqr(&y); acc = acc.add(&y); }
            assert!(acc.deg() <= 0);
            if acc.is_one() { m.set_bit(i); }
        }
        fl.trmask = m;
        fl
    }
    pub fn red(&self, a: &P) -> P { a.rem(&self.f) }
    pub fn mul(&self, a: &P, b: &P) -> P { a.mul(b).rem(&self.f) }
    pub fn sqr(&self, a: &P) -> P { a.mul(a).rem(&self.f) }
    pub fn tr(&self, a: &P) -> u32 {
        let mut c = 0u32;
        for (i, w) in a.0.iter().enumerate() { if i < self.trmask.0.len() { c ^= (w & self.trmask.0[i]).count_ones() & 1; } }
        c
    }
    pub fn inv(&self, a: &P) -> P {
        assert!(!a.is_zero());
        let (mut u, mut v) = (a.clone(), self.f.clone());
        let (mut g1, mut g2) = (P::one(), P::zero());
        while !u.is_one() {
            let mut j = u.deg() - v.deg();
            if j < 0 { std::mem::swap(&mut u, &mut v); std::mem::swap(&mut g1, &mut g2); j = -j; }
            u.xor_shl(&v, j as usize);
            g1.xor_shl(&g2, j as usize);
        }
        g1.rem(&self.f)
    }
    /// half-trace, n odd: z with z^2 + z = a when Tr(a) = 0
    pub fn htr(&self, a: &P) -> P {
        let mut acc = a.clone();
        let mut h = a.clone();
        for _ in 0..(self.n - 1) / 2 { h = self.sqr(&self.sqr(&h)); acc = acc.add(&h); }
        acc
    }
    pub fn rand(&self, rng: &mut Rng) -> P { P::random(rng, self.n) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pt { pub x: P, pub y: P, pub inf: bool }

pub const A2: u64 = 0; // ECC2K-130 family: y^2 + xy = x^3 + 1

impl Fld {
    pub fn on_curve(&self, p: &Pt) -> bool {
        if p.inf { return true; }
        let lhs = self.sqr(&p.y).add(&self.mul(&p.x, &p.y));
        let rhs = self.mul(&self.sqr(&p.x), &p.x).add(&P::one());
        lhs == rhs
    }
    /// point with given x if one exists (a=0, b=1): z^2+z = x + 1/x^2, y = x z
    pub fn lift_x(&self, x: &P, sign: bool) -> Option<Pt> {
        if x.is_zero() { return None; }
        let ix = self.inv(x);
        let u = x.add(&self.sqr(&ix));
        if self.tr(&u) != 0 { return None; }
        let mut z = self.htr(&u);
        if sign { z = z.add(&P::one()); }
        let p = Pt { x: x.clone(), y: self.mul(x, &z), inf: false };
        debug_assert!(self.on_curve(&p));
        Some(p)
    }
    pub fn is_x(&self, x: &P) -> bool {
        if x.is_zero() { return false; }
        self.tr(x) == self.tr(&self.inv(x))
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        if p.inf { return q.clone(); }
        if q.inf { return p.clone(); }
        let lam;
        if p.x == q.x {
            if p.y != q.y || p.x.is_zero() { return Pt { x: P::zero(), y: P::zero(), inf: true }; }
            // doubling: lam = x + y/x
            lam = p.x.add(&self.mul(&p.y, &self.inv(&p.x)));
            let x3 = self.sqr(&lam).add(&lam);
            let y3 = self.sqr(&p.x).add(&self.mul(&lam.add(&P::one()), &x3));
            return Pt { x: x3, y: y3, inf: false };
        }
        lam = self.mul(&p.y.add(&q.y), &self.inv(&p.x.add(&q.x)));
        let x3 = self.sqr(&lam).add(&lam).add(&p.x).add(&q.x);
        let y3 = self.mul(&lam, &p.x.add(&x3)).add(&x3).add(&p.y);
        Pt { x: x3, y: y3, inf: false }
    }
    pub fn frob(&self, p: &Pt) -> Pt { Pt { x: self.sqr(&p.x), y: self.sqr(&p.y), inf: p.inf } }
    /// random point with deg x < bits
    pub fn rand_point(&self, rng: &mut Rng, bits: usize) -> Pt {
        loop {
            let x = P::random(rng, bits.min(self.n));
            if x.is_zero() { continue; }
            if let Some(p) = self.lift_x(&x, rng.next() & 1 == 1) { return p; }
        }
    }
    /// Semaev S3 for y^2+xy=x^3+a x^2+1: (x1x2+x1x3+x2x3)^2 + x1x2x3 + 1
    pub fn s3(&self, x1: &P, x2: &P, x3: &P) -> P {
        let a = self.mul(x1, x2).add(&self.mul(x1, x3)).add(&self.mul(x2, x3));
        self.sqr(&a).add(&self.mul(&self.mul(x1, x2), x3)).add(&P::one())
    }
}

impl Fld {
    pub fn sqrt(&self, a: &P) -> P { let mut x = a.clone(); for _ in 0..self.n - 1 { x = self.sqr(&x); } x }
    /// all roots in F_{2^n} of u0 + u1 Z + u2 Z^2 (not all zero)
    pub fn solve_quad(&self, u0: &P, u1: &P, u2: &P) -> Vec<P> {
        if u2.is_zero() {
            if u1.is_zero() { return vec![]; }
            return vec![self.mul(u0, &self.inv(u1))];
        }
        if u1.is_zero() { return vec![self.sqrt(&self.mul(u0, &self.inv(u2)))]; }
        let ba = self.mul(u1, &self.inv(u2));
        let w0 = self.mul(&self.mul(u2, u0), &self.inv(&self.sqr(u1)));
        if self.tr(&w0) != 0 { return vec![]; }
        let w = self.htr(&w0);
        let z1 = self.mul(&ba, &w);
        vec![z1.clone(), z1.add(&ba)]
    }
}
