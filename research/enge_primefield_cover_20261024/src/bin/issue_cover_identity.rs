//! Issue or replay the polynomial part of the degree-two target norm identity.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};

fn sub(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}
fn square(a: Expr) -> Expr {
    Expr::times(a.clone(), a)
}

fn statement() -> IdentityStatement {
    let x_gap = sub(Expr::var("x"), Expr::var("xq"));
    let a = sub(
        sub(Expr::var("y"), Expr::var("yq")),
        Expr::times(Expr::var("b"), x_gap.clone()),
    );
    let v = Expr::product(vec![Expr::var("c"), Expr::var("t"), x_gap]);
    IdentityStatement {
        name: "degree_two_target_section_norm_product".to_owned(),
        source: "research/enge_primefield_cover_20261024/REPORT.md".to_owned(),
        variables: ["x", "xq", "y", "yq", "b", "c", "t"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        degree_bound: 6,
        lhs: Expr::times(sub(a.clone(), v.clone()), Expr::plus(a.clone(), v.clone())),
        rhs: sub(square(a), square(v)),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--check" {
        let bytes = std::fs::read(&args[2]).expect("read certificate");
        let cert: IdentityCertificate = serde_json::from_slice(&bytes).expect("parse certificate");
        assert_eq!(cert.statement, statement(), "unexpected statement");
        assert!(matches!(
            check_with_extra(&cert, Some((0x6e6765, 32))).unwrap(),
            Verdict::Accept { .. }
        ));
        eprintln!(
            "certificate {} accepted with 32 additional checker points",
            cert.id
        );
        return;
    }
    assert_eq!(args.len(), 1, "usage: issue_cover_identity [--check PATH]");
    let cert = issue(statement(), 0x20261024, 32, "Native issuer; extra independent-seed replay using the repository checker; no independent implementation.", None)
        .expect("issue true identity");
    assert!(matches!(
        check_with_extra(&cert, Some((0x6e6765, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
