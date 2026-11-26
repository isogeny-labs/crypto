//! The half-GCD algorithm, once, over any Euclidean polynomial ring
//! ([`PolyRing`]): `F[x]` for a field `F` ([`super::poly`]) and the
//! bit-packed `GF(2)[x]` ([`super::gf2`]).
//!
//! `half_gcd(a, b)` (with `deg a > deg b`) returns the 2×2 matrix `M`, a
//! product of Euclidean quotient matrices `[[0, 1], [1, −q]]`, with
//! `(c, d) = M·(a, b)` the consecutive remainders of the Euclidean sequence
//! of `(a, b)` that straddle degree `m = ⌈deg a / 2⌉`:
//! `deg c ≥ m > deg d`.  It recurses on the top halves of `a` and `b`
//! (whose quotient sequence agrees with the full one down to that degree),
//! so the whole GCD costs `O(M(n) log n)` instead of Euclid's `O(n²)`.
//! Formulation: von zur Gathen and Gerhard, *Modern Computer Algebra*,
//! §11.1; Thull and Yap, "A unified approach to HGCD algorithms" (1990).
//!
//! Remainders are never normalised along the way, so the quotient sequence,
//! and with it every matrix entry, is exactly classical Euclid's: the gcd
//! and the Bézout cofactors this computes are the classical ones, scaled
//! once at the end to make the gcd monic.  Those cofactors are the unique
//! pair with `deg s < deg b − deg g` and `deg t < deg a − deg g`.

/// A Euclidean polynomial ring with the operations half-GCD needs.  `P` is
/// the polynomial value; the ring value carries the coefficient field
/// (e.g. the modulus `p`).
pub trait PolyRing {
    type P: Clone + PartialEq + core::fmt::Debug;

    fn zero(&self) -> Self::P;
    fn one(&self) -> Self::P;
    /// Degree, `-1` for the zero polynomial.
    fn deg(&self, a: &Self::P) -> isize;
    fn add(&self, a: &Self::P, b: &Self::P) -> Self::P;
    fn sub(&self, a: &Self::P, b: &Self::P) -> Self::P;
    fn mul(&self, a: &Self::P, b: &Self::P) -> Self::P;
    /// `(q, r)` with `a = q·b + r`, `deg r < deg b`; `b ≠ 0`.
    fn divrem(&self, a: &Self::P, b: &Self::P) -> (Self::P, Self::P);
    /// `⌊a / x^k⌋`: drop the `k` lowest coefficients.
    fn shr(&self, a: &Self::P, k: usize) -> Self::P;
    /// Divide `g` and every entry of `others` by the leading coefficient of
    /// `g` (`g ≠ 0`), making `g` monic.
    fn make_monic(&self, g: &mut Self::P, others: &mut [&mut Self::P]);
    /// Degree from which [`xgcd`] and [`inv_mod`] use half-GCD (below:
    /// classical extended Euclid).
    fn hgcd_threshold(&self) -> usize;
    /// Degree from which [`gcd`] (no cofactors) uses half-GCD; classical
    /// gcd tracks no cofactors, so its crossover is higher.
    fn gcd_threshold(&self) -> usize {
        self.hgcd_threshold()
    }
    /// Degree below which the [`half_gcd`] recursion stops and builds the
    /// matrix by Euclidean steps.
    fn hgcd_leaf(&self) -> usize {
        128
    }

    fn is_zero(&self, a: &Self::P) -> bool {
        self.deg(a) < 0
    }
}

/// A 2×2 polynomial matrix `[[m00, m01], [m10, m11]]`.
#[derive(Clone, Debug)]
pub struct Mat<P> {
    pub m: [P; 4],
}

impl<P: Clone> Mat<P> {
    pub fn identity<R: PolyRing<P = P>>(r: &R) -> Self {
        Mat {
            m: [r.one(), r.zero(), r.zero(), r.one()],
        }
    }
}

