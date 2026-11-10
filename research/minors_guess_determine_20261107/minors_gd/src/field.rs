//! Counted prime-field arithmetic. Every multiplication or squaring is one
//! charged unit (`muls`); every modular inversion is counted in `invs` and
//! priced at analysis time; additions and subtractions are counted in `adds`
//! and are NOT charged (for any arm).
use std::cell::Cell;

pub struct Fp {
    pub p: u64,
    pinv: f64,
    pub muls: Cell<u64>,
    pub invs: Cell<u64>,
    pub adds: Cell<u64>,
}

impl Fp {
    pub fn new(p: u64) -> Fp {
        assert!(p > 3 && p < (1u64 << 44));
        Fp { p, pinv: 1.0 / (p as f64), muls: Cell::new(0), invs: Cell::new(0), adds: Cell::new(0) }
    }
    #[inline(always)]
    fn red(&self, a: u64, b: u64) -> u64 {
        let q = ((a as f64) * (b as f64) * self.pinv) as u64;
        let mut r = a.wrapping_mul(b).wrapping_sub(q.wrapping_mul(self.p)) as i64;
        let p = self.p as i64;
        if r < 0 { r += p } else if r >= p { r -= p }
        if r < 0 { r += p } else if r >= p { r -= p }
        r as u64
    }
    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.muls.set(self.muls.get() + 1);
        self.red(a, b)
    }
    #[inline(always)]
    pub fn mul_nc(&self, a: u64, b: u64) -> u64 { self.red(a, b) }
    #[inline(always)]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        self.adds.set(self.adds.get() + 1);
        let s = a + b;
        if s >= self.p { s - self.p } else { s }
    }
    #[inline(always)]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        self.adds.set(self.adds.get() + 1);
        if a >= b { a - b } else { a + self.p - b }
    }
    #[inline(always)]
    pub fn neg(&self, a: u64) -> u64 { if a == 0 { 0 } else { self.p - a } }
    pub fn inv_nc(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        let (mut r0, mut r1) = (self.p as i128, a as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            let r2 = r0 - q * r1; r0 = r1; r1 = r2;
            let t2 = t0 - q * t1; t0 = t1; t1 = t2;
        }
        assert!(r0 == 1);
        (((t0 % self.p as i128) + self.p as i128) % self.p as i128) as u64
    }
    pub fn inv(&self, a: u64) -> u64 {
        self.invs.set(self.invs.get() + 1);
        self.inv_nc(a)
    }
    pub fn pow_nc(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 { if e & 1 == 1 { r = self.red(r, a) } a = self.red(a, a); e >>= 1; }
        r
    }
    /// Montgomery batch inversion of nonzero entries: 3(n-1) mults + 1 inversion.
    pub fn batch_inv(&self, v: &mut [u64]) {
        let n = v.len();
        if n == 0 { return; }
        if n == 1 { v[0] = self.inv(v[0]); return; }
        let mut pre = Vec::with_capacity(n);
        let mut acc = 1u64;
        for &x in v.iter() { acc = self.mul(acc, x); pre.push(acc); }
        let mut inv = self.inv(acc);
        for i in (1..n).rev() {
            let xi = v[i];
            v[i] = self.mul(inv, pre[i - 1]);
            inv = self.mul(inv, xi);
        }
        v[0] = inv;
    }
    pub fn counters(&self) -> Counters { Counters { muls: self.muls.get(), invs: self.invs.get(), adds: self.adds.get() } }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Counters { pub muls: u64, pub invs: u64, pub adds: u64 }

/// Price of one inversion in multiplications: Fermat-equivalent 1.5*bits(p).
pub fn inv_weight(p: u64) -> f64 { 1.5 * (64 - p.leading_zeros()) as f64 }

pub fn charged(c: &Counters, p: u64) -> f64 { c.muls as f64 + inv_weight(p) * c.invs as f64 }

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for &q in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 { return n == q; }
    }
    let mut d = n - 1; let mut s = 0;
    while d % 2 == 0 { d /= 2; s += 1; }
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    let pw = |mut a: u64, mut e: u64| { let mut r = 1u64; while e > 0 { if e & 1 == 1 { r = mm(r, a) } a = mm(a, a); e >>= 1; } r };
    'w: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = pw(a % n, d);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..s - 1 { x = mm(x, x); if x == n - 1 { continue 'w; } }
        return false;
    }
    true
}
