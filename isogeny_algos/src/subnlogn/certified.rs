//! Exact signed dyadics and certified scalar exponential routines.
//!
//! Complex records contain integer numerators; their common denominator is
//! `2^p`, supplied by the caller. Completed disk records use the format of
//! sections 6--8 of the September 23 manuscript. Analytic constants are
//! enclosed by directed integer arithmetic, never by machine floating point.
//! Machin's formula uses binary splitting; the exponential uses a binary
//! splitting evaluation of each bit-burst chunk, followed by argument
//! restoration. All precision-size products use the concrete `fastint` backend.
//!
//! On the resampling parameter family, the refinement loops need only
//! `O(log p)` guard bits. There are `O(log p)` bit-burst chunks, each with
//! constant outward error in working-grid units. Reducing a real argument
//! of magnitude at most `pi*p` uses `h` squarings with `2^h <= 16*p`.
//! For negative arguments restoration amplifies absolute interval width by
//! at most a polynomial in `p`; for positive arguments the same statement
//! holds for relative width, before the final exact power-of-two scale.
//! The argument enclosure contributes `O(p)` working-grid units. Thus the
//! final width is `p^O(1) * 2^-work`; doubling the guard reaches `O(log p)`.
//! Phase arguments need at most four squarings, so obey the same bound.
//!
//! The positive Gaussian diagonal has an additional uniform disk margin:
//! `beta^2 <= 1/4`, `alpha >= 2`, `pi < 22/7`, and `ln(2) > 2/3` give
//! `log(D') < -(23/42)*alpha^2 + 4/3 <= -6/7`, hence `D' < 1/2`.
//! Its disk-enclosure test therefore cannot force extra precision near one.
//! Each balanced rational tree has `O(p*log p)` bits and there are only
//! logarithmically many chunks/restoration stages. The concrete soft-linear
//! multiplication/division backends consequently give `p*(log p)^O(1)`
//! scalar work, which is `O(p^(1+delta))` for every fixed positive `delta`.
//! These cost statements concern the used resampling family. In particular,
//! arbitrary public `scaled_positive_exp` arguments approaching its disk
//! boundary may require additional refinement, and oversized rational input
//! encodings also contribute their own bit complexity.

use crate::bigint::Big;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

fn product(a: &Big, b: &Big) -> Big {
    super::fastint::multiply(a, b)
}

fn quotient(a: &Big, b: &Big) -> (Big, Big) {
    super::fastdiv::divrem(a, b)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidPrecision,
    SizeOverflow,
    DivisionByZero,
    InvalidInterval,
    InvalidPhase,
    OutsideDisk,
    ExponentialRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "certified arithmetic: {self:?}")
    }
}
impl std::error::Error for Error {}

/// A canonical signed integer. Zero is always nonnegative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signed {
    pub negative: bool,
    pub magnitude: Big,
}

