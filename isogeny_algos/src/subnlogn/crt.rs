//! The coefficient-address isomorphism Phi from section 8.
//!
//! Payloads move through a padded mixed-radix scan, charged adjacent-axis
//! interchanges, and prefix-controlled cyclic rotations. No global scatter
//! to CRT destination indices is used. Forward triangular updates run from
//! the last axis to the first; inverse updates run in the reverse order.
//! Signed numerators are copied exactly, including widened final coefficients.

use super::certified::Complex;
use super::transforms;
use crate::field::is_prime;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Shape(&'static str),
    SourcePrime { axis: usize, value: usize },
    RepeatedPrime { axis: usize },
    SizeOverflow,
    Length { expected: usize, actual: usize },
    Movement(transforms::Error),
}
impl From<transforms::Error> for Error {
    fn from(error: transforms::Error) -> Self {
        Self::Movement(error)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CRT address layout: {self:?}")
    }
}
impl std::error::Error for Error {}

/// Counts the operations actually executed by this address procedure. Chunk
/// interchanges themselves call the concrete swap backend and its tape cost
/// accounting; they are not simulated by these counters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Receipt {
    pub chunk_swaps: usize,
    pub leading_bit_moves: usize,
    pub prefix_rotations: usize,
    pub linear_record_copies: u128,
}

pub struct CrtLayout {
    source: Vec<usize>,
    target: Vec<usize>,
    prefixes: Vec<usize>,
    inverses: Vec<usize>,
    source_volume: usize,
    target_volume: usize,
    p: usize,
}

fn product(values: &[usize]) -> Result<usize, Error> {
    values
        .iter()
        .try_fold(1usize, |a, &b| a.checked_mul(b).ok_or(Error::SizeOverflow))
}

fn modular_power(mut base: usize, mut exponent: usize, modulus: usize) -> usize {
    let mut value = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = ((value as u128 * base as u128) % modulus as u128) as usize;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as usize;
        exponent >>= 1;
    }
    value
}

fn increment(coordinates: &mut [usize], shape: &[usize], order: &[usize]) {
    for &axis in order.iter().rev() {
        coordinates[axis] += 1;
        if coordinates[axis] < shape[axis] {
            return;
        }
        coordinates[axis] = 0;
    }
}

impl CrtLayout {
    pub fn new(source: &[usize], target: &[usize], p: usize) -> Result<Self, Error> {
        if source.is_empty() || source.len() != target.len() {
            return Err(Error::Shape("matching nonempty axis lists required"));
        }
        if p < 2 {
            return Err(Error::Shape("precision must be at least two"));
        }
        let source_volume = product(source)?;
        let target_volume = product(target)?;
        let mut minimum_width = usize::MAX;
        let mut maximum_width = 0;
        let mut address_bits = 0usize;
        let mut prefixes = Vec::with_capacity(source.len());
        let mut inverses = Vec::with_capacity(source.len());
        let mut prefix = 1usize;
        for (axis, (&s, &t)) in source.iter().zip(target).enumerate() {
            if s < 3 || !is_prime(s as u64) {
                return Err(Error::SourcePrime { axis, value: s });
            }
            if source[..axis].contains(&s) {
                return Err(Error::RepeatedPrime { axis });
            }
            if !t.is_power_of_two() || t <= s {
                return Err(Error::Shape("each target must be a larger power of two"));
            }
            let width = t.trailing_zeros() as usize;
            minimum_width = minimum_width.min(width);
            maximum_width = maximum_width.max(width);
            address_bits = address_bits.checked_add(width).ok_or(Error::SizeOverflow)?;
            prefixes.push(prefix);
            // Distinct prime source lengths give Fermat inversion. This is
            // setup arithmetic on the O(p)-bit shape descriptor, not movement
            // of coefficient records.
            let inverse = modular_power(prefix % s, s - 2, s);
            if (prefix as u128 % s as u128) * inverse as u128 % s as u128 != 1 {
                return Err(Error::Shape("prefix inverse verification failed"));
            }
            inverses.push(inverse);
            prefix = prefix.checked_mul(s).ok_or(Error::SizeOverflow)?;
        }
        if maximum_width - minimum_width > 1 {
            return Err(Error::Shape(
                "target axis bit widths must differ by at most one",
            ));
        }
        if address_bits > p {
            return Err(Error::Shape(
                "address descriptor must fit in the precision bound",
            ));
        }
        Ok(Self {
            source: source.to_vec(),
            target: target.to_vec(),
            prefixes,
            inverses,
            source_volume,
            target_volume,
            p,
        })
    }

    pub fn source_volume(&self) -> usize {
        self.source_volume
    }
    pub fn target_volume(&self) -> usize {
        self.target_volume
    }
    pub fn coordinate_powers(&self) -> &[usize] {
        &self.inverses
    }

