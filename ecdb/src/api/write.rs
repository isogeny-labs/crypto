//! Token-gated writes for agents: evidence links and isogeny records.
//!
//! An agent-written row is a claim with provenance kind `agent`. Status
//! `proved` requires a certificate and, for an isogeny, a kernel polynomial of
//! the degree a cyclic isogeny of that degree has; nothing here certifies
//! the mathematics. Rows are never updated or deleted through the API.

use super::{resolve, ApiError, ApiResult, Shared};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct Agent {
    /// Who is writing: agent / session / tool name.
    pub name: String,
    pub method: Option<String>,
    pub notes: Option<String>,
}

async fn agent_provenance(client: &tokio_postgres::Client, agent: &Agent, context: &str) -> ApiResult<i64> {
    // One source per agent (keyed by uri); the context lives in the provenance method.
    let uri = format!("agent:{}", agent.name);
    let src: i64 = client
        .query_one(
            "WITH ins AS (INSERT INTO ec.source (kind, title, uri, provenance_class) VALUES ('agent', $1, $2, 'internal')
                          ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'agent' AND uri = $2 AND repository IS NULL AND path IS NULL
                AND sha256 IS NULL LIMIT 1",
            &[&format!("agent {}", agent.name), &uri],
        )
        .await?
        .get(0);
    let method = format!("{context}: {}", agent.method.as_deref().unwrap_or("unspecified"));
    let prov: i64 = client
        .query_one(
            "INSERT INTO ec.provenance (source_id, method, tool, notes, computed_at) VALUES ($1, $2, $3, $4, now())
             ON CONFLICT (source_id, artifact_id, method, tool, tool_version) DO UPDATE SET notes = EXCLUDED.notes
             RETURNING id",
            &[&src, &method, &agent.name, &agent.notes],
        )
        .await?
        .get(0);
    Ok(prov)
}

fn fact_status(status: Option<&str>) -> ApiResult<&str> {
    let status = status.unwrap_or("unknown");
    const ALLOWED: &[&str] = &[
        "proved",
        "probable",
        "bounded",
        "heuristic",
        "conjectural",
        "refuted",
        "unknown",
        "not_evaluated",
        "not_applicable",
    ];
    ALLOWED.contains(&status).then_some(status).ok_or_else(|| ApiError::bad_request("invalid fact status"))
}

fn sha256(s: &str, label: &str) -> ApiResult<String> {
    let lower = s.to_ascii_lowercase();
    (lower.len() == 64 && lower.bytes().all(|b| b.is_ascii_hexdigit()))
        .then_some(lower)
        .ok_or_else(|| ApiError::bad_request(format!("{label} must be 64 hexadecimal digits")))
}

async fn resolve_factor_base(client: &tokio_postgres::Client, key: &str) -> ApiResult<i64> {
    let rows = client
        .query(
            "SELECT fb.id FROM ec.factor_base fb WHERE fb.key = $1::text
             UNION SELECT a.factor_base_id FROM ec.factor_base_alias a WHERE a.value = $1::text
             UNION SELECT fb.id FROM ec.factor_base fb
                    WHERE $1::text ~ '^[0-9]+$' AND fb.id = $1::text::bigint",
            &[&key],
        )
        .await?;
    match rows.len() {
        0 => Err(ApiError::not_found(format!("no factor base {key:?}"))),
        1 => Ok(rows[0].get(0)),
        n => Err(ApiError::new(StatusCode::CONFLICT, format!("{key:?} names {n} factor bases"))),
    }
}

#[derive(Deserialize)]
pub struct EvidenceRequest {
    pub curve: String,
    pub kind: String,
    pub repository: Option<String>,
    pub path: Option<String>,
    pub sha256: Option<String>,
    pub summary: Option<Value>,
    pub validation_status: Option<String>,
    pub agent: Agent,
}

