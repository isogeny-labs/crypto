//! Deterministic named-subgroup transfer evidence for every odd-characteristic
//! cover-map row in the exact EC1/ICV1 registry graph.
//!
//! This corpus is deliberately separate from `cover-links.yaml`: historical
//! `subgroup_transport` fields remain `not_tested`.  Direct short-Weierstrass
//! prime-field rows inside the checked primality domain receive typed
//! degree-two pullback/pushforward witnesses.  Wide-prime, normalized-target,
//! extension-field, and invalid-data rows remain present with distinct
//! fail-closed dispositions.

#[path = "curve_cover_check/checker.rs"]
mod checker;
#[path = "curve_cover_check/links.rs"]
mod links;
#[path = "curve_cover_check/models.rs"]
mod models;

use clap::{Parser, Subcommand};
use crypto_lib::build_provenance;
use crypto_lib::prime_hyperelliptic::{
    prime_proof_evidence, prime_proof_status, BalancedDivisorP, PrimeEllipticPointP,
    PrimeProofStatus, PrimeQuadraticCoverP, PrimeSubgroupTransferStatus,
};
use num_bigint::BigUint;
use num_traits::Zero;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};

const CORPUS_SCHEMA: &str = "odd-subgroup-transfer-certificates/v1";
const VERIFICATION_SCHEMA: &str = "odd-subgroup-transfer-catalog-verification/v1";
const SOURCE_VERIFICATION_SCHEMA: &str = "odd-subgroup-transfer-source-verification/v1";
const MAX_INPUT_BYTES: u64 = 128 * 1024 * 1024;
const SOURCE_DIGEST_ENCODING: &str = "for each listed UTF-8 source in order: u64be(len(path_bytes))||path_bytes||u64be(len(content_bytes))||content_bytes";
const PROOF_ARTIFACT_ID: &str = "lean:ChainedTransfer.injective_with_recovery";
const PROOF_ARTIFACT_PATH: &str =
    "research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean";
const PROOF_DECLARATION: &str = "ChainedTransfer.injective_with_recovery";
const PROOF_LEAN_COMMIT: &str = "eb6cafb7290e8bd5f2937522d8d804f58f02bafc";

#[derive(Parser)]
#[command(
    name = "odd-transfer-catalog",
    about = "Export or replay bounded odd-characteristic named-subgroup transfer evidence",
    long_about = "Retains every odd-characteristic cover and named EC1 binding, but certifies transfer only for direct short-Weierstrass prime-field rows inside the deterministic proof domain. This is not a DLP solver or a speed claim."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Rebuild the retained odd-characteristic transfer corpus.
    Export {
        #[arg(long)]
        registry: PathBuf,
        #[arg(long)]
        cover_graph: PathBuf,
        /// Forty lowercase hexadecimal characters naming the producer commit.
        #[arg(long)]
        producer_commit: String,
        /// Exact compiler version used for this executable.
        #[arg(long)]
        rustc_version: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Regenerate the corpus and require byte-for-byte equality.
    Verify {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        registry: PathBuf,
        #[arg(long)]
        cover_graph: PathBuf,
        #[arg(long)]
        expected_sha256: Option<String>,
        #[arg(long)]
        expected_bytes: Option<u64>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Bind the saved producer manifest to the exact recorded Git tree.
    VerifySource {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        repo_root: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn read_limited(path: &PathBuf) -> Result<Vec<u8>, String> {
    let metadata =
        fs::metadata(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    if metadata.len() > MAX_INPUT_BYTES {
        return Err(format!(
            "{} exceeds the {} byte input limit",
            path.display(),
            MAX_INPUT_BYTES
        ));
    }
    fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))
}

fn parse_unsigned(value: &str) -> Result<BigUint, String> {
    let (digits, radix) = value
        .strip_prefix("0x")
        .map_or((value, 10), |digits| (digits, 16));
    if digits.is_empty()
        || (digits.len() > 1 && digits.starts_with('0'))
        || !digits.bytes().all(|byte| match radix {
            10 => byte.is_ascii_digit(),
            16 => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
            _ => false,
        })
    {
        return Err(format!("noncanonical unsigned integer `{value}`"));
    }
    BigUint::parse_bytes(digits.as_bytes(), radix)
        .ok_or_else(|| format!("invalid unsigned integer `{value}`"))
}

fn framed_source_digest(sources: &[(&str, &str)]) -> String {
    let mut framed = Vec::new();
    for (path, contents) in sources {
        framed.extend_from_slice(&(path.len() as u64).to_be_bytes());
        framed.extend_from_slice(path.as_bytes());
        framed.extend_from_slice(&(contents.len() as u64).to_be_bytes());
        framed.extend_from_slice(contents.as_bytes());
    }
    checker::digest(&framed)
}

fn producer_sources() -> Vec<(&'static str, &'static str)> {
    vec![
        ("src/lib.rs", include_str!("../lib.rs")),
        (
            "src/build_provenance.rs",
            include_str!("../build_provenance.rs"),
        ),
        ("src/ct_bignum.rs", include_str!("../ct_bignum.rs")),
        (
            "src/bin/curve_cover_check.rs",
            include_str!("curve_cover_check.rs"),
        ),
        (
            "src/bin/curve_cover_check/checker.rs",
            include_str!("curve_cover_check/checker.rs"),
        ),
        (
            "src/bin/curve_cover_check/links.rs",
            include_str!("curve_cover_check/links.rs"),
        ),
        (
            "src/bin/curve_cover_check/models.rs",
            include_str!("curve_cover_check/models.rs"),
        ),
        (
            "src/bin/odd_transfer_catalog.rs",
            include_str!("odd_transfer_catalog.rs"),
        ),
        (
            "src/binary_ecc/mod.rs",
            include_str!("../binary_ecc/mod.rs"),
        ),
        (
            "src/binary_ecc/f2m.rs",
            include_str!("../binary_ecc/f2m.rs"),
        ),
        (
            "src/binary_ecc/operation_tally.rs",
            include_str!("../binary_ecc/operation_tally.rs"),
        ),
        (
            "src/cryptanalysis/mod.rs",
            include_str!("../cryptanalysis/mod.rs"),
        ),
        (
            "src/cryptanalysis/ecc2k130_guard.rs",
            include_str!("../cryptanalysis/ecc2k130_guard.rs"),
        ),
        (
            "src/cryptanalysis/bitops.rs",
            include_str!("../cryptanalysis/bitops.rs"),
        ),
        ("src/hash/mod.rs", include_str!("../hash/mod.rs")),
        ("src/hash/sha256.rs", include_str!("../hash/sha256.rs")),
        ("src/utils/mod.rs", include_str!("../utils/mod.rs")),
        (
            "src/prime_hyperelliptic/balanced.rs",
            include_str!("../prime_hyperelliptic/balanced.rs"),
        ),
        (
            "src/prime_hyperelliptic/cover_transfer.rs",
            include_str!("../prime_hyperelliptic/cover_transfer.rs"),
        ),
        (
            "src/prime_hyperelliptic/model_transfer.rs",
            include_str!("../prime_hyperelliptic/model_transfer.rs"),
        ),
        (
            "src/prime_hyperelliptic/normalized_cover_transfer.rs",
            include_str!("../prime_hyperelliptic/normalized_cover_transfer.rs"),
        ),
        (
            "src/prime_hyperelliptic/fp2.rs",
            include_str!("../prime_hyperelliptic/fp2.rs"),
        ),
        (
            "src/prime_hyperelliptic/fp_poly.rs",
            include_str!("../prime_hyperelliptic/fp_poly.rs"),
        ),
        (
            "src/prime_hyperelliptic/mod.rs",
            include_str!("../prime_hyperelliptic/mod.rs"),
        ),
        (
            "src/prime_hyperelliptic/ops.rs",
            include_str!("../prime_hyperelliptic/ops.rs"),
        ),
        (
            "src/prime_hyperelliptic/primality.rs",
            include_str!("../prime_hyperelliptic/primality.rs"),
        ),
        (
            "docs/curves/odd-subgroup-transfer-certificates.schema.json",
            include_str!("../../docs/curves/odd-subgroup-transfer-certificates.schema.json"),
        ),
        ("Cargo.toml", include_str!("../../Cargo.toml")),
        ("Cargo.lock", include_str!("../../Cargo.lock")),
    ]
}

fn producer_source_digest() -> String {
    framed_source_digest(&producer_sources())
}

fn producer_source_paths() -> Vec<&'static str> {
    producer_sources().iter().map(|(path, _)| *path).collect()
}

fn producer_source_files() -> Vec<Value> {
    producer_sources()
        .iter()
        .map(|(path, contents)| {
            json!({
                "path": path,
                "bytes": contents.len(),
                "sha256": checker::digest(contents.as_bytes()),
            })
        })
        .collect()
}

fn proof_artifacts() -> Vec<Value> {
    let contents =
        include_str!("../../research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean");
    vec![json!({
        "id": PROOF_ARTIFACT_ID,
        "path": PROOF_ARTIFACT_PATH,
        "sha256": checker::digest(contents.as_bytes()),
        "bytes": contents.len(),
        "declaration": PROOF_DECLARATION,
        "toolchain": {
            "lean": "4.35.0",
            "lean_commit": PROOF_LEAN_COMMIT,
        },
        "verification_status": "local_unsigned_unverified",
        "scope": "conditional structural injectivity from explicit homomorphism, annihilator, Bezout, and norm hypotheses; it does not construct or identify the concrete geometric maps",
    })]
}

fn validate_git_commit(value: &str) -> Result<(), String> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err("producer commit must be exactly forty lowercase hexadecimal digits".into());
    }
    Ok(())
}

fn validate_export_build_provenance(producer_commit: &str) -> Result<&'static str, String> {
    validate_git_commit(producer_commit)?;
    let embedded = build_provenance::git_commit().ok_or_else(|| {
        "export requires a build with CRYPTO_BUILD_GIT_COMMIT (or GITHUB_SHA) embedded".to_string()
    })?;
    validate_git_commit(embedded)
        .map_err(|_| "embedded build commit is not a canonical Git commit".to_string())?;
    if embedded != producer_commit {
        return Err(format!(
            "requested producer commit {producer_commit} differs from embedded build commit {embedded}"
        ));
    }
    if build_provenance::git_dirty() != Some(false) {
        return Err("export requires embedded CRYPTO_BUILD_GIT_DIRTY=false provenance".into());
    }
    build_provenance::source()
        .ok_or_else(|| "export build provenance source is unavailable".to_string())
}

