//! Universal identities supporting the coordinate and interpolation steps.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
mod source_sha256;
pub mod hash {
    pub mod sha256 {
        pub use crate::source_sha256::sha256;
    }
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod certificate;
use certificate::{Expr as E, IdentityStatement as S};
use serde_json::json;
use std::{fs, path::Path};
fn v(s: &str) -> E {
    E::var(s)
}
fn n(i: i64) -> E {
    E::Const(i)
}
fn a(xs: Vec<E>) -> E {
    E::Add(xs)
}
fn m(xs: Vec<E>) -> E {
    E::Mul(xs)
}
fn power(x: E, k: usize) -> E {
    m(vec![x; k])
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv = std::env::args().collect::<Vec<_>>();
    let out = Path::new(argv.get(1).ok_or("fresh output directory required")?);
    if out.exists() {
        return Err("output exists".into());
    }
    fs::create_dir_all(out)?;
    let mut statements = vec![];
    let z = a(vec![m(vec![n(3), v("X")]), v("a2")]);
    statements.push(S{name:"cleared long-to-short Weierstrass coordinate identity".into(),
        source:"E19 short() coordinate conversion; generic integer polynomial, not a gluing-model certificate".into(),
        variables:vec!["X".into(),"a2".into(),"a4".into(),"a6".into()],degree_bound:3,
        lhs:a(vec![power(z.clone(),3),m(vec![a(vec![m(vec![n(9),v("a4")]),m(vec![n(-3),power(v("a2"),2)])]),z]),m(vec![n(27),v("a6")]),m(vec![n(-9),v("a2"),v("a4")]),m(vec![n(2),power(v("a2"),3)])]),
        rhs:m(vec![n(27),a(vec![power(v("X"),3),m(vec![v("a2"),power(v("X"),2)]),m(vec![v("a4"),v("X")]),v("a6")])])});
    for size in [2usize, 4, 8] {
        let factorial = |k: usize| (1..=k as i64).product::<i64>();
        let den = factorial(size - 1);
        let mut sum = vec![];
        for i in 0..size {
            let d = factorial(i)
                * factorial(size - 1 - i)
                * if (size - 1 - i) % 2 == 0 { 1 } else { -1 };
            let mut factors = vec![n(den / d)];
            for j in 0..size {
                if j != i {
                    factors.push(a(vec![v("x"), n(-(j as i64))]));
                }
            }
            sum.push(m(factors));
        }
        statements.push(S{name:format!("cleared {size}-point Lagrange partition of unity"),
            source:"E30 finite-grid interpolation; integer identity, denominators invertible on all frozen fields".into(),
            variables:vec!["x".into()],degree_bound:(size-1) as u32,lhs:a(sum),rhs:n(den)});
    }
    let pair_lift = argv.get(2).is_some_and(|s| s == "pair-lift");
    let proof = if pair_lift {
        let cubic = |x: E| a(vec![power(x.clone(), 3), m(vec![v("a"), x]), v("b")]);
        let middle = |x: E, y: E| {
            a(vec![
                m(vec![
                    a(vec![x.clone(), y.clone()]),
                    a(vec![m(vec![x, y]), v("a")]),
                ]),
                m(vec![n(2), v("b")]),
            ])
        };
        let difference = |x: E, y: E| a(vec![x, m(vec![n(-1), y])]);
        let s3 = a(vec![
            m(vec![power(difference(v("x"), v("y")), 2), power(v("z"), 2)]),
            m(vec![n(-2), middle(v("x"), v("y")), v("z")]),
            power(difference(m(vec![v("x"), v("y")]), v("a")), 2),
            m(vec![n(-4), v("b"), a(vec![v("x"), v("y")])]),
        ]);
        let numerator = a(vec![
            m(vec![n(2), power(difference(v("x"), v("z")), 2), v("y")]),
            m(vec![n(-2), middle(v("x"), v("z"))]),
        ]);
        statements=vec![S{name:"classical F3 discriminant numerator square identity".into(),source:"lean-v3/TwistPairLift.lean numerator_square; standard Semaev identity, no novelty claim".into(),variables:["x","y","z","a","b"].iter().map(|s|s.to_string()).collect(),degree_bound:6,lhs:power(numerator,2),rhs:a(vec![m(vec![n(16),cubic(v("x")),cubic(v("z"))]),m(vec![n(4),power(difference(v("x"),v("z")),2),s3])])}];
        let path = "research/twist_routes_1_3_20261021/lean-v3/TwistPairLift.lean";
        Some(certificate::ProofRef {
            kind: "lean4".into(),
            path: path.into(),
            sha256: hex::encode(source_sha256::sha256(&fs::read(path)?)),
        })
    } else {
        None
    };
    let mut records = vec![];
    for (index, statement) in statements.into_iter().enumerate() {
        let cert = certificate::issue(
            statement,
            20261023,
            32,
            "native issuer plus extra domain-separated checker points; this is same-host replay",
            proof.clone(),
        )
        .map_err(|e| format!("issue: {e:?}"))?;
        let verdict = certificate::check_with_extra(&cert, Some((0x2026_1023_4944_4331, 64)))
            .map_err(|e| format!("check: {e:?}"))?;
        let good = matches!(&verdict, certificate::Verdict::Accept { .. });
        assert!(good, "certificate checker rejected issued identity");
        let path = format!("{index}-{}.json", cert.id);
        fs::write(out.join(&path), serde_json::to_string_pretty(&cert)? + "\n")?;
        let mut false_statement = cert.statement.clone();
        false_statement.rhs = a(vec![false_statement.rhs, n(1)]);
        assert!(
            certificate::issue(false_statement, 20261023, 32, "negative control", None).is_err()
        );
        records.push(json!({"id":cert.id,"name":cert.statement.name,"file":path,"issued_points":32,"extra_checker_points":64,"false_plus_one_rejected":true,"scope":"evaluation over 2^61-1; exact field-specific map checks are separate"}));
    }
    fs::write(
        out.join("SUMMARY.json"),
        serde_json::to_string_pretty(
            &json!({"schema":"twist.followup-universal-identities/v1","certificates":records,"performance_claim":false,"full_formal_map_bridge_complete":false}),
        )? + "\n",
    )?;
    Ok(())
}
