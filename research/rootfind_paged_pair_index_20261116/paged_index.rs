//! Standalone screen for a sparse direct index over binary pair-sum keys.
//!
//! The producer's first witness wins. A later pair under the same key
//! receives that witness so the caller can retain its collision stream.
//! This file is not wired into the elliptic-curve collector.

const PAGE_BITS: u32 = 4;
const PAGE_SIZE: usize = 1 << PAGE_BITS;

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
    PageCapacityExceeded,
}

/// A four-byte directory and fixed backing for 16 eight-byte words per page.
/// `directory[p] = 0` means absent; otherwise it is `page_index + 1`.
pub struct PagedPairIndex {
    max_key: u32,
    directory: Vec<u32>,
    pages: Vec<[u64; PAGE_SIZE]>,
    used_pages: usize,
    entries: usize,
}

impl PagedPairIndex {
    /// `page_capacity` is a pre-run upper bound on distinct occupied pages.
    /// The binary collector can supply `a * a + 1` for `a` abscissae.
    pub fn new(n: u32, page_capacity: usize) -> Result<Self, IndexError> {
        if !(1..=23).contains(&n) {
            return Err(IndexError::UnsupportedDegree);
        }
        let max_key = 1u32 << n;
        let page_count = ((max_key as usize) >> PAGE_BITS) + 1;
        let capacity = page_capacity.min(page_count);
        Ok(Self {
            max_key,
            directory: vec![0; page_count],
            pages: vec![[0; PAGE_SIZE]; capacity],
            used_pages: 0,
            entries: 0,
        })
    }

    /// A bound supplied by the current folded pair-builder geometry.
    pub fn from_base_abscissae(n: u32, a: usize) -> Result<Self, IndexError> {
        let capacity = a
            .checked_mul(a)
            .and_then(|square| square.checked_add(1))
            .ok_or(IndexError::PageCapacityExceeded)?;
        Self::new(n, capacity)
    }

    #[inline]
    fn unpack(word: u64) -> (u32, u32) {
        debug_assert_ne!(word, 0);
        ((word >> 32) as u32 - 1, word as u32)
    }

    pub fn insert(&mut self, key: u32, i: u32, j: u32) -> Result<Insert, IndexError> {
        if key == 0 || key > self.max_key {
            return Err(IndexError::InvalidKey);
        }
        if i == u32::MAX {
            return Err(IndexError::InvalidWitness);
        }
        let page_number = (key >> PAGE_BITS) as usize;
        let slot = (key as usize) & (PAGE_SIZE - 1);
        let page_index = if self.directory[page_number] == 0 {
            if self.used_pages == self.pages.len() {
                return Err(IndexError::PageCapacityExceeded);
            }
            let index = self.used_pages;
            self.used_pages += 1;
            self.directory[page_number] = (index + 1) as u32;
            index
        } else {
            (self.directory[page_number] - 1) as usize
        };
        let word = &mut self.pages[page_index][slot];
        if *word != 0 {
            let (first_i, first_j) = Self::unpack(*word);
            return Ok(Insert::Existing { first_i, first_j });
        }
        *word = ((u64::from(i) + 1) << 32) | u64::from(j);
        self.entries += 1;
        Ok(Insert::New)
    }

    pub fn get(&self, key: u32) -> Option<(u32, u32)> {
        if key == 0 || key > self.max_key {
            return None;
        }
        let page_number = (key >> PAGE_BITS) as usize;
        let page_index_plus_one = self.directory[page_number];
        if page_index_plus_one == 0 {
            return None;
        }
        let word = self.pages[(page_index_plus_one - 1) as usize][(key as usize) & (PAGE_SIZE - 1)];
        (word != 0).then(|| Self::unpack(word))
    }

    pub fn entries(&self) -> usize {
        self.entries
    }

    pub fn used_pages(&self) -> usize {
        self.used_pages
    }

    pub fn page_capacity(&self) -> usize {
        self.pages.len()
    }

