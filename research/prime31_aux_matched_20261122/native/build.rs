//! Select pinned, exact production bytes. No algorithm is replaced or rewritten.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const UPSTREAM_REVISION: &str = "33b3bc3c7485df0fe80aae41e2093cf940d87ff3";
const UPSTREAM_MANIFEST: &str =
    "research/prime31_aux_matched_20261122/sparse-h2-followup/upstream/manifest.json";
const SPECIALIZED_MANIFEST: &str =
    "research/prime31_aux_matched_20261122/fiber-domain-followup/upstream/manifest.json";

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

/// Verify every upstream snapshot and every unchanged dependency against the
/// separately retained exact-revision manifest before selecting any source.
fn upstream_sources(root: &Path) -> Value {
    let bytes = read(root, UPSTREAM_MANIFEST);
    let manifest: Value = serde_json::from_slice(&bytes).expect("valid upstream source manifest");
    assert_eq!(manifest["schema"], "prime31-isogeny-upstream-source/v2");
    assert_eq!(manifest["upstream_revision"], UPSTREAM_REVISION);
    let snapshots = manifest["snapshots"]
        .as_array()
        .expect("upstream snapshots");
    let unchanged = manifest["unchanged_dependencies"]
        .as_array()
        .expect("unchanged upstream dependencies");
    assert_eq!(snapshots.len(), 6);
    assert_eq!(snapshots.len() + unchanged.len(), 43);
    for source in snapshots.iter().chain(unchanged) {
        let path = source["path"].as_str().expect("upstream source path");
        let expected = source["sha256"].as_str().expect("upstream source SHA-256");
        assert_eq!(
            digest(&read(root, path)),
            expected,
            "upstream bytes changed: {path}"
        );
    }
    manifest
}

/// A separate exact source closure keeps the maintained specialized table's
/// types and arithmetic independent of the historical study's frozen module.
fn specialized_sources(root: &Path) -> Value {
    let bytes = read(root, SPECIALIZED_MANIFEST);
    let manifest: Value = serde_json::from_slice(&bytes).expect("specialized source manifest");
    assert_eq!(
        manifest["schema"],
        "prime31-specialized-incumbent-upstream/v1"
    );
    assert_eq!(manifest["repository"], "isogeny-labs/cryptanalysis");
    assert_eq!(
        manifest["upstream_revision"],
        "457ae1818aaf8134bd833ad9c1b9d57015264510"
    );
    let snapshots = manifest["snapshots"]
        .as_array()
        .expect("specialized snapshots");
    assert_eq!(snapshots.len(), 4);
    for source in snapshots {
        let path = source["path"].as_str().expect("specialized snapshot path");
        let bytes = read(root, path);
        assert_eq!(
            digest(&bytes),
            source["sha256"]
                .as_str()
                .expect("specialized source SHA256"),
            "specialized snapshot changed: {path}"
        );
        assert_eq!(
            bytes.len() as u64,
            source["bytes"]
                .as_u64()
                .expect("specialized source byte count")
        );
        assert_eq!(source["rewritten"], false);
    }
    manifest
}

