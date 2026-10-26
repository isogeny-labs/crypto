//! `GF(2^n)` for `n ≤ 191`, one element per three `u64` words.
//!
//! [`WideGf2`](crate::cryptanalysis::wide_gf2m::WideGf2) stops at `n = 127`
//! (one `u128`); ECC2-131 needs one more word.  This field covers ECC2-131
//! and every calibration degree `61 ≤ n ≤ 127` with the same code, so a
//! calibration row and the full-size row are measured by one instrument.
//!
//! Polynomial basis, bit `i` the coefficient of `z^i`.  A product is the
//! schoolbook 3 × 3 carry-less product (nine 64-bit carry-less multiplies:
//! `PMULL` on AArch64, `PCLMULQDQ` on x86-64, bit-serial elsewhere), folded
//! back through the modulus's low terms.  The trace and the half-trace are
//! `F_2`-linear, so both are precomputed: the trace as one mask, the
//! half-trace as byte-indexed tables.

use crate::binary_ecc::IrreduciblePoly;

/// A field element: bit `i` of the 192-bit little-endian word triple is
/// the coefficient of `z^i`.
pub type W3 = [u64; 3];

pub const ZERO: W3 = [0; 3];
pub const ONE: W3 = [1, 0, 0];

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

#[inline(always)]
pub fn xor(a: W3, b: W3) -> W3 {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]]
}

#[inline(always)]
pub fn is_zero(a: &W3) -> bool {
    (a[0] | a[1] | a[2]) == 0
}

/// `deg a < l` (the zero polynomial included).
#[inline(always)]
pub fn degree_below(a: &W3, l: u32) -> bool {
    match l {
        0 => is_zero(a),
        1..=63 => (a[0] >> l) == 0 && a[1] == 0 && a[2] == 0,
        64 => a[1] == 0 && a[2] == 0,
        65..=127 => (a[1] >> (l - 64)) == 0 && a[2] == 0,
        128 => a[2] == 0,
        129..=191 => (a[2] >> (l - 128)) == 0,
        _ => true,
    }
}

/// The polynomial whose coefficient vector is the low bits of `v`.
pub fn from_u64(v: u64) -> W3 {
    [v, 0, 0]
}

pub fn from_biguint(v: &num_bigint::BigUint) -> W3 {
    let d = v.to_u64_digits();
    let mut out = ZERO;
    for (i, w) in d.iter().take(3).enumerate() {
        out[i] = *w;
    }
    out
}

pub fn to_biguint(v: &W3) -> num_bigint::BigUint {
    num_bigint::BigUint::from_slice(&[
        v[0] as u32,
        (v[0] >> 32) as u32,
        v[1] as u32,
        (v[1] >> 32) as u32,
        v[2] as u32,
        (v[2] >> 32) as u32,
    ])
}

/// `GF(2)[z] / (f)` with `deg f = n ≤ 191`.
#[derive(Clone, Debug)]
pub struct Gf2W {
    pub n: u32,
    low_terms: Vec<u32>,
    mask: W3,
    has_clmul: bool,
    /// `Tr(x) = parity(x & trace_mask)`.
    trace_mask: W3,
    /// `half_trace[c][byte]`: the half-trace of the byte placed at bits
    /// `8c … 8c+7` (empty for even `n`).
    half_trace: Vec<[W3; 256]>,
}

impl Gf2W {
    pub fn new(irr: &IrreduciblePoly) -> Self {
        #[cfg(target_arch = "x86_64")]
        let has_clmul = std::arch::is_x86_feature_detected!("pclmulqdq");
        #[cfg(target_arch = "aarch64")]
        let has_clmul = std::arch::is_aarch64_feature_detected!("aes");
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let has_clmul = false;
        Self::with_kernel(irr, has_clmul)
    }

