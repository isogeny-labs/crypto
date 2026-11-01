//! Importers for the repository's curve catalogues. Each importer is
//! idempotent (natural keys, `ON CONFLICT DO NOTHING` or a deliberate
//! update), set-based (rows travel as one jsonb array per batch and are
//! joined on natural identifiers in SQL), and records a provenance row bound
//! to the input file's SHA-256 and the repository commit.

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio_postgres::Client;

pub mod certs;
pub mod covers;
pub mod derive;
pub mod ecbench;
pub mod enrich;
pub mod generated;
pub mod lmfdb;
pub mod pack;
pub mod registry;
pub mod results;
pub mod siblings;
pub mod traits;
pub mod transfers;
pub mod walks;

pub const TOOL: &str = "ecdb import";
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");
const BATCH: usize = 2000;

pub struct Ctx<'a> {
    pub client: &'a Client,
    pub repo_root: PathBuf,
    pub repository: String,
    pub git_commit: Option<String>,
}

pub struct Input {
    pub rel_path: String,
    pub sha256: String,
    pub doc: Value,
}

impl Ctx<'_> {
    pub fn read(&self, rel_path: &str) -> Result<Input> {
        let path = self.repo_root.join(rel_path);
        read_json(&path, rel_path)
    }

    pub async fn source(&self, rel_path: &str, sha256: &str, title: &str) -> Result<i64> {
        self.source_in(&self.repository, rel_path, sha256, self.git_commit.as_deref(), title).await
    }

    /// A pinned file of any repository (the sibling importers' sources).
    pub async fn source_in(
        &self,
        repository: &str,
        rel_path: &str,
        sha256: &str,
        git_commit: Option<&str>,
        title: &str,
    ) -> Result<i64> {
        let row = self
            .client
            .query_one(
                "WITH ins AS (
                   INSERT INTO ec.source (kind, title, repository, path, git_commit, sha256, provenance_class)
                   VALUES ('repository_file', $1, $2, $3, $4, $5::text, 'internal')
                   ON CONFLICT DO NOTHING RETURNING id)
                 SELECT id FROM ins
                 UNION ALL
                 SELECT id FROM ec.source
                  WHERE kind = 'repository_file' AND repository = $2 AND path = $3
                    AND sha256::text = $5 AND uri IS NULL
                 LIMIT 1",
                &[&title, &repository, &rel_path, &git_commit, &sha256],
            )
            .await?;
        Ok(row.get(0))
    }

    pub async fn provenance(&self, source_id: i64, method: &str, parameters: Option<&Value>) -> Result<i64> {
        let row = self
            .client
            .query_one(
                "WITH ins AS (
                   INSERT INTO ec.provenance (source_id, method, tool, tool_version, parameters, computed_at)
                   VALUES ($1, $2, $3, $4, $5, now())
                   ON CONFLICT DO NOTHING RETURNING id)
                 SELECT id FROM ins
                 UNION ALL
                 SELECT id FROM ec.provenance
                  WHERE source_id = $1 AND method = $2 AND tool = $3 AND tool_version = $4 AND artifact_id IS NULL
                 LIMIT 1",
                &[&source_id, &method, &TOOL, &TOOL_VERSION, &parameters],
            )
            .await?;
        Ok(row.get(0))
    }
}

/// Read a JSON input, or its gzip copy `<path>.gz` when only that exists (the
/// Lambda package ships the catalogues compressed). The recorded SHA-256 is
/// always that of the uncompressed bytes, so it matches the repository file.
pub fn read_json(path: &Path, rel_path: &str) -> Result<Input> {
    let bytes = read_bytes(path)?;
    let sha256 = hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&bytes));
    let doc = serde_json::from_slice(&bytes).with_context(|| format!("parsing {}", path.display()))?;
    Ok(Input { rel_path: rel_path.to_string(), sha256, doc })
}

/// The bytes of `path`, or of its gzip copy `<path>.gz` when only that exists.
pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    let gz = PathBuf::from(format!("{}.gz", path.display()));
    if !path.exists() && gz.exists() {
        let mut out = Vec::new();
        std::io::Read::read_to_end(&mut flate2::read::GzDecoder::new(std::fs::File::open(&gz)?), &mut out)
            .with_context(|| format!("decompressing {}", gz.display()))?;
        Ok(out)
    } else {
        std::fs::read(path).with_context(|| format!("reading {}", path.display()))
    }
}

