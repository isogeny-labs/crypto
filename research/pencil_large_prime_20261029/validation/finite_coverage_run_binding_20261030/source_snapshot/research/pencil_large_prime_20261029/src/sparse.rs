//! Ordered map with measured comparison and traversal events. Ordering is
//! exactly usize ordering; borrowed lookups use the same counted key type.
//! Serialization accesses the underlying map without metering output work.
use crate::meter;
use std::cmp::Ordering;
use std::collections::{btree_map, BTreeMap};

#[derive(Clone, Copy, Debug, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
#[cfg_attr(feature = "reporting", serde(transparent))]
struct Key(usize);
impl PartialEq for Key {
    fn eq(&self, rhs: &Self) -> bool {
        meter::charge(|c| c.map_key_comparisons += 1);
        self.0 == rhs.0
    }
}
impl Ord for Key {
    fn cmp(&self, rhs: &Self) -> Ordering {
        meter::charge(|c| c.map_key_comparisons += 1);
        self.0.cmp(&rhs.0)
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
#[cfg_attr(feature = "reporting", serde(transparent))]
pub struct TrackedMap<V>(BTreeMap<Key, V>);

impl<V> Default for TrackedMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<V: Clone> Clone for TrackedMap<V> {
    fn clone(&self) -> Self {
        meter::charge(|c| c.map_cloned_entries += self.len() as u128);
        Self(self.0.clone())
    }
}
impl<V> TrackedMap<V> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get(&self, key: &usize) -> Option<&V> {
        meter::charge(|c| c.map_calls += 1);
        let result = self.0.get(&Key(*key));
        meter::charge(|c| c.map_entry_visits += u128::from(result.is_some()));
        result
    }
    pub fn contains_key(&self, key: &usize) -> bool {
        meter::charge(|c| c.map_calls += 1);
        self.0.contains_key(&Key(*key))
    }
    pub fn insert(&mut self, key: usize, value: V) -> Option<V> {
        meter::charge(|c| c.map_calls += 1);
        let previous = self.0.insert(Key(key), value);
        if previous.is_none() {
            meter::collection(0, 0, 1, 0);
        } else {
            meter::charge(|c| c.map_entry_visits += 1);
        }
        previous
    }
    pub fn remove(&mut self, key: &usize) -> Option<V> {
        meter::charge(|c| c.map_calls += 1);
        let result = self.0.remove(&Key(*key));
        meter::charge(|c| c.map_entry_visits += u128::from(result.is_some()));
        result
    }
    pub fn first_key_value(&self) -> Option<(&usize, &V)> {
        meter::charge(|c| c.map_calls += 1);
        self.0.first_key_value().map(|(k, v)| {
            meter::charge(|c| c.map_entry_visits += 1);
            (&k.0, v)
        })
    }
    pub fn iter(&self) -> Iter<'_, V> {
        Iter(self.0.iter())
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.0
            .values_mut()
            .inspect(|_| meter::charge(|c| c.map_entry_visits += 1))
    }
}
pub struct Iter<'a, V>(btree_map::Iter<'a, Key, V>);
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a usize, &'a V);
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| {
            meter::charge(|c| c.map_entry_visits += 1);
            (&k.0, v)
        })
    }
}
impl<V> ExactSizeIterator for Iter<'_, V> {}
impl<V> std::iter::FusedIterator for Iter<'_, V> {}
pub struct IntoIter<V>(btree_map::IntoIter<Key, V>);
impl<V> Iterator for IntoIter<V> {
    type Item = (usize, V);
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| {
            meter::charge(|c| c.map_entry_visits += 1);
            (k.0, v)
        })
    }
}
impl<V> ExactSizeIterator for IntoIter<V> {}
impl<V> std::iter::FusedIterator for IntoIter<V> {}
impl<'a, V> IntoIterator for &'a TrackedMap<V> {
    type Item = (&'a usize, &'a V);
    type IntoIter = Iter<'a, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<V> IntoIterator for TrackedMap<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}
impl<V> FromIterator<(usize, V)> for TrackedMap<V> {
    fn from_iter<T: IntoIterator<Item = (usize, V)>>(iter: T) -> Self {
        let mut out = Self::new();
        for (k, v) in iter {
            out.insert(k, v);
        }
        out
    }
}
impl<V, const N: usize> From<[(usize, V); N]> for TrackedMap<V> {
    fn from(value: [(usize, V); N]) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_returned_entries_and_new_entries_are_disjoint() {
        let mut m = TrackedMap::new();
        meter::reset();
        assert_eq!(m.insert(1, 10u64), None);
        assert_eq!(m.insert(1, 20), Some(10));
        assert_eq!(m.get(&1), Some(&20));
        assert_eq!(m.get(&2), None);
        assert!(m.contains_key(&1)); // presence only, not an exposed-entry visit
        assert_eq!(m.first_key_value(), Some((&1, &20)));
        assert_eq!(m.remove(&1), Some(20));
        assert_eq!(m.remove(&1), None);
        assert_eq!(m.first_key_value(), None);
        let c = meter::snapshot();
        assert_eq!(c.collection_elements_initialized, 1);
        assert_eq!(c.map_entry_visits, 4); // replacement, get, first, removal
        assert_eq!(c.collection_elements_copied, 0); // values moved or borrowed here
    }

    #[test]
    fn key_equality_has_one_event_without_an_ordering_event() {
        meter::reset();
        assert!(Key(7) == Key(7));
        assert!(Key(7) != Key(8));
        assert_eq!(meter::snapshot().map_key_comparisons, 2);
        let left: TrackedMap<u64> = [(7, 1)].into();
        let right: TrackedMap<u64> = [(7, 1)].into();
        meter::reset();
        assert_eq!(left, right);
        assert_eq!(meter::snapshot().map_key_comparisons, 1);
    }
    #[test]
    fn counted_lookup_and_iteration_preserve_numeric_keys() {
        let mut m = TrackedMap::new();
        m.insert(17, 91u64);
        assert_eq!(m.iter().size_hint(), (1, Some(1)));
        assert_eq!(m.iter().len(), 1);
        assert_eq!(m.clone().into_iter().size_hint(), (1, Some(1)));
        meter::reset();
        assert_eq!(m.get(&17), Some(&91));
        let c = meter::snapshot();
        assert_eq!((c.map_calls, c.map_key_comparisons), (1, 1));
        meter::reset();
        assert_eq!(
            m.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
            vec![(17, 91)]
        );
        assert_eq!(meter::snapshot().map_entry_visits, 1);
        assert_eq!(m.insert(17, 3), Some(91));
        assert_eq!(m.remove(&17), Some(3));
        assert!(m.is_empty());
    }
    #[test]
    #[cfg(feature = "reporting")]
    fn serialization_keeps_schema_without_counting_output() {
        let m: TrackedMap<u64> = [(17, 91), (2, 5)].into();
        meter::reset();
        assert_eq!(serde_json::to_string(&m).unwrap(), "{\"2\":5,\"17\":91}");
        assert_eq!(meter::snapshot(), meter::Snapshot::default());
    }
}
