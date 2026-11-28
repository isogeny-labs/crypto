//! Select exact production bytes; never rewrite or approximate an algorithm.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read(root: &Path, relative: &str) -> Vec<u8> {
    let path = root.join(relative);
    println!("cargo:rerun-if-changed={}", path.display());
    fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn unique_position(text: &str, anchor: &str, relative: &str) -> usize {
    let mut positions = text.match_indices(anchor);
    let position = positions
        .next()
        .unwrap_or_else(|| panic!("missing source anchor in {relative}: {anchor:?}"))
        .0;
    assert!(
        positions.next().is_none(),
        "source anchor is not unique in {relative}: {anchor:?}"
    );
    position
}

fn extract(
    root: &Path,
    out: &Path,
    relative: &str,
    expected_sha256: &str,
    start_anchor: &str,
    end_anchor: &str,
    generated_name: &str,
    scope: &str,
) -> Value {
    let bytes = read(root, relative);
    let source_sha256 = digest(&bytes);
    assert_eq!(
        source_sha256, expected_sha256,
        "production source changed: {relative}; review the selected closure and deliberately update its pinned hash"
    );
    let source = std::str::from_utf8(&bytes).expect("production Rust source must be UTF-8");
    let start = unique_position(source, start_anchor, relative);
    let end = unique_position(source, end_anchor, relative);
    assert!(start < end, "invalid source range in {relative}");
    let selected = &bytes[start..end];
    let generated_path = out.join(generated_name);
    fs::write(&generated_path, selected).expect("write verbatim source selection");
    assert_eq!(
        fs::read(&generated_path).expect("read back source selection"),
        selected,
        "generated source must equal the exact production byte range"
    );
    json!({
        "path": relative,
        "selection": "verbatim_byte_range",
        "scope": scope,
        "whole_source_sha256": source_sha256,
        "whole_source_bytes": bytes.len(),
        "pinned_whole_source_sha256": expected_sha256,
        "byte_start_inclusive": start,
        "byte_end_exclusive": end,
        "selected_sha256": digest(selected),
        "selected_bytes": selected.len(),
        "generated_name": generated_name,
        "rewritten": false
    })
}

fn inventory(root: &Path, relative: &str, role: &str) -> Value {
    let bytes = read(root, relative);
    json!({
        "path": relative,
        "selection": "whole_file",
        "role": role,
        "sha256": digest(&bytes),
        "bytes": bytes.len()
    })
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .join("../../..")
        .canonicalize()
        .expect("qualifier must stay inside its repository");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let extracted = vec![
        extract(
            &root,
            &out,
            "src/cryptanalysis/fp_backend.rs",
            "2722bbf0c1e58a079772af71680aabc514f7354e8b51bb3ed10ca5729a0da0e1",
            "use std::collections::HashMap;\n",
            "// ── Fp64: p < 2^63, Montgomery u64",
            "fp32_backend.rs",
            "production imports, PrimeField, generic helpers, and complete Fp32 implementation; no Fp64/wide backends or legacy tests",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/challenge_ladder.rs",
            "5dd3022fc9fe214b3d8775c55e5b04e5fa306b9442604f99ddd643d2fef7ccf2",
            "/// `a·b mod m`.\n",
            "/// Square root mod an odd prime by Tonelli–Shanks; `None` for a\n",
            "challenge_prime.rs",
            "production mulm, powm, and deterministic is_prime_u64 only",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/f4_fp.rs",
            "3dc5148e0faf82f340480bbde732fb36c37e5abbbfceb56e03e728b9828713c6",
            "/// A polynomial over `F_p`: `(exponent vector, coefficient)` terms with\n",
            "/// Opt-in, bounded trace storage with independent Buchberger checks on replay.\n",
            "poly_type.rs",
            "production Poly alias and its documentation only; no F4 solver",
        ),
    ];
    let full = [
        (
            "src/cryptanalysis/exact_modular_matrix.rs",
            "actual source included in full, including its unit tests",
        ),
        (
            "src/cryptanalysis/shared_macaulay.rs",
            "actual source included in full, including its unit tests",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/Cargo.toml",
            "qualifier dependency and profile configuration",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/Cargo.lock",
            "resolved registry versions and checksums",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/build.rs",
            "verbatim source selector and manifest producer",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/src/lib.rs",
            "qualifier module and manifest entry point",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/src/cryptanalysis.rs",
            "production module namespace and source include paths",
        ),
        (
            "research/s3_shared_elimination_20261122/qualifier/src/source_manifest.rs",
            "manifest output binary; no experiment or timing",
        ),
    ]
    .map(|(relative, role)| inventory(&root, relative, role));
    let manifest = json!({
        "schema": "shared-s3-source-component-manifest/v1",
        "scope": "two new modules and their selected production arithmetic/type dependency closure",
        "evidence_status": "build provenance only; test results require a separate receipt; PCC-unverified",
        "extracted_production_ranges": extracted,
        "whole_files": full,
        "full_root_library_compiled": false,
        "full_root_library_tests_included": false,
        "f4_solver_included": false,
        "integration_example_included": false,
        "complete_hermetic_build_manifest": false,
        "omitted_from_manifest": [
            "registry package bodies (their resolved checksums are in Cargo.lock)",
            "rustc, linker, system libraries, CPU state, and environment flags"
        ],
        "development_profile_only": true,
        "benchmark_executed": false,
        "cargo_profile": env::var("PROFILE").ok(),
        "opt_level": env::var("OPT_LEVEL").ok(),
        "target": env::var("TARGET").ok()
    });
    let manifest_bytes = serde_json::to_vec_pretty(&manifest).expect("serialize source manifest");
    fs::write(out.join("source-manifest.json"), manifest_bytes).expect("write source manifest");
}
