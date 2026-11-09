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
