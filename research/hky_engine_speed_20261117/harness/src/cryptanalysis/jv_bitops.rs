//! Stub of the opt-in `bitop/v1` counters: counting is always off, which is
//! the default state of the real module, so every hook is the no-op it is
//! there when no study has turned counting on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    FieldAdd,
    FieldMul,
    FieldSqr,
    FieldInv,
    FieldSqrt,
    Legendre,
    ModrAdd,
    ModrMul,
    ModrInv,
    WordOp,
    HashProbe,
}
pub fn enabled() -> bool {
    false
}
pub fn charge(_kind: Kind, _n: u64) {}
pub fn row_ops(_n: u64) {}
pub fn words(_n: u64) {}
pub fn probes(_n: u64, _key_words: u64) {}
pub fn hook_inv(_m: u64) {}
pub fn hook_pow(_m: u64, _e: u64) {}
pub fn sort_cost(k: usize) -> u64 {
    if k < 2 {
        return 0;
    }
    let lg = usize::BITS - (k - 1).leading_zeros();
    (k as u64) * lg as u64
}
