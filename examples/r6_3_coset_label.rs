//! Round 6, lead R6.3: the trace pair `(Tr(x), Tr(y + x·H(x)))` as a label
//! of `E/4E` on `y^2 + xy = x^3 + 1` over `F_{2^n}`, n odd; identity
//! certificates for the polynomial cores of the 4E test and of the label
//! lemma; and a constant-time variant of `BinaryCurve::is_in_4e`.
//!
//! ```sh
//! cargo run --release --example r6_3_coset_label -- exhaustive --nmin 5 --nmax 25 --out exhaustive.json
//! cargo run --release --example r6_3_coset_label -- sample --ns 131,233,239,283,409,571 \
//!     --quads 2000 --uniform 2000 --seed 63 --out sample.json
//! cargo run --release --example r6_3_coset_label -- bench --ns 131,233,239,283,409,571 \
//!     --batch 1024 --reps 15 --inner 8 --seed 64 --out bench.json
//! cargo run --release --example r6_3_coset_label -- cert --outdir .
//! ```
//!
//! # The label (lemma proved in `RESULTS.md`)
//!
//! Let `P4 = (1, 0)` (order 4, `2·P4 = T = (0, 1)`).  Write `k(P) ∈ Z/4` for
//! the coset of `P` in `E/4E = <P4 + 4E>`.  With `L(O) = (0, 0)`:
//!
//! ```text
//! k = 0 -> (0, 0)     k = 2 -> (0, 1)
//! k = 1 -> (1, (n+1)/2 mod 2)     k = 3 -> (1, (n-1)/2 mod 2)
//! ```
//!
//! The example measures this; it does not assume it.  Cosets are read from
//! `[r]P` (`r = #E/4`, odd) in López–Dahab coordinates: `[r]P = j·P4` and
//! `k = j·r mod 4`.
//!
//! # Field and constant-time code
//!
//! The field `GF(2)[z]/f(z)` is local to this example, `W` words per element
//! (`W = 1` for n <= 64, and `W = ceil(n/64)` for the sampled n), with
//! `pclmulqdq` products (a masked, branch-free software product otherwise) and
//! a reduction that always runs the same number of folds (computed from the
//! public modulus).  It is cross-checked against `crypto_lib::binary_ecc`.
//! Three half-traces are implemented: the byte table with zero-byte skip
//! (the library's method, variable-time), a masked sum over all `n` basis
//! images `H(z^i)` (constant-time), and a masked even-bit folding followed by
//! a masked sum over odd basis images (constant-time; the even-to-odd
//! reduction `H(c^2) = H(c) + c + Tr(c)` is the half-trace trick of Fong,
//! Hankerson, López and Menezes, IEEE TC 2004 — recalled citation).

use std::cell::Cell;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use crypto_lib::binary_ecc::curve::scalar_mul;
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::identity_certificate::{self as idc, Expr, IdentityStatement};
use crypto_lib::hash::sha256::sha256;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

/// Scratch length for an unreduced product: `2·W` words, `W <= 9`.
const SCR: usize = 18;

// ───────────────────────────── tallies ─────────────────────────────

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Counts {
    mul: u64,
    sqr: u64,
    tr: u64,
    ht_bytes_nonzero: u64,
    ht_mask_bits: u64,
    fong_steps: u64,
    early_exit: u64,
    word_ops: u64,
}

#[derive(Clone, Copy)]
enum Kind {
    Mul,
    Sqr,
    Tr,
    HtByte,
    HtMaskBit,
    FongStep,
    EarlyExit,
    Words,
}

trait Tally {
    fn bump(&self, k: Kind, n: u64);
}

struct NoTally;
impl Tally for NoTally {
    #[inline(always)]
    fn bump(&self, _: Kind, _: u64) {}
}

#[derive(Default)]
struct CountTally(Cell<Counts>);
impl Tally for CountTally {
    fn bump(&self, k: Kind, n: u64) {
        let mut c = self.0.get();
        match k {
            Kind::Mul => c.mul += n,
            Kind::Sqr => c.sqr += n,
            Kind::Tr => c.tr += n,
            Kind::HtByte => c.ht_bytes_nonzero += n,
            Kind::HtMaskBit => c.ht_mask_bits += n,
            Kind::FongStep => c.fong_steps += n,
            Kind::EarlyExit => c.early_exit += n,
            Kind::Words => c.word_ops += n,
        }
        self.0.set(c);
    }
}

// ───────────────────────────── the field ─────────────────────────────

