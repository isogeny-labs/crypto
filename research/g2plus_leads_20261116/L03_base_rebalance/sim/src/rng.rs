pub const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;
pub fn mix64(mut z: u64) -> u64 {
    z = z.wrapping_add(GOLDEN);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
#[derive(Clone, Debug)]
pub struct Rng {
    pub state: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        let out = mix64(self.state);
        self.state = self.state.wrapping_add(GOLDEN);
        out
    }
    /// uniform-ish in [lo, hi), hi > lo; modulo bias accepted and declared
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo)
    }
}
pub fn run_seed(p: u64, curve_seed: u64, seed: u64) -> u64 {
    mix64(
        p.wrapping_mul(1_000_003)
            ^ curve_seed.wrapping_mul(GOLDEN)
            ^ seed.wrapping_mul(0xD1B5_4A32_D192_ED03),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1_mix64() {
        assert_eq!(mix64(0), 16294208416658607535);
        assert_eq!(mix64(1), 10451216379200822465);
        assert_eq!(mix64(0x1234), 6873671191729682568);
    }
    #[test]
    fn t2_rng() {
        let mut r = Rng::new(0);
        assert_eq!(r.next_u64(), 16294208416658607535);
        assert_eq!(r.next_u64(), 7960286522194355700);
        assert_eq!(r.next_u64(), 487617019471545679);
        assert_eq!(Rng::new(0).range(5, 17), 12);
    }
}
