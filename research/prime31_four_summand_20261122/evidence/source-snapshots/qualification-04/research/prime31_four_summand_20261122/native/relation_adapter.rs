//! One-public-target consumer for signed relations and authentic right-hand sides.
//!
//! The curve operations are the existing `prime31_algebraic_ic::Arithmetic`.
//! The echelon reduction follows `Rank::add` in that crate's `native/lib.rs`
//! and the affine-form row-space test of Run69/Run72. This additive adapter
//! generalizes the row length; it does not copy their pair dictionary, target
//! search, precomputed base logs, 32-target panel, or timing conclusions.
//!
//! For an accepted public equality
//! `sum_i c_i P_i = [a]G + [b]Q`, the stored equation is
//! `[c_0, ..., c_(B-1), -b | a]` over the subgroup field. A target is
//! recovered only when its coordinate functional belongs to this row space,
//! followed by a fresh `[d]G == Q` check using the original arithmetic.
//!
//! These are development counters. In particular, allocation, input parsing,
//! and the inherited arithmetic's uninstrumented operations are not assigned
//! a price here. This module alone cannot supply a complete-cost comparison.

use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Curve, Point};
use serde::Serialize;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;

/// Signed multiplicities in the exact supplied factor-base order.
/// Repeated indices are allowed. The input represents at most four leaves.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SignedRelation {
    pub terms: Vec<(usize, i64)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum AdapterError {
    InvalidCurve(String),
    InvalidTarget,
    InvalidBasePoint(usize),
    DuplicateBasePoint(usize),
    InvalidIndex(usize),
    TooManySignedLeaves,
    InvalidRow,
    GroupEqualityFailed,
    ContradictoryRow,
    ScalarReplayFailed,
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AdapterError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Disposition {
    Duplicate,
    Dependent,
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RelationReceipt {
    /// Combined and sorted integer coefficients, with a positive first term.
    /// If the input was globally negated, `a` and `b` were negated too.
    pub canonical_terms: Vec<(usize, i64)>,
    pub a: u64,
    pub b: u64,
    pub disposition: Disposition,
    pub rank_before: usize,
    pub rank_after: usize,
    pub recovered_scalar: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ModNWork {
    pub additions: u64,
    pub subtractions: u64,
    pub multiplications: u64,
    /// Fermat inversions; their square-and-multiply products are also counted.
    pub inversions: u64,
    pub coefficient_scans: u64,
    pub coefficient_updates: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct AdapterWork {
    pub submitted: u64,
    pub rejected: u64,
    pub group_equalities_checked: u64,
    pub duplicates: u64,
    pub dependent: u64,
    pub independent: u64,
    pub scalar_replays: u64,
    pub input_signed_terms: u64,
    pub canonical_signed_terms: u64,
    pub dedup_word_comparisons: u64,
    pub distinct_rows: u64,
    /// Coefficients held by pivot rows and dedup keys; excludes tree overhead,
    /// transient vectors, allocator overhead, input points, and source code.
    pub persistent_coefficient_bytes_floor: u64,
    pub mod_n: ModNWork,
}

fn add_mod(a: u64, b: u64, n: u64, work: &mut ModNWork) -> u64 {
    work.additions += 1;
    (a + b) % n
}

fn sub_mod(a: u64, b: u64, n: u64, work: &mut ModNWork) -> u64 {
    work.subtractions += 1;
    (a + n - b) % n
}

fn mul_mod(a: u64, b: u64, n: u64, work: &mut ModNWork) -> u64 {
    work.multiplications += 1;
    ((a as u128 * b as u128) % n as u128) as u64
}

fn inverse(a: u64, n: u64, work: &mut ModNWork) -> u64 {
    debug_assert!(a != 0 && a < n);
    work.inversions += 1;
    let (mut z, mut b, mut k) = (1, a, n - 2);
    while k > 0 {
        if k & 1 != 0 {
            z = mul_mod(z, b, n, work);
        }
        b = mul_mod(b, b, n, work);
        k >>= 1;
    }
    z
}

fn negate(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}

/// Exact echelon rows over the checked prime subgroup order. No DSU proxy
/// count is used for the independence of a general four-term relation.
struct RowSpace {
    n: u64,
    variables: usize,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    work: ModNWork,
}

impl RowSpace {
    fn new(n: u64, variables: usize) -> Self {
        Self {
            n,
            variables,
            pivots: vec![None; variables],
            rank: 0,
            work: ModNWork::default(),
        }
    }

    fn validate(&self, row: &[u64]) -> Result<(), AdapterError> {
        if row.len() != self.variables + 1 || row.iter().any(|&x| x >= self.n) {
            return Err(AdapterError::InvalidRow);
        }
        Ok(())
    }

    /// Canonical projective row for duplicate detection, before reduction by
    /// other equations. This identifies permutations, global sign changes,
    /// and scalar multiples without counting them as independent relations.
    fn projective_key(&mut self, row: &[u64]) -> Result<Vec<u64>, AdapterError> {
        self.validate(row)?;
        let mut pivot = None;
        for (i, &value) in row[..self.variables].iter().enumerate() {
            self.work.coefficient_scans += 1;
            if value != 0 {
                pivot = Some(i);
                break;
            }
        }
        let Some(col) = pivot else {
            return if row[self.variables] == 0 {
                Ok(row.to_vec())
            } else {
                Err(AdapterError::ContradictoryRow)
            };
        };
        let inv = inverse(row[col], self.n, &mut self.work);
        let mut key = row.to_vec();
        for value in &mut key[col..] {
            *value = mul_mod(*value, inv, self.n, &mut self.work);
            self.work.coefficient_updates += 1;
        }
        Ok(key)
    }

    fn insert(&mut self, mut row: Vec<u64>) -> Result<bool, AdapterError> {
        self.validate(&row)?;
        for col in 0..self.variables {
            self.work.coefficient_scans += 1;
            if row[col] == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[col] {
                let factor = row[col];
                for j in col..=self.variables {
                    let product = mul_mod(factor, pivot[j], self.n, &mut self.work);
                    row[j] = sub_mod(row[j], product, self.n, &mut self.work);
                    self.work.coefficient_updates += 1;
                }
            } else {
                let inv = inverse(row[col], self.n, &mut self.work);
                for value in &mut row[col..] {
                    *value = mul_mod(*value, inv, self.n, &mut self.work);
                    self.work.coefficient_updates += 1;
                }
                self.pivots[col] = Some(row);
                self.rank += 1;
                return Ok(true);
            }
        }
        if row[self.variables] != 0 {
            return Err(AdapterError::ContradictoryRow);
        }
        Ok(false)
    }

    /// Reduce the target coordinate functional, not a guessed assignment of
    /// the remaining free factor-base logarithms.
    fn target_value(&mut self) -> Option<u64> {
        let mut form = vec![0; self.variables];
        form[self.variables - 1] = 1;
        let mut value = 0;
        for col in 0..self.variables {
            self.work.coefficient_scans += 1;
            let factor = form[col];
            if factor == 0 {
                continue;
            }
            let Some(row) = &self.pivots[col] else {
                continue;
            };
            for j in col..self.variables {
                let product = mul_mod(factor, row[j], self.n, &mut self.work);
                form[j] = sub_mod(form[j], product, self.n, &mut self.work);
                self.work.coefficient_updates += 1;
            }
            let constant = mul_mod(factor, row[self.variables], self.n, &mut self.work);
            value = add_mod(value, constant, self.n, &mut self.work);
        }
        let mut determined = true;
        for coefficient in form {
            self.work.coefficient_scans += 1;
            determined &= coefficient == 0;
        }
        determined.then_some(value)
    }
}

/// Count the actual scalar comparisons performed by ordered-key lookup.
/// This does not assert that tree traversal or allocation is fully priced.
struct RowKey {
    values: Vec<u64>,
    comparisons: Rc<Cell<u64>>,
}

impl Ord for RowKey {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.values.iter().zip(&other.values) {
            self.comparisons.set(self.comparisons.get() + 1);
            match a.cmp(b) {
                Ordering::Equal => {}
                order => return order,
            }
        }
        self.comparisons.set(self.comparisons.get() + 1);
        self.values.len().cmp(&other.values.len())
    }
}

impl PartialOrd for RowKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RowKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for RowKey {}

pub struct RelationAdapter {
    curve: Curve,
    base_points: Vec<(u64, u64)>,
    target: Point,
    arithmetic: Arithmetic,
    setup_arithmetic: Counts,
    rows: RowSpace,
    seen: BTreeSet<RowKey>,
    key_comparisons: Rc<Cell<u64>>,
    recovered: Option<u64>,
    work: AdapterWork,
}

impl RelationAdapter {
    /// One target, with all base and row state freshly allocated. The caller
    /// must separately charge construction of the supplied factor base.
    pub fn new(
        curve: Curve,
        base_points: Vec<(u64, u64)>,
        target: Point,
    ) -> Result<Self, AdapterError> {
        let mut arithmetic = Arithmetic::default();
        check_curve(curve, &mut arithmetic).map_err(AdapterError::InvalidCurve)?;
        if !arithmetic.on_curve(curve, target) {
            return Err(AdapterError::InvalidTarget);
        }
        let mut points = BTreeSet::new();
        for (i, &point) in base_points.iter().enumerate() {
            if !arithmetic.on_curve(curve, Some(point)) {
                return Err(AdapterError::InvalidBasePoint(i));
            }
            if !points.insert(point) {
                return Err(AdapterError::DuplicateBasePoint(i));
            }
        }
        let variables = base_points.len() + 1;
        let setup_arithmetic = arithmetic.counts;
        let recovered = target.is_none().then_some(0);
        Ok(Self {
            curve,
            base_points,
            target,
            arithmetic,
            setup_arithmetic,
            rows: RowSpace::new(curve.n, variables),
            seen: BTreeSet::new(),
            key_comparisons: Rc::new(Cell::new(0)),
            recovered,
            work: AdapterWork::default(),
        })
    }

    pub fn rank(&self) -> usize {
        self.rows.rank
    }

    pub fn scalar(&self) -> Option<u64> {
        self.recovered
    }

    /// Cumulative original-arithmetic counters, including setup validation.
    pub fn arithmetic_counts(&self) -> Counts {
        self.arithmetic.counts
    }

    pub fn setup_arithmetic_counts(&self) -> Counts {
        self.setup_arithmetic
    }

    pub fn work(&self) -> AdapterWork {
        let mut work = self.work;
        work.mod_n = self.rows.work;
        work.dedup_word_comparisons = self.key_comparisons.get();
        work.distinct_rows = self.seen.len() as u64;
        work.persistent_coefficient_bytes_floor =
            ((self.rows.rank + self.seen.len()) * (self.rows.variables + 1) * 8) as u64;
        work
    }

    pub fn accept(
        &mut self,
        relation: &SignedRelation,
        a: u64,
        b: u64,
    ) -> Result<RelationReceipt, AdapterError> {
        self.work.submitted += 1;
        match self.accept_inner(relation, a, b) {
            Ok(receipt) => Ok(receipt),
            Err(error) => {
                self.work.rejected += 1;
                Err(error)
            }
        }
    }

    fn accept_inner(
        &mut self,
        relation: &SignedRelation,
        a: u64,
        b: u64,
    ) -> Result<RelationReceipt, AdapterError> {
        let n = self.curve.n;
        let (mut a, mut b) = (a % n, b % n);
        let mut coefficients = BTreeMap::<usize, i64>::new();
        let mut leaves = 0u64;
        for &(index, coefficient) in &relation.terms {
            self.work.input_signed_terms += 1;
            if index >= self.base_points.len() {
                return Err(AdapterError::InvalidIndex(index));
            }
            leaves = leaves
                .checked_add(coefficient.unsigned_abs())
                .ok_or(AdapterError::TooManySignedLeaves)?;
            if leaves > 4 {
                return Err(AdapterError::TooManySignedLeaves);
            }
            // The leaf bound proves this sum fits i64, including cancellations.
            *coefficients.entry(index).or_default() += coefficient;
        }
        coefficients.retain(|_, coefficient| *coefficient != 0);
        let mut canonical_terms: Vec<_> = coefficients.into_iter().collect();
        if canonical_terms.first().is_some_and(|(_, c)| *c < 0) {
            for (_, coefficient) in &mut canonical_terms {
                *coefficient = -*coefficient;
            }
            a = negate(a, n);
            b = negate(b, n);
        }
        self.work.canonical_signed_terms += canonical_terms.len() as u64;

        // Consumer-boundary replay: no harness scalar or factor-base log is
        // accepted by this API. Repeated leaves keep their exact multiplicity.
        let mut sum = None;
        for &(index, coefficient) in &canonical_terms {
            let point = Some(self.base_points[index]);
            let point = if coefficient < 0 {
                self.arithmetic.neg(self.curve, point)
            } else {
                point
            };
            for _ in 0..coefficient.unsigned_abs() {
                sum = self.arithmetic.add(self.curve, sum, point);
            }
        }
        let ag = self.arithmetic.scalar(self.curve, a, Some(self.curve.g));
        let bq = self.arithmetic.scalar(self.curve, b, self.target);
        let rhs = self.arithmetic.add(self.curve, ag, bq);
        self.work.group_equalities_checked += 1;
        if sum != rhs {
            return Err(AdapterError::GroupEqualityFailed);
        }

        let base_size = self.base_points.len();
        let mut row = vec![0; base_size + 2];
        for &(index, coefficient) in &canonical_terms {
            row[index] = coefficient.rem_euclid(n as i64) as u64;
        }
        row[base_size] = negate(b, n);
        row[base_size + 1] = a;
        let rank_before = self.rows.rank;
        let key = RowKey {
            values: self.rows.projective_key(&row)?,
            comparisons: Rc::clone(&self.key_comparisons),
        };
        let disposition = if self.seen.contains(&key) {
            self.work.duplicates += 1;
            Disposition::Duplicate
        } else {
            let independent = self.rows.insert(row)?;
            self.seen.insert(key);
            if independent {
                self.work.independent += 1;
                Disposition::Independent
            } else {
                self.work.dependent += 1;
                Disposition::Dependent
            }
        };
        if self.recovered.is_none() {
            if let Some(value) = self.rows.target_value() {
                self.work.scalar_replays += 1;
                if self
                    .arithmetic
                    .scalar(self.curve, value, Some(self.curve.g))
                    != self.target
                {
                    return Err(AdapterError::ScalarReplayFailed);
                }
                self.recovered = Some(value);
            }
        }
        Ok(RelationReceipt {
            canonical_terms,
            a,
            b,
            disposition,
            rank_before,
            rank_after: self.rows.rank,
            recovered_scalar: self.recovered,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The existing native crate's ordinary prime-order toy fixture. Known
    /// small multiples construct unit-test inputs only; the adapter receives
    /// public points and public coefficients, never these point logarithms.
    fn fixture() -> (Curve, Vec<(u64, u64)>, Point) {
        let mut ar = Arithmetic::default();
        let mut e = Curve {
            p: 1031,
            a: 447,
            b: 997,
            n: 991,
            g: (0, 0),
        };
        let rhs = ar.rhs(e, 0);
        e.g = (0, ar.sqrt(rhs, e.p).unwrap());
        check_curve(e, &mut ar).unwrap();
        let points = [2, 3, 5, 11]
            .into_iter()
            .map(|k| ar.scalar(e, k, Some(e.g)).unwrap())
            .collect();
        let q = ar.scalar(e, 7, Some(e.g));
        (e, points, q)
    }

    fn relation(terms: &[(usize, i64)]) -> SignedRelation {
        SignedRelation {
            terms: terms.to_vec(),
        }
    }

    #[test]
    fn signed_four_leaf_replay_combines_duplicates_and_global_negation() {
        let (e, points, q) = fixture();
        let mut adapter = RelationAdapter::new(e, points, q).unwrap();
        let receipt = adapter
            .accept(&relation(&[(2, 1), (0, 1), (2, -1), (1, -1)]), e.n - 1, 0)
            .unwrap();
        assert_eq!(receipt.canonical_terms, vec![(0, 1), (1, -1)]);
        assert_eq!(receipt.disposition, Disposition::Independent);
        let duplicate = adapter.accept(&relation(&[(1, 1), (0, -1)]), 1, 0).unwrap();
        assert_eq!(duplicate.disposition, Disposition::Duplicate);
        assert_eq!(adapter.rank(), 1);
        assert_eq!(adapter.scalar(), None);
    }

    #[test]
    fn exact_rank_distinguishes_duplicates_from_new_dependent_rows() {
        let (e, points, q) = fixture();
        let mut adapter = RelationAdapter::new(e, points, q).unwrap();
        assert_eq!(
            adapter
                .accept(&relation(&[(0, 1)]), 2, 0)
                .unwrap()
                .rank_after,
            1
        );
        assert_eq!(
            adapter
                .accept(&relation(&[(1, 1)]), 3, 0)
                .unwrap()
                .rank_after,
            2
        );
        assert_eq!(
            adapter
                .accept(&relation(&[(0, 1), (1, 1)]), 5, 0)
                .unwrap()
                .disposition,
            Disposition::Dependent
        );
        assert_eq!(
            adapter
                .accept(&relation(&[(0, 2)]), 4, 0)
                .unwrap()
                .disposition,
            Disposition::Duplicate
        );
        assert_eq!(adapter.rank(), 2);
        let work = adapter.work();
        assert_eq!(
            (work.independent, work.dependent, work.duplicates),
            (2, 1, 1)
        );
        assert!(work.mod_n.multiplications > 0);
        assert!(work.dedup_word_comparisons > 0);
    }

    #[test]
    fn target_linear_form_can_be_determined_before_all_base_logs() {
        let (e, points, q) = fixture();
        let mut adapter = RelationAdapter::new(e, points, q).unwrap();
        let first = adapter.accept(&relation(&[(0, 1), (2, 1)]), 0, 1).unwrap();
        assert_eq!(first.recovered_scalar, None);
        let second = adapter.accept(&relation(&[(0, 1), (2, 1)]), 7, 0).unwrap();
        assert_eq!(second.recovered_scalar, Some(7));
        assert_eq!(adapter.rank(), 2); // Five logarithm variables, only two pivots.
        assert_eq!(adapter.work().scalar_replays, 1);
        let mut independent = Arithmetic::default();
        assert_eq!(
            independent.scalar(e, adapter.scalar().unwrap(), Some(e.g)),
            q
        );
    }

    #[test]
    fn homogeneous_and_labelled_rows_recover_with_nontrivial_target_coefficient() {
        let (e, points, q) = fixture();
        let mut adapter = RelationAdapter::new(e, points, q).unwrap();
        adapter
            .accept(&relation(&[(0, 1), (1, 1), (2, -1)]), 0, 0)
            .unwrap();
        adapter.accept(&relation(&[(0, 1)]), 2, 0).unwrap();
        adapter.accept(&relation(&[(1, 1)]), 3, 0).unwrap();
        // 2*P0 + 2*P2 = 2*Q, with a target coefficient other than +/-1.
        let receipt = adapter.accept(&relation(&[(0, 2), (2, 2)]), 0, 2).unwrap();
        assert_eq!(receipt.recovered_scalar, Some(7));
        assert_eq!(adapter.rank(), 4);
    }

    #[test]
    fn false_rhs_and_bad_input_are_rejected_without_rank_mutation() {
        let (e, points, q) = fixture();
        let mut adapter = RelationAdapter::new(e, points, q).unwrap();
        assert_eq!(
            adapter.accept(&relation(&[(0, 1)]), 3, 0),
            Err(AdapterError::GroupEqualityFailed)
        );
        assert_eq!(
            adapter.accept(&relation(&[(4, 1)]), 0, 0),
            Err(AdapterError::InvalidIndex(4))
        );
        assert_eq!(
            adapter.accept(&relation(&[(0, 5)]), 10, 0),
            Err(AdapterError::TooManySignedLeaves)
        );
        assert_eq!(
            adapter.accept(&relation(&[(0, i64::MIN)]), 0, 0),
            Err(AdapterError::TooManySignedLeaves)
        );
        assert_eq!(adapter.rank(), 0);
        assert_eq!(adapter.scalar(), None);
        assert_eq!(adapter.work().rejected, 4);
    }

    #[test]
    fn contradictory_augmented_row_is_not_inserted_or_silently_ignored() {
        let mut rows = RowSpace::new(991, 2);
        assert_eq!(rows.insert(vec![1, 0, 2]), Ok(true));
        assert_eq!(
            rows.insert(vec![1, 0, 3]),
            Err(AdapterError::ContradictoryRow)
        );
        assert_eq!(rows.rank, 1);
        assert_eq!(rows.target_value(), None);
        assert_eq!(rows.insert(vec![0, 1, 7]), Ok(true));
        assert_eq!(rows.target_value(), Some(7));
    }

    #[test]
    fn identity_target_and_cancelled_relation_keep_zero_rank() {
        let (e, points, _) = fixture();
        let mut adapter = RelationAdapter::new(e, points, None).unwrap();
        assert_eq!(adapter.scalar(), Some(0));
        let receipt = adapter
            .accept(&relation(&[(0, 1), (0, -1), (1, 1), (1, -1)]), 0, 0)
            .unwrap();
        assert_eq!(receipt.disposition, Disposition::Dependent);
        assert_eq!(receipt.canonical_terms, Vec::<(usize, i64)>::new());
        assert_eq!(receipt.rank_after, 0);
        assert_eq!(receipt.recovered_scalar, Some(0));
    }
}
