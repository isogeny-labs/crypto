//! Radical (closed-form) low-degree root finding over F_p, 5 <= p <= 2^61-1.
//!
//! Quadratic (Tonelli-Shanks), cubic (Cardano) and quartic (Ferrari) distinct
//! F_p-roots. The crux primitive is a modular cube root: bijective power for
//! p = 2 mod 3, Adleman-Manders-Miller (AMM) for p = 1 mod 3, and AMM over
//! F_{p^2} = F_p[i]/(i^2 - n) for the casus irreducibilis (three F_p roots whose
//! Cardano radicals live in F_{p^2}). No brute-force fallback anywhere.
//!
//! Honest limit: a quartic of Frobenius cycle type (1,3) (one rational root,
//! irreducible cubic cofactor) has no rational Ferrari pairing; its root needs
//! F_{p^3} radicals. That class (about 1/3 of random quartics) is delegated to
//! the distinct-degree gcd(X^p - X, f) step of `prime_root_trace::solve` and is
//! NOT accelerated (`RadicalRoots::frobenius_fallback`).
//!
//! Prototype evidence (counted field multiplications, validation vs brute force
//! and Cantor-Zassenhaus): research/radical_quartic_20261117/. Off the
//! `prime_fast` subtraction hot path. [pcc:unverified]
use super::prime_root_trace::{solve, RootLimits, RootStop, MAX_PRIME, MAX_WORK};
use super::residual_walk::is_prime_u64;

/// Distinct roots plus whether the (1,3) Frobenius fallback was used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadicalRoots {
    pub roots: Vec<u64>,
    pub frobenius_fallback: bool,
}

fn frobenius_roots(p: u64, poly: &[u64]) -> Result<Vec<u64>, String> {
    let limits = RootLimits {
        field_work: MAX_WORK,
        split_attempts: 1000,
        events: 100_000,
        degree: 8,
    };
    let r = solve(poly, p, 0x5eed, limits, &mut |_| Ok(()))?;
    if r.stop != RootStop::Complete {
        return Err(format!("frobenius fallback incomplete: {:?}", r.stop));
    }
    Ok(r.roots)
}

#[derive(Clone, Copy)]
pub(crate) struct Fp {
    pub p: u64,
}
impl Fp {
    pub fn new(p: u64) -> Self {
        Fp { p }
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        // Radical arithmetic normally operates on canonical residues.
        // For a,b < p <= 2^61-1, a+b fits in u64 and needs one correction.
        // Keep the former wide-modulus behavior for noncanonical callers.
        if a < self.p && b < self.p {
            let sum = a + b;
            if sum >= self.p {
                sum - self.p
            } else {
                sum
            }
        } else {
            ((a as u128 + b as u128) % self.p as u128) as u64
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a < self.p && b < self.p {
            if a >= b {
                a - b
            } else {
                a + self.p - b
            }
        } else {
            ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64
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
        if self.p == MAX_PRIME && a < MAX_PRIME && b < MAX_PRIME {
            // p = 2^61 - 1: 2^61 == 1 mod p. The 122-bit product
            // reduces with two end-around-carry folds, no wide division.
            let product = a as u128 * b as u128;
            let folded = ((product as u64) & MAX_PRIME) + (product >> 61) as u64;
            let folded = (folded & MAX_PRIME) + (folded >> 61);
            if folded >= MAX_PRIME {
                folded - MAX_PRIME
            } else {
                folded
            }
        } else if self.p <= u32::MAX as u64 && a < self.p && b < self.p {
            // Canonical 32-bit operands fit in a native 64-bit product.
            (a * b) % self.p
        } else {
            ((a as u128 * b as u128) % self.p as u128) as u64
        }
    }
    #[inline]
    pub fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    #[inline]
    pub fn half(&self, a: u64) -> u64 {
        // In an odd prime field, (a + (a&1)*p)/2 is a/2 mod p.
        // No division or modular multiplication on canonical residues.
        let a = if a < self.p { a } else { a % self.p };
        (a >> 1) + ((a & 1) * ((self.p + 1) >> 1))
    }
    pub fn powm(&self, a: u64, e: u64) -> u64 {
        // Actual production-Rust timing shows the new generic window
        // regresses at the 61-bit Mersenne modulus. Keep the original
        // binary Fp path there; Fp2 still benefits from adaptive rpow.
        if self.p == MAX_PRIME {
            let mut b = a % self.p;
            let mut exponent = e;
            let mut acc = 1u64;
            while exponent > 0 {
                if exponent & 1 != 0 {
                    acc = self.mul(acc, b);
                }
                b = self.sqr(b);
                exponent >>= 1;
            }
            acc
        } else {
            rpow(self, a % self.p, e as u128)
        }
    }
    pub fn inv(&self, a: u64) -> u64 {
        // Extended Euclid replaces ~log(p) Fermat squarings/multiplies on
        // public root-finder inputs. The modulus is prime and < 2^61;
        // Euclidean coefficients and q*t stay within signed i64 bounds.
        // Variable time: NEVER use for secret-dependent field operations.
        let mut r0 = self.p;
        let mut r1 = if a < self.p { a } else { a % self.p };
        if r1 == 0 {
            return 0; // preserve powm(0, p-2)
        }
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            let r2 = r0 - q * r1;
            let t2 = t0 - (q as i64) * t1;
            r0 = r1;
            r1 = r2;
            t0 = t1;
            t1 = t2;
        }
        if t0 < 0 {
            (t0 + self.p as i64) as u64
        } else {
            t0 as u64
        }
    }
}

// ---------- generic finite-field ring for the AMM cube root ----------
pub(crate) trait Ring {
    type E: Copy + PartialEq + std::fmt::Debug;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn sqr(&self, a: Self::E) -> Self::E {
        self.mul(a, a)
    }
    // AMM uses repeated cubing; extension fields can specialize it.
    fn cube(&self, a: Self::E) -> Self::E {
        self.mul(a, self.sqr(a))
    }
    // Override when a field admits a faster powering method.
    fn pow(&self, a: Self::E, e: u128) -> Self::E where Self: Sized {
        rpow(self, a, e)
    }
    fn order(&self) -> u128; // |K^*|
    fn cand(&self, k: u64) -> Self::E; // k-th nonzero candidate (for non-cube search)
}
impl Ring for Fp {
    type E = u64;
    fn zero(&self) -> u64 {
        0
    }
    fn one(&self) -> u64 {
        1
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        Fp::mul(self, a, b)
    }
    fn order(&self) -> u128 {
        (self.p - 1) as u128
    }
    fn cand(&self, k: u64) -> u64 {
        (k + 2) % self.p
    }
}
// F_{p^2} = F_p[i]/(i^2 - n), n a fixed quadratic non-residue.
#[derive(Clone, Copy)]
pub(crate) struct Fq2 {
    pub f: Fp,
    pub n: u64,
}
impl Fq2 {
    #[inline]
    fn mul_nonresidue(&self, x: u64) -> u64 {
        let f = &self.f;
        match self.n {
            2 => f.add(x, x),
            3 => f.add(f.add(x, x), x),
            _ => f.mul(self.n, x),
        }
    }
}
impl Ring for Fq2 {
    type E = (u64, u64);
    fn zero(&self) -> (u64, u64) {
        (0, 0)
    }
    fn one(&self) -> (u64, u64) {
        (1, 0)
    }
    fn mul(&self, x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
        let f = &self.f;
        let ac = f.mul(x.0, y.0);
        let bd = f.mul(x.1, y.1);
        // The first quadratic non-residue is commonly 2 or 3.
        // In these cases, one or two cheap modular additions replace a
        // fourth full base-field multiplication (particularly costly for
        // generic primes using 128-bit remainder).
        let nbd = match self.n {
            2 => f.add(bd, bd),
            3 => f.add(f.add(bd, bd), bd),
            _ => f.mul(self.n, bd),
        };
        let s = f.mul(f.add(x.0, x.1), f.add(y.0, y.1));
        (f.add(ac, nbd), f.sub(f.sub(s, ac), bd))
    }
    // Dedicated Fp2 squaring: (a+bi)^2=(a^2+n*b^2)+2abi.
    // Saves additions/subtractions versus generic Karatsuba multiply.
    fn sqr(&self, x: (u64, u64)) -> (u64, u64) {
        let f = &self.f;
        let aa = f.sqr(x.0);
        let bb = f.sqr(x.1);
        let nbb = match self.n {
            2 => f.add(bb, bb),
            3 => f.add(f.add(bb, bb), bb),
            _ => f.mul(self.n, bb),
        };
        let ab = f.mul(x.0, x.1);
        (f.add(aa, nbb), f.add(ab, ab))
    }
    // (a+bi)^3 = a(a^2+3nb^2) + b(3a^2+nb^2)i.
    // Uses four Fp products, versus six for squaring then multiplying.
    fn cube(&self, x: (u64, u64)) -> (u64, u64) {
        let f = &self.f;
        let aa = f.sqr(x.0);
        let nbb = self.mul_nonresidue(f.sqr(x.1));
        let three_nbb = f.add(nbb, f.add(nbb, nbb));
        let three_aa = f.add(aa, f.add(aa, aa));
        (f.mul(x.0, f.add(aa, three_nbb)),
         f.mul(x.1, f.add(three_aa, nbb)))
    }
    fn order(&self) -> u128 {
        (self.f.p as u128) * (self.f.p as u128) - 1
    }
    // x^p = conjugate(x) in F_{p^2}. Write e = lo + p*hi;
    // simultaneous powering uses ~log2(p) Fp2 squares, not ~2*log2(p).
    // When lo==hi, x^e = Norm(x)^lo lives entirely in Fp.
    // Public-input, variable-time arithmetic only.
    fn pow(&self, x: (u64, u64), e: u128) -> (u64, u64) {
        // Sparse extension-field elements admit base-field exponentiation.
        // This path is variable-time and only for public root-finding inputs.
        let f = &self.f;
        if x.1 == 0 && x.0 != 0 && x.0 < f.p {
            return (f.powm(x.0, (e % (f.p - 1) as u128) as u64), 0);
        }
        if x.0 == 0 && x.1 != 0 && x.1 < f.p {
            let squared = self.mul_nonresidue(f.sqr(x.1));
            let power = f.powm(squared, ((e >> 1) % (f.p - 1) as u128) as u64);
            return if e & 1 == 0 { (power, 0) }
                   else { (0, f.mul(x.1, power)) };
        }
        let p = self.f.p as u128;
        if e < p || e >= self.order() || x == (0, 0)
            || x.0 >= self.f.p || x.1 >= self.f.p
        {
            // Preserve generic support for noncanonical public inputs.
            return rpow(self, x, e);
        }
        let lo = (e % p) as u64;
        let hi = (e / p) as u64;
        let f = &self.f;
        let norm = f.sub(f.sqr(x.0), f.mul(self.n, f.sqr(x.1)));
        if lo == hi {
            return (f.powm(norm, lo), 0);
        }
        let conjugate = (x.0, f.neg(x.1));
        let mut acc = self.one();
        let bits = u64::BITS - (lo | hi).leading_zeros();
        for bit in (0..bits).rev() {
            acc = self.sqr(acc);
            match ((lo >> bit) & 1) | (((hi >> bit) & 1) << 1) {
                1 => acc = self.mul(acc, x),
                2 => acc = self.mul(acc, conjugate),
                3 => acc = (f.mul(acc.0, norm), f.mul(acc.1, norm)),
                _ => {}
            }
        }
        acc
    }
    fn cand(&self, k: u64) -> (u64, u64) {
        (k % self.f.p, 1)
    }
}
pub(crate) fn rpow<R: Ring>(r: &R, a: R::E, mut e: u128) -> R::E {
    // Sliding window of up to four bits: 7 precomputed odd powers, fewer
    // field multiplications for dense exponents in Fp and, especially, Fp2.
    // Sparse or short exponents skip the table: AMM has many cheap 3^k
    // exponents for which precomputation would be a regression.
    let bits = u128::BITS - e.leading_zeros();
    if bits >= 32 && e.count_ones() > (bits + 3) / 4 + 7 {
        let a2 = r.sqr(a);
        let mut odd = [r.one(); 8];
        odd[0] = a;
        for j in 1..8 {
            odd[j] = r.mul(odd[j - 1], a2);
        }
        let mut acc = r.one();
        let mut high = bits as i32 - 1;
        while high >= 0 {
            if ((e >> (high as u32)) & 1) == 0 {
                acc = r.sqr(acc);
                high -= 1;
                continue;
            }
            let mut low = (high - 3).max(0);
            while ((e >> (low as u32)) & 1) == 0 {
                low += 1;
            }
            let width = (high - low + 1) as u32;
            let digit = ((e >> (low as u32)) & ((1u128 << width) - 1)) as usize;
            for _ in low..=high {
                acc = r.sqr(acc);
            }
            acc = r.mul(acc, odd[(digit - 1) / 2]);
            high = low - 1;
        }
        return acc;
    }

    let mut acc = r.one();
    let mut b = a;
    while e > 0 {
        if e & 1 == 1 {
            acc = r.mul(acc, b);
        }
        e >>= 1;
        if e > 0 {
            b = r.sqr(b);
        }
    }
    acc
}

/// Adleman-Manders-Miller cube root in a finite field K with group order N = |K^*|.
/// Per-field precomputation (non-cube, Sylow generator) is done once in `new`.
pub(crate) struct Cube<R: Ring> {
    pub r: R,
    n: u128,
    s: u32,
    e3: u128,
    c: R::E,
    // c^{-1} in the 3-Sylow; computed once for rolling digit corrections.
    c_inv: R::E,
    // Cached digit-correction factors: c^(-3^j) and c^(-2*3^j).
    inverse_powers: Vec<R::E>,
    inverse_squares: Vec<R::E>,
    pub w: R::E,
    p3s: u128,
}
impl<R: Ring> Cube<R> {
    pub fn new(r: R) -> Self {
        let n = r.order();
        let (mut t, mut s) = (n, 0u32);
        while t % 3 == 0 {
            t /= 3;
            s += 1;
        }
        if s == 0 {
            // cubing is a bijection: root = a^(3^{-1} mod N)
            let e3 = if n % 3 == 1 {
                (2 * n + 1) / 3
            } else {
                (n + 1) / 3
            };
            let one = r.one();
            return Cube {
                r,
                n,
                s,
                e3,
                c: one,
                c_inv: one,
                inverse_powers: vec![],
                inverse_squares: vec![],
                w: one,
                p3s: 1,
            };
        }
        let mut k = 0u64;
        let g = loop {
            let g = r.cand(k);
            if r.pow(g, n / 3) != r.one() {
                break g;
            }
            k += 1;
        };
        let c = r.pow(g, t); // generator of the 3-Sylow, order 3^s
        let p3s = 3u128.pow(s);
        let c_inv = if s >= 2 { r.pow(c, p3s - 1) } else { r.one() };
        let w = r.pow(c, p3s / 3); // primitive cube root of unity
        let e3 = if t % 3 == 1 {
            (2 * t + 1) / 3
        } else {
            (t + 1) / 3
        }; // 3*e3 = 1 mod t
        // Precompute only the factors needed by digits 1..s-1.
        // The s<=2 specialized paths do not need these arrays.
        let mut inverse_powers = Vec::new();
        let mut inverse_squares = Vec::new();
        if s >= 3 {
            inverse_powers.reserve((s - 1) as usize);
            inverse_squares.reserve((s - 1) as usize);
            let mut value = c_inv;
            for j in 0..s - 1 {
                inverse_powers.push(value);
                inverse_squares.push(r.sqr(value));
                if j + 2 < s {
                    value = r.cube(value);
                }
            }
        }
        Cube {
            r,
            n,
            s,
            e3,
            c,
            c_inv,
            inverse_powers,
            inverse_squares,
            w,
            p3s,
        }
    }
    pub fn bijective(&self) -> bool {
        self.s == 0
    }
    /// One cube root of `a`, or None if `a` is not a cube.
    pub fn root(&self, a: R::E) -> Option<R::E> {
        let r = &self.r;
        if a == r.zero() {
            return Some(a);
        }
        if self.s == 0 {
            return Some(r.pow(a, self.e3));
        }
        // For v3(|K*|)=1, a is a cube iff (a^e3)^3 == a.
        // Verify the candidate rather than computing a second full power.
        // Variable-time public rootfinder inputs only.
        if self.s == 1 {
            let x0 = r.pow(a, self.e3);
            return if r.cube(x0) == a { Some(x0) } else { None };
        }
        // N = t * 3^s. Reuse a^t for both the cube-residuosity
        // test (a^t)^(3^(s-1)) and the Sylow correction h.
        // Since 3*e3-1 is either t or 2*t, this removes one
        // full-length exponentiation from the AMM hot path.
        let t = self.n / self.p3s;
        let g = r.pow(a, t);
        let mut residue = g;
        for _ in 1..self.s {
            residue = r.cube(residue);
        }
        if residue != r.one() {
            return None;
        }
        let x0 = r.pow(a, self.e3); // x0^3 = a * h, h in the 3-Sylow
        let h = if t % 3 == 1 { r.mul(g, g) } else { g };
        // For s=1, a cube has h=1 and x0 is already a root.
        if self.s == 1 {
            return Some(x0);
        }
        // Membership guarantees h^(3^(s-1)) = 1 for cubes. If h=1,
        // the initial Cardano/AMM power is already a cube root.
        // Only branch here for s>=3: smaller cases had mixed perf results.
        if self.s >= 3 && h == r.one() {
            return Some(x0);
        }
        let w2 = r.mul(self.w, self.w);
        // For s=2, cube membership forces h into <w> of order 3.
        // The correction is c^{-d} for h=w^d, d=0,1,2.
        if self.s == 2 {
            let factor = if h == r.one() {
                r.one()
            } else if h == self.w {
                self.c_inv
            } else if h == w2 {
                r.mul(self.c_inv, self.c_inv)
            } else {
                return None;
            };
            return Some(r.mul(x0, factor));
        }
        // General s: find 3-adic digits of log_c(h), but maintain
        // c^{-l} incrementally instead of recomputing c^{-l} for each
        // digit. For cubes, the low digit is zero, so accumulate
        // c^{-l/3} at the same time and eliminate the final power.
        // Public root-finding inputs only: branches are variable-time.
        let mut correction = r.one();
        let mut root_correction = r.one();
        // The low 3-adic digit is guaranteed zero by the cube-residuosity
        // test above. Skip its s-1 repeated cubings and start at digit 1.
        // These are the states the original digit-0 loop left behind.
        for j in 1..self.s {
            let mut z = r.mul(h, correction);
            // z = (h*c^{-l})^(3^(s-1-j)). Repeated cubing uses only
            // 2(s-1-j) field multiplications, not binary exponentiation.
            for _ in (j + 1)..self.s {
                z = r.cube(z);
            }
            let digit = if z == r.one() {
                0
            } else if z == self.w {
                1
            } else if z == w2 {
                2
            } else {
                return None;
            };
            if digit != 0 {
                let k = (j - 1) as usize;
                let factor = if digit == 1 {
                    self.inverse_powers[k]
                } else {
                    self.inverse_squares[k]
                };
                root_correction = r.mul(root_correction, factor);
                if j + 1 < self.s {
                    let k = j as usize;
                    let term = if digit == 1 {
                        self.inverse_powers[k]
                    } else {
                        self.inverse_squares[k]
                    };
                    correction = r.mul(correction, term);
                }
            }
        }
        Some(r.mul(x0, root_correction))
    }

