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
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn sub(a: E, b: E) -> E {
    add(a, E::minus(b))
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
}
fn pow(a: E, n: usize) -> E {
    E::product(vec![a; n])
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: prime-norm-identity PROOF_SOURCE OUTPUT".into());
    }
    let xq = sub(v("X"), v("q"));
    let u = add(add(v("q"), v("x1")), v("x2"));
    let aa = add(mul(v("v"), xq.clone()), v("y"));
    let f = add(add(pow(v("X"), 3), mul(v("a"), v("X"))), v("b"));
    let base = mul(sub(v("X"), v("x1")), sub(v("X"), v("x2")));
    let lhs = add(sub(pow(aa, 2), f), mul(base, xq.clone()));
    let rank = mul(sub(pow(v("v"), 2), u.clone()), pow(xq.clone(), 2));
    let curve = sub(
        pow(v("y"), 2),
        add(add(pow(v("q"), 3), mul(v("a"), v("q"))), v("b")),
    );
    let linear = sub(
        mul(E::Const(2), mul(v("y"), v("v"))),
        sub(
            add(add(pow(v("q"), 2), v("a")), mul(v("q"), u)),
            mul(v("x1"), v("x2")),
        ),
    );
    let statement=IdentityStatement{name:"Normalized two-support norm defect".into(),source:"NormRank.defectIdentity; coefficient derivation from curve norm; no algorithmic novelty claim".into(),variables:["X","a","b","q","y","v","x1","x2"].map(str::to_string).to_vec(),degree_bound:4,lhs,rhs:add(add(rank,curve),mul(xq,linear))};
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert = identity::issue(
        statement,
        20261112,
        32,
        "Producer issue/check only; no independent signature or attestation",
        Some(ProofRef {
            kind: "lean4".into(),
            path: args[1].clone(),
            sha256: digest,
        }),
    )
    .map_err(|e| format!("{e:?}"))?;
    let verdict =
        identity::check_with_extra(&cert, Some((0x20261112, 32))).map_err(|e| format!("{e:?}"))?;
    match verdict {
        Verdict::Accept { .. } => {}
        _ => return Err(format!("identity rejected: {verdict:?}")),
    }
    std::fs::write(
        &args[2],
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; formal proof pointer is not executed by this certificate checker",
        cert.id
    );
    Ok(())
}
