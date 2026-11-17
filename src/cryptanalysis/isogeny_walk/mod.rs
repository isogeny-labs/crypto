//! **Isogeny walks** over the `F_p`-isogeny class of a prime-field curve.
//!
//! Starting from a registered curve (P-256, P-224, or any short
//! Weierstrass curve with a known order and generator), the walk visits
//! the `ℓ`-isogeny graphs for a set of small odd primes `ℓ` breadth first.
//! Neighbours come from the roots of the modular polynomial `Φ_ℓ(j, Y)`
//! ([`modpoly`], computed natively mod `p`); each edge is turned into an
//! explicit kernel polynomial by Elkies' construction ([`kernel`]) and
//! accepted only after [`kernel::verify_kernel`] certifies it without
//! reference to `Φ_ℓ`.  Every curve found is recorded in the repository's
//! formats: its ICV1 slug (`docs/curves/ICV1.md`), its EC1 representation
//! identity (`docs/curve-identities.md`), a `docs/curves/ic/curves.yaml`
//! record with its traits, and `IW1` routes (`VOLCANO_NAMING.md` in the
//! cryptanalysis catalog) for every verified edge and every root path.
//!
//! The class is a single isogeny class over `F_p`: all curves share the
//! order, trace and Frobenius discriminant.  When the order is prime the
//! order audit proves each recorded model has it ([`curve::audit_order`]).
//! The walk record does not itself map the recorded generators or establish
//! any change in DLP cost. `icx solve-prime --route-file ... --route-id ...`
//! separately re-verifies a root-origin route, transports caller-supplied
//! generator and target points through its kernel maps, and checks any scalar
//! answer back on the caller's original curve.

pub mod checkpoint;
pub mod curve;
pub mod field;
pub mod kernel;
pub mod million;
pub mod million_store;
pub mod modpoly;
pub mod p61_adjacency;
pub mod p61_class;
pub mod poly;
pub mod queue;
pub mod record;
pub mod store;
pub mod traits;
pub mod walk;

#[cfg(test)]
mod tests;
