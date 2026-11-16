//! POST /v1/sql on a pool of one connection: the per-request session reset
//! must keep the client's prepared statements valid and still clear the
//! session state a request could leave behind.
//!
//! Regression (2026-11-05): `DISCARD ALL` deallocated the statements
//! tokio-postgres prepares once per connection to resolve non-built-in types
//! (enums, domains, composites, their arrays). The client still held them, so
//! the next result column of a type not yet in its type cache failed with
//! `26000 prepared statement "sN" does not exist`.
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test`. It owns schema `ecdb_sql_session_test` there and nothing else.
//!   ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test sql_session

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ecdb::api::{router, AppState, Config};
use ecdb::db::{pool, DbArgs, TlsMode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

const SCHEMA: &str = "ecdb_sql_session_test";

fn test_url() -> Option<String> {
    let url = std::env::var("ECDB_TEST_DATABASE_URL").ok()?;
    let cfg: tokio_postgres::Config = url.parse().ok()?;
    if !cfg.get_dbname().is_some_and(|d| d.ends_with("_test")) {
        panic!("ECDB_TEST_DATABASE_URL must name a database ending in _test");
    }
    Some(url)
}

async fn sql(app: &axum::Router, query: &str) -> (StatusCode, Value) {
    let body = json!({ "sql": query }).to_string();
    let req = Request::post("/v1/sql").header("content-type", "application/json").body(Body::from(body)).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

/// The request must succeed; returns its first row.
async fn ok_row(app: &axum::Router, query: &str) -> Vec<Value> {
    let (status, out) = sql(app, query).await;
    assert_eq!(status, StatusCode::OK, "{query}: {out}");
    out["rows"][0].as_array().unwrap_or_else(|| panic!("{query}: no rows: {out}")).clone()
}

#[tokio::test]
async fn non_builtin_types_survive_the_session_reset() {
    let Some(url) = test_url() else {
        eprintln!("ECDB_TEST_DATABASE_URL not set; skipping");
        return;
    };
    let args = DbArgs { database_url: url, pg_tls: TlsMode::Disable, pg_ca_file: None, pool_size: 1 };
    let pool = pool(&args).unwrap();
    {
        let setup = pool.get().await.unwrap();
        setup
            .batch_execute(&format!(
                "DROP SCHEMA IF EXISTS {SCHEMA} CASCADE;
                 CREATE SCHEMA {SCHEMA};
                 CREATE TYPE {SCHEMA}.direction AS ENUM ('up', 'down', 'level');
                 CREATE TYPE {SCHEMA}.formula AS ENUM ('velu', 'kohel', 'sqrt_velu');
                 CREATE TYPE {SCHEMA}.edge AS (degree int, dir {SCHEMA}.direction);
                 CREATE DOMAIN {SCHEMA}.degree AS int CHECK (VALUE > 0);
                 CREATE TABLE {SCHEMA}.isogeny (
                     degree {SCHEMA}.degree, direction {SCHEMA}.direction, formula {SCHEMA}.formula);
                 INSERT INTO {SCHEMA}.isogeny VALUES (3, 'up', 'velu');"
            ))
            .await
            .unwrap();
    }
    let state = Arc::new(AppState {
        read: pool.clone(),
        write: None,
        config: Config { read_token: None, write_token: None, sql_role: None, sql_timeout_ms: 5000, sql_max_rows: 100 },
    });
    let app = router(state);

    // 1. Built-in types only (pins the backend to compare at the end).
    let pid = ok_row(&app, "SELECT pg_backend_pid() AS pid, 'x'::text AS t").await[0].clone();
    // 2. An enum column: the client prepares its type-lookup statements here.
    assert_eq!(ok_row(&app, &format!("SELECT direction FROM {SCHEMA}.isogeny")).await, vec![json!("up")]);
    // 3. The same enum again (resolved from the client's type cache).
    assert_eq!(ok_row(&app, &format!("SELECT direction FROM {SCHEMA}.isogeny")).await, vec![json!("up")]);
    // 4. A fresh enum: needs the lookup statements prepared in request 2.
    //    Under DISCARD ALL this was HTTP 400 26000 "prepared statement ... does not exist".
    assert_eq!(ok_row(&app, &format!("SELECT formula FROM {SCHEMA}.isogeny")).await, vec![json!("velu")]);
    // 5. Further non-built-in kinds, each a fresh lookup: domain, composite, enum array.
    assert_eq!(ok_row(&app, &format!("SELECT degree FROM {SCHEMA}.isogeny")).await, vec![json!(3)]);
    assert_eq!(
        ok_row(&app, &format!("SELECT ROW(degree, direction)::{SCHEMA}.edge AS e FROM {SCHEMA}.isogeny")).await,
        vec![json!({ "degree": 3, "dir": "up" })]
    );
    assert_eq!(
        ok_row(&app, &format!("SELECT ARRAY[direction, 'down'] AS a FROM {SCHEMA}.isogeny")).await,
        vec![json!(["up", "down"])]
    );
    // All of the above ran on one backend.
    assert_eq!(ok_row(&app, "SELECT pg_backend_pid()").await[0], pid);

    // The reset still clears what a request can leave on the session:
    // a session advisory lock taken by the query survives its rollback.
    assert_eq!(ok_row(&app, "SELECT pg_try_advisory_lock(424242)").await, vec![json!(true)]);
    let left = String::from(
        "SELECT (SELECT count(*) FROM pg_locks WHERE locktype = 'advisory' AND pid = pg_backend_pid()),
                (SELECT count(*) FROM pg_prepared_statements WHERE from_sql),
                (SELECT count(*) FROM pg_class WHERE relpersistence = 't' AND relname = 'leak'),
                current_setting('application_name'), current_user = session_user",
    );
    assert_eq!(ok_row(&app, &left).await, vec![json!(0), json!(0), json!(0), json!(""), json!(true)]);

    // Session state set outside a request (SQL-level PREPARE, temp table, GUC,
    // advisory lock, role) is gone after the next request too.
    {
        let c = pool.get().await.unwrap();
        c.batch_execute(
            "PREPARE leak AS SELECT 1; CREATE TEMP TABLE leak (x int); SET application_name = 'leak';
             SELECT pg_advisory_lock(424243); SET ROLE pg_read_all_data",
        )
        .await
        .unwrap();
    }
    ok_row(&app, "SELECT 1").await;
    assert_eq!(ok_row(&app, &left).await, vec![json!(0), json!(0), json!(0), json!(""), json!(true)]);
    // And the client's statements are still valid afterwards.
    assert_eq!(ok_row(&app, &format!("SELECT ARRAY[formula] AS f FROM {SCHEMA}.isogeny")).await, vec![json!(["velu"])]);
    assert_eq!(ok_row(&app, "SELECT pg_backend_pid()").await[0], pid);

    pool.get().await.unwrap().batch_execute(&format!("DROP SCHEMA {SCHEMA} CASCADE")).await.unwrap();
}