    #[cfg(test)]
    fn root_before_skip_digit(&self, a: R::E) -> Option<R::E> {
        let r = &self.r;
        if a == r.zero() {
            return Some(a);
        }
        if self.s == 0 {
            return Some(r.pow(a, self.e3));
        }
        // N = t * 3^s. Reuse a^t for both the cube-residuosity
        // test (a^t)^(3^(s-1)) and the Sylow correction h.
        // Since 3*e3-1 is either t or 2*t, this removes one
        // full-length exponentiation from the AMM hot path.
        let t = self.n / self.p3s;
        let g = r.pow(a, t);
        let mut residue = g;
        for _ in 1..self.s {
            residue = r.cube(residue);
        }
        if residue != r.one() {
            return None;
        }
        let x0 = r.pow(a, self.e3); // x0^3 = a * h, h in the 3-Sylow
        let h = if t % 3 == 1 { r.mul(g, g) } else { g };
        // For s=1, a cube has h=1 and x0 is already a root.
        if self.s == 1 {
            return Some(x0);
        }
        let w2 = r.mul(self.w, self.w);
        // For s=2, cube membership forces h into <w> of order 3.
        // The correction is c^{-d} for h=w^d, d=0,1,2.
        if self.s == 2 {
            let factor = if h == r.one() {
                r.one()
            } else if h == self.w {
                self.c_inv
            } else if h == w2 {
                r.mul(self.c_inv, self.c_inv)
            } else {
                return None;
            };
            return Some(r.mul(x0, factor));
        }
        // General s: find 3-adic digits of log_c(h), but maintain
        // c^{-l} incrementally instead of recomputing c^{-l} for each
        // digit. For cubes, the low digit is zero, so accumulate
        // c^{-l/3} at the same time and eliminate the final power.
        // Public root-finding inputs only: branches are variable-time.
        let mut correction = r.one();
        let mut root_correction = r.one();
        let mut inverse_power = self.c_inv; // c^{-3^j}
        let mut previous_power = r.one();   // c^{-3^(j-1)}
        for j in 0..self.s {
            let mut z = if j == 0 {
                h
            } else {
                r.mul(h, correction)
            };
            // z = (h*c^{-l})^(3^(s-1-j)). Repeated cubing uses only
            // 2(s-1-j) field multiplications, not binary exponentiation.
            for _ in (j + 1)..self.s {
                z = r.cube(z);
            }
            let digit = if z == r.one() {
                0
            } else if z == self.w {
                1
            } else if z == w2 {
                2
            } else {
                return None;
            };
            if digit != 0 {
                // The low digit of a cube's 3-Sylow log must be zero.
                if j == 0 {
                    return None;
                }
                let factor = if digit == 1 {
                    previous_power
                } else {
                    r.mul(previous_power, previous_power)
                };
                root_correction = r.mul(root_correction, factor);
                if j + 1 < self.s {
                    let term = if digit == 1 {
                        inverse_power
                    } else {
                        r.mul(inverse_power, inverse_power)
                    };
                    correction = r.mul(correction, term);
                }
            }
            previous_power = inverse_power;
            if j + 1 < self.s {
                inverse_power = r.cube(inverse_power);
            }
        }
        Some(r.mul(x0, root_correction))
    }
}

/// Per-prime context: one-time precomputation (non-residues, 3-Sylow generators, constants).
pub struct RadicalRootCtx {
    pub(crate) f: Fp,
    inv2: u64,
    inv3: u64,
    inv4: u64,
    inv27: u64,
    inv_n: u64,
    n: u64, // quadratic non-residue
    s2: u32,
    t2: u64,
    zc: u64, // Tonelli-Shanks: p-1 = 2^s2 t2, zc = n^t2
    cube_p: Cube<Fp>,
    cube_q: Cube<Fq2>,
    w2: (u64, u64), // primitive cube root of unity in F_{p^2}
}
impl RadicalRootCtx {
    pub fn new(p: u64) -> Result<Self, String> {
        if !(5..=MAX_PRIME).contains(&p) || !is_prime_u64(p) {
            return Err("radical root finder needs a prime 5 <= p <= 2^61-1".into());
        }
        let f = Fp::new(p);
        let n = {
            let mut z = 2u64;
            while f.powm(z, (p - 1) / 2) == 1 {
                z += 1;
            }
            z
        };
        let (mut t2, mut s2) = (p - 1, 0u32);
        while t2 & 1 == 0 {
            t2 >>= 1;
            s2 += 1;
        }
        let zc = f.powm(n, t2);
        let cube_p = Cube::new(f);
        let cube_q = Cube::new(Fq2 { f, n });
        let w2 = cube_q.w;
        let i4 = f.inv(4 % p);
        let i27 = f.inv(27 % p);
        Ok(RadicalRootCtx {
            f,
            inv2: f.inv(2),
            inv3: f.inv(3),
            inv4: i4,
            inv27: i27,
            inv_n: f.inv(n),
            n,
            s2,
            t2,
            zc,
            cube_p,
            cube_q,
            w2,
        })
    }
    /// Square root in F_p (p%4==3: one pow; else Tonelli-Shanks), None for non-residues.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let f = &self.f;
        let p = f.p;
        if a == 0 {
            return Some(0);
        }
        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        // p = 5 (mod 8): one exponentiation and one optional multiplication
        // by the precomputed square root of -1 (zc). Final square verifies
        // both roots and nonresidue rejection.
        if p % 8 == 5 {
            let mut root = f.powm(a, (p + 3) / 8);
            if f.sqr(root) == a {
                return Some(root);
            }
            root = f.mul(root, self.zc);
            return if f.sqr(root) == a { Some(root) } else { None };
        }
        // The t2=1 shortcut must retain powm's canonical reduction even
        // when a caller passes an unreduced public element.
        let a = if self.t2 == 1 && a >= p { a % p } else { a };
        // Euler's criterion from a^t2, reused by Tonelli-Shanks:
        // a^((p-1)/2) = (a^t2)^(2^(s2-1)).
        // Avoid a second full-length modular exponentiation.
        // t2 is odd. Let h=a^((t2-1)/2): h^2*a=a^t2 and h*a=a^((t2+1)/2).
        // This shares one exponentiation between Tonelli's t and r states.
        // For t2=1, avoid powering and multiplication entirely.
        let (mut t, half_power) = if self.t2 == 1 {
            (a, 1)
        } else if self.t2 >= 1_024 {
            let h = f.powm(a, (self.t2 - 1) / 2);
            (f.mul(f.sqr(h), a), h)
        } else {
            // Small exponents can be cheaper as two binary powers;
            // the fused path showed compiler-dependent regressions here.
            (f.powm(a, self.t2), 0)
        };
        let mut legendre = t;
        for _ in 1..self.s2 {
            legendre = f.sqr(legendre);
        }
        if legendre != 1 {
            return None;
        }
        let mut m = self.s2;
        let mut c = self.zc;
        let mut r = if self.t2 == 1 {
            a
        } else if self.t2 >= 1_024 {
            f.mul(half_power, a)
        } else {
            f.powm(a, self.t2.div_ceil(2))
        };
        loop {
            if t == 1 {
                return Some(r);
            }
            let mut i = 0u32;
            let mut t2 = t;
            while t2 != 1 && i < m {
                t2 = f.sqr(t2);
                i += 1;
            }
            if i == m {
                return None;
            }
            // Power-of-two exponent: repeated squaring is cheaper
            // than the generic binary-exponentiation loop.
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = f.sqr(b);
            }
            m = i;
            c = f.sqr(b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
    }
    /// Distinct roots of monic y^2 + b y + c.
    pub fn quad_monic(&self, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        // y(y+b)=0 has roots 0 and -b; no Legendre/sqrt needed.
        if c == 0 {
            return if b == 0 { vec![0] } else { vec![0, f.neg(b)] };
        }
        // Four times c needs only two reduced doublings, not a
        // generic base-field multiply (wide remainder on most p).
        let twice_c = f.add(c, c);
        let disc = f.sub(f.sqr(b), f.add(twice_c, twice_c));
        match self.sqrt(disc) {
            None => vec![],
            Some(0) => vec![f.half(f.neg(b))],
            Some(s) => {
                let nb = f.neg(b);
                // Rust benchmarks show Vieta's subtraction is faster
                // for the 61-bit modulus but slower for native 32-bit primes.
                // Retain the original second multiplication on small fields.
                let first = f.half(f.add(nb, s));
                let second = if f.p > u32::MAX as u64 {
                    f.sub(nb, first)
                } else {
                    f.half(f.sub(nb, s))
                };
                vec![first, second]
            }
        }
    }
    /// General quadratic a x^2 + b x + c (a != 0), sorted distinct roots.
    pub fn quad_roots(&self, a: u64, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        // Preserve support for noncanonical public coefficients.
        let a = if a < f.p { a } else { a % f.p };
        let b = if b < f.p { b } else { b % f.p };
        let c = if c < f.p { c } else { c % f.p };
        if a == 1 {
            let mut roots = self.quad_monic(b, c);
            roots.sort_unstable();
            roots.dedup();
            return roots;
        }
        // ax²+bx = x(ax+b): invert a once, skip the square-root path.
        if c == 0 {
            return if b == 0 {
                vec![0]
            } else {
                vec![0, f.mul(f.neg(b), f.inv(a))]
            };
        }
        // Test b^2-4ac before inverting 2a. For a nonresidue
        // discriminant, skip the expensive modular inversion entirely.
        let ac = f.mul(a, c);
        let twice_ac = f.add(ac, ac);
        let disc = f.sub(f.sqr(b), f.add(twice_ac, twice_ac));
        let s = match self.sqrt(disc) {
            Some(s) => s,
            None => return vec![],
        };
        let inv_2a = f.inv(f.add(a, a));
        let nb = f.neg(b);
        let first = f.mul(f.add(nb, s), inv_2a);
        let mut roots = if s == 0 {
            vec![first]
        } else {
            vec![first, f.mul(f.sub(nb, s), inv_2a)]
        };
        roots.sort_unstable();
        roots.dedup();
        roots
    }
    /// F_p-roots of the depressed cubic t^3 + P t + Q = 0 (Cardano; F_{p^2} when D is a non-residue).
    pub fn cubic_depressed(&self, pp: u64, q: u64) -> Vec<u64> {
        let f = &self.f;
        // x^3 + P*x = x*(x^2 + P): no Cardano or AMM cube root.
        if q == 0 {
            if pp == 0 { return vec![0]; }
            return match self.sqrt(f.neg(pp)) {
                None => vec![0],
                Some(s) => {
                    let mut roots = vec![0, s, f.neg(s)];
                    roots.sort_unstable();
                    roots.dedup();
                    roots
                }
            };
        }
        if pp == 0 {
            if q == 0 {
                return vec![0];
            }
            return match self.cube_p.root(f.neg(q)) {
                None => vec![],
                Some(u) => {
                    if self.cube_p.bijective() {
                        vec![u]
                    } else {
                        let w = self.cube_p.w;
                        let uw = f.mul(u, w);
                        vec![u, uw, f.mul(uw, w)]
                    }
                }
            };
        }
        let q2 = f.half(q);
        // Keep the original discriminant: algebraic m^3 reuse measured
        // 0.61-0.69x in actual Rust CI despite faster C++ screening.
        let p2 = f.sqr(pp);
        let d = f.add(f.sqr(q2), f.mul(f.mul(p2, pp), self.inv27));
        if d == 0 {
            // double root -3Q/(2P), simple root 3Q/P
            let ip = f.inv(pp);
            let triple_q = f.add(f.add(q, q), q);
            let r1 = f.mul(triple_q, ip);
            let r2 = f.half(f.neg(r1));
            return vec![r1, r2];
        }
        let nq2 = f.neg(q2);
        let m = f.neg(f.mul(pp, self.inv3));
        match self.sqrt(d) {
            Some(sg) => {
                let u3 = f.add(nq2, sg);
                match self.cube_p.root(u3) {
                    None => vec![],
                    Some(u) => {
                        let v = f.mul(m, f.inv(u));
                        if self.cube_p.bijective() {
                            vec![f.add(u, v)]
                        } else {
                            let w = self.cube_p.w;
                            let w_2 = f.sqr(w);
                            let (mut uk, mut vk) = (u, v);
                            let mut out = vec![f.add(u, v)];
                            for _ in 0..2 {
                                uk = f.mul(uk, w);
                                vk = f.mul(vk, w_2);
                                out.push(f.add(uk, vk));
                            }
                            out
                        }
                    }
                }
            }
            None => {
                // casus irreducibilis route: sqrt(D) = i * sqrt(D/n) in F_{p^2}
                let sg = self
                    .sqrt(f.mul(d, self.inv_n))
                    .expect("D/n must be a residue");
                let u3 = (nq2, sg);
                let k = &self.cube_q;
                let u = match k.root(u3) {
                    None => return vec![],
                    Some(u) => u,
                };
                // 1/u = conj(u)/N(u)
                let nrm = f.sub(f.sqr(u.0), f.mul(self.n, f.sqr(u.1)));
                let ni = f.inv(nrm);
                let uinv = (f.mul(u.0, ni), f.mul(f.neg(u.1), ni));
                let v = (f.mul(m, uinv.0), f.mul(m, uinv.1));
                let w = self.w2;
                let w_2 = k.r.mul(w, w);
                let mut out = vec![];
                let (mut uk, mut vk) = (u, v);
                for kk in 0..3 {
                    if kk > 0 {
                        uk = k.r.mul(uk, w);
                        vk = k.r.mul(vk, w_2);
                    }
                    let im = f.add(uk.1, vk.1);
                    if im == 0 {
                        out.push(f.add(uk.0, vk.0));
                    }
                }
                out
            }
        }
    }
    /// F_p-roots of the monic cubic x^3 + c2 x^2 + c1 x + c0.
    pub fn cubic_monic(&self, c2: u64, c1: u64, c0: u64) -> Vec<u64> {
        let f = &self.f;
        // x divides a cubic with zero constant; reduce to quadratic roots.
        if c0 == 0 {
            let mut roots = self.quad_monic(c2, c1);
            roots.push(0);
            roots.sort_unstable();
            roots.dedup();
            return roots;
        }
        // Already depressed: avoid cubic translation and coefficient products.
        if c2 == 0 { return self.cubic_depressed(c1, c0); }
        // Shift x=t-sh, sh=c2/3. Reuse sh:
        // P=c1-c2*sh; Q=2*sh^3-c1*sh+c0.
        // Five field multiplications instead of seven.
        let sh = f.mul(c2, self.inv3);
        let pp = f.sub(c1, f.mul(c2, sh));
        let sh3 = f.mul(f.sqr(sh), sh);
        let q = f.add(f.sub(f.add(sh3, sh3), f.mul(c1, sh)), c0);
        self.cubic_depressed(pp, q)
            .into_iter()
            .map(|t| f.sub(t, sh))
            .collect()
    }
    // Quartic translation x=y-b/4. For canonical residues, modular doubling
    // avoids five general multiplications by the small constants 2,3,6,8.
    #[inline]
    fn quartic_depressed_coeffs(
        &self, b: u64, c: u64, d: u64, e: u64,
    ) -> (u64, u64, u64, u64) {
        let f = &self.f;
        // Native Rust benchmarking on p=2^61-1 showed ~4% regression
        // for the small-constant rewrite. Retain the former path there.
        if f.p == MAX_PRIME {
            let bb = f.mul(b, self.inv4);
            let b2 = f.sqr(bb);
            let pp = f.sub(c, f.mul(6, b2));
            let b3 = f.mul(b2, bb);
            let qq = f.add(f.sub(d, f.mul(f.mul(2, bb), c)), f.mul(8, b3));
            let rr = f.sub(
                f.add(f.sub(e, f.mul(bb, d)), f.mul(b2, c)),
                f.mul(3, f.sqr(b2)),
            );
            return (bb, pp, qq, rr);
        }
        let bb = f.mul(b, self.inv4);
        let b2 = f.sqr(bb);
        let twice_b2 = f.add(b2, b2);
        let six_b2 = f.add(twice_b2, f.add(twice_b2, twice_b2));
        let pp = f.sub(c, six_b2);
        let b3 = f.mul(b2, bb);
        let twice_b3 = f.add(b3, b3);
        let four_b3 = f.add(twice_b3, twice_b3);
        let eight_b3 = f.add(four_b3, four_b3);
        let qq = f.add(f.sub(d, f.mul(bb, f.add(c, c))), eight_b3);
        let b4 = f.sqr(b2);
        let three_b4 = f.add(f.add(b4, b4), b4);
        let rr = f.sub(
            f.add(f.sub(e, f.mul(bb, d)), f.mul(b2, c)),
            three_b4,
        );
        (bb, pp, qq, rr)
    }

    #[inline]
    fn quartic_resolvent_coeffs(&self, pp: u64, qq: u64, rr: u64) -> (u64, u64, u64) {
        let f = &self.f;
        let twice_rr = f.add(rr, rr);
        (f.add(pp, pp), f.sub(f.sqr(pp), f.add(twice_rr, twice_rr)), f.neg(f.sqr(qq)))
    }

