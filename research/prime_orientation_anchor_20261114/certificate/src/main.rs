use orientation_anchor_certificate::{hash_sha256, identity, statement};
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: orientation-anchor-certificate LEAN_SOURCE OUTPUT".into());
    }
    let source = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let digest = hex::encode(hash_sha256::sha256(&source));
    let cert = identity::issue(
        statement(),
        20261114,
        32,
        "Unsigned producer literal certificate. Fixed ring substep only; Lean separately checks the quantified construction. No measured runtime or solver.",
        Some(identity::ProofRef { kind: "lean4".into(), path: args[1].clone(), sha256: digest }),
    ).map_err(|e| format!("{e:?}"))?;
    let verdict =
        identity::check_with_extra(&cert, Some((0x14112026, 16))).map_err(|e| format!("{e:?}"))?;
    if !matches!(verdict, identity::Verdict::Accept { .. }) {
        return Err(format!("literal identity rejected: {verdict:?}"));
    }
    std::fs::write(
        &args[2],
        serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!("{}: {verdict:?}; literal ring substep only; no parameterized-family or Lean execution certification", cert.id);
    Ok(())
}
