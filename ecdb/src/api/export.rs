//! Bulk export, for mirrors (`ecdb sync`, deploy/nas/). `GET /v1/export`
//! lists every table of schema `ec` with its copyable columns, primary key and
//! exact row count. `GET /v1/export/{table}?offset=&limit=` returns one page
//! of the table in Postgres COPY text format, gzipped, in primary-key order.
//! A page is capped at `MAX_GZIP` bytes so it fits a Lambda function-URL
//! response after base64; a larger page is refused with 413 and the client
//! asks again with a smaller `limit`. Both endpoints sit behind the read token.

use super::{ApiError, ApiResult, Shared};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures_util::{pin_mut, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::Write;

/// Largest gzipped page: 6 MB Lambda payload limit, less base64's 4/3.
pub const MAX_GZIP: usize = 4 << 20;
pub const MAX_ROWS: i64 = 200_000;

/// Header carrying the number of rows in a page.
pub const ROWS_HEADER: &str = "x-ecdb-rows";

/// Tables of schema `ec` except the migration ledger (a mirror runs its own
/// migrations). Generated columns cannot be copied in, so they are left out.
const TABLES_SQL: &str = "
SELECT c.relname::text,
       (SELECT array_agg(a.attname::text ORDER BY a.attnum) FROM pg_attribute a
         WHERE a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped AND a.attgenerated = ''),
       COALESCE((SELECT array_agg(a.attname::text ORDER BY array_position(i.indkey::int2[], a.attnum))
                   FROM pg_index i JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum = ANY (i.indkey)
                  WHERE i.indrelid = c.oid AND i.indisprimary), '{}')
  FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
 WHERE n.nspname = 'ec' AND c.relkind = 'r' AND c.relname <> 'schema_migrations'
 ORDER BY 1";

pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub primary_key: Vec<String>,
}

pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

pub fn column_list(columns: &[String]) -> String {
    columns.iter().map(|c| quote_ident(c)).collect::<Vec<_>>().join(", ")
}

async fn tables(client: &tokio_postgres::Client) -> ApiResult<Vec<Table>> {
    Ok(client
        .query(TABLES_SQL, &[])
        .await?
        .iter()
        .map(|r| Table { name: r.get(0), columns: r.get(1), primary_key: r.get(2) })
        .collect())
}

pub async fn manifest(State(state): State<Shared>) -> ApiResult<Json<Value>> {
    let client = state.read.get().await?;
    let version: i32 = client.query_one("SELECT max(version) FROM ec.schema_migrations", &[]).await?.get(0);
    let mut out = Vec::new();
    for t in tables(&client).await? {
        let rows: i64 =
            client.query_one(&format!("SELECT count(*) FROM ec.{}", quote_ident(&t.name)), &[]).await?.get(0);
        out.push(json!({ "table": t.name, "columns": t.columns, "primary_key": t.primary_key, "rows": rows }));
    }
    Ok(Json(json!({ "schema_version": version, "format": "copy-text+gzip", "max_rows": MAX_ROWS, "tables": out })))
}

#[derive(Deserialize)]
pub struct PageQuery {
    offset: Option<i64>,
    limit: Option<i64>,
}

pub async fn page(
    State(state): State<Shared>,
    Path(table): Path<String>,
    Query(q): Query<PageQuery>,
) -> ApiResult<Response> {
    let client = state.read.get().await?;
    let t = tables(&client)
        .await?
        .into_iter()
        .find(|t| t.name == table)
        .ok_or_else(|| ApiError::not_found(format!("no exportable table ec.{table}")))?;
    let offset = q.offset.unwrap_or(0);
    let limit = q.limit.unwrap_or(20_000);
    if offset < 0 || !(1..=MAX_ROWS).contains(&limit) {
        return Err(ApiError::bad_request(format!("need offset >= 0 and 1 <= limit <= {MAX_ROWS}")));
    }
    // Without a primary key the physical order is the only total order; the
    // catalogue is append-only between loads, so it is stable for a sync.
    let order = if t.primary_key.is_empty() { "ctid".to_string() } else { column_list(&t.primary_key) };
    let sql = format!(
        "COPY (SELECT {} FROM ec.{} ORDER BY {order} OFFSET {offset} LIMIT {limit}) TO STDOUT",
        column_list(&t.columns),
        quote_ident(&t.name)
    );
    let stream = client.copy_out(sql.as_str()).await?;
    pin_mut!(stream);
    let mut gz = GzEncoder::new(Vec::new(), Compression::fast());
    let mut rows = 0usize;
    let internal = |e: std::io::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        // COPY text escapes newlines inside values, so each 0x0A ends a row.
        rows += chunk.iter().filter(|&&b| b == b'\n').count();
        gz.write_all(&chunk).map_err(internal)?;
    }
    let body = gz.finish().map_err(internal)?;
    if body.len() > MAX_GZIP {
        return Err(ApiError::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("{} gzipped bytes exceed {MAX_GZIP}; ask for fewer rows", body.len()),
        ));
    }
    let mut resp = body.into_response();
    let h = resp.headers_mut();
    h.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/gzip"));
    h.insert(ROWS_HEADER, HeaderValue::from(rows as u64));
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_quoted() {
        assert_eq!(quote_ident("curve"), "\"curve\"");
        assert_eq!(quote_ident("a\"b"), "\"a\"\"b\"");
        assert_eq!(column_list(&["id".into(), "p".into()]), "\"id\", \"p\"");
    }
}
