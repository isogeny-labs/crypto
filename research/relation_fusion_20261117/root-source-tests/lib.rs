//! Compile the actual recovered SAT source and every existing module unit test.
//! This is a focused source check, not a successful full crypto_lib build.
#[path = "../../../src/cryptanalysis/sat.rs"]
pub mod sat;

pub mod cryptanalysis {
    pub use crate::sat;
}
#[cfg(test)]
#[path = "../../../src/ml_dataset/sat_initial_packed.rs"]
mod sat_initial_packed;
