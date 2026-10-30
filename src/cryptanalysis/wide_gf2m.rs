//! `GF(2^n)` for `n ≤ 127`, one element per `u128`: the field under the
//! wide Koblitz curves the m = 83 gate needs (AGENTS.md §8a), where the
//! one-word [`Gf2`](crate::cryptanalysis::semaev_decomp::Gf2) stops at 63.
//!
//! Polynomial basis, bit `i` the coefficient of `z^i`.  A product is the
//! 256-bit carry-less product of two `u128`s (Karatsuba over three 64-bit
//! carry-less multiplies: `PCLMULQDQ` on x86-64, `PMULL` on AArch64, a
//! bit-serial loop elsewhere), folded back through the modulus's low
//! terms.  Inversion is Itoh–Tsujii; a batch is inverted with one
//! inversion by Montgomery's trick, as the one-word field does it.
//!
//! The tests check every operation against the repository's
//! arbitrary-width [`F2mElement`](crate::binary_ecc::F2mElement) on the
//! m = 83 modulus `z^83 + z^45 + z^2 + z + 1`.
//!
//! # Three limbs, `n ≤ 191`
//!
//! [`WideGf2x3`] is the same field interface over `[u64; 3]` ([`Fe3`]):
//! Karatsuba over three limbs (six carry-less multiplies), bit-spread
//! squaring, a general sparse-polynomial fold, and a hand-folded reduction
//! for the pinned ECC2K-130 modulus `z^131 + z^13 + z^2 + z + 1`
//! ([`ECC2K130_LOW_TERMS`]); `n = 163` and `n = 191` exercise the general
//! path.  Measured on this host (Xeon @ 2.80 GHz, `pclmulqdq`, 4 vCPUs,
//! shared, dependent chains of 4·10^6 operations, three runs):
//! `n = 131` mul 24.0 / 24.7 / 33.9 ns, sqr 23.1 / 24.4 / 28.4 ns;
//! `n = 127` `u128` mul 13.6 / 13.9 / 13.4 ns, sqr 13.8 / 15.0 / 13.7 ns
//! (`cargo test --release --lib -- wide_gf2m::tests::bench --ignored
//! --nocapture`).  The three-limb squaring is not yet cheaper than its
//! multiply: the mask-and-shift bit-spread dominates and is the next thing
//! to tune (a byte table or `PDEP`).

use crate::binary_ecc::IrreduciblePoly;

/// The field `GF(2)[z] / (f)` with `deg f = n ≤ 127`.
#[derive(Clone, Debug)]
pub struct WideGf2 {
    pub n: u32,
    /// The exponents of `f` below `z^n`, ascending.
    low_terms: Vec<u32>,
    mask: u128,
    has_clmul: bool,
}

/// The bit-serial 64 × 64 → 128 carry-less product.
fn clmul_portable(a: u64, b: u64) -> u128 {
    let mut acc = 0u128;
    let a = a as u128;
    let mut b = b;
    let mut i = 0;
    while b != 0 {
        if b & 1 == 1 {
            acc ^= a << i;
        }
        b >>= 1;
        i += 1;
    }
    acc
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_hw(a: u64, b: u64) -> u128 {
    use std::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let z = _mm_clmulepi64_si128::<0x00>(x, y);
    let lo = _mm_cvtsi128_si64(z) as u64;
    let hi = _mm_cvtsi128_si64(_mm_srli_si128::<8>(z)) as u64;
    ((hi as u128) << 64) | (lo as u128)
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "aes")]
unsafe fn clmul_hw(a: u64, b: u64) -> u128 {
    std::arch::aarch64::vmull_p64(a, b)
}

/// `(lo, hi) << s` for a 256-bit value held as two halves, `s < 128`.
#[inline(always)]
fn shl256(lo: u128, hi: u128, s: u32) -> (u128, u128) {
    if s == 0 {
        (lo, hi)
    } else {
        (lo << s, (hi << s) | (lo >> (128 - s)))
    }
}

impl WideGf2 {
    /// The field of `irr`, for `2 ≤ n ≤ 127`.
    pub fn new(irr: &IrreduciblePoly) -> Self {
        #[cfg(target_arch = "x86_64")]
        let has_clmul = std::arch::is_x86_feature_detected!("pclmulqdq");
        #[cfg(target_arch = "aarch64")]
        let has_clmul = std::arch::is_aarch64_feature_detected!("aes");
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let has_clmul = false;
        Self::with_kernel(irr, has_clmul)
    }

