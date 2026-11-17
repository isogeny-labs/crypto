//! AWS Lambda entry points (deploy/README.md): the HTTP API behind a function
//! URL, and an admin function that runs one maintenance action per
//! invocation (status, migrate, login-role bootstrap, one importer).

use crate::api::{router, Shared};
use crate::import::{self, Ctx, Selection};
use anyhow::{anyhow, Context, Result};
use deadpool_postgres::Pool;
use lambda_runtime::{service_fn, LambdaEvent};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;

/// Serve the API router to Lambda (function URL payloads).
pub async fn api(state: Shared) -> Result<()> {
    lambda_http::run(router(state)).await.map_err(|e| anyhow!("lambda runtime: {e}"))
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Action {
    /// Schema version and row counts.
    Status,
    /// Apply pending migrations.
    Migrate,
    /// Create or re-password the API logins from ECDB_API_PASSWORD and
    /// ECDB_WRITER_PASSWORD, each a member of exactly one group role.
    BootstrapRoles,
    /// Run one importer (or `all`) over the packaged catalogues.
    Import {
        what: Selection,
        #[serde(default)]
        walk_files: Option<Vec<String>>,
    },
    /// Import one part of a pack too large for the package, sent in the
    /// event as base64 of the gzipped part (`ecdb pack` writes the parts;
    /// deploy-aws.sh sends them): `ecbench`, or `generated` (Sage output,
    /// ecdb/deploy/load-generated.sh).
    ImportPack { name: String, data: String },
}

struct Admin {
    pool: Pool,
    repo_root: PathBuf,
    repository: String,
    commit: Option<String>,
}

pub async fn admin(pool: Pool, repo_root: PathBuf, repository: String, commit: Option<String>) -> Result<()> {
    let admin = Arc::new(Admin { pool, repo_root, repository, commit });
    lambda_runtime::run(service_fn(move |event: LambdaEvent<Value>| {
        let admin = admin.clone();
        async move { handle(&admin, event.payload).await.map_err(|e| lambda_runtime::Error::from(format!("{e:#}"))) }
    }))
    .await
    .map_err(|e| anyhow!("lambda runtime: {e}"))
}

async fn handle(admin: &Admin, payload: Value) -> Result<Value> {
    let action: Action = serde_json::from_value(payload).context("event must be {\"action\": ...}")?;
    let mut client = admin.pool.get().await?;
    match action {
        Action::Status => {
            let has_schema: bool =
                client.query_one("SELECT to_regclass('ec.schema_migrations') IS NOT NULL", &[]).await?.get(0);
            if !has_schema {
                return Ok(json!({ "schema_version": null }));
            }
            let version: Option<i32> =
                client.query_one("SELECT max(version) FROM ec.schema_migrations", &[]).await?.get(0);
            let counts: String = client
                .query_one(
                    "SELECT jsonb_build_object(
                       'curves', (SELECT count(*) FROM ec.curve),
                       'isogeny_classes', (SELECT count(*) FROM ec.isogeny_class_fq),
                       'isogenies', (SELECT count(*) FROM ec.isogeny),
                       'curve_morphisms', (SELECT count(*) FROM ec.curve_morphism),
                       'dlp_transfers', (SELECT count(*) FROM ec.dlp_transfer),
                       'sources', (SELECT count(*) FROM ec.source),
                       'database_bytes', pg_database_size(current_database()))::text",
                    &[],
                )
                .await?
                .get(0);
            Ok(json!({ "schema_version": version, "counts": serde_json::from_str::<Value>(&counts)? }))
        }
        Action::Migrate => {
            let applied = crate::migrate::run(&mut client).await?;
            Ok(json!({ "applied": applied, "schema_version": crate::migrate::schema_version() }))
        }
        Action::BootstrapRoles => {
            let mut logins = Vec::new();
            for (login, group, var) in crate::db::LOGINS {
                let password = std::env::var(var).with_context(|| format!("{var} is not set"))?;
                logins.push((*login, *group, password));
            }
            Ok(json!({ "roles": crate::db::bootstrap_logins(&client, &logins).await? }))
        }
        Action::Import { what, walk_files } => {
            let ctx = Ctx {
                client: &client,
                repo_root: admin.repo_root.clone(),
                repository: admin.repository.clone(),
                git_commit: admin.commit.clone(),
            };
            let started = std::time::Instant::now();
            let reports = import::run(&ctx, what, walk_files).await?;
            Ok(json!({ "seconds": started.elapsed().as_secs(), "reports": reports }))
        }
        Action::ImportPack { name, data } => {
            if name != import::ecbench::PACK && name != import::generated::PACK {
                return Err(anyhow!("unknown pack {name}"));
            }
            let gz = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data.trim())?;
            let ctx = Ctx {
                client: &client,
                repo_root: admin.repo_root.clone(),
                repository: admin.repository.clone(),
                git_commit: admin.commit.clone(),
            };
            let started = std::time::Instant::now();
            let report = if name == import::generated::PACK {
                import::generated::import_part(&ctx, &gz).await?
            } else {
                import::ecbench::import_part(&ctx, &gz).await?
            };
            Ok(json!({ "seconds": started.elapsed().as_secs(), "reports": [report] }))
        }
    }
}
