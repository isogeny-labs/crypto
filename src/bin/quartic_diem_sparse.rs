//! Diem index calculus on a generic smooth plane quartic over F_q: factor base of about
//! `c * sqrt(q)` points, double large primes kept as sparse columns (rows of at most four
//! points plus the hyperplane unknown), minimum-weight elimination of the large-prime
//! columns and Wiedemann on the factor-base system, against Pollard rho in the same group.
//!
//! Study: `research/quartic-diem-sparse_20261115/` (PLAN.md frozen before the first run).
//! Local development tool at small parameters; not a lab benchmark.
//!
//! Usage: `quartic_diem_sparse run <out.jsonl> <q> <n_inst> <seed0> <c-list x100>
//! <none|plain|both> <dense_max_b> [rho_runs]` | `selftest [q]`.

#[allow(dead_code, clippy::wrong_self_convention)]
#[path = "../cryptanalysis/diem_quartic.rs"]
mod dq;

#[path = "../../research/quartic-diem-sparse_20261115/src/qds_core.rs"]
mod qds_core;

fn main() {
    qds_core::cli();
}
