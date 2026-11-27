//! Exact integer and rational helpers (BigInt only; no floating point in any decision).

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

pub type Z = BigInt;

pub fn z(v: i64) -> Z {
    BigInt::from(v)
}

/// Exact rational number with positive denominator in lowest terms.
#[derive(Clone, Debug)]
pub struct Rat {
    pub n: Z,
    pub d: Z,
}

impl Rat {
    pub fn new(n: Z, d: Z) -> Rat {
        assert!(!d.is_zero(), "zero denominator");
        let (mut n, mut d) = if d.is_negative() { (-n, -d) } else { (n, d) };
        let g = n.gcd(&d);
        if !g.is_zero() && !g.is_one() {
            n /= &g;
            d /= &g;
        }
        Rat { n, d }
    }
    pub fn int(n: Z) -> Rat {
        Rat { n, d: Z::one() }
    }
    pub fn add(&self, o: &Rat) -> Rat {
        Rat::new(&self.n * &o.d + &o.n * &self.d, &self.d * &o.d)
    }
    pub fn sub(&self, o: &Rat) -> Rat {
        Rat::new(&self.n * &o.d - &o.n * &self.d, &self.d * &o.d)
    }
    pub fn mul(&self, o: &Rat) -> Rat {
        Rat::new(&self.n * &o.n, &self.d * &o.d)
    }
    pub fn div(&self, o: &Rat) -> Rat {
        Rat::new(&self.n * &o.d, &self.d * &o.n)
    }
    pub fn le(&self, o: &Rat) -> bool {
        &self.n * &o.d <= &o.n * &self.d
    }
    pub fn lt(&self, o: &Rat) -> bool {
        &self.n * &o.d < &o.n * &self.d
    }
    pub fn is_neg(&self) -> bool {
        self.n.is_negative()
    }
    /// Nearest integer, half ties toward +infinity (as upstream).
    pub fn round(&self) -> Z {
        (Z::from(2) * &self.n + &self.d).div_floor(&(Z::from(2) * &self.d))
    }
    pub fn floor(&self) -> Z {
        self.n.div_floor(&self.d)
    }
}

pub fn modp(a: &Z, m: &Z) -> Z {
    a.mod_floor(m)
}

pub fn inv_mod(a: &Z, m: &Z) -> Option<Z> {
    let a = modp(a, m);
    let e = a.extended_gcd(m);
    if !e.gcd.is_one() {
        return None;
    }
    Some(modp(&e.x, m))
}

pub fn pow_mod(b: &Z, e: &Z, m: &Z) -> Z {
    modp(b, m).modpow(e, m)
}

