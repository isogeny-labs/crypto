//! Constant-time projective-coordinate point arithmetic over P-256.
//!
//! Mirrors [`crate::ecc::secp256k1_point`], but specialised for
//! P-256's curve parameters (`a = -3 mod p`, generic `b`).  Where
//! secp256k1 used the Renes–Costello–Batina algorithms specialised to
//! `a = 0` (Algorithms 7 + 9), this module uses the general-`a`
//! complete formulas (**Algorithm 1** for addition, **Algorithm 3**
//! for doubling) with `a` and `b3 = 3b` carried as
//! [`P256FieldElement`] constants.  Algorithm 1 has 12 multiplications
//! + 3 multiplications by `a` + 2 multiplications by `b3` per
//!   addition; Algorithm 3 has 8 + 3·a + 2·b3 per doubling.  Both
//!   formulas are *complete* — they work uniformly for distinct points,
//!   doublings, the identity on either side, and `P + (-P)`, with no
//!   branches on input values.
//!
//! The Montgomery ladder over this point type replaces the bit-driven
//! `if` with [`P256FieldElement::cmov`] over candidate next-states,
//! matching the constant-time profile of the secp256k1 ladder.

use super::curve::CurveParams;
use super::field::FieldElement;
use super::p256_field::P256FieldElement;
use super::point::Point;
use crate::ct_bignum::{Uint, U256};
use num_bigint::BigUint;
use subtle::Choice;

/// Curve coefficient `a = -3 mod p` in Montgomery form.
///
/// `(p - 3)·R mod p` — verified by [`tests::a_constant_is_correct`].
pub const A_FE: P256FieldElement = P256FieldElement::from_montgomery_unchecked(Uint([
    0xFFFF_FFFF_FFFF_FFFC,
    0x0000_0003_FFFF_FFFF,
    0x0000_0000_0000_0000,
    0xFFFF_FFFC_0000_0004,
]));

/// Curve coefficient `b` (from y² = x³ + ax + b) in Montgomery form.
/// `b·R mod p`.  Verified by [`tests::b_constant_is_correct`].
pub const B_FE: P256FieldElement = P256FieldElement::from_montgomery_unchecked(Uint([
    0xD89C_DF62_29C4_BDDF,
    0xACF0_05CD_7884_3090,
    0xE5A2_20AB_F721_2ED6,
    0xDC30_061D_0487_4834,
]));

/// `3·b` in Montgomery form, as it appears in the RCB formulas.
/// Verified by [`tests::b3_constant_is_correct`].
pub const B3_FE: P256FieldElement = P256FieldElement::from_montgomery_unchecked(Uint([
    0x89D6_9E26_7D4E_399F,
    0x06D0_1166_698C_91B2,
    0xB0E6_6203_E563_8C84,
    0x9490_1259_0D95_D89C,
]));

/// A projective-coordinate point on P-256.  Encoding identical to
/// [`crate::ecc::secp256k1_point::ProjectivePoint`] — the identity is
/// `(0, 1, 0)`.
#[derive(Copy, Clone, Debug)]
pub struct P256ProjectivePoint {
    pub x: P256FieldElement,
    pub y: P256FieldElement,
    pub z: P256FieldElement,
}

impl P256ProjectivePoint {
    pub const IDENTITY: P256ProjectivePoint = P256ProjectivePoint {
        x: P256FieldElement::ZERO,
        y: P256FieldElement::ONE,
        z: P256FieldElement::ZERO,
    };

    pub fn from_affine(x: &U256, y: &U256) -> Self {
        P256ProjectivePoint {
            x: P256FieldElement::from_canonical(x),
            y: P256FieldElement::from_canonical(y),
            z: P256FieldElement::ONE,
        }
    }

    pub fn to_affine(&self) -> Option<(U256, U256)> {
        if bool::from(self.z.ct_is_zero()) {
            return None;
        }
        let z_inv = self.z.inv();
        let x = self.x.mul(&z_inv).to_canonical();
        let y = self.y.mul(&z_inv).to_canonical();
        Some((x, y))
    }

    pub fn is_identity(&self) -> Choice {
        self.z.ct_is_zero()
    }

