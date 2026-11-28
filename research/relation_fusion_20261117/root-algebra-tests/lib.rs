//! Actual public-API cache/control correctness integration.
//! Internal recovered unit bodies run through the real root --lib target;
//! this preserves all crate-private relationships without mocks or visibility edits.
pub use crypto_lib;
