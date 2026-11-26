//! H6 exact-source fiber controls and count-only pilot. Run saved fibers only
//! after their own source/binary/input protocol is published.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/forced-divisor-h6-20261124/fiber-pilot-01/driver.rs"]
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
