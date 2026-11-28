//! F_q and F_{q^2} = F_q[s]/(s^2 - nr) arithmetic (q an odd prime below 2^15)
//! and dense polynomials over F_{q^2}.  Local development helper; not a lab
//! benchmark primitive.

/// Element a + b*s of F_{q^2}.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct E {
    pub a: u32,
    pub b: u32,
}

pub const ZERO: E = E { a: 0, b: 0 };
pub const ONE: E = E { a: 1, b: 0 };

pub type Poly = Vec<E>;

#[derive(Clone, Debug)]
pub struct Fld {
    pub q: u32,
    pub nr: u32,
    sqrt_tab: Vec<u32>,
    inv_tab: Vec<u32>,
}

impl Fld {
    pub fn new(q: u32) -> Fld {
        let qq = q as u64;
        let mut sqrt_tab = vec![u32::MAX; q as usize];
        for x in 0..q {
            let s = ((x as u64 * x as u64) % qq) as usize;
            if sqrt_tab[s] == u32::MAX {
                sqrt_tab[s] = x;
            }
        }
        let nr = (1..q)
            .find(|&n| sqrt_tab[n as usize] == u32::MAX)
            .expect("odd prime q has a non-residue");
        let mut inv_tab = vec![0u32; q as usize];
        for x in 1..q {
            let mut r = 1u64;
            let mut b = x as u64;
            let mut e = q as u64 - 2;
            while e > 0 {
                if e & 1 == 1 {
                    r = r * b % qq;
                }
                b = b * b % qq;
                e >>= 1;
            }
            inv_tab[x as usize] = r as u32;
        }
        Fld {
            q,
            nr,
            sqrt_tab,
            inv_tab,
        }
    }

    // ---- F_q ----
    #[inline]
    pub fn qadd(&self, x: u32, y: u32) -> u32 {
        let s = x + y;
        if s >= self.q {
            s - self.q
        } else {
            s
        }
    }
    #[inline]
    pub fn qsub(&self, x: u32, y: u32) -> u32 {
        if x >= y {
            x - y
        } else {
            x + self.q - y
        }
    }
    #[inline]
    pub fn qneg(&self, x: u32) -> u32 {
        if x == 0 {
            0
        } else {
            self.q - x
        }
    }
    #[inline]
    pub fn qmul(&self, x: u32, y: u32) -> u32 {
        ((x as u64 * y as u64) % self.q as u64) as u32
    }
    #[inline]
    pub fn qinv(&self, x: u32) -> u32 {
        self.inv_tab[x as usize]
    }
    pub fn qsqrt(&self, x: u32) -> Option<u32> {
        let r = self.sqrt_tab[x as usize];
        if r == u32::MAX {
            None
        } else {
            Some(r)
        }
    }

