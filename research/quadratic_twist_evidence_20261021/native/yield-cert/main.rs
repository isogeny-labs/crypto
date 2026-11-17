//! IDC1 for the polynomial part of the conditional budget theorem only.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr as E, IdentityCertificate, IdentityStatement, ProofRef, Verdict};
use serde_json::{json, Value};
use std::{fs, path::Path};
const PROOF: &str = "research/quadratic_twist_evidence_20261021/lean/RelationCost.lean";
fn sha(b: &[u8]) -> String {
    hex::encode(sha_backend::sha256(b))
}
fn v(s: &str) -> E {
    E::var(s)
}
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn mul(a: E, b: E) -> E {
    E::product(vec![a, b])
}
fn sub(a: E, b: E) -> E {
    add(a, E::minus(b))
}
fn statement() -> IdentityStatement {
    IdentityStatement {
        name: "twist-yield-cleared-budget-difference".into(),
        source: format!("{PROOF}:RelationCost.cleared_budget_difference_identity; polynomial part only; positivity and inequalities are checked separately in Lean"),
        variables: ["oT","oE","rows","cT","cE","pT","pE"].map(str::to_string).into(),
        degree_bound: 3,
        lhs: sub(mul(add(mul(v("oT"),v("pT")),mul(v("rows"),v("cT"))),v("pE")),
                 mul(add(mul(v("oE"),v("pE")),mul(v("rows"),v("cE"))),v("pT"))),
        rhs: add(mul(mul(sub(v("oT"),v("oE")),v("pE")),v("pT")),
                 mul(v("rows"),sub(mul(v("cT"),v("pE")),mul(v("cE"),v("pT"))))),
    }
}
fn verify(root: &Path, cert: &IdentityCertificate) -> Result<(), String> {
    if serde_json::to_value(statement()).unwrap() != serde_json::to_value(&cert.statement).unwrap()
    {
        return Err("wrong polynomial statement".into());
    }
    let p = cert.proof.as_ref().ok_or("missing source binding")?;
    if p.kind != "lean4" || p.path != PROOF {
        return Err("wrong proof kind/path".into());
    }
    if p.sha256 != sha(&fs::read(root.join(PROOF)).map_err(|e| e.to_string())?) {
        return Err("Lean source SHA-256 mismatch".into());
    }
    match identity_certificate::check_with_extra(cert, Some((2381, 16)))
        .map_err(|e| e.to_string())?
    {
        Verdict::Accept {
            points: 32,
            extra_points: 16,
            ..
        } => Ok(()),
        x => Err(format!("not accepted: {x:?}")),
    }
}
fn record(mode: &str, out: &Path, name: &str, value: &Value) {
    let text = serde_json::to_string_pretty(value).unwrap() + "\n";
    if mode == "issue" {
        fs::write(out.join(name), text).unwrap();
    } else {
        assert_eq!(fs::read_to_string(out.join(name)).unwrap(), text);
    }
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    assert_eq!(a.len(), 4, "yield-cert issue|check ROOT OUTDIR");
    let (mode, root, out) = (a[1].as_str(), Path::new(&a[2]), Path::new(&a[3]));
    assert!(mode == "issue" || mode == "check");
    let cert: IdentityCertificate = if mode == "issue" {
        assert!(!out.exists(), "refusing to replace a certificate");
        fs::create_dir_all(out).unwrap();
        identity_certificate::issue(
            statement(),
            20261024,
            32,
            "Polynomial budget difference only; no measured advantage or regularity bound",
            Some(ProofRef {
                kind: "lean4".into(),
                path: PROOF.into(),
                sha256: sha(&fs::read(root.join(PROOF)).unwrap()),
            }),
        )
        .unwrap()
    } else {
        serde_json::from_slice(&fs::read(out.join("cleared-budget.json")).unwrap()).unwrap()
    };
    verify(root, &cert).unwrap();
    record(
        mode,
        out,
        "cleared-budget.json",
        &serde_json::to_value(&cert).unwrap(),
    );
    let mut wrong = cert.clone();
    wrong.proof.as_mut().unwrap().sha256 = "0".repeat(64);
    let rejection = verify(root, &wrong).unwrap_err();
    assert_eq!(rejection, "Lean source SHA-256 mismatch");
    let mut bad = statement();
    bad.rhs = add(bad.rhs, E::Const(1));
    let c = match identity_certificate::issue(
        bad.clone(),
        20261024,
        32,
        "false-plus-one control",
        None,
    )
    .unwrap_err()
    {
        identity_certificate::IssueError::False(p) => p,
        x => panic!("expected arithmetic rejection: {x:?}"),
    };
    record(
        mode,
        out,
        "controls.json",
        &json!({"false_identity":{"statement":bad,"counterexample":{
        "index":c.index,"assignment":c.assignment,"lhs":c.lhs,"rhs":c.rhs}},
        "false_source":{"certificate":wrong,"rejection":rejection}}),
    );
    record(
        mode,
        out,
        "receipt.json",
        &json!({"status":"passed","id":cert.id,"proof":cert.proof,
        "issue_points":32,"extra_points":16,"false_plus_one":"rejected","false_source":"rejected",
        "scope":"cleared polynomial only; no actual degree-of-regularity, useful-yield, transfer or speed assertion"}),
    );
    println!("PASS: source-bound cleared cost polynomial, 32+16 points and false identity/source controls");
}
