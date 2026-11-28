// Isolated dependency adapter. The certificate module is unchanged upstream source.
#[path = "upstream_sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
#[path = "upstream_identity_certificate.rs"]
pub mod identity_certificate;
pub mod cryptanalysis {
    pub use crate::identity_certificate;
}
