//! Issue or independently replay native identity.certificate/v1 records for
//! the Dickson/S3 gate. These records evaluate over p=2^61-1 and have no formal
//! proof pointer. A separate exact, checked-i128 polynomial normalizer verifies
//! the small identities over Z; this is not Lean or PCC verification.
//!
//! cargo run --release --example identity_certificate_dickson_s3 -- OUT_DIR
//! cargo run --release --example identity_certificate_dickson_s3 -- --check OUT_DIR SEED

use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};
use serde_json::json;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

fn v(name: &str) -> Expr {
    Expr::var(name)
}
fn c(value: i64) -> Expr {
    Expr::Const(value)
}
fn add(values: Vec<Expr>) -> Expr {
    Expr::sum(values)
}
fn mul(values: Vec<Expr>) -> Expr {
    Expr::product(values)
}
fn sub(a: Expr, b: Expr) -> Expr {
    add(vec![a, Expr::minus(b)])
}
fn sq(a: Expr) -> Expr {
    mul(vec![a.clone(), a])
}
fn cube(a: Expr) -> Expr {
    mul(vec![a.clone(), a.clone(), a])
}
fn curve_rhs(x: Expr) -> Expr {
    add(vec![cube(x.clone()), mul(vec![v("a"), x]), v("b")])
}

fn statement(
    name: &str,
    variables: &[&str],
    degree: u32,
    lhs: Expr,
    rhs: Expr,
) -> IdentityStatement {
    IdentityStatement {
        name: name.into(),
        source: "Dickson/S3 quotient gate derivation, 2026-11-22; exact integer polynomial expansion included in examples/identity_certificate_dickson_s3.rs".into(),
        variables: variables.iter().map(|s| (*s).into()).collect(),
        degree_bound: degree,
        lhs,
        rhs,
    }
}

fn statements() -> Vec<(&'static str, IdentityStatement)> {
    let standard = add(vec![
        mul(vec![sq(sub(v("x"), v("z"))), sq(v("r"))]),
        Expr::minus(mul(vec![
            c(2),
            add(vec![
                mul(vec![
                    add(vec![v("x"), v("z")]),
                    add(vec![mul(vec![v("x"), v("z")]), v("a")]),
                ]),
                mul(vec![c(2), v("b")]),
            ]),
            v("r"),
        ])),
        sq(sub(mul(vec![v("x"), v("z")]), v("a"))),
        Expr::minus(mul(vec![c(4), v("b"), add(vec![v("x"), v("z")])])),
    ]);
    let centered = sub(
        sq(sub(
            mul(vec![sub(v("x"), v("r")), sub(v("z"), v("r"))]),
            add(vec![mul(vec![c(3), sq(v("r"))]), v("a")]),
        )),
        mul(vec![
            c(4),
            curve_rhs(v("r")),
            add(vec![v("x"), v("z"), v("r")]),
        ]),
    );
    let d = sq(sub(v("x"), v("r")));
    let t = mul(vec![
        c(2),
        add(vec![
            mul(vec![
                add(vec![v("x"), v("r")]),
                add(vec![mul(vec![v("x"), v("r")]), v("a")]),
            ]),
            mul(vec![c(2), v("b")]),
        ]),
    ]);
    let u = sub(
        sq(sub(mul(vec![v("x"), v("r")]), v("a"))),
        mul(vec![c(4), v("b"), add(vec![v("x"), v("r")])]),
    );
    let trace = mul(vec![v("q"), add(vec![v("u"), v("v")])]);
    let norm = mul(vec![v("q"), v("u"), v("v")]);
    let unext = sub(sq(v("u")), c(2));
    let vnext = sub(sq(v("v")), c(2));
    vec![
        (
            "centered-s3",
            statement(
                "Centered S3 identity with f(r) substituted",
                &["x", "z", "r", "a", "b"],
                4,
                standard,
                centered,
            ),
        ),
        (
            "s3-discriminant",
            statement(
                "S3 discriminant equals 16 f(x) f(r)",
                &["x", "r", "a", "b"],
                6,
                sub(sq(t), mul(vec![c(4), d, u])),
                mul(vec![c(16), curve_rhs(v("x")), curve_rhs(v("r"))]),
            ),
        ),
        (
            "homogeneous-trace-step",
            statement(
                "Homogeneous Dickson doubling trace step",
                &["q", "u", "v"],
                4,
                mul(vec![sq(v("q")), add(vec![unext.clone(), vnext.clone()])]),
                sub(
                    sub(sq(trace.clone()), mul(vec![c(2), norm.clone(), v("q")])),
                    mul(vec![c(4), sq(v("q"))]),
                ),
            ),
        ),
        (
            "homogeneous-norm-step",
            statement(
                "Homogeneous Dickson doubling norm step",
                &["q", "u", "v"],
                6,
                mul(vec![sq(v("q")), unext, vnext]),
                add(vec![
                    sq(norm.clone()),
                    Expr::minus(mul(vec![c(2), sq(trace)])),
                    mul(vec![c(4), norm, v("q")]),
                    mul(vec![c(4), sq(v("q"))]),
                ]),
            ),
        ),
    ]
}

