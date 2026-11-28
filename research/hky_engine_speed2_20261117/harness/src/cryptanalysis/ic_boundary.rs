//! Shim of `ic_boundary` for the pilot: `PrimePoint`, `PrimeCurve`,
//! `GroupOps` and the part of `CountedGroup` the pilot calls. The function
//! bodies below the marker lines are copied verbatim from
//! `src/cryptanalysis/ic_boundary.rs` (line ranges named at each block) by
//! the commands in `../../README.md`; only the field-operation tally of the
//! group law is dropped (`CountedFp` here does the same arithmetic without
//! counting), and `mulmod` is the plain 128-bit form (same value).
#![allow(dead_code)]
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
pub struct GroupOps {
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
}

pub(crate) fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
pub fn invmod(a: u64, m: u64) -> Option<u64> {
    invmod_u64(a, m)
}
type FieldOps = ();
struct CountedFp<'a> {
    p: u64,
    tally: &'a mut FieldOps,
}
impl CountedFp<'_> {
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        mulmod(a, b, self.p)
    }
    fn sqr(&mut self, a: u64) -> u64 {
        mulmod(a, a, self.p)
    }
    fn inv(&mut self, a: u64) -> u64 {
        invmod(a, self.p).expect("inverse of zero")
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        addmod(a, b, self.p)
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        submod(a, b, self.p)
    }
}

pub trait CountedGroup {
    type Elt: Copy + PartialEq + std::fmt::Debug;
    fn identity(&self) -> Self::Elt;
    fn is_identity(&self, p: &Self::Elt) -> bool;
    fn add(&self, ops: &mut GroupOps, p: Self::Elt, q: Self::Elt) -> Self::Elt;
    fn double(&self, ops: &mut GroupOps, p: Self::Elt) -> Self::Elt;
    fn neg(&self, p: Self::Elt) -> Self::Elt;
    fn key(&self, p: &Self::Elt) -> u64;
    // verbatim: ic_boundary.rs 1528-1543
    /// `[k]P` by double-and-add, counted exactly.
    fn mul(&self, ops: &mut GroupOps, p: Self::Elt, k: u64) -> Self::Elt {
        ops.scalar_mults += 1;
        if k == 0 || self.is_identity(&p) {
            return self.identity();
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.double(ops, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }
}

impl CountedGroup for PrimeCurve {
    type Elt = PrimePoint;
    fn identity(&self) -> PrimePoint {
        PrimePoint::INFINITY
    }
    fn is_identity(&self, p: &PrimePoint) -> bool {
        p.infinity
    }
    fn add(&self, ops: &mut GroupOps, p: PrimePoint, q: PrimePoint) -> PrimePoint {
        ops.adds += 1;
        self.add_raw(&mut (), p, q)
    }
    fn double(&self, ops: &mut GroupOps, p: PrimePoint) -> PrimePoint {
        ops.doubles += 1;
        self.double_raw(&mut (), p)
    }
    // verbatim: ic_boundary.rs 6263-6269, 6278-6284
    fn neg(&self, p: PrimePoint) -> PrimePoint {
        if p.infinity {
            p
        } else {
            PrimePoint::affine(p.x, if p.y == 0 { 0 } else { self.p - p.y })
        }
    }
    fn key(&self, p: &PrimePoint) -> u64 {
        if p.infinity {
            0
        } else {
            ((p.x + 1) << 1) | u64::from(p.y > self.p - p.y)
        }
    }
}
// verbatim: ic_boundary.rs 1006-1028 (addmod, submod)
/// `2^63`.
#[inline]
pub(crate) fn addmod(a: u64, b: u64, m: u64) -> u64 {
    if a < m && b < m {
        let (s, carry) = a.overflowing_add(b);
        if carry || s >= m {
            s.wrapping_sub(m)
        } else {
            s
        }
    } else {
        ((a as u128 + b as u128) % m as u128) as u64
    }
}

#[inline]
fn submod(a: u64, b: u64, m: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        m - (b - a)
    }
}
// verbatim: ic_boundary.rs 1043-1058 (powmod_counted)
/// [`powmod`] adding its multiplications to `muls`.
fn powmod_counted(mut a: u64, mut e: u64, m: u64, muls: &mut u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            *muls += 1;
            r = mulmod(r, a, m);
        }
        *muls += 1;
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

// verbatim: ic_boundary.rs 1084-1109 (invmod_u64)
/// [`invmod`] on 64-bit words, for any modulus.
fn invmod_u64(a: u64, m: u64) -> Option<u64> {
    let (mut r0, mut r1) = (m, a % m);
    // `r0 ≡ ±t0·a`, `r1 ≡ ∓t1·a (mod m)`; `neg0` is the sign of `t0`'s
    // coefficient.  Initially `m ≡ 0·a` and `a ≡ +1·a`.
    let (mut t0, mut t1) = (0u64, 1u64);
    let mut neg0 = true;
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 + q * t1);
        neg0 = !neg0;
    }
    if r0 != 1 {
        return None;
    }
    Some(if neg0 {
        // `t0 == m` only when `m == 1`, where the inverse is `0`.
        (m - t0) % m
    } else {
        t0 % m
    })
}

/// [`invmod`] for a modulus and a reduced operand that fit 32 bits: the
/// same extended Euclid, step for step.
// verbatim: ic_boundary.rs 5840-5965 (PrimePoint, PrimeCurve: is_on_curve, rhs, legendre, sqrt)

/// A point on `y² = x³ + ax + b` over `F_p`, `p < 2^63`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimePoint {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl PrimePoint {
    pub const INFINITY: Self = Self {
        x: 0,
        y: 0,
        infinity: true,
    };
    pub fn affine(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
            infinity: false,
        }
    }
}

