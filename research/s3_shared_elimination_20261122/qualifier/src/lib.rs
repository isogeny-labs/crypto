//! Development qualification of the actual shared-elimination source modules.
//!
//! The two new modules are included in full, including their unit tests. Their
//! small dependencies are verbatim, hash-checked ranges from the production
//! source. This crate contains the production `Poly` type alias, not an F4
//! solver. Passing its tests does not qualify the full F4/IC integration.

pub mod cryptanalysis;

/// Source identities and exact byte ranges selected at build time. This is a
/// source-component manifest, not a complete hermetic build or PCC receipt.
pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
