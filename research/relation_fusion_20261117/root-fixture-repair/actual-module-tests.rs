//! Compile the actual changed source modules and their unchanged original tests.
//! Other public APIs link to frozen source348/default-features-false library352.
//! This qualifies these modules, not the full root-crate libtest target.
pub use crypto_lib::{binary_ecc, hash, cryptanalysis};
pub use crypto_lib::cryptanalysis::semaev_decomp;
#[path = "../../../src/cryptanalysis/binary_radical_root.rs"]
mod binary_radical_root;
#[path = "../../../src/cryptanalysis/curve_id.rs"]
mod curve_id;