impl Signed {
    pub fn new(negative: bool, magnitude: Big) -> Self {
        Self {
            negative: negative && !magnitude.is_zero(),
            magnitude,
        }
    }
    pub fn zero() -> Self {
        Self::new(false, Big::zero())
    }
    pub fn from_i64(value: i64) -> Self {
        Self::new(value < 0, Big::from_u64(value.unsigned_abs()))
    }
    pub fn from_big(value: Big) -> Self {
        Self::new(false, value)
    }
    pub fn is_zero(&self) -> bool {
        self.magnitude.is_zero()
    }
    pub fn neg(&self) -> Self {
        Self::new(!self.negative, self.magnitude.clone())
    }
    pub fn add(&self, other: &Self) -> Self {
        if self.negative == other.negative {
            Self::new(self.negative, self.magnitude.add(&other.magnitude))
        } else if self.magnitude >= other.magnitude {
            Self::new(self.negative, self.magnitude.sub(&other.magnitude))
        } else {
            Self::new(other.negative, other.magnitude.sub(&self.magnitude))
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }
    pub fn mul(&self, other: &Self) -> Self {
        Self::new(
            self.negative ^ other.negative,
            product(&self.magnitude, &other.magnitude),
        )
    }
    pub fn mul_big(&self, other: &Big) -> Self {
        Self::new(self.negative, product(&self.magnitude, other))
    }
    pub fn mul_small(&self, other: u64) -> Self {
        self.mul_big(&Big::from_u64(other))
    }
    pub fn shl(&self, bits: usize) -> Self {
        Self::new(self.negative, self.magnitude.shl(bits))
    }
    pub fn shr_trunc(&self, bits: usize) -> Self {
        Self::new(self.negative, self.magnitude.shr(bits))
    }
    pub fn div_trunc(&self, divisor: &Big) -> Self {
        Self::new(self.negative, quotient(&self.magnitude, divisor).0)
    }
    fn div_floor(&self, divisor: &Big) -> Self {
        let (mut q, r) = quotient(&self.magnitude, divisor);
        if self.negative && !r.is_zero() {
            q = q.add_small(1);
        }
        Self::new(self.negative, q)
    }
    fn div_ceil(&self, divisor: &Big) -> Self {
        let (mut q, r) = quotient(&self.magnitude, divisor);
        if !self.negative && !r.is_zero() {
            q = q.add_small(1);
        }
        Self::new(self.negative, q)
    }
    fn shr_floor(&self, bits: usize) -> Self {
        let q = self.magnitude.shr(bits);
        let lost = q.shl(bits) != self.magnitude;
        Self::new(
            self.negative,
            if self.negative && lost {
                q.add_small(1)
            } else {
                q
            },
        )
    }
    fn shr_ceil(&self, bits: usize) -> Self {
        let q = self.magnitude.shr(bits);
        let lost = q.shl(bits) != self.magnitude;
        Self::new(
            self.negative,
            if !self.negative && lost {
                q.add_small(1)
            } else {
                q
            },
        )
    }
}

impl Ord for Signed {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.magnitude.cmp(&other.magnitude),
            (true, true) => other.magnitude.cmp(&self.magnitude),
        }
    }
}
impl PartialOrd for Signed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Numerators of a complex dyadic on a caller-specified grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Complex {
    pub re: Signed,
    pub im: Signed,
}

impl Complex {
    pub fn zero() -> Self {
        Self {
            re: Signed::zero(),
            im: Signed::zero(),
        }
    }
    pub fn real(re: Signed) -> Self {
        Self {
            re,
            im: Signed::zero(),
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re.add(&other.re),
            im: self.im.add(&other.im),
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            re: self.re.sub(&other.re),
            im: self.im.sub(&other.im),
        }
    }
    pub fn neg(&self) -> Self {
        Self {
            re: self.re.neg(),
            im: self.im.neg(),
        }
    }
    pub fn conj(&self) -> Self {
        Self {
            re: self.re.clone(),
            im: self.im.neg(),
        }
    }
    /// Exact product of integer numerator pairs; no grid rounding.
    pub fn mul_exact(&self, other: &Self) -> Self {
        Self {
            re: self.re.mul(&other.re).sub(&self.im.mul(&other.im)),
            im: self.re.mul(&other.im).add(&self.im.mul(&other.re)),
        }
    }
    pub fn mul_big(&self, scalar: &Big) -> Self {
        Self {
            re: self.re.mul_big(scalar),
            im: self.im.mul_big(scalar),
        }
    }
    pub fn mul_real(&self, scalar: &Signed) -> Self {
        Self {
            re: self.re.mul(scalar),
            im: self.im.mul(scalar),
        }
    }
    pub fn shl(&self, bits: usize) -> Self {
        Self {
            re: self.re.shl(bits),
            im: self.im.shl(bits),
        }
    }
    /// Componentwise truncation toward zero, including negative components.
    pub fn shr_trunc(&self, bits: usize) -> Self {
        Self {
            re: self.re.shr_trunc(bits),
            im: self.im.shr_trunc(bits),
        }
    }
    pub fn mul_grid(&self, other: &Self, p: usize) -> Self {
        self.mul_exact(other).shr_trunc(p)
    }
    pub fn is_disk(&self, p: usize) -> bool {
        let Some(twice) = p.checked_mul(2) else {
            return false;
        };
        product(&self.re.magnitude, &self.re.magnitude)
            .add(&product(&self.im.magnitude, &self.im.magnitude))
            <= Big::from_u64(1).shl(twice)
    }
}

/// Closed real interval with exact dyadic endpoints on the `2^-bits` grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealInterval {
    pub lower: Signed,
    pub upper: Signed,
    pub bits: usize,
}

