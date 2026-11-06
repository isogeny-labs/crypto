//! Native custody and answer-check commands for the Curve Smash secp256k1 track.
//!
//! Run `generate` in a separate, credential-free custody worker. Only the public
//! job directory is shared with the solving worker. `verify` needs the public
//! job and a proposed answer; it never reads the custody audit file.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use crypto_lib::cryptanalysis::pollard_collab::job::HexPoint;
use crypto_lib::cryptanalysis::six_class_rho::SixClassJob;
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use num_traits::Zero;
use serde::Deserialize;
use serde_json::json;

const BETA: &str = "7ae96a2b657c07106e64479eac3434e99cf0497512f58995c1396c28719501ee";
const LAMBDA: &str = "5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72";
const PROFILE_SHA256: &str = "18c3338de676f1909715814325bffdb1513f1621d678b942e8a7a9563b8fd086";
const SOURCE_REVISION: &str = "1f084c1914c81a0cdc830b176453adba5dc19ab9";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Answer {
    x: String,
}

fn check_profile(job: &SixClassJob) -> Result<CurveParams, String> {
    let curve = CurveParams::secp256k1();
    let spec = &job.curve;
    let hex = |value: &str| BigUint::parse_bytes(value.as_bytes(), 16);
    if spec.version != 1
        || hex(&spec.p) != Some(curve.p.clone())
        || hex(&spec.a) != Some(curve.a.clone())
        || hex(&spec.b) != Some(curve.b.clone())
        || hex(&spec.order) != Some(curve.n.clone())
        || hex(&spec.generator.x) != Some(curve.gx.clone())
        || hex(&spec.generator.y) != Some(curve.gy.clone())
        || job.beta != BETA
        || job.lambda != LAMBDA
        || spec.negation_map
    {
        return Err("job does not match the pinned Curve Smash secp256k1 profile".into());
    }
    Ok(curve)
}

fn load_job(path: &Path) -> Result<SixClassJob, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn write_new(path: &Path, bytes: &[u8], mode: u32) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    file.write_all(bytes).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())
}

fn draw_scalar(order: &BigUint) -> Result<BigUint, String> {
    loop {
        let mut bytes = [0u8; 32];
        getrandom::getrandom(&mut bytes).map_err(|e| e.to_string())?;
        let scalar = BigUint::from_bytes_be(&bytes);
        if &scalar < order {
            // Zero is part of the upstream distribution, but the current
            // native walk cannot encode infinity. Stop rather than redraw.
            if scalar.is_zero() {
                return Err("uniform draw was zero; native worker cannot encode infinity".into());
            }
            return Ok(scalar);
        }
    }
}

fn generate(template: &Path, public: &Path, private: &Path) -> Result<(), String> {
    if public == private
        || public
            .parent()
            .ok_or("public path has no parent")?
            .canonicalize()
            .map_err(|e| e.to_string())?
            == private
                .parent()
                .ok_or("private path has no parent")?
                .canonicalize()
                .map_err(|e| e.to_string())?
    {
        return Err("public and private outputs need distinct directories".into());
    }
    let mut job = load_job(template)?;
    let curve = check_profile(&job)?;
    let scalar = draw_scalar(&curve.n)?;
    let target = curve.generator().scalar_mul_vartime(
        &scalar,
        &FieldElement::new(curve.a.clone(), curve.p.clone()),
    );
    job.curve.target = HexPoint::from_point(&target).ok_or("identity target")?;
    let context = job.build()?;
    let mut job_bytes = serde_json::to_vec_pretty(&job).map_err(|e| e.to_string())?;
    job_bytes.push(b'\n');
    let job_sha256 = hex::encode(sha256(&job_bytes));
    let mut audit_bytes = serde_json::to_vec_pretty(&json!({
        "schema": "crypto.sixclass.private-challenge.v1",
        "source_revision": SOURCE_REVISION,
        "target_profile_sha256": PROFILE_SHA256,
        "public_job_sha256": job_sha256,
        "job_id": context.job_id,
        "scalar": format!("{scalar:064x}")
    }))
    .map_err(|e| e.to_string())?;
    audit_bytes.push(b'\n');
    write_new(private, &audit_bytes, 0o600)?;
    write_new(public, &job_bytes, 0o644)?;
    println!(
        "{}",
        json!({
            "schema": "crypto.sixclass.challenge-publication.v1",
            "public_job_sha256": job_sha256,
            "job_id": context.job_id,
            "target_profile_sha256": PROFILE_SHA256
        })
    );
    Ok(())
}

fn check_answer(job: &SixClassJob, answer_bytes: &[u8]) -> Result<bool, String> {
    let curve = check_profile(job)?;
    let answer: Answer = serde_json::from_slice(answer_bytes).map_err(|e| e.to_string())?;
    if answer.x.len() != 64
        || !answer
            .x
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("answer x must be exactly 64 lowercase hexadecimal digits".into());
    }
    let scalar = BigUint::parse_bytes(answer.x.as_bytes(), 16).ok_or("invalid answer x")?;
    if scalar >= curve.n {
        return Err("answer x exceeds the subgroup order".into());
    }
    let ctx = job.build()?;
    Ok(curve.generator().scalar_mul_vartime(&scalar, &ctx.base.a) == ctx.base.q)
}

fn verify(public: &Path, answer: &Path) -> Result<(), String> {
    let job = load_job(public)?;
    let answer_bytes = fs::read(answer).map_err(|e| e.to_string())?;
    let valid = check_answer(&job, &answer_bytes)?;
    println!(
        "{}",
        json!({
            "schema": "crypto.sixclass.answer-check.v1",
            "job_id": job.id(),
            "public_job_sha256": hex::encode(sha256(&fs::read(public).map_err(|e| e.to_string())?)),
            "valid": valid
        })
    );
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [command, template, public, private] if command == "generate" =>
            generate(Path::new(template), Path::new(public), Path::new(private)),
        [command, public, answer] if command == "verify" =>
            verify(Path::new(public), Path::new(answer)),
        _ => Err("usage: six_class_challenge generate TEMPLATE.json PUBLIC.json PRIVATE.json | verify PUBLIC.json ANSWER.json".into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("six_class_challenge: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::pollard_collab::job::JobSpec;

    #[test]
    fn secp_answer_accepts_and_tamper_rejects() {
        let curve = CurveParams::secp256k1();
        let q = curve.generator();
        let spec = JobSpec::new(&curve, &q, "custody-test", 7).unwrap();
        let job = SixClassJob {
            curve: spec,
            beta: BETA.into(),
            lambda: LAMBDA.into(),
        };
        assert!(check_answer(&job, format!("{{\"x\":\"{:064x}\"}}", 1).as_bytes()).unwrap());
        assert!(!check_answer(&job, format!("{{\"x\":\"{:064x}\"}}", 2).as_bytes()).unwrap());
        assert!(check_answer(&job, b"{\"x\":\"1\"}").is_err());
        assert!(check_answer(&job, b"{\"x\":\"0000000000000000000000000000000000000000000000000000000000000001\",\"x\":\"0000000000000000000000000000000000000000000000000000000000000001\"}").is_err());
    }
}