    pub fn with_kernel(irr: &IrreduciblePoly, has_clmul: bool) -> Self {
        let n = irr.degree;
        assert!((2..=191).contains(&n), "Gf2W handles 2 ≤ n ≤ 191, not {n}");
        let mut low_terms = irr.low_terms.clone();
        low_terms.sort_unstable();
        low_terms.dedup();
        assert!(
            low_terms.iter().all(|&t| t < n),
            "a low term at or above z^n"
        );
        // Reduction folds the part above z^n down by at most the largest
        // low term each round; it terminates for any modulus, but is only
        // fast when the low terms are small, as for all standard moduli.
        let mut mask = ZERO;
        for i in 0..n {
            mask[(i / 64) as usize] |= 1u64 << (i % 64);
        }
        let mut f = Self {
            n,
            low_terms,
            mask,
            has_clmul,
            trace_mask: ZERO,
            half_trace: Vec::new(),
        };
        // Trace mask: Tr(z^i) for every i, from the definition.
        let mut tm = ZERO;
        for i in 0..n {
            let zi = f.monomial(i);
            if !is_zero(&f.trace_slow(zi)) {
                tm[(i / 64) as usize] |= 1u64 << (i % 64);
            }
        }
        f.trace_mask = tm;
        if n % 2 == 1 {
            let chunks = n.div_ceil(8) as usize;
            let basis: Vec<W3> = (0..n).map(|i| f.half_trace_slow(f.monomial(i))).collect();
            let mut tables = vec![[ZERO; 256]; chunks];
            for (c, table) in tables.iter_mut().enumerate() {
                for byte in 1..256usize {
                    let low = byte.trailing_zeros() as usize;
                    let bit = 8 * c + low;
                    let prev = table[byte & (byte - 1)];
                    table[byte] = if bit < n as usize {
                        xor(prev, basis[bit])
                    } else {
                        prev
                    };
                }
            }
            f.half_trace = tables;
        }
        f
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

    pub fn monomial(&self, i: u32) -> W3 {
        let mut v = ZERO;
        v[(i / 64) as usize] = 1u64 << (i % 64);
        v
    }

    pub fn mask(&self) -> W3 {
        self.mask
    }

    /// Whether `a` is a canonical element (`deg a < n`).
    pub fn is_canonical(&self, a: &W3) -> bool {
        degree_below(a, self.n)
    }

    #[inline(always)]
    fn clmul64(&self, a: u64, b: u64) -> u128 {
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        if self.has_clmul {
            // SAFETY: `has_clmul` is set only when the CPU reports the
            // instruction, or by a caller bound to the same rule.
            return unsafe { clmul_hw(a, b) };
        }
        clmul_portable(a, b)
    }

    /// The unreduced 384-bit product.
    #[inline(always)]
    pub fn mul_unreduced(&self, a: &W3, b: &W3) -> [u64; 6] {
        let mut r = [0u64; 6];
        for i in 0..3 {
            if a[i] == 0 {
                continue;
            }
            for j in 0..3 {
                if b[j] == 0 {
                    continue;
                }
                let p = self.clmul64(a[i], b[j]);
                r[i + j] ^= p as u64;
                r[i + j + 1] ^= (p >> 64) as u64;
            }
        }
        r
    }

    /// Reduce a value of at most 384 bits modulo `f`.
    #[inline(always)]
    pub fn reduce(&self, mut r: [u64; 6]) -> W3 {
        let n = self.n;
        loop {
            let top = shr6(&r, n);
            if top.iter().all(|&w| w == 0) {
                return [r[0], r[1], r[2]];
            }
            for (w, m) in r.iter_mut().zip(self.mask.iter().chain([0u64; 3].iter())) {
                *w &= m;
            }
            for &t in &self.low_terms {
                let s = shl6(&top, t);
                for k in 0..6 {
                    r[k] ^= s[k];
                }
            }
        }
    }

    #[inline(always)]
    pub fn mul(&self, a: &W3, b: &W3) -> W3 {
        self.reduce(self.mul_unreduced(a, b))
    }

    #[inline(always)]
    pub fn sqr(&self, a: &W3) -> W3 {
        self.mul(a, a)
    }

    pub fn sqr_k(&self, a: &W3, k: u32) -> W3 {
        let mut v = *a;
        for _ in 0..k {
            v = self.sqr(&v);
        }
        v
    }

    /// `a^{-1}` by Itoh–Tsujii; `0 ↦ 0`.
    pub fn inv(&self, a: &W3) -> W3 {
        if is_zero(a) {
            return ZERO;
        }
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let mut c = *a;
        let mut k = 1u32;
        for i in (0..bits - 1).rev() {
            c = self.mul(&c, &self.sqr_k(&c, k));
            k *= 2;
            if (e >> i) & 1 == 1 {
                c = self.mul(&self.sqr(&c), a);
                k += 1;
            }
        }
        debug_assert_eq!(k, e);
        self.sqr(&c)
    }

    /// Invert every nonzero entry in place with one inversion
    /// (Montgomery's trick); zeros stay zero.
    pub fn batch_inv(&self, xs: &mut [W3], scratch: &mut Vec<W3>) {
        scratch.clear();
        let mut acc = ONE;
        for x in xs.iter() {
            scratch.push(acc);
            if !is_zero(x) {
                acc = self.mul(&acc, x);
            }
        }
        let mut inv = self.inv(&acc);
        for i in (0..xs.len()).rev() {
            let x = xs[i];
            if is_zero(&x) {
                continue;
            }
            xs[i] = self.mul(&inv, &scratch[i]);
            inv = self.mul(&inv, &x);
        }
    }

    fn trace_slow(&self, a: W3) -> W3 {
        let mut t = a;
        let mut acc = a;
        for _ in 1..self.n {
            t = self.sqr(&t);
            acc = xor(acc, t);
        }
        acc
    }

    fn half_trace_slow(&self, c: W3) -> W3 {
        let mut t = c;
        let mut acc = c;
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(&self.sqr(&t));
            acc = xor(acc, t);
        }
        acc
    }

