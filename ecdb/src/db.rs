//! Connection pools. RDS needs TLS with the Amazon RDS CA bundle
//! (`ECDB_PG_CA_FILE`); local development can disable TLS.

use anyhow::{anyhow, Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::sync::Arc;

#[derive(Clone, Debug, clap::ValueEnum, PartialEq, Eq)]
pub enum TlsMode {
    Disable,
    Require,
}

#[derive(Clone, Debug, clap::Args)]
pub struct DbArgs {
    /// postgres://user:password@host:5432/dbname
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,
    /// TLS to Postgres. RDS: require, with --pg-ca-file set to the RDS bundle.
    #[arg(long, env = "ECDB_PG_TLS", value_enum, default_value = "require")]
    pub pg_tls: TlsMode,
    /// PEM bundle of trusted CAs (e.g. global-bundle.pem from AWS). Default: webpki roots.
    #[arg(long, env = "ECDB_PG_CA_FILE")]
    pub pg_ca_file: Option<String>,
    #[arg(long, env = "ECDB_POOL_SIZE", default_value_t = 16)]
    pub pool_size: usize,
}

fn tls_connector(ca_file: Option<&str>) -> Result<tokio_postgres_rustls::MakeRustlsConnect> {
    let mut roots = rustls::RootCertStore::empty();
    match ca_file {
        Some(path) => {
            let pem = std::fs::read(path).with_context(|| format!("reading CA bundle {path}"))?;
            for cert in rustls_pemfile::certs(&mut pem.as_slice()) {
                roots.add(cert?)?;
            }
            if roots.is_empty() {
                return Err(anyhow!("no certificates in {path}"));
            }
        }
        None => roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned()),
    }
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(tokio_postgres_rustls::MakeRustlsConnect::new(config))
}

pub fn pool(args: &DbArgs) -> Result<Pool> {
    let cfg: tokio_postgres::Config = args.database_url.parse().context("parsing DATABASE_URL")?;
    let mgr_cfg = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    let mgr = match args.pg_tls {
        TlsMode::Disable => Manager::from_config(cfg, tokio_postgres::NoTls, mgr_cfg),
        TlsMode::Require => Manager::from_config(cfg, tls_connector(args.pg_ca_file.as_deref())?, mgr_cfg),
    };
    Ok(Pool::builder(mgr).max_size(args.pool_size).build()?)
}
