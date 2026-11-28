//! AVX-512 IFMA (`vpmadd52luq` / `vpmadd52huq`) Montgomery multiplication on
//! 52-bit limbs, two shapes (2026-11-28 study, `docs/notes/adx6-20261128/`):
//!
//! * [`mont_mul_ifma1`]: one product on one zmm (`L ≤ 8` limbs of 52 bits:
//!   5 for 256-bit, 8 for 384-bit moduli).  Rows add `a·bᵢ` and `m·p` with
//!   the lo/hi multiply-adds, shift the lanes down one (`valignq`) and move
//!   lane 0's carry up.  The row factor `m` needs a vector→scalar→vector
//!   round trip (extract, `imul`, broadcast), which is on every row's
//!   critical path.  `a` is packed pre-shifted by `52L − 64N` bits, so the
//!   `2^(52L)` division gives exactly `a·b·2^(-64N)`: a drop-in for
//!   [`Mont::mul`](super::mont_vt::Mont).
//! * [`mont_mul_ifma8`]: eight independent products, one per lane,
//!   limb-sliced (`a[k]` holds limb k of eight operands).  `m` is one more
//!   `vpmadd52luq` (it uses only the low 52 bits of t₀), there are no
//!   cross-lane operations, and values stay lazily in `[0, 2p)` because
//!   `R' = 2^(52L) > 4p`, so there is no per-product subtraction.  Its own
//!   Montgomery domain (`R'`), for batched callers.
//!
//! Only dispatched after [`has_ifma`].
#![cfg(target_arch = "x86_64")]

use super::mont_vt::{has_adx, Mont, MulKernel, ProdAdxKernel};
use core::arch::x86_64::*;

/// Whether this CPU has AVX-512F and AVX-512 IFMA, detected once.
pub(crate) fn has_ifma() -> bool {
    use std::sync::OnceLock;
    static HAS: OnceLock<bool> = OnceLock::new();
    *HAS.get_or_init(|| {
        std::is_x86_feature_detected!("avx512f") && std::is_x86_feature_detected!("avx512ifma")
    })
}

