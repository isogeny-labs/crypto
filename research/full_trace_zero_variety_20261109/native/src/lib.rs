//! Integration tests live here so production dependency modules compile with
//! their actual non-test APIs, without the unrelated root-library test graph.
pub use full_trace_zero_backend::*;
#[cfg(test)]
mod tests;
