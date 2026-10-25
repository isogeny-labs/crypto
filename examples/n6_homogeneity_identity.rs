//! Issue/replay the four N6 polynomial identities proved in Lean.
//! `cargo run --release --example n6_homogeneity_identity -- issue`
//! `cargo run --release --example n6_homogeneity_identity -- check PATH.json`
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, ProofRef, Verdict,
};
use std::{env, fs};

fn v(name: &str) -> Expr {
    Expr::var(name)
}
fn c(value: i64) -> Expr {
    Expr::Const(value)
}
fn add(xs: Vec<Expr>) -> Expr {
    Expr::sum(xs)
}
fn mul(xs: Vec<Expr>) -> Expr {
    Expr::product(xs)
}
fn sub(a: Expr, b: Expr) -> Expr {
    add(vec![a, Expr::minus(b)])
}
fn pow(x: Expr, n: usize) -> Expr {
    mul(vec![x; n])
}
fn scaled(name: &str, scale: bool) -> Expr {
    if scale {
        mul(vec![v("mu"), v(name)])
    } else {
        v(name)
    }
}
fn n5(scale: bool) -> Expr {
    sub(
        pow(scaled("s", scale), 2),
        mul(vec![v("h"), pow(scaled("w", scale), 2)]),
    )
}
fn n6(scale: bool, squared: bool) -> Expr {
    mul(vec![
        v("d"),
        pow(scaled("w", scale), if squared { 2 } else { 1 }),
    ])
}
fn aa(scale: bool, squared: bool) -> Expr {
    let n1 = sub(mul(vec![v("q"), scaled("s", scale)]), scaled("t", scale));
    let n2 = mul(vec![v("r"), scaled("s", scale)]);
    sub(mul(vec![n1, n5(scale)]), mul(vec![n2, n6(scale, squared)]))
}
fn cc(scale: bool, squared: bool) -> Expr {
    sub(
        mul(vec![scaled("s", scale), n5(scale)]),
        mul(vec![scaled("t", scale), n6(scale, squared)]),
    )
}
fn cube(expr: Expr) -> Expr {
    mul(vec![pow(v("mu"), 3), expr])
}
fn statement(index: usize) -> IdentityStatement {
    let (name, lhs, rhs) = match index {
        0 => (
            "N6 AA W2 homogeneous",
            aa(true, true),
            cube(aa(false, true)),
        ),
        1 => (
            "N6 CC W2 homogeneous",
            cc(true, true),
            cube(cc(false, true)),
        ),
        2 => (
            "N6 AA W1 exact defect",
            sub(aa(true, false), cube(aa(false, false))),
            mul(vec![
                pow(v("mu"), 2),
                sub(v("mu"), c(1)),
                v("r"),
                v("s"),
                v("d"),
                v("w"),
            ]),
        ),
        3 => (
            "N6 CC W1 exact defect",
            sub(cc(true, false), cube(cc(false, false))),
            mul(vec![
                pow(v("mu"), 2),
                sub(v("mu"), c(1)),
                v("t"),
                v("d"),
                v("w"),
            ]),
        ),
        _ => panic!("unknown identity index"),
    };
    IdentityStatement {
        name: name.into(),
        source: "research/projective_law_followup_20261022/n6-homogeneity/N6Homogeneity.lean"
            .into(),
        variables: ["q", "r", "s", "t", "h", "d", "w", "mu"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        degree_bound: 12,
        lhs,
        rhs,
    }
}
fn proof() -> ProofRef {
    ProofRef {
        kind: "lean4".into(),
        path: "research/projective_law_followup_20261022/n6-homogeneity/N6Homogeneity.lean".into(),
        sha256: "99e4d0288be44ac496d146b6471930a155a28ab606ae59aa696043f649878189".into(),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, mode] if mode == "issue" => {
            let certificates: Vec<_> = (0..4)
                .map(|index| {
                    issue(
                        statement(index),
                        0x2026_1025_0006,
                        32,
                        "issued by examples/n6_homogeneity_identity.rs; checked by the native identity-certificate module and the pinned Lean proof source",
                        Some(proof()),
                    )
                    .expect("N6 polynomial identity")
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&certificates).unwrap());
        }
        [_, mode, path] if mode == "check" => {
            let certificates: Vec<IdentityCertificate> =
                serde_json::from_slice(&fs::read(path).expect("certificate file")).unwrap();
            assert_eq!(certificates.len(), 4);
            for (index, certificate) in certificates.iter().enumerate() {
                assert_eq!(certificate.statement, statement(index));
                assert_eq!(certificate.proof, Some(proof()));
                match check_with_extra(certificate, Some((0x1025_2026_0006, 32))).unwrap() {
                    Verdict::Accept { .. } => println!("CHECKED\t{}", certificate.id),
                    other => panic!("certificate rejected: {other:?}"),
                }
            }
        }
        _ => panic!("usage: n6_homogeneity_identity issue | check CERTIFICATES.json"),
    }
}