/// `x·y` for 2×2 matrices.
pub fn mat_mul<R: PolyRing>(r: &R, x: &Mat<R::P>, y: &Mat<R::P>) -> Mat<R::P> {
    let [a, b, c, d] = &x.m;
    let [e, f, g, h] = &y.m;
    Mat {
        m: [
            r.add(&r.mul(a, e), &r.mul(b, g)),
            r.add(&r.mul(a, f), &r.mul(b, h)),
            r.add(&r.mul(c, e), &r.mul(d, g)),
            r.add(&r.mul(c, f), &r.mul(d, h)),
        ],
    }
}

/// `M·(a, b)ᵀ`.
pub fn mat_apply<R: PolyRing>(r: &R, m: &Mat<R::P>, a: &R::P, b: &R::P) -> (R::P, R::P) {
    let [m00, m01, m10, m11] = &m.m;
    (
        r.add(&r.mul(m00, a), &r.mul(m01, b)),
        r.add(&r.mul(m10, a), &r.mul(m11, b)),
    )
}

/// One Euclidean step: `(a, b) ← (b, a mod b)` and `M ← [[0,1],[1,−q]]·M`.
fn euclid_step<R: PolyRing>(r: &R, a: &mut R::P, b: &mut R::P, m: &mut Mat<R::P>) {
    let (q, rem) = r.divrem(a, b);
    let [m00, m01, m10, m11] =
        std::mem::replace(&mut m.m, [r.zero(), r.zero(), r.zero(), r.zero()]);
    let n10 = r.sub(&m00, &r.mul(&q, &m10));
    let n11 = r.sub(&m01, &r.mul(&q, &m11));
    m.m = [m10, m11, n10, n11];
    *a = std::mem::replace(b, rem);
}

/// Classical half-GCD by Euclidean steps until `deg b < m`.
fn half_gcd_euclid<R: PolyRing>(r: &R, a: &R::P, b: &R::P, m: isize) -> Mat<R::P> {
    let mut mat = Mat::identity(r);
    let (mut a, mut b) = (a.clone(), b.clone());
    while r.deg(&b) >= m {
        euclid_step(r, &mut a, &mut b, &mut mat);
    }
    mat
}

/// The half-GCD matrix of `(a, b)`, `deg a > deg b`: `(c, d) = M·(a, b)`
/// are the consecutive Euclidean remainders with `deg c ≥ ⌈deg a/2⌉ > deg d`.
pub fn half_gcd<R: PolyRing>(r: &R, a: &R::P, b: &R::P) -> Mat<R::P> {
    let n = r.deg(a);
    debug_assert!(n > r.deg(b));
    let m = (n + 1) / 2;
    if r.deg(b) < m {
        return Mat::identity(r);
    }
    if (n as usize) < r.hgcd_leaf() {
        return half_gcd_euclid(r, a, b, m);
    }
    let mu = m as usize;
    let r1 = half_gcd(r, &r.shr(a, mu), &r.shr(b, mu));
    let (mut c, mut d) = mat_apply(r, &r1, a, b);
    if r.deg(&d) < m {
        return r1;
    }
    let mut mat = r1;
    euclid_step(r, &mut c, &mut d, &mut mat);
    if r.deg(&d) < m {
        return mat;
    }
    let k = (2 * m - r.deg(&c)) as usize;
    let r2 = half_gcd(r, &r.shr(&c, k), &r.shr(&d, k));
    mat_mul(r, &r2, &mat)
}

/// `(g, M)` with `g = gcd(a, b)` (not yet monic) and `M` the full Euclidean
/// matrix: `(g, 0) = M·(a, b)`.  Requires `deg a > deg b` or `b = 0`.
fn gcd_matrix<R: PolyRing>(r: &R, a: &R::P, b: &R::P, want_matrix: bool) -> (R::P, Mat<R::P>) {
    let mut total = Mat::identity(r);
    let (mut a, mut b) = (a.clone(), b.clone());
    let thr = if want_matrix {
        r.hgcd_threshold()
    } else {
        r.gcd_threshold()
    };
    // Clamp: a threshold of usize::MAX ("never") must not wrap to −1.
    let thr = thr.min(isize::MAX as usize) as isize;
    while !r.is_zero(&b) {
        if r.deg(&a) >= thr {
            let m = half_gcd(r, &a, &b);
            let (c, d) = mat_apply(r, &m, &a, &b);
            if want_matrix {
                total = mat_mul(r, &m, &total);
            }
            a = c;
            b = d;
            if r.is_zero(&b) {
                break;
            }
        }
        if want_matrix {
            euclid_step(r, &mut a, &mut b, &mut total);
        } else {
            let (_, rem) = r.divrem(&a, &b);
            a = std::mem::replace(&mut b, rem);
        }
    }
    (a, total)
}

