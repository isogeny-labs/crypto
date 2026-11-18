//! The tower of `src/cryptanalysis/jv_cover.rs`, re-implemented without
//! counters so emitted coordinates are in exactly the library's basis:
//! `F_{p^2} = F_p[t]/(t^2 - w)`, `w` the least non-residue `>= 2`;
//! `F_{p^6} = F_{p^2}[θ]/(θ^3 - s)`, `s` the first non-cube in the library's
//! search order (base-field slice first when `p ≡ 1 mod 3`, then `x + y t`
//! with `y = 1..p`, `x = 0..p`). Compatibility is tested against the
//! committed p101 and p1009 public inputs (their points must lie on the
//! curve in this basis, which fails for any other `w` or `s`).

pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p { s - p } else { s }
}
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b { a - b } else { a + p - b }
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
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

/// Deterministic Miller–Rabin for u64 (the same witness set as
/// `residual_walk::is_prime_u64`).
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    const W: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &sp in &W {
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
    'w: for &a in &W {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct E2(pub [u64; 2]);
impl E2 {
    pub const ZERO: E2 = E2([0, 0]);
    pub const ONE: E2 = E2([1, 0]);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct E6(pub [E2; 3]);
impl E6 {
    pub const ZERO: E6 = E6([E2::ZERO; 3]);
    pub const ONE: E6 = E6([E2::ONE, E2::ZERO, E2::ZERO]);
    pub fn in_fq(&self) -> bool {
        self.0[1] == E2::ZERO && self.0[2] == E2::ZERO
    }
}

#[derive(Clone, Debug)]
pub struct Tower {
    pub p: u64,
    pub w: u64,
    pub s: E2,
    pub zeta: E2,
    pub zeta2: E2,
}

impl Tower {
    pub fn new(p: u64) -> Tower {
        assert!(p % 2 == 1 && p > 3 && p % 3 != 0 && is_prime_u64(p), "p must be a prime > 3, p != 3");
        let w = (2..p).find(|&w| powm(w, (p - 1) / 2, p) != 1).expect("non-residue");
        let mut t = Tower { p, w, s: E2::ZERO, zeta: E2::ZERO, zeta2: E2::ZERO };
        let q1 = (p as u128) * (p as u128) - 1;
        let base_limit = if p % 3 == 1 { p } else { 1 };
        let base = (1..base_limit).map(|x| E2([x, 0]));
        let ext = (1..p).flat_map(|y| (0..p).map(move |x| E2([x, y])));
        let s = base
            .chain(ext)
            .find(|s| *s != E2::ZERO && t.pow2(s, q1 / 3) != E2::ONE)
            .expect("non-cube");
        t.s = s;
        t.zeta = t.pow2(&s, q1 / 3);
        t.zeta2 = t.mul2(&t.zeta, &t.zeta);
        t
    }
    // ── F_{p^2} ──
    pub fn add2(&self, a: &E2, b: &E2) -> E2 {
        E2([addm(a.0[0], b.0[0], self.p), addm(a.0[1], b.0[1], self.p)])
    }
    pub fn sub2(&self, a: &E2, b: &E2) -> E2 {
        E2([subm(a.0[0], b.0[0], self.p), subm(a.0[1], b.0[1], self.p)])
    }
    pub fn neg2(&self, a: &E2) -> E2 {
        self.sub2(&E2::ZERO, a)
    }
    pub fn mul2(&self, a: &E2, b: &E2) -> E2 {
        let p = self.p;
        let m00 = mulm(a.0[0], b.0[0], p);
        let m11 = mulm(a.0[1], b.0[1], p);
        E2([
            addm(m00, mulm(self.w, m11, p), p),
            addm(mulm(a.0[0], b.0[1], p), mulm(a.0[1], b.0[0], p), p),
        ])
    }
    pub fn inv2(&self, a: &E2) -> E2 {
        let p = self.p;
        let norm = subm(mulm(a.0[0], a.0[0], p), mulm(self.w, mulm(a.0[1], a.0[1], p), p), p);
        let ni = invm(norm, p);
        E2([mulm(a.0[0], ni, p), mulm((p - a.0[1]) % p, ni, p)])
    }
    pub fn pow2(&self, a: &E2, mut e: u128) -> E2 {
        let mut b = *a;
        let mut r = E2::ONE;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul2(&r, &b);
            }
            b = self.mul2(&b, &b);
            e >>= 1;
        }
        r
    }
    // ── F_{p^6} ──
    pub fn add(&self, a: &E6, b: &E6) -> E6 {
        E6(core::array::from_fn(|i| self.add2(&a.0[i], &b.0[i])))
    }
    pub fn sub(&self, a: &E6, b: &E6) -> E6 {
        E6(core::array::from_fn(|i| self.sub2(&a.0[i], &b.0[i])))
    }
    pub fn neg(&self, a: &E6) -> E6 {
        self.sub(&E6::ZERO, a)
    }
    pub fn mul(&self, a: &E6, b: &E6) -> E6 {
        let mut d = [E2::ZERO; 5];
        for i in 0..3 {
            for j in 0..3 {
                d[i + j] = self.add2(&d[i + j], &self.mul2(&a.0[i], &b.0[j]));
            }
        }
        E6([
            self.add2(&d[0], &self.mul2(&self.s, &d[3])),
            self.add2(&d[1], &self.mul2(&self.s, &d[4])),
            d[2],
        ])
    }
    pub fn sq(&self, a: &E6) -> E6 {
        self.mul(a, a)
    }
    pub fn sigma(&self, a: &E6) -> E6 {
        E6([a.0[0], self.mul2(&a.0[1], &self.zeta), self.mul2(&a.0[2], &self.zeta2)])
    }
    pub fn inv(&self, a: &E6) -> E6 {
        assert!(*a != E6::ZERO, "inverse of zero in F_p^6");
        let s1 = self.sigma(a);
        let s2 = self.sigma(&s1);
        let b = self.mul(&s1, &s2);
        let n = self.mul(a, &b);
        assert!(n.in_fq(), "norm not in F_q");
        let ni = self.inv2(&n.0[0]);
        E6(core::array::from_fn(|i| self.mul2(&b.0[i], &ni)))
    }
    pub fn pow(&self, a: &E6, mut e: u128) -> E6 {
        let mut b = *a;
        let mut r = E6::ONE;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.sq(&b);
            e >>= 1;
        }
        r
    }
    pub fn q6(&self) -> u128 {
        (self.p as u128).pow(6)
    }
    pub fn is_square(&self, a: &E6) -> bool {
        *a == E6::ZERO || self.pow(a, (self.q6() - 1) / 2) == E6::ONE
    }
    pub fn from_fp(&self, x: u64) -> E6 {
        E6([E2([x % self.p, 0]), E2::ZERO, E2::ZERO])
    }
    /// Element of `F_{p^6}` fixed by `x -> x^{p^k}` (`k | 6`), i.e. in `F_{p^k}`.
    pub fn in_subfield(&self, a: &E6, k: u32) -> bool {
        self.pow(a, (self.p as u128).pow(k)) == *a
    }
    /// Tonelli–Shanks with a non-square `z` supplied by the caller (drawn
    /// from the target stream, so the root is deterministic).
    pub fn sqrt_with(&self, a: &E6, z: &E6) -> Option<E6> {
        if *a == E6::ZERO {
            return Some(E6::ZERO);
        }
        if !self.is_square(a) {
            return None;
        }
        let q1 = self.q6() - 1;
        let s = q1.trailing_zeros();
        let odd = q1 >> s;
        let mut m = s;
        let mut c = self.pow(z, odd);
        let mut t = self.pow(a, odd);
        let mut r = self.pow(a, odd.div_ceil(2));
        while t != E6::ONE {
            let mut i = 0;
            let mut tt = t;
            while tt != E6::ONE {
                tt = self.sq(&tt);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.sq(&b);
            }
            m = i;
            c = self.sq(&b);
            t = self.mul(&t, &c);
            r = self.mul(&r, &b);
        }
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tower_axioms_small_primes() {
        for p in [101u64, 127, 223, 1009] {
            let t = Tower::new(p);
            let a = E6([E2([3, 5]), E2([7, 11]), E2([13, 17])]);
            let b = E6([E2([19, 2]), E2([0, 23]), E2([29, 1])]);
            assert_eq!(t.mul(&a, &b), t.mul(&b, &a));
            assert_eq!(t.mul(&a, &t.inv(&a)), E6::ONE);
            // sigma is the q-Frobenius: sigma(a) = a^{p^2}
            assert_eq!(t.sigma(&a), t.pow(&a, (p as u128).pow(2)));
            // a^{p^6} = a
            assert_eq!(t.pow(&a, t.q6()), a);
            let sq = t.sq(&a);
            // a non-square z
            let mut z = E6([E2([2, 1]), E2::ZERO, E2([1, 0])]);
            while t.is_square(&z) {
                z.0[0].0[0] += 1;
            }
            let r = t.sqrt_with(&sq, &z).unwrap();
            assert!(r == a || r == t.neg(&a));
        }
    }
    #[test]
    fn primality() {
        assert!(is_prime_u64(265379843621));
        assert!(is_prime_u64(263807419614139553));
        assert!(!is_prime_u64(263807419614139553 - 1));
        assert!(!is_prime_u64(1009 * 1013));
    }
}
