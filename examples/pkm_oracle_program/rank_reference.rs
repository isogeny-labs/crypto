//! Independent scalar-row replay, not an admitted cost model.
//!
//! Rightmost-pivot echelon space with integer Euclidean inverses. It does not
//! call the producer's leftmost-pivot RREF, its field helpers or its counters.
//! Sparse provenance retains combinations of original input rows. The tiny
//! control uses at most 258 columns; the explicit 512-column limit is wider.
use std::collections::BTreeMap;

pub(super) type Combination = Vec<(usize, u64)>;

fn product(a: u64, b: u64, p: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}
fn difference(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(p) - u128::from(b)) % u128::from(p)) as u64
}
fn inverse(a: u64, p: u64) -> Result<u64, String> {
    let (mut r, mut next_r) = (i128::from(p), i128::from(a));
    let (mut t, mut next_t) = (0i128, 1i128);
    while next_r != 0 {
        let quotient = r / next_r;
        (r, next_r) = (next_r, r - quotient * next_r);
        (t, next_t) = (next_t, t - quotient * next_t);
    }
    if r != 1 {
        return Err("noninvertible reference coefficient".into());
    }
    Ok(t.rem_euclid(i128::from(p)) as u64)
}
fn canonical(row: &[u64], dimension: usize, p: u64) -> Result<(), String> {
    if row.len() != dimension || row.iter().any(|&x| x >= p) {
        return Err("noncanonical reference row".into());
    }
    Ok(())
}
fn accumulate(weights: &mut BTreeMap<usize, u64>, index: usize, value: u64, p: u64) {
    let sum = (weights.get(&index).copied().unwrap_or(0) + value) % p;
    if sum == 0 {
        weights.remove(&index);
    } else {
        weights.insert(index, sum);
    }
}

struct Vector {
    pivot: usize,
    values: Vec<u64>,
    provenance: BTreeMap<usize, u64>,
}

