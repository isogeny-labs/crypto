//! Private sorted-vector map for the bounded packed-constructor ablation.
//! Only the map operations used by the shared kernels are implemented.
use std::{iter::Map, ops::Index, slice, vec};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FlatMap<K: Ord, V> {
    entries: Vec<(K, V)>,
}
type Iter<'a, K, V> = Map<slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>;
fn pair_ref<K, V>((k, v): &(K, V)) -> (&K, &V) {
    (k, v)
}

impl<K: Ord, V> Default for FlatMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord, V> FlatMap<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    fn locate(&self, key: &K) -> Result<usize, usize> {
        self.entries.binary_search_by(|(k, _)| k.cmp(key))
    }
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.locate(key).ok().map(|i| &self.entries[i].1)
    }
    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.locate(key).ok().map(|i| &mut self.entries[i].1)
    }
    #[cfg(test)]
    pub(crate) fn contains_key(&self, key: &K) -> bool {
        self.locate(key).is_ok()
    }
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.locate(&key) {
            Ok(i) => Some(std::mem::replace(&mut self.entries[i].1, value)),
            Err(i) => {
                self.entries.insert(i, (key, value));
                None
            }
        }
    }
    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.locate(key).ok().map(|i| self.entries.remove(i).1)
    }
    pub(crate) fn iter(&self) -> Iter<'_, K, V> {
        self.entries.iter().map(pair_ref::<K, V>)
    }
    pub(crate) fn keys(&self) -> impl DoubleEndedIterator<Item = &K> + ExactSizeIterator {
        self.entries.iter().map(|(k, _)| k)
    }
    #[cfg(test)]
    pub(crate) fn values(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator {
        self.entries.iter().map(|(_, v)| v)
    }
    pub(crate) fn values_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = &mut V> + ExactSizeIterator {
        self.entries.iter_mut().map(|(_, v)| v)
    }
    pub(crate) fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let index = self.locate(&key);
        Entry {
            map: self,
            key,
            index,
        }
    }
}
pub(crate) struct Entry<'a, K: Ord, V> {
    map: &'a mut FlatMap<K, V>,
    key: K,
    index: Result<usize, usize>,
}
impl<'a, K: Ord, V: Default> Entry<'a, K, V> {
    pub(crate) fn or_default(self) -> &'a mut V {
        let i = match self.index {
            Ok(i) => i,
            Err(i) => {
                self.map.entries.insert(i, (self.key, V::default()));
                i
            }
        };
        &mut self.map.entries[i].1
    }
}
impl<K: Ord, V> Index<&K> for FlatMap<K, V> {
    type Output = V;
    fn index(&self, key: &K) -> &V {
        self.get(key).expect("missing flat-map key")
    }
}
impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(input: T) -> Self {
        let mut items: Vec<_> = input.into_iter().collect();
        // Stable order makes duplicate value replacement deterministic.
        items.sort_by(|(a, _), (b, _)| a.cmp(b));
        // dedup_by passes the later element first. Preserve the first key
        // and move the last value into its slot, dropping previous values.
        // This reuses the collected allocation rather than building a second Vec.
        items.dedup_by(|later, earlier| {
            if later.0.cmp(&earlier.0).is_eq() {
                std::mem::swap(&mut later.1, &mut earlier.1);
                true
            } else {
                false
            }
        });
        Self { entries: items }
    }
}
impl<K: Ord, V, const N: usize> From<[(K, V); N]> for FlatMap<K, V> {
    fn from(value: [(K, V); N]) -> Self {
        value.into_iter().collect()
    }
}
impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
impl<'a, K: Ord, V> IntoIterator for &'a FlatMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn mixed_operations_match_tree_including_removal_and_entry_defaults() {
        let mut flat = FlatMap::<u64, u64>::new();
        let mut tree = BTreeMap::<u64, u64>::new();
        let mut state = 0x4f71_ae28_932d_c610u64;
        for i in 0..6000 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let key = (state >> 32) % 67;
            match (state >> 16) % 6 {
                0 | 1 => assert_eq!(flat.insert(key, i), tree.insert(key, i)),
                2 => assert_eq!(flat.remove(&key), tree.remove(&key)),
                3 => {
                    *flat.entry(key).or_default() += i;
                    *tree.entry(key).or_default() += i;
                }
                4 => {
                    if let Some(v) = flat.get_mut(&key) {
                        *v ^= i;
                    }
                    if let Some(v) = tree.get_mut(&key) {
                        *v ^= i;
                    }
                }
                _ => {
                    for v in flat.values_mut() {
                        *v += 1;
                    }
                    for v in tree.values_mut() {
                        *v += 1;
                    }
                }
            }
            assert_eq!(flat.get(&key), tree.get(&key));
            assert_eq!(flat.contains_key(&key), tree.contains_key(&key));
            assert_eq!(flat.len(), tree.len());
            assert_eq!(
                flat.iter().collect::<Vec<_>>(),
                tree.iter().collect::<Vec<_>>()
            );
            assert_eq!(
                flat.keys().rev().collect::<Vec<_>>(),
                tree.keys().rev().collect::<Vec<_>>()
            );
            assert_eq!(
                flat.values().collect::<Vec<_>>(),
                tree.values().collect::<Vec<_>>()
            );
        }
        assert_eq!(
            flat.into_iter().collect::<Vec<_>>(),
            tree.into_iter().collect::<Vec<_>>()
        );
    }
    #[test]
    fn collection_is_sorted_unique_and_duplicate_values_match_tree() {
        let input = [
            (3, 30),
            (1, 10),
            (3, 29),
            (2, 20),
            (1, 18),
            (2, 22),
            (3, 31),
            (1, 19),
            (2, 23),
        ];
        let flat = FlatMap::from(input);
        let tree = BTreeMap::from(input);
        assert_eq!(
            flat.iter().collect::<Vec<_>>(),
            tree.iter().collect::<Vec<_>>()
        );
        assert_eq!(flat[&1], 19);
        assert_eq!(flat[&2], 23);
        assert_eq!(flat[&3], 31);
        assert!(FlatMap::<u64, u64>::default().is_empty());
    }
    #[test]
    fn insert_and_occupied_entry_preserve_the_existing_key() {
        #[derive(Debug)]
        struct Key {
            rank: u64,
            label: &'static str,
        }
        impl PartialEq for Key {
            fn eq(&self, b: &Self) -> bool {
                self.rank == b.rank
            }
        }
        impl Eq for Key {}
        impl PartialOrd for Key {
            fn partial_cmp(&self, b: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(b))
            }
        }
        impl Ord for Key {
            fn cmp(&self, b: &Self) -> std::cmp::Ordering {
                self.rank.cmp(&b.rank)
            }
        }
        let mut flat = FlatMap::new();
        let mut tree = BTreeMap::new();
        for (label, value) in [("first", 10), ("second", 20)] {
            assert_eq!(
                flat.insert(Key { rank: 1, label }, value),
                tree.insert(Key { rank: 1, label }, value)
            );
        }
        *flat
            .entry(Key {
                rank: 1,
                label: "third",
            })
            .or_default() += 1;
        *tree
            .entry(Key {
                rank: 1,
                label: "third",
            })
            .or_default() += 1;
        let (fk, fv) = flat.iter().next().unwrap();
        let (tk, tv) = tree.iter().next().unwrap();
        assert_eq!((fk.label, *fv), (tk.label, *tv));
        assert_eq!((fk.label, *fv), ("first", 21));
    }
}