fn validate_build_provenance_source(value: &str) -> Result<(), String> {
    if matches!(value, "CRYPTO_BUILD_GIT_COMMIT" | "GITHUB_SHA") {
        Ok(())
    } else {
        Err(format!(
            "unsupported build provenance source `{value}`; expected CRYPTO_BUILD_GIT_COMMIT or GITHUB_SHA"
        ))
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn producer_command_argv(producer_commit: &str, rustc_version: &str) -> Vec<String> {
    vec![
        "odd-transfer-catalog".into(),
        "export".into(),
        "--registry".into(),
        "docs/curves/registry.json".into(),
        "--cover-graph".into(),
        "docs/curves/cover-links.yaml".into(),
        "--producer-commit".into(),
        producer_commit.into(),
        "--rustc-version".into(),
        rustc_version.into(),
        "--output".into(),
        "docs/curves/odd-subgroup-transfer-certificates.json".into(),
    ]
}

fn producer_command(producer_commit: &str, rustc_version: &str) -> String {
    producer_command_argv(producer_commit, rustc_version)
        .iter()
        .map(|argument| shell_quote(argument))
        .collect::<Vec<_>>()
        .join(" ")
}

fn git_output(repo_root: &Path, arguments: &[String]) -> Result<Vec<u8>, String> {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output()
        .map_err(|error| format!("execute git in {}: {error}", repo_root.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed in {}: {}",
            arguments.join(" "),
            repo_root.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output.stdout)
}

fn verify_source_tree(corpus_bytes: &[u8], repo_root: &Path) -> Result<Value, String> {
    let corpus: Value = serde_json::from_slice(corpus_bytes)
        .map_err(|error| format!("parse certificate corpus: {error}"))?;
    if corpus["schema_version"] != CORPUS_SCHEMA {
        return Err("unsupported certificate corpus schema".into());
    }
    let commit = corpus["producer"]["git_commit"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks git_commit".to_string())?;
    validate_git_commit(commit)?;
    let commit_object = format!("{commit}^{{commit}}");
    git_output(repo_root, &["cat-file".into(), "-e".into(), commit_object])?;

    let expected_paths = producer_source_paths();
    if corpus["producer"]["source_paths"] != json!(expected_paths) {
        return Err("saved producer source path list differs from this verifier".into());
    }
    if corpus["producer"]["source_digest_encoding"] != SOURCE_DIGEST_ENCODING {
        return Err("saved producer source framing rule differs from this verifier".into());
    }
    let saved_files = corpus["producer"]["source_files"]
        .as_array()
        .ok_or_else(|| "certificate corpus producer lacks source_files".to_string())?;
    if saved_files.len() != expected_paths.len() {
        return Err("saved producer source manifest cardinality mismatch".into());
    }
    let mut git_sources = Vec::with_capacity(expected_paths.len());
    for (index, path) in expected_paths.iter().enumerate() {
        let object = format!("{commit}:{path}");
        let bytes = git_output(repo_root, &["show".into(), object])?;
        let contents =
            String::from_utf8(bytes).map_err(|_| format!("{commit}:{path} is not valid UTF-8"))?;
        if saved_files[index]["path"] != *path
            || saved_files[index]["bytes"] != contents.len()
            || saved_files[index]["sha256"] != checker::digest(contents.as_bytes())
        {
            return Err(format!(
                "saved producer source manifest differs from {commit}:{path}"
            ));
        }
        git_sources.push((*path, contents));
    }
    let framed: Vec<(&str, &str)> = git_sources
        .iter()
        .map(|(path, contents)| (*path, contents.as_str()))
        .collect();
    let source_sha256 = framed_source_digest(&framed);
    if corpus["producer"]["source_sha256"] != source_sha256 {
        return Err("saved producer source digest differs from the recorded Git tree".into());
    }

    let saved_proofs = corpus["proof_artifacts"]
        .as_array()
        .ok_or_else(|| "certificate corpus lacks proof_artifacts".to_string())?;
    let expected_proofs = proof_artifacts();
    let expected_proof = &expected_proofs[0];
    if saved_proofs.len() != 1
        || saved_proofs[0]["id"] != expected_proof["id"]
        || saved_proofs[0]["path"] != expected_proof["path"]
        || saved_proofs[0]["declaration"] != expected_proof["declaration"]
        || saved_proofs[0]["toolchain"] != expected_proof["toolchain"]
        || saved_proofs[0]["verification_status"] != expected_proof["verification_status"]
        || saved_proofs[0]["scope"] != expected_proof["scope"]
    {
        return Err("saved proof artifact identity or toolchain metadata differs".into());
    }
    let proof_bytes = git_output(
        repo_root,
        &["show".into(), format!("{commit}:{PROOF_ARTIFACT_PATH}")],
    )?;
    if saved_proofs[0]["bytes"] != proof_bytes.len()
        || saved_proofs[0]["sha256"] != checker::digest(&proof_bytes)
    {
        return Err("saved proof artifact differs from the recorded Git tree".into());
    }

    Ok(json!({
        "schema_version": SOURCE_VERIFICATION_SCHEMA,
        "status": "source_manifest_bound_to_recorded_git_tree",
        "producer_commit": commit,
        "source_sha256": source_sha256,
        "source_files_checked": expected_paths.len(),
        "proof_artifacts_checked": 1,
        "checks": {
            "commit_object_resolved": true,
            "ordered_source_paths_equal": true,
            "individual_source_hashes_equal": true,
            "framed_source_digest_equal": true,
            "proof_artifact_hash_equal": true,
        },
        "claim_boundary": "this repository-mode check binds the saved source and conditional Lean artifact bytes to the recorded Git tree; it does not provide signed PCC standing, prove concrete geometry, or establish computational advantage",
    }))
}

fn proved(status: PrimeProofStatus) -> bool {
    matches!(
        status,
        PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
    )
}

fn point_json(point: &PrimeEllipticPointP) -> Value {
    match point {
        PrimeEllipticPointP::Infinity => json!({"kind": "infinity"}),
        PrimeEllipticPointP::Affine { x, y } => json!({
            "kind": "affine",
            "x": format!("0x{x:x}"),
            "y": format!("0x{y:x}"),
        }),
    }
}

fn divisor_json(divisor: &BalancedDivisorP) -> Value {
    json!({
        "u": divisor.u.coeffs.iter().map(|value| format!("0x{value:x}")).collect::<Vec<_>>(),
        "v": divisor.v.coeffs.iter().map(|value| format!("0x{value:x}")).collect::<Vec<_>>(),
        "n_plus": divisor.n_plus,
    })
}

fn representation_ids(representation: &Value) -> Result<(&str, &str), String> {
    let ec1 = representation["ec1"]
        .as_str()
        .ok_or_else(|| "odd representation lacks EC1".to_string())?;
    let curve_uid = representation["curve_uid"]
        .as_str()
        .ok_or_else(|| format!("{ec1}: odd representation lacks curve_uid"))?;
    Ok((ec1, curve_uid))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BindingFailure {
    status: &'static str,
    obligation: &'static str,
    reason: String,
}

impl BindingFailure {
    fn invalid_named_subgroup_data(obligation: &'static str, reason: String) -> Self {
        Self {
            status: "failed_invalid_named_subgroup_data",
            obligation,
            reason,
        }
    }

    fn invalid_target_generator(reason: String) -> Self {
        Self {
            status: "failed_invalid_target_generator",
            obligation: "target_generator_membership",
            reason,
        }
    }

    fn composite_field_or_order() -> Self {
        Self::invalid_named_subgroup_data(
            "deterministic_primality",
            "field modulus or declared subgroup order is composite".to_string(),
        )
    }

    fn unsupported_kernel_intersection(reason: String) -> Self {
        Self {
            status: "unsupported_kernel_intersection",
            obligation: "kernel_intersection",
            reason,
        }
    }

    fn transfer_arithmetic(obligation: &'static str, reason: String) -> Self {
        Self {
            status: "failed_transfer_arithmetic",
            obligation,
            reason,
        }
    }
}

fn declared_annihilator(representation: &Value, ec1: &str) -> Result<BigUint, BindingFailure> {
    let annihilator_text = representation["curve"]["subgroup_order"]
        .as_str()
        .ok_or_else(|| {
            BindingFailure::invalid_named_subgroup_data(
                "subgroup_order_encoding",
                format!("{ec1}: representation lacks a decimal subgroup order"),
            )
        })?;
    if annihilator_text.starts_with("0x") {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "subgroup_order_encoding",
            format!("{ec1}: subgroup order must use canonical decimal encoding"),
        ));
    }
    let annihilator = parse_unsigned(annihilator_text).map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data(
            "subgroup_order_encoding",
            format!("{ec1}: invalid subgroup order: {reason}"),
        )
    })?;
    if annihilator.is_zero() {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "subgroup_order_encoding",
            format!("{ec1}: subgroup order is zero"),
        ));
    }
    Ok(annihilator)
}

fn parse_hex_coordinate(
    value: &Value,
    ec1: &str,
    coordinate: &str,
) -> Result<BigUint, BindingFailure> {
    let text = value.as_str().ok_or_else(|| {
        BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator {coordinate} is not a string"
        ))
    })?;
    if !text.starts_with("0x") {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator {coordinate} does not use canonical hexadecimal encoding"
        )));
    }
    parse_unsigned(text).map_err(|reason| {
        BindingFailure::invalid_target_generator(format!(
            "{ec1}: invalid generator {coordinate}: {reason}"
        ))
    })
}

fn parse_model_integer(value: &Value, label: &str) -> Result<BigUint, BindingFailure> {
    parse_unsigned(
        &value
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| value.to_string()),
    )
    .map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data(
            "target_model_binding",
            format!("invalid {label}: {reason}"),
        )
    })
}

fn mod_add(left: &BigUint, right: &BigUint, p: &BigUint) -> BigUint {
    (left + right) % p
}

fn mod_mul(left: &BigUint, right: &BigUint, p: &BigUint) -> BigUint {
    (left * right) % p
}

