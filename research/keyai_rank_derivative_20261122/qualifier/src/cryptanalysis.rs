//! Namespace bindings and imports, with no replacement arithmetic.

#[path = "../../../../src/cryptanalysis/bitops.rs"]
pub mod bitops;
#[path = "ic_framework.rs"]
pub mod ic_framework;
#[path = "../../../../src/cryptanalysis/ic_measurement.rs"]
pub mod ic_measurement;
#[path = "../../../../src/cryptanalysis/jv_bitops.rs"]
pub mod jv_bitops;
#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
pub mod semaev_prime_poly;

pub mod challenge_ladder {
    include!(concat!(env!("OUT_DIR"), "/challenge_prime.rs"));
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

pub mod prime_fast {
    include!(concat!(env!("OUT_DIR"), "/prime_fast.rs"));
}

pub mod residual_walk {
    use std::hint::select_unpredictable;

    include!(concat!(env!("OUT_DIR"), "/residual_arithmetic.rs"));
}
