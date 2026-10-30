//! The static Diffie-Hellman oracle: the auxiliary input of Cheon's
//! discrete logarithm problem with auxiliary inputs.
//!
//! A server holds the secret `α` of a public target `Q = [α]G` and answers
//! `P ↦ [α]P` for any point `P` it is handed (Brown and Gallant, 2004).  An
//! attacker who iterates it from the target gets `[α^i]G` after `i − 1`
//! queries, and Cheon's algorithms (`cheon_counted.rs`) turn `[α^d]G` (for
//! `d | n − 1`) or the powers up to `[α^{2d}]G` (for `d | n + 1`) into `α` in
//! about `√(n/d) + √d` (resp. `√(n/d) + d`) group operations.
//!
//! The oracle is a separate resource of the problem, not part of the
//! attacker's group-operation count and not free:
//!
//! * a method sees only [`StaticDhOracle::query`]; the planted scalar stays
//!   inside [`PlantedStaticDh`] and is never an argument of a solve;
//! * every query is counted ([`StaticDhOracle::queries`]);
//! * the server's own work per query (one scalar multiplication by `α`) is
//!   tallied in its own ledger ([`PlantedStaticDh::server_ops`]), so a
//!   comparison can price a query at the server's real cost as well as at
//!   a flat weight (AGENTS.md section 5a).
//!
//! The oracle exists only for a planted target: a public target has no
//! scalar to serve.

use std::cell::{Cell, RefCell};

use crate::cryptanalysis::cheon_counted::scalar_mul_plain;
use crate::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve, PrimePoint};

/// What a method may do with the oracle.
pub trait StaticDhOracle {
    /// `[α]P`.
    fn query(&self, p: PrimePoint) -> PrimePoint;
    /// Queries answered so far.
    fn queries(&self) -> u64;
    /// The group operations the server spent answering them.
    fn server_ops(&self) -> GroupOps;
}

/// The oracle of a planted workload: serves `[α]P` for the planted `α`.
pub struct PlantedStaticDh<'a> {
    curve: &'a PrimeCurve,
    alpha: u64,
    count: Cell<u64>,
    ops: RefCell<GroupOps>,
}

impl<'a> PlantedStaticDh<'a> {
    pub fn new(curve: &'a PrimeCurve, alpha: u64) -> Self {
        Self {
            curve,
            alpha,
            count: Cell::new(0),
            ops: RefCell::new(GroupOps::default()),
        }
    }
}

impl StaticDhOracle for PlantedStaticDh<'_> {
    fn query(&self, p: PrimePoint) -> PrimePoint {
        self.count.set(self.count.get() + 1);
        scalar_mul_plain(self.curve, &mut self.ops.borrow_mut(), self.alpha, p)
    }
    fn queries(&self) -> u64 {
        self.count.get()
    }
    fn server_ops(&self) -> GroupOps {
        *self.ops.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_oracle_answers_alpha_p_and_counts_each_query() {
        let curve = PrimeCurve {
            p: 10007,
            a: 3,
            b: 7,
        };
        let g = (1..curve.p)
            .find_map(|x| curve.sqrt(curve.rhs(x)).map(|y| PrimePoint::affine(x, y)))
            .unwrap();
        let o = PlantedStaticDh::new(&curve, 1234);
        let mut ops = GroupOps::default();
        let want = scalar_mul_plain(&curve, &mut ops, 1234, g);
        assert_eq!(o.query(g), want);
        let again = o.query(want);
        let mut ops2 = GroupOps::default();
        assert_eq!(again, scalar_mul_plain(&curve, &mut ops2, 1234 * 1234, g));
        assert_eq!(o.queries(), 2);
        assert!(o.server_ops().adds + o.server_ops().doubles > 0);
    }
}
