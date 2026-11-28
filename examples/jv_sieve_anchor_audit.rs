//! Public-only retained anchor audit; no F4, target or scalar computation.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/anchored-forced-divisor-20261124/audit.rs"]
mod audit;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_forced_divisor.rs"]
mod forced_divisor;
fn main() {
    if let Err(e) = audit::main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
