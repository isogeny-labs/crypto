//! Measured results: the IC leaderboard, the progress timeline, the boundary
//! ledger, the benchmarks/ index, ecbench sessions and per-curve
//! measurements. Every answer carries its unit and reference; ratios compare
//! only within one family, dataset or session.

use super::curves::{page, Filters};
use super::{json_one, json_rows, resolve, ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

const BOARD_CAUTION: &str = "IC / rho is the cold cost of the best recipe over the matched rho reference of the row's family, in that family's unit; compare ratios only within one family. Below 1 would be faster than rho; no row of tables A-C is.";

#[derive(Deserialize)]
pub struct BoardQuery {
    pub family: Option<String>,
    pub regime: Option<String>,
    pub validation: Option<String>,
    pub recipes: Option<bool>,
}

pub async fn leaderboard(State(state): State<Shared>, Query(q): Query<BoardQuery>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let mut f = Filters::default();
    f.opt("b.family = {}", &q.family);
    f.opt("b.regime = {}", &q.regime);
    f.opt("b.validation_status = {}", &q.validation);
    let recipes = if q.recipes.unwrap_or(false) {
        "'recipes', (SELECT jsonb_agg(to_jsonb(r) - 'record' - 'board_row_id' ORDER BY r.ratio_rho NULLS LAST)
                       FROM ec.ic_recipe r WHERE r.board_row_id = b.id),"
    } else {
        ""
    };
    let sql = format!(
        "SELECT (to_jsonb(b) - 'record' - 'curve_record' || jsonb_build_object({recipes}
                 'source', (SELECT jsonb_build_object('path', s.path, 'sha256', s.sha256, 'git_commit', s.git_commit)
                              FROM ec.provenance p JOIN ec.source s ON s.id = p.source_id WHERE p.id = b.provenance_id)))::text
           FROM ec.ic_board_row b {} ORDER BY b.family, b.regime, b.best_ratio_rho NULLS LAST",
        f.where_sql()
    );
    let rows = json_rows(&client, &sql, &f.params).await?;
    let families = json_rows(
        &client,
        "SELECT (to_jsonb(f) - 'record' - 'provenance_id')::text FROM ec.ic_family f ORDER BY code",
        &[],
    )
    .await?;
    Ok(Json(json!({ "families": families, "rows": rows, "caution": BOARD_CAUTION })))
}

#[derive(Deserialize)]
pub struct ProgressQuery {
    pub series: Option<String>,
}

pub async fn progress(State(state): State<Shared>, Query(q): Query<ProgressQuery>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let mut f = Filters::default();
    f.opt("s.id = {}", &q.series);
    let sql = format!(
        "SELECT jsonb_build_object('id', s.id, 'label', s.label, 'note', s.note, 'barrier_test', s.barrier_test,
                 'points', (SELECT jsonb_agg(to_jsonb(p) - 'record' - 'series_id' ORDER BY p.date, p.id)
                              FROM ec.progress_point p WHERE p.series_id = s.id))::text
           FROM ec.progress_series s {} ORDER BY s.id",
        f.where_sql()
    );
    let series = json_rows(&client, &sql, &f.params).await?;
    Ok(Json(json!({ "series": series,
        "caution": "Each point is IC / rho in its own unit; reference says whether its rho was matched. Points on different curves or units are not comparable." })))
}

#[derive(Deserialize)]
pub struct BoundaryQuery {
    pub regime: Option<String>,
    pub stage: Option<String>,
}

pub async fn boundary(State(state): State<Shared>, Query(q): Query<BoundaryQuery>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let mut f = Filters::default();
    f.opt("regime = {}", &q.regime);
    f.opt("stage = {}", &q.stage);
    let sql = format!(
        "SELECT (to_jsonb(b) - 'regime_record' - 'provenance_id')::text FROM ec.boundary_record b {} ORDER BY regime, stage",
        f.where_sql()
    );
    Ok(Json(json!({ "items": json_rows(&client, &sql, &f.params).await? })))
}

