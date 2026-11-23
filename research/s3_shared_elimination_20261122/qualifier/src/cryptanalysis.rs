//! Keep the production module namespace without substituting any algorithms.

pub mod challenge_ladder {
    include!(concat!(env!("OUT_DIR"), "/challenge_prime.rs"));
}

pub mod fp_backend {
    include!(concat!(env!("OUT_DIR"), "/fp32_backend.rs"));
}

/// Only the exact production polynomial type alias is present here. There is
/// deliberately no replacement for the existing F4 engine or root finder.
pub mod f4_fp {
    include!(concat!(env!("OUT_DIR"), "/poly_type.rs"));
}

#[path = "../../../../src/cryptanalysis/exact_modular_matrix.rs"]
pub mod exact_modular_matrix;

#[path = "../../../../src/cryptanalysis/shared_macaulay.rs"]
pub mod shared_macaulay;