    pub fn cmov(a: &Self, b: &Self, choice: Choice) -> Self {
        P256ProjectivePoint {
            x: P256FieldElement::cmov(&a.x, &b.x, choice),
            y: P256FieldElement::cmov(&a.y, &b.y, choice),
            z: P256FieldElement::cmov(&a.z, &b.z, choice),
        }
    }

    pub fn from_textbook(p: &Point) -> Self {
        match p {
            Point::Infinity => P256ProjectivePoint::IDENTITY,
            Point::Affine { x, y } => P256ProjectivePoint::from_affine(
                &U256::from_biguint(&x.value),
                &U256::from_biguint(&y.value),
            ),
        }
    }

    pub fn to_textbook(&self, p: &BigUint) -> Point {
        match self.to_affine() {
            None => Point::Infinity,
            Some((x_u, y_u)) => Point::Affine {
                x: FieldElement::new(x_u.to_biguint(), p.clone()),
                y: FieldElement::new(y_u.to_biguint(), p.clone()),
            },
        }
    }

    /// Renes–Costello–Batina **Algorithm 1**: complete addition for
    /// arbitrary short Weierstrass curves.  17 multiplications total
    /// (12 generic + 3·a + 2·b3) and 22 add/subs.
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
        t4 = x1.add(z1); // 9.  t4 = X1+Z1
        let mut t5 = x2.add(z2); // 10. t5 = X2+Z2
        t4 = t4.mul(&t5); // 11. t4 = t4*t5
        t5 = t0.add(&t2); // 12. t5 = t0+t2
        t4 = t4.sub(&t5); // 13. t4 = t4-t5
        t5 = y1.add(z1); // 14. t5 = Y1+Z1
        let mut x3 = y2.add(z2); // 15. X3 = Y2+Z2
        t5 = t5.mul(&x3); // 16. t5 = t5*X3
        x3 = t1.add(&t2); // 17. X3 = t1+t2
        t5 = t5.sub(&x3); // 18. t5 = t5-X3
        let mut z3 = A_FE.mul(&t4); // 19. Z3 = a*t4
        x3 = B3_FE.mul(&t2); // 20. X3 = b3*t2
        z3 = x3.add(&z3); // 21. Z3 = X3+Z3
        x3 = t1.sub(&z3); // 22. X3 = t1-Z3
        z3 = t1.add(&z3); // 23. Z3 = t1+Z3
        let mut y3 = x3.mul(&z3); // 24. Y3 = X3*Z3
        t1 = t0.add(&t0); // 25. t1 = t0+t0
        t1 = t1.add(&t0); // 26. t1 = t1+t0
        t2 = A_FE.mul(&t2); // 27. t2 = a*t2
        t4 = B3_FE.mul(&t4); // 28. t4 = b3*t4
        t1 = t1.add(&t2); // 29. t1 = t1+t2
        t2 = t0.sub(&t2); // 30. t2 = t0-t2
        t2 = A_FE.mul(&t2); // 31. t2 = a*t2
        t4 = t4.add(&t2); // 32. t4 = t4+t2
        t0 = t1.mul(&t4); // 33. t0 = t1*t4
        y3 = y3.add(&t0); // 34. Y3 = Y3+t0
        t0 = t5.mul(&t4); // 35. t0 = t5*t4
        x3 = t3.mul(&x3); // 36. X3 = t3*X3
        x3 = x3.sub(&t0); // 37. X3 = X3-t0
        t0 = t3.mul(&t1); // 38. t0 = t3*t1
        z3 = t5.mul(&z3); // 39. Z3 = t5*Z3
        z3 = z3.add(&t0); // 40. Z3 = Z3+t0

