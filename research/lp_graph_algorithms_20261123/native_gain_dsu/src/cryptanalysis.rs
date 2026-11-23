//! Source-exact original modules, symlinked into this small Cargo crate by
//! run.sh. The production files (and their nested children) stay unmodified.
//! Only unrelated Joux-Vitse operation metering is shimmed.
pub mod jv_bitops {
    #[inline] pub fn hook_add(_: u64) {}
    #[inline] pub fn hook_mul(_: u64) {}
    #[inline] pub fn hook_inv(_: u64) {}
    #[inline] pub fn words(_: u64) {}
}

pub mod large_prime_filter;
pub mod large_prime_graph;
