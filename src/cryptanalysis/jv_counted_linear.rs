//! Counted adapter around the unchanged incremental modular relation system.
//! The existing inverse is already hooked. Local product/subtraction paths
//! are charged from the documented exact legacy product count; native scans
//! and fixed-factor quotient/setup remain outside these arithmetic counts.
use crate::cryptanalysis::{jv_bitops, residual_walk::RelationSystem};

#[derive(Clone, Debug)]
pub struct CountedRelationSystem {
    inner: RelationSystem,
    modulus: u64,
    columns: usize,
}
impl CountedRelationSystem {
    /// As with RelationSystem, modulus must be prime and coefficients reduced.
    pub fn new(modulus: u64, columns: usize) -> Self {
        Self {
            inner: RelationSystem::new(modulus, columns),
            modulus,
            columns,
        }
    }
    pub fn rank(&self) -> usize {
        self.inner.rank()
    }
    pub(crate) fn pivot_rows(&self) -> &[(usize, Vec<u64>, u64)] {
        self.inner.pivot_rows()
    }
    pub fn ops(&self) -> u64 {
        self.inner.ops()
    }
    pub fn solved(&self, column: usize) -> Option<u64> {
        self.inner.solved(column)
    }
    pub fn normalization_products(&self) -> u64 {
        (self.rank() as u64)
            .checked_mul(self.columns as u64 + 1)
            .expect("bounded relation system")
    }
    pub fn subtraction_ops(&self) -> u64 {
        self.ops()
            .checked_sub(self.normalization_products())
            .expect("normalization is part of legacy products")
    }
    pub fn insert(&mut self, row: Vec<u64>, rhs: u64) -> bool {
        let before = self.inner.ops();
        let independent = self.inner.insert(row, rhs);
        let products = self
            .inner
            .ops()
            .checked_sub(before)
            .expect("legacy operation tally cannot regress");
        // Every forward/backward elimination product is paired with one
        // subtraction, including the RHS. Pivot normalization has cols+1
        // products (all coefficients and RHS) and no modular subtraction.
        let normalization = if independent {
            self.columns as u64 + 1
        } else {
            0
        };
        let additions = products
            .checked_sub(normalization)
            .expect("normalization counted by legacy operations");
        jv_bitops::hook_muls(self.modulus, products);
        if additions != 0 {
            jv_bitops::hook_adds(self.modulus, additions);
        }
        independent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jv_bitops::{Kind, Phase};

    #[test]
    fn two_equations_have_eleven_products_five_subtractions_two_inverses() {
        jv_bitops::enable(true);
        jv_bitops::claim();
        jv_bitops::set_moduli(103, 101);
        let _g = jv_bitops::enter(Phase::LinearAlgebra);
        let before = jv_bitops::snapshot();
        let mut system = CountedRelationSystem::new(101, 2);
        assert!(system.insert(vec![1, 2], 5));
        assert!(system.insert(vec![2, 1], 7));
        assert_eq!(system.solved(0), Some(3));
        assert_eq!(system.solved(1), Some(1));
        assert_eq!(system.ops(), 11);
        assert_eq!(system.subtraction_ops(), 5);
        let t = jv_bitops::snapshot().since(&before).total();
        assert_eq!(t.get(Kind::ModrMul), 11);
        assert_eq!(t.get(Kind::ModrAdd), 5);
        assert_eq!(t.get(Kind::ModrInv), 2);
    }

    #[test]
    fn adapter_preserves_values_and_legacy_ops_on_small_and_wide_primes() {
        jv_bitops::enable(true);
        jv_bitops::claim();
        let _g = jv_bitops::enter(Phase::LinearAlgebra);
        for n in [3, 101, (1u64 << 61) - 1, (1u64 << 63) + 29] {
            jv_bitops::set_moduli(103, n);
            let mut plain = RelationSystem::new(n, 3);
            let mut counted = CountedRelationSystem::new(n, 3);
            for (row, rhs) in [
                (vec![1, 2, 0], 5),
                (vec![2, 1, 1], 7),
                (vec![0, 1, 1], 4),
                (vec![3, 3, 1], 12),
                (vec![0, 0, 0], 0),
            ] {
                let row: Vec<_> = row.into_iter().map(|v| v % n).collect();
                let rhs = rhs % n;
                let expected = plain.insert(row.clone(), rhs);
                let old_ops = counted.ops();
                let old_rank = counted.rank();
                let before = jv_bitops::snapshot();
                assert_eq!(counted.insert(row, rhs), expected);
                assert_eq!(plain.ops(), counted.ops());
                assert_eq!(plain.rank(), counted.rank());
                for col in 0..3 {
                    assert_eq!(plain.solved(col), counted.solved(col));
                }
                let delta = jv_bitops::snapshot().since(&before).total();
                let products = counted.ops() - old_ops;
                assert_eq!(delta.get(Kind::ModrMul), products);
                assert_eq!(
                    delta.get(Kind::ModrAdd),
                    products - if expected { 4 } else { 0 }
                );
                assert_eq!(delta.get(Kind::ModrInv), (counted.rank() - old_rank) as u64);
            }
        }
    }

    #[test]
    fn batch_products_preserve_field_scalar_and_other_classification() {
        jv_bitops::enable(true);
        jv_bitops::claim();
        jv_bitops::set_moduli(103, 101);
        let _g = jv_bitops::enter(Phase::LinearAlgebra);
        let before = jv_bitops::snapshot();
        let other = jv_bitops::snapshot_other();
        jv_bitops::hook_muls(103, 3);
        jv_bitops::hook_muls(101, 5);
        jv_bitops::hook_muls(107, 7);
        jv_bitops::hook_muls(107, 0);
        let delta = jv_bitops::snapshot().since(&before).total();
        assert_eq!(delta.get(Kind::FieldMul), 3);
        assert_eq!(delta.get(Kind::ModrMul), 5);
        assert_eq!(
            jv_bitops::snapshot_other()[Phase::LinearAlgebra as usize]
                - other[Phase::LinearAlgebra as usize],
            7
        );
    }
}
