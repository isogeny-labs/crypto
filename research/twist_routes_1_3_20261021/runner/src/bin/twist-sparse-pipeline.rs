//! Cold one-target twist/ordinary IC, compression and held-out experiment.
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
mod old_engine;
mod engine {
    pub use crate::old_engine::{reference, Task};
    pub fn ic(t: &Task, gp: &mut crate::pari::Pari) -> crate::Result<serde_json::Value> {
        let mut v = crate::old_engine::ic(t, gp)?;
        v["sparse_compression"] = serde_json::json!(crate::linear::sparse());
        Ok(v)
    }
}
#[path = "../sparse_20261030/linear.rs"]
mod linear;
#[path = "../cold_20261029/pari.rs"]
mod pari;
#[path = "../registry.rs"]
mod registry;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mode = std::env::var("TWIST_SPARSE_COMPRESSION").unwrap_or_else(|_| "0".into());
    if mode != "0" && mode != "1" {
        return Err("compression mode must be0/1".into());
    }
    linear::set_sparse(mode == "1");
    driver::main()
}