    /// Ordinary increasing coefficient order -> full padded tensor order.
    pub fn forward(&self, coefficients: &[Complex]) -> Result<Vec<Complex>, Error> {
        self.forward_with_receipt(coefficients)
            .map(|(values, _)| values)
    }

    /// Full padded tensor order -> ordinary increasing coefficient order.
    /// This accepts arbitrary signed numerator widths; it does not round or
    /// require disk inputs. The swap adapter sizes its exact payload words.
    pub fn inverse(&self, values: &[Complex]) -> Result<Vec<Complex>, Error> {
        self.inverse_with_receipt(values).map(|(values, _)| values)
    }

    pub fn forward_with_receipt(
        &self,
        coefficients: &[Complex],
    ) -> Result<(Vec<Complex>, Receipt), Error> {
        if coefficients.len() != self.source_volume {
            return Err(Error::Length {
                expected: self.source_volume,
                actual: coefficients.len(),
            });
        }
        let mut receipt = Receipt::default();
        let mut order: Vec<_> = (0..self.source.len()).rev().collect();
        let mut coordinates = vec![0; self.source.len()];
        let mut values = Vec::with_capacity(self.target_volume);
        let mut next = coefficients.iter();
        // Numerical k order has a_1 innermost; write padding with precisely
        // that nested enumeration, so the source itself is read once forward.
        for _ in 0..self.target_volume {
            if coordinates.iter().zip(&self.source).all(|(&a, &s)| a < s) {
                values.push(next.next().expect("validated mixed-radix volume").clone());
            } else {
                values.push(Complex::zero());
            }
            increment(&mut coordinates, &self.target, &order);
        }
        debug_assert!(next.next().is_none());
        receipt.linear_record_copies += self.target_volume as u128;
        let desired: Vec<_> = (0..self.source.len()).collect();
        self.reorder(&mut values, &mut order, &desired, &mut receipt)?;
        for axis in (0..self.source.len()).rev() {
            values = self.rotate_prefixes(&values, axis, false, &mut receipt)?;
        }
        Ok((values, receipt))
    }

    pub fn inverse_with_receipt(
        &self,
        input: &[Complex],
    ) -> Result<(Vec<Complex>, Receipt), Error> {
        if input.len() != self.target_volume {
            return Err(Error::Length {
                expected: self.target_volume,
                actual: input.len(),
            });
        }
        let mut receipt = Receipt::default();
        let mut values = input.to_vec();
        receipt.linear_record_copies += input.len() as u128;
        for axis in 0..self.source.len() {
            values = self.rotate_prefixes(&values, axis, true, &mut receipt)?;
        }
        let mut order: Vec<_> = (0..self.source.len()).collect();
        let desired: Vec<_> = order.iter().rev().copied().collect();
        self.reorder(&mut values, &mut order, &desired, &mut receipt)?;
        let mut coordinates = vec![0; self.source.len()];
        let mut result = Vec::with_capacity(self.source_volume);
        for value in values {
            if coordinates.iter().zip(&self.source).all(|(&a, &s)| a < s) {
                result.push(value);
            }
            increment(&mut coordinates, &self.target, &order);
        }
        receipt.linear_record_copies += self.target_volume as u128;
        debug_assert_eq!(result.len(), self.source_volume);
        Ok((result, receipt))
    }

    fn reorder(
        &self,
        values: &mut Vec<Complex>,
        order: &mut [usize],
        desired: &[usize],
        receipt: &mut Receipt,
    ) -> Result<(), Error> {
        let mut shape: Vec<_> = order.iter().map(|&axis| self.target[axis]).collect();
        for (position, &wanted) in desired.iter().enumerate() {
            let mut at = order
                .iter()
                .position(|&axis| axis == wanted)
                .expect("axis permutation");
            while at > position {
                let unequal = shape[at - 1] != shape[at];
                transforms::swap_neighbor_axes(values, &mut shape, at - 1, self.p)?;
                order.swap(at - 1, at);
                receipt.chunk_swaps += 1;
                receipt.leading_bit_moves += usize::from(unequal);
                // The equal-width part is charged by the swap backend. A
                // possible leading-bit transport is one constant-pass scan.
                if unequal {
                    receipt.linear_record_copies += 2 * self.target_volume as u128;
                }
                at -= 1;
            }
        }
        Ok(())
    }

