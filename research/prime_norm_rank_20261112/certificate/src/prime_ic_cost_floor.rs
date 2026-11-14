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
fn power(a: E, n: usize) -> E {
    E::product((0..n).map(|_| a.clone()).collect())
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: prime-ic-cost-floor PROOF_SOURCE OUTPUT_DIR".into());
    }
    let (p, b, t) = (E::var("p"), E::var("b"), E::var("t"));
    let statement=IdentityStatement {
        name:"Universal cost-surplus balance".into(),
        source:"PrimeICCostFloor.surplusIdentity; universal CommRing identity only, not ordered or logarithmic conclusions".into(),
        variables:["p","b","t"].map(String::from).to_vec(),degree_bound:5,
        lhs:E::plus(mul(sub(mul(t.clone(),power(b.clone(),3)),p.clone()),b.clone()),
            mul(sub(t.clone(),power(b.clone(),4)),t.clone())),
        rhs:sub(power(t,2),mul(p,b)),
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert=identity::issue(statement,2026111306,32,
        "Unsigned ring-identity check; ordered cost and entropy bounds require separate Lean proofs",
        Some(ProofRef {kind:"lean4".into(),path:args[1].clone(),sha256:digest}))
        .map_err(|e|format!("{e:?}"))?;
    let verdict = identity::check_with_extra(&cert, Some((0x2026111360, 32)))
        .map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("rejected: {verdict:?}"));
    }
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    std::fs::write(
        format!("{}/surplus.identity.json", args[2]),
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; proof pointer not executed by this checker",
        cert.id
    );
    Ok(())
}
