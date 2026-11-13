//! Jacobian arithmetic for `y² = f(x)`, `deg f = 2g + 1`, odd characteristic:
//! reduced Mumford divisors `(u, v)` (`u` monic, `deg v < deg u ≤ g`,
//! `u | f − v²`), Cantor composition and reduction.

use crate::fp::Fp;
use crate::poly::Poly;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Div {
    pub u: Poly,
    pub v: Poly,
}

#[derive(Clone, Debug)]
pub struct Curve {
    pub fp: Fp,
    pub f: Poly,
    pub g: usize,
}

impl Curve {
    /// `f` must be monic-or-not squarefree of degree `2g + 1`; checked.
    pub fn new(fp: Fp, f: Poly) -> Self {
        let d = f.deg();
        assert!(d >= 5 && d % 2 == 1, "Curve: deg f must be 2g+1 >= 5");
        let df = f.deriv(&fp);
        assert_eq!(Poly::gcd(&f, &df, &fp).deg(), 0, "Curve: f not squarefree");
        Curve {
            fp,
            f,
            g: ((d - 1) / 2) as usize,
        }
    }

    #[inline]
    pub fn zero(&self) -> Div {
        Div {
            u: Poly::one(),
            v: Poly::zero(),
        }
    }

    #[inline]
    pub fn is_zero(&self, d: &Div) -> bool {
        d.u.deg() == 0
    }

    #[inline]
    pub fn neg(&self, d: &Div) -> Div {
        Div {
            u: d.u,
            v: d.v.neg(&self.fp),
        }
    }

    /// The degree-1 place `(x0, y0)` as `[P] − [∞]`.
    pub fn place(&self, x0: u64, y0: u64) -> Div {
        debug_assert_eq!(
            self.fp.mul(y0, y0),
            self.f.eval(x0, &self.fp),
            "place not on the curve"
        );
        Div {
            u: Poly::x_minus(x0, &self.fp),
            v: Poly::constant(y0 % self.fp.p),
        }
    }

    /// Mumford invariants hold and the divisor is reduced.
    pub fn is_valid(&self, d: &Div) -> bool {
        let fp = &self.fp;
        if d.u.lc() != 1 || d.u.deg() > self.g as isize || d.v.deg() >= d.u.deg() {
            return false;
        }
        self.f.sub(&d.v.mul(&d.v, fp), fp).rem(&d.u, fp).is_zero()
    }

    /// Reduce a (semi-reduced) `(u, v)` to the unique reduced representative.
    pub fn reduce(&self, u: Poly, v: Poly) -> Div {
        let fp = &self.fp;
        let mut u = u.monic(fp);
        let mut v = v.rem(&u, fp);
        while u.deg() > self.g as isize {
            let num = self.f.sub(&v.mul(&v, fp), fp);
            let u2 = num.div_exact(&u, fp).monic(fp);
            v = v.neg(fp).rem(&u2, fp);
            u = u2;
        }
        Div { u, v }
    }

    /// Cantor's composition followed by reduction.
    pub fn add(&self, a: &Div, b: &Div) -> Div {
        if self.is_zero(a) {
            return *b;
        }
        if self.is_zero(b) {
            return *a;
        }
        let fp = &self.fp;
        let (d1, e1, e2) = Poly::xgcd(&a.u, &b.u, fp);
        let vs = a.v.add(&b.v, fp);
        let (d, c1, c2) = Poly::xgcd(&d1, &vs, fp);
        let s1 = c1.mul(&e1, fp);
        let s2 = c1.mul(&e2, fp);
        let s3 = c2;
        let u = if d.deg() == 0 {
            a.u.mul(&b.u, fp)
        } else {
            a.u.mul(&b.u, fp).div_exact(&d.mul(&d, fp), fp)
        };
        let t = s1
            .mul(&a.u, fp)
            .mul(&b.v, fp)
            .add(&s2.mul(&b.u, fp).mul(&a.v, fp), fp)
            .add(&s3.mul(&a.v.mul(&b.v, fp).add(&self.f, fp), fp), fp);
        let v = if d.deg() == 0 { t } else { t.div_exact(&d, fp) };
        self.reduce(u, v.rem(&u, fp))
    }

