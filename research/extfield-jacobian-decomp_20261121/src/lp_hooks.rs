//! Counting shim for the hooks `large_prime_filter` calls (`hook_add`, `hook_mul`, `hook_inv`, `words`).
//! Counters are thread-local totals; the caller prices them.
use std::cell::Cell;

thread_local! {
    static OPS: Cell<u64> = const { Cell::new(0) };
    static WORDS: Cell<u64> = const { Cell::new(0) };
}

/// Cost charged for one modular inversion (extended Euclid on a <= 64-bit modulus), in mulmod units.
pub const INV_COST: u64 = 40;

pub fn hook_add(_m: u64) {
    OPS.with(|c| c.set(c.get() + 1));
}
pub fn hook_mul(_m: u64) {
    OPS.with(|c| c.set(c.get() + 1));
}
pub fn hook_inv(_m: u64) {
    OPS.with(|c| c.set(c.get() + INV_COST));
}
pub fn words(n: u64) {
    WORDS.with(|c| c.set(c.get() + n));
}
/// Total counted filter work so far: field ops plus data words.
pub fn total() -> u64 {
    OPS.with(|c| c.get()) + WORDS.with(|c| c.get())
}
