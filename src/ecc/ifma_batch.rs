//! Eight-lane scalar multiplication on the AVX-512 IFMA kernel
//! ([`super::mont_ifma::mont_mul_ifma8`]), behind
//! [`Point::scalar_mul_batch`](super::point::Point::scalar_mul_batch).
//!
//! Each lane computes `k_l · P_l` on the same curve `y² = x³ + a·x + b`
//! over the same prime `p`, in Jacobian coordinates, limb-sliced over
//! `L` 52-bit limbs (register `k` of a field element holds limb `k` of the
//! eight lanes).  Values stay lazily in `[0, 2p)` (`R' = 2^(52L) > 4p`).
//!
//! Every lane runs the same operation sequence: a fixed 4-bit window from
//! the top, four doublings per window and one general addition of the
//! lane's table entry `T[d]` (fetched with gathers), selected per lane by
//! masks: lanes whose digit is 0 keep `Q`, lanes that have not started
//! (all digits so far 0) take `T[d]`.  Exceptional cases of the Jacobian
//! formulas, `H ≡ 0` in an addition (`Q = ±T`) and `Y ≡ 0` in a doubling
//! (2-torsion), are detected per lane; such lanes are reported as `None`
//! for the caller to recompute.
#![cfg(target_arch = "x86_64")]

use core::arch::x86_64::*;

use super::field::FieldElement;
use super::mont_ifma::{has_ifma, mont_mul_ifma8, pack52, MASK52};
use super::point::Point;
use num_bigint::BigUint;

#[derive(Clone, Copy)]
struct Fe<const L: usize>([__m512i; L]);

#[derive(Clone, Copy)]
struct Jac<const L: usize> {
    x: Fe<L>,
    y: Fe<L>,
    z: Fe<L>,
}

struct F8<const L: usize> {
    p: [__m512i; L],
    p2: [__m512i; L],
    n0: __m512i,
    a: Fe<L>,
    mask: __m512i,
}

impl<const L: usize> F8<L> {
    #[inline(always)]
    unsafe fn mul(&self, x: &Fe<L>, y: &Fe<L>) -> Fe<L> {
        Fe(mont_mul_ifma8::<L>(&x.0, &y.0, &self.p, self.n0))
    }

    #[inline(always)]
    unsafe fn sqr(&self, x: &Fe<L>) -> Fe<L> {
        self.mul(x, x)
    }

    /// `s - 2p` if `s ≥ 2p`, else `s`, for normalized `s < 4p`.
    #[inline(always)]
    unsafe fn reduce2p(&self, s: [__m512i; L]) -> Fe<L> {
        let z = _mm512_setzero_si512();
        let mut d = [z; L];
        let mut br = z;
        for k in 0..L {
            let v = _mm512_sub_epi64(_mm512_sub_epi64(s[k], self.p2[k]), br);
            br = _mm512_srli_epi64(v, 63);
            d[k] = _mm512_and_si512(v, self.mask);
        }
        let ge = _mm512_cmpeq_epi64_mask(br, z);
        let mut out = [z; L];
        for k in 0..L {
            out[k] = _mm512_mask_blend_epi64(ge, s[k], d[k]);
        }
        Fe(out)
    }

    /// `x − p` if `x ≥ p`, else `x`, for normalized `x < 2p`.
    #[inline(always)]
    unsafe fn reduce_p(&self, x: &Fe<L>) -> Fe<L> {
        let z = _mm512_setzero_si512();
        let mut d = [z; L];
        let mut br = z;
        for k in 0..L {
            let v = _mm512_sub_epi64(_mm512_sub_epi64(x.0[k], self.p[k]), br);
            br = _mm512_srli_epi64(v, 63);
            d[k] = _mm512_and_si512(v, self.mask);
        }
        let ge = _mm512_cmpeq_epi64_mask(br, z);
        let mut out = [z; L];
        for k in 0..L {
            out[k] = _mm512_mask_blend_epi64(ge, x.0[k], d[k]);
        }
        Fe(out)
    }

    #[inline(always)]
    unsafe fn add(&self, x: &Fe<L>, y: &Fe<L>) -> Fe<L> {
        let z = _mm512_setzero_si512();
        let mut s = [z; L];
        let mut c = z;
        for k in 0..L {
            let v = _mm512_add_epi64(_mm512_add_epi64(x.0[k], y.0[k]), c);
            s[k] = _mm512_and_si512(v, self.mask);
            c = _mm512_srli_epi64(v, 52);
        }
        self.reduce2p(s)
    }