    // ---- F_{q^2} ----
    pub fn embed(&self, a: u32) -> E {
        E { a, b: 0 }
    }
    #[inline]
    pub fn add(&self, x: E, y: E) -> E {
        E {
            a: self.qadd(x.a, y.a),
            b: self.qadd(x.b, y.b),
        }
    }
    #[inline]
    pub fn sub(&self, x: E, y: E) -> E {
        E {
            a: self.qsub(x.a, y.a),
            b: self.qsub(x.b, y.b),
        }
    }
    #[inline]
    pub fn neg(&self, x: E) -> E {
        E {
            a: self.qneg(x.a),
            b: self.qneg(x.b),
        }
    }
    #[inline]
    pub fn mul(&self, x: E, y: E) -> E {
        let q = self.q as u64;
        let aa = x.a as u64 * y.a as u64;
        let bb = (x.b as u64 * y.b as u64) % q;
        let a = (aa + bb * self.nr as u64) % q;
        let b = (x.a as u64 * y.b as u64 + x.b as u64 * y.a as u64) % q;
        E {
            a: a as u32,
            b: b as u32,
        }
    }
    pub fn scale(&self, x: E, c: u32) -> E {
        E {
            a: self.qmul(x.a, c),
            b: self.qmul(x.b, c),
        }
    }
    pub fn norm(&self, x: E) -> u32 {
        let aa = self.qmul(x.a, x.a);
        let bb = self.qmul(self.qmul(x.b, x.b), self.nr);
        self.qsub(aa, bb)
    }
    pub fn inv(&self, x: E) -> E {
        let n = self.norm(x);
        let ni = self.qinv(n);
        E {
            a: self.qmul(x.a, ni),
            b: self.qmul(self.qneg(x.b), ni),
        }
    }
    pub fn is_square(&self, x: E) -> bool {
        x == ZERO || self.sqrt_tab[self.norm(x) as usize] != u32::MAX
    }
    pub fn sqrt(&self, x: E) -> Option<E> {
        if x == ZERO {
            return Some(ZERO);
        }
        if x.b == 0 {
            if let Some(r) = self.qsqrt(x.a) {
                return Some(E { a: r, b: 0 });
            }
            let t = self.qmul(x.a, self.qinv(self.nr));
            let r = self.qsqrt(t)?;
            return Some(E { a: 0, b: r });
        }
        let r = self.qsqrt(self.norm(x))?;
        let half = self.qinv(2);
        let c1 = self.qmul(self.qadd(x.a, r), half);
        let c = if self.qsqrt(c1).is_some() {
            c1
        } else {
            self.qmul(self.qsub(x.a, r), half)
        };
        let x0 = self.qsqrt(c)?;
        if x0 == 0 {
            return None;
        }
        let y0 = self.qmul(x.b, self.qinv(self.qmul(2, x0)));
        Some(E { a: x0, b: y0 })
    }