const SMALL_PRIMES: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Miller-Rabin with the first 12 prime bases: deterministic below 3.3e24 (~2^81).
/// Above that the 12 fixed bases plus 8 more give a probabilistic test; every
/// number tested in this experiment is below 2^81 except none (asserted by caller logs).
pub fn is_prime(n: &Z) -> bool {
    if n < &z(2) {
        return false;
    }
    for &q in SMALL_PRIMES.iter() {
        let qz = Z::from(q);
        if n == &qz {
            return true;
        }
        if (n % &qz).is_zero() {
            return false;
        }
    }
    let nm1 = n - 1u32;
    let mut d = nm1.clone();
    let mut s = 0u32;
    while d.is_even() {
        d >>= 1;
        s += 1;
    }
    let extra: [u32; 8] = [41, 43, 47, 53, 59, 61, 67, 71];
    'outer: for &a in SMALL_PRIMES.iter().chain(extra.iter()) {
        let mut x = Z::from(a).modpow(&d, n);
        if x.is_one() || x == nm1 {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == nm1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Legendre symbol (a/q) for odd prime q, in {-1,0,1}.
pub fn legendre(a: &Z, q: &Z) -> i32 {
    let a = modp(a, q);
    if a.is_zero() {
        return 0;
    }
    let r = a.modpow(&((q - 1u32) >> 1), q);
    if r.is_one() {
        1
    } else {
        -1
    }
}

/// Square root modulo an odd prime (Tonelli-Shanks), None if non-residue.
pub fn sqrt_mod(a: &Z, q: &Z) -> Option<Z> {
    let a = modp(a, q);
    if a.is_zero() {
        return Some(Z::zero());
    }
    if legendre(&a, q) != 1 {
        return None;
    }
    if (q % 4u32) == z(3) {
        let r = a.modpow(&((q + 1u32) >> 2), q);
        return Some(r);
    }
    let mut qq = q - 1u32;
    let mut s = 0u32;
    while qq.is_even() {
        qq >>= 1;
        s += 1;
    }
    let mut zn = z(2);
    while legendre(&zn, q) != -1 {
        zn += 1u32;
    }
    let mut m = s;
    let mut c = zn.modpow(&qq, q);
    let mut t = a.modpow(&qq, q);
    let mut r = a.modpow(&((&qq + 1u32) >> 1), q);
    while !t.is_one() {
        let mut i = 0u32;
        let mut tt = t.clone();
        while !tt.is_one() {
            tt = (&tt * &tt) % q;
            i += 1;
            assert!(i < m, "Tonelli-Shanks failed");
        }
        let b = c.modpow(&(Z::one() << (m - i - 1)), q);
        r = (&r * &b) % q;
        c = (&b * &b) % q;
        t = (&t * &c) % q;
        m = i;
    }
    assert_eq!((&r * &r) % q, a);
    Some(r)
}

pub fn is_square(n: &Z) -> Option<Z> {
    if n.is_negative() {
        return None;
    }
    let r = n.sqrt();
    if &(&r * &r) == n {
        Some(r)
    } else {
        None
    }
}

/// Cornacchia for a prime q = 1 mod 4: x^2 + y^2 = q.
pub fn cornacchia_prime(q: &Z) -> (Z, Z) {
    // sqrt(-1) mod q
    let mut c = z(2);
    let e = (q - 1u32) >> 2;
    let s = loop {
        if legendre(&c, q) == -1 {
            break c.modpow(&e, q);
        }
        c += 1u32;
    };
    let mut r0 = q.clone();
    let mut r1 = s;
    let bound = q.sqrt();
    while r1 > bound {
        let r2 = &r0 % &r1;
        r0 = r1;
        r1 = r2;
    }
    let x = r1;
    let y = is_square(&(q - &x * &x)).expect("Cornacchia failed for prime 1 mod 4");
    (x, y)
}

fn gmul(a: &(Z, Z), b: &(Z, Z)) -> (Z, Z) {
    (&a.0 * &b.0 - &a.1 * &b.1, &a.0 * &b.1 + &a.1 * &b.0)
}

/// Write n = a^2 + b^2 when n factors as (trial-divided part up to 1000) times
/// (1 or one prime = 1 mod 4). Returns None otherwise ("hard" cofactor or not a sum).
pub fn sum_two_squares(n: &Z, small: &[u32]) -> Option<(Z, Z)> {
    if n.is_negative() {
        return None;
    }
    if n.is_zero() {
        return Some((Z::zero(), Z::zero()));
    }
    let mut m = n.clone();
    let mut g: (Z, Z) = (Z::one(), Z::zero());
    for &q in small {
        let qz = Z::from(q);
        let mut e = 0u32;
        while (&m % &qz).is_zero() {
            m /= &qz;
            e += 1;
        }
        if e == 0 {
            continue;
        }
        if q == 2 {
            for _ in 0..e {
                g = gmul(&g, &(Z::one(), Z::one()));
            }
        } else if q % 4 == 3 {
            if e % 2 == 1 {
                return None;
            }
            for _ in 0..e / 2 {
                g = (&g.0 * &qz, &g.1 * &qz);
            }
        } else {
            let f = cornacchia_prime(&qz);
            for _ in 0..e {
                g = gmul(&g, &f);
            }
        }
    }
    if !m.is_one() {
        if (&m % 4u32) != z(1) || !is_prime(&m) {
            return None;
        }
        let f = cornacchia_prime(&m);
        g = gmul(&g, &f);
    }
    let (a, b) = (g.0.abs(), g.1.abs());
    assert_eq!(&(&a * &a + &b * &b), n, "sum of two squares wrong");
    Some((a, b))
}

pub fn small_primes(limit: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for n in 2..=limit {
        if v.iter().take_while(|&&q| q * q <= n).all(|&q| n % q != 0) {
            v.push(n);
        }
    }
    v
}

pub fn ln_big(n: &Z) -> f64 {
    // ln of a positive BigInt via bit length, exact enough for reporting.
    let bits = n.bits();
    if bits < 1000 {
        if let Some(f) = n.to_f64() {
            return f.ln();
        }
    }
    let shift = bits - 60;
    let top = (n >> shift).to_f64().unwrap();
    top.ln() + (shift as f64) * std::f64::consts::LN_2
}
