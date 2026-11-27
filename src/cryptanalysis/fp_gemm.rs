//! Exact-integer AVX-512 / AVX2 / portable `F_p` blocked GEMM (`C += A * B mod p`).
//!
//! Uses IEEE-754 `f64` fused multiply-add (`vfmadd231pd`, 16 muladds/cycle/core on
//! AVX-512 Xeon) as exact 53-bit integer arithmetic whenever `p < 2^26` and
//! `k_safe_f64(p) = floor((2^53 - (p - 1)) / (p - 1)^2) >= 16`.
//!
//! Every intermediate accumulator is guaranteed to remain an exact non-negative
//! integer in `[0, 2^53]`, so results are bit-for-bit identical to exact `u128`
//! modular arithmetic.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Maximum number of `(a * b)` products (`0 <= a, b < p`) that can be accumulated
/// into an `f64` starting from a value `< p` without exceeding `2^52` (so both `v`
/// and `floor(v / p) * p` stay strictly inside IEEE-754 `f64`'s 53-bit exact integer
/// range, enabling 4-instruction vectorized AVX-512 modular reduction).
#[inline]
pub fn k_safe_f64(p: u64) -> usize {
    if p <= 1 || p >= (1u64 << 26) {
        return 0;
    }
    let p1 = p - 1;
    let sq = p1 * p1;
    let max_int = 1u64 << 52;
    if sq == 0 || p1 >= max_int {
        return 0;
    }
    ((max_int - p1) / sq) as usize
}

