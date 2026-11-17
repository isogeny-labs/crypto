//! Catalogue endpoints that are not curve-centred: schema discovery, counts,
//! isogenies, isogeny classes, fields, transfers, polynomials.

use super::curves::{page, Filters};
use super::{json_one, json_rows, ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn meta(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let counts = json_one(
        &client,
        "SELECT jsonb_build_object(
           'curves', (SELECT count(*) FROM ec.curve),
           'curves_by_genus', (SELECT jsonb_object_agg(genus, n) FROM (SELECT genus, count(*) n FROM ec.curve GROUP BY genus) g),
           'curves_by_field_kind', (SELECT jsonb_object_agg(kind, n) FROM (SELECT f.kind, count(*) n FROM ec.curve c
                                     JOIN ec.field f ON f.id = c.field_id GROUP BY f.kind) k),
           'fields', (SELECT count(*) FROM ec.field),
           'representations', (SELECT count(*) FROM ec.curve_representation),
           'isogeny_classes', (SELECT count(*) FROM ec.isogeny_class_fq),
           'isogenies', (SELECT count(*) FROM ec.isogeny),
           'isogeny_routes', (SELECT count(*) FROM ec.isogeny_route),
           'curve_morphisms', (SELECT count(*) FROM ec.curve_morphism),
           'dlp_transfers', (SELECT count(*) FROM ec.dlp_transfer),
           'abelian_varieties', (SELECT count(*) FROM ec.abelian_variety),
           'certificates', (SELECT count(*) FROM ec.certificate),
           'number_fields', (SELECT count(*) FROM ec.number_field),
           'sources', (SELECT jsonb_agg(jsonb_build_object('title', title, 'path', path, 'sha256', sha256,
                                                           'git_commit', git_commit) ORDER BY id) FROM ec.source))::text",
        &[],
    )
    .await?;
    let version: i32 = client.query_one("SELECT max(version) FROM ec.schema_migrations", &[]).await?.get(0);
    Ok(Json(json!({ "schema_version": version, "counts": counts })))
}

pub async fn schema(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let relations = json_rows(
        &client,
        "SELECT jsonb_build_object(
                  'name', c.relname, 'kind', CASE c.relkind WHEN 'v' THEN 'view' ELSE 'table' END,
                  'comment', obj_description(c.oid, 'pg_class'),
                  'columns', (SELECT jsonb_agg(jsonb_build_object('name', a.attname,
                                  'type', format_type(a.atttypid, a.atttypmod), 'nullable', NOT a.attnotnull,
                                  'comment', col_description(c.oid, a.attnum)) ORDER BY a.attnum)
                                FROM pg_attribute a WHERE a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped),
                  'foreign_keys', (SELECT jsonb_agg(pg_get_constraintdef(k.oid)) FROM pg_constraint k
                                    WHERE k.conrelid = c.oid AND k.contype = 'f'),
                  'checks', (SELECT jsonb_agg(pg_get_constraintdef(k.oid)) FROM pg_constraint k
                              WHERE k.conrelid = c.oid AND k.contype = 'c'))::text
           FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
          WHERE n.nspname = 'ec' AND c.relkind IN ('r', 'v') ORDER BY c.relkind, c.relname",
        &[],
    )
    .await?;
    let enums = json_one(
        &client,
        "SELECT jsonb_object_agg(t.typname, (SELECT jsonb_agg(e.enumlabel ORDER BY e.enumsortorder) FROM pg_enum e
                                             WHERE e.enumtypid = t.oid))::text
           FROM pg_type t JOIN pg_namespace n ON n.oid = t.typnamespace WHERE n.nspname = 'ec' AND t.typtype = 'e'",
        &[],
    )
    .await?;
    let functions = json_rows(
        &client,
        "SELECT jsonb_build_object('name', p.proname, 'arguments', pg_get_function_arguments(p.oid),
                                   'returns', pg_get_function_result(p.oid), 'comment', obj_description(p.oid, 'pg_proc'))::text
           FROM pg_proc p JOIN pg_namespace n ON n.oid = p.pronamespace
          WHERE n.nspname = 'ec' AND obj_description(p.oid, 'pg_proc') IS NOT NULL ORDER BY p.proname",
        &[],
    )
    .await?;
    Ok(Json(json!({
        "schema": "ec",
        "conventions": {
            "integers": "exact; values beyond 2^53 are returned as decimal strings",
            "field_elements": "text in the field's element_encoding: decimal (GF(p)), 0x hex bitset (GF(2^m) polynomial basis), e_0,...,e_{k-1} (GF(p^k))",
            "polynomials": "jsonb arrays of field elements, ascending degree",
            "status": "every derived value carries ec.fact_status; unknown is null with status unknown",
            "identity": "ICV1 slug names a model; EC1 curve_uid names a representation with subgroup and generator; neither is inferred from j, order or degree",
        },
        "relations": relations, "enums": enums, "functions": functions,
    })))
}

