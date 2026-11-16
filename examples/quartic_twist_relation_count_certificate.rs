//! Seal the polynomial part of the signed four-factor count used in the study.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};

fn power(base: &Expr, degree: usize) -> Expr {
    Expr::product((0..degree).map(|_| base.clone()).collect())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--check" {
        let cert: IdentityCertificate =
            serde_json::from_str(&std::fs::read_to_string(&args[2]).unwrap()).unwrap();
        assert!(matches!(
            check_with_extra(&cert, Some((0x4348_4543_4B5F_0037, 32))).unwrap(),
            Verdict::Accept { .. }
        ));
        println!("{} pass", cert.id);
        return;
    }
    assert_eq!(
        args.len(),
        1,
        "usage: quartic_twist_relation_count_certificate [--check CERT]"
    );
    let b = Expr::var("B");
    let falling = Expr::product(
        (0..4)
            .map(|k| Expr::plus(b.clone(), Expr::Const(-k)))
            .collect(),
    );
    let lhs = Expr::times(Expr::Const(16), falling);
    let rhs = Expr::sum(vec![
        Expr::times(Expr::Const(16), power(&b, 4)),
        Expr::times(Expr::Const(-96), power(&b, 3)),
        Expr::times(Expr::Const(176), power(&b, 2)),
        Expr::times(Expr::Const(-96), b),
    ]);
    let statement = IdentityStatement {
        name: "signed four-factor falling-factorial expansion".to_string(),
        source: "research/twist_index_calculus_20261020/QUARTIC_TWIST24_REPORT.md; the expectation proof additionally uses a uniform residual and linearity of expectation".to_string(),
        variables: vec!["B".to_string()],
        degree_bound: 4,
        lhs,
        rhs,
    };
    let cert = issue(
        statement,
        0x5449_435F_5349_474E,
        32,
        "issued by this native example; checked by identity_certificate::check_with_extra with 32 independent domain-separated points",
        None,
    )
    .expect("polynomial identity holds");
    assert!(matches!(
        check_with_extra(&cert, Some((0x4348_4543_4B5F_0037, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
