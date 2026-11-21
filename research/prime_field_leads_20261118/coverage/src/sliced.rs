//! Compile the pinned helper bodies for this correctness experiment.
#![allow(dead_code)]

mod jv_bitops {
    // Tally hooks do not change the arithmetic and are outside this experiment.
    pub fn hook_pow(_modulus: u64, _exponent: u64) {}
    pub fn hook_inv(_modulus: u64) {}
}

mod residual {
    include!(concat!(env!("OUT_DIR"), "/residual.rs"));
}

pub mod fast {
    include!(concat!(env!("OUT_DIR"), "/fast.rs"));
}

pub mod aps {
    use super::fast::{FastCurve, Pt};
    use super::residual::{inv_mod, is_prime_u64};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use serde::Serialize;
    use std::collections::{BTreeMap, BTreeSet};

    type Poly = Vec<(Vec<u32>, u64)>;
    include!(concat!(env!("OUT_DIR"), "/aps.rs"));

    /// Build the two S3 equations of Split4 with the production expression
    /// constructors. The caller enforces the four coordinate domains.
    pub fn split_equations(curve: &FastCurve) -> [Poly; 2] {
        let p = curve.f.p;
        let a = curve.f.from_mont(curve.a);
        let b = curve.f.from_mont(curve.b);
        let [x, y, z, w, t] = std::array::from_fn(|i| Expr::var(i, 5, p));
        [s3(&x, &y, &t, a, b).terms, s3(&z, &w, &t, a, b).terms]
    }
}
