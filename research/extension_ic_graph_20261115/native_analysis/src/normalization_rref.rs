//! Recorded-matrix RREF experiment. Arithmetic/update order follows
//! residual_walk::RelationSystem::insert_by at source c79c11d6c.
//! The optional change only avoids pivot-normalization products by zero.
use serde::{Deserialize, Serialize};
use std::hint::select_unpredictable;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub products: u64,
    pub subtractions: u64,
    pub inversions: u64,
    pub normalization_products: u64,
    pub zero_normalizations_observed: u64,
    pub normalization_tests: u64,
    pub forward_coefficient_scans: u64,
    pub backward_support_entries: u64,
    pub pivot_selection_scans: u64,
    pub solved_coefficient_scans: u64,
    pub factor_constructions: u64,
}
#[derive(Clone, Copy)]
struct Factor {
    w: u64,
    quotient: u64,
}
impl Factor {
    fn new(w: u64, p: u64) -> Self {
        let w = if p <= 1u64 << 63 { w % p } else { w };
        Self {
            w,
            quotient: if p <= 1u64 << 63 {
                (((w as u128) << 64) / p as u128) as u64
            } else {
                0
            },
        }
    }
    fn mul(self, x: u64, p: u64) -> u64 {
        if p <= 1u64 << 63 {
            let q = ((self.quotient as u128 * x as u128) >> 64) as u64;
            let r = self.w.wrapping_mul(x).wrapping_sub(q.wrapping_mul(p));
            let (d, borrow) = r.overflowing_sub(p);
            select_unpredictable(borrow, r, d)
        } else {
            ((self.w as u128 * x as u128) % p as u128) as u64
        }
    }
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    let (d, borrow) = a.overflowing_sub(b);
    select_unpredictable(borrow, d.wrapping_add(p), d)
}
fn inverse(a: u64, p: u64) -> u64 {
    let (mut old_r, mut r) = (a % p, p);
    let (mut old_s, mut s) = (1u64, 0u64);
    let mut odd = false;
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s + q * s);
        odd = !odd;
    }
    assert_eq!(old_r, 1, "prime-field nonzero pivot");
    if odd && old_s != 0 {
        p - old_s
    } else {
        old_s
    }
}
#[derive(Clone, Debug)]
pub struct Rref<const SKIP_ZERO: bool> {
    p: u64,
    columns: usize,
    pivots: Vec<(usize, Vec<u64>, u64)>,
    pub counts: Counts,
}
impl<const SKIP_ZERO: bool> Rref<SKIP_ZERO> {
    pub fn new(p: u64, columns: usize) -> Self {
        Self {
            p,
            columns,
            pivots: Vec::new(),
            counts: Counts::default(),
        }
    }
    pub fn rank(&self) -> usize {
        self.pivots.len()
    }
    /// Read-only basis access for a separately charged quotient construction.
    pub fn pivot_rows(&self) -> &[(usize, Vec<u64>, u64)] {
        &self.pivots
    }
    pub fn state_digest(&self) -> String {
        // Diagnostic validation, excluded from arithmetic counters and included
        // in process wall. Both modes hash their entire pivot state per insert.
        let mut h = blake3::Hasher::new();
        h.update(&self.p.to_le_bytes());
        h.update(&(self.columns as u64).to_le_bytes());
        for (col, row, rhs) in &self.pivots {
            h.update(&(*col as u64).to_le_bytes());
            for value in row {
                h.update(&value.to_le_bytes());
            }
            h.update(&rhs.to_le_bytes());
        }
        h.finalize().to_hex().to_string()
    }
    pub fn solved(&mut self, column: usize) -> Option<u64> {
        let (_, row, rhs) = self.pivots.iter().find(|(c, _, _)| *c == column)?;
        for (c, &v) in row.iter().enumerate() {
            self.counts.solved_coefficient_scans += 1;
            if c != column && v != 0 {
                return None;
            }
        }
        Some(*rhs)
    }
    pub fn insert(&mut self, mut row: Vec<u64>, mut rhs: u64) -> Result<bool, &'static str> {
        if row.len() != self.columns || row.iter().any(|&v| v >= self.p) || rhs >= self.p {
            return Err("reduced bounded row required");
        }
        for (col, prow, prhs) in &self.pivots {
            let f = row[*col];
            if f != 0 {
                let f = Factor::new(f, self.p);
                self.counts.factor_constructions += 1;
                for (r, &pv) in row.iter_mut().zip(prow) {
                    self.counts.forward_coefficient_scans += 1;
                    if pv != 0 {
                        *r = sub(*r, f.mul(pv, self.p), self.p);
                        self.counts.products += 1;
                        self.counts.subtractions += 1;
                    }
                }
                rhs = sub(rhs, f.mul(*prhs, self.p), self.p);
                self.counts.products += 1;
                self.counts.subtractions += 1;
            }
        }
        let mut pivot = None;
        for (c, &v) in row.iter().enumerate() {
            self.counts.pivot_selection_scans += 1;
            if v != 0 {
                pivot = Some(c);
                break;
            }
        }
        let Some(col) = pivot else {
            if rhs != 0 {
                return Err("inconsistent affine row");
            }
            return Ok(false);
        };
        let inv = Factor::new(inverse(row[col], self.p), self.p);
        self.counts.inversions += 1;
        self.counts.factor_constructions += 1;
        for v in row.iter_mut().chain(std::iter::once(&mut rhs)) {
            // Observing zeros is validation instrumentation on the baseline.
            // The candidate also uses the predicate to avoid the actual product.
            if *v == 0 {
                self.counts.zero_normalizations_observed += 1;
            }
            if SKIP_ZERO {
                self.counts.normalization_tests += 1;
                if *v == 0 {
                    continue;
                }
            }
            *v = inv.mul(*v, self.p);
            self.counts.products += 1;
            self.counts.normalization_products += 1;
        }
        let support: Vec<_> = row
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v != 0)
            .map(|(c, &v)| (c, v))
            .collect();
        for (_, prow, prhs) in &mut self.pivots {
            let f = prow[col];
            if f != 0 {
                let f = Factor::new(f, self.p);
                self.counts.factor_constructions += 1;
                for &(c, v) in &support {
                    prow[c] = sub(prow[c], f.mul(v, self.p), self.p);
                    self.counts.products += 1;
                    self.counts.subtractions += 1;
                    self.counts.backward_support_entries += 1;
                }
                *prhs = sub(*prhs, f.mul(rhs, self.p), self.p);
                self.counts.products += 1;
                self.counts.subtractions += 1;
            }
        }
        self.pivots.push((col, row, rhs));
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hand_sparse_rows_remove_only_zero_normalizations() {
        let (mut a, mut b) = (Rref::<false>::new(101, 3), Rref::<true>::new(101, 3));
        for (row, rhs) in [(vec![2, 0, 0], 0), (vec![0, 3, 0], 6), (vec![0, 0, 4], 12)] {
            assert_eq!(a.insert(row.clone(), rhs), b.insert(row, rhs));
            assert_eq!(a.state_digest(), b.state_digest());
        }
        assert_eq!((a.counts.products, b.counts.products), (12, 5));
        assert_eq!(a.counts.zero_normalizations_observed, 7);
        assert_eq!((a.counts.inversions, b.counts.inversions), (3, 3));
        assert_eq!(b.counts.normalization_tests, 12);
        assert_eq!(a.solved(2), Some(3));
        assert_eq!(b.solved(2), Some(3));
    }
    #[test]
    fn fixed_factor_matches_wide_product_route() {
        for p in [3, 101, (1u64 << 61) - 1, (1u64 << 63) + 29] {
            for w in [0, 1, 2, p / 2, p - 1] {
                let f = Factor::new(w, p);
                for x in [0, 1, 2, p / 2, p - 1] {
                    assert_eq!(f.mul(x, p), ((w as u128 * x as u128) % p as u128) as u64);
                }
            }
        }
    }
    #[test]
    fn every_prefix_preserves_state_and_known_solution_on_small_and_wide_fields() {
        let mut rng = 0x338d3591e6373661u64;
        for p in [3, 101, (1u64 << 61) - 1, (1u64 << 63) + 29] {
            for _ in 0..128 {
                let (mut a, mut b) = (Rref::<false>::new(p, 7), Rref::<true>::new(p, 7));
                let solution = [1, 2, 0, 1, 0, 2, 1];
                for _ in 0..20 {
                    let row: Vec<_> = (0..7)
                        .map(|_| {
                            rng ^= rng << 13;
                            rng ^= rng >> 7;
                            rng ^= rng << 17;
                            if rng & 3 == 0 {
                                0
                            } else {
                                rng % p
                            }
                        })
                        .collect();
                    let rhs = row.iter().zip(solution).fold(0u64, |a, (&v, s)| {
                        ((a as u128 + v as u128 * s as u128) % p as u128) as u64
                    });
                    assert_eq!(a.insert(row.clone(), rhs), b.insert(row, rhs));
                    assert_eq!(a.state_digest(), b.state_digest());
                    for (col, &expected) in solution.iter().enumerate() {
                        let x = a.solved(col);
                        assert_eq!(x, b.solved(col));
                        if let Some(x) = x {
                            assert_eq!(x, expected % p);
                        }
                    }
                    assert_eq!(
                        a.counts.products - b.counts.products,
                        a.counts.zero_normalizations_observed
                    );
                    assert_eq!(a.counts.subtractions, b.counts.subtractions);
                    assert_eq!(a.counts.inversions, b.counts.inversions);
                }
            }
        }
    }
    #[test]
    fn inconsistent_affine_rows_and_noncanonical_values_are_rejected() {
        let mut a = Rref::<true>::new(101, 2);
        a.insert(vec![1, 0], 3).unwrap();
        assert!(a.insert(vec![2, 0], 7).is_err());
        assert!(a.insert(vec![101, 0], 0).is_err());
        assert!(a.insert(vec![0], 0).is_err());
    }
}
