//! Scoped exact-source H6 qualification and read-only planted geometry.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/forced-divisor-h6-20261124/driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_forced_divisor.rs"]
mod forced_divisor;
fn main() {
    if let Err(error) = driver::main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
