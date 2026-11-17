//! Compile the actual recovered SAT source and every existing module unit test.
//! This is a focused source check, not a successful full crypto_lib build.
#[path = "../../../src/cryptanalysis/sat.rs"]
pub mod sat;

pub mod cryptanalysis {
    pub use crate::sat;
    pub mod ecbench {
        pub use crate::{canonical, exact_scalar};
    }
}
#[path = "../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash {
    pub use crate::sha256;
}
#[path = "../../../src/cryptanalysis/ecbench/canonical.rs"]
pub mod canonical;
#[cfg(test)]
#[path = "../../../src/ml_dataset/sat_initial_packed.rs"]
mod sat_initial_packed;

#[path = "../../../src/cryptanalysis/ecbench/exact_scalar.rs"]
pub mod exact_scalar;
