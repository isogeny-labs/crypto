//! HTTP API over the catalogue. Read endpoints return JSON documents built
//! in SQL; integers beyond 2^53 are decimal strings (`crate::json`).
//! `POST /v1/sql` runs one read-only statement for agents that need a query
//! the curated endpoints do not offer. Writes are token-gated.

use crate::json::safe_numbers;
use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_postgres::types::ToSql;

mod catalog;
mod curves;
pub mod export;
mod factor_bases;
mod mcp;
mod results;
mod sql;
mod write;

pub const OPENAPI: &str = include_str!("../../openapi.yaml");

#[derive(Clone)]
pub struct Config {
    pub read_token: Option<String>,
    pub write_token: Option<String>,
    pub sql_role: Option<String>,
    pub sql_timeout_ms: u64,
    pub sql_max_rows: usize,
}

pub struct AppState {
    pub read: Pool,
    pub write: Option<Pool>,
    pub config: Config,
}

pub type Shared = Arc<AppState>;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub body: Value,
}

impl ApiError {
    pub fn new(status: StatusCode, msg: impl Into<String>) -> ApiError {
        ApiError { status, body: json!({ "error": msg.into() }) }
    }
    pub fn bad_request(msg: impl Into<String>) -> ApiError {
        ApiError::new(StatusCode::BAD_REQUEST, msg)
    }
    pub fn not_found(msg: impl Into<String>) -> ApiError {
        ApiError::new(StatusCode::NOT_FOUND, msg)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(e: tokio_postgres::Error) -> ApiError {
        match e.as_db_error() {
            Some(db) => ApiError {
                status: StatusCode::BAD_REQUEST,
                body: json!({ "error": db.message(), "code": db.code().code(), "detail": db.detail(), "hint": db.hint() }),
            },
            None => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

impl From<deadpool_postgres::PoolError> for ApiError {
    fn from(e: deadpool_postgres::PoolError) -> ApiError {
        ApiError::new(StatusCode::SERVICE_UNAVAILABLE, format!("database unavailable: {e}"))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> ApiError {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("bad JSON from the database: {e}"))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

/// Run a query whose single column is JSON text; parse and shape each row.
pub async fn json_rows(client: &tokio_postgres::Client, sql: &str, params: &[String]) -> ApiResult<Vec<Value>> {
    let refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();
    let rows = client.query(sql, &refs).await?;
    rows.iter()
        .map(|r| {
            let text: Option<String> = r.get(0);
            Ok(match text {
                Some(t) => safe_numbers(serde_json::from_str(&t)?),
                None => Value::Null,
            })
        })
        .collect()
}

pub async fn json_one(client: &tokio_postgres::Client, sql: &str, params: &[String]) -> ApiResult<Option<Value>> {
    Ok(json_rows(client, sql, params).await?.into_iter().next().filter(|v| !v.is_null()))
}

/// Resolve a curve reference to exactly one id: 404 when unknown, 409 with
/// the candidates when the name is ambiguous.
pub async fn resolve(client: &tokio_postgres::Client, reference: &str) -> ApiResult<i64> {
    let rows = client
        .query(
            "SELECT c.id, c.icv1_slug, c.genus, c.family FROM ec.resolve_curve($1::text) r(id) JOIN ec.curve c ON c.id = r.id ORDER BY c.id",
            &[&reference],
        )
        .await?;
    match rows.len() {
        0 => Err(ApiError::not_found(format!("no curve is named {reference:?}"))),
        1 => Ok(rows[0].get(0)),
        _ => Err(ApiError {
            status: StatusCode::CONFLICT,
            body: json!({
                "error": format!("{reference:?} names {} curves", rows.len()),
                "candidates": rows.iter().map(|r| json!({
                    "id": r.get::<_, i64>(0), "icv1_slug": r.get::<_, Option<String>>(1),
                    "genus": r.get::<_, i32>(2), "family": r.get::<_, Option<String>>(3)
                })).collect::<Vec<_>>(),
            }),
        }),
    }
}

fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers.get(header::AUTHORIZATION)?.to_str().ok()?.strip_prefix("Bearer ")
}

async fn read_auth(State(state): State<Shared>, req: Request, next: Next) -> Response {
    if let Some(token) = &state.config.read_token {
        let ok = bearer(req.headers()) == Some(token.as_str())
            || state.config.write_token.as_deref().is_some_and(|w| bearer(req.headers()) == Some(w));
        if !ok {
            return ApiError::new(StatusCode::UNAUTHORIZED, "missing or wrong bearer token").into_response();
        }
    }
    next.run(req).await
}

async fn write_auth(State(state): State<Shared>, req: Request, next: Next) -> Response {
    match (&state.config.write_token, &state.write) {
        (Some(token), Some(_)) if bearer(req.headers()) == Some(token.as_str()) => next.run(req).await,
        (Some(_), Some(_)) => {
            ApiError::new(StatusCode::UNAUTHORIZED, "write endpoints need the write bearer token").into_response()
        }
        _ => ApiError::new(
            StatusCode::FORBIDDEN,
            "writes are disabled (set ECDB_WRITE_TOKEN and ECDB_WRITE_DATABASE_URL)",
        )
        .into_response(),
    }
}

async fn healthz(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let v: i32 = client.query_one("SELECT max(version) FROM ec.schema_migrations", &[]).await?.get(0);
    Ok(Json(json!({ "ok": true, "schema_version": v })))
}

async fn openapi() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/yaml")], OPENAPI)
}

async fn index() -> Json<Value> {
    Json(json!({
        "service": "ecdb",
        "openapi": "/openapi.yaml",
        "start_here": ["/v1/meta", "/v1/schema", "/v1/curves?limit=5", "/v1/curves/secp256k1"],
    }))
}

pub fn router(state: Shared) -> Router {
    let reads = Router::new()
        .route("/v1/meta", get(catalog::meta))
        .route("/v1/schema", get(catalog::schema))
        .route("/v1/curves", get(curves::list))
        .route("/v1/curves/{reference}", get(curves::document))
        .route("/v1/curves/{reference}/isogenies", get(curves::isogenies))
        .route("/v1/curves/{reference}/graph", get(curves::graph))
        .route("/v1/curves/{reference}/transfers", get(curves::transfers))
        .route("/v1/curves/{reference}/covers", get(curves::covers))
        .route("/v1/curves/{reference}/traits", get(curves::traits))
        .route("/v1/curves/{reference}/relations", get(curves::relations))
        .route("/v1/curves/{reference}/endomorphisms", get(curves::endomorphisms))
        .route("/v1/curves/{reference}/division-polynomials", get(curves::division_polynomials))
        .route("/v1/curves/{reference}/measurements", get(results::curve_measurements))
        .route("/v1/ic/leaderboard", get(results::leaderboard))
        .route("/v1/ic/progress", get(results::progress))
        .route("/v1/ic/boundary", get(results::boundary))
        .route("/v1/benchmarks", get(results::benchmarks))
        .route("/v1/ecbench/sessions", get(results::sessions))
        .route("/v1/ecbench/sessions/{id}", get(results::session))
        .route("/v1/isogenies/{id}", get(catalog::isogeny))
        .route("/v1/isogeny-classes", get(catalog::classes))
        .route("/v1/isogeny-classes/{label}", get(catalog::class))
        .route("/v1/fields", get(catalog::fields))
        .route("/v1/fields/{id}", get(catalog::field))
        .route("/v1/transfers", get(catalog::transfers))
        .route("/v1/polynomials/{sha256}", get(catalog::polynomial))
        .route("/v1/modular-polynomials", get(catalog::modular_polynomials))
        .route("/v1/class-polynomials", get(catalog::class_polynomials))
        .route("/v1/summation-polynomials", get(catalog::summation_polynomials))
        .route("/v1/factor-bases", get(factor_bases::list))
        .route("/v1/factor-base-designs", get(factor_bases::designs))
        .route("/v1/ic/claims", get(factor_bases::claims))
        .route("/v1/factor-bases/{key}", get(factor_bases::document))
        .route("/v1/factor-bases/{key}/results", get(factor_bases::results))
        .route("/v1/sql", post(sql::run))
        .route("/v1/export", get(export::manifest))
        .route("/v1/export/{table}", get(export::page))
        .route_layer(middleware::from_fn_with_state(state.clone(), read_auth));
    let writes = Router::new()
        .route("/v1/evidence", post(write::evidence))
        .route("/v1/isogenies", post(write::isogeny))
        .route_layer(middleware::from_fn_with_state(state.clone(), write_auth));
    let api: Router = Router::new()
        .route("/", get(index))
        .route("/healthz", get(healthz))
        .route("/openapi.yaml", get(openapi))
        .merge(reads)
        .merge(writes)
        .with_state(state);
    // MCP (streamable HTTP, JSON responses): each tool is a request to `api` itself.
    let inner = api.clone();
    api.route(
        "/mcp",
        post(move |headers: HeaderMap, body: axum::body::Bytes| mcp::handle(inner.clone(), headers, body))
            .get(mcp::get_not_supported),
    )
    .layer(tower_http::trace::TraceLayer::new_for_http())
    .layer(tower_http::cors::CorsLayer::permissive())
}
