//! Byte-copied gaudry_cubic Fp helpers and polynomial-ring items for scoped qualification.
//! The original items are crate-private in the pinned Root41 dependency.
//! No algorithm or counter convention is changed; compare extracted source bytes.
use super::residual_walk::inv_mod;
use rand::{rngs::StdRng, Rng};
use std::cell::Cell;

#[inline]
pub(crate) fn mm(a: u64, b: u64, p: u64) -> u64 {
    super::jv_bitops::hook_mul(p);
    ((a as u128 * b as u128) % p as u128) as u64
}
#[inline]
pub(crate) fn am(a: u64, b: u64, p: u64) -> u64 {
    super::jv_bitops::hook_add(p);
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
#[inline]
pub(crate) fn sm(a: u64, b: u64, p: u64) -> u64 {
    super::jv_bitops::hook_add(p);
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UPoly(pub(crate) Vec<u64>); // coefficients, low to high, trimmed

fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

pub(crate) struct PolyRing {
    p: u64,
    pub(crate) muls: Cell<u64>,
}

impl PolyRing {
    pub(crate) fn new(p: u64) -> Self {
        PolyRing {
            p,
            muls: Cell::new(0),
        }
    }
    fn count(&self, k: u64) {
        self.muls.set(self.muls.get() + k);
    }
    fn deg(f: &UPoly) -> isize {
        f.0.len() as isize - 1
    }
    #[allow(dead_code)]
    fn add(&self, f: &UPoly, g: &UPoly) -> UPoly {
        let n = f.0.len().max(g.0.len());
        let mut out = vec![0u64; n];
        for (i, o) in out.iter_mut().enumerate() {
            let a = f.0.get(i).copied().unwrap_or(0);
            let b = g.0.get(i).copied().unwrap_or(0);
            *o = am(a, b, self.p);
        }
        UPoly(trim(out))
    }
    fn sub(&self, f: &UPoly, g: &UPoly) -> UPoly {
        let n = f.0.len().max(g.0.len());
        let mut out = vec![0u64; n];
        for (i, o) in out.iter_mut().enumerate() {
            let a = f.0.get(i).copied().unwrap_or(0);
            let b = g.0.get(i).copied().unwrap_or(0);
            *o = sm(a, b, self.p);
        }
        UPoly(trim(out))
    }
    fn mul(&self, f: &UPoly, g: &UPoly) -> UPoly {
        if f.0.is_empty() || g.0.is_empty() {
            return UPoly(vec![]);
        }
        let mut out = vec![0u64; f.0.len() + g.0.len() - 1];
        self.count((f.0.len() * g.0.len()) as u64);
        for (i, &a) in f.0.iter().enumerate() {
            for (j, &b) in g.0.iter().enumerate() {
                out[i + j] = am(out[i + j], mm(a, b, self.p), self.p);
            }
        }
        UPoly(trim(out))
    }
    fn scale(&self, f: &UPoly, k: u64) -> UPoly {
        self.count(f.0.len() as u64);
        UPoly(trim(f.0.iter().map(|&a| mm(a, k, self.p)).collect()))
    }
    /// Remainder of `f` modulo monic-normalised `g`.
    fn rem(&self, f: &UPoly, g: &UPoly) -> UPoly {
        let dg = Self::deg(g);
        assert!(dg >= 0);
        let mut r = f.0.clone();
        let lead_inv = inv_mod(*g.0.last().unwrap(), self.p);
        while r.len() as isize > dg {
            let coef = mm(*r.last().unwrap(), lead_inv, self.p);
            let shift = r.len() - g.0.len();
            self.count(g.0.len() as u64 + 1);
            for (i, &gc) in g.0.iter().enumerate() {
                r[shift + i] = sm(r[shift + i], mm(coef, gc, self.p), self.p);
            }
            r = trim(r);
            if r.is_empty() {
                break;
            }
        }
        UPoly(r)
    }
    fn gcd(&self, f: &UPoly, g: &UPoly) -> UPoly {
        let (mut a, mut b) = (f.clone(), g.clone());
        while !b.0.is_empty() {
            let r = self.rem(&a, &b);
            a = b;
            b = r;
        }
        if a.0.is_empty() {
            return a;
        }
        let inv = inv_mod(*a.0.last().unwrap(), self.p);
        self.scale(&a, inv)
    }
    fn mulmod(&self, f: &UPoly, g: &UPoly, m: &UPoly) -> UPoly {
        self.rem(&self.mul(f, g), m)
    }
    fn powmod(&self, f: &UPoly, mut e: u64, m: &UPoly) -> UPoly {
        let mut base = self.rem(f, m);
        let mut acc = UPoly(vec![1]);
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mulmod(&acc, &base, m);
            }
            base = self.mulmod(&base, &base, m);
            e >>= 1;
        }
        acc
    }
    /// All roots in `F_p` of `f` (Cantor–Zassenhaus).
    pub(crate) fn roots(&self, f: &UPoly, rng: &mut StdRng) -> Vec<u64> {
        let f = {
            let inv = match f.0.last() {
                Some(&l) => inv_mod(l, self.p),
                None => return vec![],
            };
            self.scale(f, inv)
        };
        if Self::deg(&f) <= 0 {
            return vec![];
        }
        // gcd(f, x^p − x): the product of the distinct linear factors.
        let xp = self.powmod(&UPoly(vec![0, 1]), self.p, &f);
        let xp_minus_x = self.sub(&xp, &UPoly(vec![0, 1]));
        let g = self.gcd(&f, &xp_minus_x);
        let mut out = Vec::new();
        self.split(&g, rng, &mut out);
        out.sort_unstable();
        out.dedup();
        out
    }
    fn split(&self, g: &UPoly, rng: &mut StdRng, out: &mut Vec<u64>) {
        match Self::deg(g) {
            d if d <= 0 => {}
            1 => {
                // g = x + c0 (monic)
                out.push((self.p - g.0[0]) % self.p);
            }
            _ => {
                for _ in 0..64 {
                    let c = rng.gen_range(0..self.p);
                    // (x + c)^{(p−1)/2} − 1
                    let h = self.powmod(&UPoly(vec![c, 1]), (self.p - 1) / 2, g);
                    let h = self.sub(&h, &UPoly(vec![1]));
                    let f1 = self.gcd(g, &h);
                    let d1 = Self::deg(&f1);
                    if d1 > 0 && d1 < Self::deg(g) {
                        // g = f1 · f2
                        let f2 = self.div_exact(g, &f1);
                        self.split(&f1, rng, out);
                        self.split(&f2, rng, out);
                        return;
                    }
                }
            }
        }
    }
    fn div_exact(&self, f: &UPoly, g: &UPoly) -> UPoly {
        let dg = Self::deg(g);
        let mut r = f.0.clone();
        let mut q = vec![0u64; f.0.len().saturating_sub(g.0.len()) + 1];
        let lead_inv = inv_mod(*g.0.last().unwrap(), self.p);
        while r.len() as isize > dg && !r.is_empty() {
            let coef = mm(*r.last().unwrap(), lead_inv, self.p);
            let shift = r.len() - g.0.len();
            q[shift] = coef;
            self.count(g.0.len() as u64 + 1);
            for (i, &gc) in g.0.iter().enumerate() {
                r[shift + i] = sm(r[shift + i], mm(coef, gc, self.p), self.p);
            }
            r = trim(r);
        }
        UPoly(trim(q))
    }
    fn eval(&self, f: &UPoly, x: u64) -> u64 {
        self.count(f.0.len() as u64);
        let mut acc = 0u64;
        for &c in f.0.iter().rev() {
            acc = am(mm(acc, x, self.p), c, self.p);
        }
        acc
    }
    /// Sylvester resultant of two polynomials of degree ≤ 2 whose
    /// coefficients are themselves polynomials in another variable
    /// (`a[i]`, `b[i]` = coefficient of `x2^i`), computed by the
    /// explicit 4×4 determinant formula.
    fn resultant_deg2(&self, a: &[UPoly; 3], b: &[UPoly; 3]) -> UPoly {
        // Res(a2 x² + a1 x + a0, b2 x² + b1 x + b0)
        //  = (a2 b0 − a0 b2)² − (a2 b1 − a1 b2)(a1 b0 − a0 b1)
        let t1 = self.sub(&self.mul(&a[2], &b[0]), &self.mul(&a[0], &b[2]));
        let t2 = self.sub(&self.mul(&a[2], &b[1]), &self.mul(&a[1], &b[2]));
        let t3 = self.sub(&self.mul(&a[1], &b[0]), &self.mul(&a[0], &b[1]));
        self.sub(&self.mul(&t1, &t1), &self.mul(&t2, &t3))
    }
}
