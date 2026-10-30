//! Retained row space for the graph's emitted small-variable constraints.
//! Caps count coefficients, not bytes or total memory. No row is silently lost.

use crate::sparse::TrackedMap;
use crate::{accumulate, inverse, is_prime, mul, neg, reduce, Error, Sparse};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub retained_coefficients: usize,
    pub working_coefficients: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Insert {
    Independent,
    Dependent,
    /// The input has NOT been incorporated. The caller must stop or retain it.
    WorkingCap,
    /// The input has NOT been incorporated. Existing basis stays unchanged.
    RetainedCap,
}

/// Logical algorithm counters, including failed attempts. These are not a
/// complete cost model: map comparisons, allocations and setup are not counted.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Counts {
    pub input_terms: u128,
    pub pivot_lookups: u128,
    pub coefficient_updates: u128,
    pub normalization_multiplications: u128,
    pub inversions: u128,
    pub peak_working_coefficients: usize,
}

pub struct Basis {
    p: u64,
    columns: usize,
    limits: Limits,
    rows: TrackedMap<Sparse>,
    retained: usize,
    counts: Counts,
}

impl Basis {
    pub fn new(p: u64, columns: usize, limits: Limits) -> Result<Self, Error> {
        if !is_prime(p) {
            return Err(Error::NonPrimeModulus);
        }
        Ok(Self {
            p,
            columns,
            limits,
            rows: TrackedMap::new(),
            retained: 0,
            counts: Counts::default(),
        })
    }

    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    pub fn retained_coefficients(&self) -> usize {
        self.retained
    }
    pub fn counts(&self) -> &Counts {
        &self.counts
    }
    /// Read-only echelon rows; pivots are one and strictly ordered.
    pub fn rows(&self) -> impl Iterator<Item = &Sparse> {
        self.rows.values()
    }
    /// Lookup by normalized leading column without scanning all stored rows.
    pub fn pivot(&self, column: usize) -> Option<&Sparse> {
        self.rows.get(&column)
    }

