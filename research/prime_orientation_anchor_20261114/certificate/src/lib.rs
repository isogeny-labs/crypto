//! Literal ring substep certificate. Quantified construction is checked in Lean.
#[allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod hash_sha256;
pub mod hash {
    pub use crate::hash_sha256 as sha256;
}
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity;
use identity::{Expr as E, IdentityStatement};

pub fn statement() -> IdentityStatement {
    let c = E::var("c");
    let p = E::var("p");
    let z = E::var("z");
    let q = E::var("q");
    let lhs = E::plus(
        E::product(vec![
            c.clone(),
            p.clone(),
            E::plus(E::Const(1), E::product(vec![z.clone(), q.clone()])),
        ]),
        E::minus(E::product(vec![c.clone(), p.clone()])),
    );
    let rhs = E::product(vec![p, z, c, q]);
    IdentityStatement {
        name: "Computed anchor completion ring rearrangement".into(),
        source: "OrientationAnchor.completed_identity ring substep only; substitute F=1+z*q and C=p*z. Does not encode mask, monic division or 2-adic bounds.".into(),
        variables: ["c", "p", "z", "q"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs,
        rhs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rearrangement_accepts_and_extra_points_accept() {
        let cert = identity::issue(statement(), 20261114, 32, "unsigned literal", None).unwrap();
        assert!(matches!(
            identity::check_with_extra(&cert, Some((0x14112026, 16))).unwrap(),
            identity::Verdict::Accept { .. }
        ));
    }
    #[test]
    fn dropped_subtraction_is_rejected() {
        let mut s = statement();
        s.lhs = E::plus(s.lhs, E::product(vec![E::var("c"), E::var("p")]));
        assert!(identity::issue(s, 20261114, 32, "negative control", None).is_err());
    }
    #[test]
    fn understated_degree_is_rejected() {
        let mut s = statement();
        s.degree_bound = 3;
        assert!(identity::validate_statement(&s).is_err());
    }
}
