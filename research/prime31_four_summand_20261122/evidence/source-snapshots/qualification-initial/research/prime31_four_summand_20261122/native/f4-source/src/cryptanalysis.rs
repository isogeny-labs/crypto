//! Namespace for the exact production dependency closure used by the example.

#[path = "../../../../../src/cryptanalysis/bitops.rs"]
pub mod bitops;
// A mod.rs copy preserves F4's implicit child-module resolution. Every file is
// copied byte-for-byte and inventoried by build.rs; no declaration is rewritten.
include!(concat!(env!("OUT_DIR"), "/f4_binding.rs"));
#[path = "../../../../../src/cryptanalysis/fx_hash.rs"]
pub(crate) mod fx_hash;
#[path = "../../../../../src/cryptanalysis/ic_measurement.rs"]
pub mod ic_measurement;
#[path = "../../../../../src/cryptanalysis/jv_bitops.rs"]
pub mod jv_bitops;

pub mod challenge_ladder {
    include!(concat!(env!("OUT_DIR"), "/challenge_prime.rs"));
}

pub mod glv_invariant_base {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use crate::cryptanalysis::residual_walk::inv_mod;

    include!(concat!(env!("OUT_DIR"), "/glv_roots.rs"));
}

pub mod groebner_f4 {
    include!(concat!(env!("OUT_DIR"), "/monomial_order.rs"));
}

pub mod pq_groebner_f2 {
    include!(concat!(env!("OUT_DIR"), "/mask_type.rs"));
}

pub mod residual_walk {
    use std::hint::select_unpredictable;

    include!(concat!(env!("OUT_DIR"), "/residual_arithmetic.rs"));
}