    /// The absolute trace, `0` or `1`.
    #[inline(always)]
    pub fn trace(&self, a: &W3) -> u32 {
        ((a[0] & self.trace_mask[0]).count_ones()
            ^ (a[1] & self.trace_mask[1]).count_ones()
            ^ (a[2] & self.trace_mask[2]).count_ones())
            & 1
    }

    /// The half-trace `H(c) = Σ_{i ≤ (n−1)/2} c^{4^i}` (odd `n`), an
    /// `F_2`-linear map with `H(c)² + H(c) = c + Tr(c)`.
    #[inline(always)]
    pub fn half_trace(&self, c: &W3) -> W3 {
        let mut acc = ZERO;
        for (k, table) in self.half_trace.iter().enumerate() {
            let byte = ((c[k / 8] >> (8 * (k % 8))) & 0xff) as usize;
            if byte != 0 {
                acc = xor(acc, table[byte]);
            }
        }
        acc
    }

    /// A root of `z² + z = c` (odd `n`, `Tr(c) = 0`); `None` otherwise.
    #[inline(always)]
    pub fn solve_quadratic(&self, c: &W3) -> Option<W3> {
        if self.half_trace.is_empty() || self.trace(c) != 0 {
            return None;
        }
        let z = self.half_trace(c);
        debug_assert_eq!(xor(self.sqr(&z), z), *c);
        Some(z)
    }

    /// The square root (`a^{2^{n−1}}`).
    pub fn sqrt(&self, a: &W3) -> W3 {
        self.sqr_k(a, self.n - 1)
    }

    /// A uniform element.
    pub fn random<R: rand::Rng>(&self, rng: &mut R) -> W3 {
        let mut v = [rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>()];
        for k in 0..3 {
            v[k] &= self.mask[k];
        }
        v
    }
}

/// `r >> s` for a 384-bit value.
#[inline(always)]
fn shr6(r: &[u64; 6], s: u32) -> [u64; 6] {
    let words = (s / 64) as usize;
    let bits = s % 64;
    let mut out = [0u64; 6];
    for i in 0..6 - words {
        let lo = r[i + words] >> bits;
        let hi = if bits != 0 && i + words + 1 < 6 {
            r[i + words + 1] << (64 - bits)
        } else {
            0
        };
        out[i] = lo | hi;
    }
    out
}