pub async fn evidence(
    State(state): State<Shared>,
    Json(req): Json<EvidenceRequest>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let pool = state.write.as_ref().ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "writes are disabled"))?;
    let client = pool.get().await?;
    let curve_id = resolve(&client, &req.curve).await?;
    if req.path.is_none() && req.sha256.is_none() {
        return Err(ApiError::bad_request("evidence needs a path or a sha256"));
    }
    let prov = agent_provenance(&client, &req.agent, "evidence link").await?;
    let row = client
        .query_opt(
            "INSERT INTO ec.evidence_link (curve_id, kind, repository, path, sha256, summary, validation_status, provenance_id)
             VALUES ($1, $2, $3, $4, $5::text, $6, $7, $8)
             ON CONFLICT DO NOTHING RETURNING id",
            &[&curve_id, &req.kind, &req.repository, &req.path, &req.sha256, &req.summary, &req.validation_status, &prov],
        )
        .await?;
    match row {
        Some(r) => Ok((StatusCode::CREATED, Json(json!({ "id": r.get::<_, i64>(0), "curve_id": curve_id })))),
        None => Ok((StatusCode::OK, Json(json!({ "curve_id": curve_id, "duplicate": true })))),
    }
}

#[derive(Deserialize)]
pub struct IsogenyRequest {
    pub source: String,
    pub target: String,
    pub degree: String,
    pub kernel_polynomial: Option<Vec<String>>,
    pub formula: Option<String>,
    pub direction: Option<String>,
    pub certificate: Option<String>,
    pub status: Option<String>,
    pub external_id: Option<String>,
    pub agent: Agent,
}

fn is_small_prime(n: &BigInt) -> Option<u64> {
    let v = n.to_u64()?;
    (v >= 2 && (2..).take_while(|d| d * d <= v).all(|d| v % d != 0)).then_some(v)
}

pub async fn isogeny(
    State(state): State<Shared>,
    Json(req): Json<IsogenyRequest>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let pool = state.write.as_ref().ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "writes are disabled"))?;
    let client = pool.get().await?;
    let src = resolve(&client, &req.source).await?;
    let dst = resolve(&client, &req.target).await?;
    let degree: BigInt = req.degree.parse().map_err(|_| ApiError::bad_request("degree must be an integer"))?;
    if degree < BigInt::one() {
        return Err(ApiError::bad_request("degree must be positive"));
    }
    let status = req.status.as_deref().unwrap_or("unknown");
    if !["proved", "probable", "heuristic", "conjectural", "unknown"].contains(&status) {
        return Err(ApiError::bad_request("status must be proved, probable, heuristic, conjectural or unknown"));
    }
    let ell = is_small_prime(&degree);
    if let (Some(k), Some(l)) = (&req.kernel_polynomial, ell) {
        // A cyclic l-isogeny's kernel polynomial has degree (l - 1)/2 (l odd) or 1 (l = 2).
        let expect = if l == 2 { 1 } else { (l - 1) / 2 };
        if k.len() as u64 != expect + 1 {
            return Err(ApiError::bad_request(format!(
                "a kernel polynomial of a cyclic {l}-isogeny has degree {expect}, got {}",
                k.len().saturating_sub(1)
            )));
        }
        if !matches!(k.last().map(String::as_str), Some("1") | Some("0x1"))
            && !k.last().is_some_and(|c| c.starts_with("1,") && c[2..].split(',').all(|d| d == "0"))
        {
            return Err(ApiError::bad_request("the kernel polynomial must be monic"));
        }
    }
    if status == "proved" && (req.certificate.as_deref().unwrap_or("").is_empty() || req.kernel_polynomial.is_none()) {
        return Err(ApiError::bad_request("status proved needs a certificate and a kernel polynomial"));
    }
    let shape = client
        .query_one(
            "SELECT s.genus, t.genus, s.field_id = t.field_id, sa.id, ta.id
               FROM ec.curve s JOIN ec.curve t ON t.id = $2 JOIN ec.abelian_variety sa ON sa.curve_id = s.id
               JOIN ec.abelian_variety ta ON ta.curve_id = t.id WHERE s.id = $1",
            &[&src, &dst],
        )
        .await?;
    let (sg, tg, same_field, sa, ta): (i32, i32, bool, i64, i64) =
        (shape.get(0), shape.get(1), shape.get(2), shape.get(3), shape.get(4));
    if sg != tg {
        return Err(ApiError::bad_request("isogenous varieties have equal dimension"));
    }
    if req.kernel_polynomial.is_some() && sg != 1 {
        return Err(ApiError::bad_request("kernel polynomials are for elliptic curves"));
    }
    if !same_field {
        return Err(ApiError::bad_request("source and target must be models over the same field representation"));
    }
    let prov = agent_provenance(&client, &req.agent, "isogeny").await?;
    let kernel = req.kernel_polynomial.as_ref().map(|k| json!(k));
    let row = client
        .query_opt(
            "INSERT INTO ec.isogeny (external_id, source_av_id, target_av_id, source_curve_id, target_curve_id, dimension,
                    degree, ell, kernel_structure, field_of_definition_id, formula, kernel_polynomial, direction,
                    certificate, status, provenance_id)
             SELECT $1::text, $2::bigint, $3::bigint, $4::bigint, $5::bigint, $6::int, $7::text::numeric,
                    $8::text::numeric, CASE WHEN $8::text IS NOT NULL THEN ARRAY[$7::text::numeric] END,
                    c.field_id, coalesce($9::text, 'unknown')::ec.isogeny_formula, $10::jsonb,
                    $11::text::ec.volcano_direction, $12::text, $13::text::ec.fact_status, $14::bigint
               FROM ec.curve c WHERE c.id = $4::bigint
             ON CONFLICT DO NOTHING RETURNING id",
            &[
                &req.external_id, &sa, &ta, &src, &dst, &sg, &degree.to_string(), &ell.map(|l| l.to_string()),
                &req.formula, &kernel, &req.direction, &req.certificate, &status, &prov,
            ],
        )
        .await?;
    match row {
        Some(r) => Ok((StatusCode::CREATED, Json(json!({ "id": r.get::<_, i64>(0), "status": status })))),
        None => Ok((StatusCode::OK, Json(json!({ "duplicate": true })))),
    }
}