    /// `x − y + 2p` (in `(0, 4p)`), reduced below `2p`.
    #[inline(always)]
    unsafe fn sub(&self, x: &Fe<L>, y: &Fe<L>) -> Fe<L> {
        let z = _mm512_setzero_si512();
        let mut s = [z; L];
        let mut c = z;
        for k in 0..L {
            let v = _mm512_add_epi64(
                _mm512_sub_epi64(_mm512_add_epi64(x.0[k], self.p2[k]), y.0[k]),
                c,
            );
            s[k] = _mm512_and_si512(v, self.mask);
            c = _mm512_srai_epi64(v, 52);
        }
        self.reduce2p(s)
    }

    #[inline(always)]
    unsafe fn dbl(&self, x: &Fe<L>) -> Fe<L> {
        self.add(x, x)
    }

    /// Lanes where `x ≡ 0 (mod p)`: `x ∈ {0, p}` for normalized `x < 2p`.
    #[inline(always)]
    unsafe fn is_zero(&self, x: &Fe<L>) -> __mmask8 {
        let z = _mm512_setzero_si512();
        let mut m0: __mmask8 = 0xff;
        let mut m1: __mmask8 = 0xff;
        for k in 0..L {
            m0 &= _mm512_cmpeq_epi64_mask(x.0[k], z);
            m1 &= _mm512_cmpeq_epi64_mask(x.0[k], self.p[k]);
        }
        m0 | m1
    }

    #[inline(always)]
    unsafe fn blend(m: __mmask8, a: &Fe<L>, b: &Fe<L>) -> Fe<L> {
        let mut out = a.0;
        for k in 0..L {
            out[k] = _mm512_mask_blend_epi64(m, a.0[k], b.0[k]);
        }
        Fe(out)
    }

    #[inline(always)]
    unsafe fn blend_j(m: __mmask8, a: &Jac<L>, b: &Jac<L>) -> Jac<L> {
        Jac {
            x: Self::blend(m, &a.x, &b.x),
            y: Self::blend(m, &a.y, &b.y),
            z: Self::blend(m, &a.z, &b.z),
        }
    }

    /// dbl-2007-bl (general `a`); also the lanes where `Y ≡ 0` (the result
    /// is the point at infinity, not representable here).
    #[inline(always)]
    unsafe fn pdbl(&self, q: &Jac<L>) -> (Jac<L>, __mmask8) {
        let bad = self.is_zero(&q.y);
        let xx = self.sqr(&q.x);
        let yy = self.sqr(&q.y);
        let yyyy = self.sqr(&yy);
        let zz = self.sqr(&q.z);
        let t = self.add(&q.x, &yy);
        let s = self.dbl(&self.sub(&self.sub(&self.sqr(&t), &xx), &yyyy));
        let azz2 = self.mul(&self.a, &self.sqr(&zz));
        let m = self.add(&self.add(&self.dbl(&xx), &xx), &azz2);
        let x3 = self.sub(&self.sqr(&m), &self.dbl(&s));
        let y8 = self.dbl(&self.dbl(&self.dbl(&yyyy)));
        let y3 = self.sub(&self.mul(&m, &self.sub(&s, &x3)), &y8);
        let yz = self.add(&q.y, &q.z);
        let z3 = self.sub(&self.sub(&self.sqr(&yz), &yy), &zz);
        (
            Jac {
                x: x3,
                y: y3,
                z: z3,
            },
            bad,
        )
    }

    /// add-2007-bl; also the lanes where `H ≡ 0` (`Q = ±T`).
    #[inline(always)]
    unsafe fn padd(&self, q: &Jac<L>, t: &Jac<L>) -> (Jac<L>, __mmask8) {
        let z1z1 = self.sqr(&q.z);
        let z2z2 = self.sqr(&t.z);
        let u1 = self.mul(&q.x, &z2z2);
        let u2 = self.mul(&t.x, &z1z1);
        let s1 = self.mul(&self.mul(&q.y, &t.z), &z2z2);
        let s2 = self.mul(&self.mul(&t.y, &q.z), &z1z1);
        let h = self.sub(&u2, &u1);
        let bad = self.is_zero(&h);
        let i = self.sqr(&self.dbl(&h));
        let j = self.mul(&h, &i);
        let r = self.dbl(&self.sub(&s2, &s1));
        let v = self.mul(&u1, &i);
        let x3 = self.sub(&self.sub(&self.sqr(&r), &j), &self.dbl(&v));
        let y3 = self.sub(
            &self.mul(&r, &self.sub(&v, &x3)),
            &self.dbl(&self.mul(&s1, &j)),
        );
        let zs = self.add(&q.z, &t.z);
        let z3 = self.mul(&self.sub(&self.sub(&self.sqr(&zs), &z1z1), &z2z2), &h);
        (
            Jac {
                x: x3,
                y: y3,
                z: z3,
            },
            bad,
        )
    }
}

