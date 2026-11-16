//! The Richelot pass against a real Postgres: insert, idempotence, and the
//! refusal (reported, nothing overwritten) of stored rows that disagree with
//! the computed image or isogeny.
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test` (the test drops and recreates schema `ec` there). Example:
//!   ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test richelot_db

use ecdb::db::{pool, DbArgs, TlsMode};
use ecdb::import::cover_lpoly::{class_row, Computed, Method, CLASS_SQL};
use ecdb::import::richelot::{counts, l_from_counts};
use ecdb::import::{bulk, Ctx, Report};
use num_bigint::BigInt;
use serde_json::json;

fn test_url() -> Option<String> {
    let url = std::env::var("ECDB_TEST_DATABASE_URL").ok()?;
    let cfg: tokio_postgres::Config = url.parse().ok()?;
    if !cfg.get_dbname().is_some_and(|d| d.ends_with("_test")) {
        panic!("ECDB_TEST_DATABASE_URL must name a database ending in _test");
    }
    Some(url)
}

fn total(r: &Report, keys: &[&str]) -> u64 {
    keys.iter().map(|k| r.counts.get(*k).copied().unwrap_or(0)).sum()
}

const INSERTS: [&str; 5] =
    ["richelot_curves", "richelot_models", "richelot_zeta", "richelot_isogenies", "richelot_classes"];

#[tokio::test]
async fn richelot_insert_idempotence_and_conflicts() {
    let Some(url) = test_url() else {
        eprintln!("ECDB_TEST_DATABASE_URL not set; skipping");
        return;
    };
    let args = DbArgs { database_url: url, pg_tls: TlsMode::Disable, pg_ca_file: None, pool_size: 2 };
    let pool = pool(&args).unwrap();
    let mut client = pool.get().await.unwrap();
    client.batch_execute("DROP SCHEMA IF EXISTS ec CASCADE").await.unwrap();
    ecdb::migrate::run(&mut client).await.unwrap();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap();
    let ctx = Ctx { client: &client, repo_root, repository: "isogeny-labs/crypto".into(), git_commit: None };

    // A cover source over F_7: y^2 = (x^2 + 1)(x^2 + x + 3)(x^2 + 4x + 6), linked to its isogeny class.
    let p = 7u64;
    let f = [4u64, 4, 3, 2, 0, 5, 1];
    let fs: Vec<String> = f.iter().map(|x| x.to_string()).collect();
    let field: i64 = client
        .query_one(
            "INSERT INTO ec.field (kind, characteristic, degree, order_q, order_bits, representation, element_encoding,
                                   icv1_field, label)
             VALUES ('prime', 7, 1, 7, 3, 'prime', 'decimal', 'fp-7', 'GF(7)') RETURNING id",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    let coeffs = json!({ "f": fs, "h": [] });
    let source: i64 = client
        .query_one(
            "INSERT INTO ec.curve (field_id, genus, model, form, coefficients, family, is_smooth, smoothness_status)
             VALUES ($1, 2, 'hyperelliptic', 'y^2+h(x)*y=f(x)', $2, 'cover', true, 'proved') RETURNING id",
            &[&field, &coeffs],
        )
        .await
        .unwrap()
        .get(0);
    client
        .execute(
            "INSERT INTO ec.hyperelliptic_model (curve_id, f, h, deg_f) VALUES ($1, $2::jsonb->'f', '[]', 6)",
            &[&source, &coeffs],
        )
        .await
        .unwrap();
    let n = counts(p, &f);
    let l = l_from_counts(p, n).unwrap();
    let c = Computed { g: 2, q: BigInt::from(p), p: BigInt::from(p), r: 1, l, method: Method::G2Count };
    let (label, class) = class_row(&c, "proved").unwrap();
    let src: i64 = client
        .query_one(
            "INSERT INTO ec.source (kind, title, uri) VALUES ('computation', 'test', 'test:richelot') RETURNING id",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    let prov = ctx.provenance(src, "test fixture", None).await.unwrap();
    bulk(&client, CLASS_SQL, &[class], prov).await.unwrap();
    client
        .execute(
            "UPDATE ec.abelian_variety SET isogeny_class_id = (SELECT id FROM ec.isogeny_class_fq WHERE label = $1)
             WHERE curve_id = $2",
            &[&label, &source],
        )
        .await
        .unwrap();

    // First pass: every image stored, no problem.
    let mut r1 = Report::default();
    ecdb::import::richelot::import(&ctx, &mut r1).await.unwrap();
    assert!(r1.problems.is_empty(), "{:?}", r1.problems);
    let images = r1.counts["richelot_images_verified"];
    assert!(images > 0);
    assert_eq!(r1.counts["richelot_isogenies"], images);
    assert_eq!(r1.counts.get("richelot_class_mismatches"), Some(&0));

    // Second pass: nothing inserted, no problem.
    let mut r2 = Report::default();
    ecdb::import::richelot::import(&ctx, &mut r2).await.unwrap();
    assert!(r2.problems.is_empty(), "{:?}", r2.problems);
    assert_eq!(total(&r2, &INSERTS), 0, "{:?}", r2.counts);

    // A stored curve_zeta row of an image that disagrees: reported, not overwritten.
    let target: i64 = client
        .query_one("SELECT target_curve_id FROM ec.isogeny WHERE formula = 'richelot' ORDER BY id LIMIT 1", &[])
        .await
        .unwrap()
        .get(0);
    client.execute("UPDATE ec.curve_zeta SET point_counts = '{13,70}' WHERE curve_id = $1", &[&target]).await.unwrap();
    let mut r3 = Report::default();
    ecdb::import::richelot::import(&ctx, &mut r3).await.unwrap();
    assert!(r3.problems.iter().any(|m| m.contains("curve_zeta point_counts")), "{:?}", r3.problems);
    assert!(r3.counts["richelot_existing_conflicts"] >= 1);
    assert_eq!(total(&r3, &INSERTS), 0);
    let kept: Vec<String> = client
        .query_one("SELECT point_counts::text[] FROM ec.curve_zeta WHERE curve_id = $1", &[&target])
        .await
        .unwrap()
        .get(0);
    assert_eq!(kept, vec!["13".to_string(), "70".to_string()]);
    client
        .execute(
            "UPDATE ec.curve_zeta SET point_counts = $2::text[]::numeric[] WHERE curve_id = $1",
            &[&target, &vec![n.0.to_string(), n.1.to_string()]],
        )
        .await
        .unwrap();

    // A stored isogeny with the same external_id but another kernel: reported, not overwritten.
    let ext: String = client
        .query_one(
            "UPDATE ec.isogeny SET kernel_generators = '[]' WHERE id = (SELECT min(id) FROM ec.isogeny) RETURNING external_id",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    let mut r4 = Report::default();
    ecdb::import::richelot::import(&ctx, &mut r4).await.unwrap();
    assert!(r4.problems.iter().any(|m| m.contains(&ext)), "{:?}", r4.problems);
    assert_eq!(r4.counts["richelot_existing_conflicts"], 1);
    assert_eq!(total(&r4, &INSERTS), 0);
    let kg: serde_json::Value = client
        .query_one("SELECT kernel_generators FROM ec.isogeny WHERE external_id = $1", &[&ext])
        .await
        .unwrap()
        .get(0);
    assert_eq!(kg, json!([]));
}
