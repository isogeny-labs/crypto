//! Koblitz curves `K_a: y² + xy = x³ + a x² + 1` over `GF(2^n)` for
//! `128 ≤ n ≤ 191`, past the `u128` words of
//! [`WideKoblitz`](crate::cryptanalysis::koblitz_wide::WideKoblitz): the
//! ECC2K-130 field (`n = 131`, `a = 0`) and K-163 (`n = 163`, `a = 1`) are
//! the two curves in that range whose order is a small cofactor times a
//! prime.
//!
//! The field is the repository's three-word
//! [`Gf2W`](crate::cryptanalysis::certicom131_symlp::gf2w::Gf2W); this module
//! adds the two types the width-generic strong rho
//! ([`StrongRhoG`](crate::cryptanalysis::koblitz_strong_rho::StrongRhoG)) and
//! the claw need on top of it:
//!
//! - [`W192`], a field word: three `u64` limbs compared, shifted and
//!   subtracted as one 192-bit integer, hashed to 64 bits by
//!   [`FieldWord::fold64`] so that a value below `2^128` folds exactly as the
//!   `u128` word does (the 3-word walk is the wide walk wherever both exist);
//! - [`U192`], a scalar modulo a subgroup order below `2^192`: Montgomery
//!   multiplication on three limbs, and a uniform sampler that draws exactly
//!   what `u128`'s sampler draws when the range fits a `u128`.
//!
//! The curve is built by the wide constructor's rule, unchanged: the order
//! from the Koblitz recurrence, `#E(F_2)` (4 for `a = 0`, 2 for `a = 1`)
//! times a prime `r` (Miller–Rabin, 32 fixed bases, then certified by
//! `[r]G = O`), the generator from the first abscissa `x = 1, 2, 3, …` whose
//! `x + a + x^{−2}` has trace zero, `G = [h]P`, and `λ` the root of
//! `λ² − tλ + 2 ≡ 0 (mod r)` with `φ(G) = [λ]G`.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Sub};

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
use rand::Rng;

use crate::binary_ecc::IrreduciblePoly;
use crate::cryptanalysis::certicom131_symlp::gf2w::{self, Gf2W, W3};
use crate::cryptanalysis::curve_id::{self, CurveId};
use crate::cryptanalysis::koblitz_strong_rho::{
    FieldWord, RawPointG, RhoField, RhoScalar, StrongRhoG,
};
use crate::cryptanalysis::koblitz_wide::koblitz_order;

// ---------------------------------------------------------------------------
// The field word.
// ---------------------------------------------------------------------------

/// A `GF(2^n)` element for `n ≤ 191`: bit `i` of the little-endian limb
/// triple is the coefficient of `z^i`.  Ordered, shifted and subtracted as
/// one 192-bit unsigned integer.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct W192(pub W3);

impl W192 {
    pub const ZERO: W192 = W192([0; 3]);

    pub fn from_u128(v: u128) -> Self {
        W192([v as u64, (v >> 64) as u64, 0])
    }

    /// The value, when it fits a `u128`.
    pub fn checked_u128(self) -> Option<u128> {
        (self.0[2] == 0).then(|| u128::from(self.0[0]) | (u128::from(self.0[1]) << 64))
    }

    pub fn from_biguint(v: &BigUint) -> Option<Self> {
        (v.bits() <= 192).then(|| W192(gf2w::from_biguint(v)))
    }

    pub fn to_biguint(self) -> BigUint {
        gf2w::to_biguint(&self.0)
    }

    /// Lower-case hex with a `0x` prefix, as every ecbench coordinate is
    /// written (`format!("0x{v:x}")` of the integer).
    pub fn hex(self) -> String {
        format!("0x{:x}", self.to_biguint())
    }

    /// Parse `0x…` hex of at most 192 bits.
    pub fn parse_hex(s: &str) -> Option<Self> {
        let digits = s.trim_start_matches("0x");
        if digits.is_empty() {
            return None;
        }
        Self::from_biguint(&BigUint::parse_bytes(digits.as_bytes(), 16)?)
    }

    /// `self + 1` (wrapping at `2^192`).
    pub fn plus_one(self) -> Self {
        let mut w = self.0;
        for limb in w.iter_mut() {
            let (v, carry) = limb.overflowing_add(1);
            *limb = v;
            if !carry {
                break;
            }
        }
        W192(w)
    }
}

impl fmt::Debug for W192 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "W192({})", self.hex())
    }
}

impl Ord for W192 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

impl PartialOrd for W192 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BitXor for W192 {
    type Output = W192;
    #[inline(always)]
    fn bitxor(self, o: W192) -> W192 {
        W192(gf2w::xor(self.0, o.0))
    }
}

impl BitAnd for W192 {
    type Output = W192;
    #[inline(always)]
    fn bitand(self, o: W192) -> W192 {
        W192([self.0[0] & o.0[0], self.0[1] & o.0[1], self.0[2] & o.0[2]])
    }
}

impl BitOr for W192 {
    type Output = W192;
    #[inline(always)]
    fn bitor(self, o: W192) -> W192 {
        W192([self.0[0] | o.0[0], self.0[1] | o.0[1], self.0[2] | o.0[2]])
    }
}

impl Sub for W192 {
    type Output = W192;
    /// Wrapping 192-bit subtraction.
    #[inline(always)]
    fn sub(self, o: W192) -> W192 {
        let (d0, b0) = self.0[0].overflowing_sub(o.0[0]);
        let (d1, b1a) = self.0[1].overflowing_sub(o.0[1]);
        let (d1, b1b) = d1.overflowing_sub(u64::from(b0));
        let d2 = self.0[2]
            .wrapping_sub(o.0[2])
            .wrapping_sub(u64::from(b1a || b1b));
        W192([d0, d1, d2])
    }
}

impl Shl<u32> for W192 {
    type Output = W192;
    /// `self << s`, bits past 192 dropped; zero for `s ≥ 192`.
    #[inline(always)]
    fn shl(self, s: u32) -> W192 {
        if s >= 192 {
            return W192::ZERO;
        }
        let words = (s / 64) as usize;
        let bits = s % 64;
        let mut out = [0u64; 3];
        for i in words..3 {
            let lo = self.0[i - words] << bits;
            let hi = if bits != 0 && i > words {
                self.0[i - words - 1] >> (64 - bits)
            } else {
                0
            };
            out[i] = lo | hi;
        }
        W192(out)
    }
}

