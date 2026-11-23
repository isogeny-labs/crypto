//! Certified Gaussian resampling with retained coordinate permutations.
//!
//! This implements the actual one-dimensional numerical maps of section 7:
//! `A=S/2` and `B0=D' J' C`. The finite Gaussian windows and Neumann series
//! are those of Harvey--van der Hoeven, Lemmas 4.8--4.12 (45-page author
//! manuscript), as imported by the September 23 manuscript. It never forms
//! a dense Fourier matrix or substitutes a direct DFT for resampling.
//!
//! Coefficient reads are consecutive periodic window scans. Successive S
//! window starts advance by zero or one; successive E starts advance by one.
//! For a radius m with s>2m, only O(mt/s) expansion rows (O(m) E rows) cross
//! the cyclic boundary. Charging a whole O(sp)-bit traverse at each such
//! boundary gives O(tmp) movement. When s<=2m, repeated period traversals
//! cost O((s+m)p)=O(mp) per row, also O(tmp) in total. A row rewind costs
//! O(mp). Thus these iterator scans have the paper's tape movement bound;
//! they do not depend on constant-cost access to arbitrary input records.
//! The cached E coefficients are a stream in row/window order. Replaying
//! and rewinding it costs O(smp) per iterate, within that same bound.

use super::certified::{Arithmetic, Complex, Signed};
use crate::bigint::Big;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Arithmetic(super::certified::Error),
    InvalidParameters(&'static str),
    Length { expected: usize, actual: usize },
    OutsideDisk { index: usize },
    SizeOverflow,
}
impl From<super::certified::Error> for Error {
    fn from(error: super::certified::Error) -> Self {
        Self::Arithmetic(error)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gaussian resampling: {self:?}")
    }
}
impl std::error::Error for Error {}

