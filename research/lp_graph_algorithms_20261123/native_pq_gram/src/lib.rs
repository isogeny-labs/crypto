//! Standalone source-bound Rust correctness gate.
//!
//! The two PQ modules below are symlinked to the real repository sources by
//! run.sh; they are NOT copied, translated or reimplemented here. Only
//! crate-wide dependency surfaces (modular inverse and a dense test oracle)
//! are provided locally because the root crate currently has unrelated
//! build failures. The shim does not establish full-crate integration.

pub mod utils {
    use num_bigint::{BigInt, BigUint};
    use num_traits::{One, Zero};

    /// Mathematically exact extended-Euclid inverse; test-harness adapter for
    /// the root crate's `crate::utils::mod_inverse` dependency.
    pub fn mod_inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
        if n.is_zero() || a.is_zero() {
            return None;
        }
        let modulus = BigInt::from(n.clone());
        let (mut old_r, mut r) = (BigInt::from(a % n), modulus.clone());
        let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());
        while !r.is_zero() {
            let q = &old_r / &r;
            (old_r, r) = (r.clone(), old_r - &q * &r);
            (old_s, s) = (s.clone(), old_s - q * s);
        }
        if old_r != BigInt::one() {
            return None;
        }
        (((old_s % &modulus) + &modulus) % &modulus).to_biguint()
    }
}

pub mod cryptanalysis {
    pub mod ec_index_calculus {
        use crate::utils::mod_inverse;
        use num_bigint::BigUint;
        use num_traits::Zero;

        /// Dense reference used ONLY by the original PQ module's small prime
        /// system tests. Refuses dependent columns and nonunit pivots.
        pub fn gaussian_eliminate_mod_n(
            matrix: &mut [Vec<BigUint>],
            rhs: &mut [BigUint],
            n: &BigUint,
        ) -> Option<Vec<BigUint>> {
            if matrix.len() != rhs.len() || matrix.is_empty() {
                return None;
            }
            let cols = matrix[0].len();
            if matrix.iter().any(|row| row.len() != cols) {
                return None;
            }
            for col in 0..cols {
                if col >= matrix.len() {
                    return None;
                }
                let pivot = (col..matrix.len())
                    .find(|&row| mod_inverse(&matrix[row][col], n).is_some())?;
                matrix.swap(col, pivot);
                rhs.swap(col, pivot);
                let inverse = mod_inverse(&matrix[col][col], n)?;
                for entry in &mut matrix[col] {
                    *entry = (&*entry * &inverse) % n;
                }
                rhs[col] = (&rhs[col] * &inverse) % n;
                let pivot_row = matrix[col].clone();
                let pivot_rhs = rhs[col].clone();
                for row in 0..matrix.len() {
                    if row == col {
                        continue;
                    }
                    let factor = matrix[row][col].clone();
                    if factor.is_zero() {
                        continue;
                    }
                    for (a, b) in matrix[row].iter_mut().zip(&pivot_row) {
                        *a = ((&*a + n) - ((&factor * b) % n)) % n;
                    }
                    rhs[row] = ((&rhs[row] + n) - ((&factor * &pivot_rhs) % n)) % n;
                }
            }
            if (cols..matrix.len()).any(|i| !rhs[i].is_zero()) {
                return None;
            }
            Some(rhs[..cols].to_vec())
        }
    }

    // The actual source files are linked into this directory at run time.
    pub mod pq_sparse_la;
    pub mod pq_wiedemann;
}
