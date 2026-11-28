use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Return a nonzero integer dependency among the relation rows with gcd one.
///
/// Fraction-free elimination on the transpose stops at the first dependent
/// column. Back substitution keeps the assigned weights primitive by scaling
/// them only enough to clear the next pivot denominator.
pub(super) fn primitive_dependency(relations: &[Vec<BigInt>]) -> Option<Vec<BigInt>> {
    let coordinates = relations.first()?.len();
    if relations.iter().any(|row| row.len() != coordinates) {
        return None;
    }

    let mut echelon: Vec<Vec<_>> = (0..coordinates)
        .map(|index| relations.iter().map(|row| row[index].clone()).collect())
        .collect();
    let mut previous_pivot = BigInt::one();
    let mut dependent_column = None;
    for column in 0..relations.len() {
        let Some(pivot_row) = (column..coordinates).find(|&row| !echelon[row][column].is_zero())
        else {
            dependent_column = Some(column);
            break;
        };
        echelon.swap(column, pivot_row);
        let (processed, remaining) = echelon.split_at_mut(column + 1);
        let pivot_row = &processed[column];
        let pivot = &pivot_row[column];
        for row in remaining {
            let factor = std::mem::take(&mut row[column]);
            for (entry, pivot_entry) in row[column + 1..].iter_mut().zip(&pivot_row[column + 1..]) {
                let numerator = &*entry * pivot - &factor * pivot_entry;
                let (quotient, remainder) = numerator.div_rem(&previous_pivot);
                if !remainder.is_zero() {
                    return None;
                }
                *entry = quotient;
            }
        }
        previous_pivot = pivot.clone();
    }

    let dependent_column = dependent_column?;
    let mut weights = vec![BigInt::zero(); relations.len()];
    weights[dependent_column] = BigInt::one();
    for column in (0..dependent_column).rev() {
        let tail = column + 1..=dependent_column;
        let sum: BigInt = echelon[column][tail.clone()]
            .iter()
            .zip(&weights[tail.clone()])
            .map(|(entry, weight)| entry * weight)
            .sum();
        let pivot = &echelon[column][column];
        let gcd = pivot.gcd(&sum);
        let scale = pivot.abs() / &gcd;
        for weight in &mut weights[tail] {
            *weight *= &scale;
        }
        weights[column] = -(sum / gcd) * pivot.signum();
    }
    Some(weights)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(rows: &[&[i64]]) -> Vec<Vec<BigInt>> {
        rows.iter()
            .map(|row| row.iter().copied().map(BigInt::from).collect())
            .collect()
    }

    fn check_dependency(rows: &[Vec<BigInt>], expected: &[i64]) -> Vec<BigInt> {
        let weights = primitive_dependency(rows).unwrap();
        assert_eq!(
            weights,
            expected
                .iter()
                .copied()
                .map(BigInt::from)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            weights
                .iter()
                .fold(BigInt::zero(), |gcd, weight| gcd.gcd(weight)),
            BigInt::one()
        );
        for column in 0..rows[0].len() {
            let sum: BigInt = rows
                .iter()
                .zip(&weights)
                .map(|(row, weight)| &row[column] * weight)
                .sum();
            assert!(sum.is_zero());
        }
        weights
    }

    #[test]
    fn preserves_primitivity_when_clearing_multiple_denominators() {
        let rows = matrix(&[&[6, 0], &[0, -10], &[4, 6]]);
        check_dependency(&rows, &[-10, 9, 15]);
    }

    #[test]
    fn handles_a_rank_deficient_prefix_after_a_row_swap() {
        let rows = matrix(&[&[0, 2, 4], &[0, -3, -6], &[1, 0, 1], &[4, 9, 10]]);
        check_dependency(&rows, &[3, 2, 0, 0]);
    }

    #[test]
    fn works_when_the_pivot_minor_is_zero_modulo_the_subgroup_prime() {
        let rows = matrix(&[&[5, 0], &[0, 1], &[1, 0]]);
        let weights = check_dependency(&rows, &[-1, 0, 5]);
        assert!(weights
            .iter()
            .any(|weight| !weight.mod_floor(&BigInt::from(5)).is_zero()));
    }

    #[test]
    fn returns_none_for_independent_or_malformed_rows() {
        assert!(primitive_dependency(&matrix(&[&[1, 0], &[0, 1]])).is_none());
        assert!(primitive_dependency(&matrix(&[&[1], &[1, 0]])).is_none());
        assert!(primitive_dependency(&[]).is_none());
    }

    #[test]
    fn treats_an_empty_relation_as_a_dependency() {
        check_dependency(&matrix(&[&[], &[]]), &[1, 0]);
    }
}