    // ---- polynomials (coefficient 0 first, no trailing zeros) ----
    pub fn trim(&self, p: &mut Poly) {
        while p.last() == Some(&ZERO) {
            p.pop();
        }
    }
    pub fn padd(&self, p: &[E], r: &[E]) -> Poly {
        let n = p.len().max(r.len());
        let mut out = vec![ZERO; n];
        for (i, o) in out.iter_mut().enumerate() {
            let a = p.get(i).copied().unwrap_or(ZERO);
            let b = r.get(i).copied().unwrap_or(ZERO);
            *o = self.add(a, b);
        }
        self.trim(&mut out);
        out
    }
    pub fn psub(&self, p: &[E], r: &[E]) -> Poly {
        let n = p.len().max(r.len());
        let mut out = vec![ZERO; n];
        for (i, o) in out.iter_mut().enumerate() {
            let a = p.get(i).copied().unwrap_or(ZERO);
            let b = r.get(i).copied().unwrap_or(ZERO);
            *o = self.sub(a, b);
        }
        self.trim(&mut out);
        out
    }
    pub fn pneg(&self, p: &[E]) -> Poly {
        p.iter().map(|&c| self.neg(c)).collect()
    }
    pub fn pmul(&self, p: &[E], r: &[E]) -> Poly {
        if p.is_empty() || r.is_empty() {
            return Vec::new();
        }
        let mut out = vec![ZERO; p.len() + r.len() - 1];
        for (i, &a) in p.iter().enumerate() {
            if a == ZERO {
                continue;
            }
            for (j, &b) in r.iter().enumerate() {
                out[i + j] = self.add(out[i + j], self.mul(a, b));
            }
        }
        self.trim(&mut out);
        out
    }
    pub fn pscale(&self, p: &[E], c: E) -> Poly {
        let mut out: Poly = p.iter().map(|&x| self.mul(x, c)).collect();
        self.trim(&mut out);
        out
    }
    /// Quotient and remainder; `d` must be non-zero.
    pub fn pdivrem(&self, n: &[E], d: &[E]) -> (Poly, Poly) {
        assert!(!d.is_empty(), "division by zero polynomial");
        let mut rem = n.to_vec();
        self.trim(&mut rem);
        if rem.len() < d.len() {
            return (Vec::new(), rem);
        }
        let linv = self.inv(*d.last().unwrap());
        let mut quo = vec![ZERO; rem.len() - d.len() + 1];
        while rem.len() >= d.len() && !rem.is_empty() {
            let shift = rem.len() - d.len();
            let c = self.mul(*rem.last().unwrap(), linv);
            quo[shift] = c;
            for (i, &di) in d.iter().enumerate() {
                rem[shift + i] = self.sub(rem[shift + i], self.mul(c, di));
            }
            rem.pop();
            self.trim(&mut rem);
        }
        self.trim(&mut quo);
        (quo, rem)
    }
    pub fn pmonic(&self, p: &[E]) -> Poly {
        match p.last() {
            None => Vec::new(),
            Some(&l) => self.pscale(p, self.inv(l)),
        }
    }
    /// (g, s, t) with s*a + t*b = g, g monic (or empty when a = b = 0).
    pub fn pxgcd(&self, a: &[E], b: &[E]) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
        let (mut s0, mut s1): (Poly, Poly) = (vec![ONE], Vec::new());
        let (mut t0, mut t1): (Poly, Poly) = (Vec::new(), vec![ONE]);
        while !r1.is_empty() {
            let (qt, rem) = self.pdivrem(&r0, &r1);
            let s2 = self.psub(&s0, &self.pmul(&qt, &s1));
            let t2 = self.psub(&t0, &self.pmul(&qt, &t1));
            r0 = r1;
            r1 = rem;
            s0 = s1;
            s1 = s2;
            t0 = t1;
            t1 = t2;
        }
        if let Some(&l) = r0.last() {
            let li = self.inv(l);
            return (
                self.pscale(&r0, li),
                self.pscale(&s0, li),
                self.pscale(&t0, li),
            );
        }
        (r0, s0, t0)
    }
    pub fn peval(&self, p: &[E], x: E) -> E {
        let mut acc = ZERO;
        for &c in p.iter().rev() {
            acc = self.add(self.mul(acc, x), c);
        }
        acc
    }
    pub fn pderiv(&self, p: &[E]) -> Poly {
        let mut out: Poly = Vec::new();
        for (i, &c) in p.iter().enumerate().skip(1) {
            out.push(self.scale(c, (i as u32) % self.q));
        }
        self.trim(&mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_axioms_and_sqrt() {
        for &q in &[7u32, 67, 131] {
            let f = Fld::new(q);
            let mut sq = 0;
            for a in 0..q.min(40) {
                for b in 0..q.min(40) {
                    let x = E { a, b };
                    if x != ZERO {
                        assert_eq!(f.mul(x, f.inv(x)), ONE);
                    }
                    let y = f.mul(x, x);
                    let r = f.sqrt(y).expect("square has a root");
                    assert_eq!(f.mul(r, r), y);
                    assert!(f.is_square(y));
                    if f.is_square(x) {
                        sq += 1;
                        let r = f.sqrt(x).unwrap();
                        assert_eq!(f.mul(r, r), x);
                    } else {
                        assert!(f.sqrt(x).is_none());
                    }
                }
            }
            assert!(sq > 0);
        }
    }

    #[test]
    fn poly_divrem_and_xgcd() {
        let f = Fld::new(67);
        let p: Poly = (1..7)
            .map(|i| E {
                a: i * 5 % 67,
                b: i * 3 % 67,
            })
            .collect();
        let d: Poly = vec![E { a: 3, b: 1 }, E { a: 0, b: 2 }, E { a: 1, b: 0 }];
        let (qt, rem) = f.pdivrem(&p, &d);
        assert!(rem.len() < d.len());
        assert_eq!(f.padd(&f.pmul(&qt, &d), &rem), p);
        let (g, s, t) = f.pxgcd(&p, &d);
        assert_eq!(g, vec![ONE]);
        let lhs = f.padd(&f.pmul(&s, &p), &f.pmul(&t, &d));
        assert_eq!(lhs, g);
    }
}
