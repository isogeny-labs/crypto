use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn qualifier() -> Value {
    let v = [
        ("Cargo.toml", include_bytes!("../Cargo.toml").as_slice()),
        ("Cargo.lock", include_bytes!("../Cargo.lock").as_slice()),
        ("src/lib.rs", include_bytes!("lib.rs").as_slice()),
        ("src/controls.rs", include_bytes!("controls.rs").as_slice()),
        ("src/main.rs", include_bytes!("main.rs").as_slice()),
    ];
    json!({"compiled_inputs":v.into_iter().map(|(p,b)|(p.to_string(),hash(b))).collect::<std::collections::BTreeMap<_,_>>()})
}
fn run() -> Result<(), String> {
    let a = env::args().collect::<Vec<_>>();
    if a.len() == 2 && a[1] == "--qualifier" {
        println!("{}", qualifier());
        return Ok(());
    }
    if a.len() != 3 || (a[1] != "--out" && a[1] != "--transport") {
        return Err("usage: --qualifier | --out FRESH | --transport FRESH".into());
    }
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&a[2])
            .map_err(|e| e.to_string())?,
    );
    if a[1] == "--transport" {
        writeln!(
            out,
            "{}",
            json!({"kind":"transport_only","mathematical_controls_executed":false})
        )
        .map_err(|e| e.to_string())?;
        return out.flush().map_err(|e| e.to_string());
    }
    writeln!(out,"{}",json!({"kind":"binary","sha256":hash(&fs::read(env::current_exe().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?),"qualification":qualifier()})).map_err(|e|e.to_string())?;
    let r = torus_anchored_ratio::controls::run(&mut out);
    out.flush().map_err(|e| e.to_string())?;
    r
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
