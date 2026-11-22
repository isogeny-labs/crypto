//! AVX-512 IFMA lockstep for `rootfind-kmitm` right=1 probes (PLAN lever 2).
//!
//! Eight independent targets share one IFMA Montgomery walk over the factor
//! base (R = 2^52). Candidate abscissae are returned in **canonical** form
//! so the caller can Bloom/lookup a prepare-time map keyed by affine `x`
//! without a mont conversion on the miss path.
//!
//! Sign convention matches `probe_right1_prime_batch`:
//! `dx = x_j − x_R`, `λ₊ = (y_j − y_R)/dx`, `λ₋ = −(y_j + y_R)/dx`,
//! then `x₊ = λ₊² − x_R − x_j` (pairs with `neg_index[j]`) and
//! `x₋ = λ₋² − x_R − x_j` (pairs with `j`).
//!
//! Refused without `avx512f`+`avx512ifma` or for `p ≥ 2^52`. Engineering
//! wall only — charged `S` is unchanged.

#![cfg(target_arch = "x86_64")]

use crate::cryptanalysis::ic_boundary::PrimePoint;
use std::arch::x86_64::*;

pub const LANES: usize = 8;
const M52: u64 = (1 << 52) - 1;

/// IFMA-resident factor-base walk for right=1 kmitm probes.
pub struct IfmaRight1 {
    p: u64,
    pinv: u64,
    r: u64,
    r2: u64,
    /// Factor-base abscissae / ordinates in Montgomery form (`·R mod p`).
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

pub fn has_ifma() -> bool {
    std::arch::is_x86_feature_detected!("avx512f")
        && std::arch::is_x86_feature_detected!("avx512ifma")
}

impl IfmaRight1 {
    /// `None` without IFMA or for `p ≥ 2^52`. `xs`/`ys` are canonical.
    pub fn new(p: u64, xs: &[u64], ys: &[u64]) -> Option<Self> {
        if !has_ifma() || !(5..1 << 52).contains(&p) || p & 1 == 0 {
            return None;
        }
        debug_assert_eq!(xs.len(), ys.len());
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        let r = ((1u128 << 52) % p as u128) as u64;
        let r2 = mulmod(r, r, p);
        Some(Self {
            p,
            pinv: inv.wrapping_neg() & M52,
            r,
            r2,
            xs: xs.iter().map(|&x| mulmod(x, r, p)).collect(),
            ys: ys.iter().map(|&y| mulmod(y, r, p)).collect(),
        })
    }

    /// For each live lane `l < n_live`, call `on_pair(l, j, x_plus, x_minus)`
    /// with canonical abscissae of `R±F_j` (**kmitm** dx = `x_j − x_R`).
    /// Returns lane-Montgomery mul count (each vector `mm` counts `n_live`).
    pub fn for_each_sibling_pair(
        &self,
        pts: &[PrimePoint],
        prefix: &mut Vec<[u64; LANES]>,
        on_pair: impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        self.for_each_pair(pts, prefix, false, on_pair)
    }