impl Shr<u32> for W192 {
    type Output = W192;
    /// `self >> s`; zero for `s ≥ 192`.
    #[inline(always)]
    fn shr(self, s: u32) -> W192 {
        if s >= 192 {
            return W192::ZERO;
        }
        let words = (s / 64) as usize;
        let bits = s % 64;
        let mut out = [0u64; 3];
        for (i, o) in out.iter_mut().enumerate().take(3 - words) {
            let lo = self.0[i + words] >> bits;
            let hi = if bits != 0 && i + words + 1 < 3 {
                self.0[i + words + 1] << (64 - bits)
            } else {
                0
            };
            *o = lo | hi;
        }
        W192(out)
    }
}

impl From<u8> for W192 {
    fn from(v: u8) -> Self {
        W192([u64::from(v), 0, 0])
    }
}

impl From<u64> for W192 {
    fn from(v: u64) -> Self {
        W192([v, 0, 0])
    }
}

impl FieldWord for W192 {
    const BITS: u32 = 192;
    /// `w₀ ⊕ rotl(w₁, 29) ⊕ rotl(w₂, 58)`: for a value below `2^128` the
    /// `u128` word's fold, and below `2^64` the value itself.
    #[inline(always)]
    fn fold64(self) -> u64 {
        self.0[0] ^ self.0[1].rotate_left(29) ^ self.0[2].rotate_left(58)
    }
    #[inline(always)]
    fn byte(self, i: u32) -> usize {
        if i >= 24 {
            return 0;
        }
        ((self.0[(i / 8) as usize] >> (8 * (i % 8))) & 0xff) as usize
    }
    #[inline(always)]
    fn bit(self, i: u32) -> bool {
        i < 192 && (self.0[(i / 64) as usize] >> (i % 64)) & 1 == 1
    }
    #[inline(always)]
    fn trailing_zeros(self) -> u32 {
        if self.0[0] != 0 {
            self.0[0].trailing_zeros()
        } else if self.0[1] != 0 {
            64 + self.0[1].trailing_zeros()
        } else if self.0[2] != 0 {
            128 + self.0[2].trailing_zeros()
        } else {
            192
        }
    }
}

impl RhoField for Gf2W {
    type E = W192;
    #[inline(always)]
    fn degree(&self) -> u32 {
        self.n
    }
    #[inline(always)]
    fn mul(&self, a: W192, b: W192) -> W192 {
        W192(Gf2W::mul(self, &a.0, &b.0))
    }
    #[inline(always)]
    fn sqr(&self, a: W192) -> W192 {
        W192(Gf2W::sqr(self, &a.0))
    }
    /// Montgomery's trick with one inversion, zeros left zero: the same
    /// values as [`Gf2W::batch_inv`] (inverses are unique), on the word type.
    fn batch_inv(&self, xs: &mut [W192], scratch: &mut Vec<W192>) {
        scratch.clear();
        let mut acc = gf2w::ONE;
        for x in xs.iter() {
            scratch.push(W192(acc));
            if !gf2w::is_zero(&x.0) {
                acc = Gf2W::mul(self, &acc, &x.0);
            }
        }
        let mut inv = self.inv(&acc);
        for i in (0..xs.len()).rev() {
            let x = xs[i].0;
            if gf2w::is_zero(&x) {
                continue;
            }
            xs[i] = W192(Gf2W::mul(self, &inv, &scratch[i].0));
            inv = Gf2W::mul(self, &inv, &x);
        }
    }
}

// ---------------------------------------------------------------------------
// The scalar.
// ---------------------------------------------------------------------------

/// An unsigned integer below `2^192`: a scalar modulo a subgroup order past
/// `2^128`.  Little-endian limbs.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct U192(pub [u64; 3]);

impl U192 {
    pub fn from_u128(v: u128) -> Self {
        U192([v as u64, (v >> 64) as u64, 0])
    }

    pub fn checked_u128(self) -> Option<u128> {
        (self.0[2] == 0).then(|| u128::from(self.0[0]) | (u128::from(self.0[1]) << 64))
    }

    pub fn to_big(self) -> BigUint {
        gf2w::to_biguint(&self.0)
    }

    pub fn from_big(v: &BigUint) -> Option<Self> {
        (v.bits() <= 192).then(|| U192(gf2w::from_biguint(v)))
    }

    fn is_zero(&self) -> bool {
        self.0 == [0; 3]
    }

    /// `(self + o, carry)`.
    #[inline(always)]
    fn add_carry(self, o: U192) -> (U192, bool) {
        let (s0, c0) = self.0[0].overflowing_add(o.0[0]);
        let (s1, c1a) = self.0[1].overflowing_add(o.0[1]);
        let (s1, c1b) = s1.overflowing_add(u64::from(c0));
        let (s2, c2a) = self.0[2].overflowing_add(o.0[2]);
        let (s2, c2b) = s2.overflowing_add(u64::from(c1a || c1b));
        (U192([s0, s1, s2]), c2a || c2b)
    }

    /// `(self − o, borrow)`.
    #[inline(always)]
    fn sub_borrow(self, o: U192) -> (U192, bool) {
        let (d0, b0) = self.0[0].overflowing_sub(o.0[0]);
        let (d1, b1a) = self.0[1].overflowing_sub(o.0[1]);
        let (d1, b1b) = d1.overflowing_sub(u64::from(b0));
        let (d2, b2a) = self.0[2].overflowing_sub(o.0[2]);
        let (d2, b2b) = d2.overflowing_sub(u64::from(b1a || b1b));
        (U192([d0, d1, d2]), b2a || b2b)
    }
}

impl fmt::Display for U192 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_big().fmt(f)
    }
}

impl fmt::Debug for U192 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U192({self})")
    }
}

impl Ord for U192 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

