//! `import covers` L-polynomials against a real database: idempotence, the
//! Schoof path end to end, and stored rows that contradict the computation
//! (refused and reported, never overwritten).
//!
//! Runs only when ECDB_TEST_DATABASE_URL names a database whose name ends in
//! `_test`. It drops and rebuilds schema `ec` there (all migrations).
//!   ECDB_TEST_DATABASE_URL='postgres://postgres@localhost/ecdb_test' cargo test --test cover_lpoly_db

use ecdb::import::schoof::{self, PrimeField};
use ecdb::import::{cover_lpoly, Ctx, Report};
use num_bigint::BigUint;
use tokio_postgres::{Client, NoTls};

fn test_url() -> Option<String> {
    let url = std::env::var("ECDB_TEST_DATABASE_URL").ok()?;
    let cfg: tokio_postgres::Config = url.parse().ok()?;
    if !cfg.get_dbname().is_some_and(|d| d.ends_with("_test")) {
        panic!("ECDB_TEST_DATABASE_URL must name a database ending in _test");
    }
    Some(url)
}

/// A field, a target elliptic curve with trace `t` and its class, and a
/// genus-2 cover source v^2 = u^6 + g2 u^4 + g1 u^2 + g0 over F_p; returns the
/// source's curve id.
async fn source(c: &Client, tag: &str, p: &str, g: [&str; 3], t: &str) -> i64 {
    let row = c
        .query_one(
            "WITH f AS (
               INSERT INTO ec.field (kind, characteristic, degree, order_q, representation, element_encoding,
                                     primality_status, icv1_field)
               VALUES ('prime', $1::text::numeric, 1, $1::text::numeric, 'prime', 'decimal', 'proved', 'fp-' || $1 || '-' || $2)
               RETURNING id),
             e AS (
               INSERT INTO ec.curve (field_id, genus, model, form, coefficients)
               SELECT id, 1, 'short_weierstrass', 'y^2=x^3+a*x+b', jsonb_build_object('tag', $2) FROM f RETURNING id, field_id),
             ic AS (
               INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label, status)
               VALUES ($1::text::numeric, $1::text::numeric, 1, 1,
                       ARRAY[1, -($3::text::numeric), $1::text::numeric], 'test-target-' || $2, 'proved')
               ON CONFLICT DO NOTHING RETURNING id),
             ic2 AS (
               SELECT id FROM ic UNION ALL
               SELECT id FROM ec.isogeny_class_fq
                WHERE q = $1::text::numeric AND l_polynomial = ARRAY[1, -($3::text::numeric), $1::text::numeric]::ec.zint[]
               LIMIT 1),
             eav AS (
               INSERT INTO ec.abelian_variety (field_id, dimension, kind, curve_id, isogeny_class_id)
               SELECT e.field_id, 1, 'jacobian', e.id, ic2.id FROM e, ic2 RETURNING id),
             h AS (
               INSERT INTO ec.curve (field_id, genus, model, form, coefficients, family)
               SELECT id, 2, 'hyperelliptic', 'y^2=f(x)', jsonb_build_object('tag', $2), 'cover' FROM f RETURNING id, field_id),
             hm AS (
               INSERT INTO ec.hyperelliptic_model (curve_id, f, h, deg_f)
               SELECT id, jsonb_build_array($4::text, '0', $5::text, '0', $6::text, '0', '1'), '[]', 6 FROM h RETURNING curve_id),
             hav AS (
               INSERT INTO ec.abelian_variety (field_id, dimension, kind, curve_id)
               SELECT field_id, 2, 'jacobian', id FROM h RETURNING id),
             m AS (
               INSERT INTO ec.curve_morphism (source_curve_id, target_curve_id, kind, construction)
               SELECT h.id, e.id, 'hyperelliptic_pullback', 'prime_quadratic_pullback_v1' FROM h, e RETURNING id)
             SELECT h.id FROM h, hm, hav, eav, m",
            &[&p, &tag, &t, &g[0], &g[1], &g[2]],
        )
        .await
        .unwrap();
    row.get(0)
}

async fn import(c: &Client) -> Report {
    let ctx = Ctx { client: c, repo_root: ".".into(), repository: "test".into(), git_commit: None };
    let mut report = Report::default();
    cover_lpoly::import(&ctx, &mut report).await.unwrap();
    report
}

fn n(r: &Report, key: &str) -> u64 {
    r.counts.get(key).copied().unwrap_or(0)
}

/// (abelian_variety.isogeny_class_id, curve_zeta.isogeny_class_id, curve_zeta.point_counts) of a curve.
async fn state(c: &Client, id: i64) -> (Option<i64>, Option<i64>, Option<Vec<String>>) {
    let r = c
        .query_one(
            "SELECT av.isogeny_class_id, z.isogeny_class_id, z.point_counts::text[]
               FROM ec.abelian_variety av LEFT JOIN ec.curve_zeta z ON z.curve_id = av.curve_id
              WHERE av.curve_id = $1",
            &[&id],
        )
        .await
        .unwrap();
    (r.get(0), r.get(1), r.get(2))
}

async fn l_poly(c: &Client, class: i64) -> Vec<String> {
    c.query_one("SELECT l_polynomial::text[] FROM ec.isogeny_class_fq WHERE id = $1", &[&class]).await.unwrap().get(0)
}

