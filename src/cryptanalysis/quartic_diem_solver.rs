//! Supplied-input Diem index calculus on a smooth plane quartic over `F_q`.
//!
//! The collector and group law live in the existing bounded quartic study.
//! This module exposes its checked coordinate-input solver without requiring
//! callers to use the study's random planted-instance generator.

use super::diem_quartic as dq;

#[path = "../../research/quartic-diem-sparse_20261115/src/qds_core.rs"]
mod core;

pub use core::{solve_supplied, SuppliedDlp, SuppliedDlpError, SuppliedDlpSolution};
