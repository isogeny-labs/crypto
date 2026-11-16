//! Compile the complete production modules, without replacement dependencies.
#![allow(dead_code)]

#[path = "../../src/hash/sha3.rs"]
pub mod sha3_impl;
#[path = "../../src/pqc/ml_dsa.rs"]
pub mod reference_impl;
#[path = "../../src/pqc/fast/keccak.rs"]
pub mod keccak_impl;
#[path = "../../src/pqc/fast/keccak4.rs"]
pub mod keccak4;
pub use crate::keccak_impl as keccak;

pub mod hash {
    pub use crate::sha3_impl as sha3;
}
pub mod pqc {
    pub use crate::reference_impl as ml_dsa;
    pub mod fast {
        pub use crate::keccak_impl as keccak;
    }
}

// verify.sh concatenates the entire unchanged production file and exports.rs.
// The append gives the wrappers access to private functions in that same module.
#[path = "build/ml_dsa.rs"]
pub mod implementation;
