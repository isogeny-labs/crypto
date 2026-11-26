//! `GF(2)[x]` bit-packed in `u64` words — bit `i` of word `w` is the
//! coefficient of `x^(64w+i)`, no trailing zero words (zero is empty) —
//! with GCD, extended GCD and inverse by [`super::hgcd`] (half-GCD above
//! [`HGCD_THRESHOLD_BITS`], Euclid below) and carry-less multiplication
//! (PCLMULQDQ when the CPU has it, Karatsuba on words above
//! [`KARATSUBA_WORDS`]).

use super::hgcd::{self, PolyRing};

/// Degree at and above which [`xgcd`] and [`inv_mod`] use half-GCD.
/// Measured with `src/bin/hgcd_bench.rs` (leaf 256): ≈1.0× at 512, 1.2× at
/// 1024, 1.5–1.8× at 1536–3072, 2.2–2.5× at 6144, 5× at 32768.
pub const HGCD_THRESHOLD_BITS: usize = 1024;
/// Degree from which plain [`gcd`] uses half-GCD: ≈0.96× at 3072, 1.1× at
/// 6144, 1.6× at 16384, 2.2× at 32768.
pub const GCD_HGCD_THRESHOLD_BITS: usize = 6144;
/// Recursion leaf degree of the half-GCD (sweep: 128..4096, best ≈256).
pub const HGCD_LEAF_BITS: usize = 256;
/// Word length at and above which multiplication uses Karatsuba.
pub const KARATSUBA_WORDS: usize = 12;

/// Drop trailing zero words.
pub fn trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

/// Degree, `-1` for zero.
pub fn deg(a: &[u64]) -> isize {
    for (i, &w) in a.iter().enumerate().rev() {
        if w != 0 {
            return (64 * i + 63 - w.leading_zeros() as usize) as isize;
        }
    }
    -1
}

/// `a + b` (= `a − b`).
pub fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = long.to_vec();
    for (o, &s) in out.iter_mut().zip(short) {
        *o ^= s;
    }
    trim(&mut out);
    out
}

/// Carry-less 64×64 → 128 product `(lo, hi)`.
#[inline]
fn clmul(a: u64, b: u64) -> (u64, u64) {
    #[cfg(target_arch = "x86_64")]
    {
        if has_pclmul() {
            // SAFETY: has_pclmul() checked the CPU feature.
            return unsafe { clmul_hw(a, b) };
        }
    }
    clmul_soft(a, b)
}

#[cfg(target_arch = "x86_64")]
fn has_pclmul() -> bool {
    use std::sync::OnceLock;
    static HAS: OnceLock<bool> = OnceLock::new();
    *HAS.get_or_init(|| {
        std::is_x86_feature_detected!("pclmulqdq") && std::is_x86_feature_detected!("sse2")
    })
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq,sse2")]
unsafe fn clmul_hw(a: u64, b: u64) -> (u64, u64) {
    use core::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let r = _mm_clmulepi64_si128(x, y, 0x00);
    let lo = _mm_cvtsi128_si64(r) as u64;
    let hi = _mm_cvtsi128_si64(_mm_srli_si128(r, 8)) as u64;
    (lo, hi)
}

/// Portable carry-less product (4-bit windows).
fn clmul_soft(a: u64, b: u64) -> (u64, u64) {
    let mut table = [0u128; 16];
    for i in 1..16usize {
        let mut v = 0u128;
        for bit in 0..4 {
            if i >> bit & 1 == 1 {
                v ^= (b as u128) << bit;
            }
        }
        table[i] = v;
    }
    let mut acc = 0u128;
    for nib in (0..16).rev() {
        acc <<= 4;
        acc ^= table[((a >> (4 * nib)) & 15) as usize];
    }
    (acc as u64, (acc >> 64) as u64)
}

fn mul_school(a: &[u64], b: &[u64], out: &mut [u64]) {
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            let (lo, hi) = clmul(x, y);
            out[i + j] ^= lo;
            out[i + j + 1] ^= hi;
        }
    }
}