/// Transpose eight lanes of 52-bit limbs (`[lane][limb]`) into limb vectors.
unsafe fn lanes<const L: usize>(xs: &[[u64; 8]; 8]) -> [__m512i; L] {
    std::array::from_fn(|k| {
        let col: [u64; 8] = std::array::from_fn(|lane| xs[lane][k]);
        core::mem::transmute::<[u64; 8], __m512i>(col)
    })
}

/// Read lane values back: `[lane][limb]`.
unsafe fn unlanes<const L: usize>(v: &Fe<L>) -> [[u64; 8]; 8] {
    let mut out = [[0u64; 8]; 8];
    for k in 0..L {
        let col: [u64; 8] = core::mem::transmute(v.0[k]);
        for lane in 0..8 {
            out[lane][k] = col[lane];
        }
    }
    out
}

/// Inputs/outputs in the `R' = 2^(52L)` Montgomery domain, as 52-bit limbs
/// per lane (`[lane][limb]`, `< 2p`).  `px`, `py`: affine points ×R';
/// `one`: `R' mod p`; `digits[w][lane]`: 4-bit windows, least significant
/// first.  Returns `(X, Y, Z)` per lane (Jacobian ×R'), and per lane:
/// `started` (some digit nonzero) and `bad` (an exceptional case hit).
struct LaneOut {
    x: [[u64; 8]; 8],
    y: [[u64; 8]; 8],
    z: [[u64; 8]; 8],
    started: u8,
    bad: u8,
}

/// # Safety
/// The CPU must support AVX-512F and AVX-512 IFMA.
#[target_feature(enable = "avx512f,avx512ifma")]
#[allow(clippy::too_many_arguments)]
unsafe fn ladder8<const L: usize>(
    p: &[u64; 8],
    p2: &[u64; 8],
    n0: u64,
    a_m: &[u64; 8],
    one: &[u64; 8],
    px: &[[u64; 8]; 8],
    py: &[[u64; 8]; 8],
    digits: &[[u8; 8]],
) -> LaneOut {
    let bc = |w: &[u64; 8]| -> [__m512i; L] { lanes::<L>(&[*w; 8]) };
    let f = F8::<L> {
        p: bc(p),
        p2: bc(p2),
        n0: _mm512_set1_epi64(n0 as i64),
        a: Fe(bc(a_m)),
        mask: _mm512_set1_epi64(MASK52 as i64),
    };
    // Table T[1..16) per lane; T[0] unused (zeros).
    let t1 = Jac {
        x: Fe(lanes::<L>(px)),
        y: Fe(lanes::<L>(py)),
        z: Fe(bc(one)),
    };
    let z = _mm512_setzero_si512();
    let zero_j = Jac {
        x: Fe([z; L]),
        y: Fe([z; L]),
        z: Fe([z; L]),
    };
    let mut tab = [zero_j; 16];
    tab[1] = t1;
    let (t2, mut bad) = f.pdbl(&t1);
    tab[2] = t2;
    for e in 3..16 {
        let (te, b) = f.padd(&tab[e - 1], &t1);
        tab[e] = te;
        bad |= b;
    }
    // Gather layout: u64 index ((e*3 + c)*L + k)*8 + lane.
    let mut flat = vec![0u64; 16 * 3 * L * 8];
    for (e, te) in tab.iter().enumerate() {
        for (c, fe) in [&te.x, &te.y, &te.z].into_iter().enumerate() {
            for k in 0..L {
                let col: [u64; 8] = core::mem::transmute(fe.0[k]);
                let at = ((e * 3 + c) * L + k) * 8;
                flat[at..at + 8].copy_from_slice(&col);
            }
        }
    }
    let lane_ix = _mm512_set_epi64(7, 6, 5, 4, 3, 2, 1, 0);
    let mut q = zero_j;
    let mut started: __mmask8 = 0;
    for w in (0..digits.len()).rev() {
        if started != 0 {
            for _ in 0..4 {
                let (d, b) = f.pdbl(&q);
                q = d;
                bad |= b & started;
            }
        }
        let dg = digits[w];
        let nz: __mmask8 = dg
            .iter()
            .enumerate()
            .fold(0, |m, (l, &d)| m | (((d != 0) as u8) << l));
        if nz == 0 {
            continue;
        }
        let dv = _mm512_set_epi64(
            dg[7] as i64,
            dg[6] as i64,
            dg[5] as i64,
            dg[4] as i64,
            dg[3] as i64,
            dg[2] as i64,
            dg[1] as i64,
            dg[0] as i64,
        );
        // lane + d·3L·8 by one multiply-add (no AVX-512DQ needed)
        let vix = _mm512_madd52lo_epu64(lane_ix, dv, _mm512_set1_epi64((3 * L * 8) as i64));
        let base = flat.as_ptr() as *const i64;
        let mut tsel = zero_j;
        for c in 0..3 {
            let fe = match c {
                0 => &mut tsel.x,
                1 => &mut tsel.y,
                _ => &mut tsel.z,
            };
            for k in 0..L {
                fe.0[k] = _mm512_i64gather_epi64::<8>(vix, base.add((c * L + k) * 8));
            }
        }
        let add_m = started & nz;
        let set_m = !started & nz;
        if add_m != 0 {
            let (s, b) = f.padd(&q, &tsel);
            bad |= b & add_m;
            q = F8::<L>::blend_j(add_m, &q, &s);
        }
        q = F8::<L>::blend_j(set_m, &q, &tsel);
        started |= nz;
    }
    LaneOut {
        x: unlanes(&q.x),
        y: unlanes(&q.y),
        z: unlanes(&q.z),
        started,
        bad,
    }
}

