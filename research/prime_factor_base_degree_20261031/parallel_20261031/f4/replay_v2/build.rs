use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
fn main() {
    let source = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../main.rs");
    println!("cargo:rerun-if-changed={}", source.display());
    let bytes = fs::read(&source).expect("frozen v1 source");
    let actual = format!("{:x}", Sha256::digest(&bytes));
    assert_eq!(
        actual, "e201eda231b41820b1733eefd725841036075aadceb9d4cada92926893025842",
        "frozen v1 helpers changed"
    );
    let text = String::from_utf8(bytes).unwrap();
    let (helpers, tests) = text
        .split_once("#[cfg(test)]\nmod tests {")
        .expect("frozen test-module marker");
    assert!(!tests.is_empty());
    let included = helpers
        .lines()
        .map(|line| {
            line.strip_prefix("//!")
                .map_or_else(|| line.to_string(), |s| format!("//{s}"))
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("frozen_v1_helpers.rs"),
        included,
    )
    .unwrap();
    println!("cargo:rustc-env=FROZEN_V1_SHA256={actual}");
}
