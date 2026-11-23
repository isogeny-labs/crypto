use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;

fn pinned_source(name: &str, expected_hash: &str) -> String {
    let source_dir = env::var_os("APS_PROBE_SOURCE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../.."));
    let path = source_dir.join("src/cryptanalysis").join(name);
    println!("cargo:rerun-if-changed={}", path.display());
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected_hash,
        "source changed: {}",
        path.display()
    );
    String::from_utf8(bytes).expect("UTF-8 repository source")
}

fn slices(source: &str, ranges: &[RangeInclusive<usize>]) -> String {
    let lines: Vec<_> = source.lines().collect();
    ranges
        .iter()
        .map(|range| lines[*range.start() - 1..*range.end()].join("\n"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("cargo:rerun-if-env-changed=APS_PROBE_SOURCE_ROOT");
    let output = std::path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let fast = pinned_source(
        "prime_fast.rs",
        "ace8a36eabd60f189d5b6a77e249e103091cc4353a944f52fe8027dfc175dc2a",
    );
    let aps = pinned_source(
        "aps_single_relation.rs",
        "3e71fa6b1dcec8661df3a3c1303712346c89e5d6de859939a841b1fd2ca3370f",
    );
    let residual = pinned_source(
        "residual_walk.rs",
        "4e54e030cebfc873f9288e25c83153841724d1cc0d8c79417288aa9e74f507c9",
    );
    fs::write(
        output.join("fast.rs"),
        slices(&fast, &[58..=511, 529..=544, 570..=832]),
    )
    .unwrap();
    fs::write(
        output.join("aps.rs"),
        slices(&aps, &[24..=205, 220..=228, 965..=1117, 1271..=1427]),
    )
    .unwrap();
    fs::write(
        output.join("residual.rs"),
        slices(&residual, &[113..=124, 192..=320]),
    )
    .unwrap();
}