impl RealInterval {
    fn point(value: Signed, bits: usize) -> Self {
        Self {
            lower: value.clone(),
            upper: value,
            bits,
        }
    }
    fn one(bits: usize) -> Self {
        Self::point(Signed::from_big(Big::from_u64(1).shl(bits)), bits)
    }
    fn rational(value: &Signed, denominator: &Big, bits: usize) -> Self {
        let numerator = value.shl(bits);
        Self {
            lower: numerator.div_floor(denominator),
            upper: numerator.div_ceil(denominator),
            bits,
        }
    }
    fn add(&self, other: &Self) -> Self {
        debug_assert_eq!(self.bits, other.bits);
        Self {
            lower: self.lower.add(&other.lower),
            upper: self.upper.add(&other.upper),
            bits: self.bits,
        }
    }
    fn sub(&self, other: &Self) -> Self {
        debug_assert_eq!(self.bits, other.bits);
        Self {
            lower: self.lower.sub(&other.upper),
            upper: self.upper.sub(&other.lower),
            bits: self.bits,
        }
    }
    fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.bits, other.bits);
        let values = [
            self.lower.mul(&other.lower),
            self.lower.mul(&other.upper),
            self.upper.mul(&other.lower),
            self.upper.mul(&other.upper),
        ];
        Self {
            lower: values.iter().min().unwrap().shr_floor(self.bits),
            upper: values.iter().max().unwrap().shr_ceil(self.bits),
            bits: self.bits,
        }
    }
    fn mul_small(&self, scalar: u64) -> Self {
        Self {
            lower: self.lower.mul_small(scalar),
            upper: self.upper.mul_small(scalar),
            bits: self.bits,
        }
    }
    fn mul_ratio(&self, numerator: &Big, denominator: &Big) -> Self {
        Self {
            lower: self.lower.mul_big(numerator).div_floor(denominator),
            upper: self.upper.mul_big(numerator).div_ceil(denominator),
            bits: self.bits,
        }
    }
    fn widen(&self, units: &Big) -> Self {
        let units = Signed::from_big(units.clone());
        Self {
            lower: self.lower.sub(&units),
            upper: self.upper.add(&units),
            bits: self.bits,
        }
    }
    fn divide_pow2(&self, shift: usize) -> Self {
        Self {
            lower: self.lower.shr_floor(shift),
            upper: self.upper.shr_ceil(shift),
            bits: self.bits,
        }
    }
    fn rescale_down(&self, bits: usize) -> Self {
        debug_assert!(bits <= self.bits);
        Self {
            lower: self.lower.shr_floor(self.bits - bits),
            upper: self.upper.shr_ceil(self.bits - bits),
            bits,
        }
    }
    pub fn width_units(&self) -> Big {
        self.upper.sub(&self.lower).magnitude
    }
}

#[derive(Clone)]
struct ComplexInterval {
    re: RealInterval,
    im: RealInterval,
}
impl ComplexInterval {
    fn one(bits: usize) -> Self {
        Self {
            re: RealInterval::one(bits),
            im: RealInterval::point(Signed::zero(), bits),
        }
    }
    fn mul(&self, other: &Self) -> Self {
        Self {
            re: self.re.mul(&other.re).sub(&self.im.mul(&other.im)),
            im: self.re.mul(&other.im).add(&self.im.mul(&other.re)),
        }
    }
    fn widen(&self, units: &Big) -> Self {
        Self {
            re: self.re.widen(units),
            im: self.im.widen(units),
        }
    }
}

// For ratios r_k=P_k/Q_k, a node represents the product of its ratios and
// 1+r_a+r_a*r_(a+1)+... . Its sum numerator is T and common denominator Q.
// This balanced product tree avoids a linear sequence of full-width products.
struct Split {
    p: Complex,
    q: Big,
    t: Complex,
}
enum Series<'a> {
    Exp {
        numerator: &'a Complex,
        denominator_bits: usize,
    },
    Atan {
        denominator: u64,
    },
}

fn split_series(series: &Series<'_>, start: usize, end: usize) -> Split {
    if end - start == 1 {
        let (p, q) = match series {
            Series::Exp {
                numerator,
                denominator_bits,
            } => (
                (**numerator).clone(),
                Big::from_u64((start + 1) as u64).shl(*denominator_bits),
            ),
            Series::Atan { denominator } => {
                let p = Complex::real(Signed::new(true, Big::from_u64((2 * start + 1) as u64)));
                let q = Big::from_u64((2 * start + 3) as u64).mul_small(denominator * denominator);
                (p, q)
            }
        };
        return Split {
            p,
            t: Complex::real(Signed::from_big(q.clone())),
            q,
        };
    }
    let mid = start + (end - start) / 2;
    let left = split_series(series, start, mid);
    let right = split_series(series, mid, end);
    Split {
        p: left.p.mul_exact(&right.p),
        q: product(&left.q, &right.q),
        t: left.t.mul_big(&right.q).add(&left.p.mul_exact(&right.t)),
    }
}

