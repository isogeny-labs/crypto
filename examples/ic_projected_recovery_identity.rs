//! Certificate for the scalar-recovery residual, not the finite-group bridge.
use crypto_lib::cryptanalysis::identity_certificate::{
    self as ic, Expr, IdentityCertificate, IdentityStatement, ProofRef, Verdict,
};
use sha2::{Digest, Sha256};

fn statement() -> IdentityStatement {
    let (h, h_inv, d, k) = (
        Expr::var("h"),
        Expr::var("hInv"),
        Expr::var("d"),
        Expr::var("k"),
    );
    let shifted = Expr::plus(d.clone(), k.clone());
    IdentityStatement {
        name: "Projected scalar recovery residual over every commutative ring".into(),
        source: "PROJECTED_QUERY_LAW_IMPLEMENTATION_20261026.md; exact theorem ICProjectedRecovery.scalar_recovery_residual in proofs/ProjectedRecoveryIdentity.lean. Vanishing additionally requires hInv*h=1. This is not a curve-order, direct-sum, uniformity, relation-oracle or cost certificate.".into(),
        variables: vec!["h".into(), "hInv".into(), "d".into(), "k".into()],
        degree_bound: 3,
        lhs: Expr::sum(vec![
            Expr::times(h_inv.clone(), Expr::times(h.clone(), shifted.clone())),
            Expr::minus(k),
            Expr::minus(d),
        ]),
        rhs: Expr::times(
            Expr::plus(Expr::times(h_inv, h), Expr::Const(-1)),
            shifted,
        ),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let [mode, path] = args.as_slice() {
        assert_eq!(
            mode, "check",
            "usage: ic_projected_recovery_identity [check PATH]"
        );
        let cert: IdentityCertificate =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            cert.statement,
            statement(),
            "unexpected certificate statement"
        );
        let verdict = ic::check_with_extra(&cert, Some((202610260103, 256))).unwrap();
        assert!(matches!(verdict, Verdict::Accept { .. }));
        println!("{}: {verdict:?}", cert.id);
        return;
    }
    assert!(
        args.is_empty(),
        "usage: ic_projected_recovery_identity [check PATH]"
    );
    let path = "research/ic_algebraic_crossover_20261023/proofs/ProjectedRecoveryIdentity.lean";
    let cert = ic::issue(
        statement(),
        202610260102,
        32,
        "Native issuer and separate checker invocation with256extra domain-separated points; faithful Lean commutative-ring identity built and axiom-audited separately. All polynomial coefficients have magnitude1. Fixed v1 modulus alone is not a general integer proof or finite-group certificate.",
        Some(ProofRef {
            kind: "lean4-v4.34.1".into(),
            path: path.into(),
            sha256: hex::encode(Sha256::digest(std::fs::read(path).unwrap())),
        }),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unconditional_claim_to_recover_without_an_inverse_is_rejected() {
        let mut wrong = statement();
        wrong.rhs = Expr::Const(0);
        assert!(matches!(
            ic::issue(wrong, 202610260102, 32, "negative control", None),
            Err(ic::IssueError::False(_))
        ));
    }
}
