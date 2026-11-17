//! Issue/replay two proposal identities using the unchanged native checker.
//! This produces algebra records; it does not solve an APS instance or time it.

#[allow(dead_code)]
#[path = "../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[allow(dead_code)]
#[path = "../../src/hash/sha256.rs"]
pub mod sha256;
mod hash {
    pub use crate::sha256;
}

use identity_certificate::{
    check, check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};
use std::{error::Error, fs, path::Path};

fn var(name: &str) -> Expr {
    Expr::var(name)
}
fn sum(terms: Vec<Expr>) -> Expr {
    Expr::sum(terms)
}
fn mul(terms: Vec<Expr>) -> Expr {
    Expr::product(terms)
}
fn neg(value: Expr) -> Expr {
    Expr::minus(value)
}
fn square(value: Expr) -> Expr {
    mul(vec![value.clone(), value])
}

fn statements() -> Vec<(&'static str, IdentityStatement)> {
    let x = var("X");
    let t2 = square(var("t"));
    let x3 = mul(vec![x.clone(), x.clone(), x.clone()]);
    let line = IdentityStatement {
        name: "APS nonhorizontal chart: substituted intersection cubic".into(),
        source: "research/aps_leads_20261105/README.md L1; substitution U=t^2,c=tV; proposed by this research round".into(),
        variables: ["X", "t", "V", "a", "b"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs: sum(vec![
            x3.clone(),
            neg(mul(vec![t2.clone(), square(sum(vec![x.clone(), var("V")]))])),
            mul(vec![var("a"), x.clone()]),
            var("b"),
        ]),
        rhs: sum(vec![
            x3,
            neg(mul(vec![t2.clone(), square(x.clone())])),
            mul(vec![
                sum(vec![var("a"), neg(mul(vec![Expr::Const(2), t2.clone(), var("V")]))]),
                x,
            ]),
            var("b"),
            neg(mul(vec![t2, square(var("V"))])),
        ]),
    };
    let totals: Vec<_> = (1..=3)
        .map(|i| sum(vec![var(&format!("s{i}")), var(&format!("d{i}"))]))
        .collect();
    let frontier = IdentityStatement {
        name: "APS disjoint frontier: three-part cardinality expansion".into(),
        source: "research/aps_leads_20261105/README.md L3; first-new-coordinate partition; proposed by this research round".into(),
        variables: ["s1", "s2", "s3", "d1", "d2", "d3"].map(str::to_owned).to_vec(),
        degree_bound: 3,
        lhs: sum(vec![
            mul(totals.clone()),
            neg(mul(vec![var("s1"), var("s2"), var("s3")])),
        ]),
        rhs: sum(vec![
            mul(vec![var("d1"), totals[1].clone(), totals[2].clone()]),
            mul(vec![var("s1"), var("d2"), totals[2].clone()]),
            mul(vec![var("s1"), var("s2"), var("d3")]),
        ]),
    };
    vec![("line-chart.json", line), ("frontier-count.json", frontier)]
}

fn checked(cert: &IdentityCertificate) -> Result<(), Box<dyn Error>> {
    match check_with_extra(cert, Some((0x4150_535f_4c45_4144, 32)))? {
        Verdict::Accept { .. } => Ok(()),
        other => Err(format!("native replay refused: {other:?}").into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().ok_or("expected issue or check")?;
    let directory = args.next().ok_or("expected certificate directory")?;
    if args.next().is_some() || !matches!(mode.as_str(), "issue" | "check") {
        return Err("usage: aps-lead-identities <issue|check> <directory>".into());
    }
    let directory = Path::new(&directory);
    if mode == "issue" {
        fs::create_dir_all(directory)?;
    }
    let mut records = Vec::new();
    for (filename, statement) in statements() {
        let path = directory.join(filename);
        let cert: IdentityCertificate = if mode == "issue" {
            let cert = issue(
                statement.clone(),
                0x2026_1105_0000_0001,
                32,
                "Producer-issued and producer-replayed with the existing native checker; no independent signer or formal proof",
                None,
            )?;
            checked(&cert)?;
            if path.exists() {
                return Err(format!("refusing to overwrite {}", path.display()).into());
            }
            fs::write(&path, format!("{}\n", serde_json::to_string_pretty(&cert)?))?;
            cert
        } else {
            serde_json::from_slice(&fs::read(&path)?)?
        };
        if cert.statement != statement {
            return Err(format!("{} binds another statement", path.display()).into());
        }
        checked(&cert)?;
        let mut false_statement = statement;
        false_statement.rhs = sum(vec![false_statement.rhs, Expr::Const(1)]);
        if issue(false_statement, 7, 32, "negative control", None).is_ok() {
            return Err("issuer accepted an identity with an added constant".into());
        }
        let mut bad_digest = cert.clone();
        bad_digest.value_digest = "0".repeat(64);
        if !matches!(check(&bad_digest)?, Verdict::DigestMismatch { .. }) {
            return Err("checker accepted a changed digest".into());
        }
        records.push(serde_json::json!({
            "file": filename,
            "id": cert.id,
            "native_replay": "accept",
            "certificate_points": cert.points,
            "extra_producer_replay_points": 32,
            "false_statement_refused": true,
            "changed_digest_refused": true,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema": "aps-lead-identity-receipt/v1",
            "mode": mode,
            "records": records,
            "standing": "unverified producer assertion",
            "scope": "Identity record construction/replay only; no chart-completeness, APS solve, timing or scaling result",
        }))?
    );
    Ok(())
}