/// `y² = x³ + ax + b` over `F_p` in single-word arithmetic.
#[derive(Clone, Debug, Serialize)]
pub struct PrimeCurve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

impl PrimeCurve {
    pub fn is_on_curve(&self, pt: PrimePoint) -> bool {
        if pt.infinity {
            return true;
        }
        let p = self.p;
        let lhs = mulmod(pt.y, pt.y, p);
        let rhs = addmod(
            addmod(
                mulmod(mulmod(pt.x, pt.x, p), pt.x, p),
                mulmod(self.a, pt.x, p),
                p,
            ),
            self.b,
            p,
        );
        lhs == rhs
    }

    /// `x³ + ax + b`.
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addmod(
            addmod(mulmod(mulmod(x, x, p), x, p), mulmod(self.a, x, p), p),
            self.b,
            p,
        )
    }

    /// Legendre symbol of `a` modulo the prime `p`, by the Jacobi
    /// algorithm.
    pub fn legendre(&self, a: u64) -> i32 {
        jacobi(a % self.p, self.p)
    }

    /// A square root modulo `p` (Tonelli–Shanks), or `None` for a
    /// non-residue.
    pub fn sqrt(&self, n: u64) -> Option<u64> {
        self.sqrt_counted(n, &mut 0)
    }

    /// [`PrimeCurve::sqrt`], adding to `muls` the modular multiplications it
    /// performs (squarings included: every `mulmod` call, the ones inside
    /// its exponentiations too). The result is `sqrt`'s, step for step.
    pub fn sqrt_counted(&self, n: u64, muls: &mut u64) -> Option<u64> {
        let p = self.p;
        let n = n % p;
        if n == 0 {
            return Some(0);
        }
        if p % 4 == 3 {
            let r = powmod_counted(n, (p + 1) / 4, p, muls);
            *muls += 1;
            return (mulmod(r, r, p) == n).then_some(r);
        }
        if powmod_counted(n, (p - 1) / 2, p, muls) != 1 {
            return None;
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while powmod_counted(z, (p - 1) / 2, p, muls) != p - 1 {
            z += 1;
        }
        let mut m = s;
        let mut c = powmod_counted(z, q, p, muls);
        let mut t = powmod_counted(n, q, p, muls);
        let mut r = powmod_counted(n, q.div_ceil(2), p, muls);
        loop {
            if t == 1 {
                return Some(r);
            }
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                *muls += 1;
                tt = mulmod(tt, tt, p);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let b = powmod_counted(c, 1u64 << (m - i - 1), p, muls);
            m = i;
            *muls += 3;
            c = mulmod(b, b, p);
            t = mulmod(t, c, p);
            r = mulmod(r, b, p);
        }
    }
    // verbatim: ic_boundary.rs 5998-6045 (add_raw, double_raw)
    fn add_raw(&self, tally: &mut FieldOps, a: PrimePoint, b: PrimePoint) -> PrimePoint {
        if a.infinity {
            return b;
        }
        if b.infinity {
            return a;
        }
        if a.x == b.x {
            if addmod(a.y, b.y, self.p) == 0 {
                return PrimePoint::INFINITY;
            }
            return self.double_raw(tally, a);
        }
        let mut f = CountedFp { p: self.p, tally };
        let dx = f.sub(b.x, a.x);
        let inv = f.inv(dx);
        let dy = f.sub(b.y, a.y);
        let lambda = f.mul(dy, inv);
        let lambda2 = f.sqr(lambda);
        let x3 = f.sub(f.sub(lambda2, a.x), b.x);
        let dx3 = f.sub(a.x, x3);
        let ldx3 = f.mul(lambda, dx3);
        let y3 = f.sub(ldx3, a.y);
        PrimePoint::affine(x3, y3)
    }

    /// Affine doubling, the field work tallied into `tally`
    /// ([`PrimeCurve::FIELD_OPS_PER_DOUBLE`]).  `∞` and a point of order
    /// two perform no field arithmetic.
    fn double_raw(&self, tally: &mut FieldOps, a: PrimePoint) -> PrimePoint {
        if a.infinity || a.y == 0 {
            return PrimePoint::INFINITY;
        }
        let mut f = CountedFp { p: self.p, tally };
        let x2 = f.sqr(a.x);
        let three_x2 = f.mul(3, x2);
        let num = f.add(three_x2, self.a);
        let two_y = f.mul(2, a.y);
        let inv = f.inv(two_y);
        let lambda = f.mul(num, inv);
        let lambda2 = f.sqr(lambda);
        let two_x = f.mul(2, a.x);
        let x3 = f.sub(lambda2, two_x);
        let dx3 = f.sub(a.x, x3);
        let ldx3 = f.mul(lambda, dx3);
        let y3 = f.sub(ldx3, a.y);
        PrimePoint::affine(x3, y3)
    }
    // verbatim: ic_boundary.rs 6083-6114 (point_count, jacobi)
    /// `#E(F_p)` by the Legendre symbol at every abscissa: `O(p)`.
    pub fn point_count(&self) -> u64 {
        let mut count = 1u64;
        for x in 0..self.p {
            count += (1 + self.legendre(self.rhs(x))) as u64;
        }
        count
    }
}

fn jacobi(mut a: u64, mut n: u64) -> i32 {
    let mut result = 1i32;
    a %= n;
    while a != 0 {
        while a.is_multiple_of(2) {
            a /= 2;
            if n % 8 == 3 || n % 8 == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            result = -result;
        }
        a %= n;
    }
    if n == 1 {
        result
    } else {
        0
    }
}