/// Whether `p` supports exact-integer `f64` GEMM with at least 16 accumulations per flush.
#[inline]
pub fn supports_f64_gemm(p: u64) -> bool {
    k_safe_f64(p) >= 16
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GemmSimd {
    Portable,
    Avx2Fma,
    Avx512,
}

pub fn detect_gemm_simd() -> GemmSimd {
    #[cfg(target_arch = "x86_64")]
    {
        if std::env::var_os("FP_GEMM_SIMD").as_deref() == Some(std::ffi::OsStr::new("portable")) {
            return GemmSimd::Portable;
        }
        let force_avx2 =
            std::env::var_os("FP_GEMM_SIMD").as_deref() == Some(std::ffi::OsStr::new("avx2"));
        if !force_avx2
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512dq")
            && std::arch::is_x86_feature_detected!("avx512vl")
        {
            return GemmSimd::Avx512;
        }
        if std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("fma")
        {
            return GemmSimd::Avx2Fma;
        }
    }
    GemmSimd::Portable
}

#[inline(always)]
fn barrett_mod(v: u64, p: u64, bm: u64) -> u64 {
    let q = ((u128::from(v) * u128::from(bm)) >> 64) as u64;
    let mut r = v - q * p;
    if r >= p {
        r -= p;
    }
    if r >= p {
        r -= p;
    }
    r
}

/// Strictly lower-bound `1.0 / (p as f64)` by 1 ULP so `floor(v * inv_p_lo) <= floor(v / p)`
/// for every integer `v in [0, 2^52]`, guaranteeing `v - q * p` lies exactly in `[0, 2p)`.
#[inline(always)]
fn inv_p_lo(p: u64) -> f64 {
    let inv = 1.0f64 / (p as f64);
    f64::from_bits(inv.to_bits() - 1)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn reduce_f64_slice_mod_p_avx512(c: &mut [f64], p: u64) {
    let pf = p as f64;
    let vp = _mm512_set1_pd(pf);
    let vinv = _mm512_set1_pd(inv_p_lo(p));
    let n = c.len();
    let cp = c.as_mut_ptr();
    let mut i = 0;
    unsafe {
        while i + 16 <= n {
            let v0 = _mm512_loadu_pd(cp.add(i));
            let v1 = _mm512_loadu_pd(cp.add(i + 8));
            let q0 = _mm512_roundscale_pd::<0x09>(_mm512_mul_pd(v0, vinv));
            let q1 = _mm512_roundscale_pd::<0x09>(_mm512_mul_pd(v1, vinv));
            let r0 = _mm512_fnmadd_pd(q0, vp, v0);
            let r1 = _mm512_fnmadd_pd(q1, vp, v1);
            let m0 = _mm512_cmp_pd_mask::<_CMP_GE_OQ>(r0, vp);
            let m1 = _mm512_cmp_pd_mask::<_CMP_GE_OQ>(r1, vp);
            let r0 = _mm512_mask_sub_pd(r0, m0, r0, vp);
            let r1 = _mm512_mask_sub_pd(r1, m1, r1, vp);
            _mm512_storeu_pd(cp.add(i), r0);
            _mm512_storeu_pd(cp.add(i + 8), r1);
            i += 16;
        }
        while i + 8 <= n {
            let v0 = _mm512_loadu_pd(cp.add(i));
            let q0 = _mm512_roundscale_pd::<0x09>(_mm512_mul_pd(v0, vinv));
            let r0 = _mm512_fnmadd_pd(q0, vp, v0);
            let m0 = _mm512_cmp_pd_mask::<_CMP_GE_OQ>(r0, vp);
            let r0 = _mm512_mask_sub_pd(r0, m0, r0, vp);
            _mm512_storeu_pd(cp.add(i), r0);
            i += 8;
        }
    }
    let inv = inv_p_lo(p);
    while i < n {
        let v = c[i];
        let q = (v * inv).floor();
        let mut r = v - q * pf;
        if r >= pf {
            r -= pf;
        }
        c[i] = r;
        i += 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn reduce_f64_to_u32_avx512(src: &[f64], dst: &mut [u32], p: u64) -> u32 {
    let pf = p as f64;
    let vp = _mm512_set1_pd(pf);
    let vinv = _mm512_set1_pd(inv_p_lo(p));
    let vzero = _mm512_setzero_pd();
    let n = src.len();
    let sp = src.as_ptr();
    let dp = dst.as_mut_ptr();
    let mut nnz = 0u32;
    let mut i = 0;
    unsafe {
        while i + 8 <= n {
            let v0 = _mm512_loadu_pd(sp.add(i));
            let q0 = _mm512_roundscale_pd::<0x09>(_mm512_mul_pd(v0, vinv));
            let r0 = _mm512_fnmadd_pd(q0, vp, v0);
            let m0 = _mm512_cmp_pd_mask::<_CMP_GE_OQ>(r0, vp);
            let r0 = _mm512_mask_sub_pd(r0, m0, r0, vp);
            let nz = _mm512_cmp_pd_mask::<_CMP_NEQ_OQ>(r0, vzero);
            nnz += nz.count_ones();
            let u = _mm512_cvttpd_epu32(r0);
            _mm256_storeu_si256(dp.add(i) as *mut __m256i, u);
            i += 8;
        }
    }
    let inv = inv_p_lo(p);
    while i < n {
        let v = src[i];
        let q = (v * inv).floor();
        let mut r = v - q * pf;
        if r >= pf {
            r -= pf;
        }
        let u = r as u32;
        dst[i] = u;
        if u != 0 {
            nnz += 1;
        }
        i += 1;
    }
    nnz
}

/// Exact `f64 -> f64` modular reduction of integers in `[0, 2^52]` into `[0, p)`.
#[inline]
pub fn reduce_f64_slice_mod_p(c: &mut [f64], p: u64, bm: u64) {
    #[cfg(target_arch = "x86_64")]
    if p < (1u64 << 26) && std::arch::is_x86_feature_detected!("avx512f") {
        unsafe { reduce_f64_slice_mod_p_avx512(c, p) };
        return;
    }
    for x in c.iter_mut() {
        let v = *x as u64;
        *x = barrett_mod(v, p, bm) as f64;
    }
}

/// Exact `f64 -> u32` modular reduction of integers in `[0, 2^52]` into `[0, p)`, returning nnz.
#[inline]
pub fn reduce_f64_to_u32(simd: GemmSimd, src: &[f64], dst: &mut [u32], p: u64, bm: u64) -> u32 {
    debug_assert_eq!(src.len(), dst.len());
    #[cfg(target_arch = "x86_64")]
    if simd == GemmSimd::Avx512 && p < (1u64 << 26) {
        return unsafe { reduce_f64_to_u32_avx512(src, dst, p) };
    }
    let mut nnz = 0u32;
    for (d, &x) in dst.iter_mut().zip(src.iter()) {
        let v = barrett_mod(x as u64, p, bm) as u32;
        *d = v;
        if v != 0 {
            nnz += 1;
        }
    }
    nnz
}

/// Add `c_f64[i] mod p` into `dst_u64[i]` (where `dst_u64[i] < p^2` lazily, keeping `< p^2`).
#[inline]
pub fn add_f64_reduced_into_u64(dst: &mut [u64], c_f64: &[f64], p: u64, bm: u64, p2: u64) {
    debug_assert_eq!(dst.len(), c_f64.len());
    #[cfg(target_arch = "x86_64")]
    if p < (1u64 << 26)
        && std::arch::is_x86_feature_detected!("avx512f")
        && std::arch::is_x86_feature_detected!("avx512dq")
    {
        unsafe { add_f64_reduced_into_u64_avx512(dst, c_f64, p, p2) };
        return;
    }
    for (d, &x) in dst.iter_mut().zip(c_f64.iter()) {
        if x != 0.0 {
            let v = barrett_mod(x as u64, p, bm);
            let t = *d + v;
            *d = if t >= p2 { t - p2 } else { t };
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512dq")]
unsafe fn add_f64_reduced_into_u64_avx512(dst: &mut [u64], c_f64: &[f64], p: u64, p2: u64) {
    let pf = p as f64;
    let vp = _mm512_set1_pd(pf);
    let vinv = _mm512_set1_pd(inv_p_lo(p));
    let vp2 = _mm512_set1_epi64(p2 as i64);
    let n = dst.len();
    let dp = dst.as_mut_ptr();
    let sp = c_f64.as_ptr();
    let mut i = 0;
    unsafe {
        while i + 8 <= n {
            let v0 = _mm512_loadu_pd(sp.add(i));
            let q0 = _mm512_roundscale_pd::<0x09>(_mm512_mul_pd(v0, vinv));
            let r0 = _mm512_fnmadd_pd(q0, vp, v0);
            let m0 = _mm512_cmp_pd_mask::<_CMP_GE_OQ>(r0, vp);
            let r0 = _mm512_mask_sub_pd(r0, m0, r0, vp);
            let rv = _mm512_cvttpd_epu64(r0);
            let dv = _mm512_loadu_si512(dp.add(i) as *const __m512i);
            let sum = _mm512_add_epi64(dv, rv);
            let sub = _mm512_sub_epi64(sum, vp2);
            let out = _mm512_min_epu64(sum, sub);
            _mm512_storeu_si512(dp.add(i) as *mut __m512i, out);
            i += 8;
        }
    }
    let inv = inv_p_lo(p);
    while i < n {
        let v = c_f64[i];
        if v != 0.0 {
            let q = (v * inv).floor();
            let mut r = v - q * pf;
            if r >= pf {
                r -= pf;
            }
            let t = dst[i] + r as u64;
            dst[i] = if t >= p2 { t - p2 } else { t };
        }
        i += 1;
    }
}

/// Microkernel dimensions for AVX-512 (`6 x 24` = 18 ZMM accumulators).
pub const MR512: usize = 6;
pub const NR512: usize = 24;

/// Microkernel dimensions for AVX2+FMA (`4 x 12` = 12 YMM accumulators).
pub const MR256: usize = 4;
pub const NR256: usize = 12;

/// `C[0..6, 0..24] += A[0..6, 0..kc] * B_pack[0..kc, 0..24]` in AVX-512F.
/// `a` has row stride `lda`; `b_pack` has row stride `24`; `c` has row stride `ldc`.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn microkernel_6x24_avx512(
    kc: usize,
    a: *const f64,
    lda: usize,
    b_pack: *const f64,
    c: *mut f64,
    ldc: usize,
) {
    unsafe {
        let mut c00 = _mm512_loadu_pd(c);
        let mut c01 = _mm512_loadu_pd(c.add(8));
        let mut c02 = _mm512_loadu_pd(c.add(16));

        let c1 = c.add(ldc);
        let mut c10 = _mm512_loadu_pd(c1);
        let mut c11 = _mm512_loadu_pd(c1.add(8));
        let mut c12 = _mm512_loadu_pd(c1.add(16));

        let c2 = c.add(2 * ldc);
        let mut c20 = _mm512_loadu_pd(c2);
        let mut c21 = _mm512_loadu_pd(c2.add(8));
        let mut c22 = _mm512_loadu_pd(c2.add(16));

        let c3 = c.add(3 * ldc);
        let mut c30 = _mm512_loadu_pd(c3);
        let mut c31 = _mm512_loadu_pd(c3.add(8));
        let mut c32 = _mm512_loadu_pd(c3.add(16));

        let c4 = c.add(4 * ldc);
        let mut c40 = _mm512_loadu_pd(c4);
        let mut c41 = _mm512_loadu_pd(c4.add(8));
        let mut c42 = _mm512_loadu_pd(c4.add(16));

        let c5 = c.add(5 * ldc);
        let mut c50 = _mm512_loadu_pd(c5);
        let mut c51 = _mm512_loadu_pd(c5.add(8));
        let mut c52 = _mm512_loadu_pd(c5.add(16));

        let a0 = a;
        let a1 = a.add(lda);
        let a2 = a.add(2 * lda);
        let a3 = a.add(3 * lda);
        let a4 = a.add(4 * lda);
        let a5 = a.add(5 * lda);

        let mut bp = b_pack;
        for k in 0..kc {
            let b0 = _mm512_loadu_pd(bp);
            let b1 = _mm512_loadu_pd(bp.add(8));
            let b2 = _mm512_loadu_pd(bp.add(16));
            bp = bp.add(24);

            let va0 = _mm512_set1_pd(*a0.add(k));
            c00 = _mm512_fmadd_pd(va0, b0, c00);
            c01 = _mm512_fmadd_pd(va0, b1, c01);
            c02 = _mm512_fmadd_pd(va0, b2, c02);

            let va1 = _mm512_set1_pd(*a1.add(k));
            c10 = _mm512_fmadd_pd(va1, b0, c10);
            c11 = _mm512_fmadd_pd(va1, b1, c11);
            c12 = _mm512_fmadd_pd(va1, b2, c12);

            let va2 = _mm512_set1_pd(*a2.add(k));
            c20 = _mm512_fmadd_pd(va2, b0, c20);
            c21 = _mm512_fmadd_pd(va2, b1, c21);
            c22 = _mm512_fmadd_pd(va2, b2, c22);

            let va3 = _mm512_set1_pd(*a3.add(k));
            c30 = _mm512_fmadd_pd(va3, b0, c30);
            c31 = _mm512_fmadd_pd(va3, b1, c31);
            c32 = _mm512_fmadd_pd(va3, b2, c32);

            let va4 = _mm512_set1_pd(*a4.add(k));
            c40 = _mm512_fmadd_pd(va4, b0, c40);
            c41 = _mm512_fmadd_pd(va4, b1, c41);
            c42 = _mm512_fmadd_pd(va4, b2, c42);

            let va5 = _mm512_set1_pd(*a5.add(k));
            c50 = _mm512_fmadd_pd(va5, b0, c50);
            c51 = _mm512_fmadd_pd(va5, b1, c51);
            c52 = _mm512_fmadd_pd(va5, b2, c52);
        }

        _mm512_storeu_pd(c, c00);
        _mm512_storeu_pd(c.add(8), c01);
        _mm512_storeu_pd(c.add(16), c02);

        _mm512_storeu_pd(c1, c10);
        _mm512_storeu_pd(c1.add(8), c11);
        _mm512_storeu_pd(c1.add(16), c12);

        _mm512_storeu_pd(c2, c20);
        _mm512_storeu_pd(c2.add(8), c21);
        _mm512_storeu_pd(c2.add(16), c22);

        _mm512_storeu_pd(c3, c30);
        _mm512_storeu_pd(c3.add(8), c31);
        _mm512_storeu_pd(c3.add(16), c32);

        _mm512_storeu_pd(c4, c40);
        _mm512_storeu_pd(c4.add(8), c41);
        _mm512_storeu_pd(c4.add(16), c42);

        _mm512_storeu_pd(c5, c50);
        _mm512_storeu_pd(c5.add(8), c51);
        _mm512_storeu_pd(c5.add(16), c52);
    }
}

/// Edge microkernel for `1 <= mr <= 6`, `1 <= nr <= 24`, with `b_pack` stride `24` (padded with 0).
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn microkernel_edge_avx512(
    mr: usize,
    nr: usize,
    kc: usize,
    a: *const f64,
    lda: usize,
    b_pack: *const f64,
    c: *mut f64,
    ldc: usize,
) {
    let mut tmp = [0.0f64; MR512 * NR512];
    unsafe {
        for i in 0..mr {
            std::ptr::copy_nonoverlapping(c.add(i * ldc), tmp.as_mut_ptr().add(i * NR512), nr);
        }
        let mut a_tmp = [0.0f64; MR512];
        let mut bp = b_pack;
        let mut acc = [[_mm512_setzero_pd(); 3]; MR512];
        for i in 0..mr {
            let p = tmp.as_ptr().add(i * NR512);
            acc[i][0] = _mm512_loadu_pd(p);
            acc[i][1] = _mm512_loadu_pd(p.add(8));
            acc[i][2] = _mm512_loadu_pd(p.add(16));
        }
        for k in 0..kc {
            let b0 = _mm512_loadu_pd(bp);
            let b1 = _mm512_loadu_pd(bp.add(8));
            let b2 = _mm512_loadu_pd(bp.add(16));
            bp = bp.add(24);
            for i in 0..mr {
                a_tmp[i] = *a.add(i * lda + k);
                let va = _mm512_set1_pd(a_tmp[i]);
                acc[i][0] = _mm512_fmadd_pd(va, b0, acc[i][0]);
                acc[i][1] = _mm512_fmadd_pd(va, b1, acc[i][1]);
                acc[i][2] = _mm512_fmadd_pd(va, b2, acc[i][2]);
            }
        }
        for i in 0..mr {
            let p = tmp.as_mut_ptr().add(i * NR512);
            _mm512_storeu_pd(p, acc[i][0]);
            _mm512_storeu_pd(p.add(8), acc[i][1]);
            _mm512_storeu_pd(p.add(16), acc[i][2]);
            std::ptr::copy_nonoverlapping(p, c.add(i * ldc), nr);
        }
    }
}

/// `C[0..4, 0..12] += A[0..4, 0..kc] * B_pack[0..kc, 0..12]` in AVX2+FMA.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
unsafe fn microkernel_4x12_avx2(
    kc: usize,
    a: *const f64,
    lda: usize,
    b_pack: *const f64,
    c: *mut f64,
    ldc: usize,
) {
    unsafe {
        let mut c00 = _mm256_loadu_pd(c);
        let mut c01 = _mm256_loadu_pd(c.add(4));
        let mut c02 = _mm256_loadu_pd(c.add(8));

        let c1 = c.add(ldc);
        let mut c10 = _mm256_loadu_pd(c1);
        let mut c11 = _mm256_loadu_pd(c1.add(4));
        let mut c12 = _mm256_loadu_pd(c1.add(8));

        let c2 = c.add(2 * ldc);
        let mut c20 = _mm256_loadu_pd(c2);
        let mut c21 = _mm256_loadu_pd(c2.add(4));
        let mut c22 = _mm256_loadu_pd(c2.add(8));

        let c3 = c.add(3 * ldc);
        let mut c30 = _mm256_loadu_pd(c3);
        let mut c31 = _mm256_loadu_pd(c3.add(4));
        let mut c32 = _mm256_loadu_pd(c3.add(8));

        let mut bp = b_pack;
        for k in 0..kc {
            let b0 = _mm256_loadu_pd(bp);
            let b1 = _mm256_loadu_pd(bp.add(4));
            let b2 = _mm256_loadu_pd(bp.add(8));
            bp = bp.add(12);

            let va0 = _mm256_set1_pd(*a.add(k));
            c00 = _mm256_fmadd_pd(va0, b0, c00);
            c01 = _mm256_fmadd_pd(va0, b1, c01);
            c02 = _mm256_fmadd_pd(va0, b2, c02);

            let va1 = _mm256_set1_pd(*a.add(lda + k));
            c10 = _mm256_fmadd_pd(va1, b0, c10);
            c11 = _mm256_fmadd_pd(va1, b1, c11);
            c12 = _mm256_fmadd_pd(va1, b2, c12);

            let va2 = _mm256_set1_pd(*a.add(2 * lda + k));
            c20 = _mm256_fmadd_pd(va2, b0, c20);
            c21 = _mm256_fmadd_pd(va2, b1, c21);
            c22 = _mm256_fmadd_pd(va2, b2, c22);

            let va3 = _mm256_set1_pd(*a.add(3 * lda + k));
            c30 = _mm256_fmadd_pd(va3, b0, c30);
            c31 = _mm256_fmadd_pd(va3, b1, c31);
            c32 = _mm256_fmadd_pd(va3, b2, c32);
        }

        _mm256_storeu_pd(c, c00);
        _mm256_storeu_pd(c.add(4), c01);
        _mm256_storeu_pd(c.add(8), c02);

        _mm256_storeu_pd(c1, c10);
        _mm256_storeu_pd(c1.add(4), c11);
        _mm256_storeu_pd(c1.add(8), c12);

        _mm256_storeu_pd(c2, c20);
        _mm256_storeu_pd(c2.add(4), c21);
        _mm256_storeu_pd(c2.add(8), c22);

        _mm256_storeu_pd(c3, c30);
        _mm256_storeu_pd(c3.add(4), c31);
        _mm256_storeu_pd(c3.add(8), c32);
    }
}

/// Convert `src: &[u32]` into `dst: &mut [f64]` using AVX-512 `vcvtudq2pd` when available.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn cvt_u32_to_f64_avx512(src: &[u32], dst: &mut [f64]) {
    let n = src.len();
    let sp = src.as_ptr();
    let dp = dst.as_mut_ptr();
    let mut i = 0;
    unsafe {
        while i + 8 <= n {
            let v = _mm256_loadu_si256(sp.add(i) as *const __m256i);
            let f = _mm512_cvtepu32_pd(v);
            _mm512_storeu_pd(dp.add(i), f);
            i += 8;
        }
    }
    while i < n {
        dst[i] = src[i] as f64;
        i += 1;
    }
}

#[inline]
pub fn cvt_u32_to_f64(simd: GemmSimd, src: &[u32], dst: &mut [f64]) {
    debug_assert_eq!(src.len(), dst.len());
    #[cfg(target_arch = "x86_64")]
    if simd == GemmSimd::Avx512 {
        // SAFETY: `simd == Avx512` only after runtime feature detection.
        unsafe { cvt_u32_to_f64_avx512(src, dst) };
        return;
    }
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        *d = s as f64;
    }
}

/// Pack `B[k0..k1, j0..j1]` (provided as a row-major `kc x nc` slice `b_src` with stride `ldb`)
/// into NR-wide contiguous micro-panels of `kc * nr` `f64` values (zero-padded to multiples of `nr`).
fn pack_b_panels(
    b_src: &[f64],
    kc: usize,
    nc: usize,
    ldb: usize,
    nr: usize,
    b_pack: &mut Vec<f64>,
) {
    let n_panels = nc.div_ceil(nr);
    let total = n_panels * kc * nr;
    b_pack.clear();
    b_pack.resize(total, 0.0);
    for p_idx in 0..n_panels {
        let j0 = p_idx * nr;
        let w = (nc - j0).min(nr);
        let dst_panel = &mut b_pack[p_idx * kc * nr..(p_idx + 1) * kc * nr];
        for kk in 0..kc {
            let src_row = &b_src[kk * ldb + j0..kk * ldb + j0 + w];
            let dst_row = &mut dst_panel[kk * nr..kk * nr + w];
            dst_row.copy_from_slice(src_row);
        }
    }
}

/// Compute `C[m x n] += A[m x k] * B[k x n]` in exact `f64` arithmetic without modular reduction.
/// Caller must ensure `k <= k_safe_f64(p)` (or reduce `C` between `K`-blocks).
/// `a` has shape `m x lda` (`lda >= k`), `b` has shape `k x ldb` (`ldb >= n`),
/// `c` has shape `m x ldc` (`ldc >= n`).
pub fn gemm_f64_raw(
    simd: GemmSimd,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    c: &mut [f64],
    ldc: usize,
    b_pack: &mut Vec<f64>,
) {
    if m == 0 || n == 0 || k == 0 {
        return;
    }
    const KC: usize = 256;
    const NC512: usize = 240; // 10 * NR512; 256 * 240 * 8 = 480 KiB (fits in 2 MiB L2)
    const NC256: usize = 192; // 16 * NR256; 256 * 192 * 8 = 384 KiB

    match simd {
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx512 => {
            let mut j0 = 0;
            while j0 < n {
                let nc = (n - j0).min(NC512);
                let mut k0 = 0;
                while k0 < k {
                    let kc = (k - k0).min(KC);
                    pack_b_panels(&b[k0 * ldb + j0..], kc, nc, ldb, NR512, b_pack);
                    let n_panels = nc.div_ceil(NR512);
                    for p_idx in 0..n_panels {
                        let jj = j0 + p_idx * NR512;
                        let nr = (j0 + nc - jj).min(NR512);
                        let bp = unsafe { b_pack.as_ptr().add(p_idx * kc * NR512) };
                        let mut i = 0;
                        if nr == NR512 {
                            while i + MR512 <= m {
                                unsafe {
                                    microkernel_6x24_avx512(
                                        kc,
                                        a.as_ptr().add(i * lda + k0),
                                        lda,
                                        bp,
                                        c.as_mut_ptr().add(i * ldc + jj),
                                        ldc,
                                    );
                                }
                                i += MR512;
                            }
                            if i < m {
                                unsafe {
                                    microkernel_edge_avx512(
                                        m - i,
                                        nr,
                                        kc,
                                        a.as_ptr().add(i * lda + k0),
                                        lda,
                                        bp,
                                        c.as_mut_ptr().add(i * ldc + jj),
                                        ldc,
                                    );
                                }
                            }
                        } else {
                            while i < m {
                                let mr = (m - i).min(MR512);
                                unsafe {
                                    microkernel_edge_avx512(
                                        mr,
                                        nr,
                                        kc,
                                        a.as_ptr().add(i * lda + k0),
                                        lda,
                                        bp,
                                        c.as_mut_ptr().add(i * ldc + jj),
                                        ldc,
                                    );
                                }
                                i += mr;
                            }
                        }
                    }
                    k0 += kc;
                }
                j0 += nc;
            }
        }
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx2Fma => {
            let mut j0 = 0;
            while j0 < n {
                let nc = (n - j0).min(NC256);
                let mut k0 = 0;
                while k0 < k {
                    let kc = (k - k0).min(KC);
                    pack_b_panels(&b[k0 * ldb + j0..], kc, nc, ldb, NR256, b_pack);
                    let n_panels = nc.div_ceil(NR256);
                    for p_idx in 0..n_panels {
                        let jj = j0 + p_idx * NR256;
                        let nr = (j0 + nc - jj).min(NR256);
                        let bp = unsafe { b_pack.as_ptr().add(p_idx * kc * NR256) };
                        let mut i = 0;
                        if nr == NR256 {
                            while i + MR256 <= m {
                                unsafe {
                                    microkernel_4x12_avx2(
                                        kc,
                                        a.as_ptr().add(i * lda + k0),
                                        lda,
                                        bp,
                                        c.as_mut_ptr().add(i * ldc + jj),
                                        ldc,
                                    );
                                }
                                i += MR256;
                            }
                        }
                        while i < m {
                            let c_row = &mut c[i * ldc + jj..i * ldc + jj + nr];
                            let a_row = &a[i * lda + k0..i * lda + k0 + kc];
                            for kk in 0..kc {
                                let f = a_row[kk];
                                if f == 0.0 {
                                    continue;
                                }
                                let b_row = &b_pack[p_idx * kc * NR256 + kk * NR256..][..nr];
                                for j in 0..nr {
                                    c_row[j] += f * b_row[j];
                                }
                            }
                            i += 1;
                        }
                    }
                    k0 += kc;
                }
                j0 += nc;
            }
        }
        _ => {
            for i in 0..m {
                let c_row = &mut c[i * ldc..i * ldc + n];
                for kk in 0..k {
                    let f = a[i * lda + kk];
                    if f == 0.0 {
                        continue;
                    }
                    let b_row = &b[kk * ldb..kk * ldb + n];
                    for j in 0..n {
                        c_row[j] += f * b_row[j];
                    }
                }
            }
        }
    }
}

/// Panel width `NR` matching the active SIMD microkernel.
#[inline]
pub fn nr_for(simd: GemmSimd) -> usize {
    match simd {
        GemmSimd::Avx512 => NR512,
        GemmSimd::Avx2Fma => NR256,
        GemmSimd::Portable => 8,
    }
}

/// Resize and zero-fill `b_pack` for a `kc x nc` panel with micro-panel width `nr`.
#[inline]
pub fn prepare_b_pack(b_pack: &mut Vec<f64>, kc: usize, nc: usize, nr: usize) {
    let total = nc.div_ceil(nr) * kc * nr;
    b_pack.clear();
    b_pack.resize(total, 0.0);
}

/// Write dense `u32` values `src` into row `kk` of `b_pack` starting at tile-relative column `col_off`.
#[inline]
pub fn pack_b_put_u32_row(
    simd: GemmSimd,
    b_pack: &mut [f64],
    kc: usize,
    nr: usize,
    kk: usize,
    col_off: usize,
    src: &[u32],
) {
    if src.is_empty() {
        return;
    }
    let end = col_off + src.len();
    let mut c = col_off;
    while c < end {
        let p_idx = c / nr;
        let in_p = c - p_idx * nr;
        let take = (nr - in_p).min(end - c);
        let dst = &mut b_pack[p_idx * kc * nr + kk * nr + in_p..][..take];
        let s = &src[c - col_off..][..take];
        cvt_u32_to_f64(simd, s, dst);
        c += take;
    }
}

/// Write a single entry `(col_in_tile, val)` into row `kk` of `b_pack`.
#[inline(always)]
pub fn pack_b_put_entry(b_pack: &mut [f64], kc: usize, nr: usize, kk: usize, col: usize, val: f64) {
    let p_idx = col / nr;
    let in_p = col - p_idx * nr;
    b_pack[p_idx * kc * nr + kk * nr + in_p] = val;
}

/// Compute `C[m x nc] += A[m x kc] * B_pack[kc x nc]` directly from an already-packed `b_pack`
/// (micro-panel width `nr = nr_for(simd)`). Caller ensures `kc` accumulation stays `<= 2^53`.
pub fn gemm_f64_with_packed_b(
    simd: GemmSimd,
    m: usize,
    nc: usize,
    kc: usize,
    a: &[f64],
    lda: usize,
    b_pack: &[f64],
    c: &mut [f64],
    ldc: usize,
) {
    if m == 0 || nc == 0 || kc == 0 {
        return;
    }
    match simd {
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx512 => {
            let n_panels = nc.div_ceil(NR512);
            for p_idx in 0..n_panels {
                let jj = p_idx * NR512;
                let nr = (nc - jj).min(NR512);
                let bp = unsafe { b_pack.as_ptr().add(p_idx * kc * NR512) };
                let mut i = 0;
                if nr == NR512 {
                    while i + MR512 <= m {
                        unsafe {
                            microkernel_6x24_avx512(
                                kc,
                                a.as_ptr().add(i * lda),
                                lda,
                                bp,
                                c.as_mut_ptr().add(i * ldc + jj),
                                ldc,
                            );
                        }
                        i += MR512;
                    }
                    if i < m {
                        unsafe {
                            microkernel_edge_avx512(
                                m - i,
                                nr,
                                kc,
                                a.as_ptr().add(i * lda),
                                lda,
                                bp,
                                c.as_mut_ptr().add(i * ldc + jj),
                                ldc,
                            );
                        }
                    }
                } else {
                    while i < m {
                        let mr = (m - i).min(MR512);
                        unsafe {
                            microkernel_edge_avx512(
                                mr,
                                nr,
                                kc,
                                a.as_ptr().add(i * lda),
                                lda,
                                bp,
                                c.as_mut_ptr().add(i * ldc + jj),
                                ldc,
                            );
                        }
                        i += mr;
                    }
                }
            }
        }
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx2Fma => {
            let n_panels = nc.div_ceil(NR256);
            for p_idx in 0..n_panels {
                let jj = p_idx * NR256;
                let nr = (nc - jj).min(NR256);
                let bp = unsafe { b_pack.as_ptr().add(p_idx * kc * NR256) };
                let mut i = 0;
                if nr == NR256 {
                    while i + MR256 <= m {
                        unsafe {
                            microkernel_4x12_avx2(
                                kc,
                                a.as_ptr().add(i * lda),
                                lda,
                                bp,
                                c.as_mut_ptr().add(i * ldc + jj),
                                ldc,
                            );
                        }
                        i += MR256;
                    }
                }
                while i < m {
                    let c_row = &mut c[i * ldc + jj..i * ldc + jj + nr];
                    let a_row = &a[i * lda..i * lda + kc];
                    for kk in 0..kc {
                        let f = a_row[kk];
                        if f == 0.0 {
                            continue;
                        }
                        let b_row = &b_pack[p_idx * kc * NR256 + kk * NR256..][..nr];
                        for j in 0..nr {
                            c_row[j] += f * b_row[j];
                        }
                    }
                    i += 1;
                }
            }
        }
        _ => {
            let nr = 8usize;
            let n_panels = nc.div_ceil(nr);
            for p_idx in 0..n_panels {
                let jj = p_idx * nr;
                let w = (nc - jj).min(nr);
                for i in 0..m {
                    let c_row = &mut c[i * ldc + jj..i * ldc + jj + w];
                    let a_row = &a[i * lda..i * lda + kc];
                    for kk in 0..kc {
                        let f = a_row[kk];
                        if f == 0.0 {
                            continue;
                        }
                        let b_row = &b_pack[p_idx * kc * nr + kk * nr..][..w];
                        for j in 0..w {
                            c_row[j] += f * b_row[j];
                        }
                    }
                }
            }
        }
    }
}

/// Vectorized `dst[j] += f * src[j]` on exact-integer `f64` slices.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn axpy_f64_avx512(dst: &mut [f64], src: &[f64], f: f64) {
    let n = dst.len();
    let dp = dst.as_mut_ptr();
    let sp = src.as_ptr();
    let vf = _mm512_set1_pd(f);
    let mut i = 0;
    unsafe {
        while i + 32 <= n {
            let d0 = _mm512_loadu_pd(dp.add(i));
            let d1 = _mm512_loadu_pd(dp.add(i + 8));
            let d2 = _mm512_loadu_pd(dp.add(i + 16));
            let d3 = _mm512_loadu_pd(dp.add(i + 24));
            let s0 = _mm512_loadu_pd(sp.add(i));
            let s1 = _mm512_loadu_pd(sp.add(i + 8));
            let s2 = _mm512_loadu_pd(sp.add(i + 16));
            let s3 = _mm512_loadu_pd(sp.add(i + 24));
            _mm512_storeu_pd(dp.add(i), _mm512_fmadd_pd(vf, s0, d0));
            _mm512_storeu_pd(dp.add(i + 8), _mm512_fmadd_pd(vf, s1, d1));
            _mm512_storeu_pd(dp.add(i + 16), _mm512_fmadd_pd(vf, s2, d2));
            _mm512_storeu_pd(dp.add(i + 24), _mm512_fmadd_pd(vf, s3, d3));
            i += 32;
        }
        while i + 8 <= n {
            let d0 = _mm512_loadu_pd(dp.add(i));
            let s0 = _mm512_loadu_pd(sp.add(i));
            _mm512_storeu_pd(dp.add(i), _mm512_fmadd_pd(vf, s0, d0));
            i += 8;
        }
    }
    while i < n {
        dst[i] += f * src[i];
        i += 1;
    }
}

#[inline]
pub fn axpy_f64(simd: GemmSimd, dst: &mut [f64], src: &[f64], f: f64) {
    debug_assert_eq!(dst.len(), src.len());
    if f == 0.0 || dst.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    if simd == GemmSimd::Avx512 {
        unsafe { axpy_f64_avx512(dst, src, f) };
        return;
    }
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        *d += f * s;
    }
}

/// Register-blocked AVX-512 `1 x 64` SpMM for one output row:
/// `c_row[0..w] += sum_{(kk, f) in a_row} f * b_tile[kk * ldb .. kk * ldb + w]`.
/// Keeps 8 ZMM accumulators (`64` columns) live in registers across the entire `a_row`
/// (zero `c_row` traffic inside the `K` loop, zero multiplications by zero).
/// `b_tile` rows must be padded so `ldb >= w.div_ceil(8) * 8`.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn spmm_row_avx512(
    c_row: &mut [f64],
    w: usize,
    a_row: &[(u32, f64)],
    b_tile: &[f64],
    ldb: usize,
) {
    let cp = c_row.as_mut_ptr();
    let bp_base = b_tile.as_ptr();
    let mut j = 0usize;
    unsafe {
        while j + 64 <= w {
            let mut c0 = _mm512_loadu_pd(cp.add(j));
            let mut c1 = _mm512_loadu_pd(cp.add(j + 8));
            let mut c2 = _mm512_loadu_pd(cp.add(j + 16));
            let mut c3 = _mm512_loadu_pd(cp.add(j + 24));
            let mut c4 = _mm512_loadu_pd(cp.add(j + 32));
            let mut c5 = _mm512_loadu_pd(cp.add(j + 40));
            let mut c6 = _mm512_loadu_pd(cp.add(j + 48));
            let mut c7 = _mm512_loadu_pd(cp.add(j + 56));
            for &(kk, f) in a_row {
                let vf = _mm512_set1_pd(f);
                let bp = bp_base.add(kk as usize * ldb + j);
                c0 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp), c0);
                c1 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(8)), c1);
                c2 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(16)), c2);
                c3 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(24)), c3);
                c4 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(32)), c4);
                c5 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(40)), c5);
                c6 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(48)), c6);
                c7 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(56)), c7);
            }
            _mm512_storeu_pd(cp.add(j), c0);
            _mm512_storeu_pd(cp.add(j + 8), c1);
            _mm512_storeu_pd(cp.add(j + 16), c2);
            _mm512_storeu_pd(cp.add(j + 24), c3);
            _mm512_storeu_pd(cp.add(j + 32), c4);
            _mm512_storeu_pd(cp.add(j + 40), c5);
            _mm512_storeu_pd(cp.add(j + 48), c6);
            _mm512_storeu_pd(cp.add(j + 56), c7);
            j += 64;
        }
        while j + 16 <= w {
            let mut c0 = _mm512_loadu_pd(cp.add(j));
            let mut c1 = _mm512_loadu_pd(cp.add(j + 8));
            for &(kk, f) in a_row {
                let vf = _mm512_set1_pd(f);
                let bp = bp_base.add(kk as usize * ldb + j);
                c0 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp), c0);
                c1 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp.add(8)), c1);
            }
            _mm512_storeu_pd(cp.add(j), c0);
            _mm512_storeu_pd(cp.add(j + 8), c1);
            j += 16;
        }
        while j + 8 <= w {
            let mut c0 = _mm512_loadu_pd(cp.add(j));
            for &(kk, f) in a_row {
                let vf = _mm512_set1_pd(f);
                let bp = bp_base.add(kk as usize * ldb + j);
                c0 = _mm512_fmadd_pd(vf, _mm512_loadu_pd(bp), c0);
            }
            _mm512_storeu_pd(cp.add(j), c0);
            j += 8;
        }
    }
    if j < w {
        for &(kk, f) in a_row {
            let b_row = &b_tile[kk as usize * ldb..];
            for jj in j..w {
                c_row[jj] += f * b_row[jj];
            }
        }
    }
}

