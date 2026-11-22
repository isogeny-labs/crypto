//! Standalone, structured interface to the repository's explicit
//! hyperelliptic-cover certificates and checked Jacobian arithmetic.
//!
//! This is deliberately not a discrete-log solver.  Prime-field split sextic
//! covers use a distinct checked balanced `(u,v,n_plus)` representation for
//! their two rational infinity points; they are never reinterpreted as
//! one-infinity Mumford pairs.  Binary covers use the separate checked
//! characteristic-two one-infinity implementation.

#[path = "curve_cover_check/checker.rs"]
mod checker;
#[path = "curve_cover_check/links.rs"]
mod links;
#[path = "curve_cover_check/models.rs"]
mod models;

use clap::{Parser, Subcommand};
use crypto_lib::binary_ecc::{
    BinaryPoint, ClosedPointPushforwardCertificate, F2mElement, F2mPoly, HyperellipticCurve,
    MumfordDivisor, OrdinaryBinaryCover, PullbackFiberKind, SubgroupTransferStatus,
};
use crypto_lib::prime_hyperelliptic::{
    prime_proof_evidence, prime_proof_status, BalancedDivisorP, BalancedHyperellipticError, FpPoly,
    HyperellipticCurveP, MumfordDivisorP, PrimeEllipticPointP, PrimeMontgomeryAdapterError,
    PrimeMontgomeryPointP, PrimeMontgomeryQuadraticCoverError, PrimeMontgomeryQuadraticCoverP,
    PrimeProofStatus, PrimeQuadraticCoverError, PrimeQuadraticCoverP, PrimeSubgroupTransferStatus,
    SplitHyperellipticCurveP,
};
use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

const RESPONSE_SCHEMA: &str = "hyperelliptic-infrastructure-command/v1";
const ARITHMETIC_SCHEMA: &str = "hyperelliptic-jacobian-operation/v1";
const TRANSFER_SCHEMA: &str = "hyperelliptic-transfer-operation/v1";
const TRANSFER_CATALOG_SCHEMA: &str = "binary-subgroup-transfer-certificates/v1";
const TRANSFER_CATALOG_VERIFY_SCHEMA: &str = "binary-subgroup-transfer-catalog-verification/v1";
const MAX_INPUT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_CATALOG_INPUT_BYTES: u64 = 96 * 1024 * 1024;

