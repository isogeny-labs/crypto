#[path = "../../identity_record_h/inputs/canonical_sha256.rs"]
pub mod canonical_sha256;
pub mod hash {
    pub use crate::canonical_sha256 as sha256;
}
#[path = "../../identity_record_h/inputs/canonical_identity_certificate.rs"]
pub mod identity_certificate;

use identity_certificate::{
    check, check_with_extra, issue, Expr, IdentityStatement, IssueError, Verdict,
};
use std::path::Path;

type Poly = Vec<i64>;
const SEED: u64 = 0x434d_3139_4944_4331;
const EXTRA_SEED: u64 = 0x434d_3139_4944_4332;

fn add(a: &[i64], b: &[i64]) -> Poly {
    let mut c = vec![0; a.len().max(b.len())];
    for (i, x) in a.iter().enumerate() {
        c[i] += x;
    }
    for (i, x) in b.iter().enumerate() {
        c[i] += x;
    }
    c
}

fn scale(a: &[i64], k: i64) -> Poly {
    a.iter().map(|x| x * k).collect()
}

fn mul(a: &[i64], b: &[i64]) -> Poly {
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    c
}

fn difference(a: &[i64], b: &[i64]) -> Poly {
    add(a, &scale(b, -1))
}

fn quotient_by_seven(a: &[i64], b: &[i64]) -> Poly {
    let residual = difference(a, b);
    assert!(residual.iter().all(|x| x % 7 == 0));
    let q: Poly = residual.iter().map(|x| x / 7).collect();
    assert!(difference(a, &add(b, &scale(&q, 7)))
        .iter()
        .all(|x| *x == 0));
    q
}

fn expr(a: &[i64]) -> Expr {
    let (leading, rest) = a.split_last().expect("nonempty coefficient list");
    rest.iter()
        .rev()
        .fold(Expr::Const(*leading), |acc, coefficient| {
            Expr::plus(Expr::times(acc, Expr::var("W")), Expr::Const(*coefficient))
        })
}

fn cube(a: Expr) -> Expr {
    Expr::product(vec![a.clone(), a.clone(), a])
}

fn target_polynomial(n: &[i64], g: &[i64]) -> Poly {
    add(
        &add(&mul(&mul(n, n), n), &scale(&mul(n, &mul(g, g)), 2)),
        &scale(&mul(&mul(g, g), g), 6),
    )
}

fn target_expression(n: &[i64], g: &[i64]) -> Expr {
    let n = expr(n);
    let g = expr(g);
    Expr::sum(vec![
        cube(n.clone()),
        Expr::product(vec![Expr::Const(2), n, g.clone(), g.clone()]),
        Expr::times(Expr::Const(6), cube(g)),
    ])
}

fn left(h: &[i64], n: &[i64], g: &[i64]) -> Expr {
    Expr::plus(
        expr(h),
        Expr::times(Expr::var("T"), target_expression(n, g)),
    )
}

fn expect_accept(verdict: Verdict) {
    assert!(matches!(verdict, Verdict::Accept { .. }), "{verdict:?}");
}

