//! ecdb command line: migrate the schema, import the repository's curve
//! catalogues, serve the HTTP API.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ecdb::api::{self, AppState, Config};
use ecdb::db::{self, DbArgs};
use ecdb::import::{self, Ctx};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "ecdb", version, about = "Elliptic-curve catalogue: schema, importers and HTTP API")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Apply pending schema migrations (needs a role that may create schema objects).
    Migrate {
        #[command(flatten)]
        db: DbArgs,
    },
    /// Import catalogues from a checkout of the crypto repository.
    Import {
        #[command(flatten)]
        db: DbArgs,
        /// Repository root holding docs/curves/.
        #[arg(long, default_value = "..")]
        repo_root: PathBuf,
        #[arg(long, default_value = "isogeny-labs/crypto")]
        repository: String,
        #[command(subcommand)]
        what: ImportWhat,
    },
    /// Serve the HTTP API.
    Serve {
        #[command(flatten)]
        db: DbArgs,
        #[arg(long, env = "ECDB_LISTEN", default_value = "0.0.0.0:8080")]
        listen: String,
        /// Bearer token required on reads (unset: reads are open).
        #[arg(long, env = "ECDB_READ_TOKEN")]
        read_token: Option<String>,
        /// Bearer token required on writes (unset: writes are disabled).
        #[arg(long, env = "ECDB_WRITE_TOKEN")]
        write_token: Option<String>,
        /// Connection for writes, as a member of ecdb_writer.
        #[arg(long, env = "ECDB_WRITE_DATABASE_URL")]
        write_database_url: Option<String>,
        /// Role POST /v1/sql runs as (the login role must be a member).
        #[arg(long, env = "ECDB_SQL_ROLE")]
        sql_role: Option<String>,
        #[arg(long, env = "ECDB_SQL_TIMEOUT_MS", default_value_t = 15000)]
        sql_timeout_ms: u64,
        #[arg(long, env = "ECDB_SQL_MAX_ROWS", default_value_t = 5000)]
        sql_max_rows: usize,
        /// Apply pending migrations before serving.
        #[arg(long, env = "ECDB_MIGRATE_ON_START")]
        migrate: bool,
    },
    /// Print the OpenAPI description.
    Openapi,
}

#[derive(Subcommand)]
enum ImportWhat {
    /// Everything below, in dependency order.
    All,
    /// docs/curves/registry.json
    Registry,
    /// Isogeny-walk outputs (default: the six recorded isogeny_routes.json files).
    Walks { files: Vec<String> },
    /// docs/curves/traits.json
    Traits,
    /// docs/curves/covers.json
    Covers,
    /// docs/curves/jacobian-transfers.json
    Transfers,
    /// docs/curves/level-certificates.json and jacobian-certificates.json
    Certificates,
    /// Facts derived in SQL from imported rows (automorphism groups, attack-surface summary).
    Derived,
    /// An LMFDB av_fq_isog JSON export (rows of isogeny classes over finite fields).
    LmfdbAvFq {
        path: PathBuf,
        #[arg(long, default_value = "LMFDB av_fq_isog export")]
        title: String,
    },
}

fn git_commit(root: &PathBuf) -> Option<String> {
    let out = std::process::Command::new("git").arg("-C").arg(root).args(["rev-parse", "HEAD"]).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn print<T: serde::Serialize>(v: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(v)?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .with_writer(std::io::stderr)
        .init();
    match Cli::parse().command {
        Command::Openapi => print!("{}", api::OPENAPI),
        Command::Migrate { db } => {
            let pool = db::pool(&db)?;
            let mut client = pool.get().await?;
            let applied = ecdb::migrate::run(&mut client).await?;
            print(&serde_json::json!({ "applied": applied, "schema_version": ecdb::migrate::schema_version() }))?;
        }
        Command::Import { db, repo_root, repository, what } => {
            let pool = db::pool(&db)?;
            let client = pool.get().await?;
            let repo_root = repo_root.canonicalize().with_context(|| format!("repo root {}", repo_root.display()))?;
            let ctx = Ctx { client: &client, git_commit: git_commit(&repo_root), repo_root, repository };
            let walk_defaults = || import::walks::DEFAULT_FILES.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let mut reports = Vec::new();
            match what {
                ImportWhat::All => {
                    reports.push(import::registry::import(&ctx).await?);
                    reports.extend(import::walks::import(&ctx, &walk_defaults()).await?);
                    reports.push(import::traits::import(&ctx).await?);
                    reports.push(import::covers::import(&ctx).await?);
                    reports.push(import::transfers::import(&ctx).await?);
                    reports.extend(import::certs::import(&ctx).await?);
                    reports.push(import::derive::import(&ctx).await?);
                }
                ImportWhat::Registry => reports.push(import::registry::import(&ctx).await?),
                ImportWhat::Walks { files } => {
                    let files = if files.is_empty() { walk_defaults() } else { files };
                    reports.extend(import::walks::import(&ctx, &files).await?);
                }
                ImportWhat::Traits => reports.push(import::traits::import(&ctx).await?),
                ImportWhat::Covers => reports.push(import::covers::import(&ctx).await?),
                ImportWhat::Transfers => reports.push(import::transfers::import(&ctx).await?),
                ImportWhat::Certificates => reports.extend(import::certs::import(&ctx).await?),
                ImportWhat::Derived => reports.push(import::derive::import(&ctx).await?),
                ImportWhat::LmfdbAvFq { path, title } => {
                    reports.push(import::lmfdb::import(&client, &path, &title).await?)
                }
            }
            print(&reports)?;
        }
        Command::Serve {
            db,
            listen,
            read_token,
            write_token,
            write_database_url,
            sql_role,
            sql_timeout_ms,
            sql_max_rows,
            migrate,
        } => {
            let read = db::pool(&db)?;
            if migrate {
                let mut client = read.get().await?;
                ecdb::migrate::run(&mut client).await?;
            }
            let write = match &write_database_url {
                Some(url) => Some(db::pool(&DbArgs { database_url: url.clone(), ..db.clone() })?),
                None => None,
            };
            let config = Config { read_token, write_token, sql_role, sql_timeout_ms, sql_max_rows };
            let app = api::router(Arc::new(AppState { read, write, config }));
            let listener = tokio::net::TcpListener::bind(&listen).await?;
            tracing::info!(%listen, "ecdb serving");
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await?;
        }
    }
    Ok(())
}
