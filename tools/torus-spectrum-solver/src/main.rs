use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--qualifier" {
        let files: [(&str, &[u8]); 7] = [
            ("Cargo.toml", include_bytes!("../Cargo.toml")),
            ("Cargo.lock", include_bytes!("../Cargo.lock")),
            ("src/lib.rs", include_bytes!("lib.rs")),
            ("src/model.rs", include_bytes!("model.rs")),
            ("src/backend.rs", include_bytes!("backend.rs")),
            ("src/controls.rs", include_bytes!("controls.rs")),
            ("src/main.rs", include_bytes!("main.rs")),
        ];
        let hashes: std::collections::BTreeMap<_, _> = files
            .into_iter()
            .map(|(n, b)| (n, format!("{:x}", Sha256::digest(b))))
            .collect();
        println!(
            "{}",
            serde_json::json!({"schema":"model-solver-source/v1","hashes":hashes})
        );
        return;
    }
    if args.len() != 3 || !matches!(args[1].as_str(), "--synthetic" | "--backend" | "--curve") {
        eprintln!("usage: torus-spectrum-solver (--synthetic|--backend|--curve) CVC5");
        std::process::exit(2);
    }
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let result = match args[1].as_str() {
        "--synthetic" => torus_spectrum_solver::controls::synthetic(&mut out),
        "--backend" => {
            torus_spectrum_solver::controls::backend_controls(Path::new(&args[2]), &mut out)
        }
        _ => torus_spectrum_solver::controls::curve_control(Path::new(&args[2]), &mut out),
    };
    if let Err(e) = result {
        let _ = out.flush();
        eprintln!("{e}");
        std::process::exit(1);
    }
}