#[derive(Deserialize)]
pub struct FeatureObservationRequest {
    /// Exactly one subject must be named. This endpoint currently supports
    /// curve, factor-base and FBM1 observations.
    pub curve: Option<String>,
    pub factor_base: Option<String>,
    pub fbm_id: Option<String>,
    pub feature_key: String,
    pub version: Option<i32>,
    pub value: Option<Value>,
    pub missing_reason: Option<String>,
    #[serde(default = "empty_object")]
    pub context: Value,
    pub status: Option<String>,
    pub observed_at: Option<String>,
    pub agent: Agent,
}

fn empty_object() -> Value {
    json!({})
}

fn feature_type_ok(kind: &str, value: &Value) -> bool {
    match kind {
        "integer" => match value {
            Value::Number(n) => !n.to_string().contains(['.', 'e', 'E']),
            Value::String(s) => {
                let digits = s.strip_prefix('-').unwrap_or(s);
                !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
            }
            _ => false,
        },
        "number" => {
            value.is_number() || value.as_str().is_some_and(|s| s.parse::<f64>().is_ok_and(|number| number.is_finite()))
        }
        "boolean" => value.is_boolean(),
        "string" | "category" => value.is_string(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        _ => false,
    }
}

pub async fn feature_observation(
    State(state): State<Shared>,
    Json(req): Json<FeatureObservationRequest>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let pool = state.write.as_ref().ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "writes are disabled"))?;
    let client = pool.get().await?;
    if !req.context.is_object() {
        return Err(ApiError::bad_request("context must be an object"));
    }
    if usize::from(req.curve.is_some()) + usize::from(req.factor_base.is_some()) + usize::from(req.fbm_id.is_some())
        != 1
    {
        return Err(ApiError::bad_request("exactly one of curve, factor_base or fbm_id is required"));
    }
    let version = req.version.unwrap_or(1);
    let definition = client
        .query_opt(
            "SELECT id,scope,data_type,nullable FROM ec.feature_definition WHERE feature_key=$1 AND version=$2",
            &[&req.feature_key, &version],
        )
        .await?
        .ok_or_else(|| ApiError::bad_request("unknown feature_key/version"))?;
    let definition_id: i64 = definition.get(0);
    let scope: String = definition.get(1);
    let data_type: String = definition.get(2);
    let nullable: bool = definition.get(3);
    let (curve_id, factor_base_id, measurement_id, supplied_scope) = if let Some(reference) = &req.curve {
        (Some(resolve(&client, reference).await?), None, None, "curve")
    } else if let Some(key) = &req.factor_base {
        (None, Some(resolve_factor_base(&client, key).await?), None, "factor_base")
    } else {
        let fbm_id = req.fbm_id.as_deref().unwrap_or_default();
        let id = client
            .query_opt("SELECT id FROM ec.factor_base_measurement WHERE fbm_id=$1", &[&fbm_id])
            .await?
            .ok_or_else(|| ApiError::not_found(format!("no factor-base measurement {fbm_id:?}")))?
            .get::<_, i64>(0);
        (None, None, Some(id), "factor_base_measurement")
    };
    if scope != supplied_scope {
        return Err(ApiError::bad_request(format!(
            "feature {} v{} has scope {scope}, not {supplied_scope}",
            req.feature_key, version
        )));
    }
    if req.value.is_some() && req.missing_reason.is_some() {
        return Err(ApiError::bad_request("value and missing_reason are mutually exclusive"));
    }
    if let Some(value) = &req.value {
        if !feature_type_ok(&data_type, value) {
            return Err(ApiError::bad_request(format!("value does not match feature data_type {data_type}")));
        }
    } else if !nullable && req.missing_reason.is_none() {
        return Err(ApiError::bad_request("a non-nullable feature needs a value or explicit missing_reason"));
    }
    let status = fact_status(req.status.as_deref())?;
    let provenance = agent_provenance(&client, &req.agent, "feature observation").await?;
    let row = client
        .query_opt(
            "INSERT INTO ec.feature_observation
             (feature_definition_id,curve_id,factor_base_id,factor_base_measurement_id,
              value,missing_reason,context,status,observed_at,provenance_id)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8::text::ec.fact_status,$9::text::timestamptz,$10)
             ON CONFLICT DO NOTHING RETURNING id",
            &[
                &definition_id,
                &curve_id,
                &factor_base_id,
                &measurement_id,
                &req.value,
                &req.missing_reason,
                &req.context,
                &status,
                &req.observed_at,
                &provenance,
            ],
        )
        .await?;
    match row {
        Some(row) => Ok((StatusCode::CREATED, Json(json!({"id":row.get::<_,i64>(0),"scope":scope,"status":status})))),
        None => Ok((StatusCode::OK, Json(json!({"scope":scope,"duplicate":true})))),
    }
}

