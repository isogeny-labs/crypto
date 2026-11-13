//! # Chained ECDLP transfers over prime fields: a verified, measured framework.
//!
//! This module builds explicit, efficiently evaluable, DLP-preserving
//! homomorphisms between elliptic curves over `F_p` and genus-2 Jacobians,
//! composes them into chains, verifies each hop preserves the order-`r`
//! discrete logarithm, and measures the *complete* cost of solving the
//! transferred instance against a matched Pollard-rho reference.
//!
//! Layers:
//! - [`field`]: `F_p` and `F_{p²}` arithmetic.
//! - [`ec`]: elliptic curves and Vélu isogenies.
//! - [`bielliptic`]: bielliptic genus-2 curves `y² = F(x²)`, their two
//!   elliptic quotients, and the induced Jacobian maps `φ^*` and `φ_*` on
//!   **arbitrary** (including non-split) reduced divisors.
//! - [`chain`]: a verified transfer hop and the composition of hops.
//! - [`measure`]: counted Pollard rho on both group types and a self-contained
//!   genus-2 index calculus, for the end-to-end comparison.
//!
//! ## What is established and what is not
//!
//! Every map here is same-characteristic. The framework **constructs and
//! verifies** a three-stage transport `E → E' → J(C)` (isogeny, then the
//! bielliptic pull-back) that carries any prime-order DLP without loss of
//! order, and it **measures** that the discrete log is not cheaper in the
//! destination: rho costs the same number of group operations on an isogenous
//! curve and on the order-`r` subgroup of `J(C)`, while genus-2 index calculus
//! on `J(C)` over `F_p` with a full degree-1 factor base costs about `p`
//! group operations (the usual `Õ(p^{2-2/g})` estimate; a limit of this
//! approach, not a lower bound), above rho's `√p`.
//! This is a boundary result, consistent with the dimension/relation count of
//! genus-2 index calculus; it is **not** a cryptanalytic speedup and makes no
//! cross-characteristic claim.

pub mod amortized;
pub mod atlas;
pub mod bielliptic;
pub mod certify;
pub mod chain;
pub mod ec;
pub mod endo_census;
pub mod field;
pub mod hyper64;
pub mod ic_control;
pub mod instrument;
pub mod measure;
pub mod pushforward;
pub mod pushforward_deg3;
pub mod small_de;
pub mod tower;
