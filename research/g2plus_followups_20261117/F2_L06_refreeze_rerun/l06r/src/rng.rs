pub struct SplitMix64 {
    pub state: u64,
    /// number of next() calls made on this stream (F2: recorded for the stream-disjointness gate G9)
    pub count: u64,
}
impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64 { state: seed, count: 0 }
    }
    pub fn next(&mut self) -> u64 {
        self.count += 1;
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn uniform(&mut self, n: u64) -> u64 {
        assert!(n >= 1);
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let x = self.next();
            if x < zone {
                return x % n;
            }
        }
    }
    pub fn bit(&mut self) -> u64 {
        self.next() >> 63
    }
}
/// F2 seed tags (SPEC.md section 2). TAG_L06 is yesterday's tag, kept only for the
/// collision check and for unit tests that reproduce yesterday's curves.
pub const TAG_L06: u64 = 0x4C06_0000_0000_0000;
pub const TAG_DECISION: u64 = 0x4C26_0000_0000_0000;
pub const TAG_PILOT: u64 = 0x4C36_0000_0000_0000;
pub const TAG_GATE: u64 = 0x4C46_0000_0000_0000;
/// Stream separators (XOR into the curve seed).
pub const X_ORDER: u64 = 0xD1B5_4A32_D192_ED03;
pub const X_G2: u64 = 0x8CB9_2BA7_2F3D_8DD7;
pub const X_RES: u64 = 0x2545_F491_4F6C_DD1D;

pub fn tag_by_name(name: &str) -> u64 {
    match name {
        "decision" => TAG_DECISION,
        "pilot" => TAG_PILOT,
        "gate" => TAG_GATE,
        "l06" => TAG_L06,
        _ => panic!("unknown tag {}", name),
    }
}

pub fn curve_seed_t(tag: u64, p: u64, i: u64) -> u64 {
    assert!(p < (1 << 13) && i < (1 << 24));
    tag ^ (p << 24) ^ i
}

/// yesterday's seed rule (tag 0x4C06), used by the copied unit tests
pub fn curve_seed(p: u64, i: u64) -> u64 {
    curve_seed_t(TAG_L06, p, i)
}
