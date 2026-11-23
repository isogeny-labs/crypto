//! Draft semantic observations for rho's actual collection operations.
//! Hash/equality delegate to the original element. The synthetic instrument
//! pins DefaultHasher rather than RandomState for reproducible semantic counts.
//! Capacity policy is unchanged; table layout and runtime may change. Does not count internal table
//! bucket scans/rehash moves or CPU instructions. Hash bytes are diagnostic.
use std::cell::Cell;
use std::hash::{Hash, Hasher};
pub type ReplayBuildHasher =
    std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>;
pub const HASH_POLICY_VERSION: u64 = 1;
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub key_comparisons: u64,
    pub entry_visits: u64,
    pub elements_copied: u64,
    pub elements_initialized: u64,
    pub hash_calls: u64,
    pub hash_bytes: u64,
}
thread_local! {static OBSERVED:Cell<Snapshot>=Cell::new(Snapshot::default());}
fn charge(f: impl FnOnce(&mut Snapshot)) {
    OBSERVED.with(|x| {
        let mut s = x.get();
        f(&mut s);
        x.set(s)
    })
}
pub fn snapshot() -> Snapshot {
    OBSERVED.with(Cell::get)
}
pub fn reset() {
    OBSERVED.with(|x| x.set(Snapshot::default()))
}
pub fn visit() {
    charge(|s| s.entry_visits = s.entry_visits.checked_add(1).expect("rho event overflow"))
}
pub fn copied() {
    charge(|s| {
        s.elements_copied = s
            .elements_copied
            .checked_add(1)
            .expect("rho event overflow")
    })
}
pub fn initialized() {
    charge(|s| {
        s.elements_initialized = s
            .elements_initialized
            .checked_add(1)
            .expect("rho event overflow")
    })
}
pub fn comparison() {
    charge(|s| {
        s.key_comparisons = s
            .key_comparisons
            .checked_add(1)
            .expect("rho event overflow")
    })
}
#[derive(Clone, Copy, Debug)]
pub struct Key<T>(pub T);
impl<T: PartialEq> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        comparison();
        self.0.eq(&other.0)
    }
}
impl<T: Eq> Eq for Key<T> {}
struct ObservedHasher<'a, H>(&'a mut H);
impl<H: Hasher> Hasher for ObservedHasher<'_, H> {
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        charge(|s| {
            s.hash_bytes = s
                .hash_bytes
                .checked_add(bytes.len() as u64)
                .expect("rho hash byte overflow")
        });
        self.0.write(bytes)
    }
    // Delegate specialized methods too: a custom Hasher may distinguish them from write.
    fn write_u8(&mut self, x: u8) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(1).unwrap());
        self.0.write_u8(x)
    }
    fn write_u16(&mut self, x: u16) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(2).unwrap());
        self.0.write_u16(x)
    }
    fn write_u32(&mut self, x: u32) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(4).unwrap());
        self.0.write_u32(x)
    }
    fn write_u64(&mut self, x: u64) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(8).unwrap());
        self.0.write_u64(x)
    }
    fn write_u128(&mut self, x: u128) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(16).unwrap());
        self.0.write_u128(x)
    }
    fn write_usize(&mut self, x: usize) {
        charge(|s| {
            s.hash_bytes = s
                .hash_bytes
                .checked_add(std::mem::size_of::<usize>() as u64)
                .unwrap()
        });
        self.0.write_usize(x)
    }
    fn write_i8(&mut self, x: i8) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(1).unwrap());
        self.0.write_i8(x)
    }
    fn write_i16(&mut self, x: i16) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(2).unwrap());
        self.0.write_i16(x)
    }
    fn write_i32(&mut self, x: i32) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(4).unwrap());
        self.0.write_i32(x)
    }
    fn write_i64(&mut self, x: i64) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(8).unwrap());
        self.0.write_i64(x)
    }
    fn write_i128(&mut self, x: i128) {
        charge(|s| s.hash_bytes = s.hash_bytes.checked_add(16).unwrap());
        self.0.write_i128(x)
    }
    fn write_isize(&mut self, x: isize) {
        charge(|s| {
            s.hash_bytes = s
                .hash_bytes
                .checked_add(std::mem::size_of::<isize>() as u64)
                .unwrap()
        });
        self.0.write_isize(x)
    }
}
impl<T: Hash> Hash for Key<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        charge(|s| s.hash_calls = s.hash_calls.checked_add(1).expect("rho hash call overflow"));
        self.0.hash(&mut ObservedHasher(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;
    use std::hash::BuildHasher;
    #[test]
    fn exact_hash_and_table_semantics() {
        let state = RandomState::new();
        for x in [(0u64, 1u64), (u64::MAX, 123)] {
            assert_eq!(state.hash_one(x), state.hash_one(Key(x)));
        }
        let mut plain = HashMap::with_hasher(state.clone());
        let mut observed = HashMap::with_hasher(state);
        for x in [3, 1, 3, 8, 2] {
            assert_eq!(plain.insert(x, x + 1), observed.insert(Key(x), x + 1));
        }
        for x in 0..10 {
            assert_eq!(plain.get(&x), observed.get(&Key(x)));
        }
    }
    #[test]
    fn calls_comparisons_and_semantic_events() {
        reset();
        assert!(Key(3) == Key(3));
        assert!(Key(3) != Key(4));
        visit();
        copied();
        initialized();
        let mut h = std::collections::hash_map::DefaultHasher::new();
        Key((1u64, 2u64)).hash(&mut h);
        let s = snapshot();
        assert_eq!(s.key_comparisons, 2);
        assert_eq!(s.entry_visits, 1);
        assert_eq!(s.elements_copied, 1);
        assert_eq!(s.elements_initialized, 1);
        assert_eq!(s.hash_calls, 1);
        assert_eq!(s.hash_bytes, 16);
    }
}
