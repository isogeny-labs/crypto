//! Polynomials over a field, as coefficient vectors with the constant term
//! first and no trailing zeros (the zero polynomial is empty), with monic
//! GCD, extended GCD and inverse by [`super::hgcd`] (half-GCD above
//! [`HGCD_THRESHOLD`], Euclid below).
//!
//! Callers with their own coefficient type implement [`PolyField`] for a
//! small context value (it holds the modulus or the extension-field data)
//! and call [`gcd`], [`xgcd`], [`inv_mod`].  [`Fp64`] is `F_p` for a prime
//! `p < 2^63` on `u64` residues.

use super::hgcd::{self, PolyRing};

/// Degree at and above which [`xgcd`] and [`inv_mod`] use half-GCD (below:
/// classical extended Euclid).  Measured with `src/bin/hgcd_bench.rs` (F_p,
/// p = 2^61 − 1, Karatsuba products, leaf 256): ≈1.0× at 2048, 1.2× at
/// 3072, 1.3–2.3× at 8192–16384.
pub const HGCD_THRESHOLD: usize = 3072;
/// Degree from which plain [`gcd`] (no cofactors) uses half-GCD.  Classical
/// gcd still won at every measured degree up to 8192 (half-GCD 0.6–0.8×),
/// so this is set beyond the measured range.
pub const GCD_HGCD_THRESHOLD: usize = 32768;
/// Recursion leaf degree of the half-GCD (sweep: 32..1024, best ≈256).
pub const HGCD_LEAF: usize = 256;
/// Operand length at and above which multiplication uses Karatsuba.
pub const KARATSUBA_THRESHOLD: usize = 24;

/// A coefficient field: a context value (holding e.g. the modulus) with
/// the field operations on `Self::E`.
pub trait PolyField {
    type E: Copy + PartialEq + core::fmt::Debug;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn add(&self, a: Self::E, b: Self::E) -> Self::E;
    fn sub(&self, a: Self::E, b: Self::E) -> Self::E;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    /// `a⁻¹` for `a ≠ 0`.
    fn inv(&self, a: Self::E) -> Self::E;
    fn is_zero(&self, a: Self::E) -> bool {
        a == self.zero()
    }
    fn neg(&self, a: Self::E) -> Self::E {
        self.sub(self.zero(), a)
    }
}

impl<F: PolyField + ?Sized> PolyField for &F {
    type E = F::E;
    #[inline]
    fn zero(&self) -> F::E {
        (**self).zero()
    }
    #[inline]
    fn one(&self) -> F::E {
        (**self).one()
    }
    #[inline]
    fn add(&self, a: F::E, b: F::E) -> F::E {
        (**self).add(a, b)
    }
    #[inline]
    fn sub(&self, a: F::E, b: F::E) -> F::E {
        (**self).sub(a, b)
    }
    #[inline]
    fn mul(&self, a: F::E, b: F::E) -> F::E {
        (**self).mul(a, b)
    }
    #[inline]
    fn inv(&self, a: F::E) -> F::E {
        (**self).inv(a)
    }
    #[inline]
    fn is_zero(&self, a: F::E) -> bool {
        (**self).is_zero(a)
    }
    #[inline]
    fn neg(&self, a: F::E) -> F::E {
        (**self).neg(a)
    }
}

/// `F_p` for a prime `p`, `2 ≤ p < 2^63`, residues in `[0, p)` as `u64`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp64 {
    pub p: u64,
}

impl Fp64 {
    pub fn new(p: u64) -> Self {
        debug_assert!((2..1 << 63).contains(&p));
        Fp64 { p }
    }
    /// Reduce a residue that may be `≥ p`.
    #[inline]
    pub fn reduce(&self, a: u64) -> u64 {
        a % self.p
    }
}

impl PolyField for Fp64 {
    type E = u64;
    #[inline]
    fn zero(&self) -> u64 {
        0
    }
    #[inline]
    fn one(&self) -> u64 {
        1 % self.p
    }
    #[inline]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b; // p < 2^63: no overflow
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn inv(&self, a: u64) -> u64 {
        super::int::inv_mod_u64(a, self.p).expect("inverse of zero in F_p")
    }
}

