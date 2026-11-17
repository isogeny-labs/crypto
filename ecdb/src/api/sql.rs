//! POST /v1/sql: one read-only statement for agents.
//!
//! Containment, in layers: the extended protocol accepts a single statement;
//! the statement is wrapped as a subquery (so it must be a SELECT / VALUES /
//! WITH ... SELECT); the transaction is READ ONLY with a statement timeout;
//! with ECDB_SQL_ROLE set it runs as that role (ecdb_reader: SELECT only);
//! a session reset afterwards ([`SESSION_RESET`]), on every exit path, drops
//! session state a rollback would keep (advisory locks above all), and a
//! connection it cannot clean leaves the pool. The login role must itself
//! hold no more than ecdb_reader: a query can reset `role` to the session user
//! inside its own transaction.

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

/// `DISCARD ALL` without its `DEALLOCATE ALL`: every SQL-level prepared
/// statement goes (`from_sql`), the client's protocol-level ones stay.
///
/// tokio-postgres prepares named statements once per connection to look up
/// non-built-in types (enums, domains, composites, their arrays) and keeps
/// them for the connection's life. `DISCARD ALL` deallocated them behind the
/// client's back, and the next result column of a type not yet in its cache
/// failed with `26000 prepared statement "sN" does not exist`
/// (`tests/sql_session.rs`). `Client::clear_type_cache` does not help: it
/// empties the resolved types and keeps those statements. deadpool's
/// `RecyclingMethod::Clean` uses the same list for the same reason.
pub const SESSION_RESET: &str = "\
    CLOSE ALL; \
    SET SESSION AUTHORIZATION DEFAULT; \
    RESET ALL; \
    UNLISTEN *; \
    SELECT pg_advisory_unlock_all(); \
    DISCARD PLANS; \
    DISCARD TEMP; \
    DISCARD SEQUENCES; \
    DO $$DECLARE n text; BEGIN \
      FOR n IN SELECT name FROM pg_prepared_statements WHERE from_sql LOOP \
        EXECUTE format('DEALLOCATE %I', n); \
      END LOOP; \
    END$$";

fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

pub async fn run(State(state): State<Shared>, Json(req): Json<SqlRequest>) -> ApiResult<Json<Value>> {
    let sql = req.sql.trim().trim_end_matches(';').trim();
    if sql.is_empty() {
        return Err(ApiError::bad_request("sql is empty"));
    }
    let mut client = state.read.get().await?;
    let result = query(&mut client, &state.config, sql, req.limit).await;
    // The transaction inside `query` is dropped (rolled back) on every path by
    // now; session advisory locks survive a rollback, so reset the session too.
    if client.batch_execute(SESSION_RESET).await.is_err() {
        drop(deadpool_postgres::Object::take(client));
    }
    result.map(Json)
}

async fn query(
    client: &mut deadpool_postgres::Object,
    cfg: &super::Config,
    sql: &str,
    limit: Option<usize>,
) -> ApiResult<Value> {
    let limit = limit.unwrap_or(cfg.sql_max_rows.min(1000)).min(cfg.sql_max_rows);
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
    // Rename the columns positionally so duplicate names (SELECT 1 AS x, 2 AS x) survive row_to_json.
    let aliases: Vec<String> = (0..columns.len()).map(|i| format!("c{i}")).collect();
    let wrapped =
        format!("SELECT row_to_json(t)::text FROM (\n{sql}\n) AS t({}) LIMIT {}", aliases.join(", "), limit + 1);
    let rows = tx.query(&wrapped, &[]).await?;
    let truncated = rows.len() > limit;
    let mut out = Vec::with_capacity(rows.len().min(limit));
    for r in rows.iter().take(limit) {
        let text: String = r.get(0);
        let obj: Map<String, Value> = serde_json::from_str(&text)?;
        out.push(Value::Array(
            aliases.iter().map(|a| safe_numbers(obj.get(a).cloned().unwrap_or(Value::Null))).collect(),
        ));
    }
    Ok(json!({ "columns": columns, "types": types, "rows": out, "row_count": out.len(), "truncated": truncated }))
}