#[derive(Parser)]
#[command(
    name = "hyperelliptic-cover",
    about = "Construct and verify explicit H -> E covers and run checked Jacobian arithmetic",
    long_about = "A bounded, certificate-oriented interface. It constructs the repository's fixed same-field cover families, replays their certificates, runs checked arithmetic and induced transfer maps where the infinity model is supported, and exports catalog evidence. It is not an index-calculus or key-recovery solver."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Construct and replay a cover certificate for one exact elliptic model object.
    Construct {
        /// Exact supported elliptic model shape, or `-` for stdin; no registry lookup is implied.
        #[arg(long)]
        input: PathBuf,
        /// Response path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Replay a supplied `{model, certificate}` request.
    Verify {
        /// Verification request JSON, or `-` for stdin.
        #[arg(long)]
        input: PathBuf,
        /// Response path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Run checked arithmetic on supported binary, odd-prime one-infinity, or split-sextic Jacobians.
    Arithmetic {
        /// Arithmetic request JSON, or `-` for stdin.
        #[arg(long)]
        input: PathBuf,
        /// Response path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Evaluate and certify induced maps for checked binary, direct odd-prime,
    /// and admitted Montgomery-normalized odd-prime cover families.
    Transfer {
        /// Transfer request JSON, or `-` for stdin.
        #[arg(long)]
        input: PathBuf,
        /// Response path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Rebuild the deterministic cover catalog from the exact model registry.
    CatalogExport {
        /// Exact EC1/ICV1 registry JSON (required explicitly; it is not embedded in the executable).
        #[arg(long)]
        registry: PathBuf,
        /// Catalog path; omit to write the complete catalog to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Replay every named binary EC1 generator and emit hash-bound transfer certificates.
    TransferCatalogExport {
        /// Exact EC1/ICV1 registry JSON.
        #[arg(long)]
        registry: PathBuf,
        /// Exact curve-cover graph JSON produced from the same registry.
        #[arg(long)]
        cover_graph: PathBuf,
        /// Exact curve-traits JSON used only for separately scoped order evidence.
        #[arg(long)]
        traits: PathBuf,
        /// Forty-hex revision containing the producer sources being executed.
        #[arg(long)]
        producer_commit: String,
        /// Rust compiler version for the executable, such as `rustc 1.100.0 (...)`.
        #[arg(long)]
        rustc_version: String,
        /// Certificate-corpus path; omit to write the complete corpus to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Regenerate and byte-for-byte replay a named binary subgroup corpus.
    TransferCatalogVerify {
        /// Exact certificate corpus emitted by `transfer-catalog-export`.
        #[arg(long)]
        input: PathBuf,
        /// Exact EC1/ICV1 registry JSON used by the corpus.
        #[arg(long)]
        registry: PathBuf,
        /// Exact curve-cover graph JSON used by the corpus.
        #[arg(long)]
        cover_graph: PathBuf,
        /// Exact curve-traits JSON used by the corpus.
        #[arg(long)]
        traits: PathBuf,
        /// Optional SHA-256 of the exact raw corpus bytes.
        #[arg(long)]
        expected_sha256: Option<String>,
        /// Optional raw byte length. Must accompany `--expected-sha256`.
        #[arg(long)]
        expected_bytes: Option<u64>,
        /// Verification response path; omit to write it to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VerifyRequest {
    model: Value,
    certificate: checker::Certificate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DivisorWire {
    /// Coefficients in ascending powers, as canonical unsigned integers.
    u: Vec<String>,
    /// Coefficients in ascending powers, as canonical unsigned integers.
    v: Vec<String>,
    /// Infinity-plus multiplicity for the distinct split-sextic balanced
    /// representation.  It is required there and rejected for `(u,v)` models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    n_plus: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum BinaryPointWire {
    Infinity,
    /// Canonical unsigned coordinates. Binary fields interpret them as
    /// hexadecimal bitsets; prime fields accept decimal or `0x` notation.
    Affine {
        x: String,
        y: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TransferOperation {
    Pushforward,
    PushforwardCertificate,
    ImageEndomorphismCertificate,
    TorsionProjectorCertificate,
    DeckTransform,
    Pullback,
    PullPushCompositionCertificate,
    PushPullCompositionCertificate,
    PullbackClosedPointCertificate,
    GeneratedSubgroupCertificate,
    SubgroupStatus,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TransferRequest {
    schema: String,
    /// Exact binary elliptic model. The source and map are reconstructed and
    /// rechecked; callers cannot supply an asserted source equation or map.
    model: Value,
    operation: TransferOperation,
    #[serde(default)]
    divisor: Option<DivisorWire>,
    #[serde(default)]
    point: Option<BinaryPointWire>,
    /// Declared torsion/subgroup order or annihilator. Generic subgroup-status
    /// results treat exactness as external; named-generator certificates check
    /// annihilation directly and do not require or claim exactness.
    #[serde(default)]
    order: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ArithmeticOperation {
    Validate,
    Identity,
    Equal,
    Negate,
    Add,
    ScalarMultiply,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArithmeticRequest {
    schema: String,
    /// Exact target elliptic model.  The source H is reconstructed from the
    /// verified fixed-family certificate, rather than accepted by assertion.
    #[serde(default)]
    model: Option<Value>,
    /// A separately typed odd-characteristic one-infinity or split-sextic
    /// curve.  The family and infinity fields select the representation.
    #[serde(default)]
    odd_curve: Option<OddCurveWire>,
    operation: ArithmeticOperation,
    #[serde(default)]
    left: Option<DivisorWire>,
    #[serde(default)]
    right: Option<DivisorWire>,
    #[serde(default)]
    scalar: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct OddCurveWire {
    family: String,
    p: String,
    genus: u32,
    /// `f(x)` coefficients in ascending powers for `y^2=f(x)`.
    f: Vec<String>,
    infinity: String,
}

fn read_bytes_with_limit(path: &Path, maximum: u64) -> Result<Vec<u8>, String> {
    let (reader, source): (Box<dyn Read>, String) = if path.as_os_str() == "-" {
        (Box::new(std::io::stdin()), "stdin".into())
    } else {
        let file = fs::File::open(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        (Box::new(file), path.display().to_string())
    };
    let mut bytes = Vec::new();
    reader
        .take(maximum + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read {source}: {e}"))?;
    if bytes.len() as u64 > maximum {
        return Err(format!("input exceeds the {maximum}-byte limit"));
    }
    Ok(bytes)
}

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    read_bytes_with_limit(path, MAX_INPUT_BYTES)
}

fn parse_json(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&read_bytes(path)?).map_err(|e| format!("parse JSON: {e}"))
}

fn response(command: &str, status: &str, body: Value) -> Value {
    json!({
        "schema": RESPONSE_SCHEMA,
        "command": command,
        "status": status,
        "result": body,
    })
}

fn catalog_producer_source_digest() -> String {
    let producer_sources = [
        include_str!("curve_cover_check/checker.rs"),
        include_str!("curve_cover_check/models.rs"),
        include_str!("hyperelliptic_cover.rs"),
    ]
    .concat();
    checker::digest(producer_sources.as_bytes())
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

fn transfer_catalog_producer_source_digest() -> String {
    // Bind every local semantic input used by the catalog, including the
    // primality gate behind exact-order evidence and its word-sized backend.
    framed_source_digest(&[
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
            "src/bin/hyperelliptic_cover.rs",
            include_str!("hyperelliptic_cover.rs"),
        ),
        (
            "src/binary_ecc/cover_transfer.rs",
            include_str!("../binary_ecc/cover_transfer.rs"),
        ),
        (
            "src/binary_ecc/curve.rs",
            include_str!("../binary_ecc/curve.rs"),
        ),
        (
            "src/binary_ecc/f2m.rs",
            include_str!("../binary_ecc/f2m.rs"),
        ),
        (
            "src/binary_ecc/hyperelliptic.rs",
            include_str!("../binary_ecc/hyperelliptic.rs"),
        ),
        (
            "src/binary_ecc/mod.rs",
            include_str!("../binary_ecc/mod.rs"),
        ),
        (
            "src/binary_ecc/operation_tally.rs",
            include_str!("../binary_ecc/operation_tally.rs"),
        ),
        (
            "src/binary_ecc/poly_f2m.rs",
            include_str!("../binary_ecc/poly_f2m.rs"),
        ),
        (
            "src/cryptanalysis/ecc2k130_guard.rs",
            include_str!("../cryptanalysis/ecc2k130_guard.rs"),
        ),
        (
            "src/cryptanalysis/group_op_count.rs",
            include_str!("../cryptanalysis/group_op_count.rs"),
        ),
        (
            "src/prime_hyperelliptic/fp2.rs",
            include_str!("../prime_hyperelliptic/fp2.rs"),
        ),
        (
            "src/prime_hyperelliptic/mod.rs",
            include_str!("../prime_hyperelliptic/mod.rs"),
        ),
        (
            "src/prime_hyperelliptic/primality.rs",
            include_str!("../prime_hyperelliptic/primality.rs"),
        ),
        ("src/hash/sha256.rs", include_str!("../hash/sha256.rs")),
        ("src/utils/mod.rs", include_str!("../utils/mod.rs")),
        (
            "docs/curves/binary-subgroup-transfer-certificates.schema.json",
            include_str!("../../docs/curves/binary-subgroup-transfer-certificates.schema.json"),
        ),
        ("Cargo.toml", include_str!("../../Cargo.toml")),
        ("Cargo.lock", include_str!("../../Cargo.lock")),
    ])
}

fn model_error(command: &str, error: checker::ModelError) -> (Value, i32) {
    match error {
        checker::ModelError::Unsupported(reason) => (
            response(
                command,
                "unsupported",
                json!({"reason": reason, "existence": "not_established_by_this_command"}),
            ),
            2,
        ),
        checker::ModelError::Invalid(reason) => (
            response(command, "invalid_input", json!({"reason": reason})),
            3,
        ),
    }
}

fn strict_model(value: &Value) -> Result<checker::Model, checker::ModelError> {
    let object = value.as_object().ok_or_else(|| {
        checker::ModelError::Invalid("elliptic model must be a JSON object".into())
    })?;
    let allowed: &[&str] = match value.get("form").and_then(Value::as_str) {
        Some("y^2=x^3+a*x+b") if value.get("k").is_some() => {
            &["v", "form", "p", "k", "modulus", "field", "a", "b"]
        }
        Some("y^2=x^3+a*x+b") => &["v", "form", "p", "field", "a", "b"],
        Some("y^2+xy=x^3+a*x^2+b") => &["v", "form", "modulus", "field", "a", "b"],
        Some("B*y^2=x^3+A*x^2+x") => &["v", "form", "p", "field", "A", "B"],
        Some("a*x^2+y^2=1+d*x^2*y^2") => &["v", "form", "p", "field", "a", "d"],
        Some("x^2+y^2=c^2*(1+d*x^2*y^2)") => &["v", "form", "p", "field", "c", "d"],
        _ => &[
            "v", "form", "p", "k", "modulus", "field", "a", "b", "A", "B", "c", "d",
        ],
    };
    if let Some(unexpected) = object.keys().find(|name| !allowed.contains(&name.as_str())) {
        return Err(checker::ModelError::Invalid(format!(
            "unexpected elliptic model field `{unexpected}`"
        )));
    }
    checker::model(value)
}

fn proof_receipt(model: &checker::Model) -> Result<Value, String> {
    match &model.field {
        checker::Field::Prime(p) => {
            let prime_status = prime_proof_status(p);
            if prime_status == PrimeProofStatus::Composite {
                return Err("prime modulus failed deterministic primality validation".into());
            }
            let proved_prime = matches!(
                prime_status,
                PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
            );
            let normalization_kind = model
                .normalization
                .as_ref()
                .and_then(|map| map["kind"].as_str());
            let (jacobian_arithmetic, pullback, pushforward, composition, subgroup_transfer) =
                if proved_prime
                    && normalization_kind.is_some()
                    && normalization_kind != Some("montgomery_from_short")
                {
                    (
                        "implemented_checked_balanced_split_sextic_with_deterministic_prime_revalidation",
                        "unsupported_normalized_target_transfer_backend",
                        "unsupported_normalized_target_transfer_backend",
                        "unsupported_normalized_target_transfer_backend",
                        "unknown_until_normalized_target_transfer_is_implemented",
                    )
                } else if proved_prime {
                    (
                        "implemented_checked_balanced_split_sextic_with_deterministic_prime_revalidation",
                        "implemented_for_rational_target_point_classes",
                        "implemented_for_every_canonical_balanced_genus_two_class",
                        "verified_pi_push_pi_pull_equals_multiplication_by_2_and_pi_pull_pi_push_equals_1_plus_deck",
                        "conditional_on_a_checked_annihilator_coprime_to_2_for_a_named_generated_subgroup; exact order and primality remain separate evidence",
                    )
                } else {
                    (
                        "unsupported_without_primality_certificate",
                        "unsupported_without_primality_certificate",
                        "unsupported_without_primality_certificate",
                        "unsupported_without_primality_certificate",
                        "unknown",
                    )
                };
            Ok(json!({
                "construction": "explicit",
                "field_of_definition": "same_as_target",
                "field_validation": if proved_prime {"deterministic_prime_proof"} else {"fixed_base_probable_prime_screen_only"},
                "prime_proof": prime_proof_evidence(p),
                "target_smoothness": "verified_by_discriminant",
                "source_smoothness": "verified_by_squarefree_sextic",
                "genus": "verified_genus_2",
                "defining_equation_identity": "verified_exact_polynomial_identity",
                "degree": "verified_degree_2",
                "separability": "verified_characteristic_not_2",
                "infinity": "two_base_field_rational_points",
                "jacobian_arithmetic": jacobian_arithmetic,
                "jacobian_representation": "canonical_balanced_(u,v,n_plus)",
                "target_model_transfer": match normalization_kind {
                    None => "direct_short_weierstrass",
                    Some("montgomery_from_short") if proved_prime => "implemented_checked_projective_montgomery_isomorphism",
                    Some("montgomery_from_short") => "implemented_backend_blocked_by_missing_primality_certificate",
                    Some(_) => "unsupported_projective_exceptional_point_backend",
                },
                "pullback": pullback,
                "pushforward": pushforward,
                "composition": composition,
                "subgroup_transfer": subgroup_transfer,
            }))
        }
        // The same sextic cover over `GF(p^k)`; the checker also applies
        // Rabin's irreducibility test to the modulus over `GF(p)`.
        checker::Field::Extension { .. } => Ok(json!({
            "construction": "explicit",
            "field_of_definition": "same_as_target",
            "field_validation": "fixed-base_probable_prime_screen_of_p_and_rabin_irreducible_modulus",
            "target_smoothness": "verified_by_discriminant",
            "source_smoothness": "verified_by_squarefree_sextic",
            "genus": "verified_genus_2",
            "defining_equation_identity": "verified_exact_polynomial_identity",
            "degree": "verified_degree_2",
            "separability": "verified_characteristic_not_2",
            "infinity": "two_base_field_rational_points",
            "jacobian_arithmetic": "unsupported_extension_field_balanced_backend",
            "pullback": "not_implemented_for_this_cover_family",
            "pushforward": "not_implemented_for_this_cover_family",
            "subgroup_transfer": "unknown",
        })),
        checker::Field::Binary(irreducible) => {
            let cover = OrdinaryBinaryCover::new(
                irreducible.degree,
                irreducible.clone(),
                F2mElement::from_biguint(&model.a, irreducible.degree),
                F2mElement::from_biguint(&model.b, irreducible.degree),
            )
            .map_err(|error| format!("binary cover runtime validation failed: {error}"))?;
            cover
                .verify_certificate()
                .map_err(|error| format!("binary cover certificate replay failed: {error}"))?;
            let certificate = cover.certificate();
            Ok(json!({
                "construction": "explicit",
                "field_of_definition": "same_as_target",
                "field_validation": "irreducibility_proved_by_rabin",
                "target_smoothness": "verified_b_nonzero",
                "source_smoothness": "verified_affine_and_infinity_fixed_family_criteria",
                "genus": "verified_genus_3_from_two_order_3_artin_schreier_poles",
                "defining_equation_identity": "verified_exact_polynomial_identity",
                "degree": "verified_degree_3",
                "separability": "verified_characteristic_2_does_not_divide_3",
                "infinity": "one_base_field_rational_point",
                "ramification": {
                    "infinity_index": certificate.infinity_ramification_index,
                    "exceptional_index": certificate.exceptional_ramification_index,
                    "riemann_hurwitz_total": certificate.riemann_hurwitz_ramification_total,
                },
                "jacobian_arithmetic": "implemented_checked_one_infinity_model",
                "pullback": "implemented_for_rational_target_point_classes",
                "pushforward": "implemented_for_valid_reduced_jacobian_classes_with_support_degree_at_most_3",
                "composition": "verified_phi_push_phi_pull_equals_multiplication_by_3_on_typed_rational_point_class_domain",
                "arbitrary_jacobian_pushforward": "implemented_checked_closed_point_norm_for_valid_reduced_degree_at_most_3_classes_on_this_binary_model",
                "cli_transfer_subcommand": "exposed_structured_binary_and_direct_odd_prime",
                "subgroup_transfer": "named-generator certificate proves injectivity from a checked annihilator coprime to 3; exact order and primality remain separate evidence",
            }))
        }
    }
}

fn construct(model_value: Value) -> (Value, i32) {
    let model = match strict_model(&model_value) {
        Ok(model) => model,
        Err(error) => return model_error("construct", error),
    };
    let certificate = checker::construct(&model);
    if let Err(reason) = checker::verify(&model, &certificate) {
        return (
            response(
                "construct",
                "error",
                json!({"reason": format!("internal certificate replay failed: {reason}")}),
            ),
            1,
        );
    }
    let verification = match proof_receipt(&model) {
        Ok(verification) => verification,
        Err(reason) => return (response("construct", "error", json!({"reason": reason})), 1),
    };
    (
        response(
            "construct",
            "ok",
            json!({
                "model": model_value,
                "certificate": certificate,
                "verification": verification,
                "scope": "explicit same-field cover; no minimality, unconditional subgroup-transfer, or DLP-advantage claim",
            }),
        ),
        0,
    )
}

fn verify(request: VerifyRequest) -> (Value, i32) {
    let model = match strict_model(&request.model) {
        Ok(model) => model,
        Err(error) => return model_error("verify", error),
    };
    match checker::verify(&model, &request.certificate) {
        Ok(()) => match proof_receipt(&model) {
            Ok(verification) => (
                response(
                    "verify",
                    "ok",
                    json!({
                        "certificate": request.certificate,
                        "verification": verification,
                    }),
                ),
                0,
            ),
            Err(reason) => (response("verify", "error", json!({"reason": reason})), 1),
        },
        Err(reason) => (
            response("verify", "invalid_input", json!({"reason": reason})),
            3,
        ),
    }
}

fn parse_number(text: &str) -> Result<BigUint, String> {
    if text.len() > 1300 {
        return Err("integer exceeds field-size limit".into());
    }
    let (digits, radix) = text
        .strip_prefix("0x")
        .map_or((text, 10), |digits| (digits, 16));
    BigUint::parse_bytes(digits.as_bytes(), radix)
        .ok_or_else(|| format!("invalid unsigned integer: {text}"))
}

fn parse_binary_field_element(text: &str) -> Result<BigUint, String> {
    if text.len() > 1300 {
        return Err("integer exceeds field-size limit".into());
    }
    let digits = text.strip_prefix("0x").unwrap_or(text);
    BigUint::parse_bytes(digits.as_bytes(), 16)
        .ok_or_else(|| format!("invalid hexadecimal binary-field element: {text}"))
}

fn binary_poly(encoded: &[String], field: &checker::Field, m: u32) -> Result<F2mPoly, String> {
    let coefficients = checker::decoded(encoded, field)?
        .into_iter()
        .map(|coefficient| F2mElement::from_biguint(&coefficient, m))
        .collect();
    Ok(F2mPoly::from_coeffs(coefficients, m))
}

fn divisor_from_wire(
    wire: &DivisorWire,
    field: &checker::Field,
    curve: &HyperellipticCurve,
) -> Result<MumfordDivisor, String> {
    if wire.n_plus.is_some() {
        return Err("binary one-infinity divisors do not accept `n_plus`".into());
    }
    let u = binary_poly(&wire.u, field, curve.m)?;
    let v = binary_poly(&wire.v, field, curve.m)?;
    MumfordDivisor::try_new(curve, u, v).map_err(|e| format!("{e:?}"))
}

fn encoded_poly(poly: &F2mPoly) -> Vec<String> {
    poly.coeffs
        .iter()
        .map(|coefficient| format!("0x{:x}", coefficient.to_biguint()))
        .collect()
}

fn divisor_to_wire(divisor: &MumfordDivisor) -> DivisorWire {
    DivisorWire {
        u: encoded_poly(&divisor.u),
        v: encoded_poly(&divisor.v),
        n_plus: None,
    }
}

fn binary_point_from_wire(wire: &BinaryPointWire, m: u32) -> Result<BinaryPoint, String> {
    match wire {
        BinaryPointWire::Infinity => Ok(BinaryPoint::Infinity),
        BinaryPointWire::Affine { x, y } => {
            let x = parse_binary_field_element(x)?;
            let y = parse_binary_field_element(y)?;
            if x.bits() > m as u64 || y.bits() > m as u64 {
                return Err("point coordinate is not canonical for the declared field".into());
            }
            Ok(BinaryPoint::Affine {
                x: F2mElement::from_biguint(&x, m),
                y: F2mElement::from_biguint(&y, m),
            })
        }
    }
}

fn binary_point_to_json(point: &BinaryPoint) -> Value {
    match point {
        BinaryPoint::Infinity => json!({"kind": "infinity"}),
        BinaryPoint::Affine { x, y } => json!({
            "kind": "affine",
            "x": format!("0x{:x}", x.to_biguint()),
            "y": format!("0x{:x}", y.to_biguint()),
        }),
    }
}

fn prime_point_from_wire(
    wire: &BinaryPointWire,
    p: &BigUint,
) -> Result<PrimeEllipticPointP, String> {
    match wire {
        BinaryPointWire::Infinity => Ok(PrimeEllipticPointP::Infinity),
        BinaryPointWire::Affine { x, y } => {
            let x = parse_number(x)?;
            let y = parse_number(y)?;
            if &x >= p || &y >= p {
                return Err(
                    "point coordinate is not canonical for the declared prime field".into(),
                );
            }
            Ok(PrimeEllipticPointP::Affine { x, y })
        }
    }
}

fn prime_point_to_json(point: &PrimeEllipticPointP) -> Value {
    match point {
        PrimeEllipticPointP::Infinity => json!({"kind": "infinity"}),
        PrimeEllipticPointP::Affine { x, y } => json!({
            "kind": "affine",
            "x": format!("0x{x:x}"),
            "y": format!("0x{y:x}"),
        }),
    }
}

fn prime_montgomery_point_from_wire(
    wire: &BinaryPointWire,
    p: &BigUint,
) -> Result<PrimeMontgomeryPointP, String> {
    match prime_point_from_wire(wire, p)? {
        PrimeEllipticPointP::Infinity => Ok(PrimeMontgomeryPointP::Infinity),
        PrimeEllipticPointP::Affine { x, y } => Ok(PrimeMontgomeryPointP::Affine { x, y }),
    }
}

fn prime_montgomery_point_to_json(point: &PrimeMontgomeryPointP) -> Value {
    match point {
        PrimeMontgomeryPointP::Infinity => json!({"kind": "infinity"}),
        PrimeMontgomeryPointP::Affine { x, y } => json!({
            "kind": "affine",
            "x": format!("0x{x:x}"),
            "y": format!("0x{y:x}"),
        }),
    }
}

fn fiber_kind_to_json(kind: &PullbackFiberKind) -> Value {
    match kind {
        PullbackFiberKind::InfinityRamified { ramification_index } => json!({
            "kind": "infinity_ramified",
            "ramification_index": ramification_index,
        }),
        PullbackFiberKind::ExceptionalTwoTorsionRamified { ramification_index } => json!({
            "kind": "exceptional_two_torsion_ramified",
            "ramification_index": ramification_index,
        }),
        PullbackFiberKind::GenericSeparable { geometric_degree } => json!({
            "kind": "generic_separable",
            "geometric_degree": geometric_degree,
        }),
    }
}

fn closed_point_component_to_json(component: &ClosedPointPushforwardCertificate) -> Value {
    json!({
        "support": encoded_poly(&component.support),
        "residue_degree": component.degree,
        "image": binary_point_to_json(&component.image),
    })
}

fn closed_point_components_to_json(components: &[ClosedPointPushforwardCertificate]) -> Vec<Value> {
    components
        .iter()
        .map(closed_point_component_to_json)
        .collect()
}

fn required_divisor<'a>(
    value: &'a Option<DivisorWire>,
    name: &str,
) -> Result<&'a DivisorWire, String> {
    value
        .as_ref()
        .ok_or_else(|| format!("operation requires `{name}` divisor"))
}

fn operation_name(operation: ArithmeticOperation) -> &'static str {
    match operation {
        ArithmeticOperation::Validate => "validate",
        ArithmeticOperation::Identity => "identity",
        ArithmeticOperation::Equal => "equal",
        ArithmeticOperation::Negate => "negate",
        ArithmeticOperation::Add => "add",
        ArithmeticOperation::ScalarMultiply => "scalar_multiply",
    }
}

fn validate_arithmetic_operands(request: &ArithmeticRequest) -> Result<(), String> {
    let require = |present: bool, name: &str| {
        present
            .then_some(())
            .ok_or_else(|| format!("operation requires `{name}`"))
    };
    let forbid = |present: bool, name: &str| {
        (!present)
            .then_some(())
            .ok_or_else(|| format!("operation does not accept `{name}`"))
    };

    match request.operation {
        ArithmeticOperation::Identity => {
            forbid(request.left.is_some(), "left")?;
            forbid(request.right.is_some(), "right")?;
            forbid(request.scalar.is_some(), "scalar")?;
        }
        ArithmeticOperation::Validate | ArithmeticOperation::Negate => {
            require(request.left.is_some(), "left")?;
            forbid(request.right.is_some(), "right")?;
            forbid(request.scalar.is_some(), "scalar")?;
        }
        ArithmeticOperation::Equal | ArithmeticOperation::Add => {
            require(request.left.is_some(), "left")?;
            require(request.right.is_some(), "right")?;
            forbid(request.scalar.is_some(), "scalar")?;
        }
        ArithmeticOperation::ScalarMultiply => {
            require(request.left.is_some(), "left")?;
            require(request.scalar.is_some(), "scalar")?;
            forbid(request.right.is_some(), "right")?;
        }
    }
    Ok(())
}

fn arithmetic_result(
    operation: ArithmeticOperation,
    context: Value,
    computed: Result<Value, String>,
) -> (Value, i32) {
    let operation = operation_name(operation);
    match computed {
        Ok(result) => (
            response(
                "arithmetic",
                "ok",
                json!({
                    "operation": operation,
                    "context": context,
                    "result": result,
                }),
            ),
            0,
        ),
        Err(reason) => (
            response(
                "arithmetic",
                "invalid_input",
                json!({"operation": operation, "reason": reason}),
            ),
            3,
        ),
    }
}

fn model_split_sextic_arithmetic(
    request: &ArithmeticRequest,
    model_value: &Value,
    certificate: &checker::Certificate,
    p: &BigUint,
) -> (Value, i32) {
    let f = match prime_poly(&certificate.f, p, "constructed f") {
        Ok(f) => f,
        Err(reason) => {
            return (
                response(
                    "arithmetic",
                    "error",
                    json!({"reason": format!("replayed construction could not be decoded for balanced arithmetic: {reason}")}),
                ),
                1,
            )
        }
    };
    let curve = match SplitHyperellipticCurveP::try_new(p.clone(), f) {
        Ok(curve) => curve,
        Err(BalancedHyperellipticError::UnsupportedPrimalityProofRange { .. }) => {
            return (
                response(
                    "arithmetic",
                    "unsupported",
                    json!({
                        "reason": "the explicit split-sextic cover is constructed and replayed, but balanced arithmetic requires a deterministic modulus proof",
                        "construction_status": "explicit_verified",
                        "arithmetic_status": "unsupported_without_primality_certificate",
                        "prime_proof": prime_proof_evidence(p),
                    }),
                ),
                2,
            )
        }
        Err(error) => {
            return (
                response(
                    "arithmetic",
                    "error",
                    json!({"reason": format!("replayed construction failed balanced initialization: {error}")}),
                ),
                1,
            )
        }
    };
    balanced_arithmetic_for_curve(
        request,
        &curve,
        json!({
            "family": "prime_quadratic_pullback_v1_split_sextic_balanced",
            "target_model": model_value,
            "curve_construction": certificate,
            "field": {"characteristic": p.to_str_radix(10), "representation": "prime_residue"},
            "infinity_configuration": "two_rational_points",
            "infinity_labels": {"plus": "y/x^3=+1", "minus": "y/x^3=-1"},
            "infinity_polynomial_plus": encoded_prime_poly(curve.infinity_polynomial()),
            "representation": "canonical_balanced_(u,v,n_plus)",
            "prime_proof": prime_proof_evidence(p),
            "scope": "checked balanced Jacobian arithmetic inside the strict deterministic primality-proof boundary; the transfer command separately exposes quadratic cover maps for direct short-Weierstrass and admitted projective Montgomery targets, while Edwards normalized targets remain unsupported",
        }),
    )
}

fn model_arithmetic(request: &ArithmeticRequest, model_value: &Value) -> (Value, i32) {
    let model = match strict_model(model_value) {
        Ok(model) => model,
        Err(error) => return model_error("arithmetic", error),
    };
    let certificate = checker::construct(&model);
    if let Err(reason) = checker::verify(&model, &certificate) {
        return (
            response("arithmetic", "error", json!({"reason": reason})),
            1,
        );
    }
    let irr = match &model.field {
        checker::Field::Extension { .. } => {
            return (
                response(
                    "arithmetic",
                    "unsupported",
                    json!({
                        "reason": "the explicit GF(p^k) cover is constructed and replayed, but the split-sextic balanced backend currently supports prime fields only",
                        "construction_status": "explicit_verified",
                        "arithmetic_status": "unsupported_extension_field",
                    }),
                ),
                2,
            )
        }
        checker::Field::Prime(p) => {
            return model_split_sextic_arithmetic(request, model_value, &certificate, p)
        }
        checker::Field::Binary(irr) => irr,
    };
    let h = match binary_poly(&certificate.h, &model.field, irr.degree) {
        Ok(poly) => poly,
        Err(reason) => {
            return (
                response("arithmetic", "error", json!({"reason": reason})),
                1,
            )
        }
    };
    let f = match binary_poly(&certificate.f, &model.field, irr.degree) {
        Ok(poly) => poly,
        Err(reason) => {
            return (
                response("arithmetic", "error", json!({"reason": reason})),
                1,
            )
        }
    };
    let curve = match HyperellipticCurve::try_new(irr.degree, irr.clone(), h, f, certificate.genus)
    {
        Ok(curve) => curve,
        Err(error) => {
            return (
                response(
                    "arithmetic",
                    "error",
                    json!({"reason": format!("constructed curve failed checked initialization: {error:?}")}),
                ),
                1,
            )
        }
    };

    let parse = |wire: &Option<DivisorWire>, name: &str| -> Result<MumfordDivisor, String> {
        divisor_from_wire(required_divisor(wire, name)?, &model.field, &curve)
    };
    let computed: Result<Value, String> = match request.operation {
        ArithmeticOperation::Identity => Ok(json!({
            "divisor": divisor_to_wire(&MumfordDivisor::identity(curve.m)),
        })),
        ArithmeticOperation::Validate => parse(&request.left, "left")
            .map(|divisor| json!({"valid": true, "divisor": divisor_to_wire(&divisor)})),
        ArithmeticOperation::Equal => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let equal = left.try_eq(&right, &curve).map_err(|e| format!("{e:?}"))?;
            Ok(json!({"equal": equal}))
        })(),
        ArithmeticOperation::Negate => (|| {
            let left = parse(&request.left, "left")?;
            let result = left.try_neg(&curve).map_err(|e| format!("{e:?}"))?;
            Ok(json!({"divisor": divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::Add => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let result = left.try_add(&right, &curve).map_err(|e| format!("{e:?}"))?;
            Ok(json!({"divisor": divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::ScalarMultiply => (|| {
            let left = parse(&request.left, "left")?;
            let scalar = request
                .scalar
                .as_deref()
                .ok_or_else(|| "operation requires `scalar`".to_string())
                .and_then(parse_number)?;
            let result = left
                .try_scalar_mul(&scalar, &curve)
                .map_err(|e| format!("{e:?}"))?;
            Ok(json!({"divisor": divisor_to_wire(&result), "scalar": format!("0x{scalar:x}")}))
        })(),
    };
    arithmetic_result(
        request.operation,
        json!({
            "family": "binary_cubic_pullback_v1",
            "curve_construction": certificate,
            "infinity_configuration": "one_rational_point",
        }),
        computed,
    )
}

fn prime_poly(encoded: &[String], p: &BigUint, name: &str) -> Result<FpPoly, String> {
    if p < &BigUint::from(2u32) {
        return Err("prime-field modulus must be at least 2".into());
    }
    if encoded.len() > 8194 {
        return Err(format!(
            "{name} exceeds the supported polynomial-size limit"
        ));
    }
    let coefficients = encoded
        .iter()
        .map(|text| parse_number(text))
        .collect::<Result<Vec<_>, _>>()?;
    if coefficients.iter().any(|coefficient| coefficient >= p) {
        return Err(format!("{name} has a noncanonical coefficient"));
    }
    if coefficients.last().is_some_and(Zero::is_zero) {
        return Err(format!("{name} has a noncanonical trailing zero"));
    }
    Ok(FpPoly::from_coeffs(coefficients, p.clone()))
}

fn prime_divisor_from_wire(
    wire: &DivisorWire,
    curve: &HyperellipticCurveP,
) -> Result<MumfordDivisorP, String> {
    if wire.n_plus.is_some() {
        return Err("odd one-infinity divisors do not accept `n_plus`".into());
    }
    let u = prime_poly(&wire.u, &curve.p, "u")?;
    let v = prime_poly(&wire.v, &curve.p, "v")?;
    MumfordDivisorP::try_from_polynomials(curve, u, v).map_err(|e| format!("{e:?}"))
}

fn encoded_prime_poly(poly: &FpPoly) -> Vec<String> {
    poly.coeffs
        .iter()
        .map(|coefficient| format!("0x{coefficient:x}"))
        .collect()
}

fn prime_divisor_to_wire(divisor: &MumfordDivisorP) -> DivisorWire {
    DivisorWire {
        u: encoded_prime_poly(&divisor.u),
        v: encoded_prime_poly(&divisor.v),
        n_plus: None,
    }
}

fn balanced_divisor_from_wire(
    wire: &DivisorWire,
    curve: &SplitHyperellipticCurveP,
) -> Result<BalancedDivisorP, String> {
    let n_plus = wire
        .n_plus
        .ok_or_else(|| "split-sextic balanced divisors require `n_plus`".to_string())?;
    let u = prime_poly(&wire.u, &curve.p, "u")?;
    let v = prime_poly(&wire.v, &curve.p, "v")?;
    BalancedDivisorP::try_from_polynomials(curve, u, v, n_plus).map_err(|error| error.to_string())
}

fn balanced_divisor_to_wire(divisor: &BalancedDivisorP) -> DivisorWire {
    DivisorWire {
        u: encoded_prime_poly(&divisor.u),
        v: encoded_prime_poly(&divisor.v),
        n_plus: Some(divisor.n_plus),
    }
}

fn one_infinity_odd_arithmetic(
    request: &ArithmeticRequest,
    supplied: &OddCurveWire,
) -> (Value, i32) {
    if supplied.family != "odd_prime_one_infinity_v1" || supplied.infinity != "one_rational" {
        return (
            response(
                "arithmetic",
                "unsupported",
                json!({
                    "reason": "odd arithmetic requires family odd_prime_one_infinity_v1 and infinity one_rational",
                    "arithmetic_status": "unsupported_model",
                }),
            ),
            2,
        );
    }
    let p = match parse_number(&supplied.p) {
        Ok(p) => p,
        Err(reason) => {
            return (
                response("arithmetic", "invalid_input", json!({"reason": reason})),
                3,
            )
        }
    };
    if p < BigUint::from(3u32) || !p.bit(0) {
        return (
            response(
                "arithmetic",
                "invalid_input",
                json!({"reason": "odd one-infinity arithmetic requires an odd modulus at least 3"}),
            ),
            3,
        );
    }
    let f = match prime_poly(&supplied.f, &p, "f") {
        Ok(f) => f,
        Err(reason) => {
            return (
                response("arithmetic", "invalid_input", json!({"reason": reason})),
                3,
            )
        }
    };
    let curve = match HyperellipticCurveP::try_new(p, f, supplied.genus) {
        Ok(curve) => curve,
        Err(error) => {
            return (
                response(
                    "arithmetic",
                    "invalid_input",
                    json!({"reason": format!("{error:?}")}),
                ),
                3,
            )
        }
    };
    let parse = |wire: &Option<DivisorWire>, name: &str| -> Result<MumfordDivisorP, String> {
        prime_divisor_from_wire(required_divisor(wire, name)?, &curve)
    };
    let computed: Result<Value, String> = match request.operation {
        ArithmeticOperation::Identity => MumfordDivisorP::try_identity(&curve)
            .map(|divisor| json!({"divisor": prime_divisor_to_wire(&divisor)}))
            .map_err(|e| format!("{e:?}")),
        ArithmeticOperation::Validate => parse(&request.left, "left")
            .map(|divisor| json!({"valid": true, "divisor": prime_divisor_to_wire(&divisor)})),
        ArithmeticOperation::Equal => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let equal = left
                .checked_eq(&right, &curve)
                .map_err(|e| format!("{e:?}"))?;
            Ok(json!({"equal": equal}))
        })(),
        ArithmeticOperation::Negate => (|| {
            let left = parse(&request.left, "left")?;
            let result = left.checked_neg(&curve).map_err(|e| format!("{e:?}"))?;
            Ok(json!({"divisor": prime_divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::Add => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let result = left
                .checked_add(&right, &curve)
                .map_err(|e| format!("{e:?}"))?;
            Ok(json!({"divisor": prime_divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::ScalarMultiply => (|| {
            let left = parse(&request.left, "left")?;
            let scalar = request
                .scalar
                .as_deref()
                .ok_or_else(|| "operation requires `scalar`".to_string())
                .and_then(parse_number)?;
            let result = left
                .checked_scalar_mul(&scalar, &curve)
                .map_err(|e| format!("{e:?}"))?;
            Ok(
                json!({"divisor": prime_divisor_to_wire(&result), "scalar": format!("0x{scalar:x}")}),
            )
        })(),
    };
    arithmetic_result(
        request.operation,
        json!({
            "family": supplied.family,
            "field": {"characteristic": supplied.p, "representation": "prime_residue"},
            "curve": supplied,
            "infinity_configuration": "one_rational_point",
        }),
        computed,
    )
}

fn balanced_arithmetic_for_curve(
    request: &ArithmeticRequest,
    curve: &SplitHyperellipticCurveP,
    context: Value,
) -> (Value, i32) {
    let parse = |wire: &Option<DivisorWire>, name: &str| -> Result<BalancedDivisorP, String> {
        balanced_divisor_from_wire(required_divisor(wire, name)?, curve)
    };
    let computed: Result<Value, String> = match request.operation {
        ArithmeticOperation::Identity => BalancedDivisorP::try_identity(curve)
            .map(|divisor| json!({"divisor": balanced_divisor_to_wire(&divisor)}))
            .map_err(|error| error.to_string()),
        ArithmeticOperation::Validate => parse(&request.left, "left")
            .map(|divisor| json!({"valid": true, "divisor": balanced_divisor_to_wire(&divisor)})),
        ArithmeticOperation::Equal => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let equal = left
                .checked_eq(&right, curve)
                .map_err(|error| error.to_string())?;
            Ok(json!({"equal": equal}))
        })(),
        ArithmeticOperation::Negate => (|| {
            let left = parse(&request.left, "left")?;
            let result = left.checked_neg(curve).map_err(|error| error.to_string())?;
            Ok(json!({"divisor": balanced_divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::Add => (|| {
            let left = parse(&request.left, "left")?;
            let right = parse(&request.right, "right")?;
            let result = left
                .checked_add(&right, curve)
                .map_err(|error| error.to_string())?;
            Ok(json!({"divisor": balanced_divisor_to_wire(&result)}))
        })(),
        ArithmeticOperation::ScalarMultiply => (|| {
            let left = parse(&request.left, "left")?;
            let scalar = request
                .scalar
                .as_deref()
                .ok_or_else(|| "operation requires `scalar`".to_string())
                .and_then(parse_number)?;
            let result = left
                .checked_scalar_mul(&scalar, curve)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "divisor": balanced_divisor_to_wire(&result),
                "scalar": format!("0x{scalar:x}"),
            }))
        })(),
    };
    arithmetic_result(request.operation, context, computed)
}

fn split_sextic_odd_arithmetic(
    request: &ArithmeticRequest,
    supplied: &OddCurveWire,
) -> (Value, i32) {
    if supplied.family != "odd_prime_split_sextic_balanced_v1"
        || supplied.infinity != "two_rational"
        || supplied.genus != 2
    {
        return (
            response(
                "arithmetic",
                "unsupported",
                json!({
                    "reason": "balanced arithmetic requires family odd_prime_split_sextic_balanced_v1, infinity two_rational, and genus 2",
                    "arithmetic_status": "unsupported_model",
                }),
            ),
            2,
        );
    }
    let p = match parse_number(&supplied.p) {
        Ok(p) => p,
        Err(reason) => {
            return (
                response("arithmetic", "invalid_input", json!({"reason": reason})),
                3,
            )
        }
    };
    let f = match prime_poly(&supplied.f, &p, "f") {
        Ok(f) => f,
        Err(reason) => {
            return (
                response("arithmetic", "invalid_input", json!({"reason": reason})),
                3,
            )
        }
    };
    let curve = match SplitHyperellipticCurveP::try_new(p.clone(), f) {
        Ok(curve) => curve,
        Err(BalancedHyperellipticError::UnsupportedPrimalityProofRange { .. }) => {
            return (
                response(
                    "arithmetic",
                    "unsupported",
                    json!({
                        "reason": "split-sextic balanced arithmetic requires a deterministic modulus proof",
                        "arithmetic_status": "unsupported_without_primality_certificate",
                        "prime_proof": prime_proof_evidence(&p),
                    }),
                ),
                2,
            )
        }
        Err(error) => {
            return (
                response(
                    "arithmetic",
                    "invalid_input",
                    json!({"reason": error.to_string()}),
                ),
                3,
            )
        }
    };
    balanced_arithmetic_for_curve(
        request,
        &curve,
        json!({
            "family": supplied.family,
            "field": {"characteristic": supplied.p, "representation": "prime_residue"},
            "prime_proof": prime_proof_evidence(&p),
            "curve": supplied,
            "infinity_configuration": "two_rational_points",
            "infinity_labels": {"plus": "y/x^3=+1", "minus": "y/x^3=-1"},
            "infinity_polynomial_plus": encoded_prime_poly(curve.infinity_polynomial()),
            "representation": "canonical_balanced_(u,v,n_plus)",
            "scope": "checked Jacobian arithmetic only; cover-transfer, subgroup, solver, and comparative-cost claims are outside this command",
        }),
    )
}

fn odd_arithmetic(request: &ArithmeticRequest, supplied: &OddCurveWire) -> (Value, i32) {
    match supplied.family.as_str() {
        "odd_prime_one_infinity_v1" => one_infinity_odd_arithmetic(request, supplied),
        "odd_prime_split_sextic_balanced_v1" => split_sextic_odd_arithmetic(request, supplied),
        _ => (
            response(
                "arithmetic",
                "unsupported",
                json!({
                    "reason": "unsupported odd-characteristic arithmetic family",
                    "arithmetic_status": "unsupported_model",
                }),
            ),
            2,
        ),
    }
}

fn arithmetic(request: ArithmeticRequest) -> (Value, i32) {
    if request.schema != ARITHMETIC_SCHEMA {
        return (
            response(
                "arithmetic",
                "unsupported",
                json!({"reason": "unsupported arithmetic request schema"}),
            ),
            2,
        );
    }
    if let Err(reason) = validate_arithmetic_operands(&request) {
        return (
            response(
                "arithmetic",
                "invalid_input",
                json!({
                    "operation": operation_name(request.operation),
                    "reason": reason,
                }),
            ),
            3,
        );
    }
    match (&request.model, &request.odd_curve) {
        (Some(model), None) => model_arithmetic(&request, model),
        (None, Some(curve)) => odd_arithmetic(&request, curve),
        _ => (
            response(
                "arithmetic",
                "invalid_input",
                json!({"reason": "supply exactly one of `model` or `odd_curve`"}),
            ),
            3,
        ),
    }
}

fn transfer_operation_name(operation: TransferOperation) -> &'static str {
    match operation {
        TransferOperation::Pushforward => "pushforward",
        TransferOperation::PushforwardCertificate => "pushforward_certificate",
        TransferOperation::ImageEndomorphismCertificate => "image_endomorphism_certificate",
        TransferOperation::TorsionProjectorCertificate => "torsion_projector_certificate",
        TransferOperation::DeckTransform => "deck_transform",
        TransferOperation::Pullback => "pullback",
        TransferOperation::PullPushCompositionCertificate => "pull_push_composition_certificate",
        TransferOperation::PushPullCompositionCertificate => "push_pull_composition_certificate",
        TransferOperation::PullbackClosedPointCertificate => "pullback_closed_point_certificate",
        TransferOperation::GeneratedSubgroupCertificate => "generated_subgroup_certificate",
        TransferOperation::SubgroupStatus => "subgroup_status",
    }
}

fn validate_transfer_operands(request: &TransferRequest) -> Result<(), String> {
    let require = |present: bool, name: &str| {
        present
            .then_some(())
            .ok_or_else(|| format!("operation requires `{name}`"))
    };
    let forbid = |present: bool, name: &str| {
        (!present)
            .then_some(())
            .ok_or_else(|| format!("operation does not accept `{name}`"))
    };

    match request.operation {
        TransferOperation::Pushforward
        | TransferOperation::PushforwardCertificate
        | TransferOperation::ImageEndomorphismCertificate
        | TransferOperation::DeckTransform
        | TransferOperation::PushPullCompositionCertificate => {
            require(request.divisor.is_some(), "divisor")?;
            forbid(request.point.is_some(), "point")?;
            forbid(request.order.is_some(), "order")?;
        }
        TransferOperation::TorsionProjectorCertificate => {
            require(request.divisor.is_some(), "divisor")?;
            require(request.order.is_some(), "order")?;
            forbid(request.point.is_some(), "point")?;
        }
        TransferOperation::Pullback
        | TransferOperation::PullPushCompositionCertificate
        | TransferOperation::PullbackClosedPointCertificate => {
            require(request.point.is_some(), "point")?;
            forbid(request.divisor.is_some(), "divisor")?;
            forbid(request.order.is_some(), "order")?;
        }
        TransferOperation::GeneratedSubgroupCertificate => {
            require(request.point.is_some(), "point")?;
            require(request.order.is_some(), "order")?;
            forbid(request.divisor.is_some(), "divisor")?;
        }
        TransferOperation::SubgroupStatus => {
            require(request.order.is_some(), "order")?;
            forbid(request.divisor.is_some(), "divisor")?;
            forbid(request.point.is_some(), "point")?;
        }
    }
    Ok(())
}

fn prime_transfer(
    request: TransferRequest,
    model: checker::Model,
    curve_construction: checker::Certificate,
    p: BigUint,
) -> (Value, i32) {
    if matches!(
        request.operation,
        TransferOperation::ImageEndomorphismCertificate
            | TransferOperation::TorsionProjectorCertificate
            | TransferOperation::PullbackClosedPointCertificate
    ) {
        return (
            response(
                "transfer",
                "unsupported",
                json!({
                    "operation": transfer_operation_name(request.operation),
                    "reason": "this binary-only certificate shape is not implemented for the odd-prime quadratic family",
                }),
            ),
            2,
        );
    }

    let coordinate_map = match checker::decoded(&curve_construction.x, &model.field) {
        Ok(map) => map,
        Err(reason) => return (response("transfer", "error", json!({"reason": reason})), 1),
    };
    if coordinate_map.len() != 3
        || !coordinate_map[1].is_zero()
        || coordinate_map[2] != BigUint::from(1u32)
    {
        return (
            response(
                "transfer",
                "error",
                json!({"reason": "verified prime cover has an unexpected coordinate map"}),
            ),
            1,
        );
    }
    let shift = coordinate_map[0].clone();
    let normalization = model.normalization.clone();
    if let Some(target_model_map) = &normalization {
        if target_model_map["kind"] != "montgomery_from_short" {
            return (
                response(
                    "transfer",
                    "unsupported",
                    json!({
                        "reason": "Edwards target normalization still requires a typed projective adapter with explicit exceptional-point handling",
                        "construction_status": "explicit_verified",
                        "transfer_status": "unsupported_normalized_target_transfer_backend",
                        "target_model_map_kind": target_model_map["kind"],
                    }),
                ),
                2,
            );
        }
    }

    let unsupported_prime = || {
        (
            response(
                "transfer",
                "unsupported",
                json!({
                    "reason": "odd-prime induced maps require a deterministic modulus proof",
                    "construction_status": "explicit_verified",
                    "transfer_status": "unsupported_without_primality_certificate",
                    "prime_proof": prime_proof_evidence(&p),
                    "normalized_target_backend": normalization.as_ref().map(|map| map["kind"].clone()),
                }),
            ),
            2,
        )
    };

    let (cover, montgomery_adapter) = if let Some(target_model_map) = &normalization {
        let read_map_number = |name: &str| -> Result<BigUint, String> {
            target_model_map[name]
                .as_str()
                .ok_or_else(|| format!("normalized target map is missing `{name}`"))
                .and_then(parse_number)
        };
        let montgomery_a = match read_map_number("montgomery_A") {
            Ok(value) => value,
            Err(reason) => return (response("transfer", "error", json!({"reason": reason})), 1),
        };
        let montgomery_b = match read_map_number("montgomery_B") {
            Ok(value) => value,
            Err(reason) => return (response("transfer", "error", json!({"reason": reason})), 1),
        };
        let recorded_shift = match read_map_number("shift_A_over_3") {
            Ok(value) => value,
            Err(reason) => return (response("transfer", "error", json!({"reason": reason})), 1),
        };
        let composed = match PrimeMontgomeryQuadraticCoverP::try_new(
            p.clone(),
            montgomery_a,
            montgomery_b,
            shift.clone(),
        ) {
            Ok(cover) => cover,
            Err(PrimeMontgomeryQuadraticCoverError::Adapter(
                PrimeMontgomeryAdapterError::UnsupportedPrimalityProofRange { .. },
            ))
            | Err(PrimeMontgomeryQuadraticCoverError::Cover(
                PrimeQuadraticCoverError::UnsupportedPrimalityProofRange { .. },
            )) => return unsupported_prime(),
            Err(error) => {
                return (
                    response(
                        "transfer",
                        "error",
                        json!({"reason": format!("normalized Montgomery cover runtime validation failed: {error}")}),
                    ),
                    1,
                )
            }
        };
        let short_a_decimal = model.a.to_str_radix(10);
        let short_b_decimal = model.b.to_str_radix(10);
        if composed.adapter().shift_a_over_3() != &recorded_shift
            || composed.adapter().short_a() != &model.a
            || composed.adapter().short_b() != &model.b
            || target_model_map["source_model"]["a"].as_str() != Some(short_a_decimal.as_str())
            || target_model_map["source_model"]["b"].as_str() != Some(short_b_decimal.as_str())
            || target_model_map.get("target_model") != Some(&request.model)
        {
            return (
                response(
                    "transfer",
                    "error",
                    json!({"reason": "normalized target map metadata does not match the typed Montgomery composition"}),
                ),
                1,
            );
        }
        (
            composed.short_cover().clone(),
            Some(composed.adapter().clone()),
        )
    } else {
        let cover = match PrimeQuadraticCoverP::try_new(
            p.clone(),
            model.a.clone(),
            model.b.clone(),
            shift.clone(),
        ) {
            Ok(cover) => cover,
            Err(PrimeQuadraticCoverError::UnsupportedPrimalityProofRange { .. }) => {
                return unsupported_prime()
            }
            Err(error) => {
                return (
                    response(
                        "transfer",
                        "error",
                        json!({"reason": format!("prime cover runtime validation failed: {error}")}),
                    ),
                    1,
                )
            }
        };
        (cover, None)
    };
    let certificate_source = match prime_poly(&curve_construction.f, &p, "certificate source f") {
        Ok(polynomial) => polynomial,
        Err(reason) => return (response("transfer", "error", json!({"reason": reason})), 1),
    };
    if cover.source_curve().f != certificate_source {
        return (
            response(
                "transfer",
                "error",
                json!({"reason": "runtime prime cover differs from the verified geometry certificate"}),
            ),
            1,
        );
    }

    let divisor = || -> Result<BalancedDivisorP, String> {
        let wire = request
            .divisor
            .as_ref()
            .ok_or_else(|| "operation requires `divisor`".to_string())?;
        balanced_divisor_from_wire(wire, cover.source_curve())
    };
    let point = || -> Result<PrimeEllipticPointP, String> {
        let wire = request
            .point
            .as_ref()
            .ok_or_else(|| "operation requires `point`".to_string())?;
        if let Some(adapter) = &montgomery_adapter {
            let point = prime_montgomery_point_from_wire(wire, &p)?;
            adapter
                .montgomery_to_short(&point)
                .map_err(|error| error.to_string())
        } else {
            prime_point_from_wire(wire, &p)
        }
    };
    let point_json = |point: &PrimeEllipticPointP| -> Result<Value, String> {
        if let Some(adapter) = &montgomery_adapter {
            let point = adapter
                .short_to_montgomery(point)
                .map_err(|error| error.to_string())?;
            Ok(prime_montgomery_point_to_json(&point))
        } else {
            Ok(prime_point_to_json(point))
        }
    };
    let order = || -> Result<BigUint, String> {
        request
            .order
            .as_deref()
            .ok_or_else(|| "operation requires `order`".to_string())
            .and_then(parse_number)
    };

    let computed: Result<Value, String> = match request.operation {
        TransferOperation::Pushforward | TransferOperation::PushforwardCertificate => (|| {
            let divisor = divisor()?;
            let image = cover
                .pushforward_arbitrary_balanced_class(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": balanced_divisor_to_wire(&divisor),
                "image": point_json(&image)?,
                "method": "checked_symmetric_base_field_norm_for_every_canonical_balanced_class",
            }))
        })(),
        TransferOperation::DeckTransform => (|| {
            let divisor = divisor()?;
            let transformed = cover
                .deck_transform(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": balanced_divisor_to_wire(&divisor),
                "deck_transform": balanced_divisor_to_wire(&transformed),
                "deck_map": "(u,v)->(-u,v); infinity_plus<->infinity_minus",
            }))
        })(),
        TransferOperation::Pullback => (|| {
            let point = point()?;
            let pullback = cover
                .pullback_point_class(&point)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": point_json(&point)?,
                "pullback": balanced_divisor_to_wire(&pullback),
            }))
        })(),
        TransferOperation::PullPushCompositionCertificate => (|| {
            let point = point()?;
            let certificate = cover
                .pull_push_composition_certificate(&point)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": point_json(&certificate.input)?,
                "pullback": balanced_divisor_to_wire(&certificate.pullback),
                "pushforward": point_json(&certificate.pushforward)?,
                "twice_input": point_json(&certificate.twice_input)?,
                "identity": "pi_* pi^* = [2]",
                "identity_holds": certificate.identity_holds,
            }))
        })(),
        TransferOperation::PushPullCompositionCertificate => (|| {
            let divisor = divisor()?;
            let certificate = cover
                .push_pull_composition_certificate(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": balanced_divisor_to_wire(&certificate.input),
                "pushforward": point_json(&certificate.pushforward)?,
                "pullback": balanced_divisor_to_wire(&certificate.pullback),
                "deck_transform": balanced_divisor_to_wire(&certificate.deck_transform),
                "input_plus_deck": balanced_divisor_to_wire(&certificate.input_plus_deck),
                "identity": "pi^* pi_* = 1 + deck_*",
                "identity_holds": certificate.identity_holds,
            }))
        })(),
        TransferOperation::GeneratedSubgroupCertificate => (|| {
            let generator = point()?;
            if generator == PrimeEllipticPointP::Infinity {
                return Err("named generator must be nonidentity".into());
            }
            let annihilator = order()?;
            let degree_inverse = match cover
                .subgroup_transfer_status(&annihilator)
                .map_err(|error| error.to_string())?
            {
                PrimeSubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
                    degree_inverse_mod_order,
                    ..
                } => degree_inverse_mod_order,
                PrimeSubgroupTransferStatus::KernelIntersectionUnresolved {
                    common_factor, ..
                } => {
                    return Err(format!(
                        "checked annihilator is not coprime to degree two (gcd={common_factor})"
                    ))
                }
            };
            if cover
                .checked_target_scalar_mul(&generator, &annihilator)
                .map_err(|error| error.to_string())?
                != PrimeEllipticPointP::Infinity
            {
                return Err("checked annihilator does not kill the named generator".into());
            }
            let certificate = cover
                .pull_push_composition_certificate(&generator)
                .map_err(|error| error.to_string())?;
            if certificate.pullback.is_identity() {
                return Err("named nonidentity generator had identity pullback".into());
            }
            if !certificate
                .pullback
                .checked_scalar_mul(&annihilator, cover.source_curve())
                .map_err(|error| error.to_string())?
                .is_identity()
            {
                return Err("checked annihilator did not kill the generator pullback".into());
            }
            let recovered = cover
                .checked_target_scalar_mul(&certificate.pushforward, &degree_inverse)
                .map_err(|error| error.to_string())?;
            if recovered != generator {
                return Err(
                    "degree-inverse pushforward did not recover the named generator".into(),
                );
            }
            Ok(json!({
                "claim_scope": "pullback_is_injective_on_the_named_generated_subgroup; annihilator_exactness_and_primality_not_claimed",
                "generator": point_json(&generator)?,
                "annihilator": annihilator.to_str_radix(10),
                "map_degree": 2,
                "degree_inverse_mod_annihilator": degree_inverse.to_str_radix(10),
                "checks": {
                    "generator_is_on_target": true,
                    "generator_is_nonidentity": true,
                    "generator_is_annihilated": true,
                    "pullback_is_nonidentity": true,
                    "pullback_is_annihilated": true,
                    "pull_push_composition_holds": certificate.identity_holds,
                    "recovery_matches_generator": true,
                },
                "pullback": balanced_divisor_to_wire(&certificate.pullback),
                "pushforward": point_json(&certificate.pushforward)?,
                "twice_generator": point_json(&certificate.twice_input)?,
                "recovered_generator": point_json(&recovered)?,
                "exact_order_or_primality_claimed": false,
                "proof_reference": "research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean#injective_with_recovery",
            }))
        })(),
        TransferOperation::SubgroupStatus => (|| {
            let order = order()?;
            let status = cover
                .subgroup_transfer_status(&order)
                .map_err(|error| error.to_string())?;
            Ok(match status {
                PrimeSubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
                    order,
                    degree_inverse_mod_order,
                } => json!({
                    "status": "injective_if_exact_subgroup_order",
                    "order": order.to_str_radix(10),
                    "degree_inverse_mod_order": degree_inverse_mod_order.to_str_radix(10),
                    "exact_order_verified_by_command": false,
                }),
                PrimeSubgroupTransferStatus::KernelIntersectionUnresolved {
                    order,
                    common_factor,
                } => json!({
                    "status": "kernel_intersection_unresolved",
                    "order": order.to_str_radix(10),
                    "common_factor": common_factor,
                    "exact_order_verified_by_command": false,
                }),
            })
        })(),
        TransferOperation::ImageEndomorphismCertificate
        | TransferOperation::TorsionProjectorCertificate
        | TransferOperation::PullbackClosedPointCertificate => {
            unreachable!("unsupported prime operations return before evaluation")
        }
    };

    let operation = transfer_operation_name(request.operation);
    let (family, map, coordinate_scope, scope) = if let Some(adapter) = &montgomery_adapter {
        (
            "normalized_prime_quadratic_pullback_v1_split_sextic_balanced_montgomery",
            json!({
                "short_intermediate": {"X": format!("u^2+0x{shift:x}"), "Y": "v"},
                "short_to_montgomery": {
                    "x": format!("0x{:x}*X-0x{:x}", adapter.montgomery_b(), adapter.shift_a_over_3()),
                    "y": format!("0x{:x}*Y", adapter.montgomery_b()),
                },
                "composed": {
                    "x": format!("0x{:x}*(u^2+0x{shift:x})-0x{:x}", adapter.montgomery_b(), adapter.shift_a_over_3()),
                    "y": format!("0x{:x}*v", adapter.montgomery_b()),
                },
            }),
            "all target point inputs and outputs use the original Montgomery coordinates; balanced divisors use the normalized split-sextic source",
            "checked induced map over an admitted odd prime and an everywhere-defined projective Montgomery isomorphism; exact subgroup-order, extension-field, Edwards exceptional-point, solver, scaling, and comparative-cost claims are outside this command",
        )
    } else {
        (
            "prime_quadratic_pullback_v1_split_sextic_balanced",
            json!({"x": format!("u^2+0x{shift:x}"), "y": "v"}),
            "target point inputs and outputs use direct short-Weierstrass coordinates; balanced divisors use the split-sextic source",
            "checked induced map over a direct odd prime inside the strict deterministic primality-proof boundary; exact subgroup-order, extension-field, solver, scaling, and comparative-cost claims are outside this command",
        )
    };
    match computed {
        Ok(result) => (
            response(
                "transfer",
                "ok",
                json!({
                    "operation": operation,
                    "context": {
                        "family": family,
                        "target_model": request.model,
                        "curve_construction": curve_construction,
                        "source_genus": 2,
                        "map_degree": 2,
                        "map": map,
                        "coordinate_scope": coordinate_scope,
                        "infinity": "two_base_field_rational_points_both_mapping_to_target_identity",
                        "prime_proof": prime_proof_evidence(&p),
                    },
                    "result": result,
                    "scope": scope,
                }),
            ),
            0,
        ),
        Err(reason) => (
            response(
                "transfer",
                "invalid_input",
                json!({"operation": operation, "reason": reason}),
            ),
            3,
        ),
    }
}

fn transfer(request: TransferRequest) -> (Value, i32) {
    if request.schema != TRANSFER_SCHEMA {
        return (
            response(
                "transfer",
                "unsupported",
                json!({"reason": "unsupported transfer request schema"}),
            ),
            2,
        );
    }
    let model = match strict_model(&request.model) {
        Ok(model) => model,
        Err(error) => return model_error("transfer", error),
    };
    if let Err(reason) = validate_transfer_operands(&request) {
        return (
            response(
                "transfer",
                "invalid_input",
                json!({
                    "operation": transfer_operation_name(request.operation),
                    "reason": reason,
                }),
            ),
            3,
        );
    }
    let curve_construction = checker::construct(&model);
    if let Err(reason) = checker::verify(&model, &curve_construction) {
        return (
            response(
                "transfer",
                "error",
                json!({"reason": format!("constructed cover certificate failed replay: {reason}")}),
            ),
            1,
        );
    }
    if let checker::Field::Prime(p) = &model.field {
        let p = p.clone();
        return prime_transfer(request, model, curve_construction, p);
    }
    if matches!(&model.field, checker::Field::Extension { .. }) {
        return (
            response(
                "transfer",
                "unsupported",
                json!({
                    "reason": "odd-characteristic extension-field cover geometry is available, but balanced divisor arithmetic and induced maps currently support direct prime fields only",
                    "transfer_status": "unsupported_extension_field_balanced_backend",
                }),
            ),
            2,
        );
    }
    let checker::Field::Binary(irreducible) = &model.field else {
        unreachable!("prime and extension fields returned above")
    };
    let cover = match OrdinaryBinaryCover::new(
        irreducible.degree,
        irreducible.clone(),
        F2mElement::from_biguint(&model.a, irreducible.degree),
        F2mElement::from_biguint(&model.b, irreducible.degree),
    ) {
        Ok(cover) => cover,
        Err(error) => {
            return (
                response(
                    "transfer",
                    "error",
                    json!({"reason": format!("binary cover runtime validation failed: {error}")}),
                ),
                1,
            )
        }
    };
    if matches!(
        request.operation,
        TransferOperation::DeckTransform | TransferOperation::PushPullCompositionCertificate
    ) {
        return (
            response(
                "transfer",
                "unsupported",
                json!({
                    "operation": transfer_operation_name(request.operation),
                    "reason": "this operation is exposed only for the checked odd-prime quadratic cover family",
                }),
            ),
            2,
        );
    }

    let divisor = || -> Result<MumfordDivisor, String> {
        let wire = request
            .divisor
            .as_ref()
            .ok_or_else(|| "operation requires `divisor`".to_string())?;
        divisor_from_wire(wire, &model.field, cover.source_curve())
    };
    let point = || -> Result<BinaryPoint, String> {
        let wire = request
            .point
            .as_ref()
            .ok_or_else(|| "operation requires `point`".to_string())?;
        binary_point_from_wire(wire, irreducible.degree)
    };
    let order = || -> Result<BigUint, String> {
        request
            .order
            .as_deref()
            .ok_or_else(|| "operation requires `order`".to_string())
            .and_then(parse_number)
    };

    let computed: Result<Value, String> = match request.operation {
        TransferOperation::Pushforward => (|| {
            let divisor = divisor()?;
            let image = cover
                .pushforward_arbitrary_jacobian_class(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": divisor_to_wire(&divisor),
                "image": binary_point_to_json(&image),
            }))
        })(),
        TransferOperation::PushforwardCertificate => (|| {
            let divisor = divisor()?;
            let certificate = cover
                .jacobian_pushforward_certificate(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": divisor_to_wire(&certificate.input),
                "components": closed_point_components_to_json(&certificate.components),
                "reconstructed_support": encoded_poly(&certificate.reconstructed_support),
                "component_image_sum": binary_point_to_json(&certificate.component_image_sum),
                "general_pushforward": binary_point_to_json(&certificate.general_pushforward),
                "support_reconstruction_holds": certificate.support_reconstruction_holds,
                "component_sum_holds": certificate.component_sum_holds,
            }))
        })(),
        TransferOperation::ImageEndomorphismCertificate => (|| {
            let divisor = divisor()?;
            let certificate = cover
                .image_endomorphism_certificate(&divisor)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": divisor_to_wire(&certificate.input),
                "pushforward": binary_point_to_json(&certificate.pushforward),
                "endomorphism": divisor_to_wire(&certificate.endomorphism),
                "endomorphism_squared": divisor_to_wire(&certificate.endomorphism_squared),
                "three_times_endomorphism": divisor_to_wire(&certificate.three_times_endomorphism),
                "quadratic_identity_holds": certificate.quadratic_identity_holds,
                "pushforward_kernel_member": certificate.pushforward_kernel_member,
            }))
        })(),
        TransferOperation::TorsionProjectorCertificate => (|| {
            let divisor = divisor()?;
            let order = order()?;
            let certificate = cover
                .torsion_projector_certificate(&divisor, &order)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "order": certificate.order.to_str_radix(10),
                "degree_inverse_mod_order": certificate.degree_inverse_mod_order.to_str_radix(10),
                "input": divisor_to_wire(&certificate.input),
                "normalized_projection": divisor_to_wire(&certificate.normalized_projection),
                "complementary_component": divisor_to_wire(&certificate.complementary_component),
                "projection_is_idempotent": certificate.projection_is_idempotent,
                "projected_component_is_in_image": certificate.projected_component_is_in_image,
                "complementary_component_is_in_pushforward_kernel": certificate.complementary_component_is_in_pushforward_kernel,
                "input_is_in_image": certificate.input_is_in_image,
                "input_is_in_pushforward_kernel": certificate.input_is_in_pushforward_kernel,
                "decomposition_holds": certificate.decomposition_holds,
                "exact_order_verified_by_command": false,
            }))
        })(),
        TransferOperation::Pullback => (|| {
            let point = point()?;
            let pullback = cover
                .pullback_point_class(&point)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": binary_point_to_json(&point),
                "pullback": divisor_to_wire(&pullback),
            }))
        })(),
        TransferOperation::PullPushCompositionCertificate => (|| {
            let point = point()?;
            let certificate = cover
                .pull_push_composition_certificate(&point)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": binary_point_to_json(&certificate.input),
                "pullback": divisor_to_wire(&certificate.pullback),
                "fiber_kind": fiber_kind_to_json(&certificate.fiber_kind),
                "formal_pushforward": binary_point_to_json(&certificate.formal_pushforward),
                "three_times_input": binary_point_to_json(&certificate.three_times_input),
                "identity_holds": certificate.identity_holds,
            }))
        })(),
        TransferOperation::PullbackClosedPointCertificate => (|| {
            let point = point()?;
            let certificate = cover
                .pullback_closed_point_certificate(&point)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "input": binary_point_to_json(&certificate.input),
                "pullback": divisor_to_wire(&certificate.pullback),
                "fiber_kind": fiber_kind_to_json(&certificate.fiber_kind),
                "components": closed_point_components_to_json(&certificate.components),
                "reduced_support_degree": certificate.reduced_support_degree,
                "component_image_sum": binary_point_to_json(&certificate.component_image_sum),
                "three_times_input": binary_point_to_json(&certificate.three_times_input),
                "every_component_is_degree_times_input": certificate.every_component_is_degree_times_input,
                "total_matches_three_times_input": certificate.total_matches_three_times_input,
            }))
        })(),
        TransferOperation::GeneratedSubgroupCertificate => (|| {
            let generator = point()?;
            let annihilator = order()?;
            let certificate = cover
                .generated_subgroup_transfer_certificate(&generator, &annihilator)
                .map_err(|error| error.to_string())?;
            Ok(json!({
                "claim_scope": certificate.claim_scope,
                "generator": binary_point_to_json(&certificate.generator),
                "annihilator": certificate.annihilator.to_str_radix(10),
                "map_degree": certificate.map_degree,
                "degree_inverse_mod_annihilator": certificate.degree_inverse_mod_annihilator.to_str_radix(10),
                "checks": {
                    "generator_is_on_target": certificate.generator_is_on_target,
                    "generator_is_nonidentity": certificate.generator_is_nonidentity,
                    "generator_is_annihilated": certificate.generator_is_annihilated,
                    "pullback_is_nonidentity": certificate.pullback_is_nonidentity,
                    "pullback_is_annihilated": certificate.pullback_is_annihilated,
                    "pull_push_composition_holds": certificate.composition.identity_holds,
                    "arbitrary_norm_matches_composition": certificate.arbitrary_norm_matches_composition,
                    "recovery_matches_generator": certificate.recovery_matches_generator,
                },
                "pullback": divisor_to_wire(&certificate.pullback),
                "pushforward": binary_point_to_json(&certificate.arbitrary_pushforward),
                "three_times_generator": binary_point_to_json(&certificate.composition.three_times_input),
                "recovered_generator": binary_point_to_json(&certificate.recovered_generator),
                "exact_order_or_primality_claimed": certificate.exact_order_or_primality_claimed,
                "proof_reference": "research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean#injective_with_recovery",
            }))
        })(),
        TransferOperation::DeckTransform | TransferOperation::PushPullCompositionCertificate => {
            unreachable!("odd-prime-only operations return before binary evaluation")
        }
        TransferOperation::SubgroupStatus => (|| {
            let order = order()?;
            let status = cover
                .subgroup_transfer_status(&order)
                .map_err(|error| error.to_string())?;
            Ok(match status {
                SubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
                    order,
                    degree_inverse_mod_order,
                } => json!({
                    "status": "injective_if_exact_subgroup_order",
                    "order": order.to_str_radix(10),
                    "degree_inverse_mod_order": degree_inverse_mod_order.to_str_radix(10),
                    "exact_order_verified_by_command": false,
                }),
                SubgroupTransferStatus::KernelIntersectionUnresolved {
                    order,
                    common_factor,
                } => json!({
                    "status": "kernel_intersection_unresolved",
                    "order": order.to_str_radix(10),
                    "common_factor": common_factor,
                    "exact_order_verified_by_command": false,
                }),
            })
        })(),
    };

    let operation = transfer_operation_name(request.operation);
    let scope = if matches!(
        request.operation,
        TransferOperation::GeneratedSubgroupCertificate
    ) {
        "checked injectivity and recovery on the named generated subgroup from a verified annihilator coprime to degree 3; exact order, primality, solver, and comparative-cost claims are outside this command"
    } else {
        "checked induced map only; exact subgroup-order, injectivity, solver, and comparative-cost claims are outside this command"
    };
    match computed {
        Ok(result) => (
            response(
                "transfer",
                "ok",
                json!({
                    "operation": operation,
                    "context": {
                        "family": "binary_cubic_pullback_v1",
                        "target_model": request.model,
                        "curve_construction": curve_construction,
                        "field_degree": irreducible.degree,
                        "source_genus": cover.certificate().source_genus,
                        "map_degree": cover.certificate().map_degree,
                        "map": {"x": "u^3", "y": "u*v+d", "d": format!("0x{:x}", cover.square_root_b().to_biguint())},
                    },
                    "result": result,
                    "scope": scope,
                }),
            ),
            0,
        ),
        Err(reason) => (
            response(
                "transfer",
                "invalid_input",
                json!({"operation": operation, "reason": reason}),
            ),
            3,
        ),
    }
}

