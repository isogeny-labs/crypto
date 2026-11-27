//! Source qualification only. A saved finder requires its own frozen worker.
pub use crypto_lib::cryptanalysis;
#[path = "../src/cryptanalysis/jv_forced_divisor.rs"]
pub mod jv_forced_divisor;
#[path = "../src/cryptanalysis/jv_forced_graph.rs"]
pub mod jv_forced_graph;
fn main() {
    eprintln!("source qualification only; no saved-finder mode");
}
