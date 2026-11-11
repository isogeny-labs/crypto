//! `F_p` and `F_{p²}` arithmetic with `u64` words, `p < 2^31`.
//!
//! `F_{p²} = F_p[i]/(i² − n)` for the least quadratic non-residue `n`. Every
//! element of `F_p` embeds as `(a, 0)`. Kept separate from
//! [`crate::cryptanalysis::genus2_fp`] (whose field type is private) so that
//! the transfer maps can be checked against that module's Cantor arithmetic
//! rather than against shared code.

/// Exclusive bound on `p`: products of two reduced values fit in a `u64`.
pub const MAX_P: u64 = 1 << 31;

/// `F_p` with the non-residue that defines `F_{p²}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp {
    pub p: u64,
    /// Least quadratic non-residue, used for `F_{p²}`.
    pub nr: u64,
}

/// An element `a + b·i` of `F_{p²}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct F2 {
    pub a: u64,
    pub b: u64,
}

impl F2 {
    pub const ZERO: F2 = F2 { a: 0, b: 0 };
    pub const ONE: F2 = F2 { a: 1, b: 0 };
    /// The embedding of `a ∈ F_p`.
    pub fn base(a: u64) -> F2 {
        F2 { a, b: 0 }
    }
    pub fn is_zero(&self) -> bool {
        self.a == 0 && self.b == 0
    }
    /// Whether the element lies in `F_p`.
    pub fn in_base(&self) -> bool {
        self.b == 0
    }
}

/// Deterministic Miller–Rabin for `u64`.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let mulm = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = 1u64;
        let (mut b, mut e) = (a, d);
        while e > 0 {
            if e & 1 == 1 {
                x = mulm(x, b);
            }
            b = mulm(b, b);
            e >>= 1;
        }
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

