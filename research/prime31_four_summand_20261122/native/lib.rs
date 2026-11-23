//! Bounded public finite-field qualification of an RR5 four-leaf chart.
//! The upstream source closure and curve arithmetic are dependencies, unchanged.
pub mod b677_fixture;
pub mod extractor;
pub mod relation_adapter;
mod selector_circuit;

#[cfg(test)]
mod independent_tests;

// Run the new production guard tests through their public API while leaving
// unrelated historical dependency unit-test modules disabled. The build script
// copies the complete anchored test module byte-for-byte from f4_fp.rs.
#[cfg(test)]
mod f4_resource_controls {
    use crypto_lib::cryptanalysis::f4_fp::*;
    include!(concat!(env!("OUT_DIR"), "/f4_resource_controls.rs"));
}
