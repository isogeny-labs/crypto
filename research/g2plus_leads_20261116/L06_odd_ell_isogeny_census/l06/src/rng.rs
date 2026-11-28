pub struct SplitMix64 {
    pub state: u64,
}
impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64 { state: seed }
    }
    pub fn next(&mut self) -> u64 {
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
pub fn curve_seed(p: u64, i: u64) -> u64 {
    0x4C06_0000_0000_0000u64 ^ (p << 24) ^ i
}
