//! Deterministic seeds: every stream is splitmix64 seeded by the master seed
//! xor FNV-1a(tag).
pub const SEED: u64 = 0x0DD7_ACE2_0261_1107;

pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64, tag: &str) -> Rng {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        Rng(seed ^ h)
    }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        // rejection sampling, exact
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let r = self.next();
            if r < zone {
                return r % n;
            }
        }
    }
}
