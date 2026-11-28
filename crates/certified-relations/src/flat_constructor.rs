//! Opt-in packed constructor using private sorted-vector maps.
//! Shares every algorithm and complete certificate obligation with the tree backend.
#![allow(dead_code, private_interfaces)]
use crate::flat_map::FlatMap as BTreeMap;
use std::collections::BTreeSet;
include!("packed_body.rs");
