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
    let pre = serde_json::to_vec_pretty(&json!({"schema":"membership-circuit.pre-call/v1",
        "purpose":"fixed development correctness only; no solver, search, sampling or timing",
        "toy_candidate_selector_cases":6962,"actual_signed_point_cases":18,
        "mutation_plan":"change each variable of the first actual witness",
        "compiled_sources":{
            "src/lib.rs":digest(include_bytes!("lib.rs")),
            "src/controls.rs":digest(include_bytes!("controls.rs")),
            "src/main.rs":digest(include_bytes!("main.rs")),
            "Cargo.toml":digest(include_bytes!("../Cargo.toml")),
            "Cargo.lock":digest(include_bytes!("../Cargo.lock"))},
        "binary_sha256":digest(&fs::read(env::current_exe()?)?)}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut a = torus_coset_circuit::controls::run()?;
    for (name, v) in [
        ("membership-system.json.gz", &a.membership_system),
        ("point-system.json.gz", &a.point_system),
    ] {
        let raw = serde_json::to_vec(v)?;
        let mut gz = GzEncoder::new(Vec::new(), Compression::best());
        gz.write_all(&raw)?;
        let packed = gz.finish()?;
        a.report[name] = json!({"raw_sha256":digest(&raw),"raw_bytes":raw.len(),
            "gzip_sha256":digest(&packed),"gzip_bytes":packed.len()});
        fs::write(out.join(name), packed)?;
    }
    a.report["pre_call_sha256"] = json!(digest(&pre));
    a.report["toy_trace_sha256"] = json!(digest(&a.toy_trace));
    fs::write(out.join("toy-acceptance.bin"), a.toy_trace)?;
    fs::write(
        out.join("first-point-witness.json"),
        serde_json::to_vec_pretty(&a.first_witness)?,
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
