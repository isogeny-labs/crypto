//! Deterministic multiply-rotate hasher for the internal tables of the
//! contractors (wall-only: no table here is iterated for an observable
//! result, so replacing SipHash with a cheaper hasher changes no value, row
//! count or counter).

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

const K: u64 = 0xf135_7aea_2e62_a9c5;

#[derive(Default, Clone, Copy)]
pub struct FastHasher {
    h: u64,
}

impl FastHasher {
    #[inline(always)]
    fn add(&mut self, w: u64) {
        self.h = (self.h.rotate_left(5) ^ w).wrapping_mul(K);
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut c = bytes.chunks_exact(8);
        for w in &mut c {
            self.add(u64::from_le_bytes(w.try_into().unwrap()));
        }
        let r = c.remainder();
        if !r.is_empty() {
            let mut buf = [0u8; 8];
            buf[..r.len()].copy_from_slice(r);
            self.add(u64::from_le_bytes(buf));
        }
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.h.rotate_left(26)
    }
}

pub type FastBuild = BuildHasherDefault<FastHasher>;
pub type FastMap<K, V> = HashMap<K, V, FastBuild>;