#[derive(Deserialize)]
pub struct BenchQuery {
    pub listed: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn benchmarks(State(state): State<Shared>, Query(q): Query<BenchQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let mut f = Filters::default();
    f.opt("listed = {}", &q.listed);
    f.opt("(dir ILIKE '%' || {} || '%' OR readme_row ILIKE '%' || {} || '%')", &q.q);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row', to_jsonb(b) - 'provenance_id' - 'copies')::text
           FROM ec.benchmark_entry b {} ORDER BY date DESC NULLS LAST, dir LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

#[derive(Deserialize)]
pub struct SessionQuery {
    pub method: Option<String>,
    pub curve: Option<String>,
    pub label: Option<String>,
    pub path: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn sessions(State(state): State<Shared>, Query(q): Query<SessionQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let mut f = Filters::default();
    f.opt("s.label ILIKE '%' || {} || '%'", &q.label);
    f.opt("s.path ILIKE '%' || {} || '%'", &q.path);
    f.opt("EXISTS (SELECT 1 FROM ec.ecbench_run r WHERE r.session_id = s.session_id AND r.method = {})", &q.method);
    f.opt("EXISTS (SELECT 1 FROM ec.ecbench_run r WHERE r.session_id = s.session_id AND r.curve_slug = {})", &q.curve);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row',
                 to_jsonb(s) - 'record' - 'provenance_id' || jsonb_build_object(
                   'runs', (SELECT count(*) FROM ec.ecbench_run r WHERE r.session_id = s.session_id),
                   'methods', (SELECT jsonb_agg(DISTINCT r.method) FROM ec.ecbench_run r WHERE r.session_id = s.session_id),
                   'curves', (SELECT count(DISTINCT r.curve_slug) FROM ec.ecbench_run r WHERE r.session_id = s.session_id)))::text
           FROM ec.ecbench_session s {} ORDER BY s.started_at DESC NULLS LAST, s.session_id LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

pub async fn session(State(state): State<Shared>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let doc = json_one(
        &client,
        "SELECT jsonb_build_object(
           'session', to_jsonb(s) - 'provenance_id',
           'arms', (SELECT jsonb_agg(to_jsonb(a) - 'session_id' - 'session_label' - 'session_path'
                                     ORDER BY a.curve_slug, a.arm) FROM ec.v_ecbench_arm a WHERE a.session_id = s.session_id),
           'comparisons', (SELECT jsonb_agg(jsonb_build_object('comparison_id', c.comparison_id, 'path', c.path, 'a', c.a, 'b', c.b))
                             FROM ec.ecbench_comparison c WHERE c.session_id = s.session_id),
           'caution', 'S is cost in GAE per sqrt(r); lower_bound means unpriced native work (listed per run): rank two arms only where the order holds at 0, 0.1 and 1 GAE per unpriced operation (AGENTS.md 5a).')::text
           FROM ec.ecbench_session s WHERE s.session_id = $1::text",
        std::slice::from_ref(&id),
    )
    .await?;
    doc.map(Json).ok_or_else(|| ApiError::not_found(format!("no ecbench session {id}")))
}

pub async fn curve_measurements(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?.to_string();
    let doc = json_one(
        &client,
        "SELECT jsonb_build_object(
           'ic_leaderboard', (SELECT jsonb_agg(to_jsonb(b) - 'record' - 'curve_record' || jsonb_build_object(
                                 'recipes', (SELECT jsonb_agg(to_jsonb(r) - 'record' - 'board_row_id' ORDER BY r.ratio_rho NULLS LAST)
                                               FROM ec.ic_recipe r WHERE r.board_row_id = b.id)))
                                FROM ec.ic_board_row b WHERE b.curve_id = $1::text::bigint),
           'ecbench_arms', (SELECT jsonb_agg(to_jsonb(a) ORDER BY a.session_id, a.arm)
                              FROM ec.v_ecbench_arm a WHERE a.curve_id = $1::text::bigint),
           'dataset_measurements', (SELECT jsonb_agg(to_jsonb(m) - 'record' - 'provenance_id' ORDER BY m.repository, m.dataset, m.method, m.metric)
                                      FROM ec.measurement m WHERE m.curve_id = $1::text::bigint),
           'external_records', (SELECT jsonb_agg(jsonb_build_object('repository', e.repository, 'path', e.path,
                                   'record_key', e.record_key, 'kind', e.kind, 'match', e.match, 'ec1', e.ec1,
                                   'curve_uid', e.curve_uid, 'record', e.record) ORDER BY e.repository, e.path)
                                  FROM ec.external_curve_record e WHERE e.curve_id = $1::text::bigint),
           'caution', 'Each block keeps its producer''s unit and reference: compare within one block (and one family, session or dataset), never across.')::text",
        std::slice::from_ref(&id),
    )
    .await?;
    let mut doc = doc.unwrap_or(json!({}));
    doc["curve_id"] = json!(id.parse::<i64>().unwrap_or_default());
    Ok(Json(doc))
}
