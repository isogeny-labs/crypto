use serde_json::json;
use std::{env, fs, path::PathBuf};
fn run() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().nth(1).as_deref() == Some("verify") {
        let request = PathBuf::from(
            env::args()
                .nth(2)
                .ok_or("verify REQUEST PREPROCESS_REPORT")?,
        );
        let certificate = PathBuf::from(
            env::args()
                .nth(3)
                .ok_or("verify REQUEST PREPROCESS_REPORT")?,
        );
        if fs::metadata(&request)?.len() > (2 << 20)
            || fs::metadata(&certificate)?.len() > (64 << 20)
        {
            return Err("file interface resource refusal".into());
        }
        let raw_request = fs::read(request)?;
        let (system, protected) = torus_affine_closure::check::parse_request(&raw_request)?;
        let raw_certificate = fs::read(certificate)?;
        let v: serde_json::Value = serde_json::from_slice(&raw_certificate)?;
        let artifact: torus_affine_closure::Artifact =
            serde_json::from_value(v.get("artifact").unwrap_or(&v).clone())?;
        let output = torus_affine_closure::check::verify(&system, &protected, &artifact)?;
        println!(
            "{}",
            json!({"schema":"torus.affine-file-check/v1","standing":"producer unsigned/unverified","coefficient_and_prefix_status":"accepted",
            "request_sha256":torus_affine_closure::sha256(&raw_request),"certificate_file_sha256":torus_affine_closure::sha256(&raw_certificate),
            "original_system_sha256":torus_affine_closure::system_hash(&system),"output_system_sha256":torus_affine_closure::system_hash(&output),
            "variables":output.variables.len(),"equations":output.equations.len(),"producer_counters_checked":false,"primality_execution_or_signed_standing_checked":false})
        );
        return Ok(());
    }
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let source = torus_affine_closure::controls::compiled_sources();
    fs::write(
        out.join("pre-call.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"torus.affine.pre-call/v1","standing":"producer unsigned/unverified",
        "purpose":"fixed source/certificate correctness qualification, not performance campaign","binary_sha256":torus_affine_closure::sha256(&fs::read(env::current_exe()?)?),"compiled_sources":source}),
        )?,
    )?;
    match torus_affine_closure::controls::run(&out) {
        Ok(report) => fs::write(
            out.join("controls.json"),
            serde_json::to_vec_pretty(&report)?,
        )?,
        Err(e) => {
            fs::write(out.join("failure.txt"), &e)?;
            return Err(e.into());
        }
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
