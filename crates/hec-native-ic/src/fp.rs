//! Prime field `F_p`, `2 < p < 2^62`, residues in `[0, p)` as `u64`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp {
    pub p: u64,
    /// `−p^{-1} mod 2^64` (Montgomery).
    pinv: u64,
    /// `2^128 mod p`.
    r2: u64,
}

impl Fp {
    pub fn new(p: u64) -> Self {
        assert!(
            p > 2 && p < (1u64 << 62) && p % 2 == 1,
            "Fp: need odd 2 < p < 2^62"
        );
        // Newton iteration for p^{-1} mod 2^64 (each step doubles the bits).
        let mut inv = p;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        debug_assert_eq!(p.wrapping_mul(inv), 1);
        let r = ((1u128 << 64) % p as u128) as u128;
        let r2 = ((r * r) % p as u128) as u64;
        Fp {
            p,
            pinv: inv.wrapping_neg(),
            r2,
        }
    }

    /// Montgomery reduction: `t·2^{-64} mod p` for `t < p·2^64`.
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.pinv);
        let s = (t + m as u128 * self.p as u128) >> 64;
        let s = s as u64;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }

    /// `t mod p` for `t < p·2^64` (in particular any `t < 4p²`), in
    /// standard representation: two reductions, no 128-bit division.
    #[inline(always)]
    pub fn reduce128(&self, t: u128) -> u64 {
        self.redc(self.redc(t) as u128 * self.r2 as u128)
    }

    #[inline(always)]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }

    #[inline(always)]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }

    #[inline(always)]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }

    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce128(a as u128 * b as u128)
    }

    /// Reduce a signed integer.
    #[inline]
    pub fn from_i64(&self, a: i64) -> u64 {
        a.rem_euclid(self.p as i64) as u64
    }

    pub fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        b %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }

    /// Inverse by the extended Euclidean algorithm.  Panics on zero.
    pub fn inv(&self, a: u64) -> u64 {
        assert!(a % self.p != 0, "Fp::inv(0)");
        let (mut r0, mut r1) = (self.p as i128, (a % self.p) as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        debug_assert_eq!(r0, 1);
        t0.rem_euclid(self.p as i128) as u64
    }

    /// Legendre symbol: 0, 1 or −1.
    pub fn legendre(&self, a: u64) -> i32 {
        let a = a % self.p;
        if a == 0 {
            return 0;
        }
        if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }

    /// A square root (Tonelli–Shanks), or `None` for a non-residue.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let p = self.p;
        let a = a % p;
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        if p % 4 == 3 {
            return Some(self.pow(a, (p + 1) / 4));
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while self.legendre(z) != -1 {
            z += 1;
        }
        let mut m = s;
        let mut c = self.pow(z, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, (q + 1) / 2);
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                tt = self.mul(tt, tt);
                i += 1;
            }
            let b = self.pow(c, 1u64 << (m - i - 1));
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_and_sqrt() {
        for &p in &[13u64, 17, 101, 65537, 1_000_003, (1u64 << 61) - 1] {
            let f = Fp::new(p);
            for a in 1..200u64.min(p) {
                assert_eq!(f.mul(a, f.inv(a)), 1);
                match f.sqrt(a) {
                    Some(r) => assert_eq!(f.mul(r, r), a),
                    None => assert_eq!(f.legendre(a), -1),
                }
            }
        }
    }
}
