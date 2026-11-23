use flate2::{write::GzEncoder, Compression};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, io::Write, path::PathBuf};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let pre = serde_json::to_vec_pretty(&json!({"schema":"torus.indexed-circuit.pre-call/v1",
        "purpose":"fixed original/augmented-root construction checks, no solver invocation/timing",
        "binary_sha256":hash(&fs::read(env::current_exe()?)?),"compiled_sources":{
            "Cargo.toml":hash(include_bytes!("../Cargo.toml")),"Cargo.lock":hash(include_bytes!("../Cargo.lock")),
            "src/lib.rs":hash(include_bytes!("lib.rs")),"src/controls.rs":hash(include_bytes!("controls.rs")),
            "src/main.rs":hash(include_bytes!("main.rs")),"original_full_system":hash(torus_indexed_circuit::controls::FULL)}}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut a = torus_indexed_circuit::controls::run()?;
    let mut gzip = GzEncoder::new(vec![], Compression::best());
    gzip.write_all(&a.trace)?;
    let packed = gzip.finish()?;
    a.report["trace"] = json!({"raw_bytes":a.trace.len(),"raw_sha256":hash(&a.trace),"gzip_bytes":packed.len(),"gzip_sha256":hash(&packed)});
    a.report["pre_call_sha256"] = json!(hash(&pre));
    fs::write(out.join("equivalence-trace.bin.gz"), packed)?;
    fs::write(
        out.join("controls.json"),
        serde_json::to_vec_pretty(&a.report)?,
    )?;
    fs::write(
        out.join("actual-augmented-system.json"),
        serde_json::to_vec(&a.system)?,
    )?;
    fs::write(
        out.join("actual-extended-witness.json"),
        serde_json::to_vec(&a.witness)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