pub(super) struct Space {
    p: u64,
    dimension: usize,
    input_rows: usize,
    basis: Vec<Vector>,
}
impl Space {
    pub fn new(p: u64, dimension: usize) -> Result<Self, String> {
        if !(2..1u64 << 32).contains(&p) || !(1..=512).contains(&dimension) {
            return Err("unsupported reference scalar space".into());
        }
        // Separate bounded primality check; no shared producer helper.
        let mut divisor = 2;
        while divisor <= p / divisor {
            if p.is_multiple_of(divisor) {
                return Err("reference scalar modulus is not prime".into());
            }
            divisor += 1;
        }
        Ok(Self {
            p,
            dimension,
            input_rows: 0,
            basis: Vec::new(),
        })
    }
    pub fn rank(&self) -> usize {
        self.basis.len()
    }
    fn reduce(&self, row: &[u64]) -> Result<(Vec<u64>, BTreeMap<usize, u64>), String> {
        canonical(row, self.dimension, self.p)?;
        let mut remaining = row.to_vec();
        let mut used = BTreeMap::new();
        for vector in &self.basis {
            let coefficient = remaining[vector.pivot];
            if coefficient == 0 {
                continue;
            }
            for (a, &b) in remaining.iter_mut().zip(&vector.values) {
                *a = difference(*a, product(coefficient, b, self.p), self.p);
            }
            for (&index, &weight) in &vector.provenance {
                accumulate(
                    &mut used,
                    index,
                    product(coefficient, weight, self.p),
                    self.p,
                );
            }
        }
        Ok((remaining, used))
    }
    pub fn insert(&mut self, row: &[u64]) -> Result<bool, String> {
        let (mut remaining, used) = self.reduce(row)?;
        let index = self.input_rows;
        self.input_rows = self.input_rows.checked_add(1).ok_or("row index overflow")?;
        let Some(pivot) = remaining.iter().rposition(|&x| x != 0) else {
            return Ok(false);
        };
        let scale = inverse(remaining[pivot], self.p)?;
        for x in &mut remaining {
            *x = product(*x, scale, self.p);
        }
        let mut provenance = BTreeMap::from([(index, scale)]);
        for (prior, weight) in used {
            accumulate(
                &mut provenance,
                prior,
                difference(0, product(scale, weight, self.p), self.p),
                self.p,
            );
        }
        self.basis.push(Vector {
            pivot,
            values: remaining,
            provenance,
        });
        self.basis.sort_by_key(|v| std::cmp::Reverse(v.pivot));
        Ok(true)
    }
    pub fn representation(&self, row: &[u64]) -> Result<Option<Combination>, String> {
        let (remaining, used) = self.reduce(row)?;
        Ok(remaining
            .iter()
            .all(|&x| x == 0)
            .then(|| used.into_iter().collect()))
    }
    pub fn contains(&self, row: &[u64]) -> Result<bool, String> {
        Ok(self.representation(row)?.is_some())
    }
    pub fn verify_combination(
        &self,
        rows: &[Vec<u64>],
        weights: &Combination,
        target: &[u64],
    ) -> Result<(), String> {
        canonical(target, self.dimension, self.p)?;
        let mut actual = vec![0u64; self.dimension];
        let mut previous = None;
        for &(index, weight) in weights {
            if index >= rows.len()
                || weight == 0
                || weight >= self.p
                || previous.is_some_and(|i| i >= index)
            {
                return Err("invalid sparse combination certificate".into());
            }
            canonical(&rows[index], self.dimension, self.p)?;
            for (a, &b) in actual.iter_mut().zip(&rows[index]) {
                *a = (*a + product(weight, b, self.p)) % self.p;
            }
            previous = Some(index);
        }
        if actual != target {
            return Err("row combination does not equal target equation".into());
        }
        Ok(())
    }
    pub fn verify_rref(
        &self,
        rows: &[(usize, Vec<u64>)],
        coefficient_rank: usize,
    ) -> Result<(), String> {
        if rows.len() != coefficient_rank || self.rank() != coefficient_rank {
            return Err("coefficient/augmented rank mismatch".into());
        }
        let variables = self.dimension - 1;
        let mut previous = None;
        for (pivot, row) in rows {
            canonical(row, self.dimension, self.p)?;
            if *pivot >= variables
                || previous.is_some_and(|p| p >= *pivot)
                || row[*pivot] != 1
                || row[..*pivot].iter().any(|&x| x != 0)
                || !self.contains(row)?
            {
                return Err("invalid RREF shape or row-space membership".into());
            }
            previous = Some(*pivot);
        }
        for (i, (pivot, _)) in rows.iter().enumerate() {
            if rows
                .iter()
                .enumerate()
                .any(|(j, (_, row))| i != j && row[*pivot] != 0)
            {
                return Err("RREF pivot column is not reduced".into());
            }
        }
        Ok(())
    }
    pub fn ambiguity(&self, coordinate: usize) -> Result<Option<Vec<u64>>, String> {
        if coordinate >= self.dimension {
            return Err("invalid ambiguity coordinate".into());
        }
        let mut unit = vec![0; self.dimension];
        unit[coordinate] = 1;
        if self.contains(&unit)? {
            return Ok(None);
        }
        for free in 0..self.dimension {
            if self.basis.iter().any(|v| v.pivot == free) {
                continue;
            }
            let mut kernel = vec![0; self.dimension];
            kernel[free] = 1;
            // Rightmost-pivot rows contain only earlier coordinates.
            for vector in self.basis.iter().rev() {
                let sum = vector.values[..vector.pivot]
                    .iter()
                    .zip(&kernel)
                    .fold(0, |sum, (&a, &b)| (sum + product(a, b, self.p)) % self.p);
                kernel[vector.pivot] = difference(0, sum, self.p);
            }
            if kernel[coordinate] != 0 {
                let scale = inverse(kernel[coordinate], self.p)?;
                for x in &mut kernel {
                    *x = product(*x, scale, self.p);
                }
                return Ok(Some(kernel));
            }
        }
        Err("reference space failed to produce an ambiguity witness".into())
    }
    pub fn verify_kernel(
        &self,
        rows: &[Vec<u64>],
        coordinate: usize,
        kernel: &[u64],
    ) -> Result<(), String> {
        canonical(kernel, self.dimension, self.p)?;
        if coordinate >= self.dimension || kernel[coordinate] != 1 {
            return Err("ambiguity witness must change the target by one".into());
        }
        for row in rows {
            canonical(row, self.dimension, self.p)?;
            let sum = row
                .iter()
                .zip(kernel)
                .fold(0, |sum, (&a, &b)| (sum + product(a, b, self.p)) % self.p);
            if sum != 0 {
                return Err("ambiguity witness is not in the original-row kernel".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_two_by_three_f3_spaces_match_minor_rank_and_check_every_certificate() {
        for code in 0..729u64 {
            let mut digits = code;
            let mut rows = vec![vec![0; 3]; 2];
            for row in &mut rows {
                for x in row {
                    *x = digits % 3;
                    digits /= 3;
                }
            }
            let rank = if rows.iter().flatten().all(|&x| x == 0) {
                0
            } else if (0..3).any(|i| {
                (i + 1..3).any(|j| {
                    difference(
                        product(rows[0][i], rows[1][j], 3),
                        product(rows[0][j], rows[1][i], 3),
                        3,
                    ) != 0
                })
            }) {
                2
            } else {
                1
            };
            let mut space = Space::new(3, 3).unwrap();
            for row in &rows {
                space.insert(row).unwrap();
            }
            assert_eq!(space.rank(), rank);
            for x in 0..27u64 {
                let target = vec![x % 3, x / 3 % 3, x / 9];
                let expected = (0..3).any(|a| {
                    (0..3).any(|b| {
                        (0..3).all(|i| {
                            (product(a, rows[0][i], 3) + product(b, rows[1][i], 3)) % 3 == target[i]
                        })
                    })
                });
                let representation = space.representation(&target).unwrap();
                assert_eq!(representation.is_some(), expected);
                if let Some(weights) = representation {
                    space.verify_combination(&rows, &weights, &target).unwrap();
                }
            }
            for coordinate in 0..3 {
                if let Some(h) = space.ambiguity(coordinate).unwrap() {
                    space.verify_kernel(&rows, coordinate, &h).unwrap();
                } else {
                    let mut unit = vec![0; 3];
                    unit[coordinate] = 1;
                    let weights = space.representation(&unit).unwrap().unwrap();
                    space.verify_combination(&rows, &weights, &unit).unwrap();
                }
            }
        }
    }
    #[test]
    fn recovery_combinations_survive_row_order_and_large_residues() {
        let data = [
            vec![1, 2, 3, 14],
            vec![2, 5, 7, 8],
            vec![3, 7, 10, 3],
            vec![0, 0, 1, 11],
        ];
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        if [a, b, c, d]
                            .iter()
                            .enumerate()
                            .any(|(i, x)| [a, b, c, d][..i].contains(x))
                        {
                            continue;
                        }
                        let rows: Vec<_> = [a, b, c, d].iter().map(|&i| data[i].clone()).collect();
                        let mut coeff = Space::new(19, 3).unwrap();
                        let mut augmented = Space::new(19, 4).unwrap();
                        for row in &rows {
                            coeff.insert(&row[..3]).unwrap();
                            augmented.insert(row).unwrap();
                        }
                        assert_eq!(coeff.rank(), 3);
                        let target = [0, 0, 1, 11];
                        let weights = augmented.representation(&target).unwrap().unwrap();
                        augmented
                            .verify_combination(&rows, &weights, &target)
                            .unwrap();
                        augmented
                            .verify_rref(
                                &[
                                    (0, vec![1, 0, 0, 5]),
                                    (1, vec![0, 1, 0, 7]),
                                    (2, vec![0, 0, 1, 11]),
                                ],
                                3,
                            )
                            .unwrap();
                        assert_eq!(coeff.ambiguity(2).unwrap(), None);
                    }
                }
            }
        }
        let p = 4_294_967_291;
        let rows = vec![vec![p - 1, 1]];
        let mut space = Space::new(p, 2).unwrap();
        space.insert(&rows[0]).unwrap();
        let weights = space.representation(&[1, p - 1]).unwrap().unwrap();
        space
            .verify_combination(&rows, &weights, &[1, p - 1])
            .unwrap();
    }
    #[test]
    fn malformed_rank_recovery_and_ambiguity_certificates_fail_closed() {
        assert!(Space::new(9, 2).is_err());
        let mut space = Space::new(19, 3).unwrap();
        let rows = vec![vec![1, 1, 5], vec![2, 2, 10]];
        for row in &rows {
            space.insert(row).unwrap();
        }
        assert!(space
            .verify_combination(&rows, &vec![(0, 2)], &[1, 1, 5])
            .is_err());
        assert!(space
            .verify_combination(&rows, &vec![(0, 1), (0, 1)], &[2, 2, 10])
            .is_err());
        assert!(space.verify_rref(&[(0, vec![2, 2, 10])], 1).is_err());
        assert!(space.verify_rref(&[(0, vec![1, 0, 5])], 1).is_err());
        assert!(space.verify_rref(&[(0, vec![1, 1, 5])], 0).is_err());
        let coefficients = vec![vec![1, 1], vec![2, 2]];
        let coeff = Space::new(19, 2).unwrap();
        assert!(coeff.verify_kernel(&coefficients, 1, &[1, 1]).is_err());
        assert!(coeff.verify_kernel(&coefficients, 1, &[18, 0]).is_err());
        assert!(space.insert(&[19, 0, 0]).is_err());
    }
}
