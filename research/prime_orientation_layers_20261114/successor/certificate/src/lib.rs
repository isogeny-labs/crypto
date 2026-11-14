//! Fixed finite-product difference identity only; actual family uses Lean.
use orientation_anchor_certificate::identity::{Expr as E, IdentityStatement};

pub fn statement() -> IdentityStatement {
    let a = E::var("a");
    let b = E::var("b");
    let c = E::var("c");
    let d = E::var("d");
    IdentityStatement {
        name: "Finite-product difference telescope ring substep".into(),
        source: "OrientationRank.prod_difference ring substep only. Does not certify determinants, valuations, rank bounds or cost.".into(),
        variables: ["a", "b", "c", "d"].map(str::to_owned).to_vec(),
        degree_bound: 2,
        lhs: E::plus(E::product(vec![a.clone(), b.clone()]), E::minus(E::product(vec![c.clone(), d.clone()]))),
        rhs: E::plus(E::product(vec![E::plus(a, E::minus(c.clone())), b.clone()]), E::product(vec![c, E::plus(b, E::minus(d))])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orientation_anchor_certificate::identity::{self, Verdict};
    #[test]
    fn fixed_telescope_and_extra_points_accept() {
        let cert = identity::issue(statement(), 20261114, 32, "unsigned literal", None).unwrap();
        assert!(matches!(
            identity::check_with_extra(&cert, Some((0x14112026, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
    }
    #[test]
    fn false_telescope_is_rejected() {
        let mut s = statement();
        s.rhs = E::plus(s.rhs, E::Const(1));
        assert!(identity::issue(s, 20261114, 32, "negative control", None).is_err());
    }
    #[test]
    fn understated_degree_is_rejected() {
        let mut s = statement();
        s.degree_bound = 1;
        assert!(identity::validate_statement(&s).is_err());
    }
}
