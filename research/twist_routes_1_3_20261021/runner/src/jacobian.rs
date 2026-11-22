//! Genus-2 Cantor arithmetic after an explicit branch-point change of chart.
//! The original sextic keeps its two rational infinity points; arithmetic is
//! carried into a monic odd-degree model over the declared F_(p^6), with
//! pullback and norm maps transported through that chart. No real-model
//! Mumford formulas are applied to the sextic.
use crate::toy_field::*;
use serde::Serialize;
pub type Poly = Vec<S>;
#[derive(Copy, Clone)]
pub struct Ring(pub Sext);
impl Ring {
    pub fn trim(self, mut a: Poly) -> Poly {
        while a.last() == Some(&Z) {
            a.pop();
        }
        a
    }
    pub fn add(self, a: &[S], b: &[S]) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| self.0.add(*a.get(i).unwrap_or(&Z), *b.get(i).unwrap_or(&Z)))
                .collect(),
        )
    }
    pub fn neg(self, a: &[S]) -> Poly {
        a.iter().map(|x| self.0.neg(*x)).collect()
    }
    pub fn sub(self, a: &[S], b: &[S]) -> Poly {
        self.add(a, &self.neg(b))
    }
    pub fn scale(self, a: &[S], s: S) -> Poly {
        self.trim(a.iter().map(|x| self.0.mul(*x, s)).collect())
    }
    pub fn mul(self, a: &[S], b: &[S]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut z = vec![Z; a.len() + b.len() - 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                z[i + j] = self.0.add(z[i + j], self.0.mul(a[i], b[j]));
            }
        }
        self.trim(z)
    }
    pub fn monic(self, a: &[S]) -> Poly {
        assert!(!a.is_empty());
        self.scale(a, self.0.inv(*a.last().unwrap()))
    }
    pub fn dr(self, a: &[S], b: &[S]) -> (Poly, Poly) {
        assert!(!b.is_empty());
        let mut r = self.trim(a.to_vec());
        let mut q = vec![Z; a.len().saturating_sub(b.len()) + 1];
        let bi = self.0.inv(*b.last().unwrap());
        while r.len() >= b.len() {
            let i = r.len() - b.len();
            let c = self.0.mul(*r.last().unwrap(), bi);
            q[i] = c;
            for j in 0..b.len() {
                r[i + j] = self.0.sub(r[i + j], self.0.mul(c, b[j]));
            }
            r = self.trim(r);
        }
        (self.trim(q), r)
    }
    pub fn rem(self, a: &[S], b: &[S]) -> Poly {
        self.dr(a, b).1
    }
    pub fn exact(self, a: &[S], b: &[S]) -> Poly {
        let (q, r) = self.dr(a, b);
        assert!(r.is_empty(), "nonexact polynomial division");
        q
    }
    pub fn egcd(self, a: &[S], b: &[S]) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
        let (mut s0, mut s1) = (vec![ONE], vec![]);
        let (mut t0, mut t1) = (vec![], vec![ONE]);
        while !r1.is_empty() {
            let (q, r) = self.dr(&r0, &r1);
            r0 = r1;
            r1 = r;
            let s = self.sub(&s0, &self.mul(&q, &s1));
            s0 = s1;
            s1 = s;
            let t = self.sub(&t0, &self.mul(&q, &t1));
            t0 = t1;
            t1 = t;
        }
        let c = self.0.inv(*r0.last().unwrap());
        (self.scale(&r0, c), self.scale(&s0, c), self.scale(&t0, c))
    }
    pub fn inverse(self, a: &[S], m: &[S]) -> Poly {
        let (g, s, _) = self.egcd(a, m);
        assert_eq!(g, vec![ONE], "noninvertible residue");
        self.rem(&s, m)
    }
    pub fn eval(self, a: &[S], x: S) -> S {
        a.iter()
            .rev()
            .fold(Z, |v, c| self.0.add(self.0.mul(v, x), *c))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize)]