fn validate_normalized_prime_generator(
    representation: &Value,
    model_value: &Value,
    p: &BigUint,
) -> Result<BigUint, BindingFailure> {
    let (ec1, _) = representation_ids(representation).map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data("exact_identity_join", reason)
    })?;
    if representation["field"]["representation"] != "prime"
        || representation["field"]["element_encoding"] != "hex integer modulo the characteristic"
    {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_representation",
            format!("{ec1}: unsupported normalized-prime generator encoding"),
        ));
    }
    let field_characteristic = parse_model_integer(
        &representation["field"]["characteristic"],
        "representation field characteristic",
    )?;
    if representation["field"]["degree"] != 1 || &field_characteristic != p {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_representation",
            format!("{ec1}: representation field does not match the normalized prime target"),
        ));
    }
    let coordinates = representation["curve"]["generator"]
        .as_array()
        .filter(|coordinates| coordinates.len() == 2)
        .ok_or_else(|| {
            BindingFailure::invalid_target_generator(format!(
                "{ec1}: representation lacks an affine generator pair"
            ))
        })?;
    let x = parse_hex_coordinate(&coordinates[0], ec1, "x")?;
    let y = parse_hex_coordinate(&coordinates[1], ec1, "y")?;
    if &x >= p || &y >= p {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator coordinate is not canonical"
        )));
    }
    let x2 = mod_mul(&x, &x, p);
    let y2 = mod_mul(&y, &y, p);
    let form = model_value["form"].as_str().unwrap_or("");
    let (on_curve, is_identity) = match form {
        "B*y^2=x^3+A*x^2+x" => {
            let a = parse_model_integer(&model_value["A"], "Montgomery A")?;
            let b = parse_model_integer(&model_value["B"], "Montgomery B")?;
            let recorded_a = parse_model_integer(
                &representation["curve"]["coefficients"]["a"],
                "representation Montgomery a",
            )?;
            let recorded_b = parse_model_integer(
                &representation["curve"]["coefficients"]["b"],
                "representation Montgomery b",
            )?;
            if representation["curve"]["model"] != "Montgomery"
                || recorded_a != a
                || recorded_b != b
            {
                return Err(BindingFailure::invalid_named_subgroup_data(
                    "target_model_binding",
                    format!(
                        "{ec1}: representation model or coefficients differ from the linked Montgomery target"
                    ),
                ));
            }
            let lhs = mod_mul(&b, &y2, p);
            let rhs = mod_add(
                &mod_add(&mod_mul(&x2, &x, p), &mod_mul(&a, &x2, p), p),
                &x,
                p,
            );
            (lhs == rhs, false)
        }
        "a*x^2+y^2=1+d*x^2*y^2" => {
            let a = parse_model_integer(&model_value["a"], "twisted-Edwards a")?;
            let d = parse_model_integer(&model_value["d"], "twisted-Edwards d")?;
            let recorded_a = parse_model_integer(
                &representation["curve"]["coefficients"]["a"],
                "representation twisted-Edwards a",
            )?;
            let recorded_d = parse_model_integer(
                &representation["curve"]["coefficients"]["d"],
                "representation twisted-Edwards d",
            )?;
            if representation["curve"]["model"] != "TwistedEdwards"
                || recorded_a != a
                || recorded_d != d
            {
                return Err(BindingFailure::invalid_named_subgroup_data(
                    "target_model_binding",
                    format!(
                        "{ec1}: representation model or coefficients differ from the linked twisted-Edwards target"
                    ),
                ));
            }
            let lhs = mod_add(&mod_mul(&a, &x2, p), &y2, p);
            let rhs = mod_add(
                &BigUint::from(1u32),
                &mod_mul(&d, &mod_mul(&x2, &y2, p), p),
                p,
            );
            (lhs == rhs, x.is_zero() && y == BigUint::from(1u32))
        }
        "x^2+y^2=c^2*(1+d*x^2*y^2)" => {
            let c = parse_model_integer(&model_value["c"], "Edwards c")?;
            let d = parse_model_integer(&model_value["d"], "Edwards d")?;
            let recorded_c = parse_model_integer(
                &representation["curve"]["coefficients"]["c"],
                "representation Edwards c",
            )?;
            let recorded_d = parse_model_integer(
                &representation["curve"]["coefficients"]["d"],
                "representation Edwards d",
            )?;
            if representation["curve"]["model"] != "Edwards" || recorded_c != c || recorded_d != d {
                return Err(BindingFailure::invalid_named_subgroup_data(
                    "target_model_binding",
                    format!(
                        "{ec1}: representation model or coefficients differ from the linked Edwards target"
                    ),
                ));
            }
            let lhs = mod_add(&x2, &y2, p);
            let rhs = mod_mul(
                &mod_mul(&c, &c, p),
                &mod_add(
                    &BigUint::from(1u32),
                    &mod_mul(&d, &mod_mul(&x2, &y2, p), p),
                    p,
                ),
                p,
            );
            (lhs == rhs, x.is_zero() && y == c)
        }
        _ => {
            return Err(BindingFailure::invalid_named_subgroup_data(
                "target_model_binding",
                format!("{ec1}: unsupported normalized target form {form}"),
            ))
        }
    };
    if !on_curve {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator is not on the recorded normalized target curve"
        )));
    }
    if is_identity {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator is the target identity"
        )));
    }
    declared_annihilator(representation, ec1)
}

fn parse_extension_coordinate(
    value: &Value,
    ec1: &str,
    coordinate: &str,
    p: &BigUint,
    degree: usize,
) -> Result<BigUint, BindingFailure> {
    let coefficients = value
        .as_array()
        .filter(|coefficients| coefficients.len() == degree)
        .ok_or_else(|| {
            BindingFailure::invalid_target_generator(format!(
                "{ec1}: generator {coordinate} is not a {degree}-coefficient array"
            ))
        })?;
    let mut packed = BigUint::zero();
    let mut place = BigUint::from(1u32);
    for coefficient in coefficients {
        let encoded = coefficient
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| coefficient.to_string());
        let coefficient = parse_unsigned(&encoded).map_err(|reason| {
            BindingFailure::invalid_target_generator(format!(
                "{ec1}: invalid generator {coordinate} coefficient: {reason}"
            ))
        })?;
        if &coefficient >= p {
            return Err(BindingFailure::invalid_target_generator(format!(
                "{ec1}: generator {coordinate} coefficient is not canonical"
            )));
        }
        packed += &coefficient * &place;
        place *= p;
    }
    Ok(packed)
}

fn validate_extension_generator(
    representation: &Value,
    field: &checker::Field,
    a: &BigUint,
    b: &BigUint,
) -> Result<(BigUint, BigUint), BindingFailure> {
    let (ec1, _) = representation_ids(representation).map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data("exact_identity_join", reason)
    })?;
    let checker::Field::Extension { p, low } = field else {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_binding",
            format!("{ec1}: expected an odd extension field"),
        ));
    };
    if representation["field"]["representation"] != "polynomial"
        || representation["field"]["element_encoding"]
            != "decimal coefficient list [e_0, ..., e_{k-1}] of e_0 + e_1 t + ... + e_{k-1} t^{k-1} modulo the monic modulus"
    {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_representation",
            format!("{ec1}: unsupported extension-field generator encoding"),
        ));
    }
    let coordinates = representation["curve"]["generator"]
        .as_array()
        .filter(|coordinates| coordinates.len() == 2)
        .ok_or_else(|| {
            BindingFailure::invalid_target_generator(format!(
                "{ec1}: representation lacks an affine generator pair"
            ))
        })?;
    let x = parse_extension_coordinate(&coordinates[0], ec1, "x", p, low.len())?;
    let y = parse_extension_coordinate(&coordinates[1], ec1, "y", p, low.len())?;
    let x2 = field.mul(&x, &x);
    let rhs = field.add(&field.add(&field.mul(&x2, &x), &field.mul(a, &x)), b);
    if field.mul(&y, &y) != rhs {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator is not on the recorded extension-field target curve"
        )));
    }
    Ok((p.clone(), declared_annihilator(representation, ec1)?))
}

fn backend_independent_checks(
    annihilator_action_checked: bool,
    exact_order_established: bool,
) -> Value {
    json!({
        "exact_identity_join": true,
        "field_and_model_binding": true,
        "generator_encoding_canonical": true,
        "generator_on_linked_target": true,
        "generator_nonidentity": true,
        "subgroup_order_encoding_canonical_nonzero": true,
        "annihilator_action_checked": annihilator_action_checked,
        "exact_order_established": exact_order_established,
    })
}

fn direct_generator(
    representation: &Value,
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
) -> Result<(PrimeEllipticPointP, BigUint), BindingFailure> {
    let (ec1, _) = representation_ids(representation).map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data("exact_identity_join", reason)
    })?;
    if representation["field"]["representation"] != "prime"
        || representation["field"]["degree"] != 1
        || representation["field"]["element_encoding"] != "hex integer modulo the characteristic"
    {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_representation",
            format!("{ec1}: unsupported direct-prime generator encoding"),
        ));
    }
    let characteristic = parse_unsigned(&representation["field"]["characteristic"].to_string())
        .map_err(|reason| {
            BindingFailure::invalid_named_subgroup_data(
                "target_field_binding",
                format!("{ec1}: invalid field characteristic: {reason}"),
            )
        })?;
    if &characteristic != p {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_field_binding",
            format!("{ec1}: representation characteristic differs from target field"),
        ));
    }
    let coefficients = representation["curve"]["coefficients"]
        .as_array()
        .filter(|coefficients| coefficients.len() == 5)
        .ok_or_else(|| {
            BindingFailure::invalid_named_subgroup_data(
                "target_model_binding",
                format!("{ec1}: representation lacks five Weierstrass coefficients"),
            )
        })?;
    let coefficients = coefficients
        .iter()
        .map(|coefficient| parse_unsigned(&coefficient.to_string()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|reason| {
            BindingFailure::invalid_named_subgroup_data(
                "target_model_binding",
                format!("{ec1}: invalid Weierstrass coefficient: {reason}"),
            )
        })?;
    if representation["curve"]["model"] != "short Weierstrass"
        || coefficients[..3]
            .iter()
            .any(|coefficient| !coefficient.is_zero())
        || &coefficients[3] != a
        || &coefficients[4] != b
    {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "target_model_binding",
            format!("{ec1}: representation curve coefficients differ from the linked target model"),
        ));
    }
    let coordinates = representation["curve"]["generator"]
        .as_array()
        .filter(|coordinates| coordinates.len() == 2)
        .ok_or_else(|| {
            BindingFailure::invalid_target_generator(format!(
                "{ec1}: representation lacks an affine generator pair"
            ))
        })?;
    let x = parse_hex_coordinate(&coordinates[0], ec1, "x")?;
    let y = parse_hex_coordinate(&coordinates[1], ec1, "y")?;
    if &x >= p || &y >= p {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator coordinate is not canonical"
        )));
    }
    let rhs = ((&x * &x % p) * &x + a * &x + b) % p;
    if &y * &y % p != rhs {
        return Err(BindingFailure::invalid_target_generator(format!(
            "{ec1}: generator is not on the recorded target curve"
        )));
    }
    let annihilator = declared_annihilator(representation, ec1)?;
    Ok((PrimeEllipticPointP::Affine { x, y }, annihilator))
}

fn failed_record(representation: &Value, failure: &BindingFailure) -> Result<Value, String> {
    let (ec1, curve_uid) = representation_ids(representation)?;
    Ok(json!({
        "status": failure.status,
        "claim_scope": "named_generated_subgroup_only",
        "ec1": ec1,
        "curve_uid": curve_uid,
        "reason": failure.reason,
        "failures": [{"obligation": failure.obligation, "reason": failure.reason}],
    }))
}

fn unsupported_record(
    representation: &Value,
    status: &str,
    reason: &str,
    p: Option<&BigUint>,
    annihilator: Option<&BigUint>,
    checks: Value,
) -> Result<Value, String> {
    let (ec1, curve_uid) = representation_ids(representation)?;
    Ok(json!({
        "status": status,
        "claim_scope": "named_generated_subgroup_only",
        "ec1": ec1,
        "curve_uid": curve_uid,
        "reason": reason,
        "field_primality": p.map(prime_proof_evidence),
        "annihilator_primality": annihilator.map(prime_proof_evidence),
        "backend_independent_checks": checks,
        "failures": [],
    }))
}

