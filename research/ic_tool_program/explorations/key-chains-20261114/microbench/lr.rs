// Funnel least-rotation kernel (copied from koblitz_fast.rs) with one min chain
// per vector, against the same kernel with its chain split into K accumulators.
use std::arch::x86_64::*;
use std::time::Instant;
#[inline(always)] unsafe fn doubled(v: __m512i, n: u32) -> (__m512i, __m512i) {
    let count = |s: u32| _mm_cvtsi64_si128(i64::from(s));
    (_mm512_or_si512(_mm512_sll_epi64(v, count(64 - n)), _mm512_srl_epi64(v, count(2 * n - 64))), _mm512_sll_epi64(v, count(128 - 2 * n)))
}
#[target_feature(enable = "avx512f,avx512vbmi2")]
unsafe fn k1(c: &[u64; 16], n: u32) -> [u64; 16] {
    let count = |s: u32| _mm_cvtsi64_si128(i64::from(s));
    let (ha, la) = doubled(_mm512_loadu_si512(c.as_ptr().cast()), n);
    let (hb, lb) = doubled(_mm512_loadu_si512(c.as_ptr().add(8).cast()), n);
    let mut ba = [ha; 1]; let mut bb = [hb; 1];
    if 1 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<1>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<1>(hb, lb)); }
    if 2 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<2>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<2>(hb, lb)); }
    if 3 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<3>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<3>(hb, lb)); }
    if 4 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<4>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<4>(hb, lb)); }
    if 5 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<5>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<5>(hb, lb)); }
    if 6 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<6>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<6>(hb, lb)); }
    if 7 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<7>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<7>(hb, lb)); }
    if 8 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<8>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<8>(hb, lb)); }
    if 9 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<9>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<9>(hb, lb)); }
    if 10 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<10>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<10>(hb, lb)); }
    if 11 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<11>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<11>(hb, lb)); }
    if 12 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<12>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<12>(hb, lb)); }
    if 13 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<13>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<13>(hb, lb)); }
    if 14 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<14>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<14>(hb, lb)); }
    if 15 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<15>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<15>(hb, lb)); }
    if 16 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<16>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<16>(hb, lb)); }
    if 17 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<17>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<17>(hb, lb)); }
    if 18 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<18>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<18>(hb, lb)); }
    if 19 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<19>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<19>(hb, lb)); }
    if 20 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<20>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<20>(hb, lb)); }
    if 21 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<21>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<21>(hb, lb)); }
    if 22 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<22>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<22>(hb, lb)); }
    if 23 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<23>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<23>(hb, lb)); }
    if 24 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<24>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<24>(hb, lb)); }
    if 25 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<25>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<25>(hb, lb)); }
    if 26 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<26>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<26>(hb, lb)); }
    if 27 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<27>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<27>(hb, lb)); }
    if 28 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<28>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<28>(hb, lb)); }
    if 29 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<29>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<29>(hb, lb)); }
    if 30 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<30>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<30>(hb, lb)); }
    if 31 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<31>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<31>(hb, lb)); }
    if 32 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<32>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<32>(hb, lb)); }
    if 33 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<33>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<33>(hb, lb)); }
    if 34 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<34>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<34>(hb, lb)); }
    if 35 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<35>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<35>(hb, lb)); }
    if 36 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<36>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<36>(hb, lb)); }
    if 37 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<37>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<37>(hb, lb)); }
    if 38 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<38>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<38>(hb, lb)); }
    if 39 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<39>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<39>(hb, lb)); }
    if 40 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<40>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<40>(hb, lb)); }
    if 41 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<41>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<41>(hb, lb)); }
    if 42 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<42>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<42>(hb, lb)); }
    if 43 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<43>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<43>(hb, lb)); }
    if 44 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<44>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<44>(hb, lb)); }
    if 45 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<45>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<45>(hb, lb)); }
    if 46 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<46>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<46>(hb, lb)); }
    if 47 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<47>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<47>(hb, lb)); }
    if 48 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<48>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<48>(hb, lb)); }
    if 49 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<49>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<49>(hb, lb)); }
    if 50 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<50>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<50>(hb, lb)); }
    if 51 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<51>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<51>(hb, lb)); }
    if 52 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<52>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<52>(hb, lb)); }
    if 53 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<53>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<53>(hb, lb)); }
    if 54 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<54>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<54>(hb, lb)); }
    if 55 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<55>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<55>(hb, lb)); }
    if 56 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<56>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<56>(hb, lb)); }
    if 57 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<57>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<57>(hb, lb)); }
    if 58 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<58>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<58>(hb, lb)); }
    if 59 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<59>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<59>(hb, lb)); }
    if 60 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<60>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<60>(hb, lb)); }
    if 61 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<61>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<61>(hb, lb)); }
    if 62 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<62>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<62>(hb, lb)); }
    let mut a = ba[0]; let mut b = bb[0];
    for i in 1..1 { a = _mm512_min_epu64(a, ba[i]); b = _mm512_min_epu64(b, bb[i]); }
    let mut out = [0u64; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_srl_epi64(a, count(64 - n)));
    _mm512_storeu_si512(out.as_mut_ptr().add(8).cast(), _mm512_srl_epi64(b, count(64 - n)));
    out
}

