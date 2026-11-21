//! Constant-time field arithmetic over P-256's prime field F_p.
//!
//! `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`
//!
//! Mirrors [`crate::ecc::secp256k1_field`]: every value is held in
//! Montgomery form `a·R mod p` where `R = 2^256`, and arithmetic
//! delegates to the constant-time [`crate::ct_bignum::U256`]
//! primitives.  All Montgomery constants are computed at compile time
//! via [`crate::ct_bignum::compute_minv64`] and the `from_montgomery_*`
//! constructors, and are cross-checked against `num-bigint` reference
//! computations in this module's tests.
//!
//! See [`crate::ecc::secp256k1_field`] for the design rationale and a
//! more thorough writeup of the Montgomery-form invariants.

use crate::ct_bignum::{compute_minv64, mul_wide_4, sqr_wide_4, Uint, U256};
use num_bigint::BigUint;
use subtle::{Choice, ConstantTimeEq};

/// Zero-multiplication constant-time Montgomery reduction `T · 2^(-256) mod p`
/// exploiting NIST P-256's Solinas structure `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`:
/// - `p[0] = 2^64 - 1  ⇒  -p[0]^(-1) ≡ 1 (mod 2^64)`, so `m_i = s_i` (0 multiplies)
/// - `carry(s_i + m_i·p[0]) + m_i·p[1] = m_i + m_i·(2^32 - 1) = m_i << 32` (1 shift)
/// - `p[2] = 0` (0 ops)
/// - `m_i·p[3] = (m_i << 64) - (m_i << 32) + m_i` (shifts + add/sub only)
#[inline(always)]
fn p256_mont_reduce_wide(t: &[u64; 8]) -> U256 {
    const MASK64: u128 = u64::MAX as u128;
    let s0 = t[0];
    let s1 = t[1] as u128;
    let mut s2 = t[2] as u128;
    let mut s3 = t[3] as u128;
    let mut s4 = t[4] as u128;
    let mut s5 = t[5] as u128;
    let mut s6 = t[6] as u128;
    let mut s7 = t[7] as u128;

    // Step 0 (m = s0)
    let m = s0 as u128;
    let mp1 = m << 32;
    let mp3 = (m << 64) - mp1 + m;
    let acc1 = s1 + (mp1 & MASK64);
    let m1 = acc1 as u64;
    let acc2 = s2 + (mp1 >> 64) + (acc1 >> 64);
    s2 = acc2 & MASK64;
    let acc3 = s3 + (mp3 & MASK64) + (acc2 >> 64);
    s3 = acc3 & MASK64;
    let acc4 = s4 + (mp3 >> 64) + (acc3 >> 64);
    s4 = acc4 & MASK64;
    s5 += acc4 >> 64;

    // Step 1 (m = m1)
    let m = m1 as u128;
    let mp1 = m << 32;
    let mp3 = (m << 64) - mp1 + m;
    let acc2 = s2 + (mp1 & MASK64);
    let m2 = acc2 as u64;
    let acc3 = s3 + (mp1 >> 64) + (acc2 >> 64);
    s3 = acc3 & MASK64;
    let acc4 = s4 + (mp3 & MASK64) + (acc3 >> 64);
    s4 = acc4 & MASK64;
    let acc5 = s5 + (mp3 >> 64) + (acc4 >> 64);
    s5 = acc5 & MASK64;
    s6 += acc5 >> 64;

    // Step 2 (m = m2)
    let m = m2 as u128;
    let mp1 = m << 32;
    let mp3 = (m << 64) - mp1 + m;
    let acc3 = s3 + (mp1 & MASK64);
    let m3 = acc3 as u64;
    let acc4 = s4 + (mp1 >> 64) + (acc3 >> 64);
    s4 = acc4 & MASK64;
    let acc5 = s5 + (mp3 & MASK64) + (acc4 >> 64);
    s5 = acc5 & MASK64;
    let acc6 = s6 + (mp3 >> 64) + (acc5 >> 64);
    s6 = acc6 & MASK64;
    s7 += acc6 >> 64;

    // Step 3 (m = m3)
    let m = m3 as u128;
    let mp1 = m << 32;
    let mp3 = (m << 64) - mp1 + m;
    let acc4 = s4 + (mp1 & MASK64);
    let r0 = acc4 as u64;
    let acc5 = s5 + (mp1 >> 64) + (acc4 >> 64);
    let r1 = acc5 as u64;
    let acc6 = s6 + (mp3 & MASK64) + (acc5 >> 64);
    let r2 = acc6 as u64;
    let acc7 = s7 + (mp3 >> 64) + (acc6 >> 64);
    let r3 = acc7 as u64;
    let r4 = (acc7 >> 64) as u64;

    // Constant-time conditional subtraction of P
    let (d0, b1) = r0.overflowing_sub(P.0[0]);
    let (d1, b2) = r1.overflowing_sub(P.0[1]);
    let (d1, b2b) = d1.overflowing_sub(b1 as u64);
    let br = (b2 | b2b) as u64;
    let (d2, b3) = r2.overflowing_sub(br);
    let br = b3 as u64;
    let (d3, b4) = r3.overflowing_sub(P.0[3]);
    let (d3, b4b) = d3.overflowing_sub(br);
    let borrow = (b4 | b4b) as u64;

    let need_sub = (r4 | (borrow ^ 1)) & 1;
    let mask = core::hint::black_box(0u64.wrapping_sub(need_sub));
    Uint([
        r0 ^ (mask & (r0 ^ d0)),
        r1 ^ (mask & (r1 ^ d1)),
        r2 ^ (mask & (r2 ^ d2)),
        r3 ^ (mask & (r3 ^ d3)),
    ])
}