fn words8(x: &BigUint) -> [u64; 8] {
    let mut w = [0u64; 8];
    for (i, d) in x.iter_u64_digits().enumerate() {
        w[i] = d;
    }
    w
}

fn from52(l: &[u64; 8], nl: usize) -> BigUint {
    let mut x = BigUint::from(0u8);
    for k in (0..nl).rev() {
        x = (x << 52usize) + BigUint::from(l[k]);
    }
    x
}

/// Limb count `L` for a modulus of `bits` bits (`2^(52L) > 4p`), if the
/// batch supports it (`L ≤ 8`, i.e. `p < 2^414`).
fn limbs_for(bits: u64) -> Option<usize> {
    let l = (bits as usize + 2).div_ceil(52).max(3);
    (l <= 8).then_some(l)
}

/// Up to eight `k_l·P_l` (affine `(x, y)` in `[0, p)`, or `None` for the
/// point at infinity) on `y² = x³ + a·x + b` over `p`.  Per lane:
/// `Some(point)` or `None` when the lane needs the scalar fallback
/// (exceptional case of the Jacobian formulas, or `Z` not invertible).
/// `None` overall without IFMA or for an unsupported modulus.
fn batch8(
    pts: &[Option<(&BigUint, &BigUint)>],
    ks: &[&BigUint],
    a: &BigUint,
    p: &BigUint,
) -> Option<Vec<Option<Point>>> {
    if !has_ifma() || !p.bit(0) || *p < BigUint::from(3u8) {
        return None;
    }
    let l = limbs_for(p.bits())?;
    macro_rules! go {
        ($l:literal) => {
            // SAFETY: has_ifma() checked above.
            unsafe { run::<$l>(pts, ks, a, p) }
        };
    }
    Some(match l {
        3 => go!(3),
        4 => go!(4),
        5 => go!(5),
        6 => go!(6),
        7 => go!(7),
        _ => go!(8),
    })
}