/// `out ^= a·b`; `out.len() ≥ a.len() + b.len()`.
fn mul_into(a: &[u64], b: &[u64], out: &mut [u64]) {
    if a.is_empty() || b.is_empty() {
        return;
    }
    if a.len() < KARATSUBA_WORDS || b.len() < KARATSUBA_WORDS {
        mul_school(a, b, out);
        return;
    }
    if a.len() > 2 * b.len() || b.len() > 2 * a.len() {
        let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        for (k, chunk) in long.chunks(short.len()).enumerate() {
            mul_into(chunk, short, &mut out[k * short.len()..]);
        }
        return;
    }
    let h = a.len().max(b.len()) / 2;
    let (a0, a1) = a.split_at(h.min(a.len()));
    let (b0, b1) = b.split_at(h.min(b.len()));
    let mut z0 = vec![0u64; a0.len() + b0.len()];
    mul_into(a0, b0, &mut z0);
    let mut z2 = vec![0u64; a1.len() + b1.len()];
    mul_into(a1, b1, &mut z2);
    let sa = xor_raw(a0, a1);
    let sb = xor_raw(b0, b1);
    let mut z1 = vec![0u64; sa.len() + sb.len()];
    mul_into(&sa, &sb, &mut z1);
    for (i, &v) in z0.iter().enumerate() {
        z1[i] ^= v;
        out[i] ^= v;
    }
    for (i, &v) in z2.iter().enumerate() {
        if i < z1.len() {
            z1[i] ^= v;
        }
        if 2 * h + i < out.len() {
            out[2 * h + i] ^= v;
        }
    }
    for (i, &v) in z1.iter().enumerate() {
        if h + i < out.len() {
            out[h + i] ^= v;
        }
    }
}

fn xor_raw(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = long.to_vec();
    for (o, &s) in out.iter_mut().zip(short) {
        *o ^= s;
    }
    out
}

/// `a·b`.
pub fn mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; a.len() + b.len()];
    mul_into(a, b, &mut out);
    trim(&mut out);
    out
}

/// `r ^= b·x^k`, growing `r` as needed.
fn xor_shifted(r: &mut Vec<u64>, b: &[u64], k: usize) {
    let (ws, bs) = (k / 64, k % 64);
    let need = ws + b.len() + 1;
    if r.len() < need {
        r.resize(need, 0);
    }
    if bs == 0 {
        for (i, &w) in b.iter().enumerate() {
            r[ws + i] ^= w;
        }
    } else {
        for (i, &w) in b.iter().enumerate() {
            r[ws + i] ^= w << bs;
            r[ws + i + 1] ^= w >> (64 - bs);
        }
    }
}

/// `(q, r)` with `a = q·b + r`, `deg r < deg b`; panics if `b = 0`.
pub fn divrem(a: &[u64], b: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let db = deg(b);
    assert!(db >= 0, "GF(2)[x] division by zero");
    let mut r = a.to_vec();
    trim(&mut r);
    let mut dr = deg(&r);
    if dr < db {
        return (Vec::new(), r);
    }
    let mut q = vec![0u64; ((dr - db) as usize) / 64 + 1];
    let b = {
        let mut v = b.to_vec();
        trim(&mut v);
        v
    };
    while dr >= db {
        let k = (dr - db) as usize;
        q[k / 64] |= 1u64 << (k % 64);
        xor_shifted(&mut r, &b, k);
        trim(&mut r);
        dr = deg(&r);
    }
    trim(&mut q);
    (q, r)
}

/// `⌊a / x^k⌋`.
pub fn shr(a: &[u64], k: usize) -> Vec<u64> {
    let (ws, bs) = (k / 64, k % 64);
    if ws >= a.len() {
        return Vec::new();
    }
    let src = &a[ws..];
    let mut out = Vec::with_capacity(src.len());
    if bs == 0 {
        out.extend_from_slice(src);
    } else {
        for i in 0..src.len() {
            let hi = if i + 1 < src.len() {
                src[i + 1] << (64 - bs)
            } else {
                0
            };
            out.push((src[i] >> bs) | hi);
        }
    }
    trim(&mut out);
    out
}

