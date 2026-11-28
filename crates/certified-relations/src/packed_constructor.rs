//! Bounded packed instantiation of the maintained constructor and provenance kernels.
//! Returns unverified complete wire bytes; acceptance always requires separate replay.
#![allow(dead_code, private_interfaces)]
use std::collections::{BTreeMap, BTreeSet};
include!("packed_body.rs");
