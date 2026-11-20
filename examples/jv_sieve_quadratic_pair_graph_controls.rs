//! Pure signed-incidence unit controls; no geometric relation finder or benchmark.
pub use crypto_lib::cryptanalysis;
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
pub mod quadratic_pair_graph;

/// The controls are the unit tests of the included module: `cargo test --example <this example>`.
/// Built as `crate-type = ["lib"]`, so `main` is unused; keep it for harness docs.
#[allow(dead_code)]
fn main() {}