/// Register-blocked AVX2+FMA `1 x 32` SpMM for one output row.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
unsafe fn spmm_row_avx2(
    c_row: &mut [f64],
    w: usize,
    a_row: &[(u32, f64)],
    b_tile: &[f64],
    ldb: usize,
) {
    let cp = c_row.as_mut_ptr();
    let bp_base = b_tile.as_ptr();
    let mut j = 0usize;
    unsafe {
        while j + 32 <= w {
            let mut c0 = _mm256_loadu_pd(cp.add(j));
            let mut c1 = _mm256_loadu_pd(cp.add(j + 4));
            let mut c2 = _mm256_loadu_pd(cp.add(j + 8));
            let mut c3 = _mm256_loadu_pd(cp.add(j + 12));
            let mut c4 = _mm256_loadu_pd(cp.add(j + 16));
            let mut c5 = _mm256_loadu_pd(cp.add(j + 20));
            let mut c6 = _mm256_loadu_pd(cp.add(j + 24));
            let mut c7 = _mm256_loadu_pd(cp.add(j + 28));
            for &(kk, f) in a_row {
                let vf = _mm256_set1_pd(f);
                let bp = bp_base.add(kk as usize * ldb + j);
                c0 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp), c0);
                c1 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(4)), c1);
                c2 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(8)), c2);
                c3 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(12)), c3);
                c4 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(16)), c4);
                c5 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(20)), c5);
                c6 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(24)), c6);
                c7 = _mm256_fmadd_pd(vf, _mm256_loadu_pd(bp.add(28)), c7);
            }
            _mm256_storeu_pd(cp.add(j), c0);
            _mm256_storeu_pd(cp.add(j + 4), c1);
            _mm256_storeu_pd(cp.add(j + 8), c2);
            _mm256_storeu_pd(cp.add(j + 12), c3);
            _mm256_storeu_pd(cp.add(j + 16), c4);
            _mm256_storeu_pd(cp.add(j + 20), c5);
            _mm256_storeu_pd(cp.add(j + 24), c6);
            _mm256_storeu_pd(cp.add(j + 28), c7);
            j += 32;
        }
    }
    if j < w {
        for &(kk, f) in a_row {
            let b_row = &b_tile[kk as usize * ldb..];
            for jj in j..w {
                c_row[jj] += f * b_row[jj];
            }
        }
    }
}

