//! Matched multi-thread Pollard rho (lead: matched rho reference).
//!
//! Parallel distinguished-point rho (van Oorschot-Wiener) on E(K4)[r] with an
//! r-adding walk shared by every thread, optional batched affine walks with
//! Montgomery's simultaneous inversion, optional negation map with fruitless
//! cycle detection and escape-by-doubling, and optional fast (u64 Barrett)
//! K4 arithmetic that returns exactly the same field elements as K4::mul/inv.
//!
//! Activated only by HUMBERT_RHO_THREADS / _BATCH / _NEG / _FASTMUL / _DPBITS / _NB
//! (and the newer _SIMD / _CAP / _PROF); with all of them unset the crate's scalar
//! `curve::pollard_rho` runs as before. Every walk step and every other group
//! operation is counted.
//!
//! HUMBERT_RHO_SIMD (1|auto|avx512|avx2|portable): the batched walk step runs in
//! a lane kernel, LANES walks at once in f64 lanes (exact for p < 2^17, AVX-512 /
//! AVX2+FMA by runtime detection, portable fallback). Same walk, same start
//! sequence, same negation map and DP rule: with one thread it takes exactly the
//! steps of the per-walk path. The batch is rounded up to a multiple of LANES.
//! HUMBERT_RHO_CAP: total step cap (default 2e9). HUMBERT_RHO_PROF=1 (with SIMD):
//! per-phase ns per step on stderr. rho_par needs p < 2^17; FASTMUL (Barrett) is
//! exact there, so it always runs when requested.
//!
//! p < 2^17 (was 2^16). The walk hash, DP test, fruitless-cycle test and escape
//! choice read `pk(x)` = x0 + x1 2^16 + x2 2^32 + x3 2^48 mod 2^64, which is the
//! old injective 16-bit packing for p < 2^16 and a (non-injective) hash above it.
//! Everything that needs the point itself uses the injective `fullkey` (32 bits per
//! coefficient, u128): the DP table key and the stored y. The canonical sign of y
//! compares the highest nonzero coefficient (the order pk induced for p < 2^16).
//! So every q < 2^16 walk, DP and collision is unchanged.
use crate::curve::{mulr, invr, Curve, Pt, Rng};
use crate::field::{FieldOps, E4, K4};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ParCfg {
    pub threads: usize,
    pub batch: usize,
    pub neg: bool,
    pub fast: bool,
    /// distinguished-point bits; None = automatic
    pub dp_bits: Option<u32>,
    /// r-adding table size (power of two); None = 256 (2048 with negation)
    pub nb: Option<usize>,
    /// total walk-step cap
    pub cap: u64,
    /// HUMBERT_RHO_SIMD: lane-parallel f64 K4 kernel for the batched walk step
    /// (None = the original per-walk arithmetic)
    pub simd: Option<Simd>,
}

/// Which lane-parallel kernel HUMBERT_RHO_SIMD selects (`1`/`auto`, `avx512`, `avx2`, `portable`;
/// a requested kernel the CPU lacks falls back to the next one, reported in ParResult::simd_used).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Simd {
    Auto,
    Avx512,
    Avx2,
    Portable,
}