/// Branch-free software carry-less product (fallback when `pclmulqdq` is
/// absent): 64 masked shifts, no data-dependent branch.
fn clmul_sw_ct(a: u64, b: u64) -> (u64, u64) {
    let (mut lo, mut hi) = (0u64, 0u64);
    for i in 0..64 {
        let m = 0u64.wrapping_sub((b >> i) & 1);
        lo ^= (a << i) & m;
        hi ^= ((a >> 1) >> (63 - i)) & m;
    }
    (lo, hi)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn mul_words_hw<const W: usize>(a: &[u64; W], b: &[u64; W], c: &mut [u64; SCR]) {
    use std::arch::x86_64::*;
    for i in 0..W {
        let x = _mm_set_epi64x(0, a[i] as i64);
        for j in 0..W {
            let y = _mm_set_epi64x(0, b[j] as i64);
            let z = _mm_clmulepi64_si128::<0x00>(x, y);
            c[i + j] ^= _mm_cvtsi128_si64(z) as u64;
            c[i + j + 1] ^= _mm_cvtsi128_si64(_mm_srli_si128::<8>(z)) as u64;
        }
    }
}

#[inline(always)]
fn spread32(x: u64) -> u64 {
    let mut x = x & 0xFFFF_FFFF;
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333_3333_3333;
    x = (x | (x << 1)) & 0x5555_5555_5555_5555;
    x
}

/// `GF(2)[z]/(z^n + sum_{t in low} z^t)`, `W` words per element.
struct Field<const W: usize> {
    n: u32,
    low: Vec<u32>,
    mask: [u64; W],
    one: [u64; W],
    /// Bit i set iff `Tr(z^i) = 1`.
    tmask: [u64; W],
    /// Reduction folds; fixed per modulus.
    folds: u32,
    nbytes: usize,
    /// `ht_tab[k][v] = H(v·z^{8k})`.
    ht_tab: Vec<[[u64; W]; 256]>,
    /// `ht_basis[i] = H(z^i)`.
    ht_basis: Vec<[u64; W]>,
    /// `H(1) = (n+1)/2 mod 2`.
    h1: u64,
    hw: bool,
    // Word-operation costs of each primitive, counted from its loop bounds.
    wo_reduce: u64,
    wo_mul: u64,
    wo_sqr: u64,
    wo_tr: u64,
}

impl<const W: usize> Field<W> {
    fn new(n: u32, low: &[u32]) -> Self {
        assert!(W <= 9 && n as usize <= 64 * W && n as usize > 64 * (W - 1));
        assert!(n % 2 == 1 && n >= 5);
        let mut mask = [0u64; W];
        for i in 0..n {
            mask[(i / 64) as usize] |= 1u64 << (i % 64);
        }
        let mut one = [0u64; W];
        one[0] = 1;
        let tmax = *low.iter().max().expect("low terms");
        assert!(tmax + 2 <= n, "low terms must stay below n - 1");
        let mut d = 2 * n - 2;
        let mut folds = 0;
        while d >= n {
            d = d - n + tmax;
            folds += 1;
        }
        #[cfg(target_arch = "x86_64")]
        let hw = std::arch::is_x86_feature_detected!("pclmulqdq");
        #[cfg(not(target_arch = "x86_64"))]
        let hw = false;
        let w = W as u64;
        let bs_n = n % 64;
        let hi_ops = if bs_n != 0 { 3 * w } else { w };
        let mut per_fold = hi_ops + 2 * w; // extract, mask low, clear high
        for &t in low {
            per_fold += if t % 64 != 0 { 4 * w } else { 2 * w };
        }
        let wo_reduce = folds as u64 * per_fold;
        let mut f = Field {
            n,
            low: low.to_vec(),
            mask,
            one,
            tmask: [0; W],
            folds,
            nbytes: n.div_ceil(8) as usize,
            ht_tab: Vec::new(),
            ht_basis: Vec::new(),
            h1: ((n.div_ceil(2)) % 2) as u64,
            hw,
            wo_reduce,
            wo_mul: w * w * 3 + wo_reduce,
            wo_sqr: w * 33 + wo_reduce,
            wo_tr: 3 * w + 1,
        };
        for i in 0..n {
            let mut e = [0u64; W];
            e[(i / 64) as usize] = 1u64 << (i % 64);
            // Trace by its squaring chain.
            let mut s = e;
            let mut acc = e;
            for _ in 1..n {
                s = f.sqr(&s, &NoTally);
                acc = xor(&acc, &s);
            }
            assert!(acc == [0; W] || acc == one, "trace not in F_2");
            if acc == one {
                f.tmask[(i / 64) as usize] |= 1u64 << (i % 64);
            }
            // Half-trace by its squaring chain.
            let mut h = e;
            let mut t = e;
            for _ in 0..(n - 1) / 2 {
                t = f.sqr(&f.sqr(&t, &NoTally), &NoTally);
                h = xor(&h, &t);
            }
            f.ht_basis.push(h);
        }
        let mut tab = vec![[[0u64; W]; 256]; f.nbytes];
        for (k, tk) in tab.iter_mut().enumerate() {
            for v in 1..256usize {
                let j = v.trailing_zeros() as usize;
                let bit = 8 * k + j;
                let b = if bit < n as usize {
                    f.ht_basis[bit]
                } else {
                    [0; W]
                };
                tk[v] = xor(&tk[v & (v - 1)], &b);
            }
        }
        f.ht_tab = tab;
        f
    }

    #[inline(always)]
    fn reduce(&self, mut c: [u64; SCR]) -> [u64; W] {
        let n = self.n as usize;
        let (ws, bs) = (n / 64, (n % 64) as u32);
        for _ in 0..self.folds {
            let mut hi = [0u64; W];
            for (i, h) in hi.iter_mut().enumerate() {
                let j = i + ws;
                let mut w = c[j] >> bs;
                if bs != 0 {
                    w |= c[j + 1] << (64 - bs);
                }
                *h = w;
            }
            for i in 0..W {
                c[i] &= self.mask[i];
                c[W + i] = 0;
            }
            for &t in &self.low {
                let (tw, tb) = ((t / 64) as usize, t % 64);
                for (i, &h) in hi.iter().enumerate() {
                    c[i + tw] ^= h << tb;
                    if tb != 0 {
                        c[i + tw + 1] ^= h >> (64 - tb);
                    }
                }
            }
        }
        debug_assert!(c[W..2 * W].iter().all(|&w| w == 0));
        let mut out = [0u64; W];
        out.copy_from_slice(&c[..W]);
        out
    }

    #[inline(always)]
    fn mul<T: Tally>(&self, a: &[u64; W], b: &[u64; W], t: &T) -> [u64; W] {
        t.bump(Kind::Mul, 1);
        t.bump(Kind::Words, self.wo_mul);
        let mut c = [0u64; SCR];
        #[cfg(target_arch = "x86_64")]
        if self.hw {
            // SAFETY: pclmulqdq availability was checked in `Field::new`.
            unsafe { mul_words_hw::<W>(a, b, &mut c) };
            return self.reduce(c);
        }
        for i in 0..W {
            for j in 0..W {
                let (lo, hi) = clmul_sw_ct(a[i], b[j]);
                c[i + j] ^= lo;
                c[i + j + 1] ^= hi;
            }
        }
        self.reduce(c)
    }

    #[inline(always)]
    fn sqr<T: Tally>(&self, a: &[u64; W], t: &T) -> [u64; W] {
        t.bump(Kind::Sqr, 1);
        t.bump(Kind::Words, self.wo_sqr);
        let mut c = [0u64; SCR];
        for i in 0..W {
            c[2 * i] = spread32(a[i]);
            c[2 * i + 1] = spread32(a[i] >> 32);
        }
        self.reduce(c)
    }

    /// Itoh–Tsujii `a^{2^n - 2}` (variable-time; used only to generate
    /// points and in the ground truth).
    fn inv(&self, a: &[u64; W]) -> [u64; W] {
        assert!(!is_zero(a));
        let m = self.n - 1;
        let bits = 32 - m.leading_zeros();
        let mut beta = *a;
        let mut len = 1u32;
        for i in (0..bits - 1).rev() {
            let mut s = beta;
            for _ in 0..len {
                s = self.sqr(&s, &NoTally);
            }
            beta = self.mul(&beta, &s, &NoTally);
            len *= 2;
            if m >> i & 1 == 1 {
                beta = self.mul(&self.sqr(&beta, &NoTally), a, &NoTally);
                len += 1;
            }
        }
        self.sqr(&beta, &NoTally)
    }

    /// Absolute trace as 0/1: parity of `a AND tmask` (branch-free).
    #[inline(always)]
    fn trace<T: Tally>(&self, a: &[u64; W], t: &T) -> u64 {
        t.bump(Kind::Tr, 1);
        t.bump(Kind::Words, self.wo_tr);
        let mut p = 0u32;
        for i in 0..W {
            p ^= (a[i] & self.tmask[i]).count_ones();
        }
        (p & 1) as u64
    }

    /// Byte-table half-trace with zero-byte skip (the library's method;
    /// variable-time: the table index and the skip depend on `a`).
    #[inline(always)]
    fn ht_tab<T: Tally>(&self, a: &[u64; W], t: &T) -> [u64; W] {
        let mut out = [0u64; W];
        t.bump(Kind::Words, 3 * self.nbytes as u64);
        for k in 0..self.nbytes {
            let byte = ((a[k / 8] >> (8 * (k % 8))) & 0xff) as usize;
            if byte == 0 {
                continue;
            }
            t.bump(Kind::HtByte, 1);
            t.bump(Kind::Words, W as u64 + 2);
            let e = &self.ht_tab[k][byte];
            for w in 0..W {
                out[w] ^= e[w];
            }
        }
        out
    }

    /// Constant-time half-trace: `sum_i a_i · H(z^i)` with masks; every
    /// basis image is read in a fixed order whatever `a` is.
    #[inline(always)]
    fn ht_mask<T: Tally>(&self, a: &[u64; W], t: &T) -> [u64; W] {
        let mut out = [0u64; W];
        t.bump(Kind::HtMaskBit, self.n as u64);
        t.bump(Kind::Words, self.n as u64 * (3 + 2 * W as u64));
        for (i, b) in self.ht_basis.iter().enumerate() {
            let m = 0u64.wrapping_sub((a[i / 64] >> (i % 64)) & 1);
            for w in 0..W {
                out[w] ^= b[w] & m;
            }
        }
        out
    }

    /// Constant-time half-trace with the even-bit fold: for j = (n-1)/2
    /// down to 1, `H(a) = H(a + a_{2j}(z^{2j} + z^j)) + a_{2j}(z^j + Tr(z^j))`,
    /// then a masked sum over the odd basis images and `H(1)`.
    #[inline(always)]
    fn ht_fong<T: Tally>(&self, a: &[u64; W], t: &T) -> [u64; W] {
        let half = ((self.n - 1) / 2) as usize;
        t.bump(Kind::FongStep, half as u64);
        t.bump(Kind::HtMaskBit, half as u64);
        t.bump(
            Kind::Words,
            half as u64 * 13 + half as u64 * (3 + 2 * W as u64) + 6,
        );
        let mut a = *a;
        let mut out = [0u64; W];
        let mut c = 0u64;
        for j in (1..=half).rev() {
            let e = 2 * j;
            let b = (a[e / 64] >> (e % 64)) & 1;
            let m = 0u64.wrapping_sub(b);
            a[e / 64] ^= m & (1u64 << (e % 64));
            a[j / 64] ^= m & (1u64 << (j % 64));
            out[j / 64] ^= m & (1u64 << (j % 64));
            c ^= b & ((self.tmask[j / 64] >> (j % 64)) & 1);
        }
        out[0] ^= c ^ ((a[0] & 1) & self.h1);
        for i in (1..self.n as usize).step_by(2) {
            let m = 0u64.wrapping_sub((a[i / 64] >> (i % 64)) & 1);
            let b = &self.ht_basis[i];
            for w in 0..W {
                out[w] ^= b[w] & m;
            }
        }
        out
    }

    fn random(&self, rng: &mut StdRng) -> [u64; W] {
        let mut a = [0u64; W];
        for (w, m) in a.iter_mut().zip(&self.mask) {
            *w = rng.gen::<u64>() & m;
        }
        a
    }

    fn to_lib(&self, a: &[u64; W]) -> F2mElement {
        F2mElement::from_words(a, self.n)
    }

    fn lib_to_words(&self, e: &F2mElement) -> [u64; W] {
        let mut a = [0u64; W];
        for (w, b) in a.iter_mut().zip(e.raw_bits()) {
            *w = *b;
        }
        a
    }
}

#[inline(always)]
fn xor<const W: usize>(a: &[u64; W], b: &[u64; W]) -> [u64; W] {
    let mut o = [0u64; W];
    for i in 0..W {
        o[i] = a[i] ^ b[i];
    }
    o
}

fn is_zero<const W: usize>(a: &[u64; W]) -> bool {
    a.iter().all(|&w| w == 0)
}

// ───────────────────────────── the curve ─────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pt<const W: usize> {
    x: [u64; W],
    y: [u64; W],
    inf: bool,
}

