//! SplitMix64 and seed derivation (SPEC section 3).

pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix64(self.0)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        ((self.next_u64() as u128 * n as u128) >> 64) as u64
    }
}

pub fn mix64(z: u64) -> u64 {
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn derive(p: u64, s: u64, b: u64, tag: u64) -> u64 {
    mix64(
        p.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (s.wrapping_add(1)).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ (b.wrapping_add(1)).wrapping_mul(0x1656_67B1_9E37_79F9)
            ^ tag.wrapping_mul(0xD6E8_FEB8_6659_FD93),
    )
}

pub fn hash_poly(f: &[u64]) -> u64 {
    let mut h: u64 = 0x243F_6A88_85A3_08D3;
    for &c in f {
        h = mix64(h ^ c);
    }
    h
}