/// Keep the real F4 parent/child layout when it is attached through #[path].
/// The parent is named mod.rs so its unchanged `pub mod reuse/certified`
/// declarations resolve the original children in the same directory.
fn f4_layout(root: &Path, out: &Path) -> Value {
    let directory = out.join("f4_fp");
    fs::create_dir_all(&directory).expect("create layout-preserving F4 directory");
    let mut copied = Vec::new();
    for (relative, filename) in [
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/f4_fp.rs",
            "mod.rs",
        ),
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
        .join("../../..")
        .canonicalize()
        .expect("qualifier must stay inside its repository");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let upstream_source_provenance = upstream_sources(&root);
    let specialized_source_provenance = specialized_sources(&root);
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
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/groebner_f4.rs",
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
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/ic_boundary.rs",
            "1f94ec22dbf7f865fd309e4fb770478dd555afd3282808143526fa775b81726d",
            &[
                Range {
                    start: "/// The field operations behind the group operations: modular\n",
                    end: "/// Nanoseconds per native unit, measured on the host for the instance at\n",
                    scope: "complete FieldOps and GroupOps types and implementations",
                },
                Range {
                    start: "/// `√(πr/2A)`: expected operations of a generic collision search on a\n",
                    end: "/// The same floor in the unit: `√(π/2A)`.\n",
                    scope: "actual generic collision floor helper used by unmodified A1 reference",
                },
                Range {
                    start: "/// Cost of one phase: what it did (exact), what it took (wall) and what\n",
                    end: "/// One variant of one regime on one instance, priced end to end.\n",
                    scope: "complete PhaseCost, bump_counter, and methods used by FactorBase",
                },
                Range {
                    start: "/// A counted rho run.\n",
                    end: "/// One instance of one regime: the curve, the boundaries, the reference\n",
                    scope: "complete production RhoResult",
                },
                Range {
                    start: "// ── A fast hasher for point keys",
                    end: "    /// A binary curve's sibling pair from ONE inversion `1/(x_P + x_J)`:\n",
                    scope: "actual map hasher, word modular arithmetic, IncrementalGauss, SiblingPair, and CountedGroup prefix; unused malformed upstream x_sibling_pair members omitted as recorded in upstream provenance",
                },
                Range {
                    start: "    /// `[k]P` by double-and-add, counted exactly.\n",
                    end: "fn mix(v: u64) -> u64 {",
                    scope: "actual CountedGroup scalar multiplication methods and original trait closing brace, serial_add_each, and PRIME_ADD_EACH_BLOCK; no substitute sibling members or inserted braces",
                },
                Range {
                    start: "fn mix(v: u64) -> u64 {",
                    end: "// ── The matched reference: a tuned, automorphism-aware counted rho",
                    scope: "unchanged production A1 rho_reference and point hash; not the tuned negation-map reference",
                },
                Range {
                    start: "// ── The matched reference: a tuned, automorphism-aware counted rho",
                    end: "impl RhoClasses<PrimeCurve> for CubicClasses {",
                    scope: "complete production RhoWalk, WalkPolicy, jump sizing, joint multiplication, architecture dispatch, RhoClasses, PointClasses, NegationClasses, and CubicClasses inherent implementation; no cubic class or binary/Frobenius implementation",
                },
                Range {
                    start: "/// The class representative of `p` with its coefficients, and whether\n",
                    end: "/// One lane of [`rho_walk_lanes`]: the state [`TunedWalker::walk`] keeps in\n",
                    scope: "complete production canonicalisation, tuned single-target walker, counters, policy entry points, RhoSetup, ladder verification, and rho_search_from; no batched lane or multi-target walk",
                },
                Range {
                    start: "/// How [`rho_walk_v3`] builds its table.\n",
                    end: "/// One target's share of a [`rho_batch_with`] run.\n",
                    scope: "complete production RhoV3 including v4/v5/v6 configuration, jump and ladder sizing, rho_walk_v3, and required chained_jumps helper; no multi-target batch",
                },
                Range {
                    start: "/// The matched reference for `A = 2`: [`rho_reference_walk`] with\n",
                    end: "/// The native work of a signed-Frobenius run that the unit prices at\n",
                    scope: "complete production rho_reference_negation wrapper for the registered historical counted A=2 reference",
                },
                Range {
                    start: "// ── Prime-field curves on one word",
                    end: "    /// `P + J` in full and the abscissa of `P − J` from ONE inversion: the\n",
                    scope: "complete PrimePoint, PrimeCurve, CountedFp, Jacobi helper, and CountedGroup implementation prefix; unused malformed upstream x_sibling_pair_counted override omitted as recorded in upstream provenance",
                },
                Range {
                    start: "    fn neg(&self, p: PrimePoint) -> PrimePoint {\n",
                    end: "/// A prime-field instance: curve, prime-order subgroup, generator.\n",
                    scope: "actual remaining PrimeCurve CountedGroup methods including native walk overrides and original impl closing brace; no substitute sibling method or inserted brace",
                },
                Range {
                    start: "/// A materialised factor base with its column map.  Both `P` and `−P`\n",
                    end: "/// The prime-field base: the `size` smallest abscissae that are on the\n",
                    scope: "complete FactorBase type, count_index_build, and inherent implementation including from_column_map",
                },
                Range {
                    start: "/// What building a table cost, folded into the factor-base phase of\n",
                    end: "    /// The binary negation-folded table in the scalar builder's exact pair\n",
                    scope: "complete production TableCost, table layouts, prefetch helper, PairTable and generic scalar/batched builders; original impl opening retained",
                },
                Range {
                    start: "    /// The same negation-folded table and emission order as\n",
                    end: "    /// Probe a binary sum returned by [`FastCurve::add_many_lazy`]. Its\n",
                    scope: "complete production generic/prime folded batched table builders, lookup and exact probe; binary-only methods omitted",
                },
                Range {
                    start: "    pub fn calibrate_lookup(&self, samples: u64) -> f64 {\n        let mut rng = StdRng::seed_from_u64(7);\n        let keys: Vec<u64> = (0..1024).map(|_| rng.gen::<u64>() >> 1).collect();\n",
                    end: "/// Largest arity a [`KSumTable`] builds, and the most bits a packed summand\n",
                    scope: "complete production PairTable calibration and original impl closing brace; calibration is not invoked by the study",
                },
                Range {
                    start: "/// Native counters of the oracle, per phase.\n",
                    end: "/// Run one oracle on one target: indices of a decomposition, or `None`.\n",
                    scope: "actual production oracle counters and lift_abscissae signed-witness enumeration",
                },
                Range {
                    start: "/// Meet in the middle on a built pair table: one probe for `m = 2`,\n",
                    end: "/// Meet in the middle on the Frobenius-folded table: the same loop as\n",
                    scope: "complete production scalar pair-table decomposition reference for adapter correctness checks",
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
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/prime_fast.rs",
            "03397cbcd5146df5341eea15febe34912bfed346d25759343a33a7b123712731",
            &[
                Range {
                    start: "/// Largest field size the Montgomery layer supports (`p < 2^62` keeps\n",
                    end: "    /// Adopt a crate [`CurveParams`] whose `p` and `n` fit in 62 bits.\n",
                    scope: "complete actual Montgomery arithmetic, points, scratch types, and FastCurve::new; existing impl opening retained",
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
                Range {
                    start: "// ── Parallel distinguished-point rho",
                    end: "// ── Index calculus: factor base, pair table, relations, linear algebra",
                    scope: "complete production prime_fast RhoConfig, RhoResult, operation counts, collision verification, rho_parallel, and worker/lane implementation; prescribed one-thread tuned wall reference, no index-calculus substitute",
                },
                Range {
                    start: "// ── Index calculus: factor base, pair table, relations, linear algebra",
                    end: "// ── Shared factor-base subproduct tree & lock-step multi-walker IC",
                    scope: "complete production FastFactorBase, PairTable, relation collectors and linear algebra, collect_setup_batched, ic_solve_fast, ic_multi_solve_ex, and ic_solve_table; exact complete cold single-target incumbent, without the later subproduct-tree/lockstep experimental path",
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
            "src/cryptanalysis/symmetrized_semaev.rs",
            "18fd7eb54be7d5def01cb165234ac235d02db94b0b3c39921c8101767cc3d632",
            &[Range {
                start: "#[inline]\nfn add_u64(a: u64, b: u64, p: u64) -> u64 {",
                end: "/// Exact evaluation of `S_4(x1, x2, x3, x4) mod p` via the universal 4×4 Sylvester identity.\n",
                scope: "actual scalar helpers, s3_quad_coeffs_u64, and eval_s3_u64; no symbolic polynomial substitute",
            }],
            "numeric_s3.rs",
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
        extract(
            &root, &out, "research/prime31_aux_matched_20261122/sparse-h2-followup/upstream/prime_tower.rs",
            "d05e73e5420f1c60a554c725a746d2cd6b47ea9509ac81873b7f72578297d0b2",
            &[
                Range { start: "fn mul(a: u64, b: u64, p: u64) -> u64 {", end: "/// A primitive `2^t`-th root of unity", scope: "unchanged auxiliary modular product" },
                Range { start: "/// SplitMix64: a fixed, version-independent generator", end: "/// The isogeny tower base: every point", scope: "complete production auxiliary search, point/order checks, constructors, cost counters and tower evaluator" },
            ], "prime_tower_helpers.rs",
        ),
        extract(
            &root, &out, "src/cryptanalysis/ic_framework/pkm_semaev.rs",
            "089be301e76b94a93de29461b6ee50d475ea59e4fa2cc3482cc10940a3432d1c",
            &[
                Range { start: "fn addm(a: u64, b: u64, p: u64) -> u64 {", end: "fn subm(a: u64, b: u64, p: u64) -> u64 {", scope: "unchanged modular addition used by Dickson constructor" },
                Range { start: "/// Abscissae of a Dickson / `T_2`-torus 2-power tower", end: "/// Factor base carried by a Dickson / `T_2`-torus tower", scope: "complete production Dickson fibre constructor and exact cardinality checks" },
            ], "dickson_helpers.rs",
        ),

    ];

    let mut full: Vec<_> = [
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/f4_fp.rs",
            "complete current-upstream production general F4 snapshot",
        ),
        (
            UPSTREAM_MANIFEST,
            "exact-revision upstream snapshots and unchanged dependency provenance",
        ),
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/manifest.json",
            "unchanged historical parent source manifest",
        ),
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/fixtures/kernel_correctness_fixtures.json",
            "exact kernel fixture included in the kernel_fiber_contract test executable",
        ),
        (
            "src/cryptanalysis/f4_fp/reuse.rs",
            "complete production trace reuse child",
        ),
        (
            "src/cryptanalysis/f4_fp/pair_certificate.rs",
            "complete production pair-certificate child",
        ),
        (
            "src/cryptanalysis/f4_fp/certified.rs",
            "complete production relation certificate child",
        ),
        ("research/prime31_aux_matched_20261122/sparse-h2-followup/upstream/fp_backend.rs", "complete pinned field backend"),
        ("src/cryptanalysis/fx_hash.rs", "complete hasher"),
        ("src/cryptanalysis/identity_certificate.rs", "complete existing native identity issuer and checker; source-bound separately from solver pin"),
        ("src/hash/sha256.rs", "complete existing native SHA256 for identity certificates"),
        ("src/cryptanalysis/jv_bitops.rs", "complete counter hooks"),
        (
            "src/cryptanalysis/jv_bitops_uniform.rs",
            "complete uniform sampler counter child",
        ),
        (
            "src/cryptanalysis/bitops.rs",
            "complete bit-operation definitions",
        ),
        (
            "src/cryptanalysis/ic_measurement.rs",
            "complete instrumentation hooks",
        ),
        (
            "research/prime31_aux_matched_20261122/native/Cargo.toml",
            "study source or build configuration",
        ),
        (
            "research/prime31_aux_matched_20261122/native/build.rs",
            "study source or build configuration",
        ),
    ]
    .map(|(relative, role)| inventory(&root, relative, role))
    .into_iter()
    .collect();
    let study = "research/prime31_aux_matched_20261122/native";
    for directory in ["src", "tests"] {
        let mut sources = Vec::new();
        rust_sources(&root, &root.join(study).join(directory), &mut sources);
        for relative in sources {
            full.push(inventory(
                &root,
                &relative,
                "complete study Rust source tree",
            ));
        }
    }
    let lock_path = format!("{study}/Cargo.lock");
    if root.join(&lock_path).is_file() {
        full.push(inventory(
            &root,
            &lock_path,
            "resolved standalone dependency lockfile",
        ));
    }
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
    full.push(inventory(
        &root,
        SPECIALIZED_MANIFEST,
        "separate specialized incumbent source snapshot and byte-selection declaration",
    ));
    for snapshot in specialized_source_provenance["snapshots"]
        .as_array()
        .unwrap()
    {
        full.push(inventory(
            &root,
            snapshot["path"].as_str().unwrap(),
            "complete pinned cryptanalysis specialized incumbent snapshot",
        ));
    }
    let specialized_crate =
        "research/prime31_aux_matched_20261122/fiber-domain-followup/specialized-native";
    for filename in ["Cargo.toml", "build.rs"] {
        full.push(inventory(
            &root,
            &format!("{specialized_crate}/{filename}"),
            "actual specialized incumbent path dependency build input",
        ));
    }
    let mut specialized_rust = Vec::new();
    rust_sources(
        &root,
        &root.join(specialized_crate).join("src"),
        &mut specialized_rust,
    );
    for relative in specialized_rust {
        full.push(inventory(
            &root,
            &relative,
            "complete specialized incumbent path dependency source tree",
        ));
    }
    let manifest = json!({
        "schema": "prime31-aux-matched-source-manifest/v1",
        "scope": "matched auxiliary study with unchanged production F4 and exact declared source closure",
        "evidence_status": "build provenance only; test results require a separate receipt; PCC-unverified",
        "upstream_source_provenance": upstream_source_provenance,
        "specialized_source_provenance": specialized_source_provenance,
        "extracted_production_ranges": extracted,
        "layout_preserving_f4_copy": f4_layout,
        "whole_files": full,
        "full_root_library_compiled": false,
        "full_root_library_tests_included": false,
        "legacy_module_unit_tests_included": false,
        "f4_solver_included": true,
        "prime_fast_complete_ic_included": true,
        "prime_fast_ic_source_revision": UPSTREAM_REVISION,
        "specialized_cryptanalysis_prime_batch_included": true,
        "integration_example_included": false,
        "integration_example_rewritten": false,
        "algorithm_substitutions": false,
        "counter_stubs": false,
        "complete_hermetic_build_manifest": false,
        "omitted_from_manifest": [
            "registry package bodies (resolved checksums are in Cargo.lock)",
            "rustc, linker, system libraries, CPU state, and environment flags",
            "test-only fixture resources of path dependencies (their unit tests are not built)"
        ],
        "build_is_not_execution_evidence": true,
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
