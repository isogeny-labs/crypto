//! Own F_p / F_{p^2} arithmetic for the l1-steps-2-3 checks, written without the
//! library's field types. F_{p^2} = F_p[w]/(w^2 - omega), omega the least
//! quadratic non-residue mod p (the same presentation as jv_cover::Fq, so that
//! the pair [re, im] means re + im*w in both; consistency is checked by the
//! callers on recorded factor-base points). Polynomials are little-endian.
#![allow(dead_code)]

pub type E = (u64, u64);

pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero mod p");
    powm(a, p - 2, p)
}
pub fn is_qr(a: u64, p: u64) -> bool {
    a % p == 0 || powm(a, (p - 1) / 2, p) == 1
}

#[derive(Clone)]
pub struct F2 {
    pub p: u64,
    pub w: u64,
}
impl F2 {
    pub fn new(p: u64) -> Self {
        let w = (2..p).find(|&w| !is_qr(w, p)).expect("non-residue");
        Self { p, w }
    }
    pub fn add(&self, a: E, b: E) -> E {
        ((a.0 + b.0) % self.p, (a.1 + b.1) % self.p)
    }
    pub fn sub(&self, a: E, b: E) -> E {
        ((a.0 + self.p - b.0) % self.p, (a.1 + self.p - b.1) % self.p)
    }
    pub fn neg(&self, a: E) -> E {
        ((self.p - a.0) % self.p, (self.p - a.1) % self.p)
    }
    pub fn mul(&self, a: E, b: E) -> E {
        let p = self.p;
        (
            (mulm(a.0, b.0, p) + mulm(self.w, mulm(a.1, b.1, p), p)) % p,
            (mulm(a.0, b.1, p) + mulm(a.1, b.0, p)) % p,
        )
    }
    pub fn norm(&self, a: E) -> u64 {
        let p = self.p;
        (mulm(a.0, a.0, p) + p - mulm(self.w, mulm(a.1, a.1, p), p)) % p
    }
    pub fn inv(&self, a: E) -> E {
        let ni = invm(self.norm(a), self.p);
        (mulm(a.0, ni, self.p), mulm((self.p - a.1) % self.p, ni, self.p))
    }
    pub fn pow(&self, a: E, mut e: u128) -> E {
        let (mut r, mut b) = ((1, 0), a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Quadratic character on F_{p^2}: 0, +1 or -1 (Euler criterion).
    pub fn chi(&self, a: E) -> i8 {
        if a == (0, 0) {
            return 0;
        }
        let q = self.p as u128 * self.p as u128;
        if self.pow(a, (q - 1) / 2) == (1, 0) {
            1
        } else {
            -1
        }
    }
    pub fn from_fp(&self, x: u64) -> E {
        (x % self.p, 0)
    }
    /// Table of one square root per square of F_{p^2} (brute force over all p^2 elements).
    pub fn sqrt_table(&self) -> Vec<u32> {
        let p = self.p as usize;
        let mut t = vec![u32::MAX; p * p];
        for a in 0..p {
            for b in 0..p {
                let s = self.mul((a as u64, b as u64), (a as u64, b as u64));
                let k = s.0 as usize * p + s.1 as usize;
                if t[k] == u32::MAX {
                    t[k] = (a * p + b) as u32;
                }
            }
        }
        t
    }
    pub fn sqrt(&self, table: &[u32], a: E) -> Option<E> {
        let k = table[a.0 as usize * self.p as usize + a.1 as usize];
        (k != u32::MAX).then(|| ((k as u64) / self.p, (k as u64) % self.p))
    }
    // ---- polynomials over F_{p^2} ----
    pub fn ptrim(&self, mut a: Vec<E>) -> Vec<E> {
        while a.last() == Some(&(0, 0)) {
            a.pop();
        }
        a
    }
    pub fn peval(&self, a: &[E], x: E) -> E {
        a.iter().rev().fold((0, 0), |acc, &c| self.add(self.mul(acc, x), c))
    }
    pub fn padd(&self, a: &[E], b: &[E]) -> Vec<E> {
        let n = a.len().max(b.len());
        self.ptrim((0..n).map(|i| self.add(*a.get(i).unwrap_or(&(0, 0)), *b.get(i).unwrap_or(&(0, 0)))).collect())
    }
    pub fn psub(&self, a: &[E], b: &[E]) -> Vec<E> {
        let n = a.len().max(b.len());
        self.ptrim((0..n).map(|i| self.sub(*a.get(i).unwrap_or(&(0, 0)), *b.get(i).unwrap_or(&(0, 0)))).collect())
    }
    pub fn pmul(&self, a: &[E], b: &[E]) -> Vec<E> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut c = vec![(0, 0); a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                c[i + j] = self.add(c[i + j], self.mul(x, y));
            }
        }
        self.ptrim(c)
    }
    /// Remainder modulo a polynomial with invertible leading coefficient.
    pub fn prem(&self, a: &[E], m: &[E]) -> Vec<E> {
        let mut r = self.ptrim(a.to_vec());
        let m = self.ptrim(m.to_vec());
        let dm = m.len() - 1;
        let il = self.inv(m[dm]);
        while r.len() > dm && !r.is_empty() {
            let c = self.mul(*r.last().unwrap(), il);
            let sh = r.len() - 1 - dm;
            for (i, &mi) in m.iter().enumerate() {
                r[sh + i] = self.sub(r[sh + i], self.mul(c, mi));
            }
            r = self.ptrim(r);
        }
        r
    }
}
