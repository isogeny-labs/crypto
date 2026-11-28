//! Bounded, replayable enumeration of the explicit cover shapes implemented
//! by this repository.
//!
//! A sweep is exhaustive only inside the request's finite family and
//! parameter bounds.  An empty result never establishes that no cover or
//! transfer exists.

#[path = "curve_cover_check/checker.rs"]
#[allow(dead_code)]
mod checker;
#[path = "curve_cover_check/models.rs"]
mod models;

use clap::{Parser, Subcommand};
use crypto_lib::prime_hyperelliptic::{
    prime_proof_evidence, prime_proof_status, PrimeMontgomeryQuadraticCoverP, PrimeProofStatus,
    PrimeQuadraticCoverP,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

const REQUEST_SCHEMA: &str = "curve-transfer-sweep-request/v1";
const RESULT_SCHEMA: &str = "curve-transfer-sweep/v1";
const VERIFY_SCHEMA: &str = "curve-transfer-sweep-verification/v1";
const CAPABILITIES_SCHEMA: &str = "curve-transfer-sweep-capabilities/v1";
const MAX_INPUT_BYTES: u64 = 64 * 1024 * 1024;
const HARD_MAX_ATTEMPTS: u32 = 100_000;

#[derive(Parser)]
#[command(
    name = "curve-transfer-sweep",
    about = "Enumerate and replay bounded explicit curve-cover transfer candidates",
    long_about = "Enumerates only the finite families and parameter interval declared in the request. A failed or empty sweep means no candidate was found inside that search space; it is never a proof that no cover exists."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Report constructive families and separately routed candidate searches.
    Capabilities {
        /// Result path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Resolve one target and enumerate the requested finite search space.
    Sweep {
        /// Strict JSON request, or `-` for stdin.
        #[arg(long)]
        request: PathBuf,
        /// EC1/ICV1 registry used for exact identity resolution.
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        /// Result path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Regenerate a saved result and compare the complete JSON value.
    Verify {
        /// Saved sweep result, or `-` for stdin.
        #[arg(long)]
        input: PathBuf,
        /// Exact registry used by the original sweep.
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        /// Verification response path; omit for stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn capabilities() -> Value {
    json!({
        "schema_version": CAPABILITIES_SCHEMA,
        "integrated_request_families": [
            "binary_cubic_pullback_v1",
            "prime_quadratic_pullback_v1",
        ],
        "families": [
            {
                "id": "binary_cubic_pullback_v1",
                "routing": "integrated_constructive_sweep",
                "shape": "genus_3_hyperelliptic_one_infinity",
                "field_scope": "same_binary_field",
                "map_degree": 3,
                "construction": "explicit_equations_and_map",
                "jacobian_arithmetic": "implemented_checked_reduced_divisors",
                "induced_maps": "implemented_pullback_and_arbitrary_class_pushforward",
                "subgroup_evidence": "separate_map_bound_certificate_corpus",
                "implementation": "src/binary_ecc/cover_transfer.rs",
                "limits": "the sweep constructs the family but does not itself certify a named subgroup or solver advantage",
            },
            {
                "id": "prime_quadratic_pullback_v1",
                "routing": "integrated_constructive_sweep",
                "shape": "genus_2_hyperelliptic_split_two_infinity",
                "field_scope": "odd_prime_or_declared_extension_geometry",
                "map_degree": 2,
                "construction": "explicit_equations_and_map",
                "jacobian_arithmetic": "implemented_for_checked_direct_prime_and_montgomery_normalized_models",
                "induced_maps": "implemented_for_checked_direct_short_weierstrass_and_projective_montgomery_targets",
                "subgroup_evidence": "not_verified_by_sweep",
                "implementation": "src/prime_hyperelliptic/cover_transfer.rs",
                "implementation_paths": [
                    "src/prime_hyperelliptic/cover_transfer.rs",
                    "src/prime_hyperelliptic/normalized_cover_transfer.rs"
                ],
                "limits": "Edwards exceptional-point adapters, extension arithmetic, and primes outside the accepted proof range remain structured unsupported cases",
            },
            {
                "id": "prime_bielliptic_genus2_atlas_v1",
                "routing": "separate_bounded_constructor",
                "shape": "genus_2_hyperelliptic_with_two_elliptic_quotients",
                "field_scope": "bounded_odd_prime_fields",
                "map_degree": 2,
                "construction": "explicit_bielliptic_curve_and_quotient_maps",
                "jacobian_arithmetic": "separate_chained_transfer_backend",
                "induced_maps": "separate_checked_pullback_pushforward_hops",
                "subgroup_evidence": "per_instance_only",
                "implementation": "src/bin/transfer_atlas.rs",
                "supporting_module": "src/cryptanalysis/chained_transfer/bielliptic.rs",
                "limits": "not integrated into the EC1/ICV1 sweep result schema; its own finite parameters and receipts govern completeness",
            },
            {
                "id": "nist_genus3_hyperelliptic_cover_v1",
                "routing": "separate_bounded_constructor",
                "shape": "genus_3_hyperelliptic",
                "field_scope": "selected_odd_prime_short_weierstrass_models",
                "map_degree": 2,
                "construction": "explicit_cover_and_pullback",
                "jacobian_arithmetic": "separate_genus3_backend",
                "induced_maps": "bounded_instance_checks",
                "subgroup_evidence": "per_instance_only",
                "implementation": "src/bin/nist_cover_quartic_map.rs",
                "supporting_module": "src/cryptanalysis/nist_cover_quartic.rs",
                "limits": "not a general catalog constructor and not evidence for every odd-prime curve",
            },
            {
                "id": "bielliptic_plane_quartic_diagnostic_v1",
                "routing": "separate_diagnostic_only",
                "shape": "genus_3_non_hyperelliptic_plane_quartic",
                "field_scope": "prime_fields_5_through_257",
                "map_degree": 2,
                "construction": "explicit_quartic_cover",
                "jacobian_arithmetic": "not_implemented_as_a_full_quartic_jacobian_group",
                "induced_maps": "elliptic_norm_projection_only",
                "subgroup_evidence": "not_a_full_jacobian_transfer_certificate",
                "implementation": "src/bin/quartic_ic.rs",
                "supporting_module": "src/cryptanalysis/bielliptic_quartic.rs",
                "limits": "the diagnostic's passing relation tests do not establish general quartic Jacobian arithmetic or a Jacobian index_calculus solver",
            },
            {
                "id": "gf2_small_genus_factor_search_v1",
                "routing": "candidate_filter_not_transfer",
                "shape": "hyperelliptic_genus_2_through_5_and_non_hyperelliptic_genus_3_through_4",
                "field_scope": "curves_over_F2_with_bounded_complete_model_enumeration",
                "map_degree": null,
                "construction": "curve_model_enumeration_and_frobenius_polynomial_divisibility",
                "jacobian_arithmetic": "not_supplied_by_the_factor_search",
                "induced_maps": "not_constructed",
                "subgroup_evidence": "not_constructed",
                "implementation": "src/cryptanalysis/curve_construction/enumerate.rs",
                "limits": "a matching characteristic-polynomial factor is a decomposition candidate, not an explicit cover, correspondence, or logarithm transfer",
            },
        ],
        "failure_semantics": "an unsupported family or empty bounded search is not a proof that no transfer exists",
        "claim_boundary": "this inventory distinguishes routing and implemented evidence; it does not promote a candidate filter, norm projection, or curve construction into a full Jacobian transfer or solver",
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SweepRequest {
    schema: String,
    selector: TargetSelector,
    families: Vec<SweepFamily>,
    bounds: SweepBounds,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum TargetSelector {
    /// A full ICV1 identity or its ICV1 slug.
    Icv1 {
        value: String,
    },
    Ec1 {
        value: String,
    },
    /// An exact model object.  It is linked to the registry only on equality.
    Model {
        value: Value,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum SweepFamily {
    BinaryCubicPullbackV1,
    PrimeQuadraticPullbackV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SweepBounds {
    /// Inclusive prime-subfield shift interval in `x = u^2 + c`.
    prime_shift_start: u32,
    prime_shift_end: u32,
    /// Fail closed rather than silently truncate a larger request.
    max_attempts: u32,
    max_field_bits: u32,
    max_genus: u32,
    max_degree: u32,
}

#[derive(Clone)]
struct ResolvedTarget {
    model: Value,
    model_json: String,
    model_sha256: String,
    slug: Option<String>,
    icv1: Option<String>,
    representations: Value,
    standard_names: Value,
    registry_status: &'static str,
}

type Poly = Vec<BigUint>;

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let (reader, source): (Box<dyn Read>, String) = if path.as_os_str() == "-" {
        (Box::new(std::io::stdin()), "stdin".into())
    } else {
        (
            Box::new(fs::File::open(path).map_err(|e| format!("read {}: {e}", path.display()))?),
            path.display().to_string(),
        )
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

fn write_json(value: &Value, output: Option<PathBuf>) -> Result<(), String> {
    let encoded = serde_json::to_string_pretty(value).map_err(|e| e.to_string())? + "\n";
    if let Some(path) = output {
        fs::write(&path, encoded).map_err(|e| format!("write {}: {e}", path.display()))
    } else {
        std::io::stdout()
            .lock()
            .write_all(encoded.as_bytes())
            .map_err(|e| format!("write stdout: {e}"))
    }
}

fn strict_model(value: &Value) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "elliptic model must be a JSON object".to_string())?;
    let allowed: &[&str] = match value.get("form").and_then(Value::as_str) {
        Some("y^2=x^3+a*x+b") if value.get("k").is_some() => {
            &["v", "form", "p", "k", "modulus", "field", "a", "b"]
        }
        Some("y^2=x^3+a*x+b") => &["v", "form", "p", "field", "a", "b"],
        Some("y^2+xy=x^3+a*x^2+b") => &["v", "form", "modulus", "field", "a", "b"],
        Some("B*y^2=x^3+A*x^2+x") => &["v", "form", "p", "field", "A", "B"],
        Some("a*x^2+y^2=1+d*x^2*y^2") => &["v", "form", "p", "field", "a", "d"],
        Some("x^2+y^2=c^2*(1+d*x^2*y^2)") => &["v", "form", "p", "field", "c", "d"],
        _ => return Err("unsupported elliptic model form".into()),
    };
    if let Some(name) = object.keys().find(|name| !allowed.contains(&name.as_str())) {
        return Err(format!("unexpected elliptic model field `{name}`"));
    }
    Ok(())
}

fn parse_registry(bytes: &[u8]) -> Result<Value, String> {
    let registry: Value =
        serde_json::from_slice(bytes).map_err(|e| format!("registry JSON: {e}"))?;
    if registry["schema_version"] != 1 || !registry["curves"].is_array() {
        return Err("unsupported curve registry schema".into());
    }
    Ok(registry)
}

fn target_from_row(row: &Value) -> Result<ResolvedTarget, String> {
    let model_json = row["model_json"]
        .as_str()
        .ok_or("registry row is missing model_json")?
        .to_string();
    let model: Value = serde_json::from_str(&model_json).map_err(|e| e.to_string())?;
    strict_model(&model)?;
    Ok(ResolvedTarget {
        model,
        model_sha256: checker::digest(model_json.as_bytes()),
        model_json,
        slug: Some(
            row["slug"]
                .as_str()
                .ok_or("registry row is missing slug")?
                .to_string(),
        ),
        icv1: Some(
            row["icv1"]
                .as_str()
                .ok_or("registry row is missing ICV1")?
                .to_string(),
        ),
        representations: row["representations"].clone(),
        standard_names: row["standard_names"].clone(),
        registry_status: "exact_registry_match",
    })
}

fn resolve_target(registry: &Value, selector: &TargetSelector) -> Result<ResolvedTarget, String> {
    let rows = registry["curves"].as_array().expect("checked registry");
    let matching: Vec<&Value> = match selector {
        TargetSelector::Icv1 { value } => rows
            .iter()
            .filter(|row| {
                row["icv1"].as_str() == Some(value.as_str())
                    || row["slug"].as_str() == Some(value.as_str())
            })
            .collect(),
        TargetSelector::Ec1 { value } => rows
            .iter()
            .filter(|row| {
                row["representations"].as_array().is_some_and(|reps| {
                    reps.iter()
                        .any(|rep| rep["ec1"].as_str() == Some(value.as_str()))
                })
            })
            .collect(),
        TargetSelector::Model { value } => {
            strict_model(value)?;
            rows.iter()
                .filter(|row| {
                    row["model_json"]
                        .as_str()
                        .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
                        .is_some_and(|registered| &registered == value)
                })
                .collect()
        }
    };
    if matching.len() > 1 {
        return Err("selector is ambiguous in the supplied registry".into());
    }
    if let Some(row) = matching.first() {
        return target_from_row(row);
    }
    if let TargetSelector::Model { value } = selector {
        let model_json = serde_json::to_string(value).map_err(|e| e.to_string())?;
        return Ok(ResolvedTarget {
            model: value.clone(),
            model_sha256: checker::digest(model_json.as_bytes()),
            model_json,
            slug: None,
            icv1: None,
            representations: json!([]),
            standard_names: json!([]),
            registry_status: "unregistered_exact_model",
        });
    }
    Err("selector did not resolve to exactly one registry row".into())
}

fn model_error(error: checker::ModelError) -> String {
    match error {
        checker::ModelError::Unsupported(reason) => format!("unsupported: {reason}"),
        checker::ModelError::Invalid(reason) => format!("invalid: {reason}"),
    }
}

fn trim(mut polynomial: Poly) -> Poly {
    while polynomial.last().is_some_and(Zero::is_zero) {
        polynomial.pop();
    }
    polynomial
}

fn poly_add(field: &checker::Field, left: &[BigUint], right: &[BigUint]) -> Poly {
    let mut result = vec![BigUint::zero(); left.len().max(right.len())];
    for (i, coefficient) in left.iter().enumerate() {
        result[i] = coefficient.clone();
    }
    for (i, coefficient) in right.iter().enumerate() {
        result[i] = field.add(&result[i], coefficient);
    }
    trim(result)
}

fn poly_mul(field: &checker::Field, left: &[BigUint], right: &[BigUint]) -> Poly {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut result = vec![BigUint::zero(); left.len() + right.len() - 1];
    for (i, a) in left.iter().enumerate() {
        for (j, b) in right.iter().enumerate() {
            result[i + j] = field.add(&result[i + j], &field.mul(a, b));
        }
    }
    trim(result)
}

fn encoded(polynomial: Poly) -> Vec<String> {
    trim(polynomial)
        .iter()
        .map(|coefficient| format!("0x{coefficient:x}"))
        .collect()
}

fn prime_certificate(model: &checker::Model, shift: u32) -> checker::Certificate {
    let field = &model.field;
    let x = vec![field.constant(shift), BigUint::zero(), BigUint::one()];
    let x2 = poly_mul(field, &x, &x);
    let x3 = poly_mul(field, &x2, &x);
    let f = poly_add(
        field,
        &poly_add(
            field,
            &x3,
            &poly_mul(field, std::slice::from_ref(&model.a), &x),
        ),
        std::slice::from_ref(&model.b),
    );
    checker::Certificate {
        target_model_map: model.normalization.clone(),
        construction: "prime_quadratic_pullback_v1".into(),
        genus: 2,
        degree: 2,
        h: vec![],
        f: encoded(f),
        x: encoded(x),
        y_v: vec!["0x1".into()],
        y_0: vec![],
    }
}

fn field_bits(field: &checker::Field) -> u64 {
    match field {
        checker::Field::Prime(p) => p.bits(),
        checker::Field::Binary(modulus) => modulus.degree as u64,
        checker::Field::Extension { p, low } => p.pow(low.len() as u32).bits(),
    }
}

fn family_compatible(field: &checker::Field, family: SweepFamily) -> bool {
    matches!(
        (field, family),
        (
            checker::Field::Binary(_),
            SweepFamily::BinaryCubicPullbackV1
        ) | (
            checker::Field::Prime(_) | checker::Field::Extension { .. },
            SweepFamily::PrimeQuadraticPullbackV1
        )
    )
}

fn source_and_map_identities(
    target: &ResolvedTarget,
    certificate: &checker::Certificate,
) -> (Value, Value) {
    let mut field = json!({"label": target.model["field"]});
    for key in ["p", "modulus"] {
        if !target.model[key].is_null() {
            field[key] = target.model[key].clone();
        }
    }
    let source_record = json!({
        "schema_version": "hyperelliptic-model/v1",
        "field": field,
        "form": "v^2+h(u)*v=f(u)",
        "h": certificate.h,
        "f": certificate.f,
        "genus": certificate.genus,
    });
    let cover_hash = checker::digest(source_record.to_string().as_bytes());
    let cover_uid = format!("urn:hc-model:1:sha256:{cover_hash}");
    let map_record = json!({
        "schema_version": "curve-cover-map/v1",
        "source_cover_uid": cover_uid.clone(),
        "target_model_sha256": target.model_sha256,
        "target_model_json": target.model_json,
        "degree": certificate.degree,
        "x": certificate.x,
        "y_v": certificate.y_v,
        "y_0": certificate.y_0,
        "target_model_map": certificate.target_model_map,
    });
    let map_hash = checker::digest(map_record.to_string().as_bytes());
    (
        json!({
            "cover_uid": cover_uid,
            "cover_id": format!("HC1G{}h{}", certificate.genus, &cover_hash[..12]),
            "record": source_record,
        }),
        json!({
            "map_uid": format!("urn:curve-cover-map:1:sha256:{map_hash}"),
            "map_id": format!("CV1D{}h{}", certificate.degree, &map_hash[..12]),
            "record": map_record,
        }),
    )
}

fn subgroup_obligations(target: &ResolvedTarget, degree: u32) -> Value {
    let mut rows = Vec::new();
    if let Some(representations) = target.representations.as_array() {
        for representation in representations {
            let order_text = representation["curve"]["subgroup_order"].as_str();
            let common_factor = order_text
                .and_then(|order| checker::number(order).ok())
                .map(|order| {
                    let mut left = degree;
                    let mut right = (&order % degree)
                        .to_u32_digits()
                        .first()
                        .copied()
                        .unwrap_or(0);
                    while right != 0 {
                        (left, right) = (right, left % right);
                    }
                    left
                });
            rows.push(json!({
                "ec1": representation["ec1"],
                "curve_uid": representation["curve_uid"],
                "declared_order": order_text,
                "degree_common_factor": common_factor,
                "status": "not_verified_by_sweep",
                "conditions": "requires exact subgroup order, induced-map certificates, kernel-intersection proof, and recovery replay",
            }));
        }
    }
    Value::Array(rows)
}

fn candidate(
    target: &ResolvedTarget,
    model: &checker::Model,
    certificate: &checker::Certificate,
    family: SweepFamily,
    parameters: Value,
    catalog_default: bool,
) -> Result<Value, String> {
    let (source_identity, map_identity) = source_and_map_identities(target, certificate);
    let binary = family == SweepFamily::BinaryCubicPullbackV1;
    let normalized_target = model.normalization.is_some();
    let normalization_kind = model
        .normalization
        .as_ref()
        .and_then(|map| map["kind"].as_str());
    let montgomery_target = normalization_kind == Some("montgomery_from_short");
    let unsupported_normalized_target = normalized_target && !montgomery_target;
    let (prime_proof, transfer_runtime_constructor) = match &model.field {
        checker::Field::Prime(p) => {
            let evidence = serde_json::to_value(prime_proof_evidence(p))
                .map_err(|error| format!("serialize prime proof: {error}"))?;
            let status = if unsupported_normalized_target {
                "unsupported_normalized_target_transfer_backend"
            } else if matches!(
                prime_proof_status(p),
                PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
            ) {
                let coordinate_map = checker::decoded(&certificate.x, &model.field)
                    .map_err(|reason| format!("decode verified coordinate map: {reason}"))?;
                if coordinate_map.len() != 3
                    || !coordinate_map[1].is_zero()
                    || coordinate_map[2] != BigUint::one()
                {
                    return Err("verified prime candidate has an unexpected coordinate map".into());
                }
                let runtime_source = if let Some(normalization) = &model.normalization {
                    let read = |name: &str| -> Result<BigUint, String> {
                        normalization[name]
                            .as_str()
                            .ok_or_else(|| format!("normalized map is missing `{name}`"))
                            .and_then(checker::number)
                    };
                    let runtime = PrimeMontgomeryQuadraticCoverP::try_new(
                        p.clone(),
                        read("montgomery_A")?,
                        read("montgomery_B")?,
                        coordinate_map[0].clone(),
                    )
                    .map_err(|error| {
                        format!("runtime normalized Montgomery construction: {error}")
                    })?;
                    if runtime.adapter().short_a() != &model.a
                        || runtime.adapter().short_b() != &model.b
                        || normalization["shift_A_over_3"]
                            .as_str()
                            .and_then(|value| checker::number(value).ok())
                            .as_ref()
                            != Some(runtime.adapter().shift_a_over_3())
                    {
                        return Err(
                            "normalized Montgomery metadata differs from the typed adapter".into(),
                        );
                    }
                    runtime.source_curve().f.coeffs.clone()
                } else {
                    PrimeQuadraticCoverP::try_new(
                        p.clone(),
                        model.a.clone(),
                        model.b.clone(),
                        coordinate_map[0].clone(),
                    )
                    .map_err(|error| format!("runtime prime transfer construction: {error}"))?
                    .source_curve()
                    .f
                    .coeffs
                    .clone()
                };
                if encoded(runtime_source) != certificate.f {
                    return Err(
                        "runtime prime transfer source differs from the verified family certificate"
                            .into(),
                    );
                }
                "runtime_transfer_constructor_replayed"
            } else {
                "unsupported_without_primality_certificate"
            };
            (evidence, status)
        }
        checker::Field::Binary(_) => (
            Value::Null,
            "implemented_in_library_not_replayed_by_this_sweep",
        ),
        checker::Field::Extension { .. } => (
            Value::Null,
            "unsupported_odd_extension_field_transfer_backend",
        ),
    };
    let proved_runtime_prime = matches!(
        &model.field,
        checker::Field::Prime(p)
            if !unsupported_normalized_target
                && matches!(
                    prime_proof_status(p),
                    PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
                )
    );
    let jacobian_arithmetic = match &model.field {
        checker::Field::Binary(_) => "implemented_checked_one_infinity_model",
        checker::Field::Prime(p)
            if matches!(
                prime_proof_status(p),
                PrimeProofStatus::ProvedU64 | PrimeProofStatus::ProvedPsi13
            ) =>
        {
            "strict_proof_split_sextic_backend_available_not_replayed_by_sweep"
        }
        checker::Field::Prime(_) => "unsupported_without_primality_certificate",
        checker::Field::Extension { .. } => "unsupported_odd_extension_field_backend",
    };
    let arbitrary_class_pushforward = match &model.field {
        checker::Field::Binary(_) => "implemented_in_library_not_replayed_by_this_sweep",
        checker::Field::Prime(_) if unsupported_normalized_target => {
            "unsupported_normalized_target_transfer_backend"
        }
        checker::Field::Prime(_) if proved_runtime_prime => {
            "runtime_constructor_replayed_operations_not_replayed_by_sweep"
        }
        checker::Field::Prime(p) if p <= &BigUint::from(3u32) => {
            "unsupported_odd_transfer_requires_characteristic_greater_than_three"
        }
        checker::Field::Prime(_) => "unsupported_without_primality_certificate",
        checker::Field::Extension { .. } => "unsupported_odd_extension_field_transfer_backend",
    };
    Ok(json!({
        "candidate_id": map_identity["map_id"],
        "family": family,
        "parameters": parameters,
        "catalog_record_status": if target.registry_status == "exact_registry_match" && catalog_default {
            "matches_current_constructor"
        } else {
            "not_asserted_present_in_catalog"
        },
        "target": {
            "model": target.model,
            "model_sha256": target.model_sha256,
            "icv1_slug": target.slug,
            "icv1": target.icv1,
        },
        "source": source_identity,
        "map": map_identity,
        "certificate": certificate,
        "field_of_definition": {
            "relation": "same_as_target",
            "field": target.model["field"],
            "prime_proof": prime_proof,
        },
        "statuses": {
            "explicit_construction": "verified",
            "smoothness_and_genus": "verified_by_family_checker",
            "defining_equation_identity": "verified_exact_polynomial_identity",
            "map_degree": "verified_by_family_checker",
            "transfer_runtime_constructor": if binary {
                "implemented_in_library_not_replayed_by_this_sweep"
            } else {
                transfer_runtime_constructor
            },
            "jacobian_arithmetic": jacobian_arithmetic,
            "arbitrary_class_pushforward": arbitrary_class_pushforward,
            "composition_identities": if binary {
                "implemented_in_library_not_replayed_by_this_sweep"
            } else if unsupported_normalized_target {
                "unsupported_normalized_target_transfer_backend"
            } else if proved_runtime_prime {
                "runtime_constructor_replayed_operations_not_replayed_by_sweep"
            } else if matches!(&model.field, checker::Field::Prime(_)) {
                "unsupported_without_primality_certificate"
            } else if matches!(&model.field, checker::Field::Extension { .. }) {
                "unsupported_odd_extension_field_transfer_backend"
            } else {
                "unsupported_for_this_arithmetic_backend"
            },
            "subgroup_transfer": "not_verified_by_sweep",
            "solver_evidence": "not_evaluated",
            "dlp_advantage": "not_evaluated",
        },
        "subgroup_obligations": subgroup_obligations(target, certificate.degree),
    }))
}

fn validate_request(request: &SweepRequest) -> Result<(), String> {
    if request.schema != REQUEST_SCHEMA {
        return Err(format!("unsupported request schema `{}`", request.schema));
    }
    if request.families.is_empty() {
        return Err("families must not be empty".into());
    }
    let unique: BTreeSet<_> = request.families.iter().copied().collect();
    if unique.len() != request.families.len() {
        return Err("families must not contain duplicates".into());
    }
    let bounds = &request.bounds;
    if bounds.prime_shift_start > bounds.prime_shift_end {
        return Err("prime_shift_start must not exceed prime_shift_end".into());
    }
    if bounds.max_attempts == 0 || bounds.max_attempts > HARD_MAX_ATTEMPTS {
        return Err(format!("max_attempts must be in 1..={HARD_MAX_ATTEMPTS}"));
    }
    if !(1..=4096).contains(&bounds.max_field_bits) {
        return Err("max_field_bits must be in 1..=4096".into());
    }
    if bounds.max_genus == 0 || bounds.max_degree == 0 {
        return Err("max_genus and max_degree must be positive".into());
    }
    Ok(())
}

fn sweep(request: SweepRequest, registry_bytes: &[u8]) -> Result<Value, String> {
    validate_request(&request)?;
    let registry = parse_registry(registry_bytes)?;
    let target = resolve_target(&registry, &request.selector)?;
    let model = checker::model(&target.model).map_err(model_error)?;
    if field_bits(&model.field) > request.bounds.max_field_bits as u64 {
        return Err("target field exceeds max_field_bits".into());
    }

    let default_certificate = checker::construct(&model);
    checker::verify(&model, &default_certificate)
        .map_err(|reason| format!("default constructor failed replay: {reason}"))?;
    let mut attempts = Vec::new();
    let mut candidates = Vec::new();
    let mut attempted = 0u32;

    for family in request.families.iter().copied() {
        if !family_compatible(&model.field, family) {
            attempts.push(json!({
                "family": family,
                "parameters": {},
                "disposition": "unsupported_for_input_model",
                "reason": "family characteristic/model boundary does not match the target",
            }));
            continue;
        }
        match family {
            SweepFamily::BinaryCubicPullbackV1 => {
                attempted += 1;
                if attempted > request.bounds.max_attempts {
                    return Err(
                        "declared search exceeds max_attempts; request was not truncated".into(),
                    );
                }
                if default_certificate.genus > request.bounds.max_genus
                    || default_certificate.degree > request.bounds.max_degree
                {
                    attempts.push(json!({
                        "family": family,
                        "parameters": {},
                        "disposition": "outside_declared_shape_bounds",
                    }));
                    continue;
                }
                let item = candidate(
                    &target,
                    &model,
                    &default_certificate,
                    family,
                    json!({}),
                    true,
                )?;
                attempts.push(json!({
                    "family": family,
                    "parameters": {},
                    "disposition": "verified_candidate",
                    "candidate_id": item["candidate_id"],
                }));
                candidates.push(item);
            }
            SweepFamily::PrimeQuadraticPullbackV1 => {
                for shift in request.bounds.prime_shift_start..=request.bounds.prime_shift_end {
                    attempted += 1;
                    if attempted > request.bounds.max_attempts {
                        return Err(
                            "declared search exceeds max_attempts; request was not truncated"
                                .into(),
                        );
                    }
                    let certificate = prime_certificate(&model, shift);
                    if certificate.genus > request.bounds.max_genus
                        || certificate.degree > request.bounds.max_degree
                    {
                        attempts.push(json!({
                            "family": family,
                            "parameters": {"prime_subfield_shift": shift},
                            "disposition": "outside_declared_shape_bounds",
                        }));
                        continue;
                    }
                    match checker::verify(&model, &certificate) {
                        Ok(()) => {
                            let catalog_default = serde_json::to_value(&certificate).ok()
                                == serde_json::to_value(&default_certificate).ok();
                            let item = candidate(
                                &target,
                                &model,
                                &certificate,
                                family,
                                json!({"prime_subfield_shift": shift}),
                                catalog_default,
                            )?;
                            attempts.push(json!({
                                "family": family,
                                "parameters": {"prime_subfield_shift": shift},
                                "disposition": "verified_candidate",
                                "candidate_id": item["candidate_id"],
                            }));
                            candidates.push(item);
                        }
                        Err(reason) => attempts.push(json!({
                            "family": family,
                            "parameters": {"prime_subfield_shift": shift},
                            "disposition": "rejected_invalid_candidate",
                            "reason": reason,
                        })),
                    }
                }
            }
        }
    }

    let verified = candidates.len();
    let rejected = attempts
        .iter()
        .filter(|attempt| attempt["disposition"] == "rejected_invalid_candidate")
        .count();
    let unsupported = attempts
        .iter()
        .filter(|attempt| attempt["disposition"] == "unsupported_for_input_model")
        .count();
    Ok(json!({
        "schema_version": RESULT_SCHEMA,
        "record_state": "bounded_search_result",
        "request": request,
        "registry_sha256": checker::digest(registry_bytes),
        "resolved_target": {
            "registry_status": target.registry_status,
            "model": target.model,
            "model_json": target.model_json,
            "model_sha256": target.model_sha256,
            "icv1_slug": target.slug,
            "icv1": target.icv1,
            "representations": target.representations,
            "standard_names": target.standard_names,
        },
        "search_space": {
            "exhaustiveness": "exhaustive_only_within_declared_families_and_finite_bounds",
            "failure_semantics": "zero candidates means no verified candidate was found in this search space; it is not a proof of nonexistence",
            "implemented_family_boundary": "same-field binary cubic and odd-characteristic quadratic pullback constructors only",
        },
        "attempts": attempts,
        "candidates": candidates,
        "summary": {
            "parameter_points_attempted": attempted,
            "verified_candidates": verified,
            "rejected_invalid_candidates": rejected,
            "incompatible_families": unsupported,
        },
        "claim_boundary": "construction and equation verification do not establish subgroup preservation, a Jacobian solver, or a DLP advantage",
    }))
}

fn verify(saved: Value, registry_bytes: &[u8]) -> Result<Value, String> {
    if saved["schema_version"] != RESULT_SCHEMA {
        return Err("input is not a curve-transfer-sweep/v1 result".into());
    }
    let request: SweepRequest = serde_json::from_value(saved["request"].clone())
        .map_err(|e| format!("embedded request: {e}"))?;
    let replay = sweep(request, registry_bytes)?;
    if replay != saved {
        return Err("saved result differs from deterministic replay".into());
    }
    let compact = serde_json::to_vec(&saved).map_err(|e| e.to_string())?;
    Ok(json!({
        "schema_version": VERIFY_SCHEMA,
        "status": "verified",
        "result_sha256": checker::digest(&compact),
        "result_sha256_preimage": "compact sorted-key UTF-8 JSON of the complete result value; no trailing newline",
        "candidate_count": saved["summary"]["verified_candidates"],
        "claim_boundary": saved["claim_boundary"],
    }))
}

fn execute(command: Command) -> (Value, i32, Option<PathBuf>) {
    match command {
        Command::Capabilities { output } => (capabilities(), 0, output),
        Command::Sweep {
            request,
            registry,
            output,
        } => {
            let result = read_bytes(&request)
                .and_then(|bytes| {
                    serde_json::from_slice::<SweepRequest>(&bytes)
                        .map_err(|e| format!("request JSON: {e}"))
                })
                .and_then(|request| read_bytes(&registry).and_then(|bytes| sweep(request, &bytes)));
            match result {
                Ok(value) => (value, 0, output),
                Err(reason) => (
                    json!({"schema_version": RESULT_SCHEMA, "record_state": "invalid_request", "reason": reason}),
                    3,
                    output,
                ),
            }
        }
        Command::Verify {
            input,
            registry,
            output,
        } => {
            let result = read_bytes(&input)
                .and_then(|bytes| {
                    serde_json::from_slice::<Value>(&bytes).map_err(|e| format!("result JSON: {e}"))
                })
                .and_then(|saved| read_bytes(&registry).and_then(|bytes| verify(saved, &bytes)));
            match result {
                Ok(value) => (value, 0, output),
                Err(reason) => (
                    json!({"schema_version": VERIFY_SCHEMA, "status": "invalid", "reason": reason}),
                    3,
                    output,
                ),
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let (value, mut code, output) = execute(args.command);
    if let Err(reason) = write_json(&value, output) {
        eprintln!("curve-transfer-sweep: {reason}");
        code = 1;
    }
    if code != 0 {
        std::process::exit(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_registry() -> Vec<u8> {
        br#"{"schema_version":1,"curves":[]}"#.to_vec()
    }

    fn binary_model() -> Value {
        let modulus = BigUint::from(0xbu32);
        let hash = checker::digest(format!("f2m-modulus:0x{modulus:x}").as_bytes());
        json!({
            "v":"1", "form":"y^2+xy=x^3+a*x^2+b", "modulus":"0xb",
            "field":format!("f2m-3-{}", &hash[..8]), "a":"1", "b":"1"
        })
    }

    fn prime_model() -> Value {
        json!({"v":"1","form":"y^2=x^3+a*x+b","p":"5","field":"fp-5","a":"1","b":"1"})
    }

    fn request(model: Value, family: SweepFamily, start: u32, end: u32) -> SweepRequest {
        SweepRequest {
            schema: REQUEST_SCHEMA.into(),
            selector: TargetSelector::Model { value: model },
            families: vec![family],
            bounds: SweepBounds {
                prime_shift_start: start,
                prime_shift_end: end,
                max_attempts: 32,
                max_field_bits: 64,
                max_genus: 3,
                max_degree: 3,
            },
        }
    }

    #[test]
    fn capability_inventory_separates_constructors_filters_and_diagnostics() {
        let inventory = capabilities();
        assert_eq!(inventory["schema_version"], CAPABILITIES_SCHEMA);
        assert_eq!(
            inventory["integrated_request_families"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        let families = inventory["families"].as_array().unwrap();
        assert_eq!(families.len(), 6);
        let quartic = families
            .iter()
            .find(|family| family["id"] == "bielliptic_plane_quartic_diagnostic_v1")
            .unwrap();
        assert_eq!(quartic["induced_maps"], "elliptic_norm_projection_only");
        assert_eq!(
            quartic["jacobian_arithmetic"],
            "not_implemented_as_a_full_quartic_jacobian_group"
        );
        let factor_search = families
            .iter()
            .find(|family| family["id"] == "gf2_small_genus_factor_search_v1")
            .unwrap();
        assert_eq!(factor_search["routing"], "candidate_filter_not_transfer");
        assert_eq!(factor_search["induced_maps"], "not_constructed");
    }

    #[test]
    fn binary_sweep_reports_geometry_separately_from_transfer_evidence() {
        let result = sweep(
            request(binary_model(), SweepFamily::BinaryCubicPullbackV1, 0, 0),
            &empty_registry(),
        )
        .unwrap();
        assert_eq!(result["summary"]["verified_candidates"], 1);
        assert_eq!(
            result["candidates"][0]["statuses"]["explicit_construction"],
            "verified"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["subgroup_transfer"],
            "not_verified_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["dlp_advantage"],
            "not_evaluated"
        );
    }

    #[test]
    fn prime_shift_interval_is_finite_and_replayable() {
        let registry = empty_registry();
        let result = sweep(
            request(prime_model(), SweepFamily::PrimeQuadraticPullbackV1, 0, 3),
            &registry,
        )
        .unwrap();
        assert_eq!(result["summary"]["parameter_points_attempted"], 4);
        assert_eq!(result["summary"]["verified_candidates"], 4);
        assert_eq!(
            result["candidates"][0]["statuses"]["jacobian_arithmetic"],
            "strict_proof_split_sextic_backend_available_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["arbitrary_class_pushforward"],
            "runtime_constructor_replayed_operations_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["composition_identities"],
            "runtime_constructor_replayed_operations_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["transfer_runtime_constructor"],
            "runtime_transfer_constructor_replayed"
        );
        assert_eq!(verify(result, &registry).unwrap()["status"], "verified");
    }

    #[test]
    fn montgomery_normalized_target_replays_the_composed_constructor() {
        let model = json!({
            "v":"1", "form":"B*y^2=x^3+A*x^2+x", "p":"11",
            "field":"fp-11", "A":"3", "B":"1"
        });
        let result = sweep(
            request(model, SweepFamily::PrimeQuadraticPullbackV1, 0, 0),
            &empty_registry(),
        )
        .unwrap();
        assert_eq!(result["summary"]["verified_candidates"], 1);
        assert_eq!(
            result["candidates"][0]["statuses"]["explicit_construction"],
            "verified"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["jacobian_arithmetic"],
            "strict_proof_split_sextic_backend_available_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["arbitrary_class_pushforward"],
            "runtime_constructor_replayed_operations_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["composition_identities"],
            "runtime_constructor_replayed_operations_not_replayed_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["transfer_runtime_constructor"],
            "runtime_transfer_constructor_replayed"
        );
    }

    #[test]
    fn edwards_normalized_target_keeps_the_projective_backend_gap_explicit() {
        let model = json!({
            "v":"1", "form":"a*x^2+y^2=1+d*x^2*y^2", "p":"11",
            "field":"fp-11", "a":"1", "d":"2"
        });
        let result = sweep(
            request(model, SweepFamily::PrimeQuadraticPullbackV1, 0, 0),
            &empty_registry(),
        )
        .unwrap();
        assert_eq!(result["summary"]["verified_candidates"], 1);
        assert_eq!(
            result["candidates"][0]["statuses"]["transfer_runtime_constructor"],
            "unsupported_normalized_target_transfer_backend"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["composition_identities"],
            "unsupported_normalized_target_transfer_backend"
        );
    }

    #[test]
    fn wide_montgomery_target_reports_the_proof_gate_not_a_missing_backend() {
        let model = json!({
            "v":"1", "form":"B*y^2=x^3+A*x^2+x",
            "p":"3317044064679887385962123",
            "field":"fp-3317044064679887385962123", "A":"3", "B":"1"
        });
        let mut request = request(model, SweepFamily::PrimeQuadraticPullbackV1, 0, 0);
        request.bounds.max_field_bits = 82;
        let result = sweep(request, &empty_registry()).unwrap();
        assert_eq!(
            result["candidates"][0]["statuses"]["transfer_runtime_constructor"],
            "unsupported_without_primality_certificate"
        );
        assert_eq!(
            result["candidates"][0]["statuses"]["composition_identities"],
            "unsupported_without_primality_certificate"
        );
    }

    #[test]
    fn strict_psi13_wide_prime_sweep_replays_every_declared_runtime_constructor() {
        let model = json!({
            "v":"1", "form":"y^2=x^3+a*x+b",
            "p":"5902977611310272157103",
            "field":"fp-5902977611310272157103",
            "a":"4168100599208664723916",
            "b":"240650161999037164785"
        });
        let mut request = request(model, SweepFamily::PrimeQuadraticPullbackV1, 0, 3);
        request.bounds.max_field_bits = 73;
        request.bounds.max_attempts = 4;
        let registry = empty_registry();
        let result = sweep(request, &registry).unwrap();
        assert_eq!(result["summary"]["parameter_points_attempted"], 4);
        assert_eq!(result["summary"]["verified_candidates"], 4);
        for candidate in result["candidates"].as_array().unwrap() {
            assert_eq!(
                candidate["field_of_definition"]["prime_proof"]["status"],
                "proved_psi13"
            );
            assert_eq!(
                candidate["statuses"]["transfer_runtime_constructor"],
                "runtime_transfer_constructor_replayed"
            );
            assert_eq!(
                candidate["statuses"]["subgroup_transfer"],
                "not_verified_by_sweep"
            );
            assert_eq!(candidate["statuses"]["dlp_advantage"], "not_evaluated");
        }
        assert_eq!(verify(result, &registry).unwrap()["status"], "verified");
    }

    #[test]
    fn an_empty_search_is_not_reported_as_nonexistence() {
        let model = json!({"v":"1","form":"y^2=x^3+a*x+b","p":"5","field":"fp-5","a":"0","b":"1"});
        let result = sweep(
            request(model, SweepFamily::PrimeQuadraticPullbackV1, 4, 4),
            &empty_registry(),
        )
        .unwrap();
        assert_eq!(result["summary"]["verified_candidates"], 0);
        assert_eq!(
            result["attempts"][0]["disposition"],
            "rejected_invalid_candidate"
        );
        assert!(result["search_space"]["failure_semantics"]
            .as_str()
            .unwrap()
            .contains("not a proof of nonexistence"));
    }

    #[test]
    fn max_attempts_fails_closed_without_partial_output() {
        let mut request = request(prime_model(), SweepFamily::PrimeQuadraticPullbackV1, 0, 3);
        request.bounds.max_attempts = 2;
        assert!(sweep(request, &empty_registry())
            .unwrap_err()
            .contains("not truncated"));
    }

    #[test]
    fn replay_rejects_a_changed_disposition() {
        let registry = empty_registry();
        let mut result = sweep(
            request(binary_model(), SweepFamily::BinaryCubicPullbackV1, 0, 0),
            &registry,
        )
        .unwrap();
        result["attempts"][0]["disposition"] = json!("rejected_invalid_candidate");
        assert!(verify(result, &registry)
            .unwrap_err()
            .contains("differs from deterministic replay"));
    }

    #[test]
    fn duplicate_families_are_malformed_input() {
        let mut request = request(binary_model(), SweepFamily::BinaryCubicPullbackV1, 0, 0);
        request.families.push(SweepFamily::BinaryCubicPullbackV1);
        assert!(sweep(request, &empty_registry())
            .unwrap_err()
            .contains("must not contain duplicates"));
    }

    #[test]
    fn ec1_selector_resolves_exact_registry_row() {
        let model = binary_model();
        let raw = serde_json::to_string(&model).unwrap();
        let registry = serde_json::to_vec(&json!({
            "schema_version": 1,
            "curves": [{
                "slug": "icv1-test",
                "icv1": "ICV1:test",
                "model_json": raw,
                "representations": [{"ec1":"EC1-test","curve_uid":"urn:test","curve":{"subgroup_order":"5"}}],
                "standard_names": [],
            }]
        }))
        .unwrap();
        let result = sweep(
            SweepRequest {
                schema: REQUEST_SCHEMA.into(),
                selector: TargetSelector::Ec1 {
                    value: "EC1-test".into(),
                },
                families: vec![SweepFamily::BinaryCubicPullbackV1],
                bounds: SweepBounds {
                    prime_shift_start: 0,
                    prime_shift_end: 0,
                    max_attempts: 1,
                    max_field_bits: 8,
                    max_genus: 3,
                    max_degree: 3,
                },
            },
            &registry,
        )
        .unwrap();
        assert_eq!(result["resolved_target"]["icv1_slug"], "icv1-test");
        assert_eq!(
            result["candidates"][0]["subgroup_obligations"][0]["status"],
            "not_verified_by_sweep"
        );
        assert_eq!(
            result["candidates"][0]["subgroup_obligations"][0]["degree_common_factor"],
            1
        );
    }
}
