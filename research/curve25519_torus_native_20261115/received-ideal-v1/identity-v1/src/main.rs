//! Fixed identity-record issuance; separate from the frozen 102-input native control.
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash {
    pub use crate::sha256;
}
#[allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod certificate;
use certificate::{Expr as E, IdentityStatement, ProofRef, Verdict};

fn pow(x: E, n: usize) -> E {
    E::product(vec![x; n])
}
fn sub(x: E, y: E) -> E {
    E::plus(x, E::minus(y))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("exactly one fresh output directory required".into());
    }
    let out = Path::new(&args[1]);
    std::fs::create_dir(out)?;
    let a = E::var("A");
    let t = E::var("tau");
    let w = E::var("w");
    let j = E::times(E::Const(4), pow(t.clone(), 2));
    let b = E::times(pow(a.clone(), 2), sub(pow(t.clone(), 2), E::Const(1)));
    let c = sub(j.clone(), b.clone());
    let g = E::sum(vec![
        E::times(pow(j.clone(), 5), pow(w.clone(), 4)),
        E::product(vec![
            a.clone(),
            pow(j.clone(), 3),
            c.clone(),
            sub(
                E::times(pow(a.clone(), 2), j),
                E::times(E::Const(3), b.clone()),
            ),
            pow(w.clone(), 2),
        ]),
        E::times(pow(b, 3), pow(c.clone(), 2)),
    ]);
    let h = |epsilon: i64| {
        sub(
            E::product(vec![E::Const(32), pow(t.clone(), 5), pow(w.clone(), 2)]),
            E::product(vec![
                pow(a.clone(), 3),
                pow(sub(E::Const(epsilon), t.clone()), 3),
                c.clone(),
            ]),
        )
    };
    let statement = IdentityStatement {
        name: "Received-symbol denominator-cleared branch factorization".into(),
        source: "ReceivedIdeal.branch_factorization; PCC-20261123-09b3cb".into(),
        variables: vec!["A".into(), "tau".into(), "w".into()],
        degree_bound: 20,
        lhs: g,
        rhs: E::times(h(1), h(-1)),
    };
    let proof_path =
        "research/curve25519_torus_native_20261115/received-ideal-v1/lean/ReceivedIdeal.lean";
    let proof = ProofRef {
        kind: "lean4-source-pointer".into(),
        path: proof_path.into(),
        sha256: hex(&sha256::sha256(&std::fs::read(proof_path)?)),
    };
    let cert = certificate::issue(
        statement.clone(),
        20261124,
        32,
        "same producer; maintained checker replay, not an independent signed verdict",
        Some(proof),
    )?;
    let verdict = certificate::check_with_extra(&cert, Some((0x5245434549564544, 32)))?;
    if !matches!(
        verdict,
        Verdict::Accept {
            points: 32,
            extra_points: 32,
            ..
        }
    ) {
        return Err(format!("unexpected verdict: {verdict:?}").into());
    }
    let mut corrupt = cert.clone();
    corrupt.statement.rhs = E::plus(corrupt.statement.rhs, E::Const(1));
    corrupt.id = certificate::statement_id(&corrupt.statement);
    let corruption_refused = matches!(certificate::check(&corrupt)?, Verdict::Reject(_));
    if !corruption_refused {
        return Err("plus-one corruption accepted".into());
    }
    let mut false_statement = statement;
    false_statement.rhs = E::plus(false_statement.rhs, E::Const(1));
    if certificate::issue(false_statement, 20261124, 32, "negative control", None).is_ok() {
        return Err("false statement issued".into());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("identity.json"))?;
    writeln!(file, "{}", serde_json::to_string_pretty(&cert)?)?;
    let mut log = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("receipt.json"))?;
    writeln!(
        log,
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "id": cert.id, "degree_bound": 20, "points": 32, "extra_points": 32,
            "plus_one_refused_by_checker": corruption_refused, "false_issue_refused": true,
            "modulus": certificate::MODULUS, "verdict": format!("{verdict:?}"),
            "scope": "maintained fixed-field evaluation; Lean pointer not executed by this checker; same producer"
        }))?
    )?;
    Ok(())
}
