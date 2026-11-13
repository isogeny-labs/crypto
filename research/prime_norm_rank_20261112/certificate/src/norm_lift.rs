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
fn v(s: &str) -> E {
    E::var(s)
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-lift PROOF_SOURCE OUTPUT".into());
    }
    let mut terms = Vec::new();
    let mut aa = Vec::new();
    let mut bb = Vec::new();
    let mut vars = vec!["f".to_string()];
    for i in 0..4 {
        let wi = v(&format!("w{i}"));
        let ai = v(&format!("a{i}"));
        let bi = v(&format!("b{i}"));
        vars.extend([format!("w{i}"), format!("a{i}"), format!("b{i}")]);
        aa.push(mul(wi.clone(), ai.clone()));
        bb.push(mul(wi.clone(), bi.clone()));
        for j in 0..4 {
            terms.push(mul(
                mul(wi.clone(), v(&format!("w{j}"))),
                sub(
                    mul(ai.clone(), v(&format!("a{j}"))),
                    mul(v("f"), mul(bi.clone(), v(&format!("b{j}")))),
                ),
            ));
        }
    }
    let a = E::sum(aa);
    let b = E::sum(bb);
    let statement=IdentityStatement {
        name:"Four-component polynomial norm lift expansion".into(),
        source:"NormLift.outerNorm, fixed four-component instance; universal finite-index theorem checked separately in Lean".into(),
        variables:vars,degree_bound:5,lhs:E::sum(terms),rhs:sub(mul(a.clone(),a),mul(v("f"),mul(b.clone(),b)))
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert = identity::issue(
        statement,
        20261113,
        32,
        "Unsigned producer certificate; no benchmark, algorithmic bound or independent signature",
        Some(ProofRef {
            kind: "lean4".into(),
            path: args[1].clone(),
            sha256: digest,
        }),
    )
    .map_err(|e| format!("{e:?}"))?;
    let verdict =
        identity::check_with_extra(&cert, Some((0x20261113, 32))).map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("identity rejected: {verdict:?}"));
    }
    std::fs::write(
        &args[2],
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; four-component identity only; checker does not execute Lean",
        cert.id
    );
    Ok(())
}
