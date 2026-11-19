//! Pure signed-incidence unit controls; no geometric relation finder or benchmark.
pub use crypto_lib::cryptanalysis;
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
pub mod quadratic_pair_graph;

/// The controls are the unit tests of the included module: `cargo test --example <this example>`.
#[allow(dead_code)] // Cargo test harnesses exercise the included module instead.
fn main() {}