/// `GF(2)[x]` as a [`PolyRing`]: xgcd/inverse and plain-gcd half-GCD
/// thresholds and the recursion leaf degree.
#[derive(Clone, Copy, Debug)]
pub struct Gf2Polys {
    pub threshold: usize,
    pub gcd_threshold: usize,
    pub leaf: usize,
}

impl Default for Gf2Polys {
    fn default() -> Self {
        Gf2Polys {
            threshold: HGCD_THRESHOLD_BITS,
            gcd_threshold: GCD_HGCD_THRESHOLD_BITS,
            leaf: HGCD_LEAF_BITS,
        }
    }
}

impl PolyRing for Gf2Polys {
    type P = Vec<u64>;
    fn zero(&self) -> Self::P {
        Vec::new()
    }
    fn one(&self) -> Self::P {
        vec![1]
    }
    fn deg(&self, a: &Self::P) -> isize {
        deg(a)
    }
    fn add(&self, a: &Self::P, b: &Self::P) -> Self::P {
        add(a, b)
    }
    fn sub(&self, a: &Self::P, b: &Self::P) -> Self::P {
        add(a, b)
    }
    fn mul(&self, a: &Self::P, b: &Self::P) -> Self::P {
        mul(a, b)
    }
    fn divrem(&self, a: &Self::P, b: &Self::P) -> (Self::P, Self::P) {
        divrem(a, b)
    }
    fn shr(&self, a: &Self::P, k: usize) -> Self::P {
        shr(a, k)
    }
    fn make_monic(&self, _g: &mut Self::P, _others: &mut [&mut Self::P]) {}
    fn hgcd_threshold(&self) -> usize {
        self.threshold
    }
    fn gcd_threshold(&self) -> usize {
        self.gcd_threshold
    }
    fn hgcd_leaf(&self) -> usize {
        self.leaf
    }
}

fn norm(a: &[u64]) -> Vec<u64> {
    let mut v = a.to_vec();
    trim(&mut v);
    v
}

/// `gcd(a, b)` (empty when both are zero).
pub fn gcd(a: &[u64], b: &[u64]) -> Vec<u64> {
    hgcd::gcd(&Gf2Polys::default(), &norm(a), &norm(b))
}

/// `(g, s, t)` with `g = s·a + t·b`, the classical minimal-degree cofactors.
pub fn xgcd(a: &[u64], b: &[u64]) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    hgcd::xgcd(&Gf2Polys::default(), &norm(a), &norm(b))
}

/// `a⁻¹ mod m` (`deg < deg m`), `None` unless `gcd(a, m) = 1`.
pub fn inv_mod(a: &[u64], m: &[u64]) -> Option<Vec<u64>> {
    hgcd::inv_mod(&Gf2Polys::default(), &norm(a), &norm(m))
}

/// [`gcd`] with an explicit half-GCD degree threshold; for benchmarks.
pub fn gcd_with_threshold(a: &[u64], b: &[u64], threshold: usize) -> Vec<u64> {
    gcd_with_params(a, b, threshold, HGCD_LEAF_BITS)
}

/// [`gcd`] with explicit threshold and recursion leaf degree; for tuning.
pub fn gcd_with_params(a: &[u64], b: &[u64], threshold: usize, leaf: usize) -> Vec<u64> {
    let r = Gf2Polys {
        threshold,
        gcd_threshold: threshold,
        leaf,
    };
    hgcd::gcd(&r, &norm(a), &norm(b))
}

/// [`xgcd`] with an explicit half-GCD degree threshold; for benchmarks.
pub fn xgcd_with_threshold(
    a: &[u64],
    b: &[u64],
    threshold: usize,
) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    xgcd_with_params(a, b, threshold, HGCD_LEAF_BITS)
}

/// [`xgcd`] with explicit threshold and recursion leaf degree; for tuning.
pub fn xgcd_with_params(
    a: &[u64],
    b: &[u64],
    threshold: usize,
    leaf: usize,
) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let r = Gf2Polys {
        threshold,
        gcd_threshold: threshold,
        leaf,
    };
    hgcd::xgcd(&r, &norm(a), &norm(b))
}

/// Classical Euclid only (test and benchmark reference).
pub fn gcd_euclid(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (mut x, mut y) = (norm(a), norm(b));
    while !y.is_empty() {
        let (_, r) = divrem(&x, &y);
        x = std::mem::replace(&mut y, r);
    }
    x
}