/// `F[x]` as a [`PolyRing`] for the half-GCD driver: the field, then the
/// xgcd/inverse threshold, the plain-gcd threshold and the recursion leaf
/// degree ([`HGCD_THRESHOLD`], [`GCD_HGCD_THRESHOLD`], [`HGCD_LEAF`]).
#[derive(Clone, Copy, Debug)]
pub struct Polys<F>(pub F, pub usize, pub usize, pub usize);

/// Drop trailing zero coefficients.
pub fn trim<F: PolyField>(f: &F, a: &mut Vec<F::E>) {
    while let Some(&c) = a.last() {
        if f.is_zero(c) {
            a.pop();
        } else {
            break;
        }
    }
}

/// `a + b`.
pub fn add<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = long.to_vec();
    for (o, &s) in out.iter_mut().zip(short) {
        *o = f.add(*o, s);
    }
    trim(f, &mut out);
    out
}

/// `a − b`.
pub fn sub<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or_else(|| f.zero());
        let y = b.get(i).copied().unwrap_or_else(|| f.zero());
        out.push(f.sub(x, y));
    }
    trim(f, &mut out);
    out
}

fn mul_school<F: PolyField>(f: &F, a: &[F::E], b: &[F::E], out: &mut [F::E]) {
    for (i, &x) in a.iter().enumerate() {
        if f.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = f.add(out[i + j], f.mul(x, y));
        }
    }
}

/// `out[..a.len()+b.len()-1] += a·b`, Karatsuba above the threshold.
fn mul_into<F: PolyField>(f: &F, a: &[F::E], b: &[F::E], out: &mut [F::E]) {
    if a.is_empty() || b.is_empty() {
        return;
    }
    if a.len() < KARATSUBA_THRESHOLD || b.len() < KARATSUBA_THRESHOLD {
        mul_school(f, a, b, out);
        return;
    }
    // Unbalanced: split the longer operand into chunks of the shorter length.
    if a.len() > 2 * b.len() || b.len() > 2 * a.len() {
        let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        for (k, chunk) in long.chunks(short.len()).enumerate() {
            mul_into(f, chunk, short, &mut out[k * short.len()..]);
        }
        return;
    }
    let h = a.len().max(b.len()) / 2;
    let (a0, a1) = a.split_at(h.min(a.len()));
    let (b0, b1) = b.split_at(h.min(b.len()));
    // z0 = a0·b0, z2 = a1·b1, z1 = (a0+a1)(b0+b1) − z0 − z2.
    let mut z0 = vec![f.zero(); a0.len() + b0.len()];
    mul_into(f, a0, b0, &mut z0);
    let mut z2 = vec![f.zero(); (a1.len() + b1.len()).max(1)];
    mul_into(f, a1, b1, &mut z2);
    let sa = add_raw(f, a0, a1);
    let sb = add_raw(f, b0, b1);
    let mut z1 = vec![f.zero(); sa.len() + sb.len()];
    mul_into(f, &sa, &sb, &mut z1);
    for (i, &v) in z0.iter().enumerate() {
        z1[i] = f.sub(z1[i], v);
        out[i] = f.add(out[i], v);
    }
    for (i, &v) in z2.iter().enumerate() {
        if i < z1.len() {
            z1[i] = f.sub(z1[i], v);
        }
        if 2 * h + i < out.len() {
            out[2 * h + i] = f.add(out[2 * h + i], v);
        }
    }
    for (i, &v) in z1.iter().enumerate() {
        if h + i < out.len() {
            out[h + i] = f.add(out[h + i], v);
        }
    }
}

fn add_raw<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = long.to_vec();
    for (o, &s) in out.iter_mut().zip(short) {
        *o = f.add(*o, s);
    }
    out
}

/// `a·b` (Karatsuba above [`KARATSUBA_THRESHOLD`]).
pub fn mul<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![f.zero(); a.len() + b.len() - 1];
    mul_into(f, a, b, &mut out);
    trim(f, &mut out);
    out
}

