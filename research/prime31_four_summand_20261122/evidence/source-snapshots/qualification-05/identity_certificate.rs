//! Issue and recheck the RR5 norm identity using the production certificate code.
//! This is evaluated evidence over 2^61-1, not a formal divisor-chart proof.

use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, IssueError, Verdict,
};

fn variable(name: &str) -> Expr {
    Expr::var(name)
}
fn sum(terms: Vec<Expr>) -> Expr {
    Expr::sum(terms)
}
fn product(terms: Vec<Expr>) -> Expr {
    Expr::product(terms)
}
fn negative(value: Expr) -> Expr {
    Expr::minus(value)
}
fn square(value: &Expr) -> Expr {
    product(vec![value.clone(), value.clone()])
}
fn cube(value: &Expr) -> Expr {
    product(vec![value.clone(), value.clone(), value.clone()])
}

fn statement(include_cross_term: bool) -> IdentityStatement {
    let x = variable("X");
    let r = variable("r");
    let s = variable("s");
    let a = variable("A");
    let v = sum(vec![x.clone(), variable("b")]);
    let l = sum(vec![product(vec![variable("c"), x.clone()]), variable("d")]);
    let delta = sum(vec![x.clone(), negative(r.clone())]);
    // Eliminate the curve coefficient B using s^2 = r^3 + A*r + B.
    let curve_b = sum(vec![
        square(&s),
        negative(cube(&r)),
        negative(product(vec![a.clone(), r.clone()])),
    ]);
    let curve_rhs = sum(vec![cube(&x), product(vec![a.clone(), x.clone()]), curve_b]);
    let numerator = sum(vec![
        product(vec![s.clone(), v.clone()]),
        product(vec![delta.clone(), l.clone()]),
    ]);
    let lhs = sum(vec![
        product(vec![curve_rhs, square(&v)]),
        negative(square(&numerator)),
    ]);
    let quadratic = sum(vec![square(&x), product(vec![r.clone(), x]), square(&r), a]);
    let mut quartic_terms = vec![
        product(vec![quadratic, square(&v)]),
        negative(product(vec![delta.clone(), square(&l)])),
    ];
    if include_cross_term {
        quartic_terms.push(negative(product(vec![Expr::Const(2), s, l, v])));
    }
    IdentityStatement {
        name: if include_cross_term {
            "RR5 norm factorization with target on curve"
        } else {
            "RR5 negative control: omitted signed cross term"
        }
        .into(),
        source:
            "research/prime31_four_summand_20261122/MATHEMATICS.md; native/extractor.rs::system"
                .into(),
        variables: ["X", "r", "s", "A", "b", "c", "d"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        degree_bound: 6,
        lhs,
        rhs: product(vec![delta, sum(quartic_terms)]),
    }
}

fn main() {
    let seed = 0x5252_355f_4944_4331;
    let certificate = issue(
        statement(true), seed, 32,
        "Same-team producer execution with an additional deterministic checker seed; unsigned evaluated evidence, no formal proof or independent verifier.",
        None,
    ).expect("the RR5 polynomial identity must issue");
    let verdict = check_with_extra(&certificate, Some((0x5245_504c_4159_5f31, 32)))
        .expect("the generated identity certificate must be well formed");
    assert!(matches!(
        verdict,
        Verdict::Accept {
            points: 32,
            extra_points: 32,
            ..
        }
    ));
    let counterexample = match issue(
        statement(false),
        seed,
        32,
        "deliberately false control",
        None,
    ) {
        Err(IssueError::False(counterexample)) => counterexample,
        other => panic!("the omitted cross term must yield a counterexample, got {other:?}"),
    };
    eprintln!(
        "RR5_IDENTITY_CHECK {}",
        serde_json::json!({
            "certificate_id": certificate.id,
            "status": "evaluated",
            "modulus": certificate.modulus,
            "certificate_points": 32,
            "additional_points": 32,
            "omitted_cross_term_rejected": true,
            "negative_control_counterexample": {
                "index": counterexample.index,
                "assignment": counterexample.assignment,
                "lhs": counterexample.lhs,
                "rhs": counterexample.rhs,
            },
            "formal_proof": false,
            "signed_independent_verification": false,
        })
    );
    println!("{}", serde_json::to_string_pretty(&certificate).unwrap());
}
