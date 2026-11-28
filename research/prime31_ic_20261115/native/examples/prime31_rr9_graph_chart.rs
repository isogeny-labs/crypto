//! Prime31 RR9 four-plane Schur-complement benchmark.
//! Reuses original Prime31 Rust Arithmetic and public-only exterior4 module.
//! Graph chart performs exhaustive algebraic pair tests, not a nonenumerative DLP.
#[path = "prime31_rr9_exterior4.rs"]
mod exterior;
fn main() { exterior::graph_chart_main(); }
