//! Bind the actual instance and useful-rank implementation.

pub mod stages {
    use crate::cryptanalysis::ic_boundary::CountedGroup;

    include!(concat!(env!("OUT_DIR"), "/instance_ctx.rs"));
}

#[path = "../../../../src/cryptanalysis/ic_framework/useful_rank.rs"]
pub mod useful_rank;
