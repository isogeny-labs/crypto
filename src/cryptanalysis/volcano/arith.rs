//! 63-bit Montgomery arithmetic and short-Weierstrass points over `F_q`.
//!
//! Built for the CRT evaluation in [`super::level1`]: thousands of word-size
//! primes `q < 2^63`, each needing a few hundred million field
//! multiplications.  Elements are kept in Montgomery form (`a·2^64 mod q`).

/// Montgomery context for an odd modulus `n < 2^63`.
#[derive(Clone, Copy, Debug)]
pub struct Mont {
    pub n: u64,
    ninv: u64,
    r2: u64,
    one: u64,
}

impl Mont {
    pub fn new(n: u64) -> Self {
        assert!(
            n & 1 == 1 && n < (1u64 << 63) && n > 2,
            "modulus must be odd, 2 < n < 2^63"
        );
        let mut inv: u64 = n;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(inv)));
        }
        let r = ((1u128 << 64) % n as u128) as u64;
        let r2 = ((r as u128 * r as u128) % n as u128) as u64;
        Mont {
            n,
            ninv: inv.wrapping_neg(),
            r2,
            one: r,
        }
    }
    #[inline(always)]
    pub fn one(&self) -> u64 {
        self.one
    }
    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        let t = (a as u128) * (b as u128);
        let m = (t as u64).wrapping_mul(self.ninv);
        let u = ((t + (m as u128) * (self.n as u128)) >> 64) as u64;
        if u >= self.n {
            u - self.n
        } else {
            u
        }
    }
    #[inline(always)]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.n {
            s - self.n
        } else {
            s
        }
    }
    #[inline(always)]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.n - b
        }
    }
    #[inline(always)]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.n - a
        }
    }
    #[inline(always)]
    pub fn to_mont(&self, x: u64) -> u64 {
        self.mul(x % self.n, self.r2)
    }
    #[inline(always)]
    pub fn from_mont(&self, a: u64) -> u64 {
        self.mul(a, 1)
    }
    /// Reduce a signed 128-bit integer into Montgomery form.
    pub fn from_i128(&self, x: i128) -> u64 {
        let n = self.n as i128;
        self.to_mont((((x % n) + n) % n) as u64)
    }
    pub fn pow(&self, a: u64, mut e: u64) -> u64 {
        let mut r = self.one;
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Fermat inverse.  Panics on zero.
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.pow(a, self.n - 2)
    }
    /// Montgomery's trick: invert every entry of `v` in place with one inversion.
    /// `scratch` is reused to avoid allocation.
    pub fn batch_inv(&self, v: &mut [u64], scratch: &mut Vec<u64>) {
        scratch.clear();
        let mut acc = self.one;
        for &x in v.iter() {
            assert!(x != 0, "batch_inv: zero entry");
            scratch.push(acc);
            acc = self.mul(acc, x);
        }
        let mut inv = self.inv(acc);
        for i in (0..v.len()).rev() {
            let xi = v[i];
            v[i] = self.mul(inv, scratch[i]);
            inv = self.mul(inv, xi);
        }
    }
    /// Square root of `a` (Montgomery form) if it exists.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        let n = self.n;
        if self.pow(a, (n - 1) / 2) != self.one {
            return None;
        }
        if n % 4 == 3 {
            return Some(self.pow(a, (n + 1) / 4));
        }
        // Tonelli-Shanks
        let mut s = 0u32;
        let mut q = n - 1;
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = self.to_mont(2);
        while self.pow(z, (n - 1) / 2) == self.one {
            z = self.add(z, self.one);
        }
        let mut m = s;
        let mut c = self.pow(z, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, (q + 1) / 2);
        while t != self.one {
            let mut i = 0u32;
            let mut tt = t;
            while tt != self.one {
                tt = self.mul(tt, tt);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
}

/// Deterministic Miller-Rabin for `n < 2^64`.
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let mulmod = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    let powmod = |mut b: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mulmod(r, b);
            }
            b = mulmod(b, b);
            e >>= 1;
        }
        r
    };
    'outer: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Small deterministic generator (xorshift64*), so every run is reproducible.
#[derive(Clone, Debug)]
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// Affine point `(x, y)` in Montgomery form, `None` = infinity.
pub type Pt = Option<(u64, u64)>;

/// `y² = x³ + a x + b` over `F_q`, coefficients in Montgomery form.
#[derive(Clone, Copy, Debug)]
pub struct Ec {
    pub f: Mont,
    pub a: u64,
    pub b: u64,
}

impl Ec {
    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        let lam;
        if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return None;
            }
            let xx = f.mul(x1, x1);
            let num = f.add(f.add(f.add(xx, xx), xx), self.a);
            lam = f.mul(num, f.inv(f.add(y1, y1)));
        } else {
            lam = f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)));
        }
        let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    pub fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.f.neg(y)))
    }
    pub fn mul(&self, mut k: u64, p: Pt) -> Pt {
        let mut r: Pt = None;
        let mut b = p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    pub fn random_point(&self, rng: &mut Rng) -> Pt {
        let f = &self.f;
        loop {
            let x = f.to_mont(rng.next());
            let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(self.a, x)), self.b);
            if let Some(y) = f.sqrt(rhs) {
                if y != 0 {
                    return Some((x, y));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mont_matches_u128() {
        let q = 4611686018427387847u64; // < 2^62, prime
        assert!(is_prime_u64(q));
        let f = Mont::new(q);
        let mut r = Rng(12345);
        for _ in 0..1000 {
            let a = r.next() % q;
            let b = r.next() % q;
            let want = ((a as u128 * b as u128) % q as u128) as u64;
            assert_eq!(f.from_mont(f.mul(f.to_mont(a), f.to_mont(b))), want);
        }
        let a = f.to_mont(123456789);
        assert_eq!(f.mul(a, f.inv(a)), f.one());
    }
    #[test]
    fn sqrt_both_residue_classes() {
        for q in [1000003u64, 998244353, 1000033] {
            assert!(is_prime_u64(q));
            let f = Mont::new(q);
            for x in 2..50u64 {
                let a = f.mul(f.to_mont(x), f.to_mont(x));
                let s = f.sqrt(a).unwrap();
                assert_eq!(f.mul(s, s), a);
            }
        }
    }
    #[test]
    fn batch_inverse() {
        let f = Mont::new(1000003);
        let mut v: Vec<u64> = (1..100).map(|x| f.to_mont(x)).collect();
        let orig = v.clone();
        let mut s = Vec::new();
        f.batch_inv(&mut v, &mut s);
        for (a, b) in orig.iter().zip(&v) {
            assert_eq!(f.mul(*a, *b), f.one());
        }
    }
    #[test]
    fn group_law_order() {
        // Count points of y^2 = x^3 + 2x + 3 over F_97 by brute force, then check [N]P = O.
        let f = Mont::new(97);
        let e = Ec {
            f,
            a: f.to_mont(2),
            b: f.to_mont(3),
        };
        let mut n = 1u64;
        for x in 0..97u64 {
            let xm = f.to_mont(x);
            let rhs = f.add(f.add(f.mul(f.mul(xm, xm), xm), f.mul(e.a, xm)), e.b);
            n += if rhs == 0 {
                1
            } else if f.sqrt(rhs).is_some() {
                2
            } else {
                0
            };
        }
        let mut r = Rng(7);
        for _ in 0..20 {
            let p = e.random_point(&mut r);
            assert_eq!(e.mul(n, p), None);
        }
    }
}
