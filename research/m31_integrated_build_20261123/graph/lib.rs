//! Bounded, source-pinned M31 graph development comparator.
//! Coordinate arithmetic is F_2147483647; relation arithmetic is F_2147446831.
//! The original graph engines and curve arithmetic are imported unchanged.

pub mod cryptanalysis;
pub use cryptanalysis::{large_prime_filter, large_prime_graph};

/// Preserve the original hooks and count their scope rather than eliding them.
/// Original forest helpers have separate WitnessStats and do not use these hooks.
pub mod jv_bitops {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ADDS: AtomicU64 = AtomicU64::new(0);
    static MULS: AtomicU64 = AtomicU64::new(0);
    static INVS: AtomicU64 = AtomicU64::new(0);
    static WORDS: AtomicU64 = AtomicU64::new(0);
    pub fn hook_add(_: u64) { ADDS.fetch_add(1, Ordering::Relaxed); }
    pub fn hook_mul(_: u64) { MULS.fetch_add(1, Ordering::Relaxed); }
    pub fn hook_inv(_: u64) { INVS.fetch_add(1, Ordering::Relaxed); }
    pub fn words(n: u64) { WORDS.fetch_add(n, Ordering::Relaxed); }
    pub fn reset() { for c in [&ADDS, &MULS, &INVS, &WORDS] { c.store(0, Ordering::Relaxed); } }
    pub fn snapshot() -> [u64; 4] {
        [ADDS.load(Ordering::Relaxed), MULS.load(Ordering::Relaxed),
         INVS.load(Ordering::Relaxed), WORDS.load(Ordering::Relaxed)]
    }
}
pub mod admission;
pub mod compare;
pub mod tape;

pub const COORDINATE_PRIME: u64 = 2_147_483_647;
pub const ELL: u64 = 2_147_446_831;
pub const TARGET: u32 = u32::MAX;
pub const MAX_ROWS: usize = 4096;
pub const MAX_INPUT_BYTES: u64 = 64 * 1024 * 1024;
pub const SOURCE_MANIFEST_SHA256: &str = env!("M31_GRAPH_SOURCE_MANIFEST_SHA256");

pub fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn mul(a: u64, b: u64) -> u64 { (a as u128 * b as u128 % ELL as u128) as u64 }
pub(crate) fn add(a: u64, b: u64) -> u64 { (a + b) % ELL }
pub(crate) fn sub(a: u64, b: u64) -> u64 { (a + ELL - b) % ELL }
pub(crate) fn inv(mut a: u64) -> u64 {
    assert!(a > 0 && a < ELL);
    let mut e = ELL - 2;
    let mut b = 1;
    while e > 0 { if e & 1 == 1 { b = mul(b, a); } a = mul(a, a); e >>= 1; }
    b
}
