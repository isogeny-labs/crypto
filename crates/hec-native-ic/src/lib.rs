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

pub mod fp;
pub mod ic;
pub mod instance;
pub mod jac;
pub mod oracle;
pub mod poly;
pub mod rng;
pub mod solve;