impl Simd {
    fn parse(v: &str) -> Simd {
        match v {
            "1" | "auto" => Simd::Auto,
            "avx512" => Simd::Avx512,
            "avx2" => Simd::Avx2,
            "portable" => Simd::Portable,
            _ => panic!("HUMBERT_RHO_SIMD must be 1|auto|avx512|avx2|portable"),
        }
    }
    /// the kernel that will actually run on this host
    pub fn resolve(self) -> Simd {
        #[cfg(target_arch = "x86_64")]
        {
            let has512 = std::is_x86_feature_detected!("avx512f") && std::is_x86_feature_detected!("avx512dq") && std::is_x86_feature_detected!("avx512vl");
            let has2 = std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma");
            match self {
                Simd::Auto if has512 => Simd::Avx512,
                Simd::Auto if has2 => Simd::Avx2,
                Simd::Avx512 if has512 => Simd::Avx512,
                Simd::Avx512 | Simd::Avx2 if has2 => Simd::Avx2,
                _ => Simd::Portable,
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Simd::Portable
        }
    }
}

impl ParCfg {
    /// None when no rho flag is set (scalar rho, unchanged output).
    pub fn from_env() -> Option<ParCfg> {
        let g = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let threads = g("HUMBERT_RHO_THREADS");
        let batch = g("HUMBERT_RHO_BATCH");
        let neg = g("HUMBERT_RHO_NEG").map(|v| v == "1").unwrap_or(false);
        let fast = g("HUMBERT_RHO_FASTMUL").map(|v| v == "1").unwrap_or(false);
        let dp = g("HUMBERT_RHO_DPBITS");
        let nb = g("HUMBERT_RHO_NB");
        // HUMBERT_RHO_SIMD=0 is the same as unset
        let simd = g("HUMBERT_RHO_SIMD").filter(|v| v != "0").map(|v| Simd::parse(&v));
        let cap = g("HUMBERT_RHO_CAP").map(|v| v.parse::<u64>().expect("HUMBERT_RHO_CAP"));
        if threads.is_none() && batch.is_none() && !neg && !fast && dp.is_none() && nb.is_none() && simd.is_none() && cap.is_none() {
            return None;
        }
        let nb = nb.map(|v| v.parse::<usize>().expect("HUMBERT_RHO_NB"));
        if let Some(n) = nb {
            assert!(n.is_power_of_two() && n >= 2, "HUMBERT_RHO_NB must be a power of two");
        }
        Some(ParCfg {
            threads: threads.map(|v| v.parse().expect("HUMBERT_RHO_THREADS")).unwrap_or(1).max(1),
            batch: batch.map(|v| v.parse().expect("HUMBERT_RHO_BATCH")).unwrap_or(1).max(1),
            neg,
            fast,
            dp_bits: dp.map(|v| v.parse().expect("HUMBERT_RHO_DPBITS")),
            nb,
            cap: cap.unwrap_or(2_000_000_000),
            simd,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParResult {
    pub log: Option<u64>,
    /// walk steps (one point addition each), all threads; includes steps spent
    /// re-walking a detected fruitless cycle
    pub steps: u64,
    pub seconds: f64,
    /// all K4 multiplications (walks + table + starts + escapes + verification), 3 per inversion
    pub k4_muls: u64,
    pub k4_muls_walk: u64,
    /// K4 inversions in the walks (one per batch iteration)
    pub walk_inversions: u64,
    /// group operations outside the walk steps (table, start points, restarts, doublings, verifications)
    pub extra_group_ops: u64,
    pub dp_bits: u32,
    pub nb: usize,
    pub dps: u64,
    pub useless_collisions: u64,
    pub abandoned_walks: u64,
    pub fruitless_cycles: u64,
    pub fruitless_cycle_steps: u64,
    pub degenerate: u64,
    pub steps_per_thread: Vec<u64>,
    pub seconds_per_thread: Vec<f64>,
    /// fast (Barrett) scalar arithmetic actually ran (FASTMUL requested and p < 2^17)
    pub fast_used: bool,
    /// lane kernel that actually ran (None: per-walk arithmetic)
    pub simd_used: Option<Simd>,
    /// walks per thread actually used (batch rounded up to a multiple of the lane count with SIMD)
    pub batch_used: usize,
}

/// Walk/DP hash input: x0 + x1 2^16 + x2 2^32 + x3 2^48 mod 2^64. For p < 2^16 the
/// terms occupy disjoint bits, so this is the injective packing x0 | x1 << 16 | ...
/// (unchanged); for 2^16 <= p < 2^17 it is a hash (bit 16 of x3 is dropped and
/// neighbouring terms carry into each other). The lane kernel computes the same
/// value as (x0 + x1 2^16) + ((x2 + x3 2^16) << 32), wrapping.
#[inline]
fn pk(x: &E4) -> u64 {
    x[0].wrapping_add(x[1] << 16).wrapping_add(x[2] << 32).wrapping_add(x[3] << 48)
}
/// Injective key of a field element (coefficients < 2^32): DP table key and stored y.
#[inline]
fn fullkey(x: &E4) -> u128 {
    x[0] as u128 | (x[1] as u128) << 32 | (x[2] as u128) << 64 | (x[3] as u128) << 96
}
#[inline]
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// K4 arithmetic with its own counters. `fast`: u64 products + Barrett reduction
/// (valid for p < 2^17: every product sum stays below 7 p^2 < 2^37); otherwise the crate's K4::mul / K4::inv.
pub struct Ar<'a> {
    pub k: &'a K4,
    p: u64,
    bm: u64,
    fast: bool,
    pub muls: u64,
    pub invs: u64,
}

impl<'a> Ar<'a> {
    pub fn new(k: &'a K4, fast: bool) -> Ar<'a> {
        let fast = fast && k.p < (1 << 17);
        Ar { k, p: k.p, bm: u64::MAX / k.p, fast, muls: 0, invs: 0 }
    }
    #[inline(always)]
    fn red(&self, n: u64) -> u64 {
        // bm = floor(2^64 / p): q in {floor(n/p) - 1, floor(n/p)} for every u64 n
        let q = ((n as u128 * self.bm as u128) >> 64) as u64;
        let r = n - q * self.p;
        if r >= self.p {
            r - self.p
        } else {
            r
        }
    }
    #[inline(always)]
    fn mulf(&self, a: &E4, b: &E4) -> E4 {
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] += a[i] * b[j];
            }
        }
        let mut r = [c[0], c[1], c[2], c[3]];
        for h in 0..3 {
            let ch = self.red(c[4 + h]);
            let rh = &self.k.red[h];
            for i in 0..4 {
                r[i] += ch * rh[i];
            }
        }
        [self.red(r[0]), self.red(r[1]), self.red(r[2]), self.red(r[3])]
    }
    #[inline(always)]
    fn frobf(&self, a: &E4) -> E4 {
        let mut r = [0u64; 4];
        for j in 0..4 {
            let ft = &self.k.frob_t[j];
            for i in 0..4 {
                r[i] += a[j] * ft[i];
            }
        }
        [self.red(r[0]), self.red(r[1]), self.red(r[2]), self.red(r[3])]
    }
    fn inv_fp(&self, a: u64) -> u64 {
        // extended Euclid
        let (mut t, mut nt) = (0i64, 1i64);
        let (mut rr, mut nr) = (self.p as i64, a as i64);
        while nr != 0 {
            let qq = rr / nr;
            (t, nt) = (nt, t - qq * nt);
            (rr, nr) = (nr, rr - qq * nr);
        }
        assert!(rr == 1);
        if t < 0 {
            (t + self.p as i64) as u64
        } else {
            t as u64
        }
    }
    #[inline(always)]
    pub fn mul(&mut self, a: &E4, b: &E4) -> E4 {
        self.muls += 1;
        if self.fast {
            self.mulf(a, b)
        } else {
            self.k.mul(a, b)
        }
    }
    pub fn inv(&mut self, a: &E4) -> E4 {
        self.muls += 3;
        self.invs += 1;
        if self.fast {
            assert!(*a != [0; 4], "K4 inverse of zero");
            let f1 = self.frobf(a);
            let f2 = self.frobf(&f1);
            let f3 = self.frobf(&f2);
            let c = self.mulf(&self.mulf(&f1, &f2), &f3);
            let n = self.mulf(a, &c)[0];
            let ni = self.inv_fp(n);
            [self.red(c[0] * ni), self.red(c[1] * ni), self.red(c[2] * ni), self.red(c[3] * ni)]
        } else {
            self.k.inv(a)
        }
    }
    #[inline(always)]
    pub fn add(&self, a: &E4, b: &E4) -> E4 {
        self.k.add(a, b)
    }
    #[inline(always)]
    pub fn sub(&self, a: &E4, b: &E4) -> E4 {
        self.k.sub(a, b)
    }
    #[inline(always)]
    pub fn neg(&self, a: &E4) -> E4 {
        self.k.neg(a)
    }
}

// ---------------------------------------------------------------------------
// Lane-parallel K4 kernel (HUMBERT_RHO_SIMD). Elements are kept as exact
// integers in f64 lanes, LANES walks side by side (structure of arrays).
// For p < 2^16 every intermediate is an integer below 2^51, so products and
// sums are exact; one reduction per output coefficient (4 per K4 product, the
// t^4..t^6 terms folded in unreduced) uses the round-to-nearest magic constant,
// leaving |remainder| < 0.62 p, fixed with one conditional add. The results are
// the canonical representatives, i.e. bit-for-bit the elements K4::mul returns.
//
// 2^16 <= p < 2^17 (`Fk::wide`, kernels instantiated with W = true): folding
// c4..c6 unreduced would reach (p-1)^2 (4 + 6 (p-1)) ~ 1.35e16 > 2^53 near 2^17
// (exact only up to p ~ 114500). With W the largest high term c4 <= 3 (p-1)^2 is
// reduced first, so a folded sum is at most 4 (p-1)^2 + (p-1)^2 + (2 + 1) (p-1)^3
// = (p-1)^2 (5 + 3 (p-1)) < 6.76e15 < 2^53 (every partial sum and product too),
// and `fred` is exact for 0 <= n < 2^53: n pinv < 2^37, so MAGIC + n pinv rounds
// to an integer (ulp 1 in [2^52, 2^53)) within 1/2 + 2^-15 of n/p (pinv and, without
// FMA, the product add 2^-16 each), so |n - q p| <= p (1/2 + 2^-15) < 0.62 p, and
// q p <= n + 0.62 p < 2^53 and n - q p are exact. One extra
// reduction per product (5 instead of 4); the p < 2^16 kernels (W = false) are
// the unchanged code.
// ---------------------------------------------------------------------------
pub const LANES: usize = 8;
/// salt of the distinguished-point hash (the same in `walker` and `walker_v`)
const DPSALT: u64 = 0xD15C_0DE5_A5A5_0F0F;

/// one r-adding table entry for the lane kernel: x0..x3, y0..y3
#[derive(Clone, Copy)]
#[repr(C, align(64))]
struct TabE([f64; 8]);

/// per-group lane outputs of the backward pass
#[derive(Clone, Copy, Default)]
struct LaneOut {
    pkx: [u64; LANES],
    flip: [u64; LANES],
    hj: [u64; LANES],
    hd: [u64; LANES],
}
type V = [f64; LANES];
/// one K4 element per lane: [coefficient][lane]
type VE = [V; 4];

#[derive(Clone, Copy)]
struct Fk {
    /// 2^16 <= p < 2^17: the W = true kernels (c4 reduced before the fold)
    wide: bool,
    p: f64,
    pinv: f64,
    /// t^4, t^5, t^6 reduced
    red: [[f64; 4]; 3],
    a2: [f64; 4],
}

impl Fk {
    fn new(k: &K4, a2: &E4) -> Fk {
        assert!(k.p < (1 << 17), "lane kernel needs p < 2^17");
        let mut red = [[0.0; 4]; 3];
        for h in 0..3 {
            for i in 0..4 {
                red[h][i] = k.red[h][i] as f64;
            }
        }
        Fk { wide: k.p >= (1 << 16), p: k.p as f64, pinv: 1.0 / k.p as f64, red, a2: [a2[0] as f64, a2[1] as f64, a2[2] as f64, a2[3] as f64] }
    }
}

const MAGIC: f64 = 6755399441055744.0; // 1.5 * 2^52

/// a * b + c. With F (kernels compiled with the fma feature) a fused multiply-add,
/// otherwise two operations; every value here is an integer below 2^53 (or, for
/// n * pinv, rounded once more), so both give the same exact results below.
#[inline(always)]
fn ma<const F: bool>(a: f64, b: f64, c: f64) -> f64 {
    if F {
        a.mul_add(b, c)
    } else {
        a * b + c
    }
}

#[inline(always)]
fn fred<const F: bool>(n: f64, p: f64, pinv: f64) -> f64 {
    // n integer, 0 <= n < 2^53 (p < 2^17): q = nearest(n/p) up to rounding, |n - q p| < 0.62 p
    let q = ma::<F>(n, pinv, MAGIC) - MAGIC;
    let r = ma::<F>(-q, p, n);
    if r < 0.0 {
        r + p
    } else {
        r
    }
}

#[inline(always)]
fn fold<const F: bool>(c: [f64; 7], k: &Fk, i: usize) -> f64 {
    fred::<F>(ma::<F>(c[6], k.red[2][i], ma::<F>(c[5], k.red[1][i], ma::<F>(c[4], k.red[0][i], c[i]))), k.p, k.pinv)
}
/// the four output coefficients; W: reduce c4 first (2^16 <= p < 2^17)
#[inline(always)]
fn fold4<const F: bool, const W: bool>(mut c: [f64; 7], k: &Fk) -> [f64; 4] {
    if W {
        c[4] = fred::<F>(c[4], k.p, k.pinv);
    }
    [fold::<F>(c, k, 0), fold::<F>(c, k, 1), fold::<F>(c, k, 2), fold::<F>(c, k, 3)]
}

#[inline(always)]
fn vmul<const F: bool, const W: bool>(a: &VE, b: &VE, k: &Fk) -> VE {
    let mut o = [[0.0; LANES]; 4];
    for l in 0..LANES {
        let (a0, a1, a2, a3) = (a[0][l], a[1][l], a[2][l], a[3][l]);
        let (b0, b1, b2, b3) = (b[0][l], b[1][l], b[2][l], b[3][l]);
        let c = [
            a0 * b0,
            ma::<F>(a0, b1, a1 * b0),
            ma::<F>(a0, b2, ma::<F>(a1, b1, a2 * b0)),
            ma::<F>(a0, b3, ma::<F>(a1, b2, ma::<F>(a2, b1, a3 * b0))),
            ma::<F>(a1, b3, ma::<F>(a2, b2, a3 * b1)),
            ma::<F>(a2, b3, a3 * b2),
            a3 * b3,
        ];
        let f = fold4::<F, W>(c, k);
        for i in 0..4 {
            o[i][l] = f[i];
        }
    }
    o
}

#[inline(always)]
fn vsq<const F: bool, const W: bool>(a: &VE, k: &Fk) -> VE {
    let mut o = [[0.0; LANES]; 4];
    for l in 0..LANES {
        let (a0, a1, a2, a3) = (a[0][l], a[1][l], a[2][l], a[3][l]);
        let (d0, d1, d2) = (a0 + a0, a1 + a1, a2 + a2);
        let c = [a0 * a0, d0 * a1, ma::<F>(d0, a2, a1 * a1), ma::<F>(d0, a3, d1 * a2), ma::<F>(d1, a3, a2 * a2), d2 * a3, a3 * a3];
        let f = fold4::<F, W>(c, k);
        for i in 0..4 {
            o[i][l] = f[i];
        }
    }
    o
}

#[inline(always)]
fn vsub(a: &VE, b: &VE, p: f64) -> VE {
    let mut o = [[0.0; LANES]; 4];
    for i in 0..4 {
        for l in 0..LANES {
            let d = a[i][l] - b[i][l];
            o[i][l] = if d < 0.0 { d + p } else { d };
        }
    }
    o
}

/// a - (b, broadcast)
#[inline(always)]
fn vsubc(a: &VE, b: &[f64; 4], p: f64) -> VE {
    let mut o = [[0.0; LANES]; 4];
    for i in 0..4 {
        for l in 0..LANES {
            let d = a[i][l] - b[i];
            o[i][l] = if d < 0.0 { d + p } else { d };
        }
    }
    o
}

/// Forward pass: dx = tx - x (bad lanes -> 1, flagged), prefix products per lane.
#[inline(always)]
fn kfwd_body<const F: bool, const W: bool>(k: &Fk, xs: &[VE], tx: &[VE], dx: &mut [VE], pre: &mut [VE], bad: &mut [bool]) {
    let ng = xs.len();
    for g in 0..ng {
        let mut d = vsub(&tx[g], &xs[g], k.p);
        for l in 0..LANES {
            if d[0][l] == 0.0 && d[1][l] == 0.0 && d[2][l] == 0.0 && d[3][l] == 0.0 {
                bad[g * LANES + l] = true;
                d[0][l] = 1.0;
            } else {
                bad[g * LANES + l] = false;
            }
        }
        dx[g] = d;
        pre[g] = if g == 0 { d } else { vmul::<F, W>(&pre[g - 1], &d, k) };
    }
}

/// Backward pass: per-group inverses from the lane inverses of the chain totals,
/// then the affine addition W + R_j for every walk, written in place into xs, ys.
/// With `neg`, y is replaced by the canonical representative of {y, -y} (the one
/// with the smaller packed value, as `canon`) and `flip` records the negation.
/// `out` receives the packed x of every new point, its walk hash mix(pk(x)) and
/// its DP hash mix(pk(x) ^ DPSALT), computed in the lanes.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn kback_body<const F: bool, const W: bool>(k: &Fk, neg: bool, inv0: VE, xs: &mut [VE], ys: &mut [VE], tx: &[VE], ty: &[VE], dx: &[VE], pre: &[VE], out: &mut [LaneOut]) {
    let mut inv = inv0;
    let p = k.p;
    for g in (0..xs.len()).rev() {
        let ig = if g > 0 {
            let v = vmul::<F, W>(&inv, &pre[g - 1], k);
            inv = vmul::<F, W>(&inv, &dx[g], k);
            v
        } else {
            inv
        };
        let (x1, y1) = (xs[g], ys[g]);
        let dy = vsub(&ty[g], &y1, p);
        let lam = vmul::<F, W>(&dy, &ig, k);
        let l2 = vsq::<F, W>(&lam, k);
        let x3 = vsub(&vsub(&vsubc(&l2, &k.a2, p), &x1, p), &tx[g], p);
        let t = vsub(&x1, &x3, p);
        let ly = vmul::<F, W>(&lam, &t, k);
        let y3 = vsub(&ly, &y1, p);
        xs[g] = x3;
        for l in 0..LANES {
            let lo = x3[0][l] + x3[1][l] * 65536.0;
            let hi = x3[2][l] + x3[3][l] * 65536.0;
            // = pk(x3): lo, hi < 2^34; for p < 2^16 lo < 2^32 and this is the old `|`
            let px = ((hi as u64) << 32).wrapping_add(lo as u64);
            out[g].pkx[l] = px;
            out[g].hj[l] = mix(px);
            out[g].hd[l] = mix(px ^ DPSALT);
        }
        if neg {
            let mut yc = y3;
            for l in 0..LANES {
                // pk(-y) < pk(y) iff the highest nonzero coefficient c of y has p - y_c < y_c
                let top = if y3[3][l] != 0.0 {
                    y3[3][l]
                } else if y3[2][l] != 0.0 {
                    y3[2][l]
                } else if y3[1][l] != 0.0 {
                    y3[1][l]
                } else {
                    y3[0][l]
                };
                let f = top != 0.0 && p - top < top;
                for c in 0..4 {
                    let v = y3[c][l];
                    let nv = if v == 0.0 { 0.0 } else { p - v };
                    yc[c][l] = if f { nv } else { v };
                }
                out[g].flip[l] = f as u64;
            }
            ys[g] = yc;
        } else {
            ys[g] = y3;
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512dq,avx512vl,avx2,fma")]
unsafe fn kfwd_512<const W: bool>(k: &Fk, xs: &[VE], tx: &[VE], dx: &mut [VE], pre: &mut [VE], bad: &mut [bool]) {
    kfwd_body::<true, W>(k, xs, tx, dx, pre, bad)
}
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
unsafe fn kfwd_256<const W: bool>(k: &Fk, xs: &[VE], tx: &[VE], dx: &mut [VE], pre: &mut [VE], bad: &mut [bool]) {
    kfwd_body::<true, W>(k, xs, tx, dx, pre, bad)
}
#[cfg(target_arch = "x86_64")]
#[allow(clippy::too_many_arguments)]
#[target_feature(enable = "avx512f,avx512dq,avx512vl,avx2,fma")]
unsafe fn kback_512<const W: bool>(k: &Fk, neg: bool, inv0: VE, xs: &mut [VE], ys: &mut [VE], tx: &[VE], ty: &[VE], dx: &[VE], pre: &[VE], out: &mut [LaneOut]) {
    kback_body::<true, W>(k, neg, inv0, xs, ys, tx, ty, dx, pre, out)
}
#[cfg(target_arch = "x86_64")]
#[allow(clippy::too_many_arguments)]
#[target_feature(enable = "avx2,fma")]
unsafe fn kback_256<const W: bool>(k: &Fk, neg: bool, inv0: VE, xs: &mut [VE], ys: &mut [VE], tx: &[VE], ty: &[VE], dx: &[VE], pre: &[VE], out: &mut [LaneOut]) {
    kback_body::<true, W>(k, neg, inv0, xs, ys, tx, ty, dx, pre, out)
}

fn kfwd_w<const W: bool>(s: Simd, k: &Fk, xs: &[VE], tx: &[VE], dx: &mut [VE], pre: &mut [VE], bad: &mut [bool]) {
    match s {
        // SAFETY: `s` comes from Simd::resolve, which checked the CPU features
        #[cfg(target_arch = "x86_64")]
        Simd::Avx512 => unsafe { kfwd_512::<W>(k, xs, tx, dx, pre, bad) },
        #[cfg(target_arch = "x86_64")]
        Simd::Avx2 => unsafe { kfwd_256::<W>(k, xs, tx, dx, pre, bad) },
        _ => kfwd_body::<false, W>(k, xs, tx, dx, pre, bad),
    }
}
fn kfwd(s: Simd, k: &Fk, xs: &[VE], tx: &[VE], dx: &mut [VE], pre: &mut [VE], bad: &mut [bool]) {
    if k.wide {
        kfwd_w::<true>(s, k, xs, tx, dx, pre, bad)
    } else {
        kfwd_w::<false>(s, k, xs, tx, dx, pre, bad)
    }
}
#[allow(clippy::too_many_arguments)]
fn kback_w<const W: bool>(s: Simd, k: &Fk, neg: bool, inv0: VE, xs: &mut [VE], ys: &mut [VE], tx: &[VE], ty: &[VE], dx: &[VE], pre: &[VE], out: &mut [LaneOut]) {
    match s {
        // SAFETY: as in kfwd
        #[cfg(target_arch = "x86_64")]
        Simd::Avx512 => unsafe { kback_512::<W>(k, neg, inv0, xs, ys, tx, ty, dx, pre, out) },
        #[cfg(target_arch = "x86_64")]
        Simd::Avx2 => unsafe { kback_256::<W>(k, neg, inv0, xs, ys, tx, ty, dx, pre, out) },
        _ => kback_body::<false, W>(k, neg, inv0, xs, ys, tx, ty, dx, pre, out),
    }
}
#[allow(clippy::too_many_arguments)]
fn kback(s: Simd, k: &Fk, neg: bool, inv0: VE, xs: &mut [VE], ys: &mut [VE], tx: &[VE], ty: &[VE], dx: &[VE], pre: &[VE], out: &mut [LaneOut]) {
    if k.wide {
        kback_w::<true>(s, k, neg, inv0, xs, ys, tx, ty, dx, pre, out)
    } else {
        kback_w::<false>(s, k, neg, inv0, xs, ys, tx, ty, dx, pre, out)
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
#[target_feature(enable = "avx512f,avx512dq,avx512vl,avx2,fma")]
unsafe fn tmul_512<const W: bool>(a: &VE, b: &VE, k: &Fk, sq: bool) -> VE {
    if sq {
        vsq::<true, W>(a, k)
    } else {
        vmul::<true, W>(a, b, k)
    }
}
#[cfg(all(test, target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn tmul_256<const W: bool>(a: &VE, b: &VE, k: &Fk, sq: bool) -> VE {
    if sq {
        vsq::<true, W>(a, k)
    } else {
        vmul::<true, W>(a, b, k)
    }
}
/// test hook: one lane product (or squaring of a) with the kernel `s` and the
/// fold variant W (`tmul` picks the one the walk uses for `k`)
#[cfg(test)]
fn tmul_w<const W: bool>(s: Simd, a: &VE, b: &VE, k: &Fk, sq: bool) -> VE {
    match s {
        // SAFETY: `s` comes from Simd::resolve
        #[cfg(target_arch = "x86_64")]
        Simd::Avx512 => unsafe { tmul_512::<W>(a, b, k, sq) },
        #[cfg(target_arch = "x86_64")]
        Simd::Avx2 => unsafe { tmul_256::<W>(a, b, k, sq) },
        _ if sq => vsq::<false, W>(a, k),
        _ => vmul::<false, W>(a, b, k),
    }
}
#[cfg(test)]
fn tmul(s: Simd, a: &VE, b: &VE, k: &Fk, sq: bool) -> VE {
    if k.wide {
        tmul_w::<true>(s, a, b, k, sq)
    } else {
        tmul_w::<false>(s, a, b, k, sq)
    }
}

#[inline(always)]
fn vget(v: &[VE], i: usize) -> E4 {
    let (g, l) = (i / LANES, i % LANES);
    [v[g][0][l] as u64, v[g][1][l] as u64, v[g][2][l] as u64, v[g][3][l] as u64]
}
#[inline(always)]
fn vset(v: &mut [VE], i: usize, e: &E4) {
    let (g, l) = (i / LANES, i % LANES);
    for c in 0..4 {
        v[g][c][l] = e[c] as f64;
    }
}

/// Affine group law over Ar (counted); same formulas as Curve::add.
struct G<'a, 'b> {
    ar: &'b mut Ar<'a>,
    a2: E4,
    a4: E4,
    ops: u64,
}
impl<'a, 'b> G<'a, 'b> {
    fn add(&mut self, p: &Pt<E4>, q: &Pt<E4>) -> Pt<E4> {
        self.ops += 1;
        let (x1, y1) = match p {
            None => return *q,
            Some(v) => *v,
        };
        let (x2, y2) = match q {
            None => return *p,
            Some(v) => *v,
        };
        let lam = if x1 == x2 {
            if self.ar.add(&y1, &y2) == [0; 4] {
                return None;
            }
            let x1s = self.ar.mul(&x1, &x1);
            let t2 = self.ar.mul(&self.a2, &x1);
            let num = self.ar.add(&self.ar.add(&self.ar.add(&x1s, &x1s), &x1s), &self.ar.add(&self.ar.add(&t2, &t2), &self.a4));
            let yy = self.ar.add(&y1, &y1);
            let den = self.ar.inv(&yy);
            self.ar.mul(&num, &den)
        } else {
            let dxx = self.ar.sub(&x2, &x1);
            let den = self.ar.inv(&dxx);
            let dyy = self.ar.sub(&y2, &y1);
            self.ar.mul(&dyy, &den)
        };
        let l2 = self.ar.mul(&lam, &lam);
        let x3 = self.ar.sub(&self.ar.sub(&self.ar.sub(&l2, &self.a2), &x1), &x2);
        let t = self.ar.sub(&x1, &x3);
        let ly = self.ar.mul(&lam, &t);
        let y3 = self.ar.sub(&ly, &y1);
        Some((x3, y3))
    }
    fn mul(&mut self, p: &Pt<E4>, k: u64) -> Pt<E4> {
        let mut r: Pt<E4> = None;
        let mut b = *p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(&b, &b);
            }
        }
        r
    }
}

/// canonical representative of {P, -P}: the y that is smaller in the order of
/// (y3, y2, y1, y0), i.e. -y when the highest nonzero coefficient c has p - c < c
/// (the lane kernel's rule; for p < 2^16 this is the old "smaller pk(y)").
#[inline(always)]
fn canon(ar: &Ar, y: &E4) -> (E4, bool) {
    let top = if y[3] != 0 {
        y[3]
    } else if y[2] != 0 {
        y[2]
    } else if y[1] != 0 {
        y[1]
    } else {
        y[0]
    };
    if top != 0 && ar.p - top < top {
        (ar.neg(y), true)
    } else {
        (*y, false)
    }
}

struct Shared {
    done: AtomicBool,
    /// DP table: fullkey(x) -> (a, b, fullkey(y)) (injective keys for every p < 2^17)
    table: Mutex<HashMap<u128, (u64, u64, u128)>>,
    result: Mutex<Option<u64>>,
}

#[derive(Default)]
struct ThreadStats {
    steps: u64,
    secs: f64,
    muls_walk: u64,
    muls_other: u64,
    invs_walk: u64,
    extra_ops: u64,
    dps: u64,
    useless: u64,
    abandoned: u64,
    fruitless: u64,
    fruitless_steps: u64,
    degenerate: u64,
}

fn dp_bits_auto(r: u64, w: usize, neg: bool) -> u32 {
    let mut n = (std::f64::consts::PI * r as f64 / 2.0).sqrt();
    if neg {
        n /= std::f64::consts::SQRT_2;
    }
    // expected DP overhead ~ W * 2^d steps: keep it near 1/64 of the expected work
    let target = n / (w as f64 * 64.0);
    let d = if target <= 1.0 { 0 } else { target.log2().floor() as u32 };
    d.clamp(2, 26)
}

/// Parallel DP rho for x with q = x * gen, gen of prime order r.
pub fn rho_par(c: &Curve<K4>, gen: &Pt<E4>, q: &Pt<E4>, r: u64, rng: &mut Rng, cfg: &ParCfg) -> ParResult {
    let t0 = std::time::Instant::now();
    // pk() (walk and DP hash) is injective below 2^16 and a hash below 2^17; the DP
    // table uses the injective fullkey(); the f64 lane kernel is exact below 2^17
    assert!(c.f.p < (1 << 17), "rho_par needs p < 2^17");
    // HUMBERT_RHO_SIMD: resolve the kernel and round the batch up to whole lane groups
    let simd_used = cfg.simd.map(|s| s.resolve());
    let mut cfg_eff = cfg.clone();
    cfg_eff.simd = simd_used;
    if simd_used.is_some() {
        cfg_eff.batch = cfg.batch.div_ceil(LANES) * LANES;
    }
    let cfg = &cfg_eff;
    let nb = cfg.nb.unwrap_or(if cfg.neg { 2048 } else { 256 });
    let w = cfg.threads * cfg.batch;
    let dp_bits = cfg.dp_bits.unwrap_or_else(|| dp_bits_auto(r, w, cfg.neg));
    let dp_mask: u64 = (1u64 << dp_bits) - 1;
    let maxlen: u64 = 32u64 << dp_bits;
    // shared r-adding table R_j = ma_j gen + mb_j q (the walk function every thread uses)
    let k0 = c.f.clone();
    let mut ar0 = Ar::new(&k0, cfg.fast);
    let mut tab_x = Vec::with_capacity(nb);
    let mut tab_y = Vec::with_capacity(nb);
    let mut tab_a = Vec::with_capacity(nb);
    let mut tab_b = Vec::with_capacity(nb);
    let table_ops;
    {
        let mut g = G { ar: &mut ar0, a2: c.a2, a4: c.a4, ops: 0 };
        while tab_x.len() < nb {
            let (ma, mb) = (rng.below(r), rng.below(r));
            let (pa, pb) = (g.mul(gen, ma), g.mul(q, mb));
            let pt = g.add(&pa, &pb);
            if let Some((x, y)) = pt {
                tab_x.push(x);
                tab_y.push(y);
                tab_a.push(ma);
                tab_b.push(mb);
            }
        }
        table_ops = g.ops;
    }
    let table_muls = ar0.muls;
    let fast_used = ar0.fast;
    // lane kernel: R_j as f64, x and y in one 64-byte line
    let tabf: Vec<TabE> = if simd_used.is_some() {
        tab_x.iter().zip(&tab_y).map(|(x, y)| TabE([x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64, y[0] as f64, y[1] as f64, y[2] as f64, y[3] as f64])).collect()
    } else {
        Vec::new()
    };
    if let Some(s) = simd_used {
        eprintln!("[rho_par] simd={s:?} lanes={LANES} batch_used={} fast_used={fast_used}", cfg.batch);
    }
    let seeds: Vec<u64> = (0..cfg.threads).map(|_| rng.next()).collect();
    let sh = Shared { done: AtomicBool::new(false), table: Mutex::new(HashMap::new()), result: Mutex::new(None) };
    let cap_per_thread = cfg.cap / cfg.threads as u64;
    let stats: Vec<ThreadStats> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..cfg.threads)
            .map(|ti| {
                let kt = c.f.clone();
                let (a2, a4) = (c.a2, c.a4);
                let (tab_x, tab_y, tab_a, tab_b) = (&tab_x, &tab_y, &tab_a, &tab_b);
                let tabf = &tabf;
                let sh = &sh;
                let seed = seeds[ti];
                let cfg = cfg.clone();
                let (gen, q) = (*gen, *q);
                scope.spawn(move || {
                    walker(&kt, a2, a4, &gen, &q, r, seed, &cfg, nb, dp_mask, maxlen, tab_x, tab_y, tab_a, tab_b, tabf, sh, cap_per_thread)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("rho thread")).collect()
    });
    let log = *sh.result.lock().unwrap();
    let dps = sh.table.lock().unwrap().len() as u64;
    let mut res = ParResult {
        log,
        seconds: t0.elapsed().as_secs_f64(),
        dp_bits,
        nb,
        k4_muls: table_muls,
        extra_group_ops: table_ops,
        fast_used,
        simd_used,
        batch_used: cfg.batch,
        ..Default::default()
    };
    let mut dp_seen = 0;
    for s in &stats {
        res.steps += s.steps;
        res.k4_muls += s.muls_walk + s.muls_other;
        res.k4_muls_walk += s.muls_walk;
        res.walk_inversions += s.invs_walk;
        res.extra_group_ops += s.extra_ops;
        dp_seen += s.dps;
        res.useless_collisions += s.useless;
        res.abandoned_walks += s.abandoned;
        res.fruitless_cycles += s.fruitless;
        res.fruitless_cycle_steps += s.fruitless_steps;
        res.degenerate += s.degenerate;
        res.steps_per_thread.push(s.steps);
        res.seconds_per_thread.push(s.secs);
    }
    res.dps = dp_seen.max(dps);
    res
}

#[allow(clippy::too_many_arguments)]
fn walker(
    k: &K4,
    a2: E4,
    a4: E4,
    gen: &Pt<E4>,
    q: &Pt<E4>,
    r: u64,
    seed: u64,
    cfg: &ParCfg,
    nb: usize,
    dp_mask: u64,
    maxlen: u64,
    tab_x: &[E4],
    tab_y: &[E4],
    tab_a: &[u64],
    tab_b: &[u64],
    tabf: &[TabE],
    sh: &Shared,
    cap: u64,
) -> ThreadStats {
    if cfg.simd.is_some() {
        return walker_v(k, a2, a4, gen, q, r, seed, cfg, nb, dp_mask, maxlen, tab_x, tab_y, tab_a, tab_b, tabf, sh, cap);
    }
    let t0 = std::time::Instant::now();
    let mut st = ThreadStats::default();
    let mut rng = Rng(seed);
    let mut ar = Ar::new(k, cfg.fast);
    let m = cfg.batch;
    let neg = cfg.neg;
    let nbm = (nb - 1) as u64;
    // ---- start-point sequence S_k = S_0 + k T (one group addition per restart) ----
    let mut other_muls = 0u64;
    let (mut s_pt, mut s_a, mut s_b, t_pt, t_a, t_b);
    {
        let m0 = ar.muls;
        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
        loop {
            let (a, b) = (rng.below(r), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                s_pt = p;
                s_a = a;
                s_b = b;
                break;
            }
        }
        loop {
            let (a, b) = (1 + rng.below(r - 1), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                t_pt = p;
                t_a = a;
                t_b = b;
                break;
            }
        }
        st.extra_ops += g.ops;
        other_muls += ar.muls - m0;
    }
    // next start (x, y, a, b), canonical under negation
    macro_rules! next_start {
        () => {{
            loop {
                let m0 = ar.muls;
                let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                s_pt = g.add(&s_pt, &t_pt);
                st.extra_ops += g.ops;
                other_muls += ar.muls - m0;
                s_a = (s_a + t_a) % r;
                s_b = (s_b + t_b) % r;
                if let Some((x, y)) = s_pt {
                    let (y, fl) = if neg { canon(&ar, &y) } else { (y, false) };
                    let (a, b) = if fl { ((r - s_a) % r, (r - s_b) % r) } else { (s_a, s_b) };
                    break (x, y, a, b);
                }
            }
        }};
    }
    let mut wx = vec![[0u64; 4]; m];
    let mut wy = vec![[0u64; 4]; m];
    let mut wa = vec![0u64; m];
    let mut wb = vec![0u64; m];
    let mut wlen = vec![0u64; m];
    let mut prev1 = vec![u64::MAX; m]; // packed x one step back (negation only)
    let mut prev2 = vec![u64::MAX; m]; // packed x two steps back
    let mut mark = vec![u64::MAX; m]; // cycle marker (negation only)
    let mut wj = vec![0usize; m];
    let mut dx = vec![[0u64; 4]; m];
    let mut pre = vec![[0u64; 4]; m];
    let mut bad = vec![false; m];
    for i in 0..m {
        let (x, y, a, b) = next_start!();
        wx[i] = x;
        wy[i] = y;
        wa[i] = a;
        wb[i] = b;
    }
    let one: E4 = [1, 0, 0, 0];
    const CYC: u64 = 1024;
    let walk_muls0 = ar.muls;
    let other_at_walk0 = other_muls;
    let mut escape_muls = 0u64;
    let mut iter = 0u64;
    'outer: loop {
        if iter & 63 == 0 && sh.done.load(Ordering::Relaxed) {
            break;
        }
        if st.steps >= cap {
            break;
        }
        iter += 1;
        // ---- one step of every walk: Montgomery's simultaneous inversion ----
        for i in 0..m {
            let h = mix(pk(&wx[i]));
            let j = (h & nbm) as usize;
            wj[i] = j;
            let d = ar.sub(&tab_x[j], &wx[i]);
            if d == [0; 4] {
                // W = +-R_j: never in practice; restart this walk
                bad[i] = true;
                dx[i] = one;
            } else {
                bad[i] = false;
                dx[i] = d;
            }
        }
        pre[0] = dx[0];
        for i in 1..m {
            pre[i] = ar.mul(&pre[i - 1], &dx[i]);
        }
        let mut inv = ar.inv(&pre[m - 1]);
        for i in (0..m).rev() {
            let inv_i = if i > 0 {
                let v = ar.mul(&inv, &pre[i - 1]);
                inv = ar.mul(&inv, &dx[i]);
                v
            } else {
                inv
            };
            if bad[i] {
                st.degenerate += 1;
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
                continue;
            }
            let j = wj[i];
            let (x1, y1) = (wx[i], wy[i]);
            let dy = ar.sub(&tab_y[j], &y1);
            let lam = ar.mul(&dy, &inv_i);
            let l2 = ar.mul(&lam, &lam);
            let x3 = ar.sub(&ar.sub(&ar.sub(&l2, &a2), &x1), &tab_x[j]);
            let t = ar.sub(&x1, &x3);
            let ly = ar.mul(&lam, &t);
            let y3 = ar.sub(&ly, &y1);
            let mut na = wa[i] + tab_a[j];
            if na >= r {
                na -= r;
            }
            let mut nb_ = wb[i] + tab_b[j];
            if nb_ >= r {
                nb_ -= r;
            }
            st.steps += 1;
            wlen[i] += 1;
            if neg {
                let (yc, fl) = canon(&ar, &y3);
                wx[i] = x3;
                wy[i] = yc;
                if fl {
                    wa[i] = if na == 0 { 0 } else { r - na };
                    wb[i] = if nb_ == 0 { 0 } else { r - nb_ };
                } else {
                    wa[i] = na;
                    wb[i] = nb_;
                }
                let px = pk(&x3);
                let cyc_len = if px == prev2[i] {
                    2
                } else if px == mark[i] {
                    // cycle of length <= CYC since the marker
                    (wlen[i] - 1) % CYC + 1
                } else {
                    0
                };
                prev2[i] = prev1[i];
                prev1[i] = px;
                if wlen[i] % CYC == 0 {
                    mark[i] = px;
                }
                if cyc_len > 0 {
                    // fruitless cycle: walk it once (scalar, counted), find the point
                    // with the smallest packed x, and escape to canon(2 * min)
                    st.fruitless += 1;
                    let m0 = ar.muls;
                    let (mut cx, mut cy, mut ca, mut cb) = (wx[i], wy[i], wa[i], wb[i]);
                    let (mut bx, mut by, mut ba, mut bb) = (cx, cy, ca, cb);
                    let mut steps_c = 0u64;
                    loop {
                        let jj = (mix(pk(&cx)) & nbm) as usize;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let nx = g.add(&Some((cx, cy)), &Some((tab_x[jj], tab_y[jj])));
                        steps_c += 1;
                        let (xx, yy) = match nx {
                            Some(v) => v,
                            None => break,
                        };
                        let mut aa = (ca + tab_a[jj]) % r;
                        let mut bbb = (cb + tab_b[jj]) % r;
                        let (yc, fl) = canon(&ar, &yy);
                        if fl {
                            aa = (r - aa) % r;
                            bbb = (r - bbb) % r;
                        }
                        cx = xx;
                        cy = yc;
                        ca = aa;
                        cb = bbb;
                        if pk(&cx) < pk(&bx) {
                            bx = cx;
                            by = cy;
                            ba = ca;
                            bb = cb;
                        }
                        if cx == wx[i] || steps_c > 4 * CYC {
                            break;
                        }
                    }
                    st.steps += steps_c;
                    st.fruitless_steps += steps_c;
                    let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                    let d = g.add(&Some((bx, by)), &Some((bx, by)));
                    st.extra_ops += 1;
                    escape_muls += ar.muls - m0;
                    match d {
                        Some((xx, yy)) => {
                            let (yc, fl) = canon(&ar, &yy);
                            let (aa, bbb) = ((2 * ba) % r, (2 * bb) % r);
                            wx[i] = xx;
                            wy[i] = yc;
                            wa[i] = if fl { (r - aa) % r } else { aa };
                            wb[i] = if fl { (r - bbb) % r } else { bbb };
                        }
                        None => {
                            let (x, y, a, b) = next_start!();
                            wx[i] = x;
                            wy[i] = y;
                            wa[i] = a;
                            wb[i] = b;
                            wlen[i] = 0;
                        }
                    }
                    prev1[i] = u64::MAX;
                    prev2[i] = u64::MAX;
                    mark[i] = pk(&wx[i]);
                }
            } else {
                wx[i] = x3;
                wy[i] = y3;
                wa[i] = na;
                wb[i] = nb_;
            }
            // ---- distinguished point? ----
            let h = mix(pk(&wx[i]) ^ 0xD15C_0DE5_A5A5_0F0F);
            if (h >> 20) & dp_mask == 0 {
                st.dps += 1;
                let key = fullkey(&wx[i]);
                let (a1, b1, y1p) = (wa[i], wb[i], fullkey(&wy[i]));
                let prev = {
                    let mut t = sh.table.lock().unwrap();
                    match t.get(&key) {
                        Some(v) => Some(*v),
                        None => {
                            t.insert(key, (a1, b1, y1p));
                            None
                        }
                    }
                };
                if let Some((a2c, b2c, y2p)) = prev {
                    // a1 + b1 x = +-(a2 + b2 x)
                    let cand = if y2p == y1p {
                        let den = (b2c + r - b1) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((a1 + r - a2c) % r, invr(den, r), r))
                        }
                    } else {
                        let den = (b1 + b2c) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((r - (a1 + a2c) % r) % r, invr(den, r), r))
                        }
                    };
                    let mut ok = false;
                    if let Some(x) = cand {
                        let m0 = ar.muls;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let chk = g.mul(gen, x);
                        st.extra_ops += g.ops;
                        escape_muls += ar.muls - m0;
                        if chk == *q {
                            ok = true;
                            let mut res = sh.result.lock().unwrap();
                            if res.is_none() {
                                *res = Some(x);
                            }
                            sh.done.store(true, Ordering::Relaxed);
                        }
                    }
                    if !ok {
                        st.useless += 1;
                    }
                    if ok {
                        break 'outer;
                    }
                }
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
            } else if wlen[i] > maxlen {
                st.abandoned += 1;
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
            }
        }
    }
    // walk muls: the batch steps only (escapes, restarts and verifications excluded)
    st.invs_walk = iter;
    st.muls_walk = (ar.muls - walk_muls0) - escape_muls - (other_muls - other_at_walk0);
    st.muls_other = ar.muls - st.muls_walk;
    st.secs = t0.elapsed().as_secs_f64();
    st
}

