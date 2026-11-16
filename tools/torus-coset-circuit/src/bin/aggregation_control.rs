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
    let pre = serde_json::to_vec_pretty(&json!({"schema":"target-circuit.pre-call/v2",
        "purpose":"fixed native correctness only; no solver, sampling, timing or paid job",
        "toy_pair_target_check_cap":30000,"actual_fixture_signed_points":18,
        "actual_max_arity":8,"mutation_plan":"increment every variable of the actual six-member witness",
        "compiled_sources":{
            "src/lib.rs":digest(include_bytes!("../lib.rs")),
            "src/index.rs":digest(include_bytes!("../index.rs")),
            "src/aggregation.rs":digest(include_bytes!("../aggregation.rs")),
            "src/aggregation_controls.rs":digest(include_bytes!("../aggregation_controls.rs")),
            "src/controls.rs":digest(include_bytes!("../controls.rs")),
            "src/main.rs":digest(include_bytes!("../main.rs")),
            "src/bin/aggregation_control.rs":digest(include_bytes!("aggregation_control.rs")),
            "Cargo.toml":digest(include_bytes!("../../Cargo.toml")),
            "Cargo.lock":digest(include_bytes!("../../Cargo.lock")),
            "v1-controls-fixture":digest(torus_coset_circuit::aggregation_controls::FIXTURE)},
        "binary_sha256":digest(&fs::read(env::current_exe()?)?)}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut a = torus_coset_circuit::aggregation_controls::run()?;
    let raw = serde_json::to_vec(&a.system)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&raw)?;
    let packed = encoder.finish()?;
    a.report["system"] = json!({"raw_sha256":digest(&raw),"raw_bytes":raw.len(),
        "gzip_sha256":digest(&packed),"gzip_bytes":packed.len()});
    a.report["pre_call_sha256"] = json!(digest(&pre));
    a.report["trace_sha256"] = json!(digest(&a.trace));
    fs::write(out.join("six-member-system.json.gz"), packed)?;
    fs::write(
        out.join("first-six-member-witness.json"),
        serde_json::to_vec_pretty(&a.witness)?,
    )?;
    fs::write(out.join("toy-target-acceptance.bin"), a.trace)?;
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
