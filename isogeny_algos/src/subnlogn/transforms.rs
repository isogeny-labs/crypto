//! Complex synthetic transforms with retained layout, manuscript §6.
//!
//! Every round completes all participating normalized butterflies before one
//! truncation. Polynomial twiddles are exact signed two-piece rotations. The
//! named-slot layout is constructed by linear single-bit moves and the concrete
//! rank-saving chunk-swap engine; frequency records remain in that layout
//! through pointwise products and the opposite transform.

use super::certified::Complex;
use super::ring;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Shape,
    Precision,
    SizeOverflow,
    OutsideDisk { index: usize },
    Backend(String),
    Ring(ring::Error),
}
impl From<ring::Error> for Error {
    fn from(e: ring::Error) -> Self {
        Self::Ring(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "synthetic complex transform: {self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Debug)]
enum Movement {
    Move {
        from: usize,
        to: usize,
    },
    Swap {
        first: usize,
        second: usize,
        width: usize,
    },
}

#[derive(Clone, Debug)]
pub struct Layout {
    axes: Vec<usize>,
    r: usize,
    p: usize,
    chunk_bits: usize,
    low_bits: usize,
    slots: Vec<(usize, usize)>,
    movements: Vec<Movement>,
    records: usize,
}

#[derive(Debug, Default)]
pub struct Receipt {
    pub completed_rounds: usize,
    pub individual_axis_kernels: usize,
    pub simultaneous_layers: usize,
    pub chunk_swaps: usize,
    pub polynomial_products: usize,
    pub layer_receipts: Vec<super::tensor::LayerReceipt>,
    pub swap_receipts: Vec<super::swap::SwapReceipt>,
}

fn power2(bits: usize) -> Result<usize, Error> {
    1usize
        .checked_shl(u32::try_from(bits).map_err(|_| Error::SizeOverflow)?)
        .ok_or(Error::SizeOverflow)
}

fn check_disk(values: &[Complex], p: usize) -> Result<(), Error> {
    for (index, x) in values.iter().enumerate() {
        if !x.is_disk(p) {
            return Err(Error::OutsideDisk { index });
        }
    }
    Ok(())
}

impl Layout {
    pub fn new(axes: &[usize], r: usize, p: usize, chunk_bits: usize) -> Result<Self, Error> {
        if r < 4 || !r.is_power_of_two() || axes.is_empty() {
            return Err(Error::Shape);
        }
        let ell = r.trailing_zeros() as usize;
        if ell >= p || chunk_bits == 0 || chunk_bits >= ell {
            return Err(Error::Precision);
        }
        if axes.iter().any(|&t| t != r && t != r / 2) {
            return Err(Error::Shape);
        }
        let records = axes
            .iter()
            .try_fold(1usize, |v, &t| v.checked_mul(t).ok_or(Error::SizeOverflow))?;
        records.checked_mul(r).ok_or(Error::SizeOverflow)?;
        let mut slots = Vec::new();
        for (i, &t) in axes.iter().enumerate() {
            for h in (0..t.trailing_zeros() as usize).rev() {
                slots.push((i, h));
            }
        }
        let low_bits = (ell - 1) % chunk_bits;
        let mut prefix = Vec::new();
        for (i, &t) in axes.iter().enumerate() {
            if t == r {
                prefix.push((i, ell - 1));
            }
        }
        for h in (0..low_bits).rev() {
            for i in 0..axes.len() {
                prefix.push((i, h));
            }
        }
        let mut movements = Vec::new();
        for (to, slot) in prefix.iter().enumerate() {
            let from = slots.iter().position(|x| x == slot).ok_or(Error::Shape)?;
            if from != to {
                movements.push(Movement::Move { from, to });
                let value = slots.remove(from);
                slots.insert(to, value);
            }
        }
        let chunks = (ell - 1) / chunk_bits;
        for j in 0..chunks {
            for i in 0..axes.len() {
                let first = prefix.len() + (j * axes.len() + i) * chunk_bits;
                let high = ell - 2 - j * chunk_bits;
                let second = slots
                    .iter()
                    .position(|&x| x == (i, high))
                    .ok_or(Error::Shape)?;
                if first != second {
                    if first > second {
                        return Err(Error::Shape);
                    }
                    movements.push(Movement::Swap {
                        first,
                        second,
                        width: chunk_bits,
                    });
                    for k in 0..chunk_bits {
                        slots.swap(first + k, second + k);
                    }
                }
            }
        }
        Ok(Self {
            axes: axes.to_vec(),
            r,
            p,
            chunk_bits,
            low_bits,
            slots,
            movements,
            records,
        })
    }

    pub fn record_count(&self) -> usize {
        self.records
    }
    pub fn ring_degree(&self) -> usize {
        self.r
    }
    pub fn precision(&self) -> usize {
        self.p
    }

    fn movement(
        &self,
        values: &mut Vec<Complex>,
        movement: &Movement,
        reverse: bool,
        receipt: &mut Receipt,
    ) -> Result<(), Error> {
        match *movement {
            Movement::Move { from, to } => {
                let (from, to) = if reverse { (to, from) } else { (from, to) };
                move_bit(values, self.r, self.slots.len(), from, to)?;
            }
            Movement::Swap {
                first,
                second,
                width,
            } => {
                let work = super::swap::swap_record_chunks(
                    values,
                    first,
                    width,
                    second - first - width,
                    self.slots.len() - second - width,
                    self.r,
                    self.p,
                )
                .map_err(|e| Error::Backend(format!("chunk swap: {e}")))?;
                receipt.swap_receipts.push(work);
                receipt.chunk_swaps += 1;
            }
        }
        Ok(())
    }

    fn coordinate(&self, address: usize, axis: usize) -> usize {
        let mut coordinate = 0;
        for (position, &(i, h)) in self.slots.iter().enumerate() {
            if i == axis && address & (1 << (self.slots.len() - position - 1)) != 0 {
                coordinate |= 1 << h;
            }
        }
        coordinate
    }

    fn monomials(&self, values: &mut Vec<Complex>, h: usize, opposite: bool) -> Result<(), Error> {
        let period = self.r.checked_mul(2).ok_or(Error::SizeOverflow)?;
        let u = power2(h + 1)?;
        let mut output = Vec::with_capacity(values.len());
        for (address, record) in values.chunks_exact(self.r).enumerate() {
            let mut exponent = 0usize;
            for (axis, &t) in self.axes.iter().enumerate() {
                if t < u {
                    continue;
                }
                let coordinate = self.coordinate(address, axis);
                if coordinate & (u / 2) != 0 {
                    let k = coordinate & (u / 2 - 1);
                    exponent = ((exponent as u128 + (period / u) as u128 * k as u128)
                        % period as u128) as usize;
                }
            }
            // Forward exponent is negative; the opposite uses its inverse.
            if !opposite && exponent != 0 {
                exponent = period - exponent;
            }
            let shift = exponent % self.r;
            let negate = exponent >= self.r;
            for value in &record[self.r - shift..] {
                output.push(if negate { value.clone() } else { value.neg() });
            }
            for value in &record[..self.r - shift] {
                output.push(if negate { value.neg() } else { value.clone() });
            }
        }
        *values = output;
        Ok(())
    }

    fn butterflies(
        &self,
        values: &mut Vec<Complex>,
        h: usize,
        receipt: &mut Receipt,
    ) -> Result<(), Error> {
        let ell = self.r.trailing_zeros() as usize;
        if h < self.low_bits || h == ell - 1 {
            // A prefix round is evaluated exactly at grid p+#axes, followed
            // by a single truncation, as prescribed in the source proof.
            let mut participating = 0;
            for (axis, &t) in self.axes.iter().enumerate() {
                if t <= (1usize << h) {
                    continue;
                }
                let position = self
                    .slots
                    .iter()
                    .position(|&s| s == (axis, h))
                    .ok_or(Error::Shape)?;
                move_bit(
                    values,
                    self.r,
                    self.slots.len(),
                    position,
                    self.slots.len() - 1,
                )?;
                for pair in values.chunks_exact_mut(2 * self.r) {
                    let (left, right) = pair.split_at_mut(self.r);
                    for (a, b) in left.iter_mut().zip(right) {
                        let sum = a.add(b);
                        let difference = a.sub(b);
                        *a = sum;
                        *b = difference;
                    }
                }
                move_bit(
                    values,
                    self.r,
                    self.slots.len(),
                    self.slots.len() - 1,
                    position,
                )?;
                participating += 1;
            }
            for x in values.iter_mut() {
                *x = x.shr_trunc(participating);
            }
            receipt.individual_axis_kernels += participating;
        } else {
            let rho = (h - self.low_bits) % self.chunk_bits;
            let position = self
                .slots
                .iter()
                .position(|&s| s == (0, h))
                .ok_or(Error::Shape)?;
            let prefix_bits = position - (self.chunk_bits - 1 - rho);
            let suffix_bits = self.slots.len() - prefix_bits - self.axes.len() * self.chunk_bits;
            let work = super::tensor::normalized_layer_values(
                values,
                power2(prefix_bits)?,
                power2(suffix_bits)?,
                self.axes.len(),
                self.chunk_bits,
                rho,
                self.r,
                self.p,
            )
            .map_err(|e| Error::Backend(format!("simultaneous layer: {e}")))?;
            receipt.layer_receipts.push(work);
            receipt.simultaneous_layers += 1;
        }
        check_disk(values, self.p)
    }

    /// Forward is L B F^-; opposite consumes that same layout and computes
    /// F^+ B^-1 L^-1. Neither transform is an unnormalized inverse.
    pub fn apply(&self, values: &mut Vec<Complex>, opposite: bool) -> Result<Receipt, Error> {
        if values.len() != self.records * self.r {
            return Err(Error::Shape);
        }
        check_disk(values, self.p)?;
        let mut receipt = Receipt::default();
        let ell = self.r.trailing_zeros() as usize;
        if opposite {
            for h in 0..ell {
                if !self.axes.iter().any(|&t| t > (1usize << h)) {
                    continue;
                }
                self.monomials(values, h, true)?;
                self.butterflies(values, h, &mut receipt)?;
                receipt.completed_rounds += 1;
            }
            for movement in self.movements.iter().rev() {
                self.movement(values, movement, true, &mut receipt)?;
            }
        } else {
            for movement in &self.movements {
                self.movement(values, movement, false, &mut receipt)?;
            }
            for h in (0..ell).rev() {
                if !self.axes.iter().any(|&t| t > (1usize << h)) {
                    continue;
                }
                self.butterflies(values, h, &mut receipt)?;
                self.monomials(values, h, false)?;
                receipt.completed_rounds += 1;
            }
        }
        check_disk(values, self.p)?;
        Ok(receipt)
    }

    /// Return the normalized negacyclic/cyclic convolution (f*g)/(r*M).
    pub fn convolve(&self, a: &[Complex], b: &[Complex]) -> Result<(Vec<Complex>, Receipt), Error> {
        let mut left = a.to_vec();
        let mut right = b.to_vec();
        let mut total = self.apply(&mut left, false)?;
        let b_receipt = self.apply(&mut right, false)?;
        total.completed_rounds += b_receipt.completed_rounds;
        total.simultaneous_layers += b_receipt.simultaneous_layers;
        total.individual_axis_kernels += b_receipt.individual_axis_kernels;
        total.chunk_swaps += b_receipt.chunk_swaps;
        total.layer_receipts.extend(b_receipt.layer_receipts);
        total.swap_receipts.extend(b_receipt.swap_receipts);
        let mut products = Vec::with_capacity(left.len());
        for (a, b) in left.chunks_exact(self.r).zip(right.chunks_exact(self.r)) {
            products.extend(ring::normalized_product(a, b, self.p)?);
            total.polynomial_products += 1;
        }
        let opposite = self.apply(&mut products, true)?;
        total.completed_rounds += opposite.completed_rounds;
        total.simultaneous_layers += opposite.simultaneous_layers;
        total.individual_axis_kernels += opposite.individual_axis_kernels;
        total.chunk_swaps += opposite.chunk_swaps;
        total.layer_receipts.extend(opposite.layer_receipts);
        total.swap_receipts.extend(opposite.swap_receipts);
        let scale = self.records.trailing_zeros() as usize;
        for value in &mut products {
            *value = value.shl(scale);
        }
        Ok((products, total))
    }
}

/// Stable single-bit movement using two monotone subsequence streams. This
/// does not implement the sublinear-exponent bulk chunk interchange.
pub(crate) fn move_bit(
    values: &mut Vec<Complex>,
    record: usize,
    bits: usize,
    from: usize,
    to: usize,
) -> Result<(), Error> {
    if from >= bits || to >= bits {
        return Err(Error::Shape);
    }
    if from == to {
        return Ok(());
    }
    let mut output = Vec::with_capacity(values.len());
    if from > to {
        let block = power2(bits - to)?;
        let mask = power2(bits - from - 1)?;
        for chunk in values.chunks_exact(block * record) {
            for selected in [false, true] {
                for (index, value) in chunk.chunks_exact(record).enumerate() {
                    if (index & mask != 0) == selected {
                        output.extend_from_slice(value);
                    }
                }
            }
        }
    } else {
        let block = power2(bits - from)?;
        let group = power2(bits - to - 1)? * record;
        for chunk in values.chunks_exact(block * record) {
            let (left, right) = chunk.split_at(chunk.len() / 2);
            for (a, b) in left.chunks_exact(group).zip(right.chunks_exact(group)) {
                output.extend_from_slice(a);
                output.extend_from_slice(b);
            }
        }
    }
    if output.len() != values.len() {
        return Err(Error::Shape);
    }
    *values = output;
    Ok(())
}

/// Exchange adjacent power-of-two axes whose logarithms differ by at most
/// one. Their common bits use one full chunk swap; the possible extra bit
/// uses one linear stable move. This is also the resampling axis transport.
pub(crate) fn swap_neighbor_axes(
    values: &mut Vec<Complex>,
    shape: &mut [usize],
    axis: usize,
    p: usize,
) -> Result<(), Error> {
    if axis + 1 >= shape.len() || shape.iter().any(|x| !x.is_power_of_two()) {
        return Err(Error::Shape);
    }
    let widths: Vec<_> = shape.iter().map(|x| x.trailing_zeros() as usize).collect();
    let a = widths[axis];
    let b = widths[axis + 1];
    if a.abs_diff(b) > 1 {
        return Err(Error::Shape);
    }
    let prefix = widths[..axis].iter().sum::<usize>();
    let suffix = widths[axis + 2..].iter().sum::<usize>();
    let bits = prefix + a + b + suffix;
    if a == b {
        super::swap::swap_record_chunks(values, prefix, a, 0, suffix, 1, p)
            .map_err(|e| Error::Backend(format!("axis interchange: {e}")))?;
    } else if a > b {
        super::swap::swap_record_chunks(values, prefix + 1, b, 0, suffix, 1, p)
            .map_err(|e| Error::Backend(format!("axis interchange: {e}")))?;
        move_bit(values, 1, bits, prefix, prefix + b)?;
    } else {
        move_bit(values, 1, bits, prefix + a, prefix)?;
        super::swap::swap_record_chunks(values, prefix + 1, a, 0, suffix, 1, p)
            .map_err(|e| Error::Backend(format!("axis interchange: {e}")))?;
    }
    shape.swap(axis, axis + 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::certified::Signed;
    use super::*;

    #[test]
    fn named_layout_example_and_single_bit_inverse() {
        let layout = Layout::new(&[64, 32], 64, 128, 2).unwrap();
        assert_eq!(
            layout.slots,
            vec![
                (0, 5),
                (0, 0),
                (1, 0),
                (0, 4),
                (0, 3),
                (1, 4),
                (1, 3),
                (0, 2),
                (0, 1),
                (1, 2),
                (1, 1)
            ]
        );
        for from in 0..5 {
            for to in 0..5 {
                let original: Vec<_> = (0..32)
                    .map(|j| Complex::real(Signed::from_i64(j)))
                    .collect();
                let mut moved = original.clone();
                move_bit(&mut moved, 1, 5, from, to).unwrap();
                move_bit(&mut moved, 1, 5, to, from).unwrap();
                assert_eq!(moved, original);
            }
        }
    }

    #[test]
    fn retained_multiaxis_layout_and_opposite_normalization() {
        for (axes, r, k) in [(vec![4, 8], 8, 1), (vec![8, 16], 16, 2)] {
            let p = 80;
            let layout = Layout::new(&axes, r, p, k).unwrap();
            let mut values = vec![Complex::zero(); layout.records * r];
            for (j, value) in values.iter_mut().enumerate() {
                *value = Complex {
                    re: Signed::from_i64(j as i64 % 7 - 3).shl(60),
                    im: Signed::from_i64(j as i64 % 3 - 1).shl(60),
                };
            }
            let original = values.clone();
            let forward = layout.apply(&mut values, false).unwrap();
            let opposite = layout.apply(&mut values, true).unwrap();
            assert!(forward.simultaneous_layers > 0);
            assert_eq!(forward.completed_rounds, opposite.completed_rounds);
            let normalization = layout.records.trailing_zeros() as usize;
            // Inputs have enough trailing zeros for all exact dyadic rounds.
            for (got, input) in values.iter().zip(&original) {
                assert_eq!(*got, input.shr_trunc(normalization));
            }
        }
    }

    #[test]
    fn normalized_convolution_matches_sparse_signed_wrap_oracle() {
        let layout = Layout::new(&[4, 8], 8, 80, 1).unwrap();
        let mut a = vec![Complex::zero(); 256];
        let mut b = vec![Complex::zero(); 256];
        a[7 * 8 + 3] = Complex {
            re: Signed::from_i64(1).shl(77),
            im: Signed::from_i64(-1).shl(77),
        };
        a[24 * 8 + 7] = Complex::real(Signed::from_i64(-1).shl(77));
        b[19 * 8 + 6] = Complex::real(Signed::from_i64(1).shl(77));
        b[8 * 8 + 1] = Complex {
            re: Signed::zero(),
            im: Signed::from_i64(-1).shl(77),
        };
        let mut expected = vec![Complex::zero(); 256];
        for (i, x) in a
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.re.is_zero() || !x.im.is_zero())
        {
            for (j, y) in b
                .iter()
                .enumerate()
                .filter(|(_, x)| !x.re.is_zero() || !x.im.is_zero())
            {
                let ir = i / 8;
                let jr = j / 8;
                let record = (((ir / 8 + jr / 8) % 4) * 8) + (ir % 8 + jr % 8) % 8;
                let coefficient = i % 8 + j % 8;
                let product = x.mul_exact(y);
                let target = &mut expected[record * 8 + coefficient % 8];
                *target = if coefficient >= 8 {
                    target.sub(&product)
                } else {
                    target.add(&product)
                };
            }
        }
        for value in &mut expected {
            *value = value.shr_trunc(80 + 8);
        }
        let (actual, receipt) = layout.convolve(&a, &b).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(receipt.polynomial_products, 32);
    }
}