    /// The same field with the multiply chosen by the caller (`has_clmul`
    /// must be false unless the CPU has the instruction), so a test can
    /// compare both kernels on one machine.
    pub fn with_kernel(irr: &IrreduciblePoly, has_clmul: bool) -> Self {
        let n = irr.degree;
        assert!(
            (2..=127).contains(&n),
            "WideGf2 handles 2 ≤ n ≤ 127, not {n}"
        );
        let mut low_terms = irr.low_terms.clone();
        low_terms.sort_unstable();
        low_terms.dedup();
        assert!(
            low_terms.iter().all(|&t| t < n),
            "a low term at or above z^n"
        );
        Self {
            n,
            low_terms,
            mask: (1u128 << n) - 1,
            has_clmul,
        }
    }

    /// The modulus with its leading term, as a word.
    pub fn modulus_bits(&self) -> u128 {
        self.low_terms
            .iter()
            .fold(1u128 << self.n, |acc, &t| acc | (1u128 << t))
    }

    /// Which carry-less kernel this instance uses.
    pub fn kernel_name(&self) -> &'static str {
        if !self.has_clmul {
            "portable"
        } else if cfg!(target_arch = "x86_64") {
            "pclmulqdq"
        } else {
            "pmull"
        }
    }

    #[inline(always)]
    fn clmul64(&self, a: u64, b: u64) -> u128 {
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        if self.has_clmul {
            // SAFETY: `has_clmul` is set only when the CPU reports the
            // instruction (or by a caller bound to the same rule).
            return unsafe { clmul_hw(a, b) };
        }
        clmul_portable(a, b)
    }

    /// The 256-bit carry-less product `a·b`, unreduced, as `(lo, hi)`.
    #[inline(always)]
    pub fn mul_unreduced(&self, a: u128, b: u128) -> (u128, u128) {
        let (a0, a1) = (a as u64, (a >> 64) as u64);
        let (b0, b1) = (b as u64, (b >> 64) as u64);
        let lo = self.clmul64(a0, b0);
        let hi = self.clmul64(a1, b1);
        let mid = self.clmul64(a0 ^ a1, b0 ^ b1) ^ lo ^ hi;
        (lo ^ (mid << 64), hi ^ (mid >> 64))
    }

    /// Reduce a value below `z^{2n−1}` modulo `f`.
    #[inline(always)]
    pub fn reduce(&self, mut lo: u128, mut hi: u128) -> u128 {
        let n = self.n;
        loop {
            // The part at and above z^n, shifted down: below 2^(n−1) < 2^127.
            let top = if n == 128 {
                hi
            } else {
                (lo >> n) | (hi << (128 - n))
            };
            if top == 0 {
                return lo;
            }
            lo &= self.mask;
            hi = 0;
            for &t in &self.low_terms {
                let (l, h) = shl256(top, 0, t);
                lo ^= l;
                hi ^= h;
            }
        }
    }

    #[inline(always)]
    pub fn mul(&self, a: u128, b: u128) -> u128 {
        let (lo, hi) = self.mul_unreduced(a, b);
        self.reduce(lo, hi)
    }

    #[inline(always)]
    pub fn sqr(&self, a: u128) -> u128 {
        self.mul(a, a)
    }

    /// `a^{2^k}`.
    pub fn sqr_k(&self, mut a: u128, k: u32) -> u128 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }

    /// `a^{-1}` by Itoh–Tsujii: `a^{-1} = (a^{2^{n−1} − 1})^2`.  `0` maps
    /// to `0`.
    pub fn inv(&self, a: u128) -> u128 {
        if a == 0 {
            return 0;
        }
        // beta(k) = a^{2^k − 1}; build beta(n − 1) along the bits of n − 1.
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let mut c = a;
        let mut k = 1u32;
        for i in (0..bits - 1).rev() {
            c = self.mul(c, self.sqr_k(c, k));
            k *= 2;
            if (e >> i) & 1 == 1 {
                c = self.mul(self.sqr(c), a);
                k += 1;
            }
        }
        debug_assert_eq!(k, e);
        self.sqr(c)
    }

    /// Invert every nonzero entry of `xs` in place with one inversion
    /// (Montgomery's trick); zeros stay zero.  `scratch` is reused.
    pub fn batch_inv(&self, xs: &mut [u128], scratch: &mut Vec<u128>) {
        scratch.clear();
        let mut acc = 1u128;
        for &x in xs.iter() {
            scratch.push(acc);
            if x != 0 {
                acc = self.mul(acc, x);
            }
        }
        let mut inv = self.inv(acc);
        for i in (0..xs.len()).rev() {
            let x = xs[i];
            if x == 0 {
                continue;
            }
            xs[i] = self.mul(inv, scratch[i]);
            inv = self.mul(inv, x);
        }
    }

    /// The absolute trace `Tr(a) = Σ a^{2^i}`, `0` or `1`.
    pub fn trace(&self, a: u128) -> u128 {
        let mut t = a;
        let mut acc = a;
        for _ in 1..self.n {
            t = self.sqr(t);
            acc ^= t;
        }
        acc
    }

    /// A root `z` of `z² + z = c` for odd `n` and `Tr(c) = 0`: the
    /// half-trace `Σ_{i ≤ (n−1)/2} c^{4^i}`.  `None` otherwise.
    pub fn solve_quadratic(&self, c: u128) -> Option<u128> {
        if self.n.is_multiple_of(2) || self.trace(c) != 0 {
            return None;
        }
        let mut t = c;
        let mut acc = c;
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(self.sqr(t));
            acc ^= t;
        }
        debug_assert_eq!(self.sqr(acc) ^ acc, c);
        Some(acc)
    }

    pub fn mask(&self) -> u128 {
        self.mask
    }
}