    /// Incorporate a homogeneous equation. On a cap outcome the basis is
    /// unchanged, but counters retain work already performed. A working cap
    /// applies at each intermediate map update, even if later terms cancel.
    pub fn insert(&mut self, terms: &[(usize, u64)]) -> Result<Insert, Error> {
        if terms.iter().any(|&(i, _)| {
            crate::meter::collection(1, 0, 0, 1);
            i >= self.columns
        }) {
            return Err(Error::ColumnOutOfRange);
        }
        let mut row = Sparse::new();
        for &(i, value) in terms {
            crate::meter::collection(1, 0, 0, 0);
            self.counts.input_terms += 1;
            let value = reduce(value, self.p);
            if value != 0 && !row.contains_key(&i) && {
                crate::meter::collection(0, 0, 0, 1);
                row.len() == self.limits.working_coefficients
            } {
                return Ok(Insert::WorkingCap);
            }
            accumulate(&mut row, i, value, self.p);
            self.counts.peak_working_coefficients =
                self.counts.peak_working_coefficients.max(row.len());
        }
        while let Some((&column, &value)) = row.first_key_value() {
            crate::meter::collection(0, 1, 0, 0);
            self.counts.pivot_lookups += 1;
            let Some(pivot) = self.rows.get(&column) else {
                crate::meter::collection(0, 0, 0, 1);
                if row.len()
                    > self
                        .limits
                        .retained_coefficients
                        .saturating_sub(self.retained)
                {
                    return Ok(Insert::RetainedCap);
                }
                self.counts.inversions += 1;
                let inverse = inverse(value, self.p);
                for coefficient in row.values_mut() {
                    *coefficient = mul(*coefficient, inverse, self.p);
                    self.counts.normalization_multiplications += 1;
                }
                self.retained += row.len();
                self.rows.insert(column, row);
                return Ok(Insert::Independent);
            };
            let factor = neg(value, self.p);
            for (&i, &coefficient) in pivot {
                if !row.contains_key(&i) && {
                    crate::meter::collection(0, 0, 0, 1);
                    row.len() == self.limits.working_coefficients
                } {
                    return Ok(Insert::WorkingCap);
                }
                accumulate(&mut row, i, mul(factor, coefficient, self.p), self.p);
                self.counts.coefficient_updates += 1;
                self.counts.peak_working_coefficients =
                    self.counts.peak_working_coefficients.max(row.len());
            }
        }
        Ok(Insert::Dependent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cap_predicates_charge_only_evaluated_length_checks() {
        let mut working = Basis::new(
            7,
            1,
            Limits {
                working_coefficients: 0,
                retained_coefficients: 10,
            },
        )
        .unwrap();
        crate::meter::reset();
        assert_eq!(working.insert(&[(0, 1)]), Ok(Insert::WorkingCap));
        assert_eq!(crate::meter::snapshot().collection_comparisons, 2); // bound + working length
        assert_eq!(crate::meter::snapshot().collection_elements_initialized, 0);
        assert_eq!(crate::meter::snapshot().collection_elements_copied, 0);
        let mut retained = Basis::new(
            7,
            1,
            Limits {
                working_coefficients: 10,
                retained_coefficients: 0,
            },
        )
        .unwrap();
        crate::meter::reset();
        assert_eq!(retained.insert(&[(0, 1)]), Ok(Insert::RetainedCap));
        assert_eq!(crate::meter::snapshot().collection_comparisons, 3); // bound + working + retained
        assert_eq!(crate::meter::snapshot().collection_elements_initialized, 1);
        assert_eq!(crate::meter::snapshot().collection_elements_copied, 1);
    }

    #[test]
    fn slice_validation_and_consumption_charge_attempts() {
        let mut b = Basis::new(
            7,
            2,
            Limits {
                retained_coefficients: 10,
                working_coefficients: 10,
            },
        )
        .unwrap();
        crate::meter::reset();
        assert_eq!(b.insert(&[(0, 0), (1, 0)]), Ok(Insert::Dependent));
        assert_eq!(crate::meter::snapshot().collection_entry_visits, 4);
        assert_eq!(crate::meter::snapshot().collection_comparisons, 2);
        crate::meter::reset();
        assert_eq!(b.insert(&[(2, 1), (0, 1)]), Err(Error::ColumnOutOfRange));
        assert_eq!(crate::meter::snapshot().collection_entry_visits, 1);
        assert_eq!(crate::meter::snapshot().collection_comparisons, 1);
    }

    #[test]
    fn caps_preserve_basis_and_charge_attempts() {
        let mut b = Basis::new(
            7,
            5,
            Limits {
                retained_coefficients: 3,
                working_coefficients: 3,
            },
        )
        .unwrap();
        assert_eq!(b.insert(&[(0, 1), (2, 1), (3, 1)]), Ok(Insert::Independent));
        let before: Vec<_> = b.rows().cloned().collect();
        // Eliminating column zero expands this row past the working cap.
        assert_eq!(b.insert(&[(0, 1), (1, 2), (4, 1)]), Ok(Insert::WorkingCap));
        assert_eq!(b.rows().cloned().collect::<Vec<_>>(), before);
        assert!(b.counts().coefficient_updates > 0);
        assert_eq!(b.insert(&[(3, 1)]), Ok(Insert::RetainedCap));
        assert_eq!(b.rows().cloned().collect::<Vec<_>>(), before);
        assert_eq!(b.insert(&[(0, 2), (2, 2), (3, 2)]), Ok(Insert::Dependent));
        assert_eq!(b.retained_coefficients(), 3);
        assert_eq!(b.rank(), 1);
        let counts = b.counts().clone();
        assert_eq!(b.insert(&[(5, 1)]), Err(Error::ColumnOutOfRange));
        assert_eq!(b.counts(), &counts);
    }

    #[test]
    fn zero_budget_and_duplicate_cancellation() {
        let mut b = Basis::new(
            7,
            2,
            Limits {
                retained_coefficients: 0,
                working_coefficients: 1,
            },
        )
        .unwrap();
        assert_eq!(b.insert(&[(0, 1), (0, 6)]), Ok(Insert::Dependent));
        assert_eq!(b.insert(&[(0, 1)]), Ok(Insert::RetainedCap));
        assert_eq!(b.insert(&[(0, 1), (1, 1)]), Ok(Insert::WorkingCap));
        assert_eq!(b.rank(), 0);
    }
}