#[target_feature(enable = "avx512f,avx512vbmi2")]
unsafe fn k2(c: &[u64; 16], n: u32) -> [u64; 16] {
    let count = |s: u32| _mm_cvtsi64_si128(i64::from(s));
    let (ha, la) = doubled(_mm512_loadu_si512(c.as_ptr().cast()), n);
    let (hb, lb) = doubled(_mm512_loadu_si512(c.as_ptr().add(8).cast()), n);
    let mut ba = [ha; 2]; let mut bb = [hb; 2];
    if 1 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<1>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<1>(hb, lb)); }
    if 2 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<2>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<2>(hb, lb)); }
    if 3 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<3>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<3>(hb, lb)); }
    if 4 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<4>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<4>(hb, lb)); }
    if 5 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<5>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<5>(hb, lb)); }
    if 6 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<6>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<6>(hb, lb)); }
    if 7 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<7>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<7>(hb, lb)); }
    if 8 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<8>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<8>(hb, lb)); }
    if 9 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<9>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<9>(hb, lb)); }
    if 10 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<10>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<10>(hb, lb)); }
    if 11 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<11>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<11>(hb, lb)); }
    if 12 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<12>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<12>(hb, lb)); }
    if 13 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<13>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<13>(hb, lb)); }
    if 14 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<14>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<14>(hb, lb)); }
    if 15 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<15>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<15>(hb, lb)); }
    if 16 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<16>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<16>(hb, lb)); }
    if 17 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<17>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<17>(hb, lb)); }
    if 18 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<18>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<18>(hb, lb)); }
    if 19 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<19>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<19>(hb, lb)); }
    if 20 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<20>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<20>(hb, lb)); }
    if 21 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<21>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<21>(hb, lb)); }
    if 22 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<22>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<22>(hb, lb)); }
    if 23 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<23>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<23>(hb, lb)); }
    if 24 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<24>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<24>(hb, lb)); }
    if 25 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<25>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<25>(hb, lb)); }
    if 26 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<26>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<26>(hb, lb)); }
    if 27 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<27>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<27>(hb, lb)); }
    if 28 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<28>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<28>(hb, lb)); }
    if 29 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<29>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<29>(hb, lb)); }
    if 30 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<30>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<30>(hb, lb)); }
    if 31 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<31>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<31>(hb, lb)); }
    if 32 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<32>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<32>(hb, lb)); }
    if 33 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<33>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<33>(hb, lb)); }
    if 34 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<34>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<34>(hb, lb)); }
    if 35 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<35>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<35>(hb, lb)); }
    if 36 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<36>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<36>(hb, lb)); }
    if 37 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<37>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<37>(hb, lb)); }
    if 38 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<38>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<38>(hb, lb)); }
    if 39 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<39>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<39>(hb, lb)); }
    if 40 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<40>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<40>(hb, lb)); }
    if 41 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<41>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<41>(hb, lb)); }
    if 42 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<42>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<42>(hb, lb)); }
    if 43 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<43>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<43>(hb, lb)); }
    if 44 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<44>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<44>(hb, lb)); }
    if 45 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<45>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<45>(hb, lb)); }
    if 46 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<46>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<46>(hb, lb)); }
    if 47 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<47>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<47>(hb, lb)); }
    if 48 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<48>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<48>(hb, lb)); }
    if 49 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<49>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<49>(hb, lb)); }
    if 50 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<50>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<50>(hb, lb)); }
    if 51 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<51>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<51>(hb, lb)); }
    if 52 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<52>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<52>(hb, lb)); }
    if 53 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<53>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<53>(hb, lb)); }
    if 54 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<54>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<54>(hb, lb)); }
    if 55 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<55>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<55>(hb, lb)); }
    if 56 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<56>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<56>(hb, lb)); }
    if 57 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<57>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<57>(hb, lb)); }
    if 58 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<58>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<58>(hb, lb)); }
    if 59 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<59>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<59>(hb, lb)); }
    if 60 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<60>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<60>(hb, lb)); }
    if 61 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<61>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<61>(hb, lb)); }
    if 62 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<62>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<62>(hb, lb)); }
    let mut a = ba[0]; let mut b = bb[0];
    for i in 1..2 { a = _mm512_min_epu64(a, ba[i]); b = _mm512_min_epu64(b, bb[i]); }
    let mut out = [0u64; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_srl_epi64(a, count(64 - n)));
    _mm512_storeu_si512(out.as_mut_ptr().add(8).cast(), _mm512_srl_epi64(b, count(64 - n)));
    out
}