// ---------------------------------------------------------------------
// Three limbs: GF(2^n) for n <= 191.
// ---------------------------------------------------------------------

/// An element of [`WideGf2x3`]: three little-endian 64-bit limbs, bit `i`
/// the coefficient of `z^i`.
pub type Fe3 = [u64; 3];

/// The ECC2K-130 field `GF(2^131)` has degree 131.
pub const ECC2K130_N: u32 = 131;
/// The ECC2K-130 reduction polynomial `x^131 + x^13 + x^2 + x + 1`: its
/// terms below `x^131`, descending.
pub const ECC2K130_LOW_TERMS: [u32; 4] = [13, 2, 1, 0];
/// NIST B-163 / K-163 reduction polynomial `x^163 + x^7 + x^6 + x^3 + 1`.
pub const NIST163_LOW_TERMS: [u32; 4] = [7, 6, 3, 0];
/// The `x^191 + x^9 + 1` trinomial (NIST-size field of the 191-bit curves).
pub const NIST191_LOW_TERMS: [u32; 2] = [9, 0];

/// The ECC2K-130 modulus as an [`IrreduciblePoly`].
pub fn ecc2k130_poly() -> IrreduciblePoly {
    IrreduciblePoly {
        degree: ECC2K130_N,
        low_terms: ECC2K130_LOW_TERMS.to_vec(),
    }
}

#[inline(always)]
fn clmul_dispatch(has_clmul: bool, a: u64, b: u64) -> u128 {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    if has_clmul {
        // SAFETY: `has_clmul` is set only when the CPU reports the
        // instruction (or by a caller bound to the same rule).
        return unsafe { clmul_hw(a, b) };
    }
    let _ = has_clmul;
    clmul_portable(a, b)
}

/// Interleave the bits of `x` with zeros: bit `i` moves to bit `2i`.
#[inline(always)]
fn spread64(x: u64) -> u128 {
    let mut v = x as u128;
    v = (v | (v << 32)) & 0x0000_0000_ffff_ffff_0000_0000_ffff_ffff;
    v = (v | (v << 16)) & 0x0000_ffff_0000_ffff_0000_ffff_0000_ffff;
    v = (v | (v << 8)) & 0x00ff_00ff_00ff_00ff_00ff_00ff_00ff_00ff;
    v = (v | (v << 4)) & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    v = (v | (v << 2)) & 0x3333_3333_3333_3333_3333_3333_3333_3333;
    v = (v | (v << 1)) & 0x5555_5555_5555_5555_5555_5555_5555_5555;
    v
}

#[inline(always)]
fn xor_at(c: &mut [u64; 6], v: u128, limb: usize) {
    c[limb] ^= v as u64;
    c[limb + 1] ^= (v >> 64) as u64;
}

/// `v >> s` for a 384-bit value.
#[inline(always)]
fn shr6(v: &[u64; 6], s: u32) -> [u64; 6] {
    let (w, b) = ((s / 64) as usize, s % 64);
    let mut out = [0u64; 6];
    for i in 0..6 - w {
        let lo = v[i + w] >> b;
        let hi = if b != 0 && i + w + 1 < 6 {
            v[i + w + 1] << (64 - b)
        } else {
            0
        };
        out[i] = lo | hi;
    }
    out
}

/// `acc ^= v << s` (bits past 384 dropped).
#[inline(always)]
fn xor_shl6(acc: &mut [u64; 6], v: &[u64; 6], s: u32) {
    let (w, b) = ((s / 64) as usize, s % 64);
    for i in w..6 {
        let lo = v[i - w] << b;
        let hi = if b != 0 && i > w {
            v[i - w - 1] >> (64 - b)
        } else {
            0
        };
        acc[i] ^= lo | hi;
    }
}