/// `(q, r)` with `a = q·b + r`, `deg r < deg b`; panics if `b = 0`.
pub fn divrem<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> (Vec<F::E>, Vec<F::E>) {
    assert!(!b.is_empty(), "polynomial division by zero");
    if a.len() < b.len() {
        return (Vec::new(), a.to_vec());
    }
    let lc_inv = f.inv(*b.last().unwrap());
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let mut q = vec![f.zero(); a.len() - db];
    for k in (0..q.len()).rev() {
        let c = r[k + db];
        if f.is_zero(c) {
            continue;
        }
        let t = f.mul(c, lc_inv);
        q[k] = t;
        for (j, &bj) in b.iter().enumerate() {
            r[k + j] = f.sub(r[k + j], f.mul(t, bj));
        }
    }
    r.truncate(db);
    trim(f, &mut r);
    trim(f, &mut q);
    (q, r)
}

impl<F: PolyField> PolyRing for Polys<F> {
    type P = Vec<F::E>;
    fn zero(&self) -> Self::P {
        Vec::new()
    }
    fn one(&self) -> Self::P {
        vec![self.0.one()]
    }
    fn deg(&self, a: &Self::P) -> isize {
        a.len() as isize - 1
    }
    fn add(&self, a: &Self::P, b: &Self::P) -> Self::P {
        add(&self.0, a, b)
    }
    fn sub(&self, a: &Self::P, b: &Self::P) -> Self::P {
        sub(&self.0, a, b)
    }
    fn mul(&self, a: &Self::P, b: &Self::P) -> Self::P {
        mul(&self.0, a, b)
    }
    fn divrem(&self, a: &Self::P, b: &Self::P) -> (Self::P, Self::P) {
        divrem(&self.0, a, b)
    }
    fn shr(&self, a: &Self::P, k: usize) -> Self::P {
        if k >= a.len() {
            Vec::new()
        } else {
            a[k..].to_vec()
        }
    }
    fn make_monic(&self, g: &mut Self::P, others: &mut [&mut Self::P]) {
        let inv = self.0.inv(*g.last().expect("make_monic of zero"));
        for c in g.iter_mut() {
            *c = self.0.mul(*c, inv);
        }
        for o in others.iter_mut() {
            for c in o.iter_mut() {
                *c = self.0.mul(*c, inv);
            }
            trim(&self.0, o);
        }
    }
    fn hgcd_threshold(&self) -> usize {
        self.1
    }
    fn gcd_threshold(&self) -> usize {
        self.2
    }
    fn hgcd_leaf(&self) -> usize {
        self.3
    }
}

fn normalized<F: PolyField>(f: &F, a: &[F::E]) -> Vec<F::E> {
    let mut v = a.to_vec();
    trim(f, &mut v);
    v
}

/// Monic `gcd(a, b)` (empty when both are zero).  Inputs may carry
/// trailing zeros.
pub fn gcd<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    hgcd::gcd(
        &Polys(f, HGCD_THRESHOLD, GCD_HGCD_THRESHOLD, HGCD_LEAF),
        &normalized(f, a),
        &normalized(f, b),
    )
}

/// `(g, s, t)`: `g = gcd(a, b)` monic, `g = s·a + t·b`, the classical
/// minimal-degree cofactors ([`super::hgcd::xgcd`]).
pub fn xgcd<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> (Vec<F::E>, Vec<F::E>, Vec<F::E>) {
    hgcd::xgcd(
        &Polys(f, HGCD_THRESHOLD, GCD_HGCD_THRESHOLD, HGCD_LEAF),
        &normalized(f, a),
        &normalized(f, b),
    )
}

/// `a⁻¹ mod m` reduced (`deg < deg m`), `None` unless `gcd(a, m) = 1`;
/// `None` for `deg m < 1`.
pub fn inv_mod<F: PolyField>(f: &F, a: &[F::E], m: &[F::E]) -> Option<Vec<F::E>> {
    hgcd::inv_mod(
        &Polys(f, HGCD_THRESHOLD, GCD_HGCD_THRESHOLD, HGCD_LEAF),
        &normalized(f, a),
        &normalized(f, m),
    )
}