#[target_feature(enable = "avx512f,avx512vbmi2")]
unsafe fn k4(c: &[u64; 16], n: u32) -> [u64; 16] {
    let count = |s: u32| _mm_cvtsi64_si128(i64::from(s));
    let (ha, la) = doubled(_mm512_loadu_si512(c.as_ptr().cast()), n);
    let (hb, lb) = doubled(_mm512_loadu_si512(c.as_ptr().add(8).cast()), n);
    let mut ba = [ha; 4]; let mut bb = [hb; 4];
    if 1 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<1>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<1>(hb, lb)); }
    if 2 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<2>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<2>(hb, lb)); }
    if 3 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<3>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<3>(hb, lb)); }
    if 4 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<4>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<4>(hb, lb)); }
    if 5 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<5>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<5>(hb, lb)); }
    if 6 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<6>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<6>(hb, lb)); }
    if 7 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<7>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<7>(hb, lb)); }
    if 8 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<8>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<8>(hb, lb)); }
    if 9 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<9>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<9>(hb, lb)); }
    if 10 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<10>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<10>(hb, lb)); }
    if 11 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<11>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<11>(hb, lb)); }
    if 12 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<12>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<12>(hb, lb)); }
    if 13 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<13>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<13>(hb, lb)); }
    if 14 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<14>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<14>(hb, lb)); }
    if 15 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<15>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<15>(hb, lb)); }
    if 16 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<16>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<16>(hb, lb)); }
    if 17 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<17>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<17>(hb, lb)); }
    if 18 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<18>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<18>(hb, lb)); }
    if 19 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<19>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<19>(hb, lb)); }
    if 20 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<20>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<20>(hb, lb)); }
    if 21 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<21>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<21>(hb, lb)); }
    if 22 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<22>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<22>(hb, lb)); }
    if 23 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<23>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<23>(hb, lb)); }
    if 24 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<24>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<24>(hb, lb)); }
    if 25 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<25>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<25>(hb, lb)); }
    if 26 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<26>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<26>(hb, lb)); }
    if 27 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<27>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<27>(hb, lb)); }
    if 28 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<28>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<28>(hb, lb)); }
    if 29 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<29>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<29>(hb, lb)); }
    if 30 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<30>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<30>(hb, lb)); }
    if 31 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<31>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<31>(hb, lb)); }
    if 32 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<32>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<32>(hb, lb)); }
    if 33 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<33>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<33>(hb, lb)); }
    if 34 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<34>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<34>(hb, lb)); }
    if 35 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<35>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<35>(hb, lb)); }
    if 36 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<36>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<36>(hb, lb)); }
    if 37 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<37>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<37>(hb, lb)); }
    if 38 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<38>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<38>(hb, lb)); }
    if 39 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<39>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<39>(hb, lb)); }
    if 40 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<40>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<40>(hb, lb)); }
    if 41 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<41>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<41>(hb, lb)); }
    if 42 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<42>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<42>(hb, lb)); }
    if 43 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<43>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<43>(hb, lb)); }
    if 44 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<44>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<44>(hb, lb)); }
    if 45 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<45>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<45>(hb, lb)); }
    if 46 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<46>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<46>(hb, lb)); }
    if 47 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<47>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<47>(hb, lb)); }
    if 48 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<48>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<48>(hb, lb)); }
    if 49 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<49>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<49>(hb, lb)); }
    if 50 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<50>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<50>(hb, lb)); }
    if 51 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<51>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<51>(hb, lb)); }
    if 52 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<52>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<52>(hb, lb)); }
    if 53 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<53>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<53>(hb, lb)); }
    if 54 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<54>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<54>(hb, lb)); }
    if 55 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<55>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<55>(hb, lb)); }
    if 56 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<56>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<56>(hb, lb)); }
    if 57 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<57>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<57>(hb, lb)); }
    if 58 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<58>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<58>(hb, lb)); }
    if 59 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<59>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<59>(hb, lb)); }
    if 60 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<60>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<60>(hb, lb)); }
    if 61 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<61>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<61>(hb, lb)); }
    if 62 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<62>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<62>(hb, lb)); }
    let mut a = ba[0]; let mut b = bb[0];
    for i in 1..4 { a = _mm512_min_epu64(a, ba[i]); b = _mm512_min_epu64(b, bb[i]); }
    let mut out = [0u64; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_srl_epi64(a, count(64 - n)));
    _mm512_storeu_si512(out.as_mut_ptr().add(8).cast(), _mm512_srl_epi64(b, count(64 - n)));
    out
}