impl PartialOrd for U192 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Montgomery arithmetic modulo an odd `m < 2^192` with `R = 2^192`.
#[derive(Clone, Copy, Debug)]
pub struct Mont192 {
    m: [u64; 3],
    /// `−m^{−1} mod 2^64`.
    minv: u64,
    /// `R² mod m`.
    r2: [u64; 3],
}

impl Mont192 {
    pub fn new(m: U192) -> Self {
        assert!(
            m > U192::from_u128(1) && m.0[0] & 1 == 1,
            "Montgomery arithmetic needs an odd modulus above 1"
        );
        // Newton's iteration for m₀^{−1} mod 2^64 (correct bits double).
        let m0 = m.0[0];
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(m0.wrapping_mul(inv)));
        }
        let r2 = (BigUint::one() << 384usize) % m.to_big();
        Self {
            m: m.0,
            minv: inv.wrapping_neg(),
            r2: gf2w::from_biguint(&r2),
        }
    }

    /// `a·b·R^{−1} mod m` for `a, b < m` (CIOS).
    #[inline(always)]
    fn mont(&self, a: &[u64; 3], b: &[u64; 3]) -> [u64; 3] {
        let m = &self.m;
        let mut t = [0u64; 5];
        for &bi in b.iter() {
            let mut c = 0u64;
            for j in 0..3 {
                let s = u128::from(t[j]) + u128::from(a[j]) * u128::from(bi) + u128::from(c);
                t[j] = s as u64;
                c = (s >> 64) as u64;
            }
            let s = u128::from(t[3]) + u128::from(c);
            t[3] = s as u64;
            t[4] = (s >> 64) as u64;
            let q = t[0].wrapping_mul(self.minv);
            let s = u128::from(t[0]) + u128::from(q) * u128::from(m[0]);
            let mut c = (s >> 64) as u64;
            for j in 1..3 {
                let s = u128::from(t[j]) + u128::from(q) * u128::from(m[j]) + u128::from(c);
                t[j - 1] = s as u64;
                c = (s >> 64) as u64;
            }
            let s = u128::from(t[3]) + u128::from(c);
            t[2] = s as u64;
            t[3] = t[4] + (s >> 64) as u64;
            t[4] = 0;
        }
        // t < 2m: one conditional subtraction.
        let v = U192([t[0], t[1], t[2]]);
        let (d, borrow) = v.sub_borrow(U192(*m));
        if t[3] != 0 || !borrow {
            d.0
        } else {
            v.0
        }
    }

    #[inline(always)]
    fn mul(&self, a: U192, b: U192) -> U192 {
        U192(self.mont(&self.mont(&a.0, &b.0), &self.r2))
    }

    /// `w·R mod m`, the Montgomery form of `w`.
    #[inline(always)]
    fn mont_form(&self, w: U192) -> [u64; 3] {
        self.mont(&w.0, &self.r2)
    }
}

impl RhoScalar for U192 {
    type MulCtx = Mont192;
    fn zero() -> Self {
        U192([0; 3])
    }
    fn one() -> Self {
        U192([1, 0, 0])
    }
    fn from_biguint(v: &BigUint) -> Option<Self> {
        U192::from_big(v)
    }
    fn to_f64(self) -> f64 {
        match self.checked_u128() {
            Some(v) => v as f64,
            None => self.to_big().to_f64().expect("a 192-bit value is finite"),
        }
    }
    fn checked_u128(self) -> Option<u128> {
        U192::checked_u128(self)
    }
    fn to_biguint(self) -> BigUint {
        self.to_big()
    }
    fn bit_len(self) -> u32 {
        for i in (0..3).rev() {
            if self.0[i] != 0 {
                return 64 * i as u32 + 64 - self.0[i].leading_zeros();
            }
        }
        0
    }
    fn bit(self, i: u32) -> bool {
        i < 192 && (self.0[(i / 64) as usize] >> (i % 64)) & 1 == 1
    }
    fn mul_ctx(m: Self) -> Mont192 {
        Mont192::new(m)
    }
    #[inline(always)]
    fn mul_in(ctx: &Mont192, a: Self, b: Self) -> Self {
        ctx.mul(a, b)
    }
    /// The multiplier in Montgomery form: one Montgomery product then gives
    /// `a·w mod m`.
    type Fixed = [u64; 3];
    fn fixed(ctx: &Mont192, w: Self) -> [u64; 3] {
        ctx.mont_form(w)
    }
    #[inline(always)]
    fn mul_fixed(ctx: &Mont192, a: Self, w: [u64; 3]) -> Self {
        U192(ctx.mont(&a.0, &w))
    }
    #[inline(always)]
    fn mul_pair_in(ctx: &Mont192, a: Self, b: Self, multiplier: Self) -> (Self, Self) {
        let w = ctx.mont_form(multiplier);
        (U192(ctx.mont(&a.0, &w)), U192(ctx.mont(&b.0, &w)))
    }
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        let m_big = m.to_big();
        let v = ((a.to_big() % &m_big) * (b.to_big() % &m_big)) % &m_big;
        U192::from_big(&v).expect("a residue is below the modulus")
    }
    #[inline(always)]
    fn add_mod(a: Self, b: Self, m: Self) -> Self {
        if a >= m || b >= m {
            let m_big = m.to_big();
            let v = (a.to_big() + b.to_big()) % &m_big;
            return U192::from_big(&v).expect("a residue is below the modulus");
        }
        let (s, carry) = a.add_carry(b);
        if carry || s >= m {
            s.sub_borrow(m).0
        } else {
            s
        }
    }
    fn sub_mod(a: Self, b: Self, m: Self) -> Self {
        if a >= b {
            a.sub_borrow(b).0
        } else {
            m.sub_borrow(b.sub_borrow(a).0).0
        }
    }
    fn inverse_mod(v: Self, m: Self) -> Option<Self> {
        if v.is_zero() {
            return None;
        }
        let e = BigInt::from(v.to_big()).extended_gcd(&BigInt::from(m.to_big()));
        if e.gcd != BigInt::one() {
            return None;
        }
        let x = e.x.mod_floor(&BigInt::from(m.to_big()));
        U192::from_big(&x.to_biguint()?)
    }
}

