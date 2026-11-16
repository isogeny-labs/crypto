#[allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod hash_sha256;
pub mod hash {
    pub use crate::hash_sha256 as sha256;
}
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};
fn mul(a: E, b: E) -> E {
    E::product(vec![a, b])
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn square(a: E) -> E {
    mul(a.clone(), a)
}
fn norm(a: E, b: E, f: E) -> E {
    sub(square(a), mul(f, square(b)))
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-pencil-functional PROOF_SOURCE OUTPUT_DIR".into());
    }
    let (a, b, q, t, f) = (
        E::var("A"),
        E::var("B"),
        E::var("Q"),
        E::var("T"),
        E::var("f"),
    );
    let statement = IdentityStatement {
        name: "Universal norm polarization".into(),
        source: "NormPencilFunctional.normPolarization; universal CommRing polynomial identity"
            .into(),
        variables: ["A", "B", "Q", "T", "f"].map(String::from).to_vec(),
        degree_bound: 3,
        lhs: sub(
            sub(
                norm(
                    E::plus(a.clone(), q.clone()),
                    E::plus(b.clone(), t.clone()),
                    f.clone(),
                ),
                norm(a.clone(), b.clone(), f.clone()),
            ),
            norm(q.clone(), t.clone(), f.clone()),
        ),
        rhs: mul(E::Const(2), sub(mul(a, q), mul(f, mul(b, t)))),
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert=identity::issue(statement,2026111304,32,
        "Unsigned universal expression check; separate Lean proof supplies functional/degree hypotheses",
        Some(ProofRef {kind:"lean4".into(),path:args[1].clone(),sha256:digest}))
        .map_err(|e|format!("{e:?}"))?;
    let verdict = identity::check_with_extra(&cert, Some((0x2026111340, 32)))
        .map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}"));
    }
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    std::fs::write(
        format!("{}/polarization.identity.json", args[2]),
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; proof pointer not executed by this checker",
        cert.id
    );
    Ok(())
}