fn supported_direct_record(
    cover: &PrimeQuadraticCoverP,
    representation: &Value,
    generator: &PrimeEllipticPointP,
    annihilator: &BigUint,
) -> Result<Value, BindingFailure> {
    let (ec1, curve_uid) = representation_ids(representation).map_err(|reason| {
        BindingFailure::invalid_named_subgroup_data("exact_identity_join", reason)
    })?;
    if cover
        .checked_target_scalar_mul(generator, annihilator)
        .map_err(|error| {
            BindingFailure::transfer_arithmetic(
                "annihilator_kills_target_generator",
                format!("{ec1}: target scalar multiplication: {error}"),
            )
        })?
        != PrimeEllipticPointP::Infinity
    {
        return Err(BindingFailure::invalid_named_subgroup_data(
            "annihilator_kills_target_generator",
            format!("{ec1}: annihilator does not kill the target generator"),
        ));
    }
    if !annihilator.bit(0) {
        return Err(BindingFailure::unsupported_kernel_intersection(format!(
            "{ec1}: exact-order subgroup is not coprime to map degree two"
        )));
    }
    let composition = cover
        .pull_push_composition_certificate(generator)
        .map_err(|error| {
            BindingFailure::transfer_arithmetic(
                "pull_push_composition",
                format!("{ec1}: pull-push certificate: {error}"),
            )
        })?;
    if !composition.identity_holds {
        return Err(BindingFailure::transfer_arithmetic(
            "pull_push_composition",
            format!("{ec1}: pull-push composition identity failed"),
        ));
    }
    if composition.pullback.is_identity() {
        return Err(BindingFailure::transfer_arithmetic(
            "pullback_nonidentity",
            format!("{ec1}: nonidentity generator has identity pullback"),
        ));
    }
    if !composition
        .pullback
        .checked_scalar_mul(annihilator, cover.source_curve())
        .map_err(|error| {
            BindingFailure::transfer_arithmetic(
                "annihilator_kills_pullback",
                format!("{ec1}: pullback scalar multiplication: {error}"),
            )
        })?
        .is_identity()
    {
        return Err(BindingFailure::transfer_arithmetic(
            "annihilator_kills_pullback",
            format!("{ec1}: annihilator does not kill the pullback"),
        ));
    }
    let degree_inverse = match cover
        .subgroup_transfer_status(annihilator)
        .map_err(|error| {
            BindingFailure::transfer_arithmetic(
                "kernel_intersection",
                format!("{ec1}: subgroup classification: {error}"),
            )
        })? {
        PrimeSubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
            degree_inverse_mod_order,
            ..
        } => degree_inverse_mod_order,
        PrimeSubgroupTransferStatus::KernelIntersectionUnresolved { common_factor, .. } => {
            return Err(BindingFailure::unsupported_kernel_intersection(format!(
                "{ec1}: map-degree kernel intersection remains unresolved (gcd={common_factor})"
            )))
        }
    };
    let recovered = cover
        .checked_target_scalar_mul(&composition.pushforward, &degree_inverse)
        .map_err(|error| {
            BindingFailure::transfer_arithmetic(
                "degree_inverse_recovery",
                format!("{ec1}: inverse-degree recovery: {error}"),
            )
        })?;
    if &recovered != generator {
        return Err(BindingFailure::transfer_arithmetic(
            "degree_inverse_recovery",
            format!("{ec1}: inverse-degree recovery did not recover generator"),
        ));
    }
    let branch = match generator {
        PrimeEllipticPointP::Affine { x, .. } if x == cover.shift() => "ramified_repeated_support",
        _ => "ordinary_degree_two_fiber",
    };
    Ok(json!({
        "status": "supported_on_named_generated_subgroup",
        "claim_scope": "named_generated_subgroup_of_exact_prime_order",
        "ec1": ec1,
        "curve_uid": curve_uid,
        "generator": point_json(generator),
        "annihilator": annihilator.to_str_radix(10),
        "map_degree": 2,
        "gcd_degree_annihilator": 1,
        "degree_inverse_mod_annihilator": degree_inverse.to_str_radix(10),
        "fiber_kind": branch,
        "pullback_witness": divisor_json(&composition.pullback),
        "typed_pushforward": point_json(&composition.pushforward),
        "twice_generator": point_json(&composition.twice_input),
        "recovered_generator": point_json(&recovered),
        "field_primality": prime_proof_evidence(cover.modulus()),
        "exact_order_evidence": {
            "annihilator_primality": prime_proof_evidence(&annihilator),
            "generated_subgroup_cardinality": {
                "status": "proved",
                "argument": "prime annihilator plus checked nonidentity and annihilation",
            },
        },
        "checks": {
            "source_cover_reconstructed_and_valid": true,
            "target_generator_on_curve": true,
            "target_generator_nonidentity": true,
            "target_generator_coordinates_canonical": true,
            "annihilator_kills_target_generator": true,
            "annihilator_is_prime_and_exact_order": true,
            "pullback_valid": true,
            "pullback_nonidentity": true,
            "annihilator_kills_pullback": true,
            "pushforward_valid": true,
            "pull_push_equals_degree_times_generator": composition.identity_holds,
            "degree_inverse_recovers_generator": true,
        },
        "proof_reference": PROOF_ARTIFACT_ID,
        "failures": [],
    }))
}

fn graph_shift(map: &Value, slug: &str) -> Result<BigUint, String> {
    let x = map["record"]["x"]
        .as_array()
        .filter(|x| x.len() == 3)
        .ok_or_else(|| format!("{slug}: direct map x polynomial is not quadratic"))?;
    if x[1] != "0x0" || x[2] != "0x1" {
        return Err(format!("{slug}: direct map is not x=u^2+c"));
    }
    x[0].as_str()
        .ok_or_else(|| format!("{slug}: direct map shift is not a string"))
        .and_then(parse_unsigned)
}

fn membership_digest(lines: &BTreeSet<String>) -> String {
    let mut bytes = Vec::new();
    for line in lines {
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'\n');
    }
    checker::digest(&bytes)
}

fn binding_membership_line(map_uid: &str, representation: &Value) -> Result<String, String> {
    let (ec1, curve_uid) = representation_ids(representation)?;
    Ok(format!("{map_uid}\t{ec1}\t{curve_uid}"))
}

fn direct_map_status(
    field_status: PrimeProofStatus,
    named_subgroup_count: usize,
    every_named_subgroup_supported: bool,
    has_invalid_data: bool,
    has_transfer_failures: bool,
) -> &'static str {
    if field_status == PrimeProofStatus::Composite {
        "failed_invalid_map_field_data"
    } else if has_transfer_failures {
        "failed_transfer_arithmetic"
    } else if has_invalid_data {
        "failed_invalid_named_subgroup_data"
    } else if proved(field_status) {
        if named_subgroup_count == 0 {
            "supported_map_no_named_subgroup"
        } else if every_named_subgroup_supported {
            "supported_map_all_named_subgroups"
        } else {
            "supported_map_with_unsupported_named_subgroups"
        }
    } else {
        "unsupported_without_primality_certificate"
    }
}

fn validate_graph_link(
    row: &Value,
    graph_maps: &serde_json::Map<String, Value>,
    graph_covers: &serde_json::Map<String, Value>,
) -> Result<(String, String, String), String> {
    let slug = row["icv1_identity"]["slug"]
        .as_str()
        .ok_or_else(|| "odd graph row lacks an ICV1 slug".to_string())?;
    let model_json = row["model_json"]
        .as_str()
        .ok_or_else(|| format!("{slug}: graph row lacks model_json"))?;
    let model_sha256 = checker::digest(model_json.as_bytes());
    if row["model_sha256"] != model_sha256 {
        return Err(format!("{slug}: model digest mismatch"));
    }
    let link = row["cover_links"]
        .as_array()
        .filter(|links| links.len() == 1)
        .and_then(|links| links.first())
        .ok_or_else(|| format!("{slug}: odd row must have exactly one cover link"))?;
    let cover_uid = link["cover_uid"]
        .as_str()
        .ok_or_else(|| format!("{slug}: cover link lacks cover_uid"))?;
    let map_uid = link["map_uid"]
        .as_str()
        .ok_or_else(|| format!("{slug}: cover link lacks map_uid"))?;
    let map = graph_maps
        .get(map_uid)
        .ok_or_else(|| format!("{slug}: graph lacks linked map {map_uid}"))?;
    if !graph_covers.contains_key(cover_uid)
        || map["record"]["source_cover_uid"] != cover_uid
        || map["record"]["target_model_sha256"] != model_sha256
        || map["record"]["degree"] != 2
    {
        return Err(format!("{slug}: linked cover/map record is inconsistent"));
    }
    Ok((model_sha256, cover_uid.to_owned(), map_uid.to_owned()))
}