type ExactPolynomial = BTreeMap<Vec<u16>, i128>;

fn accumulate(p: &mut ExactPolynomial, exponent: Vec<u16>, coefficient: i128) {
    let next = p
        .get(&exponent)
        .copied()
        .unwrap_or(0)
        .checked_add(coefficient)
        .expect("exact coefficient overflow");
    if next == 0 {
        p.remove(&exponent);
    } else {
        p.insert(exponent, next);
    }
}

fn expand(e: &Expr, variables: &[String]) -> ExactPolynomial {
    match e {
        Expr::Var(name) => {
            let mut powers = vec![0; variables.len()];
            powers[variables
                .iter()
                .position(|v| v == name)
                .expect("declared variable")] = 1;
            BTreeMap::from([(powers, 1)])
        }
        Expr::Const(value) => {
            if *value == 0 {
                BTreeMap::new()
            } else {
                BTreeMap::from([(vec![0; variables.len()], *value as i128)])
            }
        }
        Expr::Neg(inner) => expand(inner, variables)
            .into_iter()
            .map(|(m, a)| (m, a.checked_neg().expect("exact coefficient overflow")))
            .collect(),
        Expr::Add(parts) => {
            let mut out = BTreeMap::new();
            for part in parts {
                for (m, a) in expand(part, variables) {
                    accumulate(&mut out, m, a);
                }
            }
            out
        }
        Expr::Mul(parts) => {
            let mut out: ExactPolynomial = BTreeMap::from([(vec![0u16; variables.len()], 1i128)]);
            for part in parts {
                let next = expand(part, variables);
                let mut product = BTreeMap::new();
                for (m, a) in &out {
                    for (n, b) in &next {
                        let exponents = m
                            .iter()
                            .zip(n)
                            .map(|(x, y)| (*x).checked_add(*y).expect("exact exponent overflow"))
                            .collect();
                        accumulate(
                            &mut product,
                            exponents,
                            a.checked_mul(*b).expect("exact coefficient overflow"),
                        );
                    }
                }
                out = product;
            }
            out
        }
    }
}

fn exact_check(s: &IdentityStatement) -> serde_json::Value {
    let lhs = expand(&s.lhs, &s.variables);
    let rhs = expand(&s.rhs, &s.variables);
    assert_eq!(
        lhs, rhs,
        "integer coefficient comparison failed: {}",
        s.name
    );
    json!({"equal_over_Z_by_checked_i128_expansion":true,
        "normalized_terms_per_side":lhs.len(),
        "max_absolute_normalized_coefficient":lhs.values().map(|v|v.checked_abs().unwrap()).max().unwrap_or(0).to_string(),
        "formal_kernel_verified":false})
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (checking, directory, checker_seed) = match args.as_slice() {
        [directory] => (false, Path::new(directory), 0x50A4_21E6_80C7_3D29),
        [flag, directory, seed] if flag == "--check" => {
            (true, Path::new(directory), seed.parse::<u64>()?)
        }
        _ => {
            return Err(
                "usage: identity_certificate_dickson_s3 OUT_DIR | --check OUT_DIR SEED".into(),
            )
        }
    };
    if !checking {
        std::fs::create_dir_all(directory)?;
    }
    let mut records = Vec::new();
    for (index, (slug, s)) in statements().into_iter().enumerate() {
        let exact = exact_check(&s);
        let path = directory.join(format!("{slug}.json"));
        let cert: IdentityCertificate = if checking {
            let cert: IdentityCertificate = serde_json::from_slice(&std::fs::read(&path)?)?;
            assert_eq!(
                cert.statement, s,
                "certificate differs from issuer's explicit identity"
            );
            cert
        } else {
            let cert=issue(s,0xD1C5_0A03_2026_1122u64+(index as u64),32,
                "Issued by the algebra derivation agent through unchanged native identity_certificate.rs; self-replayed with extra points. Independent replay must be recorded separately. No Lean or PCC verification.",None)?;
            std::fs::write(&path, format!("{}\n", serde_json::to_string_pretty(&cert)?))?;
            cert
        };
        let verdict = check_with_extra(&cert, Some((checker_seed, 64)))?;
        let (points, extra_points) = match verdict {
            Verdict::Accept {
                points,
                extra_points,
                ..
            } => (points, extra_points),
            other => return Err(format!("identity check failed: {other:?}").into()),
        };
        records.push(
            json!({"file":path.file_name().unwrap().to_string_lossy(),"id":cert.id,
            "modulus":cert.modulus,"points":points,"extra_points":extra_points,
            "proof":cert.proof,"exact_polynomial_check":exact}),
        );
    }
    let summary = json!({"schema":"dickson-s3-identity-issuance-v1",
        "mode":if checking{"replay"}else{"issue_and_self_replay"},
        "checker_seed":checker_seed,
        "certificate_scope":"native evaluated tier over 2^61-1; proof:null; no Lean/PCC proof",
        "additional_check":"exact checked-i128 coefficient normalization over Z; no formal proof kernel",
        "identities":records});
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