/// `r << s` for a 384-bit value (bits past 384 dropped).
#[inline(always)]
fn shl6(r: &[u64; 6], s: u32) -> [u64; 6] {
    let words = (s / 64) as usize;
    let bits = s % 64;
    let mut out = [0u64; 6];
    for i in (words..6).rev() {
        let lo = r[i - words] << bits;
        let hi = if bits != 0 && i > words {
            r[i - words - 1] >> (64 - bits)
        } else {
            0
        };
        out[i] = lo | hi;
    }
    out
}

/// The standard low-weight irreducible polynomials used by this module:
/// ECC2-131's pentanomial and, for the calibration degrees, the
/// lexicographically first irreducible trinomial `z^n + z^k + 1` or, when
/// none exists, pentanomial `z^n + z^{k3} + z^{k2} + z^{k1} + 1`, found
/// by the irreducibility test below.
pub fn low_weight_irreducible(n: u32) -> IrreduciblePoly {
    if n == 131 {
        return IrreduciblePoly {
            degree: 131,
            low_terms: vec![0, 1, 2, 13],
        };
    }
    for k in 1..n / 2 + 1 {
        let cand = IrreduciblePoly {
            degree: n,
            low_terms: vec![0, k],
        };
        if is_irreducible(&cand) {
            return cand;
        }
    }
    for k3 in 3..n {
        for k2 in 2..k3 {
            for k1 in 1..k2 {
                let cand = IrreduciblePoly {
                    degree: n,
                    low_terms: vec![0, k1, k2, k3],
                };
                if is_irreducible(&cand) {
                    return cand;
                }
            }
        }
    }
    panic!("no low-weight irreducible polynomial of degree {n}");
}

/// Ben-Or / Rabin test: `f` of degree `n` is irreducible iff
/// `z^{2^n} ≡ z (mod f)` and `gcd(z^{2^{n/q}} − z, f) = 1` for every prime
/// `q | n`.  Uses the field's own (possibly reducible) quotient ring, which
/// is all the test needs.
pub fn is_irreducible(irr: &IrreduciblePoly) -> bool {
    let n = irr.degree;
    if !irr.low_terms.contains(&0) {
        return false;
    }
    let ring = Gf2W::ring_only(irr);
    let z = ring.monomial(1);
    if ring.sqr_k(&z, n) != z {
        return false;
    }
    let mut m = n;
    let mut q = 2;
    let mut primes = Vec::new();
    while q * q <= m {
        if m.is_multiple_of(q) {
            primes.push(q);
            while m.is_multiple_of(q) {
                m /= q;
            }
        }
        q += 1;
    }
    if m > 1 {
        primes.push(m);
    }
    let f = poly_of(irr);
    for q in primes {
        let h = xor(ring.sqr_k(&z, n / q), z);
        if !poly_gcd_is_one(&f, &h) {
            return false;
        }
    }
    true
}

impl Gf2W {
    /// The quotient ring only (no trace tables), for the irreducibility
    /// test, where the trace is undefined.
    fn ring_only(irr: &IrreduciblePoly) -> Self {
        let n = irr.degree;
        let mut low_terms = irr.low_terms.clone();
        low_terms.sort_unstable();
        low_terms.dedup();
        let mut mask = ZERO;
        for i in 0..n {
            mask[(i / 64) as usize] |= 1u64 << (i % 64);
        }
        Self {
            n,
            low_terms,
            mask,
            has_clmul: false,
            trace_mask: ZERO,
            half_trace: Vec::new(),
        }
    }
}

/// The full modulus as a 4-word polynomial (degree ≤ 191 fits in 3 words
/// plus the leading term).
fn poly_of(irr: &IrreduciblePoly) -> Vec<u64> {
    let mut v = vec![0u64; 4];
    v[(irr.degree / 64) as usize] |= 1u64 << (irr.degree % 64);
    for &t in &irr.low_terms {
        v[(t / 64) as usize] |= 1u64 << (t % 64);
    }
    v
}

