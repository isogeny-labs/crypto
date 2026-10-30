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
