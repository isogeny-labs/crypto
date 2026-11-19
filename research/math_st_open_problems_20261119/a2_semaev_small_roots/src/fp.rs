//! Arithmetic modulo a prime p < 2^64 (u128 intermediates) and the elliptic
//! curve group law in affine coordinates.
use rand::Rng;

#[inline]
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
#[inline]
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a as u128 + b as u128;
    (s % p as u128) as u64
}
#[inline]
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b { a - b } else { (a as u128 + p as u128 - b as u128) as u64 }
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 { r = mulm(r, a, p); }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero");
    powm(a, p - 2, p)
}
/// Map a signed integer to its residue mod p.
pub fn from_i64(x: i64, p: u64) -> u64 {
    if x >= 0 { (x as u64) % p } else { subm(0, ((-(x as i128)) as u64) % p, p) }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for &q in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 { return n == q; }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 { d /= 2; s += 1; }
    'outer: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        for _ in 0..s - 1 {
            x = mulm(x, x, n);
            if x == n - 1 { continue 'outer; }
        }
        return false;
    }
    true
}

/// Uniform random prime with exactly `bits` bits (top bit set).
pub fn random_prime<R: Rng>(bits: u32, rng: &mut R) -> u64 {
    assert!(bits >= 3 && bits <= 64);
    loop {
        let mut c: u64 = rng.gen();
        if bits < 64 { c &= (1u64 << bits) - 1; }
        c |= 1u64 << (bits - 1);
        c |= 1;
        if is_prime(c) { return c; }
    }
}

pub fn is_qr(a: u64, p: u64) -> bool {
    a % p == 0 || powm(a, (p - 1) / 2, p) == 1
}

/// Tonelli-Shanks square root; None if a is a non-residue.
pub fn sqrtm(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 { return Some(0); }
    if p == 2 { return Some(a); }
    if powm(a, (p - 1) / 2, p) != 1 { return None; }
    if p % 4 == 3 { return Some(powm(a, (p + 1) / 4, p)); }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 { q /= 2; s += 1; }
    let mut z = 2u64;
    while powm(z, (p - 1) / 2, p) != p - 1 { z += 1; }
    let mut m = s;
    let mut c = powm(z, q, p);
    let mut t = powm(a, q, p);
    let mut r = powm(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 { tt = mulm(tt, tt, p); i += 1; }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt { Inf, A(u64, u64) }

#[derive(Clone, Copy, Debug)]
pub struct Curve { pub p: u64, pub a: u64, pub b: u64 }

impl Curve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p), self.b, p)
    }
    pub fn nonsingular(&self) -> bool {
        let p = self.p;
        let d = addm(mulm(4, mulm(mulm(self.a, self.a, p), self.a, p), p), mulm(27, mulm(self.b, self.b, p), p), p);
        d != 0
    }
    pub fn on(&self, q: Pt) -> bool {
        match q { Pt::Inf => true, Pt::A(x, y) => mulm(y, y, self.p) == self.rhs(x) }
    }
    pub fn neg(&self, q: Pt) -> Pt {
        match q { Pt::Inf => Pt::Inf, Pt::A(x, y) => Pt::A(x, subm(0, y, self.p)) }
    }
    pub fn add(&self, u: Pt, v: Pt) -> Pt {
        let p = self.p;
        match (u, v) {
            (Pt::Inf, _) => v,
            (_, Pt::Inf) => u,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                let lam;
                if x1 == x2 {
                    if addm(y1, y2, p) == 0 { return Pt::Inf; }
                    let num = addm(mulm(3, mulm(x1, x1, p), p), self.a, p);
                    lam = mulm(num, invm(mulm(2, y1, p), p), p);
                } else {
                    lam = mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p);
                }
                let x3 = subm(subm(mulm(lam, lam, p), x1, p), x2, p);
                let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
                Pt::A(x3, y3)
            }
        }
    }
    /// Point with x-coordinate x (a fixed square root), if x is on the curve.
    pub fn lift_x(&self, x: u64) -> Option<Pt> {
        sqrtm(self.rhs(x), self.p).map(|y| Pt::A(x % self.p, y))
    }
}
