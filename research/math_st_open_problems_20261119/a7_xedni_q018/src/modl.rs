//! Small prime-field elliptic-curve arithmetic (u64), used for the finite input
//! over F_17 and as an exact *necessary-condition* filter for relation search:
//! a relation n1 P1 + n2 P2 = O over Q implies the same relation mod every good
//! prime at which P1, P2 are integral. Survivors are always re-verified over Q.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};

pub type Fp = Option<(u64, u64)>;

#[derive(Clone, Copy, Debug)]
pub struct CurveP {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

pub fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * b as u128) % m as u128) as u64;
        }
        b = ((b as u128 * b as u128) % m as u128) as u64;
        e >>= 1;
    }
    r
}
pub fn inv(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}

impl CurveP {
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    pub fn contains(&self, q: &Fp) -> bool {
        match q {
            None => true,
            Some((x, y)) => {
                let l = self.mul(*y, *y);
                let r = (self.mul(self.mul(*x, *x), *x) + self.mul(self.a, *x) + self.b) % self.p;
                l == r
            }
        }
    }
    pub fn neg(&self, q: &Fp) -> Fp {
        q.map(|(x, y)| (x, (self.p - y) % self.p))
    }
    pub fn add(&self, p1: &Fp, p2: &Fp) -> Fp {
        let (x1, y1) = match p1 {
            None => return *p2,
            Some(v) => *v,
        };
        let (x2, y2) = match p2 {
            None => return *p1,
            Some(v) => *v,
        };
        let p = self.p;
        let l = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            let num = (3 * self.mul(x1, x1) + self.a) % p;
            self.mul(num, inv(2 * y1 % p, p))
        } else {
            self.mul((y2 + p - y1) % p, inv((x2 + p - x1) % p, p))
        };
        let x3 = (self.mul(l, l) + 2 * p - x1 - x2) % p;
        let y3 = (self.mul(l, (x1 + p - x3) % p) + p - y1) % p;
        Some((x3, y3))
    }
    pub fn mul_pt(&self, k: i64, q: &Fp) -> Fp {
        let mut r: Fp = None;
        let mut b = if k < 0 { self.neg(q) } else { *q };
        let mut m = k.unsigned_abs();
        while m > 0 {
            if m & 1 == 1 {
                r = self.add(&r, &b);
            }
            m >>= 1;
            b = self.add(&b, &b);
        }
        r
    }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

pub fn bigint_mod(v: &BigInt, p: u64) -> u64 {
    v.mod_floor(&BigInt::from(p)).to_u64().unwrap()
}

/// Reduce a rational q = n/d mod p, None if p | d.
pub fn rat_mod(n: &BigInt, d: &BigInt, p: u64) -> Option<u64> {
    let dm = bigint_mod(d, p);
    if dm == 0 {
        return None;
    }
    Some(((bigint_mod(n, p) as u128 * inv(dm, p) as u128) % p as u128) as u64)
}

pub fn bigint_divisible(v: &BigInt, p: u64) -> bool {
    bigint_mod(v, p).is_zero()
}
