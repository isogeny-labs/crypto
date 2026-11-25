//! Strassen's algorithm for the dense matrix products in the toy
//! multivariate and code-based schemes (`mayo`, `uov`, `qr_uov`,
//! `mceliece`).
//!
//! [`mul`] is the entry point. It runs the schoolbook product when the
//! smallest of the three dimensions is at most [`STRASSEN_CUTOFF`], and
//! Strassen's seven-product recursion above it. Odd dimensions are handled
//! by reading out-of-range entries as zero when the operands are split into
//! quadrants; results are cropped back to the true size.
//!
//! The recursion only uses ring addition, subtraction and multiplication,
//! so it is correct over any commutative ring, including characteristic 2
//! (where `sub` is the same as `add`). Its exponent is log2 7 ≈ 2.807.
//! Asymptotically faster bounds (down to ω ≤ 9/4, OpenAI 2026; Amanov,
//! van den Berg & Zuiddam, arXiv:2610.30054) are proved by existence or
//! through tensor powers and give no algorithm usable at these sizes.
//!
//! Field arithmetic comes in through the [`Ring`] trait, so each scheme's
//! `mul` is monomorphised and inlined exactly as in the loops this replaced.
//!
//! # Measured (release, Apple M-series, 2026-11-24)
//! Time of Strassen with cutoff 32 divided by the schoolbook loop, square
//! n×n (lower is better):
//!
//! | n   | GF(2) bytes | GF(16) | GF(256) | F_31^3 |
//! |-----|-------------|--------|---------|--------|
//! | 36  | 1.78        | 0.93   | 0.89    | 1.00   |
//! | 128 | 1.82        | 0.84   | 0.62    | 0.82   |
//! | 256 | 2.48        | 0.46   | 0.42    | 0.70   |
//! | 512 | 2.52        | 0.29   | 0.34    | 0.62   |
//!
//! Over GF(2) stored as one bit per byte, the schoolbook loop is a
//! vectorised AND/XOR and Strassen never pays, so that field sets
//! [`Ring::CUTOFF`] to `usize::MAX`. With the toy parameters in this crate
//! the largest product dimension is 36 (UOV), so only UOV's 36×36 products
//! reach one level of recursion; the rest run the schoolbook loop.

/// A commutative ring for [`mul`]. Implemented by a zero-sized marker type
/// per field, so calls are statically dispatched and inline.
pub trait Ring {
    type Elt: Copy + PartialEq;
    const ZERO: Self::Elt;
    /// Skip zero entries of the left operand in the schoolbook loop. The
    /// multivariate schemes already did this; McEliece did not (its scrambler
    /// is secret), so it keeps a branch-free inner loop.
    const SKIP_ZERO: bool;
    /// Smallest dimension above which [`mul`] recurses. `usize::MAX` keeps
    /// the field on the schoolbook loop.
    const CUTOFF: usize = STRASSEN_CUTOFF;
    fn add(a: Self::Elt, b: Self::Elt) -> Self::Elt;
    fn sub(a: Self::Elt, b: Self::Elt) -> Self::Elt;
    fn mul(a: Self::Elt, b: Self::Elt) -> Self::Elt;
}

/// Default [`Ring::CUTOFF`]. Cutoffs of 16 and 32 were fastest over GF(16),
/// GF(256) and F_31^3; 32 keeps every product with a dimension of at most
/// 32 on the plain loop.
pub const STRASSEN_CUTOFF: usize = 32;

/// `a · b` for row-major `a` (r×k) and `b` (k×c), recursing above
/// [`Ring::CUTOFF`].
pub fn mul<R: Ring>(a: &[Vec<R::Elt>], b: &[Vec<R::Elt>]) -> Vec<Vec<R::Elt>> {
    mul_with_cutoff::<R>(a, b, R::CUTOFF)
}