impl<const W: usize> Pt<W> {
    fn o() -> Self {
        Pt {
            x: [0; W],
            y: [0; W],
            inf: true,
        }
    }
    fn aff(x: [u64; W], y: [u64; W]) -> Self {
        Pt { x, y, inf: false }
    }
}

fn on_curve<const W: usize>(f: &Field<W>, p: &Pt<W>) -> bool {
    if p.inf {
        return true;
    }
    let lhs = xor(&f.sqr(&p.y, &NoTally), &f.mul(&p.x, &p.y, &NoTally));
    let rhs = xor(&f.mul(&f.sqr(&p.x, &NoTally), &p.x, &NoTally), &f.one);
    lhs == rhs
}

fn p4<const W: usize>(f: &Field<W>) -> Pt<W> {
    Pt::aff(f.one, [0; W])
}

/// Affine addition (variable-time; generation and checks only).
fn add_aff<const W: usize>(f: &Field<W>, p: &Pt<W>, q: &Pt<W>) -> Pt<W> {
    if p.inf {
        return *q;
    }
    if q.inf {
        return *p;
    }
    let dx = xor(&p.x, &q.x);
    if is_zero(&dx) {
        if p.y == q.y {
            return dbl_aff(f, p);
        }
        return Pt::o();
    }
    let lam = f.mul(&xor(&p.y, &q.y), &f.inv(&dx), &NoTally);
    let x3 = xor(&xor(&f.sqr(&lam, &NoTally), &lam), &dx);
    let y3 = xor(&xor(&f.mul(&lam, &xor(&p.x, &x3), &NoTally), &x3), &p.y);
    Pt::aff(x3, y3)
}

fn dbl_aff<const W: usize>(f: &Field<W>, p: &Pt<W>) -> Pt<W> {
    if p.inf || is_zero(&p.x) {
        return Pt::o();
    }
    let lam = xor(&p.x, &f.mul(&p.y, &f.inv(&p.x), &NoTally));
    let x3 = xor(&f.sqr(&lam, &NoTally), &lam);
    let y3 = xor(
        &f.sqr(&p.x, &NoTally),
        &f.mul(&xor(&lam, &f.one), &x3, &NoTally),
    );
    Pt::aff(x3, y3)
}

/// `[k]P` by left-to-right double-and-add in López–Dahab coordinates
/// (`x = X/Z`, `y = Y/Z^2`), a = 0, b = 1.  Returns `j` with
/// `[k]P = j·P4`, or `None` if `[k]P` is not 4-torsion.
fn ld_mul_class<const W: usize>(f: &Field<W>, p: &Pt<W>, kbits: &[bool]) -> Option<u32> {
    if p.inf {
        return Some(0);
    }
    let t = &NoTally;
    let (mut x, mut y, mut z) = ([0u64; W], [0u64; W], [0u64; W]);
    let mut started = false;
    for &bit in kbits {
        if started && !is_zero(&z) {
            // Doubling: Z3 = X^2 Z^2, X3 = X^4 + Z^4, Y3 = Z^4 Z3 + X3 (Y^2 + Z^4).
            let x2 = f.sqr(&x, t);
            let z2 = f.sqr(&z, t);
            let z4 = f.sqr(&z2, t);
            let z3 = f.mul(&x2, &z2, t);
            let x3 = xor(&f.sqr(&x2, t), &z4);
            let y3 = xor(
                &f.mul(&z4, &z3, t),
                &f.mul(&x3, &xor(&f.sqr(&y, t), &z4), t),
            );
            x = x3;
            y = y3;
            z = z3;
        }
        if bit {
            if !started || is_zero(&z) {
                x = p.x;
                y = p.y;
                z = f.one;
                started = true;
            } else {
                // Mixed addition with the affine base point.
                let z2 = f.sqr(&z, t);
                let a = xor(&y, &f.mul(&p.y, &z2, t));
                let b = xor(&x, &f.mul(&p.x, &z, t));
                if is_zero(&b) {
                    if is_zero(&a) {
                        // R = P: double the affine point.
                        let d = dbl_aff(f, p);
                        if d.inf {
                            z = [0; W];
                        } else {
                            x = d.x;
                            y = d.y;
                            z = f.one;
                        }
                    } else {
                        z = [0; W];
                    }
                    continue;
                }
                let c = f.mul(&z, &b, t);
                let z3 = f.sqr(&c, t);
                let x3 = xor(&f.sqr(&a, t), &f.mul(&c, &xor(&a, &f.sqr(&b, t)), t));
                let d = f.mul(&p.x, &z3, t);
                let y3 = xor(
                    &xor(
                        &f.mul(&f.mul(&a, &c, t), &xor(&d, &x3), t),
                        &f.mul(&x3, &z3, t),
                    ),
                    &f.mul(&p.y, &f.sqr(&z3, t), t),
                );
                x = x3;
                y = y3;
                z = z3;
            }
        }
    }
    if !started || is_zero(&z) {
        return Some(0);
    }
    if is_zero(&x) {
        return Some(2);
    }
    if x == z {
        if is_zero(&y) {
            return Some(1);
        }
        if y == f.sqr(&z, t) {
            return Some(3);
        }
    }
    None
}

fn bits_msb(k: &BigUint) -> Vec<bool> {
    let nb = k.bits();
    (0..nb).rev().map(|i| k.bit(i)).collect()
}

/// `#E(F_{2^n})` for `y^2 + xy = x^3 + 1`: `t_0 = 2`, `t_1 = -1`,
/// `t_m = -t_{m-1} - 2 t_{m-2}`, `#E = 2^n + 1 - t_n`.
fn koblitz_order(n: u32) -> (BigUint, String) {
    use num_bigint::BigInt;
    let (mut prev, mut tr) = (BigInt::from(2), BigInt::from(-1));
    for _ in 1..n {
        let next = -&tr - (&prev << 1u32);
        prev = tr;
        tr = next;
    }
    let count = (BigInt::one() << n) + BigInt::one() - &tr;
    (count.to_biguint().expect("positive"), tr.to_string())
}

// ───────────────────────── the 4E tests and labels ─────────────────────────

/// The library's algorithm on this example's field: on-curve check, early
/// exit on `Tr(x) = 1`, byte-table half-trace, final trace.
#[inline(always)]
fn is_in_4e_tab<const W: usize, T: Tally>(f: &Field<W>, p: &Pt<W>, t: &T) -> bool {
    if p.inf {
        return true;
    }
    let lhs = xor(&f.sqr(&p.y, t), &f.mul(&p.x, &p.y, t));
    let rhs = xor(&f.mul(&f.sqr(&p.x, t), &p.x, t), &f.one);
    t.bump(Kind::Words, 3 * W as u64);
    if lhs != rhs {
        return false;
    }
    if f.trace(&p.x, t) != 0 {
        t.bump(Kind::EarlyExit, 1);
        return false;
    }
    let h = f.ht_tab(&p.x, t);
    let s = xor(&p.y, &f.mul(&p.x, &h, t));
    t.bump(Kind::Words, W as u64);
    f.trace(&s, t) == 0
}

/// Constant-time 4E test and label.  No branch, early exit or table index
/// depends on the coordinates; the point at infinity is an input bit and is
/// folded in with masks.  Returns `(member, Tr(x), Tr(y + x H(x)))`, the
/// label being `(0, 0)` at `O`.
#[inline(always)]
fn is_in_4e_ct<const W: usize, const FONG: bool, T: Tally>(
    f: &Field<W>,
    x: &[u64; W],
    y: &[u64; W],
    inf: u64,
    t: &T,
) -> (u64, u64, u64) {
    let lhs = xor(&f.sqr(y, t), &f.mul(x, y, t));
    let rhs = xor(&f.mul(&f.sqr(x, t), x, t), &f.one);
    let mut acc = 0u64;
    for i in 0..W {
        acc |= lhs[i] ^ rhs[i];
    }
    // Coordinates must be reduced (bits at or above n clear).
    acc |= (x[W - 1] | y[W - 1]) & !f.mask[W - 1];
    let nonzero = (acc | acc.wrapping_neg()) >> 63;
    let on = 1 ^ nonzero;
    let tx = f.trace(x, t);
    let h = if FONG {
        f.ht_fong(x, t)
    } else {
        f.ht_mask(x, t)
    };
    let s = xor(y, &f.mul(x, &h, t));
    let t2 = f.trace(&s, t);
    t.bump(Kind::Words, 4 * W as u64 + 12);
    let fin = 1 ^ inf;
    let member = inf | (on & (1 ^ tx) & (1 ^ t2) & fin);
    (member, tx & fin, t2 & fin)
}

fn label_tab<const W: usize>(f: &Field<W>, p: &Pt<W>) -> (u64, u64) {
    if p.inf {
        return (0, 0);
    }
    let h = f.ht_tab(&p.x, &NoTally);
    (
        f.trace(&p.x, &NoTally),
        f.trace(&xor(&p.y, &f.mul(&p.x, &h, &NoTally)), &NoTally),
    )
}

