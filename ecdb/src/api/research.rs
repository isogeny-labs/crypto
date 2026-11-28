//! Saved research sweeps, versioned features and model-ready dataset recipes.

use super::curves::page;
use super::{json_one, json_rows, resolve, ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct PageQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn sweeps(State(state): State<Shared>, Query(q): Query<PageQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let rows = json_rows(
        &client,
        &format!(
            "SELECT jsonb_build_object('total',count(*) OVER (),'row',
              (to_jsonb(s)-'provenance_id') || jsonb_build_object(
                'cell_count',(SELECT count(*) FROM ec.research_sweep_cell c WHERE c.sweep_id=s.id),
                'member_count',(SELECT count(*) FROM ec.research_sweep_member m
                  JOIN ec.research_sweep_cell c ON c.id=m.cell_id WHERE c.sweep_id=s.id)))::text
             FROM ec.research_sweep s ORDER BY s.name,s.version DESC LIMIT {limit} OFFSET {offset}"
        ),
        &[],
    )
    .await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({"total":total,"limit":limit,"offset":offset,"items":items})))
}

pub async fn sweep(State(state): State<Shared>, Path(name): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT ((to_jsonb(s)-'provenance_id') || jsonb_build_object('cells',
          (SELECT jsonb_agg((to_jsonb(c)-'sweep_id'-'provenance_id') || jsonb_build_object('members',
            (SELECT jsonb_agg(jsonb_build_object('curve_id',m.curve_id,'icv1_slug',cv.icv1_slug,
              'selection_order',m.selection_order,'role',m.role,'status',m.status)
              ORDER BY m.selection_order,m.curve_id)
             FROM ec.research_sweep_member m JOIN ec.curve cv ON cv.id=m.curve_id WHERE m.cell_id=c.id))
            ORDER BY c.ordinal)
           FROM ec.research_sweep_cell c WHERE c.sweep_id=s.id)))::text
         FROM ec.research_sweep s WHERE s.name=$1::text ORDER BY s.version DESC LIMIT 1",
        std::slice::from_ref(&name),
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no sweep {name:?}")))
}

pub async fn curve_features(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let structural = json_one(
        &client,
        "SELECT (to_jsonb(v)-'curve_id')::text FROM ec.v_curve_ml_features v WHERE v.curve_id=$1::text::bigint",
        &[id.to_string()],
    )
    .await?;
    let observations = json_rows(
        &client,
        "SELECT jsonb_build_object('feature_key',d.feature_key,'version',d.version,
          'data_type',d.data_type,'unit',d.unit,'value',o.value,'missing_reason',o.missing_reason,
          'context',o.context,'status',o.status,'observed_at',o.observed_at)::text
         FROM ec.feature_observation o JOIN ec.feature_definition d ON d.id=o.feature_definition_id
         WHERE o.curve_id=$1::text::bigint ORDER BY d.feature_key,d.version,o.id",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({"curve_id":id,"structural":structural,"observations":observations})))
}

#[derive(Deserialize)]
pub struct FeatureQuery {
    pub scope: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn features(State(state): State<Shared>, Query(q): Query<FeatureQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let (where_sql, params) = match q.scope {
        Some(scope) => ("WHERE scope=$1::text", vec![scope]),
        None => ("", vec![]),
    };
    let rows = json_rows(
        &client,
        &format!(
            "SELECT jsonb_build_object('total',count(*) OVER (),'row',to_jsonb(d)-'provenance_id')::text
             FROM ec.feature_definition d {where_sql}
             ORDER BY d.feature_key,d.version LIMIT {limit} OFFSET {offset}"
        ),
        &params,
    )
    .await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({"total":total,"limit":limit,"offset":offset,"items":items})))
}

pub async fn datasets(State(state): State<Shared>, Query(q): Query<PageQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let client = state.read.get().await?;
    let rows = json_rows(
        &client,
        &format!(
            "SELECT jsonb_build_object('total',count(*) OVER (),'row',
              (to_jsonb(d)-'provenance_id'-'artifact_id') || jsonb_build_object(
                'artifact',(SELECT to_jsonb(a) FROM ec.artifact a WHERE a.id=d.artifact_id),
                'feature_count',(SELECT count(*) FROM ec.ml_dataset_feature f WHERE f.dataset_id=d.id),
                'member_count',(SELECT count(*) FROM ec.ml_dataset_member m WHERE m.dataset_id=d.id)))::text
             FROM ec.ml_dataset d ORDER BY d.name,d.version DESC LIMIT {limit} OFFSET {offset}"
        ),
        &[],
    )
    .await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({"total":total,"limit":limit,"offset":offset,"items":items})))
}

pub async fn dataset(State(state): State<Shared>, Path(name): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT ((to_jsonb(d)-'provenance_id'-'artifact_id') || jsonb_build_object(
          'artifact',(SELECT to_jsonb(a) FROM ec.artifact a WHERE a.id=d.artifact_id),
          'features',(SELECT jsonb_agg(jsonb_build_object(
            'feature_key',f.feature_key,'version',f.version,'scope',f.scope,'data_type',f.data_type,
            'unit',f.unit,'role',x.role,'ordinal',x.ordinal) ORDER BY x.role,x.ordinal)
            FROM ec.ml_dataset_feature x JOIN ec.feature_definition f ON f.id=x.feature_definition_id
            WHERE x.dataset_id=d.id),
          'members',(SELECT jsonb_agg(jsonb_build_object(
            'sample_key',m.sample_key,'curve_id',m.curve_id,'factor_base_id',m.factor_base_id,
            'factor_base_measurement_id',m.factor_base_measurement_id,'split',m.split,
            'sample_weight',m.sample_weight,'labels',m.labels) ORDER BY m.sample_key)
            FROM ec.ml_dataset_member m WHERE m.dataset_id=d.id)))::text
         FROM ec.ml_dataset d WHERE d.name=$1::text ORDER BY d.version DESC LIMIT 1",
        std::slice::from_ref(&name),
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no dataset {name:?}")))
}
