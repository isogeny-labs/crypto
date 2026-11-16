//! Polynomial preconditions for the experiment program, not an ECDLP solver.
//! Reuses the repository's identity certificate issuer/checker and SHA-256.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[path = "../../../../src/hash/sha256.rs"]
pub mod source_sha256;
mod hash {
    pub use crate::source_sha256 as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;

use identity::{Expr, IdentityCertificate, IdentityStatement};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Polynomial = BTreeMap<Vec<u16>, i128>;

fn v(s: &str) -> Expr {
    Expr::var(s)
}
fn c(n: i64) -> Expr {
    Expr::Const(n)
}
fn add(xs: Vec<Expr>) -> Expr {
    Expr::sum(xs)
}
fn mul(xs: Vec<Expr>) -> Expr {
    Expr::product(xs)
}
fn sub(x: Expr, y: Expr) -> Expr {
    add(vec![x, Expr::minus(y)])
}
fn pow(x: Expr, n: usize) -> Expr {
    mul(vec![x; n])
}
fn scale(n: i64, x: Expr) -> Expr {
    mul(vec![c(n), x])
}
fn f(x: Expr) -> Expr {
    add(vec![pow(x.clone(), 3), mul(vec![v("A"), x]), v("B")])
}

fn statement(name: &str, variables: &[&str], lhs: Expr, rhs: Expr) -> IdentityStatement {
    IdentityStatement {
        name: name.to_owned(),
        source:
            "research/twist_routes_1_3_20261021/EXPERIMENTS.md; E00 algebraic preconditions only"
                .to_owned(),
        variables: variables.iter().map(|x| x.to_string()).collect(),
        degree_bound: identity::syntactic_degree(&lhs).max(identity::syntactic_degree(&rhs)),
        lhs,
        rhs,
    }
}

fn statements() -> Vec<IdentityStatement> {
    let x = v("X");
    let y = v("Y");
    let m = v("m");
    let t = v("t");
    let q = add(vec![
        pow(x.clone(), 2),
        mul(vec![x.clone(), pow(m.clone(), 2)]),
        scale(2, mul(vec![m.clone(), y.clone()])),
        v("A"),
    ]);
    let delta = add(vec![
        pow(m.clone(), 4),
        scale(-6, mul(vec![x.clone(), pow(m.clone(), 2)])),
        scale(-8, mul(vec![y.clone(), m.clone()])),
        scale(-3, pow(x.clone(), 2)),
        scale(-4, v("A")),
    ]);
    let discriminant = sub(
        pow(sub(pow(m.clone(), 2), x.clone()), 2),
        scale(4, q.clone()),
    );
    let line = sub(mul(vec![m.clone(), sub(t.clone(), x.clone())]), y.clone());
    let quotient = add(vec![
        pow(t.clone(), 2),
        mul(vec![sub(x.clone(), pow(m.clone(), 2)), t.clone()]),
        q,
    ]);
    let division = add(vec![
        mul(vec![sub(t.clone(), x.clone()), quotient]),
        sub(f(x.clone()), pow(y.clone(), 2)),
    ]);
    let cc = v("c");
    let fc = f(cc.clone());
    let a2 = add(vec![scale(3, pow(cc.clone(), 2)), v("A")]);
    let ad3 = add(vec![
        scale(3, mul(vec![v("A"), pow(cc.clone(), 2)])),
        scale(9, mul(vec![v("B"), cc.clone()])),
        Expr::minus(pow(v("A"), 2)),
    ]);
    let bd27 = add(vec![
        scale(-27, mul(vec![v("B"), pow(cc.clone(), 3)])),
        scale(18, mul(vec![pow(v("A"), 2), pow(cc.clone(), 2)])),
        scale(27, mul(vec![v("A"), v("B"), cc.clone()])),
        scale(27, pow(v("B"), 2)),
        scale(2, pow(v("A"), 3)),
    ]);
    let change = add(vec![scale(3, fc.clone()), mul(vec![a2.clone(), t.clone()])]);
    vec![
        statement(
            "trace_quartic_discriminant",
            &["A", "X", "Y", "m"],
            delta,
            discriminant,
        ),
        statement(
            "trace_chord_polynomial_division",
            &["A", "B", "X", "Y", "m", "t"],
            sub(f(t.clone()), pow(line, 2)),
            division,
        ),
        statement(
            "cover_complement_A_cleared",
            &["A", "B", "c"],
            ad3.clone(),
            sub(
                scale(9, mul(vec![cc.clone(), fc.clone()])),
                pow(a2.clone(), 2),
            ),
        ),
        statement(
            "cover_complement_B_cleared",
            &["A", "B", "c"],
            bd27.clone(),
            add(vec![
                scale(2, pow(a2.clone(), 3)),
                scale(-27, mul(vec![a2, cc.clone(), fc.clone()])),
                scale(27, pow(fc.clone(), 2)),
            ]),
        ),
        statement(
            "cover_complement_change_of_variables",
            &["A", "B", "c", "t"],
            scale(
                27,
                mul(vec![pow(fc.clone(), 2), f(add(vec![t.clone(), cc]))]),
            ),
            add(vec![
                pow(change.clone(), 3),
                scale(3, mul(vec![pow(t.clone(), 2), ad3, change])),
                mul(vec![pow(t, 3), bd27]),
            ]),
        ),
    ]
}

fn accumulate(out: &mut Polynomial, powers: Vec<u16>, coefficient: i128) -> Result<()> {
    let value = out
        .get(&powers)
        .copied()
        .unwrap_or(0)
        .checked_add(coefficient)
        .ok_or("coefficient overflow")?;
    if value == 0 {
        out.remove(&powers);
    } else {
        out.insert(powers, value);
    }
    Ok(())
}
fn product(a: &Polynomial, b: &Polynomial) -> Result<Polynomial> {
    let mut out = Polynomial::new();
    for (pa, ca) in a {
        for (pb, cb) in b {
            let powers = pa
                .iter()
                .zip(pb)
                .map(|(a, b)| a.checked_add(*b).ok_or("degree overflow"))
                .collect::<std::result::Result<Vec<_>, _>>()?;
            accumulate(
                &mut out,
                powers,
                ca.checked_mul(*cb).ok_or("coefficient overflow")?,
            )?;
        }
    }
    Ok(out)
}
fn expand(e: &Expr, variables: &[String]) -> Result<Polynomial> {
    let zero = vec![0; variables.len()];
    Ok(match e {
        Expr::Var(s) => {
            let i = variables
                .iter()
                .position(|x| x == s)
                .ok_or("unknown variable")?;
            let mut powers = zero;
            powers[i] = 1;
            BTreeMap::from([(powers, 1)])
        }
        Expr::Const(n) => {
            if *n == 0 {
                Polynomial::new()
            } else {
                BTreeMap::from([(zero, i128::from(*n))])
            }
        }
        Expr::Neg(x) => {
            let mut out = Polynomial::new();
            for (p, c) in expand(x, variables)? {
                accumulate(&mut out, p, c.checked_neg().ok_or("negation overflow")?)?;
            }
            out
        }
        Expr::Add(xs) => {
            let mut out = Polynomial::new();
            for x in xs {
                for (p, c) in expand(x, variables)? {
                    accumulate(&mut out, p, c)?;
                }
            }
            out
        }
        Expr::Mul(xs) => {
            let mut out = BTreeMap::from([(zero, 1)]);
            for x in xs {
                out = product(&out, &expand(x, variables)?)?;
            }
            out
        }
    })
}

fn exact_check(s: &IdentityStatement) -> Result<usize> {
    identity::validate_statement(s)?;
    let lhs = expand(&s.lhs, &s.variables)?;
    let rhs = expand(&s.rhs, &s.variables)?;
    if lhs != rhs {
        return Err(format!("exact polynomial mismatch: {}", s.name).into());
    }
    Ok(lhs.len())
}
fn accepted(v: identity::Verdict) -> Result<()> {
    match v {
        identity::Verdict::Accept { .. } => Ok(()),
        _ => Err("certificate rejected".into()),
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || !["issue", "check"].contains(&args[1].as_str()) {
        return Err("usage: twist-route-preflight issue|check CERTIFICATE_DIRECTORY".into());
    }
    let out = Path::new(&args[2]);
    if args[1] == "issue" {
        if out.exists() {
            return Err("issuance requires a fresh output directory".into());
        }
        fs::create_dir_all(out)?;
    }
    let mut rows = vec![];
    for s in statements() {
        let terms = exact_check(&s)?;
        let path = out.join(format!("{}.json", s.name));
        let cert = if args[1] == "issue" {
            let c = identity::issue(s.clone(), 20261021, 32,
                "Authoring-session check with repository issuer plus separate exact integer coefficient expansion; independent-host replay not performed.", None)?;
            fs::write(&path, serde_json::to_string_pretty(&c)? + "\n")?;
            c
        } else {
            serde_json::from_slice::<IdentityCertificate>(&fs::read(&path)?)?
        };
        if cert.statement != s {
            return Err("certificate statement differs from compiled specification".into());
        }
        accepted(identity::check(&cert)?)?;
        accepted(identity::check_with_extra(
            &cert,
            Some((0x747769737463686b, 32)),
        )?)?;
        let mut false_statement = s.clone();
        false_statement.rhs = add(vec![false_statement.rhs, c(1)]);
        if exact_check(&false_statement).is_ok() {
            return Err("exact negative control accepted".into());
        }
        if identity::issue(false_statement, 20261021, 1, "negative control", None).is_ok() {
            return Err("modular negative control accepted".into());
        }
        rows.push(
            serde_json::json!({"name":s.name,"id":cert.id,"degree_bound":s.degree_bound,
            "exact_integer_coefficients_match":true,"expanded_terms":terms,
            "issuer_points":32,"additional_checker_points":32,"negative_controls_rejected":true,
            "formal_proof_checked":false,"independent_host_replay":false}),
        );
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema":"twist.identity-preflight/v1","mode":args[1],"identities":rows,
            "scope":"Polynomial preconditions only; no elliptic group theorem, useful factor base, cover transfer, solver or speedup is certified."
        }))?
    );
    Ok(())
}
