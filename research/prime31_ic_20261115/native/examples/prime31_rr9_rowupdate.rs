//! Exact one-leaf Grassmann rank-one chart update gate on original Prime31.
//! Reuses unchanged original Prime31 affine group arithmetic and RR9 exterior
//! / graph-chart implementation; NOT a nonenumerative full ECDLP solver.
#[path = "prime31_rr9_exterior4.rs"]
mod exterior;
fn main() { exterior::prime31_rowupdate_main(); }
