//! Bind unchanged production identity and hash modules to three small witnesses.
#[path = "../source/sha256.rs"]
pub mod production_sha256;
pub mod hash {
    pub use crate::production_sha256 as sha256;
}
#[path = "../source/identity_certificate.rs"]
pub mod identity_certificate;
pub mod witness;
