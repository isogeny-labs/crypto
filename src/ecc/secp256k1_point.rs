//! Constant-time projective-coordinate point arithmetic over secp256k1.
//!
//! # Why a separate point type?
//!
//! [`crate::ecc::point::Point`] is the textbook affine-coordinate
//! implementation that backs both secp256k1 and P-256.  Every addition
//! and doubling there computes a modular inverse to normalize the
//! output, which (a) is expensive and (b) — more importantly here —
//! routes through a variable-time field inverse and therefore through
//! `num-bigint` arithmetic that leaks operand magnitudes at the limb
//! level.
//!
//! This module sidesteps both problems for secp256k1 specifically:
//!
//! - **Homogeneous projective coordinates** `(X, Y, Z)` represent the
//!   affine point `(X/Z, Y/Z)`.  The identity is `(0, 1, 0)`.  No
//!   inverses are needed inside the addition or doubling chain — only
//!   one final inverse to convert back to affine.
//! - **Renes–Costello–Batina (2015) complete formulas** for the
//!   `y² = x³ + b` family work uniformly for all input combinations,
//!   including doublings, the identity, and `P + (-P)`.  No branches
//!   on input values are needed, so the formula is a fixed sequence
//!   of [`SecpFieldElement`] adds, subs, and muls — every one of which
//!   is constant-time.
//! - **Field arithmetic via [`SecpFieldElement`]** delegates to the
//!   Montgomery-form `mont_mul` over [`U256`], which is built from
//!   `u64×u64→u128` partial products and `u128` accumulators with
//!   no operand-dependent branching.
//!
//! The Montgomery ladder over this point type therefore has uniform
//! work per scalar bit at every layer of the stack — addition,
//! doubling, multiplication, reduction.  See [`SECURITY.md`] for the
//! wider hardening picture.
//!
//! # Reference
//!
//! Joost Renes, Craig Costello, Lejla Batina, *Complete addition
//! formulas for prime order elliptic curves*, EUROCRYPT 2016.  The
//! algorithms here are **Algorithm 7** (addition) and **Algorithm 9**
//! (doubling) for the special case `a = 0`, which secp256k1 satisfies.

use super::curve::CurveParams;
use super::field::FieldElement;
use super::point::Point;
use super::secp256k1_field::SecpFieldElement;
use crate::ct_bignum::{Uint, U256};
use num_bigint::BigUint;
use subtle::Choice;

/// Curve coefficient `b = 7` in Montgomery form.
///
/// `7·R mod p = 7·(2^32 + 977) = 0x7_0000_1AB7`.  Verified by
/// [`tests::b_constant_is_correct`].
pub const B_FE: SecpFieldElement =
    SecpFieldElement::from_montgomery_unchecked(Uint([0x0000_0007_0000_1AB7, 0, 0, 0]));

/// Precomputed `3·b = 21` in Montgomery form, as it appears in the RCB
/// formulas.
///
/// `21·R mod p = 21·(2^32 + 977) = 0x15_0000_5025`.  Verified by
/// [`tests::b3_constant_is_correct`].
pub const B3_FE: SecpFieldElement =
    SecpFieldElement::from_montgomery_unchecked(Uint([0x0000_0015_0000_5025, 0, 0, 0]));

/// A projective-coordinate point on secp256k1.  The triple
/// `(X, Y, Z)` encodes the affine point `(X/Z, Y/Z)`; the identity
/// is `(0, 1, 0)`.
#[derive(Copy, Clone, Debug)]
pub struct ProjectivePoint {
    pub x: SecpFieldElement,
    pub y: SecpFieldElement,
    pub z: SecpFieldElement,
}

impl ProjectivePoint {
    /// The point at infinity (curve identity), `(0, 1, 0)`.
    pub const IDENTITY: ProjectivePoint = ProjectivePoint {
        x: SecpFieldElement::ZERO,
        y: SecpFieldElement::ONE,
        z: SecpFieldElement::ZERO,
    };

    /// Build a projective point from canonical affine coordinates
    /// `(x, y) ∈ [0, p)²`.  The `Z` coordinate is set to 1.  Caller
    /// must ensure the point lies on the curve; this is not checked.
    pub fn from_affine(x: &U256, y: &U256) -> Self {
        ProjectivePoint {
            x: SecpFieldElement::from_canonical(x),
            y: SecpFieldElement::from_canonical(y),
            z: SecpFieldElement::ONE,
        }
    }

    /// Convert back to canonical affine coordinates.  Returns `None`
    /// for the point at infinity (`Z == 0`).
    ///
    /// Performs one field inverse — variable-time inside, but the
    /// inverse routine is implemented via the constant-time Fermat
    /// ladder over Montgomery-form arithmetic.  Acceptable as a final
    /// step of scalar multiplication, where the result is no longer
    /// dependent on per-bit secrets.
    pub fn to_affine(&self) -> Option<(U256, U256)> {
        if bool::from(self.z.ct_is_zero()) {
            return None;
        }
        let z_inv = self.z.inv();
        let x = self.x.mul(&z_inv).to_canonical();
        let y = self.y.mul(&z_inv).to_canonical();
        Some((x, y))
    }

    /// Constant-time check for the point at infinity.
    pub fn is_identity(&self) -> Choice {
        self.z.ct_is_zero()
    }

    /// Constant-time conditional move: returns `b` iff `choice == 1`.
    pub fn cmov(a: &Self, b: &Self, choice: Choice) -> Self {
        ProjectivePoint {
            x: SecpFieldElement::cmov(&a.x, &b.x, choice),
            y: SecpFieldElement::cmov(&a.y, &b.y, choice),
            z: SecpFieldElement::cmov(&a.z, &b.z, choice),
        }
    }