#[tokio::test]
async fn cover_lpoly_import_is_idempotent_and_refuses_conflicts() {
    let Some(url) = test_url() else {
        eprintln!("ECDB_TEST_DATABASE_URL not set; skipping");
        return;
    };
    let (mut c, conn) = tokio_postgres::connect(&url, NoTls).await.unwrap();
    tokio::spawn(conn);
    c.batch_execute("DROP SCHEMA IF EXISTS ec CASCADE").await.unwrap();
    ecdb::migrate::run(&mut c).await.unwrap();

    // The live row icv1-fp24-t553 (baby-step giant-step path): L = [1, -2156, 17718717, ...].
    let small = ("8416129", ["1713668", "1040051", "0"], "553");
    let a = source(&c, "a", small.0, small.1, small.2).await;
    // A Schoof-path source, p = 2^89 - 1; the target trace from the same module.
    let p = (BigUint::from(1u32) << 89u32) - 1u32;
    let field = PrimeField::new(p.clone());
    let g = [5u32, 7, 3].map(BigUint::from);
    let (sa, sb) = schoof::short_weierstrass(&field, &g[2], &g[1], &g[0]);
    let t = schoof::frobenius_trace(&field, &sa, &sb, &schoof::IMPORT).unwrap().to_string();
    let b = source(&c, "b", &p.to_string(), ["5", "7", "3"], &t).await;
    // C: the Jacobian is already linked to a different class.
    let cc = source(&c, "c", small.0, small.1, small.2).await;
    let q2 = 8416129i64 * 8416129;
    let x: i64 = c
        .query_one(
            "INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label)
             VALUES (8416129, 8416129, 1, 2, ARRAY[1, 0, 0, 0, $1::bigint], 'test-other') RETURNING id",
            &[&q2],
        )
        .await
        .unwrap()
        .get(0);
    c.execute("UPDATE ec.abelian_variety SET isogeny_class_id = $1 WHERE curve_id = $2", &[&x, &cc]).await.unwrap();
    // D: a zeta row with a different point count.
    let d = source(&c, "d", small.0, small.1, small.2).await;
    c.execute("INSERT INTO ec.curve_zeta (curve_id, point_counts, order_method) VALUES ($1, '{1,2}', 'other')", &[&d])
        .await
        .unwrap();

    let r1 = import(&c).await;
    assert_eq!(n(&r1, "cover_lpoly_computed"), 4, "{r1:?}");
    assert_eq!(n(&r1, "cover_lpoly_crosscheck_failures"), 0, "{r1:?}");
    assert_eq!(n(&r1, "cover_lpoly_refused_conflict"), 2, "{r1:?}");
    assert_eq!(n(&r1, "cover_lpoly_jacobians_linked"), 2, "{r1:?}");
    assert_eq!(n(&r1, "cover_lpoly_zeta"), 2, "{r1:?}");
    assert_eq!(r1.problems.len(), 2, "{r1:?}");
    for id in [cc, d] {
        assert!(r1.problems.iter().any(|m| m.contains(&format!("curve id {id}:"))), "{r1:?}");
    }

    let (sa_av, sa_z, sa_pc) = state(&c, a).await;
    let class_a = sa_av.unwrap();
    assert_eq!(sa_z, Some(class_a));
    let q = 8416129i64;
    let want: Vec<String> = [1, -2156, 17718717, -2156 * q, q * q].iter().map(|v| v.to_string()).collect();
    assert_eq!(l_poly(&c, class_a).await, want);
    let (sb_av, sb_z, _) = state(&c, b).await;
    assert_eq!(sb_av, sb_z);
    let lb = l_poly(&c, sb_av.unwrap()).await;
    let (tt, tp) = cover_lpoly::quotient_traces_schoof(&field, &g[0], &g[1], &g[2], &schoof::IMPORT).unwrap();
    assert_eq!(tt.to_string(), t);
    assert_eq!(lb[1], (-(tt.clone() + &tp)).to_string());
    let method: String =
        c.query_one("SELECT order_method FROM ec.curve_zeta WHERE curve_id = $1", &[&b]).await.unwrap().get(0);
    assert!(method.contains("Schoof"), "{method}");
    // Refused: nothing overwritten, nothing added.
    assert_eq!(state(&c, cc).await, (Some(x), None, None));
    assert_eq!(state(&c, d).await, (None, None, Some(vec!["1".to_string(), "2".to_string()])));

    // E: a zeta row that agrees (same counts) but lacks the class: filled.
    let e = source(&c, "e", small.0, small.1, small.2).await;
    c.execute(
        "INSERT INTO ec.curve_zeta (curve_id, point_counts, order_method) VALUES ($1, $2::text[]::numeric[], 'other')",
        &[&e, &sa_pc.clone().unwrap()],
    )
    .await
    .unwrap();
    let r2 = import(&c).await;
    assert_eq!(n(&r2, "cover_lpoly_computed"), 3, "{r2:?}"); // C, D again and E; A and B are complete
    assert_eq!(n(&r2, "cover_lpoly_refused_conflict"), 2, "{r2:?}");
    assert_eq!(n(&r2, "cover_lpoly_jacobians_linked"), 1, "{r2:?}");
    assert_eq!(n(&r2, "cover_lpoly_zeta"), 1, "{r2:?}");
    assert_eq!(state(&c, e).await, (Some(class_a), Some(class_a), sa_pc.clone()));
    let me: String =
        c.query_one("SELECT order_method FROM ec.curve_zeta WHERE curve_id = $1", &[&e]).await.unwrap().get(0);
    assert_eq!(me, "other");
    assert_eq!(state(&c, a).await, (Some(class_a), Some(class_a), sa_pc));

    // Third run: no writes at all.
    let r3 = import(&c).await;
    assert_eq!(n(&r3, "cover_lpoly_computed"), 2, "{r3:?}");
    assert_eq!(n(&r3, "cover_lpoly_classes"), 0, "{r3:?}");
    assert_eq!(n(&r3, "cover_lpoly_jacobians_linked"), 0, "{r3:?}");
    assert_eq!(n(&r3, "cover_lpoly_zeta"), 0, "{r3:?}");
    assert_eq!(state(&c, cc).await, (Some(x), None, None));
}