/// Order `(a, b)` so that `deg a > deg b` or `b = 0`, returning the
/// permutation (`true` when swapped) and a first division if the degrees
/// are equal.
fn prepare<R: PolyRing>(r: &R, a: &R::P, b: &R::P) -> (R::P, R::P, Mat<R::P>) {
    // Start matrix S with (a', b') = S·(a, b).
    let (da, db) = (r.deg(a), r.deg(b));
    if da > db {
        return (a.clone(), b.clone(), Mat::identity(r));
    }
    if r.is_zero(a) {
        // gcd(0, b) = b: (b, 0) = [[0,1],[1,0]]·(0, b).
        return (
            b.clone(),
            r.zero(),
            Mat {
                m: [r.zero(), r.one(), r.one(), r.zero()],
            },
        );
    }
    // Equal degrees: classical Euclid's first step divides a by b.
    if da == db {
        let mut s = Mat::identity(r);
        let (mut x, mut y) = (a.clone(), b.clone());
        euclid_step(r, &mut x, &mut y, &mut s);
        return (x, y, s);
    }
    // deg a < deg b: classical Euclid's first quotient is 0, a swap; then
    // one step reduces b mod a.
    let mut s = Mat {
        m: [r.zero(), r.one(), r.one(), r.zero()],
    };
    let (mut x, mut y) = (b.clone(), a.clone());
    euclid_step(r, &mut x, &mut y, &mut s);
    (x, y, s)
}

/// Monic `gcd(a, b)`; `0` when both are zero.
pub fn gcd<R: PolyRing>(r: &R, a: &R::P, b: &R::P) -> R::P {
    if r.is_zero(a) && r.is_zero(b) {
        return r.zero();
    }
    let (x, y, _) = prepare(r, a, b);
    let (mut g, _) = gcd_matrix(r, &x, &y, false);
    r.make_monic(&mut g, &mut []);
    g
}

/// `(g, s, t)` with `g = gcd(a, b)` monic and `g = s·a + t·b`, `s` and `t`
/// the classical extended-Euclid cofactors (unique with `deg s < deg b −
/// deg g`, `deg t < deg a − deg g` when both inputs are nonzero).  Both
/// zero gives `(0, 0, 0)`.
pub fn xgcd<R: PolyRing>(r: &R, a: &R::P, b: &R::P) -> (R::P, R::P, R::P) {
    if r.is_zero(a) && r.is_zero(b) {
        return (r.zero(), r.zero(), r.zero());
    }
    let (x, y, s0) = prepare(r, a, b);
    let (mut g, m) = gcd_matrix(r, &x, &y, true);
    let total = mat_mul(r, &m, &s0);
    let [mut s, mut t, _, _] = total.m;
    r.make_monic(&mut g, &mut [&mut s, &mut t]);
    (g, s, t)
}

/// `a⁻¹ mod m` (reduced, `deg < deg m`) when `gcd(a, m) = 1`, else `None`;
/// `deg m ≥ 1`.
pub fn inv_mod<R: PolyRing>(r: &R, a: &R::P, m: &R::P) -> Option<R::P> {
    if r.deg(m) < 1 {
        return None;
    }
    let (_, a_red) = r.divrem(a, m);
    if r.is_zero(&a_red) {
        return None;
    }
    let (g, s, _) = xgcd(r, &a_red, m);
    if r.deg(&g) != 0 {
        return None;
    }
    let (_, s) = r.divrem(&s, m);
    Some(s)
}