/// [`mul`] with an explicit cutoff (tests use small cutoffs to force deep
/// recursion). `cutoff` is clamped to at least 1.
pub fn mul_with_cutoff<R: Ring>(
    a: &[Vec<R::Elt>],
    b: &[Vec<R::Elt>],
    cutoff: usize,
) -> Vec<Vec<R::Elt>> {
    let (r, k) = (a.len(), b.len());
    let c = b.first().map_or(0, Vec::len);
    if r.min(k).min(c) <= cutoff.max(1) {
        return naive::<R>(a, b);
    }
    let am = Mat::from_rows(a, k);
    let bm = Mat::from_rows(b, c);
    let prod = strassen::<R>(&am, &bm, cutoff.max(1));
    (0..r)
        .map(|i| prod.d[i * c..(i + 1) * c].to_vec())
        .collect()
}

/// The schoolbook product on rows: the i-l-j loop MAYO, UOV and QR-UOV
/// used before (McEliece used i-j-k; over GF(2) the result is identical).
/// Public as the reference for tests.
pub fn naive<R: Ring>(a: &[Vec<R::Elt>], b: &[Vec<R::Elt>]) -> Vec<Vec<R::Elt>> {
    let c = b.first().map_or(0, Vec::len);
    a.iter()
        .map(|arow| {
            assert_eq!(arow.len(), b.len(), "inner dimensions differ");
            let mut out = vec![R::ZERO; c];
            for (&x, brow) in arow.iter().zip(b) {
                if R::SKIP_ZERO && x == R::ZERO {
                    continue;
                }
                for (o, &y) in out.iter_mut().zip(brow) {
                    *o = R::add(*o, R::mul(x, y));
                }
            }
            out
        })
        .collect()
}

/// Dense row-major matrix used inside the recursion.
struct Mat<T> {
    r: usize,
    c: usize,
    d: Vec<T>,
}

impl<T: Copy> Mat<T> {
    fn from_rows(rows: &[Vec<T>], c: usize) -> Self {
        let mut d = Vec::with_capacity(rows.len() * c);
        for row in rows {
            assert_eq!(row.len(), c, "ragged matrix or inner dimensions differ");
            d.extend_from_slice(row);
        }
        Mat {
            r: rows.len(),
            c,
            d,
        }
    }

    fn zeros(r: usize, c: usize, zero: T) -> Self {
        Mat {
            r,
            c,
            d: vec![zero; r * c],
        }
    }

    /// The h×w block starting at (r0, c0); entries outside `self` read as zero.
    fn block(&self, r0: usize, c0: usize, h: usize, w: usize, zero: T) -> Self {
        let mut out = Mat::zeros(h, w, zero);
        let n = w.min(self.c.saturating_sub(c0));
        for i in 0..h.min(self.r.saturating_sub(r0)) {
            let src = (r0 + i) * self.c + c0;
            out.d[i * w..i * w + n].copy_from_slice(&self.d[src..src + n]);
        }
        out
    }

    /// Copy the part of `blk` that fits into `self` at (r0, c0).
    fn put(&mut self, r0: usize, c0: usize, blk: &Self) {
        let n = blk.c.min(self.c.saturating_sub(c0));
        for i in 0..blk.r.min(self.r.saturating_sub(r0)) {
            let dst = (r0 + i) * self.c + c0;
            self.d[dst..dst + n].copy_from_slice(&blk.d[i * blk.c..i * blk.c + n]);
        }
    }

    fn zip(&self, other: &Self, f: impl Fn(T, T) -> T) -> Self {
        debug_assert!(self.r == other.r && self.c == other.c);
        Mat {
            r: self.r,
            c: self.c,
            d: self
                .d
                .iter()
                .zip(&other.d)
                .map(|(&x, &y)| f(x, y))
                .collect(),
        }
    }
}

