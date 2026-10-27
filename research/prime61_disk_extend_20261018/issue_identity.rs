//! Issue/recheck the discriminant polynomial certificate; no genus/class claim.
use crypto_lib::{
    cryptanalysis::identity_certificate::{
        check_with_extra, issue, Expr, IdentityStatement, ProofRef, Verdict,
    },
    hash::sha256::sha256,
};
use serde_json::{json, Value};
use std::{env, fs};
fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 4, "GENUS_JSON LEAN_SOURCE OUTPUT_CERTIFICATE");
    let data: Value = serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();
    let d = data["discriminant"].as_i64().unwrap();
    let rows = data["rows"].as_array().unwrap();
    let variables: Vec<_> = (0..rows.len()).map(|i| format!("x{i}")).collect();
    let terms = rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let f = row["form"].as_array().unwrap();
            let a = Expr::Const(f[0].as_i64().unwrap());
            let b = Expr::Const(f[1].as_i64().unwrap());
            let c = Expr::Const(f[2].as_i64().unwrap());
            let residual = Expr::sum(vec![
                Expr::times(b.clone(), b),
                Expr::minus(Expr::product(vec![Expr::Const(4), a, c])),
                Expr::Const(-d),
            ]);
            Expr::times(residual, Expr::var(&variables[i]))
        })
        .collect();
    let proof_digest = sha256(&fs::read(&args[2]).unwrap())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let cert=issue(IdentityStatement{name:"Eight prime61 primitive-form discriminant residuals".into(),
        source:"prime61_disk_extend_20261018; GenusSpan.discriminant_polynomial_identity over Z".into(),variables,degree_bound:1,lhs:Expr::sum(terms),rhs:Expr::Const(0)},
        2026101801,32,"Same-session issuer and checker; 32 domain-separated extra points; not an independent issuer",
        Some(ProofRef{kind:"lean4".into(),path:args[2].clone(),sha256:proof_digest})).unwrap();
    let verdict = check_with_extra(&cert, Some((2026101802, 32))).unwrap();
    assert!(matches!(verdict, Verdict::Accept { .. }));
    fs::write(
        &args[3],
        serde_json::to_string_pretty(&cert).unwrap() + "\n",
    )
    .unwrap();
    println!(
        "{}",
        json!({"schema":"prime61.identity-check/v1","id":cert.id,"status":"PASS","certificate_points":32,"extra_checker_points":32,
        "scope":"Polynomial identity over F_(2^61-1); integer identity is proved by the checked Lean source. Does not certify genus characters or class coverage."})
    );
}
