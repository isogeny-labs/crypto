//! Scoped integration of the Dickson-S3 example with actual production code.
//!
//! Whole modules are loaded at their actual repository paths. Broad modules
//! contribute byte-for-byte source ranges selected and hashed by `build.rs`.
//! The integration target tests the example; this library is built without
//! `cfg(test)`, and does not include the legacy modules' unit-test suites.

pub mod cryptanalysis;

#[path = "../../../../src/hash/sha256.rs"]
pub mod production_sha256;
pub mod hash {
    pub use crate::production_sha256 as sha256;
}

pub mod prime_hyperelliptic {
    pub mod ops {
        include!(concat!(env!("OUT_DIR"), "/hyperelliptic_ops.rs"));
    }
}

/// Build provenance, not a test receipt or complete hermetic build manifest.
pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
