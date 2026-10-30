//! POST /v1/sql: one read-only statement for agents.
//!
//! Containment, in layers: the extended protocol accepts a single statement;
//! the statement is wrapped as a subquery (so it must be a SELECT / VALUES /
//! WITH ... SELECT); the transaction is READ ONLY with a statement timeout;
//! with ECDB_SQL_ROLE set it runs as that role (ecdb_reader: SELECT only);
//! DISCARD ALL afterwards drops session state a rollback would keep. The
//! login role must itself hold no more than ecdb_reader: a query can reset
//! `role` to the session user inside its own transaction.

use super::{ApiError, ApiResult, Shared};
use crate::json::safe_numbers;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize)]
pub struct SqlRequest {
    pub sql: String,
    pub limit: Option<usize>,
}

fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

pub async fn run(State(state): State<Shared>, Json(req): Json<SqlRequest>) -> ApiResult<Json<Value>> {
    let cfg = &state.config;
    let sql = req.sql.trim().trim_end_matches(';').trim();
    if sql.is_empty() {
        return Err(ApiError::bad_request("sql is empty"));
    }
    let limit = req.limit.unwrap_or(cfg.sql_max_rows.min(1000)).min(cfg.sql_max_rows);
    let mut client = state.read.get().await?;
    let tx = client.build_transaction().read_only(true).start().await?;
    tx.batch_execute(&format!(
        "SET LOCAL statement_timeout = {}; SET LOCAL search_path = ec, public",
        cfg.sql_timeout_ms
    ))
    .await?;
    if let Some(role) = &cfg.sql_role {
        tx.batch_execute(&format!("SET LOCAL ROLE {}", quote_ident(role))).await?;
    }
    let stmt = tx.prepare(sql).await?;
    let columns: Vec<String> = stmt.columns().iter().map(|c| c.name().to_string()).collect();
    let types: Vec<String> = stmt.columns().iter().map(|c| c.type_().name().to_string()).collect();
    if columns.is_empty() {
        return Err(ApiError::bad_request("the statement returns no columns; only queries are accepted"));
    }
    let wrapped = format!("SELECT row_to_json(t)::text FROM (\n{sql}\n) AS t LIMIT {}", limit + 1);
    let rows = tx.query(&wrapped, &[]).await?;
    let truncated = rows.len() > limit;
    let mut out = Vec::with_capacity(rows.len().min(limit));
    for r in rows.iter().take(limit) {
        let text: String = r.get(0);
        let obj: Map<String, Value> = serde_json::from_str(&text)?;
        out.push(Value::Array(
            columns.iter().map(|c| safe_numbers(obj.get(c).cloned().unwrap_or(Value::Null))).collect(),
        ));
    }
    // Dropping the transaction rolls it back, so nothing a query does outlives the request, except
    // session state a rollback keeps (session advisory locks) on the pooled connection.
    drop(tx);
    client.batch_execute("DISCARD ALL").await?;
    Ok(Json(json!({ "columns": columns, "types": types, "rows": out, "row_count": out.len(), "truncated": truncated })))
}
