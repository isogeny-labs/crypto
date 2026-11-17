//! Literal residual scaling only. The parameterized kernel is checked in Lean.
use orientation_anchor_certificate::identity::{Expr as E, IdentityStatement};

pub fn statement() -> IdentityStatement {
    let c = E::var("c");
    let n = E::var("N");
    let uv = E::product(vec![E::var("U"), E::var("V")]);
    IdentityStatement {
        name: "Actual first-jet weighted residual ring scaling".into(),
        source: "OrientationLeading.actual_weighted_factorization ring substep only, after deriving crossingFirst=c*normalizedCrossing. Does not certify masks, valuations, sums, rank or cost.".into(),
        variables: ["c", "N", "U", "V"].map(str::to_owned).to_vec(),
        degree_bound: 3,
        lhs: E::product(vec![E::plus(n.clone(), E::minus(uv.clone())), c.clone()]),
        rhs: E::plus(E::product(vec![c.clone(), n]), E::minus(E::product(vec![c, uv]))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orientation_anchor_certificate::identity::{self, Verdict};
    #[test]
    fn fixed_scaling_and_extra_points_accept() {
        let cert = identity::issue(statement(), 20261114, 32, "unsigned literal", None).unwrap();
        assert!(matches!(
            identity::check_with_extra(&cert, Some((0x14112026, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
    }
    #[test]
    fn false_scaling_is_rejected() {
        let mut s = statement();
        s.rhs = E::plus(s.rhs, E::Const(1));
        assert!(identity::issue(s, 20261114, 32, "negative control", None).is_err());
    }
    #[test]
    fn understated_degree_is_rejected() {
        let mut s = statement();
        s.degree_bound = 2;
        assert!(identity::validate_statement(&s).is_err());
    }
}
