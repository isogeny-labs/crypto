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
#[path = "curve_cover_check/models.rs"]
mod models;

use clap::{Parser, Subcommand};
use crypto_lib::binary_ecc::{
    BinaryPoint, ClosedPointPushforwardCertificate, F2mElement, F2mPoly, HyperellipticCurve,
    MumfordDivisor, OrdinaryBinaryCover, PullbackFiberKind, SubgroupTransferStatus,
};
use crypto_lib::prime_hyperelliptic::{
    BalancedDivisorP, FpPoly, HyperellipticCurveP, MumfordDivisorP, SplitHyperellipticCurveP,
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
const MAX_INPUT_BYTES: u64 = 32 * 1024 * 1024;

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
    /// Evaluate and certify the induced maps for the checked binary cover family.
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
    /// Coordinates are hexadecimal strings, with an optional `0x` prefix.
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
    Pullback,
    PullPushCompositionCertificate,
    PullbackClosedPointCertificate,
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
    /// Declared torsion/subgroup order. Exactness is an external premise and
    /// is reported as such by the corresponding result.
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

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let (reader, source): (Box<dyn Read>, String) = if path.as_os_str() == "-" {
        (Box::new(std::io::stdin()), "stdin".into())
    } else {
        let file = fs::File::open(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        (Box::new(file), path.display().to_string())
    };
    let mut bytes = Vec::new();
    reader
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read {source}: {e}"))?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err(format!("input exceeds the {MAX_INPUT_BYTES}-byte limit"));
    }
    Ok(bytes)
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
            let jacobian_arithmetic = if p.bits() <= 64 {
                "implemented_checked_balanced_split_sextic_with_deterministic_prime_revalidation"
            } else {
                "unsupported_prime_exceeds_64_bit_balanced_backend"
            };
            Ok(json!({
                "construction": "explicit",
                "field_of_definition": "same_as_target",
                "field_validation": "fixed-base_probable_prime_screen",
                "target_smoothness": "verified_by_discriminant",
                "source_smoothness": "verified_by_squarefree_sextic",
                "genus": "verified_genus_2",
                "defining_equation_identity": "verified_exact_polynomial_identity",
                "degree": "verified_degree_2",
                "separability": "verified_characteristic_not_2",
                "infinity": "two_base_field_rational_points",
                "jacobian_arithmetic": jacobian_arithmetic,
                "jacobian_representation": "canonical_balanced_(u,v,n_plus)",
                "pullback": "not_implemented_for_this_cover_family",
                "pushforward": "not_implemented_for_this_cover_family",
                "subgroup_transfer": "unknown",
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
                "cli_transfer_subcommand": "exposed_structured_binary_only",
                "subgroup_transfer": "conditional_on_an_independently_verified_exact_subgroup_order_coprime_to_3; this command does not accept or verify an order",
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
    if p.bits() > 64 {
        return (
            response(
                "arithmetic",
                "unsupported",
                json!({
                    "reason": "the explicit split-sextic cover is constructed and replayed, but balanced arithmetic has a deterministic prime-validation boundary of at most 64 bits",
                    "construction_status": "explicit_verified",
                    "arithmetic_status": "unsupported_prime_size",
                }),
            ),
            2,
        );
    }
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
            "scope": "checked Jacobian arithmetic only; induced cover transfers are not implemented on this backend",
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
    if p.bits() > 64 {
        return (
            response(
                "arithmetic",
                "unsupported",
                json!({
                    "reason": "split-sextic balanced arithmetic has a deterministic odd-prime validation boundary of at most 64 bits",
                    "arithmetic_status": "unsupported_prime_size",
                }),
            ),
            2,
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
    let curve = match SplitHyperellipticCurveP::try_new(p, f) {
        Ok(curve) => curve,
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
        TransferOperation::Pullback => "pullback",
        TransferOperation::PullPushCompositionCertificate => "pull_push_composition_certificate",
        TransferOperation::PullbackClosedPointCertificate => "pullback_closed_point_certificate",
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
        | TransferOperation::ImageEndomorphismCertificate => {
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
        TransferOperation::SubgroupStatus => {
            require(request.order.is_some(), "order")?;
            forbid(request.divisor.is_some(), "divisor")?;
            forbid(request.point.is_some(), "point")?;
        }
    }
    Ok(())
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
    let irreducible = match &model.field {
        checker::Field::Binary(irreducible) => irreducible,
        checker::Field::Prime(_) | checker::Field::Extension { .. } => {
            return (
                response(
                    "transfer",
                    "unsupported",
                    json!({
                        "reason": "the supplied cover has an odd-characteristic two-infinity source; this command currently exposes induced maps only for the checked one-infinity binary family",
                        "transfer_status": "unsupported_infinity_configuration",
                    }),
                ),
                2,
            )
        }
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
                    "scope": "checked induced map only; exact subgroup-order, injectivity, solver, and comparative-cost claims are outside this command",
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
            "exposed_structured_binary_only"
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
            "exposed_structured_binary_only"
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
    fn normalized_montgomery_cover_reaches_verified_construction() {
        let model = json!({
            "v":"1", "form":"B*y^2=x^3+A*x^2+x", "p":"11",
            "field":"fp-11", "A":"3", "B":"1"
        });
        let (answer, code) = construct(model);
        assert_eq!(code, 0, "{answer}");
        assert_eq!(answer["status"], "ok");
        assert_eq!(
            answer["result"]["certificate"]["target_model_map"]["status"],
            "verified_algebraic_identity"
        );
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
