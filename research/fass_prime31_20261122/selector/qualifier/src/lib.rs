//! Source-bound development qualifier. This is not the whole crypto library.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod cryptanalysis {
    pub use crate::identity_certificate;
    pub use crate::pkm_smooth_tower;
    pub use crate::semaev_prime_poly;
    pub use crate::semaev_projection;
    pub use crate::semaev_selector;
}

pub mod hash {
    pub use crate::source_sha256 as sha256;
}

pub const SOURCE_BUILD_MANIFEST: &str =
    include_str!(concat!(env!("OUT_DIR"), "/source-build.json"));