fn pi_interval(bits: usize) -> Result<RealInterval, Error> {
    let work = bits.checked_add(8).ok_or(Error::SizeOverflow)?;
    let atan = |q, decay| {
        let count = (work + 8).div_ceil(decay);
        let value = split_series(&Series::Atan { denominator: q }, 0, count);
        // The alternating tail is below q^(-2N-1), hence below 2^(-work-6)
        // with these conservative counts (q=5>=4, q=239>=128).
        RealInterval::rational(&value.t.re, &value.q.mul_small(q), work).widen(&Big::from_u64(1))
    };
    Ok(atan(5, 4)
        .mul_small(16)
        .sub(&atan(239, 14).mul_small(4))
        .rescale_down(bits))
}

// exp(real*x) or exp(i*x), for an EXACT dyadic |x|<=1/2. Decompose x into
// chunks of 2,4,8,... fractional bits. A chunk after b known bits has
// magnitude <2^-b, so O(bits/b) Taylor terms suffice. Each balanced rational
// tree has O(bits*log(bits)) bits, instead of quadratic-size denominators.
fn exp_small_point(
    value: &Signed,
    denominator_bits: usize,
    bits: usize,
    imaginary: bool,
) -> ComplexInterval {
    let mut result = ComplexInterval::one(bits);
    let mut previous = Big::zero();
    let mut previous_bits = 0;
    let mut chunk_bits = 2.min(denominator_bits);
    loop {
        let prefix = value.magnitude.shr(denominator_bits - chunk_bits);
        let chunk = prefix.sub(&previous.shl(chunk_bits - previous_bits));
        if !chunk.is_zero() {
            let component = Signed::new(value.negative, chunk);
            let numerator = if imaginary {
                Complex {
                    re: Signed::zero(),
                    im: component,
                }
            } else {
                Complex::real(component)
            };
            let count = (bits + 8).div_ceil(previous_bits.max(1));
            let series = split_series(
                &Series::Exp {
                    numerator: &numerator,
                    denominator_bits: chunk_bits,
                },
                0,
                count,
            );
            let mut factor = ComplexInterval {
                re: RealInterval::rational(&series.t.re, &series.q, bits),
                im: RealInterval::rational(&series.t.im, &series.q, bits),
            };
            // |tail| <= 2|x|^N < 2^(-bits-6). One outward unit is ample.
            factor.re = factor.re.widen(&Big::from_u64(1));
            if imaginary {
                factor.im = factor.im.widen(&Big::from_u64(1));
            }
            result = result.mul(&factor);
        }
        if chunk_bits == denominator_bits {
            break;
        }
        previous = prefix;
        previous_bits = chunk_bits;
        chunk_bits = chunk_bits.saturating_mul(2).min(denominator_bits);
    }
    result
}

fn exp_point(value: &Signed, bits: usize, imaginary: bool) -> Result<ComplexInterval, Error> {
    // After division by 2^s the exact dyadic has modulus at most 1/2.
    let shifts = value
        .magnitude
        .bits()
        .saturating_add(1)
        .saturating_sub(bits);
    let denominator_bits = bits.checked_add(shifts).ok_or(Error::SizeOverflow)?;
    let mut result = exp_small_point(value, denominator_bits, bits, imaginary);
    for _ in 0..shifts {
        result = result.mul(&result);
    }
    Ok(result)
}

/// Certified scalar routines at the disk-grid precision `p`.
pub struct Arithmetic {
    p: usize,
    pi: BTreeMap<usize, RealInterval>,
}

impl Arithmetic {
    pub fn new(p: usize) -> Result<Self, Error> {
        if p < 2 {
            return Err(Error::InvalidPrecision);
        }
        p.checked_mul(8).ok_or(Error::SizeOverflow)?;
        Ok(Self {
            p,
            pi: BTreeMap::new(),
        })
    }
    pub fn precision(&self) -> usize {
        self.p
    }
    fn pi_at(&mut self, bits: usize) -> Result<RealInterval, Error> {
        if let Some(value) = self.pi.get(&bits) {
            return Ok(value.clone());
        }
        let value = pi_interval(bits)?;
        self.pi.insert(bits, value.clone());
        Ok(value)
    }
    fn initial_guard(&self) -> usize {
        32 + 2 * (usize::BITS - self.p.leading_zeros()) as usize
    }