/// Compute `c_row[0..w] += sum_{(kk, f) in a_row} f * b_tile[kk * ldb .. kk * ldb + w]`
/// with exact `f64` integer arithmetic (`a_row.len() <= k_safe_f64(p)`).
#[inline]
pub fn spmm_row_f64(
    simd: GemmSimd,
    c_row: &mut [f64],
    w: usize,
    a_row: &[(u32, f64)],
    b_tile: &[f64],
    ldb: usize,
) {
    if a_row.is_empty() || w == 0 {
        return;
    }
    match simd {
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx512 => unsafe { spmm_row_avx512(c_row, w, a_row, b_tile, ldb) },
        #[cfg(target_arch = "x86_64")]
        GemmSimd::Avx2Fma => unsafe { spmm_row_avx2(c_row, w, a_row, b_tile, ldb) },
        _ => {
            for &(kk, f) in a_row {
                let b_row = &b_tile[kk as usize * ldb..];
                for jj in 0..w {
                    c_row[jj] += f * b_row[jj];
                }
            }
        }
    }
}

/// Adaptive SpMM / GEMM: `C[m x w] += A_sparse[m x kc] * B_tile[kc x w]` (exact `f64`, no mod).
/// Automatically selects register-blocked `6 x 24` GEMM when `A` has density >= 20% and `m >= 6`,
/// or register-blocked `1 x 64` AVX-512 SpMM (zero wasted flops on zeros) when `A` is sparser.
pub fn spmm_or_gemm_f64(
    simd: GemmSimd,
    m: usize,
    w: usize,
    kc: usize,
    a_rows: &[Vec<(u32, f64)>],
    b_tile: &[f64],
    ldb: usize,
    c: &mut [f64],
    ldc: usize,
    a_dense_buf: &mut Vec<f64>,
    b_pack_buf: &mut Vec<f64>,
) {
    if m == 0 || w == 0 || kc == 0 {
        return;
    }
    let nnz: usize = a_rows.iter().map(|r| r.len()).sum();
    if nnz == 0 {
        return;
    }
    // Use dense 6x24 GEMM when density >= 20% and matrix is big enough to amortize B packing.
    if m >= 6 && kc >= 8 && w >= 24 && nnz * 5 >= m * kc {
        let nr = nr_for(simd);
        pack_b_panels(b_tile, kc, w, ldb, nr, b_pack_buf);
        a_dense_buf.clear();
        a_dense_buf.resize(m * kc, 0.0);
        for (i, row) in a_rows.iter().enumerate() {
            let base = i * kc;
            for &(kk, f) in row {
                a_dense_buf[base + kk as usize] = f;
            }
        }
        gemm_f64_with_packed_b(simd, m, w, kc, a_dense_buf, kc, b_pack_buf, c, ldc);
    } else {
        for (i, row) in a_rows.iter().enumerate() {
            if !row.is_empty() {
                spmm_row_f64(simd, &mut c[i * ldc..i * ldc + w], w, row, b_tile, ldb);
            }
        }
    }
}

