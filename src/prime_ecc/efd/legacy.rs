//! Legacy EFD formulas that the current sections do not carry.
//!
//! The old Explicit-Formulas Database page for extended Jacobi-quartic
//! coordinates ships full Magma straight-line programs but no `.op3` files.
//! `legacy_catalogue.json` converts those programs faithfully (statement order
//! and expressions preserved) and pins the derived text and its SHA-256.
//!
//! These formulas are a supplementary section: they are not counted in the
//! current-EFD totals (606) and they use the `jquartic/extended` chart, which
//! is implemented in `formats.rs` but has no current-EFD representation.
use super::*;
use std::sync::OnceLock;

/// Pinned legacy catalogue (extended Jacobi-quartic coordinates).
pub fn catalogue() -> &'static Catalogue {
    static C: OnceLock<Catalogue> = OnceLock::new();
    C.get_or_init(|| {
        serde_json::from_str(include_str!("legacy_catalogue.json"))
            .expect("checked legacy EFD catalogue")
    })
}

/// SHA-256 of the upstream `extendedquartic.html` page these programs were
/// converted from. Pinned so a re-conversion can be checked against the same
/// source bytes.
pub const SOURCE_PAGE_SHA256: &str =
    "c0d4a89fcf346bd03c875adfda2f6a820fbd3435d417128d7d8f89135ec9a28b";
