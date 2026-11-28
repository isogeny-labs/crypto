//! Verified black-box solve for a square sparse matrix over a prime field.
//! The modulus must be prime; callers must verify relation rows separately.
use crate::{add_mod, inv_mod, mul_mod, sub_mod, P};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SparseError {
    InvalidMatrix,
    InvalidRightHandSide,
    /// Probes did not yield a verified solution; singularity is not proven.
    Inconclusive,
}

/// Rows contain sorted, strictly increasing `(column, nonzero_coefficient)`
/// entries. The modulus is a trusted prime at most the native field prime.
pub struct SparseMatrix {
    rows: Vec<Vec<(usize, u64)>>,
    modulus: u64,
    nonzeros: usize,
}

/// A homogeneous relation system with at least `columns-1` sparse rows.
/// Rows are validated once; the caller must verify their group identities.
pub struct TallSparseRows {
    columns: usize,
    rows: Vec<Vec<(usize, u64)>>,
    modulus: u64,
}

/// Exact kernel and rank certificate obtained from a nonsingular compressed
/// minor. `vector[free_column]` is one.
pub struct RankCertifiedKernel {
    pub vector: Vec<u64>,
    pub free_column: usize,
    pub compression_attempts_used: usize,
    pub compressed_nonzeros: usize,
}

impl TallSparseRows {
    pub fn new(
        columns: usize,
        rows: Vec<Vec<(usize, u64)>>,
        modulus: u64,
    ) -> Result<Self, SparseError> {
        if columns < 2 || rows.len() < columns - 1 || !(2..=P).contains(&modulus) {
            return Err(SparseError::InvalidMatrix);
        }
        for row in &rows {
            let mut previous = None;
            for &(column, coefficient) in row {
                if column >= columns
                    || coefficient == 0
                    || coefficient >= modulus
                    || previous.is_some_and(|prior| column <= prior)
                {
                    return Err(SparseError::InvalidMatrix);
                }
                previous = Some(column);
            }
        }
        Ok(Self {
            columns,
            rows,
            modulus,
        })
    }

    /// Hash each source row into one of `columns-1` buckets with a nonzero
    /// field weight. A full-degree Wiedemann solve certifies the compressed
    /// square minor is nonsingular. The resulting null vector is checked
    /// against every original row before it is returned. Therefore a
    /// successful result certifies original row rank exactly `columns-1`.
    /// Exhausted attempts are inconclusive; no rank claim follows.
    pub fn rank_certified_nullspace(
        &self,
        free_column: usize,
        compression_attempts: usize,
        probe_attempts: usize,
    ) -> Result<RankCertifiedKernel, SparseError> {
        if free_column >= self.columns {
            return Err(SparseError::InvalidMatrix);
        }
        let n = self.columns - 1;
        let mut state = 0xa7d3_95b1_2f06_2026_u64;
        for attempt in 1..=compression_attempts {
            let mut buckets = vec![HashMap::<usize, u64>::new(); n];
            let mut rhs = vec![0; n];
            for row in &self.rows {
                let bucket = (splitmix64(&mut state) as usize) % n;
                let weight = 1 + splitmix64(&mut state) % (self.modulus - 1);
                for &(column, coefficient) in row {
                    let product = mul_mod(weight, coefficient, self.modulus);
                    if column == free_column {
                        rhs[bucket] = sub_mod(rhs[bucket], product, self.modulus);
                    } else {
                        let compressed_column = if column < free_column {
                            column
                        } else {
                            column - 1
                        };
                        let prior = buckets[bucket]
                            .get(&compressed_column)
                            .copied()
                            .unwrap_or(0);
                        let value = add_mod(prior, product, self.modulus);
                        if value == 0 {
                            buckets[bucket].remove(&compressed_column);
                        } else {
                            buckets[bucket].insert(compressed_column, value);
                        }
                    }
                }
            }
            let compressed_rows = buckets
                .into_iter()
                .map(|row| {
                    let mut entries = row.into_iter().collect::<Vec<_>>();
                    entries.sort_unstable_by_key(|&(column, _)| column);
                    entries
                })
                .collect();
            let square = SparseMatrix::new(compressed_rows, self.modulus)?;
            let Ok(solution) = square.solve_certified(&rhs, probe_attempts) else {
                continue;
            };
            let mut vector = vec![0; self.columns];
            vector[free_column] = 1;
            for (column, &value) in solution.iter().enumerate() {
                vector[if column < free_column {
                    column
                } else {
                    column + 1
                }] = value;
            }
            let valid = self.rows.iter().all(|row| {
                row.iter().fold(0, |sum, &(column, coefficient)| {
                    add_mod(
                        sum,
                        mul_mod(coefficient, vector[column], self.modulus),
                        self.modulus,
                    )
                }) == 0
            });
            if valid {
                return Ok(RankCertifiedKernel {
                    vector,
                    free_column,
                    compression_attempts_used: attempt,
                    compressed_nonzeros: square.nonzeros(),
                });
            }
        }
        Err(SparseError::Inconclusive)
    }
}

