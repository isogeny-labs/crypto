//! Compile and test the complete production PQC modules and their real dependencies.
//! This checks correctness and build portability; it does not test side channels.
#![allow(dead_code)]

#[path = "../../src/hash"]
pub mod hash {
    pub mod sha3;
}

#[path = "../../src/utils"]
pub mod utils {
    pub mod random;
}

#[path = "../../src/pqc"]
pub mod pqc {
    pub mod ml_dsa;
    pub mod ml_kem;

    pub mod fast {
        pub mod keccak;
        pub mod keccak4;
        pub mod ml_dsa;
        pub mod ml_kem;
    }
}
