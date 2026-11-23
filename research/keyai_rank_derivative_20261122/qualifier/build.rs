//! Pin and select actual production bytes. No algorithm is replaced or rewritten.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

struct Range<'a> {
    start: &'a str,
    end: &'a str,
    scope: &'a str,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read(root: &Path, relative: &str) -> Vec<u8> {
    let path = root.join(relative);
    println!("cargo:rerun-if-changed={}", path.display());
    fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn pinned_read(root: &Path, relative: &str, pins: &Value) -> Vec<u8> {
    let expected = pins[relative]
        .as_str()
        .unwrap_or_else(|| panic!("missing whole-source pin for {relative}"));
    assert!(
        expected.len() == 64 && expected.bytes().all(|b| b.is_ascii_hexdigit()),
        "source must be reviewed and frozen before building: {relative}"
    );
    let bytes = read(root, relative);
    assert_eq!(
        digest(&bytes),
        expected,
        "production source changed: {relative}; review and freeze a new attempt before updating its pin"
    );
    bytes
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

/// Concatenate exact source intervals in source order. No imports, braces,
/// attributes, or function bodies are inserted into the generated selection.
fn extract(
    root: &Path,
    out: &Path,
    relative: &str,
    pins: &Value,
    ranges: &[Range<'_>],
    generated_name: &str,
) -> Value {
    let bytes = pinned_read(root, relative, pins);
    let source = std::str::from_utf8(&bytes).expect("production Rust source must be UTF-8");
    let mut generated = Vec::new();
    let mut selections = Vec::new();
    let mut previous_end = 0;
    for range in ranges {
        let start = unique_position(source, range.start, relative);
        let end = unique_position(source, range.end, relative);
        assert!(start < end, "empty or reversed source range in {relative}");
        assert!(
            previous_end <= start,
            "source selections must be nonoverlapping and in source order: {relative}"
        );
        previous_end = end;
        let selected = &bytes[start..end];
        let generated_start = generated.len();
        generated.extend_from_slice(selected);
        selections.push(json!({
            "scope": range.scope,
            "start_anchor": range.start,
            "end_anchor": range.end,
            "byte_start_inclusive": start,
            "byte_end_exclusive": end,
            "generated_byte_start_inclusive": generated_start,
            "generated_byte_end_exclusive": generated.len(),
            "selected_sha256": digest(selected),
            "selected_bytes": selected.len(),
            "rewritten": false
        }));
    }
    let path = out.join(generated_name);
    fs::write(&path, &generated).expect("write verbatim source selection");
    let written = fs::read(&path).expect("read back source selection");
    assert_eq!(written, generated, "source selection changed while writing");
    for selection in &selections {
        let offset = |key: &str| selection[key].as_u64().unwrap() as usize;
        assert_eq!(
            &written
                [offset("generated_byte_start_inclusive")..offset("generated_byte_end_exclusive")],
            &bytes[offset("byte_start_inclusive")..offset("byte_end_exclusive")],
            "generated interval must equal its exact production bytes"
        );
    }
    json!({
        "path": relative,
        "selection": "concatenated_verbatim_byte_ranges",
        "whole_source_sha256": digest(&bytes),
        "pinned_whole_source_sha256": pins[relative],
        "whole_source_bytes": bytes.len(),
        "ranges": selections,
        "generated_name": generated_name,
        "generated_sha256": digest(&generated),
        "generated_bytes": generated.len(),
        "inserted_bytes": 0,
        "rewritten": false
    })
}

fn inventory(root: &Path, relative: &str, role: &str, pins: Option<&Value>) -> Value {
    let bytes = match pins {
        Some(pins) => pinned_read(root, relative, pins),
        None => read(root, relative),
    };
    json!({
        "path": relative,
        "selection": "whole_file",
        "role": role,
        "sha256": digest(&bytes),
        "bytes": bytes.len(),
        "pinned": pins.is_some(),
        "rewritten": false
    })
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .join("../../..")
        .canonicalize()
        .expect("qualifier must stay inside its repository");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let prefix = "research/keyai_rank_derivative_20261122/qualifier";
    let pins: Value = serde_json::from_slice(&read(&root, &format!("{prefix}/source-pins.json")))
        .expect("read reviewed whole-source pins");

    let extracted = vec![
        extract(
            &root,
            &out,
            "src/cryptanalysis/challenge_ladder.rs",
            &pins,
            &[Range {
                start: "/// `a·b mod m`.\n",
                end: "/// Square root mod an odd prime by Tonelli–Shanks; `None` for a\n",
                scope: "mulm, powm, and actual challenge_ladder deterministic primality test",
            }],
            "challenge_prime.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/ic_boundary.rs",
            &pins,
            &[
                Range {
                    start: "/// The field operations behind the group operations: modular\n",
                    end: "/// Nanoseconds per native unit, measured on the host for the instance at\n",
                    scope: "complete FieldOps and GroupOps types and implementations",
                },
                Range {
                    start: "/// Cost of one phase: what it did (exact), what it took (wall) and what\n",
                    end: "/// One variant of one regime on one instance, priced end to end.\n",
                    scope: "complete PhaseCost, bump_counter, and methods used by FactorBase",
                },
                Range {
                    start: "// ── A fast hasher for point keys",
                    end: "fn mix(v: u64) -> u64 {",
                    scope: "actual map hasher, word modular arithmetic, IncrementalGauss, SiblingPair, and complete CountedGroup trait",
                },
                Range {
                    start: "/// `a` when `first`, else `b`, as one `cmov` where the target has one.  A\n",
                    end: "#[inline]\nfn negmod(a: u64, m: u64) -> u64 {",
                    scope: "actual pick_word architecture dispatch",
                },
                Range {
                    start: "pub struct CubicClasses {",
                    end: "impl RhoClasses<PrimeCurve> for CubicClasses {",
                    scope: "CubicClasses type and full inherent implementation required by PrimeCurve's automorphism method",
                },
                Range {
                    start: "// ── Prime-field curves on one word",
                    end: "/// A prime-field instance: curve, prime-order subgroup, generator.\n",
                    scope: "complete PrimePoint, PrimeCurve, CountedFp, Jacobi helper, and full CountedGroup implementation including native walk overrides",
                },
                Range {
                    start: "/// A materialised factor base with its column map.  Both `P` and `−P`\n",
                    end: "/// The prime-field base: the `size` smallest abscissae that are on the\n",
                    scope: "complete FactorBase type, count_index_build, and inherent implementation",
                },
                Range {
                    start: "#[inline(always)]\n#[allow(unused_unsafe)]\nfn prefetch_cache_line<T>(lines: &[T], index: usize) {",
                    end: "struct GuardLineSet {",
                    scope: "complete actual prefetch helper used by the unchanged point-key map",
                },
                Range {
                    start: "/// **The relation matrix, as a plug point.**\n",
                    end: "/// The relation \"matrix\" of a factor base whose column logarithms are\n",
                    scope: "actual RelationSolver trait and complete IncrementalGauss trait implementation",
                },
            ],
            "ic_boundary.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/ic_framework/stages.rs",
            &pins,
            &[Range {
                start: "/// Everything a stage may need to know about the instance it is\n",
                end: "/// **Stage 1 — the factor base.**\n",
                scope: "complete actual InstanceCtx public-input structure with no secret field",
            }],
            "instance_ctx.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/prime_fast.rs",
            &pins,
            &[
                Range {
                    start: "/// Largest field size the Montgomery layer supports (`p < 2^62` keeps\n",
                    end: "    /// Adopt a crate [`CurveParams`] whose `p` and `n` fit in 62 bits.\n",
                    scope: "complete Montgomery arithmetic, points, scratch types, and FastCurve::new; existing impl opening retained",
                },
                Range {
                    start: "    /// Group-order field context (Montgomery mod `n`) for the linear\n",
                    end: "    pub fn to_textbook(&self, p: Option<Pt>) -> Point {",
                    scope: "actual scalar_field, point, and canonical methods; unused from_params adapter omitted",
                },
                Range {
                    start: "    pub fn is_on_curve(&self, p: Pt) -> bool {\n",
                    end: "fn biguint_to_u64(v: &BigUint) -> Option<u64> {",
                    scope: "remaining FastCurve arithmetic methods including batched additions and existing impl closing brace; unused textbook conversion adapters omitted",
                },
            ],
            "prime_fast.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/residual_walk.rs",
            &pins,
            &[Range {
                start: "// ── u64 modular arithmetic",
                end: "fn isqrt(n: u64) -> u64 {",
                scope: "actual modular arithmetic, pow_mod/inv_mod with unchanged hooks, square root, and residual_walk primality test",
            }],
            "residual_arithmetic.rs",
        ),
        extract(
            &root,
            &out,
            "src/prime_hyperelliptic/ops.rs",
            &pins,
            &[
                Range {
                    start: "use std::cell::Cell;\n",
                    end: "/// Words a hash of a divisor absorbs: one per 64-bit limb of each\n",
                    scope: "actual counter types, tally operations, sort cost, and exponentiation charging used by jv_bitops",
                },
                Range {
                    start: "/// Everything this thread has tallied, by phase.\n",
                    end: "#[cfg(test)]\nmod tests {",
                    scope: "remaining actual tally snapshots and phase guards; unused divisor_hash_words omitted",
                },
            ],
            "hyperelliptic_ops.rs",
        ),
    ];

    let mut whole: Vec<_> = [
        (
            "src/cryptanalysis/ic_framework/useful_rank.rs",
            "complete touched production module; not a copied rank implementation",
        ),
        (
            "src/cryptanalysis/semaev_prime_poly.rs",
            "complete touched production module with existing polynomial operations",
        ),
        (
            "src/cryptanalysis/bitops.rs",
            "complete actual pricing definitions required by unchanged hooks",
        ),
        (
            "src/cryptanalysis/jv_bitops.rs",
            "complete actual counter hooks",
        ),
        (
            "src/cryptanalysis/jv_bitops_uniform.rs",
            "complete actual counter uniform-sampling child",
        ),
        (
            "src/cryptanalysis/ic_measurement.rs",
            "complete actual measurement hooks; no counter or dump stubs",
        ),
        (
            "examples/keyai_rank_derivative_controls.rs",
            "actual root example compiled directly as a binary test target",
        ),
    ]
    .map(|(relative, role)| inventory(&root, relative, role, Some(&pins)))
    .into_iter()
    .collect();
    for (file, role) in [
        (
            "Cargo.toml",
            "qualifier package dependencies, targets, and profile",
        ),
        ("Cargo.lock", "resolved registry versions and checksums"),
        ("build.rs", "verbatim selector and provenance producer"),
        (
            "source-pins.json",
            "reviewed production and example source pins",
        ),
        ("src/lib.rs", "source-closure namespace root"),
        (
            "src/cryptanalysis.rs",
            "namespace bindings and required imports",
        ),
        (
            "src/ic_framework.rs",
            "actual instance and useful-rank bindings",
        ),
        ("src/source_manifest.rs", "provenance output binary"),
        (
            "src/adversarial.rs",
            "independent correctness test entry point",
        ),
        (
            "src/support_module_tests.rs",
            "whole unchanged semaev module test binding; execution filters its six support tests",
        ),
        (
            "README.md",
            "declared qualification plan, scope, and commands",
        ),
    ] {
        whole.push(inventory(&root, &format!("{prefix}/{file}"), role, None));
    }

    let manifest = json!({
        "schema": "keyai-rank-derivative-source-manifest/v1",
        "scope": "actual KeyAI regression example and independent tests against complete touched production modules plus a declared exact-source dependency closure",
        "evidence_status": "build provenance only; tests require separate receipts; PCC-unverified",
        "extracted_production_ranges": extracted,
        "whole_files": whole,
        "production_sources_and_example_hash_pinned": true,
        "full_root_library_compiled": false,
        "full_root_library_tests_included": false,
        "broad_dependency_module_unit_tests_included": false,
        "whole_semaev_module_tests_compiled_in_separate_target": true,
        "original_support_module_test_target_included": true,
        "support_module_test_selection": "six original tests matching support; remaining tests compiled but not executed",
        "integration_example_included": true,
        "integration_example_rewritten": false,
        "algorithm_substitutions": false,
        "counter_stubs": false,
        "complete_hermetic_build_manifest": false,
        "omitted_from_manifest": [
            "registry package bodies (resolved checksums are in Cargo.lock)",
            "rustc, linker, system libraries, CPU state, and environment flags"
        ],
        "correctness_qualification_only": true,
        "benchmark_executed_by_build_script": false,
        "cargo_profile": env::var("PROFILE").ok(),
        "opt_level": env::var("OPT_LEVEL").ok(),
        "target": env::var("TARGET").ok()
    });
    fs::write(
        out.join("source-manifest.json"),
        serde_json::to_vec_pretty(&manifest).expect("serialize source manifest"),
    )
    .expect("write source manifest");
}