/// P-256 prime modulus, as little-endian limbs.
///
/// `p = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF`
/// `  = 2^256 - 2^224 + 2^192 + 2^96 - 1`
pub const P: U256 = Uint([
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_0000_FFFF_FFFF,
    0x0000_0000_0000_0000,
    0xFFFF_FFFF_0000_0001,
]);

/// Montgomery constant `m' = -p^(-1) mod 2^64`.
pub const P_INV_LOW: u64 = compute_minv64(P.0[0]);

/// `R mod p`, where `R = 2^256`.  Equal to "1" in Montgomery form.
pub const R_MOD_P: U256 = Uint([
    0x0000_0000_0000_0001,
    0xFFFF_FFFF_0000_0000,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_0000_FFFF_FFFE,
]);

/// `R^2 mod p`.  Used to convert canonical values into Montgomery form
/// via `mont_mul(a, R^2 mod p)`.
pub const R_SQUARED_MOD_P: U256 = Uint([
    0x0000_0000_0000_0003,
    0xFFFF_FFFB_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFE,
    0x0000_0004_FFFF_FFFD,
]);

/// A field element over P-256's prime, stored in Montgomery form.
#[derive(Copy, Clone, Debug)]
pub struct P256FieldElement(U256);

impl P256FieldElement {
    pub const ZERO: P256FieldElement = P256FieldElement(U256::ZERO);
    pub const ONE: P256FieldElement = P256FieldElement(R_MOD_P);

    pub fn from_canonical(a: &U256) -> Self {
        P256FieldElement(U256::mont_mul(a, &R_SQUARED_MOD_P, &P, P_INV_LOW))
    }

    pub fn to_canonical(&self) -> U256 {
        U256::mont_mul(&self.0, &U256::ONE, &P, P_INV_LOW)
    }

    pub fn from_biguint(v: &BigUint) -> Self {
        let p_bu = P.to_biguint();
        let reduced = U256::from_biguint(&(v % &p_bu));
        Self::from_canonical(&reduced)
    }

    pub fn to_biguint(&self) -> BigUint {
        self.to_canonical().to_biguint()
    }

    pub fn to_bytes_be(&self) -> [u8; 32] {
        self.to_canonical().to_bytes_be()
    }

    pub fn from_bytes_be(bytes: &[u8; 32]) -> Self {
        Self::from_canonical(&U256::from_bytes_be(bytes))
    }

    pub fn add(&self, other: &Self) -> Self {
        P256FieldElement(self.0.add_mod(&other.0, &P))
    }

    pub fn sub(&self, other: &Self) -> Self {
        P256FieldElement(self.0.sub_mod(&other.0, &P))
    }

    #[inline]
    pub fn mul(&self, other: &Self) -> Self {
        P256FieldElement(p256_mont_reduce_wide(&mul_wide_4(&self.0 .0, &other.0 .0)))
    }

    #[inline]
    pub fn sqr(&self) -> Self {
        P256FieldElement(p256_mont_reduce_wide(&sqr_wide_4(&self.0 .0)))
    }

    pub fn neg(&self) -> Self {
        P256FieldElement(U256::ZERO.sub_mod(&self.0, &P))
    }

    #[inline]
    fn sqr_n(&self, n: usize) -> Self {
        let mut r = *self;
        for _ in 0..n {
            r = r.sqr();
        }
        r
    }

    /// Multiplicative inverse via Fermat: `a^(p-2) mod p` using a fixed
    /// constant-time addition chain (255 squarings + 12 multiplications,
    /// zero secret-dependent branches; maps `0 → 0`).
    pub fn inv(&self) -> Self {
        let f1 = *self;
        let f2 = f1.sqr().mul(&f1);
        let f4 = f2.sqr_n(2).mul(&f2);
        let f8 = f4.sqr_n(4).mul(&f4);
        let f16 = f8.sqr_n(8).mul(&f8);
        let f32 = f16.sqr_n(16).mul(&f16);
        let acc = f32.sqr_n(32).mul(&f1);
        let acc = acc.sqr_n(128).mul(&f32);
        let acc = acc.sqr_n(32).mul(&f32);
        let acc = acc.sqr_n(16).mul(&f16);
        let acc = acc.sqr_n(8).mul(&f8);
        let acc = acc.sqr_n(4).mul(&f4);
        let acc = acc.sqr_n(2).mul(&f2);
        acc.sqr_n(2).mul(&f1)
    }

