use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../../../..")
        .canonicalize()
        .unwrap();
    let path = root.join("src/cryptanalysis/ecbench/claim.rs");
    println!("cargo:rerun-if-changed={}", path.display());
    let source = fs::read(&path).unwrap();
    let whole_hash = format!("{:x}", Sha256::digest(&source));
    assert_eq!(
        whole_hash, "bcaefc6e0bc0c0e0c567b8f453e09c132d912272e74cd69898d0c1b764128230",
        "the reused native canonical JSON source changed"
    );
    let text = std::str::from_utf8(&source).unwrap();
    let start_anchor = "pub fn canonical_utf8(v: &Value) -> Result<String, String> {\n";
    let end_anchor = "/// `identity.sha256`: SHA-256 of the canonical bytes.\n";
    assert_eq!(text.matches(start_anchor).count(), 1);
    assert_eq!(text.matches(end_anchor).count(), 1);
    let start = text.find(start_anchor).unwrap();
    let end = text.find(end_anchor).unwrap();
    assert!(start < end);
    let selected = &source[start..end];
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(output.join("canonical_utf8.rs"), selected).unwrap();
    println!("cargo:rustc-env=PCC_CANONICAL_WHOLE_SHA256={whole_hash}");
    println!("cargo:rustc-env=PCC_CANONICAL_RANGE_START={start}");
    println!("cargo:rustc-env=PCC_CANONICAL_RANGE_END={end}");
    println!(
        "cargo:rustc-env=PCC_CANONICAL_SELECTED_SHA256={:x}",
        Sha256::digest(selected)
    );
}