fn odd_transfer_catalog(
    registry_bytes: &[u8],
    graph_bytes: &[u8],
    producer_commit: &str,
    rustc_version: &str,
    build_provenance_source: &str,
) -> Result<(Value, bool), String> {
    validate_git_commit(producer_commit)?;
    if rustc_version.trim().is_empty() {
        return Err("rustc_version must not be empty".into());
    }
    validate_build_provenance_source(build_provenance_source)?;
    let registry: Value = serde_json::from_slice(registry_bytes)
        .map_err(|error| format!("parse registry: {error}"))?;
    let geometry = checker::catalog(registry_bytes)?;
    let graph: Value = serde_json::from_slice(graph_bytes)
        .map_err(|error| format!("parse cover graph: {error}"))?;
    if graph["schema_version"] != "curve-cover-graph/v1"
        || graph["registry_sha256"] != geometry["registry_sha256"]
    {
        return Err("registry and cover graph schema/digest do not match".into());
    }
    let expected_graph = links::graph(&registry, &geometry)
        .map_err(|error| format!("rebuild cover graph: {error}"))?;
    if graph != expected_graph {
        return Err("cover graph is not the exact deterministic registry graph".into());
    }
    drop(expected_graph);
    drop(geometry);
    drop(registry);
    let graph_maps = graph["maps"]
        .as_object()
        .ok_or_else(|| "cover graph lacks a map object".to_string())?;
    let graph_covers = graph["covers"]
        .as_object()
        .ok_or_else(|| "cover graph lacks a cover object".to_string())?;
    let rows = graph["curves"]
        .as_array()
        .ok_or_else(|| "cover graph lacks a curves array".to_string())?;

    let mut records = Vec::new();
    let mut map_uids = BTreeSet::new();
    let mut binding_membership = BTreeSet::new();
    let mut supported_membership = BTreeSet::new();
    let mut unsupported_or_failed_direct_membership = BTreeSet::new();
    let mut unsupported_primality_membership = BTreeSet::new();
    let mut unsupported_kernel_membership = BTreeSet::new();
    let mut unsupported_normalized_membership = BTreeSet::new();
    let mut unsupported_extension_membership = BTreeSet::new();
    let mut failed_invalid_membership = BTreeSet::new();
    let mut failed_transfer_membership = BTreeSet::new();
    let mut direct_maps = 0usize;
    let mut normalized_maps = 0usize;
    let mut extension_maps = 0usize;
    let mut maps_with_named_subgroups = 0usize;
    let mut supported = 0usize;
    let mut unsupported_primality = 0usize;
    let mut unsupported_direct_primality = 0usize;
    let mut unsupported_kernel_intersection = 0usize;
    let mut unsupported_normalized = 0usize;
    let mut unsupported_extension = 0usize;
    let mut failed_invalid_target_generator = 0usize;
    let mut failed_invalid_named_subgroup_data = 0usize;
    let mut failed_direct_invalid_data = 0usize;
    let mut failed_transfer_arithmetic = 0usize;
    let mut ramified_supported = 0usize;
    let mut ordinary_supported = 0usize;

    for row in rows {
        let model_json = row["model_json"]
            .as_str()
            .ok_or_else(|| "cover graph row lacks model_json".to_string())?;
        let model_value: Value = serde_json::from_str(model_json)
            .map_err(|error| format!("parse graph model: {error}"))?;
        if model_value["form"] == "y^2+xy=x^3+a*x^2+b" {
            continue;
        }
        let (model_sha256, cover_uid, map_uid) =
            validate_graph_link(row, graph_maps, graph_covers)?;
        let slug = row["icv1_identity"]["slug"]
            .as_str()
            .expect("validated ICV1 slug");
        if !map_uids.insert(map_uid.clone()) {
            return Err(format!("{slug}: duplicate odd map UID {map_uid}"));
        }
        let model = checker::model(&model_value).map_err(|error| match error {
            checker::ModelError::Unsupported(reason) | checker::ModelError::Invalid(reason) => {
                format!("{slug}: {reason}")
            }
        })?;
        let representations = row["representations"]
            .as_array()
            .ok_or_else(|| format!("{slug}: representations is not an array"))?;
        if !representations.is_empty() {
            maps_with_named_subgroups += 1;
        }
        for representation in representations {
            let (ec1, _) = representation_ids(representation)?;
            if representation["subgroup_transport"]["status"] != "not_tested"
                || !representation["subgroup_transport"]["evidence_ref"].is_null()
            {
                return Err(format!(
                    "{ec1}: historical graph subgroup_transport must remain not_tested/null"
                ));
            }
            if !binding_membership.insert(binding_membership_line(&map_uid, representation)?) {
                return Err(format!("duplicate odd binding tuple for {ec1}"));
            }
        }

        let graph_map = &graph_maps[&map_uid];
        let family;
        let map_status;
        let mut named_subgroups = Vec::new();
        let mut failures = Vec::new();
        match &model.field {
            checker::Field::Extension { p, .. } => {
                family = "odd_extension_quadratic_pullback_v1";
                extension_maps += 1;
                let field_status = prime_proof_status(p);
                if field_status == PrimeProofStatus::Composite {
                    failures.push(json!({
                        "obligation": "deterministic_primality",
                        "reason": "field characteristic is composite",
                    }));
                }
                for representation in representations {
                    let membership = binding_membership_line(&map_uid, representation)?;
                    match validate_extension_generator(
                        representation,
                        &model.field,
                        &model.a,
                        &model.b,
                    ) {
                        Ok((p, annihilator)) => {
                            if field_status == PrimeProofStatus::Composite
                                || prime_proof_status(&annihilator) == PrimeProofStatus::Composite
                            {
                                let failure = BindingFailure::composite_field_or_order();
                                failed_invalid_membership.insert(membership);
                                named_subgroups.push(failed_record(representation, &failure)?);
                                failures.push(json!({
                                    "obligation": failure.obligation,
                                    "ec1": representation["ec1"],
                                    "reason": failure.reason,
                                }));
                                failed_invalid_named_subgroup_data += 1;
                            } else {
                                unsupported_extension_membership.insert(membership);
                                named_subgroups.push(unsupported_record(
                                    representation,
                                    "unsupported_odd_extension_field_transfer_backend",
                                    "the catalog-basis-preserving extension-field balanced transfer backend is not implemented",
                                    Some(&p),
                                    Some(&annihilator),
                                    backend_independent_checks(false, false),
                                )?);
                                unsupported_extension += 1;
                            }
                        }
                        Err(failure) => {
                            failed_invalid_membership.insert(membership);
                            named_subgroups.push(failed_record(representation, &failure)?);
                            failures.push(json!({
                                "obligation": failure.obligation,
                                "ec1": representation["ec1"],
                                "reason": failure.reason,
                            }));
                            if failure.status == "failed_invalid_target_generator" {
                                failed_invalid_target_generator += 1;
                            } else {
                                failed_invalid_named_subgroup_data += 1;
                            }
                        }
                    }
                }
                map_status = if field_status == PrimeProofStatus::Composite {
                    "failed_invalid_map_field_data"
                } else if failures.is_empty() {
                    "unsupported_odd_extension_field_transfer_backend"
                } else {
                    "failed_invalid_named_subgroup_data"
                };
            }
            checker::Field::Prime(p) if model.normalization.is_some() => {
                family = "normalized_prime_quadratic_pullback_v1";
                normalized_maps += 1;
                let normalization_kind = model
                    .normalization
                    .as_ref()
                    .and_then(|normalization| normalization["kind"].as_str())
                    .ok_or_else(|| format!("{slug}: normalized target lacks a map kind"))?;
                let target_form = model_value["form"].as_str().unwrap_or("");
                let is_montgomery = match (normalization_kind, target_form) {
                    ("montgomery_from_short", "B*y^2=x^3+A*x^2+x") => true,
                    (
                        "edwards_from_short",
                        "a*x^2+y^2=1+d*x^2*y^2"
                        | "x^2+y^2=c^2*(1+d*x^2*y^2)",
                    ) => false,
                    _ => {
                        return Err(format!(
                            "{slug}: normalization kind {normalization_kind} is inconsistent with target form {target_form}"
                        ))
                    }
                };
                let field_status = prime_proof_status(p);
                if field_status == PrimeProofStatus::Composite {
                    failures.push(json!({
                        "obligation": "deterministic_primality",
                        "reason": "field modulus is composite",
                    }));
                }
                for representation in representations {
                    let membership = binding_membership_line(&map_uid, representation)?;
                    match validate_normalized_prime_generator(representation, &model_value, p) {
                        Ok(annihilator) => {
                            if field_status == PrimeProofStatus::Composite
                                || prime_proof_status(&annihilator) == PrimeProofStatus::Composite
                            {
                                let failure = BindingFailure::composite_field_or_order();
                                failed_invalid_membership.insert(membership);
                                named_subgroups.push(failed_record(representation, &failure)?);
                                failures.push(json!({
                                    "obligation": failure.obligation,
                                    "ec1": representation["ec1"],
                                    "reason": failure.reason,
                                }));
                                failed_invalid_named_subgroup_data += 1;
                            } else if is_montgomery
                                && (!proved(field_status)
                                    || !proved(prime_proof_status(&annihilator)))
                            {
                                unsupported_primality_membership.insert(membership);
                                named_subgroups.push(unsupported_record(
                                    representation,
                                    "unsupported_without_primality_certificate",
                                    "the Montgomery transfer backend is implemented, but the field modulus or annihilator is outside the accepted deterministic proof range",
                                    Some(p),
                                    Some(&annihilator),
                                    backend_independent_checks(false, false),
                                )?);
                                unsupported_primality += 1;
                            } else if is_montgomery {
                                return Err(format!(
                                    "{slug}: an admitted Montgomery field/order binding requires the composed supported-record certificate path"
                                ));
                            } else {
                                unsupported_normalized_membership.insert(membership);
                                named_subgroups.push(unsupported_record(
                                    representation,
                                    "unsupported_normalized_target_transfer_backend",
                                    "the Edwards target still requires an exceptional-point-safe projective transfer adapter",
                                    Some(p),
                                    Some(&annihilator),
                                    backend_independent_checks(false, false),
                                )?);
                                unsupported_normalized += 1;
                            }
                        }
                        Err(failure) => {
                            failed_invalid_membership.insert(membership);
                            named_subgroups.push(failed_record(representation, &failure)?);
                            failures.push(json!({
                                "obligation": failure.obligation,
                                "ec1": representation["ec1"],
                                "reason": failure.reason,
                            }));
                            if failure.status == "failed_invalid_target_generator" {
                                failed_invalid_target_generator += 1;
                            } else {
                                failed_invalid_named_subgroup_data += 1;
                            }
                        }
                    }
                }
                map_status = if field_status == PrimeProofStatus::Composite {
                    "failed_invalid_map_field_data"
                } else if !failures.is_empty() {
                    "failed_invalid_named_subgroup_data"
                } else if is_montgomery {
                    if representations.is_empty() && proved(field_status) {
                        return Err(format!(
                            "{slug}: an admitted Montgomery map requires composed map-certificate integration"
                        ));
                    }
                    "unsupported_without_primality_certificate"
                } else {
                    "unsupported_normalized_target_transfer_backend"
                };
            }
            checker::Field::Prime(p) => {
                family = "prime_quadratic_pullback_v1";
                direct_maps += 1;
                let shift = graph_shift(graph_map, slug)?;
                let field_status = prime_proof_status(p);
                if field_status == PrimeProofStatus::Composite {
                    failures.push(json!({
                        "obligation": "deterministic_primality",
                        "reason": "field modulus is composite",
                    }));
                }
                let runtime_cover = if proved(field_status) {
                    Some(
                        PrimeQuadraticCoverP::try_new(
                            p.clone(),
                            model.a.clone(),
                            model.b.clone(),
                            shift.clone(),
                        )
                        .map_err(|error| format!("{slug}: checked cover construction: {error}"))?,
                    )
                } else {
                    None
                };
                if let Some(cover) = &runtime_cover {
                    cover
                        .validate()
                        .map_err(|error| format!("{slug}: checked cover validation: {error}"))?;
                    let graph_f = graph_covers[&cover_uid]["record"]["f"]
                        .as_array()
                        .ok_or_else(|| format!("{slug}: graph cover lacks source f"))?
                        .iter()
                        .map(|coefficient| {
                            coefficient
                                .as_str()
                                .ok_or_else(|| {
                                    format!("{slug}: source coefficient is not a string")
                                })
                                .and_then(parse_unsigned)
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if graph_f != cover.source_curve().f.coeffs {
                        return Err(format!("{slug}: runtime source differs from graph cover"));
                    }
                }
                for representation in representations {
                    let parsed = direct_generator(representation, p, &model.a, &model.b);
                    let (generator, annihilator) = match parsed {
                        Ok(parsed) => parsed,
                        Err(failure) => {
                            unsupported_or_failed_direct_membership
                                .insert(binding_membership_line(&map_uid, representation)?);
                            failed_invalid_membership
                                .insert(binding_membership_line(&map_uid, representation)?);
                            named_subgroups.push(failed_record(representation, &failure)?);
                            failures.push(json!({
                                "obligation": failure.obligation,
                                "ec1": representation["ec1"],
                                "reason": failure.reason,
                            }));
                            if failure.status == "failed_invalid_target_generator" {
                                failed_invalid_target_generator += 1;
                            } else {
                                failed_invalid_named_subgroup_data += 1;
                            }
                            failed_direct_invalid_data += 1;
                            continue;
                        }
                    };
                    let order_status = prime_proof_status(&annihilator);
                    if field_status == PrimeProofStatus::Composite
                        || order_status == PrimeProofStatus::Composite
                    {
                        let failure = BindingFailure::composite_field_or_order();
                        unsupported_or_failed_direct_membership
                            .insert(binding_membership_line(&map_uid, representation)?);
                        failed_invalid_membership
                            .insert(binding_membership_line(&map_uid, representation)?);
                        named_subgroups.push(failed_record(representation, &failure)?);
                        failures.push(json!({
                            "obligation": failure.obligation,
                            "ec1": representation["ec1"],
                            "reason": failure.reason,
                        }));
                        failed_invalid_named_subgroup_data += 1;
                        failed_direct_invalid_data += 1;
                    } else if !proved(field_status) || !proved(order_status) {
                        unsupported_or_failed_direct_membership
                            .insert(binding_membership_line(&map_uid, representation)?);
                        unsupported_primality_membership
                            .insert(binding_membership_line(&map_uid, representation)?);
                        named_subgroups.push(unsupported_record(
                            representation,
                            "unsupported_without_primality_certificate",
                            "field modulus or annihilator is outside the accepted deterministic proof range",
                            Some(p),
                            Some(&annihilator),
                            backend_independent_checks(false, false),
                        )?);
                        unsupported_primality += 1;
                        unsupported_direct_primality += 1;
                    } else {
                        let membership = binding_membership_line(&map_uid, representation)?;
                        match supported_direct_record(
                            runtime_cover
                                .as_ref()
                                .expect("proved field has checked cover"),
                            representation,
                            &generator,
                            &annihilator,
                        ) {
                            Ok(record) => {
                                supported_membership.insert(membership);
                                if record["fiber_kind"] == "ramified_repeated_support" {
                                    ramified_supported += 1;
                                } else {
                                    ordinary_supported += 1;
                                }
                                supported += 1;
                                named_subgroups.push(record);
                            }
                            Err(failure) if failure.status == "unsupported_kernel_intersection" => {
                                unsupported_or_failed_direct_membership.insert(membership.clone());
                                unsupported_kernel_membership.insert(membership);
                                named_subgroups.push(unsupported_record(
                                    representation,
                                    failure.status,
                                    &failure.reason,
                                    Some(p),
                                    Some(&annihilator),
                                    backend_independent_checks(true, true),
                                )?);
                                unsupported_kernel_intersection += 1;
                            }
                            Err(failure) => {
                                unsupported_or_failed_direct_membership.insert(membership.clone());
                                if failure.status == "failed_transfer_arithmetic" {
                                    failed_transfer_membership.insert(membership);
                                    failed_transfer_arithmetic += 1;
                                } else if failure.status == "failed_invalid_target_generator" {
                                    failed_invalid_membership.insert(membership);
                                    failed_invalid_target_generator += 1;
                                    failed_direct_invalid_data += 1;
                                } else {
                                    failed_invalid_membership.insert(membership);
                                    failed_invalid_named_subgroup_data += 1;
                                    failed_direct_invalid_data += 1;
                                }
                                named_subgroups.push(failed_record(representation, &failure)?);
                                failures.push(json!({
                                    "obligation": failure.obligation,
                                    "ec1": representation["ec1"],
                                    "reason": failure.reason,
                                }));
                            }
                        }
                    }
                }
                let has_invalid_data = named_subgroups.iter().any(|row| {
                    matches!(
                        row["status"].as_str(),
                        Some("failed_invalid_target_generator")
                            | Some("failed_invalid_named_subgroup_data")
                    )
                });
                let has_transfer_failures = named_subgroups
                    .iter()
                    .any(|row| row["status"] == "failed_transfer_arithmetic");
                map_status = direct_map_status(
                    field_status,
                    representations.len(),
                    named_subgroups
                        .iter()
                        .all(|row| row["status"] == "supported_on_named_generated_subgroup"),
                    has_invalid_data,
                    has_transfer_failures,
                );
            }
            checker::Field::Binary(_) => unreachable!("binary form was excluded"),
        }
        records.push(json!({
            "status": map_status,
            "family": family,
            "icv1_identity": row["icv1_identity"],
            "model_sha256": model_sha256,
            "cover_uid": cover_uid,
            "map_uid": map_uid,
            "map_degree": 2,
            "named_subgroups": named_subgroups,
            "failures": failures,
        }));
    }
    let named_subgroup_records = binding_membership.len();
    let failed_invalid_data = failed_invalid_target_generator + failed_invalid_named_subgroup_data;
    let classified = supported
        + unsupported_primality
        + unsupported_kernel_intersection
        + unsupported_normalized
        + unsupported_extension
        + failed_invalid_data
        + failed_transfer_arithmetic;
    if classified != named_subgroup_records {
        return Err(format!(
            "internal binding accounting mismatch: classified {classified}, retained {named_subgroup_records}"
        ));
    }
    if map_uids.len() != records.len() {
        return Err("internal odd map accounting mismatch".into());
    }
    if binding_membership.len() != named_subgroup_records
        || supported_membership.len() != supported
        || unsupported_or_failed_direct_membership.len()
            != unsupported_direct_primality
                + unsupported_kernel_intersection
                + failed_direct_invalid_data
                + failed_transfer_arithmetic
        || unsupported_primality_membership.len() != unsupported_primality
        || unsupported_kernel_membership.len() != unsupported_kernel_intersection
        || unsupported_normalized_membership.len() != unsupported_normalized
        || unsupported_extension_membership.len() != unsupported_extension
        || failed_invalid_membership.len() != failed_invalid_data
        || failed_transfer_membership.len() != failed_transfer_arithmetic
    {
        return Err("internal membership-set accounting mismatch".into());
    }
    let all_named_subgroup_records_supported = unsupported_primality == 0
        && unsupported_kernel_intersection == 0
        && unsupported_normalized == 0
        && unsupported_extension == 0
        && failed_invalid_data == 0
        && failed_transfer_arithmetic == 0;
    Ok((
        json!({
            "schema_version": CORPUS_SCHEMA,
            "generated_by": "odd-transfer-catalog export",
            "producer": {
                "repository": "aburan28/crypto",
                "git_commit": producer_commit,
                "source_sha256": producer_source_digest(),
                "source_digest_encoding": SOURCE_DIGEST_ENCODING,
                "source_paths": producer_source_paths(),
                "source_files": producer_source_files(),
                "rustc_version": rustc_version,
                "rustc_version_attestation": "caller_supplied_unverified",
                "attestation": "embedded_clean_build_provenance_matches_requested_commit",
                "build_provenance_source": build_provenance_source,
                "build_git_dirty": false,
                "command": producer_command(producer_commit, rustc_version),
                "command_argv": producer_command_argv(producer_commit, rustc_version),
            },
            "proof_artifacts": proof_artifacts(),
            "inputs": {
                "registry": {"path": "docs/curves/registry.json", "sha256": checker::digest(registry_bytes)},
                "cover_graph": {"path": "docs/curves/cover-links.yaml", "sha256": checker::digest(graph_bytes)},
            },
            "scope": {
                "supported_family": "direct_short_weierstrass_prime_quadratic_pullback_v1",
                "source": "genus_2_split_sextic_with_two_rational_infinities",
                "map": "pi(u,v)=(u^2+c,v)",
                "map_degree": 2,
                "supported_claim": "exact-prime-order_named-generated-subgroup_pullback_is_injective_with_checked_recovery",
                "normalized_target_backend": "montgomery_implemented_live_rows_primality_gated_edwards_unsupported",
                "extension_field_backend": "unsupported",
                "independent_source_jacobian_oracle": "not_provided_catalog_wide",
                "solver_assessment": "not_provided",
                "computational_advantage": "not_assessed",
            },
            "summary": {
                "odd_maps": records.len(),
                "direct_prime_maps": direct_maps,
                "normalized_prime_maps": normalized_maps,
                "odd_extension_maps": extension_maps,
                "maps_with_named_subgroups": maps_with_named_subgroups,
                "maps_without_named_subgroups": records.len() - maps_with_named_subgroups,
                "named_subgroup_records": named_subgroup_records,
                "supported_named_subgroup_records": supported,
                "supported_ramified_repeated_support_records": ramified_supported,
                "supported_ordinary_fiber_records": ordinary_supported,
                "unsupported_without_primality_certificate": unsupported_primality,
                "unsupported_kernel_intersection": unsupported_kernel_intersection,
                "unsupported_normalized_target_transfer_backend": unsupported_normalized,
                "unsupported_odd_extension_field_transfer_backend": unsupported_extension,
                "failed_invalid_target_generator": failed_invalid_target_generator,
                "failed_invalid_named_subgroup_data": failed_invalid_named_subgroup_data,
                "failed_transfer_arithmetic": failed_transfer_arithmetic,
            },
            "membership": {
                "canonical_encoding": "C-sorted LF-terminated map UIDs and map_uid<TAB>ec1<TAB>curve_uid tuples",
                "odd_maps_sha256": membership_digest(&map_uids),
                "all_bindings_sha256": membership_digest(&binding_membership),
                "supported_direct_bindings_sha256": membership_digest(&supported_membership),
                "unsupported_or_failed_direct_bindings_sha256": membership_digest(&unsupported_or_failed_direct_membership),
                "unsupported_primality_bindings_sha256": membership_digest(&unsupported_primality_membership),
                "unsupported_kernel_intersection_bindings_sha256": membership_digest(&unsupported_kernel_membership),
                "unsupported_normalized_bindings_sha256": membership_digest(&unsupported_normalized_membership),
                "unsupported_extension_bindings_sha256": membership_digest(&unsupported_extension_membership),
                "failed_invalid_data_bindings_sha256": membership_digest(&failed_invalid_membership),
                "failed_transfer_arithmetic_bindings_sha256": membership_digest(&failed_transfer_membership),
            },
            "records": records,
        }),
        all_named_subgroup_records_supported,
    ))
}

fn pretty_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn verification_status(all_named_subgroup_records_supported: bool) -> &'static str {
    if all_named_subgroup_records_supported {
        "verified_named_subgroup_complete"
    } else {
        "verified_named_subgroup_incomplete"
    }
}

fn verify_corpus(
    corpus_bytes: &[u8],
    registry_bytes: &[u8],
    graph_bytes: &[u8],
    expected_sha256: Option<&str>,
    expected_bytes: Option<u64>,
) -> Result<(Value, bool), String> {
    if expected_sha256.is_some() != expected_bytes.is_some() {
        return Err("expected_sha256 and expected_bytes must be supplied together".into());
    }
    let corpus_sha256 = checker::digest(corpus_bytes);
    if let Some(expected) = expected_sha256 {
        if expected.len() != 64
            || !expected
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err("expected_sha256 must be exactly 64 lowercase hexadecimal digits".into());
        }
        if expected != corpus_sha256 {
            return Err(format!(
                "raw certificate corpus SHA-256 mismatch: expected {expected}, got {corpus_sha256}"
            ));
        }
        if expected_bytes != Some(corpus_bytes.len() as u64) {
            return Err(format!(
                "raw certificate corpus byte-length mismatch: expected {}, got {}",
                expected_bytes.expect("paired option checked"),
                corpus_bytes.len()
            ));
        }
    }
    let saved: Value = serde_json::from_slice(corpus_bytes)
        .map_err(|error| format!("parse certificate corpus: {error}"))?;
    if saved["schema_version"] != CORPUS_SCHEMA {
        return Err("unsupported certificate corpus schema".into());
    }
    let producer_commit = saved["producer"]["git_commit"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks git_commit".to_string())?;
    let rustc_version = saved["producer"]["rustc_version"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks rustc_version".to_string())?;
    let build_provenance_source = saved["producer"]["build_provenance_source"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks build_provenance_source".to_string())?;
    let (regenerated, all_named_subgroup_records_supported) = odd_transfer_catalog(
        registry_bytes,
        graph_bytes,
        producer_commit,
        rustc_version,
        build_provenance_source,
    )?;
    if corpus_bytes != pretty_bytes(&regenerated)? {
        return Err(
            "certificate corpus is not the exact canonical byte serialization regenerated from the supplied inputs"
                .into(),
        );
    }
    Ok((
        json!({
            "schema_version": VERIFICATION_SCHEMA,
            "status": verification_status(all_named_subgroup_records_supported),
            "corpus": {
                "sha256": corpus_sha256,
                "bytes": corpus_bytes.len(),
                "schema_version": CORPUS_SCHEMA,
                "canonical_pretty_json_with_trailing_newline": true,
            },
            "producer": {
                "git_commit": producer_commit,
                "rustc_version": rustc_version,
                "rustc_version_attestation": "caller_supplied_unverified",
                "source_sha256": producer_source_digest(),
                "attestation": "saved_producer_claim_replayed_but_not_git_tree_checked_by_this_command",
            },
            "inputs": {
                "registry_sha256": checker::digest(registry_bytes),
                "cover_graph_sha256": checker::digest(graph_bytes),
            },
            "checks": {
                "optional_raw_hash_and_size": expected_sha256.is_some(),
                "cover_graph_rebuilt_exactly": true,
                "all_odd_maps_and_named_bindings_retained": true,
                "supported_bindings_semantically_replayed": true,
                "entire_corpus_value_regenerated": true,
                "canonical_bytes_equal": true,
                "all_named_subgroup_records_supported": all_named_subgroup_records_supported,
            },
            "summary": regenerated["summary"],
            "claim_boundary": "replay establishes only the recorded bounded construction, map, exact-prime-order named-subgroup, and structured unsupported/failure dispositions; it does not establish catalog-wide independent Jacobian arithmetic, a relation solver, or computational advantage",
        }),
        all_named_subgroup_records_supported,
    ))
}

fn write_value(value: &Value, output: Option<PathBuf>) -> Result<(), String> {
    let bytes = pretty_bytes(value)?;
    if let Some(path) = output {
        fs::write(&path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
    } else {
        std::io::stdout()
            .lock()
            .write_all(&bytes)
            .map_err(|error| format!("write stdout: {error}"))
    }
}

fn run(command: Command) -> Result<(Value, bool, Option<PathBuf>), String> {
    match command {
        Command::Export {
            registry,
            cover_graph,
            producer_commit,
            rustc_version,
            output,
        } => {
            let build_provenance_source = validate_export_build_provenance(&producer_commit)?;
            let registry = read_limited(&registry)?;
            let graph = read_limited(&cover_graph)?;
            let (value, all_named_subgroup_records_supported) = odd_transfer_catalog(
                &registry,
                &graph,
                &producer_commit,
                &rustc_version,
                build_provenance_source,
            )?;
            Ok((value, all_named_subgroup_records_supported, output))
        }
        Command::Verify {
            input,
            registry,
            cover_graph,
            expected_sha256,
            expected_bytes,
            output,
        } => {
            let corpus = read_limited(&input)?;
            let registry = read_limited(&registry)?;
            let graph = read_limited(&cover_graph)?;
            let (value, all_named_subgroup_records_supported) = verify_corpus(
                &corpus,
                &registry,
                &graph,
                expected_sha256.as_deref(),
                expected_bytes,
            )?;
            Ok((value, all_named_subgroup_records_supported, output))
        }
        Command::VerifySource {
            input,
            repo_root,
            output,
        } => {
            let corpus = read_limited(&input)?;
            let value = verify_source_tree(&corpus, &repo_root)?;
            Ok((value, true, output))
        }
    }
}

fn semantic_exit_code(all_named_subgroup_records_supported: bool) -> i32 {
    if all_named_subgroup_records_supported {
        0
    } else {
        1
    }
}

fn main() {
    let args = Args::parse();
    match run(args.command) {
        Ok((value, all_named_subgroup_records_supported, output)) => {
            if let Err(error) = write_value(&value, output) {
                eprintln!("{error}");
                std::process::exit(3);
            }
            let exit_code = semantic_exit_code(all_named_subgroup_records_supported);
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;
    use std::collections::BTreeMap;

    #[test]
    fn canonical_unsigned_parser_rejects_ambiguous_encodings() {
        assert_eq!(parse_unsigned("17").unwrap(), BigUint::from(17u32));
        assert_eq!(parse_unsigned("0x11").unwrap(), BigUint::from(17u32));
        for bad in ["", "017", "0x", "0x01", "0X11", "0xAB", "-1"] {
            assert!(parse_unsigned(bad).is_err(), "accepted {bad}");
        }
    }

    #[test]
    fn producer_manifest_and_conditional_proof_metadata_are_exact() {
        let expected_paths = vec![
            "src/lib.rs",
            "src/build_provenance.rs",
            "src/ct_bignum.rs",
            "src/bin/curve_cover_check.rs",
            "src/bin/curve_cover_check/checker.rs",
            "src/bin/curve_cover_check/links.rs",
            "src/bin/curve_cover_check/models.rs",
            "src/bin/odd_transfer_catalog.rs",
            "src/binary_ecc/mod.rs",
            "src/binary_ecc/f2m.rs",
            "src/binary_ecc/operation_tally.rs",
            "src/cryptanalysis/mod.rs",
            "src/cryptanalysis/ecc2k130_guard.rs",
            "src/cryptanalysis/bitops.rs",
            "src/hash/mod.rs",
            "src/hash/sha256.rs",
            "src/utils/mod.rs",
            "src/prime_hyperelliptic/balanced.rs",
            "src/prime_hyperelliptic/cover_transfer.rs",
            "src/prime_hyperelliptic/model_transfer.rs",
            "src/prime_hyperelliptic/normalized_cover_transfer.rs",
            "src/prime_hyperelliptic/fp2.rs",
            "src/prime_hyperelliptic/fp_poly.rs",
            "src/prime_hyperelliptic/mod.rs",
            "src/prime_hyperelliptic/ops.rs",
            "src/prime_hyperelliptic/primality.rs",
            "docs/curves/odd-subgroup-transfer-certificates.schema.json",
            "Cargo.toml",
            "Cargo.lock",
        ];
        let sources = producer_sources();
        assert_eq!(producer_source_paths(), expected_paths);
        assert!(sources.iter().all(|(_, contents)| !contents.is_empty()));
        let source_files = producer_source_files();
        assert_eq!(source_files.len(), sources.len());
        for (saved, (path, contents)) in source_files.iter().zip(&sources) {
            assert_eq!(saved["path"], *path);
            assert_eq!(saved["bytes"], contents.len());
            assert_eq!(saved["sha256"], checker::digest(contents.as_bytes()));
        }
        assert_eq!(producer_source_digest(), framed_source_digest(&sources));

        let proofs = proof_artifacts();
        assert_eq!(proofs.len(), 1);
        let proof = &proofs[0];
        assert_eq!(proof["id"], PROOF_ARTIFACT_ID);
        assert_eq!(proof["path"], PROOF_ARTIFACT_PATH);
        assert_eq!(proof["declaration"], PROOF_DECLARATION);
        assert_eq!(proof["toolchain"]["lean"], "4.35.0");
        assert_eq!(proof["toolchain"]["lean_commit"], PROOF_LEAN_COMMIT);
        assert_eq!(proof["verification_status"], "local_unsigned_unverified");
        let proof_contents = include_str!(
            "../../research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean"
        );
        assert_eq!(proof["bytes"], proof_contents.len());
        assert_eq!(proof["sha256"], checker::digest(proof_contents.as_bytes()));

        assert!(validate_build_provenance_source("CRYPTO_BUILD_GIT_COMMIT").is_ok());
        assert!(validate_build_provenance_source("GITHUB_SHA").is_ok());
        assert!(validate_build_provenance_source("caller-supplied").is_err());
    }

    #[test]
    fn binding_membership_distinguishes_one_ec1_across_multiple_maps() {
        let representation = small_direct_representation();
        let first = binding_membership_line(
            "urn:curve-cover-map:1:sha256:0000000000000000000000000000000000000000000000000000000000000000",
            &representation,
        )
        .unwrap();
        let second = binding_membership_line(
            "urn:curve-cover-map:1:sha256:1111111111111111111111111111111111111111111111111111111111111111",
            &representation,
        )
        .unwrap();
        let mut bindings = BTreeSet::new();
        assert!(bindings.insert(first));
        assert!(bindings.insert(second));
        assert_eq!(bindings.len(), 2);
    }

    #[test]
    fn normalized_and_extension_preflight_fail_closed() {
        let p = BigUint::from(5u32);
        let normalized_model = json!({
            "form": "B*y^2=x^3+A*x^2+x",
            "A": "0",
            "B": "1"
        });
        let normalized = json!({
            "ec1": "EC1P2Cfph000000000000",
            "curve_uid": "urn:ec-record:1:sha256:0000000000000000000000000000000000000000000000000000000000000000",
            "field": {
                "characteristic": 5,
                "degree": 1,
                "element_encoding": "hex integer modulo the characteristic",
                "representation": "prime"
            },
            "curve": {
                "model": "Montgomery",
                "coefficients": {"a": 0, "b": 1},
                "generator": ["0x0", "0x0"],
                "subgroup_order": "2"
            }
        });
        assert_eq!(
            validate_normalized_prime_generator(&normalized, &normalized_model, &p).unwrap(),
            BigUint::from(2u32)
        );
        let mut wrong_degree = normalized.clone();
        wrong_degree["field"]["degree"] = json!(2);
        let failure =
            validate_normalized_prime_generator(&wrong_degree, &normalized_model, &p).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_field_representation");

        let mut wrong_characteristic = normalized.clone();
        wrong_characteristic["field"]["characteristic"] = json!(7);
        let failure =
            validate_normalized_prime_generator(&wrong_characteristic, &normalized_model, &p)
                .unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_field_representation");

        let mut wrong_model = normalized.clone();
        wrong_model["curve"]["model"] = json!("Edwards");
        let failure =
            validate_normalized_prime_generator(&wrong_model, &normalized_model, &p).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_model_binding");

        let mut wrong_coefficient = normalized.clone();
        wrong_coefficient["curve"]["coefficients"]["b"] = json!(2);
        let failure =
            validate_normalized_prime_generator(&wrong_coefficient, &normalized_model, &p)
                .unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_model_binding");

        let mut off_normalized = normalized.clone();
        off_normalized["curve"]["generator"][1] = json!("0x1");
        assert_eq!(
            validate_normalized_prime_generator(&off_normalized, &normalized_model, &p)
                .unwrap_err()
                .status,
            "failed_invalid_target_generator"
        );

        let extension = checker::Field::Extension {
            p: p.clone(),
            low: vec![BigUint::from(2u32), BigUint::zero()],
        };
        let extension_representation = json!({
            "ec1": "EC1Q5D2Cfpkh000000000000",
            "curve_uid": "urn:ec-record:1:sha256:0000000000000000000000000000000000000000000000000000000000000000",
            "field": {
                "characteristic": 5,
                "degree": 2,
                "element_encoding": "decimal coefficient list [e_0, ..., e_{k-1}] of e_0 + e_1 t + ... + e_{k-1} t^{k-1} modulo the monic modulus",
                "modulus": [2, 0, 1],
                "representation": "polynomial"
            },
            "curve": {
                "generator": [[0, 0], [1, 0]],
                "subgroup_order": "3"
            }
        });
        let (_, order) = validate_extension_generator(
            &extension_representation,
            &extension,
            &BigUint::zero(),
            &BigUint::one(),
        )
        .unwrap();
        assert_eq!(order, BigUint::from(3u32));
        let mut noncanonical_extension = extension_representation.clone();
        noncanonical_extension["curve"]["generator"][0][0] = json!(5);
        assert_eq!(
            validate_extension_generator(
                &noncanonical_extension,
                &extension,
                &BigUint::zero(),
                &BigUint::one(),
            )
            .unwrap_err()
            .status,
            "failed_invalid_target_generator"
        );
        assert_eq!(
            prime_proof_status(&BigUint::from(9u32)),
            PrimeProofStatus::Composite
        );
    }

    #[test]
    fn pin_pair_is_fail_closed_before_catalog_replay() {
        let error = verify_corpus(b"{}", b"", b"", Some(&"0".repeat(64)), None)
            .expect_err("unpaired pin must fail");
        assert!(error.contains("must be supplied together"));
    }

    #[test]
    fn valid_but_incomplete_named_subgroup_scope_uses_exit_one() {
        assert_eq!(semantic_exit_code(true), 0);
        assert_eq!(semantic_exit_code(false), 1);
        assert_eq!(
            verification_status(false),
            "verified_named_subgroup_incomplete"
        );
    }

    #[test]
    fn inverse_of_two_mod_odd_annihilator_is_exact() {
        for order in [3u32, 5, 29, 101] {
            let order = BigUint::from(order);
            let inverse = (&order + BigUint::one()) / 2u32;
            assert_eq!((&inverse * 2u32) % &order, BigUint::one());
        }
    }

    fn small_direct_representation() -> Value {
        json!({
            "ec1": "EC1P3Cfphe20cb33f3186",
            "curve_uid": "urn:ec-record:1:sha256:e20cb33f3186b70438ea6cbb73835323433b59250ccc521360aa8328a31c296c",
            "field": {
                "characteristic": 5,
                "degree": 1,
                "element_encoding": "hex integer modulo the characteristic",
                "representation": "prime"
            },
            "curve": {
                "model": "short Weierstrass",
                "coefficients": [0, 0, 0, 0, 1],
                "generator": ["0x0", "0x1"],
                "subgroup_order": "3"
            }
        })
    }

    #[test]
    fn direct_fixture_and_mutations_fail_closed() {
        let p = BigUint::from(5u32);
        let a = BigUint::zero();
        let b = BigUint::one();
        let cover = PrimeQuadraticCoverP::try_new(p.clone(), a.clone(), b.clone(), BigUint::zero())
            .unwrap();
        let representation = small_direct_representation();
        let (generator, annihilator) = direct_generator(&representation, &p, &a, &b).unwrap();
        let record =
            supported_direct_record(&cover, &representation, &generator, &annihilator).unwrap();
        assert_eq!(record["status"], "supported_on_named_generated_subgroup");
        assert_eq!(record["fiber_kind"], "ramified_repeated_support");
        assert_eq!(
            record["checks"]["target_generator_coordinates_canonical"],
            true
        );
        assert_eq!(record["checks"]["pullback_valid"], true);

        let mut changed_generator = representation.clone();
        changed_generator["curve"]["generator"][1] = json!("0x0");
        let failure = direct_generator(&changed_generator, &p, &a, &b).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_target_generator");
        assert_eq!(failure.obligation, "target_generator_membership");
        assert!(failure.reason.contains("not on the recorded target curve"));
        let failed = failed_record(&changed_generator, &failure).unwrap();
        assert_eq!(failed["status"], "failed_invalid_target_generator");
        assert_eq!(
            failed["failures"][0]["obligation"],
            "target_generator_membership"
        );

        let mut changed_order = representation.clone();
        changed_order["curve"]["subgroup_order"] = json!("5");
        let (generator, annihilator) = direct_generator(&changed_order, &p, &a, &b).unwrap();
        assert!(
            supported_direct_record(&cover, &changed_order, &generator, &annihilator)
                .unwrap_err()
                .reason
                .contains("does not kill")
        );

        let mut even_order = representation.clone();
        even_order["curve"]["subgroup_order"] = json!("2");
        let (generator, annihilator) = direct_generator(&even_order, &p, &a, &b).unwrap();
        let failure =
            supported_direct_record(&cover, &even_order, &generator, &annihilator).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "annihilator_kills_target_generator");
        assert!(failure.reason.contains("does not kill"));

        let order_two_cover = PrimeQuadraticCoverP::try_new(
            p.clone(),
            BigUint::one(),
            BigUint::zero(),
            BigUint::one(),
        )
        .unwrap();
        let order_two_representation = json!({
            "ec1": "EC1P2Cfph000000000000",
            "curve_uid": "urn:ec-record:1:sha256:0000000000000000000000000000000000000000000000000000000000000000",
            "field": {
                "characteristic": 5,
                "degree": 1,
                "element_encoding": "hex integer modulo the characteristic",
                "representation": "prime"
            },
            "curve": {
                "model": "short Weierstrass",
                "coefficients": [0, 0, 0, 1, 0],
                "generator": ["0x0", "0x0"],
                "subgroup_order": "2"
            }
        });
        let (order_two_generator, order_two_annihilator) = direct_generator(
            &order_two_representation,
            &p,
            &BigUint::one(),
            &BigUint::zero(),
        )
        .unwrap();
        let failure = supported_direct_record(
            &order_two_cover,
            &order_two_representation,
            &order_two_generator,
            &order_two_annihilator,
        )
        .unwrap_err();
        assert_eq!(failure.status, "unsupported_kernel_intersection");
        assert_eq!(failure.obligation, "kernel_intersection");
        assert!(failure.reason.contains("exact-order subgroup"));

        let mut changed_field = representation;
        changed_field["field"]["element_encoding"] = json!("coerced scalar");
        let failure = direct_generator(&changed_field, &p, &a, &b).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_field_representation");
        assert!(failure
            .reason
            .contains("unsupported direct-prime generator encoding"));
        let failed = failed_record(&changed_field, &failure).unwrap();
        assert_eq!(failed["status"], "failed_invalid_named_subgroup_data");
        assert_eq!(
            failed["failures"][0]["obligation"],
            "target_field_representation"
        );

        let mut changed_model = small_direct_representation();
        changed_model["curve"]["coefficients"][4] = json!(2);
        let failure = direct_generator(&changed_model, &p, &a, &b).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "target_model_binding");
        assert!(failure
            .reason
            .contains("differ from the linked target model"));

        let mut changed_order_encoding = small_direct_representation();
        changed_order_encoding["curve"]["subgroup_order"] = json!("0x3");
        let failure = direct_generator(&changed_order_encoding, &p, &a, &b).unwrap_err();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "subgroup_order_encoding");

        let failure = BindingFailure::composite_field_or_order();
        assert_eq!(failure.status, "failed_invalid_named_subgroup_data");
        assert_eq!(failure.obligation, "deterministic_primality");

        assert_eq!(
            direct_map_status(PrimeProofStatus::Composite, 0, true, true, false),
            "failed_invalid_map_field_data"
        );
    }

    #[test]
    fn live_catalog_partition_and_membership_are_exact() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let registry = fs::read(root.join("docs/curves/registry.json")).unwrap();
        let graph = fs::read(root.join("docs/curves/cover-links.yaml")).unwrap();
        let (corpus, all_named_subgroup_records_supported) = odd_transfer_catalog(
            &registry,
            &graph,
            "0000000000000000000000000000000000000000",
            "test-toolchain",
            "CRYPTO_BUILD_GIT_COMMIT",
        )
        .unwrap();
        assert!(!all_named_subgroup_records_supported);
        assert_eq!(
            corpus["summary"],
            json!({
                "odd_maps": 22_243,
                "direct_prime_maps": 20_808,
                "normalized_prime_maps": 30,
                "odd_extension_maps": 1_405,
                "maps_with_named_subgroups": 935,
                "maps_without_named_subgroups": 21_308,
                "named_subgroup_records": 940,
                "supported_named_subgroup_records": 675,
                "supported_ramified_repeated_support_records": 133,
                "supported_ordinary_fiber_records": 542,
                "unsupported_without_primality_certificate": 240,
                "unsupported_kernel_intersection": 0,
                "unsupported_normalized_target_transfer_backend": 15,
                "unsupported_odd_extension_field_transfer_backend": 9,
                "failed_invalid_target_generator": 1,
                "failed_invalid_named_subgroup_data": 0,
                "failed_transfer_arithmetic": 0,
            })
        );
        assert_eq!(
            corpus["membership"]["odd_maps_sha256"],
            "2f95c2605c0b6f223baa3e1b42781eccc35055bf31358dccae0d9b56f4a5bdc6"
        );
        assert_eq!(
            corpus["membership"]["all_bindings_sha256"],
            "ce85473a79697fee333f945cb4219ef3dac3451bea7e129db099960147e0b6fc"
        );
        assert_eq!(
            corpus["membership"]["supported_direct_bindings_sha256"],
            "c6a9d57c70bc21c69d2e8223c3e3cf87836b0330dc01b198f17179599aeafa40"
        );
        assert_eq!(
            corpus["membership"]["unsupported_or_failed_direct_bindings_sha256"],
            "faa88c5abc76918cdda5b2c92a100c9e720a0b334d56b79ace1e4782acbd18da"
        );
        assert_eq!(
            corpus["membership"]["unsupported_primality_bindings_sha256"],
            "b324e5589ccd7ebd09dcee580742f84eb7fcfc88465fa18b73821f11dc25b36e"
        );
        assert_eq!(
            corpus["membership"]["unsupported_kernel_intersection_bindings_sha256"],
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            corpus["membership"]["unsupported_normalized_bindings_sha256"],
            "492d04af25b7b6bb0f06efb1c1f6603c579af97963bb73549452bf2c5e8f67d4"
        );
        assert_eq!(
            corpus["membership"]["unsupported_extension_bindings_sha256"],
            "2575ccf0193436b0bf71f1114c07e24fccb4d20a34b1ab700637179f06b3a9f6"
        );
        assert_eq!(
            corpus["membership"]["failed_invalid_data_bindings_sha256"],
            "d591b72e559804ad8813b9cbe1ebfb43acc1de627fea1c1f586dc1d7a9bd8c41"
        );
        assert_eq!(
            corpus["membership"]["failed_transfer_arithmetic_bindings_sha256"],
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            corpus["scope"]["normalized_target_backend"],
            "montgomery_implemented_live_rows_primality_gated_edwards_unsupported"
        );

        let graph_value: Value = serde_json::from_slice(&graph).unwrap();
        let records_by_map = corpus["records"]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| (record["map_uid"].as_str().unwrap(), record))
            .collect::<BTreeMap<_, _>>();
        let mut montgomery_maps = 0usize;
        let mut edwards_maps = 0usize;
        let mut montgomery_bindings = BTreeSet::new();
        let mut edwards_bindings = BTreeSet::new();
        for row in graph_value["curves"].as_array().unwrap() {
            let model: Value = serde_json::from_str(row["model_json"].as_str().unwrap()).unwrap();
            let form = model["form"].as_str().unwrap();
            let is_montgomery = form == "B*y^2=x^3+A*x^2+x";
            let is_edwards = matches!(form, "a*x^2+y^2=1+d*x^2*y^2" | "x^2+y^2=c^2*(1+d*x^2*y^2)");
            if !is_montgomery && !is_edwards {
                continue;
            }
            let map_uid = row["cover_links"][0]["map_uid"].as_str().unwrap();
            let record = records_by_map[map_uid];
            let membership = if is_montgomery {
                montgomery_maps += 1;
                assert_eq!(
                    record["status"],
                    "unsupported_without_primality_certificate"
                );
                &mut montgomery_bindings
            } else {
                edwards_maps += 1;
                assert_eq!(
                    record["status"],
                    "unsupported_normalized_target_transfer_backend"
                );
                &mut edwards_bindings
            };
            for representation in row["representations"].as_array().unwrap() {
                membership.insert(binding_membership_line(map_uid, representation).unwrap());
            }
        }
        assert_eq!((montgomery_maps, montgomery_bindings.len()), (6, 7));
        assert_eq!((edwards_maps, edwards_bindings.len()), (24, 15));
        assert_eq!(
            membership_digest(&montgomery_bindings),
            "dc3677987a113316d7abb6b4529b0035d7b3250c49f3575d158bedb71664cc01"
        );
        assert_eq!(
            membership_digest(&edwards_bindings),
            "492d04af25b7b6bb0f06efb1c1f6603c579af97963bb73549452bf2c5e8f67d4"
        );
        montgomery_bindings.extend(edwards_bindings);
        assert_eq!(
            membership_digest(&montgomery_bindings),
            "e6030c86de8bea5b1504ea662368518057a784be88fd7c7f77e035f6edb3b22e"
        );

        let failed = corpus["records"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|record| record["named_subgroups"].as_array().unwrap())
            .filter(|record| record["status"] == "failed_invalid_target_generator")
            .collect::<Vec<_>>();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0]["ec1"], "EC1P224Cfph4cfa5ca71ead");
        assert_eq!(
            failed[0]["failures"][0]["obligation"],
            "target_generator_membership"
        );
        assert!(failed[0]["reason"]
            .as_str()
            .unwrap()
            .contains("not on the recorded target curve"));
    }
}