    /// Renes–Costello–Batina **Algorithm 7**: complete addition for
    /// short-Weierstrass curves with `a = 0`.  Works uniformly for all
    /// input combinations — different points, doublings, identity on
    /// either side, and `P + (-P)` — without branching on operand values.
    ///
    /// 12 multiplications + 2 multiplications by `b3` + 22 add/subs.
    pub fn add(&self, other: &Self) -> Self {
        let (x1, y1, z1) = (&self.x, &self.y, &self.z);
        let (x2, y2, z2) = (&other.x, &other.y, &other.z);

        let mut t0 = x1.mul(x2); // 1.  t0 = X1*X2
        let mut t1 = y1.mul(y2); // 2.  t1 = Y1*Y2
        let mut t2 = z1.mul(z2); // 3.  t2 = Z1*Z2
        let mut t3 = x1.add(y1); // 4.  t3 = X1+Y1
        let mut t4 = x2.add(y2); // 5.  t4 = X2+Y2
        t3 = t3.mul(&t4); // 6.  t3 = t3*t4
        t4 = t0.add(&t1); // 7.  t4 = t0+t1
        t3 = t3.sub(&t4); // 8.  t3 = t3-t4
        t4 = y1.add(z1); // 9.  t4 = Y1+Z1
        let mut x3 = y2.add(z2); // 10. X3 = Y2+Z2
        t4 = t4.mul(&x3); // 11. t4 = t4*X3
        x3 = t1.add(&t2); // 12. X3 = t1+t2
        t4 = t4.sub(&x3); // 13. t4 = t4-X3
        x3 = x1.add(z1); // 14. X3 = X1+Z1
        let mut y3 = x2.add(z2); // 15. Y3 = X2+Z2
        x3 = x3.mul(&y3); // 16. X3 = X3*Y3
        y3 = t0.add(&t2); // 17. Y3 = t0+t2
        y3 = x3.sub(&y3); // 18. Y3 = X3-Y3
        x3 = t0.add(&t0); // 19. X3 = t0+t0
        t0 = x3.add(&t0); // 20. t0 = X3+t0
        t2 = B3_FE.mul(&t2); // 21. t2 = b3*t2
        let mut z3 = t1.add(&t2); // 22. Z3 = t1+t2
        t1 = t1.sub(&t2); // 23. t1 = t1-t2
        y3 = B3_FE.mul(&y3); // 24. Y3 = b3*Y3
        x3 = t4.mul(&y3); // 25. X3 = t4*Y3
        t2 = t3.mul(&t1); // 26. t2 = t3*t1
        x3 = t2.sub(&x3); // 27. X3 = t2-X3
        y3 = y3.mul(&t0); // 28. Y3 = Y3*t0
        t1 = t1.mul(&z3); // 29. t1 = t1*Z3
        y3 = t1.add(&y3); // 30. Y3 = t1+Y3
        t0 = t0.mul(&t3); // 31. t0 = t0*t3
        z3 = z3.mul(&t4); // 32. Z3 = Z3*t4
        z3 = z3.add(&t0); // 33. Z3 = Z3+t0

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Renes–Costello–Batina **Algorithm 9**: complete doubling for
    /// short-Weierstrass curves with `a = 0`.  6 multiplications +
    /// 2 multiplications by `b3` + 9 add/subs.  Faster than calling
    /// `add(self, self)` because the formula exploits the equality.
    pub fn double(&self) -> Self {
        let (x, y, z) = (&self.x, &self.y, &self.z);

        let mut t0 = y.mul(y); // 1.  t0 = Y*Y
        let mut z3 = t0.add(&t0); // 2.  Z3 = t0+t0
        z3 = z3.add(&z3); // 3.  Z3 = Z3+Z3
        z3 = z3.add(&z3); // 4.  Z3 = Z3+Z3
        let mut t1 = y.mul(z); // 5.  t1 = Y*Z
        let mut t2 = z.mul(z); // 6.  t2 = Z*Z
        t2 = B3_FE.mul(&t2); // 7.  t2 = b3*t2
        let mut x3 = t2.mul(&z3); // 8.  X3 = t2*Z3
        let mut y3 = t0.add(&t2); // 9.  Y3 = t0+t2
        z3 = t1.mul(&z3); // 10. Z3 = t1*Z3
        t1 = t2.add(&t2); // 11. t1 = t2+t2
        t2 = t1.add(&t2); // 12. t2 = t1+t2
        t0 = t0.sub(&t2); // 13. t0 = t0-t2
        y3 = t0.mul(&y3); // 14. Y3 = t0*Y3
        y3 = x3.add(&y3); // 15. Y3 = X3+Y3
        t1 = x.mul(y); // 16. t1 = X*Y
        x3 = t0.mul(&t1); // 17. X3 = t0*t1
        x3 = x3.add(&x3); // 18. X3 = X3+X3

        ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Negate a point: `-(X, Y, Z) = (X, -Y, Z)`.
    pub fn neg(&self) -> Self {
        ProjectivePoint {
            x: self.x,
            y: self.y.neg(),
            z: self.z,
        }
    }

    /// Construct a `ProjectivePoint` from a textbook affine `Point`,
    /// converting the field elements via the canonical bignum
    /// representation.  The point at infinity maps to
    /// [`ProjectivePoint::IDENTITY`].
    pub fn from_textbook(p: &Point) -> Self {
        match p {
            Point::Infinity => ProjectivePoint::IDENTITY,
            Point::Affine { x, y } => ProjectivePoint::from_affine(
                &U256::from_biguint(&x.value),
                &U256::from_biguint(&y.value),
            ),
        }
    }

    /// Convert back to a textbook affine `Point` over the given prime
    /// modulus.  Returns [`Point::Infinity`] when `Z == 0`.
    pub fn to_textbook(&self, p: &BigUint) -> Point {
        match self.to_affine() {
            None => Point::Infinity,
            Some((x_u, y_u)) => Point::Affine {
                x: FieldElement::new(x_u.to_biguint(), p.clone()),
                y: FieldElement::new(y_u.to_biguint(), p.clone()),
            },
        }
    }

    /// Scalar multiplication via the Montgomery ladder, processing
    /// `scalar_bits` bits MSB-first.
    ///
    /// Every iteration performs **exactly one** addition and one
    /// doubling, regardless of the bit value.  The bit-driven branch
    /// is replaced with [`ProjectivePoint::cmov`] over the two
    /// candidate next-states, so on the Rust source level there is no
    /// `if`/`else` keyed on a secret bit.
    ///
    /// Caller specifies `scalar_bits` so the iteration count is fixed
    /// regardless of the position of the most-significant set bit of
    /// `k`.  For secp256k1 secrets this should be the curve order's
    /// bit length (256), not `k.bits()`.
    #[inline]
    pub fn cswap(a: &mut Self, b: &mut Self, choice: Choice) {
        SecpFieldElement::cswap(&mut a.x, &mut b.x, choice);
        SecpFieldElement::cswap(&mut a.y, &mut b.y, choice);
        SecpFieldElement::cswap(&mut a.z, &mut b.z, choice);
    }

    pub fn scalar_mul_ct(&self, k: &BigUint, scalar_bits: usize) -> Self {
        let mut r0 = ProjectivePoint::IDENTITY;
        let mut r1 = *self;
        let mut prev_bit = 0u8;
        for i in (0..scalar_bits).rev() {
            let bit = k.bit(i as u64) as u8;
            let swap = Choice::from(prev_bit ^ bit);
            Self::cswap(&mut r0, &mut r1, swap);
            prev_bit = bit;
            r1 = r0.add(&r1);
            r0 = r0.double();
        }
        Self::cswap(&mut r0, &mut r1, Choice::from(prev_bit));
        r0
    }
}

/// Drop-in constant-time scalar multiplication for **secp256k1 only**:
/// converts `point` into projective form, runs the
/// [`ProjectivePoint::scalar_mul_ct`] Montgomery ladder over the
/// Renes–Costello–Batina formulas, and returns the result as a
/// textbook [`Point`] for compatibility with the existing ECDH /
/// ECDSA code paths.
///
/// Falls back to the textbook affine [`Point::scalar_mul_ct`] for any
/// other curve, since the projective formulas in this module assume
/// `a = 0` (which only secp256k1 satisfies among the curves the crate
/// supports).
///
/// Use this for **secret** scalars only.  Public-input scalar
/// multiplications (the ECDSA verifier's `u₁·G + u₂·Q`) should keep
/// using the variable-time but faster [`Point::scalar_mul`].
pub fn ct_scalar_mul(point: &Point, k: &BigUint, curve: &CurveParams) -> Point {
    if curve.name == "secp256k1" {
        let pp = ProjectivePoint::from_textbook(point);
        let out = pp.scalar_mul_ct(k, curve.order_bits());
        out.to_textbook(&curve.p)
    } else {
        // Other curves (P-256) fall back to the affine ladder.  The
        // affine ladder still has the operation-count uniformity
        // property; what it lacks is the limb-level constant-time
        // field arithmetic that this module's projective ladder gets
        // from `SecpFieldElement`.
        point.scalar_mul_ct(k, &curve.a_fe(), curve.order_bits())
    }
}

// ── Variable-time GLV endomorphism & 4D Shamir fast paths (public inputs) ──

/// Cube root of unity `β ∈ F_p` (`β³ ≡ 1 mod p`, `β ≠ 1`) in Montgomery form,
/// defining the `j = 0` GLV endomorphism `ψ(x, y) = (β·x, y)` on secp256k1.
pub const BETA_FE: SecpFieldElement = SecpFieldElement::from_montgomery_unchecked(Uint([
    0x58a4_361c_8e81_894e,
    0x03fd_e163_1c4b_80af,
    0xf8e9_8978_d02e_3905,
    0x7a4a_36ae_bcbb_3d53,
]));

const GX_FE: SecpFieldElement = SecpFieldElement::from_montgomery_unchecked(Uint([
    0xd736_2e5a_487e_2097,
    0x231e_2953_29bc_66db,
    0x979f_48c0_33fd_129c,
    0x9981_e643_e908_9f48,
]));

const GY_FE: SecpFieldElement = SecpFieldElement::from_montgomery_unchecked(Uint([
    0xb15e_a6d2_d3db_abe2,
    0x8dfc_5d5d_1f1d_c64d,
    0x70b6_b59a_ac19_c136,
    0xcf3f_851f_d4a5_82d6,
]));

pub const SECP_N: U256 = Uint([
    0xbfd2_5e8c_d036_4141,
    0xbaae_dce6_af48_a03b,
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
]);

/// `P - N` for secp256k1 (`0x14551231950b75fc4402da1722fc9baee`).
const SECP_P_MINUS_N: U256 = Uint([
    0x402d_a172_2fc9_baee,
    0x4551_2319_50b7_5fc4,
    0x0000_0000_0000_0001,
    0x0000_0000_0000_0000,
]);

#[derive(Copy, Clone)]
struct SecpAffine {
    x: SecpFieldElement,
    y: SecpFieldElement,
}

#[derive(Copy, Clone)]
struct SecpJacobian {
    x: SecpFieldElement,
    y: SecpFieldElement,
    z: SecpFieldElement,
}

impl SecpJacobian {
    const IDENTITY: Self = Self {
        x: SecpFieldElement::ONE,
        y: SecpFieldElement::ONE,
        z: SecpFieldElement::ZERO,
    };

    #[inline(always)]
    fn is_identity(&self) -> bool {
        bool::from(self.z.ct_is_zero())
    }

    #[inline(always)]
    fn from_affine(p: &SecpAffine) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: SecpFieldElement::ONE,
        }
    }

