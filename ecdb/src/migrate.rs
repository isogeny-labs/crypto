//! Embedded, immutable migrations. An applied migration whose bytes changed
//! stops the runner: corrections are new migrations.

use anyhow::{bail, Result};
use tokio_postgres::Client;

pub struct Migration {
    pub version: i32,
    pub name: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    Migration { version: 1, name: "foundations", sql: include_str!("../migrations/0001_foundations.sql") },
    Migration { version: 2, name: "fields", sql: include_str!("../migrations/0002_fields.sql") },
    Migration { version: 3, name: "curves", sql: include_str!("../migrations/0003_curves.sql") },
    Migration { version: 4, name: "arithmetic", sql: include_str!("../migrations/0004_arithmetic.sql") },
    Migration { version: 5, name: "maps", sql: include_str!("../migrations/0005_maps.sql") },
    Migration { version: 6, name: "polynomials", sql: include_str!("../migrations/0006_polynomials.sql") },
    Migration {
        version: 7,
        name: "global_security_evidence",
        sql: include_str!("../migrations/0007_global_security_evidence.sql"),
    },
    Migration {
        version: 8,
        name: "views_functions_roles",
        sql: include_str!("../migrations/0008_views_functions_roles.sql"),
    },
    Migration { version: 9, name: "measurements", sql: include_str!("../migrations/0009_measurements.sql") },
    Migration { version: 10, name: "volcano_key", sql: include_str!("../migrations/0010_volcano_key.sql") },
    Migration { version: 11, name: "factor_bases", sql: include_str!("../migrations/0011_factor_bases.sql") },
];

pub fn schema_version() -> i32 {
    MIGRATIONS.last().map(|m| m.version).unwrap_or(0)
}

/// Apply pending migrations under an advisory lock; returns the versions applied.
pub async fn run(client: &mut Client) -> Result<Vec<i32>> {
    client.batch_execute("SELECT pg_advisory_lock(hashtext('ecdb-migrate'))").await?;
    let result = apply(client).await;
    client.batch_execute("SELECT pg_advisory_unlock(hashtext('ecdb-migrate'))").await?;
    result
}

async fn apply(client: &mut Client) -> Result<Vec<i32>> {
    client
        .batch_execute(
            "CREATE SCHEMA IF NOT EXISTS ec;
             CREATE TABLE IF NOT EXISTS ec.schema_migrations (
               version integer PRIMARY KEY, name text NOT NULL,
               sha256 char(64) NOT NULL, applied_at timestamptz NOT NULL DEFAULT now())",
        )
        .await?;
    let applied: Vec<(i32, String)> = client
        .query("SELECT version, sha256::text FROM ec.schema_migrations ORDER BY version", &[])
        .await?
        .iter()
        .map(|r| (r.get(0), r.get(1)))
        .collect();
    let mut done = Vec::new();
    for m in MIGRATIONS {
        let sha = crate::math::sha256_hex(m.sql);
        if let Some((_, have)) = applied.iter().find(|(v, _)| *v == m.version) {
            if *have != sha {
                bail!("migration {:04}_{} changed after it was applied (db {have}, file {sha})", m.version, m.name);
            }
            continue;
        }
        let tx = client.transaction().await?;
        tx.batch_execute(m.sql).await?;
        tx.execute(
            "INSERT INTO ec.schema_migrations (version, name, sha256) VALUES ($1, $2, $3)",
            &[&m.version, &m.name, &sha],
        )
        .await?;
        tx.commit().await?;
        tracing::info!(version = m.version, name = m.name, "applied migration");
        done.push(m.version);
    }
    Ok(done)
}