fn main() {
    let out_arg = std::env::args().nth(1).expect("fresh output directory");
    let out = Path::new(&out_arg);
    std::fs::create_dir(out).expect("fresh output directory");
    assert_eq!(
        hex::encode(hash::sha256::sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let h = [1, 0, 0, 0, 2, 0, 6];
    let f = [1, 3, 1, 1];
    let g = [1, 4, 1, 6];
    let n = [3, 1, 5, 1];
    let r = [2, 1, 0, 2];
    let k1 = quotient_by_seven(&h, &mul(&f, &g));
    let lhs2 = target_polynomial(&n, &g);
    let rhs2 = mul(&f, &mul(&r, &r));
    let k2 = quotient_by_seven(&lhs2, &rhs2);
    let statement = IdentityStatement {
        name: "Exact integer lift of CM19 F7 factorization and cubic-cover identities".to_string(),
        source: "crypto source91db340ea313aaefe12e9c39a458a7b81929eeb7; EC1P3Ccm19hf49aa6b9d975; H=FG+7K1 and N^3+2NG^2+6G^3=FR^2+7K2; independent indeterminate T separates identities".to_string(),
        variables: vec!["W".to_string(), "T".to_string()],
        degree_bound: 10,
        lhs: left(&h, &n, &g),
        rhs: Expr::plus(
            Expr::plus(Expr::times(expr(&f), expr(&g)), Expr::times(Expr::Const(7), expr(&k1))),
            Expr::times(Expr::var("T"), Expr::plus(
                Expr::product(vec![expr(&f), expr(&r), expr(&r)]),
                Expr::times(Expr::Const(7), expr(&k2)))),
        ),
    };
    let cert = issue(statement.clone(), SEED, 32,
        "Issued and replayed by the same unsigned producer with the exact upstream canonical v1 instrument; not an independent sign-off. Exact integer coefficient residuals also checked natively. The v1 evaluation field stays 2^61-1; reduction of the integer-lift equations gives the specified F7 identities.",
        None).expect("true lifted identities");
    expect_accept(check(&cert).expect("well formed"));
    expect_accept(
        check_with_extra(&cert, Some((EXTRA_SEED, 64))).expect("well formed extra check"),
    );

    let mut mutated_n = n;
    mutated_n[0] += 1;
    assert!(difference(&target_polynomial(&mutated_n, &g), &rhs2)
        .iter()
        .any(|x| x % 7 != 0));
    let mut false_n = statement.clone();
    false_n.lhs = left(&h, &mutated_n, &g);
    let error_n = issue(false_n, SEED, 32, "actual N0+1 control", None)
        .expect_err("false N identity refuses");
    assert!(matches!(error_n, IssueError::False(_)));

    let mut mutated_r = r;
    mutated_r[0] += 1;
    assert!(difference(&lhs2, &mul(&f, &mul(&mutated_r, &mutated_r)))
        .iter()
        .any(|x| x % 7 != 0));
    let mut false_r = statement.clone();
    false_r.rhs = Expr::plus(
        Expr::plus(
            Expr::times(expr(&f), expr(&g)),
            Expr::times(Expr::Const(7), expr(&k1)),
        ),
        Expr::times(
            Expr::var("T"),
            Expr::plus(
                Expr::product(vec![expr(&f), expr(&mutated_r), expr(&mutated_r)]),
                Expr::times(Expr::Const(7), expr(&k2)),
            ),
        ),
    );
    let error_r = issue(false_r, SEED, 32, "actual R0+1 control", None)
        .expect_err("false R identity refuses");
    assert!(matches!(error_r, IssueError::False(_)));

    let mut wrong_digest = cert.clone();
    let replacement = if wrong_digest.value_digest.starts_with('0') {
        "1"
    } else {
        "0"
    };
    wrong_digest.value_digest.replace_range(..1, replacement);
    assert!(!matches!(
        check(&wrong_digest).expect("well formed poisoned digest"),
        Verdict::Accept { .. }
    ));

    let max_lhs_coefficient = h.iter().chain(lhs2.iter()).map(|x| x.abs()).max().unwrap();
    let reconstructed_rhs = add(&rhs2, &scale(&k2, 7));
    let max_rhs_coefficient = add(&mul(&f, &g), &scale(&k1, 7))
        .iter()
        .chain(reconstructed_rhs.iter())
        .map(|x| x.abs())
        .max()
        .unwrap();
    assert_eq!(max_lhs_coefficient, max_rhs_coefficient);
    assert!(max_lhs_coefficient < (1_i64 << 60));
    let lifts = serde_json::json!({
        "schema": "chained-cubic-integer-lifts/v1",
        "ascending_integer_coefficients": {"H": h, "F": f, "G": g, "N": n, "R": r, "K1": k1, "K2": k2},
        "exact_coefficient_residuals_all_zero": true,
        "maximum_coefficient_after_reconstruction": max_lhs_coefficient,
        "canonical_evaluation_field": identity_certificate::MODULUS,
        "intended_reduction_field": 7,
        "certificate_id": cert.id,
        "formal_proof": null,
        "scope": "Only the two polynomial identities, not proper geometry, Jacobian identification, timing or a signed PCC verdict."
    });
    std::fs::write(
        out.join("identity.certificate.json"),
        serde_json::to_string_pretty(&cert).unwrap() + "\n",
    )
    .unwrap();
    std::fs::write(
        out.join("integer_lifts.json"),
        serde_json::to_string_pretty(&lifts).unwrap() + "\n",
    )
    .unwrap();
    println!("INTEGER_LIFTS\tK1={k1:?}\tK2={k2:?}\texact-Z-residuals=0\tmax-reconstructed-coefficient={max_lhs_coefficient}");
    println!("CANONICAL_IDENTITY\t{}\tschema=identity.certificate/v1\tpoints=32+64\tproof=None\tunsigned", cert.id);
    println!("ACTUAL_N0_PLUS1_REJECT\t{error_n:?}");
    println!("ACTUAL_R0_PLUS1_REJECT\t{error_r:?}");
    println!("ACTUAL_VALUE_DIGEST_MUTATION_REJECT\tpristine-certificate-restored=true");
}
