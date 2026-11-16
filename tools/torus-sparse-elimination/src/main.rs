use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let pre = serde_json::to_vec_pretty(
        &json!({"schema":"torus.sparse-elimination.pre-call/v1","purpose":"fixed development qualification, no performance/solver campaign",
        "binary_sha256":hash(&fs::read(env::current_exe()?)?),"limits":torus_sparse_elimination::Limits::default(),
        "compiled_sources":{"Cargo.toml":hash(include_bytes!("../Cargo.toml")),"Cargo.lock":hash(include_bytes!("../Cargo.lock")),
        "src/lib.rs":hash(include_bytes!("lib.rs")),"src/controls.rs":hash(include_bytes!("controls.rs")),"src/main.rs":hash(include_bytes!("main.rs")),
        "frozen_full_system":hash(torus_sparse_elimination::controls::FULL_FIXTURE)}}),
    )?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let a = torus_sparse_elimination::controls::run()?;
    fs::write(
        out.join("controls.json"),
        serde_json::to_vec_pretty(&a.report)?,
    )?;
    fs::write(
        out.join("q1-k2-original-system.json"),
        serde_json::to_vec_pretty(&a.q1_system)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
