//! Isolated exact-source namespace for cryptanalysis's maintained prime batch
//! oracle. Boundary code converts public study inputs; production algorithms
//! remain byte selections from the pinned snapshots.

pub mod cryptanalysis {
    pub mod ic_boundary {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};
        use serde::Serialize;
        use std::collections::HashMap;
        use std::hash::{BuildHasherDefault, Hasher};
        use std::time::Instant;

        include!(concat!(env!("OUT_DIR"), "/ic_boundary.rs"));

        pub mod prime_batch {
            include!(concat!(env!("OUT_DIR"), "/prime_batch.rs"));
        }
    }

    pub mod ic_framework {
        pub mod stages {
            use crate::cryptanalysis::ic_boundary::{
                CountedGroup, FactorBase, GroupOps, OracleCounters,
            };
            use serde::Serialize;
            use std::collections::BTreeMap;
            include!(concat!(env!("OUT_DIR"), "/stages.rs"));
        }

        pub mod prime_batch_mitm {
            include!(concat!(env!("OUT_DIR"), "/prime_batch_mitm.rs"));

            /// This child can read the native cached table for the explicitly
            /// labelled scalar fallback, without changing its source fields.
            pub mod study_boundary {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/boundary.rs"));
            }
        }
    }
}

pub use cryptanalysis::ic_framework::prime_batch_mitm::study_boundary::{
    NativeQuery, Prepared, SignedPoint, SpecializedConfig,
};

pub const SOURCE_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/source-manifest.json"));
