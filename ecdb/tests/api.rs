//! End-to-end test against a real Postgres: migrate twice, import the
//! registry, the isogeny walks and the trait catalogue from this checkout,
//! then query through the router.
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test` (the test drops and recreates schema `ec` there). Example:
//!   ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test api

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ecdb::api::{router, AppState, Config};
use ecdb::db::{pool, DbArgs, TlsMode};
use ecdb::import::{self, Ctx};
use http_body_util::BodyExt;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

fn test_url() -> Option<String> {
    let url = std::env::var("ECDB_TEST_DATABASE_URL").ok()?;
    let cfg: tokio_postgres::Config = url.parse().ok()?;
    if !cfg.get_dbname().is_some_and(|d| d.ends_with("_test")) {
        panic!("ECDB_TEST_DATABASE_URL must name a database ending in _test");
    }
    Some(url)
}

async fn get(app: &axum::Router, uri: &str) -> (StatusCode, Value) {
    let res = app.clone().oneshot(Request::get(uri).body(Body::empty()).unwrap()).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn post(app: &axum::Router, uri: &str, body: &str) -> (StatusCode, Value) {
    let req = Request::post(uri).header("content-type", "application/json").body(Body::from(body.to_string())).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn migrate_import_and_query() {
    let Some(url) = test_url() else {
        eprintln!("ECDB_TEST_DATABASE_URL not set; skipping");
        return;
    };
    let args = DbArgs { database_url: url, pg_tls: TlsMode::Disable, pg_ca_file: None, pool_size: 4 };
    let pool = pool(&args).unwrap();
    let mut client = pool.get().await.unwrap();
    client.batch_execute("DROP SCHEMA IF EXISTS ec CASCADE").await.unwrap();
    assert_eq!(ecdb::migrate::run(&mut client).await.unwrap().len(), ecdb::migrate::MIGRATIONS.len());
    assert!(ecdb::migrate::run(&mut client).await.unwrap().is_empty(), "second migrate must be a no-op");

    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap();
    let ctx = Ctx { client: &client, repo_root, repository: "isogeny-labs/crypto".into(), git_commit: None };
    let reg = import::registry::import(&ctx).await.unwrap();
    assert!(reg.problems.is_empty(), "registry problems: {:?}", reg.problems);
    let walks: Vec<String> = import::walks::DEFAULT_FILES.iter().map(|s| s.to_string()).collect();
    for r in import::walks::import(&ctx, &walks).await.unwrap() {
        assert!(r.problems.is_empty(), "walk problems: {:?}", r.problems);
    }
    import::traits::import(&ctx).await.unwrap();
    // Idempotent: a second registry import adds nothing.
    let again = import::registry::import(&ctx).await.unwrap();
    assert_eq!(again.counts.get("curves"), Some(&0));

    let state = Arc::new(AppState {
        read: pool.clone(),
        write: None,
        config: Config { read_token: None, write_token: None, sql_role: None, sql_timeout_ms: 5000, sql_max_rows: 100 },
    });
    let app = router(state);

    let (s, doc) = get(&app, "/v1/curves/secp256k1").await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(doc["genus1_invariants"]["j_invariant"], "0");
    assert_eq!(doc["frobenius"]["cm_discriminant"], -3);
    assert_eq!(doc["endomorphism_ring"]["is_maximal"], true);
    // 256-bit group order survives as an exact decimal string
    assert_eq!(
        doc["isogeny_class"]["order_value"],
        "115792089237316195423570985008687907852837564279074904382605163141518161494337"
    );

    let (s, list) = get(&app, "/v1/curves?family=koblitz&field_bits_min=163&field_bits_max=163").await;
    assert_eq!(s, StatusCode::OK);
    assert!(list["items"].as_array().unwrap().iter().any(|c| c["standard_names"].to_string().contains("sect163k1")));

    let (s, graph) = get(&app, "/v1/curves/P-256/graph?depth=1").await;
    assert_eq!(s, StatusCode::OK);
    assert!(!graph["edges"].as_array().unwrap().is_empty());

    let (s, _) = get(&app, "/v1/curves/not-a-curve").await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    let (s, out) = post(&app, "/v1/sql", r#"{"sql": "select count(*) as n from curve where genus = 1"}"#).await;
    assert_eq!(s, StatusCode::OK);
    assert!(out["rows"][0][0].as_i64().unwrap() > 20000);
    let (s, _) = post(&app, "/v1/sql", r#"{"sql": "select 1; drop schema ec cascade"}"#).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    // Duplicate column names keep their own values.
    let (_, out) = post(&app, "/v1/sql", r#"{"sql": "select 1 as x, 2 as x"}"#).await;
    assert_eq!(out["rows"][0], serde_json::json!([1, 2]));

    // l-torsion is per curve: two curves of one GF(2^17) class sit at different
    // levels of the 17-volcano (traits.json: Jordan block vs scalar).
    let torsion_sql =
        "select c.icv1_slug, t.frobenius_action from curve_ell_torsion t join curve c on c.id = t.curve_id \
        where t.ell = 17 and c.icv1_slug in ('icv1-f2m17-t209-0eba1a1e', 'icv1-f2m17-t209-e5c8e740') order by 1";
    let (_, out) = post(&app, "/v1/sql", &serde_json::json!({ "sql": torsion_sql }).to_string()).await;
    assert_eq!(
        out["rows"],
        serde_json::json!([["icv1-f2m17-t209-0eba1a1e", "jordan"], ["icv1-f2m17-t209-e5c8e740", "scalar"]])
    );
    let (s, _) = post(&app, "/v1/evidence", r#"{}"#).await;
    assert_eq!(s, StatusCode::FORBIDDEN);
}
