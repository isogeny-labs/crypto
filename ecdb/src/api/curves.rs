//! /v1/curves: filtered listing over ec.v_curve_summary, the full curve
//! document, and per-curve isogenies, isogeny graph, transfers, covers and
//! raw trait records.

use super::{json_one, json_rows, resolve, ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

/// WHERE-clause builder: every value travels as a text parameter and is cast in SQL.
#[derive(Default)]
pub struct Filters {
    pub conds: Vec<String>,
    pub params: Vec<String>,
}

impl Filters {
    /// `template` uses `{}` for the parameter, e.g. `trace = {}::numeric`.
    pub fn add(&mut self, template: &str, value: impl ToString) {
        self.params.push(value.to_string());
        let ph = format!("${}::text", self.params.len());
        self.conds.push(template.replace("{}", &ph));
    }

    pub fn opt<T: ToString>(&mut self, template: &str, value: &Option<T>) {
        if let Some(v) = value {
            self.add(template, v.to_string());
        }
    }

    pub fn raw(&mut self, cond: &str) {
        self.conds.push(cond.to_string());
    }

    pub fn where_sql(&self) -> String {
        if self.conds.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", self.conds.join(" AND "))
        }
    }
}

pub fn page(limit: Option<i64>, offset: Option<i64>) -> ApiResult<(i64, i64)> {
    let limit = limit.unwrap_or(100);
    if !(1..=1000).contains(&limit) {
        return Err(ApiError::bad_request("limit must be between 1 and 1000"));
    }
    let offset = offset.unwrap_or(0);
    if offset < 0 {
        return Err(ApiError::bad_request("offset must be non-negative"));
    }
    Ok((limit, offset))
}

#[derive(Deserialize, Default)]
pub struct CurveQuery {
    pub field_kind: Option<String>,
    pub characteristic: Option<String>,
    pub field_degree: Option<i64>,
    pub field_bits_min: Option<i64>,
    pub field_bits_max: Option<i64>,
    pub q: Option<String>,
    pub icv1_field: Option<String>,
    pub genus: Option<i64>,
    pub model: Option<String>,
    pub family: Option<String>,
    pub trace: Option<String>,
    pub isogeny_class: Option<String>,
    pub j_invariant: Option<String>,
    pub cm_discriminant: Option<String>,
    pub class_number: Option<String>,
    pub class_number_max: Option<String>,
    pub conductor_bits_max: Option<i64>,
    pub end_discriminant: Option<String>,
    pub end_is_maximal: Option<bool>,
    pub subgroup_bits_min: Option<i64>,
    pub subgroup_bits_max: Option<i64>,
    pub cofactor: Option<String>,
    pub embedding_degree_max: Option<String>,
    pub twist_prime_bits_min: Option<i64>,
    pub is_anomalous: Option<bool>,
    pub is_supersingular: Option<bool>,
    pub definition_degree: Option<i64>,
    pub standard: Option<String>,
    pub name: Option<String>,
    pub has_isogenies: Option<bool>,
    pub has_covers: Option<bool>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

const SORTABLE: &[&str] = &[
    "id",
    "icv1_slug",
    "field_bits",
    "group_order_bits",
    "subgroup_bits",
    "trace",
    "class_number",
    "conductor_bits",
    "embedding_degree",
    "cm_discriminant",
    "isogeny_count",
    "cover_count",
];

pub fn curve_filters(q: &CurveQuery) -> Filters {
    let mut f = Filters::default();
    f.opt("field_kind::text = {}", &q.field_kind);
    f.opt("characteristic = {}::numeric", &q.characteristic);
    f.opt("field_degree = {}::int", &q.field_degree);
    f.opt("field_bits >= {}::int", &q.field_bits_min);
    f.opt("field_bits <= {}::int", &q.field_bits_max);
    f.opt("q = {}::numeric", &q.q);
    f.opt("icv1_field = {}", &q.icv1_field);
    f.opt("genus = {}::int", &q.genus);
    f.opt("model::text = {}", &q.model);
    f.opt("family = {}", &q.family);
    f.opt("trace = {}::numeric", &q.trace);
    f.opt("isogeny_class_label = {}", &q.isogeny_class);
    f.opt("j_invariant = {}", &q.j_invariant);
    f.opt("cm_discriminant = {}::numeric", &q.cm_discriminant);
    f.opt("class_number = {}::numeric", &q.class_number);
    f.opt("class_number <= {}::numeric", &q.class_number_max);
    f.opt("conductor_bits <= {}::int", &q.conductor_bits_max);
    f.opt("end_discriminant = {}::numeric", &q.end_discriminant);
    f.opt("end_is_maximal = {}::boolean", &q.end_is_maximal);
    f.opt("subgroup_bits >= {}::int", &q.subgroup_bits_min);
    f.opt("subgroup_bits <= {}::int", &q.subgroup_bits_max);
    f.opt("cofactor = {}::numeric", &q.cofactor);
    f.opt("embedding_degree <= {}::numeric", &q.embedding_degree_max);
    f.opt("twist_largest_prime_bits >= {}::int", &q.twist_prime_bits_min);
    f.opt("is_anomalous = {}::boolean", &q.is_anomalous);
    f.opt("is_supersingular = {}::boolean", &q.is_supersingular);
    f.opt("definition_degree = {}::int", &q.definition_degree);
    if let Some(s) = &q.standard {
        f.add("EXISTS (SELECT 1 FROM unnest(standard_names) n WHERE n ILIKE '%' || {} || '%')", s);
    }
    if let Some(n) = &q.name {
        // Every `{}` in a template is the same parameter.
        f.add(
            "id IN (SELECT a.curve_id FROM ec.curve_alias a WHERE a.alias ILIKE '%' || {} || '%'
                    UNION SELECT s.curve_id FROM ec.curve_standard s WHERE s.display_name ILIKE '%' || {} || '%')",
            n,
        );
    }
    match q.has_isogenies {
        Some(true) => f.raw("isogeny_count > 0"),
        Some(false) => f.raw("isogeny_count = 0"),
        None => {}
    }
    match q.has_covers {
        Some(true) => f.raw("cover_count > 0"),
        Some(false) => f.raw("cover_count = 0"),
        None => {}
    }
    f
}

