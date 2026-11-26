//! Exact-source qualification and separately frozen H6 unit count pilot.
use cryptanalysis::{f4_fp, residual_walk};
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/forced-divisor-h6-20261124/unit-fiber-01/driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_forced_divisor.rs"]
mod jv_forced_divisor;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_forced_divisor_unit.rs"]
mod jv_forced_divisor_unit;
fn main() {
    if let Err(error) = driver::main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
