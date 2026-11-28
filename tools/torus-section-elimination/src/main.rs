use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};
use torus_section_elimination::Result;
fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut out = None;
    let mut qualify = false;
    let mut transport = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => out = Some(PathBuf::from(args.next().ok_or("output path")?)),
            "--qualify" => qualify = true,
            "--transport-only" => transport = true,
            _ => return Err("unknown argument".into()),
        }
    }
    if qualify {
        let files = [
            ("Cargo.toml", include_str!("../Cargo.toml")),
            ("Cargo.lock", include_str!("../Cargo.lock")),
            ("lib.rs", include_str!("lib.rs")),
            ("sparse.rs", include_str!("sparse.rs")),
            ("controls.rs", include_str!("controls.rs")),
            ("main.rs", include_str!("main.rs")),
        ];
        println!(
            "{}",
            json!({"kind":"source_qualifier","inputs":files.iter().map(|(p,b)|json!({"path":p,"sha256":format!("{:x}",Sha256::digest(b.as_bytes()))})).collect::<Vec<_>>(),"fixture_sha256":format!("{:x}",Sha256::digest(include_bytes!("../../torus-point-solver/FIXTURES.jsonl.gz"))),"backend_invocations":0})
        );
        return Ok(());
    }
    let mut w = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out.ok_or("--out required")?)
            .map_err(|e| e.to_string())?,
    );
    if transport {
        writeln!(w, "{}", json!({"kind":"transport_only","math_run":false}))
            .map_err(|e| e.to_string())?;
    } else {
        torus_section_elimination::controls::run(&mut w)?;
    }
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("section-elimination-control: {e}");
        std::process::exit(1);
    }
}
