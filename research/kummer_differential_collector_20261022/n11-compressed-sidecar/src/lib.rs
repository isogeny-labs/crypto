use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "kummer-compressed-sidecar/v2";

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn read_json(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON")).expect("parse JSON")
}

pub fn sha256(path: &Path) -> String {
    let bytes = fs::read(path).expect("read checksum input");
    format!("{:x}", Sha256::digest(bytes))
}

pub fn u64_field(value: &serde_json::Value, name: &str) -> u64 {
    value[name]
        .as_u64()
        .unwrap_or_else(|| panic!("missing u64 field {name}"))
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}
