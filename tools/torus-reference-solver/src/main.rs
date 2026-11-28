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
    let pre = serde_json::to_vec_pretty(&json!({"schema":"public-reference.pre-call/v1",
        "purpose":"fixed public-input native correctness; no solver campaign, sampling, timing or paid job",
        "caps":{"raw_columns":64,"signed_half_tuples":131072},
        "fixtures":"existing full p101 base/13 public targets; actual six-column q1 fixture; complete finite compression comparisons",
        "compiled_sources":{
            "src/lib.rs":digest(include_bytes!("lib.rs")),"src/controls.rs":digest(include_bytes!("controls.rs")),
            "src/determinant.rs":digest(include_bytes!("determinant.rs")),"src/main.rs":digest(include_bytes!("main.rs")),
            "Cargo.toml":digest(include_bytes!("../Cargo.toml")),"Cargo.lock":digest(include_bytes!("../Cargo.lock")),
            "frozen-v2-fixture":digest(torus_reference_solver::controls::FIXTURE)},
        "binary_sha256":digest(&fs::read(env::current_exe()?)?)}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut a = torus_reference_solver::controls::run()?;
    let mut gzip = GzEncoder::new(Vec::new(), Compression::best());
    gzip.write_all(&a.trace)?;
    let packed = gzip.finish()?;
    a.report["compression_trace"] = json!({"raw_bytes":a.trace.len(),"raw_sha256":digest(&a.trace),"gzip_bytes":packed.len(),"gzip_sha256":digest(&packed)});
    a.report["pre_call_sha256"] = json!(digest(&pre));
    fs::write(out.join("compression-trace.bin.gz"), packed)?;
    fs::write(
        out.join("representative-tables.json"),
        serde_json::to_vec_pretty(&a.tables)?,
    )?;
    fs::write(
        out.join("vandermonde.identity.json"),
        serde_json::to_vec(&a.determinant)?,
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
        std::process::exit(1);
    }
}