    /// Distinct F_p roots of a x^4 + b x^3 + c x^2 + d x + e (a != 0). Returns (sorted roots, used_fallback).
    pub(crate) fn quartic(
        &self,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    ) -> Result<(Vec<u64>, bool), String> {
        let f = &self.f;
        // For nonmonic even quartics, set z=x^2 and solve a*z^2+c*z+e.
        // Avoid Ferrari depression and coefficient normalization.
        // Keep monic even quartics on the old path: local GCC/Clang screening
        // found that direct reduction neutral or slightly slower there.
        if a != 1 && b == 0 && d == 0 {
            let mut roots = Vec::new();
            for z in self.quad_roots(a, c, e) {
                if let Some(s) = self.sqrt(z) {
                    roots.push(s);
                    if s != 0 { roots.push(f.neg(s)); }
                }
            }
            roots.sort_unstable();
            roots.dedup();
            return Ok((roots, false));
        }
        // Factor out x before monic normalization for exact-zero constants.
        if e == 0 {
            let mut roots = if a == 1 {
                self.cubic_monic(b, c, d)
            } else {
                let ia = f.inv(a);
                self.cubic_monic(f.mul(b, ia), f.mul(c, ia), f.mul(d, ia))
            };
            roots.push(0);
            roots.sort_unstable();
            roots.dedup();
            return Ok((roots, false));
        }
        // monic
        let (b1, c1, d1, e1) = if a == 1 {
            (b, c, d, e)
        } else {
            let ia = f.inv(a);
            (f.mul(b, ia), f.mul(c, ia), f.mul(d, ia), f.mul(e, ia))
        };
        // x divides the quartic when e1=0. Solve the cubic cofactor,
        // bypassing Ferrari's resolvent and the Frobenius fallback.
        if e1 == 0 {
            let mut roots = self.cubic_monic(b1, c1, d1);
            roots.push(0);
            roots.sort_unstable();
            roots.dedup();
            return Ok((roots, false));
        }
        // x = y - B, B = b1/4
        // A zero cubic coefficient means the quartic is already depressed.
        // Bypass seven or more field operations while retaining the dense path.
        let (bb, pp, qq, rr) = if b1 == 0 {
            (0, c1, d1, e1)
        } else {
            self.quartic_depressed_coeffs(b1, c1, d1, e1)
        };
        let mut ys: Vec<u64> = vec![];
        let mut fallback = false;
        if qq == 0 {
            for z in self.quad_monic(pp, rr) {
                match self.sqrt(z) {
                    None => {}
                    Some(0) => ys.push(0),
                    Some(s) => {
                        ys.push(s);
                        ys.push(f.neg(s));
                    }
                }
            }
        } else {
            // resolvent in z = (r1+r2)^2:  z^3 + 2P z^2 + (P^2 - 4R) z - Q^2 = 0
            let (z2, z1, z0) = self.quartic_resolvent_coeffs(pp, qq, rr);
            let zs = self.cubic_monic(z2, z1, z0);
            if zs.is_empty() {
                // Frobenius type (1,3): no rational pairing; the unique root needs F_{p^3} radicals.
                // Use the distinct-degree (CZ) path on the depressed quartic - NOT radical.
                fallback = true;
                ys = frobenius_roots(f.p, &[rr, qq, pp, 0, 1])?;
            } else {
                for &z in &zs {
                    if let Some(al) = self.sqrt(z) {
                        // quartic = (y^2 + al y + be)(y^2 - al y + ga)
                        let qa = f.mul(qq, f.inv(al));
                        let s = f.add(pp, z);
                        let be = f.half(f.sub(s, qa));
                        let ga = f.half(f.add(s, qa));
                        ys.extend(self.quad_monic(al, be));
                        ys.extend(self.quad_monic(f.neg(al), ga));
                        break;
                    }
                }
            }
        }
        let mut xs: Vec<u64> = ys.into_iter().map(|y| f.sub(y, bb)).collect();
        xs.sort_unstable();
        xs.dedup();
        Ok((xs, fallback))
    }
}

impl RadicalRootCtx {
    /// Distinct F_p roots of `f` (coefficients low to high, any trailing zeros allowed).
    /// Degrees 1..=4 are supported; the zero polynomial and constants return an error / no roots.
    pub fn roots(&self, f: &[u64]) -> Result<RadicalRoots, String> {
        let fp = &self.f;
        // Polynomials of degree <=4 fit in a stack buffer. Scan trailing
        // zeroes first (including coefficients congruent to 0 mod p) so
        // arbitrarily padded input remains accepted without a heap Vec.
        let mut n = f.len();
        while n > 0 && f[n - 1] % fp.p == 0 {
            n -= 1;
        }
        if n > 5 {
            return Err("radical root finder supports degree <= 4".into());
        }
        let mut c = [0u64; 5];
        for j in 0..n {
            c[j] = f[j] % fp.p;
        }
        let ok = |roots: Vec<u64>| {
            Ok(RadicalRoots {
                roots,
                frobenius_fallback: false,
            })
        };
        match n {
            0 => Err("zero polynomial".into()),
            1 => ok(vec![]),
            2 => {
                // x + c0 and a*x have roots without an inversion.
                let root = if c[0] == 0 { 0 }
                    else if c[1] == 1 { fp.neg(c[0]) }
                    else { fp.mul(fp.neg(c[0]), fp.inv(c[1])) };
                ok(vec![root])
            },
            3 => ok(self.quad_roots(c[2], c[1], c[0])),
            4 => {
                // Factor x before normalizing a nonmonic cubic.
                if c[0] == 0 {
                    let mut roots = self.quad_roots(c[3], c[2], c[1]);
                    roots.push(0);
                    roots.sort_unstable();
                    roots.dedup();
                    return ok(roots);
                }
                // Monic cubics are common in normalized rootfinding.
                // Avoid one inversion and three modular products.
                let mut v = if c[3] == 1 {
                    self.cubic_monic(c[2], c[1], c[0])
                } else {
                    let ia = fp.inv(c[3]);
                    self.cubic_monic(fp.mul(c[2], ia), fp.mul(c[1], ia), fp.mul(c[0], ia))
                };
                v.sort_unstable();
                v.dedup();
                ok(v)
            }
            5 => {
                let (roots, fb) = self.quartic(c[4], c[3], c[2], c[1], c[0])?;
                Ok(RadicalRoots {
                    roots,
                    frobenius_fallback: fb,
                })
            }
            _ => Err("radical root finder supports degree <= 4".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Frozen immediate-pre-optimization Tonelli path (no p=5 mod 8 shortcut).
    fn sqrt_before_mod8_5(ctx: &RadicalRootCtx, a: u64) -> Option<u64> {
        let f = &ctx.f;
        if a == 0 { return Some(0); }
        if f.p % 4 == 3 {
            let r = f.powm(a, (f.p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        let a = if ctx.t2 == 1 && a >= f.p { a % f.p } else { a };
        let (mut t, half_power) = if ctx.t2 == 1 {
            (a, 1)
        } else if ctx.t2 >= 1_024 {
            let h = f.powm(a, (ctx.t2 - 1) / 2);
            (f.mul(f.sqr(h), a), h)
        } else {
            (f.powm(a, ctx.t2), 0)
        };
        let mut legendre = t;
        for _ in 1..ctx.s2 { legendre = f.sqr(legendre); }
        if legendre != 1 { return None; }
        let mut m = ctx.s2;
        let mut c = ctx.zc;
        let mut r = if ctx.t2 == 1 { a }
            else if ctx.t2 >= 1_024 { f.mul(half_power, a) }
            else { f.powm(a, ctx.t2.div_ceil(2)) };
        loop {
            if t == 1 { return Some(r); }
            let mut i = 0u32;
            let mut t2 = t;
            while t2 != 1 && i < m { t2 = f.sqr(t2); i += 1; }
            if i == m { return None; }
            let mut b = c;
            for _ in 0..(m - i - 1) { b = f.sqr(b); }
            m = i;
            c = f.sqr(b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
    }

    #[derive(Clone, Copy)]
    struct Fq2BeforeCube(Fq2);
    impl Ring for Fq2BeforeCube {
        type E = (u64, u64);
        fn zero(&self) -> Self::E { self.0.zero() }
        fn one(&self) -> Self::E { self.0.one() }
        fn mul(&self, x: Self::E, y: Self::E) -> Self::E { self.0.mul(x, y) }
        fn sqr(&self, x: Self::E) -> Self::E { self.0.sqr(x) }
        // Intentionally uses Ring::cube default (old sqr+mul).
        fn pow(&self, x: Self::E, e: u128) -> Self::E { self.0.pow(x, e) }
        fn order(&self) -> u128 { self.0.order() }
        fn cand(&self, k: u64) -> Self::E { self.0.cand(k) }
    }

    #[test]
    fn fp2_cube_norm_and_mod8_sqrt_match_previous_algorithm() {
        let primes = [13u64, 29, 101, 487, 17_497, 998_244_353,
                      1_000_000_009, MAX_PRIME];
        let mut state = 0x631a_ef35_8f61_2071u64;
        for p in primes {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let q = Fq2 { f: ctx.f, n: ctx.n };
            for _ in 0..2048 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = state % p;
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = state % p;
                let x = (a,b);
                assert_eq!(q.cube(x), q.mul(x, q.sqr(x)), "cube p={p}");
            }
        }
        for p in [13u64, 29, 101, 1_000_037, 536_870_909,
                  1_000_000_021, 2_147_483_693] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            assert_eq!(p % 8, 5);
            for _ in 0..4096 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = state % p;
                for v in [a, ctx.f.sqr(a), ctx.f.mul(ctx.n, ctx.f.sqr(a))] {
                    let old = sqrt_before_mod8_5(&ctx, v);
                    let new = ctx.sqrt(v);
                    assert_eq!(old.is_some(), new.is_some(), "sqrt status p={p} v={v}");
                    if let Some(r) = new {
                        assert_eq!(ctx.f.sqr(r), v, "bad sqrt p={p} v={v}");
                    }
                }
            }
        }
        for p in [487u64, 17_497, 1_000_000_009] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let old = Cube::new(Fq2BeforeCube(ctx.cube_q.r));
            for _ in 0..128 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = state % p;
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = state % p;
                assert_eq!(old.root((a,b)), ctx.cube_q.root((a,b)),
                           "Fp2 AMM p={p}");
            }
        }
    }

    #[test]
    #[ignore]
    fn bench_fp2_cubing_and_mod8_sqrt_actual_rust() {
        fn sample<T: Copy, F: Fn(T) -> u64>(xs: &[T], reps: usize, f: &F) -> f64 {
            let start = std::time::Instant::now();
            let mut check = 0u64;
            for _ in 0..reps {
                for &x in xs {
                    check = check.wrapping_add(std::hint::black_box(
                        f(std::hint::black_box(x))
                    ));
                }
            }
            std::hint::black_box(check);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<T: Copy, B: Fn(T)->u64, A: Fn(T)->u64>(
            name: &str, xs: &[T], reps: usize, before: B, after: A,
        ) {
            let mut old = Vec::new();
            let mut new = Vec::new();
            for i in 0..13 {
                if i % 2 == 0 {
                    old.push(sample(xs, reps, &before));
                    new.push(sample(xs, reps, &after));
                } else {
                    new.push(sample(xs, reps, &after));
                    old.push(sample(xs, reps, &before));
                }
            }
            old.sort_by(|x,y| x.total_cmp(y));
            new.sort_by(|x,y| x.total_cmp(y));
            let (a,b) = (old[6],new[6]);
            println!("{name} before={a:.3}ns after={b:.3}ns speedup={:.3}x",a/b);
        }
        let mut seed = 0xadef_6143_2026_1118u64;
        for p in [487u64,17_497,998_244_353,1_000_000_009,MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let q = ctx.cube_q.r;
            let mut xs = Vec::new();
            for _ in 0..512 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = seed % p;
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = seed % p;
                xs.push((a,b));
            }
            paired(&format!("p={p} fp2 cube"), &xs, 100,
                   |x| { let y=q.mul(x,q.sqr(x)); y.0 ^ y.1 },
                   |x| { let y=q.cube(x); y.0 ^ y.1 });
            paired(&format!("p={p} seven fp2 cubes"), &xs, 25,
                   |mut x| { for _ in 0..7 { x=q.mul(x,q.sqr(x)); } x.0 ^ x.1 },
                   |mut x| { for _ in 0..7 { x=q.cube(x); } x.0 ^ x.1 });
        }
        // Complete Fp2 AMM root extraction, old generic cubing vs new formula.
        for p in [487u64, 17_497, 1_000_000_009] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let q = ctx.cube_q.r;
            let old = Cube::new(Fq2BeforeCube(q));
            let mut cubes = Vec::new();
            let mut mixed = Vec::new();
            for _ in 0..256 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = seed % p;
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = seed % p;
                let x = (a,b);
                cubes.push(q.cube(x));
                mixed.push(x);
            }
            for (name,xs) in [("cubes",cubes),("mixed",mixed)] {
                paired(&format!("p={p} full fp2 AMM {name}"),&xs,16,
                       |x| { let y=old.root(x).unwrap_or((u64::MAX,u64::MAX)); y.0 ^ y.1 },
                       |x| { let y=ctx.cube_q.root(x).unwrap_or((u64::MAX,u64::MAX)); y.0 ^ y.1 });
            }
        }
        for p in [536_870_909u64, 1_000_000_021, 2_147_483_693] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let mut squares = Vec::new();
            let mut nonsquares = Vec::new();
            for _ in 0..512 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let v = seed % p;
                squares.push(ctx.f.sqr(v));
                nonsquares.push(ctx.f.mul(ctx.n,ctx.f.sqr(v)));
            }
            for (name,xs) in [("squares",squares),("nonsquares",nonsquares)] {
                paired(&format!("p={p} sqrt {name}"), &xs, 35,
                       |x| sqrt_before_mod8_5(&ctx,x).unwrap_or(u64::MAX),
                       |x| ctx.sqrt(x).unwrap_or(u64::MAX));
            }
        }
    }


