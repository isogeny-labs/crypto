use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
fn main() {
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 2 && a[1] == "--qualifier" {
        let files: [(&str, &[u8]); 8] = [
            ("Cargo.toml", include_bytes!("../Cargo.toml")),
            ("Cargo.lock", include_bytes!("../Cargo.lock")),
            ("FIXTURES.jsonl.gz", include_bytes!("../FIXTURES.jsonl.gz")),
            ("src/lib.rs", include_bytes!("lib.rs")),
            ("src/model.rs", include_bytes!("model.rs")),
            ("src/process.rs", include_bytes!("process.rs")),
            ("src/controls.rs", include_bytes!("controls.rs")),
            ("src/main.rs", include_bytes!("main.rs")),
        ];
        let hashes: std::collections::BTreeMap<_, _> = files
            .into_iter()
            .map(|(n, b)| (n, format!("{:x}", Sha256::digest(b))))
            .collect();
        println!(
            "{}",
            serde_json::json!({"schema":"point-solver-source/v1","hashes":hashes})
        );
        return;
    }
    if a.len() != 3 || !matches!(a[1].as_str(), "--synthetic" | "--backend" | "--public") {
        eprintln!("usage: torus-point-solver (--synthetic|--backend|--public) CVC5");
        std::process::exit(2);
    }
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let r = match a[1].as_str() {
        "--synthetic" => torus_point_solver::controls::synthetic(&mut out),
        "--backend" => torus_point_solver::controls::backend_controls(Path::new(&a[2]), &mut out),
        _ => torus_point_solver::controls::public_control(Path::new(&a[2]), &mut out),
    };
    if let Err(e) = r {
        let _ = out.flush();
        eprintln!("{e}");
        std::process::exit(1);
    }
}
