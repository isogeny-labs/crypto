//! Preserve std BTreeSet ordering and bulk construction while observing comparisons
//! and consumed entries. API calls are diagnostic, never additional primitive cost.
use crate::meter;
use std::cmp::Ordering;
use std::collections::{btree_set, BTreeSet};
#[derive(Clone, Copy, Debug)]
struct Key(u64);
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        meter::charge(|c| c.map_key_comparisons += 1);
        self.0 == other.0
    }
}
impl Eq for Key {}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        meter::charge(|c| c.map_key_comparisons += 1);
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Default, Debug)]
pub struct TrackedSet(BTreeSet<Key>);
impl TrackedSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn insert(&mut self, value: u64) -> bool {
        meter::charge(|c| c.map_calls += 1);
        let inserted = self.0.insert(Key(value));
        if inserted {
            meter::collection(0, 0, 1, 0);
        }
        inserted
    }
}
impl Extend<u64> for TrackedSet {
    fn extend<T: IntoIterator<Item = u64>>(&mut self, values: T) {
        meter::charge(|c| c.map_calls += 1);
        let before = self.0.len();
        self.0.extend(values.into_iter().map(Key));
        meter::collection(0, 0, self.0.len() - before, 0);
    }
}
impl FromIterator<u64> for TrackedSet {
    fn from_iter<T: IntoIterator<Item = u64>>(values: T) -> Self {
        meter::charge(|c| c.map_calls += 1);
        let result = Self(values.into_iter().map(Key).collect());
        meter::collection(0, 0, result.len(), 0);
        result
    }
}
pub struct IntoIter(btree_set::IntoIter<Key>);
impl Iterator for IntoIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.0.next().map(|k| {
            meter::charge(|c| c.map_entry_visits += 1);
            k.0
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<u64> {
        self.0.next_back().map(|k| {
            meter::charge(|c| c.map_entry_visits += 1);
            k.0
        })
    }
}
impl ExactSizeIterator for IntoIter {}
impl std::iter::FusedIterator for IntoIter {}
impl IntoIterator for TrackedSet {
    type Item = u64;
    type IntoIter = IntoIter;
    fn into_iter(self) -> IntoIter {
        IntoIter(self.0.into_iter())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bulk_duplicate_initialization_keeps_std_bulk_semantics() {
        meter::reset();
        let mut s: TrackedSet = [3, 1, 3].into_iter().collect();
        assert_eq!(meter::snapshot().collection_elements_initialized, 2);
        s.extend([1, 4, 4]);
        assert_eq!(meter::snapshot().collection_elements_initialized, 3);
        assert!(!s.insert(4));
        assert!(s.insert(5));
        assert_eq!(meter::snapshot().collection_elements_initialized, 4);
        assert_eq!(meter::snapshot().map_entry_visits, 0);
        assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![1, 3, 4, 5]);
        assert_eq!(meter::snapshot().map_entry_visits, 4);
    }

    #[test]
    fn duplicate_insert_and_partial_consumption_have_exact_events() {
        meter::reset();
        let mut s = TrackedSet::new();
        assert!(s.insert(3));
        assert!(s.insert(1));
        assert!(!s.insert(3));
        let observed = meter::snapshot();
        assert_eq!(observed.map_key_comparisons, 3);
        assert_eq!(observed.map_calls, 3);
        assert_eq!(observed.map_entry_visits, 0);
        let mut it = s.into_iter();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back(), Some(3));
        drop(it);
        assert_eq!(meter::snapshot().map_entry_visits, 1);
    }
    #[test]
    fn equality_is_observed_including_bulk_duplicate_elimination() {
        meter::reset();
        assert_eq!(Key(5), Key(5));
        assert_ne!(Key(5), Key(6));
        assert_eq!(meter::snapshot().map_key_comparisons, 2);
        meter::reset();
        let s: TrackedSet = [5, 5].into_iter().collect();
        assert_eq!(s.len(), 1);
        // Two-element sorted construction: one order test, one equality dedup.
        assert_eq!(meter::snapshot().map_key_comparisons, 2);
    }
    #[test]
    fn bulk_construction_extend_and_order_match_standard_set() {
        for values in [vec![], vec![0], vec![5, 1, 5, 0, u64::MAX, 2, 2]] {
            let mut plain: BTreeSet<_> = values.iter().copied().collect();
            let mut tracked: TrackedSet = values.into_iter().collect();
            plain.extend([7, 0, u64::MAX]);
            tracked.extend([7, 0, u64::MAX]);
            assert_eq!(tracked.len(), plain.len());
            assert_eq!(tracked.is_empty(), plain.is_empty());
            assert_eq!(
                tracked.into_iter().collect::<Vec<_>>(),
                plain.into_iter().collect::<Vec<_>>()
            );
        }
    }
}
