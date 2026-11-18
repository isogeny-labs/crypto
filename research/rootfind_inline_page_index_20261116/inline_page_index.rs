//! Binary pair-sum first-witness index with inline singleton pages.
//!
//! For bases with fewer than 512 point indices, a 32-bit directory word can
//! hold one page slot and its pair witness. A second occupied slot promotes
//! the page to a 16-word spill page. This is a standalone representation
//! screen; the production collector remains unchanged.

const PAGE_BITS: u32 = 4;
const PAGE_SIZE: usize = 1 << PAGE_BITS;
const INDEX_BITS: u32 = 9;
const INDEX_LIMIT: u32 = 1 << INDEX_BITS;
const WITNESS_BITS: u32 = 19;
const WITNESS_MASK: u32 = (1 << WITNESS_BITS) - 1;
const SPILL_FLAG: u32 = 1 << 31;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Insert {
    New,
    Existing { first_i: u32, first_j: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexError {
    UnsupportedDegree,
    InvalidKey,
    InvalidWitness,
    SpillLimitExceeded,
}

/// An empty directory entry is zero. An entry with a clear high bit contains
/// the first key's four-bit slot and a nonzero packed witness; an entry with
/// the high bit set contains the one-based index of a 16-word spill page.
pub struct InlinePagePairIndex {
    max_key: u32,
    directory: Vec<u32>,
    spills: Vec<[u32; PAGE_SIZE]>,
    spill_limit: usize,
    occupied_pages: usize,
    entries: usize,
}

impl InlinePagePairIndex {
    pub fn new(n: u32, spill_limit: usize) -> Result<Self, IndexError> {
        if !(1..=23).contains(&n) {
            return Err(IndexError::UnsupportedDegree);
        }
        let max_key = 1u32 << n;
        let page_count = ((max_key as usize) >> PAGE_BITS) + 1;
        let spill_limit = spill_limit.min(page_count);
        Ok(Self {
            max_key,
            directory: vec![0; page_count],
            spills: Vec::with_capacity(spill_limit.min(512)),
            spill_limit,
            occupied_pages: 0,
            entries: 0,
        })
    }

    /// Each spill page needs at least two retained keys. The folded builder
    /// inserts at most `a*a+1` distinct keys for `a` base abscissae.
    pub fn from_base_abscissae(n: u32, a: usize) -> Result<Self, IndexError> {
        let key_bound = a
            .checked_mul(a)
            .and_then(|square| square.checked_add(1))
            .ok_or(IndexError::SpillLimitExceeded)?;
        Self::new(n, key_bound / 2)
    }

    #[inline]
    fn pack_witness(i: u32, j: u32) -> Result<u32, IndexError> {
        if i >= INDEX_LIMIT || j >= INDEX_LIMIT {
            return Err(IndexError::InvalidWitness);
        }
        Ok(((i << INDEX_BITS) | j) + 1)
    }

    #[inline]
    fn unpack_witness(code: u32) -> (u32, u32) {
        debug_assert_ne!(code, 0);
        let packed = code - 1;
        (packed >> INDEX_BITS, packed & (INDEX_LIMIT - 1))
    }

    pub fn insert(&mut self, key: u32, i: u32, j: u32) -> Result<Insert, IndexError> {
        if key == 0 || key > self.max_key {
            return Err(IndexError::InvalidKey);
        }
        let code = Self::pack_witness(i, j)?;
        let page_number = (key >> PAGE_BITS) as usize;
        let slot = (key as usize) & (PAGE_SIZE - 1);
        let entry = self.directory[page_number];
        if entry == 0 {
            self.directory[page_number] = ((slot as u32) << WITNESS_BITS) | code;
            self.occupied_pages += 1;
            self.entries += 1;
            return Ok(Insert::New);
        }
        if entry & SPILL_FLAG == 0 {
            let old_slot = ((entry >> WITNESS_BITS) as usize) & (PAGE_SIZE - 1);
            let old_code = entry & WITNESS_MASK;
            if old_slot == slot {
                let (first_i, first_j) = Self::unpack_witness(old_code);
                return Ok(Insert::Existing { first_i, first_j });
            }
            if self.spills.len() >= self.spill_limit {
                return Err(IndexError::SpillLimitExceeded);
            }
            let mut page = [0u32; PAGE_SIZE];
            page[old_slot] = old_code;
            page[slot] = code;
            self.spills.push(page);
            self.directory[page_number] = SPILL_FLAG | self.spills.len() as u32;
            self.entries += 1;
            return Ok(Insert::New);
        }
        let page_index = ((entry & !SPILL_FLAG) - 1) as usize;
        let stored = &mut self.spills[page_index][slot];
        if *stored != 0 {
            let (first_i, first_j) = Self::unpack_witness(*stored);
            return Ok(Insert::Existing { first_i, first_j });
        }
        *stored = code;
        self.entries += 1;
        Ok(Insert::New)
    }

    pub fn get(&self, key: u32) -> Option<(u32, u32)> {
        if key == 0 || key > self.max_key {
            return None;
        }
        let page_number = (key >> PAGE_BITS) as usize;
        let slot = (key as usize) & (PAGE_SIZE - 1);
        let entry = self.directory[page_number];
        if entry == 0 {
            return None;
        }
        let code = if entry & SPILL_FLAG == 0 {
            let old_slot = ((entry >> WITNESS_BITS) as usize) & (PAGE_SIZE - 1);
            if old_slot != slot {
                return None;
            }
            entry & WITNESS_MASK
        } else {
            let page_index = ((entry & !SPILL_FLAG) - 1) as usize;
            self.spills[page_index][slot]
        };
        (code != 0).then(|| Self::unpack_witness(code))
    }

    pub fn entries(&self) -> usize {
        self.entries
    }

    pub fn occupied_pages(&self) -> usize {
        self.occupied_pages
    }

    pub fn singleton_pages(&self) -> usize {
        self.occupied_pages - self.spills.len()
    }

    pub fn spill_pages(&self) -> usize {
        self.spills.len()
    }

    pub fn spill_capacity(&self) -> usize {
        self.spills.capacity()
    }

    pub fn directory_words(&self) -> usize {
        self.directory.len()
    }

    /// Requested backing, including reserved but unused spill capacity;
    /// excludes Vec headers and allocator overhead.
    pub fn requested_backing_bytes(&self) -> usize {
        self.directory.len() * std::mem::size_of::<u32>()
            + self.spills.capacity() * std::mem::size_of::<[u32; PAGE_SIZE]>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn promotion_keeps_first_witness_across_page_boundary() {
        let mut index = InlinePagePairIndex::new(13, 1).unwrap();
        for key in 1..=15u32 {
            assert_eq!(index.insert(key, key - 1, 511), Ok(Insert::New));
        }
        assert_eq!(index.insert(16, 511, 0), Ok(Insert::New));
        assert_eq!(index.spill_pages(), 1);
        assert_eq!(index.singleton_pages(), 1);
        for key in 1..=15u32 {
            assert_eq!(index.get(key), Some((key - 1, 511)));
            assert_eq!(
                index.insert(key, 500, 500),
                Ok(Insert::Existing {
                    first_i: key - 1,
                    first_j: 511
                })
            );
        }
        assert_eq!(index.get(16), Some((511, 0)));
        assert_eq!(index.entries(), 16);
    }

    #[test]
    fn every_valid_slot_in_one_page_survives_promotion() {
        let mut index = InlinePagePairIndex::new(13, 1).unwrap();
        for key in 16..32u32 {
            assert_eq!(index.insert(key, key - 16, 511), Ok(Insert::New));
        }
        assert_eq!(index.occupied_pages(), 1);
        assert_eq!(index.spill_pages(), 1);
        assert_eq!(index.singleton_pages(), 0);
        assert_eq!(index.entries(), 16);
        for key in 16..32u32 {
            assert_eq!(index.get(key), Some((key - 16, 511)));
            assert_eq!(
                index.insert(key, 500, 500),
                Ok(Insert::Existing {
                    first_i: key - 16,
                    first_j: 511
                })
            );
        }
        assert_eq!(index.get(15), None);
        assert_eq!(index.get(32), None);
    }

    #[test]
    fn refusals_leave_existing_first_witness_intact() {
        assert!(matches!(
            InlinePagePairIndex::new(0, 0),
            Err(IndexError::UnsupportedDegree)
        ));
        assert!(matches!(
            InlinePagePairIndex::new(24, 0),
            Err(IndexError::UnsupportedDegree)
        ));
        let mut index = InlinePagePairIndex::new(13, 0).unwrap();
        assert_eq!(index.insert(1, 0, 0), Ok(Insert::New));
        assert_eq!(index.insert(2, 1, 1), Err(IndexError::SpillLimitExceeded));
        assert_eq!(index.insert(0, 1, 1), Err(IndexError::InvalidKey));
        assert_eq!(index.insert(8193, 1, 1), Err(IndexError::InvalidKey));
        assert_eq!(index.insert(1, 512, 0), Err(IndexError::InvalidWitness));
        assert_eq!(index.insert(1, 0, 512), Err(IndexError::InvalidWitness));
        assert_eq!(index.get(1), Some((0, 0)));
        assert_eq!(index.get(2), None);
        assert_eq!(index.entries(), 1);
        assert_eq!(index.spill_pages(), 0);
        assert_eq!(index.singleton_pages(), 1);
    }

    #[test]
    fn deterministic_stream_matches_hash_map_and_full_k13_lookup() {
        let mut index = InlinePagePairIndex::new(13, 513).unwrap();
        let mut reference = HashMap::<u32, (u32, u32)>::new();
        let mut state = 0x5310_3515_2026_1116u64;
        for row in 0..16_384u32 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let key = 1 + ((state as u32) & 8191);
            let current = (row & 511, row.wrapping_mul(17) & 511);
            let expected = if let Some(&(first_i, first_j)) = reference.get(&key) {
                Insert::Existing { first_i, first_j }
            } else {
                reference.insert(key, current);
                Insert::New
            };
            assert_eq!(index.insert(key, current.0, current.1), Ok(expected));
        }
        assert_eq!(index.entries(), reference.len());
        for key in 1..=8192 {
            assert_eq!(index.get(key), reference.get(&key).copied());
        }
    }
}
