//! `ecdb sync`: copy a whole ecdb (normally the deployed one) into a local
//! Postgres through the API's bulk export (`api::export`). Used by the NAS
//! mirror in deploy/nas/.
//!
//! The local schema is migrated first and must reach the remote's version.
//! Every table is then truncated and refilled inside one transaction, with
//! foreign-key triggers off (`session_replication_role = replica`, which
//! needs a superuser login), so the load order does not matter and a failed
//! sync leaves the previous contents in place. Identity sequences are moved
//! past the copied ids and every table's row count is checked against the
//! remote manifest before the commit.

use crate::api::export::{column_list, quote_ident, MAX_ROWS, ROWS_HEADER};
use anyhow::{anyhow, bail, Context, Result};
use bytes::Bytes;
use flate2::read::GzDecoder;
use futures_util::SinkExt;
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::Read;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
struct Manifest {
    schema_version: i32,
    tables: Vec<RemoteTable>,
}

#[derive(Deserialize)]
struct RemoteTable {
    table: String,
    columns: Vec<String>,
    rows: i64,
}

struct Remote {
    http: reqwest::Client,
    base: String,
    token: Option<String>,
}

impl Remote {
    /// GET with up to four retries on transport errors and 5xx (2, 4, 8, 16 s).
    async fn get(&self, path: &str, query: &[(&str, String)]) -> Result<reqwest::Response> {
        let url = format!("{}{path}", self.base);
        let mut delay = Duration::from_secs(2);
        for attempt in 0..5 {
            let mut req = self.http.get(&url).query(query);
            if let Some(t) = &self.token {
                req = req.bearer_auth(t);
            }
            match req.send().await {
                Ok(r) if !r.status().is_server_error() => return Ok(r),
                Ok(r) if attempt == 4 => bail!("GET {url}: {} {}", r.status(), r.text().await.unwrap_or_default()),
                Err(e) if attempt == 4 => return Err(e).with_context(|| format!("GET {url}")),
                Ok(r) => tracing::warn!(status = %r.status(), %url, "retrying"),
                Err(e) => tracing::warn!(error = %e, %url, "retrying"),
            }
            tokio::time::sleep(delay).await;
            delay *= 2;
        }
        unreachable!()
    }
}

