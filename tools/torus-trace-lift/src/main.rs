use flate2::{write::GzEncoder, Compression};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, io::Write, path::PathBuf};
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let pre = serde_json::to_vec_pretty(
        &json!({"schema":"torus.trace-lift.pre-call/v1","purpose":"fixed original-coordinate correctness; no solver campaign or timing",
        "binary_sha256":digest(&fs::read(env::current_exe()?)?),"compiled_sources":{
        "src/lib.rs":digest(include_bytes!("lib.rs")),"src/controls.rs":digest(include_bytes!("controls.rs")),
        "src/identities.rs":digest(include_bytes!("identities.rs")),"src/main.rs":digest(include_bytes!("main.rs")),
        "Cargo.toml":digest(include_bytes!("../Cargo.toml")),"Cargo.lock":digest(include_bytes!("../Cargo.lock"))}}),
    )?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut a = torus_trace_lift::controls::run()?;
    let mut gz = GzEncoder::new(vec![], Compression::best());
    gz.write_all(&a.trace)?;
    let packed = gz.finish()?;
    a.report["trace"] = json!({"raw_bytes":a.trace.len(),"raw_sha256":digest(&a.trace),"gzip_bytes":packed.len(),"gzip_sha256":digest(&packed)});
    a.report["pre_call_sha256"] = json!(digest(&pre));
    fs::write(out.join("equivalence-trace.bin.gz"), packed)?;
    fs::write(
        out.join("identities.json"),
        serde_json::to_vec(&a.identities)?,
    )?;
    fs::write(
        out.join("controls.json"),
        serde_json::to_vec_pretty(&a.report)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
