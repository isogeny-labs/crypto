use serde_json::json;
use std::{env, fs, path::PathBuf};
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let compiled = torus_branch_affine::controls::compiled_sources();
    fs::write(
        out.join("pre-call.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"torus.branch-affine.pre-call/v1","standing":"producer unsigned/unverified",
        "purpose":"fixed native development qualification, not benchmark/campaign",
        "compiled_sources":compiled,"binary_sha256":torus_affine_closure::sha256(&fs::read(env::current_exe()?)?)}))?,
    )?;
    match torus_branch_affine::controls::run(&out) {
        Ok(value) => fs::write(
            out.join("controls.json"),
            serde_json::to_vec_pretty(&value)?,
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