pub async fn list(State(state): State<Shared>, Query(q): Query<CurveQuery>) -> ApiResult<Json<Value>> {
    let (limit, offset) = page(q.limit, q.offset)?;
    let sort = q.sort.as_deref().unwrap_or("id");
    if !SORTABLE.contains(&sort) {
        return Err(ApiError::bad_request(format!("sort must be one of {SORTABLE:?}")));
    }
    let order = match q.order.as_deref() {
        None | Some("asc") => "ASC",
        Some("desc") => "DESC",
        Some(_) => return Err(ApiError::bad_request("order must be asc or desc")),
    };
    let f = curve_filters(&q);
    let sql = format!(
        "SELECT jsonb_build_object('total', count(*) OVER (), 'row', to_jsonb(s))::text
           FROM ec.v_curve_summary s {} ORDER BY {sort} {order} NULLS LAST, id LIMIT {limit} OFFSET {offset}",
        f.where_sql()
    );
    let client = state.read.get().await?;
    let rows = json_rows(&client, &sql, &f.params).await?;
    let total = rows.first().map(|r| r["total"].clone()).unwrap_or(json!(0));
    let items: Vec<Value> = rows.into_iter().map(|mut r| r["row"].take()).collect();
    Ok(Json(json!({ "total": total, "limit": limit, "offset": offset, "items": items })))
}

pub async fn document(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let doc = json_one(&client, "SELECT ec.curve_document($1::text::bigint)::text", &[id.to_string()]).await?;
    doc.map(Json).ok_or_else(|| ApiError::not_found("curve vanished"))
}

pub async fn isogenies(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT (to_jsonb(e) || jsonb_build_object('kernel_polynomial', i.kernel_polynomial,
                 'codomain_raw', i.codomain_raw, 'post_isomorphism', i.post_isomorphism, 'certificate', i.certificate))::text
           FROM ec.v_isogeny_edge e JOIN ec.isogeny i ON i.id = e.id
          WHERE e.source_curve_id = $1::text::bigint OR e.target_curve_id = $1::text::bigint
          ORDER BY e.degree, e.id",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "items": rows })))
}

#[derive(Deserialize)]
pub struct GraphQuery {
    pub depth: Option<i32>,
    pub ell: Option<String>,
}

