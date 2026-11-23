//! Bounded public finite-field qualification of an RR5 four-leaf chart.
//! The upstream source closure and curve arithmetic are dependencies, unchanged.
pub mod extractor;
pub mod identity_record;
pub mod relation_adapter;
pub mod solver;

#[cfg(test)]
mod independent_tests;

#[cfg(test)]
mod selector_certificate;