impl SparseMatrix {
    pub fn new(rows: Vec<Vec<(usize, u64)>>, modulus: u64) -> Result<Self, SparseError> {
        let n = rows.len();
        if n == 0 || !(2..=P).contains(&modulus) {
            return Err(SparseError::InvalidMatrix);
        }
        let mut nonzeros = 0;
        for row in &rows {
            let mut previous = None;
            for &(column, coefficient) in row {
                if column >= n
                    || coefficient == 0
                    || coefficient >= modulus
                    || previous.is_some_and(|prior| column <= prior)
                {
                    return Err(SparseError::InvalidMatrix);
                }
                previous = Some(column);
                nonzeros += 1;
            }
        }
        Ok(Self {
            rows,
            modulus,
            nonzeros,
        })
    }

    pub fn dimension(&self) -> usize {
        self.rows.len()
    }

    pub fn nonzeros(&self) -> usize {
        self.nonzeros
    }

    fn apply(&self, vector: &[u64]) -> Vec<u64> {
        self.rows
            .iter()
            .map(|row| {
                row.iter().fold(0, |sum, &(column, coefficient)| {
                    add_mod(
                        sum,
                        mul_mod(coefficient, vector[column], self.modulus),
                        self.modulus,
                    )
                })
            })
            .collect()
    }

    fn dot(&self, left: &[u64], right: &[u64]) -> u64 {
        left.iter().zip(right).fold(0, |sum, (&a, &b)| {
            add_mod(sum, mul_mod(a, b, self.modulus), self.modulus)
        })
    }

    /// Solve using supplied scalar-sequence probes, checking each candidate
    /// by a full sparse matrix-vector product before returning it.
    pub fn solve_with_probes(
        &self,
        rhs: &[u64],
        probes: &[Vec<u64>],
    ) -> Result<Vec<u64>, SparseError> {
        self.solve_with_probes_mode(rhs, probes, false)
    }

    fn solve_with_probes_mode(
        &self,
        rhs: &[u64],
        probes: &[Vec<u64>],
        require_full_degree: bool,
    ) -> Result<Vec<u64>, SparseError> {
        let n = self.dimension();
        if rhs.len() != n || rhs.iter().any(|&value| value >= self.modulus) {
            return Err(SparseError::InvalidRightHandSide);
        }
        if rhs.iter().all(|&value| value == 0) {
            return if require_full_degree {
                Err(SparseError::Inconclusive)
            } else {
                Ok(vec![0; n])
            };
        }
        for probe in probes {
            if probe.len() != n || probe.iter().any(|&value| value >= self.modulus) {
                return Err(SparseError::InvalidRightHandSide);
            }
            let mut vector = rhs.to_vec();
            let mut sequence = Vec::with_capacity(2 * n);
            for _ in 0..2 * n {
                sequence.push(self.dot(probe, &vector));
                vector = self.apply(&vector);
            }
            let connection = berlekamp_massey(&sequence, self.modulus);
            let degree = connection.len() - 1;
            if degree == 0 || connection[degree] == 0 || (require_full_degree && degree != n) {
                continue;
            }
            let inverse =
                inv_mod(connection[degree], self.modulus).ok_or(SparseError::Inconclusive)?;
            let mut candidate = vec![0; n];
            let mut power = rhs.to_vec();
            for exponent in 0..degree {
                let coefficient = connection[degree - 1 - exponent];
                for (entry, &value) in candidate.iter_mut().zip(&power) {
                    *entry = add_mod(
                        *entry,
                        mul_mod(coefficient, value, self.modulus),
                        self.modulus,
                    );
                }
                if exponent + 1 != degree {
                    power = self.apply(&power);
                }
            }
            for entry in &mut candidate {
                *entry = sub_mod(0, mul_mod(*entry, inverse, self.modulus), self.modulus);
            }
            if self.apply(&candidate) == rhs {
                return Ok(candidate);
            }
        }
        Err(SparseError::Inconclusive)
    }

