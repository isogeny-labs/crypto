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
fn p(v: Vec<E>) -> E {
    E::product(v)
}
fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn poly(cs: &[i64]) -> E {
    E::sum(
        cs.iter()
            .enumerate()
            .map(|(j, c)| {
                let mut factors = vec![E::Const(*c)];
                factors.extend((0..j).map(|_| E::var("X")));
                p(factors)
            })
            .collect(),
    )
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: norm-quotient-moment INTEGER_PROOF_SOURCE OUTPUT_DIR".into());
    }
    let aa = poly(&[6, 11, 5]);
    let ff = poly(&[2, 2, 0, 1]);
    let target = p(vec![
        poly(&[0, 1]),
        poly(&[-5, 1]),
        poly(&[-6, 1]),
        poly(&[-3, 1]),
    ]);
    let norm=IdentityStatement{name:"Integer lift of three-support F17 norm relation".into(),source:"NormQuotientInteger.normLift; universal integer identity, separately specialized by NormQuotientMoment.actualNorm".into(),variables:vec!["X".into()],degree_bound:4,lhs:sub(sub(p(vec![aa.clone(),aa]),ff.clone()),p(vec![E::Const(8),target])),rhs:p(vec![E::Const(17),poly(&[2,50,-19,13,1])])};
    let inverse=IdentityStatement{name:"Integer lift of quotient inverse relation".into(),source:"NormQuotientInteger.inverseLift; universal integer identity for the exact factored modulus".into(),variables:vec!["X".into()],degree_bound:4,lhs:sub(sub(p(vec![ff,poly(&[6,7])]),E::Const(1)),p(vec![poly(&[-16,1]),poly(&[-9,1]),poly(&[12,11,7])])),rhs:p(vec![E::Const(17),poly(&[-101,-74,-43,10])])};
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    for (index, (name, statement)) in [("norm", norm), ("inverse", inverse)]
        .into_iter()
        .enumerate()
    {
        let cert = identity::issue(
            statement,
            20261113 + index as u64,
            32,
            "Unsigned producer expression checks; not a finder or timing campaign",
            Some(ProofRef {
                kind: "lean4".into(),
                path: args[1].clone(),
                sha256: digest.clone(),
            }),
        )
        .map_err(|e| format!("{e:?}"))?;
        let verdict = identity::check_with_extra(&cert, Some((0x2026111320 + index as u64, 32)))
            .map_err(|e| format!("{e:?}"))?;
        if !matches!(verdict, Verdict::Accept { .. }) {
            return Err(format!("rejected: {verdict:?}"));
        }
        std::fs::write(
            format!("{}/{name}.identity.json", args[2]),
            serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        println!(
            "{name}: {}: {verdict:?}; universal proof pointer not executed by this checker",
            cert.id
        );
    }
    Ok(())
}