/// Exact `C[m x n] = (C[m x n] + A[m x k] * B[k x n]) mod p`, automatically splitting `K`
/// into `k_safe_f64(p)` chunks so every intermediate sum stays strictly `<= 2^53`.
pub fn gemm_f64_mod_p(
    simd: GemmSimd,
    m: usize,
    n: usize,
    k: usize,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    c: &mut [f64],
    ldc: usize,
    p: u64,
    bm: u64,
    b_pack: &mut Vec<f64>,
) {
    let k_safe = k_safe_f64(p);
    assert!(k_safe >= 1, "prime {p} exceeds exact f64 GEMM bound");
    if m == 0 || n == 0 || k == 0 {
        return;
    }
    let mut k0 = 0;
    while k0 < k {
        let k_chunk = (k - k0).min(k_safe);
        gemm_f64_raw(
            simd,
            m,
            n,
            k_chunk,
            &a[k0..],
            lda,
            &b[k0 * ldb..],
            ldb,
            c,
            ldc,
            b_pack,
        );
        for i in 0..m {
            reduce_f64_slice_mod_p(&mut c[i * ldc..i * ldc + n], p, bm);
        }
        k0 += k_chunk;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn barrett_m(p: u64) -> u64 {
        ((1u128 << 64) / u128::from(p)) as u64
    }

    #[test]
    fn gemm_f64_mod_p_matches_exact_u128_across_sizes() {
        let p = 786_433u64;
        let bm = barrett_m(p);
        let simd = detect_gemm_simd();
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % p
        };
        for &(m, n, k) in &[(1, 1, 1), (6, 24, 17), (7, 25, 33), (19, 73, 300), (37, 263, 270)] {
            let a_u: Vec<u64> = (0..m * k).map(|_| next()).collect();
            let b_u: Vec<u64> = (0..k * n).map(|_| next()).collect();
            let c0_u: Vec<u64> = (0..m * n).map(|_| next()).collect();

            let a_f: Vec<f64> = a_u.iter().map(|&x| x as f64).collect();
            let b_f: Vec<f64> = b_u.iter().map(|&x| x as f64).collect();
            let mut c_f: Vec<f64> = c0_u.iter().map(|&x| x as f64).collect();
            let mut pack = Vec::new();
            gemm_f64_mod_p(simd, m, n, k, &a_f, k, &b_f, n, &mut c_f, n, p, bm, &mut pack);

            for i in 0..m {
                for j in 0..n {
                    let mut ref_v = u128::from(c0_u[i * n + j]);
                    for kk in 0..k {
                        ref_v += u128::from(a_u[i * k + kk]) * u128::from(b_u[kk * n + j]);
                    }
                    let want = (ref_v % u128::from(p)) as u64;
                    assert_eq!(c_f[i * n + j] as u64, want, "mismatch at ({i},{j}) for ({m},{n},{k})");
                }
            }
        }
    }

    #[test]
    fn reduce_f64_avx512_exact_at_edges_up_to_2_pow_52() {
        for &p in &[3u64, 5, 17, 257, 65521, 786_433, 33_554_393] {
            let bm = barrett_m(p);
            let max_v = 1u64 << 52;
            let mut vals: Vec<u64> = vec![
                0, 1, p - 1, p, p + 1, 2 * p - 1, 2 * p, 2 * p + 1,
                p * p - 1, p * p, p * p + 1,
                max_v - 2 * p, max_v - p - 1, max_v - p, max_v - p + 1, max_v - 1, max_v,
            ];
            let q_max = max_v / p;
            for dq in 0..16u64 {
                let base = (q_max - dq) * p;
                for off in [0u64, 1, p - 1] {
                    if base + off <= max_v {
                        vals.push(base + off);
                    }
                }
            }
            let mut f: Vec<f64> = vals.iter().map(|&v| v as f64).collect();
            let mut u = vec![0u32; vals.len()];
            let nnz = reduce_f64_to_u32(detect_gemm_simd(), &f, &mut u, p, bm);
            reduce_f64_slice_mod_p(&mut f, p, bm);
            let mut want_nnz = 0u32;
            for i in 0..vals.len() {
                let want = vals[i] % p;
                assert_eq!(f[i] as u64, want, "reduce_f64 mismatch for p={p} v={}", vals[i]);
                assert_eq!(u[i] as u64, want, "reduce_u32 mismatch for p={p} v={}", vals[i]);
                if want != 0 {
                    want_nnz += 1;
                }
            }
            assert_eq!(nnz, want_nnz);
        }
    }

    #[test]
    fn gemm_f64_mod_p_multi_chunk_near_limit_is_exact() {
        // Large prime near 2^25 where k_safe_f64(p) is small (~4), forcing many chunks.
        let p = 33_554_393u64;
        let bm = barrett_m(p);
        assert!(k_safe_f64(p) < 16 && k_safe_f64(p) >= 3);
        let (m, n, k) = (13, 51, 40);
        let a_u: Vec<u64> = (0..m * k).map(|i| (i as u64 * 1234567 + 99991) % p).collect();
        let b_u: Vec<u64> = (0..k * n).map(|i| (i as u64 * 7654321 + 31337) % p).collect();
        let c0_u: Vec<u64> = (0..m * n).map(|i| p - 1 - (i as u64 % 97)).collect();
        let a_f: Vec<f64> = a_u.iter().map(|&x| x as f64).collect();
        let b_f: Vec<f64> = b_u.iter().map(|&x| x as f64).collect();
        let mut c_f: Vec<f64> = c0_u.iter().map(|&x| x as f64).collect();
        let mut pack = Vec::new();
        gemm_f64_mod_p(
            detect_gemm_simd(),
            m,
            n,
            k,
            &a_f,
            k,
            &b_f,
            n,
            &mut c_f,
            n,
            p,
            bm,
            &mut pack,
        );
        for i in 0..m {
            for j in 0..n {
                let mut ref_v = u128::from(c0_u[i * n + j]);
                for kk in 0..k {
                    ref_v += u128::from(a_u[i * k + kk]) * u128::from(b_u[kk * n + j]);
                }
                assert_eq!(c_f[i * n + j] as u64, (ref_v % u128::from(p)) as u64);
            }
        }
    }
}
