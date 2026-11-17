//! Native IDC1 replay for the polynomial parts of the actual-point Lean proofs.
//! IDC1 does not check field inverses, elliptic points, or Frobenius; Lean does.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr as E, IdentityCertificate, IdentityStatement, ProofRef, Verdict};
use serde_json::json;
use std::{fs, path::Path};

fn v(s: &str) -> E {
    E::var(s)
}
fn c(n: i64) -> E {
    E::Const(n)
}
fn pow(s: &str, n: usize) -> E {
    E::product(vec![v(s); n])
}
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn mul(a: E, b: E) -> E {
    E::product(vec![a, b])
}
fn sha(raw: &[u8]) -> String {
    hex::encode(sha_backend::sha256(raw))
}
const PREFIX: &str = "research/quadratic_twist_evidence_20261021/lean";

fn statements() -> Vec<(IdentityStatement, String)> {
    let tangent = add(mul(c(3), pow("x", 2)), mul(pow("d", 2), v("a")));
    let sum = add(mul(c(2), v("x1")), v("x2"));
    let src_x = sub(sub(pow("l", 2), v("x1")), v("x2"));
    let src_y = sub(mul(v("l"), sub(sum.clone(), pow("l", 2))), v("y1"));
    let d2a = mul(pow("d", 2), v("a"));
    let d3b = mul(pow("d", 3), v("b"));
    let cube = |e: E| E::product(vec![e.clone(), e.clone(), e]);
    let square = |e: E| mul(e.clone(), e);
    let raw = vec![
        (
            "tangent",
            "TwistGroupMap",
            "cleared_tangent_identity",
            vec!["a", "d", "u", "x"],
            6,
            sub(
                mul(tangent.clone(), pow("u", 2)),
                mul(v("d"), tangent.clone()),
            ),
            mul(sub(pow("u", 2), v("d")), tangent),
        ),
        (
            "add-x",
            "TwistGroupMap",
            "cleared_addX_identity",
            vec!["d", "u", "x1", "x2", "l"],
            4,
            sub(
                sub(
                    sub(mul(v("d"), pow("l", 2)), mul(pow("u", 2), v("x1"))),
                    mul(pow("u", 2), v("x2")),
                ),
                mul(pow("u", 2), src_x),
            ),
            mul(sub(v("d"), pow("u", 2)), pow("l", 2)),
        ),
        (
            "add-y",
            "TwistGroupMap",
            "cleared_addY_identity",
            vec!["d", "u", "x1", "x2", "y1", "l"],
            5,
            sub(
                sub(
                    mul(v("l"), sub(mul(pow("u", 2), sum), mul(v("d"), pow("l", 2)))),
                    mul(v("y1"), pow("u", 2)),
                ),
                mul(pow("u", 2), src_y),
            ),
            mul(sub(pow("u", 2), v("d")), pow("l", 3)),
        ),
        (
            "discriminant",
            "TwistEigenspace",
            "twist_delta",
            vec!["a", "b", "d"],
            9,
            mul(c(-16), add(mul(c(4), cube(d2a)), mul(c(27), square(d3b)))),
            mul(
                pow("d", 6),
                mul(c(-16), add(mul(c(4), pow("a", 3)), mul(c(27), pow("b", 2)))),
            ),
        ),
    ];
    raw.into_iter().map(|(name, module, decl, variables, degree_bound, lhs, rhs)| {
        let path = format!("{PREFIX}/{module}.lean");
        (IdentityStatement {
            name: format!("twist-group-map-{name}"),
            source: format!("{path}:{module}.{decl}; polynomial part only; actual field and point claims checked by Lean"),
            variables: variables.into_iter().map(str::to_string).collect(), degree_bound, lhs, rhs,
        }, path)
    }).collect()
}