    pub fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq_full(&other.0)
    }

    pub fn ct_is_zero(&self) -> Choice {
        self.0.ct_is_zero()
    }

    pub fn cmov(a: &Self, b: &Self, choice: Choice) -> Self {
        P256FieldElement(U256::cmov(&a.0, &b.0, choice))
    }

    #[inline]
    pub fn cswap(a: &mut Self, b: &mut Self, choice: Choice) {
        U256::cswap(&mut a.0, &mut b.0, choice);
    }

    #[inline]
    pub fn as_montgomery(&self) -> &U256 {
        &self.0
    }

    pub const fn from_montgomery_unchecked(limbs: U256) -> Self {
        P256FieldElement(limbs)
    }
}

impl PartialEq for P256FieldElement {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}

impl Eq for P256FieldElement {}

impl ConstantTimeEq for P256FieldElement {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.ct_eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    fn p_bu() -> BigUint {
        P.to_biguint()
    }

    fn samples() -> Vec<BigUint> {
        let p = p_bu();
        vec![
            BigUint::zero(),
            BigUint::one(),
            BigUint::from(2u8),
            BigUint::from(0xdeadbeefu64),
            BigUint::from(1u8) << 32,
            BigUint::from(1u8) << 96, // exercises the limb boundary in P-256's prime
            BigUint::from(1u8) << 192,
            &p - 1u8,
            &p - 2u8,
            &p / 2u8,
            (&p / 2u8) + 1u8,
            BigUint::parse_bytes(
                b"DEADBEEFCAFEBABE0123456789ABCDEF112233445566778899AABBCCDDEEFF00",
                16,
            )
            .unwrap(),
            BigUint::parse_bytes(
                b"FEDCBA9876543210FEDCBA9876543210FEDCBA9876543210FEDCBA9876543210",
                16,
            )
            .unwrap(),
        ]
    }

    #[test]
    fn p_constant_matches_curve_params() {
        let want = U256::from_biguint(&crate::ecc::CurveParams::p256().p);
        assert!(bool::from(P.ct_eq_full(&want)));
    }

    #[test]
    fn r_mod_p_constant_is_correct() {
        let r = BigUint::one() << 256;
        let want = U256::from_biguint(&(&r % p_bu()));
        assert!(bool::from(R_MOD_P.ct_eq_full(&want)));
    }

    #[test]
    fn r_squared_constant_is_correct() {
        let r = BigUint::one() << 256;
        let want = U256::from_biguint(&((&r * &r) % p_bu()));
        assert!(bool::from(R_SQUARED_MOD_P.ct_eq_full(&want)));
    }

    #[test]
    fn p_inv_low_consistent_with_compute_minv64() {
        let prod = P.0[0].wrapping_mul(P_INV_LOW);
        assert_eq!(prod.wrapping_add(1), 0);
    }

    #[test]
    fn from_to_canonical_roundtrips() {
        for v in samples() {
            let fe = P256FieldElement::from_biguint(&v);
            assert_eq!(fe.to_biguint(), v);
        }
    }

    #[test]
    fn add_matches_biguint() {
        let p = p_bu();
        for a in samples() {
            for b in samples() {
                let fa = P256FieldElement::from_biguint(&a);
                let fb = P256FieldElement::from_biguint(&b);
                assert_eq!(fa.add(&fb).to_biguint(), (&a + &b) % &p);
            }
        }
    }

    #[test]
    fn sub_matches_biguint() {
        let p = p_bu();
        for a in samples() {
            for b in samples() {
                let fa = P256FieldElement::from_biguint(&a);
                let fb = P256FieldElement::from_biguint(&b);
                let want = (&p + (&a % &p) - (&b % &p)) % &p;
                assert_eq!(fa.sub(&fb).to_biguint(), want);
            }
        }
    }

    #[test]
    fn mul_matches_biguint() {
        let p = p_bu();
        for a in samples() {
            for b in samples() {
                let fa = P256FieldElement::from_biguint(&a);
                let fb = P256FieldElement::from_biguint(&b);
                assert_eq!(fa.mul(&fb).to_biguint(), (&a * &b) % &p);
            }
        }
    }

    #[test]
    fn sqr_matches_mul_self() {
        let p = p_bu();
        for a in samples() {
            let fa = P256FieldElement::from_biguint(&a);
            assert_eq!(fa.sqr().to_biguint(), (&a * &a) % &p);
            assert_eq!(fa.sqr(), fa.mul(&fa));
        }
    }

    #[test]
    fn neg_matches_biguint() {
        let p = p_bu();
        for a in samples() {
            let fa = P256FieldElement::from_biguint(&a);
            let want = (&p - (&a % &p)) % &p;
            assert_eq!(fa.neg().to_biguint(), want);
        }
    }

    #[test]
    fn inv_times_self_is_one() {
        for a in samples() {
            if a.is_zero() {
                continue;
            }
            let fa = P256FieldElement::from_biguint(&a);
            assert_eq!(fa.mul(&fa.inv()).to_biguint(), BigUint::one());
        }
    }
}