/// Every exact output lies within `error_ulps * 2^-precision` (strictly) of
/// the computed coordinate, in the maximum complex modulus norm.
#[derive(Clone, Debug)]
pub struct Approximation {
    pub values: Vec<Complex>,
    pub precision: usize,
    pub error_ulps: Big,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn ceil_sqrt(value: usize) -> usize {
    let mut lo = 0;
    let mut hi = value.min(1usize << (usize::BITS / 2));
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mid != 0 && mid >= value.div_ceil(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn integer(value: i128) -> Signed {
    Signed::new(value < 0, Big::from_u128(value.unsigned_abs()))
}

fn checked_vector(values: &[Complex], expected: usize, p: usize) -> Result<(), Error> {
    if values.len() != expected {
        return Err(Error::Length {
            expected,
            actual: values.len(),
        });
    }
    for (index, value) in values.iter().enumerate() {
        if !value.is_disk(p) {
            return Err(Error::OutsideDisk { index });
        }
    }
    Ok(())
}

fn periodic<T>(values: &[T], start: usize) -> impl Iterator<Item = &T> {
    values[start..].iter().chain(values[..start].iter()).cycle()
}

/// One coordinate's resampling parameters and numerical constants.
pub struct ResamplingPlan {
    p: usize,
    s: usize,
    t: usize,
    alpha: usize,
    alpha2: usize,
    expansion_radius: usize,
    inverse_radius: usize,
    inverse_terms: usize,
    selected: Vec<usize>,
    beta_numerators: Vec<Signed>,
    arithmetic: Arithmetic,
    // Only the 2m explicitly retained off-diagonal Gaussian terms per row.
    // Reusing this coefficient stream does not change the prescribed rounding.
    inverse_weights: Option<Vec<Vec<Signed>>>,
    diagonal: Option<Vec<Signed>>,
}

impl ResamplingPlan {
    /// Check every finite hypothesis of the no-sort resampling lemma by exact
    /// integer arithmetic: p>100, 2<=s<t<2^p, gcd(s,t)=1, 2<=alpha<sqrt(p),
    /// and alpha^4*(t-s)>p*s.
    pub fn new(p: usize, s: usize, t: usize, alpha: usize) -> Result<Self, Error> {
        if p <= 100 {
            return Err(Error::InvalidParameters("p must exceed 100"));
        }
        if s < 2 || s >= t || (usize::BITS - t.leading_zeros()) as usize > p {
            return Err(Error::InvalidParameters("require 2 <= s < t < 2^p"));
        }
        if gcd(s, t) != 1 {
            return Err(Error::InvalidParameters(
                "source and target lengths must be coprime",
            ));
        }
        let alpha2 = alpha.checked_mul(alpha).ok_or(Error::SizeOverflow)?;
        if alpha < 2 || alpha2 >= p {
            return Err(Error::InvalidParameters("require 2 <= alpha < sqrt(p)"));
        }
        let a2 = Big::from_u64(alpha2 as u64);
        let separation = super::fastint::multiply(&a2, &a2).mul_small((t - s) as u64);
        if separation <= Big::from_u64(p as u64).mul_small(s as u64) {
            return Err(Error::InvalidParameters("require alpha^4*(t-s) > p*s"));
        }
        let sqrt = ceil_sqrt(p);
        let expansion_radius = sqrt.checked_mul(alpha).ok_or(Error::SizeOverflow)?;
        let inverse_radius = sqrt.div_ceil(2 * alpha);
        let numerator = Big::from_u64(p as u64).mul_small(s as u64);
        let denominator = a2.mul_small((t - s) as u64);
        let (terms, remainder) = super::fastdiv::divrem(&numerator, &denominator);
        let terms = if remainder.is_zero() {
            terms
        } else {
            terms.add_small(1)
        };
        let inverse_terms = usize::try_from(terms.to_u128().ok_or(Error::SizeOverflow)?)
            .map_err(|_| Error::SizeOverflow)?;
        let mut selected = Vec::with_capacity(s);
        let mut beta_numerators = Vec::with_capacity(s);
        // Ordered quotient/remainder recurrence from section 7. No coordinate
        // permutation is executed: C consumes an increasing subsequence.
        let (whole, step) = (t / s, t % s);
        let (mut floor, mut remainder) = (0usize, 0usize);
        for j in 0..s {
            let rounded = floor + usize::from((remainder as u128) * 2 >= s as u128);
            selected.push(rounded);
            beta_numerators.push(if rounded == floor {
                Signed::from_big(Big::from_u64(remainder as u64))
            } else {
                Signed::new(true, Big::from_u64((s - remainder) as u64))
            });
            if j + 1 < s {
                let sum = remainder as u128 + step as u128;
                let carry = usize::from(sum >= s as u128);
                remainder = (sum - carry as u128 * s as u128) as usize;
                floor += whole + carry;
            }
        }
        debug_assert!(selected.windows(2).all(|w| w[0] < w[1]));
        debug_assert!(selected[s - 1] < t);
        Ok(Self {
            p,
            s,
            t,
            alpha,
            alpha2,
            expansion_radius,
            inverse_radius,
            inverse_terms,
            selected,
            beta_numerators,
            arithmetic: Arithmetic::new(p)?,
            inverse_weights: None,
            diagonal: None,
        })
    }

    pub fn precision(&self) -> usize {
        self.p
    }
    pub fn source_len(&self) -> usize {
        self.s
    }
    pub fn target_len(&self) -> usize {
        self.t
    }
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected
    }
    pub fn neumann_terms(&self) -> usize {
        self.inverse_terms
    }

    /// Approximate `S/2` using precisely `|j-sk/t| < ceil(sqrt(p))*alpha`.
    /// Each Gaussian, coefficient multiplication, and input multiplication is
    /// separately rounded as in Lemma 4.9; row sums are exact signed sums.
    pub fn expand(&mut self, input: &[Complex]) -> Result<Approximation, Error> {
        checked_vector(input, self.s, self.p)?;
        let beta = Signed::from_big(
            Big::from_u64(1)
                .shl(self.p - 1)
                .divrem_small(self.alpha as u64)
                .0,
        );
        let alpha_t = Big::from_u64(self.alpha as u64).mul_small(self.t as u64);
        let denominator = super::fastint::multiply(&alpha_t, &alpha_t);
        let target = Big::from_u64(self.t as u64);
        let mut values = Vec::with_capacity(self.t);
        for k in 0..self.t {
            let sk = self.s as u128 * k as u128;
            let floor = (sk / self.t as u128) as i128;
            let remainder = sk % self.t as u128;
            let radius = self.expansion_radius as i128;
            let first = floor - radius + 1;
            let last = floor + radius - i128::from(remainder == 0);
            let center = Signed::from_big(Big::from_u128(sk));
            let mut sum = Complex::zero();
            let start = first.rem_euclid(self.s as i128) as usize;
            for (j, input_value) in (first..=last).zip(periodic(input, start)) {
                let difference = integer(j).mul_big(&target).sub(&center);
                let numerator =
                    super::fastint::multiply(&difference.magnitude, &difference.magnitude);
                let gaussian = self.arithmetic.gaussian(&numerator, &denominator)?;
                let weight = beta.mul(&gaussian).shr_trunc(self.p);
                sum = sum.add(&input_value.mul_real(&weight).shr_trunc(self.p));
            }
            if !sum.is_disk(self.p) {
                return Err(Error::OutsideDisk { index: k });
            }
            values.push(sum);
        }
        Ok(Approximation {
            values,
            precision: self.p,
            error_ulps: Big::from_u64(self.p as u64).mul_small(16),
        })
    }

    fn prepare_inverse(&mut self) -> Result<(), Error> {
        if self.inverse_weights.is_some() {
            return Ok(());
        }
        let source = Big::from_u64(self.s as u64);
        let denominator = super::fastint::multiply(&source, &source);
        let target = Big::from_u64(self.t as u64);
        let mut rows = Vec::with_capacity(self.s);
        let mut diagonal = Vec::with_capacity(self.s);
        for j in 0..self.s {
            let mut row = Vec::with_capacity(2 * self.inverse_radius);
            for h in -(self.inverse_radius as i128)..=self.inverse_radius as i128 {
                if h == 0 {
                    continue;
                }
                let index = (j as i128 + h).rem_euclid(self.s as i128) as usize;
                let displacement = integer(h).mul_big(&target).add(&self.beta_numerators[j]);
                let first =
                    super::fastint::multiply(&displacement.magnitude, &displacement.magnitude);
                let beta = &self.beta_numerators[index].magnitude;
                let second = super::fastint::multiply(beta, beta);
                if first < second {
                    return Err(Error::InvalidParameters(
                        "nonpositive off-diagonal Gaussian exponent",
                    ));
                }
                let numerator = first.sub(&second).mul_small(self.alpha2 as u64);
                let weight = self.arithmetic.gaussian(&numerator, &denominator)?;
                row.push(weight);
            }
            rows.push(row);
            let beta = &self.beta_numerators[j].magnitude;
            let numerator = super::fastint::multiply(beta, beta).mul_small(self.alpha2 as u64);
            diagonal.push(self.arithmetic.scaled_positive_exp(
                &numerator,
                &denominator,
                2 * self.alpha2 - 2,
            )?);
        }
        self.inverse_weights = Some(rows);
        self.diagonal = Some(diagonal);
        Ok(())
    }

    fn apply_e(&self, input: &[Complex]) -> Result<Vec<Complex>, Error> {
        let rows = self
            .inverse_weights
            .as_ref()
            .expect("prepared inverse coefficients");
        let mut values = Vec::with_capacity(self.s);
        for (j, row) in rows.iter().enumerate() {
            let mut sum = Complex::zero();
            let start =
                (j as i128 - self.inverse_radius as i128).rem_euclid(self.s as i128) as usize;
            let mut window = periodic(input, start);
            for (position, weight) in row.iter().enumerate() {
                if position == self.inverse_radius {
                    // The off-diagonal E window omits h=0, but the tape still
                    // consumes that input record on its way to positive h.
                    window.next();
                }
                let input_value = window.next().expect("nonempty periodic input");
                sum = sum.add(&input_value.mul_real(weight).shr_trunc(self.p));
            }
            if !sum.is_disk(self.p) {
                return Err(Error::OutsideDisk { index: j });
            }
            values.push(sum);
        }
        Ok(values)
    }

    /// Approximate `D' J' C`, retaining the source/target permutations in the
    /// transform identity. The Neumann series has exactly
    /// `ceil(p*s / (alpha^2*(t-s)))` terms; E uses `0<|h|<=ceil(sqrt(p)/(2alpha))`.
    pub fn compress(&mut self, input: &[Complex]) -> Result<Approximation, Error> {
        checked_vector(input, self.t, self.p)?;
        self.prepare_inverse()?;
        let mut iterate = Vec::with_capacity(self.s);
        let mut selected = 0;
        // C is literally an ordered subsequence scan, including consumption
        // of unselected records. No frequency permutation occurs here.
        for (index, value) in input.iter().enumerate() {
            if selected < self.s && index == self.selected[selected] {
                iterate.push(value.shr_trunc(1));
                selected += 1;
            }
        }
        debug_assert_eq!(selected, self.s);
        let mut sum = iterate.clone();
        for step in 1..self.inverse_terms {
            iterate = self.apply_e(&iterate)?;
            for (total, value) in sum.iter_mut().zip(&iterate) {
                *total = if step % 2 == 0 {
                    total.add(value)
                } else {
                    total.sub(value)
                };
            }
        }
        checked_vector(&sum, self.s, self.p)?;
        let diagonal = self
            .diagonal
            .as_ref()
            .expect("prepared diagonal coefficients");
        let values: Vec<_> = sum
            .iter()
            .zip(diagonal)
            .map(|(value, weight)| value.mul_real(weight).shr_trunc(self.p))
            .collect();
        checked_vector(&values, self.s, self.p)?;
        let p = Big::from_u64(self.p as u64);
        Ok(Approximation {
            values,
            precision: self.p,
            error_ulps: super::fastint::multiply(&p, &p),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent, quadratic defining Fourier sum, used ONLY as an oracle.
    // Each direct result has error <4ulp for a disk input: <2ulp from
    // phases and <sqrt(2)ulp from one componentwise truncation.
    fn direct_permuted_dft(input: &[Complex], p: usize, permutation: usize) -> Vec<Complex> {
        let n = input.len();
        let denominator = Big::from_u64(n as u64);
        let mut arithmetic = Arithmetic::new(p).unwrap();
        (0..n)
            .map(|j| {
                let mut sum = Complex::zero();
                for (k, value) in input.iter().enumerate() {
                    let exponent = (n - (permutation * j * k) % n) % n;
                    let phase = arithmetic
                        .phase(&Big::from_u64(exponent as u64), &denominator)
                        .unwrap();
                    sum = sum.add(&value.mul_exact(&phase));
                }
                Complex {
                    re: sum.re.div_trunc(&denominator),
                    im: sum.im.div_trunc(&denominator),
                }
                .shr_trunc(p)
            })
            .collect()
    }

    #[test]
    fn hypotheses_and_nearest_indices_are_exact() {
        let a = ResamplingPlan::new(128, 5, 8, 4).unwrap();
        assert_eq!(a.selected_indices(), &[0, 2, 3, 5, 6]);
        assert_eq!(a.neumann_terms(), 14);
        let b = ResamplingPlan::new(128, 7, 16, 4).unwrap();
        assert_eq!(b.selected_indices(), &[0, 2, 5, 7, 9, 11, 14]);
        assert!(ResamplingPlan::new(100, 5, 8, 4).is_err());
        assert!(ResamplingPlan::new(128, 4, 8, 4).is_err());
        assert!(ResamplingPlan::new(128, 5, 8, 12).is_err());
        assert!(ResamplingPlan::new(128, 127, 128, 2).is_err());
    }

    #[test]
    fn sparse_maps_preserve_disk_and_zero() {
        let mut plan = ResamplingPlan::new(104, 3, 8, 4).unwrap();
        let input = vec![Complex::zero(); 3];
        let expanded = plan.expand(&input).unwrap();
        assert_eq!(expanded.values, vec![Complex::zero(); 8]);
        let compressed = plan.compress(&expanded.values).unwrap();
        assert_eq!(compressed.values, input);
        assert_eq!(expanded.error_ulps, Big::from_u64(16 * 104));
        assert_eq!(compressed.error_ulps, Big::from_u64(104 * 104));
    }

    #[test]
    fn gaussian_transform_identity_matches_independent_fourier_sums() {
        let p = 128;
        let alpha = 4;
        for (s, t) in [(5, 8), (7, 16)] {
            let mut plan = ResamplingPlan::new(p, s, t, alpha).unwrap();
            let input: Vec<_> = (0..s)
                .map(|j| Complex {
                    re: Signed::from_i64((j % 3) as i64 - 1).shl(p - 4),
                    im: Signed::from_i64((j % 2) as i64 * 2 - 1).shl(p - 4),
                })
                .collect();
            let expanded = plan.expand(&input).unwrap();
            let middle = direct_permuted_dft(&expanded.values, p, (t - s % t) % t);
            let compressed = plan.compress(&middle).unwrap();
            let expected = direct_permuted_dft(&input, p, t % s);
            // Keep the finite, unsimplified bound. No asymptotic size margin
            // is assumed for these deliberately small, legal parameter sets.
            let bound = expanded
                .error_ulps
                .add(&compressed.error_ulps)
                .add_small(4)
                .shl(2 * alpha * alpha)
                .add_small(4);
            for (actual, reference) in compressed.values.iter().zip(expected) {
                let actual = actual.shl(2 * alpha * alpha);
                assert!(actual.re.sub(&reference.re).magnitude < bound);
                assert!(actual.im.sub(&reference.im).magnitude < bound);
            }
        }
    }
}
