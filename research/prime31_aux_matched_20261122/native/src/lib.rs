//! Controlled Prime31 auxiliary-base study. Production algorithms are reused
//! verbatim through the explicitly inventoried source closure in build.rs.
pub mod audit;
pub mod bases;
pub mod collector;
pub mod cryptanalysis;
pub mod kernel_fibers;
pub mod oracle;
pub mod pair_control;
pub mod plain_reference;
pub mod provenance;
pub mod replay;
pub mod sparse_identity;
pub mod target_transport;

#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256_source;
pub mod hash {
    pub use crate::sha256_source as sha256;
}

pub use cryptanalysis::ic_boundary::{
    pivot_inversion_steps, rho_reference, CountedGroup, FactorBase, GroupOps, IncrementalGauss,
    PrimeCurve, PrimePoint, RhoResult, RowStatus,
};

pub mod prime_hyperelliptic {
    pub mod ops {
        include!(concat!(env!("OUT_DIR"), "/hyperelliptic_ops.rs"));
    }
}

pub mod prime_tower {
    use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
    use crate::cryptanalysis::residual_walk::pow_mod;
    include!(concat!(env!("OUT_DIR"), "/prime_tower_helpers.rs"));
}

mod dickson {
    use crate::cryptanalysis::ic_boundary::PrimeCurve;
    use std::collections::HashSet;
    include!(concat!(env!("OUT_DIR"), "/dickson_helpers.rs"));
}
pub use dickson::dickson_abscissae;

/// Build provenance only: execution and independent validation are separate.
pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
