//! Scoped integration of the unchanged shared-S3 example with production code.
//!
//! Whole modules are loaded at their actual repository paths. Broad modules
//! contribute byte-for-byte source ranges selected and hashed by `build.rs`.
//! The integration target tests the example; this library is built without
//! `cfg(test)`, and does not include the legacy modules' unit-test suites.

pub mod cryptanalysis;

pub mod prime_hyperelliptic {
    pub mod ops {
        include!(concat!(env!("OUT_DIR"), "/hyperelliptic_ops.rs"));
    }
}

/// Build provenance, not a test receipt or complete hermetic build manifest.
pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
