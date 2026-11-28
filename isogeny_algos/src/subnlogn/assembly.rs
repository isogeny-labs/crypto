//! Whole-integer assembly with explicit finite precision admission.
//!
//! The numerical path is sections 6--8 of the pinned September manuscript:
//! radix digits -> padded CRT -> Gaussian expansion -> chirp convolution ->
//! compression -> pointwise product -> opposite -> both S scales -> exact
//! coefficient rounding -> inverse CRT -> carries. `Plan` accepts explicit
//! finite parameters so this entire path can be tested without silently
//! selecting a conventional multiplication base case.

use super::certified::{Arithmetic, Complex, Signed};
use super::crt::CrtLayout;
use super::fastint::{bit_slice, multiply};
use super::resampling::ResamplingPlan;
use super::transforms::{self, Layout};
use crate::bigint::Big;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Parameters(&'static str),
    SizeOverflow,
    InputCapacity { digits: usize, available: usize },
    PrecisionMargin(&'static str),
    Scalar(super::certified::Error),
    Resampling(super::resampling::Error),
    Transform(transforms::Error),
    Crt(super::crt::Error),
    OutsideDisk { stage: &'static str, index: usize },
    CoefficientRange,
    CarryRange,
    LayerAdmission(super::tensor::TensorError),
}
impl From<super::certified::Error> for Error {
    fn from(e: super::certified::Error) -> Self {
        Self::Scalar(e)
    }
}
impl From<super::resampling::Error> for Error {
    fn from(e: super::resampling::Error) -> Self {
        Self::Resampling(e)
    }
}
impl From<transforms::Error> for Error {
    fn from(e: transforms::Error) -> Self {
        Self::Transform(e)
    }
}
impl From<super::crt::Error> for Error {
    fn from(e: super::crt::Error) -> Self {
        Self::Crt(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "integer assembly: {self:?}")
    }
}
impl std::error::Error for Error {}

/// Explicit parameters; these do not imply that the asymptotic prescription
/// or its eventual cutoff holds. Every correctness hypothesis is checked.
#[derive(Clone, Debug)]
pub struct Parameters {
    pub radix_bits: usize,
    pub precision: usize,
    pub source_lengths: Vec<usize>,
    pub target_lengths: Vec<usize>,
    pub alpha: usize,
    pub chunk_bits: usize,
}

/// Finite, integer-ULP bounds, without replacing an eventual inequality by an
/// assumption. In particular no `2*d*p^2 < T*log2(T)` simplification is used.
#[derive(Clone, Debug)]
pub struct PrecisionCertificate {
    pub precision: usize,
    pub gaussian_expansion_ulps: Big,
    pub ring_convolution_ulps: Big,
    pub chirp_ulps: Big,
    pub source_transform_ulps: Big,
    pub first_scale_ulps: Big,
    pub final_coefficient_ulps: Big,
}

#[derive(Clone, Debug, Default)]
pub struct Execution {
    pub source_transforms: usize,
    pub gaussian_expansions: usize,
    pub gaussian_compressions: usize,
    pub cyclic_convolutions: usize,
    pub normalized_polynomial_products: usize,
    pub synthetic_rounds: usize,
    pub simultaneous_layer_calls: usize,
    pub integer_coefficients: usize,
    pub source_volume_factors: usize,
    pub layer_receipts: Vec<super::tensor::LayerReceipt>,
    pub transform_swap_receipts: Vec<super::swap::SwapReceipt>,
}

#[derive(Debug)]
pub struct Product {
    pub value: Big,
    pub execution: Execution,
    pub precision: PrecisionCertificate,
}

pub struct Plan {
    parameters: Parameters,
    source_volume: usize,
    target_volume: usize,
    gamma: usize,
    crt: CrtLayout,
    resampling: Vec<ResamplingPlan>,
    synthetic: Layout,
    twists: Vec<Complex>,
    chirps: Vec<Complex>,
    certificate: PrecisionCertificate,
}

fn big(n: usize) -> Big {
    Big::from_u64(n as u64)
}
fn pow2(bits: usize) -> Big {
    Big::from_u64(1).shl(bits)
}
fn volume(shape: &[usize]) -> Result<usize, Error> {
    shape
        .iter()
        .try_fold(1usize, |a, &b| a.checked_mul(b).ok_or(Error::SizeOverflow))
}

fn disk(values: &[Complex], p: usize, stage: &'static str) -> Result<(), Error> {
    for (index, value) in values.iter().enumerate() {
        if !value.is_disk(p) {
            return Err(Error::OutsideDisk { stage, index });
        }
    }
    Ok(())
}

impl Plan {
    pub fn new(parameters: Parameters) -> Result<Self, Error> {
        let p = parameters.precision;
        let b = parameters.radix_bits;
        let d = parameters.source_lengths.len();
        if d < 2 || d != parameters.target_lengths.len() {
            return Err(Error::Parameters(
                "at least two matching source/target axes required",
            ));
        }
        if b == 0 || b.checked_add(2).ok_or(Error::SizeOverflow)? > p {
            return Err(Error::Parameters(
                "radix digits must embed exactly in the precision grid",
            ));
        }
        let source_volume = volume(&parameters.source_lengths)?;
        let target_volume = volume(&parameters.target_lengths)?;
        target_volume
            .checked_mul(p.checked_add(2).ok_or(Error::SizeOverflow)?)
            .and_then(|v| v.checked_mul(2))
            .ok_or(Error::SizeOverflow)?;
        super::tensor::check_layer_admission(d - 1, d, parameters.chunk_bits, p)
            .map_err(Error::LayerAdmission)?;
        let r = *parameters
            .target_lengths
            .last()
            .ok_or(Error::Parameters("missing ring axis"))?;
        let synthetic = Layout::new(
            &parameters.target_lengths[..d - 1],
            r,
            p,
            parameters.chunk_bits,
        )?;
        let crt = CrtLayout::new(&parameters.source_lengths, &parameters.target_lengths, p)?;
        let mut resampling = Vec::with_capacity(d);
        for (&s, &t) in parameters
            .source_lengths
            .iter()
            .zip(&parameters.target_lengths)
        {
            resampling.push(ResamplingPlan::new(p, s, t, parameters.alpha)?);
        }
        let gamma = parameters
            .alpha
            .checked_mul(parameters.alpha)
            .and_then(|v| v.checked_mul(d))
            .and_then(|v| v.checked_mul(2))
            .ok_or(Error::SizeOverflow)?;
        let ell = r.trailing_zeros() as usize;
        let gaussian_expansion_ulps = big(d).mul_small(16).mul_small(p as u64);
        // Each transform has error <sqrt(2)*ell, bounded here by 2*ell.
        let ring_convolution_ulps = big(target_volume / r).mul_small((6 * ell + 2) as u64);
        let chirp_ulps = ring_convolution_ulps.add_small(22);
        let gaussian_compression_ulps = multiply(&big(p), &big(p)).mul_small(d as u64);
        let source_transform_ulps = gaussian_expansion_ulps
            .add(&gaussian_compression_ulps)
            .add(&chirp_ulps)
            .shl(gamma);
        let first_scale_ulps = source_transform_ulps
            .mul_small(3)
            .add_small(2)
            .mul_small(source_volume as u64);
        let input_scale = b
            .checked_mul(2)
            .and_then(|v| v.checked_add(4))
            .ok_or(Error::SizeOverflow)?;
        let final_coefficient_ulps = first_scale_ulps
            .mul_small(source_volume as u64)
            .shl(input_scale);
        let unit = pow2(p);
        if gaussian_expansion_ulps.mul_small(100) >= unit {
            return Err(Error::PrecisionMargin(
                "expansion needs a strict 0.01 disk margin",
            ));
        }
        if ring_convolution_ulps.mul_small(1000) >= unit {
            return Err(Error::PrecisionMargin(
                "ring convolution needs a strict 0.001 disk margin",
            ));
        }
        if source_transform_ulps.mul_small(100) >= unit {
            return Err(Error::PrecisionMargin(
                "source transform needs a strict 0.01 disk margin",
            ));
        }
        if first_scale_ulps.mul_small(1000) >= unit {
            return Err(Error::PrecisionMargin(
                "first S scale needs a strict 0.001 disk margin",
            ));
        }
        if final_coefficient_ulps.mul_small(2) >= unit {
            return Err(Error::PrecisionMargin(
                "final absolute coefficient error must be strictly below 1/2",
            ));
        }
        let certificate = PrecisionCertificate {
            precision: p,
            gaussian_expansion_ulps,
            ring_convolution_ulps,
            chirp_ulps,
            source_transform_ulps,
            first_scale_ulps,
            final_coefficient_ulps,
        };
        let period = r.checked_mul(2).ok_or(Error::SizeOverflow)?;
        let mut arithmetic = Arithmetic::new(p)?;
        let denominator = big(period);
        let mut twists = Vec::with_capacity(r);
        for k in 0..r {
            twists.push(arithmetic.phase(&big(k), &denominator)?);
        }
        let mut chirps = Vec::with_capacity(target_volume);
        for address in 0..target_volume {
            let mut remaining = address;
            let mut sum = 0usize;
            for (&s, &t) in parameters
                .source_lengths
                .iter()
                .zip(&parameters.target_lengths)
                .rev()
            {
                let j = remaining % t;
                remaining /= t;
                let square = (j as u128 * j as u128) % period as u128;
                let coefficient = (s as u128 * (r / t) as u128) % period as u128;
                sum = ((sum as u128 + coefficient * square) % period as u128) as usize;
            }
            let exponent = if sum == 0 { 0 } else { period - sum };
            chirps.push(arithmetic.phase(&big(exponent), &denominator)?);
        }
        Ok(Self {
            parameters,
            source_volume,
            target_volume,
            gamma,
            crt,
            resampling,
            synthetic,
            twists,
            chirps,
            certificate,
        })
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
    pub fn certificate(&self) -> &PrecisionCertificate {
        &self.certificate
    }

    fn gaussian_axes(
        &mut self,
        mut values: Vec<Complex>,
        compress: bool,
        execution: &mut Execution,
    ) -> Result<Vec<Complex>, Error> {
        let d = self.parameters.source_lengths.len();
        let mut order: Vec<_> = (0..d).collect();
        let mut shape = self.parameters.target_lengths.clone();
        let mut valid = if compress {
            self.parameters.target_lengths.clone()
        } else {
            self.parameters.source_lengths.clone()
        };
        for axis in 0..d {
            let at = order
                .iter()
                .position(|&i| i == axis)
                .ok_or(Error::Parameters("axis order"))?;
            for pos in at..d - 1 {
                transforms::swap_neighbor_axes(
                    &mut values,
                    &mut shape,
                    pos,
                    self.parameters.precision,
                )?;
                order.swap(pos, pos + 1);
            }
            let t = self.parameters.target_lengths[axis];
            let s = self.parameters.source_lengths[axis];
            let mut next = Vec::with_capacity(self.target_volume);
            for (row_index, row) in values.chunks_exact(t).enumerate() {
                let mut index = row_index;
                let mut permitted = true;
                for (&other, &length) in order[..d - 1].iter().zip(&shape[..d - 1]).rev() {
                    if index % length >= valid[other] {
                        permitted = false;
                    }
                    index /= length;
                }
                if !permitted {
                    next.resize(next.len() + t, Complex::zero());
                    continue;
                }
                if compress {
                    let result = self.resampling[axis].compress(row)?;
                    next.extend(result.values);
                    next.resize(next.len() + t - s, Complex::zero());
                    execution.gaussian_compressions += 1;
                } else {
                    let result = self.resampling[axis].expand(&row[..s])?;
                    next.extend(result.values);
                    execution.gaussian_expansions += 1;
                }
            }
            values = next;
            valid[axis] = if compress { s } else { t };
        }
        if order != (0..d).collect::<Vec<_>>() {
            return Err(Error::Parameters("axis transport did not restore order"));
        }
        Ok(values)
    }

    fn cyclic_convolve(
        &self,
        a: &[Complex],
        b: &[Complex],
        execution: &mut Execution,
    ) -> Result<Vec<Complex>, Error> {
        let p = self.parameters.precision;
        let r = self.twists.len();
        let twisted_a: Vec<_> = a
            .iter()
            .enumerate()
            .map(|(i, x)| x.mul_grid(&self.twists[i % r], p))
            .collect();
        let twisted_b: Vec<_> = b
            .iter()
            .enumerate()
            .map(|(i, x)| x.mul_grid(&self.twists[i % r], p))
            .collect();
        let (mut result, receipt) = self.synthetic.convolve(&twisted_a, &twisted_b)?;
        disk(&result, p, "normalized ring convolution")?;
        for (i, x) in result.iter_mut().enumerate() {
            *x = x.mul_grid(&self.twists[i % r].conj(), p);
        }
        execution.cyclic_convolutions += 1;
        execution.normalized_polynomial_products += receipt.polynomial_products;
        execution.synthetic_rounds += receipt.completed_rounds;
        execution.simultaneous_layer_calls += receipt.simultaneous_layers;
        execution.layer_receipts.extend(receipt.layer_receipts);
        execution
            .transform_swap_receipts
            .extend(receipt.swap_receipts);
        Ok(result)
    }

    fn source_transform(
        &mut self,
        input: Vec<Complex>,
        opposite: bool,
        execution: &mut Execution,
    ) -> Result<Vec<Complex>, Error> {
        let p = self.parameters.precision;
        let input = if opposite {
            input.into_iter().map(|x| x.conj()).collect()
        } else {
            input
        };
        let expanded = self.gaussian_axes(input, false, execution)?;
        let endpoints: Vec<_> = expanded
            .iter()
            .zip(&self.chirps)
            .map(|(x, a)| x.mul_grid(&a.conj(), p))
            .collect();
        let convolution = self.cyclic_convolve(&self.chirps, &endpoints, execution)?;
        let chirped: Vec<_> = convolution
            .iter()
            .zip(&self.chirps)
            .map(|(x, a)| x.mul_grid(&a.conj(), p))
            .collect();
        disk(&chirped, p, "chirp output")?;
        let mut result = self.gaussian_axes(chirped, true, execution)?;
        for value in &mut result {
            *value = value.shl(self.gamma);
            if opposite {
                *value = value.conj();
            }
        }
        disk(&result, p, "scaled source transform")?;
        execution.source_transforms += 1;
        Ok(result)
    }

    /// Execute the complete numerical assembly on these explicitly admitted
    /// parameters. The input capacity is imposed by zero-padding, not by a
    /// hidden fallback or transform substitution.
    pub fn multiply(&mut self, a: &Big, b: &Big) -> Result<Product, Error> {
        let radix = self.parameters.radix_bits;
        let p = self.parameters.precision;
        let digits = a.bits().max(b.bits()).div_ceil(radix);
        let capacity = self.source_volume.div_ceil(2);
        if digits > capacity {
            return Err(Error::InputCapacity {
                digits,
                available: capacity,
            });
        }
        let encode = |input: &Big| {
            let mut values = Vec::with_capacity(self.source_volume);
            for j in 0..self.source_volume {
                values.push(Complex::real(
                    Signed::from_big(bit_slice(input, j * radix, radix)).shl(p - radix - 2),
                ));
            }
            values
        };
        let u = self.crt.forward(&encode(a))?;
        let v = self.crt.forward(&encode(b))?;
        let mut execution = Execution::default();
        let u = self.source_transform(u, false, &mut execution)?;
        let v = self.source_transform(v, false, &mut execution)?;
        let pointwise: Vec<_> = u.iter().zip(&v).map(|(a, b)| a.mul_grid(b, p)).collect();
        let mut result = self.source_transform(pointwise, true, &mut execution)?;
        // First S: recover (u*v)/S. Second S: undo the remaining normalization.
        for value in &mut result {
            value.re = value.re.mul_small(self.source_volume as u64);
            value.im = value.im.mul_small(self.source_volume as u64);
        }
        execution.source_volume_factors += 1;
        disk(&result, p, "first source-volume scale")?;
        for value in &mut result {
            value.re = value
                .re
                .mul_small(self.source_volume as u64)
                .shl(2 * radix + 4);
            value.im = value
                .im
                .mul_small(self.source_volume as u64)
                .shl(2 * radix + 4);
        }
        execution.source_volume_factors += 1;
        // CRT only permutes fixed records, so restore addresses before rounding.
        let result = self.crt.inverse(&result)?;
        let half = pow2(p - 1);
        let coefficient_max = big(digits).mul_small(1); // q*(2^b-1)^2
        let radix_max = pow2(radix).sub_small(1);
        let coefficient_max = multiply(&coefficient_max, &multiply(&radix_max, &radix_max));
        let mut coefficients = Vec::with_capacity(self.source_volume);
        for value in result {
            let rounded = value.re.magnitude.add(&half).shr(p);
            if (value.re.negative && !rounded.is_zero()) || rounded > coefficient_max {
                return Err(Error::CoefficientRange);
            }
            // A nonzero imaginary residual is allowed, but must fit the same
            // certified absolute-error interval around the true real integer.
            if value.im.magnitude >= self.certificate.final_coefficient_ulps {
                return Err(Error::CoefficientRange);
            }
            coefficients.push(rounded);
        }
        execution.integer_coefficients = coefficients.len();
        let value = carry_digits(&coefficients, radix, digits)?;
        Ok(Product {
            value,
            execution,
            precision: self.certificate.clone(),
        })
    }
}

fn carry_digits(coefficients: &[Big], radix: usize, digits: usize) -> Result<Big, Error> {
    let max_bits = coefficients
        .len()
        .checked_add(2)
        .and_then(|n| n.checked_mul(radix))
        .ok_or(Error::SizeOverflow)?;
    let mut out = vec![0u64; max_bits.div_ceil(64)];
    let mut carry = Big::zero();
    let bound = big(digits).mul_small(1);
    let bound = multiply(&bound, &pow2(radix).sub_small(1));
    for (j, coefficient) in coefficients.iter().enumerate() {
        let total = coefficient.add(&carry);
        let digit = bit_slice(&total, 0, radix);
        let offset = j * radix;
        for (k, &limb) in digit.0.iter().enumerate() {
            out[offset / 64 + k] |= limb << (offset % 64);
            if !offset.is_multiple_of(64) {
                out[offset / 64 + k + 1] |= limb >> (64 - offset % 64);
            }
        }
        carry = total.shr(radix);
        if carry >= bound && !carry.is_zero() {
            return Err(Error::CarryRange);
        }
    }
    if !carry.is_zero() {
        return Err(Error::CarryRange);
    }
    Ok(Big::from_limbs(&out))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn toy() -> Parameters {
        Parameters {
            radix_bits: 8,
            precision: 128,
            source_lengths: vec![5, 7],
            target_lengths: vec![8, 16],
            alpha: 4,
            chunk_bits: 1,
        }
    }

    #[test]
    fn finite_admission_rejects_insufficient_precision() {
        let plan = Plan::new(toy()).unwrap();
        assert!(plan.certificate.final_coefficient_ulps.mul_small(2) < pow2(128));
        let mut too_low = toy();
        too_low.precision = 104;
        assert!(matches!(Plan::new(too_low), Err(Error::PrecisionMargin(_))));
    }

    #[test]
    fn whole_integer_flow_requires_all_stages_and_both_scales() {
        let mut plan = Plan::new(toy()).unwrap();
        for (a, b) in [
            (0, 0),
            (0, 12345),
            (1, 1),
            (255, 257),
            (u64::MAX as u128, 0x123456789abcdef),
            (u128::MAX, u128::MAX),
            (1u128 << 127, (1u128 << 126) + 1),
        ] {
            let a = Big::from_u128(a);
            let b = Big::from_u128(b);
            let product = plan.multiply(&a, &b).unwrap();
            assert_eq!(product.value, a.mul(&b));
            assert_eq!(product.execution.source_transforms, 3);
            assert_eq!(product.execution.source_volume_factors, 2);
            assert_eq!(product.execution.cyclic_convolutions, 3);
            assert!(product.execution.normalized_polynomial_products > 0);
            assert!(product.execution.gaussian_expansions > 0);
            assert!(product.execution.gaussian_compressions > 0);
            assert!(product.execution.simultaneous_layer_calls > 0);
        }
    }
}
