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
        return Err("usage: norm-branch-radical PROOF_SOURCE OUTPUT_DIR".into());
    }
    let (a, b, f, q, t, modulus, s) = (
        E::var("A"),
        E::var("B"),
        E::var("f"),
        E::var("Q"),
        E::var("T"),
        E::var("F"),
        E::var("s"),
    );
    let statement=IdentityStatement {
        name:"Universal branch-projection pairing identity".into(),
        source:"NormBranchRadical.projectionIdentity; universal CommRing identity, not a radical or search-cost proof".into(),
        variables:["A","B","f","Q","T","F","s"].map(String::from).to_vec(),
        degree_bound:3,
        lhs:sub(mul(a.clone(),sub(q.clone(),mul(s.clone(),modulus.clone()))),mul(f.clone(),mul(b.clone(),t.clone()))),
        rhs:sub(sub(mul(a.clone(),q),mul(f,mul(b,t))),mul(modulus,mul(s,a))),
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert=identity::issue(statement,2026111307,32,
        "Unsigned universal expression check; branch guards, radical and norm transfer checked separately in Lean",
        Some(ProofRef{kind:"lean4".into(),path:args[1].clone(),sha256:digest})).map_err(|e|format!("{e:?}"))?;
    let verdict = identity::check_with_extra(&cert, Some((0x2026111370, 32)))
        .map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}"));
    }
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    std::fs::write(
        format!("{}/projection.identity.json", args[2]),
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; proof pointer not executed by this checker",
        cert.id
    );
    Ok(())
}