fn geq<const N: usize>(a: &[u64; N], b: &[u64; N]) -> bool {
    for i in (0..N).rev() {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

fn sub_in_place<const N: usize>(a: &mut [u64; N], b: &[u64; N]) {
    let mut borrow = false;
    for i in 0..N {
        let (d, b1) = a[i].overflowing_sub(b[i]);
        let (d, b2) = d.overflowing_sub(borrow as u64);
        a[i] = d;
        borrow = b1 | b2;
    }
}

pub(crate) const MASK52: u64 = (1u64 << 52) - 1;

/// Pack `x` (64-bit words, little endian) shifted left by `sh` bits into
/// `L` limbs of 52 bits.
#[inline(always)]
pub(crate) fn pack52<const L: usize>(x: &[u64], sh: u32) -> [u64; 8] {
    let mut out = [0u64; 8];
    for k in 0..L {
        let bit = 52 * k as i64 - sh as i64;
        let mut v: u64 = 0;
        // bits [bit, bit+52) of x
        if bit < 0 {
            let s = (-bit) as u32;
            v = x[0] << s;
        } else {
            let w = (bit / 64) as usize;
            let o = (bit % 64) as u32;
            if w < x.len() {
                v = x[w] >> o;
            }
            if o > 12 && w + 1 < x.len() {
                v |= x[w + 1] << (64 - o);
            }
        }
        out[k] = v & MASK52;
    }
    out
}

/// Unpack 52-bit limbs (normalized) into `N` 64-bit words; returns the
/// words and whatever is left above `64N` bits.
#[inline(always)]
pub(crate) fn unpack52<const N: usize>(l: &[u64; 8], nl: usize) -> ([u64; N], u64) {
    let mut out = [0u64; N];
    let mut over = 0u64;
    for k in 0..nl {
        let bit = 52 * k;
        let (w, o) = (bit / 64, (bit % 64) as u32);
        if w < N {
            out[w] |= l[k] << o;
            if o > 12 {
                if w + 1 < N {
                    out[w + 1] |= l[k] >> (64 - o);
                } else {
                    over |= l[k] >> (64 - o);
                }
            }
        } else {
            over |= l[k] << (bit - 64 * N);
        }
    }
    (out, over)
}

/// Single-product Montgomery multiplication on one zmm (`L ≤ 8` limbs of 52
/// bits): returns `a52 · b52 · 2^(-52L) mod p` as unnormalized-free 52-bit
/// limbs, `< 2p` when `a52·b52 < 2^(52L)·p`.
#[target_feature(enable = "avx512f,avx512ifma")]
pub(crate) unsafe fn amm1<const L: usize>(
    a52: &[u64; 8],
    b52: &[u64; 8],
    p52: __m512i,
    n0: u64,
) -> [u64; 8] {
    let a = _mm512_loadu_si512(a52.as_ptr() as *const _);
    let z = _mm512_setzero_si512();
    let mut t = z;
    for i in 0..L {
        let bi = _mm512_set1_epi64(b52[i] as i64);
        t = _mm512_madd52lo_epu64(t, a, bi);
        let t0 = _mm_cvtsi128_si64(_mm512_castsi512_si128(t)) as u64;
        let m = t0.wrapping_mul(n0) & MASK52;
        let mm = _mm512_set1_epi64(m as i64);
        t = _mm512_madd52lo_epu64(t, p52, mm);
        let c = _mm512_maskz_srli_epi64(1, t, 52);
        t = _mm512_alignr_epi64(z, t, 1);
        t = _mm512_add_epi64(t, c);
        t = _mm512_madd52hi_epu64(t, a, bi);
        t = _mm512_madd52hi_epu64(t, p52, mm);
    }
    let mut out = [0u64; 8];
    _mm512_storeu_si512(out.as_mut_ptr() as *mut _, t);
    // normalize
    let mut c = 0u64;
    for k in 0..L {
        let v = out[k] + c;
        out[k] = v & MASK52;
        c = v >> 52;
    }
    debug_assert_eq!(c, 0);
    out
}

/// Drop-in `a·b·2^(-64N) mod p` in `[0, p)` for `N`-word inputs `< p`, by
/// [`amm1`] with `a` packed pre-shifted by `52L − 64N` bits.
#[target_feature(enable = "avx512f,avx512ifma")]
pub(crate) unsafe fn mont_mul_ifma1<const N: usize, const L: usize>(
    a: &[u64; N],
    b: &[u64; N],
    p: &[u64; N],
    p52: __m512i,
    n0: u64,
) -> [u64; N] {
    let sh = (52 * L - 64 * N) as u32;
    let a52 = pack52::<L>(a, sh);
    let b52 = pack52::<L>(b, 0);
    let r = amm1::<L>(&a52, &b52, p52, n0);
    let (mut t, over) = unpack52::<N>(&r, L);
    if over != 0 || geq(&t, p) {
        sub_in_place(&mut t, p);
    }
    t
}

/// Eight independent Montgomery products, one per lane, limb-sliced:
/// `a[k]`, `b[k]` hold limb k (52 bits) of eight operands, values `< 2p`,
/// `R' = 2^(52L) > 4p`.  Returns `a·b·R'^-1 mod p` per lane, `< 2p`,
/// normalized to 52-bit limbs.
#[target_feature(enable = "avx512f,avx512ifma")]
pub(crate) unsafe fn mont_mul_ifma8<const L: usize>(
    a: &[__m512i; L],
    b: &[__m512i; L],
    p: &[__m512i; L],
    n0: __m512i,
) -> [__m512i; L] {
    let z = _mm512_setzero_si512();
    let mut t = [z; 9];
    for i in 0..L {
        let bi = b[i];
        for j in 0..L {
            t[j] = _mm512_madd52lo_epu64(t[j], a[j], bi);
            t[j + 1] = _mm512_madd52hi_epu64(t[j + 1], a[j], bi);
        }
        let m = _mm512_madd52lo_epu64(z, t[0], n0);
        for j in 0..L {
            t[j] = _mm512_madd52lo_epu64(t[j], p[j], m);
            t[j + 1] = _mm512_madd52hi_epu64(t[j + 1], p[j], m);
        }
        let c = _mm512_srli_epi64(t[0], 52);
        t[1] = _mm512_add_epi64(t[1], c);
        for j in 0..L {
            t[j] = t[j + 1];
        }
        t[L] = z;
    }
    let mask = _mm512_set1_epi64(MASK52 as i64);
    let mut out = [z; L];
    let mut c = z;
    for j in 0..L {
        let v = _mm512_add_epi64(t[j], c);
        out[j] = _mm512_and_si512(v, mask);
        c = _mm512_srli_epi64(v, 52);
    }
    out
}

/// The IFMA single-product kernel for a [`Mont`] context: `N = 4` (5 limbs)
/// and `N = 6` (8 limbs), else `None`.  Dispatch only after [`has_ifma`].
///
/// # Safety
/// The CPU must support AVX-512F and AVX-512 IFMA.
#[inline(always)]
pub(crate) unsafe fn mul_ifma1_unchecked<const N: usize>(
    ctx: &Mont<N>,
    a: &[u64; N],
    b: &[u64; N],
) -> Option<[u64; N]> {
    let (p52, n0) = ctx.ifma_consts();
    let p52 = _mm512_loadu_si512(p52.as_ptr() as *const _);
    if N == 4 {
        let a4 = &*(a as *const [u64; N] as *const [u64; 4]);
        let b4 = &*(b as *const [u64; N] as *const [u64; 4]);
        let p4 = &*(ctx.p_words() as *const [u64; N] as *const [u64; 4]);
        let r = mont_mul_ifma1::<4, 5>(a4, b4, p4, p52, n0);
        return Some(*(&r as *const [u64; 4] as *const [u64; N]));
    }
    if N == 6 {
        let a6 = &*(a as *const [u64; N] as *const [u64; 6]);
        let b6 = &*(b as *const [u64; N] as *const [u64; 6]);
        let p6 = &*(ctx.p_words() as *const [u64; N] as *const [u64; 6]);
        let r = mont_mul_ifma1::<6, 8>(a6, b6, p6, p52, n0);
        return Some(*(&r as *const [u64; 6] as *const [u64; N]));
    }
    None
}

/// [`mul_ifma1_unchecked`] when the CPU has IFMA, else `None`.
pub(crate) fn mul_ifma1<const N: usize>(
    ctx: &Mont<N>,
    a: &[u64; N],
    b: &[u64; N],
) -> Option<[u64; N]> {
    if has_ifma() {
        // SAFETY: has_ifma() checked the CPU features.
        return unsafe { mul_ifma1_unchecked(ctx, a, b) };
    }
    None
}

/// [`ProdAdxKernel`] with the IFMA single-product kernel at 4 and 6 limbs
/// (products and squares).  Benchmark arm only.  Dispatch only after
/// [`has_ifma`] and [`has_adx`].
pub(crate) struct IfmaKernel;

impl MulKernel for IfmaKernel {
    #[inline(always)]
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        debug_assert!(has_ifma() && has_adx());
        // SAFETY: dispatched only after has_ifma().
        unsafe { mul_ifma1_unchecked(ctx, a, b) }.unwrap_or_else(|| ProdAdxKernel::mul(ctx, a, b))
    }
    #[inline(always)]
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N] {
        // SAFETY: as above.
        unsafe { mul_ifma1_unchecked(ctx, a, a) }.unwrap_or_else(|| ProdAdxKernel::sqr(ctx, a))
    }
}

