//! Ciani importer against a real Postgres: first import, idempotent second
//! import, and refusal (never overwrite, never silent skip) of rows that
//! conflict with stored values.
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test` (the test drops and recreates schema `ec` there). Example:
//!   ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test ciani

use ecdb::db::{pool, DbArgs, TlsMode};
use ecdb::import::{ciani, Ctx};

fn test_url() -> Option<String> {
    let url = std::env::var("ECDB_TEST_DATABASE_URL").ok()?;
    let cfg: tokio_postgres::Config = url.parse().ok()?;
    if !cfg.get_dbname().is_some_and(|d| d.ends_with("_test")) {
        panic!("ECDB_TEST_DATABASE_URL must name a database ending in _test");
    }
    Some(url)
}

fn inserted(r: &ecdb::import::Report) -> u64 {
    r.counts
        .iter()
        .filter(|(k, _)| !matches!(k.as_str(), "ciani_rejected_singular" | "ciani_sampled" | "ciani_verified"))
        .map(|(_, v)| *v)
        .sum()
}

#[tokio::test]
async fn conflicting_stored_rows_are_refused_not_overwritten() {
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

    let first = ciani::import(&ctx).await.unwrap();
    assert!(first.problems.is_empty(), "{:?}", first.problems);
    assert_eq!(first.counts["ciani_quartics"], 128);
    assert_eq!(first.counts["ciani_morphisms"], 384);
    let second = ciani::import(&ctx).await.unwrap();
    assert!(second.problems.is_empty(), "{:?}", second.problems);
    assert_eq!(inserted(&second), 0, "second import must insert nothing: {:?}", second.counts);

    // Corrupt three stored values (each belonging to a different quartic) and delete one morphism.
    client
        .batch_execute(
            "
UPDATE ec.abelian_variety SET isogeny_class_id = (SELECT max(id) FROM ec.isogeny_class_fq WHERE g = 3)
 WHERE curve_id = (SELECT min(id) FROM ec.curve WHERE family = 'ciani-quartic');
UPDATE ec.curve_zeta SET point_counts = ARRAY[999]
 WHERE curve_id = (SELECT min(id) FROM ec.curve WHERE family = 'ciani-quotient');
UPDATE ec.curve_morphism SET map = '{\"x\": 1}'
 WHERE id = (SELECT max(id) FROM ec.curve_morphism WHERE construction LIKE 'ciani_quotient_%');
DELETE FROM ec.curve_morphism WHERE id = (SELECT min(id) + 40 FROM ec.curve_morphism
 WHERE construction LIKE 'ciani_quotient_%');",
        )
        .await
        .unwrap();
    let third = ciani::import(&ctx).await.unwrap();
    assert!(third.counts["ciani_refused_conflict"] >= 3, "{:?}", third.counts);
    let joined = third.problems.join("\n");
    assert!(joined.contains("Jacobian linked to class"), "{joined}");
    assert!(joined.contains("stored zeta counts {999}"), "{joined}");
    assert!(joined.contains("stored morphism ciani_quotient_"), "{joined}");
    assert_eq!(third.counts.get("ciani_morphisms"), Some(&1), "the deleted map is restored: {:?}", third.counts);
    assert_eq!(third.counts.get("ciani_morphisms_missing"), None);
    // nothing was overwritten
    let still: i64 = client
        .query_one(
            "SELECT count(*) FROM ec.abelian_variety
              WHERE curve_id = (SELECT min(id) FROM ec.curve WHERE family = 'ciani-quartic')
                AND isogeny_class_id = (SELECT max(id) FROM ec.isogeny_class_fq WHERE g = 3)",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert_eq!(still, 1, "a stored class link must not be overwritten");
    let zeta: String = client
        .query_one(
            "SELECT point_counts::text FROM ec.curve_zeta
              WHERE curve_id = (SELECT min(id) FROM ec.curve WHERE family = 'ciani-quotient')",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert_eq!(zeta, "{999}");
}
