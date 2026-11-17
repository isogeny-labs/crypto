//! N=51 honest-prior panel. Run with:
//! cargo run --release --example seed_grind_honest_prior -- <samples> <seed-hex-64> <new-output-dir>
use crypto_lib::cryptanalysis::seed_grind_trapdoor::{modulus, observe, summarize};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn verify(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let report: serde_json::Value = serde_json::from_slice(&fs::read(dir.join("summary.json"))?)?;
    let summary = &report["summary"];
    let seed_vec = hex::decode(summary["seed_hex"].as_str().ok_or("missing seed_hex")?)?;
    let seed: [u8; 32] = seed_vec.try_into().map_err(|_| "seed must be 32 bytes")?;
    let count = summary["samples"].as_u64().ok_or("missing sample count")?;
    let bytes = fs::read(dir.join("observations.jsonl"))?;
    if report["raw_blake3"] != blake3::hash(&bytes).to_hex().to_string()
        || report["raw_bytes"] != bytes.len() as u64
    {
        return Err("raw evidence digest or size mismatch".into());
    }
    let irr = modulus();
    let mut observations = Vec::with_capacity(count as usize);
    let lines: Vec<_> = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect();
    if lines.len() != count as usize {
        return Err("raw observation count mismatch".into());
    }
    for (i, line) in lines.into_iter().enumerate() {
        let observed: serde_json::Value = serde_json::from_slice(line)?;
        let expected = observe(&seed, i as u64, &irr);
        if observed != serde_json::to_value(&expected)? {
            return Err(format!("audit replay mismatch at index {i}").into());
        }
        observations.push(expected);
    }
    if *summary != serde_json::to_value(summarize(&seed, &irr, &observations))? {
        return Err("summary replay mismatch".into());
    }
    println!(
        "verified {} observations; digest {}",
        count, report["raw_blake3"]
    );
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 && args[1] == "--verify" {
        return verify(Path::new(&args[2]));
    }
    if args.len() != 4 {
        return Err(
            "usage: seed_grind_honest_prior <samples> <seed-hex-64> <new-output-dir>".into(),
        );
    }
    let samples: u64 = args[1].parse()?;
    if samples == 0 || samples > 1_000_000 {
        return Err("samples must be in 1..=1000000".into());
    }
    let seed_vec = hex::decode(&args[2])?;
    let seed: [u8; 32] = seed_vec.try_into().map_err(|_| "seed must be 32 bytes")?;
    let dir = Path::new(&args[3]);
    fs::create_dir(dir)?; // Fail closed: never overwrite frozen evidence.
    let irr = modulus();
    let raw_path = dir.join("observations.jsonl");
    let mut raw = BufWriter::new(File::create(&raw_path)?);
    let mut all = Vec::with_capacity(samples as usize);
    let mut raw_hash = blake3::Hasher::new();
    for i in 0..samples {
        let observation = observe(&seed, i, &irr);
        let mut line = serde_json::to_vec(&observation)?;
        line.push(b'\n');
        raw.write_all(&line)?;
        raw_hash.update(&line);
        all.push(observation);
    }
    raw.flush()?;
    let summary = summarize(&seed, &irr, &all);
    let report = serde_json::json!({
        "summary": summary,
        "raw_file": "observations.jsonl",
        "raw_blake3": raw_hash.finalize().to_hex().to_string(),
        "raw_bytes": fs::metadata(&raw_path)?.len(),
        "command": "cargo run --release --example seed_grind_honest_prior -- <samples> <seed-hex-64> <new-output-dir>"
    });
    let mut summary_file = File::create(dir.join("summary.json"))?;
    serde_json::to_writer_pretty(&mut summary_file, &report)?;
    summary_file.write_all(b"\n")?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("seed_grind_honest_prior: {err}");
        std::process::exit(1);
    }
}
