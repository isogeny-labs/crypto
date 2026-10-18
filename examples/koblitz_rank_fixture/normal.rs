//! Normal-coordinate x keys for the degree-53 Koblitz field.
//!
//! A normal basis turns Frobenius squaring into a cyclic rotation.  The
//! construction is deterministic and derives its columns from the same field
//! squaring operation as the point arithmetic; no fixed conversion table is
//! trusted by the pair-table experiment.

const N: usize = 53;
const MASK: u64 = (1u64 << N) - 1;

#[derive(Clone)]
pub(super) struct NormalBasis53 {
    columns: [u64; N],
    inverse_columns: [u64; N],
}

impl NormalBasis53 {
    pub(super) fn new(mut square: impl FnMut(u64) -> u64) -> Self {
        for beta in 1..=MASK {
            let mut columns = [0u64; N];
            let mut value = beta;
            for column in &mut columns {
                *column = value;
                value = square(value);
            }
            let mut pivots: [Option<(u64, u64)>; N] = [None; N];
            let mut independent = true;
            for (index, &column) in columns.iter().enumerate() {
                let mut row = column;
                let mut mask = 1u64 << index;
                while row != 0 {
                    let bit = row.ilog2() as usize;
                    if let Some((pivot_row, pivot_mask)) = pivots[bit] {
                        row ^= pivot_row;
                        mask ^= pivot_mask;
                    } else {
                        pivots[bit] = Some((row, mask));
                        break;
                    }
                }
                if row == 0 {
                    independent = false;
                    break;
                }
            }
            if !independent {
                continue;
            }
            let mut inverse_columns = [0u64; N];
            for (bit, inverse) in inverse_columns.iter_mut().enumerate() {
                let mut row = 1u64 << bit;
                while row != 0 {
                    let pivot_bit = row.ilog2() as usize;
                    let (pivot_row, pivot_mask) =
                        pivots[pivot_bit].expect("independent normal basis spans degree-53 field");
                    row ^= pivot_row;
                    *inverse ^= pivot_mask;
                }
            }
            return Self {
                columns,
                inverse_columns,
            };
        }
        unreachable!("finite fields admit a normal basis")
    }

    pub(super) fn to_normal(&self, value: u64) -> u64 {
        assert_eq!(value & !MASK, 0, "field element exceeds degree 53");
        Self::linear(value, &self.inverse_columns)
    }

    pub(super) fn to_poly(&self, mask: u64) -> u64 {
        assert_eq!(mask & !MASK, 0, "normal mask exceeds degree 53");
        Self::linear(mask, &self.columns)
    }

    fn linear(mut mask: u64, columns: &[u64; N]) -> u64 {
        let mut result = 0;
        while mask != 0 {
            let bit = mask.trailing_zeros() as usize;
            result ^= columns[bit];
            mask &= mask - 1;
        }
        result
    }

    pub(super) fn rotate(mask: u64, shift: usize) -> u64 {
        let shift = shift % N;
        if shift == 0 {
            mask
        } else {
            ((mask << shift) | (mask >> (N - shift))) & MASK
        }
    }

    /// Return the smallest x key in the Frobenius orbit and the forward shift.
    /// A full point must still be compared with both signs after lookup.
    pub(super) fn canonical_x(&self, polynomial_x: u64) -> (u64, usize) {
        let mask = self.to_normal(polynomial_x);
        (0..N)
            .map(|shift| (Self::rotate(mask, shift), shift))
            .min()
            .expect("nonempty Frobenius orbit")
    }
}