pub async fn run(
    client: &mut tokio_postgres::Client,
    from: &str,
    token: Option<String>,
    replace: bool,
) -> Result<Value> {
    let started = Instant::now();
    let remote = Remote {
        http: reqwest::Client::builder().timeout(Duration::from_secs(180)).build()?,
        base: from.trim_end_matches('/').to_string(),
        token,
    };
    let resp = remote.get("/v1/export", &[]).await?;
    if !resp.status().is_success() {
        bail!("GET /v1/export: {} {}", resp.status(), resp.text().await.unwrap_or_default());
    }
    let manifest: Manifest = serde_json::from_slice(&resp.bytes().await?).context("parsing the export manifest")?;

    crate::migrate::run(client).await?;
    let local = crate::migrate::schema_version();
    if manifest.schema_version != local {
        bail!(
            "remote schema is version {}, this ecdb build migrates to {local}; use the ecdb build that matches the remote",
            manifest.schema_version
        );
    }
    let mut present = 0i64;
    for t in &manifest.tables {
        let n: i64 = client
            .query_one(&format!("SELECT count(*) FROM ec.{}", quote_ident(&t.table)), &[])
            .await
            .with_context(|| format!("table ec.{} is missing locally", t.table))?
            .get(0);
        present += n;
    }
    if present > 0 && !replace {
        bail!("the local catalogue already holds {present} rows; pass --replace to overwrite it");
    }

    let tx = client.transaction().await?;
    tx.batch_execute("SET LOCAL session_replication_role = replica").await.map_err(|e| {
        anyhow!("{e}: the sync login must be a superuser (it switches foreign-key triggers off for the load)")
    })?;
    let all = manifest.tables.iter().map(|t| format!("ec.{}", quote_ident(&t.table))).collect::<Vec<_>>();
    if !all.is_empty() {
        tx.batch_execute(&format!("TRUNCATE {}", all.join(", "))).await?;
    }

    let mut report = Vec::new();
    let (mut total_rows, mut total_bytes) = (0i64, 0usize);
    for t in &manifest.tables {
        let t0 = Instant::now();
        let (mut offset, mut limit, mut bytes) = (0i64, 20_000i64, 0usize);
        while offset < t.rows {
            let q = [("offset", offset.to_string()), ("limit", limit.to_string())];
            let resp = remote.get(&format!("/v1/export/{}", t.table), &q).await?;
            if resp.status() == reqwest::StatusCode::PAYLOAD_TOO_LARGE && limit > 1 {
                limit = (limit / 2).max(1);
                continue;
            }
            if !resp.status().is_success() {
                bail!("export of ec.{} at offset {offset}: {} {}", t.table, resp.status(), resp.text().await?);
            }
            let rows: i64 = resp
                .headers()
                .get(ROWS_HEADER)
                .and_then(|v| v.to_str().ok()?.parse().ok())
                .ok_or_else(|| anyhow!("export page without {ROWS_HEADER}"))?;
            let gz = resp.bytes().await?;
            let mut raw = Vec::new();
            GzDecoder::new(&gz[..]).read_to_end(&mut raw).context("gunzipping an export page")?;
            if rows == 0 {
                bail!("ec.{} ended at {offset} rows, the manifest said {}", t.table, t.rows);
            }
            let sink = tx
                .copy_in(&format!("COPY ec.{} ({}) FROM STDIN", quote_ident(&t.table), column_list(&t.columns)))
                .await?;
            futures_util::pin_mut!(sink);
            sink.send(Bytes::from(raw)).await?;
            let copied = sink.finish().await? as i64;
            if copied != rows {
                bail!("ec.{}: page at {offset} had {rows} rows, COPY took {copied}", t.table);
            }
            offset += rows;
            bytes += gz.len();
            // Aim for pages of 1–4 MB gzipped.
            if gz.len() < (1 << 20) {
                limit = (limit * 2).min(MAX_ROWS);
            }
        }
        tracing::info!(table = %t.table, rows = t.rows, gz_bytes = bytes, secs = t0.elapsed().as_secs_f32(), "copied");
        total_rows += t.rows;
        total_bytes += bytes;
        report.push(json!({ "table": t.table, "rows": t.rows, "gz_bytes": bytes }));
    }

    // Identity and serial sequences continue after the copied ids.
    let setvals: Vec<String> = tx
        .query(
            "SELECT format('SELECT setval(%L, COALESCE((SELECT max(%I) FROM ec.%I), 0) + 1, false)',
                           s.oid::regclass::text, a.attname, t.relname)
               FROM pg_depend d
               JOIN pg_class s ON s.oid = d.objid AND s.relkind = 'S'
               JOIN pg_class t ON t.oid = d.refobjid
               JOIN pg_namespace n ON n.oid = t.relnamespace AND n.nspname = 'ec'
               JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = d.refobjsubid
              WHERE d.deptype IN ('a', 'i')",
            &[],
        )
        .await?
        .iter()
        .map(|r| r.get(0))
        .collect();
    for stmt in &setvals {
        tx.batch_execute(stmt).await?;
    }
    for t in &manifest.tables {
        let n: i64 = tx.query_one(&format!("SELECT count(*) FROM ec.{}", quote_ident(&t.table)), &[]).await?.get(0);
        if n != t.rows {
            bail!("ec.{} holds {n} rows after the load, the remote has {}", t.table, t.rows);
        }
    }
    tx.commit().await?;
    client.batch_execute("ANALYZE").await?;
    Ok(json!({
        "from": remote.base,
        "schema_version": local,
        "tables": manifest.tables.len(),
        "rows": total_rows,
        "gz_bytes": total_bytes,
        "sequences_reset": setvals.len(),
        "seconds": started.elapsed().as_secs(),
        "per_table": report,
    }))
}
