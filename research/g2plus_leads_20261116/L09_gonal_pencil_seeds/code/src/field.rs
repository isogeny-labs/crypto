//! Counted F_p (SPEC section 2).
use std::cell::Cell;

pub struct Fp {
    pub p: u64,
    pub muls: Cell<u64>,
    pub invs: Cell<u64>,
}

impl Fp {
    pub fn new(p: u64) -> Fp {
        Fp { p, muls: Cell::new(0), invs: Cell::new(0) }
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p { s - self.p } else { s }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + self.p - b }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 { 0 } else { self.p - a }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.muls.set(self.muls.get() + 1);
        a * b % self.p
    }
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.invs.set(self.invs.get() + 1);
        let p = self.p;
        let mut e = p - 2;
        let mut b = a % p;
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % p;
            }
            b = b * b % p;
            e >>= 1;
        }
        r
    }
    pub fn snap(&self) -> (u64, u64) {
        (self.muls.get(), self.invs.get())
    }
    pub fn fpm(&self) -> u64 {
        self.muls.get() + 10 * self.invs.get()
    }
}

pub fn diff(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    (b.0 - a.0, b.1 - a.1)
}
pub fn cost(a: (u64, u64)) -> u64 {
    a.0 + 10 * a.1
}
