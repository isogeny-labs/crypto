//! Bind exact production modules to a bounded mathematical-review witness.
//! No production arithmetic or certificate implementation is rewritten here.

#[path = "../source/sha256.rs"]
pub mod production_sha256;

pub mod hash {
    pub use crate::production_sha256 as sha256;
}

#[path = "../source/identity_certificate.rs"]
pub mod identity_certificate;

pub mod witness;