    pub fn directory_words(&self) -> usize {
        self.directory.len()
    }

    /// Fixed requested backing bytes, excluding Vec headers and allocator overhead.
    pub fn requested_backing_bytes(&self) -> usize {
        self.directory.len() * std::mem::size_of::<u32>()
            + self.pages.len() * std::mem::size_of::<[u64; PAGE_SIZE]>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn every_k13_key_and_collision_stream_match_hash_map() {
        let mut index = PagedPairIndex::new(13, 513).unwrap();
        let mut reference = HashMap::<u32, (u32, u32)>::new();
        let mut reference_collisions = Vec::new();
        let mut paged_collisions = Vec::new();
        let mut state = 0x5310_3515_2026_1116u64;
        for row in 0..16_384u32 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let key = 1 + ((state as u32) & 8191);
            let current = (row, row.wrapping_mul(17));
            if let Some(&(fi, fj)) = reference.get(&key) {
                reference_collisions.push((key, current.0, current.1, fi, fj));
            } else {
                reference.insert(key, current);
            }
            match index.insert(key, current.0, current.1).unwrap() {
                Insert::New => assert_eq!(reference.get(&key), Some(&current)),
                Insert::Existing { first_i, first_j } => {
                    paged_collisions.push((key, current.0, current.1, first_i, first_j));
                }
            }
        }
        assert_eq!(paged_collisions, reference_collisions);
        assert_eq!(index.entries(), reference.len());
        for key in 1..=8192 {
            assert_eq!(index.get(key), reference.get(&key).copied(), "key={key}");
        }
    }

    #[test]
    fn boundary_keys_and_full_low_word() {
        let mut index = PagedPairIndex::new(13, 2).unwrap();
        assert_eq!(index.insert(1, 0, u32::MAX), Ok(Insert::New));
        assert_eq!(index.get(1), Some((0, u32::MAX)));
        assert_eq!(index.insert(8192, 19, 23), Ok(Insert::New));
        assert_eq!(index.get(8192), Some((19, 23)));
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(0), None);
        assert_eq!(index.get(8193), None);
        assert_eq!(index.directory_words(), 513);
        assert_eq!(index.requested_backing_bytes(), 513 * 4 + 2 * 128);
    }

    #[test]
    fn refusals_preserve_existing_entries() {
        assert!(matches!(
            PagedPairIndex::new(0, 1),
            Err(IndexError::UnsupportedDegree)
        ));
        assert!(matches!(
            PagedPairIndex::new(24, 1),
            Err(IndexError::UnsupportedDegree)
        ));
        let mut index = PagedPairIndex::new(13, 1).unwrap();
        assert_eq!(index.insert(1, 7, 8), Ok(Insert::New));
        assert_eq!(index.insert(0, 9, 10), Err(IndexError::InvalidKey));
        assert_eq!(index.insert(8193, 9, 10), Err(IndexError::InvalidKey));
        assert_eq!(
            index.insert(1, u32::MAX, 10),
            Err(IndexError::InvalidWitness)
        );
        assert_eq!(
            index.insert(17, 9, 10),
            Err(IndexError::PageCapacityExceeded)
        );
        assert_eq!(index.used_pages(), 1);
        assert_eq!(index.entries(), 1);
        assert_eq!(index.get(1), Some((7, 8)));
        assert_eq!(index.get(17), None);
    }

    #[test]
    fn fixed_backing_requests_match_predeclared_cells() {
        for (n, a, words, cap, bytes) in [
            (13, 17, 513, 290, 39_172),
            (19, 70, 32_769, 4_901, 758_404),
            (23, 138, 524_289, 19_045, 4_534_916),
        ] {
            let index = PagedPairIndex::from_base_abscissae(n, a).unwrap();
            assert_eq!(index.directory_words(), words);
            assert_eq!(index.page_capacity(), cap);
            assert_eq!(index.requested_backing_bytes(), bytes);
        }
    }
}
