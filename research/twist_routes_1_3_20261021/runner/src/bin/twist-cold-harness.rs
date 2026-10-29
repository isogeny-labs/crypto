//! Native frozen-plan execution, single-event ROI audit and cost aggregation.
#![allow(dead_code)]
#[path = "../cold_20261029/arithmetic.rs"]
mod arithmetic;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
mod toy_field {
    pub use crate::arithmetic::inv;
}
#[path = "../cold_20261029/driver.rs"]
mod driver;
#[path = "../cold_20261029/engine.rs"]
mod engine;
#[path = "../cold_harness_20261029.rs"]
mod harness;
#[path = "../cold_20261029/linear.rs"]
mod linear;
#[path = "../cold_20261029/pari.rs"]
mod pari;
#[path = "../registry.rs"]
mod registry;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    harness::main()
}
