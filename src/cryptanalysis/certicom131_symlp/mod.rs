//! Index-calculus building blocks sized for the two non-Koblitz 131-bit
//! Certicom curves, ECC2-131 (`GF(2^131)`, cofactor 2) and ECCp-131
//! (131-bit prime, cofactor 1), and for same-family calibration curves of
//! 61–127 bits measured with the same code.
//!
//! * [`gf2w`] — `GF(2^n)` for `n ≤ 191` in three words (ECC2-131 and the
//!   binary calibration degrees).
//! * [`curves`] — both 131-bit curves behind one trait, with the
//!   negation-shared evaluation of `x(T ± P)`.
//! * [`relgraph`] — the double-large-prime relation graph over `Z/r` for a
//!   130-bit `r`, with independent replay of every emitted relation.
//!
//! Nothing here recovers a published logarithm or claims a cost below
//! Pollard rho; see `docs/ic/CERTICOM131_IC_PROGRAM.md` for the gates.

pub mod curves;
pub mod gf2w;
pub mod planted;
pub mod relgraph;
pub mod sym2;
pub mod sym2p;
