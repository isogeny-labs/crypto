//! Arithmetic certificate supporting the restricted D=-19, t=-2, h=4 obstruction.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
use serde_json::json;

fn statement() -> IdentityStatement {
    let x = Expr::var("ell");
    let x2 = Expr::times(x.clone(), x);
    let x4 = Expr::times(x2.clone(), x2.clone());
    let term = |coefficient, value| Expr::times(Expr::Const(coefficient), value);
    IdentityStatement {
        name: "D19 trace-minus-two cofactor-four product decomposition".into(),
        source: "research/conductor_gap_size_ladder_20261026/AMENDMENT-v2.md; integer coefficients have magnitude below 370000".into(),
        variables: vec!["ell".into()],
        degree_bound: 4,
        lhs: Expr::times(
            Expr::plus(term(1216, x2.clone()), Expr::Const(1)),
            Expr::plus(term(304, x2.clone()), Expr::Const(1)),
        ),
        rhs: Expr::sum(vec![
            term(5, Expr::plus(term(73933, x4.clone()), term(304, x2))),
            Expr::Const(1),
            Expr::minus(x4),
        ]),
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--residues") {
        let rows: Vec<_> = (1u64..5)
            .map(|x| {
                let p = (1216 * x * x + 1) % 5;
                let r = (304 * x * x + 1) % 5;
                assert!(p == 0 || r == 0);
                json!({"ell_mod5": x, "p_mod5": p, "r_mod5": r})
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json!({
            "schema":"conductor-gap.mod5-cases/v1", "nonzero_residue_cases":rows,
            "scope":"For prime ell>5, one of the two candidates is a proper multiple of 5; no obstruction is claimed for other traces or cofactors."
        })).unwrap());
        return;
    }
    let cert = issue(
        statement(),
        2026102601,
        32,
        "Issued by the native example; replayed with 64 additional domain-separated points. Evaluated over F_(2^61-1); the integer modular proof is the explicit four-case argument, not a formal Lean proof.",
        None,
    ).unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((2026102602, 64))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_nonzero_residue_forces_a_factor_of_five() {
        for x in 1u64..5 {
            assert!((1216 * x * x + 1) % 5 == 0 || (304 * x * x + 1) % 5 == 0);
        }
    }

    #[test]
    fn a_changed_constant_cannot_be_sealed_as_the_same_identity() {
        let mut altered = statement();
        altered.rhs = Expr::plus(altered.rhs, Expr::Const(1));
        assert!(issue(altered, 2026102601, 32, "negative validation fixture", None).is_err());
    }
}
