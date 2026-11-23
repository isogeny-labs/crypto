//! Isolated compilation of unchanged production source files from the current
//! checkout. Only the unrelated Joux-Vitse operation metering is shimmed.
pub mod jv_bitops {
    #[inline] pub fn hook_add(_: u64) {}
    #[inline] pub fn hook_mul(_: u64) {}
    #[inline] pub fn hook_inv(_: u64) {}
    #[inline] pub fn words(_: u64) {}
}

#[path = "../../../../src/cryptanalysis/large_prime_filter.rs"]
pub mod large_prime_filter;
#[path = "../../../../src/cryptanalysis/large_prime_graph.rs"]
pub mod large_prime_graph;