/// Label by the library's own `trace`/`half_trace`/`mul`.
fn label_lib(c: &BinaryCurve, p: &BinaryPoint) -> (u64, u64) {
    match p {
        BinaryPoint::Infinity => (0, 0),
        BinaryPoint::Affine { x, y } => {
            let irr = &c.irreducible;
            let h = x.half_trace(irr).expect("odd n");
            (
                x.trace(irr) as u64,
                y.add(&x.mul(&h, irr)).trace(irr) as u64,
            )
        }
    }
}

fn to_lib_pt<const W: usize>(f: &Field<W>, p: &Pt<W>) -> BinaryPoint {
    if p.inf {
        BinaryPoint::Infinity
    } else {
        BinaryPoint::Affine {
            x: f.to_lib(&p.x),
            y: f.to_lib(&p.y),
        }
    }
}

/// Predicted label of coset k (the lemma).
fn predicted(n: u32, k: u32) -> (u64, u64) {
    let up = ((n.div_ceil(2)) % 2) as u64;
    let dn = (((n - 1) / 2) % 2) as u64;
    match k {
        0 => (0, 0),
        1 => (1, up),
        2 => (0, 1),
        _ => (1, dn),
    }
}

fn lib_curve_generic(n: u32, irr: IrreduciblePoly, r: &BigUint) -> BinaryCurve {
    BinaryCurve {
        m: n,
        irreducible: irr,
        a: F2mElement::zero(n),
        b: F2mElement::one(n),
        generator: BinaryPoint::Affine {
            x: F2mElement::zero(n),
            y: F2mElement::one(n),
        },
        order: r.clone(),
        cofactor: BigUint::from(4u32),
    }
}

// ───────────────────────── small irreducibles ─────────────────────────

fn pmulmod(a: u64, b: u64, f: u64, n: u32) -> u64 {
    let mut r = 0u64;
    let mut a = a;
    for i in 0..n {
        if b >> i & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        if a >> n & 1 == 1 {
            a ^= f;
        }
    }
    r
}

