//! Compile the complete unchanged module with its original cfg(test) tests.
//! Execute only the six support-related tests by the `support` name filter.
//! The other closure modules are not compiled with cfg(test) in this target.

#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
pub mod semaev_prime_poly;