/// `gcd` of single-word polynomials (degree ≤ 63) by shifts.
pub fn gcd_word(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let (da, db) = (63 - a.leading_zeros() as i32, 63 - b.leading_zeros() as i32);
        if a == 0 || da < db {
            std::mem::swap(&mut a, &mut b);
            continue;
        }
        a ^= b << (da - db);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn rand_poly(rng: &mut StdRng, deg: usize) -> Vec<u64> {
        let words = deg / 64 + 1;
        let mut v: Vec<u64> = (0..words).map(|_| rng.gen()).collect();
        let top = deg % 64;
        let last = v.last_mut().unwrap();
        *last &= if top == 63 {
            u64::MAX
        } else {
            (1u64 << (top + 1)) - 1
        };
        *last |= 1u64 << top;
        v
    }

    #[test]
    fn clmul_hw_matches_soft() {
        let mut rng = StdRng::seed_from_u64(9);
        for _ in 0..2000 {
            let (a, b): (u64, u64) = (rng.gen(), rng.gen());
            assert_eq!(clmul(a, b), clmul_soft(a, b));
        }
        assert_eq!(
            clmul_soft(u64::MAX, u64::MAX),
            (0x5555_5555_5555_5555, 0x5555_5555_5555_5555)
        );
    }

    #[test]
    fn karatsuba_matches_schoolbook() {
        let mut rng = StdRng::seed_from_u64(10);
        for (da, db) in [
            (0, 0),
            (63, 64),
            (700, 700),
            (767, 768),
            (2000, 300),
            (3000, 2999),
        ] {
            let a = rand_poly(&mut rng, da);
            let b = rand_poly(&mut rng, db);
            let mut want = vec![0u64; a.len() + b.len()];
            mul_school(&a, &b, &mut want);
            trim(&mut want);
            assert_eq!(mul(&a, &b), want, "{da}x{db}");
        }
    }

    #[test]
    fn half_gcd_matches_euclid() {
        let mut rng = StdRng::seed_from_u64(11);
        for &(da, db, dc) in &[
            (1usize, 0usize, 0usize),
            (100, 99, 5),
            (1535, 1534, 0),
            (1536, 1535, 0),
            (2000, 1999, 77),
            (4100, 4000, 300),
            (6000, 5999, 1),
        ] {
            let c = rand_poly(&mut rng, dc);
            let a = mul(&rand_poly(&mut rng, da), &c);
            let b = mul(&rand_poly(&mut rng, db), &c);
            let want = gcd_euclid(&a, &b);
            assert_eq!(gcd(&a, &b), want, "{da} {db} {dc}");
            assert_eq!(gcd_with_threshold(&a, &b, 2), want, "thr=2 {da} {db} {dc}");
            assert_eq!(xgcd_with_threshold(&a, &b, 2), xgcd(&a, &b), "thr=2 xgcd");
            let (g, s, t) = xgcd(&a, &b);
            assert_eq!(g, want);
            assert_eq!(add(&mul(&s, &a), &mul(&t, &b)), g);
            assert!(deg(&s) < deg(&b) - deg(&g) || s.is_empty());
        }
        // (x³+x+1)(x²+x+1) and (x³+x²+1)(x²+x+1): carry-less products.
        let (u, v) = (mul(&[0b1011], &[0b111])[0], mul(&[0b1101], &[0b111])[0]);
        assert_eq!(gcd_word(u, v), 0b111);
    }

    #[test]
    fn inverse_mod_trinomial() {
        // x^233 + x^74 + 1 (sect233 field): every nonzero element inverts.
        let mut m = vec![0u64; 4];
        m[0] |= 1;
        m[1] |= 1 << (74 - 64);
        m[3] |= 1 << (233 - 192);
        let mut rng = StdRng::seed_from_u64(12);
        for _ in 0..20 {
            let d = rng.gen_range(0..233);
            let a = rand_poly(&mut rng, d);
            let inv = inv_mod(&a, &m).unwrap();
            assert_eq!(divrem(&mul(&a, &inv), &m).1, vec![1]);
        }
    }
}