    /// Floor / subtract convention: dx = `x_R − x_i`, callback
    /// `(lane, j, x_minus, x_plus)` matching `rootfind-floor`'s
    /// `(rep.plus, x_minus)` / `(rep.minus, x_plus)`.
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
            xr[l] = mulmod(q.x, self.r, self.p);
            yr[l] = mulmod(q.y, self.r, self.p);
        }
        // SAFETY: `new` returned `Some` only where the CPU has IFMA.
        unsafe { self.sweep_avx(&xr, &yr, n_live, floor_dx, prefix, &mut on_pair) }
    }

    /// Scalar Montgomery product, `R = 2^52` (same reduction as the vector `mm`).
    #[inline(always)]
    fn mont_mul(&self, a: u64, b: u64) -> u64 {
        let t = a as u128 * b as u128;
        let lo = (t as u64) & M52;
        let hi = (t >> 52) as u64;
        let m = lo.wrapping_mul(self.pinv) & M52;
        let mp = m as u128 * self.p as u128;
        let carry = (lo as u128 + (mp & M52 as u128)) >> 52;
        let u = hi
            .wrapping_add((mp >> 52) as u64)
            .wrapping_add(carry as u64);
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }

    /// Batch `a⁻¹·R` from eight `a·R` values: one Fermat of the product,
    /// Montgomery trick for the rest (was eight independent Fermats).
    fn inv_lanes(&self, mut v: [u64; LANES]) -> [u64; LANES] {
        let mut scratch = [0u64; LANES];
        let mut acc = self.r; // Montgomery 1
        for i in 0..LANES {
            scratch[i] = acc;
            acc = self.mont_mul(acc, v[i]);
        }
        // Same closed form as the previous per-lane Fermat, on the product.
        let mut inv_acc = mulmod(self.r2, powmod(acc, self.p - 2, self.p), self.p);
        for i in (0..LANES).rev() {
            let xi = v[i];
            v[i] = self.mont_mul(inv_acc, scratch[i]);
            inv_acc = self.mont_mul(inv_acc, xi);
        }
        v
    }

    #[target_feature(enable = "avx512f,avx512ifma")]
    unsafe fn sweep_avx(
        &self,
        xr: &[u64; LANES],
        yr: &[u64; LANES],
        n_live: usize,
        floor_dx: bool,
        prefix: &mut Vec<[u64; LANES]>,
        on_pair: &mut impl FnMut(usize, usize, u64, u64),
    ) -> u64 {
        let ld = |a: &[u64; LANES]| _mm512_loadu_si512(a.as_ptr() as *const _);
        let st = |v: __m512i| {
            let mut a = [0u64; LANES];
            _mm512_storeu_si512(a.as_mut_ptr() as *mut _, v);
            a
        };
        let p = _mm512_set1_epi64(self.p as i64);
        let pinv = _mm512_set1_epi64(self.pinv as i64);
        let z = _mm512_setzero_si512();
        let one_r = _mm512_set1_epi64(self.r as i64);
        let one = _mm512_set1_epi64(1);
        let live = n_live as u64;
        let mut vec_muls = 0u64;
        let mut mm = |a: __m512i, b: __m512i| {
            vec_muls += live;
            let lo = _mm512_madd52lo_epu64(z, a, b);
            let hi = _mm512_madd52hi_epu64(z, a, b);
            let m = _mm512_madd52lo_epu64(z, lo, pinv);
            let c = _mm512_srli_epi64::<52>(_mm512_madd52lo_epu64(lo, m, p));
            let u = _mm512_add_epi64(_mm512_madd52hi_epu64(hi, m, p), c);
            _mm512_min_epu64(u, _mm512_sub_epi64(u, p))
        };
        let add = |a: __m512i, b: __m512i| {
            let s = _mm512_add_epi64(a, b);
            _mm512_min_epu64(s, _mm512_sub_epi64(s, p))
        };
        let sub = |a: __m512i, b: __m512i| {
            let d = _mm512_sub_epi64(a, b);
            _mm512_min_epu64(d, _mm512_add_epi64(d, p))
        };
        let vxr = ld(xr);
        let vyr = ld(yr);
        let k = self.xs.len();
        // kmitm: dx = x_j − x_R; floor: dx = x_R − x_j.
        let diff = |j: usize| {
            let xj = _mm512_set1_epi64(self.xs[j] as i64);
            let d = if floor_dx {
                sub(vxr, xj)
            } else {
                sub(xj, vxr)
            };
            let zm = _mm512_cmpeq_epi64_mask(d, z);
            (_mm512_mask_blend_epi64(zm, d, one_r), zm)
        };
        prefix.clear();
        prefix.reserve(k);
        let mut acc = one_r;
        for j in 0..k {
            acc = mm(acc, diff(j).0);
            prefix.push(st(acc));
        }
        let inv_arr = self.inv_lanes(st(acc));
        let fermat_muls = 2 * LANES as u64 + ((64 - self.p.leading_zeros()) as u64 * 3 / 2);
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
            let xj = _mm512_set1_epi64(self.xs[j] as i64);
            let yj = _mm512_set1_epi64(self.ys[j] as i64);
            let inv_j = ld(&prefix[j]);
            let base = add(vxr, xj);
            let (a0, a1) = if floor_dx {
                // lam_minus = (y_R + y_i)/dx → x_minus; lam_plus = (y_R − y_i)/dx → x_plus
                let lm = mm(add(vyr, yj), inv_j);
                let lp = mm(sub(vyr, yj), inv_j);
                let lm2 = mm(lm, lm);
                let lp2 = mm(lp, lp);
                (st(mm(sub(lm2, base), one)), st(mm(sub(lp2, base), one)))
            } else {
                // λ₊ = (y_j − y_R)/dx → x₊; λ₋ = −(y_j + y_R)/dx → x₋
                let lp = mm(sub(yj, vyr), inv_j);
                let lm = mm(sub(z, add(yj, vyr)), inv_j);
                let lp2 = mm(lp, lp);
                let lm2 = mm(lm, lm);
                (st(mm(sub(lp2, base), one)), st(mm(sub(lm2, base), one)))
            };
            for l in 0..n_live {
                if zm & (1 << l) != 0 {
                    continue;
                }
                on_pair(l, j, a0[l], a1[l]);
            }
        }
        vec_muls + fermat_muls
    }
}