/// [`gcd`] with an explicit half-GCD degree threshold (`0` forces the
/// recursion at every size, `usize::MAX` forces Euclid); for benchmarks.
pub fn gcd_with_threshold<F: PolyField>(f: &F, a: &[F::E], b: &[F::E], thr: usize) -> Vec<F::E> {
    gcd_with_params(f, a, b, thr, HGCD_LEAF)
}

/// [`gcd`] with explicit threshold and recursion leaf degree; for tuning.
pub fn gcd_with_params<F: PolyField>(
    f: &F,
    a: &[F::E],
    b: &[F::E],
    thr: usize,
    leaf: usize,
) -> Vec<F::E> {
    hgcd::gcd(
        &Polys(f, thr, thr, leaf),
        &normalized(f, a),
        &normalized(f, b),
    )
}

/// [`xgcd`] with an explicit half-GCD degree threshold; for benchmarks.
pub fn xgcd_with_threshold<F: PolyField>(
    f: &F,
    a: &[F::E],
    b: &[F::E],
    thr: usize,
) -> (Vec<F::E>, Vec<F::E>, Vec<F::E>) {
    xgcd_with_params(f, a, b, thr, HGCD_LEAF)
}

/// [`xgcd`] with explicit threshold and recursion leaf degree; for tuning.
pub fn xgcd_with_params<F: PolyField>(
    f: &F,
    a: &[F::E],
    b: &[F::E],
    thr: usize,
    leaf: usize,
) -> (Vec<F::E>, Vec<F::E>, Vec<F::E>) {
    hgcd::xgcd(
        &Polys(f, thr, thr, leaf),
        &normalized(f, a),
        &normalized(f, b),
    )
}

/// Classical Euclid only (the reference the half-GCD path is tested and
/// benchmarked against).
pub fn gcd_euclid<F: PolyField>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    let mut x = normalized(f, a);
    let mut y = normalized(f, b);
    while !y.is_empty() {
        let (_, r) = divrem(f, &x, &y);
        x = std::mem::replace(&mut y, r);
    }
    if let Some(&lc) = x.last() {
        let inv = f.inv(lc);
        for c in x.iter_mut() {
            *c = f.mul(*c, inv);
        }
    }
    x
}

