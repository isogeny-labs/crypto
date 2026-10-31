//! Packs: the rows an importer collects by scanning a directory tree
//! (benchmarks/, every ecbench session), written as one JSON object per line.
//! The Lambda package carries packs instead of the trees (737 MB of ecbench
//! records pack to a few MB gzipped); a local import scans the tree directly
//! and gets the same rows, so the recorded SHA-256 (of the packed bytes)
//! is the same either way.

use super::{read_bytes, Ctx};
use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Write;
use std::path::Path;

pub const DIR: &str = "ecdb-pack";

pub fn encode(rows: &[Value]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for r in rows {
        serde_json::to_writer(&mut out, r)?;
        out.push(b'\n');
    }
    Ok(out)
}

pub fn decode(bytes: &[u8]) -> Result<Vec<Value>> {
    bytes
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_slice(l).context("pack line"))
        .collect()
}

fn sha(bytes: &[u8]) -> String {
    hex::encode(<sha2::Sha256 as sha2::Digest>::digest(bytes))
}

/// The pack `<repo_root>/ecdb-pack/<name>.jsonl[.gz]` when present, else `collect()`.
/// Returns the rows and the SHA-256 of their packed form.
pub fn load_or(
    ctx: &Ctx<'_>,
    name: &str,
    collect: impl FnOnce() -> Result<Vec<Value>>,
) -> Result<(Vec<Value>, String)> {
    let path = ctx.repo_root.join(DIR).join(format!("{name}.jsonl"));
    let gz = ctx.repo_root.join(DIR).join(format!("{name}.jsonl.gz"));
    if path.exists() || gz.exists() {
        let bytes = read_bytes(&path)?;
        return Ok((decode(&bytes)?, sha(&bytes)));
    }
    let rows = collect()?;
    let bytes = encode(&rows)?;
    Ok((rows, sha(&bytes)))
}

/// Write `rows` gzipped to `<out>/<name>.jsonl.gz`; returns the uncompressed SHA-256.
pub fn write(out: &Path, name: &str, rows: &[Value]) -> Result<String> {
    std::fs::create_dir_all(out)?;
    let bytes = encode(rows)?;
    let file = std::fs::File::create(out.join(format!("{name}.jsonl.gz")))?;
    let mut enc = flate2::write::GzEncoder::new(file, flate2::Compression::best());
    enc.write_all(&bytes)?;
    enc.finish()?;
    Ok(sha(&bytes))
}
