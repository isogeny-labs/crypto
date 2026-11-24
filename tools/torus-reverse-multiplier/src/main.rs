use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
};
fn hash(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
fn qualifier() -> Value {
    let inputs = [
        ("Cargo.toml", include_bytes!("../Cargo.toml").as_slice()),
        ("Cargo.lock", include_bytes!("../Cargo.lock").as_slice()),
        ("src/lib.rs", include_bytes!("lib.rs").as_slice()),
        ("src/poly.rs", include_bytes!("poly.rs").as_slice()),
        ("src/controls.rs", include_bytes!("controls.rs").as_slice()),
        ("src/main.rs", include_bytes!("main.rs").as_slice()),
    ];
    json!({"kind":"qualifier","compiled_inputs":inputs.into_iter().map(|(p,b)|(p.to_string(),hash(b))).collect::<std::collections::BTreeMap<_,_>>()})
}
fn run() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 && args[1] == "--qualifier" {
        println!("{}", qualifier());
        return Ok(());
    }
    if args.len() != 3 || args[1] != "--out" {
        return Err("usage: --qualifier | --out FRESH_JSONL".into());
    }
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])
        .map_err(|e| e.to_string())?;
    let mut out = BufWriter::new(file);
    writeln!(out,"{}",json!({"kind":"binary","sha256":hash(&fs::read(env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?),"qualification":qualifier(),"scope":"producer fixed correctness; no independent signed standing"})).map_err(|e|e.to_string())?;
    let result = torus_reverse_multiplier::controls::run(&mut out);
    out.flush().map_err(|e| e.to_string())?;
    result
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
