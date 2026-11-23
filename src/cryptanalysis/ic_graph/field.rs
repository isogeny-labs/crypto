//! Prime-field arithmetic over `F_p` with `p < 2^63`, plus a deterministic
//! seeded RNG so every randomized routine in `ic_graph` is replayable.
//!
//! The constructor checks primality with a deterministic Miller-Rabin test for
//! 64-bit inputs. The report notes that the existing LP-filter API only checks
//! for an odd modulus; here division-based algorithms refuse composite moduli.

/// A prime field `F_p`. Elements are canonical `u64` residues in `[0, p)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp {
    p: u64,
}

impl Fp {
    /// Build `F_p`. Returns `None` unless `2 <= p < 2^63` and `p` is prime.
    pub fn new(p: u64) -> Option<Self> {
        if !(2..(1u64 << 63)).contains(&p) || !is_prime_u64(p) {
            return None;
        }
        Some(Self { p })
    }

    #[inline]
    pub fn modulus(self) -> u64 {
        self.p
    }
    #[inline]
    pub fn reduce(self, x: u64) -> u64 {
        x % self.p
    }
    #[inline]
    pub fn reduce_i64(self, x: i64) -> u64 {
        let r = x.rem_euclid(self.p as i64);
        r as u64
    }
    #[inline]
    pub fn add(self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    pub fn neg(self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    pub fn mul(self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    /// `a + b*c`.
    #[inline]
    pub fn mul_add(self, a: u64, b: u64, c: u64) -> u64 {
        ((a as u128 + b as u128 * c as u128) % self.p as u128) as u64
    }
    pub fn pow(self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    /// Multiplicative inverse; `None` for zero.
    pub fn inv(self, a: u64) -> Option<u64> {
        let a = a % self.p;
        if a == 0 {
            return None;
        }
        // Extended Euclid on i128 to avoid the cost of exponentiation.
        let (mut r0, mut r1) = (self.p as i128, a as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        debug_assert_eq!(r0, 1);
        Some(t0.rem_euclid(self.p as i128) as u64)
    }
    #[inline]
    pub fn div(self, a: u64, b: u64) -> Option<u64> {
        self.inv(b).map(|bi| self.mul(a, bi))
    }
    /// Dot product of two equal-length vectors.
    pub fn dot(self, a: &[u64], b: &[u64]) -> u64 {
        let mut acc: u128 = 0;
        let p = self.p as u128;
        for (&x, &y) in a.iter().zip(b) {
            acc += x as u128 * y as u128;
            if acc >= (1u128 << 126) {
                acc %= p;
            }
        }
        (acc % p) as u64
    }
}

fn mul_mod_u64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn pow_mod_u64(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mul_mod_u64(r, a, m);
        }
        a = mul_mod_u64(a, a, m);
        e >>= 1;
    }
    r
}

/// Deterministic Miller-Rabin for all `u64` (bases from Jaeschke / Sinclair).
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &sp in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % sp == 0 {
            return n == sp;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for &a in &[2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let a = a % n;
        if a == 0 {
            continue;
        }
        let mut x = pow_mod_u64(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod_u64(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// SplitMix64: small, seeded and replayable. Not cryptographic.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `[0, n)` for `n > 0` (rejection sampling).
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let x = self.next_u64();
            if x < zone {
                return x % n;
            }
        }
    }
    /// Uniform field element.
    pub fn element(&mut self, f: Fp) -> u64 {
        self.below(f.modulus())
    }
    /// Uniform nonzero field element.
    pub fn nonzero(&mut self, f: Fp) -> u64 {
        1 + self.below(f.modulus() - 1)
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i as u64 + 1) as usize;
            v.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primality_and_inverse() {
        assert!(Fp::new(9).is_none());
        assert!(Fp::new(1_000_000_007).is_some());
        assert!(is_prime_u64((1u64 << 61) - 1));
        assert!(!is_prime_u64(3_215_031_751)); // strong pseudoprime to 2,3,5,7
        let f = Fp::new((1u64 << 61) - 1).unwrap();
        let mut rng = SplitMix64::new(1);
        for _ in 0..100 {
            let a = rng.nonzero(f);
            assert_eq!(f.mul(a, f.inv(a).unwrap()), 1);
        }
        assert_eq!(f.inv(0), None);
    }
}