pub async fn graph(
    State(state): State<Shared>,
    Path(reference): Path<String>,
    Query(q): Query<GraphQuery>,
) -> ApiResult<Json<Value>> {
    let depth = q.depth.unwrap_or(2);
    if !(0..=8).contains(&depth) {
        return Err(ApiError::bad_request("depth must be between 0 and 8"));
    }
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let mut params = vec![id.to_string(), depth.to_string()];
    let ell = match &q.ell {
        Some(e) => {
            params.push(e.clone());
            "$3::text::numeric"
        }
        None => "NULL::numeric",
    };
    let edges = json_rows(
        &client,
        &format!("SELECT to_jsonb(n)::text FROM ec.isogeny_neighbourhood($1::text::bigint, $2::text::int, {ell}) n"),
        &params,
    )
    .await?;
    let mut ids: Vec<String> = vec![id.to_string()];
    for e in &edges {
        for k in ["source_curve_id", "target_curve_id"] {
            if let Some(v) = e[k].as_i64() {
                ids.push(v.to_string());
            }
        }
    }
    ids.sort();
    ids.dedup();
    let ell_param = q.ell.clone().unwrap_or_default();
    let nodes = json_rows(
        &client,
        "SELECT jsonb_build_object('id', s.id, 'icv1_slug', s.icv1_slug, 'j_invariant', s.j_invariant, 'trace', s.trace,
                'end_discriminant', s.end_discriminant, 'end_conductor', s.end_conductor,
                'standard_names', s.standard_names,
                'levels', (SELECT jsonb_object_agg(l.ell::text, l.level) FROM ec.endomorphism_level l
                            WHERE l.curve_id = s.id AND ($2::text = '' OR l.ell = nullif($2::text, '')::numeric)))::text
           FROM ec.v_curve_summary s WHERE s.id = ANY ($1::text::bigint[]) ORDER BY s.id",
        &[format!("{{{}}}", ids.join(",")), ell_param],
    )
    .await?;
    Ok(Json(json!({ "root": id, "depth": depth, "ell": q.ell, "nodes": nodes, "edges": edges })))
}

pub async fn transfers(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT to_jsonb(t)::text FROM ec.dlp_transfer t WHERE t.source_curve_id = $1::text::bigint ORDER BY t.route_index",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "items": rows })))
}

pub async fn covers(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT (to_jsonb(m) || jsonb_build_object(
                  'cover_curve', jsonb_build_object('id', h.id, 'genus', h.genus, 'equation', h.equation,
                                                    'coefficients', h.coefficients),
                  'hyperelliptic_model', (SELECT to_jsonb(hm) - 'curve_id' FROM ec.hyperelliptic_model hm WHERE hm.curve_id = h.id)))::text
           FROM ec.curve_morphism m JOIN ec.curve h ON h.id = m.source_curve_id
          WHERE m.target_curve_id = $1::text::bigint OR m.source_curve_id = $1::text::bigint
          ORDER BY m.id",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "items": rows })))
}

pub async fn traits(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT jsonb_build_object('schema', t.schema, 'source', to_jsonb(s), 'record', t.record)::text
           FROM ec.trait_record t JOIN ec.source s ON s.id = t.source_id WHERE t.curve_id = $1::text::bigint",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "items": rows })))
}

pub async fn relations(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT (to_jsonb(r) || jsonb_build_object(
                  'other_curve_id', CASE WHEN r.source_curve_id = $1::text::bigint THEN r.target_curve_id ELSE r.source_curve_id END,
                  'other_slug', (SELECT c.icv1_slug FROM ec.curve c WHERE c.id = CASE WHEN r.source_curve_id = $1::text::bigint
                                  THEN r.target_curve_id ELSE r.source_curve_id END)))::text
           FROM ec.curve_relation r
          WHERE r.source_curve_id = $1::text::bigint OR r.target_curve_id = $1::text::bigint ORDER BY r.kind, r.id",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "items": rows })))
}

pub async fn endomorphisms(State(state): State<Shared>, Path(reference): Path<String>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let endos = json_rows(
        &client,
        "SELECT to_jsonb(e)::text FROM ec.endomorphism e WHERE e.curve_id = $1::text::bigint ORDER BY e.kind, e.id",
        &[id.to_string()],
    )
    .await?;
    let lattices = json_rows(
        &client,
        "SELECT to_jsonb(d)::text FROM ec.scalar_decomposition d WHERE d.curve_id = $1::text::bigint ORDER BY d.id",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({ "curve_id": id, "endomorphisms": endos, "scalar_decompositions": lattices })))
}

pub async fn division_polynomials(
    State(state): State<Shared>,
    Path(reference): Path<String>,
) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let id = resolve(&client, &reference).await?;
    let rows = json_rows(
        &client,
        "SELECT jsonb_build_object('n', d.n, 'variant', d.variant, 'degree', p.degrees[1], 'sha256', p.sha256,
                 'coefficients_ascending', p.coefficients, 'factor_degrees', d.factor_degrees, 'status', p.status)::text
           FROM ec.division_polynomial d JOIN ec.polynomial p ON p.id = d.polynomial_id
          WHERE d.curve_id = $1::text::bigint ORDER BY d.n",
        &[id.to_string()],
    )
    .await?;
    Ok(Json(json!({
        "curve_id": id,
        "convention": "x-only: n = 2 is psi_2^2 = 4x^3 + b2 x^2 + 2 b4 x + b6; odd n is psi_n; n = 4 is psi_4/psi_2",
        "items": rows,
    })))
}