#[target_feature(enable = "avx512f,avx512vbmi2")]
unsafe fn k8(c: &[u64; 16], n: u32) -> [u64; 16] {
    let count = |s: u32| _mm_cvtsi64_si128(i64::from(s));
    let (ha, la) = doubled(_mm512_loadu_si512(c.as_ptr().cast()), n);
    let (hb, lb) = doubled(_mm512_loadu_si512(c.as_ptr().add(8).cast()), n);
    let mut ba = [ha; 8]; let mut bb = [hb; 8];
    if 1 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<1>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<1>(hb, lb)); }
    if 2 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<2>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<2>(hb, lb)); }
    if 3 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<3>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<3>(hb, lb)); }
    if 4 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<4>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<4>(hb, lb)); }
    if 5 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<5>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<5>(hb, lb)); }
    if 6 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<6>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<6>(hb, lb)); }
    if 7 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<7>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<7>(hb, lb)); }
    if 8 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<8>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<8>(hb, lb)); }
    if 9 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<9>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<9>(hb, lb)); }
    if 10 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<10>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<10>(hb, lb)); }
    if 11 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<11>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<11>(hb, lb)); }
    if 12 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<12>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<12>(hb, lb)); }
    if 13 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<13>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<13>(hb, lb)); }
    if 14 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<14>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<14>(hb, lb)); }
    if 15 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<15>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<15>(hb, lb)); }
    if 16 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<16>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<16>(hb, lb)); }
    if 17 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<17>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<17>(hb, lb)); }
    if 18 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<18>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<18>(hb, lb)); }
    if 19 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<19>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<19>(hb, lb)); }
    if 20 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<20>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<20>(hb, lb)); }
    if 21 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<21>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<21>(hb, lb)); }
    if 22 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<22>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<22>(hb, lb)); }
    if 23 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<23>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<23>(hb, lb)); }
    if 24 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<24>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<24>(hb, lb)); }
    if 25 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<25>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<25>(hb, lb)); }
    if 26 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<26>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<26>(hb, lb)); }
    if 27 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<27>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<27>(hb, lb)); }
    if 28 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<28>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<28>(hb, lb)); }
    if 29 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<29>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<29>(hb, lb)); }
    if 30 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<30>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<30>(hb, lb)); }
    if 31 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<31>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<31>(hb, lb)); }
    if 32 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<32>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<32>(hb, lb)); }
    if 33 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<33>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<33>(hb, lb)); }
    if 34 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<34>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<34>(hb, lb)); }
    if 35 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<35>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<35>(hb, lb)); }
    if 36 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<36>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<36>(hb, lb)); }
    if 37 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<37>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<37>(hb, lb)); }
    if 38 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<38>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<38>(hb, lb)); }
    if 39 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<39>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<39>(hb, lb)); }
    if 40 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<40>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<40>(hb, lb)); }
    if 41 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<41>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<41>(hb, lb)); }
    if 42 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<42>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<42>(hb, lb)); }
    if 43 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<43>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<43>(hb, lb)); }
    if 44 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<44>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<44>(hb, lb)); }
    if 45 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<45>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<45>(hb, lb)); }
    if 46 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<46>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<46>(hb, lb)); }
    if 47 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<47>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<47>(hb, lb)); }
    if 48 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<48>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<48>(hb, lb)); }
    if 49 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<49>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<49>(hb, lb)); }
    if 50 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<50>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<50>(hb, lb)); }
    if 51 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<51>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<51>(hb, lb)); }
    if 52 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<52>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<52>(hb, lb)); }
    if 53 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<53>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<53>(hb, lb)); }
    if 54 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<54>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<54>(hb, lb)); }
    if 55 < n { ba[7] = _mm512_min_epu64(ba[7], _mm512_shldi_epi64::<55>(ha, la)); bb[7] = _mm512_min_epu64(bb[7], _mm512_shldi_epi64::<55>(hb, lb)); }
    if 56 < n { ba[0] = _mm512_min_epu64(ba[0], _mm512_shldi_epi64::<56>(ha, la)); bb[0] = _mm512_min_epu64(bb[0], _mm512_shldi_epi64::<56>(hb, lb)); }
    if 57 < n { ba[1] = _mm512_min_epu64(ba[1], _mm512_shldi_epi64::<57>(ha, la)); bb[1] = _mm512_min_epu64(bb[1], _mm512_shldi_epi64::<57>(hb, lb)); }
    if 58 < n { ba[2] = _mm512_min_epu64(ba[2], _mm512_shldi_epi64::<58>(ha, la)); bb[2] = _mm512_min_epu64(bb[2], _mm512_shldi_epi64::<58>(hb, lb)); }
    if 59 < n { ba[3] = _mm512_min_epu64(ba[3], _mm512_shldi_epi64::<59>(ha, la)); bb[3] = _mm512_min_epu64(bb[3], _mm512_shldi_epi64::<59>(hb, lb)); }
    if 60 < n { ba[4] = _mm512_min_epu64(ba[4], _mm512_shldi_epi64::<60>(ha, la)); bb[4] = _mm512_min_epu64(bb[4], _mm512_shldi_epi64::<60>(hb, lb)); }
    if 61 < n { ba[5] = _mm512_min_epu64(ba[5], _mm512_shldi_epi64::<61>(ha, la)); bb[5] = _mm512_min_epu64(bb[5], _mm512_shldi_epi64::<61>(hb, lb)); }
    if 62 < n { ba[6] = _mm512_min_epu64(ba[6], _mm512_shldi_epi64::<62>(ha, la)); bb[6] = _mm512_min_epu64(bb[6], _mm512_shldi_epi64::<62>(hb, lb)); }
    let mut a = ba[0]; let mut b = bb[0];
    for i in 1..8 { a = _mm512_min_epu64(a, ba[i]); b = _mm512_min_epu64(b, bb[i]); }
    let mut out = [0u64; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_srl_epi64(a, count(64 - n)));
    _mm512_storeu_si512(out.as_mut_ptr().add(8).cast(), _mm512_srl_epi64(b, count(64 - n)));
    out
}

