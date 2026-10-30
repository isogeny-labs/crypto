//! ecdb command line: migrate the schema, import the repository's curve
//! catalogues, serve the HTTP API, or run as an AWS Lambda function.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ecdb::api::{self, AppState, Config};
use ecdb::db::{self, DbArgs};
use ecdb::import::{self, Ctx, Selection};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "ecdb", version, about = "Elliptic-curve catalogue: schema, importers and HTTP API")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args, Clone)]
struct ServeArgs {
    #[command(flatten)]
    db: DbArgs,
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
}

impl ServeArgs {
    fn state(&self) -> Result<Arc<AppState>> {
        let read = db::pool(&self.db)?;
        let write = match &self.write_database_url {
            Some(url) => Some(db::pool(&DbArgs { database_url: url.clone(), ..self.db.clone() })?),
            None => None,
        };
        let config = Config {
            read_token: self.read_token.clone(),
            write_token: self.write_token.clone(),
            sql_role: self.sql_role.clone(),
            sql_timeout_ms: self.sql_timeout_ms,
            sql_max_rows: self.sql_max_rows,
        };
        Ok(Arc::new(AppState { read, write, config }))
    }
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
        serve: ServeArgs,
        #[arg(long, env = "ECDB_LISTEN", default_value = "0.0.0.0:8080")]
        listen: String,
        /// Apply pending migrations before serving.
        #[arg(long, env = "ECDB_MIGRATE_ON_START")]
        migrate: bool,
    },
    /// Run as an AWS Lambda function (see deploy/README.md).
    Lambda {
        #[command(subcommand)]
        mode: LambdaMode,
    },
    /// Print the OpenAPI description.
    Openapi,
}

#[derive(Subcommand)]
enum LambdaMode {
    /// The HTTP API behind a function URL; configured like `serve`.
    Api {
        #[command(flatten)]
        serve: ServeArgs,
    },
    /// Migrations, login roles and imports, one action per invocation.
    Admin {
        #[command(flatten)]
        db: DbArgs,
        /// Directory holding docs/curves/ (files may be gzipped).
        #[arg(long, env = "ECDB_REPO_ROOT", default_value = "data")]
        repo_root: PathBuf,
        #[arg(long, env = "ECDB_REPOSITORY", default_value = "isogeny-labs/crypto")]
        repository: String,
        /// Commit the packaged catalogues were taken from.
        #[arg(long, env = "ECDB_SOURCE_COMMIT")]
        source_commit: Option<String>,
    },
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
    let in_lambda = std::env::var_os("AWS_LAMBDA_RUNTIME_API").is_some();
    let filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into());
    if in_lambda {
        // CloudWatch adds its own timestamps; ANSI codes would show up literally.
        tracing_subscriber::fmt().with_env_filter(filter).with_ansi(false).without_time().init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).with_writer(std::io::stderr).init();
    }
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
            let reports = match what {
                ImportWhat::All => import::run(&ctx, Selection::All, None).await?,
                ImportWhat::Registry => import::run(&ctx, Selection::Registry, None).await?,
                ImportWhat::Walks { files } => {
                    import::run(&ctx, Selection::Walks, (!files.is_empty()).then_some(files)).await?
                }
                ImportWhat::Traits => import::run(&ctx, Selection::Traits, None).await?,
                ImportWhat::Covers => import::run(&ctx, Selection::Covers, None).await?,
                ImportWhat::Transfers => import::run(&ctx, Selection::Transfers, None).await?,
                ImportWhat::Certificates => import::run(&ctx, Selection::Certificates, None).await?,
                ImportWhat::Derived => import::run(&ctx, Selection::Derived, None).await?,
                ImportWhat::LmfdbAvFq { path, title } => vec![import::lmfdb::import(&client, &path, &title).await?],
            };
            print(&reports)?;
        }
        Command::Serve { serve, listen, migrate } => {
            let state = serve.state()?;
            if migrate {
                let mut client = state.read.get().await?;
                ecdb::migrate::run(&mut client).await?;
            }
            let app = api::router(state);
            let listener = tokio::net::TcpListener::bind(&listen).await?;
            tracing::info!(%listen, "ecdb serving");
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await?;
        }
        Command::Lambda { mode: LambdaMode::Api { serve } } => ecdb::lambda::api(serve.state()?).await?,
        Command::Lambda { mode: LambdaMode::Admin { db, repo_root, repository, source_commit } } => {
            ecdb::lambda::admin(db::pool(&db)?, repo_root, repository, source_commit).await?
        }
    }
    Ok(())
}