fn schoolbook<R: Ring>(a: &Mat<R::Elt>, b: &Mat<R::Elt>) -> Mat<R::Elt> {
    let (r, k, c) = (a.r, a.c, b.c);
    let mut out = Mat::zeros(r, c, R::ZERO);
    for i in 0..r {
        for l in 0..k {
            let x = a.d[i * k + l];
            if R::SKIP_ZERO && x == R::ZERO {
                continue;
            }
            let brow = &b.d[l * c..(l + 1) * c];
            let orow = &mut out.d[i * c..(i + 1) * c];
            for (o, &y) in orow.iter_mut().zip(brow) {
                *o = R::add(*o, R::mul(x, y));
            }
        }
    }
    out
}

fn strassen<R: Ring>(a: &Mat<R::Elt>, b: &Mat<R::Elt>, cutoff: usize) -> Mat<R::Elt> {
    let (r, k, c) = (a.r, a.c, b.c);
    if r.min(k).min(c) <= cutoff {
        return schoolbook::<R>(a, b);
    }
    let z = R::ZERO;
    let (hr, hk, hc) = (r.div_ceil(2), k.div_ceil(2), c.div_ceil(2));
    let a11 = a.block(0, 0, hr, hk, z);
    let a12 = a.block(0, hk, hr, hk, z);
    let a21 = a.block(hr, 0, hr, hk, z);
    let a22 = a.block(hr, hk, hr, hk, z);
    let b11 = b.block(0, 0, hk, hc, z);
    let b12 = b.block(0, hc, hk, hc, z);
    let b21 = b.block(hk, 0, hk, hc, z);
    let b22 = b.block(hk, hc, hk, hc, z);
    let rec = |x: &Mat<R::Elt>, y: &Mat<R::Elt>| strassen::<R>(x, y, cutoff);

    let m1 = rec(&a11.zip(&a22, R::add), &b11.zip(&b22, R::add));
    let m2 = rec(&a21.zip(&a22, R::add), &b11);
    let m3 = rec(&a11, &b12.zip(&b22, R::sub));
    let m4 = rec(&a22, &b21.zip(&b11, R::sub));
    let m5 = rec(&a11.zip(&a12, R::add), &b22);
    let m6 = rec(&a21.zip(&a11, R::sub), &b11.zip(&b12, R::add));
    let m7 = rec(&a12.zip(&a22, R::sub), &b21.zip(&b22, R::add));

    let c11 = m1.zip(&m4, R::add).zip(&m5, R::sub).zip(&m7, R::add);
    let c12 = m3.zip(&m5, R::add);
    let c21 = m2.zip(&m4, R::add);
    let c22 = m1.zip(&m2, R::sub).zip(&m3, R::add).zip(&m6, R::add);

    let mut out = Mat::zeros(r, c, z);
    out.put(0, 0, &c11);
    out.put(0, hc, &c12);
    out.put(hr, 0, &c21);
    out.put(hr, hc, &c22);
    out
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Small deterministic generator, so tests need no RNG plumbing.
    pub(crate) struct XorShift(pub u64);
    impl XorShift {
        pub(crate) fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
    }

    pub(crate) fn random_matrix<T>(
        rng: &mut XorShift,
        r: usize,
        c: usize,
        mut elt: impl FnMut(u64) -> T,
    ) -> Vec<Vec<T>> {
        (0..r)
            .map(|_| (0..c).map(|_| elt(rng.next())).collect())
            .collect()
    }

    /// Check Strassen against the schoolbook product on random shapes,
    /// including odd and rectangular ones, with cutoffs that force deep
    /// recursion. Each scheme's test module calls this with its own field.
    pub(crate) fn assert_strassen_matches_naive<R: Ring>(elt: impl Fn(u64) -> R::Elt)
    where
        R::Elt: std::fmt::Debug,
    {
        let mut rng = XorShift(0x9E37_79B9_7F4A_7C15);
        let shapes = [
            (1, 1, 1),
            (2, 2, 2),
            (3, 5, 7),
            (7, 3, 5),
            (8, 8, 8),
            (9, 17, 4),
            (16, 16, 16),
            (20, 12, 20),
            (33, 31, 35),
            (36, 36, 36),
            (65, 40, 70),
        ];
        for &(r, k, c) in &shapes {
            let a = random_matrix(&mut rng, r, k, &elt);
            let b = random_matrix(&mut rng, k, c, &elt);
            let want = naive::<R>(&a, &b);
            for cutoff in [1, 2, 3, 4, 8, 16, STRASSEN_CUTOFF] {
                assert_eq!(
                    mul_with_cutoff::<R>(&a, &b, cutoff),
                    want,
                    "shape {r}x{k}x{c}, cutoff {cutoff}"
                );
            }
        }
    }

    /// Print schoolbook vs Strassen timings for square sizes, for picking
    /// [`STRASSEN_CUTOFF`]. Run per field with
    /// `cargo test --release --lib strassen_crossover -- --ignored --nocapture`.
    pub(crate) fn crossover_report<R: Ring>(name: &str, elt: impl Fn(u64) -> R::Elt) {
        use std::hint::black_box;
        use std::time::Instant;
        let mut rng = XorShift(0xC0FF_EE00_1234_5678);
        println!("{name}: n, schoolbook ms, then Strassen ms for cutoff 16 / 32 / 64 / 128");
        for n in [32usize, 48, 64, 96, 128, 192, 256, 384] {
            let a = random_matrix(&mut rng, n, n, &elt);
            let b = random_matrix(&mut rng, n, n, &elt);
            let reps = (200_000_000 / (n * n * n)).clamp(1, 200);
            let time = |f: &dyn Fn() -> Vec<Vec<R::Elt>>| {
                let t = Instant::now();
                for _ in 0..reps {
                    black_box(f());
                }
                t.elapsed().as_secs_f64() * 1e3 / reps as f64
            };
            let base = time(&|| naive::<R>(black_box(&a), black_box(&b)));
            let cuts: Vec<String> = [16, 32, 64, 128]
                .iter()
                .map(|&cut| {
                    let ms = time(&|| mul_with_cutoff::<R>(black_box(&a), black_box(&b), cut));
                    format!("{ms:8.3}")
                })
                .collect();
            println!("  {n:4}  {base:8.3}  {}", cuts.join(" "));
        }
    }

    struct Gf2;
    impl Ring for Gf2 {
        type Elt = u8;
        const ZERO: u8 = 0;
        const SKIP_ZERO: bool = false;
        fn add(a: u8, b: u8) -> u8 {
            a ^ b
        }
        fn sub(a: u8, b: u8) -> u8 {
            a ^ b
        }
        fn mul(a: u8, b: u8) -> u8 {
            a & b
        }
    }

    struct F97;
    impl Ring for F97 {
        type Elt = u32;
        const ZERO: u32 = 0;
        const SKIP_ZERO: bool = true;
        fn add(a: u32, b: u32) -> u32 {
            (a + b) % 97
        }
        fn sub(a: u32, b: u32) -> u32 {
            (a + 97 - b) % 97
        }
        fn mul(a: u32, b: u32) -> u32 {
            (a * b) % 97
        }
    }

    #[test]
    fn strassen_matches_naive_gf2() {
        assert_strassen_matches_naive::<Gf2>(|x| (x & 1) as u8);
    }

    #[test]
    fn strassen_matches_naive_odd_characteristic() {
        // Odd characteristic exercises `sub` separately from `add`.
        assert_strassen_matches_naive::<F97>(|x| (x % 97) as u32);
    }

    #[test]
    fn identity_is_neutral() {
        let id: Vec<Vec<u32>> = (0..70)
            .map(|i| (0..70).map(|j| u32::from(i == j)).collect())
            .collect();
        let mut rng = XorShift(7);
        let a = random_matrix(&mut rng, 70, 70, |x| (x % 97) as u32);
        assert_eq!(mul_with_cutoff::<F97>(&a, &id, 2), a);
        assert_eq!(mul_with_cutoff::<F97>(&id, &a, 2), a);
    }
}
