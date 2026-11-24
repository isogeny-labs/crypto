use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
fn main() {
    let here = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = here.join("../../..");
    for name in ["large_prime_filter.rs", "large_prime_filter_mst.rs", "large_prime_graph.rs", "jv_large_prime_rank_bound.rs", "large_prime_filter", "large_prime_graph"] {
        assert_eq!(fs::canonicalize(here.join("cryptanalysis").join(name)).expect("source link"),
            fs::canonicalize(root.join("src/cryptanalysis").join(name)).expect("original source"), "source link target changed: {name}");
    }
    let bytes = fs::read(here.join("SOURCE_SHA256SUMS")).expect("source pin manifest");
    println!("cargo:rerun-if-changed=SOURCE_SHA256SUMS");
    for line in std::str::from_utf8(&bytes).unwrap().lines() {
        let (hash, relative) = line.split_once("  ").expect("hash and source path");
        let path = root.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        let got = format!("{:x}", Sha256::digest(fs::read(&path).expect("pinned source")));
        assert_eq!(got, hash, "source pin changed: {relative}; review before refreshing");
    }
    println!("cargo:rustc-env=M31_GRAPH_SOURCE_MANIFEST_SHA256={:x}", Sha256::digest(bytes));
}
