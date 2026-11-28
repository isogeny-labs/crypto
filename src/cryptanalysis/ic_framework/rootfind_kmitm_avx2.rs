//! AVX2 dual-pack Montgomery lockstep for `rootfind-kmitm` / floor right=1
//! probes when AVX-512 IFMA is absent (PLAN lever 2 fallback).
//!
//! Two `__m256i` packs (4+4) share one walk over the factor base with
//! `R = 2^32` Montgomery via `_mm256_mul_epu32`. Restricted to `p < 2^31`
//! so `a·b + m·p` fits in 64 bits. Refused without AVX2 or for larger `p`.
//! Sign conventions match [`super::rootfind_kmitm_ifma::IfmaRight1`].
//! Engineering wall only — charged `S` unchanged.

#![cfg(target_arch = "x86_64")]

use crate::cryptanalysis::ic_boundary::PrimePoint;
use std::arch::x86_64::*;

pub const LANES: usize = 4;
pub const MAX_TARGETS: usize = 8;
const M32: u64 = 0xffff_ffff;

/// AVX2-resident factor-base walk for right=1 probes (`p < 2^31`).
pub struct Avx2Right1 {
    p: u64,
    pinv: u64,
    r: u64,
    r2: u64,
    xs: Vec<u64>,
    ys: Vec<u64>,
}

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

pub fn has_avx2() -> bool {
    std::arch::is_x86_feature_detected!("avx2")
}