fn pgcd(mut a: u64, mut b: u64) -> u64 {
    let deg = |v: u64| 63 - v.leading_zeros() as i32;
    while b != 0 {
        while a != 0 && deg(a) >= deg(b) {
            a ^= b << (deg(a) - deg(b));
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// Rabin's test for `f` of degree `n <= 40`.
fn irreducible(f: u64, n: u32) -> bool {
    let frob = |k: u32| {
        let mut z = 2u64;
        for _ in 0..k {
            z = pmulmod(z, z, f, n);
        }
        z
    };
    if frob(n) != 2 {
        return false;
    }
    let primes: Vec<u32> = (2..=n)
        .filter(|q| n.is_multiple_of(*q) && (2..*q).all(|d| q % d != 0))
        .collect();
    primes.iter().all(|&q| pgcd(f, frob(n / q) ^ 2) == 1)
}

/// The first irreducible trinomial `z^n + z^k + 1`, `k <= n/2`, else the
/// first pentanomial.
fn small_modulus(n: u32) -> Vec<u32> {
    for k in 1..=n / 2 {
        if irreducible((1u64 << n) | (1u64 << k) | 1, n) {
            return vec![0, k];
        }
    }
    for k3 in 3..n / 2 {
        for k2 in 2..k3 {
            for k1 in 1..k2 {
                let f = (1u64 << n) | (1 << k3) | (1 << k2) | (1 << k1) | 1;
                if irreducible(f, n) {
                    return vec![0, k1, k2, k3];
                }
            }
        }
    }
    panic!("no modulus for n = {n}");
}

// ───────────────────────────── exhaustive ─────────────────────────────

fn exhaustive_one(n: u32, rng: &mut StdRng) -> Value {
    let t0 = Instant::now();
    let low = small_modulus(n);
    let f = Field::<1>::new(n, &low);
    let irr = IrreduciblePoly {
        degree: n,
        low_terms: low.clone(),
    };
    let (ecount, tn) = koblitz_order(n);
    let r = &ecount >> 2u32;
    assert!(r.bit(0), "#E/4 must be odd");
    let r_mod4 = (&r % 4u32).to_u32().unwrap();
    let rbits = bits_msb(&r);
    let lib = lib_curve_generic(n, irr, &r);
    assert_eq!(lib.is_in_4e(&BinaryPoint::Infinity), Some(true));

    let mut coset_size = [0u64; 4];
    // hist[k][label index 2·l0 + l1]
    let mut hist = [[0u64; 4]; 4];
    let mut not_4torsion = 0u64;
    let mut lib_vs_truth = 0u64;
    let mut tab_vs_lib = 0u64;
    let mut ctm_vs_lib = 0u64;
    let mut ctf_vs_lib = 0u64;
    let mut label_lib_vs_tab = 0u64;
    let mut label_lib_vs_ct = 0u64;
    let mut ht_variants_disagree = 0u64;
    let mut step_checked = 0u64;
    let mut step_violations = 0u64;
    let mut core_identity_violations = 0u64;
    let mut stream: Vec<u8> = Vec::new();
    let mut nib: Vec<u8> = Vec::with_capacity(1 << 24);
    let mut count = 0u64;
    let pp4 = p4(&f);
    let step_const = (((n - 1) / 2) % 2) as u64;

    let mut visit =
        |p: &Pt<1>, coset_size: &mut [u64; 4], hist: &mut [[u64; 4]; 4], nib: &mut Vec<u8>| {
            count += 1;
            let j = match ld_mul_class(&f, p, &rbits) {
                Some(j) => j,
                None => {
                    not_4torsion += 1;
                    return;
                }
            };
            let k = (j * r_mod4) % 4;
            coset_size[k as usize] += 1;
            let lp = to_lib_pt(&f, p);
            let (l0, l1) = label_lib(&lib, &lp);
            hist[k as usize][(2 * l0 + l1) as usize] += 1;
            let truth = k == 0;
            let libv = lib.is_in_4e(&lp) == Some(true);
            lib_vs_truth += (libv != truth) as u64;
            tab_vs_lib += (is_in_4e_tab(&f, p, &NoTally) != libv) as u64;
            let inf = p.inf as u64;
            let (mm, a0, a1) = is_in_4e_ct::<1, false, _>(&f, &p.x, &p.y, inf, &NoTally);
            let (mf, b0, b1) = is_in_4e_ct::<1, true, _>(&f, &p.x, &p.y, inf, &NoTally);
            ctm_vs_lib += ((mm == 1) != libv) as u64;
            ctf_vs_lib += ((mf == 1) != libv) as u64;
            label_lib_vs_tab += (label_tab(&f, p) != (l0, l1)) as u64;
            label_lib_vs_ct += ((a0, a1) != (l0, l1) || (b0, b1) != (l0, l1)) as u64;
            if !p.inf {
                let h1 = f.ht_tab(&p.x, &NoTally);
                if h1 != f.ht_mask(&p.x, &NoTally) || h1 != f.ht_fong(&p.x, &NoTally) {
                    ht_variants_disagree += 1;
                }
            }
            // The proof's step: for odd P != ±P4, b(P) = b(P + P4) + (n-1)/2,
            // and the core identity x l^2 + x l + l = g^2 + g + 1, g = x + l,
            // l = y/(x+1).
            if !p.inf && l0 == 1 && p.x != f.one {
                step_checked += 1;
                let q = add_aff(&f, p, &pp4);
                let (_, m1) = label_lib(&lib, &to_lib_pt(&f, &q));
                if l1 != m1 ^ step_const {
                    step_violations += 1;
                }
                let l = f.mul(&p.y, &f.inv(&xor(&p.x, &f.one)), &NoTally);
                let lhs = xor(
                    &xor(
                        &f.mul(&p.x, &f.sqr(&l, &NoTally), &NoTally),
                        &f.mul(&p.x, &l, &NoTally),
                    ),
                    &l,
                );
                let g = xor(&p.x, &l);
                let rhs = xor(&xor(&f.sqr(&g, &NoTally), &g), &f.one);
                if lhs != rhs {
                    core_identity_violations += 1;
                }
            }
            nib.push((((2 * l0 + l1) << 2) | k as u64) as u8);
        };

    visit(&Pt::o(), &mut coset_size, &mut hist, &mut nib);
    let size = 1u64 << n;
    for xv in 0..size {
        let x = [xv];
        if xv == 0 {
            visit(&Pt::aff([0], [1]), &mut coset_size, &mut hist, &mut nib);
            continue;
        }
        let xi = f.inv(&x);
        let c = xor(&x, &f.sqr(&xi, &NoTally));
        if f.trace(&c, &NoTally) != 0 {
            continue;
        }
        let zz = f.ht_tab(&c, &NoTally);
        let y1 = f.mul(&x, &zz, &NoTally);
        let y2 = xor(&y1, &x);
        visit(&Pt::aff(x, y1), &mut coset_size, &mut hist, &mut nib);
        visit(&Pt::aff(x, y2), &mut coset_size, &mut hist, &mut nib);
    }
    for pair in nib.chunks(2) {
        stream.push((pair[0] << 4) | pair.get(1).copied().unwrap_or(0));
    }
    let digest = hex::encode(sha256(&stream));

    // Off-curve inputs: the constant-time test must reject exactly what the
    // library rejects.
    let mut offcurve_tested = 0u64;
    let mut offcurve_disagree = 0u64;
    for _ in 0..20000 {
        let p = Pt::aff(f.random(rng), f.random(rng));
        if on_curve(&f, &p) {
            continue;
        }
        offcurve_tested += 1;
        let libv = lib.is_in_4e(&to_lib_pt(&f, &p)) == Some(true);
        let (mm, _, _) = is_in_4e_ct::<1, false, _>(&f, &p.x, &p.y, 0, &NoTally);
        let (mf, _, _) = is_in_4e_ct::<1, true, _>(&f, &p.x, &p.y, 0, &NoTally);
        offcurve_disagree += ((mm == 1) != libv || (mf == 1) != libv || libv) as u64;
    }

    let total: u64 = coset_size.iter().sum();
    let mut labels = Vec::new();
    let mut constant = true;
    let mut matches_pred = true;
    for k in 0..4u32 {
        let row = hist[k as usize];
        let nz: Vec<usize> = (0..4).filter(|&i| row[i] > 0).collect();
        let is_const = nz.len() == 1;
        constant &= is_const;
        let lab = if is_const {
            Some(((nz[0] >> 1) as u64, (nz[0] & 1) as u64))
        } else {
            None
        };
        matches_pred &= lab == Some(predicted(n, k));
        labels.push(json!({
            "coset": k,
            "size": coset_size[k as usize],
            "label_histogram_00_01_10_11": row,
            "constant": is_const,
            "label": lab.map(|(a, b)| vec![a, b]),
            "predicted": [predicted(n, k).0, predicted(n, k).1],
        }));
    }
    let rec = json!({
        "n": n,
        "n_mod_4": n % 4,
        "n_mod_8": n % 8,
        "modulus_low_terms": low,
        "points_enumerated": count,
        "count_formula": ecount.to_string(),
        "frobenius_trace": tn,
        "count_matches": BigUint::from(total + not_4torsion) == ecount && count == total + not_4torsion,
        "r": r.to_string(),
        "r_mod_4": r_mod4,
        "not_4torsion_after_r": not_4torsion,
        "cosets": labels,
        "label_constant_on_every_coset": constant,
        "labels_match_lemma": matches_pred,
        "lib_is_in_4e_vs_truth_disagree": lib_vs_truth,
        "tab_vs_lib_disagree": tab_vs_lib,
        "ct_mask_vs_lib_disagree": ctm_vs_lib,
        "ct_fong_vs_lib_disagree": ctf_vs_lib,
        "label_lib_vs_tab_disagree": label_lib_vs_tab,
        "label_lib_vs_ct_disagree": label_lib_vs_ct,
        "halftrace_variants_disagree": ht_variants_disagree,
        "proof_step_checked_points": step_checked,
        "proof_step_violations": step_violations,
        "core_identity_violations": core_identity_violations,
        "offcurve_tested": offcurve_tested,
        "offcurve_disagree_or_accepted": offcurve_disagree,
        "stream_sha256": digest,
        "stream_format": "one nibble per point (label 2·l0+l1 in bits 3..2, coset k in bits 1..0), O first, then x = 0..2^n-1 ascending, y = x·H(x + x^-2) before y + x; two nibbles per byte, high first",
        "seconds": t0.elapsed().as_secs_f64(),
    });
    eprintln!(
        "n={n:2} points={count} const={constant} lemma={matches_pred} lib/truth={lib_vs_truth} ct={ctm_vs_lib}/{ctf_vs_lib} step={step_violations}/{step_checked} {:.1}s",
        t0.elapsed().as_secs_f64()
    );
    rec
}

// ───────────────────────────── sampling ─────────────────────────────

fn random_point<const W: usize>(f: &Field<W>, rng: &mut StdRng) -> Pt<W> {
    loop {
        let x = f.random(rng);
        if is_zero(&x) {
            continue;
        }
        let xi = f.inv(&x);
        let c = xor(&x, &f.sqr(&xi, &NoTally));
        if f.trace(&c, &NoTally) != 0 {
            continue;
        }
        let z = f.ht_tab(&c, &NoTally);
        let mut y = f.mul(&x, &z, &NoTally);
        if rng.gen::<bool>() {
            y = xor(&y, &x);
        }
        return Pt::aff(x, y);
    }
}

struct LargeN {
    n: u32,
    low: Vec<u32>,
    curve: BinaryCurve,
    name: &'static str,
}

fn large(n: u32) -> LargeN {
    let (curve, name) = match n {
        131 => {
            let (e, _) = koblitz_order(131);
            (
                lib_curve_generic(131, IrreduciblePoly::deg_131(), &(e >> 2u32)),
                "ECC2K-130 curve (a = 0, b = 1, deg_131 modulus)",
            )
        }
        233 => (BinaryCurve::sect233k1(), "sect233k1"),
        239 => (BinaryCurve::sect239k1(), "sect239k1"),
        283 => (BinaryCurve::sect283k1(), "sect283k1"),
        409 => (BinaryCurve::sect409k1(), "sect409k1"),
        571 => (BinaryCurve::sect571k1(), "sect571k1"),
        _ => panic!("unsupported n = {n}"),
    };
    LargeN {
        n,
        low: curve.irreducible.low_terms.clone(),
        curve,
        name,
    }
}

fn sample_one<const W: usize>(ln: &LargeN, quads: usize, uniform: usize, seed: u64) -> Value {
    let t0 = Instant::now();
    let n = ln.n;
    let f = Field::<W>::new(n, &ln.low);
    let lib = &ln.curve;
    let irr = &lib.irreducible;
    let mut rng = StdRng::seed_from_u64(seed ^ n as u64);
    // Field cross-check.
    let mut field_ok = true;
    for _ in 0..2000 {
        let a = f.random(&mut rng);
        let b = f.random(&mut rng);
        let (la, lb) = (f.to_lib(&a), f.to_lib(&b));
        field_ok &= f.lib_to_words(&la.mul(&lb, irr)) == f.mul(&a, &b, &NoTally);
        field_ok &= f.lib_to_words(&la.square(irr)) == f.sqr(&a, &NoTally);
        field_ok &= la.trace(irr) as u64 == f.trace(&a, &NoTally);
        let h = f.lib_to_words(&la.half_trace(irr).unwrap());
        field_ok &= h == f.ht_tab(&a, &NoTally);
        field_ok &= h == f.ht_mask(&a, &NoTally);
        field_ok &= h == f.ht_fong(&a, &NoTally);
        let tr = f.trace(&a, &NoTally);
        let mut want = a;
        want[0] ^= tr;
        field_ok &= xor(&f.sqr(&h, &NoTally), &h) == want;
    }
    let (ecount, _) = koblitz_order(n);
    let r = &ecount >> 2u32;
    let r_mod4 = (&r % 4u32).to_u32().unwrap();
    let rbits = bits_msb(&r);
    let order_matches_lib = lib.order == r && lib.cofactor == BigUint::from(4u32);
    let fast = lib.has_fast_4e_subgroup_check();

    let pp4 = p4(&f);
    let mut pts: Vec<(Pt<W>, Option<u32>)> = Vec::new();
    for _ in 0..quads {
        let p = random_point(&f, &mut rng);
        let q = dbl_aff(&f, &dbl_aff(&f, &p));
        let mut acc = q;
        for k in 0..4u32 {
            pts.push((acc, Some(k)));
            acc = add_aff(&f, &acc, &pp4);
        }
    }
    for _ in 0..uniform {
        pts.push((random_point(&f, &mut rng), None));
    }
    let mut hist = [[0u64; 4]; 4];
    let mut size = [0u64; 4];
    let mut construct_vs_r = 0u64;
    let mut not_4torsion = 0u64;
    let mut lib_vs_truth = 0u64;
    let mut ct_vs_lib = 0u64;
    let mut label_ct_vs_lib = 0u64;
    let mut on_curve_fail = 0u64;
    for (p, ck) in &pts {
        on_curve_fail += (!on_curve(&f, p)) as u64;
        let j = match ld_mul_class(&f, p, &rbits) {
            Some(j) => j,
            None => {
                not_4torsion += 1;
                continue;
            }
        };
        let k = (j * r_mod4) % 4;
        if let Some(c) = ck {
            construct_vs_r += (*c != k) as u64;
        }
        size[k as usize] += 1;
        let lp = to_lib_pt(&f, p);
        let (l0, l1) = label_lib(lib, &lp);
        hist[k as usize][(2 * l0 + l1) as usize] += 1;
        let libv = lib.is_in_4e(&lp) == Some(true);
        lib_vs_truth += (libv != (k == 0)) as u64;
        let inf = p.inf as u64;
        let (mm, a0, a1) = is_in_4e_ct::<W, false, _>(&f, &p.x, &p.y, inf, &NoTally);
        let (mf, b0, b1) = is_in_4e_ct::<W, true, _>(&f, &p.x, &p.y, inf, &NoTally);
        ct_vs_lib += ((mm == 1) != libv || (mf == 1) != libv) as u64;
        label_ct_vs_lib += ((a0, a1) != (l0, l1) || (b0, b1) != (l0, l1)) as u64;
    }
    // Ground truth cross-check against the library's affine scalar_mul.
    let mut lib_scalar_checked = 0u64;
    let mut lib_scalar_disagree = 0u64;
    for (p, _) in pts.iter().step_by(pts.len() / 8).take(8) {
        let lp = to_lib_pt(&f, p);
        let rp = scalar_mul(lib, &lp, &r);
        let j = ld_mul_class(&f, p, &rbits);
        let want = match &rp {
            BinaryPoint::Infinity => Some(0),
            BinaryPoint::Affine { x, y } => {
                let (x, y) = (f.lib_to_words(x), f.lib_to_words(y));
                if is_zero(&x) {
                    Some(2)
                } else if x == f.one && is_zero(&y) {
                    Some(1)
                } else if x == f.one && y == f.one {
                    Some(3)
                } else {
                    None
                }
            }
        };
        lib_scalar_checked += 1;
        lib_scalar_disagree += (want != j) as u64;
    }
    let mut cosets = Vec::new();
    let mut constant = true;
    let mut matches_pred = true;
    for k in 0..4u32 {
        let row = hist[k as usize];
        let nz: Vec<usize> = (0..4).filter(|&i| row[i] > 0).collect();
        let is_const = nz.len() == 1;
        constant &= is_const;
        let lab = is_const.then(|| ((nz[0] >> 1) as u64, (nz[0] & 1) as u64));
        matches_pred &= lab == Some(predicted(n, k));
        cosets.push(json!({
            "coset": k,
            "size": size[k as usize],
            "label_histogram_00_01_10_11": row,
            "constant": is_const,
            "label": lab.map(|(a, b)| vec![a, b]),
            "predicted": [predicted(n, k).0, predicted(n, k).1],
        }));
    }
    let rec = json!({
        "n": n,
        "curve": ln.name,
        "n_mod_4": n % 4,
        "n_mod_8": n % 8,
        "modulus_low_terms": ln.low,
        "words": W,
        "field_vs_library_ok": field_ok,
        "r": r.to_string(),
        "r_mod_4": r_mod4,
        "library_order_and_cofactor_match": order_matches_lib,
        "library_has_fast_4e_subgroup_check": fast,
        "points": pts.len(),
        "balanced_quadruples": quads,
        "uniform_points": uniform,
        "on_curve_failures": on_curve_fail,
        "not_4torsion_after_r": not_4torsion,
        "constructed_coset_vs_r_disagree": construct_vs_r,
        "cosets": cosets,
        "label_constant_on_every_coset": constant,
        "labels_match_lemma": matches_pred,
        "lib_is_in_4e_vs_truth_disagree": lib_vs_truth,
        "ct_vs_lib_disagree": ct_vs_lib,
        "label_ct_vs_lib_disagree": label_ct_vs_lib,
        "lib_scalar_mul_checked": lib_scalar_checked,
        "lib_scalar_mul_disagree": lib_scalar_disagree,
        "seconds": t0.elapsed().as_secs_f64(),
    });
    eprintln!(
        "n={n} {} pts={} const={constant} lemma={matches_pred} construct/r={construct_vs_r} lib/truth={lib_vs_truth} ct/lib={ct_vs_lib} {:.1}s",
        ln.name,
        pts.len(),
        t0.elapsed().as_secs_f64()
    );
    rec
}

// ───────────────────────────── benchmark ─────────────────────────────

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        0.5 * (v[m - 1] + v[m])
    }
}

fn quantile(v: &[f64], q: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((s.len() - 1) as f64 * q).round() as usize;
    s[i]
}

fn counts_json(c: &Counts, calls: u64) -> Value {
    let per = |v: u64| v as f64 / calls as f64;
    json!({
        "calls": calls,
        "mul_per_call": per(c.mul),
        "sqr_per_call": per(c.sqr),
        "trace_per_call": per(c.tr),
        "ht_table_lookups_per_call": per(c.ht_bytes_nonzero),
        "ht_masked_basis_terms_per_call": per(c.ht_mask_bits),
        "fong_fold_steps_per_call": per(c.fong_steps),
        "early_exit_fraction": per(c.early_exit),
        "word_ops_per_call": per(c.word_ops),
    })
}

fn bench_one<const W: usize>(
    ln: &LargeN,
    batch: usize,
    reps: usize,
    inner: usize,
    seed: u64,
) -> Value {
    let n = ln.n;
    let f = Field::<W>::new(n, &ln.low);
    let lib = &ln.curve;
    let mut rng = StdRng::seed_from_u64(seed ^ ((n as u64) << 8));
    let mut members = Vec::with_capacity(batch);
    let mut uniform = Vec::with_capacity(batch);
    for _ in 0..batch {
        let p = random_point(&f, &mut rng);
        members.push(dbl_aff(&f, &dbl_aff(&f, &p)));
        uniform.push(random_point(&f, &mut rng));
    }
    let mut pops = Vec::new();
    for (pname, pop) in [("members_of_4E", &members), ("uniform", &uniform)] {
        let libpts: Vec<BinaryPoint> = pop.iter().map(|p| to_lib_pt(&f, p)).collect();
        // Counted operations (deterministic).
        let tabc = CountTally::default();
        let ctmc = CountTally::default();
        let ctfc = CountTally::default();
        let mut ctm_per_call = Vec::new();
        let mut ctf_per_call = Vec::new();
        for p in pop.iter() {
            is_in_4e_tab(&f, p, &tabc);
            let before = ctmc.0.get();
            is_in_4e_ct::<W, false, _>(&f, &p.x, &p.y, 0, &ctmc);
            ctm_per_call.push(ctmc.0.get().word_ops - before.word_ops);
            let before = ctfc.0.get();
            is_in_4e_ct::<W, true, _>(&f, &p.x, &p.y, 0, &ctfc);
            ctf_per_call.push(ctfc.0.get().word_ops - before.word_ops);
        }
        let ct_counts_constant = ctm_per_call.iter().all(|&v| v == ctm_per_call[0])
            && ctf_per_call.iter().all(|&v| v == ctf_per_call[0]);
        // Timed, interleaved per rep (contended host).
        let mut t_lib = Vec::new();
        let mut t_tab = Vec::new();
        let mut t_ctm = Vec::new();
        let mut t_ctf = Vec::new();
        let mut t_ht_tab = Vec::new();
        let mut t_ht_mask = Vec::new();
        let mut t_ht_fong = Vec::new();
        let mut t_mul = Vec::new();
        let calls = (pop.len() * inner) as f64;
        let mut sink = 0u64;
        for _ in 0..reps {
            let s = Instant::now();
            for _ in 0..inner {
                for p in &libpts {
                    sink ^= (lib.is_in_4e(black_box(p)) == Some(true)) as u64;
                }
            }
            t_lib.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    sink ^= is_in_4e_tab(&f, black_box(p), &NoTally) as u64;
                }
            }
            t_tab.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    let p = black_box(p);
                    sink ^= is_in_4e_ct::<W, false, _>(&f, &p.x, &p.y, 0, &NoTally).0;
                }
            }
            t_ctm.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    let p = black_box(p);
                    sink ^= is_in_4e_ct::<W, true, _>(&f, &p.x, &p.y, 0, &NoTally).0;
                }
            }
            t_ctf.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    sink ^= f.ht_tab(black_box(&p.x), &NoTally)[0];
                }
            }
            t_ht_tab.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    sink ^= f.ht_mask(black_box(&p.x), &NoTally)[0];
                }
            }
            t_ht_mask.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    sink ^= f.ht_fong(black_box(&p.x), &NoTally)[0];
                }
            }
            t_ht_fong.push(s.elapsed().as_nanos() as f64 / calls);
            let s = Instant::now();
            for _ in 0..inner {
                for p in pop.iter() {
                    sink ^= f.mul(black_box(&p.x), &p.y, &NoTally)[0];
                }
            }
            t_mul.push(s.elapsed().as_nanos() as f64 / calls);
        }
        black_box(sink);
        let summ = |v: &Vec<f64>| {
            let mut c = v.clone();
            json!({"median_ns": median(&mut c), "q10_ns": quantile(v, 0.1), "q90_ns": quantile(v, 0.9)})
        };
        let mtab = median(&mut t_tab.clone());
        let mctm = median(&mut t_ctm.clone());
        let mctf = median(&mut t_ctf.clone());
        let tc = tabc.0.get();
        let mc = ctmc.0.get();
        let fc = ctfc.0.get();
        let k = pop.len() as u64;
        pops.push(json!({
            "population": pname,
            "points": pop.len(),
            "wall_contended": {
                "lib_is_in_4e": summ(&t_lib),
                "tab": summ(&t_tab),
                "ct_mask": summ(&t_ctm),
                "ct_fong": summ(&t_ctf),
                "ht_tab_only": summ(&t_ht_tab),
                "ht_mask_only": summ(&t_ht_mask),
                "ht_fong_only": summ(&t_ht_fong),
                "mul_only": summ(&t_mul),
                "ratio_ct_mask_over_tab": mctm / mtab,
                "ratio_ct_fong_over_tab": mctf / mtab,
            },
            "counted": {
                "tab": counts_json(&tc, k),
                "ct_mask": counts_json(&mc, k),
                "ct_fong": counts_json(&fc, k),
                "ct_word_ops_identical_on_every_call": ct_counts_constant,
                "ratio_ct_mask_over_tab_word_ops": mc.word_ops as f64 / tc.word_ops as f64,
                "ratio_ct_fong_over_tab_word_ops": fc.word_ops as f64 / tc.word_ops as f64,
            },
        }));
        eprintln!(
            "n={n} {pname}: tab {mtab:.0} ns, ct_mask {mctm:.0} ns ({:.2}x), ct_fong {mctf:.0} ns ({:.2}x); words tab {:.0} ct_mask {:.0} ct_fong {:.0}",
            mctm / mtab,
            mctf / mtab,
            tc.word_ops as f64 / k as f64,
            mc.word_ops as f64 / k as f64,
            fc.word_ops as f64 / k as f64,
        );
    }
    json!({
        "n": n,
        "curve": ln.name,
        "words": W,
        "pclmulqdq": f.hw,
        "reduction_folds": f.folds,
        "word_op_model": {
            "mul": f.wo_mul, "sqr": f.wo_sqr, "trace": f.wo_tr, "reduce": f.wo_reduce,
            "ht_table_per_nonzero_byte": W + 2, "ht_table_per_byte_scan": 3,
            "ht_mask_per_basis_term": 3 + 2 * W, "fong_per_fold_step": 13,
        },
        "populations": pops,
    })
}