/// `GF(2)[z] / (f)` with `deg f = n`, `2 ≤ n ≤ 191`, one element per
/// [`Fe3`].  The interface mirrors [`WideGf2`]: `mul`, `sqr`, `sqr_k`,
/// `inv`, `batch_inv`, `trace`, `solve_quadratic`; addition is `^` on the
/// limbs ([`add`](Self::add)).  The ECC2K-130 modulus gets a hand-folded
/// reduction; every other modulus goes through the general sparse fold.
#[derive(Clone, Debug)]
pub struct WideGf2x3 {
    pub n: u32,
    low_terms: Vec<u32>,
    mask: Fe3,
    has_clmul: bool,
    ecc2k130: bool,
}

impl WideGf2x3 {
    /// The field of `irr`, for `2 ≤ n ≤ 191`.
    pub fn new(irr: &IrreduciblePoly) -> Self {
        #[cfg(target_arch = "x86_64")]
        let has_clmul = std::arch::is_x86_feature_detected!("pclmulqdq");
        #[cfg(target_arch = "aarch64")]
        let has_clmul = std::arch::is_aarch64_feature_detected!("aes");
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let has_clmul = false;
        Self::with_kernel(irr, has_clmul)
    }

    /// The ECC2K-130 field `GF(2^131)`, `z^131 + z^13 + z^2 + z + 1`.
    pub fn ecc2k130() -> Self {
        Self::new(&ecc2k130_poly())
    }

    /// As [`new`](Self::new) with the carry-less kernel chosen by the
    /// caller (`has_clmul` must be false unless the CPU has the
    /// instruction).
    pub fn with_kernel(irr: &IrreduciblePoly, has_clmul: bool) -> Self {
        Self::build(irr, has_clmul, true)
    }

    /// The same field forced onto the general reduction, so a test can
    /// compare it with the specialised ECC2K-130 path.
    pub fn with_general_reduction(irr: &IrreduciblePoly, has_clmul: bool) -> Self {
        Self::build(irr, has_clmul, false)
    }

    fn build(irr: &IrreduciblePoly, has_clmul: bool, allow_special: bool) -> Self {
        let n = irr.degree;
        assert!(
            (2..=191).contains(&n),
            "WideGf2x3 handles 2 ≤ n ≤ 191, not {n}"
        );
        let mut low_terms = irr.low_terms.clone();
        low_terms.sort_unstable();
        low_terms.dedup();
        assert!(
            low_terms.iter().all(|&t| t < n),
            "a low term at or above z^n"
        );
        let mut mask = [0u64; 3];
        for (i, m) in mask.iter_mut().enumerate() {
            let lo = 64 * i as u32;
            *m = if n >= lo + 64 {
                u64::MAX
            } else if n > lo {
                (1u64 << (n - lo)) - 1
            } else {
                0
            };
        }
        let mut pinned: Vec<u32> = ECC2K130_LOW_TERMS.to_vec();
        pinned.sort_unstable();
        let ecc2k130 = allow_special && n == ECC2K130_N && low_terms == pinned;
        Self {
            n,
            low_terms,
            mask,
            has_clmul,
            ecc2k130,
        }
    }

    /// Whether the hand-folded ECC2K-130 reduction is in use.
    pub fn uses_pinned_reduction(&self) -> bool {
        self.ecc2k130
    }