/// # Safety
/// The CPU must support AVX-512F and AVX-512 IFMA.
#[target_feature(enable = "avx512f,avx512ifma")]
unsafe fn run<const L: usize>(
    pts: &[Option<(&BigUint, &BigUint)>],
    ks: &[&BigUint],
    a: &BigUint,
    p: &BigUint,
) -> Vec<Option<Point>> {
    let n = pts.len().min(8);
    let rp = BigUint::from(1u8) << (52 * L);
    let to_m = |x: &BigUint| -> [u64; 8] { pack52::<L>(&words8(&((x * &rp) % p)), 0) };
    let p52 = pack52::<L>(&words8(p), 0);
    let p2_52 = pack52::<L>(&words8(&(p * 2u32)), 0);
    let p0 = p.iter_u64_digits().next().unwrap_or(1);
    let mut inv: u64 = 1;
    for _ in 0..6 {
        inv = inv.wrapping_mul(2u64.wrapping_sub(p0.wrapping_mul(inv)));
    }
    let n0 = inv.wrapping_neg() & MASK52;
    let mut px = [[0u64; 8]; 8];
    let mut py = [[0u64; 8]; 8];
    let mut active = [false; 8];
    let mut maxbits = 0u64;
    for l in 0..n {
        if let Some((x, y)) = pts[l] {
            if ks[l].bits() > 0 {
                px[l] = to_m(x);
                py[l] = to_m(y);
                active[l] = true;
                maxbits = maxbits.max(ks[l].bits());
            }
        }
    }
    let nwin = (maxbits as usize).div_ceil(4).max(1);
    let mut digits = vec![[0u8; 8]; nwin];
    for l in 0..n {
        if !active[l] {
            continue;
        }
        let d: Vec<u64> = ks[l].iter_u64_digits().collect();
        for (w, dw) in digits.iter_mut().enumerate() {
            let bit = 4 * w;
            if let Some(word) = d.get(bit / 64) {
                dw[l] = ((word >> (bit % 64)) & 15) as u8;
            }
        }
    }
    let out = ladder8::<L>(
        &p52,
        &p2_52,
        n0,
        &to_m(a),
        &to_m(&BigUint::from(1u8)),
        &px,
        &py,
        &digits,
    );
    let zero = BigUint::from(0u8);
    let mut res: Vec<Option<Point>> = (0..n).map(|_| Some(Point::Infinity)).collect();
    let mut zs = Vec::new();
    let mut idx = Vec::new();
    for l in 0..n {
        if !active[l] {
            continue;
        }
        if out.bad >> l & 1 == 1 {
            res[l] = None;
            continue;
        }
        if out.started >> l & 1 == 0 {
            continue;
        }
        let zl = from52(&out.z[l], L) % p;
        if zl == zero {
            res[l] = None;
            continue;
        }
        zs.push(zl);
        idx.push(l);
    }
    if zs.is_empty() {
        return res;
    }
    // Montgomery's batch inversion: one inverse for all lanes.
    let mut pre = Vec::with_capacity(zs.len());
    let mut acc = BigUint::from(1u8);
    for z in &zs {
        acc = (&acc * z) % p;
        pre.push(acc.clone());
    }
    let Some(mut inv) = crate::gcd::int::modinv_big(&acc, p) else {
        for &l in &idx {
            res[l] = None;
        }
        return res;
    };
    // x = X·R'/Z², y = Y·R'²/Z³ for X, Y, Z in the R' domain.
    let rp1 = &rp % p;
    let rp2 = (&rp1 * &rp1) % p;
    for i in (0..zs.len()).rev() {
        let zi = if i == 0 {
            inv.clone()
        } else {
            (&inv * &pre[i - 1]) % p
        };
        inv = (&inv * &zs[i]) % p;
        let l = idx[i];
        let zi2 = (&zi * &zi) % p;
        let zi3 = (&zi2 * &zi) % p;
        let x = (from52(&out.x[l], L) * &rp1 % p) * zi2 % p;
        let y = (from52(&out.y[l], L) * &rp2 % p) * zi3 % p;
        res[l] = Some(Point::Affine {
            x: FieldElement::new(x, p.clone()),
            y: FieldElement::new(y, p.clone()),
        });
    }
    res
}

/// [`Point::scalar_mul_batch`] on IFMA: chunks of eight lanes; lanes the
/// batch cannot finish are recomputed with [`Point::scalar_mul`].  `None`
/// when the batch does not apply (no IFMA, mixed or unsupported moduli),
/// so the caller runs the scalar path for everything.
pub(crate) fn scalar_mul_batch(
    points: &[Point],
    ks: &[BigUint],
    a: &FieldElement,
) -> Option<Vec<Point>> {
    let p = &a.modulus;
    let mut affine = Vec::with_capacity(points.len());
    for pt in points {
        match pt {
            Point::Infinity => affine.push(None),
            Point::Affine { x, y } => {
                if &x.modulus != p || &y.modulus != p || x.value >= *p || y.value >= *p {
                    return None;
                }
                affine.push(Some((&x.value, &y.value)));
            }
        }
    }
    if !has_ifma() || limbs_for(p.bits()).is_none() || !p.bit(0) {
        return None;
    }
    let mut out = Vec::with_capacity(points.len());
    for (c, chunk) in affine.chunks(8).enumerate() {
        let kc: Vec<&BigUint> = ks[8 * c..8 * c + chunk.len()].iter().collect();
        let lanes = batch8(chunk, &kc, &a.value, p)?;
        for (j, r) in lanes.into_iter().enumerate() {
            out.push(r.unwrap_or_else(|| points[8 * c + j].scalar_mul(&ks[8 * c + j], a)));
        }
    }
    Some(out)
}

