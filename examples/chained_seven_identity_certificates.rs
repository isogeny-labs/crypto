//! Issue/replay the two polynomial certificates used by the degree7 report.
//! Fixed development artifacts; no benchmark or solver is run.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict,
};
use crypto_lib::hash::sha256::sha256;
use std::path::Path;

fn power(e: Expr, n: usize) -> Expr {
    Expr::product(vec![e; n])
}
fn sub(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}

fn statements() -> [(IdentityStatement, &'static str, &'static str); 2] {
    let v = Expr::var;
    let m = Expr::times;
    let scaling = IdentityStatement {
        name: "Elliptic coordinate scaling, denominator-free ring identity".into(),
        source: "EllipticSeven.isomorphism_scaling; known Weierstrass coordinate change".into(),
        variables: ["x", "y", "a", "b", "u"].map(str::to_owned).to_vec(),
        degree_bound: 9,
        lhs: sub(
            power(m(power(v("u"), 3), v("y")), 2),
            Expr::sum(vec![
                power(m(power(v("u"), 2), v("x")), 3),
                m(m(power(v("u"), 4), v("a")), m(power(v("u"), 2), v("x"))),
                m(power(v("u"), 6), v("b")),
            ]),
        ),
        rhs: m(
            power(v("u"), 6),
            sub(
                power(v("y"), 2),
                Expr::sum(vec![power(v("x"), 3), m(v("a"), v("x")), v("b")]),
            ),
        ),
    };
    let support = IdentityStatement {
        name: "Matched quadratic pullback support, denominator clearing".into(),
        source: "SevenJacobian.support_cleared_identity; UV=b and VW=1 imply U=bW".into(),
        variables: ["u", "v", "b", "w"].map(str::to_owned).to_vec(),
        degree_bound: 3,
        lhs: m(sub(v("u"), m(v("b"), v("w"))), v("v")),
        rhs: Expr::sum(vec![
            m(v("u"), v("v")),
            Expr::minus(v("b")),
            m(v("b"), sub(Expr::Const(1), m(v("v"), v("w")))),
        ]),
    };
    [
        (
            scaling,
            "research/chained_jacobian_transfers_20261110/lean/EllipticSeven.lean",
            "seven-scaling-identity-01.json",
        ),
        (
            support,
            "research/chained_jacobian_transfers_20261110/lean/SevenJacobian.lean",
            "seven-support-identity-01.json",
        ),
    ]
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (statement, proof_path, filename) in statements() {
        let proof_bytes = std::fs::read(root.join(proof_path)).unwrap();
        let digest: String = sha256(&proof_bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let cert = issue(
            statement,
            0x2026_1111_0728_0032,
            32,
            "Producer issued/replayed with the unchanged native checker and16 extra checker-seed points;Lean gate executed separately;no signed independence claimed",
            Some(ProofRef {
                kind: "lean4".into(),
                path: proof_path.into(),
                sha256: digest,
            }),
        )
        .unwrap();
        assert!(matches!(
            check_with_extra(&cert, Some((0x2026_1111_0728_0016, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
        if let Ok(out) = std::env::var("CHAINED_IDENTITY_OUT_DIR") {
            let out = Path::new(&out);
            std::fs::create_dir_all(out).unwrap();
            let bytes = format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()).into_bytes();
            let path = out.join(filename);
            if path.exists() {
                assert_eq!(
                    std::fs::read(path).unwrap(),
                    bytes,
                    "immutable certificate differs"
                );
            } else {
                std::fs::write(path, bytes).unwrap();
            }
        }
        println!("PASS {} native identity.certificate/v1,32 issue+16 extra points,F_(2^61-1);Lean proof pointer {proof_path};formal gate is separate,no finite-field/Jacobian geometry inferred", cert.id);
    }
}

#[test]
fn issue_and_replay_fixed_seven_identity_certificates() {
    main();
}