        P256ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Renes–Costello–Batina **Algorithm 3**: complete doubling for
    /// arbitrary short Weierstrass curves.  13 multiplications total
    /// (8 generic + 3·a + 2·b3) and 14 add/subs.
    pub fn double(&self) -> Self {
        let (x, y, z) = (&self.x, &self.y, &self.z);

        let mut t0 = x.mul(x); // 1.  t0 = X*X
        let t1 = y.mul(y); // 2.  t1 = Y*Y
        let mut t2 = z.mul(z); // 3.  t2 = Z*Z
        let mut t3 = x.mul(y); // 4.  t3 = X*Y
        t3 = t3.add(&t3); // 5.  t3 = t3+t3
        let mut z3 = x.mul(z); // 6.  Z3 = X*Z
        z3 = z3.add(&z3); // 7.  Z3 = Z3+Z3
        let mut x3 = A_FE.mul(&z3); // 8.  X3 = a*Z3
        let mut y3 = B3_FE.mul(&t2); // 9.  Y3 = b3*t2
        y3 = x3.add(&y3); // 10. Y3 = X3+Y3
        x3 = t1.sub(&y3); // 11. X3 = t1-Y3
        y3 = t1.add(&y3); // 12. Y3 = t1+Y3
        y3 = x3.mul(&y3); // 13. Y3 = X3*Y3
        x3 = t3.mul(&x3); // 14. X3 = t3*X3
        z3 = B3_FE.mul(&z3); // 15. Z3 = b3*Z3
        t2 = A_FE.mul(&t2); // 16. t2 = a*t2
        t3 = t0.sub(&t2); // 17. t3 = t0-t2
        t3 = A_FE.mul(&t3); // 18. t3 = a*t3
        t3 = t3.add(&z3); // 19. t3 = t3+Z3
        z3 = t0.add(&t0); // 20. Z3 = t0+t0
        t0 = z3.add(&t0); // 21. t0 = Z3+t0
        t0 = t0.add(&t2); // 22. t0 = t0+t2
        t0 = t0.mul(&t3); // 23. t0 = t0*t3
        y3 = y3.add(&t0); // 24. Y3 = Y3+t0
        t2 = y.mul(z); // 25. t2 = Y*Z
        t2 = t2.add(&t2); // 26. t2 = t2+t2
        t0 = t2.mul(&t3); // 27. t0 = t2*t3
        x3 = x3.sub(&t0); // 28. X3 = X3-t0
        z3 = t2.mul(&t1); // 29. Z3 = t2*t1
        z3 = z3.add(&z3); // 30. Z3 = Z3+Z3
        z3 = z3.add(&z3); // 31. Z3 = Z3+Z3

        P256ProjectivePoint {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn neg(&self) -> Self {
        P256ProjectivePoint {
            x: self.x,
            y: self.y.neg(),
            z: self.z,
        }
    }

    #[inline]
    pub fn cswap(a: &mut Self, b: &mut Self, choice: Choice) {
        P256FieldElement::cswap(&mut a.x, &mut b.x, choice);
        P256FieldElement::cswap(&mut a.y, &mut b.y, choice);
        P256FieldElement::cswap(&mut a.z, &mut b.z, choice);
    }

    /// Constant-time scalar multiplication via the Montgomery ladder.
    /// Same shape as
    /// [`crate::ecc::secp256k1_point::ProjectivePoint::scalar_mul_ct`]:
    /// uniform per-bit work (one add + one double), cswap-driven
    /// re-binding, fixed iteration count.
    pub fn scalar_mul_ct(&self, k: &BigUint, scalar_bits: usize) -> Self {
        let mut r0 = P256ProjectivePoint::IDENTITY;
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

/// Drop-in CT scalar multiplication for **P-256** that returns a
/// textbook [`Point`].  Companion to
/// [`crate::ecc::secp256k1_point::ct_scalar_mul`]; the unified
/// dispatcher in [`crate::ecc::ct`] picks the right one based on
/// `curve.name`.
pub fn ct_scalar_mul_p256(point: &Point, k: &BigUint, curve: &CurveParams) -> Point {
    let pp = P256ProjectivePoint::from_textbook(point);
    let out = pp.scalar_mul_ct(k, curve.order_bits());
    out.to_textbook(&curve.p)
}

// ── Variable-time Jacobian wNAF & 2D Shamir fast paths (public inputs) ──────

const P256_GX_FE: P256FieldElement = P256FieldElement::from_montgomery_unchecked(Uint([
    0x79e7_30d4_18a9_143c,
    0x75ba_95fc_5fed_b601,
    0x79fb_732b_7762_2510,
    0x1890_5f76_a537_55c6,
]));

const P256_GY_FE: P256FieldElement = P256FieldElement::from_montgomery_unchecked(Uint([
    0xddf2_5357_ce95_560a,
    0x8b4a_b8e4_ba19_e45c,
    0xd2e8_8688_dd21_f325,
    0x8571_ff18_2588_5d85,
]));

pub const P256_N: U256 = Uint([
    0xf3b9_cac2_fc63_2551,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
]);

/// `P - N` for P-256 (`0x00000000ffffffffffffffffffffffff4319055258e8617b0c46353d039cdaae`).
const P256_P_MINUS_N: U256 = Uint([
    0x0c46_353d_039c_daae,
    0x4319_0552_58e8_617b,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0001,
]);

#[derive(Copy, Clone)]
struct P256Affine {
    x: P256FieldElement,
    y: P256FieldElement,
}

#[derive(Copy, Clone)]
struct P256Jacobian {
    x: P256FieldElement,
    y: P256FieldElement,
    z: P256FieldElement,
}

impl P256Jacobian {
    const IDENTITY: Self = Self {
        x: P256FieldElement::ONE,
        y: P256FieldElement::ONE,
        z: P256FieldElement::ZERO,
    };

    #[inline(always)]
    fn is_identity(&self) -> bool {
        bool::from(self.z.ct_is_zero())
    }

    #[inline(always)]
    fn from_affine(p: &P256Affine) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: P256FieldElement::ONE,
        }
    }

    /// `a = -3` Jacobian doubling (`dbl-2001-b`, `3M + 5S`).
    #[inline]
    fn double(&self) -> Self {
        if self.is_identity() || bool::from(self.y.ct_is_zero()) {
            return Self::IDENTITY;
        }
        let delta = self.z.sqr();
        let gamma = self.y.sqr();
        let beta = self.x.mul(&gamma);
        let t = self.x.sub(&delta).mul(&self.x.add(&delta));
        let alpha = t.add(&t).add(&t);
        let beta2 = beta.add(&beta);
        let beta4 = beta2.add(&beta2);
        let beta8 = beta4.add(&beta4);
        let x3 = alpha.sqr().sub(&beta8);
        let z3 = self.y.add(&self.z).sqr().sub(&gamma).sub(&delta);
        let g2 = gamma.sqr();
        let g4 = g2.add(&g2);
        let g8 = g4.add(&g4);
        let g8 = g8.add(&g8);
        let y3 = alpha.mul(&beta4.sub(&x3)).sub(&g8);
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Mixed Jacobian + Affine addition (`madd-2007-bl`, `7M + 4S`).
    #[inline]
    fn add_affine(&self, q: &P256Affine) -> Self {
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

fn batch_p256_jac_to_affine_8(pts: &[P256Jacobian; 8]) -> [P256Affine; 8] {
    let mut pref = [P256FieldElement::ONE; 8];
    pref[0] = pts[0].z;
    for i in 1..8 {
        pref[i] = pref[i - 1].mul(&pts[i].z);
    }
    let mut inv_acc = pref[7].inv();
    let mut out = [P256Affine {
        x: P256FieldElement::ZERO,
        y: P256FieldElement::ZERO,
    }; 8];
    for i in (1..8).rev() {
        let zi = inv_acc.mul(&pref[i - 1]);
        inv_acc = inv_acc.mul(&pts[i].z);
        let zi2 = zi.sqr();
        out[i] = P256Affine {
            x: pts[i].x.mul(&zi2),
            y: pts[i].y.mul(&zi2.mul(&zi)),
        };
    }
    let z0i2 = inv_acc.sqr();
    out[0] = P256Affine {
        x: pts[0].x.mul(&z0i2),
        y: pts[0].y.mul(&z0i2.mul(&inv_acc)),
    };
    out
}

fn p256_gen_odd_table() -> &'static [P256Affine; 8] {
    use std::sync::OnceLock;
    static TAB: OnceLock<[P256Affine; 8]> = OnceLock::new();
    TAB.get_or_init(|| {
        let g = P256Affine {
            x: P256_GX_FE,
            y: P256_GY_FE,
        };
        let g_jac = P256Jacobian::from_affine(&g);
        let g2 = g_jac.double();
        let mut jtab = [g_jac; 8];
        for i in 1..8 {
            jtab[i] = jtab[i - 1].add_jac(&g2);
        }
        batch_p256_jac_to_affine_8(&jtab)
    })
}

#[inline]
fn wnaf_p256_u256(mut limbs: [u64; 4], w: u32, out: &mut [i8; 258]) -> usize {
    let mask = (1u64 << w) - 1;
    let half = 1i64 << (w - 1);
    let full = 1i64 << w;
    let mut len = 0usize;
    while (limbs[0] | limbs[1] | limbs[2] | limbs[3]) != 0 {
        if (limbs[0] & 1) == 1 {
            let mut d = (limbs[0] & mask) as i64;
            if d >= half {
                d -= full;
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

#[inline(always)]
fn lookup_p256_aff(tab: &[P256Affine], d: i8) -> P256Affine {
    let idx = ((d.unsigned_abs() as usize) - 1) >> 1;
    let mut pt = tab[idx];
    if d < 0 {
        pt.y = pt.y.neg();
    }
    pt
}

#[inline(always)]
fn lookup_p256_jac(tab: &[P256Jacobian], d: i8) -> P256Jacobian {
    let idx = ((d.unsigned_abs() as usize) - 1) >> 1;
    let mut pt = tab[idx];
    if d < 0 {
        pt.y = pt.y.neg();
    }
    pt
}

/// Fast variable-time `k · P` on P-256 (`a = -3`, `dbl-2001-b` `3M + 5S` + `madd-2007-bl`).
pub fn scalar_mul_p256_vartime(point: &Point, k: &BigUint) -> Point {
    use num_traits::Zero;
    if k.is_zero() {
        return Point::Infinity;
    }
    let (px_bu, py_bu, p_mod) = match point {
        Point::Infinity => return Point::Infinity,
        Point::Affine { x, y } => (&x.value, &y.value, &x.modulus),
    };
    let p_aff = P256Affine {
        x: P256FieldElement::from_canonical(&U256::from_biguint(px_bu)),
        y: P256FieldElement::from_canonical(&U256::from_biguint(py_bu)),
    };

    if k.bits() > 256 {
        let mut acc = P256Jacobian::IDENTITY;
        for i in (0..k.bits()).rev() {
            if !acc.is_identity() {
                acc = acc.double();
            }
            if k.bit(i) {
                acc = acc.add_affine(&p_aff);
            }
        }
        return acc.to_textbook(p_mod);
    }

    let k_limbs = U256::from_biguint(k).0;
    let is_gen = p_aff.x == P256_GX_FE && p_aff.y == P256_GY_FE;
    if is_gen {
        let mut digits = [0i8; 258];
        let len = wnaf_p256_u256(k_limbs, 5, &mut digits);
        let g_tab = p256_gen_odd_table();
        let mut acc = P256Jacobian::IDENTITY;
        for i in (0..len).rev() {
            if !acc.is_identity() {
                acc = acc.double();
            }
            let d = digits[i];
            if d != 0 {
                acc = acc.add_affine(&lookup_p256_aff(g_tab, d));
            }
        }
        return acc.to_textbook(p_mod);
    }

    let mut digits = [0i8; 258];
    let len = wnaf_p256_u256(k_limbs, 4, &mut digits);
    let p_jac = P256Jacobian::from_affine(&p_aff);
    let p2 = p_jac.double();
    let mut tab = [p_jac; 4];
    for i in 1..4 {
        tab[i] = tab[i - 1].add_jac(&p2);
    }
    let mut acc = P256Jacobian::IDENTITY;
    for i in (0..len).rev() {
        if !acc.is_identity() {
            acc = acc.double();
        }
        let d = digits[i];
        if d != 0 {
            if d.unsigned_abs() == 1 {
                acc = acc.add_affine(&lookup_p256_aff(&[p_aff], d));
            } else {
                acc = acc.add_jac(&lookup_p256_jac(&tab, d));
            }
        }
    }
    acc.to_textbook(p_mod)
}

/// Fast `P256FieldElement` on-curve check for public-key validation on unaltered P-256.
#[inline]
pub fn is_valid_p256_public_point(point: &Point) -> bool {
    let (x, y) = match point {
        Point::Infinity => return false,
        Point::Affine { x, y } => (&x.value, &y.value),
    };
    if x.bits() > 256 || y.bits() > 256 {
        return false;
    }
    let xu = U256::from_biguint(x);
    let yu = U256::from_biguint(y);
    if !bool::from(xu.ct_lt(&super::p256_field::P)) || !bool::from(yu.ct_lt(&super::p256_field::P))
    {
        return false;
    }
    let xf = P256FieldElement::from_canonical(&xu);
    let yf = P256FieldElement::from_canonical(&yu);
    let rhs = xf.sqr().mul(&xf).add(&A_FE.mul(&xf)).add(&B_FE);
    yf.sqr() == rhs
}

/// **2D Shamir ECDSA Verification** on P-256 (`[u₁]G + [u₂]Q`, cached `w = 5` affine `G`
/// table + `w = 4` `Q` table + **zero-inversion** projective `X ≡ r·Z² (mod p)` check).
pub fn verify_ecdsa_p256_shamir(
    u1: &BigUint,
    pubkey: &Point,
    u2: &BigUint,
    sig_r: &BigUint,
) -> bool {
    let (qx_bu, qy_bu) = match pubkey {
        Point::Infinity => return false,
        Point::Affine { x, y } => (&x.value, &y.value),
    };
    let q_aff = P256Affine {
        x: P256FieldElement::from_canonical(&U256::from_biguint(qx_bu)),
        y: P256FieldElement::from_canonical(&U256::from_biguint(qy_bu)),
    };

    let mut d1 = [0i8; 258];
    let mut d2 = [0i8; 258];
    let l1 = wnaf_p256_u256(U256::from_biguint(u1).0, 5, &mut d1);
    let l2 = wnaf_p256_u256(U256::from_biguint(u2).0, 4, &mut d2);
    let top = l1.max(l2);
    if top == 0 {
        return false;
    }

    let g_tab = p256_gen_odd_table();
    let q_jac = P256Jacobian::from_affine(&q_aff);
    let q2 = q_jac.double();
    let mut q_tab = [q_jac; 4];
    for i in 1..4 {
        q_tab[i] = q_tab[i - 1].add_jac(&q2);
    }

    let mut acc = P256Jacobian::IDENTITY;
    for i in (0..top).rev() {
        if !acc.is_identity() {
            acc = acc.double();
        }
        let a = if i < l1 { d1[i] } else { 0 };
        if a != 0 {
            acc = acc.add_affine(&lookup_p256_aff(g_tab, a));
        }
        let b = if i < l2 { d2[i] } else { 0 };
        if b != 0 {
            if b.unsigned_abs() == 1 {
                acc = acc.add_affine(&lookup_p256_aff(&[q_aff], b));
            } else {
                acc = acc.add_jac(&lookup_p256_jac(&q_tab, b));
            }
        }
    }

    if acc.is_identity() {
        return false;
    }

    let r_u256 = U256::from_biguint(sig_r);
    let r_fe = P256FieldElement::from_canonical(&r_u256);
    let z2 = acc.z.sqr();
    if acc.x == r_fe.mul(&z2) {
        return true;
    }
    if bool::from(r_u256.ct_lt(&P256_P_MINUS_N)) {
        let (rn, _) = U256::adc(&r_u256, &P256_N);
        let rn_fe = P256FieldElement::from_canonical(&rn);
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
    use crate::ecc::point::Point;
    use num_bigint::BigUint;

    fn p256() -> CurveParams {
        CurveParams::p256()
    }

    fn proj_eq_textbook(pp: &P256ProjectivePoint, p: &Point) -> bool {
        match (pp.to_affine(), p) {
            (None, Point::Infinity) => true,
            (Some((x, y)), Point::Affine { x: xf, y: yf }) => {
                x.to_biguint() == xf.value && y.to_biguint() == yf.value
            }
            _ => false,
        }
    }

    #[test]
    fn a_constant_is_correct() {
        // -3 mod p == p - 3
        let p_minus_3 = &p256().p - 3u8;
        let want = P256FieldElement::from_biguint(&p_minus_3);
        assert_eq!(A_FE, want);
    }

    #[test]
    fn b_constant_is_correct() {
        let want = P256FieldElement::from_biguint(&p256().b);
        assert_eq!(B_FE, want);
    }

    #[test]
    fn b3_constant_is_correct() {
        let want = P256FieldElement::from_biguint(&((&p256().b * 3u8) % &p256().p));
        assert_eq!(B3_FE, want);
    }

    #[test]
    fn from_affine_roundtrips() {
        let g = p256().generator();
        let (gx, gy) = match &g {
            Point::Affine { x, y } => (x.value.clone(), y.value.clone()),
            Point::Infinity => panic!(),
        };
        let pp =
            P256ProjectivePoint::from_affine(&U256::from_biguint(&gx), &U256::from_biguint(&gy));
        let (xb, yb) = pp.to_affine().unwrap();
        assert_eq!(xb.to_biguint(), gx);
        assert_eq!(yb.to_biguint(), gy);
    }

    #[test]
    fn add_matches_textbook_for_distinct_points() {
        let curve = p256();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2 = g.double(&a);
        let g3 = g.add(&g2, &a);

        let pg = P256ProjectivePoint::from_textbook(&g);
        let pg2 = P256ProjectivePoint::from_textbook(&g2);
        assert!(proj_eq_textbook(&pg.add(&pg2), &g3));
    }

    #[test]
    fn add_matches_textbook_for_doubling_via_add() {
        let curve = p256();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2 = g.double(&a);

        let pg = P256ProjectivePoint::from_textbook(&g);
        assert!(proj_eq_textbook(&pg.add(&pg), &g2));
    }

    #[test]
    fn double_matches_textbook() {
        let curve = p256();
        let g = curve.generator();
        let a = curve.a_fe();
        let g2 = g.double(&a);

        let pg = P256ProjectivePoint::from_textbook(&g);
        assert!(proj_eq_textbook(&pg.double(), &g2));
    }

    #[test]
    fn add_with_identity_law() {
        let g = p256().generator();
        let pg = P256ProjectivePoint::from_textbook(&g);
        assert!(proj_eq_textbook(
            &pg.add(&P256ProjectivePoint::IDENTITY),
            &g
        ));
        assert!(proj_eq_textbook(
            &P256ProjectivePoint::IDENTITY.add(&pg),
            &g
        ));
    }

    #[test]
    fn add_with_neg_yields_identity() {
        let g = p256().generator();
        let pg = P256ProjectivePoint::from_textbook(&g);
        let sum = pg.add(&pg.neg());
        assert!(bool::from(sum.is_identity()));
    }

    #[test]
    fn scalar_mul_ct_matches_textbook_full_size_scalars() {
        let curve = p256();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let pg = P256ProjectivePoint::from_textbook(&g);
        let bits = curve.order_bits();
        for k_hex in [
            "1",
            "2",
            "deadbeef",
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550",
            "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        ] {
            let k = BigUint::parse_bytes(k_hex.as_bytes(), 16).unwrap();
            let want = g.scalar_mul_ct(&k, &a_fe, bits);
            let got = pg.scalar_mul_ct(&k, bits);
            assert!(proj_eq_textbook(&got, &want), "mismatch at k={}", k_hex);
        }
    }

    #[test]
    fn scalar_mul_by_order_is_identity() {
        let curve = p256();
        let g = curve.generator();
        let pg = P256ProjectivePoint::from_textbook(&g);
        let result = pg.scalar_mul_ct(&curve.n, curve.order_bits());
        assert!(bool::from(result.is_identity()));
    }

    #[test]
    fn external_kat_matches_via_projective() {
        // Derive Q = d·G for the P-256 KAT private key used in
        // ecdsa.rs / ecdh.rs cross-checks.  Then verify the
        // projective ladder lands on the same (x, y) as the
        // textbook ladder.
        let curve = p256();
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let d_hex = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
        let d = BigUint::parse_bytes(d_hex.as_bytes(), 16).unwrap();
        let want = g.scalar_mul_ct(&d, &a_fe, curve.order_bits());

        let pg = P256ProjectivePoint::from_textbook(&g);
        let got = pg.scalar_mul_ct(&d, curve.order_bits());
        assert!(proj_eq_textbook(&got, &want));
    }
}