    /// Disk-grid approximation of `exp(2*pi*i*j/k)`, with strict error
    /// `<2*2^-p`. Enclosures choose each component toward zero; thus both
    /// component magnitudes are no larger than those of the exact unit phase.
    pub fn phase(&mut self, j: &Big, k: &Big) -> Result<Complex, Error> {
        if k.is_zero() || j >= k {
            return Err(Error::InvalidPhase);
        }
        if j.is_zero() {
            return Ok(Complex::real(Signed::from_big(
                Big::from_u64(1).shl(self.p),
            )));
        }
        let mut guard = self.initial_guard();
        loop {
            let work = self.p.checked_add(guard).ok_or(Error::SizeOverflow)?;
            let angle = self.pi_at(work)?.mul_ratio(&j.shl(1), k);
            let enclosure = exp_point(&angle.lower, work, true)?.widen(&angle.width_units());
            let threshold = Big::from_u64(1).shl(guard - 2);
            if enclosure.re.width_units() < threshold && enclosure.im.width_units() < threshold {
                let inside = |x: &RealInterval| {
                    if x.lower > Signed::zero() {
                        x.lower.shr_trunc(guard)
                    } else if x.upper < Signed::zero() {
                        x.upper.shr_trunc(guard)
                    } else {
                        Signed::zero()
                    }
                };
                let value = Complex {
                    re: inside(&enclosure.re),
                    im: inside(&enclosure.im),
                };
                if !value.is_disk(self.p) {
                    return Err(Error::OutsideDisk);
                }
                // Each coordinate error <(1+1/4)ulp; sqrt(2)*5/4<2.
                return Ok(value);
            }
            guard = guard.checked_mul(2).ok_or(Error::SizeOverflow)?;
        }
    }

    /// Lower approximation of `exp(-pi*numerator/denominator)`, with strict
    /// error `<2*2^-p` and a returned value in `[0,1]`.
    pub fn gaussian(&mut self, numerator: &Big, denominator: &Big) -> Result<Signed, Error> {
        if denominator.is_zero() {
            return Err(Error::DivisionByZero);
        }
        if numerator > &denominator.mul_small(self.p as u64) {
            // pi>1 and exp(-p)<2^-p, so this certified tail rounds to zero.
            return Ok(Signed::zero());
        }
        self.real_exponential(numerator, denominator, true, 0)
    }

    /// Lower approximation of `2^-sigma * exp(pi*numerator/denominator)`.
    /// The result is accepted only after the enclosure proves it lies in the
    /// disk. The Gaussian compression parameters satisfy this hypothesis.
    pub fn scaled_positive_exp(
        &mut self,
        numerator: &Big,
        denominator: &Big,
        sigma: usize,
    ) -> Result<Signed, Error> {
        if denominator.is_zero() {
            return Err(Error::DivisionByZero);
        }
        if sigma > 2 * self.p || numerator > &denominator.mul_small(self.p as u64) {
            return Err(Error::ExponentialRange);
        }
        self.real_exponential(numerator, denominator, false, sigma)
    }