pub async fn isogeny(State(state): State<Shared>, Path(id): Path<i64>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT (to_jsonb(i) || jsonb_build_object('source_slug', sc.icv1_slug, 'target_slug', tc.icv1_slug,
                 'routes', (SELECT jsonb_agg(r.external_id) FROM ec.isogeny_route_step s JOIN ec.isogeny_route r ON r.id = s.route_id
                             WHERE s.isogeny_id = i.id),
                 'provenance', (SELECT to_jsonb(p) FROM ec.provenance p WHERE p.id = i.provenance_id)))::text
           FROM ec.isogeny i LEFT JOIN ec.curve sc ON sc.id = i.source_curve_id LEFT JOIN ec.curve tc ON tc.id = i.target_curve_id
          WHERE i.id = $1::text::bigint",
        &[id.to_string()],
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no isogeny {id}")))
}

#[derive(Deserialize)]
pub struct ClassQuery {
    pub q: Option<String>,
    pub p: Option<String>,
    pub g: Option<i64>,
    pub trace: Option<String>,
    pub label_prefix: Option<String>,
    pub is_supersingular: Option<bool>,
    pub is_simple: Option<bool>,
    pub p_rank: Option<i64>,
    pub has_jacobian: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn classes(State(state): State<Shared>, Query(q): Query<ClassQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let mut f = Filters::default();
    f.opt("q = {}::numeric", &q.q);
    f.opt("p = {}::numeric", &q.p);
    f.opt("g = {}::int", &q.g);
    f.opt("trace = {}::numeric", &q.trace);
    f.opt("label LIKE {} || '%'", &q.label_prefix);
    f.opt("is_supersingular = {}::boolean", &q.is_supersingular);
    f.opt("is_simple = {}::boolean", &q.is_simple);
    f.opt("p_rank = {}::int", &q.p_rank);
    f.opt("has_jacobian = {}::boolean", &q.has_jacobian);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row', to_jsonb(ic) - 'lmfdb')::text FROM ec.isogeny_class_fq ic {}
          ORDER BY g, q, id LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let client = state.read.get().await?;
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

pub async fn class(State(state): State<Shared>, Path(label): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT (to_jsonb(ic) || jsonb_build_object(
                  'frobenius', (SELECT to_jsonb(fr) FROM ec.frobenius_genus1 fr WHERE fr.isogeny_class_id = ic.id),
                  'frobenius_ell', (SELECT jsonb_agg(to_jsonb(fe) - 'isogeny_class_id' ORDER BY fe.ell)
                                      FROM ec.frobenius_ell fe WHERE fe.isogeny_class_id = ic.id),
                  'members', (SELECT jsonb_agg(jsonb_build_object('curve_id', c.id, 'icv1_slug', c.icv1_slug,
                                  'genus', c.genus, 'model', c.model) ORDER BY c.id)
                                FROM ec.abelian_variety av JOIN ec.curve c ON c.id = av.curve_id
                               WHERE av.isogeny_class_id = ic.id),
                  'volcanoes', (SELECT jsonb_agg(to_jsonb(v)) FROM ec.isogeny_volcano v WHERE v.isogeny_class_id = ic.id)))::text
           FROM ec.isogeny_class_fq ic
          WHERE ic.label = $1::text OR ($1::text ~ '^[0-9]+$' AND ic.id = $1::text::bigint)",
        std::slice::from_ref(&label),
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no isogeny class {label}")))
}

#[derive(Deserialize)]
pub struct FieldQuery {
    pub kind: Option<String>,
    pub characteristic: Option<String>,
    pub degree: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn fields(State(state): State<Shared>, Query(q): Query<FieldQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let mut f = Filters::default();
    f.opt("f.kind::text = {}", &q.kind);
    f.opt("f.characteristic = {}::numeric", &q.characteristic);
    f.opt("f.degree = {}::int", &q.degree);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row',
                  to_jsonb(f) - 'defining_polynomial' || jsonb_build_object('curve_count',
                    (SELECT count(*) FROM ec.curve c WHERE c.field_id = f.id)))::text
           FROM ec.field f {} ORDER BY f.characteristic, f.degree, f.id LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let client = state.read.get().await?;
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

pub async fn field(State(state): State<Shared>, Path(id): Path<i64>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT (to_jsonb(f) || jsonb_build_object(
                  'ec1_records', (SELECT jsonb_agg(jsonb_build_object('field_sha256', r.field_sha256, 'record', r.record))
                                    FROM ec.field_ec1_record r WHERE r.field_id = f.id),
                  'embeddings', (SELECT jsonb_agg(to_jsonb(e)) FROM ec.field_embedding e
                                  WHERE e.subfield_id = f.id OR e.superfield_id = f.id)))::text
           FROM ec.field f WHERE f.id = $1::text::bigint",
        &[id.to_string()],
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no field {id}")))
}

#[derive(Deserialize)]
pub struct TransferQuery {
    pub route_kind: Option<String>,
    pub verdict: Option<String>,
    pub target: Option<String>,
    pub min_ratio_log2: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn transfers(State(state): State<Shared>, Query(q): Query<TransferQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let mut f = Filters::default();
    f.opt("t.route_kind = {}", &q.route_kind);
    f.opt("t.verdict = {}", &q.verdict);
    f.opt("t.target::text = {}", &q.target);
    f.opt("t.ratio_log2 >= {}::numeric", &q.min_ratio_log2);
    f.opt("t.status::text = {}", &q.status);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row',
                  to_jsonb(t) - 'cost' - 'invariants' || jsonb_build_object('icv1_slug', c.icv1_slug))::text
           FROM ec.dlp_transfer t JOIN ec.curve c ON c.id = t.source_curve_id {}
          ORDER BY t.ratio_log2 DESC NULLS LAST, t.id LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let client = state.read.get().await?;
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items,
                    "caution": "ratio_log2 is a formula estimate under the stated cost model, not a measured speedup" })))
}

