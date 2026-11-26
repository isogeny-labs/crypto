//! Source-qualified policy pilot; saved execution needs its final protocol.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/anchored-forced-divisor-20261124/pilot-01/driver.rs"]
mod driver;
#[path = "../src/cryptanalysis/jv_forced_divisor.rs"]
pub mod jv_forced_divisor;
#[path = "../src/cryptanalysis/jv_forced_graph.rs"]
pub mod jv_forced_graph;
fn main() {
    if let Err(e) = driver::main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