    #[test]
    fn frobenius_fp2_power_matches_frozen_binary_and_cube_root() {
        let mut seed = 0x52f1_911a_2026_1118u64;
        for &p in &[5u64, 7, 11, 17, 97, 487, 17_497, 998_244_353,
                    1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let q = &ctx.cube_q.r;
            let n = q.order();
            let mut t = n;
            while t % 3 == 0 { t /= 3; }
            let e3 = if t % 3 == 1 { (2 * t + 1) / 3 } else { (t + 1) / 3 };
            let exp = [0, 1, p as u128 - 1, p as u128, p as u128 + 1,
                       n / 3, t, e3, n - 1, n, u128::MAX];
            for j in 0..128 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = if j == 0 { (0, 0) } else { (seed % p, (seed >> 7) % p) };
                for &e in &exp {
                    assert_eq!(q.pow(a, e), rpow(q, a, e),
                               "Fp2 Frobenius p={p} a={a:?} e={e}");
                }
                let random_e = (((seed as u128) << 59) | ((seed >> 5) as u128)) % n;
                assert_eq!(q.pow(a, random_e), rpow(q, a, random_e),
                           "random exponent p={p} e={random_e}");
                if j == 1 {
                    let raw = (p + 1, p + 2);
                    assert_eq!(q.pow(raw, n / 3), rpow(q, raw, n / 3),
                               "noncanonical Fp2 p={p}");
                }
                if j < 24 {
                    assert_eq!(ctx.cube_q.root(a), previous_amm_root(&ctx.cube_q, a),
                               "AMM Fp2 p={p} a={a:?}");
                }
            }
        }
    }

    #[test]
    #[ignore = "paired native production-Rust Fp2 Frobenius exponentiation benchmark"]
    fn bench_frobenius_fp2_power() {
        use std::hint::black_box;
        use std::time::Instant;
        let mut seed = 0x50ff_ee12_2026_1118u64;
        for &p in &[487u64, 17_497, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let q = &ctx.cube_q.r;
            let n = q.order();
            let mut t = n;
            while t % 3 == 0 { t /= 3; }
            let e3 = if t % 3 == 1 { (2 * t + 1) / 3 } else { (t + 1) / 3 };
            let mut xs = Vec::with_capacity(512);
            for _ in 0..512 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                xs.push((seed % p, (seed >> 11) % p));
            }
            for &(name, e) in &[("residuosity N/3", n / 3), ("AMM e3", e3), ("AMM t", t)] {
                for &x in &xs { assert_eq!(rpow(q, x, e), q.pow(x, e)); }
                let sample = |optimized: bool| {
                    let start = Instant::now();
                    let mut checksum = 0u64;
                    for _ in 0..4 {
                        for &x in &xs {
                            let x = black_box(x);
                            let y = if optimized { q.pow(x, e) } else { rpow(q, x, e) };
                            checksum = checksum.wrapping_add(y.0 ^ y.1);
                        }
                    }
                    black_box(checksum);
                    start.elapsed().as_nanos() as f64 / (xs.len() * 4) as f64
                };
                let (mut b, mut a) = (Vec::new(), Vec::new());
                for trial in 0..13 {
                    if trial % 2 == 0 { b.push(sample(false)); a.push(sample(true)); }
                    else { a.push(sample(true)); b.push(sample(false)); }
                }
                b.sort_by(f64::total_cmp);
                a.sort_by(f64::total_cmp);
                println!("Fp2 Frobenius p={p} {name}: {:.3} -> {:.3} ns/op ({:.3}x)",
                         b[6], a[6], b[6] / a[6]);
            }
        }
    }

    #[test]
    fn modular_half_and_linear_shortcuts_match_frozen_formula() {
        let mut seed = 0x5fee_a1fe_2026_1118u64;
        for &p in &[5u64, 7, 17, 97, 487, 65_537, 998_244_353,
                    1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            for _ in 0..4_096 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = seed % p;
                assert_eq!(f.half(a), f.mul(a, ctx.inv2), "half p={p}, a={a}");
                assert_eq!(f.half(a + p), f.mul(a, ctx.inv2), "noncanonical half p={p}");
                let want_monic = f.mul(f.neg(a), f.inv(1));
                assert_eq!(ctx.roots(&[a, 1]).unwrap().roots, vec![want_monic]);
                assert_eq!(ctx.roots(&[0, a.max(1)]).unwrap().roots, vec![0]);
                let nonmonic = 2 % p;
                let want = f.mul(f.neg(a), f.inv(nonmonic));
                assert_eq!(ctx.roots(&[a, nonmonic]).unwrap().roots, vec![want]);
                assert_eq!(f.add(f.add(a, a), a), f.mul(3, a));
            }
        }
    }

    #[test]
    #[ignore = "paired native production-Rust modular half, linear and tripling benchmark"]
    fn bench_half_linear_and_tripling() {
        use std::hint::black_box;
        use std::time::Instant;
        let mut seed = 0x60ff_a111_2026_1118u64;
        for &p in &[487u64, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            let mut xs = Vec::new();
            for _ in 0..2_048 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                xs.push(seed % p);
            }
            let sample = |optimized: bool, old: fn(&Fp,u64,u64)->u64,
                          new: fn(&Fp,u64,u64)->u64| {
                let start = Instant::now();
                let mut checksum = 0u64;
                for _ in 0..64 {
                    for &x in &xs {
                        let x = black_box(x);
                        checksum = checksum.wrapping_add(if optimized {
                            new(&f, x, ctx.inv2)
                        } else {
                            old(&f, x, ctx.inv2)
                        });
                    }
                }
                black_box(checksum);
                start.elapsed().as_nanos() as f64 / (xs.len() * 64) as f64
            };
            let cases: [(&str, fn(&Fp,u64,u64)->u64, fn(&Fp,u64,u64)->u64); 3] = [
                ("Fp half", |f,x,inv2| f.mul(x,inv2), |f,x,_| f.half(x)),
                ("linear monic", |f,x,_| f.mul(f.neg(x),f.inv(1)),
                                 |f,x,_| f.neg(x)),
                ("triple", |f,x,_| f.mul(3,x), |f,x,_| f.add(f.add(x,x),x)),
            ];
            for &(label, before, after) in &cases {
                for &x in &xs { assert_eq!(before(&f,x,ctx.inv2), after(&f,x,ctx.inv2)); }
                let (mut b, mut a) = (Vec::new(), Vec::new());
                for trial in 0..13 {
                    if trial % 2 == 0 {
                        b.push(sample(false,before,after));
                        a.push(sample(true,before,after));
                    } else {
                        a.push(sample(true,before,after));
                        b.push(sample(false,before,after));
                    }
                }
                b.sort_by(f64::total_cmp); a.sort_by(f64::total_cmp);
                println!("p={p} {label}: {:.3} -> {:.3} ns/op ({:.3}x)",
                         b[6], a[6], b[6] / a[6]);
            }
        }
    }

    // Control ring: same Fp2 mul/sqr and same precomputed AMM constants,
    // but the old generic exponentiation used before Frobenius powering.
    #[derive(Clone, Copy)]
    struct Fq2BeforePower(Fq2);
    impl Ring for Fq2BeforePower {
        type E = (u64, u64);
        fn zero(&self) -> Self::E { self.0.zero() }
        fn one(&self) -> Self::E { self.0.one() }
        fn mul(&self, a: Self::E, b: Self::E) -> Self::E { self.0.mul(a, b) }
        fn sqr(&self, a: Self::E) -> Self::E { self.0.sqr(a) }
        fn order(&self) -> u128 { self.0.order() }
        fn cand(&self, k: u64) -> Self::E { self.0.cand(k) }
    }
    fn cube_before_frobenius(ctx: &RadicalRootCtx) -> Cube<Fq2BeforePower> {
        let c = &ctx.cube_q;
        Cube { r: Fq2BeforePower(c.r), n: c.n, s: c.s, e3: c.e3,
               c: c.c, c_inv: c.c_inv, inverse_powers: c.inverse_powers.clone(),
               inverse_squares: c.inverse_squares.clone(), w: c.w, p3s: c.p3s }
    }

    #[test]
    fn actual_fp2_amm_cube_roots_match_pre_frobenius() {
        let mut seed = 0x500d_2026_1118_4411u64;
        for &p in &[5u64, 7, 97, 487, 17_497, 998_244_353,
                    1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let old = cube_before_frobenius(&ctx);
            let q = &ctx.cube_q.r;
            for _ in 0..256 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let x = (seed % p, (seed >> 9) % p);
                let cube = q.mul(q.sqr(x), x);
                assert_eq!(old.root(cube), ctx.cube_q.root(cube),
                           "AMM cube full root p={p}, x={x:?}");
                assert_eq!(old.root(x), ctx.cube_q.root(x),
                           "AMM arbitrary full root p={p}, x={x:?}");
            }
        }
    }

    #[test]
    #[ignore = "paired end-to-end Fp2 AMM cube-root kernel benchmark"]
    fn bench_fp2_amm_cube_root_before_after_frobenius() {
        use std::hint::black_box;
        use std::time::Instant;
        let mut seed = 0x1f2f_2026_1118_aa55u64;
        for &p in &[487u64, 17_497, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let old = cube_before_frobenius(&ctx);
            let q = &ctx.cube_q.r;
            let mut inputs = Vec::new();
            for _ in 0..256 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let x = (seed % p, (seed >> 19) % p);
                inputs.push(q.mul(q.sqr(x), x)); // definite cubes
            }
            for &x in &inputs { assert_eq!(old.root(x), ctx.cube_q.root(x)); }
            let sample = |optimized: bool| {
                let start = Instant::now();
                let mut checksum = 0u64;
                for _ in 0..3 {
                    for &x in &inputs {
                        let result = if optimized { ctx.cube_q.root(black_box(x)) }
                                     else { old.root(black_box(x)) };
                        checksum = checksum.wrapping_add(result.unwrap().0);
                    }
                }
                black_box(checksum);
                start.elapsed().as_nanos() as f64 / (inputs.len() * 3) as f64
            };
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for k in 0..13 {
                if k % 2 == 0 { b.push(sample(false)); a.push(sample(true)); }
                else { a.push(sample(true)); b.push(sample(false)); }
            }
            b.sort_by(f64::total_cmp); a.sort_by(f64::total_cmp);
            println!("full Fp2 AMM cube root p={p}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     b[6], a[6], b[6] / a[6]);
        }
    }

    // Frozen November 18 Tonelli path (before sharing the t/r exponent).
    // The production path must match the chosen root, not only the root set.
    fn sqrt_before_fused(ctx: &RadicalRootCtx, a: u64) -> Option<u64> {
        let f = &ctx.f;
        let p = f.p;
        if a == 0 { return Some(0); }
        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        let mut t = f.powm(a, ctx.t2);
        let mut legendre = t;
        for _ in 1..ctx.s2 { legendre = f.sqr(legendre); }
        if legendre != 1 { return None; }
        let mut m = ctx.s2;
        let mut c = ctx.zc;
        let mut r = f.powm(a, ctx.t2.div_ceil(2));
        loop {
            if t == 1 { return Some(r); }
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 && i < m { tt = f.sqr(tt); i += 1; }
            if i == m { return None; }
            let mut b = c;
            for _ in 0..m - i - 1 { b = f.sqr(b); }
            m = i;
            c = f.sqr(b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
    }

    fn quartic_coeffs_before(
        ctx: &RadicalRootCtx, b: u64, c: u64, d: u64, e: u64,
    ) -> (u64, u64, u64, u64) {
        let f = &ctx.f;
        let bb = f.mul(b, ctx.inv4);
        let b2 = f.sqr(bb);
        let pp = f.sub(c, f.mul(6, b2));
        let b3 = f.mul(b2, bb);
        let qq = f.add(f.sub(d, f.mul(f.mul(2, bb), c)), f.mul(8, b3));
        let rr = f.sub(
            f.add(f.sub(e, f.mul(bb, d)), f.mul(b2, c)),
            f.mul(3, f.sqr(b2)),
        );
        (bb, pp, qq, rr)
    }

    fn resolvent_before(
        ctx: &RadicalRootCtx, pp: u64, qq: u64, rr: u64,
    ) -> (u64, u64, u64) {
        let f = &ctx.f;
        (f.mul(2, pp), f.sub(f.sqr(pp), f.mul(4, rr)), f.neg(f.sqr(qq)))
    }

    #[test]
    fn fused_tonelli_and_quartic_coefficients_match_frozen() {
        let mut seed = 0x18a7_26fb_91dc_7e23u64;
        for &p in &[5u64, 7, 13, 17, 19, 37, 97, 193, 257, 487,
                    65_537, 998_244_353, 1_000_000_009, 2_013_265_921,
                    MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let count = if p <= 257 { p as usize } else { 2_048 };
            for i in 0..count {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = if p <= 257 { i as u64 } else { seed % p };
                assert_eq!(ctx.sqrt(a), sqrt_before_fused(&ctx, a),
                           "sqrt p={p} a={a}");
                let b = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let c = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let d = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let e = seed % p;
                assert_eq!(ctx.quartic_depressed_coeffs(b, c, d, e),
                           quartic_coeffs_before(&ctx, b, c, d, e),
                           "quartic depression p={p}");
                assert_eq!(ctx.quartic_resolvent_coeffs(b, c, d),
                           resolvent_before(&ctx, b, c, d),
                           "quartic resolvent p={p}");
            }
        }
    }

    #[test]
    #[ignore = "paired native Rust benchmark for fused Tonelli and quartic coefficients"]
    fn bench_fused_tonelli_and_quartic_coefficients() {
        use std::hint::black_box;
        use std::time::Instant;

        fn timed<T: Copy, F: Fn(T) -> u64>(xs: &[T], reps: usize, op: &F) -> f64 {
            let mut hash = 0u64;
            let start = Instant::now();
            for _ in 0..reps {
                for &x in xs {
                    hash ^= black_box(op(black_box(x)));
                }
            }
            black_box(hash);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<T: Copy, F: Fn(T) -> u64, G: Fn(T) -> u64>(
            label: &str, xs: &[T], reps: usize, before: F, after: G,
        ) {
            for &x in xs { assert_eq!(before(x), after(x), "{label} mismatch"); }
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for trial in 0..13 {
                if trial % 2 == 0 {
                    b.push(timed(xs, reps, &before));
                    a.push(timed(xs, reps, &after));
                } else {
                    a.push(timed(xs, reps, &after));
                    b.push(timed(xs, reps, &before));
                }
            }
            b.sort_by(f64::total_cmp);
            a.sort_by(f64::total_cmp);
            println!("{label}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     b[6], a[6], b[6] / a[6]);
        }

        let mut seed = 0x9123_abba_2026_cdefu64;
        for &p in &[17u64, 97, 998_244_353, 1_000_000_009, 2_013_265_921, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            let mut squares = Vec::new();
            let mut mixed = Vec::new();
            let mut coeffs = Vec::new();
            for _ in 0..2048 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = seed % p;
                squares.push(f.sqr(a));
                mixed.push(a);
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let b = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let c = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let d = seed % p;
                coeffs.push((a,b,c,d));
            }
            println!("p={p} Tonelli t2={} s2={}", ctx.t2, ctx.s2);
            for (label, xs) in [("sqrt squares", &squares), ("sqrt mixed", &mixed)] {
                paired(label, xs, 2,
                       |a| sqrt_before_fused(&ctx, a).unwrap_or(u64::MAX),
                       |a| ctx.sqrt(a).unwrap_or(u64::MAX));
            }
            paired("quartic depression", &coeffs, 10,
                   |(b,c,d,e)| { let (x,y,z,w)=quartic_coeffs_before(&ctx,b,c,d,e);x^y^z^w },
                   |(b,c,d,e)| { let (x,y,z,w)=ctx.quartic_depressed_coeffs(b,c,d,e);x^y^z^w });
            paired("quartic resolvent", &coeffs, 12,
                   |(p,q,r,_)| { let (x,y,z)=resolvent_before(&ctx,p,q,r);x^y^z },
                   |(p,q,r,_)| { let (x,y,z)=ctx.quartic_resolvent_coeffs(p,q,r);x^y^z });
        }
    }


    #[test]
    fn quadratic_direct_discriminant_matches_frozen_normalized_formula() {
        let mut seed = 0xd341_78a5_96a2_dbc0u64;
        for &p in &[5u64, 7, 11, 13, 17, 19,
                    487, 1_000_000_007, 2_147_483_647, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            let count = if p <= 19 { p.pow(3) as usize } else { 4096 };
            for i in 0..count {
                let (a, b, c) = if p <= 19 {
                    let a = (i as u64 % (p - 1)) + 1;
                    let b = (i as u64 / (p - 1)) % p;
                    let c = (i as u64 / (p * (p - 1))) % p;
                    (a, b, c)
                } else {
                    (1 + lcg(&mut seed) % (p - 1),
                     lcg(&mut seed) % p, lcg(&mut seed) % p)
                };
                let inv = f.inv(a);
                let mut frozen = ctx.quad_monic(f.mul(b, inv), f.mul(c, inv));
                frozen.sort_unstable();
                frozen.dedup();
                let direct = ctx.quad_roots(a, b, c);
                assert_eq!(direct, frozen, "p={p} a={a} b={b} c={c}");
            }
            // Noncanonical public coefficients follow the prior normalized
            // equation semantics, not the fast-path canonical assumption.
            assert_eq!(ctx.quad_roots(p + 1, p + 2, p + 3),
                       ctx.quad_roots(1, 2, 3));
        }
    }

    #[test]
    #[ignore = "paired actual-Rust quadratic root timing"]
    fn bench_quadratic_direct_discriminant() {
        use std::hint::black_box;
        use std::time::Instant;
        let mut seed = 0x37d8_6d12_b731_01fau64;
        for &p in &[487u64, 1_000_000_007, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            // All primes p=3 mod 4. A fixed nonsquare yields no roots;
            // two random linear factors yield roots. These are disjoint.
            let nr = (2..p).find(|&v| f.powm(v, (p - 1) / 2) == p - 1).unwrap();
            for yes in [false, true] {
                let mut cases = Vec::new();
                for _ in 0..1024 {
                    let a = 1 + lcg(&mut seed) % (p - 1);
                    let b;
                    let c;
                    if yes {
                        let u = lcg(&mut seed) % p;
                        let v = lcg(&mut seed) % p;
                        b = f.neg(f.mul(a, f.add(u, v)));
                        c = f.mul(a, f.mul(u, v));
                    } else {
                        b = lcg(&mut seed) % p;
                        c = f.mul(f.sub(f.sqr(b), nr), f.inv(f.mul(4, a)));
                    }
                    cases.push((a, b, c));
                }
                let old = |(a, b, c): (u64, u64, u64)| {
                    let ia = f.inv(a);
                    let mut r = ctx.quad_monic(f.mul(b, ia), f.mul(c, ia));
                    r.sort_unstable();
                    r.dedup();
                    r
                };
                let new = |(a, b, c)| ctx.quad_roots(a, b, c);
                for &x in &cases {
                    assert_eq!(old(x), new(x));
                }
                let mut b = Vec::new();
                let mut a = Vec::new();
                let sample = |opt: bool| -> f64 {
                    let start = Instant::now();
                    let mut checksum = 0u64;
                    for _ in 0..3 {
                        for &x in &cases {
                            let v = if opt { new(black_box(x)) } else { old(black_box(x)) };
                            checksum = checksum.wrapping_add(v.iter().copied().sum::<u64>());
                        }
                    }
                    black_box(checksum);
                    start.elapsed().as_nanos() as f64 / (cases.len() * 3) as f64
                };
                for k in 0..13 {
                    if k % 2 == 0 {
                        b.push(sample(false));
                        a.push(sample(true));
                    } else {
                        a.push(sample(true));
                        b.push(sample(false));
                    }
                }
                b.sort_by(f64::total_cmp);
                a.sort_by(f64::total_cmp);
                println!("quadratic p={p} {}: {:.3} -> {:.3} ns/op ({:.3}x)",
                         if yes { "two-roots" } else { "no-roots" },
                         b[6], a[6], b[6] / a[6]);
            }
        }
    }

    #[test]
    fn monic_cubic_entry_point_matches_frozen_normalization() {
        let mut seed = 0x48fb_1298_2394_86ffu64;
        for &p in &[5u64, 7, 11, 13, 97, 487, 65_537, 1_000_000_009] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            for _ in 0..64 {
                let c0 = lcg(&mut seed) % p;
                let c1 = lcg(&mut seed) % p;
                let c2 = lcg(&mut seed) % p;
                let inverse = f.inv(1);
                let mut frozen = ctx.cubic_monic(
                    f.mul(c2, inverse), f.mul(c1, inverse), f.mul(c0, inverse)
                );
                frozen.sort_unstable();
                frozen.dedup();
                let got = ctx.roots(&[c0, c1, c2, 1]).unwrap().roots;
                assert_eq!(got, frozen, "p={p} coefficients={c0},{c1},{c2}");
            }
        }
    }

    // Freeze the original right-to-left binary exponentiation independently
    // of the new shared Fp/Fp2 implementation.
    fn frozen_binary_power<R: Ring>(r: &R, a: R::E, mut e: u128) -> R::E {
        let (mut acc, mut b) = (r.one(), a);
        while e > 0 {
            if e & 1 != 0 {
                acc = r.mul(acc, b);
            }
            e >>= 1;
            if e != 0 {
                b = r.sqr(b);
            }
        }
        acc
    }

    #[test]
    fn adaptive_window_power_and_quadratic_finish_match_frozen() {
        let exponents = [
            0u128, 1, 2, 3, 17, (1u128 << 32) - 1, 1u128 << 32,
            (1u128 << 60) - 67, (1u128 << 61) - 3,
            (1u128 << 120) - 827_365,
            (1u128 << 121) + (1u128 << 63) + 1_234_567,
            u128::MAX,
        ];
        let mut seed = 0x7732_b92c_51d8_073fu64;
        for &p in &[5u64, 7, 97, 487, 65_537,
                    998_244_353, 1_000_000_009, MAX_PRIME] {
            let f = Fp::new(p);
            let q = Fq2 { f, n: 3 };
            let ctx = RadicalRootCtx::new(p).unwrap();
            for _ in 0..128 {
                let x = lcg(&mut seed) % p;
                let y = lcg(&mut seed) % p;
                for &e in &exponents {
                    assert_eq!(rpow(&f, x, e), frozen_binary_power(&f, x, e),
                               "Fp p={p} x={x} e={e}");
                    assert_eq!(rpow(&q, (x, y), e),
                               frozen_binary_power(&q, (x, y), e),
                               "Fp2 p={p} x={x} y={y} e={e}");
                    if e <= u64::MAX as u128 {
                        assert_eq!(f.powm(x, e as u64),
                                   frozen_binary_power(&f, x, e),
                                   "powm p={p} e={e}");
                    }
                }
                // Exact old formula output, not just an unordered root set.
                let s = lcg(&mut seed) % p;
                let nb = f.neg(x);
                let want_first = f.mul(f.add(nb, s), ctx.inv2);
                let want_second = f.mul(f.sub(nb, s), ctx.inv2);
                assert_eq!(f.sub(nb, want_first), want_second, "quad p={p}");
                let selected = if p > u32::MAX as u64 {
                    f.sub(nb, want_first)
                } else {
                    f.mul(f.sub(nb, s), ctx.inv2)
                };
                assert_eq!(selected, want_second, "selected quad p={p}");
            }
        }
    }

    #[test]
    #[ignore = "paired actual-Rust microbenchmark, not an end-to-end rootfinding result"]
    fn bench_adaptive_window_and_quad_finish() {
        use std::hint::black_box;
        use std::time::Instant;
        fn sample<T: Copy, F: Fn(T) -> u64>(xs: &[T], reps: usize, f: &F) -> f64 {
            let start = Instant::now();
            let mut checksum = 0u64;
            for _ in 0..reps {
                for &x in xs {
                    checksum ^= black_box(f(black_box(x)));
                }
            }
            black_box(checksum);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<T: Copy, B: Fn(T) -> u64, A: Fn(T) -> u64>(
            name: &str, xs: &[T], reps: usize, before: B, after: A
        ) {
            for &x in xs {
                assert_eq!(before(x), after(x), "{name}: output mismatch");
            }
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for k in 0..13 {
                if k % 2 == 0 {
                    b.push(sample(xs, reps, &before));
                    a.push(sample(xs, reps, &after));
                } else {
                    a.push(sample(xs, reps, &after));
                    b.push(sample(xs, reps, &before));
                }
            }
            b.sort_by(f64::total_cmp);
            a.sort_by(f64::total_cmp);
            println!("{name}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     b[6], a[6], b[6] / a[6]);
        }
        let mut seed = 0x4a36_9f11_67b2_8201u64;
        for &p in &[487u64, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let f = Fp::new(p);
            let q = Fq2 { f, n: 3 };
            let ctx = RadicalRootCtx::new(p).unwrap();
            let mut xs = Vec::new();
            for _ in 0..1024 {
                xs.push((lcg(&mut seed) % p, lcg(&mut seed) % p));
            }
            let e = (1u128 << 60) - 67; // deterministic dense exponent
            let e2 = (1u128 << 120) - 827_365;
            println!("p={p}, 13 AB/BA samples, 1024 public inputs");
            paired("Fp powm dense-60", &xs, 2,
                   |x| frozen_binary_power(&f, x.0, e),
                   |x| f.powm(x.0, e as u64));
            paired("Fp2 rpow dense-120", &xs, 1,
                   |x| frozen_binary_power(&q, x, e2).0,
                   |x| rpow(&q, x, e2).0);
            // Realistic AMM cube-root exponent (not synthetic all-ones).
            let mut t3 = (p as u128) * (p as u128) - 1;
            while t3 % 3 == 0 {
                t3 /= 3;
            }
            let e3 = if t3 % 3 == 1 { (2 * t3 + 1) / 3 } else { (t3 + 1) / 3 };
            paired("Fp2 rpow actual-AMM-e3", &xs, 1,
                   |x| frozen_binary_power(&q, x, e3).0,
                   |x| rpow(&q, x, e3).0);
            paired("quadratic two-root finish", &xs, 24,
                   |(b, s)| {
                       let nb = f.neg(b);
                       let a = f.mul(f.add(nb, s), ctx.inv2);
                       let c = f.mul(f.sub(nb, s), ctx.inv2);
                       a ^ c
                   },
                   |(b, s)| {
                       let nb = f.neg(b);
                       let a = f.mul(f.add(nb, s), ctx.inv2);
                       let c = if p > u32::MAX as u64 {
                           f.sub(nb, a)
                       } else {
                           f.mul(f.sub(nb, s), ctx.inv2)
                       };
                       a ^ c
                   });
        }
    }

    // Frozen unoptimized routines for independent bitwise before/after checks.
    fn old_sqrt(ctx: &RadicalRootCtx, a: u64) -> Option<u64> {
        let f = &ctx.f;
        let p = f.p;
        if a == 0 { return Some(0); }
        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        if f.powm(a, (p - 1) / 2) != 1 { return None; }
        let mut m = ctx.s2;
        let mut c = ctx.zc;
        let mut t = f.powm(a, ctx.t2);
        let mut r = f.powm(a, ctx.t2.div_ceil(2));
        loop {
            if t == 1 { return Some(r); }
            let mut i = 0;
            let mut temp = t;
            while temp != 1 && i < m {
                temp = f.sqr(temp);
                i += 1;
            }
            if i == m { return None; }
            let b = f.powm(c, 1u64 << (m - i - 1));
            m = i;
            c = f.sqr(b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
    }

    fn old_cube_root<R: Ring>(ctx: &Cube<R>, a: R::E) -> Option<R::E> {
        let r = &ctx.r;
        if a == r.zero() { return Some(a); }
        if ctx.s == 0 { return Some(rpow(r, a, ctx.e3)); }
        if rpow(r, a, ctx.n / 3) != r.one() { return None; }
        let x0 = rpow(r, a, ctx.e3);
        let h = rpow(r, a, 3 * ctx.e3 - 1);
        let w2 = r.mul(ctx.w, ctx.w);
        let mut l = 0u128;
        for j in 0..ctx.s {
            let corr = rpow(r, ctx.c, (ctx.p3s - l) % ctx.p3s);
            let x = r.mul(h, corr);
            let z = rpow(r, x, 3u128.pow(ctx.s - 1 - j));
            let d = if z == r.one() { 0 }
                else if z == ctx.w { 1 }
                else if z == w2 { 2 }
                else { return None };
            l += d * 3u128.pow(j);
        }
        if !l.is_multiple_of(3) { return None; }
        let y_inv = rpow(r, ctx.c, (ctx.p3s - l / 3) % ctx.p3s);
        Some(r.mul(x0, y_inv))
    }


    // Frozen November 17 AMM implementation, after the a^t reuse but
    // before rolling digits. This is the exact baseline for this change.
    fn previous_amm_root<R: Ring>(ctx: &Cube<R>, a: R::E) -> Option<R::E> {
        let r = &ctx.r;
        if a == r.zero() { return Some(a); }
        if ctx.s == 0 { return Some(rpow(r, a, ctx.e3)); }
        let t = ctx.n / ctx.p3s;
        let g = rpow(r, a, t);
        let mut residue = g;
        for _ in 1..ctx.s {
            residue = r.mul(residue, r.mul(residue, residue));
        }
        if residue != r.one() { return None; }
        let x0 = rpow(r, a, ctx.e3);
        let h = if t % 3 == 1 { r.mul(g, g) } else { g };
        let w2 = r.mul(ctx.w, ctx.w);
        let mut l = 0u128;
        for j in 0..ctx.s {
            let corr = rpow(r, ctx.c, (ctx.p3s - l) % ctx.p3s);
            let x = r.mul(h, corr);
            let z = rpow(r, x, 3u128.pow(ctx.s - 1 - j));
            let d = if z == r.one() { 0 }
                else if z == ctx.w { 1 }
                else if z == w2 { 2 }
                else { return None };
            l += d * 3u128.pow(j);
        }
        if !l.is_multiple_of(3) { return None; }
        let y_inv = rpow(r, ctx.c, (ctx.p3s - l / 3) % ctx.p3s);
        Some(r.mul(x0, y_inv))
    }


    // Frozen pre-reuse coefficient arithmetic, for exact regression comparison.
    #[inline(never)]
    fn cubic_coeffs_before(f: Fp, c2: u64, c1: u64, c0: u64, inv3: u64, inv27: u64)
        -> (u64, u64, u64)
    {
        let c2sq = f.sqr(c2);
        let pp = f.sub(c1, f.mul(c2sq, inv3));
        let t1 = f.mul(f.mul(c2sq, c2), f.add(inv27, inv27));
        let t2 = f.mul(f.mul(c1, c2), inv3);
        (pp, f.add(f.sub(t1, t2), c0), f.mul(c2, inv3))
    }

    #[inline(never)]
    fn cubic_coeffs_after(f: Fp, c2: u64, c1: u64, c0: u64, inv3: u64)
        -> (u64, u64, u64)
    {
        let sh = f.mul(c2, inv3);
        let pp = f.sub(c1, f.mul(c2, sh));
        let sh3 = f.mul(f.sqr(sh), sh);
        (pp, f.add(f.sub(f.add(sh3, sh3), f.mul(c1, sh)), c0), sh)
    }

    #[test]
    fn cubic_shift_fq2_square_and_small_field_mul_match_reference() {
        let mut seed = 0xd1da_a200_26fa_0001u64;
        for &p in &[5u64, 17, 487, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let f = Fp::new(p);
            let inv3 = f.inv(3);
            let inv27 = f.inv(27 % p);
            let n = (2..p).find(|&z| f.powm(z, (p - 1) / 2) != 1).unwrap();
            let q = Fq2 { f, n };
            for _ in 0..4096 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let c2 = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let c1 = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let c0 = seed % p;
                assert_eq!(cubic_coeffs_before(f,c2,c1,c0,inv3,inv27),
                           cubic_coeffs_after(f,c2,c1,c0,inv3));
                assert_eq!(f.mul(c2, c1),
                           ((c2 as u128 * c1 as u128) % p as u128) as u64);
                let x = (c2, c1);
                assert_eq!(q.sqr(x), q.mul(x, x));
                let mut e = 0x12d5_931f_u128;
                let (mut acc, mut b) = (q.one(), x);
                while e > 0 {
                    if e & 1 == 1 { acc = q.mul(acc,b); }
                    e >>= 1;
                    if e > 0 { b = q.mul(b,b); }
                }
                assert_eq!(rpow(&q, x, 0x12d5_931f), acc);
            }
            if p <= u32::MAX as u64 {
                assert_eq!(f.mul(p + 1, p + 2),
                           (((p + 1) as u128 * (p + 2) as u128) % p as u128) as u64);
            }
        }
    }

    #[test]
    #[ignore = "paired actual Rust microbenchmarks"]
    fn bench_cubic_shift_fq2_square_and_small_field_mul() {
        use std::hint::black_box;
        use std::time::Instant;

        fn timed<T: Copy, F: Fn(T) -> u64>(xs: &[T], reps: usize, op: &F) -> f64 {
            let mut sink = 0u64;
            let start = Instant::now();
            for _ in 0..reps {
                for &x in xs {
                    sink ^= op(black_box(x));
                }
            }
            black_box(sink);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<T: Copy, F: Fn(T) -> u64, G: Fn(T) -> u64>(
            label: &str, xs: &[T], reps: usize, before: F, after: G,
        ) {
            for &x in xs { assert_eq!(before(x), after(x), "{label} mismatch"); }
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for i in 0..13 {
                if i % 2 == 0 {
                    b.push(timed(xs, reps, &before));
                    a.push(timed(xs, reps, &after));
                } else {
                    a.push(timed(xs, reps, &after));
                    b.push(timed(xs, reps, &before));
                }
            }
            b.sort_by(f64::total_cmp);
            a.sort_by(f64::total_cmp);
            println!("{label}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     b[6], a[6], b[6]/a[6]);
        }

        let mut seed = 0x2026_abcd_1234_ef56u64;
        for &p in &[487u64, 998_244_353, 1_000_000_009, MAX_PRIME] {
            let f = Fp::new(p);
            let inv3 = f.inv(3);
            let inv27 = f.inv(27 % p);
            let n = (2..p).find(|&z| f.powm(z, (p-1)/2) != 1).unwrap();
            let q = Fq2 { f, n };
            let mut triples = Vec::with_capacity(2048);
            let mut pairs = Vec::with_capacity(2048);
            for _ in 0..2048 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let x = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let y = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let z = seed % p;
                triples.push((x,y,z));
                pairs.push((x,y));
            }
            println!("p={p}, Fp2 nonsquare={n}");
            paired("cubic shift", &triples, 8,
                 |(x,y,z)| { let (a,b,c)=cubic_coeffs_before(f,x,y,z,inv3,inv27); a^b^c },
                 |(x,y,z)| { let (a,b,c)=cubic_coeffs_after(f,x,y,z,inv3); a^b^c });
            paired("Fq2 square", &pairs, 20,
                 |x| { let y=q.mul(x,x); y.0^y.1 },
                 |x| { let y=q.sqr(x); y.0^y.1 });
            if p <= u32::MAX as u64 {
                paired("Fp canonical mul", &pairs, 24,
                     |(x,y)| ((x as u128 * y as u128) % p as u128) as u64,
                     |(x,y)| f.mul(x,y));
            }
        }
    }

    #[inline(never)]
    fn cardano_before(f: Fp, inv2: u64, inv3: u64, inv27: u64,
                      pp: u64, q: u64) -> (u64, u64) {
        let q2 = f.mul(q, inv2);
        let p2 = f.sqr(pp);
        let d = f.add(f.mul(q2, q2), f.mul(f.mul(p2, pp), inv27));
        let m = f.neg(f.mul(pp, inv3));
        (d, m)
    }

    #[inline(never)]
    fn cardano_after(f: Fp, inv2: u64, inv3: u64, _inv27: u64,
                     pp: u64, q: u64) -> (u64, u64) {
        let q2 = f.mul(q, inv2);
        let m = f.neg(f.mul(pp, inv3));
        let d = f.sub(f.sqr(q2), f.mul(f.sqr(m), m));
        (d, m)
    }

    #[test]
    fn cardano_discriminant_reuse_matches_reference() {
        let mut seed = 0x1459_1749_c0de_2026u64;
        for &p in &[5u64, 7, 17, 163, 487, 1459, 17_497,
                    998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for _ in 0..10_000 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let pp = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let q = seed % p;
                assert_eq!(
                    cardano_before(ctx.f, ctx.inv2, ctx.inv3, ctx.inv27, pp, q),
                    cardano_after(ctx.f, ctx.inv2, ctx.inv3, ctx.inv27, pp, q),
                    "p={p} P={pp} Q={q}"
                );
                assert_eq!(ctx.f.mul(2, ctx.inv27),
                           ctx.f.add(ctx.inv27, ctx.inv27));
            }
        }
    }

    #[test]
    #[ignore = "manual paired actual-Rust Cardano arithmetic timing"]
    fn bench_cardano_discriminant() {
        use std::hint::black_box;
        use std::time::Instant;

        fn time(
            ctx: &RadicalRootCtx, xs: &[(u64, u64)], reps: usize,
            op: fn(Fp, u64, u64, u64, u64, u64) -> (u64, u64),
        ) -> f64 {
            let mut hash = 0u64;
            let start = Instant::now();
            for _ in 0..reps {
                for &(pp, q) in xs {
                    let (d, m) = op(ctx.f, ctx.inv2, ctx.inv3, ctx.inv27,
                                    black_box(pp), black_box(q));
                    hash ^= d ^ m;
                }
            }
            black_box(hash);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }

        let mut seed = 0xfaced00d_12345678u64;
        for &p in &[17u64, 487, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let mut xs = Vec::new();
            for _ in 0..8192 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let pp = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                xs.push((pp, seed % p));
            }
            let (mut before, mut after) = (Vec::new(), Vec::new());
            for i in 0..13 {
                if i % 2 == 0 {
                    before.push(time(&ctx, &xs, 32, cardano_before));
                    after.push(time(&ctx, &xs, 32, cardano_after));
                } else {
                    after.push(time(&ctx, &xs, 32, cardano_after));
                    before.push(time(&ctx, &xs, 32, cardano_before));
                }
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("Cardano discriminant p={p}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     before[6], after[6], before[6] / after[6]);
        }
    }

    #[test]
    fn amm_digit_loop_matches_previous_reference() {
        // Cover s=0,1,2 and large s=3..7 on Fp, plus Fp2 and non-cubes.
        let mut seed = 0x1234_4321_9e37_2026u64;
        for &p in &[5u64, 7, 13, 19, 37, 73, 109, 163, 487,
                    1459, 17_497, 998_244_353, 1_000_000_009,
                    2_013_265_921, MAX_PRIME] {
            let f = Fp::new(p);
            let k = Cube::new(f);
            for _ in 0..512 {
                seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut z = seed;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                let a = (z ^ (z >> 31)) % p;
                assert_eq!(k.root(a), previous_amm_root(&k, a),
                           "Fp p={p} s={} a={a}", k.s);
            }
            if p <= 1459 {
                let ctx = RadicalRootCtx::new(p).unwrap();
                for _ in 0..192 {
                    seed = seed.wrapping_mul(6364136223846793005)
                               .wrapping_add(1442695040888963407);
                    let a = seed % p;
                    seed = seed.wrapping_mul(6364136223846793005)
                               .wrapping_add(1442695040888963407);
                    let b = seed % p;
                    assert_eq!(ctx.cube_q.root((a, b)),
                               previous_amm_root(&ctx.cube_q, (a, b)),
                               "Fp2 p={p} a={a} b={b}");
                }
            }
        }
    }

    // Runs on the exact production Rust implementation. Use the ignored
    // benchmark separately from correctness (it is noisy on shared CI).
    #[test]
    #[ignore = "manual paired before/after AMM timing"]
    fn bench_amm_digit_loop() {
        use std::hint::black_box;
        use std::time::Instant;

        fn time<R: Ring>(
            ctx: &Cube<R>, xs: &[R::E], reps: usize,
            root: fn(&Cube<R>, R::E) -> Option<R::E>,
        ) -> f64 {
            let mut hits = 0usize;
            let start = Instant::now();
            for _ in 0..reps {
                for &x in xs {
                    if black_box(root(ctx, black_box(x))).is_some() {
                        hits += 1;
                    }
                }
            }
            black_box(hits);
            start.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }

        let mut seed = 0x5377_19d0_1234_1234u64;
        for &p in &[163u64, 487, 1459, 17_497, 1_000_000_009, MAX_PRIME] {
            let f = Fp::new(p);
            let k = Cube::new(f);
            let mut cubes = Vec::with_capacity(2048);
            let mut mixed = Vec::with_capacity(2048);
            for _ in 0..2048 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = 1 + seed % (p - 1);
                cubes.push(f.mul(f.sqr(a), a));
                mixed.push(a);
            }
            for (label, xs) in [("cubes", &cubes), ("mixed", &mixed)] {
                for &x in xs {
                    assert_eq!(previous_amm_root(&k, x), k.root(x));
                }
                let (mut before, mut after) = (Vec::new(), Vec::new());
                for trial in 0..13 {
                    if trial % 2 == 0 {
                        before.push(time(&k, xs, 2, previous_amm_root::<Fp>));
                        after.push(time(&k, xs, 2, Cube::<Fp>::root));
                    } else {
                        after.push(time(&k, xs, 2, Cube::<Fp>::root));
                        before.push(time(&k, xs, 2, previous_amm_root::<Fp>));
                    }
                }
                before.sort_by(f64::total_cmp);
                after.sort_by(f64::total_cmp);
                println!("AMM digits p={p} s={} {label}: {:.3} -> {:.3} ns/op ({:.3}x)",
                         k.s, before[6], after[6], before[6] / after[6]);
            }
        }
    }

    #[test]
    fn radical_power_reuse_matches_frozen_reference() {
        // Include high 2-adicity, high 3-adicity (487-1 = 2*3^5),
        // and large generic moduli. Both residues and non-residues.
        let mut seed = 0x8e13_d970_ac55_10b2u64;
        for &p in &[5u64, 7, 13, 19, 97, 109, 487, 65_537,
                    998_244_353, 1_000_000_009, 2_013_265_921, MAX_PRIME] {
            let f = Fp::new(p);
            let ctx = RadicalRootCtx::new(p).unwrap();
            for _ in 0..128 {
                seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut z = seed;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                let a = (z ^ (z >> 31)) % p;
                assert_eq!(ctx.sqrt(a), old_sqrt(&ctx, a), "sqrt p={p} a={a}");
                assert_eq!(ctx.cube_p.root(a), old_cube_root(&ctx.cube_p, a),
                           "Fp AMM p={p} a={a}");
                if p <= 487 {
                    let b = f.mul(a, a);
                    assert_eq!(ctx.cube_q.root((a, b)),
                               old_cube_root(&ctx.cube_q, (a, b)),
                               "Fp2 AMM p={p} a={a}");
                }
                let twice = f.add(a, a);
                assert_eq!(f.add(twice, twice), f.mul(4, a),
                           "quad discriminant p={p} c={a}");
            }
        }
    }

    // Explicitly run with:
    // cargo test --release --manifest-path research/radical_field_perf_20261117/isolated/Cargo.toml
    //   bench_radical_power_reuse -- --ignored --nocapture
    #[test]
    #[ignore = "paired local timing, separate from correctness gates"]
    fn bench_radical_power_reuse() {
        use std::hint::black_box;
        use std::time::Instant;
        fn timing<F: Fn(u64) -> u64>(xs: &[u64], reps: usize, f: &F) -> f64 {
            let begin = Instant::now();
            let mut sum = 0u64;
            for _ in 0..reps {
                for &x in xs {
                    sum ^= black_box(f(black_box(x)));
                }
            }
            black_box(sum);
            begin.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<F: Fn(u64) -> u64, G: Fn(u64) -> u64>(
            label: &str, xs: &[u64], reps: usize, before: F, after: G,
        ) {
            for &x in xs { assert_eq!(before(x), after(x), "{label} input={x}"); }
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for i in 0..13 {
                if i % 2 == 0 {
                    b.push(timing(xs, reps, &before));
                    a.push(timing(xs, reps, &after));
                } else {
                    a.push(timing(xs, reps, &after));
                    b.push(timing(xs, reps, &before));
                }
            }
            b.sort_by(f64::total_cmp);
            a.sort_by(f64::total_cmp);
            println!("{label}: {:.3} -> {:.3} ns/op {:.3}x",
                     b[6], a[6], b[6] / a[6]);
        }
        let mut seed = 0x1234_86f0_d9cb_5678u64;
        for &p in &[97u64, 998_244_353, 1_000_000_009, 2_013_265_921] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            let mut xs = Vec::new();
            for _ in 0..2048 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = seed % p;
                xs.push(f.sqr(a));
            }
            println!("p={p}");
            paired("sqrt square", &xs, 2,
                   |a| old_sqrt(&ctx, a).unwrap(),
                   |a| ctx.sqrt(a).unwrap());
            let cs: Vec<u64> = xs.iter().map(|&x| f.mul(x, f.sqr(x))).collect();
            if !ctx.cube_p.bijective() {
                paired("AMM cube", &cs, 2,
                       |a| old_cube_root(&ctx.cube_p, a).unwrap(),
                       |a| ctx.cube_p.root(a).unwrap());
            }
            paired("quad 4*c", &xs, 24,
                   |c| f.mul(4, c),
                   |c| { let t = f.add(c, c); f.add(t, t) });
        }
    }

    #[test]
    fn optimized_field_arithmetic_matches_wide_reference() {
        fn ref_mul(p: u64, a: u64, b: u64) -> u64 {
            ((a as u128 * b as u128) % p as u128) as u64
        }
        let mut seed = 0x7d3af0192ebc42u64;
        for &p in &[5u64, 7, 11, 13, 97, 65_537, 998_244_353,
                     1_000_000_007, 2_147_483_647, MAX_PRIME] {
            let f = Fp::new(p);
            let mut input = vec![0, 1, 2, p / 2, p - 2, p - 1];
            for _ in 0..4_096 {
                input.push(lcg(&mut seed) % p);
            }
            for (i, &a) in input.iter().enumerate() {
                let b = input[(i * 37 + 1) % input.len()];
                let expected_add = ((a as u128 + b as u128) % p as u128) as u64;
                let expected_sub = ((a as u128 + p as u128 - b as u128) % p as u128) as u64;
                assert_eq!(f.add(a, b), expected_add, "add p={p} a={a} b={b}");
                assert_eq!(f.sub(a, b), expected_sub, "sub p={p} a={a} b={b}");
                assert_eq!(f.mul(a, b), ref_mul(p, a, b), "mul p={p} a={a} b={b}");
                if i < 300 {
                    let inv = f.inv(a);
                    assert_eq!(inv, f.powm(a, p - 2), "inv p={p} a={a}");
                    if a != 0 {
                        assert_eq!(ref_mul(p, a, inv), 1);
                    }
                }
                let q = Fq2 { f, n: if i & 1 == 0 { 2 } else { 3 } };
                let x = (a, b);
                let y = (input[(i + 7) % input.len()], input[(i + 13) % input.len()]);
                let ac = ref_mul(p, x.0, y.0);
                let bd = ref_mul(p, x.1, y.1);
                let nbd = ref_mul(p, q.n, bd);
                let s = ref_mul(
                    p,
                    ((x.0 as u128 + x.1 as u128) % p as u128) as u64,
                    ((y.0 as u128 + y.1 as u128) % p as u128) as u64,
                );
                let real = ((ac as u128 + nbd as u128) % p as u128) as u64;
                let imag = ((s as u128 + 2 * p as u128 - ac as u128 - bd as u128)
                    % p as u128) as u64;
                assert_eq!(q.mul(x, y), (real, imag), "Fq2 p={p} x={x:?} y={y:?}");
            }
            // Public Fp entry points previously accepted noncanonical values.
            if p == MAX_PRIME {
                assert_eq!(f.mul(p + 1, p + 2), 2);
                assert_eq!(f.add(p + 1, p + 2), 3);
                assert_eq!(f.sub(p + 1, p + 2), p - 1);
                assert_eq!(f.inv(p + 2), f.inv(2));
            }
        }
    }

    // Run explicitly with:
    // cargo test --release --lib bench_radical_field_actual_rust -- --ignored --nocapture
    // Same corpus for frozen before and actual implementation; microbench only.
    #[test]
    #[ignore = "local CPU timing study; not a CI correctness gate"]
    fn bench_radical_field_actual_rust() {
        use std::hint::black_box;
        use std::time::Instant;

        fn frozen_mul(p: u64, a: u64, b: u64) -> u64 {
            ((a as u128 * b as u128) % p as u128) as u64
        }
        fn frozen_inv(p: u64, a: u64) -> u64 {
            let mut x = a % p;
            let mut e = p - 2;
            let mut acc = 1u64 % p;
            while e > 0 {
                if e & 1 == 1 {
                    acc = frozen_mul(p, acc, x);
                }
                x = frozen_mul(p, x, x);
                e >>= 1;
            }
            acc
        }
        fn sample<F: Fn(u64, u64) -> u64>(
            op: &F,
            corpus: &[(u64, u64)],
            reps: usize,
        ) -> f64 {
            let mut sink = 0u64;
            let start = Instant::now();
            for _ in 0..reps {
                for &(a, b) in corpus {
                    sink = sink.wrapping_add(op(black_box(a), black_box(b)));
                }
            }
            black_box(sink);
            start.elapsed().as_nanos() as f64 / (corpus.len() * reps) as f64
        }
        fn paired<F: Fn(u64, u64) -> u64, G: Fn(u64, u64) -> u64>(
            label: &str,
            before: F,
            after: G,
            corpus: &[(u64, u64)],
            reps: usize,
        ) {
            for &(a, b) in corpus {
                assert_eq!(before(a, b), after(a, b), "{label} a={a} b={b}");
            }
            let mut pre = Vec::new();
            let mut post = Vec::new();
            for i in 0..9 {
                if i % 2 == 0 {
                    pre.push(sample(&before, corpus, reps));
                    post.push(sample(&after, corpus, reps));
                } else {
                    post.push(sample(&after, corpus, reps));
                    pre.push(sample(&before, corpus, reps));
                }
            }
            pre.sort_by(f64::total_cmp);
            post.sort_by(f64::total_cmp);
            let a = pre[pre.len() / 2];
            let b = post[post.len() / 2];
            println!("{label}: {a:.3} -> {b:.3} ns/op, {:.3}x", a / b);
        }
        let mut seed = 0x6ab9_d270_99e4_57f1u64;
        for &p in &[MAX_PRIME, 1_000_000_007u64] {
            let f = Fp::new(p);
            let mut corpus = Vec::new();
            for _ in 0..8192 {
                seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut v = seed;
                v = (v ^ (v >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                v = (v ^ (v >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                let a = (v ^ (v >> 31)) % (p - 1) + 1;
                seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let b = seed % p;
                corpus.push((a, b));
            }
            println!("p={p}, 8192 inputs, 9 paired medians");
            paired("add", |a, b| ((a as u128 + b as u128) % p as u128) as u64,
                |a, b| f.add(a, b), &corpus, 32);
            paired("sub", |a, b| ((a as u128 + p as u128 - b as u128) % p as u128) as u64,
                |a, b| f.sub(a, b), &corpus, 32);
            paired("mul", |a, b| frozen_mul(p, a, b),
                |a, b| f.mul(a, b), &corpus, 32);
            paired("inv", |a, _| frozen_inv(p, a),
                |a, _| f.inv(a), &corpus, 2);
        }
    }

    fn eval(p: u64, poly: &[u64], x: u64) -> u64 {
        poly.iter().rev().fold(0u64, |a, &b| {
            (((a as u128 * x as u128) + b as u128) % p as u128) as u64
        })
    }
    fn brute(p: u64, poly: &[u64]) -> Vec<u64> {
        (0..p).filter(|&x| eval(p, poly, x) == 0).collect()
    }
    fn lcg(s: &mut u64) -> u64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s >> 33
    }

    #[test]
    fn rejects_bad_primes() {
        for p in [0u64, 1, 2, 3, 4, 9, 15, MAX_PRIME + 2] {
            assert!(RadicalRootCtx::new(p).is_err(), "{p}");
        }
    }

    #[test]
    fn cube_root_exhaustive_fp_and_fp2_both_classes() {
        use std::collections::HashSet;
        for &p in &[5u64, 7, 11, 13, 17, 19, 23, 37, 53, 109, 163] {
            let f = Fp::new(p);
            let k = Cube::new(f);
            let cubes: HashSet<u64> = (0..p).map(|x| f.mul(x, f.mul(x, x))).collect();
            for a in 0..p {
                match (k.root(a), cubes.contains(&a)) {
                    (Some(r), true) => assert_eq!(f.mul(r, f.mul(r, r)), a),
                    (None, false) => {}
                    other => panic!("Fp p={p} a={a} {other:?}"),
                }
            }
            if p <= 163 {
                let n = (2..p).find(|&z| f.powm(z, (p - 1) / 2) != 1).unwrap();
                let r = Fq2 { f, n };
                let kq = Cube::new(r);
                let cs: HashSet<(u64, u64)> = (0..p)
                    .flat_map(|x| (0..p).map(move |y| (x, y)))
                    .map(|e| r.mul(e, r.mul(e, e)))
                    .collect();
                for x in 0..p {
                    for y in 0..p {
                        let e = (x, y);
                        match (kq.root(e), cs.contains(&e)) {
                            (Some(t), true) => assert_eq!(r.mul(t, r.mul(t, t)), e),
                            (None, false) => {}
                            other => panic!("Fq2 p={p} e={e:?} {other:?}"),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn exhaustive_quadratics_cubics_quartics_small_primes() {
        for &p in &[5u64, 7, 11, 13, 17] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for deg in 1..=4usize {
                let n = (p as usize).pow(deg as u32 + 1);
                for code in 0..n {
                    let mut c = vec![0u64; deg + 1];
                    let mut t = code;
                    for x in c.iter_mut() {
                        *x = (t % p as usize) as u64;
                        t /= p as usize;
                    }
                    if c[deg] == 0 {
                        continue;
                    }
                    let got = ctx.roots(&c).unwrap();
                    assert_eq!(got.roots, brute(p, &c), "p={p} poly={c:?}");
                }
            }
        }
    }

    #[test]
    fn matches_prime_root_trace_on_random_large_instances() {
        let limits = || RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: 100_000,
            degree: 8,
        };
        let mut seed = 1u64;
        let (mut fb, mut classes) = (0, [0u32; 5]);
        for &p in &[
            1_000_003u64,
            1_000_000_007,
            998_244_353,
            2_147_483_647,
            MAX_PRIME,
        ] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for it in 0..60 {
                let poly: Vec<u64> = if it % 3 == 0 {
                    // product of four random linear factors
                    let mut pl = vec![1u64];
                    for _ in 0..4 {
                        let r = lcg(&mut seed) % p;
                        let mut np = vec![0u64; pl.len() + 1];
                        for (i, &cf) in pl.iter().enumerate() {
                            np[i + 1] = (np[i + 1] + cf) % p;
                            np[i] = ((np[i] as u128 + p as u128
                                - (cf as u128 * r as u128 % p as u128))
                                % p as u128) as u64;
                        }
                        pl = np;
                    }
                    pl
                } else {
                    (0..5)
                        .map(|i| {
                            if i == 4 {
                                1 + lcg(&mut seed) % (p - 1)
                            } else {
                                lcg(&mut seed) % p
                            }
                        })
                        .collect()
                };
                let want = solve(&poly, p, 3, limits(), &mut |_| Ok(())).unwrap();
                assert_eq!(want.stop, RootStop::Complete);
                let got = ctx.roots(&poly).unwrap();
                assert_eq!(got.roots, want.roots, "p={p} poly={poly:?}");
                if got.frobenius_fallback {
                    fb += 1;
                }
                classes[want.roots.len()] += 1;
            }
        }
        assert!(
            classes[0] > 0 && classes[1] > 0 && classes[2] > 0 && classes[4] > 0,
            "{classes:?}"
        );
        assert!(fb > 0, "the (1,3) class must be exercised");
    }
    #[test]
    fn skip_zeroth_amm_digit_preserves_roots_fp_and_fp2() {
        let mut seed = 0x16a1_2026_b00f_1234u64;
        for &p in &[7u64, 17, 97, 163, 487, 1459, 17_497,
                    998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            for _ in 0..384 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let a = seed % p;
                let cube = f.mul(a, f.sqr(a));
                for x in [a, cube] {
                    assert_eq!(ctx.cube_p.root_before_skip_digit(x), ctx.cube_p.root(x),
                               "Fp p={p}, x={x}, s={}", ctx.cube_p.s);
                }
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let x = (a, seed % p);
                let cube_q = ctx.cube_q.r.cube(x);
                for y in [x, cube_q] {
                    assert_eq!(ctx.cube_q.root_before_skip_digit(y), ctx.cube_q.root(y),
                               "Fp2 p={p}, x={y:?}, s={}", ctx.cube_q.s);
                }
            }
        }
    }

    #[test]
    #[ignore = "isolated production-Rust benchmark of skipped AMM digit"]
    fn bench_skip_zeroth_amm_digit_actual_rust() {
        use std::hint::black_box;
        use std::time::Instant;
        fn sample<R: Ring>(ctx: &Cube<R>, xs: &[R::E], reps: usize, after: bool) -> f64 {
            let now = Instant::now();
            let mut hits = 0u64;
            for _ in 0..reps {
                for &x in xs {
                    let got = if after { ctx.root(black_box(x)) }
                              else { ctx.root_before_skip_digit(black_box(x)) };
                    if black_box(got).is_some() { hits += 1; }
                }
            }
            black_box(hits);
            now.elapsed().as_nanos() as f64 / (xs.len() * reps) as f64
        }
        fn paired<R: Ring>(label: &str, ctx: &Cube<R>, xs: &[R::E], reps: usize) {
            for &x in xs { assert_eq!(ctx.root_before_skip_digit(x), ctx.root(x)); }
            let (mut before, mut after) = (Vec::new(), Vec::new());
            for j in 0..13 {
                if j % 2 == 0 {
                    before.push(sample(ctx, xs, reps, false));
                    after.push(sample(ctx, xs, reps, true));
                } else {
                    after.push(sample(ctx, xs, reps, true));
                    before.push(sample(ctx, xs, reps, false));
                }
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("{label}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     before[6], after[6], before[6] / after[6]);
        }
        let mut seed = 0x6811_2026_1655_abcd_u64;
        for &p in &[163u64, 487, 1459, 17_497, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = ctx.f;
            let mut fp_cubes = Vec::new();
            let mut fp_mixed = Vec::new();
            let mut fq_cubes = Vec::new();
            let mut fq_mixed = Vec::new();
            for _ in 0..256 {
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let x = seed % p;
                seed = seed.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let y = (x, seed % p);
                fp_cubes.push(f.mul(x, f.sqr(x)));
                fp_mixed.push(x);
                fq_cubes.push(ctx.cube_q.r.cube(y));
                fq_mixed.push(y);
            }
            for (label, xs) in [("Fp cubes", &fp_cubes), ("Fp mixed", &fp_mixed)] {
                paired(&format!("p={p} s={} {label}", ctx.cube_p.s), &ctx.cube_p, xs, 8);
            }
            for (label, xs) in [("Fp2 cubes", &fq_cubes), ("Fp2 mixed", &fq_mixed)] {
                paired(&format!("p={p} s={} {label}", ctx.cube_q.s), &ctx.cube_q, xs, 8);
            }
        }
    }


    // Frozen before-cache variant, retaining the repeated per-root cubings
    // and squarings of the 3-Sylow correction constants.
    fn root_before_cached_sylow<R: Ring>(cube: &Cube<R>, a: R::E) -> Option<R::E> {
        let r = &cube.r;
        if a == r.zero() { return Some(a); }
        if cube.s <= 2 { return cube.root(a); }
        let t = cube.n / cube.p3s;
        let g = r.pow(a, t);
        let mut residue = g;
        for _ in 1..cube.s { residue = r.cube(residue); }
        if residue != r.one() { return None; }
        let x0 = r.pow(a, cube.e3);
        let h = if t % 3 == 1 { r.mul(g, g) } else { g };
        if h == r.one() { return Some(x0); }
        let w2 = r.mul(cube.w, cube.w);
        let mut correction = r.one();
        let mut root_correction = r.one();
        let mut previous_power = cube.c_inv;
        let mut inverse_power = r.cube(cube.c_inv);
        for j in 1..cube.s {
            let mut z = r.mul(h, correction);
            for _ in (j + 1)..cube.s { z = r.cube(z); }
            let digit = if z == r.one() { 0 }
                        else if z == cube.w { 1 }
                        else if z == w2 { 2 }
                        else { return None };
            if digit != 0 {
                let factor = if digit == 1 { previous_power }
                             else { r.mul(previous_power, previous_power) };
                root_correction = r.mul(root_correction, factor);
                if j + 1 < cube.s {
                    let term = if digit == 1 { inverse_power }
                               else { r.mul(inverse_power, inverse_power) };
                    correction = r.mul(correction, term);
                }
            }
            previous_power = inverse_power;
            if j + 1 < cube.s { inverse_power = r.cube(inverse_power); }
        }
        Some(r.mul(x0, root_correction))
    }

    #[test]
    fn cached_sylow_factors_match_precache_roots() {
        let mut seed = 0x2026_cafe_17b0_ade2u64;
        for &p in &[7u64, 17, 97, 163, 487, 1459, 17_497,
                     998_244_353, 1_000_000_009, MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for _ in 0..512 {
                let a = lcg(&mut seed) % p;
                let b = lcg(&mut seed) % p;
                let fq = (a, b);
                let fq_cube = ctx.cube_q.r.cube(fq);
                let fp_cube = ctx.f.mul(a, ctx.f.sqr(a));
                for x in [a, fp_cube] {
                    assert_eq!(root_before_cached_sylow(&ctx.cube_p, x),
                               ctx.cube_p.root(x), "Fp p={p} x={x}");
                }
                for x in [fq, fq_cube] {
                    assert_eq!(root_before_cached_sylow(&ctx.cube_q, x),
                               ctx.cube_q.root(x), "Fp2 p={p} x={x:?}");
                }
            }
        }
    }

    #[test]
    #[ignore = "paired actual-production-Rust Sylow factor cache benchmark"]
    fn bench_cached_sylow_factors() {
        use std::hint::black_box;
        use std::time::Instant;
        fn measure<R: Ring>(ctx: &Cube<R>, xs: &[R::E], cached: bool) -> f64 {
            let start = Instant::now();
            let mut checksum = 0usize;
            for _ in 0..12 {
                for &x in xs {
                    let v = if cached { ctx.root(black_box(x)) }
                            else { root_before_cached_sylow(ctx, black_box(x)) };
                    checksum = checksum.wrapping_add(usize::from(v.is_some()));
                    black_box(v);
                }
            }
            black_box(checksum);
            start.elapsed().as_nanos() as f64 / (xs.len() * 12) as f64
        }
        fn paired<R: Ring>(name: &str, ctx: &Cube<R>, xs: &[R::E]) {
            for &x in xs {
                assert_eq!(root_before_cached_sylow(ctx, x), ctx.root(x), "{name}");
            }
            let (mut b, mut a) = (Vec::new(), Vec::new());
            for k in 0..13 {
                if k % 2 == 0 {
                    b.push(measure(ctx, xs, false));
                    a.push(measure(ctx, xs, true));
                } else {
                    a.push(measure(ctx, xs, true));
                    b.push(measure(ctx, xs, false));
                }
            }
            b.sort_by(f64::total_cmp);
            a.sort_by(f64::total_cmp);
            println!("{name}: {:.3} -> {:.3} ns/root ({:.3}x)",
                     b[6], a[6], b[6] / a[6]);
        }
        let mut seed = 0xcace_2026_1118_1700u64;
        for &p in &[163u64, 487, 1459, 17_497, 1_000_000_009] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let mut fp_cubes = Vec::new();
            let mut fq_cubes = Vec::new();
            let mut fq_mixed = Vec::new();
            for _ in 0..384 {
                let a = lcg(&mut seed) % p;
                let b = lcg(&mut seed) % p;
                fp_cubes.push(ctx.f.mul(a, ctx.f.sqr(a)));
                let qx = (a, b);
                fq_cubes.push(ctx.cube_q.r.cube(qx));
                fq_mixed.push(qx);
            }
            paired(&format!("Fp p={p} s={}: cubes", ctx.cube_p.s),
                   &ctx.cube_p, &fp_cubes);
            paired(&format!("Fp2 p={p} s={}: cubes", ctx.cube_q.s),
                   &ctx.cube_q, &fq_cubes);
            paired(&format!("Fp2 p={p} s={}: mixed", ctx.cube_q.s),
                   &ctx.cube_q, &fq_mixed);
        }
    }

}

#[cfg(test)]
impl RadicalRootCtx {
    fn quartic_before_sparse(
        &self,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    ) -> Result<(Vec<u64>, bool), String> {
        let f = &self.f;
        // monic
        let (b1, c1, d1, e1) = if a == 1 {
            (b, c, d, e)
        } else {
            let ia = f.inv(a);
            (f.mul(b, ia), f.mul(c, ia), f.mul(d, ia), f.mul(e, ia))
        };
        // x = y - B, B = b1/4
        let (bb, pp, qq, rr) = self.quartic_depressed_coeffs(b1, c1, d1, e1);
        let mut ys: Vec<u64> = vec![];
        let mut fallback = false;
        if qq == 0 {
            for z in self.quad_monic(pp, rr) {
                match self.sqrt(z) {
                    None => {}
                    Some(0) => ys.push(0),
                    Some(s) => {
                        ys.push(s);
                        ys.push(f.neg(s));
                    }
                }
            }
        } else {
            // resolvent in z = (r1+r2)^2:  z^3 + 2P z^2 + (P^2 - 4R) z - Q^2 = 0
            let (z2, z1, z0) = self.quartic_resolvent_coeffs(pp, qq, rr);
            let zs = self.cubic_monic(z2, z1, z0);
            if zs.is_empty() {
                // Frobenius type (1,3): no rational pairing; the unique root needs F_{p^3} radicals.
                // Use the distinct-degree (CZ) path on the depressed quartic - NOT radical.
                fallback = true;
                ys = frobenius_roots(f.p, &[rr, qq, pp, 0, 1])?;
            } else {
                for &z in &zs {
                    if let Some(al) = self.sqrt(z) {
                        // quartic = (y^2 + al y + be)(y^2 - al y + ga)
                        let qa = f.mul(qq, f.inv(al));
                        let s = f.add(pp, z);
                        let be = f.half(f.sub(s, qa));
                        let ga = f.half(f.add(s, qa));
                        ys.extend(self.quad_monic(al, be));
                        ys.extend(self.quad_monic(f.neg(al), ga));
                        break;
                    }
                }
            }
        }
        let mut xs: Vec<u64> = ys.into_iter().map(|y| f.sub(y, bb)).collect();
        xs.sort_unstable();
        xs.dedup();
        Ok((xs, fallback))
    }
}

#[cfg(test)]
mod sparse_round18_tests {
    use super::*;

    fn before_s1<R: Ring>(c: &Cube<R>, a: R::E) -> Option<R::E> {
        assert_eq!(c.s, 1);
        if a == c.r.zero() { return Some(a); }
        // The previous path computed a^t and performed membership first.
        let t = c.n / c.p3s;
        if c.r.pow(a, t) != c.r.one() { return None; }
        Some(c.r.pow(a, c.e3))
    }
    fn before_quad_monic(ctx: &RadicalRootCtx, b: u64, c: u64) -> Vec<u64> {
        let f = &ctx.f;
        let twice_c = f.add(c,c);
        let disc = f.sub(f.sqr(b),f.add(twice_c,twice_c));
        match ctx.sqrt(disc) {
            None => vec![],
            Some(0) => vec![f.half(f.neg(b))],
            Some(s) => {
                let nb=f.neg(b);
                let first=f.half(f.add(nb,s));
                let second=if f.p>u32::MAX as u64 {
                    f.sub(nb,first)
                } else { f.half(f.sub(nb,s)) };
                vec![first,second]
            }
        }
    }
    fn before_quad(ctx: &RadicalRootCtx, a: u64, b: u64, c: u64) -> Vec<u64> {
        let f=&ctx.f;
        if a == 1 { let mut r=before_quad_monic(ctx,b,c);r.sort_unstable();r.dedup();return r; }
        let ac=f.mul(a,c);let t=f.add(ac,ac);
        let d=f.sub(f.sqr(b),f.add(t,t));
        let Some(s)=ctx.sqrt(d) else {return vec![]};
        let inv=f.inv(f.add(a,a));
        let nb=f.neg(b);
        let x=f.mul(f.add(nb,s),inv);
        let mut out=if s==0 {vec![x]} else {vec![x,f.mul(f.sub(nb,s),inv)]};
        out.sort_unstable();out.dedup();out
    }
    fn sort(mut v:Vec<u64>)->Vec<u64>{v.sort_unstable();v.dedup();v}
    fn next(seed:&mut u64)->u64{
        *seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *seed
    }
    #[test]
    fn s1_and_sparse_quadratic_quartic_match_previous() {
        let mut seed=0x6d16_f124_2026_1118u64;
        for p in [5u64,7,11,13,31,97,163,487,998_244_353] {
            let ctx=RadicalRootCtx::new(p).unwrap();
            if ctx.cube_p.s==1 {
                for _ in 0..384 {
                    let x=next(&mut seed)%p;
                    assert_eq!(ctx.cube_p.root(x),before_s1(&ctx.cube_p,x),"Fp p={p}");
                    let y=ctx.f.mul(x,ctx.f.sqr(x));
                    assert_eq!(ctx.cube_p.root(y),before_s1(&ctx.cube_p,y),"Fp cube p={p}");
                }
            }
            if ctx.cube_q.s==1 {
                for _ in 0..384 {
                    let x=(next(&mut seed)%p,next(&mut seed)%p);
                    let y=ctx.cube_q.r.cube(x);
                    assert_eq!(ctx.cube_q.root(x),before_s1(&ctx.cube_q,x),"Fp2 p={p}");
                    assert_eq!(ctx.cube_q.root(y),before_s1(&ctx.cube_q,y),"Fp2 cube p={p}");
                }
            }
        }
        for p in [5u64,7,11,17,97,487,998_244_353,1_000_000_007,MAX_PRIME] {
            let ctx=RadicalRootCtx::new(p).unwrap();
            let f=&ctx.f;
            for _ in 0..128 {
                let a=1+(next(&mut seed)%(p-1));
                let b=next(&mut seed)%p;
                let mut x=ctx.quad_monic(b,0);
                assert_eq!(sort(x.clone()),sort(before_quad_monic(&ctx,b,0)),"monic p={p}");
                for root in x.drain(..) {
                    assert_eq!(f.mul(root,f.add(root,b)),0,"bad monic root p={p}");
                }
                let mut y=ctx.quad_roots(a,b,0);
                assert_eq!(y,before_quad(&ctx,a,b,0),"nonmonic p={p}");
                for root in y.drain(..) {
                    assert_eq!(f.mul(root,f.add(f.mul(a,root),b)),0,"bad quadratic root p={p}");
                }
                // Construct x*(x-r1)*(x-r2)*(x-r3), then scale by a.
                let r1=next(&mut seed)%p;
                let r2=next(&mut seed)%p;
                let r3=next(&mut seed)%p;
                let sum=f.add(f.add(r1,r2),r3);
                let pairs=f.add(f.add(f.mul(r1,r2),f.mul(r1,r3)),f.mul(r2,r3));
                let product=f.mul(f.mul(r1,r2),r3);
                let bb=f.mul(a,f.neg(sum));
                let cc=f.mul(a,pairs);
                let dd=f.mul(a,f.neg(product));
                let (actual,unused)=ctx.quartic(a,bb,cc,dd,0).unwrap();
                let (previous,_)=ctx.quartic_before_sparse(a,bb,cc,dd,0).unwrap();
                let expected=sort(vec![0,r1,r2,r3]);
                assert_eq!(actual,expected,"quartic p={p}");
                assert_eq!(actual,previous,"quartic old/new p={p}");
                assert!(!unused);
            }
        }
    }

    #[test]
    #[ignore]
    fn bench_s1_and_sparse_roots_actual_rust() {
        fn sample<T: Copy,F:Fn(T)->u64>(xs:&[T],reps:usize,f:&F)->f64 {
            let mut hash=0u64;
            let start=std::time::Instant::now();
            for _ in 0..reps {for &x in xs {
                hash=hash.wrapping_add(std::hint::black_box(f(std::hint::black_box(x))));
            }}
            std::hint::black_box(hash);
            start.elapsed().as_nanos() as f64 / (xs.len()*reps) as f64
        }
        fn paired<T:Copy,F:Fn(T)->u64,G:Fn(T)->u64>(
            label:&str,xs:&[T],reps:usize,before:F,after:G
        ) {
            let mut a=Vec::new();let mut b=Vec::new();
            for j in 0..13 {
                if j%2==0 {a.push(sample(xs,reps,&before));b.push(sample(xs,reps,&after))}
                else {b.push(sample(xs,reps,&after));a.push(sample(xs,reps,&before))}
            }
            a.sort_by(f64::total_cmp);b.sort_by(f64::total_cmp);
            println!("{label} before={:.3}ns after={:.3}ns speedup={:.3}x",
                     a[6],b[6],a[6]/b[6]);
        }
        let mut seed=0x515a_8818_2026_1118u64;
        let ctx=RadicalRootCtx::new(998_244_353).unwrap();
        assert_eq!(ctx.cube_q.s,1);
        let mut cubes=Vec::new();let mut mixed=Vec::new();
        for _ in 0..256 {
            let x=(next(&mut seed)%ctx.f.p,next(&mut seed)%ctx.f.p);
            mixed.push(x);cubes.push(ctx.cube_q.r.cube(x));
        }
        for (name,xs) in [("cubes",cubes),("mixed",mixed)] {
            paired(&format!("Fp2 s1 {name} p=998244353"),&xs,12,
                |x|{let y=before_s1(&ctx.cube_q,x);y.map(|v|v.0^v.1).unwrap_or(u64::MAX)},
                |x|{let y=ctx.cube_q.root(x);y.map(|v|v.0^v.1).unwrap_or(u64::MAX)});
        }
        let ctx=RadicalRootCtx::new(1_000_000_007).unwrap();
        let mut monic=Vec::new();let mut general=Vec::new();
        for _ in 0..512 {
            let a=2+(next(&mut seed)%(ctx.f.p-2));
            let b=1+(next(&mut seed)%(ctx.f.p-1));
            monic.push(b);general.push((a,b));
        }
        paired("monic quadratic c=0 p=1000000007",&monic,32,
            |b| before_quad_monic(&ctx,b,0).iter().fold(0,|h,&v|h^v),
            |b| ctx.quad_monic(b,0).iter().fold(0,|h,&v|h^v));
        paired("general quadratic c=0 p=1000000007",&general,32,
            |(a,b)| before_quad(&ctx,a,b,0).iter().fold(0,|h,&v|h^v),
            |(a,b)| ctx.quad_roots(a,b,0).iter().fold(0,|h,&v|h^v));
        let ctx=RadicalRootCtx::new(487).unwrap();
        let f=&ctx.f;let mut quartics=Vec::new();
        for _ in 0..128 {
            let a=1+next(&mut seed)%(f.p-1);
            let x=next(&mut seed)%f.p;let y=next(&mut seed)%f.p;
            let z=next(&mut seed)%f.p;
            let b=f.mul(a,f.neg(f.add(f.add(x,y),z)));
            let c=f.mul(a,f.add(f.add(f.mul(x,y),f.mul(x,z)),f.mul(y,z)));
            let d=f.mul(a,f.neg(f.mul(f.mul(x,y),z)));
            quartics.push((a,b,c,d));
        }
        paired("quartic constant=0 p=487",&quartics,6,
            |(a,b,c,d)|ctx.quartic_before_sparse(a,b,c,d,0).unwrap().0.into_iter().fold(0,|h,v|h^v),
            |(a,b,c,d)|ctx.quartic(a,b,c,d,0).unwrap().0.into_iter().fold(0,|h,v|h^v));
        // Unchanged-input controls expose branch overhead and regressions.
        let ctx=RadicalRootCtx::new(1_000_000_007).unwrap();
        let mut monic_nonzero=Vec::new();let mut nonmonic_nonzero=Vec::new();
        for _ in 0..512 {
            let a=2+(next(&mut seed)%(ctx.f.p-2));
            let b=next(&mut seed)%ctx.f.p;
            let c=1+(next(&mut seed)%(ctx.f.p-1));
            monic_nonzero.push((b,c));nonmonic_nonzero.push((a,b,c));
        }
        paired("monic quadratic c!=0 control p=1000000007",&monic_nonzero,32,
            |(b,c)| before_quad_monic(&ctx,b,c).iter().fold(0,|h,&v|h^v),
            |(b,c)| ctx.quad_monic(b,c).iter().fold(0,|h,&v|h^v));
        paired("general quadratic c!=0 control p=1000000007",&nonmonic_nonzero,32,
            |(a,b,c)| before_quad(&ctx,a,b,c).iter().fold(0,|h,&v|h^v),
            |(a,b,c)| ctx.quad_roots(a,b,c).iter().fold(0,|h,&v|h^v));
        let ctx=RadicalRootCtx::new(487).unwrap();
        let f=&ctx.f;let mut quartic_controls=Vec::new();
        for _ in 0..128 {
            let a=1+(next(&mut seed)%(f.p-1));
            let x=1+(next(&mut seed)%(f.p-1));
            let y=1+(next(&mut seed)%(f.p-1));
            let z=1+(next(&mut seed)%(f.p-1));
            let w=1+(next(&mut seed)%(f.p-1));
            let roots=[x,y,z,w];
            let mut poly=[a,0,0,0,0];
            // Construct coefficients in ascending order by multiplying
            // (X-r) successively, then scale by a.
            let mut coeff=[1u64,0,0,0,0];
            let mut degree=0;
            for &root in &roots {
                for j in (1..=degree+1).rev() {
                    coeff[j]=f.sub(if j<=degree {f.mul(coeff[j],f.neg(root))} else {0},f.neg(coeff[j-1]));
                }
                coeff[0]=f.mul(coeff[0],f.neg(root));
                degree+=1;
            }
            for j in 0..5 {poly[j]=f.mul(coeff[j],a);}
            quartic_controls.push((poly[4],poly[3],poly[2],poly[1],poly[0]));
        }
        for &(a,b,c,d,e) in &quartic_controls {
            assert_eq!(ctx.quartic(a,b,c,d,e).unwrap().0,
                       ctx.quartic_before_sparse(a,b,c,d,e).unwrap().0);
        }
        paired("quartic e!=0 control p=487",&quartic_controls,6,
            |(a,b,c,d,e)| ctx.quartic_before_sparse(a,b,c,d,e).unwrap().0.into_iter().fold(0,|h,v|h^v),
            |(a,b,c,d,e)| ctx.quartic(a,b,c,d,e).unwrap().0.into_iter().fold(0,|h,v|h^v));
    }
}

#[cfg(test)]
impl RadicalRootCtx {
    fn roots_before_stack(&self, f: &[u64]) -> Result<RadicalRoots, String> {
        let fp = &self.f;
        let mut c: Vec<u64> = f.iter().map(|&x| x % fp.p).collect();
        while c.last() == Some(&0) {
            c.pop();
        }
        let ok = |roots: Vec<u64>| {
            Ok(RadicalRoots {
                roots,
                frobenius_fallback: false,
            })
        };
        match c.len() {
            0 => Err("zero polynomial".into()),
            1 => ok(vec![]),
            2 => {
                // x + c0 and a*x have roots without an inversion.
                let root = if c[0] == 0 { 0 }
                    else if c[1] == 1 { fp.neg(c[0]) }
                    else { fp.mul(fp.neg(c[0]), fp.inv(c[1])) };
                ok(vec![root])
            },
            3 => ok(self.quad_roots(c[2], c[1], c[0])),
            4 => {
                // Monic cubics are common in normalized rootfinding.
                // Avoid one inversion and three modular products.
                let mut v = if c[3] == 1 {
                    self.cubic_monic(c[2], c[1], c[0])
                } else {
                    let ia = fp.inv(c[3]);
                    self.cubic_monic(fp.mul(c[2], ia), fp.mul(c[1], ia), fp.mul(c[0], ia))
                };
                v.sort_unstable();
                v.dedup();
                ok(v)
            }
            5 => {
                let (roots, fb) = self.quartic(c[4], c[3], c[2], c[1], c[0])?;
                Ok(RadicalRoots {
                    roots,
                    frobenius_fallback: fb,
                })
            }
            _ => Err("radical root finder supports degree <= 4".into()),
        }
    }
}

#[cfg(test)]
mod stack_normalization_round18_tests {
    use super::*;
    fn next(seed: &mut u64) -> u64 {
        *seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *seed
    }
    #[test]
    fn stack_coefficient_buffer_preserves_padding_and_modular_zeroes() {
        let mut seed=0x71ab_a1e8_cda1_2026u64;
        for p in [5u64,7,11,17,97,487,1_000_000_007,MAX_PRIME] {
            let ctx=RadicalRootCtx::new(p).unwrap();
            // Sparse, dense and oversized polynomials, including arbitrary
            // numbers of trailing coefficients congruent to zero mod p.
            for degree in 0..=7 {
                for k in 0..96 {
                    let mut poly=Vec::new();
                    for j in 0..=degree {
                        let v=next(&mut seed)%p;
                        let x=if j==degree && degree<=4 && k%3==0 {1}
                              else if k%7==0 {v+p} else {v};
                        poly.push(x);
                    }
                    if degree<=4 && k%3==1 {for _ in 0..7{poly.push(p)}}
                    if degree<=4 && k%3==2 {for _ in 0..7{poly.push(0)}}
                    // Avoid expensive generic quartic fallback in this
                    // normalization test, independently tested above.
                    if degree==4 && p>487 {continue;}
                    assert_eq!(ctx.roots_before_stack(&poly),ctx.roots(&poly),
                               "p={p} degree={degree} k={k} f={poly:?}");
                }
            }
        }
    }
    #[test]
    #[ignore]
    fn bench_stack_coefficient_buffer_actual_rust() {
        fn sample<T:Copy,F:Fn(T)->u64>(xs:&[T],reps:usize,f:&F)->f64 {
            let start=std::time::Instant::now();let mut digest=0u64;
            for _ in 0..reps {
                for &x in xs {
                    digest=digest.wrapping_add(std::hint::black_box(f(std::hint::black_box(x))));
                }
            }
            std::hint::black_box(digest);
            start.elapsed().as_nanos() as f64/(reps*xs.len()) as f64
        }
        fn paired<T:Copy,F:Fn(T)->u64,G:Fn(T)->u64>(name:&str,xs:&[T],reps:usize,old:F,new:G) {
            let mut a=Vec::new();let mut b=Vec::new();
            for j in 0..13 {
                if j%2==0 {a.push(sample(xs,reps,&old));b.push(sample(xs,reps,&new))}
                else {b.push(sample(xs,reps,&new));a.push(sample(xs,reps,&old))}
            }
            a.sort_by(f64::total_cmp);b.sort_by(f64::total_cmp);
            println!("{name}: before={:.3}ns after={:.3}ns speedup={:.3}x",a[6],b[6],a[6]/b[6]);
        }
        let ctx=RadicalRootCtx::new(1_000_000_007).unwrap();
        let mut seed=0x9a82_3bed_2026_1118;
        let mut linear=Vec::new();let mut padded=Vec::new();
        for _ in 0..512 {
            let a=next(&mut seed)%ctx.f.p;
            linear.push([a,1]);padded.push([a,1,0,0,0]);
        }
        paired("linear input (2 coefficients) p=1000000007",&linear,64,
            |x|ctx.roots_before_stack(&x).unwrap().roots.iter().fold(0u64,|h,&v|h^v),
            |x|ctx.roots(&x).unwrap().roots.iter().fold(0u64,|h,&v|h^v));
        paired("linear input (5 coefficients, padded) p=1000000007",&padded,64,
            |x|ctx.roots_before_stack(&x).unwrap().roots.iter().fold(0u64,|h,&v|h^v),
            |x|ctx.roots(&x).unwrap().roots.iter().fold(0u64,|h,&v|h^v));
    }
}

#[cfg(test)]
impl RadicalRootCtx {
    // Frozen pre-round19 implementations. Actual Rust before/after comparison
    // uses the same Fp, Fp2 and context as the new production entry points.
    fn cubic_depressed_before_sparse(&self, pp: u64, q: u64) -> Vec<u64> {
        let f = &self.f;
        if pp == 0 {
            if q == 0 {
                return vec![0];
            }
            return match self.cube_p.root(f.neg(q)) {
                None => vec![],
                Some(u) => {
                    if self.cube_p.bijective() {
                        vec![u]
                    } else {
                        let w = self.cube_p.w;
                        let uw = f.mul(u, w);
                        vec![u, uw, f.mul(uw, w)]
                    }
                }
            };
        }
        let q2 = f.half(q);
        // Keep the original discriminant: algebraic m^3 reuse measured
        // 0.61-0.69x in actual Rust CI despite faster C++ screening.
        let p2 = f.sqr(pp);
        let d = f.add(f.sqr(q2), f.mul(f.mul(p2, pp), self.inv27));
        if d == 0 {
            // double root -3Q/(2P), simple root 3Q/P
            let ip = f.inv(pp);
            let triple_q = f.add(f.add(q, q), q);
            let r1 = f.mul(triple_q, ip);
            let r2 = f.half(f.neg(r1));
            return vec![r1, r2];
        }
        let nq2 = f.neg(q2);
        let m = f.neg(f.mul(pp, self.inv3));
        match self.sqrt(d) {
            Some(sg) => {
                let u3 = f.add(nq2, sg);
                match self.cube_p.root(u3) {
                    None => vec![],
                    Some(u) => {
                        let v = f.mul(m, f.inv(u));
                        if self.cube_p.bijective() {
                            vec![f.add(u, v)]
                        } else {
                            let w = self.cube_p.w;
                            let w_2 = f.sqr(w);
                            let (mut uk, mut vk) = (u, v);
                            let mut out = vec![f.add(u, v)];
                            for _ in 0..2 {
                                uk = f.mul(uk, w);
                                vk = f.mul(vk, w_2);
                                out.push(f.add(uk, vk));
                            }
                            out
                        }
                    }
                }
            }
            None => {
                // casus irreducibilis route: sqrt(D) = i * sqrt(D/n) in F_{p^2}
                let sg = self
                    .sqrt(f.mul(d, self.inv_n))
                    .expect("D/n must be a residue");
                let u3 = (nq2, sg);
                let k = &self.cube_q;
                let u = match k.root(u3) {
                    None => return vec![],
                    Some(u) => u,
                };
                // 1/u = conj(u)/N(u)
                let nrm = f.sub(f.sqr(u.0), f.mul(self.n, f.sqr(u.1)));
                let ni = f.inv(nrm);
                let uinv = (f.mul(u.0, ni), f.mul(f.neg(u.1), ni));
                let v = (f.mul(m, uinv.0), f.mul(m, uinv.1));
                let w = self.w2;
                let w_2 = k.r.mul(w, w);
                let mut out = vec![];
                let (mut uk, mut vk) = (u, v);
                for kk in 0..3 {
                    if kk > 0 {
                        uk = k.r.mul(uk, w);
                        vk = k.r.mul(vk, w_2);
                    }
                    let im = f.add(uk.1, vk.1);
                    if im == 0 {
                        out.push(f.add(uk.0, vk.0));
                    }
                }
                out
            }
        }
    }

    fn cubic_monic_before_sparse(&self, c2: u64, c1: u64, c0: u64) -> Vec<u64> {
        let f = &self.f;
        // Shift x=t-sh, sh=c2/3. Reuse sh:
        // P=c1-c2*sh; Q=2*sh^3-c1*sh+c0.
        // Five field multiplications instead of seven.
        let sh = f.mul(c2, self.inv3);
        let pp = f.sub(c1, f.mul(c2, sh));
        let sh3 = f.mul(f.sqr(sh), sh);
        let q = f.add(f.sub(f.add(sh3, sh3), f.mul(c1, sh)), c0);
        self.cubic_depressed_before_sparse(pp, q)
            .into_iter()
            .map(|t| f.sub(t, sh))
            .collect()
    }

    fn quartic_before_round19(
        &self,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    ) -> Result<(Vec<u64>, bool), String> {
        let f = &self.f;
        // monic
        let (b1, c1, d1, e1) = if a == 1 {
            (b, c, d, e)
        } else {
            let ia = f.inv(a);
            (f.mul(b, ia), f.mul(c, ia), f.mul(d, ia), f.mul(e, ia))
        };
        // x divides the quartic when e1=0. Solve the cubic cofactor,
        // bypassing Ferrari's resolvent and the Frobenius fallback.
        if e1 == 0 {
            let mut roots = self.cubic_monic_before_sparse(b1, c1, d1);
            roots.push(0);
            roots.sort_unstable();
            roots.dedup();
            return Ok((roots, false));
        }
        // x = y - B, B = b1/4
        let (bb, pp, qq, rr) = self.quartic_depressed_coeffs(b1, c1, d1, e1);
        let mut ys: Vec<u64> = vec![];
        let mut fallback = false;
        if qq == 0 {
            for z in self.quad_monic(pp, rr) {
                match self.sqrt(z) {
                    None => {}
                    Some(0) => ys.push(0),
                    Some(s) => {
                        ys.push(s);
                        ys.push(f.neg(s));
                    }
                }
            }
        } else {
            // resolvent in z = (r1+r2)^2:  z^3 + 2P z^2 + (P^2 - 4R) z - Q^2 = 0
            let (z2, z1, z0) = self.quartic_resolvent_coeffs(pp, qq, rr);
            let zs = self.cubic_monic_before_sparse(z2, z1, z0);
            if zs.is_empty() {
                // Frobenius type (1,3): no rational pairing; the unique root needs F_{p^3} radicals.
                // Use the distinct-degree (CZ) path on the depressed quartic - NOT radical.
                fallback = true;
                ys = frobenius_roots(f.p, &[rr, qq, pp, 0, 1])?;
            } else {
                for &z in &zs {
                    if let Some(al) = self.sqrt(z) {
                        // quartic = (y^2 + al y + be)(y^2 - al y + ga)
                        let qa = f.mul(qq, f.inv(al));
                        let s = f.add(pp, z);
                        let be = f.half(f.sub(s, qa));
                        let ga = f.half(f.add(s, qa));
                        ys.extend(self.quad_monic(al, be));
                        ys.extend(self.quad_monic(f.neg(al), ga));
                        break;
                    }
                }
            }
        }
        let mut xs: Vec<u64> = ys.into_iter().map(|y| f.sub(y, bb)).collect();
        xs.sort_unstable();
        xs.dedup();
        Ok((xs, fallback))
    }

    fn roots_before_sparse(&self, f: &[u64]) -> Result<RadicalRoots, String> {
        let fp = &self.f;
        // Polynomials of degree <=4 fit in a stack buffer. Scan trailing
        // zeroes first (including coefficients congruent to 0 mod p) so
        // arbitrarily padded input remains accepted without a heap Vec.
        let mut n = f.len();
        while n > 0 && f[n - 1] % fp.p == 0 {
            n -= 1;
        }
        if n > 5 {
            return Err("radical root finder supports degree <= 4".into());
        }
        let mut c = [0u64; 5];
        for j in 0..n {
            c[j] = f[j] % fp.p;
        }
        let ok = |roots: Vec<u64>| {
            Ok(RadicalRoots {
                roots,
                frobenius_fallback: false,
            })
        };
        match n {
            0 => Err("zero polynomial".into()),
            1 => ok(vec![]),
            2 => {
                // x + c0 and a*x have roots without an inversion.
                let root = if c[0] == 0 { 0 }
                    else if c[1] == 1 { fp.neg(c[0]) }
                    else { fp.mul(fp.neg(c[0]), fp.inv(c[1])) };
                ok(vec![root])
            },
            3 => ok(self.quad_roots(c[2], c[1], c[0])),
            4 => {
                // Monic cubics are common in normalized rootfinding.
                // Avoid one inversion and three modular products.
                let mut v = if c[3] == 1 {
                    self.cubic_monic_before_sparse(c[2], c[1], c[0])
                } else {
                    let ia = fp.inv(c[3]);
                    self.cubic_monic_before_sparse(fp.mul(c[2], ia), fp.mul(c[1], ia), fp.mul(c[0], ia))
                };
                v.sort_unstable();
                v.dedup();
                ok(v)
            }
            5 => {
                let (roots, fb) = self.quartic_before_round19(c[4], c[3], c[2], c[1], c[0])?;
                Ok(RadicalRoots {
                    roots,
                    frobenius_fallback: fb,
                })
            }
            _ => Err("radical root finder supports degree <= 4".into()),
        }
    }
}
#[cfg(test)]
mod sparse_round19_tests {
    use super::*;
    fn next(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *seed
    }
    fn sorted(mut r: Vec<u64>) -> Vec<u64> {
        r.sort_unstable(); r.dedup(); r
    }
    #[test]
    fn sparse_cubics_and_even_quartics_match_frozen_reference() {
        let mut seed = 0x2f00_2026_1118_0019u64;
        for p in [5u64,7,11,17,97,487,17_497,998_244_353,
                  1_000_000_007,1_000_000_009,MAX_PRIME] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            let f = &ctx.f;
            for j in 0..96 {
                let a = 1 + next(&mut seed) % (p-1);
                let b = next(&mut seed) % p;
                let c = next(&mut seed) % p;
                let d = next(&mut seed) % p;
                let pp = if j % 4 == 0 {0} else {c};
                let q = if j % 3 == 0 {0} else {d};
                if q == 0 {
                    assert_eq!(sorted(ctx.cubic_depressed_before_sparse(pp,q)),
                               sorted(ctx.cubic_depressed(pp,q)),
                               "depressed cubic p={p} j={j}");
                }
                let (c2,c1,c0) = if j%2 == 0 {(b,c,0)} else {(0,c,d)};
                assert_eq!(sorted(ctx.cubic_monic_before_sparse(c2,c1,c0)),
                           sorted(ctx.cubic_monic(c2,c1,c0)),
                           "monic cubic p={p} j={j}");
                let cubic = [0,c,b,a];
                assert_eq!(ctx.roots_before_sparse(&cubic).unwrap(),
                           ctx.roots(&cubic).unwrap(), "general cubic p={p} j={j}");
                let e = if j%2==0 {next(&mut seed)%p} else {
                    let r=next(&mut seed)%p; let s=next(&mut seed)%p;
                    f.mul(a,f.mul(f.sqr(r),f.sqr(s)))
                };
                let even = [e,0,c,0,a];
                assert_eq!(ctx.roots_before_sparse(&even).unwrap(),
                           ctx.roots(&even).unwrap(), "even quartic p={p} j={j}");
            }
        }
    }
    fn digest(r: &RadicalRoots) -> u64 {
        r.roots.iter().fold(r.frobenius_fallback as u64,|h,&v|h.wrapping_mul(33).wrapping_add(v))
    }
    #[test]
    #[ignore]
    fn bench_sparse_round19_actual_rust() {
        fn timed<T:Copy,F:Fn(T)->u64>(xs:&[T],reps:usize,f:&F)->f64 {
            let t=std::time::Instant::now(); let mut digest=0u64;
            for _ in 0..reps {for &x in xs {
                digest=digest.wrapping_add(std::hint::black_box(f(std::hint::black_box(x))));
            }}
            std::hint::black_box(digest);
            t.elapsed().as_nanos() as f64/(xs.len()*reps) as f64
        }
        fn paired<T:Copy,F:Fn(T)->u64,G:Fn(T)->u64>(
            label:&str,xs:&[T],reps:usize,old:F,new:G) {
            let mut a=Vec::new();let mut b=Vec::new();
            for i in 0..13 {
                if i%2==0 {a.push(timed(xs,reps,&old));b.push(timed(xs,reps,&new))}
                else {b.push(timed(xs,reps,&new));a.push(timed(xs,reps,&old))}
            }
            a.sort_by(f64::total_cmp); b.sort_by(f64::total_cmp);
            println!("{label}: before={:.3}ns after={:.3}ns speedup={:.3}x",
                     a[6],b[6],a[6]/b[6]);
        }
        let ctx=RadicalRootCtx::new(1_000_000_007).unwrap();
        let mut seed=0x1118_2026_0019_0abcu64;
        let mut depressed=Vec::new();let mut monic=Vec::new();
        let mut general=Vec::new();let mut even=Vec::new();
        let mut even_monic=Vec::new();let mut dense=Vec::new();
        for _ in 0..256 {
            let a=2+next(&mut seed)%(ctx.f.p-2);
            let b=next(&mut seed)%ctx.f.p;
            let c=next(&mut seed)%ctx.f.p;
            let e=1+next(&mut seed)%(ctx.f.p-1);
            depressed.push(c); monic.push([0,c,b,1]);
            general.push([0,c,b,a]); even.push([e,0,c,0,a]);
            even_monic.push([e,0,c,0,1]); dense.push([e,c,b,a]);
        }
        paired("depressed Q=0 p=1000000007",&depressed,64,
            |p| sorted(ctx.cubic_depressed_before_sparse(p,0)).iter().fold(0u64,|h,&v|h.wrapping_mul(33).wrapping_add(v)),
            |p| sorted(ctx.cubic_depressed(p,0)).iter().fold(0u64,|h,&v|h.wrapping_mul(33).wrapping_add(v)));
        paired("monic cubic c0=0 p=1000000007",&monic,64,
            |x|digest(&ctx.roots_before_sparse(&x).unwrap()),
            |x|digest(&ctx.roots(&x).unwrap()));
        paired("nonmonic cubic c0=0 p=1000000007",&general,64,
            |x|digest(&ctx.roots_before_sparse(&x).unwrap()),
            |x|digest(&ctx.roots(&x).unwrap()));
        paired("nonmonic even quartic p=1000000007",&even,64,
            |x|digest(&ctx.roots_before_sparse(&x).unwrap()),
            |x|digest(&ctx.roots(&x).unwrap()));
        paired("monic even quartic unchanged control",&even_monic,64,
            |x|digest(&ctx.roots_before_sparse(&x).unwrap()),
            |x|digest(&ctx.roots(&x).unwrap()));
        paired("dense cubic unchanged control",&dense,64,
            |x|digest(&ctx.roots_before_sparse(&x).unwrap()),
            |x|digest(&ctx.roots(&x).unwrap()));
    }
}
