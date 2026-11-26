//! Seed derivation (policy isogeny-labs/experiment-seeds/v1).
//!
//! Every logical stream is keyed by a domain string
//! `bcr-20261126/v1|<part>|<part>|...`; its 64-bit seed is the first eight
//! bytes, big-endian, of SHA-256 of that UTF-8 string. The stream itself is
//! SplitMix64 (Steele, Lea, Flood 2014; constants below) started at that
//! seed. Uniform values below `m` use rejection sampling on the full 64-bit
//! output. No wall clock, process id or hash randomisation enters any stream.
use sha2::{Digest, Sha256};

pub const STUDY: &str = "bcr-20261126/v1";

pub fn derive(parts: &[&str]) -> u64 {
    let mut s = String::from(STUDY);
    for p in parts {
        s.push('|');
        s.push_str(p);
    }
    let h = Sha256::digest(s.as_bytes());
    u64::from_be_bytes(h[0..8].try_into().unwrap())
}

#[derive(Clone, Debug)]
pub struct Sm64(pub u64);

impl Sm64 {
    pub fn keyed(parts: &[&str]) -> Self {
        Sm64(derive(parts))
    }
    #[inline]
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `[0, m)`, `m ≥ 1`, by rejection.
    pub fn below(&mut self, m: u64) -> u64 {
        assert!(m > 0);
        let lim = u64::MAX - (u64::MAX % m);
        loop {
            let x = self.next();
            if x < lim {
                return x % m;
            }
        }
    }
    pub fn bits(&mut self, n: u32) -> u64 {
        if n >= 64 {
            self.next()
        } else {
            self.next() & ((1u64 << n) - 1)
        }
    }
}
