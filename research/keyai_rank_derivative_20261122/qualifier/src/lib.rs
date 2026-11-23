//! Exact production-source closure for the KeyAI regression controls.
//!
//! Whole touched modules are loaded from their actual repository paths.
//! Broad dependencies supply pinned, verbatim source ranges selected by
//! `build.rs`. This library is built without `cfg(test)`: the example and
//! separate adversarial integration tests are the tested entry points.

pub mod cryptanalysis;

pub mod prime_hyperelliptic {
    pub mod ops {
        include!(concat!(env!("OUT_DIR"), "/hyperelliptic_ops.rs"));
    }
}

/// Build provenance only; test outcomes need a separate execution receipt.
pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
