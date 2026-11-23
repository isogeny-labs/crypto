use crate::fp::Fp;
use crate::poly::Poly;
use crate::rng::SplitMix64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Div {
    pub u: Poly,
    pub v: Poly,
}

pub struct Curve<'a> {
    pub fp: &'a Fp,
    pub f: Poly,
}

impl<'a> Curve<'a> {
    pub fn new(fp: &'a Fp, f: Poly) -> Curve<'a> {
        Curve { fp, f }
    }
    pub fn identity(&self) -> Div {
        Div { u: Poly::one(), v: Poly::zero() }
    }
    pub fn is_id(&self, d: &Div) -> bool {
        d.u.n == 1
    }
    pub fn neg(&self, d: &Div) -> Div {
        Div { u: d.u, v: self.fp.pneg(&d.v) }
    }
    pub fn add(&self, d1: &Div, d2: &Div) -> Div {
        let fp = self.fp;
        let (d0, e1, e2) = fp.pxgcd(&d1.u, &d2.u);
        let vs = fp.padd(&d1.v, &d2.v);
        let (d, c1, c2) = fp.pxgcd(&d0, &vs);
        let s1 = fp.pmul(&c1, &e1);
        let s2 = fp.pmul(&c1, &e2);
        let s3 = c2;
        let uu = fp.pmul(&d1.u, &d2.u);
        let dd = fp.pmul(&d, &d);
        let (mut u, r) = fp.pdivrem(&uu, &dd);
        assert!(r.is_zero(), "inexact u division");
        let t1 = fp.pmul(&fp.pmul(&s1, &d1.u), &d2.v);
        let t2 = fp.pmul(&fp.pmul(&s2, &d2.u), &d1.v);
        let vv = fp.padd(&fp.pmul(&d1.v, &d2.v), &self.f);
        let t3 = fp.pmul(&s3, &vv);
        let sum = fp.padd(&fp.padd(&t1, &t2), &t3);
        let (q, r) = fp.pdivrem(&sum, &d);
        assert!(r.is_zero(), "inexact v division");
        let mut v = fp.pmod(&q, &u);
        while u.n - 1 > 3 {
            let num = fp.psub(&self.f, &fp.pmul(&v, &v));
            let (u2, r) = fp.pdivrem(&num, &u);
            assert!(r.is_zero(), "inexact reduction division");
            let v2 = fp.pmod(&fp.pneg(&v), &u2);
            u = fp.pmonic(&u2);
            v = v2;
        }
        u = fp.pmonic(&u);
        v = fp.pmod(&v, &u);
        Div { u, v }
    }
    pub fn dbl(&self, d: &Div) -> Div {
        self.add(d, d)
    }
    pub fn mul(&self, n: i64, d: &Div) -> Div {
        if n < 0 {
            return self.mul(-n, &self.neg(d));
        }
        let mut r = self.identity();
        let mut started = false;
        let mut bit = 63;
        while bit >= 0 {
            if started {
                r = self.dbl(&r);
            }
            if (n >> bit) & 1 == 1 {
                r = if started { self.add(&r, d) } else { *d };
                started = true;
            }
            bit -= 1;
        }
        r
    }
    pub fn random_point(&self, rng: &mut SplitMix64) -> (u64, u64) {
        let fp = self.fp;
        loop {
            let x0 = rng.uniform(fp.p);
            let w = fp.peval(&self.f, x0);
            if w == 0 {
                return (x0, 0);
            }
            if fp.legendre(w) == 1 {
                let mut y0 = fp.sqrt(w);
                if rng.bit() == 1 {
                    y0 = fp.neg(y0);
                }
                return (x0, y0);
            }
        }
    }
    pub fn point_divisor(&self, x0: u64, y0: u64) -> Div {
        let fp = self.fp;
        Div { u: Poly::from(&[fp.neg(x0), 1], fp.p), v: Poly::from(&[y0], fp.p) }
    }
    /// Gives up (returns identity) after 100000 attempts so that tiny-p tests cannot hang.
    pub fn random_class(&self, rng: &mut SplitMix64) -> Div {
        for _ in 0..100000 {
            let mut d = self.identity();
            for _ in 0..3 {
                let (x0, y0) = self.random_point(rng);
                d = self.add(&d, &self.point_divisor(x0, y0));
            }
            if !self.is_id(&d) {
                return d;
            }
        }
        self.identity()
    }
}

/// Key of a reduced divisor: deg u (2 bits), 3 low coefficients of u and of v (13 bits each).
pub fn key(d: &Div) -> u128 {
    let mut k: u128 = (d.u.n - 1) as u128;
    for i in 0..3 {
        k = (k << 13) | d.u.get(i) as u128;
    }
    for i in 0..3 {
        k = (k << 13) | d.v.get(i) as u128;
    }
    k
}