fn affine_certificate_point(point: &BinaryPoint) -> Result<Value, String> {
    match point {
        BinaryPoint::Affine { x, y } => Ok(json!({
            "x": format!("0x{:x}", x.to_biguint()),
            "y": format!("0x{:x}", y.to_biguint()),
        })),
        BinaryPoint::Infinity => {
            Err("generated-subgroup certificate unexpectedly produced the target identity".into())
        }
    }
}

fn unknown_exact_order_evidence() -> Value {
    json!({
        "annihilator_primality": {"status": "unknown", "evidence_ref": null},
        "generated_subgroup_cardinality": {"status": "unknown", "evidence_ref": null},
    })
}

fn exact_order_evidence(annihilator: &BigUint) -> Value {
    // A proved-prime annihilator, a checked nonidentity generator, and
    // [r]G=O establish ord(G)=r. This narrow statement does not assert the
    // ambient curve order.
    if matches!(
        prime_proof_status(annihilator),
        PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
    ) {
        let reference = "Sorenson-Webster-Theorem-1.1-psi13;strong-Miller-Rabin-first-16-prime-bases-through-53;exclusive-bound-3317044064679887385961981;src/prime_hyperelliptic/primality.rs#prime_proof_status;named-generator-nonidentity-and-annihilation-checks";
        return json!({
            "annihilator_primality": {"status": "proved", "evidence_ref": reference},
            "generated_subgroup_cardinality": {"status": "proved", "evidence_ref": reference},
        });
    }

    unknown_exact_order_evidence()
}

