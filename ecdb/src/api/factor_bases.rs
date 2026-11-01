//! Factor bases of the IC work (migration 0011): a filtered list over
//! `ec.v_factor_base_summary`, one factor base with its aliases, FBM1
//! configurations and result summary, and its results page by page.

use super::curves::{page, Filters};
use super::{json_one, json_rows, ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct ListQuery {
    /// Any curve reference `resolve_curve` understands, or the producer's name.
    curve: Option<String>,
    family: Option<String>,
    scheme: Option<String>,
    min_columns: Option<i64>,
    max_columns: Option<i64>,
    /// true: only bases with at least one solved result.
    solved: Option<bool>,
    /// results (default), best_s, columns, key
    sort: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn list(State(state): State<Shared>, Query(q): Query<ListQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let mut f = Filters::default();
    f.opt("(curve_id IN (SELECT x.id FROM ec.resolve_curve({}) x(id)) OR curve_name = {} OR icv1_slug = {})", &q.curve);
    f.opt("family = {}", &q.family);
    f.opt("scheme = {}", &q.scheme);
    f.opt("columns >= {}::bigint", &q.min_columns);
    f.opt("columns <= {}::bigint", &q.max_columns);
    if q.solved == Some(true) {
        f.raw("solved > 0");
    }
    let order = match q.sort.as_deref().unwrap_or("results") {
        "results" => "results DESC, id",
        "best_s" => "best_s_solved ASC NULLS LAST, id",
        "columns" => "columns ASC NULLS LAST, id",
        "key" => "key",
        other => return Err(ApiError::bad_request(format!("sort {other:?}: one of results, best_s, columns, key"))),
    };
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row', to_jsonb(s))::text
           FROM ec.v_factor_base_summary s {} ORDER BY {order} LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let client = state.read.get().await?;
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

/// A factor base by its key (FB1h..., FBS1h..., an archive digest), an alias
/// value (plug-in string, PS1 label, cited digest) or its numeric id.
const RESOLVE: &str = "
  SELECT fb.id FROM ec.factor_base fb WHERE fb.key = $1::text
  UNION SELECT a.factor_base_id FROM ec.factor_base_alias a WHERE a.value = $1::text AND a.kind <> 'plug_in'
  UNION SELECT fb.id FROM ec.factor_base fb WHERE $1::text ~ '^[0-9]+$' AND fb.id = $1::text::bigint";

async fn resolve(client: &tokio_postgres::Client, key: &str) -> ApiResult<i64> {
    let rows = client.query(RESOLVE, &[&key]).await?;
    match rows.len() {
        0 => Err(ApiError::not_found(format!("no factor base {key:?}"))),
        1 => Ok(rows[0].get(0)),
        n => Err(ApiError::new(axum::http::StatusCode::CONFLICT, format!("{key:?} names {n} factor bases"))),
    }
}

pub async fn document(State(state): State<Shared>, Path(key): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &key).await?;
    let row = json_one(
        &client,
        "SELECT (to_jsonb(s) || jsonb_build_object(
                  'factor_base', to_jsonb(fb) - 'record',
                  'record', fb.record,
                  'aliases', (SELECT jsonb_agg(jsonb_build_object('kind', a.kind, 'value', a.value) ORDER BY a.kind, a.value)
                                FROM ec.factor_base_alias a WHERE a.factor_base_id = fb.id),
                  'measurements', (SELECT jsonb_agg(to_jsonb(m) - 'config' - 'factor_base_id' || jsonb_build_object(
                                     'results', (SELECT count(*) FROM ec.ecbench_run r WHERE r.fbm_id = m.fbm_id))
                                     ORDER BY m.fbm_id)
                                     FROM ec.factor_base_measurement m WHERE m.factor_base_id = fb.id),
                  'results_by_source', (SELECT jsonb_object_agg(source, n) FROM (SELECT source, count(*) n
                                          FROM ec.v_factor_base_result r WHERE r.factor_base_id = fb.id GROUP BY source) x),
                  'leaderboard_rows', (SELECT jsonb_agg(jsonb_build_object('family', b.family, 'regime', b.regime,
                                          'curve_slug', b.curve_slug, 'best_ratio_rho', b.best_ratio_rho))
                                         FROM ec.ic_board_row b WHERE b.factor_base_id = fb.id)))::text
           FROM ec.factor_base fb JOIN ec.v_factor_base_summary s ON s.id = fb.id
          WHERE fb.id = $1::text::bigint",
        &[id.to_string()],
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no factor base {key:?}")))
}

#[derive(Deserialize)]
pub struct ResultQuery {
    source: Option<String>,
    fbm_id: Option<String>,
    solved: Option<bool>,
    /// true: include the point list (where a dump was loaded) instead of results.
    points: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn results(
    State(state): State<Shared>,
    Path(key): Path<String>,
    Query(q): Query<ResultQuery>,
) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let id = resolve(&client, &key).await?;
    let mut f = Filters::default();
    f.add("factor_base_id = {}::bigint", id);
    let (table, order) = if q.points == Some(true) {
        ("ec.factor_base_point", "idx")
    } else {
        f.opt("source = {}", &q.source);
        f.opt("fbm_id = {}", &q.fbm_id);
        f.opt("solved = {}::boolean", &q.solved);
        ("ec.v_factor_base_result", "source, source_key")
    };
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row', to_jsonb(t) - 'factor_base_id')::text
           FROM {table} t {} ORDER BY {order} LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "factor_base_id": id, "total": total, "limit": limit, "offset": offset, "items": items })))
}