    fn rotate_prefixes(
        &self,
        input: &[Complex],
        axis: usize,
        inverse: bool,
        receipt: &mut Receipt,
    ) -> Result<Vec<Complex>, Error> {
        let suffix = product(&self.target[axis + 1..])?;
        let block_length = self.target[axis]
            .checked_mul(suffix)
            .ok_or(Error::SizeOverflow)?;
        let valid_length = self.source[axis]
            .checked_mul(suffix)
            .ok_or(Error::SizeOverflow)?;
        let mut output = Vec::with_capacity(input.len());
        let mut controls = vec![0usize; axis];
        let control_order: Vec<_> = (0..axis).collect();
        for block in input.chunks_exact(block_length) {
            let valid_controls = controls.iter().zip(&self.source).all(|(&a, &s)| a < s);
            let offset = if valid_controls {
                let mut lower = 0usize;
                for (&digit, &prefix) in controls.iter().zip(&self.prefixes) {
                    lower = lower
                        .checked_add(digit.checked_mul(prefix).ok_or(Error::SizeOverflow)?)
                        .ok_or(Error::SizeOverflow)?;
                }
                let offset = ((lower as u128 * self.inverses[axis] as u128)
                    % self.source[axis] as u128) as usize;
                if inverse && offset != 0 {
                    self.source[axis] - offset
                } else {
                    offset
                }
            } else {
                0
            };
            if offset == 0 {
                output.extend_from_slice(block);
                receipt.linear_record_copies += block.len() as u128;
            } else {
                let cut = (self.source[axis] - offset)
                    .checked_mul(suffix)
                    .ok_or(Error::SizeOverflow)?;
                // Stream the first piece to a temporary tape, emit the second
                // piece, then drain the saved prefix. Spectator suffixes stay
                // intact. The invalid target interval is copied unchanged.
                let first = block[..cut].to_vec();
                output.extend_from_slice(&block[cut..valid_length]);
                output.extend(first);
                output.extend_from_slice(&block[valid_length..]);
                receipt.prefix_rotations += 1;
                receipt.linear_record_copies += (block.len() + cut) as u128;
            }
            increment(&mut controls, &self.target[..axis], &control_order);
        }
        debug_assert_eq!(output.len(), input.len());
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::super::certified::Signed;
    use super::*;

    fn records(n: usize) -> Vec<Complex> {
        (0..n)
            .map(|j| Complex {
                re: Signed::from_i64(j as i64 + 1),
                im: Signed::from_i64(-(j as i64) - 3),
            })
            .collect()
    }

    #[test]
    fn triangular_layout_matches_independent_crt_definition() {
        for (source, target) in [
            (vec![5, 7], vec![8, 16]),
            (vec![3, 5, 7], vec![8, 8, 8]),
            (vec![3, 5, 7], vec![4, 8, 8]),
        ] {
            let layout = CrtLayout::new(&source, &target, 128).unwrap();
            let input = records(layout.source_volume());
            let (actual, receipt) = layout.forward_with_receipt(&input).unwrap();
            let mut expected = vec![Complex::zero(); layout.target_volume()];
            // Test-only direct definition: x^k maps to coordinates mu_i*k mod s_i.
            for (k, value) in input.iter().enumerate() {
                let mut address = 0;
                for ((&s, &t), &mu) in source.iter().zip(&target).zip(layout.coordinate_powers()) {
                    let coordinate = ((mu as u128 * k as u128) % s as u128) as usize;
                    address = address * t + coordinate;
                }
                expected[address] = value.clone();
            }
            assert_eq!(actual, expected);
            assert_eq!(layout.inverse(&actual).unwrap(), input);
            assert!(receipt.chunk_swaps > 0);
            assert!(receipt.prefix_rotations > 0);
        }
    }

    #[test]
    fn inverse_preserves_wide_signed_payloads() {
        let layout = CrtLayout::new(&[5, 7], &[8, 16], 128).unwrap();
        let input: Vec<_> = records(35)
            .into_iter()
            .map(|value| value.shl(300))
            .collect();
        let values = layout.forward(&input).unwrap();
        assert_eq!(layout.inverse(&values).unwrap(), input);
    }

    #[test]
    fn padded_prefixes_and_invalid_target_intervals_are_fixed() {
        let layout = CrtLayout::new(&[3, 5], &[4, 8], 128).unwrap();
        let values = records(32);
        let mut receipt = Receipt::default();
        let moved = layout
            .rotate_prefixes(&values, 1, false, &mut receipt)
            .unwrap();
        assert_eq!(&moved[3 * 8..], &values[3 * 8..]);
        for prefix in 0..3 {
            assert_eq!(
                &moved[prefix * 8 + 5..(prefix + 1) * 8],
                &values[prefix * 8 + 5..(prefix + 1) * 8]
            );
        }
        assert_eq!(
            layout
                .rotate_prefixes(&moved, 1, true, &mut receipt)
                .unwrap(),
            values
        );
    }

    #[test]
    fn malformed_shapes_are_rejected() {
        assert!(CrtLayout::new(&[], &[], 128).is_err());
        assert!(CrtLayout::new(&[3, 3], &[4, 4], 128).is_err());
        assert!(CrtLayout::new(&[3, 9], &[8, 16], 128).is_err());
        assert!(CrtLayout::new(&[3, 5], &[4, 32], 128).is_err());
        assert!(CrtLayout::new(&[3, 5], &[4, 8], 2).is_err());
        let layout = CrtLayout::new(&[3], &[4], 128).unwrap();
        assert!(layout.forward(&[]).is_err());
        assert!(layout.inverse(&[]).is_err());
    }
}