fn named_subgroup_record(
    cover: &OrdinaryBinaryCover,
    irreducible: &crypto_lib::binary_ecc::IrreduciblePoly,
    representation: &Value,
) -> Result<Value, String> {
    if representation["field"]["element_encoding"]
        != "hex polynomial coefficient bitset, least significant bit is constant"
    {
        return Err("unsupported binary generator coordinate encoding".into());
    }
    let coordinates = representation["curve"]["generator"]
        .as_array()
        .filter(|coordinates| coordinates.len() == 2)
        .ok_or_else(|| "binary representation lacks an affine generator pair".to_string())?;
    let x = parse_binary_field_element(
        coordinates[0]
            .as_str()
            .ok_or_else(|| "binary generator x is not a string".to_string())?,
    )?;
    let y = parse_binary_field_element(
        coordinates[1]
            .as_str()
            .ok_or_else(|| "binary generator y is not a string".to_string())?,
    )?;
    if x.bits() > irreducible.degree as u64 || y.bits() > irreducible.degree as u64 {
        return Err("binary generator coordinate is not canonical for the declared field".into());
    }
    let generator = BinaryPoint::Affine {
        x: F2mElement::from_biguint(&x, irreducible.degree),
        y: F2mElement::from_biguint(&y, irreducible.degree),
    };
    let annihilator = representation["curve"]["subgroup_order"]
        .as_str()
        .ok_or_else(|| "binary representation lacks a decimal subgroup order".to_string())
        .and_then(parse_number)?;
    let certificate = cover
        .generated_subgroup_transfer_certificate(&generator, &annihilator)
        .map_err(|error| error.to_string())?;
    let pullback_witness = serde_json::to_value(divisor_to_wire(&certificate.pullback))
        .map_err(|error| error.to_string())?;

    Ok(json!({
        "status": "supported_on_named_generated_subgroup",
        "claim_scope": "named_generated_subgroup_only",
        "ec1": representation["ec1"],
        "curve_uid": representation["curve_uid"],
        "generator": affine_certificate_point(&certificate.generator)?,
        "annihilator": certificate.annihilator.to_str_radix(10),
        "map_degree": certificate.map_degree,
        "gcd_degree_annihilator": 1,
        "degree_inverse_mod_annihilator": certificate.degree_inverse_mod_annihilator.to_str_radix(10),
        "pullback_witness": pullback_witness,
        "typed_pushforward": affine_certificate_point(&certificate.arbitrary_pushforward)?,
        "three_times_generator": affine_certificate_point(&certificate.composition.three_times_input)?,
        "checks": {
            "target_generator_on_curve": certificate.generator_is_on_target,
            "target_generator_nonidentity": certificate.generator_is_nonidentity,
            "annihilator_kills_target_generator": certificate.generator_is_annihilated,
            "pullback_nonidentity": certificate.pullback_is_nonidentity,
            "annihilator_kills_pullback": certificate.pullback_is_annihilated,
            "pull_push_equals_degree_times_generator": certificate.composition.identity_holds,
            "arbitrary_norm_matches_typed_pushforward": certificate.arbitrary_norm_matches_composition,
            "degree_inverse_recovers_generator": certificate.recovery_matches_generator,
        },
        "exact_order_evidence": exact_order_evidence(&annihilator),
        "failures": [],
    }))
}