pub async fn polynomial(State(state): State<Shared>, Path(sha256): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let row = json_one(
        &client,
        "SELECT (to_jsonb(p) || jsonb_build_object('artifact', (SELECT to_jsonb(a) FROM ec.artifact a WHERE a.id = p.artifact_id)))::text
           FROM ec.polynomial p WHERE p.sha256::text = lower($1::text)",
        std::slice::from_ref(&sha256),
    )
    .await?;
    row.map(Json).ok_or_else(|| ApiError::not_found(format!("no polynomial {sha256}")))
}

#[derive(Deserialize)]
pub struct ModularQuery {
    pub level: Option<i64>,
    pub kind: Option<String>,
    /// Include the coefficients (Phi_19 has about 200 terms of up to ~1000 bits).
    pub coefficients: Option<bool>,
}

pub async fn modular_polynomials(State(state): State<Shared>, Query(q): Query<ModularQuery>) -> ApiResult<Json<Value>> {
    let mut f = Filters::default();
    f.opt("m.level = {}::int", &q.level);
    f.opt("m.kind = {}", &q.kind);
    let coeffs = if q.coefficients.unwrap_or(false) { "p.coefficients" } else { "NULL::jsonb" };
    let sql = format!(
        "SELECT (to_jsonb(m) || jsonb_build_object('polynomial_sha256', p.sha256, 'variables', p.variables,
                 'degrees', p.degrees, 'term_count', p.term_count, 'max_coefficient_bits', p.max_coefficient_bits,
                 'coefficients', {coeffs}))::text
           FROM ec.modular_polynomial m JOIN ec.polynomial p ON p.id = m.polynomial_id {} ORDER BY m.level, m.kind",
        f.where_sql()
    );
    let client = state.read.get().await?;
    Ok(Json(
        json!({ "convention": "sparse terms [[i, j], c] of c X^i Y^j", "items": json_rows(&client, &sql, &f.params).await? }),
    ))
}

pub async fn class_polynomials(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let rows = json_rows(
        &client,
        "SELECT (to_jsonb(c) || jsonb_build_object('coefficients_ascending', p.coefficients, 'polynomial_sha256', p.sha256))::text
           FROM ec.class_polynomial c JOIN ec.polynomial p ON p.id = c.polynomial_id ORDER BY c.discriminant DESC",
        &[],
    )
    .await?;
    Ok(Json(json!({ "items": rows })))
}

pub async fn summation_polynomials(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let rows = json_rows(
        &client,
        "SELECT (to_jsonb(s) || jsonb_build_object('variables', p.variables, 'coefficients', p.coefficients,
                 'polynomial_sha256', p.sha256))::text
           FROM ec.summation_polynomial s JOIN ec.polynomial p ON p.id = s.polynomial_id ORDER BY s.model, s.m",
        &[],
    )
    .await?;
    Ok(Json(json!({ "convention": "sparse terms [[exponents per variable], c]; y^2 = x^3 + a x + b", "items": rows })))
}