/// The uniform sampler: `u128`'s own sampler while the range fits a `u128`
/// (so a draw is bit for bit what a `u128` scalar draws from the same RNG),
/// rejection on the range's bit length past it.
#[derive(Clone, Copy, Debug)]
pub enum UniformU192 {
    Narrow(UniformInt<u128>),
    Wide { low: U192, range: U192 },
}

/// A uniform draw in `[low, low + range)` for `range > 0` by rejection.
fn sample_wide<R: Rng + ?Sized>(low: U192, range: U192, rng: &mut R) -> U192 {
    let bits = range.bit_len();
    loop {
        let mut v = [0u64; 3];
        for (i, w) in v.iter_mut().enumerate() {
            let lo = 64 * i as u32;
            if lo >= bits {
                break;
            }
            let x = rng.next_u64();
            *w = if bits - lo >= 64 {
                x
            } else {
                x & ((1u64 << (bits - lo)) - 1)
            };
        }
        let v = U192(v);
        if v < range {
            return low.add_carry(v).0;
        }
    }
}

impl UniformSampler for UniformU192 {
    type X = U192;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<U192> + Sized,
        B2: SampleBorrow<U192> + Sized,
    {
        let (low, high) = (*low.borrow(), *high.borrow());
        assert!(low < high, "UniformU192::new called with `low >= high`");
        match (low.checked_u128(), high.checked_u128()) {
            (Some(l), Some(h)) => UniformU192::Narrow(UniformInt::<u128>::new(l, h)),
            _ => UniformU192::Wide {
                low,
                range: high.sub_borrow(low).0,
            },
        }
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<U192> + Sized,
        B2: SampleBorrow<U192> + Sized,
    {
        let (low, high) = (*low.borrow(), *high.borrow());
        assert!(
            low <= high,
            "UniformU192::new_inclusive called with `low > high`"
        );
        match (low.checked_u128(), high.checked_u128()) {
            (Some(l), Some(h)) => UniformU192::Narrow(UniformInt::<u128>::new_inclusive(l, h)),
            _ => {
                let (range, _) = high.sub_borrow(low).0.add_carry(U192::one());
                UniformU192::Wide { low, range }
            }
        }
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> U192 {
        match self {
            UniformU192::Narrow(u) => U192::from_u128(u.sample(rng)),
            UniformU192::Wide { low, range } => sample_wide(*low, *range, rng),
        }
    }
    fn sample_single<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> U192
    where
        B1: SampleBorrow<U192> + Sized,
        B2: SampleBorrow<U192> + Sized,
    {
        let (low, high) = (*low.borrow(), *high.borrow());
        assert!(low < high, "UniformU192::sample_single: low >= high");
        match (low.checked_u128(), high.checked_u128()) {
            (Some(l), Some(h)) => U192::from_u128(
                <UniformInt<u128> as UniformSampler>::sample_single(l, h, rng),
            ),
            _ => sample_wide(low, high.sub_borrow(low).0, rng),
        }
    }
}

impl SampleUniform for U192 {
    type Sampler = UniformU192;
}

// ---------------------------------------------------------------------------
// The curve.
// ---------------------------------------------------------------------------

/// The strong reference (and every big method's group arithmetic) on a
/// three-word Koblitz curve.
pub type BigStrongRho = StrongRhoG<Gf2W, U192>;

/// The field degrees this class covers in ecbench.
pub const BIG_DEGREES: std::ops::RangeInclusive<u32> = 128..=191;

/// A Koblitz curve over `GF(2^n)`, `n ≤ 191`, with its prime subgroup.
#[derive(Clone, Debug)]
pub struct BigKoblitz {
    pub a: u8,
    pub n: u32,
    pub irreducible: IrreduciblePoly,
    /// Frobenius trace over `F_2`: `−1` for `a = 0`, `+1` for `a = 1`.
    pub trace: i64,
    pub group_order: BigUint,
    pub r: BigUint,
    pub cofactor: u64,
    /// `φ(Q) = [λ]Q` on `⟨G⟩`.
    pub lambda: BigUint,
    pub generator: (W192, W192),
    /// The `x` the generator rule stopped at (its `P` gave `G = [h]P`).
    pub generator_seed_x: u64,
}

const MR_BASES: [u32; 32] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131,
];

/// Miller–Rabin with the first 32 primes as bases, the wide constructor's
/// rule on a `BigUint` (the order is certified again by `[r]G = O`).
pub fn is_probable_prime(n: &BigUint) -> bool {
    let two = BigUint::from(2u8);
    if *n < two {
        return false;
    }
    for p in MR_BASES {
        let p = BigUint::from(p);
        if *n == p {
            return true;
        }
        if (n % &p).is_zero() {
            return false;
        }
    }
    let n1 = n - 1u8;
    let s = n1.trailing_zeros().expect("n − 1 > 0");
    let d = &n1 >> s;
    'base: for a in MR_BASES {
        let mut x = BigUint::from(a).modpow(&d, n);
        if x.is_one() || x == n1 {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n1 {
                continue 'base;
            }
        }
        return false;
    }
    true
}