pub fn read_text(path: &Path) -> Result<String> {
    Ok(String::from_utf8(read_bytes(path)?)?)
}

/// Run `sql` once per batch with `$1` = jsonb array of rows and `$2` = provenance id.
pub async fn bulk(client: &Client, sql: &str, rows: &[Value], provenance: i64) -> Result<u64> {
    let mut n = 0;
    for chunk in rows.chunks(BATCH) {
        let arr = Value::Array(chunk.to_vec());
        n += client.execute(sql, &[&arr, &provenance]).await.with_context(|| first_line(sql))?;
    }
    Ok(n)
}

/// As [`bulk`] for statements without a provenance column (`$1` only).
pub async fn bulk1(client: &Client, sql: &str, rows: &[Value]) -> Result<u64> {
    let mut n = 0;
    for chunk in rows.chunks(BATCH) {
        let arr = Value::Array(chunk.to_vec());
        n += client.execute(sql, &[&arr]).await.with_context(|| first_line(sql))?;
    }
    Ok(n)
}

fn first_line(sql: &str) -> String {
    sql.trim().lines().next().unwrap_or_default().to_string()
}

/// Accumulates what an importer did and what it refused, for the CLI report.
#[derive(Default, Debug, serde::Serialize)]
pub struct Report {
    pub input: String,
    pub sha256: String,
    pub counts: std::collections::BTreeMap<String, u64>,
    pub problems: Vec<String>,
}

impl Report {
    pub fn new(input: &Input) -> Report {
        Report { input: input.rel_path.clone(), sha256: input.sha256.clone(), ..Default::default() }
    }

    pub fn count(&mut self, key: &str, n: u64) {
        *self.counts.entry(key.to_string()).or_default() += n;
    }

    pub fn problem(&mut self, msg: String) {
        if self.problems.len() < 200 {
            self.problems.push(msg);
        } else if self.problems.len() == 200 {
            self.problems.push("... further problems omitted".into());
        }
    }
}

pub fn s(v: &Value) -> Option<String> {
    match v {
        Value::String(x) => Some(x.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Which repository importers to run; `All` runs them in dependency order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Selection {
    All,
    Registry,
    Walks,
    Traits,
    Covers,
    Transfers,
    Certificates,
    Derived,
    Enrich,
    /// The IC leaderboard, progress timeline, boundary ledger and benchmarks/ index.
    Results,
    /// Every ecbench session with its runs and comparisons.
    Ecbench,
    /// The sibling repositories' curve records, dataset curves and measurements.
    Siblings,
}

/// Run the selected importers; `walk_files` overrides the default walk outputs.
pub async fn run(ctx: &Ctx<'_>, what: Selection, walk_files: Option<Vec<String>>) -> Result<Vec<Report>> {
    let walks = walk_files.unwrap_or_else(|| walks::DEFAULT_FILES.iter().map(|s| s.to_string()).collect());
    let mut reports = Vec::new();
    let all = what == Selection::All;
    if all || what == Selection::Registry {
        reports.push(registry::import(ctx).await?);
    }
    if all || what == Selection::Walks {
        reports.extend(walks::import(ctx, &walks).await?);
    }
    if all || what == Selection::Traits {
        reports.push(traits::import(ctx).await?);
    }
    if all || what == Selection::Covers {
        reports.push(covers::import(ctx).await?);
    }
    if all || what == Selection::Transfers {
        reports.push(transfers::import(ctx).await?);
    }
    if all || what == Selection::Certificates {
        reports.extend(certs::import(ctx).await?);
    }
    if all || what == Selection::Siblings {
        reports.push(siblings::import(ctx).await?);
    }
    if all || what == Selection::Derived {
        reports.push(derive::import(ctx).await?);
    }
    if all || what == Selection::Enrich {
        reports.push(enrich::import(ctx).await?);
    }
    if all || what == Selection::Results {
        reports.push(results::leaderboard(ctx).await?);
        reports.push(results::progress(ctx).await?);
        reports.push(results::boundary(ctx).await?);
        reports.push(results::benchmarks(ctx).await?);
    }
    if all || what == Selection::Ecbench {
        reports.push(ecbench::import(ctx).await?);
    }
    Ok(reports)
}
