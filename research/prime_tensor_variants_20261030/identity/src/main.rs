//! Small native client of the repository's unchanged identity-certificate API.
//! The pointer to the separate Lean pair theorem is documented in README.md;
//! this record itself remains in the evaluated tier.
#![allow(dead_code)]

#[path = "../../../../src/hash/sha256.rs"]
pub mod sha256_impl;
mod hash {
    pub use crate::sha256_impl as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod certificate;

use certificate::{Expr, IdentityCertificate, IdentityStatement, Verdict};
use std::{env, fs};

fn rotate((a, b): (Expr, Expr)) -> (Expr, Expr) {
    (Expr::minus(Expr::plus(a.clone(), b)), a)
}

fn statement() -> IdentityStatement {
    let (a, b, z) = (Expr::var("a"), Expr::var("b"), Expr::var("z"));
    let (left, right) = rotate(rotate(rotate((a.clone(), b.clone()))));
    IdentityStatement {
        name: "Ternary pair rotation cubed, both coordinates".into(),
        source: "prime_tensor_variants_20261030 T2; S(a,b)=(-a-b,a); z mixes both coordinates"
            .into(),
        variables: vec!["a".into(), "b".into(), "z".into()],
        degree_bound: 2,
        lhs: Expr::plus(Expr::times(z.clone(), left), right),
        rhs: Expr::plus(Expr::times(z, a), b),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("issue") if args.len() == 3 => {
            let cert = certificate::issue(
                statement(),
                20261030,
                64,
                "Issued by the parent native harness. Additional check stream is domain-separated; this is not a claim of independent human certificate issuance.",
                None,
            )?;
            fs::write(
                &args[2],
                format!("{}\n", serde_json::to_string_pretty(&cert)?),
            )?;
            println!("issued {} (evaluated tier)", cert.id);
        }
        Some("check") if args.len() == 4 => {
            let cert: IdentityCertificate = serde_json::from_slice(&fs::read(&args[2])?)?;
            if cert.statement != statement() {
                return Err("certificate is not the stated ternary identity".into());
            }
            let seed = args[3].parse::<u64>()?;
            match certificate::check_with_extra(&cert, Some((seed, 64)))? {
                Verdict::Accept {
                    points,
                    extra_points,
                    ..
                } => {
                    println!("PASS {}: {points} sealed + {extra_points} extra points; evaluated tier only", cert.id);
                }
                other => return Err(format!("identity rejected: {other:?}").into()),
            }
        }
        _ => return Err("usage: identity-record issue OUTPUT | check CERT CHECKER_SEED".into()),
    }
    Ok(())
}