    fn real_exponential(
        &mut self,
        numerator: &Big,
        denominator: &Big,
        negative: bool,
        scale: usize,
    ) -> Result<Signed, Error> {
        if numerator.is_zero() {
            return Ok(Signed::from_big(Big::from_u64(1).shl(self.p).shr(scale)));
        }
        let mut guard = self.initial_guard();
        loop {
            let work = self.p.checked_add(guard).ok_or(Error::SizeOverflow)?;
            let mut argument = self.pi_at(work)?.mul_ratio(numerator, denominator);
            if negative {
                argument = RealInterval {
                    lower: argument.upper.neg(),
                    upper: argument.lower.neg(),
                    bits: work,
                };
            }
            let lower = exp_point(&argument.lower, work, false)?.re.lower;
            let upper = exp_point(&argument.upper, work, false)?.re.upper;
            let enclosure = RealInterval {
                lower,
                upper,
                bits: work,
            }
            .divide_pow2(scale);
            let unit = Signed::from_big(Big::from_u64(1).shl(work));
            if !negative && enclosure.lower > unit {
                return Err(Error::ExponentialRange);
            }
            if enclosure.width_units() < Big::from_u64(1).shl(guard)
                && (negative || enclosure.upper <= unit)
            {
                let value = if enclosure.lower.negative {
                    Signed::zero()
                } else {
                    enclosure.lower.shr_trunc(guard)
                };
                // Lower directed endpoint plus final truncation loses <2ulp.
                return Ok(value);
            }
            guard = guard.checked_mul(2).ok_or(Error::SizeOverflow)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_rounding_is_directed_for_both_signs() {
        for n in -40i64..=40 {
            let x = Signed::from_i64(n);
            for shift in 0..5 {
                let den = 1i64 << shift;
                assert_eq!(x.shr_trunc(shift), Signed::from_i64(n / den));
                assert_eq!(x.shr_floor(shift), Signed::from_i64(n.div_euclid(den)));
                assert_eq!(x.shr_ceil(shift), Signed::from_i64(-(-n).div_euclid(den)));
            }
        }
        assert!(!Signed::new(true, Big::zero()).negative);
    }

    #[test]
    fn machin_interval_contains_known_pi_prefix() {
        let x = pi_interval(64).unwrap();
        // floor(pi*2^64), a hexadecimal integer independent of this routine.
        let known = Signed::from_big(Big::from_u128(0x3_243f6a8885a308d3));
        assert!(x.lower <= known);
        assert!(x.upper >= known.add(&Signed::from_i64(1)));
        assert!(x.width_units() <= Big::from_u64(2));
    }

    #[test]
    fn certified_phases_obey_disk_and_symmetries() {
        let mut arithmetic = Arithmetic::new(104).unwrap();
        let unit = Big::from_u64(1).shl(104);
        for (j, k, re, im) in [(0, 4, 1, 0), (1, 4, 0, 1), (2, 4, -1, 0), (3, 4, 0, -1)] {
            let value = arithmetic
                .phase(&Big::from_u64(j), &Big::from_u64(k))
                .unwrap();
            let exact = Complex {
                re: Signed::from_i64(re).mul_big(&unit),
                im: Signed::from_i64(im).mul_big(&unit),
            };
            assert!(value.is_disk(104));
            assert!(value.re.sub(&exact.re).magnitude <= Big::from_u64(1));
            assert!(value.im.sub(&exact.im).magnitude <= Big::from_u64(1));
        }
        let a = arithmetic
            .phase(&Big::from_u64(1), &Big::from_u64(7))
            .unwrap();
        let b = arithmetic
            .phase(&Big::from_u64(6), &Big::from_u64(7))
            .unwrap();
        assert!(a.re.sub(&b.re).magnitude <= Big::from_u64(2));
        assert!(a.im.add(&b.im).magnitude <= Big::from_u64(2));
    }

    #[test]
    fn eighth_roots_match_an_independent_integer_square_root() {
        let p = 192;
        let mut arithmetic = Arithmetic::new(p).unwrap();
        let magnitude = Big::from_u64(1).shl(2 * p - 1).isqrt();
        for (j, re_sign, im_sign) in [
            (1, false, false),
            (3, true, false),
            (5, true, true),
            (7, false, true),
        ] {
            let value = arithmetic
                .phase(&Big::from_u64(j), &Big::from_u64(8))
                .unwrap();
            let real = Signed::new(re_sign, magnitude.clone());
            let imaginary = Signed::new(im_sign, magnitude.clone());
            assert!(value.re.sub(&real).magnitude <= Big::from_u64(1));
            assert!(value.im.sub(&imaginary).magnitude <= Big::from_u64(1));
            assert!(value.is_disk(p));
        }
    }

    #[test]
    fn gaussian_and_scaled_positive_are_certified_and_consistent() {
        let mut arithmetic = Arithmetic::new(104).unwrap();
        let one = Big::from_u64(1);
        let neg = arithmetic.gaussian(&one, &one).unwrap();
        let pos = arithmetic.scaled_positive_exp(&one, &one, 8).unwrap();
        assert!(!neg.negative && !pos.negative);
        // exp(-pi)*[exp(pi)/256] = 1/256. Lower approximations multiply
        // below the exact result; each loses <2ulp and both are in the disk.
        let combined = neg.mul(&pos).shr_trunc(104);
        let exact = Signed::from_big(one.shl(96));
        assert!(combined <= exact);
        assert!(exact.sub(&combined).magnitude < Big::from_u64(5));
        assert_eq!(
            arithmetic.gaussian(&Big::from_u64(105), &one).unwrap(),
            Signed::zero()
        );
    }
}
