//! Public-target B8 cached Vandermonde RR9 half-chart correctness and cost.
//! Reuses original Prime31 Rust group arithmetic; still exhaustive pairwork.
#[path = "prime31_rr9_exterior4.rs"]
mod exterior;
fn main() { exterior::cached_vandermonde_main(); }