fn failed_named_subgroup_record(
    representation: &Value,
    obligation: String,
    reason: String,
) -> Result<Value, String> {
    let ec1 = representation["ec1"]
        .as_str()
        .ok_or_else(|| "failed binary representation lacks EC1".to_string())?;
    let curve_uid = representation["curve_uid"]
        .as_str()
        .ok_or_else(|| format!("{ec1}: failed binary representation lacks curve_uid"))?;
    Ok(json!({
        "status": "failed",
        "claim_scope": "named_generated_subgroup_only",
        "ec1": ec1,
        "curve_uid": curve_uid,
        "exact_order_evidence": unknown_exact_order_evidence(),
        "failures": [{"obligation": obligation, "reason": reason}],
    }))
}

fn binary_transfer_catalog(
    registry_bytes: &[u8],
    graph_bytes: &[u8],
    traits_bytes: &[u8],
    producer_commit: &str,
    rustc_version: &str,
) -> Result<(Value, bool), String> {
    if producer_commit.len() != 40
        || !producer_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err("producer_commit must be exactly forty lowercase hexadecimal digits".into());
    }
    if rustc_version.trim().is_empty() {
        return Err("rustc_version must not be empty".into());
    }

    let registry: Value = serde_json::from_slice(registry_bytes)
        .map_err(|error| format!("parse registry: {error}"))?;
    let geometry = checker::catalog(registry_bytes)?;
    let graph: Value = serde_json::from_slice(graph_bytes)
        .map_err(|error| format!("parse cover graph: {error}"))?;
    let traits: Value =
        serde_json::from_slice(traits_bytes).map_err(|error| format!("parse traits: {error}"))?;
    if graph["schema_version"] != "curve-cover-graph/v1" {
        return Err("unsupported cover graph schema".into());
    }
    if graph["registry_sha256"] != geometry["registry_sha256"] {
        return Err("registry and cover graph digests differ".into());
    }
    let expected_graph = links::graph(&registry, &geometry)
        .map_err(|error| format!("rebuild cover graph from registry: {error}"))?;
    if graph != expected_graph {
        return Err(
            "cover graph is not the exact deterministic graph rebuilt from the registry".into(),
        );
    }

    if traits["schema"] != "curve-traits/3" {
        return Err("unsupported traits schema".into());
    }
    let mut trait_slugs = std::collections::BTreeSet::new();
    for row in traits["curves"]
        .as_array()
        .ok_or_else(|| "traits document lacks a curves array".to_string())?
    {
        let slug = row["slug"]
            .as_str()
            .ok_or_else(|| "trait row lacks a slug".to_string())?;
        if !trait_slugs.insert(slug.to_owned()) {
            return Err(format!("duplicate trait row for {slug}"));
        }
    }

    let curves = graph["curves"]
        .as_array()
        .ok_or_else(|| "cover graph lacks a curves array".to_string())?;
    let graph_maps = graph["maps"]
        .as_object()
        .ok_or_else(|| "cover graph lacks a map object".to_string())?;
    let graph_covers = graph["covers"]
        .as_object()
        .ok_or_else(|| "cover graph lacks a cover object".to_string())?;
    let mut records = Vec::new();
    let mut map_uids = std::collections::BTreeSet::new();
    let mut ec1_uids = std::collections::BTreeSet::new();
    let mut maps_with_named_subgroups = 0usize;
    let mut maps_without_named_subgroups = 0usize;
    let mut named_subgroup_records = 0usize;
    let mut supported_named_subgroup_records = 0usize;
    let mut failed_named_subgroup_records = 0usize;

    for row in curves {
        let model_json = row["model_json"]
            .as_str()
            .ok_or_else(|| "cover graph row lacks model_json".to_string())?;
        let model_value: Value = serde_json::from_str(model_json)
            .map_err(|error| format!("parse cover graph model: {error}"))?;
        if model_value["form"] != "y^2+xy=x^3+a*x^2+b" {
            continue;
        }
        let slug = row["icv1_identity"]["slug"]
            .as_str()
            .ok_or_else(|| "binary graph row lacks an ICV1 slug".to_string())?;
        let model_sha256 = checker::digest(model_json.as_bytes());
        if row["model_sha256"] != model_sha256 {
            return Err(format!("{slug}: model digest mismatch"));
        }
        let links = row["cover_links"]
            .as_array()
            .filter(|links| links.len() == 1)
            .ok_or_else(|| format!("{slug}: binary row must have exactly one cover link"))?;
        let cover_uid = links[0]["cover_uid"]
            .as_str()
            .ok_or_else(|| format!("{slug}: cover link lacks cover_uid"))?;
        let map_uid = links[0]["map_uid"]
            .as_str()
            .ok_or_else(|| format!("{slug}: cover link lacks map_uid"))?;
        if !map_uids.insert(map_uid.to_owned()) {
            return Err(format!("{slug}: duplicate binary map UID {map_uid}"));
        }
        if !graph_covers.contains_key(cover_uid) || !graph_maps.contains_key(map_uid) {
            return Err(format!(
                "{slug}: cover/map UID is absent from graph objects"
            ));
        }
        if graph_maps[map_uid]["record"]["source_cover_uid"] != cover_uid
            || graph_maps[map_uid]["record"]["target_model_sha256"] != model_sha256
            || graph_maps[map_uid]["record"]["degree"] != 3
        {
            return Err(format!(
                "{slug}: linked map record does not match the curve row"
            ));
        }

        let model = match strict_model(&model_value) {
            Ok(model) => model,
            Err(checker::ModelError::Unsupported(reason))
            | Err(checker::ModelError::Invalid(reason)) => return Err(format!("{slug}: {reason}")),
        };
        let checker::Field::Binary(irreducible) = &model.field else {
            return Err(format!(
                "{slug}: binary form did not produce a binary field"
            ));
        };
        let representations = row["representations"]
            .as_array()
            .ok_or_else(|| format!("{slug}: representations is not an array"))?;
        named_subgroup_records += representations.len();
        if representations.is_empty() {
            maps_without_named_subgroups += 1;
        } else {
            maps_with_named_subgroups += 1;
        }

        let mut named_subgroups = Vec::new();
        let mut failures = Vec::new();
        match OrdinaryBinaryCover::new(
            irreducible.degree,
            irreducible.clone(),
            F2mElement::from_biguint(&model.a, irreducible.degree),
            F2mElement::from_biguint(&model.b, irreducible.degree),
        ) {
            Ok(cover) => {
                cover
                    .verify_certificate()
                    .map_err(|error| format!("{slug}: cover replay failed: {error}"))?;
                for representation in representations {
                    let ec1 = representation["ec1"]
                        .as_str()
                        .ok_or_else(|| format!("{slug}: representation lacks EC1"))?;
                    if !ec1_uids.insert(ec1.to_owned()) {
                        return Err(format!("duplicate binary EC1 binding {ec1}"));
                    }
                    match named_subgroup_record(&cover, irreducible, representation) {
                        Ok(record) => {
                            supported_named_subgroup_records += 1;
                            named_subgroups.push(record);
                        }
                        Err(reason) => {
                            failed_named_subgroup_records += 1;
                            named_subgroups.push(failed_named_subgroup_record(
                                representation,
                                format!("generated_subgroup_certificate:{ec1}"),
                                reason.clone(),
                            )?);
                            failures.push(json!({
                                "obligation": format!("generated_subgroup_certificate:{ec1}"),
                                "reason": reason,
                            }));
                        }
                    }
                }
            }
            Err(error) => {
                failed_named_subgroup_records += representations.len();
                for representation in representations {
                    let ec1 = representation["ec1"]
                        .as_str()
                        .ok_or_else(|| format!("{slug}: representation lacks EC1"))?;
                    if !ec1_uids.insert(ec1.to_owned()) {
                        return Err(format!("duplicate binary EC1 binding {ec1}"));
                    }
                    named_subgroups.push(failed_named_subgroup_record(
                        representation,
                        "binary_cover_construction".into(),
                        error.to_string(),
                    )?);
                }
                failures.push(json!({
                    "obligation": "binary_cover_construction",
                    "reason": error.to_string(),
                }));
            }
        }

        let status = if !failures.is_empty() {
            "failed"
        } else if representations.is_empty() {
            "not_applicable_no_named_subgroup"
        } else {
            "supported_on_all_named_generated_subgroups"
        };
        records.push(json!({
            "status": status,
            "icv1_identity": row["icv1_identity"],
            "model_sha256": model_sha256,
            "model_json": model_json,
            "cover_uid": cover_uid,
            "map_uid": map_uid,
            "map_degree": 3,
            "named_subgroups": named_subgroups,
            "failures": failures,
        }));
    }

    if records.is_empty() {
        return Err("cover graph contains no binary map rows".into());
    }
    if supported_named_subgroup_records + failed_named_subgroup_records != named_subgroup_records {
        return Err("internal named-subgroup accounting mismatch".into());
    }
    let complete = failed_named_subgroup_records == 0
        && records.iter().all(|record| record["status"] != "failed");
    Ok((
        json!({
            "schema_version": TRANSFER_CATALOG_SCHEMA,
            "generated_by": "hyperelliptic-cover transfer-catalog-export",
            "producer": {
                "repository": "aburan28/crypto",
                "git_commit": producer_commit,
                "source_sha256": transfer_catalog_producer_source_digest(),
                "command": "hyperelliptic-cover transfer-catalog-export --registry docs/curves/registry.json --cover-graph docs/curves/cover-links.yaml --traits docs/curves/traits.json",
                "rustc_version": rustc_version,
                "attestation": "caller_supplied_unverified",
            },
            "inputs": {
                "registry": {"path": "docs/curves/registry.json", "sha256": checker::digest(registry_bytes)},
                "cover_graph": {"path": "docs/curves/cover-links.yaml", "sha256": checker::digest(graph_bytes)},
                "traits": {"path": "docs/curves/traits.json", "sha256": checker::digest(traits_bytes)},
            },
            "scope": {
                "family": "binary_cubic_pullback_v1",
                "field_relation": "same_field",
                "source_infinity": "one_base_field_rational_point",
                "map_degree": 3,
                "supported_claim": "injective_on_each_named_generated_subgroup_with_checked_annihilator_coprime_to_3",
                "exact_cardinality_not_implied": true,
                "traits_role": "advisory_provenance_only_not_used_for_exact_order",
                "solver_assessment": "not_provided",
                "computational_advantage": "not_assessed",
            },
            "summary": {
                "binary_maps": records.len(),
                "maps_with_named_subgroups": maps_with_named_subgroups,
                "maps_without_named_subgroups": maps_without_named_subgroups,
                "named_subgroup_records": named_subgroup_records,
                "supported_named_subgroup_records": supported_named_subgroup_records,
                "failed_named_subgroup_records": failed_named_subgroup_records,
            },
            "records": records,
        }),
        complete,
    ))
}

fn pretty_json_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn require_exact_catalog_bytes(input: &[u8], expected: &Value) -> Result<(), String> {
    if input != pretty_json_bytes(expected)? {
        return Err(
            "certificate corpus is not the exact canonical byte serialization regenerated from the supplied inputs"
                .into(),
        );
    }
    Ok(())
}