impl Fp {
    /// `F_p` for an odd prime `p < 2^31`; `None` otherwise.
    pub fn new(p: u64) -> Option<Fp> {
        if !(3..MAX_P).contains(&p) || !is_prime(p) {
            return None;
        }
        let mut k = Fp { p, nr: 0 };
        k.nr = (2..p).find(|&n| k.legendre(n) == -1)?;
        Some(k)
    }
    #[inline]
    pub fn red(&self, a: u64) -> u64 {
        a % self.p
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
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
    /// Inverse of a nonzero element (Fermat).
    pub fn inv(&self, a: u64) -> u64 {
        debug_assert!(a % self.p != 0);
        self.pow(a, self.p - 2)
    }
    /// Legendre symbol in `{−1, 0, 1}`.
    pub fn legendre(&self, a: u64) -> i8 {
        let a = a % self.p;
        if a == 0 {
            0
        } else if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    /// A square root in `F_p` (Tonelli–Shanks), if one exists.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let a = a % self.p;
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        let p = self.p;
        let (mut q, mut s) = (p - 1, 0u32);
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut m = s;
        let mut c = self.pow(self.nr, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        while t != 1 {
            let mut i = 0;
            let mut t2 = t;
            while t2 != 1 {
                t2 = self.mul(t2, t2);
                i += 1;
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

    // ── F_{p²} ──────────────────────────────────────────────────────────
    pub fn e(&self, a: u64) -> F2 {
        F2::base(a % self.p)
    }
    pub fn add2(&self, x: F2, y: F2) -> F2 {
        F2 {
            a: self.add(x.a, y.a),
            b: self.add(x.b, y.b),
        }
    }
    pub fn sub2(&self, x: F2, y: F2) -> F2 {
        F2 {
            a: self.sub(x.a, y.a),
            b: self.sub(x.b, y.b),
        }
    }
    pub fn neg2(&self, x: F2) -> F2 {
        F2 {
            a: self.neg(x.a),
            b: self.neg(x.b),
        }
    }
    pub fn mul2(&self, x: F2, y: F2) -> F2 {
        let ac = self.mul(x.a, y.a);
        let bd = self.mul(x.b, y.b);
        F2 {
            a: self.add(ac, self.mul(self.nr, bd)),
            b: self.add(self.mul(x.a, y.b), self.mul(x.b, y.a)),
        }
    }
    pub fn sqr2(&self, x: F2) -> F2 {
        self.mul2(x, x)
    }
    /// `x^{p}`: the Frobenius `a + b i ↦ a − b i`.
    pub fn frob2(&self, x: F2) -> F2 {
        F2 {
            a: x.a,
            b: self.neg(x.b),
        }
    }
    /// The norm `x · x^p ∈ F_p`.
    pub fn norm2(&self, x: F2) -> u64 {
        self.sub(self.mul(x.a, x.a), self.mul(self.nr, self.mul(x.b, x.b)))
    }
    pub fn inv2(&self, x: F2) -> F2 {
        let n = self.inv(self.norm2(x));
        let c = self.frob2(x);
        F2 {
            a: self.mul(c.a, n),
            b: self.mul(c.b, n),
        }
    }
    pub fn pow2(&self, mut x: F2, mut e: u128) -> F2 {
        let mut r = F2::ONE;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul2(r, x);
            }
            x = self.sqr2(x);
            e >>= 1;
        }
        r
    }
    /// A square root in `F_{p²}` (every element of `F_p` has one), if any.
    pub fn sqrt2(&self, x: F2) -> Option<F2> {
        if x.is_zero() {
            return Some(F2::ZERO);
        }
        if x.in_base() {
            return Some(match self.sqrt(x.a) {
                Some(r) => F2::base(r),
                // a = nr·s²  ⇒  √a = s·i
                None => F2 {
                    a: 0,
                    b: self.sqrt(self.mul(x.a, self.inv(self.nr)))?,
                },
            });
        }
        // x is a square iff its norm is a square in F_p.
        let n = self.sqrt(self.norm2(x))?;
        // (c + d i)² = x  ⇒  c² = (a ± n)/2.
        let half = self.inv(2);
        for s in [n, self.neg(n)] {
            let c2 = self.mul(self.add(x.a, s), half);
            if let Some(c) = self.sqrt(c2) {
                if c == 0 {
                    continue;
                }
                let d = self.mul(x.b, self.inv(self.mul(2, c)));
                let r = F2 { a: c, b: d };
                if self.sqr2(r) == x {
                    return Some(r);
                }
            }
        }
        None
    }
    /// Roots in `F_{p²}` of a monic polynomial of degree 1 or 2 over `F_p`
    /// (coefficients lowest first), with multiplicity.
    pub fn roots_deg_le2(&self, u: &[u64]) -> Vec<F2> {
        match u.len() {
            2 => vec![F2::base(self.neg(u[0]))],
            3 => {
                // t² + b t + c: t = (−b ± √(b² − 4c)) / 2
                let (c, b) = (u[0], u[1]);
                let disc = self.sub(self.mul(b, b), self.mul(4, c));
                let s = self
                    .sqrt2(F2::base(disc))
                    .expect("F_p elements are squares in F_p²");
                let h = F2::base(self.inv(2));
                let mb = F2::base(self.neg(b));
                vec![
                    self.mul2(self.add2(mb, s), h),
                    self.mul2(self.sub2(mb, s), h),
                ]
            }
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fp2_field_axioms_and_sqrt() {
        for p in [101u64, 103, 1009, 65537] {
            let k = Fp::new(p).unwrap();
            let mut s = 7u64;
            for _ in 0..200 {
                s = s
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let x = F2 {
                    a: (s >> 7) % p,
                    b: (s >> 37) % p,
                };
                let y = F2 {
                    a: (s >> 3) % p,
                    b: (s >> 29) % p,
                };
                if !x.is_zero() {
                    assert_eq!(k.mul2(x, k.inv2(x)), F2::ONE);
                    // Fermat in F_{p²}
                    assert_eq!(k.pow2(x, (p as u128) * (p as u128) - 1), F2::ONE);
                }
                assert_eq!(k.mul2(x, y), k.mul2(y, x));
                let sq = k.sqr2(x);
                let r = k.sqrt2(sq).unwrap();
                assert_eq!(k.sqr2(r), sq);
                assert!(k.sqrt2(F2::base(x.a)).is_some());
            }
        }
    }
}