/// A square root of `v` modulo the odd prime `p` (Tonelli–Shanks), if any.
fn sqrt_mod(v: &BigUint, p: &BigUint) -> Option<BigUint> {
    let v = v % p;
    if v.is_zero() {
        return Some(v);
    }
    let one = BigUint::one();
    let p1 = p - 1u8;
    let half = &p1 >> 1usize;
    if v.modpow(&half, p) != one {
        return None;
    }
    let s = p1.trailing_zeros().expect("p − 1 > 0");
    let q = &p1 >> s;
    let mut z = BigUint::from(2u8);
    while z.modpow(&half, p) != p1 {
        z += 1u8;
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = v.modpow(&q, p);
    let mut r = v.modpow(&((&q + 1u8) >> 1usize), p);
    while !t.is_one() {
        let mut i = 0u64;
        let mut t2 = t.clone();
        while !t2.is_one() {
            t2 = (&t2 * &t2) % p;
            i += 1;
        }
        let b = c.modpow(&(BigUint::one() << (m - i - 1)), p);
        m = i;
        c = (&b * &b) % p;
        t = (&t * &c) % p;
        r = (&r * &b) % p;
    }
    Some(r)
}

impl BigKoblitz {
    /// Build `K_a` over the field of `irr` for `128 ≤ n ≤ 191`, the range
    /// ecbench gives this class.
    pub fn new(a: u8, irr: &IrreduciblePoly) -> Result<Self, String> {
        if !BIG_DEGREES.contains(&irr.degree) {
            return Err(format!(
                "a three-word Koblitz curve takes 128 ≤ n ≤ 191, not n={}",
                irr.degree
            ));
        }
        Self::for_any_degree(a, irr)
    }

    /// The same constructor at any degree `3 ≤ n ≤ 191`: what the tests use
    /// to put the three-word arithmetic beside the one-word and `u128`
    /// arithmetic on one curve.
    pub fn for_any_degree(a: u8, irr: &IrreduciblePoly) -> Result<Self, String> {
        let n = irr.degree;
        if a > 1 || !(3..=191).contains(&n) {
            return Err(format!(
                "a Koblitz curve takes a ∈ {{0,1}} and n ≤ 191, not a={a} n={n}"
            ));
        }
        let field = Gf2W::new(irr);
        let order = koblitz_order(a, n);
        let cofactor: u64 = if a == 0 { 4 } else { 2 };
        let h = BigUint::from(cofactor);
        if !(&order % &h).is_zero() {
            return Err(format!("#E(F_2) = {cofactor} does not divide #E = {order}"));
        }
        let r = &order / &h;
        if r.bits() >= 192 || !is_probable_prime(&r) {
            return Err(format!("#E / {cofactor} = {r} is not a usable prime"));
        }
        let r192 = U192::from_big(&r).expect("r < 2^192");
        let trace: i64 = if a == 0 { -1 } else { 1 };
        // λ: roots of λ² − tλ + 2 mod r.
        let t_mod = if trace < 0 { &r - 1u8 } else { BigUint::one() };
        let eight = BigUint::from(8u8) % &r;
        let disc = ((&t_mod * &t_mod) % &r + &r - eight) % &r;
        let root = sqrt_mod(&disc, &r).ok_or("t² − 8 is not a square mod r")?;
        let inv2 = (&r + 1u8) >> 1usize;
        let roots = [
            ((&t_mod + &root) % &r * &inv2) % &r,
            ((&t_mod + &r - &root) % &r * &inv2) % &r,
        ];
        let ops = BigStrongRho::from_parts_unchecked(
            field.clone(),
            W192::from(u64::from(a)),
            r192,
            RawPointG::Infinity,
        );
        let mut x = 0u64;
        let g = loop {
            x += 1;
            if x > 1 << 20 {
                return Err("no generator in 2^20 abscissae".into());
            }
            let xw = gf2w::from_u64(x);
            let xi = field.inv(&xw);
            let mut c = gf2w::xor(xw, field.sqr(&xi));
            c[0] ^= u64::from(a);
            let Some(z) = field.solve_quadratic(&c) else {
                continue;
            };
            let p = RawPointG::Affine {
                x: W192(xw),
                y: W192(field.mul(&xw, &z)),
            };
            let g = ops.scalar_mul(p, U192::from_u128(u128::from(cofactor)));
            if g == RawPointG::Infinity {
                continue;
            }
            if ops.scalar_mul(g, r192) != RawPointG::Infinity {
                return Err("[r][h]P is not the identity: the order is wrong".into());
            }
            break g;
        };
        let RawPointG::Affine { x: gx, y: gy } = g else {
            unreachable!()
        };
        let frob = RawPointG::Affine {
            x: W192(field.sqr(&gx.0)),
            y: W192(field.sqr(&gy.0)),
        };
        let lambda = roots
            .into_iter()
            .find(|l| ops.scalar_mul(g, U192::from_big(l).expect("λ < r")) == frob)
            .ok_or("neither root of λ² − tλ + 2 is the Frobenius eigenvalue on ⟨G⟩")?;
        Ok(Self {
            a,
            n,
            irreducible: irr.clone(),
            trace,
            group_order: order,
            r,
            cofactor,
            lambda,
            generator: (gx, gy),
            generator_seed_x: x,
        })
    }

    pub fn field(&self) -> Gf2W {
        Gf2W::new(&self.irreducible)
    }

    pub fn r192(&self) -> U192 {
        U192::from_big(&self.r).expect("r < 2^192")
    }

    pub fn lambda192(&self) -> U192 {
        U192::from_big(&self.lambda).expect("λ < r")
    }

    /// The strong reference (and the group arithmetic every big method
    /// uses) on this curve.
    pub fn strong_rho(&self) -> BigStrongRho {
        BigStrongRho::from_parts(
            self.field(),
            W192::from(u64::from(self.a)),
            self.r192(),
            self.lambda192(),
            RawPointG::Affine {
                x: self.generator.0,
                y: self.generator.1,
            },
        )
    }

    /// The curve's ICV1 identity, by the same rule as every other curve.
    pub fn curve_id(&self) -> CurveId {
        curve_id::binary(
            self.n,
            &curve_id::modulus_integer(&self.irreducible),
            &BigUint::from(self.a),
            &BigUint::one(),
            &self.group_order,
            Some(-7),
        )
        .expect("a constructed curve is non-singular and inside the Hasse interval")
    }

    /// The points with abscissa `x` (none, one when `x = 0`, or two).
    pub fn points_with_x(&self, f: &Gf2W, x: W192) -> Vec<(W192, W192)> {
        if gf2w::is_zero(&x.0) {
            // y² = 1.
            return vec![(W192::ZERO, W192::from(1u8))];
        }
        let mut c = gf2w::xor(x.0, f.sqr(&f.inv(&x.0)));
        c[0] ^= u64::from(self.a);
        match f.solve_quadratic(&c) {
            Some(z) => {
                let y = W192(f.mul(&x.0, &z));
                vec![(x, y), (x, y ^ x)]
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
    use crate::cryptanalysis::koblitz_strong_rho::{
        StrongRho, StrongRhoCharges, StrongRhoParams, WideStrongRho,
    };
    use crate::cryptanalysis::koblitz_wide::WideKoblitz;
    use crate::cryptanalysis::wide_gf2m::WideGf2;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn ecc2k130() -> IrreduciblePoly {
        IrreduciblePoly {
            degree: 131,
            low_terms: vec![13, 2, 1, 0],
        }
    }

    fn k163() -> IrreduciblePoly {
        IrreduciblePoly {
            degree: 163,
            low_terms: vec![7, 6, 3, 0],
        }
    }

    fn m83() -> IrreduciblePoly {
        IrreduciblePoly {
            degree: 83,
            low_terms: vec![45, 2, 1, 0],
        }
    }

    fn w(v: u128) -> W192 {
        W192::from_u128(v)
    }

    #[test]
    fn the_word_is_a_192_bit_integer_where_it_meets_u128() {
        let samples = [
            0u128,
            1,
            2,
            0xff,
            u64::MAX as u128,
            1u128 << 64,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
            u128::MAX,
        ];
        for &a in &samples {
            assert_eq!(w(a).fold64(), FieldWord::fold64(a), "fold {a:x}");
            assert_eq!(
                FieldWord::trailing_zeros(w(a)).min(128),
                FieldWord::trailing_zeros(a)
            );
            for i in 0..16 {
                assert_eq!(w(a).byte(i), a.byte(i));
            }
            for s in [0u32, 1, 7, 8, 63, 64, 65, 100, 127] {
                assert_eq!(w(a) >> s, w(a >> s), "{a:x} >> {s}");
                assert_eq!(
                    (w(a) << s).to_biguint(),
                    (BigUint::from(a) << s) % (BigUint::one() << 192usize)
                );
            }
            for &b in &samples {
                assert_eq!(w(a).cmp(&w(b)), a.cmp(&b));
                if a >= b {
                    assert_eq!(w(a) - w(b), w(a - b));
                }
                assert_eq!(w(a) ^ w(b), w(a ^ b));
            }
        }
        let top = W192([0, 0, 1 << 63]);
        assert_eq!(FieldWord::trailing_zeros(top), 191);
        assert_eq!(
            top - W192::from(1u8),
            W192([u64::MAX, u64::MAX, (1 << 63) - 1])
        );
        assert_eq!(W192::ZERO - W192::from(1u8), W192([u64::MAX; 3]));
        assert!(top > w(u128::MAX));
        assert_eq!(top >> 191, W192::from(1u8));
        assert_eq!(W192::from(1u8) << 191, top);
        assert_eq!(W192::parse_hex(&top.hex()), Some(top));
    }

    #[test]
    fn montgomery_ring_matches_bigint() {
        let mut rng = StdRng::seed_from_u64(11);
        let k = BigKoblitz::new(0, &ecc2k130()).unwrap();
        for m in [
            BigUint::from(97u8),
            BigUint::from((1u128 << 127) - 1),
            k.r.clone(),
            (BigUint::one() << 191usize) - 19u8,
            (BigUint::one() << 192usize) - 237u8,
        ] {
            let m192 = U192::from_big(&m).unwrap();
            let ctx = U192::mul_ctx(m192);
            for _ in 0..300 {
                let a: U192 = rng.gen_range(U192::zero()..m192);
                let b: U192 = rng.gen_range(U192::zero()..m192);
                let want = (a.to_big() * b.to_big()) % &m;
                assert_eq!(U192::mul_in(&ctx, a, b).to_big(), want, "m={m}");
                assert_eq!(U192::mul_mod(a, b, m192).to_big(), want);
                assert_eq!(
                    U192::mul_fixed(&ctx, a, U192::fixed(&ctx, b)).to_big(),
                    want
                );
                let (x, y) = U192::mul_pair_in(&ctx, a, b, b);
                assert_eq!(
                    (x.to_big(), y.to_big()),
                    (want, (b.to_big() * b.to_big()) % &m)
                );
                assert_eq!(
                    U192::add_mod(a, b, m192).to_big(),
                    (a.to_big() + b.to_big()) % &m
                );
                assert_eq!(
                    U192::sub_mod(a, b, m192).to_big(),
                    (a.to_big() + &m - b.to_big()) % &m
                );
                if !a.is_zero() && is_probable_prime(&m) {
                    let inv = U192::inverse_mod(a, m192).unwrap();
                    assert_eq!(U192::mul_mod(a, inv, m192), U192::one());
                }
            }
        }
    }

    /// Below `2^128` the sampler is `u128`'s, draw for draw; above, it is
    /// uniform on the range.
    #[test]
    fn the_sampler_draws_what_u128_draws_and_covers_wide_ranges() {
        let mut a = StdRng::seed_from_u64(5);
        let mut b = StdRng::seed_from_u64(5);
        for hi in [2u128, 97, 1 << 70, u128::MAX] {
            for _ in 0..50 {
                let x: u128 = a.gen_range(1..hi);
                let y: U192 = b.gen_range(U192::one()..U192::from_u128(hi));
                assert_eq!(U192::from_u128(x), y);
            }
        }
        let k = BigKoblitz::new(1, &k163()).unwrap();
        let r = k.r192();
        let mut high_bit = 0;
        for _ in 0..2000 {
            let v: U192 = a.gen_range(U192::one()..r);
            assert!(v >= U192::one() && v < r);
            high_bit += usize::from(v.bit(161));
        }
        // About half of [1, r) has bit 161 set (r ≈ 1.0·2^162).
        assert!((800..1200).contains(&high_bit), "{high_bit}");
    }

    #[test]
    fn the_big_field_matches_the_library_field() {
        use crate::binary_ecc::F2mElement;
        for irr in [ecc2k130(), k163()] {
            let f = Gf2W::new(&irr);
            let n = irr.degree;
            let mut rng = StdRng::seed_from_u64(u64::from(n));
            for _ in 0..100 {
                let (a, b) = (f.random(&mut rng), f.random(&mut rng));
                let la = F2mElement::from_biguint(&gf2w::to_biguint(&a), n);
                let lb = F2mElement::from_biguint(&gf2w::to_biguint(&b), n);
                let want = la.mul(&lb, &irr);
                let bits = want.raw_bits();
                let mut got = [0u64; 3];
                got[..bits.len().min(3)].copy_from_slice(&bits[..bits.len().min(3)]);
                assert_eq!(RhoField::mul(&f, W192(a), W192(b)), W192(got));
            }
            // Frobenius has order n.
            let x = f.random(&mut rng);
            assert_eq!(f.sqr_k(&x, n), x);
        }
    }

    /// ECC2K-130's field and K-163: the orders are the registry's, the
    /// group law closes, `[r]G = O`, `λ` is the Frobenius eigenvalue, and
    /// `[k]G` round-trips through additions and Frobenius.
    #[test]
    fn the_two_big_koblitz_curves_have_their_registered_orders_and_laws() {
        for (a, irr, slug, order) in [
            (
                0u8,
                ecc2k130(),
                "icv1-f2m131-tm22283658519494248867-115e0dc5",
                "2722258935367507707729280517973639940516",
            ),
            (
                1,
                k163(),
                "icv1-f2m163-tm4845466632539410776804317-e15e581e",
                "11692013098647223345629483507196896696658237148126",
            ),
        ] {
            let k = BigKoblitz::new(a, &irr).unwrap();
            assert_eq!(k.group_order.to_string(), order);
            assert_eq!(k.curve_id().slug, slug);
            assert_eq!(&k.r * k.cofactor, k.group_order);
            assert!(k.r.bits() > 128, "the class is past u128");
            let rho = k.strong_rho();
            assert_eq!(rho.automorphisms(), 2 * k.n as usize);
            let g = rho.generator();
            let r = k.r192();
            assert_eq!(rho.scalar_mul(g, r), RawPointG::Infinity);
            assert_eq!(
                k.lambda.modpow(&BigUint::from(k.n), &k.r),
                BigUint::one(),
                "λ^n = 1"
            );
            assert_eq!(rho.frobenius(1, g), rho.scalar_mul(g, k.lambda192()));
            let mut rng = StdRng::seed_from_u64(3);
            for _ in 0..3 {
                let x: U192 = rng.gen_range(U192::one()..r);
                let y: U192 = rng.gen_range(U192::one()..r);
                let sum = U192::add_mod(x, y, r);
                assert_eq!(
                    rho.add(rho.scalar_mul(g, x), rho.scalar_mul(g, y)),
                    rho.scalar_mul(g, sum)
                );
                // φ([x]G) = [λx]G.
                let lx = U192::mul_mod(x, k.lambda192(), r);
                assert_eq!(
                    rho.frobenius(1, rho.scalar_mul(g, x)),
                    rho.scalar_mul(g, lx)
                );
                // [r − x]G = −[x]G.
                let neg = U192::sub_mod(U192::zero(), x, r);
                let RawPointG::Affine { x: px, y: py } = rho.scalar_mul(g, x) else {
                    panic!("x < r")
                };
                assert_eq!(
                    rho.scalar_mul(g, neg),
                    RawPointG::Affine { x: px, y: py ^ px }
                );
            }
            // Every point the abscissa rule lifts is on the curve: [#E]P = O.
            let f = k.field();
            let h = U192::from_u128(u128::from(k.cofactor));
            for xv in 2..6u64 {
                for (x, y) in k.points_with_x(&f, W192::from(xv)) {
                    let p = RawPointG::Affine { x, y };
                    assert_eq!(rho.scalar_mul(rho.scalar_mul(p, h), r), RawPointG::Infinity);
                }
            }
        }
    }

    #[test]
    fn unusable_degrees_are_refused() {
        // K_0 over F_2^137 is registered, but its order is not 4 · prime.
        let irr = IrreduciblePoly {
            degree: 137,
            low_terms: vec![21, 0],
        };
        assert!(BigKoblitz::new(0, &irr).is_err());
        assert!(BigKoblitz::new(0, &m83()).is_err(), "83 is the wide class");
    }

    /// At `n = 83` the three-word constructor builds the wide constructor's
    /// curve: the same order, generator, λ and ICV1 slug.
    #[test]
    fn the_three_word_constructor_is_the_wide_constructor_at_n83() {
        let wide = WideKoblitz::new(0, &m83()).unwrap();
        let big = BigKoblitz::for_any_degree(0, &m83()).unwrap();
        assert_eq!(big.group_order, BigUint::from(wide.group_order));
        assert_eq!(big.r, BigUint::from(wide.r));
        assert_eq!(big.lambda, BigUint::from(wide.lambda));
        assert_eq!(big.generator, (w(wide.generator.0), w(wide.generator.1)));
        assert_eq!(big.generator_seed_x as u128, wide.generator_seed_x);
        assert_eq!(big.curve_id(), wide.curve_id());
    }

    fn wide_and_big(curve: &KoblitzCurve) -> (WideStrongRho, BigStrongRho) {
        let narrow = StrongRho::new(curve);
        let RawPointG::Affine { x, y } = narrow.generator() else {
            unreachable!()
        };
        let wide: WideStrongRho = StrongRhoG::from_parts(
            WideGf2::new(&curve.curve.irreducible),
            curve.a as u128,
            u128::from(narrow.modulus()),
            u128::from(narrow.lambda()),
            RawPointG::Affine {
                x: x as u128,
                y: y as u128,
            },
        );
        let big: BigStrongRho = StrongRhoG::from_parts(
            Gf2W::new(&curve.curve.irreducible),
            W192::from(curve.a as u64),
            U192::from_u128(u128::from(narrow.modulus())),
            U192::from_u128(u128::from(narrow.lambda())),
            RawPointG::Affine {
                x: W192::from(x),
                y: W192::from(y),
            },
        );
        (wide, big)
    }

    fn to_big_point(p: RawPointG<u128>) -> RawPointG<W192> {
        match p {
            RawPointG::Infinity => RawPointG::Infinity,
            RawPointG::Affine { x, y } => RawPointG::Affine { x: w(x), y: w(y) },
        }
    }

    /// The strongest check this class can have below a size anyone can
    /// solve: on curves cheap enough to solve, the three-word
    /// instantiation (`Gf2W`, `U192`) walks exactly as the committed wide
    /// one (`WideGf2`, `u128`) — the same recovered logarithm, steps,
    /// walks, abandoned walks, table and every charge — on the reference
    /// walk, the legacy canonicalisation, the escape and the `_v2`
    /// fixed-base table.
    #[test]
    fn the_three_word_walk_is_the_wide_walk_step_for_step() {
        let mut tested = 0;
        for (a, n) in [17u32, 19, 23, 29, 31, 37, 41]
            .into_iter()
            .flat_map(|n| [(0u8, n), (1u8, n)])
        {
            let Some(curve) = KoblitzCurve::new(a, n) else {
                continue;
            };
            tested += 1;
            let (wide, big) = wide_and_big(&curve);
            assert_eq!(wide.automorphisms(), big.automorphisms());
            for seed in 0..3u64 {
                let mut rng = StdRng::seed_from_u64(seed + 99);
                let d: u128 = rng.gen_range(1..wide.modulus());
                let qw = wide.scalar_mul(wide.generator(), d);
                let qb = big.scalar_mul(big.generator(), U192::from_u128(d));
                assert_eq!(qb, to_big_point(qw));
                for (variant, escape, legacy, fixed) in [
                    ("reference", false, false, false),
                    ("legacy", false, true, false),
                    ("escape", true, false, false),
                    ("v2", false, false, true),
                ] {
                    let params = StrongRhoParams {
                        escape_fruitless: escape,
                        dp_bits: if escape { 6 } else { 4 },
                        ..StrongRhoParams::default()
                    };
                    let (mut cw, mut cb) =
                        (StrongRhoCharges::default(), StrongRhoCharges::default());
                    let (jw, jb) = if fixed {
                        (
                            wide.jumps_fixed_base(seed ^ 0x55, 8, &mut cw),
                            big.jumps_fixed_base(seed ^ 0x55, 8, &mut cb),
                        )
                    } else {
                        (
                            wide.jumps(seed ^ 0x55, &mut cw),
                            big.jumps(seed ^ 0x55, &mut cb),
                        )
                    };
                    assert_eq!(cw, cb);
                    for (x, y) in jw.iter().zip(&jb) {
                        assert_eq!(to_big_point(x.point), y.point);
                        assert_eq!(U192::from_u128(x.a), y.a);
                    }
                    let (ow, ob) = if legacy {
                        (
                            wide.solve_legacy(
                                qw,
                                &jw,
                                &mut StdRng::seed_from_u64(seed),
                                &params,
                                cw,
                            ),
                            big.solve_legacy(
                                qb,
                                &jb,
                                &mut StdRng::seed_from_u64(seed),
                                &params,
                                cb,
                            ),
                        )
                    } else {
                        (
                            wide.solve(qw, &jw, &mut StdRng::seed_from_u64(seed), &params, cw),
                            big.solve(qb, &jb, &mut StdRng::seed_from_u64(seed), &params, cb),
                        )
                    };
                    let (ow, ob) = (ow.expect("wide solve"), ob.expect("big solve"));
                    assert_eq!(ow.scalar, d, "n={n} {variant}");
                    assert_eq!(
                        (
                            U192::from_u128(ow.scalar),
                            ow.walk_steps,
                            ow.walks,
                            ow.fruitless,
                            ow.escapes,
                            ow.capped,
                            ow.wasted_merges,
                            ow.table_entries,
                            ow.charges,
                            ow.ideal_steps.to_bits(),
                            &ow.dp_trail_lengths,
                            &ow.abandoned_trail_lengths,
                        ),
                        (
                            ob.scalar,
                            ob.walk_steps,
                            ob.walks,
                            ob.fruitless,
                            ob.escapes,
                            ob.capped,
                            ob.wasted_merges,
                            ob.table_entries,
                            ob.charges,
                            ob.ideal_steps.to_bits(),
                            &ob.dp_trail_lengths,
                            &ob.abandoned_trail_lengths,
                        ),
                        "n={n} seed={seed} {variant}"
                    );
                }
            }
        }
        assert!(tested >= 5, "only {tested} curves constructed");
    }

    /// At the wide gate degree (`n = 83`, a 2^81 subgroup nobody solves
    /// here) a solve capped at its first million steps spends exactly what
    /// the wide solve spends on the same curve, seeds and target.
    #[test]
    fn a_capped_solve_at_n83_spends_what_the_wide_solve_spends() {
        let wk = WideKoblitz::new(0, &m83()).unwrap();
        let bk = BigKoblitz::for_any_degree(0, &m83()).unwrap();
        let (wide, big) = (wk.strong_rho(), bk.strong_rho());
        let d = 0x1234_5678_9abc_def0_1234u128 % wk.r;
        let qw = wide.scalar_mul(wide.generator(), d);
        let qb = big.scalar_mul(big.generator(), U192::from_u128(d));
        assert_eq!(qb, to_big_point(qw));
        // A factor of 0 leaves the floor of the cap: 10^6 steps.
        let params = StrongRhoParams {
            step_cap_factor: 0,
            ..StrongRhoParams::default()
        };
        let (mut cw, mut cb) = (StrongRhoCharges::default(), StrongRhoCharges::default());
        let jw = wide.jumps(7, &mut cw);
        let jb = big.jumps(7, &mut cb);
        let extras = crate::cryptanalysis::koblitz_strong_rho::StrongRhoExtras::default();
        let sw = wide
            .solve_accounted(qw, &jw, &mut StdRng::seed_from_u64(1), &params, cw, extras)
            .expect_err("a 2^81 subgroup is not solved in 10^6 steps");
        let sb = big
            .solve_accounted(qb, &jb, &mut StdRng::seed_from_u64(1), &params, cb, extras)
            .expect_err("a 2^81 subgroup is not solved in 10^6 steps");
        assert_eq!(
            (
                sw.walk_steps,
                sw.walks,
                sw.fruitless,
                sw.capped,
                sw.table_entries,
                sw.charges
            ),
            (
                sb.walk_steps,
                sb.walks,
                sb.fruitless,
                sb.capped,
                sb.table_entries,
                sb.charges
            )
        );
        assert!(sw.walk_steps >= 1_000_000);
    }
}