fn verify_binary_transfer_catalog(
    corpus_bytes: &[u8],
    registry_bytes: &[u8],
    graph_bytes: &[u8],
    traits_bytes: &[u8],
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
        let actual_bytes = corpus_bytes.len() as u64;
        if expected_bytes != Some(actual_bytes) {
            return Err(format!(
                "raw certificate corpus byte-length mismatch: expected {}, got {actual_bytes}",
                expected_bytes.expect("paired option checked")
            ));
        }
    }

    let saved: Value = serde_json::from_slice(corpus_bytes)
        .map_err(|error| format!("parse certificate corpus: {error}"))?;
    if saved["schema_version"] != TRANSFER_CATALOG_SCHEMA {
        return Err("unsupported certificate corpus schema".into());
    }
    let producer_commit = saved["producer"]["git_commit"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks git_commit".to_string())?;
    let rustc_version = saved["producer"]["rustc_version"]
        .as_str()
        .ok_or_else(|| "certificate corpus producer lacks rustc_version".to_string())?;
    let (regenerated, complete) = binary_transfer_catalog(
        registry_bytes,
        graph_bytes,
        traits_bytes,
        producer_commit,
        rustc_version,
    )?;
    require_exact_catalog_bytes(corpus_bytes, &regenerated)?;

    Ok((
        json!({
            "schema_version": TRANSFER_CATALOG_VERIFY_SCHEMA,
            "status": if complete { "verified_complete" } else { "verified_incomplete" },
            "corpus": {
                "sha256": corpus_sha256,
                "bytes": corpus_bytes.len(),
                "schema_version": TRANSFER_CATALOG_SCHEMA,
                "canonical_pretty_json_with_trailing_newline": true,
            },
            "producer": {
                "git_commit": producer_commit,
                "rustc_version": rustc_version,
                "source_sha256": transfer_catalog_producer_source_digest(),
                "attestation": "caller_supplied_unverified",
            },
            "inputs": {
                "registry_sha256": checker::digest(registry_bytes),
                "cover_graph_sha256": checker::digest(graph_bytes),
                "traits_sha256": checker::digest(traits_bytes),
            },
            "checks": {
                "optional_raw_hash_and_size": expected_sha256.is_some(),
                "schema_and_metadata_reparsed": true,
                "cover_graph_rebuilt_exactly": true,
                "all_maps_and_named_generators_replayed": true,
                "complete_value_regenerated": true,
                "canonical_bytes_equal": true,
                "all_records_supported": complete,
            },
            "summary": regenerated["summary"],
            "claim_boundary": "exact corpus replay establishes the recorded construction, map, and named-generated-subgroup checks; it does not establish exact orders marked unknown, a Jacobian solver, or a DLP advantage",
        }),
        complete,
    ))
}

fn execute(command: Command) -> (Value, i32, Option<PathBuf>) {
    match command {
        Command::Construct { input, output } => match parse_json(&input) {
            Ok(model) => {
                let (value, code) = construct(model);
                (value, code, output)
            }
            Err(reason) => (
                response("construct", "invalid_input", json!({"reason": reason})),
                3,
                output,
            ),
        },
        Command::Verify { input, output } => {
            let parsed = read_bytes(&input).and_then(|bytes| {
                serde_json::from_slice::<VerifyRequest>(&bytes)
                    .map_err(|e| format!("parse verification request: {e}"))
            });
            match parsed {
                Ok(request) => {
                    let (value, code) = verify(request);
                    (value, code, output)
                }
                Err(reason) => (
                    response("verify", "invalid_input", json!({"reason": reason})),
                    3,
                    output,
                ),
            }
        }
        Command::Arithmetic { input, output } => {
            let parsed = read_bytes(&input).and_then(|bytes| {
                serde_json::from_slice::<ArithmeticRequest>(&bytes)
                    .map_err(|e| format!("parse arithmetic request: {e}"))
            });
            match parsed {
                Ok(request) => {
                    let (value, code) = arithmetic(request);
                    (value, code, output)
                }
                Err(reason) => (
                    response("arithmetic", "invalid_input", json!({"reason": reason})),
                    3,
                    output,
                ),
            }
        }
        Command::Transfer { input, output } => {
            let parsed = read_bytes(&input).and_then(|bytes| {
                serde_json::from_slice::<TransferRequest>(&bytes)
                    .map_err(|e| format!("parse transfer request: {e}"))
            });
            match parsed {
                Ok(request) => {
                    let (value, code) = transfer(request);
                    (value, code, output)
                }
                Err(reason) => (
                    response("transfer", "invalid_input", json!({"reason": reason})),
                    3,
                    output,
                ),
            }
        }
        Command::CatalogExport { registry, output } => match read_bytes(&registry) {
            Ok(bytes) => match checker::catalog(&bytes) {
                Ok(mut catalog) => {
                    catalog["generated_by"] = json!("hyperelliptic-cover catalog-export");
                    catalog["producer_source_sha256"] = json!(catalog_producer_source_digest());
                    (catalog, 0, output)
                }
                Err(reason) => (
                    response("catalog_export", "invalid_input", json!({"reason": reason})),
                    3,
                    output,
                ),
            },
            Err(reason) => (
                response("catalog_export", "invalid_input", json!({"reason": reason})),
                3,
                output,
            ),
        },
        Command::TransferCatalogExport {
            registry,
            cover_graph,
            traits,
            producer_commit,
            rustc_version,
            output,
        } => {
            let generated = read_bytes(&registry)
                .and_then(|registry_bytes| {
                    read_bytes_with_limit(&cover_graph, MAX_CATALOG_INPUT_BYTES)
                        .map(|graph_bytes| (registry_bytes, graph_bytes))
                })
                .and_then(|(registry_bytes, graph_bytes)| {
                    read_bytes(&traits)
                        .map(|traits_bytes| (registry_bytes, graph_bytes, traits_bytes))
                })
                .and_then(|(registry_bytes, graph_bytes, traits_bytes)| {
                    binary_transfer_catalog(
                        &registry_bytes,
                        &graph_bytes,
                        &traits_bytes,
                        &producer_commit,
                        &rustc_version,
                    )
                });
            match generated {
                Ok((catalog, complete)) => (catalog, if complete { 0 } else { 1 }, output),
                Err(reason) => (
                    response(
                        "transfer_catalog_export",
                        "invalid_input",
                        json!({"reason": reason}),
                    ),
                    3,
                    output,
                ),
            }
        }
        Command::TransferCatalogVerify {
            input,
            registry,
            cover_graph,
            traits,
            expected_sha256,
            expected_bytes,
            output,
        } => {
            let verified = read_bytes_with_limit(&input, MAX_CATALOG_INPUT_BYTES)
                .and_then(|corpus_bytes| {
                    read_bytes(&registry).map(|registry_bytes| (corpus_bytes, registry_bytes))
                })
                .and_then(|(corpus_bytes, registry_bytes)| {
                    read_bytes_with_limit(&cover_graph, MAX_CATALOG_INPUT_BYTES)
                        .map(|graph_bytes| (corpus_bytes, registry_bytes, graph_bytes))
                })
                .and_then(|(corpus_bytes, registry_bytes, graph_bytes)| {
                    read_bytes(&traits).map(|traits_bytes| {
                        (corpus_bytes, registry_bytes, graph_bytes, traits_bytes)
                    })
                })
                .and_then(
                    |(corpus_bytes, registry_bytes, graph_bytes, traits_bytes)| {
                        verify_binary_transfer_catalog(
                            &corpus_bytes,
                            &registry_bytes,
                            &graph_bytes,
                            &traits_bytes,
                            expected_sha256.as_deref(),
                            expected_bytes,
                        )
                    },
                );
            match verified {
                Ok((verification, complete)) => {
                    (verification, if complete { 0 } else { 1 }, output)
                }
                Err(reason) => (
                    response(
                        "transfer_catalog_verify",
                        "invalid_input",
                        json!({"reason": reason}),
                    ),
                    3,
                    output,
                ),
            }
        }
    }
}

fn write_value(value: &Value, output: Option<PathBuf>) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())? + "\n";
    if let Some(path) = output {
        fs::write(&path, text).map_err(|e| format!("write {}: {e}", path.display()))
    } else {
        std::io::stdout()
            .lock()
            .write_all(text.as_bytes())
            .map_err(|e| format!("write stdout: {e}"))
    }
}

