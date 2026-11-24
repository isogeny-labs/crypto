//! Select pinned, exact production bytes. No algorithm is replaced or rewritten.

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

/// Concatenate only exact source byte ranges, in their declared order. A range
/// can contain several complete items, or an existing impl's method span. The
/// selector inserts no imports, braces, attributes, or function bodies.
fn extract(
    root: &Path,
    out: &Path,
    relative: &str,
    expected_sha256: &str,
    ranges: &[Range<'_>],
    generated_name: &str,
) -> Value {
    let bytes = read(root, relative);
    let source_sha256 = digest(&bytes);
    assert_eq!(
        source_sha256, expected_sha256,
        "production source changed: {relative}; review the dependency closure before deliberately updating its pinned hash"
    );
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
    // Check every generated interval directly against its production bytes,
    // independently of the concatenation's whole-file equality check.
    for selection in &selections {
        let offset = |key: &str| selection[key].as_u64().unwrap() as usize;
        assert_eq!(
            &written
                [offset("generated_byte_start_inclusive")..offset("generated_byte_end_exclusive")],
            &bytes[offset("byte_start_inclusive")..offset("byte_end_exclusive")],
            "generated source interval must equal its exact production range"
        );
    }
    json!({
        "path": relative,
        "selection": "concatenated_verbatim_byte_ranges",
        "whole_source_sha256": source_sha256,
        "pinned_whole_source_sha256": expected_sha256,
        "whole_source_bytes": bytes.len(),
        "ranges": selections,
        "generated_name": generated_name,
        "generated_sha256": digest(&generated),
        "generated_bytes": generated.len(),
        "inserted_bytes": 0,
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

/// Keep the real F4 parent/child layout when it is attached through #[path].
/// The parent is named mod.rs so its unchanged `pub mod reuse/certified`
/// declarations resolve the original children in the same directory.
fn f4_layout(root: &Path, out: &Path) -> Value {
    let directory = out.join("f4_fp");
    fs::create_dir_all(&directory).expect("create layout-preserving F4 directory");
    let mut copied = Vec::new();
    for (relative, filename) in [
        ("src/cryptanalysis/f4_fp.rs", "mod.rs"),
        ("src/cryptanalysis/f4_fp/reuse.rs", "reuse.rs"),
        (
            "src/cryptanalysis/f4_fp/pair_certificate.rs",
            "pair_certificate.rs",
        ),
        ("src/cryptanalysis/f4_fp/certified.rs", "certified.rs"),
    ] {
        let bytes = read(root, relative);
        let path = directory.join(filename);
        fs::write(&path, &bytes).expect("copy complete F4 source file");
        assert_eq!(
            fs::read(&path).expect("read back complete F4 source copy"),
            bytes,
            "F4 layout copy must equal its complete production source"
        );
        copied.push(json!({
            "path": relative,
            "selection": "whole_file_verbatim_copy",
            "generated_relative_path": format!("f4_fp/{filename}"),
            "source_and_copy_sha256": digest(&bytes),
            "bytes": bytes.len(),
            "rewritten": false
        }));
    }
    let module_path = directory.join("mod.rs");
    let binding = format!(
        "#[path = {:?}]\npub mod f4_fp;\n",
        module_path.to_str().expect("UTF-8 F4 module path")
    );
    fs::write(out.join("f4_binding.rs"), &binding).expect("write F4 module binding");
    json!({
        "reason": "preserve implicit child resolution of unchanged full F4 source",
        "copied_files": copied,
        "generated_binding_sha256": digest(binding.as_bytes()),
        "generated_binding_contains_algorithm": false
    })
}

fn rust_sources(root: &Path, directory: &Path, files: &mut Vec<String>) {
    println!("cargo:rerun-if-changed={}", directory.display());
    let mut entries: Vec<_> = fs::read_dir(directory)
        .expect("read path dependency source directory")
        .map(|entry| entry.expect("read source directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rust_sources(root, &path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(
                path.strip_prefix(root)
                    .expect("path dependency must be in repository")
                    .to_str()
                    .expect("UTF-8 source path")
                    .to_string(),
            );
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .join("../../../..")
        .canonicalize()
        .expect("qualifier must stay inside its repository");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let f4_layout = f4_layout(&root, &out);
    let extracted = vec![
        extract(
            &root,
            &out,
            "src/cryptanalysis/challenge_ladder.rs",
            "5dd3022fc9fe214b3d8775c55e5b04e5fa306b9442604f99ddd643d2fef7ccf2",
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
            "src/cryptanalysis/glv_invariant_base.rs",
            "32c38ce6d7207030b50696b36e109fc7acebda7ba3ee518c7083e3fb08722336",
            &[
                Range {
                    start: "/// `a·b mod m` without overflow, `m < 2^63`.\n",
                    end: "/// The multiplicative order of `lambda` modulo the prime `r`, by\n",
                    scope: "mulm, addm, subm, and negm",
                },
                Range {
                    start: "/// Coefficients low to high, no trailing zeros.\n",
                    end: "// ── Degree-2 CM endomorphisms by Vélu (type C)",
                    scope: "complete polynomial helpers and exact rational-root extraction, including the actual deadline-aware randomized split",
                },
            ],
            "glv_roots.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/groebner_f4.rs",
            "36b0bc3232df109b08feb246b0ff97d132397470a6897c84ac63500d5005974f",
            &[Range {
                start: "// ── Monomial ordering",
                end: "/// Leading monomial of `f` under `ord`, as `(exponent, coefficient)`.\n",
                scope: "actual Ordering enum and cmp_monomial",
            }],
            "monomial_order.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/ic_boundary.rs",
            // Reviewed in scaling-b677/payload-integration/SOURCE_REVIEW.md.
            // Only two cfg(test) helpers changed inside the selected ranges.
            "7a7ff906dd2206b236a31155f0bdaa2d753b890b2edd79a67aaeb5cb52d72311",
            &[
                Range {
                    start: "/// The field operations behind the group operations: modular\n",
                    end: "/// Nanoseconds per native unit, measured on the host for the instance at\n",
                    scope: "complete FieldOps and GroupOps types and implementations",
                },
                Range {
                    start: "// ── A fast hasher for point keys",
                    end: "fn mix(v: u64) -> u64 {",
                    scope: "actual map hasher, all word modular arithmetic, RowStatus, IncrementalGauss, SiblingPair, and complete CountedGroup trait",
                },
                Range {
                    start: "/// `a` when `first`, else `b`, as one `cmov` where the target has one.  A\n",
                    end: "#[inline]\nfn negmod(a: u64, m: u64) -> u64 {",
                    scope: "actual pick_word architecture dispatch",
                },
                Range {
                    start: "pub struct CubicClasses {",
                    end: "impl RhoClasses<PrimeCurve> for CubicClasses {",
                    scope: "CubicClasses type and complete inherent implementation required by PrimeCurve's automorphism method; no rho class implementation",
                },
                Range {
                    start: "// ── Prime-field curves on one word",
                    end: "/// A prime-field instance: curve, prime-order subgroup, generator.\n",
                    scope: "complete PrimePoint, PrimeCurve, CountedFp, Jacobi helper, and full CountedGroup implementation including native walk overrides",
                },
                Range {
                    start: "#[inline(always)]\n#[allow(unused_unsafe)]\nfn prefetch_cache_line<T>(lines: &[T], index: usize) {\n",
                    end: "struct GuardLineSet {\n",
                    scope: "complete unchanged architecture-specific prefetch helper used by OpenCoefMap",
                },
            ],
            "ic_boundary.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/pq_groebner_f2.rs",
            "2d014644853e7815a58fb34c29177dd4894b08588d59ad64dffa904bffd35056",
            &[Range {
                start: "/// The bitmask a monomial is stored in: 64 bits, or 128 under the\n",
                end: "/// The one-bit mask of variable `k`.\n",
                scope: "actual Mask aliases and MASK_BITS for the unmodified fx_hash module; no Boolean solver",
            }],
            "mask_type.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/prime_fast.rs",
            "03397cbcd5146df5341eea15febe34912bfed346d25759343a33a7b123712731",
            &[
                Range {
                    start: "/// Largest field size the Montgomery layer supports (`p < 2^62` keeps\n",
                    end: "    /// Adopt a crate [`CurveParams`] whose `p` and `n` fit in 62 bits.\n",
                    scope: "complete actual Montgomery arithmetic, points, scratch types, and FastCurve constructors; existing impl opening retained",
                },
                Range {
                    start: "    /// Group-order field context (Montgomery mod `n`) for the linear\n",
                    end: "    pub fn to_textbook(&self, p: Option<Pt>) -> Point {",
                    scope: "actual scalar_field, point, and canonical methods; unused from_params adapter omitted",
                },
                Range {
                    start: "    pub fn is_on_curve(&self, p: Pt) -> bool {\n",
                    end: "fn biguint_to_u64(v: &BigUint) -> Option<u64> {",
                    scope: "all remaining FastCurve arithmetic methods including batched additions and existing impl closing brace; unused textbook conversion adapters omitted",
                },
            ],
            "prime_fast.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/residual_walk.rs",
            "4e54e030cebfc873f9288e25c83153841724d1cc0d8c79417288aa9e74f507c9",
            &[Range {
                start: "// ── u64 modular arithmetic",
                end: "fn isqrt(n: u64) -> u64 {",
                scope: "actual modular arithmetic, pow_mod/inv_mod with unchanged counting hooks, square root, and residual_walk primality test",
            }],
            "residual_arithmetic.rs",
        ),
        extract(
            &root,
            &out,
            "src/cryptanalysis/semaev_prime_poly.rs",
            // The selected Fp implementation is byte-identical; changes are
            // outside this range (SOURCE_REVIEW.md records both digests).
            "e77ebb1576469b24e399cf2cd62059b80607eb65eeaa486847782e6f11714f9b",
            &[Range {
                start: "/// `F_p` arithmetic for `p < 2^62`.\n",
                end: "/// The curve `y² = x³ + ax + b` over `F_p`, coefficients reduced.\n",
                scope: "complete production Fp type and inherent arithmetic implementation",
            }],
            "semaev_fp.rs",
        ),
        extract(
            &root,
            &out,
            "src/prime_hyperelliptic/ops.rs",
            "668c2ea7804578fe09b8b3dc2dd905b1564be5ebbf0f5e6571d785fa7ad842f4",
            &[
                Range {
                    start: "use std::cell::Cell;\n",
                    end: "/// Words a hash of a divisor absorbs: one per 64-bit limb of each\n",
                    scope: "actual counter types, tally operations, sort cost, and exponentiation charging used by jv_bitops",
                },
                Range {
                    start: "/// Everything this thread has tallied, by phase.\n",
                    end: "#[cfg(test)]\nmod tests {",
                    scope: "remaining actual tally snapshots and phase guards; unused divisor_hash_words and legacy tests omitted",
                },
            ],
            "hyperelliptic_ops.rs",
        ),
    ];

    let mut full: Vec<_> = [
        (
            "src/cryptanalysis/identity_certificate.rs",
            "complete actual evaluated-identity issuer and checker; dependency unit tests remain disabled",
        ),
        (
            "src/hash/sha256.rs",
            "complete actual SHA-256 used by identity certificates; dependency unit tests remain disabled",
        ),
        (
            "src/cryptanalysis/f4_fp.rs",
            "complete actual F4 solver source",
        ),
        (
            "src/cryptanalysis/f4_fp/reuse.rs",
            "complete actual F4 trace child",
        ),
        (
            "src/cryptanalysis/f4_fp/pair_certificate.rs",
            "complete actual F4 pair-certificate child",
        ),
        (
            "src/cryptanalysis/f4_fp/certified.rs",
            "complete actual certified-relation adapter child",
        ),
        ("src/cryptanalysis/fx_hash.rs", "complete actual F4 hasher"),
        (
            "src/cryptanalysis/jv_bitops.rs",
            "complete actual counter hooks",
        ),
        (
            "src/cryptanalysis/jv_bitops_uniform.rs",
            "complete actual counter uniform-sampling child",
        ),
        (
            "src/cryptanalysis/bitops.rs",
            "complete actual pricing definitions used by the counters",
        ),
        (
            "src/cryptanalysis/ic_measurement.rs",
            "complete actual measurement hooks; no substitute dump function",
        ),
        (
            "examples/prime31_rr5_quartic.rs",
            "actual public RR5 example bound directly by the Cargo example target",
        ),
        (
            "research/prime31_four_summand_20261122/native/Cargo.toml",
            "qualification package metadata",
        ),
        (
            "research/prime31_four_summand_20261122/native/Cargo.lock",
            "resolved package versions",
        ),
        (
            "research/prime31_four_summand_20261122/native/lib.rs",
            "candidate module bindings",
        ),
        (
            "research/prime31_four_summand_20261122/native/build.rs",
            "verbatim selector for the actual production F4 public resource-limit tests",
        ),
        (
            "research/prime31_four_summand_20261122/native/extractor.rs",
            "RR5 candidate source",
        ),
        (
            "research/prime31_four_summand_20261122/native/relation_adapter.rs",
            "signed-RHS consumer",
        ),
        (
            "research/prime31_four_summand_20261122/native/main.rs",
            "qualification entry point",
        ),
        (
            "research/prime31_four_summand_20261122/native/independent_tests.rs",
            "independent integer group-law oracle and extractor-to-adapter tests",
        ),
        (
            "research/prime31_four_summand_20261122/native/b677_fixture.rs",
            "validated original B509 plus Y7 public hybrid base and exact selector certificates",
        ),
        (
            "research/prime31_four_summand_20261122/native/b677_controls.rs",
            "independent B677 fixture, frozen input and forced construction-stop controls",
        ),
        (
            "research/prime31_four_summand_20261122/native/selector_circuit.rs",
            "quadratic quartic-modular membership circuit and independent dense-selector controls",
        ),
        (
            "research/prime31_four_summand_20261122/native/b677_gate.rs",
            "fixed bounded extraction driver with independent signed replay and authentic-RHS adapter",
        ),
        (
            "research/cheap_ecdlp_cold_gate_20261123/src/bin/rr5_receipt_check.rs",
            "unchanged independently authored public-RHS receipt checker, separately compiled as a binary",
        ),
        (
            "research/cheap_ecdlp_cold_gate_20261123/rr5_grevlex_b8_producer_row.json",
            "historical B8 receipt used only by the independent checker's unit tests",
        ),
        (
            "research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv",
            "unchanged exact public Y7 base input included in the compiled fixture",
        ),
        (
            "research/prime31_four_summand_20261122/scaling-b677/PROTOCOL.md",
            "prospective frozen bounds, targets, controls, admission rules and interpretation",
        ),
        (
            "research/prime31_four_summand_20261122/native/identity_certificate.rs",
            "RR5 evaluated-identity certificate entry point using the actual production issuer and checker",
        ),
        (
            "research/prime31_ic_20261115/native/lib.rs",
            "complete unchanged original prime-field arithmetic and solver library",
        ),
        (
            "src/cryptanalysis/f4_fp_tower.rs",
            "complete original arithmetic dependency's included tower-F4 module",
        ),
        (
            "src/cryptanalysis/sig_fp_tower.rs",
            "complete original arithmetic dependency's included signature module",
        ),
        (
            "research/prime31_four_summand_20261122/native/original-native/Cargo.toml",
            "original arithmetic source metadata",
        ),
        (
            "research/prime31_four_summand_20261122/native/f4-source/Cargo.toml",
            "original F4 closure metadata",
        ),
        (
            "research/prime31_four_summand_20261122/native/f4-source/build.rs",
            "reviewed source selector",
        ),
        (
            "research/prime31_four_summand_20261122/native/f4-source/src/lib.rs",
            "closure namespace",
        ),
        (
            "research/prime31_four_summand_20261122/native/f4-source/src/cryptanalysis.rs",
            "original source bindings",
        ),
    ]
    .map(|(relative, role)| inventory(&root, relative, role))
    .into_iter()
    .collect();
    for dependency in ["certified-relations", "relation-replay-checker"] {
        let directory = format!("crates/{dependency}");
        full.push(inventory(
            &root,
            &format!("{directory}/Cargo.toml"),
            "actual path dependency manifest",
        ));
        let mut sources = Vec::new();
        rust_sources(&root, &root.join(&directory).join("src"), &mut sources);
        for relative in sources {
            full.push(inventory(
                &root,
                &relative,
                "actual path dependency Rust source tree; test-only items remain disabled",
            ));
        }
    }
    let manifest = json!({
        "schema": "rr5-original-f4-source-manifest/v1",
        "scope": "RR5 native extractor, public example, and identity-certificate target against the declared exact production-source closure",
        "evidence_status": "build provenance only; test results require a separate receipt; PCC-unverified",
        "extracted_production_ranges": extracted,
        "layout_preserving_f4_copy": f4_layout,
        "whole_files": full,
        "full_root_library_compiled": false,
        "full_root_library_tests_included": false,
        "legacy_module_unit_tests_included": false,
        "f4_solver_included": true,
        "integration_example_included": true,
        "integration_example_rewritten": false,
        "candidate_source_included": true,
        "algorithm_substitutions": false,
        "counter_stubs": false,
        "complete_hermetic_build_manifest": false,
        "omitted_from_manifest": [
            "registry package bodies (resolved checksums are in Cargo.lock)",
            "rustc, linker, system libraries, CPU state, and environment flags",
            "test-only fixture resources of path dependencies (their unit tests are not built)"
        ],
        "correctness_qualification_only": true,
        "benchmark_executed_by_build_script": false,
        "cargo_profile": env::var("PROFILE").ok(),
        "opt_level": env::var("OPT_LEVEL").ok(),
        "target": env::var("TARGET").ok(),
        "wide_vars_feature": env::var_os("CARGO_FEATURE_WIDE_VARS").is_some()
    });
    fs::write(
        out.join("source-manifest.json"),
        serde_json::to_vec_pretty(&manifest).expect("serialize integration source manifest"),
    )
    .expect("write integration source manifest");
}
