//! Deterministic splitmix64 generator and seed derivation. No wall clock, no thread_rng.
pub struct Rng(pub u64);
impl Rng {
    pub fn new(seed: u64) -> Rng { Rng(seed) }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        let lim = u64::MAX - (u64::MAX % n);
        loop { let x = self.next(); if x < lim { return x % n; } }
    }
    /// Sorted uniform k-subset of 0..n.
    pub fn subset(&mut self, n: usize, k: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        for i in 0..k { let j = i + self.below((n - i) as u64) as usize; v.swap(i, j); }
        let mut s = v[..k].to_vec(); s.sort(); s
    }
}
pub fn mix(a: u64, b: u64) -> u64 { Rng::new(a ^ b.rotate_left(32) ^ 0xD1B54A32D192ED03).next() }
pub fn hash_str(s: &str) -> u64 { let mut h = 0xcbf29ce484222325u64; for b in s.bytes() { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); } h }
