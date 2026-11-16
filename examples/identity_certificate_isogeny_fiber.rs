//! Replay the denominator-cleared equation of the existing degree-three map.
//! This certifies a polynomial over the IDC1 evaluation field, not the
//! separate finite-group claim phi = psi_u o [m], nor a performance result.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict,
};

fn power(value: Expr, exponent: usize) -> Expr {
    Expr::product((0..exponent).map(|_| value.clone()).collect())
}

fn statement() -> IdentityStatement {
    let x = Expr::var("x");
    let b = Expr::var("b");
    let x3 = power(x.clone(), 3);
    // Substituting y²=x³+b and clearing x⁶ from
    // X=(x³+4b)/x², Y=y(x³-8b)/x³ gives Y²=X³-27b.
    IdentityStatement {
        name: "degree-three j=0 map: denominator-cleared curve equation".into(),
        source: "examples/isogeny_fiber_pair_20261012.rs::phi; research/isogeny_torsion_fibers_20261012/paired_round/REVIEW-20261019.md".into(),
        variables: vec!["x".into(), "b".into()],
        degree_bound: 9,
        lhs: Expr::times(
            Expr::plus(x3.clone(), b.clone()),
            power(Expr::plus(x3.clone(), Expr::times(Expr::Const(-8), b.clone())), 2),
        ),
        rhs: Expr::plus(
            power(Expr::plus(x3, Expr::times(Expr::Const(4), b.clone())), 3),
            Expr::product(vec![Expr::Const(-27), b, power(x, 6)]),
        ),
    }
}

fn main() {
    let certificate = issue(
        statement(), 0x2026_1019_0000_1749, 32,
        "IDC1 derives evaluation points from statement id and seed; replay includes 32 domain-separated checker points. No finite-group decomposition or historical-run admission is implied.",
        Some(ProofRef {
            kind: "lean4".into(),
            path: "research/isogeny_torsion_fibers_20261012/lean/CubicAutomorphism.lean".into(),
            sha256: crypto_lib::hash::sha256::sha256(include_bytes!(
                "../research/isogeny_torsion_fibers_20261012/lean/CubicAutomorphism.lean"
            )).iter().map(|b| format!("{b:02x}")).collect(),
        }),
    ).expect("denominator-cleared equation holds");
    assert!(matches!(
        check_with_extra(&certificate, Some((0x5041_4952_4544_4d41, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&certificate).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_an_incorrect_constant_term() {
        let mut false_statement = statement();
        false_statement.rhs = Expr::plus(false_statement.rhs, Expr::Const(1));
        assert!(issue(false_statement, 1, 32, "negative control", None).is_err());
    }

    #[test]
    fn replays_with_domain_separated_checker_points() {
        let certificate = issue(statement(), 2, 32, "unit-test replay", None).unwrap();
        assert!(matches!(
            check_with_extra(&certificate, Some((3, 32))).unwrap(),
            Verdict::Accept { .. }
        ));
    }
}
