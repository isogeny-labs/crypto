//! Small deterministic RNG (splitmix64) and seed derivation. std only.

pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        let mut r = Rng(seed ^ 0x9E37_79B9_7F4A_7C15);
        r.next_u64();
        r
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in 0..n (n > 0); modulo bias is below 2^-40 for the n used here.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        self.next_u64() % n
    }
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i as u64 + 1) as usize;
            v.swap(i, j);
        }
    }
}

pub fn mix(a: u64, b: u64) -> u64 {
    let mut r = Rng(a ^ b.rotate_left(17) ^ 0xD1B5_4A32_D192_ED03);
    r.next_u64();
    r.next_u64()
}

pub fn hash_str(s: &str) -> u64 {
    let mut h = 0xCBF2_9CE4_8422_2325u64;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// Seed of one (cell, arm, replicate, half) stream: arms never share an RNG stream.
pub fn derive_seed(seed: u64, cell: &str, arm: &str, half: &str, rep: u64) -> u64 {
    mix(
        mix(
            mix(mix(seed, hash_str(cell)), hash_str(arm)),
            hash_str(half),
        ),
        rep,
    )
}