/// The walker with the lane kernel (HUMBERT_RHO_SIMD): the same r-adding walk,
/// start-point sequence, negation map, fruitless-cycle handling, DP rule and
/// processing order as `walker`, so with one thread it takes exactly the same
/// steps. Walk points live in f64 lanes (xs, ys); the kernel also returns each
/// new point's packed x and whether canonicalisation negated y. Rare events
/// (restarts, escapes, DPs) read and write single lanes.
#[allow(clippy::too_many_arguments)]
fn walker_v(
    k: &K4,
    a2: E4,
    a4: E4,
    gen: &Pt<E4>,
    q: &Pt<E4>,
    r: u64,
    seed: u64,
    cfg: &ParCfg,
    nb: usize,
    dp_mask: u64,
    maxlen: u64,
    tab_x: &[E4],
    tab_y: &[E4],
    tab_a: &[u64],
    tab_b: &[u64],
    tabf: &[TabE],
    sh: &Shared,
    cap: u64,
) -> ThreadStats {
    let t0 = std::time::Instant::now();
    let sk = cfg.simd.expect("walker_v needs a lane kernel");
    let mut st = ThreadStats::default();
    let mut rng = Rng(seed);
    let mut ar = Ar::new(k, cfg.fast);
    let m = cfg.batch;
    assert!(m % LANES == 0 && m > 0);
    let neg = cfg.neg;
    let nbm = (nb - 1) as u64;
    let fk = Fk::new(k, &a2);
    let ng = m / LANES;
    // ---- start-point sequence S_k = S_0 + k T (one group addition per restart) ----
    let mut other_muls = 0u64;
    let (mut s_pt, mut s_a, mut s_b, t_pt, t_a, t_b);
    {
        let m0 = ar.muls;
        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
        loop {
            let (a, b) = (rng.below(r), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                s_pt = p;
                s_a = a;
                s_b = b;
                break;
            }
        }
        loop {
            let (a, b) = (1 + rng.below(r - 1), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                t_pt = p;
                t_a = a;
                t_b = b;
                break;
            }
        }
        st.extra_ops += g.ops;
        other_muls += ar.muls - m0;
    }
    macro_rules! next_start {
        () => {{
            loop {
                let m0 = ar.muls;
                let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                s_pt = g.add(&s_pt, &t_pt);
                st.extra_ops += g.ops;
                other_muls += ar.muls - m0;
                s_a = (s_a + t_a) % r;
                s_b = (s_b + t_b) % r;
                if let Some((x, y)) = s_pt {
                    let (y, fl) = if neg { canon(&ar, &y) } else { (y, false) };
                    let (a, b) = if fl { ((r - s_a) % r, (r - s_b) % r) } else { (s_a, s_b) };
                    break (x, y, a, b);
                }
            }
        }};
    }
    let zv: VE = [[0.0; LANES]; 4];
    let (mut xs, mut ys, mut tx, mut ty) = (vec![zv; ng], vec![zv; ng], vec![zv; ng], vec![zv; ng]);
    let (mut vdx, mut vpre) = (vec![zv; ng], vec![zv; ng]);
    let mut out = vec![LaneOut::default(); ng];
    let mut hx = vec![0u64; m]; // packed x of the current point
    let mut hj = vec![0u64; m]; // mix(hx): selects R_j
    let mut wa = vec![0u64; m];
    let mut wb = vec![0u64; m];
    let mut wlen = vec![0u64; m];
    let mut prev1 = vec![u64::MAX; m];
    let mut prev2 = vec![u64::MAX; m];
    let mut mark = vec![u64::MAX; m];
    let mut wj = vec![0usize; m];
    let mut bad = vec![false; m];
    macro_rules! restart {
        ($i:expr) => {{
            let i = $i;
            let (x, y, a, b) = next_start!();
            vset(&mut xs, i, &x);
            vset(&mut ys, i, &y);
            hx[i] = pk(&x);
            hj[i] = mix(hx[i]);
            wa[i] = a;
            wb[i] = b;
            wlen[i] = 0;
            prev1[i] = u64::MAX;
            prev2[i] = u64::MAX;
            mark[i] = u64::MAX;
        }};
    }
    for i in 0..m {
        let (x, y, a, b) = next_start!();
        vset(&mut xs, i, &x);
        vset(&mut ys, i, &y);
        hx[i] = pk(&x);
        hj[i] = mix(hx[i]);
        wa[i] = a;
        wb[i] = b;
    }
    const CYC: u64 = 1024;
    // HUMBERT_RHO_PROF=1: wall ns per phase, printed to stderr per thread
    let prof = std::env::var("HUMBERT_RHO_PROF").map(|v| v == "1").unwrap_or(false);
    let mut pt = [0u128; 5];
    let mut tick = std::time::Instant::now();
    macro_rules! lap {
        ($k:expr) => {
            if prof {
                let now = std::time::Instant::now();
                pt[$k] += (now - tick).as_nanos();
                tick = now;
            }
        };
    }
    let walk_muls0 = ar.muls;
    let other_at_walk0 = other_muls;
    let mut escape_muls = 0u64;
    let mut lane_muls = 0u64;
    let mut iter = 0u64;
    'outer: loop {
        if iter & 63 == 0 && sh.done.load(Ordering::Relaxed) {
            break;
        }
        if st.steps >= cap {
            break;
        }
        iter += 1;
        lap!(4);
        // ---- gather R_j for every walk ----
        for i in 0..m {
            let j = (hj[i] & nbm) as usize;
            wj[i] = j;
            let (g, l) = (i / LANES, i % LANES);
            for c in 0..4 {
                tx[g][c][l] = tabf[j].0[c];
                ty[g][c][l] = tabf[j].0[4 + c];
            }
        }
        lap!(0);
        kfwd(sk, &fk, &xs, &tx, &mut vdx, &mut vpre, &mut bad);
        lap!(1);
        // one inversion for the LANES chain totals (Montgomery's trick, scalar Ar)
        let tot: [E4; LANES] = std::array::from_fn(|l| vget(&vpre, (ng - 1) * LANES + l));
        let mut tp = [[0u64; 4]; LANES];
        tp[0] = tot[0];
        for l in 1..LANES {
            tp[l] = ar.mul(&tp[l - 1], &tot[l]);
        }
        let mut iv = ar.inv(&tp[LANES - 1]);
        let mut inv0: VE = zv;
        for l in (0..LANES).rev() {
            let il = if l > 0 {
                let v = ar.mul(&iv, &tp[l - 1]);
                iv = ar.mul(&iv, &tot[l]);
                v
            } else {
                iv
            };
            for c in 0..4 {
                inv0[c][l] = il[c] as f64;
            }
        }
        lap!(2);
        kback(sk, &fk, neg, inv0, &mut xs, &mut ys, &tx, &ty, &vdx, &vpre, &mut out);
        // lane products: prefix (ng-1) + back pass 2 (ng-1) per lane, 3 per walk step (squaring counted as a product)
        lane_muls += (3 * (ng - 1) * LANES + 3 * m) as u64;
        lap!(3);
        for i in (0..m).rev() {
            if bad[i] {
                st.degenerate += 1;
                restart!(i);
                continue;
            }
            let (g, l) = (i / LANES, i % LANES);
            let j = wj[i];
            let na = wa[i] + tab_a[j];
            let na = na - (r & ((na >= r) as u64).wrapping_neg());
            let nb_ = wb[i] + tab_b[j];
            let nb_ = nb_ - (r & ((nb_ >= r) as u64).wrapping_neg());
            st.steps += 1;
            wlen[i] += 1;
            let px = out[g].pkx[l];
            hx[i] = px;
            hj[i] = out[g].hj[l];
            let mut hdp = out[g].hd[l];
            if neg {
                // branchless (the flip is a coin toss): x -> (r - x) mod r when flipped
                let fm = out[g].flip[l].wrapping_neg();
                let nza = ((na != 0) as u64).wrapping_neg();
                let nzb = ((nb_ != 0) as u64).wrapping_neg();
                wa[i] = (((r - na) & nza) & fm) | (na & !fm);
                wb[i] = (((r - nb_) & nzb) & fm) | (nb_ & !fm);
                let cyc_len = if px == prev2[i] {
                    2
                } else if px == mark[i] {
                    (wlen[i] - 1) % CYC + 1
                } else {
                    0
                };
                prev2[i] = prev1[i];
                prev1[i] = px;
                if wlen[i] % CYC == 0 {
                    mark[i] = px;
                }
                if cyc_len > 0 {
                    // fruitless cycle: identical to `walker`
                    st.fruitless += 1;
                    let m0 = ar.muls;
                    let (wx_i, wy_i) = (vget(&xs, i), vget(&ys, i));
                    let (mut cx, mut cy, mut ca, mut cb) = (wx_i, wy_i, wa[i], wb[i]);
                    let (mut bx, mut by, mut ba, mut bb) = (cx, cy, ca, cb);
                    let mut steps_c = 0u64;
                    loop {
                        let jj = (mix(pk(&cx)) & nbm) as usize;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let nx = g.add(&Some((cx, cy)), &Some((tab_x[jj], tab_y[jj])));
                        steps_c += 1;
                        let (xx, yy) = match nx {
                            Some(v) => v,
                            None => break,
                        };
                        let mut aa = (ca + tab_a[jj]) % r;
                        let mut bbb = (cb + tab_b[jj]) % r;
                        let (yc, fl) = canon(&ar, &yy);
                        if fl {
                            aa = (r - aa) % r;
                            bbb = (r - bbb) % r;
                        }
                        cx = xx;
                        cy = yc;
                        ca = aa;
                        cb = bbb;
                        if pk(&cx) < pk(&bx) {
                            bx = cx;
                            by = cy;
                            ba = ca;
                            bb = cb;
                        }
                        if cx == wx_i || steps_c > 4 * CYC {
                            break;
                        }
                    }
                    st.steps += steps_c;
                    st.fruitless_steps += steps_c;
                    let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                    let d = g.add(&Some((bx, by)), &Some((bx, by)));
                    st.extra_ops += 1;
                    escape_muls += ar.muls - m0;
                    match d {
                        Some((xx, yy)) => {
                            let (yc, fl) = canon(&ar, &yy);
                            let (aa, bbb) = ((2 * ba) % r, (2 * bb) % r);
                            vset(&mut xs, i, &xx);
                            vset(&mut ys, i, &yc);
                            hx[i] = pk(&xx);
                            wa[i] = if fl { (r - aa) % r } else { aa };
                            wb[i] = if fl { (r - bbb) % r } else { bbb };
                        }
                        None => {
                            let (x, y, a, b) = next_start!();
                            vset(&mut xs, i, &x);
                            vset(&mut ys, i, &y);
                            hx[i] = pk(&x);
                            wa[i] = a;
                            wb[i] = b;
                            wlen[i] = 0;
                        }
                    }
                    prev1[i] = u64::MAX;
                    prev2[i] = u64::MAX;
                    mark[i] = hx[i];
                    // the walk moved: recompute both hashes
                    hj[i] = mix(hx[i]);
                    hdp = mix(hx[i] ^ DPSALT);
                }
            } else {
                wa[i] = na;
                wb[i] = nb_;
            }
            // ---- distinguished point? (same rule as `walker`) ----
            if (hdp >> 20) & dp_mask == 0 {
                st.dps += 1;
                let key = fullkey(&vget(&xs, i));
                let (a1, b1, y1p) = (wa[i], wb[i], fullkey(&vget(&ys, i)));
                let prev = {
                    let mut t = sh.table.lock().unwrap();
                    match t.get(&key) {
                        Some(v) => Some(*v),
                        None => {
                            t.insert(key, (a1, b1, y1p));
                            None
                        }
                    }
                };
                if let Some((a2c, b2c, y2p)) = prev {
                    let cand = if y2p == y1p {
                        let den = (b2c + r - b1) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((a1 + r - a2c) % r, invr(den, r), r))
                        }
                    } else {
                        let den = (b1 + b2c) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((r - (a1 + a2c) % r) % r, invr(den, r), r))
                        }
                    };
                    let mut ok = false;
                    if let Some(x) = cand {
                        let m0 = ar.muls;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let chk = g.mul(gen, x);
                        st.extra_ops += g.ops;
                        escape_muls += ar.muls - m0;
                        if chk == *q {
                            ok = true;
                            let mut res = sh.result.lock().unwrap();
                            if res.is_none() {
                                *res = Some(x);
                            }
                            sh.done.store(true, Ordering::Relaxed);
                        }
                    }
                    if !ok {
                        st.useless += 1;
                    }
                    if ok {
                        break 'outer;
                    }
                }
                restart!(i);
            } else if wlen[i] > maxlen {
                st.abandoned += 1;
                restart!(i);
            }
        }
    }
    if prof {
        let n = st.steps.max(1) as f64;
        eprintln!(
            "[rho_prof] steps={} ns/step: gather={:.1} fwd={:.1} lane_inv={:.1} back={:.1} control={:.1}",
            st.steps,
            pt[0] as f64 / n,
            pt[1] as f64 / n,
            pt[2] as f64 / n,
            pt[3] as f64 / n,
            pt[4] as f64 / n
        );
    }
    ar.muls += lane_muls;
    st.invs_walk = iter;
    st.muls_walk = (ar.muls - walk_muls0) - escape_muls - (other_muls - other_at_walk0);
    st.muls_other = ar.muls - st.muls_walk;
    st.secs = t0.elapsed().as_secs_f64();
    st
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cover::Setup;
    use crate::curve::random_point;

    fn instance(p: u64, tw: i32, seed: u64) -> (Setup, Pt<E4>, Pt<E4>, u64, u64, Rng) {
        let mut rng = Rng(seed);
        let s = Setup::new(p, tw, &mut rng);
        let r = s.r;
        let (gen, q, x) = {
            let ce = s.curve();
            let cof = s.order / r as u128;
            let sq = |a: &E4| s.sqrt_k4(a);
            let gen = loop {
                let pt = ce.mul(&random_point(&ce, &sq, &mut rng), cof);
                if pt.is_some() {
                    break pt;
                }
            };
            let x = 1 + rng.below(r - 1);
            (gen, ce.mul(&gen, x as u128), x)
        };
        (s, gen, q, r, x, rng)
    }

    #[test]
    fn fast_arith_matches_k4() {
        for &p in &[17u64, 16001, 32003] {
            let mut rng = Rng(p);
            let s = Setup::new(p, 1, &mut rng);
            let k = s.k();
            let mut ar = Ar::new(k, true);
            assert!(ar.fast);
            for _ in 0..2000 {
                let a = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                let b = [p - 1, rng.below(p), p - 1, rng.below(p)];
                assert_eq!(ar.mul(&a, &b), k.mul(&a, &b));
                if a != [0; 4] {
                    assert_eq!(ar.inv(&a), k.inv(&a));
                }
            }
        }
    }

    fn solve(p: u64, tw: i32, seed: u64, cfg: ParCfg) -> (ParResult, u64) {
        let (s, gen, q, r, x, mut rng) = instance(p, tw, seed);
        let ce = s.curve();
        let res = rho_par(&ce, &gen, &q, r, &mut rng, &cfg);
        (res, x)
    }

    fn cfg(threads: usize, batch: usize, neg: bool, fast: bool) -> ParCfg {
        ParCfg { threads, batch, neg, fast, dp_bits: None, nb: None, cap: 2_000_000_000, simd: None }
    }

    #[test]
    fn par_rho_solves_small_instances() {
        // toy primes: r ~ p^4 / cofactor, sqrt(r) ~ p^2
        for &(p, seed) in &[(101u64, 1u64), (101, 2), (211, 3), (211, 4), (401, 5)] {
            let tw = if seed % 2 == 1 { 1 } else { -1 };
            for c in [cfg(1, 1, false, false), cfg(1, 1, false, true), cfg(4, 1, false, true), cfg(2, 16, false, true), cfg(1, 1, true, true), cfg(4, 8, true, true), cfg(3, 5, true, false)] {
                let (res, x) = solve(p, tw, seed, c.clone());
                assert_eq!(res.log, Some(x), "p={p} seed={seed} cfg={c:?}");
                assert!(res.steps > 0);
            }
        }
    }

    #[test]
    fn par_rho_negation_finds_fruitless_cycles_and_still_solves() {
        // tiny table to force many 2-cycles
        let (res, x) = solve(211, 1, 7, ParCfg { nb: Some(4), ..cfg(2, 4, true, true) });
        assert_eq!(res.log, Some(x));
        assert!(res.fruitless_cycles > 0);
    }

    fn kernels() -> Vec<Simd> {
        let mut v = vec![Simd::Portable];
        for s in [Simd::Avx2, Simd::Avx512] {
            if s.resolve() == s {
                v.push(s);
            }
        }
        v
    }

    fn random_k4(p: u64, rng: &mut Rng) -> K4 {
        loop {
            let m = vec![rng.below(p), rng.below(p), rng.below(p), rng.below(p), 1];
            if crate::poly::is_irreducible_quartic(&m, p) {
                break K4::new(p, [m[0], m[1], m[2], m[3]]);
            }
        }
    }

    /// Barrett (FASTMUL) and every lane kernel equal K4::mul / K4::inv, including
    /// p = 65521 (the largest prime below 2^16) with edge-valued coefficients.
    #[test]
    fn lane_and_barrett_arith_match_k4_up_to_2_16() {
        for &p in &[17u64, 101, 16001, 32003, 32749, 32771, 65519, 65521] {
            let mut rng = Rng(p ^ 0x5eed);
            for _rep in 0..3 {
                let k = random_k4(p, &mut rng);
                let mut ar = Ar::new(&k, true);
                assert!(ar.fast, "p={p}");
                let edge = [0, 1, 2, p / 2, p / 2 + 1, p - 2, p - 1];
                let el = |rng: &mut Rng| -> E4 {
                    std::array::from_fn(|_| if rng.below(3) == 0 { edge[rng.below(edge.len() as u64) as usize] } else { rng.below(p) })
                };
                let fk = Fk::new(&k, &el(&mut rng));
                for _ in 0..300 {
                    let a: [E4; LANES] = std::array::from_fn(|_| el(&mut rng));
                    let b: [E4; LANES] = std::array::from_fn(|_| el(&mut rng));
                    let mut va = vec![[[0.0; LANES]; 4]];
                    let mut vb = vec![[[0.0; LANES]; 4]];
                    for l in 0..LANES {
                        vset(&mut va, l, &a[l]);
                        vset(&mut vb, l, &b[l]);
                        assert_eq!(ar.mul(&a[l], &b[l]), k.mul(&a[l], &b[l]), "p={p}");
                        if a[l] != [0; 4] {
                            assert_eq!(ar.inv(&a[l]), k.inv(&a[l]), "p={p}");
                        }
                    }
                    for s in kernels() {
                        let pr = vec![tmul(s, &va[0], &vb[0], &fk, false)];
                        let sq = vec![tmul(s, &va[0], &vb[0], &fk, true)];
                        for l in 0..LANES {
                            assert_eq!(vget(&pr, l), k.mul(&a[l], &b[l]), "p={p} {s:?}");
                            assert_eq!(vget(&sq, l), k.mul(&a[l], &a[l]), "p={p} {s:?} sq");
                        }
                    }
                    // all-(p-1) worst case for the bound
                    let w = [p - 1; 4];
                    assert_eq!(ar.mul(&w, &w), k.mul(&w, &w));
                }
            }
        }
    }

    fn simd_cfg(threads: usize, batch: usize, neg: bool, s: Simd) -> ParCfg {
        ParCfg { simd: Some(s), ..cfg(threads, batch, neg, true) }
    }

    /// One thread: the lane walker takes exactly the steps of the original walker
    /// (same walk, start sequence, negation map, cycles, DPs and solution).
    #[test]
    fn simd_walk_identical_to_scalar_walk_one_thread() {
        for &(p, seed) in &[(101u64, 1u64), (211, 4), (401, 5), (1009, 6)] {
            let tw = if seed % 2 == 1 { 1 } else { -1 };
            for &(batch, neg, nb) in &[(8usize, false, None), (64, true, None), (16, true, Some(4usize)), (24, false, Some(8))] {
                let base = ParCfg { nb, ..cfg(1, batch, neg, true) };
                let (r0, x) = solve(p, tw, seed, base.clone());
                assert_eq!(r0.log, Some(x));
                for s in kernels() {
                    let (r1, _) = solve(p, tw, seed, ParCfg { simd: Some(s), ..base.clone() });
                    let key = |r: &ParResult| (r.log, r.steps, r.dps, r.useless_collisions, r.fruitless_cycles, r.fruitless_cycle_steps, r.abandoned_walks, r.degenerate, r.walk_inversions, r.extra_group_ops);
                    assert_eq!(key(&r1), key(&r0), "p={p} seed={seed} batch={batch} neg={neg} nb={nb:?} {s:?}");
                    assert_eq!(r1.simd_used, Some(s));
                    assert!(r1.fast_used);
                }
            }
        }
    }

    #[test]
    fn simd_rho_solves_small_instances_multithread() {
        for &(p, seed) in &[(101u64, 1u64), (211, 2), (401, 3)] {
            let tw = if seed % 2 == 1 { 1 } else { -1 };
            for s in kernels() {
                for c in [simd_cfg(4, 64, true, s), simd_cfg(3, 20, false, s), simd_cfg(2, 512, true, s)] {
                    let (res, x) = solve(p, tw, seed, c.clone());
                    assert_eq!(res.log, Some(x), "p={p} seed={seed} cfg={c:?}");
                    assert_eq!(res.batch_used % LANES, 0);
                }
            }
        }
    }

    /// p = 65521 (a random curve, no Setup needed): a random curve through a chosen
    /// point; capped single-thread walks of the original and the lane walker agree
    /// in every counter (the group order is unknown, so nothing is solved).
    #[test]
    fn simd_walk_identical_at_p65521_capped() {
        let p = 65521u64;
        let mut rng = Rng(77);
        let k = random_k4(p, &mut rng);
        let el = |rng: &mut Rng| -> E4 { std::array::from_fn(|_| rng.below(p)) };
        let (a2, a4, x0, y0) = (el(&mut rng), el(&mut rng), el(&mut rng), el(&mut rng));
        // a6 = y0^2 - x0^3 - a2 x0^2 - a4 x0
        let x2 = k.mul(&x0, &x0);
        let rhs = k.add(&k.add(&k.mul(&x2, &x0), &k.mul(&a2, &x2)), &k.mul(&a4, &x0));
        let a6 = k.sub(&k.mul(&y0, &y0), &rhs);
        let c = Curve { f: &k, a2, a4, a6 };
        let gen = Some((x0, y0));
        let q = c.mul(&gen, 123_456_789);
        let r = 1_000_000_007u64; // bookkeeping modulus only
        for neg in [false, true] {
            let base = ParCfg { cap: 200_000, dp_bits: Some(6), ..cfg(1, 64, neg, true) };
            let r0 = rho_par(&c, &gen, &q, r, &mut Rng(5), &base);
            assert!(r0.fast_used);
            for s in kernels() {
                let r1 = rho_par(&c, &gen, &q, r, &mut Rng(5), &ParCfg { simd: Some(s), ..base.clone() });
                let key = |r: &ParResult| (r.steps, r.dps, r.useless_collisions, r.fruitless_cycles, r.fruitless_cycle_steps, r.abandoned_walks, r.degenerate, r.walk_inversions, r.extra_group_ops);
                assert_eq!(key(&r1), key(&r0), "neg={neg} {s:?}");
                assert!(r1.steps >= 200_000);
            }
        }
    }

    /// 2^16 <= p < 2^17: FASTMUL and every lane kernel (now the W = true fold) equal
    /// K4::mul / K4::inv on random and edge-valued elements, up to p = 131071.
    #[test]
    fn lane_and_barrett_arith_match_k4_up_to_2_17() {
        for &p in &[65537u64, 81001, 98009, 114593, 115001, 131063, 131071] {
            assert!(crate::setupfast::is_prime_u64(p), "p={p}");
            let mut rng = Rng(p ^ 0x17);
            for _rep in 0..3 {
                let k = random_k4(p, &mut rng);
                let mut ar = Ar::new(&k, true);
                assert!(ar.fast, "p={p}");
                let edge = [0, 1, 2, p / 2, p / 2 + 1, p - 2, p - 1];
                let el = |rng: &mut Rng| -> E4 {
                    std::array::from_fn(|_| if rng.below(3) == 0 { edge[rng.below(edge.len() as u64) as usize] } else { rng.below(p) })
                };
                let fk = Fk::new(&k, &el(&mut rng));
                assert!(fk.wide, "p={p}");
                for it in 0..300 {
                    let a: [E4; LANES] = std::array::from_fn(|l| if it == 0 && l == 0 { [p - 1; 4] } else { el(&mut rng) });
                    let b: [E4; LANES] = std::array::from_fn(|l| if it == 0 && l == 0 { [p - 1; 4] } else { el(&mut rng) });
                    let mut va = vec![[[0.0; LANES]; 4]];
                    let mut vb = vec![[[0.0; LANES]; 4]];
                    for l in 0..LANES {
                        vset(&mut va, l, &a[l]);
                        vset(&mut vb, l, &b[l]);
                        assert_eq!(ar.mul(&a[l], &b[l]), k.mul(&a[l], &b[l]), "p={p}");
                        if a[l] != [0; 4] {
                            assert_eq!(ar.inv(&a[l]), k.inv(&a[l]), "p={p}");
                        }
                    }
                    for s in kernels() {
                        let pr = vec![tmul(s, &va[0], &vb[0], &fk, false)];
                        let sq = vec![tmul(s, &va[0], &vb[0], &fk, true)];
                        for l in 0..LANES {
                            assert_eq!(vget(&pr, l), k.mul(&a[l], &b[l]), "p={p} {s:?}");
                            assert_eq!(vget(&sq, l), k.mul(&a[l], &a[l]), "p={p} {s:?} sq");
                        }
                    }
                }
            }
        }
    }

    /// The W = true fold at the extreme: p = 131071 with every reduction-table
    /// entry and every input p - 1 (and random inputs near p - 1) against an exact
    /// u128 reference, on every kernel. Also the arithmetic of the bound: the
    /// unreduced fold (p-1)^2 (4 + 6 (p-1)) exceeds 2^53 there, the W fold
    /// (p-1)^2 (5 + 3 (p-1)) does not, and the unreduced one is fine below 2^16.
    #[test]
    fn lane_fold_exact_on_extreme_inputs_near_2_17() {
        let two53 = 1u128 << 53;
        let w = |p: u128| (p - 1) * (p - 1) * (4 + 6 * (p - 1));
        let wr = |p: u128| (p - 1) * (p - 1) * (5 + 3 * (p - 1));
        assert!(w(65521) < 1u128 << 51 && w(131071) > two53 && wr(131071) < two53);
        for &p in &[65537u64, 115001, 131071] {
            let pm = p - 1;
            let red = [[pm; 4]; 3];
            let fk = Fk { wide: true, p: p as f64, pinv: 1.0 / p as f64, red: red.map(|r| r.map(|v| v as f64)), a2: [0.0; 4] };
            let reference = |a: &E4, b: &E4| -> E4 {
                let mut c = [0u128; 7];
                for i in 0..4 {
                    for j in 0..4 {
                        c[i + j] += a[i] as u128 * b[j] as u128;
                    }
                }
                std::array::from_fn(|i| ((c[i] + c[4] * red[0][i] as u128 + c[5] * red[1][i] as u128 + c[6] * red[2][i] as u128) % p as u128) as u64)
            };
            let mut rng = Rng(p);
            let mut narrow_wrong = 0;
            for it in 0..400 {
                let a: [E4; LANES] = std::array::from_fn(|l| if it == 0 && l == 0 { [pm; 4] } else { std::array::from_fn(|_| p - 1 - rng.below(64)) });
                let b: [E4; LANES] = std::array::from_fn(|l| if it == 0 && l == 0 { [pm; 4] } else { std::array::from_fn(|_| if rng.below(2) == 0 { p - 1 - rng.below(64) } else { rng.below(p) }) });
                let mut va = vec![[[0.0; LANES]; 4]];
                let mut vb = vec![[[0.0; LANES]; 4]];
                for l in 0..LANES {
                    vset(&mut va, l, &a[l]);
                    vset(&mut vb, l, &b[l]);
                }
                for s in kernels() {
                    let pr = vec![tmul_w::<true>(s, &va[0], &vb[0], &fk, false)];
                    let sq = vec![tmul_w::<true>(s, &va[0], &vb[0], &fk, true)];
                    let nr = vec![tmul_w::<false>(s, &va[0], &vb[0], &fk, false)];
                    for l in 0..LANES {
                        assert_eq!(vget(&pr, l), reference(&a[l], &b[l]), "p={p} {s:?}");
                        assert_eq!(vget(&sq, l), reference(&a[l], &a[l]), "p={p} {s:?} sq");
                        narrow_wrong += (vget(&nr, l) != reference(&a[l], &b[l])) as u32;
                    }
                }
            }
            // the old (unreduced) fold is not exact there: W is needed
            if p == 131071 {
                assert!(narrow_wrong > 0, "p={p}: the unreduced fold was exact on every input");
            }
        }
    }

    /// pk is the old 16-bit packing and canon the old "smaller pk(y)" rule for every
    /// coefficient < 2^16; above 2^16 canon picks the smaller of y, -y in the order
    /// (y3, y2, y1, y0), and fullkey is injective.
    #[test]
    fn pk_canon_fullkey_contract() {
        let mut rng = Rng(0xC0DE);
        for &p in &[17u64, 32003, 65521, 65537, 131071] {
            let k = random_k4(p, &mut rng);
            let ar = Ar::new(&k, true);
            for _ in 0..20000 {
                let y: E4 = std::array::from_fn(|_| if rng.below(4) == 0 { 0 } else { rng.below(p) });
                let (c, fl) = canon(&ar, &y);
                let ny = ar.neg(&y);
                assert_eq!(c, if fl { ny } else { y });
                let rev = |v: &E4| [v[3], v[2], v[1], v[0]];
                assert_eq!(fl, rev(&ny) < rev(&y), "p={p} y={y:?}");
                assert_eq!(fullkey(&y), y[0] as u128 + ((y[1] as u128) << 32) + ((y[2] as u128) << 64) + ((y[3] as u128) << 96));
                if p < (1 << 16) {
                    let old = y[0] | (y[1] << 16) | (y[2] << 32) | (y[3] << 48);
                    assert_eq!(pk(&y), old);
                    assert_eq!(fl, pk(&ny) < pk(&y));
                }
                // the lane kernel's packing formula
                let lo = y[0] as f64 + y[1] as f64 * 65536.0;
                let hi = y[2] as f64 + y[3] as f64 * 65536.0;
                assert_eq!(((hi as u64) << 32).wrapping_add(lo as u64), pk(&y));
            }
        }
    }

    /// p = 115001 and 131071 (both use the W fold; 115001 is above the ~114500 limit
    /// of the unreduced fold): capped single-thread walks of the original and the lane
    /// walker agree in every counter, with and without the negation map.
    #[test]
    fn simd_walk_identical_near_2_17_capped() {
        for &p in &[115001u64, 131071] {
            let mut rng = Rng(77);
            let k = random_k4(p, &mut rng);
            let el = |rng: &mut Rng| -> E4 { std::array::from_fn(|_| rng.below(p)) };
            let (a2, a4, x0, y0) = (el(&mut rng), el(&mut rng), el(&mut rng), el(&mut rng));
            let x2 = k.mul(&x0, &x0);
            let rhs = k.add(&k.add(&k.mul(&x2, &x0), &k.mul(&a2, &x2)), &k.mul(&a4, &x0));
            let a6 = k.sub(&k.mul(&y0, &y0), &rhs);
            let c = Curve { f: &k, a2, a4, a6 };
            let gen = Some((x0, y0));
            let q = c.mul(&gen, 987_654_321);
            let r = (1u64 << 61) - 1; // bookkeeping modulus only (prime, as large as a real r)
            for neg in [false, true] {
                let base = ParCfg { cap: 200_000, dp_bits: Some(6), ..cfg(1, 64, neg, true) };
                let r0 = rho_par(&c, &gen, &q, r, &mut Rng(5), &base);
                assert!(r0.fast_used);
                for s in kernels() {
                    let r1 = rho_par(&c, &gen, &q, r, &mut Rng(5), &ParCfg { simd: Some(s), ..base.clone() });
                    let key = |r: &ParResult| (r.steps, r.dps, r.useless_collisions, r.fruitless_cycles, r.fruitless_cycle_steps, r.abandoned_walks, r.degenerate, r.walk_inversions, r.extra_group_ops);
                    assert_eq!(key(&r1), key(&r0), "p={p} neg={neg} {s:?}");
                    assert!(r1.steps >= 200_000);
                }
                if neg {
                    assert!(r0.fruitless_cycles > 0, "p={p}: the negation-map cycle path is exercised");
                }
            }
        }
    }

    /// Throughput only (ignored by default): capped walks at p = 65521 on a random
    /// curve, old FASTMUL path vs the lane kernel. RHO_BENCH_STEPS, RHO_BENCH_THREADS,
    /// RHO_BENCH_BATCH, RHO_BENCH_SIMD (0 = old path) select the run.
    #[test]
    #[ignore]
    fn bench_throughput_p65521() {
        let e = |k: &str, d: u64| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
        let p = e("RHO_BENCH_P", 65521);
        let mut rng = Rng(77);
        let k = random_k4(p, &mut rng);
        let el = |rng: &mut Rng| -> E4 { std::array::from_fn(|_| rng.below(p)) };
        let (a2, a4, x0, y0) = (el(&mut rng), el(&mut rng), el(&mut rng), el(&mut rng));
        let x2 = k.mul(&x0, &x0);
        let rhs = k.add(&k.add(&k.mul(&x2, &x0), &k.mul(&a2, &x2)), &k.mul(&a4, &x0));
        let a6 = k.sub(&k.mul(&y0, &y0), &rhs);
        let c = Curve { f: &k, a2, a4, a6 };
        let gen = Some((x0, y0));
        let q = c.mul(&gen, 123_456_789);
        let simd = if e("RHO_BENCH_SIMD", 1) == 1 { Some(Simd::Auto) } else { None };
        let cfgb = ParCfg { cap: e("RHO_BENCH_STEPS", 100_000_000), simd, dp_bits: Some(e("RHO_BENCH_DP", 12) as u32), ..cfg(e("RHO_BENCH_THREADS", 4) as usize, e("RHO_BENCH_BATCH", 256) as usize, true, true) };
        let res = rho_par(&c, &gen, &q, 1_000_000_007, &mut Rng(5), &cfgb);
        println!(
            "BENCH p={p} simd={:?} threads={} batch={} steps={} secs={:.3} steps_per_sec={:.0} fast_used={}",
            res.simd_used, cfgb.threads, res.batch_used, res.steps, res.seconds, res.steps as f64 / res.seconds, res.fast_used
        );
    }
}