fn main() {
    let n: u32 = std::env::args().nth(1).map_or(61, |v| v.parse().unwrap());
    let mask = (1u64 << n) - 1;
    let mut x = 0x9e3779b97f4a7c15u64;
    let words: Vec<[u64; 16]> = (0..200_000).map(|_| std::array::from_fn(|_| { x ^= x << 13; x ^= x >> 7; x ^= x << 17; x & mask })).collect();
    let fs: [(&str, unsafe fn(&[u64; 16], u32) -> [u64; 16]); 4] = [("1 chain", k1), ("2 chains", k2), ("4 chains", k4), ("8 chains", k8)];
    let refs: Vec<[u64; 16]> = words.iter().map(|w| unsafe { k1(w, n) }).collect();
    for _ in 0..3 { for (name, f) in fs {
        let t = Instant::now(); let mut acc = 0u64;
        for _ in 0..5 { for w in &words { let o = unsafe { f(w, n) }; acc ^= o[0] ^ o[15]; } }
        let ns = t.elapsed().as_secs_f64() * 1e9 / (5.0 * 16.0 * words.len() as f64);
        for (w, r) in words.iter().take(1000).zip(&refs) { assert_eq!(unsafe { f(w, n) }, *r); }
        println!("n {n} {name:9} {ns:.3} ns/key ({acc:x})");
    } }
}