#[derive(Deserialize)]
pub struct FactorBaseArtifactRequest {
    pub factor_base: String,
    pub role: String,
    #[serde(default)]
    pub is_primary: bool,
    pub sha256: String,
    pub size_bytes: Option<i64>,
    pub media_type: Option<String>,
    pub storage_uri: String,
    pub repo_path: Option<String>,
    pub description: Option<String>,
    pub agent: Agent,
}

/// Link an already-uploaded content-addressed object to an existing factor
/// base. The API records identity and location; it does not upload or verify
/// the remote bytes.
pub async fn factor_base_artifact(
    State(state): State<Shared>,
    Json(req): Json<FactorBaseArtifactRequest>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let pool = state.write.as_ref().ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "writes are disabled"))?;
    let client = pool.get().await?;
    let factor_base_id = resolve_factor_base(&client, &req.factor_base).await?;
    const ROLES: &[&str] = &["dump", "matrix", "relations", "log_table", "certificate", "manifest", "dataset", "other"];
    if !ROLES.contains(&req.role.as_str()) {
        return Err(ApiError::bad_request(format!("role must be one of {ROLES:?}")));
    }
    if req.size_bytes.is_some_and(|n| n < 0) {
        return Err(ApiError::bad_request("size_bytes must be non-negative"));
    }
    if !(req.storage_uri.starts_with("s3://") || req.storage_uri.starts_with("https://")) {
        return Err(ApiError::bad_request("storage_uri must be an s3:// or https:// object URI"));
    }
    let digest = sha256(&req.sha256, "sha256")?;
    if req.is_primary {
        let existing = client
            .query_opt(
                "SELECT a.artifact_id FROM ec.factor_base_artifact a
                  WHERE a.factor_base_id=$1 AND a.role=$2 AND a.is_primary",
                &[&factor_base_id, &req.role],
            )
            .await?;
        if let Some(row) = existing {
            let existing_id: i64 = row.get(0);
            let same: bool = client
                .query_one("SELECT sha256::text=$2 FROM ec.artifact WHERE id=$1", &[&existing_id, &digest])
                .await?
                .get(0);
            if !same {
                return Err(ApiError::bad_request(format!(
                    "factor base already has a different primary {} artifact",
                    req.role
                )));
            }
        }
    }
    let artifact_id: i64 =
        match client.query_opt("SELECT id,storage_uri FROM ec.artifact WHERE sha256::text=$1", &[&digest]).await? {
            Some(row) => {
                let id: i64 = row.get(0);
                let uri: Option<String> = row.get(1);
                if uri.as_deref().is_some_and(|u| u != req.storage_uri) {
                    return Err(ApiError::bad_request("artifact sha256 already has a different storage_uri"));
                }
                client
                    .execute(
                        "UPDATE ec.artifact SET storage_uri=coalesce(storage_uri,$2),
                      size_bytes=coalesce(size_bytes,$3),media_type=coalesce(media_type,$4),
                      repo_path=coalesce(repo_path,$5),description=coalesce(description,$6) WHERE id=$1",
                        &[&id, &req.storage_uri, &req.size_bytes, &req.media_type, &req.repo_path, &req.description],
                    )
                    .await?;
                id
            }
            None => client
                .query_one(
                    "INSERT INTO ec.artifact (sha256,size_bytes,media_type,storage_uri,repo_path,description)
                 VALUES ($1::text,$2,$3,$4,$5,$6) RETURNING id",
                    &[&digest, &req.size_bytes, &req.media_type, &req.storage_uri, &req.repo_path, &req.description],
                )
                .await?
                .get(0),
        };
    let provenance = agent_provenance(&client, &req.agent, "factor-base artifact link").await?;
    let inserted = client
        .execute(
            "INSERT INTO ec.factor_base_artifact
             (factor_base_id,artifact_id,role,is_primary,description,provenance_id)
             VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT DO NOTHING",
            &[&factor_base_id, &artifact_id, &req.role, &req.is_primary, &req.description, &provenance],
        )
        .await?;
    let status = if inserted == 1 { StatusCode::CREATED } else { StatusCode::OK };
    Ok((
        status,
        Json(json!({
            "factor_base_id":factor_base_id,"artifact_id":artifact_id,
            "role":req.role,"is_primary":req.is_primary,"duplicate":inserted == 0
        })),
    ))
}

#[cfg(test)]
mod research_tests {
    use super::*;

    #[test]
    fn exact_integer_feature_values() {
        assert!(feature_type_ok("integer", &json!(42)));
        assert!(feature_type_ok("integer", &json!("115792089237316195423570985008687907852837")));
        assert!(!feature_type_ok("integer", &json!(1.5)));
        assert!(feature_type_ok("number", &json!("1.25e-3")));
        assert!(!feature_type_ok("number", &json!("NaN")));
        assert!(!feature_type_ok("number", &json!("inf")));
    }
}