    pub fn kernel_name(&self) -> &'static str {
        if !self.has_clmul {
            "portable"
        } else if cfg!(target_arch = "x86_64") {
            "pclmulqdq"
        } else {
            "pmull"
        }
    }

    pub fn mask(&self) -> Fe3 {
        self.mask
    }

    pub const ZERO: Fe3 = [0; 3];
    pub const ONE: Fe3 = [1, 0, 0];

    #[inline(always)]
    pub fn add(&self, a: Fe3, b: Fe3) -> Fe3 {
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]]
    }

    /// A `u128` field element (`n ≤ 127`) as limbs.
    pub fn from_u128(&self, v: u128) -> Fe3 {
        assert!(self.n <= 127, "from_u128 needs n ≤ 127");
        [v as u64, (v >> 64) as u64, 0]
    }

    /// The `u128` of an element of a field with `n ≤ 127`.
    pub fn to_u128(&self, a: Fe3) -> u128 {
        assert!(self.n <= 127 && a[2] == 0, "element does not fit a u128");
        a[0] as u128 | ((a[1] as u128) << 64)
    }

    /// The 384-bit carry-less product, unreduced: Karatsuba over three
    /// limbs, six 64 × 64 multiplies.
    #[inline(always)]
    pub fn mul_unreduced(&self, a: Fe3, b: Fe3) -> [u64; 6] {
        let k = self.has_clmul;
        let p0 = clmul_dispatch(k, a[0], b[0]);
        let p1 = clmul_dispatch(k, a[1], b[1]);
        let p2 = clmul_dispatch(k, a[2], b[2]);
        let m01 = clmul_dispatch(k, a[0] ^ a[1], b[0] ^ b[1]) ^ p0 ^ p1;
        let m02 = clmul_dispatch(k, a[0] ^ a[2], b[0] ^ b[2]) ^ p0 ^ p2;
        let m12 = clmul_dispatch(k, a[1] ^ a[2], b[1] ^ b[2]) ^ p1 ^ p2;
        let mut c = [0u64; 6];
        xor_at(&mut c, p0, 0);
        xor_at(&mut c, m01, 1);
        xor_at(&mut c, m02 ^ p1, 2);
        xor_at(&mut c, m12, 3);
        c[4] ^= p2 as u64;
        c[5] ^= (p2 >> 64) as u64;
        c
    }

    /// Reduce a value below `z^{2n−1}`: the general sparse fold.
    pub fn reduce_general(&self, mut c: [u64; 6]) -> Fe3 {
        let n = self.n;
        loop {
            let top = shr6(&c, n);
            if top == [0; 6] {
                return [c[0], c[1], c[2]];
            }
            c = [
                c[0] & self.mask[0],
                c[1] & self.mask[1],
                c[2] & self.mask[2],
                0,
                0,
                0,
            ];
            for &t in &self.low_terms {
                xor_shl6(&mut c, &top, t);
            }
        }
    }

    /// Reduce modulo `z^131 + z^13 + z^2 + z + 1`, input below `z^261`.
    #[inline(always)]
    fn reduce_ecc2k130(c: [u64; 6]) -> Fe3 {
        // top = c >> 131 (130 bits, three limbs).
        let t0 = (c[2] >> 3) | (c[3] << 61);
        let t1 = (c[3] >> 3) | (c[4] << 61);
        let t2 = (c[4] >> 3) | (c[5] << 61);
        // z^131 = z^13 + z^2 + z + 1: r = low ^ top·(1 + z + z² + z^13).
        let r0 = c[0] ^ t0 ^ (t0 << 1) ^ (t0 << 2) ^ (t0 << 13);
        let r1 = c[1]
            ^ t1
            ^ ((t1 << 1) | (t0 >> 63))
            ^ ((t1 << 2) | (t0 >> 62))
            ^ ((t1 << 13) | (t0 >> 51));
        let r2 = (c[2] & 7)
            ^ t2
            ^ ((t2 << 1) | (t1 >> 63))
            ^ ((t2 << 2) | (t1 >> 62))
            ^ ((t2 << 13) | (t1 >> 51));
        // r has at most 142 bits: fold the (≤ 12-bit) overflow once more.
        let h = r2 >> 3;
        [r0 ^ h ^ (h << 1) ^ (h << 2) ^ (h << 13), r1, r2 & 7]
    }

    #[inline(always)]
    pub fn reduce(&self, c: [u64; 6]) -> Fe3 {
        if self.ecc2k130 {
            Self::reduce_ecc2k130(c)
        } else {
            self.reduce_general(c)
        }
    }

    #[inline(always)]
    pub fn mul(&self, a: Fe3, b: Fe3) -> Fe3 {
        self.reduce(self.mul_unreduced(a, b))
    }

    /// Squaring: spread the bits of each limb (no multiplies), reduce.
    #[inline(always)]
    pub fn sqr(&self, a: Fe3) -> Fe3 {
        let mut c = [0u64; 6];
        for i in 0..3 {
            let s = spread64(a[i]);
            c[2 * i] = s as u64;
            c[2 * i + 1] = (s >> 64) as u64;
        }
        self.reduce(c)
    }

    /// `a^{2^k}`.
    pub fn sqr_k(&self, mut a: Fe3, k: u32) -> Fe3 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }

    /// `a^{-1}` by Itoh–Tsujii, as [`WideGf2::inv`]; `0` maps to `0`.
    pub fn inv(&self, a: Fe3) -> Fe3 {
        if a == Self::ZERO {
            return Self::ZERO;
        }
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let mut c = a;
        let mut k = 1u32;
        for i in (0..bits - 1).rev() {
            c = self.mul(c, self.sqr_k(c, k));
            k *= 2;
            if (e >> i) & 1 == 1 {
                c = self.mul(self.sqr(c), a);
                k += 1;
            }
        }
        debug_assert_eq!(k, e);
        self.sqr(c)
    }

    /// Invert every nonzero entry in place with one inversion
    /// (Montgomery's trick); zeros stay zero.
    pub fn batch_inv(&self, xs: &mut [Fe3], scratch: &mut Vec<Fe3>) {
        scratch.clear();
        let mut acc = Self::ONE;
        for &x in xs.iter() {
            scratch.push(acc);
            if x != Self::ZERO {
                acc = self.mul(acc, x);
            }
        }
        let mut inv = self.inv(acc);
        for i in (0..xs.len()).rev() {
            let x = xs[i];
            if x == Self::ZERO {
                continue;
            }
            xs[i] = self.mul(inv, scratch[i]);
            inv = self.mul(inv, x);
        }
    }

    /// The absolute trace, as `0` or `1`.
    pub fn trace(&self, a: Fe3) -> u64 {
        let mut t = a;
        let mut acc = a;
        for _ in 1..self.n {
            t = self.sqr(t);
            acc = self.add(acc, t);
        }
        debug_assert!(acc[1] == 0 && acc[2] == 0 && acc[0] <= 1);
        acc[0]
    }

    /// A root of `z² + z = c` for odd `n` and `Tr(c) = 0` (half-trace).
    pub fn solve_quadratic(&self, c: Fe3) -> Option<Fe3> {
        if self.n.is_multiple_of(2) || self.trace(c) != 0 {
            return None;
        }
        let mut t = c;
        let mut acc = c;
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(self.sqr(t));
            acc = self.add(acc, t);
        }
        debug_assert_eq!(self.add(self.sqr(acc), acc), c);
        Some(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use num_bigint::BigUint;

    /// AGENTS.md §8a's m = 83 modulus.
    fn m83() -> IrreduciblePoly {
        IrreduciblePoly {
            degree: 83,
            low_terms: vec![45, 2, 1, 0],
        }
    }

    fn lib(v: u128, n: u32) -> F2mElement {
        F2mElement::from_biguint(&BigUint::from(v), n)
    }

    fn back(e: &F2mElement) -> u128 {
        let bits = e.raw_bits();
        bits.first().copied().unwrap_or(0) as u128
            | ((bits.get(1).copied().unwrap_or(0) as u128) << 64)
    }

    fn samples(n: u32) -> Vec<u128> {
        let mask = (1u128 << n) - 1;
        let mut s = 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210u128;
        let mut out = vec![0, 1, 2, mask, 1u128 << (n - 1)];
        for _ in 0..200 {
            s = s
                .wrapping_mul(0x2360_ed05_1fc6_5da4_4385_df64_9fcc_f645)
                .wrapping_add(0x5851_f42d_4c95_7f2d);
            out.push((s ^ (s >> 61)) & mask);
        }
        out
    }

    #[test]
    fn mul_matches_the_library_field_on_both_kernels() {
        let irr = m83();
        let portable = WideGf2::with_kernel(&irr, false);
        let native = WideGf2::new(&irr);
        let xs = samples(83);
        for (i, &a) in xs.iter().enumerate() {
            let b = xs[(i * 7 + 3) % xs.len()];
            let want = back(&lib(a, 83).mul(&lib(b, 83), &irr));
            assert_eq!(portable.mul(a, b), want, "portable a={a:x} b={b:x}");
            assert_eq!(
                native.mul(a, b),
                want,
                "{} a={a:x} b={b:x}",
                native.kernel_name()
            );
        }
    }

    #[test]
    fn mul_matches_the_library_at_the_width_limits() {
        for irr in [
            IrreduciblePoly {
                degree: 127,
                low_terms: vec![1, 0],
            },
            IrreduciblePoly {
                degree: 67,
                low_terms: vec![5, 2, 1, 0],
            },
        ] {
            let f = WideGf2::new(&irr);
            let xs = samples(irr.degree);
            for (i, &a) in xs.iter().enumerate() {
                let b = xs[(i * 5 + 1) % xs.len()];
                assert_eq!(
                    f.mul(a, b),
                    back(&lib(a, irr.degree).mul(&lib(b, irr.degree), &irr))
                );
            }
        }
    }

    #[test]
    fn inverses_and_batches_invert() {
        let f = WideGf2::new(&m83());
        let mut xs = samples(83);
        for &x in &xs {
            if x != 0 {
                assert_eq!(f.mul(x, f.inv(x)), 1, "x={x:x}");
            }
        }
        let orig = xs.clone();
        let mut scratch = Vec::new();
        f.batch_inv(&mut xs, &mut scratch);
        for (x, y) in orig.iter().zip(&xs) {
            if *x == 0 {
                assert_eq!(*y, 0);
            } else {
                assert_eq!(f.mul(*x, *y), 1);
            }
        }
    }

    #[test]
    fn half_trace_solves_artin_schreier() {
        let f = WideGf2::new(&m83());
        let mut solved = 0;
        for c in samples(83) {
            match f.solve_quadratic(c) {
                Some(z) => {
                    assert_eq!(f.sqr(z) ^ z, c);
                    solved += 1;
                }
                None => assert_eq!(f.trace(c), 1),
            }
        }
        assert!(solved > 50, "about half of random c have trace 0: {solved}");
    }

    #[test]
    fn frobenius_has_order_n() {
        let f = WideGf2::new(&m83());
        for x in samples(83) {
            assert_eq!(f.sqr_k(x, 83), x);
        }
    }

    // ---- three-limb field -------------------------------------------

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }
        fn fe3(&mut self, f: &WideGf2x3) -> Fe3 {
            let m = f.mask();
            [self.next() & m[0], self.next() & m[1], self.next() & m[2]]
        }
    }

    fn poly(n: u32, low: &[u32]) -> IrreduciblePoly {
        IrreduciblePoly {
            degree: n,
            low_terms: low.to_vec(),
        }
    }

    fn three_fields() -> Vec<WideGf2x3> {
        vec![
            WideGf2x3::ecc2k130(),
            WideGf2x3::new(&poly(163, &NIST163_LOW_TERMS)),
            WideGf2x3::new(&poly(191, &NIST191_LOW_TERMS)),
        ]
    }

    fn big(a: Fe3) -> BigUint {
        BigUint::from(a[0]) | (BigUint::from(a[1]) << 64) | (BigUint::from(a[2]) << 128)
    }

    fn lib3(a: Fe3, n: u32) -> F2mElement {
        F2mElement::from_biguint(&big(a), n)
    }

    fn back3(e: &F2mElement) -> Fe3 {
        let b = e.raw_bits();
        [
            b.first().copied().unwrap_or(0),
            b.get(1).copied().unwrap_or(0),
            b.get(2).copied().unwrap_or(0),
        ]
    }

    #[test]
    fn pinned_polynomial_is_the_ecc2k130_one() {
        let f = WideGf2x3::ecc2k130();
        assert_eq!(f.n, 131);
        assert!(f.uses_pinned_reduction());
        assert_eq!(ecc2k130_poly().low_terms, vec![13, 2, 1, 0]);
        let g = WideGf2x3::with_general_reduction(&ecc2k130_poly(), false);
        assert!(!g.uses_pinned_reduction());
    }

    #[test]
    fn x3_field_axioms_at_131_163_191() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for f in three_fields() {
            for _ in 0..300 {
                let (a, b, c) = (rng.fe3(&f), rng.fe3(&f), rng.fe3(&f));
                assert_eq!(
                    f.mul(f.mul(a, b), c),
                    f.mul(a, f.mul(b, c)),
                    "assoc n={}",
                    f.n
                );
                assert_eq!(f.mul(a, b), f.mul(b, a));
                assert_eq!(
                    f.mul(a, f.add(b, c)),
                    f.add(f.mul(a, b), f.mul(a, c)),
                    "distrib n={}",
                    f.n
                );
                assert_eq!(f.mul(a, WideGf2x3::ONE), a);
                assert_eq!(f.sqr(a), f.mul(a, a));
                if a != WideGf2x3::ZERO {
                    assert_eq!(f.mul(a, f.inv(a)), WideGf2x3::ONE, "inv n={}", f.n);
                }
                // Frobenius: x^(2^n) = x.
                assert_eq!(f.sqr_k(a, f.n), a, "frobenius n={}", f.n);
            }
            assert_eq!(f.inv(WideGf2x3::ZERO), WideGf2x3::ZERO);
        }
    }

    #[test]
    fn x3_portable_and_native_kernels_agree() {
        let mut rng = Rng(7);
        for irr in [
            ecc2k130_poly(),
            poly(163, &NIST163_LOW_TERMS),
            poly(191, &NIST191_LOW_TERMS),
        ] {
            let p = WideGf2x3::with_kernel(&irr, false);
            let h = WideGf2x3::new(&irr);
            for _ in 0..500 {
                let (a, b) = (rng.fe3(&p), rng.fe3(&p));
                assert_eq!(p.mul(a, b), h.mul(a, b));
            }
        }
    }

    #[test]
    fn pinned_reduction_matches_the_general_fold() {
        let mut rng = Rng(11);
        let pin = WideGf2x3::ecc2k130();
        let gen = WideGf2x3::with_general_reduction(&ecc2k130_poly(), true);
        for _ in 0..20_000 {
            let (a, b) = (rng.fe3(&pin), rng.fe3(&pin));
            assert_eq!(pin.mul(a, b), gen.mul(a, b));
            assert_eq!(pin.sqr(a), gen.sqr(a));
        }
        let ones = pin.mask();
        assert_eq!(pin.mul(ones, ones), gen.mul(ones, ones));
    }

    #[test]
    fn x3_agrees_with_the_u128_type_at_97_and_127() {
        let mut rng = Rng(0xdead_beef_cafe_f00d);
        for irr in [poly(97, &[6, 0]), poly(127, &[1, 0])] {
            let w = WideGf2::new(&irr);
            let t = WideGf2x3::new(&irr);
            for _ in 0..10_000 {
                let a = (rng.next() as u128 | ((rng.next() as u128) << 64)) & w.mask();
                let b = (rng.next() as u128 | ((rng.next() as u128) << 64)) & w.mask();
                let (ta, tb) = (t.from_u128(a), t.from_u128(b));
                assert_eq!(t.to_u128(t.mul(ta, tb)), w.mul(a, b), "n={}", irr.degree);
                assert_eq!(t.to_u128(t.sqr(ta)), w.sqr(a));
            }
            let a = (rng.next() as u128 | ((rng.next() as u128) << 64)) & w.mask() | 1;
            assert_eq!(t.to_u128(t.inv(t.from_u128(a))), w.inv(a));
        }
    }

    #[test]
    fn x3_known_answer_at_131_against_the_slow_path() {
        let irr = ecc2k130_poly();
        let f = WideGf2x3::ecc2k130();
        // Fixed operands (no generator): the slow BigUint path derives the product.
        let a: Fe3 = [0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210, 0x7];
        let b: Fe3 = [0xdead_beef_0bad_f00d, 0x1357_9bdf_0246_8ace, 0x5];
        let want = back3(&lib3(a, 131).mul(&lib3(b, 131), &irr));
        assert_eq!(f.mul(a, b), want);
        // And on random operands, with the library inverse and square too.
        let mut rng = Rng(131);
        for _ in 0..200 {
            let (x, y) = (rng.fe3(&f), rng.fe3(&f));
            assert_eq!(f.mul(x, y), back3(&lib3(x, 131).mul(&lib3(y, 131), &irr)));
        }
    }

    #[test]
    fn x3_batch_inverse_and_half_trace() {
        let mut rng = Rng(5);
        let f = WideGf2x3::ecc2k130();
        let mut xs: Vec<Fe3> = (0..50).map(|_| rng.fe3(&f)).collect();
        xs.push(WideGf2x3::ZERO);
        let orig = xs.clone();
        let mut scratch = Vec::new();
        f.batch_inv(&mut xs, &mut scratch);
        for (x, y) in orig.iter().zip(&xs) {
            if *x == WideGf2x3::ZERO {
                assert_eq!(*y, WideGf2x3::ZERO);
            } else {
                assert_eq!(f.mul(*x, *y), WideGf2x3::ONE);
            }
        }
        let g = WideGf2x3::new(&poly(163, &NIST163_LOW_TERMS));
        let mut solved = 0;
        for _ in 0..40 {
            let c = rng.fe3(&g);
            if let Some(z) = g.solve_quadratic(c) {
                assert_eq!(g.add(g.sqr(z), z), c);
                solved += 1;
            }
        }
        assert!(solved > 5);
    }

    /// `cargo test --release --lib -- wide_gf2m --ignored --nocapture`
    #[test]
    #[ignore]
    fn bench_x3_131_vs_u128_127() {
        use std::hint::black_box;
        use std::time::Instant;
        const N: u32 = 4_000_000;
        let f3 = WideGf2x3::ecc2k130();
        let w = WideGf2::new(&poly(127, &[1, 0]));
        let mut rng = Rng(99);
        let (a3, b3) = (rng.fe3(&f3), rng.fe3(&f3));
        let (a, b) = (
            (rng.next() as u128 | ((rng.next() as u128) << 64)) & w.mask(),
            (rng.next() as u128 | ((rng.next() as u128) << 64)) & w.mask(),
        );
        let per = |t: std::time::Duration| t.as_nanos() as f64 / N as f64;
        let t = Instant::now();
        let mut x = a3;
        for _ in 0..N {
            x = f3.mul(black_box(x), black_box(b3));
        }
        black_box(x);
        let m3 = per(t.elapsed());
        let t = Instant::now();
        let mut x = a3;
        for _ in 0..N {
            x = f3.sqr(black_box(x));
        }
        black_box(x);
        let s3 = per(t.elapsed());
        let t = Instant::now();
        let mut y = a;
        for _ in 0..N {
            y = w.mul(black_box(y), black_box(b));
        }
        black_box(y);
        let m1 = per(t.elapsed());
        let t = Instant::now();
        let mut y = a;
        for _ in 0..N {
            y = w.sqr(black_box(y));
        }
        black_box(y);
        let s1 = per(t.elapsed());
        println!(
            "kernel {}: n=131 x3: mul {m3:.1} ns, sqr {s3:.1} ns | n=127 u128: mul {m1:.1} ns, sqr {s1:.1} ns",
            f3.kernel_name()
        );
    }
}