// ───────────────────────────── certificates ─────────────────────────────

/// Integer polynomial in the statement's variables (exponent vector -> coeff).
type Poly = BTreeMap<Vec<u32>, i128>;

fn padd(a: &Poly, b: &Poly, sign: i128) -> Poly {
    let mut o = a.clone();
    for (k, v) in b {
        *o.entry(k.clone()).or_insert(0) += sign * v;
    }
    o.retain(|_, v| *v != 0);
    o
}

fn pmul(a: &Poly, b: &Poly) -> Poly {
    let mut o = Poly::new();
    for (ka, va) in a {
        for (kb, vb) in b {
            let k: Vec<u32> = ka.iter().zip(kb).map(|(x, y)| x + y).collect();
            *o.entry(k).or_insert(0) += va * vb;
        }
    }
    o.retain(|_, v| *v != 0);
    o
}

fn to_poly(e: &Expr, vars: &[&str]) -> Poly {
    let nv = vars.len();
    match e {
        Expr::Var(v) => {
            let mut k = vec![0; nv];
            k[vars.iter().position(|x| x == v).expect("declared")] = 1;
            Poly::from([(k, 1)])
        }
        Expr::Const(c) => {
            if *c == 0 {
                Poly::new()
            } else {
                Poly::from([(vec![0; nv], *c as i128)])
            }
        }
        Expr::Neg(a) => padd(&Poly::new(), &to_poly(a, vars), -1),
        Expr::Add(ts) => ts
            .iter()
            .fold(Poly::new(), |acc, t| padd(&acc, &to_poly(t, vars), 1)),
        Expr::Mul(ts) => ts.iter().fold(Poly::from([(vec![0; nv], 1)]), |acc, t| {
            pmul(&acc, &to_poly(t, vars))
        }),
    }
}