/// Classical extended Euclid only: `(g, s, t)` as [`xgcd`] returns.
pub fn xgcd_euclid<F: PolyField>(
    f: &F,
    a: &[F::E],
    b: &[F::E],
) -> (Vec<F::E>, Vec<F::E>, Vec<F::E>) {
    let (mut r0, mut r1) = (normalized(f, a), normalized(f, b));
    let (mut s0, mut s1) = (vec![f.one()], Vec::new());
    let (mut t0, mut t1) = (Vec::new(), vec![f.one()]);
    if r0.is_empty() && r1.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
    }
    while !r1.is_empty() {
        let (q, r) = divrem(f, &r0, &r1);
        let s = sub(f, &s0, &mul(f, &q, &s1));
        let t = sub(f, &t0, &mul(f, &q, &t1));
        r0 = std::mem::replace(&mut r1, r);
        s0 = std::mem::replace(&mut s1, s);
        t0 = std::mem::replace(&mut t1, t);
    }
    let inv = f.inv(*r0.last().unwrap());
    for v in [&mut r0, &mut s0, &mut t0] {
        for c in v.iter_mut() {
            *c = f.mul(*c, inv);
        }
        trim(f, v);
    }
    (r0, s0, t0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn rand_poly(f: &Fp64, rng: &mut StdRng, deg: usize) -> Vec<u64> {
        let mut v: Vec<u64> = (0..=deg).map(|_| rng.gen_range(0..f.p)).collect();
        if let Some(last) = v.last_mut() {
            if *last == 0 {
                *last = 1;
            }
        }
        v
    }

    #[test]
    fn karatsuba_matches_schoolbook() {
        let f = Fp64::new(1_000_000_007);
        let mut rng = StdRng::seed_from_u64(1);
        for (da, db) in [
            (0, 0),
            (5, 7),
            (23, 23),
            (24, 24),
            (50, 13),
            (100, 100),
            (300, 41),
            (257, 255),
        ] {
            let a = rand_poly(&f, &mut rng, da);
            let b = rand_poly(&f, &mut rng, db);
            let mut want = vec![0u64; a.len() + b.len() - 1];
            mul_school(&f, &a, &b, &mut want);
            trim(&f, &mut want);
            assert_eq!(mul(&f, &a, &b), want, "{da}x{db}");
        }
    }

    /// Half-GCD gcd/xgcd equal classical Euclid bit for bit, on random
    /// pairs with planted common factors, across the crossover, for several
    /// primes including p = 2 and p near 2^63.
    #[test]
    fn half_gcd_matches_euclid() {
        let mut rng = StdRng::seed_from_u64(2);
        for p in [
            2u64,
            3,
            7,
            65_521,
            1_000_000_007,
            (1u64 << 61) - 1,
            9_223_372_036_854_775_783,
        ] {
            let f = Fp64::new(p);
            for &(da, db, dc) in &[
                (0usize, 0usize, 0usize),
                (3, 2, 1),
                (40, 39, 0),
                (95, 94, 3),
                (96, 95, 0),
                (150, 149, 10),
                (200, 120, 0),
                (300, 299, 37),
                (513, 511, 64),
            ] {
                let c = rand_poly(&f, &mut rng, dc);
                let a = mul(&f, &rand_poly(&f, &mut rng, da), &c);
                let b = mul(&f, &rand_poly(&f, &mut rng, db), &c);
                let want = xgcd_euclid(&f, &a, &b);
                let got = xgcd(&f, &a, &b);
                assert_eq!(got, want, "p={p} da={da} db={db} dc={dc}");
                // The recursion at every size (threshold 2) agrees too.
                assert_eq!(
                    xgcd_with_threshold(&f, &a, &b, 2),
                    want,
                    "thr=2 p={p} da={da}"
                );
                assert_eq!(gcd_with_threshold(&f, &a, &b, 2), want.0);
                assert_eq!(gcd(&f, &a, &b), want.0);
                assert_eq!(gcd(&f, &b, &a), want.0);
                assert_eq!(gcd_euclid(&f, &a, &b), want.0);
                // Bézout identity.
                let lhs = add(&f, &mul(&f, &got.1, &a), &mul(&f, &got.2, &b));
                assert_eq!(lhs, got.0);
            }
        }
    }

    #[test]
    fn edge_cases_and_inverse() {
        let f = Fp64::new(101);
        assert_eq!(gcd(&f, &[], &[]), Vec::<u64>::new());
        assert_eq!(gcd(&f, &[0, 0], &[5]), vec![1]);
        assert_eq!(gcd(&f, &[3, 0, 2], &[]), vec![3 * 51 % 101, 0, 1]);
        assert_eq!(xgcd(&f, &[], &[4]), (vec![1], vec![], vec![f.inv(4)]));
        let mut rng = StdRng::seed_from_u64(3);
        for d in [1usize, 2, 10, 97, 130, 260] {
            let m = rand_poly(&f, &mut rng, d);
            for _ in 0..5 {
                let da = rng.gen_range(0..2 * d + 1);
                let a = rand_poly(&f, &mut rng, da);
                let g = gcd_euclid(&f, &a, &m);
                match inv_mod(&f, &a, &m) {
                    Some(inv) => {
                        assert_eq!(g, vec![1]);
                        let (_, r) = divrem(&f, &mul(&f, &a, &inv), &m);
                        assert_eq!(r, vec![1]);
                        assert!(inv.len() < m.len());
                    }
                    None => assert_ne!(g, vec![1]),
                }
            }
        }
        assert_eq!(inv_mod(&f, &[1], &[5]), None);
    }
}
