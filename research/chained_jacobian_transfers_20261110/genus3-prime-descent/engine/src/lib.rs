//! Normal dependencies on the original field, homogeneous-polynomial and Picard engines.
#[allow(dead_code, unused_imports, clippy::needless_range_loop)]
#[path = "../../../../../src/cryptanalysis/diem_quartic.rs"]
pub mod cover;
#[allow(dead_code, clippy::chunks_exact_to_as_chunks)]
#[path = "../../../../../src/hash/sha256.rs"]
pub mod digest;
#[allow(dead_code, unused_imports, clippy::needless_range_loop)]
#[path = "../../../../../src/cryptanalysis/plane_quartic_pic.rs"]
pub mod pic;