/// Batched affine addition `(x1, y1) + (x2, y2)` over `n = 8·G` lanes
/// (`G` vector groups), inputs and outputs as 52-bit limbs `[walk][limb]`
/// in plain (non-Montgomery) form, `< p`.  One vector Fermat inversion is
/// shared by the `G` walks of each lane (Montgomery's simultaneous
/// inversion).  Returns per walk whether it was exceptional (`x1 = x2`:
/// doubling or a sum at infinity); those outputs are garbage.
///
/// # Safety
/// The CPU must support AVX-512F and AVX-512 IFMA.
#[target_feature(enable = "avx512f,avx512ifma")]
#[allow(clippy::too_many_arguments)]
unsafe fn affine_add_lanes<const L: usize>(
    p: &[u64; 8],
    p2: &[u64; 8],
    n0: u64,
    r2: &[u64; 8],
    pm2_bits: &[bool],
    x1: &[[u64; 8]],
    y1: &[[u64; 8]],
    x2: &[[u64; 8]],
    y2: &[[u64; 8]],
    x3: &mut [[u64; 8]],
    y3: &mut [[u64; 8]],
) -> Vec<bool> {
    let bc = |w: &[u64; 8]| -> [__m512i; L] { lanes::<L>(&[*w; 8]) };
    let f = F8::<L> {
        p: bc(p),
        p2: bc(p2),
        n0: _mm512_set1_epi64(n0 as i64),
        a: Fe(bc(&[0; 8])),
        mask: _mm512_set1_epi64(MASK52 as i64),
    };
    let n = x1.len();
    let groups = n.div_ceil(8);
    let z = _mm512_setzero_si512();
    let r2v = Fe(bc(r2));
    let mut one_plain = [0u64; 8];
    one_plain[0] = 1;
    let onev = Fe(bc(&one_plain));
    let load = |src: &[[u64; 8]], g: usize| -> Fe<L> {
        let block: [[u64; 8]; 8] =
            std::array::from_fn(|l| src.get(8 * g + l).copied().unwrap_or([0; 8]));
        Fe(lanes::<L>(&block))
    };
    // To the R' domain: v·R'² ·R'⁻¹ = v·R'.
    let mut xa = Vec::with_capacity(groups);
    let mut ya = Vec::with_capacity(groups);
    let mut xb = Vec::with_capacity(groups);
    let mut yb = Vec::with_capacity(groups);
    let mut d = Vec::with_capacity(groups);
    let mut bad = vec![false; n];
    let one_m = f.mul(&onev, &r2v); // R' mod p (Montgomery one)
    for g in 0..groups {
        let (a1, b1, a2, b2) = (load(x1, g), load(y1, g), load(x2, g), load(y2, g));
        let (a1, b1, a2, b2) = (
            f.mul(&a1, &r2v),
            f.mul(&b1, &r2v),
            f.mul(&a2, &r2v),
            f.mul(&b2, &r2v),
        );
        let mut dg = f.sub(&a2, &a1);
        let zm = f.is_zero(&dg);
        for l in 0..8 {
            if zm >> l & 1 == 1 && 8 * g + l < n {
                bad[8 * g + l] = true;
            }
        }
        // padding lanes and exceptional lanes: d := 1 so the product stays invertible
        let pad: __mmask8 = if 8 * g + 8 <= n {
            0
        } else {
            !((1u16 << (n - 8 * g)) as u8).wrapping_sub(1)
        };
        dg = F8::<L>::blend(zm | pad, &dg, &one_m);
        xa.push(a1);
        ya.push(b1);
        xb.push(a2);
        yb.push(b2);
        d.push(dg);
    }
    // prefix products
    let mut c = Vec::with_capacity(groups);
    let mut acc = d[0];
    c.push(acc);
    for g in 1..groups {
        acc = f.mul(&acc, &d[g]);
        c.push(acc);
    }
    // inv = acc^(p-2) (square-and-multiply, MSB first)
    let mut inv = one_m;
    for &bit in pm2_bits {
        inv = f.sqr(&inv);
        if bit {
            inv = f.mul(&inv, &acc);
        }
    }
    // back substitution and the addition
    for g in (0..groups).rev() {
        let ig = if g == 0 { inv } else { f.mul(&inv, &c[g - 1]) };
        if g > 0 {
            inv = f.mul(&inv, &d[g]);
        }
        let lam = f.mul(&f.sub(&yb[g], &ya[g]), &ig);
        let xs = f.sub(&f.sub(&f.sqr(&lam), &xa[g]), &xb[g]);
        let ys = f.sub(&f.mul(&lam, &f.sub(&xa[g], &xs)), &ya[g]);
        // out of the R' domain and into [0, p)
        let xo = f.reduce_p(&f.mul(&xs, &onev));
        let yo = f.reduce_p(&f.mul(&ys, &onev));
        let (xr, yr) = (unlanes(&xo), unlanes(&yo));
        for l in 0..8 {
            if 8 * g + l < n {
                x3[8 * g + l] = xr[l];
                y3[8 * g + l] = yr[l];
            }
        }
    }
    let _ = z;
    bad
}