fn poly_to_expr(p: &Poly, vars: &[&str]) -> Expr {
    let terms: Vec<Expr> = p
        .iter()
        .map(|(k, c)| {
            let mut fs = vec![Expr::Const(i64::try_from(*c).expect("small coefficient"))];
            for (i, &e) in k.iter().enumerate() {
                for _ in 0..e {
                    fs.push(Expr::var(vars[i]));
                }
            }
            Expr::Mul(fs)
        })
        .collect();
    if terms.is_empty() {
        Expr::Const(0)
    } else {
        Expr::Add(terms)
    }
}

fn v(s: &str) -> Expr {
    Expr::var(s)
}
fn c(k: i64) -> Expr {
    Expr::Const(k)
}
fn add(ts: Vec<Expr>) -> Expr {
    Expr::Add(ts)
}
fn mul(ts: Vec<Expr>) -> Expr {
    Expr::Mul(ts)
}

/// Lift a characteristic-2 identity `lhs == rhs_core (mod 2, mod F)` to an
/// exact integer identity `lhs = rhs_core + C·F + 2·K`, with `F` monic of
/// degree 2 in variable `yvar`.  Returns the full right-hand side and the
/// witness sizes, or an error if `lhs - rhs_core` is not divisible.
fn lift_char2(
    vars: &[&str],
    lhs: &Expr,
    rhs_core: &Expr,
    modulus: Option<(&Expr, usize)>,
) -> Result<(Expr, Value), String> {
    let d = padd(&to_poly(lhs, vars), &to_poly(rhs_core, vars), -1);
    let mut dbar: Poly = d
        .iter()
        .filter(|(_, v)| v.rem_euclid(2) == 1)
        .map(|(k, _)| (k.clone(), 1))
        .collect();
    let mut quot = Poly::new();
    let mut fz = Poly::new();
    if let Some((fe, yv)) = modulus {
        fz = to_poly(fe, vars);
        let fbar: Poly = fz
            .iter()
            .filter(|(_, v)| v.rem_euclid(2) == 1)
            .map(|(k, _)| (k.clone(), 1))
            .collect();
        loop {
            let lead = dbar
                .keys()
                .filter(|k| k[yv] >= 2)
                .max_by_key(|k| (k[yv], (*k).clone()))
                .cloned();
            let Some(k) = lead else { break };
            let mut m = k.clone();
            m[yv] -= 2;
            let mono = Poly::from([(m.clone(), 1)]);
            *quot.entry(m).or_insert(0) ^= 1;
            let sub = pmul(&mono, &fbar);
            for (kk, _) in sub {
                let e = dbar.entry(kk).or_insert(0);
                *e ^= 1;
            }
            dbar.retain(|_, v| *v != 0);
        }
        quot.retain(|_, v| *v != 0);
    }
    if !dbar.is_empty() {
        return Err(format!(
            "not divisible: {} remainder monomials mod 2",
            dbar.len()
        ));
    }
    let cf = pmul(&quot, &fz);
    let rest = padd(&d, &cf, -1);
    if rest.values().any(|v| v % 2 != 0) {
        return Err("odd coefficient after removing C·F".into());
    }
    let k: Poly = rest.iter().map(|(kk, v)| (kk.clone(), v / 2)).collect();
    let mut parts = vec![rhs_core.clone()];
    if !quot.is_empty() {
        parts.push(mul(vec![
            poly_to_expr(&quot, vars),
            modulus.unwrap().0.clone(),
        ]));
    }
    if !k.is_empty() {
        parts.push(mul(vec![c(2), poly_to_expr(&k, vars)]));
    }
    let rhs = add(parts);
    // Exact integer check (not modulo p).
    let exact = padd(&to_poly(lhs, vars), &to_poly(&rhs, vars), -1).is_empty();
    if !exact {
        return Err("exact integer expansion differs".into());
    }
    let maxk = k.values().map(|v| v.abs()).max().unwrap_or(0);
    Ok((
        rhs,
        json!({
            "curve_multiplier_terms": quot.len(),
            "two_k_terms": k.len(),
            "max_abs_k_coefficient": maxk.to_string(),
            "exact_integer_expansion_equal": exact,
        }),
    ))
}