pub struct Div {
    pub u: Poly,
    pub v: Poly,
}
impl Div {
    pub fn zero() -> Self {
        Self {
            u: vec![ONE],
            v: vec![],
        }
    }
}
#[derive(Clone)]
pub struct Chart {
    pub k: Sext,
    pub c: u64,
    pub f: Poly,
    pub root: S,
    pub lambda: S,
    pub ad: u64,
    pub bd: u64,
}
fn binom(n: usize, k: usize) -> u64 {
    (0..k).fold(1u64, |r, i| r * (n - i) as u64 / (i + 1) as u64)
}
impl Chart {
    pub fn new(e: Quad, c: u64) -> Self {
        let k = Sext::new(e);
        let cc = k.c(c);
        let alpha = S([0, 1, 0, 0, 0, 0]);
        let root = k.sqrt(k.sub(alpha, cc)).unwrap();
        let fc = e.rhs(Q(c, 0)).0;
        assert_ne!(fc, 0);
        let a2 = (3 * c * c + e.a) % e.p;
        let ad = (9 * c % e.p * fc + e.p - a2 * a2 % e.p) % e.p * inv(3, e.p) % e.p;
        let bd = (2 * a2 % e.p * a2 % e.p * a2 + 27 * fc % e.p * fc + e.p
            - 27 * a2 % e.p * c % e.p * fc % e.p)
            % e.p
            * inv(27 % e.p, e.p)
            % e.p;
        let old = [fc, 0, a2, 0, 3 * c % e.p, 0, 1];
        let lambda = k.mul(
            k.mul(k.c(2), root),
            k.add(k.mul(k.c(3), k.mul(alpha, alpha)), k.c(e.a)),
        );
        assert_ne!(lambda, Z);
        let li = k.inv(lambda);
        let l4 = k.pow(lambda, 4);
        let mut g = vec![Z; 7];
        for i in 0..7 {
            for j in 0..=i {
                g[6 - j] = k.add(
                    g[6 - j],
                    k.mul(
                        k.c(old[i] * binom(i, j) % e.p),
                        k.pow(root, (i - j) as u128),
                    ),
                );
            }
        }
        assert_eq!(g[6], Z);
        assert_eq!(g[5], lambda);
        let f = Ring(k).trim(
            (0..6)
                .map(|i| k.mul(g[i], k.mul(l4, k.pow(li, i as u128))))
                .collect(),
        );
        assert_eq!(f.len(), 6);
        assert_eq!(f[5], ONE);
        let h = Self {
            k,
            c,
            f,
            root,
            lambda,
            ad,
            bd,
        };
        h.valid(&Div::zero());
        h
    }
    pub fn valid(&self, d: &Div) {
        let r = Ring(self.k);
        assert!(!d.u.is_empty() && d.u.len() <= 3 && d.u.last() == Some(&ONE));
        assert!(d.v.len() < d.u.len());
        assert!(r.rem(&r.sub(&r.mul(&d.v, &d.v), &self.f), &d.u).is_empty());
    }
    pub(crate) fn reduce(&self, mut u: Poly, mut v: Poly) -> Div {
        let r = Ring(self.k);
        u = r.monic(&u);
        v = r.rem(&v, &u);
        while u.len() > 3 {
            let next = r.monic(&r.exact(&r.sub(&self.f, &r.mul(&v, &v)), &u));
            assert!(next.len() < u.len());
            u = next;
            v = r.rem(&r.neg(&v), &u);
        }
        let d = Div { u, v };
        self.valid(&d);
        d
    }
    pub fn neg(&self, d: &Div) -> Div {
        Div {
            u: d.u.clone(),
            v: Ring(self.k).neg(&d.v),
        }
    }
    pub fn add(&self, x: &Div, y: &Div) -> Div {
        self.valid(x);
        self.valid(y);
        let r = Ring(self.k);
        let (d1, e1, e2) = r.egcd(&x.u, &y.u);
        let (d, c1, c2) = r.egcd(&d1, &r.add(&x.v, &y.v));
        let s1 = r.mul(&c1, &e1);
        let s2 = r.mul(&c1, &e2);
        let u = r.exact(&r.mul(&x.u, &y.u), &r.mul(&d, &d));
        let numerator = r.add(
            &r.add(
                &r.mul(&r.mul(&s1, &x.u), &y.v),
                &r.mul(&r.mul(&s2, &y.u), &x.v),
            ),
            &r.mul(&c2, &r.add(&r.mul(&x.v, &y.v), &self.f)),
        );
        let v = r.rem(&r.exact(&numerator, &d), &u);
        self.reduce(u, v)
    }
    pub fn scalar(&self, d: &Div, mut n: u128) -> Div {
        let mut x = d.clone();
        let mut y = Div::zero();
        while n != 0 {
            if n & 1 != 0 {
                y = self.add(&y, &x);
            }
            x = self.add(&x, &x);
            n >>= 1;
        }
        y
    }
    pub fn point(&self, u: S, v: S) -> Div {
        let k = self.k;
        if u == self.root {
            assert_eq!(v, Z);
            return Div::zero();
        }
        let x = k.div(self.lambda, k.sub(u, self.root));
        let y = k.div(k.mul(v, k.pow(x, 3)), self.lambda);
        assert_eq!(k.mul(y, y), Ring(k).eval(&self.f, x));
        let d = Div {
            u: vec![k.neg(x), ONE],
            v: Ring(k).trim(vec![y]),
        };
        self.valid(&d);
        d
    }
    fn lift_quadratic(&self, t: S, ypoly: impl FnOnce(&[S]) -> Poly) -> Div {
        let k = self.k;
        let r = Ring(k);
        let u = r.monic(&[
            k.mul(self.lambda, self.lambda),
            k.mul(k.c(2), k.mul(self.root, self.lambda)),
            k.sub(k.mul(self.root, self.root), t),
        ]);
        let v = r.rem(&ypoly(&u), &u);
        self.reduce(u, v)
    }
    pub fn pull_e(&self, p: P) -> Div {
        let Some((x, y)) = p else { return Div::zero() };
        assert_eq!(x.1, 0);
        assert_eq!(y.1, 0);
        let k = self.k;
        self.lift_quadratic(k.c((x.0 + k.p - self.c) % k.p), |_| {
            vec![Z, Z, Z, k.div(k.c(y.0), self.lambda)]
        })
    }
    pub fn pull_d(&self, p: P) -> Div {
        let Some((x, y)) = p else { return Div::zero() };
        assert_eq!(x.1, 0);
        assert_eq!(y.1, 0);
        let k = self.k;
        let r = Ring(k);
        let fc = (self.c * self.c % k.p * self.c + k.a * self.c + k.b) % k.p;
        let a2 = (3 * self.c * self.c + k.a) % k.p;
        let shift = a2 * inv(3, k.p) % k.p;
        if x.0 == shift {
            assert!(y.0 == fc || y.0 == (k.p - fc) % k.p);
            let v = k.mul(
                k.pow(self.lambda, 2),
                k.c(if y.0 == fc { 1 } else { k.p - 1 }),
            );
            let d = Div {
                u: vec![Z, ONE],
                v: vec![v],
            };
            return self.add(&d, &d);
        }
        let t = fc * inv((x.0 + k.p - shift) % k.p, k.p) % k.p;
        let z = y.0 * t % k.p * t % k.p * inv(fc, k.p) % k.p;
        self.lift_quadratic(k.c(t), |u| {
            let denominator = [self.lambda, self.root];
            let y = vec![Z, Z, Z, Z, k.div(k.c(z), self.lambda)];
            r.rem(&r.mul(&y, &r.inverse(&denominator, u)), u)
        })
    }
    fn mapped(&self, x: S, y: S, second: bool) -> SP {
        let k = self.k;
        let fc = k.c((self.c * self.c % k.p * self.c + k.a * self.c + k.b) % k.p);
        let shift = k.c((3 * self.c * self.c + k.a) % k.p * inv(3, k.p) % k.p);
        if x == Z {
            return if second {
                Some((shift, k.mul(fc, k.div(y, k.pow(self.lambda, 2)))))
            } else {
                None
            };
        }
        let u = k.add(self.root, k.div(self.lambda, x));
        let v = k.div(k.mul(self.lambda, y), k.pow(x, 3));
        let t = k.mul(u, u);
        if second {
            if t == Z {
                return None;
            }
            Some((
                k.add(k.div(fc, t), shift),
                k.div(k.mul(fc, k.mul(u, v)), k.mul(t, t)),
            ))
        } else {
            Some((k.add(t, k.c(self.c)), v))
        }
    }
    fn raw_norm(&self, u: &[S], v: &[S], second: bool) -> SP {
        let k = self.k;
        let r = Ring(k);
        let a = k.c(if second { self.ad } else { k.a });
        if u.len() == 1 {
            return None;
        }
        if u.len() == 2 {
            return self.mapped(k.neg(u[0]), r.eval(v, k.neg(u[0])), second);
        }
        for x in [Z, k.neg(k.div(self.lambda, self.root))] {
            if r.eval(u, x) == Z {
                let rest = r.exact(u, &[k.neg(x), ONE]);
                let left = self.mapped(x, r.eval(v, x), second);
                return k.pa(left, self.raw_norm(&rest, &r.rem(v, &rest), second), a);
            }
        }
        let xi = r.inverse(&[Z, ONE], u);
        let old_u = r.add(&[self.root], &r.scale(&xi, self.lambda));
        let old_v = r.rem(
            &r.scale(&r.mul(v, &r.mul(&r.mul(&xi, &xi), &xi)), self.lambda),
            u,
        );
        let t = r.rem(&r.mul(&old_u, &old_u), u);
        let (xp, yp) = if second {
            let fc = k.c((self.c * self.c % k.p * self.c + k.a * self.c + k.b) % k.p);
            let ti = r.inverse(&t, u);
            let shift = k.c((3 * self.c * self.c + k.a) % k.p * inv(3, k.p) % k.p);
            (
                r.add(&r.scale(&ti, fc), &[shift]),
                r.rem(
                    &r.scale(&r.mul(&r.mul(&old_u, &old_v), &r.mul(&ti, &ti)), fc),
                    u,
                ),
            )
        } else {
            (r.add(&t, &[k.c(self.c)]), old_v)
        };
        let (x0, x1) = (*xp.first().unwrap_or(&Z), *xp.get(1).unwrap_or(&Z));
        let (y0, y1) = (*yp.first().unwrap_or(&Z), *yp.get(1).unwrap_or(&Z));
        if x1 == Z {
            if y1 == Z {
                return k.pa(Some((x0, y0)), Some((x0, y0)), a);
            }
            assert_eq!(k.sub(k.mul(k.c(2), y0), k.mul(y1, u[1])), Z);
            return None;
        }
        let m = k.div(y1, x1);
        let x3 = k.add(k.sub(k.mul(m, m), k.mul(k.c(2), x0)), k.mul(x1, u[1]));
        Some((x3, k.sub(k.mul(m, k.sub(x0, x3)), y0)))
    }
    pub fn norm(&self, d: &Div, second: bool) -> SP {
        self.valid(d);
        let k = self.k;
        let raw = self.raw_norm(&d.u, &d.v, second);
        if (d.u.len() - 1) % 2 == 0 {
            return raw;
        }
        let base = if second {
            let h = k.mul(self.root, self.root);
            let fc = k.c((self.c * self.c % k.p * self.c + k.a * self.c + k.b) % k.p);
            let shift = k.c((3 * self.c * self.c + k.a) % k.p * inv(3, k.p) % k.p);
            Some((k.add(k.div(fc, h), shift), Z))
        } else {
            Some((k.add(k.mul(self.root, self.root), k.c(self.c)), Z))
        };
        k.pa(raw, k.pn(base), k.c(if second { self.ad } else { k.a }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transport_and_exceptional_infinity_charts() {
        let e = Quad::new(59, 1, 13);
        let h = Chart::new(e, 0);
        let d = Quad::new(59, h.ad, h.bd);
        let k = h.k;
        for p in e.rational() {
            let j = h.pull_e(p);
            assert_eq!(
                h.norm(&j, false),
                e.pm(p, 2).map(|(x, y)| (k.lift(x), k.lift(y)))
            );
            assert_eq!(h.norm(&j, true), None);
            assert_eq!(h.add(&j, &h.neg(&j)), Div::zero());
        }
        for p in d.rational() {
            let j = h.pull_d(p);
            assert_eq!(
                h.norm(&j, true),
                d.pm(p, 2).map(|(x, y)| (k.lift(x), k.lift(y)))
            );
            assert_eq!(h.norm(&j, false), None);
        }
    }
}
