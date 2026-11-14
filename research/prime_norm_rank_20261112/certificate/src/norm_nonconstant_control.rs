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
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-nonconstant-control PROOF_SOURCE OUTPUT_DIR".into());
    }
    let (f, l, m) = (E::var("f"), E::var("L"), E::var("M"));
    let statement=IdentityStatement {
        name:"Universal two-line norm factorization".into(),
        source:"NormNonconstantControl.twoLineNorm; universal CommRing identity, separate from F17 control".into(),
        variables:["f","L","M"].map(String::from).to_vec(),degree_bound:4,
        lhs:sub(square(E::plus(f.clone(),mul(l.clone(),m.clone()))),
            mul(f.clone(),square(E::plus(l.clone(),m.clone())))),
        rhs:mul(sub(f.clone(),square(l)),sub(f,square(m))),
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert=identity::issue(statement,2026111305,32,
        "Unsigned universal expression check; F17 control and matrix hypotheses checked separately in Lean",
        Some(ProofRef {kind:"lean4".into(),path:args[1].clone(),sha256:digest}))
        .map_err(|e|format!("{e:?}"))?;
    let verdict = identity::check_with_extra(&cert, Some((0x2026111350, 32)))
        .map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}"));
    }
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    std::fs::write(
        format!("{}/two-lines.identity.json", args[2]),
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; proof pointer not executed by this checker",
        cert.id
    );
    Ok(())
}