fn issue_and_check(
    name: &str,
    source: &str,
    vars: &[&str],
    lhs: Expr,
    rhs_core: Expr,
    modulus: Option<(&Expr, usize)>,
    seed: u64,
) -> (Value, Value) {
    let (rhs, witness) = lift_char2(vars, &lhs, &rhs_core, modulus).expect("lift");
    let deg = idc::syntactic_degree(&lhs).max(idc::syntactic_degree(&rhs));
    let st = IdentityStatement {
        name: name.to_string(),
        source: source.to_string(),
        variables: vars.iter().map(|s| s.to_string()).collect(),
        degree_bound: deg,
        lhs,
        rhs,
    };
    let cert = idc::issue(
        st,
        seed,
        32,
        "native issuer examples/r6_3_coset_label.rs; integer witness found by division modulo 2 and the curve polynomial; exact integer expansion of both sides compared in the same run; no formal proof pointer",
        None,
    )
    .expect("issue");
    let verdict = idc::check(&cert).expect("well-formed");
    let extra = idc::check_with_extra(&cert, Some((0x5236_3363, 64))).expect("well-formed");
    let summary = json!({
        "id": cert.id,
        "name": name,
        "degree_bound": deg,
        "witness": witness,
        "check": format!("{verdict:?}"),
        "check_with_64_extra_points": format!("{extra:?}"),
    });
    (serde_json::to_value(&cert).unwrap(), summary)
}

fn certificates(outdir: &str) {
    // Curve polynomial over Z, reducing to y^2 + xy + x^3 + 1 mod 2.
    let curve_xy = add(vec![
        mul(vec![v("y"), v("y")]),
        mul(vec![v("x"), v("y")]),
        mul(vec![c(-1), v("x"), v("x"), v("x")]),
        c(-1),
    ]);
    // (1) Prop. 14.2 halving step.  Variables u, l (the half's x and its
    // lambda).  x = l^2 + l, y = u^2 + (l + 1) x, v = u (l + u):
    // F(x, y) = F(u, v) in characteristic 2, identically.
    let x_of = add(vec![mul(vec![v("l"), v("l")]), v("l")]);
    let y_of = add(vec![
        mul(vec![v("u"), v("u")]),
        mul(vec![add(vec![v("l"), c(1)]), x_of.clone()]),
    ]);
    let v_of = mul(vec![v("u"), add(vec![v("l"), v("u")])]);
    let fpos = |a: &Expr, b: &Expr| {
        add(vec![
            mul(vec![b.clone(), b.clone()]),
            mul(vec![a.clone(), b.clone()]),
            mul(vec![a.clone(), a.clone(), a.clone()]),
            c(1),
        ])
    };
    let (cert1, sum1) = issue_and_check(
        "r6.3 halving step of Prop. 14.2: F(l^2+l, u^2+(l+1)(l^2+l)) = F(u, u(l+u)) in char 2, F = Y^2+XY+X^3+1",
        "RESEARCH_ISOGENY_FIBERS_ECDLP.md Prop. 14.2 (doubling Q=(u,v) with lambda=l: x=l^2+l, y=u^2+(l+1)x, v=u(l+u)); lifted to Z as lhs = rhs + 2K",
        &["u", "l"],
        fpos(&x_of, &y_of),
        fpos(&v("u"), &v_of),
        None,
        20261018,
    );
    // (2) Label lemma core.  On F = 0 in char 2, with l = y/(x+1):
    // (x+1)^2 (x l^2 + x l + l) = (x+1)^2 ((x+l)^2 + (x+l) + 1).
    let x1 = add(vec![v("x"), c(1)]);
    let lhs2 = add(vec![
        mul(vec![v("x"), v("y"), v("y")]),
        mul(vec![v("x"), x1.clone(), v("y")]),
        mul(vec![x1.clone(), v("y")]),
    ]);
    let s = add(vec![mul(vec![v("x"), x1.clone()]), v("y")]);
    let rhs2 = add(vec![
        mul(vec![s.clone(), s.clone()]),
        mul(vec![s.clone(), x1.clone()]),
        mul(vec![x1.clone(), x1.clone()]),
    ]);
    let (cert2, sum2) = issue_and_check(
        "r6.3 label lemma core: (x+1)^2 (x l^2 + x l + l) = (x+1)^2 (g^2 + g + 1), l = y/(x+1), g = x + l, modulo 2 and y^2+xy+x^3+1",
        "research/isogeny_fibers_20261010/round6/r6_3_coset_label/RESULTS.md Lemma R6.3 step (iii); lifted to Z as lhs = rhs + C*(y^2+xy-x^3-1) + 2K",
        &["x", "y"],
        lhs2,
        rhs2,
        Some((&curve_xy, 1)),
        20261019,
    );
    for (name, cert) in [
        ("certificate_halving_step.json", &cert1),
        ("certificate_label_core.json", &cert2),
    ] {
        let path = format!("{outdir}/{name}");
        std::fs::write(&path, serde_json::to_string_pretty(cert).unwrap() + "\n").unwrap();
        // Re-read and re-check from disk.
        let back: idc::IdentityCertificate =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let v = idc::check(&back).expect("well-formed");
        eprintln!("{path}: {v:?}");
    }
    let summary = json!({"certificates": [sum1, sum2]});
    let path = format!("{outdir}/certificates_check.json");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

// ───────────────────────────── main ─────────────────────────────

fn arg(args: &[String], key: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == key)
        .map(|i| args[i + 1].clone())
        .unwrap_or_else(|| default.to_string())
}

fn ns_list(s: &str) -> Vec<u32> {
    s.split(',').map(|t| t.trim().parse().unwrap()).collect()
}

fn dispatch_sample(n: u32, quads: usize, uniform: usize, seed: u64) -> Value {
    let ln = large(n);
    match n {
        131 => sample_one::<3>(&ln, quads, uniform, seed),
        233 | 239 => sample_one::<4>(&ln, quads, uniform, seed),
        283 => sample_one::<5>(&ln, quads, uniform, seed),
        409 => sample_one::<7>(&ln, quads, uniform, seed),
        571 => sample_one::<9>(&ln, quads, uniform, seed),
        _ => unreachable!(),
    }
}

fn dispatch_bench(n: u32, batch: usize, reps: usize, inner: usize, seed: u64) -> Value {
    let ln = large(n);
    match n {
        131 => bench_one::<3>(&ln, batch, reps, inner, seed),
        233 | 239 => bench_one::<4>(&ln, batch, reps, inner, seed),
        283 => bench_one::<5>(&ln, batch, reps, inner, seed),
        409 => bench_one::<7>(&ln, batch, reps, inner, seed),
        571 => bench_one::<9>(&ln, batch, reps, inner, seed),
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    let started = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let out = arg(&args, "--out", "out.json");
    let rec = match cmd {
        "exhaustive" => {
            let nmin: u32 = arg(&args, "--nmin", "5").parse().unwrap();
            let nmax: u32 = arg(&args, "--nmax", "25").parse().unwrap();
            let mut rng = StdRng::seed_from_u64(arg(&args, "--seed", "6").parse().unwrap());
            let per: Vec<Value> = (nmin..=nmax)
                .filter(|n| n % 2 == 1)
                .map(|n| exhaustive_one(n, &mut rng))
                .collect();
            json!({"command": args[1..].join(" "), "unix_start": started, "per_n": per})
        }
        "sample" => {
            let ns = ns_list(&arg(&args, "--ns", "131,233,239,283,409,571"));
            let quads: usize = arg(&args, "--quads", "2000").parse().unwrap();
            let uniform: usize = arg(&args, "--uniform", "2000").parse().unwrap();
            let seed: u64 = arg(&args, "--seed", "63").parse().unwrap();
            let per: Vec<Value> = ns
                .iter()
                .map(|&n| dispatch_sample(n, quads, uniform, seed))
                .collect();
            json!({"command": args[1..].join(" "), "unix_start": started, "per_n": per})
        }
        "bench" => {
            let ns = ns_list(&arg(&args, "--ns", "131,233,239,283,409,571"));
            let batch: usize = arg(&args, "--batch", "1024").parse().unwrap();
            let reps: usize = arg(&args, "--reps", "15").parse().unwrap();
            let inner: usize = arg(&args, "--inner", "8").parse().unwrap();
            let seed: u64 = arg(&args, "--seed", "64").parse().unwrap();
            let per: Vec<Value> = ns
                .iter()
                .map(|&n| dispatch_bench(n, batch, reps, inner, seed))
                .collect();
            json!({"command": args[1..].join(" "), "unix_start": started,
                   "note": "wall times are contended (shared 4-core host, single thread, interleaved per rep)",
                   "per_n": per})
        }
        "cert" => {
            certificates(&arg(&args, "--outdir", "."));
            return;
        }
        _ => {
            eprintln!("usage: r6_3_coset_label exhaustive|sample|bench|cert [options]");
            std::process::exit(2);
        }
    };
    std::fs::write(&out, serde_json::to_string_pretty(&rec).unwrap() + "\n").unwrap();
    eprintln!("wrote {out}");
}
