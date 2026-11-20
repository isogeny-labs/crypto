//! Native index calculus on Jacobians of imaginary hyperelliptic curves
//! `y² = f(x)`, `deg f = 2g + 1`, of genus 2, 3 and 4 over `F_p`, `p < 2^62`.
//!
//! Track T0 of `research/hec_ic_speedup_20261113/PLAN.md`.  The algorithm is
//! the one in `src/cryptanalysis/hyperelliptic_index_calculus.rs` (full
//! degree-1 factor base, factor-base walk, discriminant-parity filter then
//! `gcd(u, x^p − x)` oracle, sparse elimination mod `ℓ`), rebuilt on `u64`
//! residues and fixed-size polynomials instead of `BigUint`.  It changes wall
//! clock, not the counted unit.
//!
//! Every discrete logarithm this crate returns has been checked (`k·D₁ = D₂`).

#[path="../../../counted/native/src/fp.rs"]
pub mod fp;
#[path="../../../counted/native/src/hybrid.rs"]
pub mod hybrid;
#[path="../../../counted/native/src/ic.rs"]
pub mod ic;
#[path="../../../counted/native/src/instance.rs"]
pub mod instance;
#[path="../../../counted/native/src/jac.rs"]
pub mod jac;
#[path="../../../counted/native/src/large_prime.rs"]
pub mod large_prime;
#[path="../../../counted/native/src/oracle.rs"]
pub mod oracle;
#[path="../../../counted/native/src/poly.rs"]
pub mod poly;
#[path="../../../counted/native/src/rng.rs"]
pub mod rng;
#[path="../../../counted/native/src/solve.rs"]
pub mod solve;

pub mod counted;
