//! Focused PostgreSQL integration test for the saved research/ML layer.
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test`. The test drops and recreates schema `ec` in that database.

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use ecdb::api::{router, AppState, Config};
use ecdb::db::{pool, DbArgs, TlsMode};
use ecdb::import::{self, Ctx};
use http_body_util::BodyExt;
use serde_json::{json, Value};
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

async fn json_response(res: axum::response::Response) -> (StatusCode, Value) {
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn get(app: &axum::Router, uri: &str) -> (StatusCode, Value) {
    let res = app.clone().oneshot(Request::get(uri).body(Body::empty()).unwrap()).await.unwrap();
    json_response(res).await
}

async fn post(app: &axum::Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, "Bearer test-write")
        .body(Body::from(body.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    json_response(res).await
}

#[tokio::test]
async fn migrate_import_and_use_research_api() {
    let Some(url) = test_url() else {
        eprintln!("ECDB_TEST_DATABASE_URL not set; skipping");
        return;
    };
    let args = DbArgs { database_url: url, pg_tls: TlsMode::Disable, pg_ca_file: None, pool_size: 4 };
    let pool = pool(&args).unwrap();
    let mut client = pool.get().await.unwrap();
    client.batch_execute("DROP SCHEMA IF EXISTS ec CASCADE").await.unwrap();
    assert_eq!(ecdb::migrate::run(&mut client).await.unwrap().len(), ecdb::migrate::MIGRATIONS.len());
    assert!(ecdb::migrate::run(&mut client).await.unwrap().is_empty());

    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap();
    let ctx = Ctx { client: &client, repo_root, repository: "isogeny-labs/crypto".into(), git_commit: None };
    let first = import::research::import(&ctx).await.unwrap();
    assert!(first.iter().all(|r| r.problems.is_empty()), "research import problems: {first:?}");
    assert_eq!(first.iter().map(|r| r.counts.get("feature_definitions").copied().unwrap_or(0)).sum::<u64>(), 91);
    let second = import::research::import(&ctx).await.unwrap();
    assert!(second.iter().all(|r| r.problems.is_empty()), "research reimport problems: {second:?}");
    for report in second {
        for key in ["feature_definitions", "sweeps", "cells", "datasets", "dataset_features"] {
            assert_eq!(
                report.counts.get(key).copied().unwrap_or(0),
                0,
                "research import was not idempotent: {report:?}"
            );
        }
    }

    // A compact fixture is enough to exercise curve resolution, all three ML
    // views, observations and factor-base object references. It deliberately
    // does not claim arithmetic facts that were not computed.
    let provenance_id: i64 =
        client.query_one("SELECT provenance_id FROM ec.feature_definition LIMIT 1", &[]).await.unwrap().get(0);
    let field_id: i64 = client
        .query_one(
            "INSERT INTO ec.field
             (kind,characteristic,degree,order_q,order_bits,representation,element_encoding,
              icv1_field,label,irreducibility_status,primality_status,provenance_id)
             VALUES ('prime',101,1,101,7,'prime','decimal','fp-101','GF(101)',
                     'not_applicable','proved',$1) RETURNING id",
            &[&provenance_id],
        )
        .await
        .unwrap()
        .get(0);
    let curve_id: i64 = client
        .query_one(
            "INSERT INTO ec.curve
             (field_id,genus,model,form,coefficients,equation,family,icv1_slug,
              is_smooth,smoothness_status,provenance_id)
             VALUES ($1,1,'short_weierstrass','y^2=x^3+a*x+b',
                     '{\"a\":\"2\",\"b\":\"3\"}'::jsonb,'y^2 = x^3 + 2x + 3',
                     'prime','icv1-fp101-t0-deadbeef',true,'proved',$2) RETURNING id",
            &[&field_id, &provenance_id],
        )
        .await
        .unwrap()
        .get(0);
    let factor_base_id: i64 = client
        .query_one(
            "INSERT INTO ec.factor_base
             (scheme,key,curve_id,curve_slug,family,params,signed_points,abscissae,columns,
              dimension,record,provenance_id)
             VALUES ('spec','FBS1htest00000001',$1,'icv1-fp101-t0-deadbeef','prime-abscissa',
                     '{\"bound\":8}'::jsonb,16,8,8,1,'{}'::jsonb,$2) RETURNING id",
            &[&curve_id, &provenance_id],
        )
        .await
        .unwrap()
        .get(0);
    client
        .execute(
            "INSERT INTO ec.factor_base_measurement
             (fbm_id,factor_base_id,curve_slug,oracle,solver,linalg,instrument,config,provenance_id)
             VALUES ('FBM1htest0000001',$1,'icv1-fp101-t0-deadbeef','fixture','fixture',
                     'fixture','fixture','{}'::jsonb,$2)",
            &[&factor_base_id, &provenance_id],
        )
        .await
        .unwrap();

    let state = Arc::new(AppState {
        read: pool.clone(),
        write: Some(pool.clone()),
        config: Config {
            read_token: None,
            write_token: Some("test-write".into()),
            sql_role: None,
            sql_timeout_ms: 5000,
            sql_max_rows: 100,
        },
    });
    let app = router(state);

    let (status, features) = get(&app, "/v1/features?scope=curve&limit=100").await;
    assert_eq!(status, StatusCode::OK);
    assert!(features["items"].as_array().unwrap().len() >= 40);
    let (status, sweep) = get(&app, "/v1/sweeps/prime-ladder").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(sweep["cells"].as_array().unwrap().len(), 7);
    let (status, dataset) = get(&app, "/v1/datasets/fbm1-solver-outcomes").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(dataset["subject_scope"], "factor_base_measurement");
    assert_eq!(dataset["features"].as_array().unwrap().len(), 14);

    let (status, observation) = post(
        &app,
        "/v1/features/observations",
        json!({
            "curve":"icv1-fp101-t0-deadbeef",
            "feature_key":"graph.isogeny_count",
            "value":0,
            "status":"proved",
            "context":{"fixture":true},
            "agent":{"name":"research-integration-test","method":"fixture assertion"}
        }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{observation}");
    assert_eq!(observation["scope"], "curve");

    // The database enforces the same feature scope/type contract even when a
    // writer bypasses HTTP and inserts directly.
    let graph_feature_id: i64 = client
        .query_one("SELECT id FROM ec.feature_definition WHERE feature_key='graph.isogeny_count' AND version=1", &[])
        .await
        .unwrap()
        .get(0);
    let wrong_scope = client
        .execute(
            "INSERT INTO ec.feature_observation
             (feature_definition_id,factor_base_id,value,context,status,provenance_id)
             VALUES ($1,$2,'0'::jsonb,'{}'::jsonb,'proved',$3)",
            &[&graph_feature_id, &factor_base_id, &provenance_id],
        )
        .await
        .unwrap_err();
    assert!(wrong_scope.as_db_error().unwrap().message().contains("expects scope curve"));
    let wrong_type = client
        .execute(
            "INSERT INTO ec.feature_observation
             (feature_definition_id,curve_id,value,context,status,provenance_id)
             VALUES ($1,$2,'\"not-an-integer\"'::jsonb,'{}'::jsonb,'proved',$3)",
            &[&graph_feature_id, &curve_id, &provenance_id],
        )
        .await
        .unwrap_err();
    assert!(wrong_type.as_db_error().unwrap().message().contains("expects data type integer"));

    let digest = "a".repeat(64);
    let (status, artifact) = post(
        &app,
        "/v1/factor-base-artifacts",
        json!({
            "factor_base":"FBS1htest00000001",
            "role":"relations",
            "is_primary":true,
            "sha256":digest,
            "size_bytes":4096,
            "media_type":"application/zstd",
            "storage_uri":"s3://ecdb-test/factor-bases/relations.zst",
            "description":"integration fixture",
            "agent":{"name":"research-integration-test","method":"fixture assertion"}
        }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{artifact}");
    assert_eq!(artifact["factor_base_id"], factor_base_id);

    let (status, curve) = get(&app, "/v1/curves/icv1-fp101-t0-deadbeef").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(curve["ml_features"]["factor_base_count"], 1);
    assert_eq!(curve["feature_observations"][0]["feature_key"], "graph.isogeny_count");
    assert_eq!(
        curve["factor_bases"][0]["artifacts"][0]["artifact"]["storage_uri"],
        "s3://ecdb-test/factor-bases/relations.zst"
    );

    let (status, curve_features) = get(&app, "/v1/curves/icv1-fp101-t0-deadbeef/features").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(curve_features["curve_id"], curve_id);
    assert_eq!(curve_features["observations"].as_array().unwrap().len(), 1);

    let (status, factor_base) = get(&app, "/v1/factor-bases/FBS1htest00000001").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(factor_base["ml_features"]["has_object_store_artifact"], true);
    assert_eq!(factor_base["artifacts"][0]["artifact"]["size_bytes"], 4096);

    let measurement_rows: i64 =
        client.query_one("SELECT count(*) FROM ec.v_factor_base_measurement_ml_features", &[]).await.unwrap().get(0);
    assert_eq!(measurement_rows, 1);
}