    /// Retry with deterministic pseudorandom probes. A returned answer is
    /// exact; exhaustion is inconclusive rather than a singularity proof.
    fn solve_mode(
        &self,
        rhs: &[u64],
        attempts: usize,
        require_full_degree: bool,
    ) -> Result<Vec<u64>, SparseError> {
        let mut state = 0x65d3_14a1_1ed5_2026_u64;
        if attempts == 0 {
            return self.solve_with_probes_mode(rhs, &[], require_full_degree);
        }
        for _ in 0..attempts {
            let probe = (0..self.dimension())
                .map(|_| splitmix64(&mut state) % self.modulus)
                .collect::<Vec<_>>();
            match self.solve_with_probes_mode(rhs, &[probe], require_full_degree) {
                Ok(solution) => return Ok(solution),
                Err(SparseError::Inconclusive) => {}
                Err(error) => return Err(error),
            }
        }
        Err(SparseError::Inconclusive)
    }

    /// Find a verified solution; does not prove the matrix is nonsingular.
    pub fn solve(&self, rhs: &[u64], attempts: usize) -> Result<Vec<u64>, SparseError> {
        self.solve_mode(rhs, attempts, false)
    }

    /// Return only after a scalar recurrence of full dimension with nonzero
    /// constant term is found. Since the matrix minimal polynomial then has
    /// degree at least `n`, Cayley–Hamilton forces it to equal the degree-n
    /// characteristic polynomial; its nonzero constant proves det(A) != 0.
    /// An inconclusive result does not imply that A is singular.
    pub fn solve_certified(&self, rhs: &[u64], attempts: usize) -> Result<Vec<u64>, SparseError> {
        self.solve_mode(rhs, attempts, true)
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// C(z)=1+c1*z+...+cL*z^L, with
// s_k + c1*s_(k-1) + ... + cL*s_(k-L) = 0 for k >= L.
fn berlekamp_massey(sequence: &[u64], modulus: u64) -> Vec<u64> {
    let mut connection = vec![1];
    let mut previous_connection = vec![1];
    let mut length = 0;
    let mut shift = 1;
    let mut last_nonzero = 1;
    for (index, &value) in sequence.iter().enumerate() {
        let mut discrepancy = value;
        for j in 1..=length {
            discrepancy = add_mod(
                discrepancy,
                mul_mod(connection[j], sequence[index - j], modulus),
                modulus,
            );
        }
        if discrepancy == 0 {
            shift += 1;
            continue;
        }
        let old = connection.clone();
        let factor = mul_mod(
            discrepancy,
            inv_mod(last_nonzero, modulus).expect("prime modulus and nonzero discrepancy"),
            modulus,
        );
        connection.resize(connection.len().max(previous_connection.len() + shift), 0);
        for (j, &coefficient) in previous_connection.iter().enumerate() {
            connection[j + shift] = sub_mod(
                connection[j + shift],
                mul_mod(factor, coefficient, modulus),
                modulus,
            );
        }
        if 2 * length <= index {
            length = index + 1 - length;
            previous_connection = old;
            last_nonzero = discrepancy;
            shift = 1;
        } else {
            shift += 1;
        }
    }
    connection.truncate(length + 1);
    connection
}

#[cfg(test)]
mod tests {
    use super::{SparseError, SparseMatrix, TallSparseRows};
    use crate::R;

    fn toy_rows() -> Vec<Vec<(usize, u64)>> {
        let fixture = include_str!("../fixtures/toy_8191.txt");
        let mut rows = Vec::new();
        for line in fixture.lines().filter(|line| line.starts_with('R')) {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 8 || !parts[0].starts_with('R') {
                continue;
            }
            let mut row = Vec::new();
            for term in &parts[1..] {
                let (index, sign) = term.split_once(':').unwrap();
                let index = index.parse::<usize>().unwrap();
                let coefficient = if sign == "+1" { 1 } else { 58 };
                row.push((index, coefficient));
            }
            rows.push(row);
        }
        assert_eq!(rows.len(), 19);
        rows
    }

    #[test]
    fn toy_relation_matrix_kernel_after_fixing_last_column() {
        let mut rhs = Vec::new();
        let rows = toy_rows()
            .into_iter()
            .map(|mut row| {
                let right = if row.last().is_some_and(|&(column, _)| column == 19) {
                    let (_, coefficient) = row.pop().unwrap();
                    (59 - coefficient) % 59
                } else {
                    0
                };
                rhs.push(right);
                row
            })
            .collect();
        let matrix = SparseMatrix::new(rows, 59).unwrap();
        let solution = matrix.solve(&rhs, 8).unwrap();
        assert_eq!(matrix.solve_certified(&rhs, 8).unwrap(), solution);
        assert_eq!(
            solution,
            vec![12, 31, 20, 54, 16, 55, 6, 2, 12, 14, 24, 50, 6, 28, 2, 34, 58, 9, 50]
        );
    }

    #[test]
    fn overdetermined_toy_rows_certify_exact_rank_and_kernel() {
        let base_rows = toy_rows();
        let rows = (0..8)
            .flat_map(|_| base_rows.iter().cloned())
            .collect::<Vec<_>>();
        let tall = TallSparseRows::new(20, rows, 59).unwrap();
        let certificate = tall.rank_certified_nullspace(19, 64, 8).unwrap();
        assert_eq!(certificate.free_column, 19);
        assert!(certificate.compression_attempts_used <= 64);
        assert!(certificate.compressed_nonzeros > 19);
        assert_eq!(
            certificate.vector,
            vec![12, 31, 20, 54, 16, 55, 6, 2, 12, 14, 24, 50, 6, 28, 2, 34, 58, 9, 50, 1]
        );
    }

    #[test]
    fn tall_rank_deficiency_and_inconsistent_rows_remain_inconclusive() {
        assert!(TallSparseRows::new(3, vec![vec![(3, 1)]; 2], 59).is_err());
        assert!(TallSparseRows::new(3, vec![vec![(0, 1), (0, 1)]; 2], 59).is_err());
        let deficient = TallSparseRows::new(3, vec![vec![(0, 1)]; 8], 59).unwrap();
        assert!(matches!(
            deficient.rank_certified_nullspace(3, 1, 1),
            Err(SparseError::InvalidMatrix)
        ));
        assert!(matches!(
            deficient.rank_certified_nullspace(2, 16, 4),
            Err(SparseError::Inconclusive)
        ));
        let inconsistent = TallSparseRows::new(
            3,
            (0..8)
                .flat_map(|_| [vec![(0, 1)], vec![(1, 1)], vec![(2, 1)]])
                .collect(),
            59,
        )
        .unwrap();
        assert!(matches!(
            inconsistent.rank_certified_nullspace(2, 16, 4),
            Err(SparseError::Inconclusive)
        ));
    }

    #[test]
    fn bad_probe_is_rejected_and_later_probe_recovers() {
        let matrix = SparseMatrix::new(vec![vec![(0, 2)], vec![(1, 3)]], 59).unwrap();
        let rhs = [1, 1];
        assert_eq!(
            matrix.solve_with_probes(&rhs, &[vec![1, 0]]),
            Err(SparseError::Inconclusive)
        );
        let solution = matrix
            .solve_with_probes(&rhs, &[vec![1, 0], vec![1, 1]])
            .unwrap();
        assert_eq!(solution, vec![30, 20]);
    }

    #[test]
    fn solves_at_selected_61_bit_subgroup_modulus() {
        let matrix = SparseMatrix::new(
            vec![
                vec![(0, R - 1), (1, 2)],
                vec![(0, 3), (2, 4)],
                vec![(1, 5), (2, 6)],
            ],
            R,
        )
        .unwrap();
        let expected = vec![7, 11, 13];
        let rhs = matrix.apply(&expected);
        assert_eq!(matrix.solve(&rhs, 8).unwrap(), expected);
    }

    #[test]
    fn solves_nontrivial_sparse_recurrence() {
        let n = 64;
        let rows = (0..n)
            .map(|i| {
                let mut row = vec![(i, (i + 1) as u64)];
                for j in i + 1..n.min(i + 4) {
                    row.push((j, (2 * i + j + 1) as u64));
                }
                row
            })
            .collect();
        let matrix = SparseMatrix::new(rows, R).unwrap();
        let expected = (0..n).map(|i| (3 * i + 7) as u64).collect::<Vec<_>>();
        let rhs = matrix.apply(&expected);
        assert_eq!(matrix.solve(&rhs, 4).unwrap(), expected);
    }

    #[test]
    fn invalid_and_singular_inputs_fail_closed() {
        assert!(SparseMatrix::new(vec![vec![(1, 1)]], 59).is_err());
        assert!(SparseMatrix::new(vec![vec![(0, 1), (0, 1)]], 59).is_err());
        let singular = SparseMatrix::new(vec![vec![(0, 1)], vec![]], 59).unwrap();
        assert_eq!(
            singular.solve_with_probes(&[0, 1], &[vec![1, 1]]),
            Err(SparseError::Inconclusive)
        );
        assert_eq!(
            singular.solve_certified(&[1, 0], 8),
            Err(SparseError::Inconclusive)
        );
        assert_eq!(
            singular.solve_certified(&[0, 0], 8),
            Err(SparseError::Inconclusive)
        );
    }
}
