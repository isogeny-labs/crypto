//! Namespace for the exact production dependency closure used by the example.

#[path = "../../../../src/cryptanalysis/bitops.rs"]
pub mod bitops;
// A mod.rs copy preserves F4's implicit child-module resolution. Every file is
// copied byte-for-byte and inventoried by build.rs; no declaration is rewritten.
include!(concat!(env!("OUT_DIR"), "/f4_binding.rs"));
#[path = "../../../../src/cryptanalysis/fp_backend.rs"]
pub mod fp_backend;
#[path = "../../../../src/cryptanalysis/fx_hash.rs"]
pub(crate) mod fx_hash;
#[path = "../../../../src/cryptanalysis/ic_measurement.rs"]
pub mod ic_measurement;
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
#[path = "../../../../src/cryptanalysis/jv_bitops.rs"]
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

pub mod ic_boundary {
    use std::collections::HashMap;
    use std::hash::{BuildHasherDefault, Hasher};

    use num_bigint::BigUint;
    use num_traits::ToPrimitive;
    use serde::{Deserialize, Serialize};

    use crate::cryptanalysis::prime_fast::{
        FastCurve as FastPrimeCurve, Fp64, Pt as FastPrimePoint,
    };
    use crate::cryptanalysis::residual_walk::is_prime_u64;

    include!(concat!(env!("OUT_DIR"), "/ic_boundary.rs"));
}

pub mod pq_groebner_f2 {
    include!(concat!(env!("OUT_DIR"), "/mask_type.rs"));
}

pub mod prime_fast {
    include!(concat!(env!("OUT_DIR"), "/prime_fast.rs"));
}

pub mod residual_walk {
    use std::hint::select_unpredictable;

    include!(concat!(env!("OUT_DIR"), "/residual_arithmetic.rs"));
}

pub mod symmetrized_semaev {
    include!(concat!(env!("OUT_DIR"), "/numeric_s3.rs"));
}

pub mod ic_framework {
    pub mod pkm_semaev {
        use crate::cryptanalysis::ic_boundary::PrimeCurve;
        use std::collections::HashSet;
        include!(concat!(env!("OUT_DIR"), "/dickson_selector.rs"));
    }
}
