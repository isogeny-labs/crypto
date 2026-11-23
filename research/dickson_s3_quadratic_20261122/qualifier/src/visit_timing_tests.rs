//! Exact new production timing tests, against the actual compiled F4 library.
//! build.rs selects the complete module byte-for-byte; no test is rewritten.
use crypto_lib::cryptanalysis::f4_fp::*;
use std::time::Duration;

include!(concat!(env!("OUT_DIR"), "/visit_timing_tests.rs"));