/// Batched affine addition `P_i + Q_i` for affine points with coordinates
/// in `[0, p)`, on IFMA: eight walks per vector, one vector Fermat inversion
/// shared by all walks of a lane (Montgomery's simultaneous inversion).
/// Per pair `Some((x, y))`, or `None` when `x_P = x_Q` (doubling or a sum at
/// infinity: the caller adds those itself).  `None` overall without IFMA or
/// for an unsupported modulus (even, below 3, or `≥ 2^414`).
pub(crate) fn add_affine_batch(
    pts1: &[(&BigUint, &BigUint)],
    pts2: &[(&BigUint, &BigUint)],
    p: &BigUint,
) -> Option<Vec<Option<(BigUint, BigUint)>>> {
    if !has_ifma() || !p.bit(0) || *p < BigUint::from(3u8) || pts1.len() != pts2.len() {
        return None;
    }
    if pts1.is_empty() {
        return Some(Vec::new());
    }
    let l = limbs_for(p.bits())?;
    macro_rules! go {
        ($l:literal) => {
            // SAFETY: has_ifma() checked above.
            unsafe { add_run::<$l>(pts1, pts2, p) }
        };
    }
    Some(match l {
        3 => go!(3),
        4 => go!(4),
        5 => go!(5),
        6 => go!(6),
        7 => go!(7),
        _ => go!(8),
    })
}

/// 52-bit limbs `< 2^(52·8)` back to a `BigUint`.
fn big_from52<const L: usize>(l: &[u64; 8]) -> BigUint {
    let (w, _) = super::mont_ifma::unpack52::<7>(l, L);
    let mut d = Vec::with_capacity(14);
    for x in w {
        d.push(x as u32);
        d.push((x >> 32) as u32);
    }
    BigUint::new(d)
}