fn poly_deg(v: &[u64]) -> i64 {
    for i in (0..v.len()).rev() {
        if v[i] != 0 {
            return 64 * i as i64 + 63 - v[i].leading_zeros() as i64;
        }
    }
    -1
}

fn poly_gcd_is_one(f: &[u64], h: &W3) -> bool {
    let mut a: Vec<u64> = f.to_vec();
    let mut b: Vec<u64> = vec![h[0], h[1], h[2], 0];
    loop {
        let db = poly_deg(&b);
        if db < 0 {
            return poly_deg(&a) == 0;
        }
        // a mod b
        loop {
            let da = poly_deg(&a);
            if da < db {
                break;
            }
            let s = (da - db) as u32;
            let words = (s / 64) as usize;
            let bits = s % 64;
            for i in (0..a.len()).rev() {
                if i < words {
                    break;
                }
                let mut v = b[i - words] << bits;
                if bits != 0 && i > words {
                    v |= b[i - words - 1] >> (64 - bits);
                }
                a[i] ^= v;
            }
        }
        std::mem::swap(&mut a, &mut b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use rand::{rngs::StdRng, SeedableRng};

    fn lib(v: &W3, irr: &IrreduciblePoly) -> F2mElement {
        F2mElement::from_biguint(&to_biguint(v), irr.degree)
    }

    #[test]
    fn mul_and_inverse_match_the_library_field_at_131() {
        let irr = low_weight_irreducible(131);
        let f = Gf2W::new(&irr);
        let mut rng = StdRng::seed_from_u64(131);
        for _ in 0..200 {
            let a = f.random(&mut rng);
            let b = f.random(&mut rng);
            let got = f.mul(&a, &b);
            let want = lib(&a, &irr).mul(&lib(&b, &irr), &irr);
            assert_eq!(to_biguint(&got), want.to_biguint());
            if !is_zero(&a) {
                assert_eq!(f.mul(&a, &f.inv(&a)), ONE);
            }
        }
    }

    #[test]
    fn both_kernels_agree_and_the_trace_tables_are_exact() {
        for n in [61u32, 67, 79, 83, 127, 131] {
            let irr = low_weight_irreducible(n);
            let hw = Gf2W::new(&irr);
            let sw = Gf2W::with_kernel(&irr, false);
            let mut rng = StdRng::seed_from_u64(n as u64);
            for _ in 0..64 {
                let a = hw.random(&mut rng);
                let b = hw.random(&mut rng);
                assert_eq!(hw.mul(&a, &b), sw.mul(&a, &b));
                assert_eq!(hw.trace(&a) as u64, hw.trace_slow(a)[0]);
                let h = hw.half_trace(&a);
                let lhs = xor(hw.sqr(&h), h);
                let mut rhs = a;
                rhs[0] ^= hw.trace(&a) as u64;
                assert_eq!(lhs, rhs, "H(c)^2 + H(c) = c + Tr(c) at n={n}");
            }
        }
    }

    #[test]
    fn calibration_moduli_are_irreducible_and_reducible_ones_are_rejected() {
        for n in [61u32, 67, 73, 79, 89, 97, 107, 113, 127, 131] {
            assert!(is_irreducible(&low_weight_irreducible(n)), "n={n}");
        }
        // z^4 + z^2 + 1 = (z^2 + z + 1)^2.
        assert!(!is_irreducible(&IrreduciblePoly {
            degree: 4,
            low_terms: vec![0, 2]
        }));
    }

    #[test]
    fn batch_inverse_matches_single_inverses() {
        let f = Gf2W::new(&low_weight_irreducible(131));
        let mut rng = StdRng::seed_from_u64(7);
        let mut xs: Vec<W3> = (0..33).map(|_| f.random(&mut rng)).collect();
        xs[5] = ZERO;
        let want: Vec<W3> = xs.iter().map(|x| f.inv(x)).collect();
        let mut scratch = Vec::new();
        f.batch_inv(&mut xs, &mut scratch);
        assert_eq!(xs, want);
    }
}