    /// `a = 0` Jacobian doubling (`dbl-2009-l`, `2M + 5S`).
    #[inline]
    fn double(&self) -> Self {
        if self.is_identity() || bool::from(self.y.ct_is_zero()) {
            return Self::IDENTITY;
        }
        let a = self.x.sqr();
        let b = self.y.sqr();
        let c = b.sqr();
        let xpb = self.x.add(&b);
        let t = xpb.sqr().sub(&a).sub(&c);
        let d = t.add(&t);
        let e = a.add(&a).add(&a);
        let f = e.sqr();
        let x3 = f.sub(&d.add(&d));
        let c2 = c.add(&c);
        let c4 = c2.add(&c2);
        let c8 = c4.add(&c4);
        let y3 = e.mul(&d.sub(&x3)).sub(&c8);
        let yz = self.y.mul(&self.z);
        let z3 = yz.add(&yz);
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Mixed Jacobian + Affine addition (`madd-2007-bl`, `7M + 4S`).
    #[inline]
    fn add_affine(&self, q: &SecpAffine) -> Self {
        if self.is_identity() {
            return Self::from_affine(q);
        }
        let z1z1 = self.z.sqr();
        let u2 = q.x.mul(&z1z1);
        let s2 = q.y.mul(&self.z).mul(&z1z1);
        let h = u2.sub(&self.x);
        let hh = h.sqr();
        let i = hh.add(&hh);
        let i = i.add(&i);
        let j = h.mul(&i);
        let r = s2.sub(&self.y);
        if bool::from(h.ct_is_zero()) {
            return if bool::from(r.ct_is_zero()) {
                self.double()
            } else {
                Self::IDENTITY
            };
        }
        let r = r.add(&r);
        let v = self.x.mul(&i);
        let x3 = r.sqr().sub(&j).sub(&v.add(&v));
        let y1j = self.y.mul(&j);
        let y3 = r.mul(&v.sub(&x3)).sub(&y1j.add(&y1j));
        let z3 = self.z.add(&h).sqr().sub(&z1z1).sub(&hh);
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Full Jacobian + Jacobian addition (`add-2007-bl`, `11M + 5S`).
    #[inline]
    fn add_jac(&self, q: &Self) -> Self {
        if self.is_identity() {
            return *q;
        }
        if q.is_identity() {
            return *self;
        }
        let z1z1 = self.z.sqr();
        let z2z2 = q.z.sqr();
        let u1 = self.x.mul(&z2z2);
        let u2 = q.x.mul(&z1z1);
        let s1 = self.y.mul(&q.z).mul(&z2z2);
        let s2 = q.y.mul(&self.z).mul(&z1z1);
        let h = u2.sub(&u1);
        let r = s2.sub(&s1);
        if bool::from(h.ct_is_zero()) {
            return if bool::from(r.ct_is_zero()) {
                self.double()
            } else {
                Self::IDENTITY
            };
        }
        let i = h.add(&h).sqr();
        let j = h.mul(&i);
        let r = r.add(&r);
        let v = u1.mul(&i);
        let x3 = r.sqr().sub(&j).sub(&v.add(&v));
        let s1j = s1.mul(&j);
        let y3 = r.mul(&v.sub(&x3)).sub(&s1j.add(&s1j));
        let z3 = self.z.add(&q.z).sqr().sub(&z1z1).sub(&z2z2).mul(&h);
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    fn to_textbook(&self, p: &BigUint) -> Point {
        if self.is_identity() {
            return Point::Infinity;
        }
        let zi = self.z.inv();
        let zi2 = zi.sqr();
        let zi3 = zi2.mul(&zi);
        let x = self.x.mul(&zi2).to_biguint();
        let y = self.y.mul(&zi3).to_biguint();
        Point::Affine {
            x: FieldElement::new(x, p.clone()),
            y: FieldElement::new(y, p.clone()),
        }
    }
}

fn batch_jac_to_affine_8(pts: &[SecpJacobian; 8]) -> [SecpAffine; 8] {
    let mut pref = [SecpFieldElement::ONE; 8];
    pref[0] = pts[0].z;
    for i in 1..8 {
        pref[i] = pref[i - 1].mul(&pts[i].z);
    }
    let mut inv_acc = pref[7].inv();
    let mut out = [SecpAffine {
        x: SecpFieldElement::ZERO,
        y: SecpFieldElement::ZERO,
    }; 8];
    for i in (1..8).rev() {
        let zi = inv_acc.mul(&pref[i - 1]);
        inv_acc = inv_acc.mul(&pts[i].z);
        let zi2 = zi.sqr();
        out[i] = SecpAffine {
            x: pts[i].x.mul(&zi2),
            y: pts[i].y.mul(&zi2.mul(&zi)),
        };
    }
    let z0i2 = inv_acc.sqr();
    out[0] = SecpAffine {
        x: pts[0].x.mul(&z0i2),
        y: pts[0].y.mul(&z0i2.mul(&inv_acc)),
    };
    out
}

fn secp_gen_odd_table() -> &'static [SecpAffine; 8] {
    use std::sync::OnceLock;
    static TAB: OnceLock<[SecpAffine; 8]> = OnceLock::new();
    TAB.get_or_init(|| {
        let g = SecpAffine { x: GX_FE, y: GY_FE };
        let g_jac = SecpJacobian::from_affine(&g);
        let g2 = g_jac.double();
        let mut jtab = [g_jac; 8];
        for i in 1..8 {
            jtab[i] = jtab[i - 1].add_jac(&g2);
        }
        batch_jac_to_affine_8(&jtab)
    })
}

/// Stack-resident width-`w` NAF recoding of a <= 256-bit unsigned integer (`[u64; 4]`).
#[inline]
fn wnaf_u256(mut limbs: [u64; 4], w: u32, out: &mut [i8; 258]) -> usize {
    let mask = (1u64 << w) - 1;
    let half = 1i64 << (w - 1);
    let full = 1i64 << w;
    let mut len = 0usize;
    while (limbs[0] | limbs[1] | limbs[2] | limbs[3]) != 0 {
        if (limbs[0] & 1) == 1 {
            let mut d = (limbs[0] & mask) as i64;
            if d >= half {
                d -= full;
                // limbs += (-d)
                let add_v = (-d) as u64;
                let (l0, c1) = limbs[0].overflowing_add(add_v);
                let (l1, c2) = limbs[1].overflowing_add(c1 as u64);
                let (l2, c3) = limbs[2].overflowing_add(c2 as u64);
                limbs[0] = l0;
                limbs[1] = l1;
                limbs[2] = l2;
                limbs[3] = limbs[3].wrapping_add(c3 as u64);
            } else {
                limbs[0] &= !mask;
            }
            out[len] = d as i8;
        } else {
            out[len] = 0;
        }
        limbs[0] = (limbs[0] >> 1) | (limbs[1] << 63);
        limbs[1] = (limbs[1] >> 1) | (limbs[2] << 63);
        limbs[2] = (limbs[2] >> 1) | (limbs[3] << 63);
        limbs[3] >>= 1;
        len += 1;
    }
    len
}

/// Babai nearest-plane GLV-2 decomposition `k ≡ k₁ + k₂·λ (mod n)` on secp256k1,
/// returning `(abs_k1_limbs, neg_k1, abs_k2_limbs, neg_k2)` with `|k₁|, |k₂| < 2^129`.
fn glv_decompose_secp(k: &BigUint) -> ([u64; 4], bool, [u64; 4], bool) {
    use num_bigint::{BigInt, Sign};
    use num_integer::Integer;
    use std::sync::OnceLock;

    struct GlvConsts {
        n: BigUint,
        n_int: BigInt,
        a1: BigInt,
        b1: BigInt,
        a2: BigInt,
        b2: BigInt,
    }
    static CONSTS: OnceLock<GlvConsts> = OnceLock::new();
    let c = CONSTS.get_or_init(|| {
        let n = SECP_N.to_biguint();
        let n_int = BigInt::from(n.clone());
        let a1 = BigInt::from(BigUint::parse_bytes(b"3086d221a7d46bcde86c90e49284eb15", 16).unwrap());
        let b1 = -BigInt::from(BigUint::parse_bytes(b"e4437ed6010e88286f547fa90abfe4c3", 16).unwrap());
        let a2 = BigInt::from(BigUint::parse_bytes(b"0114ca50f7a8e2f3f657c1108d9d44cfd8", 16).unwrap());
        let b2 = a1.clone();
        GlvConsts { n, n_int, a1, b1, a2, b2 }
    });

    let k_red = BigInt::from(k % &c.n);
    let two_n = &c.n_int << 1usize;
    let r1 = ((&k_red * &c.b2) * 2u8 + &c.n_int).div_floor(&two_n);
    let r2 = ((-&k_red * &c.b1) * 2u8 + &c.n_int).div_floor(&two_n);
    let k1 = &k_red - &r1 * &c.a1 - &r2 * &c.a2;
    let k2 = -&r1 * &c.b1 - &r2 * &c.b2;

    let to_limbs = |v: &BigInt| -> ([u64; 4], bool) {
        let neg = v.sign() == Sign::Minus;
        let u = U256::from_biguint(v.magnitude());
        (u.0, neg)
    };
    let (l1, n1) = to_limbs(&k1);
    let (l2, n2) = to_limbs(&k2);
    (l1, n1, l2, n2)
}

#[inline(always)]
fn lookup_signed_aff(tab: &[SecpAffine], d: i8, flip: bool) -> SecpAffine {
    let neg = (d < 0) ^ flip;
    let idx = ((d.unsigned_abs() as usize) - 1) >> 1;
    let mut pt = tab[idx];
    if neg {
        pt.y = pt.y.neg();
    }
    pt
}

#[inline(always)]
fn lookup_signed_jac(tab: &[SecpJacobian], d: i8, flip: bool) -> SecpJacobian {
    let neg = (d < 0) ^ flip;
    let idx = ((d.unsigned_abs() as usize) - 1) >> 1;
    let mut pt = tab[idx];
    if neg {
        pt.y = pt.y.neg();
    }
    pt
}

/// Fast variable-time `k · P` on secp256k1 using the `ψ(x, y) = (β·x, y)` GLV-2
/// endomorphism (`128` shared `2M+5S` doublings).  Verifies `P` lies on `y² = x³ + 7`
/// before applying `ψ`; off-curve inputs fall back to Jacobian `w = 4` wNAF on their
/// own curve so formula-compatible tests behave identically to `Point::scalar_mul`.
pub fn scalar_mul_glv_vartime(point: &Point, k: &BigUint) -> Point {
    use num_traits::Zero;
    if k.is_zero() {
        return Point::Infinity;
    }
    let (px_bu, py_bu, p_mod) = match point {
        Point::Infinity => return Point::Infinity,
        Point::Affine { x, y } => (&x.value, &y.value, &x.modulus),
    };
    let p_aff = SecpAffine {
        x: SecpFieldElement::from_canonical(&U256::from_biguint(px_bu)),
        y: SecpFieldElement::from_canonical(&U256::from_biguint(py_bu)),
    };
    let on_secp = p_aff.y.sqr() == p_aff.x.sqr().mul(&p_aff.x).add(&B_FE);
    if !on_secp {
        // Off-curve fallback (exact `a = 0` Jacobian wNAF without GLV):
        let p_jac = SecpJacobian::from_affine(&p_aff);
        let mut acc = SecpJacobian::IDENTITY;
        for i in (0..k.bits()).rev() {
            if !acc.is_identity() {
                acc = acc.double();
            }
            if k.bit(i) {
                acc = acc.add_affine(&p_aff);
            }
        }
        let _ = p_jac;
        return acc.to_textbook(p_mod);
    }

    let (l1, neg1, l2, neg2) = glv_decompose_secp(k);
    let mut d1 = [0i8; 258];
    let mut d2 = [0i8; 258];
    let len1 = wnaf_u256(l1, 5, &mut d1);
    let len2 = wnaf_u256(l2, 5, &mut d2);
    let top = len1.max(len2);
    if top == 0 {
        return Point::Infinity;
    }

    let is_gen = p_aff.x == GX_FE && p_aff.y == GY_FE;
    if is_gen {
        let g_tab = secp_gen_odd_table();
        let mut q = SecpJacobian::IDENTITY;
        for i in (0..top).rev() {
            if !q.is_identity() {
                q = q.double();
            }
            let a = if i < len1 { d1[i] } else { 0 };
            if a != 0 {
                let entry = lookup_signed_aff(g_tab, a, neg1);
                q = q.add_affine(&entry);
            }
            let b = if i < len2 { d2[i] } else { 0 };
            if b != 0 {
                let mut entry = lookup_signed_aff(g_tab, b, neg2);
                entry.x = entry.x.mul(&BETA_FE);
                q = q.add_affine(&entry);
            }
        }
        return q.to_textbook(p_mod);
    }

    // Arbitrary on-curve base point: `w = 4` table (`1, 3, 5, 7`), `ψ` applied by `X * β`.
    let mut d1 = [0i8; 258];
    let mut d2 = [0i8; 258];
    let len1 = wnaf_u256(l1, 4, &mut d1);
    let len2 = wnaf_u256(l2, 4, &mut d2);
    let top = len1.max(len2);

    let p_jac = SecpJacobian::from_affine(&p_aff);
    let p2 = p_jac.double();
    let mut tab = [p_jac; 4];
    for i in 1..4 {
        tab[i] = tab[i - 1].add_jac(&p2);
    }
    let mut q = SecpJacobian::IDENTITY;
    for i in (0..top).rev() {
        if !q.is_identity() {
            q = q.double();
        }
        let a = if i < len1 { d1[i] } else { 0 };
        if a != 0 {
            if a.unsigned_abs() == 1 {
                let entry = lookup_signed_aff(&[p_aff], a, neg1);
                q = q.add_affine(&entry);
            } else {
                let entry = lookup_signed_jac(&tab, a, neg1);
                q = q.add_jac(&entry);
            }
        }
        let b = if i < len2 { d2[i] } else { 0 };
        if b != 0 {
            if b.unsigned_abs() == 1 {
                let mut entry = lookup_signed_aff(&[p_aff], b, neg2);
                entry.x = entry.x.mul(&BETA_FE);
                q = q.add_affine(&entry);
            } else {
                let mut entry = lookup_signed_jac(&tab, b, neg2);
                entry.x = entry.x.mul(&BETA_FE);
                q = q.add_jac(&entry);
            }
        }
    }
    q.to_textbook(p_mod)
}

/// Fast `SecpFieldElement` on-curve check for public-key validation on unaltered secp256k1.
#[inline]
pub fn is_valid_secp256k1_public_point(point: &Point) -> bool {
    let (x, y) = match point {
        Point::Infinity => return false,
        Point::Affine { x, y } => (&x.value, &y.value),
    };
    if x.bits() > 256 || y.bits() > 256 {
        return false;
    }
    let xu = U256::from_biguint(x);
    let yu = U256::from_biguint(y);
    if !bool::from(xu.ct_lt(&super::secp256k1_field::P))
        || !bool::from(yu.ct_lt(&super::secp256k1_field::P))
    {
        return false;
    }
    let xf = SecpFieldElement::from_canonical(&xu);
    let yf = SecpFieldElement::from_canonical(&yu);
    yf.sqr() == xf.sqr().mul(&xf).add(&B_FE)
}

/// **4-Dimensional GLV + Shamir ECDSA Verification** on secp256k1:
/// computes `[u₁]G + [u₂]Q = [a₁]G + [a₂]ψ(G) + [b₁]Q + [b₂]ψ(Q)` in `128` shared
/// `2M + 5S` doublings and checks `(x₁ mod n) == r` **projectively (`0` field inversions!)**
/// via `X ≡ r·Z² (mod p)` (and `X ≡ (r + n)·Z² (mod p)` when `r < p − n`).
pub fn verify_ecdsa_secp256k1_glv4d(
    u1: &BigUint,
    pubkey: &Point,
    u2: &BigUint,
    sig_r: &BigUint,
) -> bool {
    let (qx_bu, qy_bu) = match pubkey {
        Point::Infinity => return false,
        Point::Affine { x, y } => (&x.value, &y.value),
    };
    let q_aff = SecpAffine {
        x: SecpFieldElement::from_canonical(&U256::from_biguint(qx_bu)),
        y: SecpFieldElement::from_canonical(&U256::from_biguint(qy_bu)),
    };
    let psi_q_aff = SecpAffine {
        x: q_aff.x.mul(&BETA_FE),
        y: q_aff.y,
    };

    let (a1_l, neg_a1, a2_l, neg_a2) = glv_decompose_secp(u1);
    let (b1_l, neg_b1, b2_l, neg_b2) = glv_decompose_secp(u2);

    let mut da1 = [0i8; 258];
    let mut da2 = [0i8; 258];
    let mut db1 = [0i8; 258];
    let mut db2 = [0i8; 258];
    let la1 = wnaf_u256(a1_l, 5, &mut da1);
    let la2 = wnaf_u256(a2_l, 5, &mut da2);
    let lb1 = wnaf_u256(b1_l, 4, &mut db1);
    let lb2 = wnaf_u256(b2_l, 4, &mut db2);
    let top = la1.max(la2).max(lb1).max(lb2);
    if top == 0 {
        return false;
    }

    let g_tab = secp_gen_odd_table();
    let q_jac = SecpJacobian::from_affine(&q_aff);
    let q2 = q_jac.double();
    let mut q_tab = [q_jac; 4];
    for i in 1..4 {
        q_tab[i] = q_tab[i - 1].add_jac(&q2);
    }

    let mut acc = SecpJacobian::IDENTITY;
    for i in (0..top).rev() {
        if !acc.is_identity() {
            acc = acc.double();
        }
        let d = if i < la1 { da1[i] } else { 0 };
        if d != 0 {
            acc = acc.add_affine(&lookup_signed_aff(g_tab, d, neg_a1));
        }
        let d = if i < la2 { da2[i] } else { 0 };
        if d != 0 {
            let mut e = lookup_signed_aff(g_tab, d, neg_a2);
            e.x = e.x.mul(&BETA_FE);
            acc = acc.add_affine(&e);
        }
        let d = if i < lb1 { db1[i] } else { 0 };
        if d != 0 {
            if d.unsigned_abs() == 1 {
                acc = acc.add_affine(&lookup_signed_aff(&[q_aff], d, neg_b1));
            } else {
                acc = acc.add_jac(&lookup_signed_jac(&q_tab, d, neg_b1));
            }
        }
        let d = if i < lb2 { db2[i] } else { 0 };
        if d != 0 {
            if d.unsigned_abs() == 1 {
                acc = acc.add_affine(&lookup_signed_aff(&[psi_q_aff], d, neg_b2));
            } else {
                let mut e = lookup_signed_jac(&q_tab, d, neg_b2);
                e.x = e.x.mul(&BETA_FE);
                acc = acc.add_jac(&e);
            }
        }
    }

    if acc.is_identity() {
        return false;
    }

    // Inversion-free projective equality test `(X / Z² mod p) mod n == sig_r`:
    let r_u256 = U256::from_biguint(sig_r);
    let r_fe = SecpFieldElement::from_canonical(&r_u256);
    let z2 = acc.z.sqr();
    if acc.x == r_fe.mul(&z2) {
        return true;
    }
    if bool::from(r_u256.ct_lt(&SECP_P_MINUS_N)) {
        let (rn, _) = U256::adc(&r_u256, &SECP_N);
        let rn_fe = SecpFieldElement::from_canonical(&rn);
        if acc.x == rn_fe.mul(&z2) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecc::curve::CurveParams;
    use crate::ecc::field::FieldElement;
    use crate::ecc::point::Point;
    use num_bigint::BigUint;
    use num_traits::Zero;

    fn secp() -> CurveParams {
        CurveParams::secp256k1()
    }

    /// Convert a textbook `Point` to a `ProjectivePoint` for cross-checking.
    fn proj_from_textbook(p: &Point) -> ProjectivePoint {
        match p {
            Point::Infinity => ProjectivePoint::IDENTITY,
            Point::Affine { x, y } => {
                let xu = U256::from_biguint(&x.value);
                let yu = U256::from_biguint(&y.value);
                ProjectivePoint::from_affine(&xu, &yu)
            }
        }
    }

    /// Compare a `ProjectivePoint` to a textbook `Point` by mapping the
    /// projective one back to affine and comparing canonical coordinates.
    fn proj_eq_textbook(pp: &ProjectivePoint, p: &Point) -> bool {
        match (pp.to_affine(), p) {
            (None, Point::Infinity) => true,
            (Some((x, y)), Point::Affine { x: xf, y: yf }) => {
                x.to_biguint() == xf.value && y.to_biguint() == yf.value
            }
            _ => false,
        }
    }

    #[test]
    fn b_constant_is_correct() {
        let want = SecpFieldElement::from_biguint(&BigUint::from(7u8));
        assert_eq!(B_FE, want);
    }

    #[test]
    fn b3_constant_is_correct() {
        let want = SecpFieldElement::from_biguint(&BigUint::from(21u8));
        assert_eq!(B3_FE, want);
    }

    #[test]
    fn identity_to_affine_is_none() {
        assert!(ProjectivePoint::IDENTITY.to_affine().is_none());
        assert!(bool::from(ProjectivePoint::IDENTITY.is_identity()));
    }

    #[test]
    fn from_affine_roundtrips() {
        let g = secp().generator();
        let (gx_bu, gy_bu) = match &g {
            Point::Affine { x, y } => (x.value.clone(), y.value.clone()),
            Point::Infinity => panic!("generator is not infinity"),
        };
        let pp =
            ProjectivePoint::from_affine(&U256::from_biguint(&gx_bu), &U256::from_biguint(&gy_bu));
        let (x_back, y_back) = pp.to_affine().unwrap();
        assert_eq!(x_back.to_biguint(), gx_bu);
        assert_eq!(y_back.to_biguint(), gy_bu);
    }

    #[test]
    fn add_matches_textbook_for_distinct_points() {
        let curve = secp();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2 = g.double(&a);
        let g3_textbook = g.add(&g2, &a);

        let pg = proj_from_textbook(&g);
        let pg2 = proj_from_textbook(&g2);
        let pg3 = pg.add(&pg2);

        assert!(proj_eq_textbook(&pg3, &g3_textbook));
    }

    #[test]
    fn add_matches_textbook_for_doubling_via_add() {
        let curve = secp();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2_textbook = g.double(&a);

        let pg = proj_from_textbook(&g);
        let pg_plus_pg = pg.add(&pg);

        assert!(proj_eq_textbook(&pg_plus_pg, &g2_textbook));
    }

    #[test]
    fn double_matches_textbook() {
        let curve = secp();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2_textbook = g.double(&a);

        let pg = proj_from_textbook(&g);
        let pg2 = pg.double();

        assert!(proj_eq_textbook(&pg2, &g2_textbook));
    }

    #[test]
    fn add_with_identity_is_identity_law() {
        let g = secp().generator();
        let pg = proj_from_textbook(&g);
        let lhs = pg.add(&ProjectivePoint::IDENTITY);
        let rhs = ProjectivePoint::IDENTITY.add(&pg);
        assert!(proj_eq_textbook(&lhs, &g));
        assert!(proj_eq_textbook(&rhs, &g));
    }

    #[test]
    fn add_with_neg_yields_identity() {
        let g = secp().generator();
        let pg = proj_from_textbook(&g);
        let neg_pg = pg.neg();
        let sum = pg.add(&neg_pg);
        assert!(bool::from(sum.is_identity()));
    }

    #[test]
    fn scalar_mul_ct_matches_textbook_small_scalars() {
        let curve = secp();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let pg = proj_from_textbook(&g);
        for k in 0u64..=10 {
            let k_bu = BigUint::from(k);
            let want = g.scalar_mul_ct(&k_bu, &a_fe, 4);
            let got = pg.scalar_mul_ct(&k_bu, 4);
            assert!(proj_eq_textbook(&got, &want), "mismatch at k={}", k);
        }
    }

    #[test]
    fn scalar_mul_ct_matches_textbook_full_size_scalars() {
        let curve = secp();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let pg = proj_from_textbook(&g);
        let bits = curve.order_bits();
        for k_hex in [
            "1",
            "2",
            "deadbeef",
            // Just under the curve order
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364140",
            // Random 256-bit value (fits in scalar range)
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        ] {
            let k = BigUint::parse_bytes(k_hex.as_bytes(), 16).unwrap();
            let want = g.scalar_mul_ct(&k, &a_fe, bits);
            let got = pg.scalar_mul_ct(&k, bits);
            assert!(proj_eq_textbook(&got, &want), "mismatch at k={}", k_hex);
        }
    }

    #[test]
    fn cmov_selects_correctly() {
        let g = secp().generator();
        let pg = proj_from_textbook(&g);
        let id = ProjectivePoint::IDENTITY;

        let r0 = ProjectivePoint::cmov(&pg, &id, Choice::from(0));
        let r1 = ProjectivePoint::cmov(&pg, &id, Choice::from(1));

        assert!(proj_eq_textbook(&r0, &g));
        assert!(bool::from(r1.is_identity()));
    }

    /// Sanity: scalar mul by curve order should yield identity.
    #[test]
    fn scalar_mul_by_order_is_identity() {
        let curve = secp();
        let g = curve.generator();
        let pg = proj_from_textbook(&g);
        let n = curve.n.clone();
        let bits = curve.order_bits();
        let result = pg.scalar_mul_ct(&n, bits);
        assert!(bool::from(result.is_identity()));
        // (Defeat unused-import warning for FieldElement if reached here.)
        let _ = FieldElement::zero(BigUint::from(7u8));
    }

    /// Sanity: doubling identity stays identity.
    #[test]
    fn double_identity_is_identity() {
        let r = ProjectivePoint::IDENTITY.double();
        assert!(bool::from(r.is_identity()));
        // Suppress `Zero`-import warning if test set narrows.
        let _ = BigUint::zero();
    }
}