    /// Add the degree-1 place `(x0, y0)`.  When `u(x0) ≠ 0` the composition
    /// is a CRT step (`gcd = 1`), done directly; otherwise it falls back to
    /// [`Curve::add`].
    pub fn add_place(&self, a: &Div, x0: u64, y0: u64) -> Div {
        let fp = &self.fp;
        let ux0 = a.u.eval(x0, fp);
        if ux0 == 0 {
            return self.add(a, &self.place(x0, y0));
        }
        // v ≡ a.v (mod a.u), v ≡ y0 (mod x − x0):
        // v = a.v + a.u · (y0 − a.v(x0)) / a.u(x0).
        let k = fp.mul(fp.sub(y0, a.v.eval(x0, fp)), fp.inv(ux0));
        let u = a.u.mul(&Poly::x_minus(x0, fp), fp);
        let v = a.v.add(&a.u.scale(k, fp), fp);
        self.reduce(u, v)
    }

    pub fn mul(&self, d: &Div, k: u64) -> Div {
        let mut r = self.zero();
        let mut b = *d;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(&b, &b);
            }
        }
        r
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::rng::Rng;

    /// The thread's curve families (round seven / eight).
    pub fn thread_curve(p: u64, c: u64, g: usize) -> Option<Curve> {
        let fp = Fp::new(p);
        let cs: Vec<u64> = match g {
            2 => vec![c % p, 1, 2, 3, 0, 1],
            3 => vec![1, c % p, 0, 1, 0, 0, 0, 1],
            4 => vec![1, c % p, 0, 1, 0, 0, 0, 0, 0, 1],
            _ => return None,
        };
        let f = Poly::from_coeffs(&cs);
        if Poly::gcd(&f, &f.deriv(&fp), &fp).deg() != 0 {
            return None;
        }
        Some(Curve::new(fp, f))
    }

    pub fn random_place(c: &Curve, rng: &mut Rng) -> (u64, u64) {
        loop {
            let x = rng.below(c.fp.p);
            if let Some(y) = c.fp.sqrt(c.f.eval(x, &c.fp)) {
                return (x, y);
            }
        }
    }

    pub fn random_div(c: &Curve, rng: &mut Rng) -> Div {
        let mut d = c.zero();
        for _ in 0..c.g {
            let (x, y) = random_place(c, rng);
            d = c.add(&d, &c.place(x, y));
        }
        d
    }

    #[test]
    fn group_law_g1() {
        for g in 2..=4 {
            for &p in &[13u64, 17, 101, 1009, 65537] {
                let Some(c) = (1..40).find_map(|cc| thread_curve(p, cc, g)) else {
                    continue;
                };
                let mut rng = Rng::new(p * 31 + g as u64);
                for _ in 0..60 {
                    let a = random_div(&c, &mut rng);
                    let b = random_div(&c, &mut rng);
                    let d = random_div(&c, &mut rng);
                    for x in [&a, &b, &d] {
                        assert!(c.is_valid(x), "invalid divisor g={g} p={p}");
                    }
                    let ab = c.add(&a, &b);
                    assert!(c.is_valid(&ab));
                    assert_eq!(ab, c.add(&b, &a), "commutativity g={g} p={p}");
                    assert_eq!(
                        c.add(&ab, &d),
                        c.add(&a, &c.add(&b, &d)),
                        "associativity g={g} p={p}"
                    );
                    assert!(c.is_zero(&c.add(&a, &c.neg(&a))), "inverse g={g} p={p}");
                    let aa = c.add(&a, &a);
                    assert_eq!(c.mul(&a, 2), aa);
                    assert_eq!(c.mul(&a, 5), c.add(&c.add(&aa, &aa), &a));
                    // Fast degree-1 path equals generic Cantor, including
                    // u(x0) = 0 (adding a place already in the support, or
                    // its opposite).
                    let (x0, y0) = random_place(&c, &mut rng);
                    assert_eq!(c.add_place(&a, x0, y0), c.add(&a, &c.place(x0, y0)));
                    let s = c.place(x0, y0);
                    let a2 = c.add(&a, &s);
                    assert_eq!(c.add_place(&a2, x0, y0), c.add(&a2, &s));
                    assert_eq!(
                        c.add_place(&a2, x0, c.fp.neg(y0)),
                        c.add(&a2, &c.neg(&s))
                    );
                }
            }
        }
    }
}
