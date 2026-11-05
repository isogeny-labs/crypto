//! Toy implementation of index calculus for the elliptic-curve discrete
//! logarithm problem in which relations are found by Boolean tensor
//! contraction (Rust port of the Go `ecdltensor` prototype, package `ecdl`).
//! Every component runs at toy sizes (p below 2^24 or so); see the README for
//! what this prototype does and does not demonstrate, and PORTING.md for the
//! Go -> Rust identifier map.

// Var carries interior mutability only in its hint bookkeeping, never in its hash/eq.
#![allow(clippy::mutable_key_type, clippy::type_complexity)]

use std::fmt;

pub mod curve;
pub mod field;
pub mod gadgets;
pub mod linalg;
pub mod opcount;
pub mod pointgadget;
pub mod relation;
pub mod scale;
pub mod schedule;
pub mod solver;
pub mod tensor;

pub use curve::{random_prime_order_curve, Curve, Point, INFINITY};
pub use field::is_prime;
pub use gadgets::{Arith, ArithRef, Builder, Operand, Term, Var};
pub use linalg::{left_kernel_vector, left_kernel_vector_sparse};
pub use pointgadget::{gadget_relation_network, gadget_shapes, FieldPoint};
pub use relation::{mitm_find_relation, relation_network, FactorBases};
pub use schedule::{DFSCounter, EdgeHint, Step};
pub use solver::{Relation, Solver, Stats};
pub use tensor::{
    find_witness, BruteForceCounter, Counter, FuncVertex, Network, NetworkStats, PredVertex,
    SparseContractor, SupportVertex, Vertex,
};

/// Error type for the whole crate (Go: `errors.New` / `fmt.Errorf` values).
/// The message strings are kept identical to the Go ones ("ecdl: ...").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl Error {
    pub fn new(msg: impl Into<String>) -> Error {
        Error(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// Crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;
