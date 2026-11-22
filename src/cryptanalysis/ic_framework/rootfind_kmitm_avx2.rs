//! AVX2 4-wide Montgomery lockstep for `rootfind-kmitm` / floor right=1
//! probes when AVX-512 IFMA is absent (PLAN lever 2 fallback).
//!
//! Uses `R = 2^32` Montgomery arithmetic in four `u64` lanes (low 32 bits
//! live) via `_mm256_mul_epu32`. Restricted to `p < 2^31` so `a·b + m·p`
//! fits in 64 bits without a 128-bit add. Refused without AVX2 or for
//! larger `p`. Sign conventions match [`super::rootfind_kmitm_ifma::IfmaRight1`].
//! Engineering wall only — charged `S` unchanged.

#![cfg(target_arch = "x86_64")]

use crate::cryptanalysis::ic_boundary::PrimePoint;
use std::arch::x86_64::*;

pub const LANES: usize = 4;
const M32: u64 = 0xffff_ffff;

/// AVX2-resident factor-base walk for right=1 probes (`p < 2^32`).
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
        // p < 2^31 keeps a·b + m·p inside u64 for the vector reduction.
        if !has_avx2() || !(5..1u64 << 31).contains(&p) || p & 1 == 0 {
            return None;
        }
        debug_assert_eq!(xs.len(), ys.len());
        // −p⁻¹ mod 2^32
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

    pub fn for_each_floor_pair(
        &self,
        pts: &[PrimePoint],
        prefix: &mut Vec<[u64; LANES]>,
        on_pair: impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        self.for_each_pair(pts, prefix, true, on_pair)
    }

    fn for_each_pair(
        &self,
        pts: &[PrimePoint],
        prefix: &mut Vec<[u64; LANES]>,
        floor_dx: bool,
        mut on_pair: impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        debug_assert!(!pts.is_empty() && pts.len() <= LANES);
        let n_live = pts.len();
        let mut xr = [0u64; LANES];
        let mut yr = [0u64; LANES];
        for l in 0..LANES {
            let q = pts[l.min(n_live - 1)];
            xr[l] = mulmod(q.x % self.p, self.r, self.p);
            yr[l] = mulmod(q.y % self.p, self.r, self.p);
        }
        unsafe { self.sweep_avx2(&xr, &yr, n_live, floor_dx, prefix, &mut on_pair) }
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
    unsafe fn sweep_avx2(
        &self,
        xr: &[u64; LANES],
        yr: &[u64; LANES],
        n_live: usize,
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
        let live = n_live as u64;
        let mut vec_muls = 0u64;
        // Montgomery product, R = 2^32; inputs/outputs in [0, p) in low 32 bits.
        let mut mm = |a: __m256i, b: __m256i| {
            vec_muls += live;
            // a,b < 2^31 → t < 2^62; m·p < 2^62; sum < 2^63 (fits u64).
            let t = _mm256_mul_epu32(a, b);
            let m = _mm256_and_si256(_mm256_mul_epu32(t, pinv), mask32);
            let mp = _mm256_mul_epu32(m, p);
            let u = _mm256_srli_epi64(_mm256_add_epi64(t, mp), 32);
            // u < 2p; conditional sub
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
        let vxr = ld(xr);
        let vyr = ld(yr);
        let k = self.xs.len();
        let diff = |j: usize| {
            let xj = _mm256_set1_epi64x(self.xs[j] as i64);
            let d = if floor_dx {
                sub(vxr, xj)
            } else {
                sub(xj, vxr)
            };
            let zm = _mm256_cmpeq_epi64(d, z);
            (_mm256_blendv_epi8(d, one_r, zm), zm)
        };
        prefix.clear();
        prefix.reserve(k);
        let mut acc = one_r;
        for j in 0..k {
            acc = mm(acc, diff(j).0);
            prefix.push(st(acc));
        }
        let inv_arr = self.inv_lanes(st(acc));
        let fermat_muls = 2 * LANES as u64 + ((32 - self.p.leading_zeros()) as u64 * 3 / 2);
        let mut inv = ld(&inv_arr);
        for j in (1..k).rev() {
            let inv_j = mm(inv, ld(&prefix[j - 1]));
            inv = mm(inv, diff(j).0);
            prefix[j] = st(inv_j);
        }
        if k > 0 {
            prefix[0] = st(inv);
        }
        for j in 0..k {
            let (_, zm) = diff(j);
            let xj = _mm256_set1_epi64x(self.xs[j] as i64);
            let yj = _mm256_set1_epi64x(self.ys[j] as i64);
            let inv_j = ld(&prefix[j]);
            let base = add(vxr, xj);
            let (a0, a1) = if floor_dx {
                let lm = mm(add(vyr, yj), inv_j);
                let lp = mm(sub(vyr, yj), inv_j);
                let lm2 = mm(lm, lm);
                let lp2 = mm(lp, lp);
                (
                    st(mm(sub(lm2, base), one)),
                    st(mm(sub(lp2, base), one)),
                )
            } else {
                let lp = mm(sub(yj, vyr), inv_j);
                let lm = mm(sub(z, add(yj, vyr)), inv_j);
                let lp2 = mm(lp, lp);
                let lm2 = mm(lm, lm);
                (
                    st(mm(sub(lp2, base), one)),
                    st(mm(sub(lm2, base), one)),
                )
            };
            // zm is all-ones lanes where dx==0
            let mut zm_bits = [0u64; LANES];
            _mm256_storeu_si256(zm_bits.as_mut_ptr() as *mut __m256i, zm);
            for l in 0..n_live {
                if zm_bits[l] != 0 {
                    continue;
                }
                on_pair(l, j, a0[l], a1[l]);
            }
        }
        vec_muls + fermat_muls
    }
}
