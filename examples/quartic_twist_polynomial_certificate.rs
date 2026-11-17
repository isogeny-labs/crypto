//! Native identity certificates for the S3/S4 polynomial peeling formulas.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};

fn difference(a: Expr, b: Expr) -> Expr {
    Expr::plus(a, Expr::minus(b))
}
fn square(x: Expr) -> Expr {
    Expr::times(x.clone(), x)
}
fn det(m: &[Vec<Expr>]) -> Expr {
    if m.len() == 1 {
        return m[0][0].clone();
    }
    Expr::sum(
        (0..m.len())
            .map(|col| {
                let minor: Vec<Vec<Expr>> = m[1..]
                    .iter()
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .filter(|(j, _)| *j != col)
                            .map(|(_, x)| x.clone())
                            .collect()
                    })
                    .collect();
                let term = Expr::times(m[0][col].clone(), det(&minor));
                if col % 2 == 0 {
                    term
                } else {
                    Expr::minus(term)
                }
            })
            .collect(),
    )
}

fn resultant_statement() -> IdentityStatement {
    let (a0, a1, a2) = (Expr::var("a0"), Expr::var("a1"), Expr::var("a2"));
    let (b0, b1, b2) = (Expr::var("b0"), Expr::var("b1"), Expr::var("b2"));
    let zero = Expr::Const(0);
    let lhs = det(&[
        vec![a2.clone(), a1.clone(), a0.clone(), zero.clone()],
        vec![zero.clone(), a2.clone(), a1.clone(), a0.clone()],
        vec![b2.clone(), b1.clone(), b0.clone(), zero.clone()],
        vec![zero, b2.clone(), b1.clone(), b0.clone()],
    ]);
    let t1 = difference(
        Expr::times(a2.clone(), b0.clone()),
        Expr::times(a0.clone(), b2.clone()),
    );
    let t2 = difference(
        Expr::times(a2.clone(), b1.clone()),
        Expr::times(a1.clone(), b2.clone()),
    );
    let t3 = difference(Expr::times(a1, b0), Expr::times(a0, b1));
    let rhs = difference(square(t1), Expr::times(t2, t3));
    IdentityStatement {
        name: "quadratic-quadratic Sylvester resultant for quartic S4".into(),
        source: "research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md; used by examples/quartic_twist_tagged_s5.rs".into(),
        variables: ["a0", "a1", "a2", "b0", "b1", "b2"]
            .iter().map(|s| (*s).into()).collect(),
        degree_bound: 4,
        lhs,
        rhs,
    }
}

fn s3_statement() -> IdentityStatement {
    let (x, y, r, a, b) = (
        Expr::var("x"),
        Expr::var("y"),
        Expr::var("r"),
        Expr::var("a"),
        Expr::var("b"),
    );
    let e1 = Expr::plus(x.clone(), y.clone());
    let e2 = Expr::times(x.clone(), y.clone());
    let r2 = square(r.clone());
    let lhs = Expr::sum(vec![
        Expr::times(square(difference(x, y)), r2.clone()),
        Expr::times(
            Expr::Const(-2),
            Expr::times(
                Expr::plus(
                    Expr::times(e1.clone(), Expr::plus(e2.clone(), a.clone())),
                    Expr::times(Expr::Const(2), b.clone()),
                ),
                r.clone(),
            ),
        ),
        square(difference(e2.clone(), a.clone())),
        Expr::times(Expr::Const(-4), Expr::times(b.clone(), e1.clone())),
    ]);
    let rhs = Expr::sum(vec![
        Expr::times(
            difference(square(e1.clone()), Expr::times(Expr::Const(4), e2.clone())),
            r2,
        ),
        Expr::times(
            Expr::Const(-2),
            Expr::times(
                Expr::plus(
                    Expr::times(e1.clone(), Expr::plus(e2.clone(), a.clone())),
                    Expr::times(Expr::Const(2), b.clone()),
                ),
                r,
            ),
        ),
        square(difference(e2, a)),
        Expr::times(Expr::Const(-4), Expr::times(b, e1)),
    ]);
    IdentityStatement {
        name: "Semaev S3 in two elementary symmetric abscissa variables".into(),
        source: "research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md; a=0 specialization used by examples/quartic_twist_tagged_s5.rs".into(),
        variables: ["x", "y", "r", "a", "b"].iter().map(|s| (*s).into()).collect(),
        degree_bound: 6,
        lhs,
        rhs,
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--check" {
        let cert: IdentityCertificate =
            serde_json::from_str(&std::fs::read_to_string(&args[2]).unwrap()).unwrap();
        assert!(matches!(
            check_with_extra(&cert, Some((0x5155_4152_5449_4353, 32))).unwrap(),
            Verdict::Accept { .. }
        ));
        println!("{} pass", cert.id);
        return;
    }
    assert_eq!(
        args.len(),
        2,
        "usage: quartic_twist_polynomial_certificate resultant|s3 [or --check CERT]"
    );
    let statement = match args[1].as_str() {
        "resultant" => resultant_statement(),
        "s3" => s3_statement(),
        _ => panic!("unknown identity"),
    };
    let cert = issue(
        statement,
        0x5345_4D41_4556_5335,
        32,
        "native issuer; 32 sealed and 32 independent domain-separated checker points",
        None,
    )
    .expect("polynomial identity holds");
    assert!(matches!(
        check_with_extra(&cert, Some((0x5155_4152_5449_4353, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    println!("{}", serde_json::to_string_pretty(&cert).unwrap());
}
