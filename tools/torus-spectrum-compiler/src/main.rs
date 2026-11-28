use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--qualifier" {
        let inputs: [(&str, &[u8]); 8] = [
            ("Cargo.toml", include_bytes!("../Cargo.toml")),
            ("Cargo.lock", include_bytes!("../Cargo.lock")),
            ("FIXTURES.json", include_bytes!("../FIXTURES.json")),
            ("src/lib.rs", include_bytes!("lib.rs")),
            ("src/ir.rs", include_bytes!("ir.rs")),
            ("src/controls.rs", include_bytes!("controls.rs")),
            ("src/backend.rs", include_bytes!("backend.rs")),
            ("src/main.rs", include_bytes!("main.rs")),
        ];
        let hashes: std::collections::BTreeMap<_, _> = inputs
            .into_iter()
            .map(|(p, b)| (p, format!("{:x}", Sha256::digest(b))))
            .collect();
        println!(
            "{}",
            serde_json::json!({"schema":"spectrum-compiler-source/v1","hashes":hashes,"scope":"compiled-in crate source hashes; no signed execution standing"})
        );
        return;
    }
    if args.len() == 3 && args[1] == "--backend-admit" {
        if let Err(e) = torus_spectrum_compiler::backend::admit(
            Path::new(&args[2]),
            &mut std::io::stdout().lock(),
        ) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
    if args.len() != 2 {
        eprintln!("usage: torus-spectrum-compiler EMPTY_EXPORT_DIRECTORY | --backend-admit CVC5");
        std::process::exit(2);
    }
    let dir = Path::new(&args[1]);
    if !dir.is_dir() {
        eprintln!("export directory must already exist");
        std::process::exit(2);
    }
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    if let Err(e) = torus_spectrum_compiler::controls::run(Some(dir), &mut out) {
        let _ = out.flush();
        eprintln!("{e}");
        std::process::exit(1);
    }
}
