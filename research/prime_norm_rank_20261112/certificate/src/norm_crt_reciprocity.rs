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
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-crt-reciprocity PROOF_SOURCE OUTPUT_DIR".into());
    }
    let (f, h, p, u, v, q, t) = (
        E::var("F"),
        E::var("H"),
        E::var("P"),
        E::var("U"),
        E::var("V"),
        E::var("Q"),
        E::var("T"),
    );
    let statement = IdentityStatement {
        name: "Universal CRT reconstruction residual".into(),
        source: "NormCRTReciprocity.reconstructionResidual; universal CommRing identity, without assuming Bezout or quotient equations".into(),
        variables: ["F", "H", "P", "U", "V", "Q", "T"].map(String::from).to_vec(),
        degree_bound: 3,
        lhs: sub(E::plus(
            mul(sub(mul(p.clone(), v.clone()), mul(f.clone(), q.clone())), h.clone()),
            mul(sub(mul(p.clone(), u.clone()), mul(h.clone(), t.clone())), f.clone()),
        ), p.clone()),
        rhs: sub(
            mul(p, sub(E::plus(mul(u, f.clone()), mul(v, h.clone())), E::Const(1))),
            mul(mul(f, h), E::plus(q, t)),
        ),
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert = identity::issue(
        statement,
        2026111303,
        32,
        "Unsigned expression checks; conditional degree/CRT claims require separate Lean proof",
        Some(ProofRef {
            kind: "lean4".into(),
            path: args[1].clone(),
            sha256: digest,
        }),
    )
    .map_err(|e| format!("{e:?}"))?;
    let verdict = identity::check_with_extra(&cert, Some((0x2026111330, 32)))
        .map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}"));
    }
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    std::fs::write(
        format!("{}/residual.identity.json", args[2]),
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; proof pointer not executed by this checker",
        cert.id
    );
    Ok(())
}
