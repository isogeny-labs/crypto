use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    fs::write(
        out.join("pre-call.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"torus.worklist.pre-call/v1","purpose":"fixed development qualification; no performance campaign",
        "binary_sha256":hash(&fs::read(env::current_exe()?)?),"limits":torus_worklist_solver::Limits::default(),
        "compiled_sources":{"Cargo.toml":hash(include_bytes!("../Cargo.toml")),"Cargo.lock":hash(include_bytes!("../Cargo.lock")),
        "src/lib.rs":hash(include_bytes!("lib.rs")),"src/controls.rs":hash(include_bytes!("controls.rs")),"src/main.rs":hash(include_bytes!("main.rs")),"frozen_public_targets":hash(torus_worklist_solver::controls::FIXTURE)}}))?,
    )?;
    let a = torus_worklist_solver::controls::run()?;
    fs::write(
        out.join("controls.json"),
        serde_json::to_vec_pretty(&a.report)?,
    )?;
    fs::write(
        out.join("actual-original.json"),
        serde_json::to_vec_pretty(&a.actual_original)?,
    )?;
    fs::write(
        out.join("actual-indexed.json"),
        serde_json::to_vec_pretty(&a.actual_augmented)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
