//! Durable target-local relation stream. Every flushed JSONL record is synced;
//! a missing Terminal remains a retained prefix and is never promoted complete.
use super::diagnostics;
use pencil_symbolic_graph::relation_stream as stream;
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    time::Instant,
};

pub const MAX_RECORDS: u64 = 8_000_000;
pub const MAX_BYTES: usize = 4 * 1024 * 1024 * 1024;

#[derive(Clone, Debug, Serialize)]
pub struct Outcome {
    pub complete: bool,
    pub records: u64,
    pub bytes: usize,
    pub failure: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Totals {
    pub schema: &'static str,
    pub path: PathBuf,
    pub sha256: String,
    pub bytes: u64,
    pub synced_records: u64,
    pub write_ns: u128,
    pub sync_ns: u128,
    pub complete: bool,
    pub failure: Option<String>,
}

pub struct DurableWriter {
    path: PathBuf,
    directory: PathBuf,
    file: File,
    bytes: u64,
    synced_records: u64,
    write_ns: u128,
    sync_ns: u128,
}

impl DurableWriter {
    pub fn create(directory: &Path) -> io::Result<Self> {
        let path = directory.join("relations.jsonl");
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(&path)?;
        Ok(Self {
            path,
            directory: directory.to_owned(),
            file,
            bytes: 0,
            synced_records: 0,
            write_ns: 0,
            sync_ns: 0,
        })
    }

    pub fn finish(mut self, outcome: Outcome) -> io::Result<Totals> {
        let sync_start = Instant::now();
        let sync_result = self
            .file
            .flush()
            .and_then(|_| self.file.sync_all())
            .and_then(|_| fs::File::open(&self.directory)?.sync_all());
        self.sync_ns = self
            .sync_ns
            .saturating_add(sync_start.elapsed().as_nanos());
        sync_result?;
        let (sha256, bytes) = crate::store::hash_file(&self.path)
            .map_err(|error| io::Error::other(error.to_string()))?;
        if bytes != self.bytes
            || outcome.bytes as u64 != self.bytes
            || outcome.records != self.synced_records
        {
            return Err(io::Error::other("relation trace writer/trace totals mismatch"));
        }
        Ok(Totals {
            schema: "ecbench.pencil-relation-file/v1",
            path: self.path,
            sha256,
            bytes: self.bytes,
            synced_records: self.synced_records,
            write_ns: self.write_ns,
            sync_ns: self.sync_ns,
            complete: outcome.complete,
            failure: outcome.failure,
        })
    }
}

impl Write for DurableWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let start = Instant::now();
        let result = self.file.write_all(bytes);
        self.write_ns = self.write_ns.saturating_add(start.elapsed().as_nanos());
        result?;
        self.bytes = self
            .bytes
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| io::Error::other("relation writer byte overflow"))?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let start = Instant::now();
        let result = self.file.flush().and_then(|_| self.file.sync_data());
        self.sync_ns = self.sync_ns.saturating_add(start.elapsed().as_nanos());
        result?;
        self.synced_records = self
            .synced_records
            .checked_add(1)
            .ok_or_else(|| io::Error::other("relation writer record overflow"))?;
        Ok(())
    }
}

pub fn identity(
    diagnostic: &diagnostics::Identity,
    source_revision: &str,
) -> io::Result<stream::Identity> {
    let revision_ok = source_revision.len() == 40
        && source_revision
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
    if !revision_ok {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "managed relation trace requires exact lowercase source revision",
        ));
    }
    Ok(stream::Identity {
        source_revision: source_revision.to_owned(),
        source_sha256: diagnostic.source_sha256.clone(),
        workload_id: diagnostic.workload_sha256.clone(),
        run_id: format!(
            "{}:execution-{}:attempt-{}",
            diagnostic.session, diagnostic.execution, diagnostic.attempt
        ),
    })
}
