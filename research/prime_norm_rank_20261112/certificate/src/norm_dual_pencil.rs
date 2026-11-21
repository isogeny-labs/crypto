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
        return Err("usage: norm-dual-pencil PROOF_SOURCE OUTPUT_DIR".into());
    }
    let bottom = E::minus(E::sum(vec![
        p(vec![v("a"), v("U")]),
        p(vec![v("b"), v("V")]),
    ]));
    let off = p(vec![v("b"), v("y")]);
    let det = IdentityStatement {
        name: "Two-support dual pencil determinant".into(),
        source: "NormDualPencil.pencilDet; finite two-by-two identity only".into(),
        variables: ["U", "V", "y", "a", "b"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs: sub(
            p(vec![v("a"), bottom.clone()]),
            p(vec![off.clone(), off.clone()]),
        ),
        rhs: E::minus(E::sum(vec![
            p(vec![v("U"), v("a"), v("a")]),
            p(vec![v("V"), v("a"), v("b")]),
            p(vec![v("y"), v("y"), v("b"), v("b")]),
        ])),
    };
    let first = sub(bottom, p(vec![v("U"), v("a")]));
    let second = sub(
        p(vec![E::Const(2), v("y"), E::minus(off)]),
        p(vec![v("V"), v("a")]),
    );
    let cof=IdentityStatement{name:"Two-support dual pencil cofactor pairings".into(),source:"NormDualPencil.pencilCofactors; two coefficients encoded by independent tags; no generic search bound".into(),variables:["U","V","y","a","b","z1","z2"].map(str::to_string).to_vec(),degree_bound:4,lhs:E::sum(vec![p(vec![v("z1"),first]),p(vec![v("z2"),second])]),rhs:E::minus(E::sum(vec![p(vec![v("z1"),E::Const(2),v("U"),v("a")]),p(vec![v("z1"),v("V"),v("b")]),p(vec![v("z2"),v("V"),v("a")]),p(vec![v("z2"),E::Const(2),v("y"),v("y"),v("b")])]))};
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest: String = hash_sha256::sha256(&source)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    std::fs::create_dir_all(&args[2]).map_err(|e| e.to_string())?;
    for (index, (name, statement)) in [("det", det), ("cofactors", cof)].into_iter().enumerate() {
        let cert = identity::issue(
            statement,
            20261113 + index as u64,
            32,
            "Unsigned producer expression identity; full recovery theorem checked in Lean",
            Some(ProofRef {
                kind: "lean4".into(),
                path: args[1].clone(),
                sha256: digest.clone(),
            }),
        )
        .map_err(|e| format!("{e:?}"))?;
        let verdict = identity::check_with_extra(&cert, Some((0x2026111300 + index as u64, 32)))
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
            "{name}: {}: {verdict:?}; expression only; checker does not execute Lean",
            cert.id
        );
    }
    Ok(())
}
