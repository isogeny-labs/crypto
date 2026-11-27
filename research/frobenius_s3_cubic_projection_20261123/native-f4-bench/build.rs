use std::{env, fs, path::PathBuf};

fn main() {
    let local = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let source = local.join("../../../src/cryptanalysis/semaev_prime_poly.rs");
    let target = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("m31-field.rs");
    let src = fs::read_to_string(&source).unwrap();
    let start = src.find("/// `F_p` arithmetic").unwrap();
    let end = src.find("    /// `S₃(u, v, X)` as coefficients").unwrap();
    let det_start = src.find("/// Sylvester resultant of").unwrap();
    let det_end = src.find("/// A dense polynomial in").unwrap();
    fs::write(
        &target,
        format!("{}\n}}\n{}", &src[start..end], &src[det_start..det_end]),
    )
    .unwrap();
    println!("cargo:rerun-if-changed={}", source.display());
    println!("cargo:rustc-env=S3_PRIME_FIELD={}", target.display());
}