impl Avx2Right1 {
    /// `None` without AVX2 or for `p ≥ 2^31`. `xs`/`ys` are canonical.
    pub fn new(p: u64, xs: &[u64], ys: &[u64]) -> Option<Self> {
        if !has_avx2() || !(5..1u64 << 31).contains(&p) || p & 1 == 0 {
            return None;
        }
        debug_assert_eq!(xs.len(), ys.len());
        let mut inv = 1u64;
        for _ in 0..5 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv))) & M32;
        }
        let r = ((1u128 << 32) % p as u128) as u64;
        let r2 = mulmod(r, r, p);
        Some(Self {
            p,
            pinv: inv.wrapping_neg() & M32,
            r,
            r2,
            xs: xs.iter().map(|&x| mulmod(x % p, r, p)).collect(),
            ys: ys.iter().map(|&y| mulmod(y % p, r, p)).collect(),
        })
    }

    pub fn for_each_sibling_pair(
        &self,
        pts: &[PrimePoint],
        prefix: &mut Vec<[u64; LANES]>,
        on_pair: impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        self.for_each_pair(pts, prefix, false, on_pair)
    }

    fn for_each_pair(
        &self,
        pts: &[PrimePoint],
        prefix: &mut Vec<[u64; LANES]>,
        floor_dx: bool,
        mut on_pair: impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        debug_assert!(!pts.is_empty() && pts.len() <= MAX_TARGETS);
        let n_live = pts.len();
        // Pack A: lanes 0..3, pack B: lanes 4..7 (padded with last point).
        let mut xr_a = [0u64; LANES];
        let mut yr_a = [0u64; LANES];
        let mut xr_b = [0u64; LANES];
        let mut yr_b = [0u64; LANES];
        for l in 0..LANES {
            let qa = pts[l.min(n_live - 1)];
            xr_a[l] = mulmod(qa.x % self.p, self.r, self.p);
            yr_a[l] = mulmod(qa.y % self.p, self.r, self.p);
            let qb = if n_live > LANES {
                pts[(LANES + l).min(n_live - 1)]
            } else {
                qa
            };
            xr_b[l] = mulmod(qb.x % self.p, self.r, self.p);
            yr_b[l] = mulmod(qb.y % self.p, self.r, self.p);
        }
        let dual = n_live > LANES;
        unsafe {
            self.sweep_dual(
                &xr_a,
                &yr_a,
                &xr_b,
                &yr_b,
                n_live,
                dual,
                floor_dx,
                prefix,
                &mut on_pair,
            )
        }
    }

    #[inline(always)]
    fn mont_mul(&self, a: u64, b: u64) -> u64 {
        let t = a as u128 * b as u128;
        let m = ((t as u64).wrapping_mul(self.pinv)) & M32;
        let u = ((t + m as u128 * self.p as u128) >> 32) as u64;
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }

    fn inv_lanes(&self, mut v: [u64; LANES]) -> [u64; LANES] {
        let mut scratch = [0u64; LANES];
        let mut acc = self.r;
        for i in 0..LANES {
            scratch[i] = acc;
            acc = self.mont_mul(acc, v[i]);
        }
        let mut inv_acc = mulmod(self.r2, powmod(acc, self.p - 2, self.p), self.p);
        for i in (0..LANES).rev() {
            let xi = v[i];
            v[i] = self.mont_mul(inv_acc, scratch[i]);
            inv_acc = self.mont_mul(inv_acc, xi);
        }
        v
    }

    #[target_feature(enable = "avx2")]
    unsafe fn sweep_dual(
        &self,
        xr_a: &[u64; LANES],
        yr_a: &[u64; LANES],
        xr_b: &[u64; LANES],
        yr_b: &[u64; LANES],
        n_live: usize,
        dual: bool,
        floor_dx: bool,
        prefix: &mut Vec<[u64; LANES]>,
        on_pair: &mut impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        let ld = |a: &[u64; LANES]| _mm256_loadu_si256(a.as_ptr() as *const __m256i);
        let st = |v: __m256i| {
            let mut a = [0u64; LANES];
            _mm256_storeu_si256(a.as_mut_ptr() as *mut __m256i, v);
            a
        };
        let p = _mm256_set1_epi64x(self.p as i64);
        let pinv = _mm256_set1_epi64x(self.pinv as i64);
        let z = _mm256_setzero_si256();
        let one_r = _mm256_set1_epi64x(self.r as i64);
        let one = _mm256_set1_epi64x(1);
        let mask32 = _mm256_set1_epi64x(M32 as i64);
        let live_a = n_live.min(LANES) as u64;
        let live_b = n_live.saturating_sub(LANES) as u64;
        let mut vec_muls = 0u64;
        let mut mm = |a: __m256i, b: __m256i, live: u64| {
            vec_muls += live;
            let t = _mm256_mul_epu32(a, b);
            let m = _mm256_and_si256(_mm256_mul_epu32(t, pinv), mask32);
            let mp = _mm256_mul_epu32(m, p);
            let u = _mm256_srli_epi64(_mm256_add_epi64(t, mp), 32);
            // unsigned u >= p ? u - p : u  (values < 2^31)
            let too = _mm256_cmpgt_epi64(u, _mm256_sub_epi64(p, _mm256_set1_epi64x(1)));
            _mm256_blendv_epi8(u, _mm256_sub_epi64(u, p), too)
        };
        let add = |a: __m256i, b: __m256i| {
            let s = _mm256_add_epi64(a, b);
            let too = _mm256_cmpgt_epi64(s, _mm256_sub_epi64(p, _mm256_set1_epi64x(1)));
            _mm256_blendv_epi8(s, _mm256_sub_epi64(s, p), too)
        };
        let sub = |a: __m256i, b: __m256i| {
            let d = _mm256_sub_epi64(a, b);
            let borrow = _mm256_cmpgt_epi64(b, a);
            _mm256_blendv_epi8(d, _mm256_add_epi64(d, p), borrow)
        };
        let vxr_a = ld(xr_a);
        let vyr_a = ld(yr_a);
        let vxr_b = ld(xr_b);
        let vyr_b = ld(yr_b);
        let k = self.xs.len();
        // prefix layout: for each j, store pack-A then (if dual) we keep pack-B
        // prefix in a parallel vec via interleaved pairs in `prefix` as
        // [acc_a_j] only for pack A; pack B uses `prefix_b` local.
        let mut prefix_b: Vec<[u64; LANES]> = Vec::new();
        if dual {
            prefix_b.reserve(k);
        }
        prefix.clear();
        prefix.reserve(k);
        let mut acc_a = one_r;
        let mut acc_b = one_r;
        for j in 0..k {
            let xj = _mm256_set1_epi64x(self.xs[j] as i64);
            let d_a = if floor_dx {
                sub(vxr_a, xj)
            } else {
                sub(xj, vxr_a)
            };
            let zm_a = _mm256_cmpeq_epi64(d_a, z);
            let d_a = _mm256_blendv_epi8(d_a, one_r, zm_a);
            acc_a = mm(acc_a, d_a, live_a);
            prefix.push(st(acc_a));
            if dual {
                let d_b = if floor_dx {
                    sub(vxr_b, xj)
                } else {
                    sub(xj, vxr_b)
                };
                let zm_b = _mm256_cmpeq_epi64(d_b, z);
                let d_b = _mm256_blendv_epi8(d_b, one_r, zm_b);
                acc_b = mm(acc_b, d_b, live_b);
                prefix_b.push(st(acc_b));
            }
        }
        let inv_a = self.inv_lanes(st(acc_a));
        let inv_b = if dual {
            self.inv_lanes(st(acc_b))
        } else {
            [0u64; LANES]
        };
        let fermat_muls =
            (1 + dual as u64) * (2 * LANES as u64 + ((32 - self.p.leading_zeros()) as u64 * 3 / 2));
        let mut inv_va = ld(&inv_a);
        let mut inv_vb = if dual { ld(&inv_b) } else { z };
        for j in (1..k).rev() {
            let xj = _mm256_set1_epi64x(self.xs[j] as i64);
            let d_a = {
                let d = if floor_dx {
                    sub(vxr_a, xj)
                } else {
                    sub(xj, vxr_a)
                };
                let zm = _mm256_cmpeq_epi64(d, z);
                _mm256_blendv_epi8(d, one_r, zm)
            };
            let inv_j_a = mm(inv_va, ld(&prefix[j - 1]), live_a);
            inv_va = mm(inv_va, d_a, live_a);
            prefix[j] = st(inv_j_a);
            if dual {
                let d_b = {
                    let d = if floor_dx {
                        sub(vxr_b, xj)
                    } else {
                        sub(xj, vxr_b)
                    };
                    let zm = _mm256_cmpeq_epi64(d, z);
                    _mm256_blendv_epi8(d, one_r, zm)
                };
                let inv_j_b = mm(inv_vb, ld(&prefix_b[j - 1]), live_b);
                inv_vb = mm(inv_vb, d_b, live_b);
                prefix_b[j] = st(inv_j_b);
            }
        }
        if k > 0 {
            prefix[0] = st(inv_va);
            if dual {
                prefix_b[0] = st(inv_vb);
            }
        }
        for j in 0..k {
            let xj = _mm256_set1_epi64x(self.xs[j] as i64);
            let yj = _mm256_set1_epi64x(self.ys[j] as i64);
            // Pack A
            {
                let d = if floor_dx {
                    sub(vxr_a, xj)
                } else {
                    sub(xj, vxr_a)
                };
                let zm = _mm256_cmpeq_epi64(d, z);
                let inv_j = ld(&prefix[j]);
                let base = add(vxr_a, xj);
                let (a0, a1) = if floor_dx {
                    let lm = mm(add(vyr_a, yj), inv_j, live_a);
                    let lp = mm(sub(vyr_a, yj), inv_j, live_a);
                    let lm2 = mm(lm, lm, live_a);
                    let lp2 = mm(lp, lp, live_a);
                    (
                        st(mm(sub(lm2, base), one, live_a)),
                        st(mm(sub(lp2, base), one, live_a)),
                    )
                } else {
                    let lp = mm(sub(yj, vyr_a), inv_j, live_a);
                    let lm = mm(sub(z, add(vyr_a, yj)), inv_j, live_a);
                    let lp2 = mm(lp, lp, live_a);
                    let lm2 = mm(lm, lm, live_a);
                    (
                        st(mm(sub(lp2, base), one, live_a)),
                        st(mm(sub(lm2, base), one, live_a)),
                    )
                };
                let mut zm_bits = [0u64; LANES];
                _mm256_storeu_si256(zm_bits.as_mut_ptr() as *mut __m256i, zm);
                let n_a = n_live.min(LANES);
                for l in 0..n_a {
                    if zm_bits[l] != 0 {
                        continue;
                    }
                    on_pair(l, j, a0[l], a1[l]);
                }
            }
            if dual {
                let d = if floor_dx {
                    sub(vxr_b, xj)
                } else {
                    sub(xj, vxr_b)
                };
                let zm = _mm256_cmpeq_epi64(d, z);
                let inv_j = ld(&prefix_b[j]);
                let base = add(vxr_b, xj);
                let (a0, a1) = if floor_dx {
                    let lm = mm(add(vyr_b, yj), inv_j, live_b);
                    let lp = mm(sub(vyr_b, yj), inv_j, live_b);
                    let lm2 = mm(lm, lm, live_b);
                    let lp2 = mm(lp, lp, live_b);
                    (
                        st(mm(sub(lm2, base), one, live_b)),
                        st(mm(sub(lp2, base), one, live_b)),
                    )
                } else {
                    let lp = mm(sub(yj, vyr_b), inv_j, live_b);
                    let lm = mm(sub(z, add(vyr_b, yj)), inv_j, live_b);
                    let lp2 = mm(lp, lp, live_b);
                    let lm2 = mm(lm, lm, live_b);
                    (
                        st(mm(sub(lp2, base), one, live_b)),
                        st(mm(sub(lm2, base), one, live_b)),
                    )
                };
                let mut zm_bits = [0u64; LANES];
                _mm256_storeu_si256(zm_bits.as_mut_ptr() as *mut __m256i, zm);
                for l in 0..(n_live - LANES) {
                    if zm_bits[l] != 0 {
                        continue;
                    }
                    on_pair(LANES + l, j, a0[l], a1[l]);
                }
            }
        }
        vec_muls + fermat_muls
    }
}