fn verify(root: &Path, cert: &IdentityCertificate) -> Result<(), String> {
    let (expected, path) = statements()
        .into_iter()
        .find(|(s, _)| s.name == cert.statement.name)
        .ok_or("unrecognized statement")?;
    if serde_json::to_value(&expected).unwrap() != serde_json::to_value(&cert.statement).unwrap() {
        return Err("statement differs from the specified polynomial".into());
    }
    let proof = cert.proof.as_ref().ok_or("missing Lean source reference")?;
    if proof.kind != "lean4" || proof.path != path {
        return Err("wrong Lean source path/kind".into());
    }
    let raw = fs::read(root.join(&path)).map_err(|e| e.to_string())?;
    if proof.sha256 != sha(&raw) {
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
        other => Err(format!("certificate not accepted: {other:?}")),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4, "group-map-cert issue|check ROOT OUTDIR");
    let root = Path::new(&args[2]);
    let out = Path::new(&args[3]);
    let mut records = Vec::new();
    for (statement, path) in statements() {
        let file = out.join(format!("{}.json", statement.name));
        let cert: IdentityCertificate = match args[1].as_str() {
            "issue" => {
                fs::create_dir_all(out).unwrap();
                let proof = ProofRef {
                    kind: "lean4".into(),
                    path: path.clone(),
                    sha256: sha(&fs::read(root.join(&path)).unwrap()),
                };
                let cert = identity_certificate::issue(
                    statement.clone(),
                    20261023,
                    32,
                    "Native IDC1 replay plus 16 independent-seed points; no performance claim",
                    Some(proof),
                )
                .unwrap();
                fs::write(&file, serde_json::to_string_pretty(&cert).unwrap() + "\n").unwrap();
                cert
            }
            "check" => serde_json::from_slice(&fs::read(&file).unwrap()).unwrap(),
            _ => panic!("expected issue or check"),
        };
        verify(root, &cert).unwrap();
        let mut wrong_source = cert.clone();
        wrong_source.proof.as_mut().unwrap().sha256 = "0".repeat(64);
        let source_rejection = verify(root, &wrong_source).unwrap_err();
        assert_eq!(source_rejection, "Lean source SHA-256 mismatch");
        let mut false_statement = statement;
        false_statement.rhs = add(false_statement.rhs, c(1));
        let false_error = identity_certificate::issue(
            false_statement.clone(),
            20261023,
            32,
            "false-plus-one control",
            None,
        )
        .unwrap_err();
        let counterexample = match false_error {
            identity_certificate::IssueError::False(point) => point,
            other => panic!("expected arithmetic rejection, got {other:?}"),
        };
        let controls = json!({
            "false_identity":{"statement":false_statement,"counterexample":{
                "index":counterexample.index,"assignment":counterexample.assignment,
                "lhs":counterexample.lhs,"rhs":counterexample.rhs}},
            "false_source":{"certificate":wrong_source,"rejection":source_rejection}
        });
        let control_file = out.join(format!("{}-controls.json", cert.statement.name));
        let control_text = serde_json::to_string_pretty(&controls).unwrap() + "\n";
        if args[1] == "issue" {
            fs::write(control_file, &control_text).unwrap();
        } else {
            assert_eq!(fs::read_to_string(control_file).unwrap(), control_text);
        }
        records.push(json!({"id":cert.id,"statement":cert.statement.name,"proof":cert.proof,
            "issue_points":32,"extra_points":16,"false_plus_one":"rejected","false_source":source_rejection}));
    }
    let receipt = serde_json::to_string_pretty(&json!({"status":"passed","identities":records,
        "scope":"polynomial identities only; Lean separately checks division, group law and Frobenius"})).unwrap()+"\n";
    if args[1] == "issue" {
        fs::write(out.join("receipt.json"), &receipt).unwrap();
    } else {
        assert_eq!(
            fs::read_to_string(out.join("receipt.json")).unwrap(),
            receipt
        );
    }
    println!("PASS: four source-bound polynomial identities, 32+16 points each, false-identity and source-binding controls");
}
