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
fn p(xs: Vec<E>) -> E {
    E::product(xs)
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-wronskian PROOF_SOURCE OUTPUT".into());
    }
    let a2 = p(vec![v("A"), v("A")]);
    let b2 = p(vec![v("B"), v("B")]);
    let nd = E::sum(vec![
        p(vec![E::Const(2), v("A"), v("Ap")]),
        E::minus(p(vec![v("fp"), b2.clone()])),
        E::minus(p(vec![E::Const(2), v("f"), v("B"), v("Bp")])),
    ]);
    let n = sub(a2.clone(), p(vec![v("f"), b2.clone()]));
    let lhs = sub(p(vec![v("G"), nd]), p(vec![v("Gp"), n]));
    let rhs = E::sum(vec![
        p(vec![E::Const(2), v("G"), v("A"), v("Ap")]),
        E::minus(p(vec![v("G"), v("fp"), b2.clone()])),
        E::minus(p(vec![E::Const(2), v("G"), v("f"), v("B"), v("Bp")])),
        E::minus(p(vec![v("Gp"), a2])),
        p(vec![v("Gp"), v("f"), b2]),
    ]);
    let statement=IdentityStatement {
        name:"Sparse differential norm residual expansion".into(),
        source:"NormWronskian.normDerivative and definition of filter; expression expansion only, not a solver or characteristic-17 certificate".into(),
        variables:["A","Ap","B","Bp","f","fp","G","Gp"].map(str::to_string).to_vec(),degree_bound:4,lhs,rhs
    };
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    let cert=identity::issue(statement,20261113,32,"Unsigned producer certificate; derivatives and kernel implications checked separately in Lean",Some(ProofRef{kind:"lean4".into(),path:args[1].clone(),sha256:digest})).map_err(|e|format!("{e:?}"))?;
    let verdict =
        identity::check_with_extra(&cert, Some((0x13112026, 32))).map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err(format!("identity rejected: {verdict:?}"));
    }
    std::fs::write(
        &args[2],
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}: {verdict:?}; expression identity only; checker does not execute Lean",
        cert.id
    );
    Ok(())
}
