// Administrative dependency adapter; canonical source paths are not modified.
#[path = "../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
#[path = "../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
pub mod cryptanalysis { pub use crate::identity_certificate; }