/// Limb-slice eight values (each `≤ 8` 64-bit words) into `L` lane vectors
/// of 52-bit limbs.
pub(crate) fn to_lanes<const L: usize>(xs: &[[u64; 8]; 8]) -> [__m512i; L] {
    let packs: [[u64; 8]; 8] = std::array::from_fn(|lane| pack52::<L>(&xs[lane], 0));
    // SAFETY: plain unaligned loads of 8 u64 (AVX-512F is checked by callers
    // before any kernel runs; the loads themselves are SSE2-safe types).
    std::array::from_fn(|k| {
        let col: [u64; 8] = std::array::from_fn(|lane| packs[lane][k]);
        unsafe { core::mem::transmute::<[u64; 8], __m512i>(col) }
    })
}

/// IFMA benchmark probes for [`field_kernel_probe`](super::point::field_kernel_probe):
/// `ifma1_mul` (dependent-chain latency of [`mul_ifma1`]), `ifma8_mul`
/// ([`mont_mul_ifma8`], eight dependent chains in the lanes, ns per
/// product) and `adx8_mul` (eight independent chains of the production ADX
/// kernel interleaved, ns per product, the scalar throughput reference).
/// `None` without IFMA (or ADX for `adx8_mul`) or for widths other than 4
/// and 6.  Returns ns per product and a checksum.
pub(crate) fn probe<const N: usize>(
    op: &str,
    ctx: &Mont<N>,
    a: &[u64; N],
    b: &[u64; N],
    iters: u64,
) -> Option<(f64, u64)> {
    if N != 4 && N != 6 {
        return None;
    }
    match op {
        "ifma1_mul" => {
            mul_ifma1(ctx, a, b)?;
            let mut acc = *a;
            let t0 = std::time::Instant::now();
            for _ in 0..iters {
                acc = mul_ifma1(ctx, std::hint::black_box(&acc), b).unwrap();
            }
            Some((t0.elapsed().as_nanos() as f64 / iters as f64, acc[0]))
        }
        "adx8_mul" => {
            if !has_adx() {
                return None;
            }
            let mut acc: [[u64; N]; 8] = std::array::from_fn(|k| {
                let mut x = *a;
                x[0] ^= k as u64;
                if geq(&x, ctx.p_words()) {
                    x = *a;
                }
                x
            });
            let rounds = (iters / 8).max(1);
            let t0 = std::time::Instant::now();
            for _ in 0..rounds {
                for x in acc.iter_mut() {
                    *x = ProdAdxKernel::mul(ctx, std::hint::black_box(x), b);
                }
            }
            Some((
                t0.elapsed().as_nanos() as f64 / (rounds * 8) as f64,
                acc[7][0],
            ))
        }
        "ifma8_mul" => {
            if !has_ifma() {
                return None;
            }
            let wide = |x: &[u64; N]| -> [u64; 8] {
                let mut w = [0u64; 8];
                w[..N].copy_from_slice(x);
                w
            };
            let av: [[u64; 8]; 8] = std::array::from_fn(|k| {
                let mut w = wide(a);
                w[0] ^= k as u64 & 1;
                w
            });
            let bv = [wide(b); 8];
            let pv = [wide(ctx.p_words()); 8];
            let n0 = ctx.ifma_consts().1;
            let rounds = (iters / 8).max(1);
            // SAFETY: has_ifma() checked the CPU features.
            unsafe {
                let n0v = _mm512_set1_epi64(n0 as i64);
                macro_rules! run {
                    ($l:literal) => {{
                        let mut acc = to_lanes::<$l>(&av);
                        let bl = to_lanes::<$l>(&bv);
                        let pl = to_lanes::<$l>(&pv);
                        let t0 = std::time::Instant::now();
                        for _ in 0..rounds {
                            acc = mont_mul_ifma8::<$l>(std::hint::black_box(&acc), &bl, &pl, n0v);
                        }
                        let ns = t0.elapsed().as_nanos() as f64 / (rounds * 8) as f64;
                        let lanes: [u64; 8] = core::mem::transmute(acc[0]);
                        (ns, lanes[0])
                    }};
                }
                Some(if N == 4 { run!(5) } else { run!(8) })
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::{BigUint, RandBigInt};
    use rand::{rngs::StdRng, SeedableRng};

    fn two(e: u32) -> BigUint {
        BigUint::from(1u8) << e
    }

    /// The single-product kernel equals CIOS, and the eight-lane kernel is
    /// congruent to `a·b·R'^-1` and below `2p`, at 4 and 6 limbs.
    #[test]
    fn ifma_kernels_match_cios() {
        if !has_ifma() {
            eprintln!("skipping: no AVX-512 IFMA on this CPU");
            return;
        }
        let mut rng = StdRng::seed_from_u64(0x6966_6d61);
        fn check<const N: usize, const L: usize>(m: &BigUint, rng: &mut StdRng) {
            let ctx = Mont::<N>::new(m).unwrap();
            let mut vals = vec![
                BigUint::from(0u8),
                BigUint::from(1u8),
                m - 1u32,
                m - 2u32,
                m >> 1,
            ];
            for _ in 0..200 {
                vals.push(rng.gen_biguint_below(m));
            }
            let ms: Vec<_> = vals.iter().map(|v| ctx.enter(v).unwrap()).collect();
            for a in &ms {
                for b in ms.iter().take(12) {
                    assert_eq!(
                        mul_ifma1(&ctx, a, b),
                        Some(ctx.mul(a, b)),
                        "ifma1 N={N} m={m}"
                    );
                }
            }
            // eight lanes, inputs in [0, 2p)
            let rp = BigUint::from(1u8) << (52 * L);
            let rinv = rp.modinv(m).unwrap();
            let tp = m * 2u32;
            let wide = |x: &BigUint| -> [u64; 8] {
                let mut w = [0u64; 8];
                for (i, d) in x.iter_u64_digits().enumerate() {
                    w[i] = d;
                }
                w
            };
            let p8 = [wide(m); 8];
            let n0 = ctx.ifma_consts().1;
            for round in 0..200 {
                let a: [BigUint; 8] = std::array::from_fn(|k| {
                    if round == 0 && k < 2 {
                        &tp - 1u32
                    } else {
                        rng.gen_biguint_below(&tp)
                    }
                });
                let b: [BigUint; 8] = std::array::from_fn(|_| rng.gen_biguint_below(&tp));
                // SAFETY: has_ifma() checked above.
                let out = unsafe {
                    let r = mont_mul_ifma8::<L>(
                        &to_lanes::<L>(&std::array::from_fn(|k| wide(&a[k]))),
                        &to_lanes::<L>(&std::array::from_fn(|k| wide(&b[k]))),
                        &to_lanes::<L>(&p8),
                        _mm512_set1_epi64(n0 as i64),
                    );
                    r.map(|v| core::mem::transmute::<__m512i, [u64; 8]>(v))
                };
                for lane in 0..8 {
                    let mut x = BigUint::from(0u8);
                    for k in (0..L).rev() {
                        x = (x << 52usize) + BigUint::from(out[k][lane]);
                    }
                    assert!(x < tp, "ifma8 bound N={N}");
                    assert_eq!(
                        x % m,
                        (&a[lane] * &b[lane] * &rinv) % m,
                        "ifma8 N={N} m={m}"
                    );
                }
            }
        }
        check::<4, 5>(&(two(255) - 19u32), &mut rng);
        check::<4, 5>(&(two(256) - 189u32), &mut rng);
        check::<6, 8>(&(two(384) - two(128) - two(96) + two(32) - 1u32), &mut rng);
        check::<6, 8>(&(two(383) + 231u32), &mut rng);
    }
}