fn main() {
    let args = Args::parse();
    let (value, mut code, output) = execute(args.command);
    if let Err(reason) = write_value(&value, output) {
        eprintln!(
            "{}",
            serde_json::to_string(&response("write", "error", json!({"reason": reason})))
                .expect("static response serializes")
        );
        code = 1;
    }
    if code != 0 {
        std::process::exit(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::binary_ecc::{curve::scalar_mul as binary_scalar_mul, BinaryCurve};

    const CURRENT_REGISTRY: &str = include_str!("../../docs/curves/registry.json");
    const CURRENT_COVER_GRAPH: &str = include_str!("../../docs/curves/cover-links.yaml");
    const CURRENT_TRAITS: &str = include_str!("../../docs/curves/traits.json");

    fn binary_model() -> Value {
        let modulus = BigUint::from(0xbu32);
        let hash = checker::digest(format!("f2m-modulus:0x{modulus:x}").as_bytes());
        json!({
            "v": "1",
            "form": "y^2+xy=x^3+a*x^2+b",
            "modulus": "0xb",
            "field": format!("f2m-3-{}", &hash[..8]),
            "a": "1",
            "b": "1",
        })
    }

    fn fixed_f8_arbitrary_pushforward_model() -> Value {
        let mut model = binary_model();
        model["b"] = json!("3");
        model
    }

    fn binary_catalog_order_29_model() -> Value {
        let modulus = BigUint::from(0x83u32);
        let hash = checker::digest(format!("f2m-modulus:0x{modulus:x}").as_bytes());
        json!({
            "v": "1",
            "form": "y^2+xy=x^3+a*x^2+b",
            "modulus": "0x83",
            "field": format!("f2m-7-{}", &hash[..8]),
            "a": "0x0",
            "b": "0x1",
        })
    }

    fn prime_f5_model() -> Value {
        json!({
            "v": "1",
            "form": "y^2=x^3+a*x+b",
            "p": "5",
            "field": "fp-5",
            "a": "0",
            "b": "1",
        })
    }

    fn prime_p73_model() -> Value {
        json!({
            "v": "1",
            "form": "y^2=x^3+a*x+b",
            "p": "5902977611310272157103",
            "field": "fp-5902977611310272157103",
            "a": "4168100599208664723916",
            "b": "240650161999037164785",
        })
    }

    #[test]
    fn construction_has_separate_verified_claims() {
        let (answer, code) = construct(binary_model());
        assert_eq!(code, 0);
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["verification"]["defining_equation_identity"],
            "verified_exact_polynomial_identity"
        );
        assert_eq!(
            answer["result"]["verification"]["jacobian_arithmetic"],
            "implemented_checked_one_infinity_model"
        );
    }

    #[test]
    fn exact_fixed_f8_receipt_marks_general_arbitrary_pushforward_and_cli() {
        let (answer, code) = construct(fixed_f8_arbitrary_pushforward_model());
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["verification"]["arbitrary_jacobian_pushforward"],
            "implemented_checked_closed_point_norm_for_valid_reduced_degree_at_most_3_classes_on_this_binary_model"
        );
        assert_eq!(
            answer["result"]["verification"]["cli_transfer_subcommand"],
            "exposed_structured_binary_and_direct_odd_prime"
        );
    }

    #[test]
    fn non_fixture_binary_receipt_marks_general_arbitrary_pushforward_and_cli() {
        let (answer, code) = construct(binary_model());
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["verification"]["arbitrary_jacobian_pushforward"],
            "implemented_checked_closed_point_norm_for_valid_reduced_degree_at_most_3_classes_on_this_binary_model"
        );
        assert_eq!(
            answer["result"]["verification"]["cli_transfer_subcommand"],
            "exposed_structured_binary_and_direct_odd_prime"
        );
    }

    #[test]
    fn catalog_producer_digest_covers_checker_models_and_wrapper() {
        let expected = checker::digest(
            [
                include_str!("curve_cover_check/checker.rs"),
                include_str!("curve_cover_check/models.rs"),
                include_str!("hyperelliptic_cover.rs"),
            ]
            .concat()
            .as_bytes(),
        );
        assert_eq!(catalog_producer_source_digest(), expected);
    }

    #[test]
    fn transfer_catalog_digest_covers_semantics_schema_and_build_pins() {
        let expected = framed_source_digest(&[
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
                "src/bin/hyperelliptic_cover.rs",
                include_str!("hyperelliptic_cover.rs"),
            ),
            (
                "src/binary_ecc/cover_transfer.rs",
                include_str!("../binary_ecc/cover_transfer.rs"),
            ),
            (
                "src/binary_ecc/curve.rs",
                include_str!("../binary_ecc/curve.rs"),
            ),
            (
                "src/binary_ecc/f2m.rs",
                include_str!("../binary_ecc/f2m.rs"),
            ),
            (
                "src/binary_ecc/hyperelliptic.rs",
                include_str!("../binary_ecc/hyperelliptic.rs"),
            ),
            (
                "src/binary_ecc/mod.rs",
                include_str!("../binary_ecc/mod.rs"),
            ),
            (
                "src/binary_ecc/operation_tally.rs",
                include_str!("../binary_ecc/operation_tally.rs"),
            ),
            (
                "src/binary_ecc/poly_f2m.rs",
                include_str!("../binary_ecc/poly_f2m.rs"),
            ),
            (
                "src/cryptanalysis/ecc2k130_guard.rs",
                include_str!("../cryptanalysis/ecc2k130_guard.rs"),
            ),
            (
                "src/cryptanalysis/group_op_count.rs",
                include_str!("../cryptanalysis/group_op_count.rs"),
            ),
            (
                "src/prime_hyperelliptic/fp2.rs",
                include_str!("../prime_hyperelliptic/fp2.rs"),
            ),
            (
                "src/prime_hyperelliptic/mod.rs",
                include_str!("../prime_hyperelliptic/mod.rs"),
            ),
            (
                "src/prime_hyperelliptic/primality.rs",
                include_str!("../prime_hyperelliptic/primality.rs"),
            ),
            ("src/hash/sha256.rs", include_str!("../hash/sha256.rs")),
            ("src/utils/mod.rs", include_str!("../utils/mod.rs")),
            (
                "docs/curves/binary-subgroup-transfer-certificates.schema.json",
                include_str!("../../docs/curves/binary-subgroup-transfer-certificates.schema.json"),
            ),
            ("Cargo.toml", include_str!("../../Cargo.toml")),
            ("Cargo.lock", include_str!("../../Cargo.lock")),
        ]);
        assert_eq!(transfer_catalog_producer_source_digest(), expected);
    }

    #[test]
    fn exact_order_evidence_is_local_and_conservative() {
        let proved = exact_order_evidence(&BigUint::from(29u32));
        assert_eq!(proved["annihilator_primality"]["status"], "proved");
        assert_eq!(proved["generated_subgroup_cardinality"]["status"], "proved");

        let proved_wide =
            exact_order_evidence(&BigUint::parse_bytes(b"5902977611275912361963", 10).unwrap());
        assert_eq!(proved_wide["annihilator_primality"]["status"], "proved");

        let psi_12 =
            exact_order_evidence(&BigUint::parse_bytes(b"318665857834031151167461", 10).unwrap());
        assert_eq!(psi_12["annihilator_primality"]["status"], "unknown");

        let unknown = exact_order_evidence(&BigUint::from(100_000_001u64));
        assert_eq!(unknown["annihilator_primality"]["status"], "unknown");
        assert_eq!(
            unknown["generated_subgroup_cardinality"]["status"],
            "unknown"
        );

        let at_bound =
            exact_order_evidence(&BigUint::parse_bytes(b"3317044064679887385961981", 10).unwrap());
        assert_eq!(at_bound["annihilator_primality"]["status"], "unknown");
    }

    #[test]
    fn transfer_catalog_verify_rejects_invalid_raw_pins_and_schema_before_replay() {
        let corpus = b"{}";
        let empty = b"";
        let digest = checker::digest(corpus);

        let unpaired =
            verify_binary_transfer_catalog(corpus, empty, empty, empty, Some(&digest), None)
                .unwrap_err();
        assert!(unpaired.contains("must be supplied together"));

        let malformed = verify_binary_transfer_catalog(
            corpus,
            empty,
            empty,
            empty,
            Some(&digest.to_ascii_uppercase()),
            Some(corpus.len() as u64),
        )
        .unwrap_err();
        assert!(malformed.contains("lowercase hexadecimal"));

        let wrong_hash = verify_binary_transfer_catalog(
            corpus,
            empty,
            empty,
            empty,
            Some(&"0".repeat(64)),
            Some(corpus.len() as u64),
        )
        .unwrap_err();
        assert!(wrong_hash.contains("SHA-256 mismatch"));

        let wrong_size = verify_binary_transfer_catalog(
            corpus,
            empty,
            empty,
            empty,
            Some(&digest),
            Some(corpus.len() as u64 + 1),
        )
        .unwrap_err();
        assert!(wrong_size.contains("byte-length mismatch"));

        let unsupported =
            verify_binary_transfer_catalog(corpus, empty, empty, empty, None, None).unwrap_err();
        assert!(unsupported.contains("unsupported certificate corpus schema"));
    }

    #[test]
    fn transfer_catalog_rejects_a_tampered_graph_with_a_retained_registry_digest() {
        let model_json = binary_model().to_string();
        let model_sha256 = checker::digest(model_json.as_bytes());
        let registry = json!({
            "schema_version": 1,
            "curves": [{
                "slug": format!("fixture-{}", &model_sha256[..8]),
                "icv1": format!("ICV1-fixture:{}", &model_sha256[..12]),
                "model_json": model_json,
                "order": "1",
                "representations": [],
                "standard_names": [],
                "standards_provenance": [],
            }],
        });
        let registry_bytes = registry.to_string().into_bytes();
        let geometry = checker::catalog(&registry_bytes).expect("fixture geometry");
        let mut graph = links::graph(&registry, &geometry).expect("fixture graph");
        graph["curves"][0]["status"] = json!("tampered_while_digest_retained");
        let error = binary_transfer_catalog(
            &registry_bytes,
            graph.to_string().as_bytes(),
            br#"{"schema":"curve-traits/3","curves":[]}"#,
            &"a".repeat(40),
            "rustc test fixture",
        )
        .expect_err("tampered graph must fail closed");
        assert!(
            error.contains("not the exact deterministic graph"),
            "{error}"
        );
    }

    #[test]
    fn construction_rejects_unknown_model_fields() {
        let mut model = binary_model();
        model
            .as_object_mut()
            .unwrap()
            .insert("unexpected".into(), json!(true));
        let (answer, code) = construct(model);
        assert_eq!(code, 3);
        assert_eq!(answer["status"], "invalid_input");
    }

    #[test]
    fn transfer_pushes_identity_on_non_fixture_binary_model() {
        let request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_model(),
            operation: TransferOperation::PushforwardCertificate,
            divisor: Some(DivisorWire {
                u: vec!["0x1".into()],
                v: vec![],
                n_plus: None,
            }),
            point: None,
            order: None,
        };
        let (answer, code) = transfer(request);
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(answer["result"]["context"]["target_model"], binary_model());
        assert_eq!(
            answer["result"]["context"]["curve_construction"]["construction"],
            "binary_cubic_pullback_v1"
        );
        assert_eq!(
            answer["result"]["result"]["general_pushforward"]["kind"],
            "infinity"
        );
        assert_eq!(
            answer["result"]["result"]["support_reconstruction_holds"],
            true
        );
        assert_eq!(answer["result"]["result"]["component_sum_holds"], true);
    }

    #[test]
    fn transfer_certifies_exceptional_pull_push_and_projector() {
        let point = BinaryPointWire::Affine {
            x: "0x0".into(),
            y: "0x1".into(),
        };
        let composition_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_model(),
            operation: TransferOperation::PullPushCompositionCertificate,
            divisor: None,
            point: Some(point.clone()),
            order: None,
        };
        let (composition, composition_code) = transfer(composition_request);
        assert_eq!(composition_code, 0, "{composition}");
        assert_eq!(composition["result"]["result"]["identity_holds"], true);

        let projector_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_model(),
            operation: TransferOperation::TorsionProjectorCertificate,
            divisor: Some(DivisorWire {
                u: vec!["0x0".into(), "0x1".into()],
                v: vec![],
                n_plus: None,
            }),
            point: None,
            order: Some("2".into()),
        };
        let (projector, projector_code) = transfer(projector_request);
        assert_eq!(projector_code, 0, "{projector}");
        assert_eq!(projector["result"]["result"]["decomposition_holds"], true);
        assert_eq!(
            projector["result"]["result"]["exact_order_verified_by_command"],
            false
        );
    }

    #[test]
    fn transfer_certifies_named_generated_subgroup_from_a_checked_annihilator() {
        let request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_catalog_order_29_model(),
            operation: TransferOperation::GeneratedSubgroupCertificate,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "0x26".into(),
                y: "0x5".into(),
            }),
            // The named point has order 29, so the deliberately composite
            // multiple 58 is a valid annihilator.  The certificate must not
            // promote it to an exact-order or primality statement.
            order: Some("58".into()),
        };
        let (answer, code) = transfer(request);
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        let result = &answer["result"]["result"];
        assert_eq!(result["annihilator"], "58");
        assert_eq!(result["degree_inverse_mod_annihilator"], "39");
        assert_eq!(result["checks"]["generator_is_annihilated"], true);
        assert_eq!(result["checks"]["pull_push_composition_holds"], true);
        assert_eq!(result["checks"]["arbitrary_norm_matches_composition"], true);
        assert_eq!(result["checks"]["recovery_matches_generator"], true);
        assert_eq!(result["exact_order_or_primality_claimed"], false);

        let wrong_annihilator = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_catalog_order_29_model(),
            operation: TransferOperation::GeneratedSubgroupCertificate,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "0x26".into(),
                y: "0x5".into(),
            }),
            order: Some("2".into()),
        };
        let (rejected, rejected_code) = transfer(wrong_annihilator);
        assert_eq!(rejected_code, 3, "{rejected}");
        assert_eq!(rejected["status"], "invalid_input");
    }

    #[test]
    fn transfer_rejects_malformed_divisor_and_wrong_schema() {
        let malformed = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_model(),
            operation: TransferOperation::Pushforward,
            divisor: Some(DivisorWire {
                u: vec![],
                v: vec![],
                n_plus: None,
            }),
            point: None,
            order: None,
        };
        let (answer, code) = transfer(malformed);
        assert_eq!(code, 3, "{answer}");
        assert_eq!(answer["status"], "invalid_input");

        let wrong_schema = TransferRequest {
            schema: "hyperelliptic-transfer-operation/v0".into(),
            model: binary_model(),
            operation: TransferOperation::SubgroupStatus,
            divisor: None,
            point: None,
            order: Some("29".into()),
        };
        let (answer, code) = transfer(wrong_schema);
        assert_eq!(code, 2, "{answer}");
        assert_eq!(answer["status"], "unsupported");
    }

    #[test]
    fn transfer_rejects_operands_irrelevant_to_the_selected_operation() {
        let request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: binary_model(),
            operation: TransferOperation::SubgroupStatus,
            divisor: Some(DivisorWire {
                u: vec!["0x1".into()],
                v: vec![],
                n_plus: None,
            }),
            point: None,
            order: Some("29".into()),
        };
        let (answer, code) = transfer(request);
        assert_eq!(code, 3, "{answer}");
        assert_eq!(answer["status"], "invalid_input");
        assert_eq!(
            answer["result"]["reason"],
            "operation does not accept `divisor`"
        );
    }

    #[test]
    fn every_current_catalog_model_reaches_the_standalone_constructor() {
        let registry: Value = serde_json::from_str(CURRENT_REGISTRY).expect("current registry");
        let rows = registry["curves"].as_array().expect("registry curve array");
        let graph: Value = serde_json::from_str(CURRENT_COVER_GRAPH).expect("current cover graph");
        let graph_rows = graph["curves"].as_array().expect("graph curve array");
        let registry_models: std::collections::BTreeSet<_> = rows
            .iter()
            .map(|row| row["model_json"].as_str().expect("registry model"))
            .collect();
        let graph_models: std::collections::BTreeSet<_> = graph_rows
            .iter()
            .map(|row| row["model_json"].as_str().expect("graph model"))
            .collect();
        assert!(
            !registry_models.is_empty(),
            "current catalog must not be empty"
        );
        assert_eq!(
            registry_models.len(),
            rows.len(),
            "duplicate registry model"
        );
        assert_eq!(
            graph_models.len(),
            graph_rows.len(),
            "duplicate graph model"
        );
        assert_eq!(
            registry_models, graph_models,
            "registry/graph model coverage"
        );
        for row in rows {
            let slug = row["slug"].as_str().expect("catalog slug");
            let model: Value = serde_json::from_str(
                row["model_json"]
                    .as_str()
                    .expect("catalog model_json string"),
            )
            .expect("catalog model JSON");
            let (answer, code) = construct(model);
            assert_eq!(code, 0, "standalone construction rejected {slug}: {answer}");
            assert_eq!(answer["status"], "ok", "unexpected status for {slug}");
        }
    }

    #[test]
    fn transfer_catalog_emits_every_binary_map_and_named_generator() {
        let (catalog, complete) = binary_transfer_catalog(
            CURRENT_REGISTRY.as_bytes(),
            CURRENT_COVER_GRAPH.as_bytes(),
            CURRENT_TRAITS.as_bytes(),
            &"a".repeat(40),
            "rustc test fixture",
        )
        .expect("current transfer catalog");
        assert!(complete, "{catalog}");
        assert_eq!(catalog["schema_version"], TRANSFER_CATALOG_SCHEMA);
        assert_eq!(catalog["summary"]["binary_maps"], 174);
        assert_eq!(catalog["summary"]["maps_with_named_subgroups"], 147);
        assert_eq!(catalog["summary"]["maps_without_named_subgroups"], 27);
        assert_eq!(catalog["summary"]["named_subgroup_records"], 150);
        assert_eq!(catalog["summary"]["supported_named_subgroup_records"], 150);
        assert_eq!(catalog["summary"]["failed_named_subgroup_records"], 0);
        let records = catalog["records"].as_array().expect("map records");
        assert_eq!(records.len(), 174);
        assert_eq!(
            records
                .iter()
                .filter(|record| record["status"] == "supported_on_all_named_generated_subgroups")
                .count(),
            147
        );
        assert_eq!(
            records
                .iter()
                .filter(|record| record["status"] == "not_applicable_no_named_subgroup")
                .count(),
            27
        );
        let subgroup_rows = records
            .iter()
            .flat_map(|record| {
                record["named_subgroups"]
                    .as_array()
                    .expect("named subgroup array")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            subgroup_rows
                .iter()
                .filter(
                    |row| row["exact_order_evidence"]["annihilator_primality"]["status"]
                        == "proved"
                )
                .count(),
            102
        );
        assert_eq!(
            subgroup_rows
                .iter()
                .filter(
                    |row| row["exact_order_evidence"]["annihilator_primality"]["status"]
                        == "unknown"
                )
                .count(),
            48
        );

        let encoded = pretty_json_bytes(&catalog).expect("canonical corpus bytes");
        require_exact_catalog_bytes(&encoded, &catalog).expect("exact replay bytes");
        let mut noncanonical = encoded.clone();
        noncanonical.push(b' ');
        assert!(require_exact_catalog_bytes(&noncanonical, &catalog)
            .unwrap_err()
            .contains("exact canonical byte serialization"));
    }

    #[test]
    fn every_binary_ec1_generator_has_checked_typed_transfer_hypotheses() {
        let graph: Value = serde_json::from_str(CURRENT_COVER_GRAPH).expect("current cover graph");
        let curves = graph["curves"].as_array().expect("graph curve array");
        let registry: Value = serde_json::from_str(CURRENT_REGISTRY).expect("current registry");
        let mut expected = std::collections::BTreeSet::new();
        for row in registry["curves"].as_array().expect("registry curve array") {
            let model = row["model_json"].as_str().expect("registry model");
            let parsed: Value = serde_json::from_str(model).expect("registry model JSON");
            if parsed["form"] == "y^2+xy=x^3+a*x^2+b" {
                for representation in row["representations"]
                    .as_array()
                    .expect("registry representations")
                {
                    let ec1 = representation["ec1"].as_str().expect("registry EC1 alias");
                    assert!(
                        expected.insert((model.to_owned(), ec1.to_owned())),
                        "duplicate registry binding"
                    );
                }
            }
        }
        assert!(
            !expected.is_empty(),
            "binary generator controls must not be empty"
        );
        let mut checked = std::collections::BTreeSet::new();
        let mut binary_maps_checked = 0usize;
        let mut nonsplit_component_degrees = std::collections::BTreeSet::new();
        for row in curves {
            let model_value: Value =
                serde_json::from_str(row["model_json"].as_str().expect("model_json"))
                    .expect("model JSON");
            if model_value["form"] != "y^2+xy=x^3+a*x^2+b" {
                continue;
            }
            let model = match strict_model(&model_value) {
                Ok(model) => model,
                Err(_) => panic!("catalog binary model failed strict parsing"),
            };
            let checker::Field::Binary(irreducible) = &model.field else {
                panic!("binary form did not produce a binary field");
            };
            let cover = OrdinaryBinaryCover::new(
                irreducible.degree,
                irreducible.clone(),
                F2mElement::from_biguint(&model.a, irreducible.degree),
                F2mElement::from_biguint(&model.b, irreducible.degree),
            )
            .expect("checked catalog cover");
            binary_maps_checked += 1;
            let row_slug = row["icv1_identity"]["slug"]
                .as_str()
                .expect("catalog ICV1 slug");

            let identity = MumfordDivisor::identity(irreducible.degree);
            assert_eq!(
                cover
                    .pushforward_arbitrary_jacobian_class(&identity)
                    .expect("identity arbitrary norm"),
                BinaryPoint::Infinity,
                "{}: identity did not push to the target identity",
                row_slug
            );
            let identity_certificate = cover
                .jacobian_pushforward_certificate(&identity)
                .expect("identity component certificate");
            assert!(identity_certificate.components.is_empty());
            assert!(identity_certificate.support_reconstruction_holds);
            assert!(identity_certificate.component_sum_holds);

            let exceptional = MumfordDivisor::try_new(
                cover.source_curve(),
                F2mPoly::x(irreducible.degree),
                F2mPoly::zero(irreducible.degree),
            )
            .expect("exceptional source-point divisor");
            let exceptional_image = cover
                .pushforward_arbitrary_jacobian_class(&exceptional)
                .expect("exceptional arbitrary norm");
            assert_eq!(
                exceptional_image,
                BinaryPoint::Affine {
                    x: F2mElement::zero(irreducible.degree),
                    y: cover.square_root_b().clone(),
                },
                "{}: exceptional divisor norm changed",
                row_slug
            );
            let exceptional_certificate = cover
                .jacobian_pushforward_certificate(&exceptional)
                .expect("exceptional component certificate");
            assert!(exceptional_certificate.support_reconstruction_holds);
            assert!(exceptional_certificate.component_sum_holds);
            assert_eq!(exceptional_certificate.components.len(), 1);

            for representation in row["representations"]
                .as_array()
                .expect("representation array")
            {
                let ec1 = representation["ec1"].as_str().expect("EC1 alias");
                let generator = representation["curve"]["generator"]
                    .as_array()
                    .filter(|coordinates| coordinates.len() == 2)
                    .expect("binary generator pair");
                assert_eq!(
                    representation["field"]["element_encoding"],
                    "hex polynomial coefficient bitset, least significant bit is constant",
                    "{ec1}: unsupported binary coordinate encoding"
                );
                let generator_x =
                    parse_binary_field_element(generator[0].as_str().expect("binary generator x"))
                        .expect("parsed x");
                let generator_y =
                    parse_binary_field_element(generator[1].as_str().expect("binary generator y"))
                        .expect("parsed y");
                assert!(
                    generator_x.bits() <= irreducible.degree as u64,
                    "{ec1}: generator x is not canonical for the declared field"
                );
                assert!(
                    generator_y.bits() <= irreducible.degree as u64,
                    "{ec1}: generator y is not canonical for the declared field"
                );
                let point = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&generator_x, irreducible.degree),
                    y: F2mElement::from_biguint(&generator_y, irreducible.degree),
                };
                let order = parse_number(
                    representation["curve"]["subgroup_order"]
                        .as_str()
                        .expect("decimal subgroup order"),
                )
                .expect("positive subgroup order");
                let cofactor = BigUint::from(
                    representation["curve"]["cofactor"]
                        .as_u64()
                        .expect("small cofactor"),
                );
                let target = BinaryCurve {
                    m: irreducible.degree,
                    irreducible: irreducible.clone(),
                    a: F2mElement::from_biguint(&model.a, irreducible.degree),
                    b: F2mElement::from_biguint(&model.b, irreducible.degree),
                    generator: point.clone(),
                    order: order.clone(),
                    cofactor,
                };

                assert!(target.is_on_curve(&point), "{ec1}: generator is off curve");
                assert_eq!(
                    binary_scalar_mul(&target, &point, &order),
                    BinaryPoint::Infinity,
                    "{ec1}: declared order does not annihilate generator"
                );
                let pullback = cover.pullback_point_class(&point).expect("typed pullback");
                assert!(
                    !pullback.is_identity(),
                    "{ec1}: nonidentity generator had identity pullback"
                );
                assert!(
                    pullback
                        .scalar_mul(&order, cover.source_curve())
                        .is_identity(),
                    "{ec1}: catalog order did not annihilate pullback image"
                );
                let composition = cover
                    .pull_push_composition_certificate(&point)
                    .expect("typed composition");
                assert_eq!(
                    composition.pullback, pullback,
                    "{ec1}: composition certificate used a different pullback witness"
                );
                assert!(
                    composition.identity_holds,
                    "{ec1}: typed composition failed"
                );
                let arbitrary_pushforward = cover
                    .pushforward_arbitrary_jacobian_class(&pullback)
                    .expect("general arbitrary-class norm on catalog pullback");
                assert_eq!(
                    arbitrary_pushforward, composition.formal_pushforward,
                    "{ec1}: arbitrary norm differs from typed formal pushforward"
                );
                let arbitrary_certificate = cover
                    .jacobian_pushforward_certificate(&pullback)
                    .expect("catalog arbitrary-class component certificate");
                assert!(
                    arbitrary_certificate.support_reconstruction_holds,
                    "{ec1}: component support reconstruction failed"
                );
                assert!(
                    arbitrary_certificate.component_sum_holds,
                    "{ec1}: component image sum failed"
                );
                assert_eq!(
                    arbitrary_certificate.general_pushforward, composition.formal_pushforward,
                    "{ec1}: certified arbitrary norm differs from typed pushforward"
                );
                let closed_point_certificate = cover
                    .pullback_closed_point_certificate(&point)
                    .expect("catalog closed-point pullback certificate");
                assert!(closed_point_certificate.every_component_is_degree_times_input);
                assert!(closed_point_certificate.total_matches_three_times_input);
                for component in &closed_point_certificate.components {
                    if component.degree > 1 {
                        nonsplit_component_degrees.insert(component.degree);
                    }
                }
                let status = cover
                    .subgroup_transfer_status(&order)
                    .expect("conditional subgroup status");
                let SubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
                    degree_inverse_mod_order,
                    ..
                } = status
                else {
                    panic!("{ec1}: declared order is not coprime to degree three");
                };
                assert_eq!(
                    binary_scalar_mul(
                        &target,
                        &composition.formal_pushforward,
                        &degree_inverse_mod_order,
                    ),
                    point,
                    "{ec1}: degree-inverse recovery failed"
                );
                assert!(
                    checked.insert((
                        row["model_json"].as_str().expect("graph model").to_owned(),
                        ec1.to_owned()
                    )),
                    "{ec1}: duplicate graph binding"
                );
            }
        }
        assert_eq!(
            checked, expected,
            "complete binary registry/graph EC1 coverage"
        );
        assert!(binary_maps_checked > 0, "no binary maps were replayed");
        assert!(
            !nonsplit_component_degrees.is_empty(),
            "catalog generator replay did not exercise any nonsplit closed-point support"
        );
    }

    #[test]
    fn prime_transfer_exposes_both_compositions_and_named_subgroup_recovery() {
        let point = BinaryPointWire::Affine {
            x: "0".into(),
            y: "1".into(),
        };
        let pull_push_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: prime_f5_model(),
            operation: TransferOperation::PullPushCompositionCertificate,
            divisor: None,
            point: Some(point.clone()),
            order: None,
        };
        let (pull_push, pull_push_code) = transfer(pull_push_request);
        assert_eq!(pull_push_code, 0, "{pull_push}");
        assert_eq!(pull_push["result"]["result"]["identity_holds"], true);
        assert_eq!(
            pull_push["result"]["result"]["pushforward"],
            json!({"kind":"affine", "x":"0x0", "y":"0x4"})
        );

        let divisor = DivisorWire {
            u: vec!["0".into(), "1".into()],
            v: vec!["1".into()],
            n_plus: Some(0),
        };
        let push_pull_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: prime_f5_model(),
            operation: TransferOperation::PushPullCompositionCertificate,
            divisor: Some(divisor.clone()),
            point: None,
            order: None,
        };
        let (push_pull, push_pull_code) = transfer(push_pull_request);
        assert_eq!(push_pull_code, 0, "{push_pull}");
        assert_eq!(push_pull["result"]["result"]["identity_holds"], true);

        let deck_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: prime_f5_model(),
            operation: TransferOperation::DeckTransform,
            divisor: Some(divisor),
            point: None,
            order: None,
        };
        let (deck, deck_code) = transfer(deck_request);
        assert_eq!(deck_code, 0, "{deck}");
        assert_eq!(deck["result"]["result"]["deck_transform"]["n_plus"], 1);

        let subgroup_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: prime_f5_model(),
            operation: TransferOperation::GeneratedSubgroupCertificate,
            divisor: None,
            point: Some(point),
            order: Some("3".into()),
        };
        let (subgroup, subgroup_code) = transfer(subgroup_request);
        assert_eq!(subgroup_code, 0, "{subgroup}");
        assert_eq!(subgroup["result"]["result"]["annihilator"], "3");
        assert_eq!(
            subgroup["result"]["result"]["degree_inverse_mod_annihilator"],
            "2"
        );
        assert_eq!(
            subgroup["result"]["result"]["checks"]["recovery_matches_generator"],
            true
        );
        assert_eq!(
            subgroup["result"]["result"]["exact_order_or_primality_claimed"],
            false
        );
    }

    #[test]
    fn prime_transfer_keeps_extension_and_even_order_boundaries_structured() {
        let even_order = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: prime_f5_model(),
            operation: TransferOperation::SubgroupStatus,
            divisor: None,
            point: None,
            order: Some("2".into()),
        };
        let (status, status_code) = transfer(even_order);
        assert_eq!(status_code, 0, "{status}");
        assert_eq!(
            status["result"]["result"]["status"],
            "kernel_intersection_unresolved"
        );

        let label_hash = checker::digest(b"fpk-modulus:5:2,0");
        let extension = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: json!({
                "v":"1", "form":"y^2=x^3+a*x+b", "p":"5", "k":"2",
                "modulus":["2", "0"],
                "field":format!("fpk-5-2-{}", &label_hash[..8]),
                "a":["1", "0"], "b":["1", "0"]
            }),
            operation: TransferOperation::SubgroupStatus,
            divisor: None,
            point: None,
            order: Some("3".into()),
        };
        let (unsupported, unsupported_code) = transfer(extension);
        assert_eq!(unsupported_code, 2, "{unsupported}");
        assert_eq!(unsupported["status"], "unsupported");
        assert_eq!(
            unsupported["result"]["transfer_status"],
            "unsupported_extension_field_balanced_backend"
        );
    }

    #[test]
    fn strict_psi13_wide_prime_reaches_arithmetic_and_transfer_but_fails_closed_above_bound() {
        let model = prime_p73_model();
        let (constructed, construct_code) = construct(model.clone());
        assert_eq!(construct_code, 0, "{constructed}");
        assert_eq!(
            constructed["result"]["verification"]["prime_proof"]["status"],
            "proved_psi13"
        );
        assert_eq!(
            constructed["result"]["verification"]["jacobian_arithmetic"],
            "implemented_checked_balanced_split_sextic_with_deterministic_prime_revalidation"
        );

        let arithmetic_request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(model.clone()),
            odd_curve: None,
            operation: ArithmeticOperation::Identity,
            left: None,
            right: None,
            scalar: None,
        };
        let (arithmetic_answer, arithmetic_code) = arithmetic(arithmetic_request);
        assert_eq!(arithmetic_code, 0, "{arithmetic_answer}");
        assert_eq!(
            arithmetic_answer["result"]["context"]["prime_proof"]["status"],
            "proved_psi13"
        );

        let transfer_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model,
            operation: TransferOperation::GeneratedSubgroupCertificate,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "1".into(),
                y: "1565394211633018828840".into(),
            }),
            order: Some("5902977611275912361963".into()),
        };
        let (transfer_answer, transfer_code) = transfer(transfer_request);
        assert_eq!(transfer_code, 0, "{transfer_answer}");
        assert_eq!(
            transfer_answer["result"]["context"]["prime_proof"]["status"],
            "proved_psi13"
        );
        assert_eq!(
            transfer_answer["result"]["result"]["checks"]["recovery_matches_generator"],
            true
        );

        let unsupported_model = json!({
            "v": "1", "form": "y^2=x^3+a*x+b",
            "p": "3317044064679887385962123",
            "field": "fp-3317044064679887385962123",
            "a": "1", "b": "1",
        });
        let unsupported_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: unsupported_model,
            operation: TransferOperation::SubgroupStatus,
            divisor: None,
            point: None,
            order: Some("3".into()),
        };
        let (unsupported, unsupported_code) = transfer(unsupported_request);
        assert_eq!(unsupported_code, 2, "{unsupported}");
        assert_eq!(unsupported["status"], "unsupported");
        assert_eq!(
            unsupported["result"]["transfer_status"],
            "unsupported_without_primality_certificate"
        );
    }

    #[test]
    fn prime_cover_arithmetic_uses_balanced_split_sextic_backend() {
        let request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(json!({
                "v":"1", "form":"y^2=x^3+a*x+b", "p":"11",
                "field":"fp-11", "a":"1", "b":"1"
            })),
            odd_curve: None,
            operation: ArithmeticOperation::Identity,
            left: None,
            right: None,
            scalar: None,
        };
        let (answer, code) = arithmetic(request);
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["context"]["representation"],
            "canonical_balanced_(u,v,n_plus)"
        );
        assert_eq!(answer["result"]["result"]["divisor"]["u"], json!(["0x1"]));
        assert_eq!(answer["result"]["result"]["divisor"]["v"], json!([]));
        assert_eq!(answer["result"]["result"]["divisor"]["n_plus"], 1);
    }

    #[test]
    fn extension_cover_reaches_verified_construction_and_structured_arithmetic_boundary() {
        let label_hash = checker::digest(b"fpk-modulus:5:2,0");
        let model = json!({
            "v":"1", "form":"y^2=x^3+a*x+b", "p":"5", "k":"2",
            "modulus":["2", "0"],
            "field":format!("fpk-5-2-{}", &label_hash[..8]),
            "a":["1", "0"], "b":["1", "0"]
        });
        let (constructed, construct_code) = construct(model.clone());
        assert_eq!(construct_code, 0, "{constructed}");
        assert_eq!(constructed["status"], "ok");

        let request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(model),
            odd_curve: None,
            operation: ArithmeticOperation::Identity,
            left: None,
            right: None,
            scalar: None,
        };
        let (answer, code) = arithmetic(request);
        assert_eq!(code, 2, "{answer}");
        assert_eq!(answer["status"], "unsupported");
        assert_eq!(
            answer["result"]["arithmetic_status"],
            "unsupported_extension_field"
        );
    }

    #[test]
    fn normalized_montgomery_cover_reaches_checked_composed_transfer() {
        let model = json!({
            "v":"1", "form":"B*y^2=x^3+A*x^2+x", "p":"11",
            "field":"fp-11", "A":"3", "B":"1"
        });
        let (answer, code) = construct(model.clone());
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["certificate"]["target_model_map"]["status"],
            "verified_algebraic_identity"
        );
        assert_eq!(
            answer["result"]["verification"]["pullback"],
            "implemented_for_rational_target_point_classes"
        );
        assert_eq!(
            answer["result"]["verification"]["pushforward"],
            "implemented_for_every_canonical_balanced_genus_two_class"
        );
        assert_eq!(
            answer["result"]["verification"]["composition"],
            "verified_pi_push_pi_pull_equals_multiplication_by_2_and_pi_pull_pi_push_equals_1_plus_deck"
        );
        assert_eq!(
            answer["result"]["verification"]["target_model_transfer"],
            "implemented_checked_projective_montgomery_isomorphism"
        );

        // Montgomery (10,1) maps to short (0,1); with the deterministic
        // shift c=0 it exercises the ramified repeated-support fiber U=T^2.
        let supported_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: model.clone(),
            operation: TransferOperation::PullPushCompositionCertificate,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "10".into(),
                y: "1".into(),
            }),
            order: None,
        };
        let (supported, supported_code) = transfer(supported_request);
        assert_eq!(supported_code, 0, "{supported}");
        assert_eq!(supported["result"]["result"]["identity_holds"], true);
        assert_eq!(
            supported["result"]["result"]["input"],
            json!({"kind":"affine", "x":"0xa", "y":"0x1"})
        );
        assert_eq!(
            supported["result"]["result"]["pullback"]["u"],
            json!(["0x0", "0x0", "0x1"])
        );
        assert_eq!(
            supported["result"]["context"]["family"],
            "normalized_prime_quadratic_pullback_v1_split_sextic_balanced_montgomery"
        );

        let divisor = DivisorWire {
            u: vec!["0".into(), "0".into(), "1".into()],
            v: vec!["1".into()],
            n_plus: Some(0),
        };
        let pushforward_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: model.clone(),
            operation: TransferOperation::Pushforward,
            divisor: Some(divisor.clone()),
            point: None,
            order: None,
        };
        let (pushforward, pushforward_code) = transfer(pushforward_request);
        assert_eq!(pushforward_code, 0, "{pushforward}");
        assert_eq!(
            pushforward["result"]["result"]["image"],
            json!({"kind":"affine", "x":"0x0", "y":"0x0"})
        );

        let push_pull_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: model.clone(),
            operation: TransferOperation::PushPullCompositionCertificate,
            divisor: Some(divisor),
            point: None,
            order: None,
        };
        let (push_pull, push_pull_code) = transfer(push_pull_request);
        assert_eq!(push_pull_code, 0, "{push_pull}");
        assert_eq!(push_pull["result"]["result"]["identity_holds"], true);
        assert_eq!(
            push_pull["result"]["result"]["pushforward"],
            json!({"kind":"affine", "x":"0x0", "y":"0x0"})
        );
        assert_eq!(
            push_pull["result"]["result"]["pullback"],
            json!({"u":["0xa", "0x0", "0x1"], "v":[], "n_plus":0})
        );
        assert_eq!(
            push_pull["result"]["result"]["deck_transform"],
            json!({"u":["0x0", "0x0", "0x1"], "v":["0x1"], "n_plus":0})
        );

        let subgroup_request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model: json!({
                "v":"1", "form":"B*y^2=x^3+A*x^2+x", "p":"7",
                "field":"fp-7", "A":"3", "B":"1"
            }),
            operation: TransferOperation::GeneratedSubgroupCertificate,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "4".into(),
                y: "2".into(),
            }),
            order: Some("3".into()),
        };
        let (subgroup, subgroup_code) = transfer(subgroup_request);
        assert_eq!(subgroup_code, 0, "{subgroup}");
        assert_eq!(
            subgroup["result"]["result"]["generator"],
            json!({"kind":"affine", "x":"0x4", "y":"0x2"})
        );
        assert_eq!(
            subgroup["result"]["result"]["degree_inverse_mod_annihilator"],
            "2"
        );
        assert_eq!(
            subgroup["result"]["result"]["pullback"],
            json!({"u":["0x2", "0x0", "0x1"], "v":["0x2"], "n_plus":0})
        );
        assert_eq!(
            subgroup["result"]["result"]["pushforward"],
            json!({"kind":"affine", "x":"0x4", "y":"0x5"})
        );
        assert_eq!(
            subgroup["result"]["result"]["recovered_generator"],
            json!({"kind":"affine", "x":"0x4", "y":"0x2"})
        );
        assert_eq!(
            subgroup["result"]["result"]["checks"]["recovery_matches_generator"],
            true
        );
        assert_eq!(
            subgroup["result"]["result"]["exact_order_or_primality_claimed"],
            false
        );

        // (0,1) lies on the normalized short-Weierstrass curve but not on
        // the original Montgomery target. It must never be accepted under
        // an ambiguous coordinate interpretation.
        let request = TransferRequest {
            schema: TRANSFER_SCHEMA.into(),
            model,
            operation: TransferOperation::Pullback,
            divisor: None,
            point: Some(BinaryPointWire::Affine {
                x: "0".into(),
                y: "1".into(),
            }),
            order: None,
        };
        let (rejected, rejected_code) = transfer(request);
        assert_eq!(rejected_code, 3, "{rejected}");
        assert_eq!(rejected["status"], "invalid_input");
        assert!(rejected["result"]["reason"]
            .as_str()
            .unwrap()
            .contains("not on the Montgomery model"));
    }

    #[test]
    fn malformed_divisor_is_rejected_before_arithmetic() {
        let request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(binary_model()),
            odd_curve: None,
            operation: ArithmeticOperation::Validate,
            left: Some(DivisorWire {
                u: Vec::new(),
                v: Vec::new(),
                n_plus: None,
            }),
            right: None,
            scalar: None,
        };
        let (answer, code) = arithmetic(request);
        assert_eq!(code, 3);
        assert_eq!(answer["status"], "invalid_input");
    }

    #[test]
    fn arithmetic_rejects_operands_irrelevant_to_the_selected_operation() {
        let identity_with_left = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(binary_model()),
            odd_curve: None,
            operation: ArithmeticOperation::Identity,
            left: Some(DivisorWire {
                u: vec!["1".into()],
                v: Vec::new(),
                n_plus: None,
            }),
            right: None,
            scalar: None,
        };
        let (identity, identity_code) = arithmetic(identity_with_left);
        assert_eq!(identity_code, 3, "{identity}");
        assert_eq!(
            identity["result"]["reason"],
            "operation does not accept `left`"
        );

        let add_with_scalar = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(binary_model()),
            odd_curve: None,
            operation: ArithmeticOperation::Add,
            left: Some(DivisorWire {
                u: vec!["1".into()],
                v: Vec::new(),
                n_plus: None,
            }),
            right: Some(DivisorWire {
                u: vec!["1".into()],
                v: Vec::new(),
                n_plus: None,
            }),
            scalar: Some("2".into()),
        };
        let (add, add_code) = arithmetic(add_with_scalar);
        assert_eq!(add_code, 3, "{add}");
        assert_eq!(
            add["result"]["reason"],
            "operation does not accept `scalar`"
        );
    }

    #[test]
    fn odd_one_infinity_branch_divisor_is_supported_separately() {
        let request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(OddCurveWire {
                family: "odd_prime_one_infinity_v1".into(),
                p: "5".into(),
                genus: 2,
                // y^2 = x^5 + x; derivative is 1, so this is squarefree.
                f: ["0", "1", "0", "0", "0", "1"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                infinity: "one_rational".into(),
            }),
            operation: ArithmeticOperation::Validate,
            // The Weierstrass point (0,0) gives the order-two class (x,0).
            left: Some(DivisorWire {
                u: vec!["0".into(), "1".into()],
                v: Vec::new(),
                n_plus: None,
            }),
            right: None,
            scalar: None,
        };
        let (answer, code) = arithmetic(request);
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert!(answer["result"]["result"]["valid"]
            .as_bool()
            .expect("valid is a boolean"));
    }

    fn balanced_f5_curve() -> OddCurveWire {
        OddCurveWire {
            family: "odd_prime_split_sextic_balanced_v1".into(),
            p: "5".into(),
            genus: 2,
            f: ["1", "0", "0", "0", "0", "0", "1"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            infinity: "two_rational".into(),
        }
    }

    fn balanced_infinity_divisor(n_plus: u32) -> DivisorWire {
        DivisorWire {
            u: vec!["1".into()],
            v: Vec::new(),
            n_plus: Some(n_plus),
        }
    }

    fn balanced_x0_point_divisor(y: &str, n_plus: u32) -> DivisorWire {
        DivisorWire {
            u: vec!["0".into(), "1".into()],
            v: vec![y.into()],
            n_plus: Some(n_plus),
        }
    }

    #[test]
    fn balanced_f5_identity_negation_and_addition_are_exposed() {
        let identity_request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(balanced_f5_curve()),
            operation: ArithmeticOperation::Identity,
            left: None,
            right: None,
            scalar: None,
        };
        let (identity, identity_code) = arithmetic(identity_request);
        assert_eq!(identity_code, 0, "{identity}");
        assert_eq!(identity["result"]["result"]["divisor"]["n_plus"], 1);

        let negate_request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(balanced_f5_curve()),
            operation: ArithmeticOperation::Negate,
            left: Some(balanced_infinity_divisor(2)),
            right: None,
            scalar: None,
        };
        let (negated, negate_code) = arithmetic(negate_request);
        assert_eq!(negate_code, 0, "{negated}");
        assert_eq!(negated["result"]["result"]["divisor"]["n_plus"], 0);

        let add_request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(balanced_f5_curve()),
            operation: ArithmeticOperation::Add,
            left: Some(balanced_infinity_divisor(2)),
            right: Some(balanced_infinity_divisor(0)),
            scalar: None,
        };
        let (sum, add_code) = arithmetic(add_request);
        assert_eq!(add_code, 0, "{sum}");
        assert_eq!(sum["result"]["result"]["divisor"]["u"], json!(["0x1"]));
        assert_eq!(sum["result"]["result"]["divisor"]["v"], json!([]));
        assert_eq!(sum["result"]["result"]["divisor"]["n_plus"], 1);

        // On y^2=x^6+1 over F5, (0,1) is rational.  Its balanced inverse
        // has the same affine support, y-coordinate -1=4, and the other
        // infinity counter.  This exercises shared support in the CLI path.
        let shared_support_add = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(balanced_f5_curve()),
            operation: ArithmeticOperation::Add,
            left: Some(balanced_x0_point_divisor("1", 0)),
            right: Some(balanced_x0_point_divisor("4", 1)),
            scalar: None,
        };
        let (shared_sum, shared_code) = arithmetic(shared_support_add);
        assert_eq!(shared_code, 0, "{shared_sum}");
        assert_eq!(
            shared_sum["result"]["result"]["divisor"],
            json!({"u": ["0x1"], "v": [], "n_plus": 1})
        );
    }

    #[test]
    fn balanced_divisors_require_n_plus_and_binary_divisors_reject_it() {
        let missing_balanced_counter = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(balanced_f5_curve()),
            operation: ArithmeticOperation::Validate,
            left: Some(DivisorWire {
                u: vec!["1".into()],
                v: Vec::new(),
                n_plus: None,
            }),
            right: None,
            scalar: None,
        };
        let (missing, missing_code) = arithmetic(missing_balanced_counter);
        assert_eq!(missing_code, 3, "{missing}");
        assert_eq!(missing["status"], "invalid_input");

        let binary_with_balanced_counter = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: Some(binary_model()),
            odd_curve: None,
            operation: ArithmeticOperation::Validate,
            left: Some(DivisorWire {
                u: vec!["1".into()],
                v: Vec::new(),
                n_plus: Some(1),
            }),
            right: None,
            scalar: None,
        };
        let (binary, binary_code) = arithmetic(binary_with_balanced_counter);
        assert_eq!(binary_code, 3, "{binary}");
        assert_eq!(binary["status"], "invalid_input");
    }

    #[test]
    fn zero_prime_modulus_is_a_structured_error_not_a_panic() {
        let request = ArithmeticRequest {
            schema: ARITHMETIC_SCHEMA.into(),
            model: None,
            odd_curve: Some(OddCurveWire {
                family: "odd_prime_one_infinity_v1".into(),
                p: "0".into(),
                genus: 2,
                f: vec!["1".into()],
                infinity: "one_rational".into(),
            }),
            operation: ArithmeticOperation::Identity,
            left: None,
            right: None,
            scalar: None,
        };
        let (answer, code) = arithmetic(request);
        assert_eq!(code, 3);
        assert_eq!(answer["status"], "invalid_input");
    }
}