/// # Safety
/// The CPU must support AVX-512F and AVX-512 IFMA.
#[target_feature(enable = "avx512f,avx512ifma")]
unsafe fn add_run<const L: usize>(
    pts1: &[(&BigUint, &BigUint)],
    pts2: &[(&BigUint, &BigUint)],
    p: &BigUint,
) -> Vec<Option<(BigUint, BigUint)>> {
    let rp = BigUint::from(1u8) << (52 * L);
    let pk = |x: &BigUint| pack52::<L>(&words8(x), 0);
    let p0 = p.iter_u64_digits().next().unwrap_or(1);
    let mut inv: u64 = 1;
    for _ in 0..6 {
        inv = inv.wrapping_mul(2u64.wrapping_sub(p0.wrapping_mul(inv)));
    }
    let n0 = inv.wrapping_neg() & MASK52;
    let pm2 = p - 2u32;
    let bits: Vec<bool> = (0..pm2.bits()).rev().map(|i| pm2.bit(i)).collect();
    let r2 = pk(&((&rp * &rp) % p));
    let x1: Vec<_> = pts1.iter().map(|q| pk(q.0)).collect();
    let y1: Vec<_> = pts1.iter().map(|q| pk(q.1)).collect();
    let x2: Vec<_> = pts2.iter().map(|q| pk(q.0)).collect();
    let y2: Vec<_> = pts2.iter().map(|q| pk(q.1)).collect();
    let mut x3 = vec![[0u64; 8]; x1.len()];
    let mut y3 = x3.clone();
    let bad = affine_add_lanes::<L>(
        &pk(p),
        &pk(&(p * 2u32)),
        n0,
        &r2,
        &bits,
        &x1,
        &y1,
        &x2,
        &y2,
        &mut x3,
        &mut y3,
    );
    (0..x1.len())
        .map(|i| (!bad[i]).then(|| (big_from52::<L>(&x3[i]), big_from52::<L>(&y3[i]))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::RandBigInt;
    use rand::{rngs::StdRng, SeedableRng};

    /// `add_affine_batch` equals `Point::add` pair by pair (odd counts,
    /// `P = ±Q` flagged), at L = 3, 5, 8.
    #[test]
    fn affine_batch_matches_add() {
        if !has_ifma() {
            return;
        }
        let two = |e: u32| BigUint::from(1u8) << e;
        let mut rng = StdRng::seed_from_u64(0x6166_6669);
        for p in [
            two(127) - 1u32,
            two(255) - 19u32,
            two(384) - two(128) - two(96) + two(32) - 1u32,
        ] {
            let a = FieldElement::new(BigUint::from(3u8), p.clone());
            for cnt in [1usize, 9, 70] {
                let r = |rng: &mut StdRng| (rng.gen_biguint_below(&p), rng.gen_biguint_below(&p));
                let p1: Vec<(BigUint, BigUint)> = (0..cnt).map(|_| r(&mut rng)).collect();
                let mut p2: Vec<(BigUint, BigUint)> = (0..cnt).map(|_| r(&mut rng)).collect();
                if cnt > 3 {
                    p2[1] = p1[1].clone();
                    p2[2] = (p1[2].0.clone(), (&p - &p1[2].1) % &p);
                }
                let refs = |v: &[(BigUint, BigUint)]| -> Vec<(BigUint, BigUint)> { v.to_vec() };
                let (v1, v2) = (refs(&p1), refs(&p2));
                let r1: Vec<(&BigUint, &BigUint)> = v1.iter().map(|(x, y)| (x, y)).collect();
                let r2: Vec<(&BigUint, &BigUint)> = v2.iter().map(|(x, y)| (x, y)).collect();
                let got = add_affine_batch(&r1, &r2, &p).unwrap();
                for i in 0..cnt {
                    let pt = |(x, y): &(BigUint, BigUint)| Point::Affine {
                        x: FieldElement::new(x.clone(), p.clone()),
                        y: FieldElement::new(y.clone(), p.clone()),
                    };
                    if p1[i].0 == p2[i].0 {
                        assert!(got[i].is_none(), "p={p} cnt {cnt} i {i} not flagged");
                        continue;
                    }
                    let want = pt(&p1[i]).add(&pt(&p2[i]), &a);
                    let (x, y) = got[i].clone().unwrap();
                    assert_eq!(pt(&(x, y)), want, "p={p} cnt {cnt} i {i}");
                }
            }
        }
    }

    /// `Point::scalar_mul_batch` equals `scalar_mul` lane by lane, edge
    /// cases included (k = 0, 1, 16, 17, p − 1, the point at infinity, a
    /// 2-torsion point that forces the fallback), at L = 3, 5, 7, 8.
    #[test]
    fn batch_matches_scalar_mul() {
        let two = |e: u32| BigUint::from(1u8) << e;
        let mut rng = StdRng::seed_from_u64(0x6261_7463);
        for p in [
            two(127) - 1u32,
            two(255) - 19u32,
            two(256) - two(224) + two(192) + two(96) - 1u32,
            two(384) - two(128) - two(96) + two(32) - 1u32,
            two(521) - 1u32, // above 2^414: scalar path
        ] {
            let a = FieldElement::new(rng.gen_biguint_below(&p), p.clone());
            let mut pts: Vec<Point> = (0..19)
                .map(|_| Point::Affine {
                    x: FieldElement::new(rng.gen_biguint_below(&p), p.clone()),
                    y: FieldElement::new(rng.gen_biguint_below(&p), p.clone()),
                })
                .collect();
            pts[2] = Point::Infinity;
            pts[5] = Point::Affine {
                x: FieldElement::new(rng.gen_biguint_below(&p), p.clone()),
                y: FieldElement::new(BigUint::from(0u8), p.clone()),
            };
            let mut ks: Vec<BigUint> = (0..19).map(|_| rng.gen_biguint(p.bits())).collect();
            for (i, v) in [0u32, 1, 16, 17].iter().enumerate() {
                ks[8 + i] = BigUint::from(*v);
            }
            ks[12] = &p - 1u32;
            let got = Point::scalar_mul_batch(&pts, &ks, &a);
            for i in 0..19 {
                assert_eq!(got[i], pts[i].scalar_mul(&ks[i], &a), "p={p} lane {i}");
            }
        }
    }
}
