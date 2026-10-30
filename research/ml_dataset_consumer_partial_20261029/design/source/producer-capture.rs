//! Bounded local capture of exposed solver artifacts. No cloud operations.
//!
//! A successful event call appends and fsyncs a hash-chained record, then
//! atomically commits its byte watermark. The supervisor copies only that
//! acknowledged prefix into immutable chunks. Neither a watermark nor a
//! captured delta is a completed cell, scientific verdict, or solver checkpoint.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::storage;
use super::types::{Artifact, ArtifactPayload, CaseConfig};
use crate::store::index::validate_sha256;

const BINDING_SCHEMA: &str = "crypto.ml-dataset.attempt-binding/v1";
const EVENT_SCHEMA: &str = "crypto.ml-dataset.live-event/v2";
const HEAD_SCHEMA: &str = "crypto.ml-dataset.capture-head/v1";
const PAYLOAD_KIND: &str = "capture_payload_committed";
const MAX_METADATA: u64 = 1024 * 1024;
const MAX_EVENT: u64 = 1024 * 1024;
const MAX_CHUNK: u64 = 16 * 1024 * 1024;
const MAX_STREAM: u64 = 4 * 1024 * 1024 * 1024;
const MAX_PAYLOAD: u64 = 4 * 1024 * 1024 * 1024;
const MAX_ITEMS: u64 = 100_000;
static TEMP_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttemptBinding {
    pub schema: String,
    pub run_sha256: String,
    pub plan_sha256: String,
    pub protocol_sha256: String,
    pub config_sha256: String,
    pub source_commit: String,
    pub binary_sha256: String,
    pub cell_index: u64,
    pub attempt_sha256: String,
}

impl AttemptBinding {
    pub fn from_cell_files(cell_dir: &Path) -> Result<Self, String> {
        let root = storage::root_dir(cell_dir, false)?;
        let run: Value = read_json(&root, "run.json", super::JSON_INPUT_MAX_BYTES)?;
        let plan: super::DatasetPlan = read_json(&root, "plan.json", super::JSON_INPUT_MAX_BYTES)?;
        plan.validate()?;
        let config: CaseConfig = read_json(&root, "config.json", super::JSON_INPUT_MAX_BYTES)?;
        let attempt: Value = read_json(&root, "attempt.json", MAX_METADATA)?;
        if run["schema"] != "crypto.ml-dataset.run/v1"
            || attempt["schema"] != "crypto.ml-dataset.attempt/v1"
        {
            return Err("capture requires the original run and attempt schemas".into());
        }
        let field = |value: &Value, key: &str| -> Result<String, String> {
            value[key]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("capture binding missing {key}"))
        };
        let cell_index = attempt["cell_index"]
            .as_u64()
            .ok_or("attempt has no cell index")?;
        let plan_sha256 = super::digest(&serde_json::to_vec(&plan).map_err(|e| e.to_string())?);
        if run["plan_sha256"] != plan_sha256 || attempt["plan_sha256"] != plan_sha256 {
            return Err("capture attempt/run/plan mismatch".into());
        }
        if plan.cell(cell_index)? != config {
            return Err("capture config does not match frozen plan cell".into());
        }
        let protocol = read_bytes(&root, "protocol.json", super::JSON_INPUT_MAX_BYTES)?;
        super::verify_protocol(&plan, &protocol)?;
        let result = Self {
            schema: BINDING_SCHEMA.into(),
            run_sha256: super::digest(&read_bytes(&root, "run.json", super::JSON_INPUT_MAX_BYTES)?),
            plan_sha256: super::digest(&read_bytes(
                &root,
                "plan.json",
                super::JSON_INPUT_MAX_BYTES,
            )?),
            protocol_sha256: super::digest(&protocol),
            config_sha256: super::digest(&read_bytes(
                &root,
                "config.json",
                super::JSON_INPUT_MAX_BYTES,
            )?),
            source_commit: field(&run, "source_commit")?,
            binary_sha256: field(&run, "binary_sha256")?,
            cell_index,
            attempt_sha256: super::digest(&read_bytes(&root, "attempt.json", MAX_METADATA)?),
        };
        result.validate()?;
        Ok(result)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != BINDING_SCHEMA {
            return Err("unsupported capture binding schema".into());
        }
        for hash in [
            &self.run_sha256,
            &self.plan_sha256,
            &self.protocol_sha256,
            &self.config_sha256,
            &self.binary_sha256,
            &self.attempt_sha256,
        ] {
            validate_sha256(hash)?;
        }
        if self.source_commit.len() != 40
            || !self
                .source_commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("capture requires a full lowercase source commit".into());
        }
        Ok(())
    }

    pub fn digest(&self) -> Result<String, String> {
        self.validate()?;
        Ok(super::digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaptureLimits {
    pub max_event_bytes: u64,
    pub max_stream_bytes: u64,
    /// Cumulative acknowledged payload bytes, also bounding each payload.
    pub max_payload_bytes: u64,
    pub max_payload_count: u64,
    /// Conservative peak of live data plus immutable snapshot copies and
    /// temporary chunk/head storage. Root-owned context/finalization is separate.
    pub max_capture_bytes: u64,
    pub chunk_bytes: u64,
    pub max_chunks: u64,
    pub metadata_reserve_bytes: u64,
}

impl CaptureLimits {
    pub fn validate(&self) -> Result<(), String> {
        if !(512..=MAX_EVENT).contains(&self.max_event_bytes)
            || !(self.max_event_bytes..=MAX_CHUNK).contains(&self.chunk_bytes)
            || !(self.max_event_bytes..=MAX_STREAM).contains(&self.max_stream_bytes)
            || self.max_payload_bytes > MAX_PAYLOAD
            || self.max_payload_count > MAX_ITEMS
            || !(1..=MAX_ITEMS).contains(&self.max_chunks)
            || !(4096..=MAX_METADATA).contains(&self.metadata_reserve_bytes)
            || self.max_capture_bytes == 0
        {
            return Err("invalid or unsupported capture limits".into());
        }
        self.local_peak(0, 0)?;
        Ok(())
    }

    fn local_peak(&self, stream: u64, payloads: u64) -> Result<(), String> {
        let peak = stream
            .checked_add(payloads)
            .and_then(|n| n.checked_mul(2))
            .and_then(|n| {
                self.metadata_reserve_bytes
                    .checked_mul(2)
                    .and_then(|m| n.checked_add(m))
            })
            .and_then(|n| n.checked_add(self.chunk_bytes))
            .ok_or("capture byte accounting overflow")?;
        if peak > self.max_capture_bytes {
            return Err("capture local byte reservation exhausted".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureStop {
    EventLimit,
    StreamByteLimit,
    PayloadByteLimit,
    PayloadCountLimit,
    LocalByteLimit,
    ChunkLimit,
    StorageFailure,
}

#[derive(Debug)]
pub enum CaptureError {
    LimitReached(CaptureStop),
    Io(String),
    Integrity(String),
}
impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitReached(reason) => write!(f, "capture limit reached: {reason:?}"),
            Self::Io(reason) => write!(f, "capture storage failure: {reason}"),
            Self::Integrity(reason) => write!(f, "capture integrity failure: {reason}"),
        }
    }
}
impl std::error::Error for CaptureError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventRecord {
    pub schema: String,
    pub binding_sha256: String,
    pub sequence: u64,
    pub previous_event_sha256: String,
    pub event: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CommittedHead {
    pub schema: String,
    pub binding_sha256: String,
    pub event_count: u64,
    pub byte_end: u64,
    pub last_event_sha256: String,
    pub payload_count: u64,
    pub payload_bytes: u64,
    pub stop: Option<CaptureStop>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaptureCursor {
    pub byte_end: u64,
    pub event_count: u64,
    pub last_event_sha256: String,
    pub next_chunk: u64,
    pub payload_count: u64,
    pub payload_bytes: u64,
}
impl CaptureCursor {
    pub fn initial(binding: &AttemptBinding) -> Result<Self, String> {
        let genesis = format!(
            "crypto.ml-dataset.capture-genesis/v1\n{}\n",
            binding.digest()?
        );
        Ok(Self {
            byte_end: 0,
            event_count: 0,
            last_event_sha256: super::digest(genesis.as_bytes()),
            next_chunk: 0,
            payload_count: 0,
            payload_bytes: 0,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EventSegment {
    pub artifact: Artifact,
    pub byte_start: u64,
    pub byte_end: u64,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub preceding_event_sha256: String,
    pub last_event_sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedDelta {
    pub binding: AttemptBinding,
    pub head: CommittedHead,
    pub segments: Vec<EventSegment>,
    /// Exact union of event chunks and newly acknowledged payload files.
    pub artifacts: Vec<Artifact>,
    pub cursor: CaptureCursor,
}

struct LimitedBytes {
    bytes: Vec<u8>,
    limit: usize,
}
impl Write for LimitedBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self
            .bytes
            .len()
            .checked_add(bytes.len())
            .is_none_or(|n| n > self.limit)
        {
            return Err(io::Error::other("serialization byte limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct CaptureEmitter {
    directory: PathBuf,
    events: File,
    head: CommittedHead,
    limits: CaptureLimits,
    payloads: BTreeMap<String, Artifact>,
    failed: bool,
}

impl CaptureEmitter {
    pub fn create(
        directory: &Path,
        binding: AttemptBinding,
        limits: CaptureLimits,
    ) -> Result<Self, String> {
        binding.validate()?;
        limits.validate()?;
        let directory = storage::root_dir(directory, true)?;
        let existing_head = storage::contained_path(&directory, "committed.json", false)?;
        if existing_head.exists() {
            return Err("capture watermark already exists; implicit resume is prohibited".into());
        }
        let events_path = storage::contained_path(&directory, "events.jsonl", false)?;
        // Existing streams are never resumed implicitly, even when empty.
        let events = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&events_path)
            .map_err(|e| format!("create exclusive capture stream: {e}"))?;
        events.sync_all().map_err(|e| e.to_string())?;
        sync_dir(&directory)?;
        if let Some(parent) = directory.parent() {
            sync_dir(parent)?;
        }
        let cursor = CaptureCursor::initial(&binding)?;
        let emitter = Self {
            directory,
            events,
            limits,
            payloads: BTreeMap::new(),
            failed: false,
            head: CommittedHead {
                schema: HEAD_SCHEMA.into(),
                binding_sha256: binding.digest()?,
                event_count: 0,
                byte_end: 0,
                last_event_sha256: cursor.last_event_sha256,
                payload_count: 0,
                payload_bytes: 0,
                stop: None,
            },
        };
        emitter
            .store_head(&emitter.head)
            .map_err(|e| e.to_string())?;
        Ok(emitter)
    }

    fn ready(&self) -> Result<(), CaptureError> {
        if let Some(stop) = &self.head.stop {
            return Err(CaptureError::LimitReached(stop.clone()));
        }
        if self.failed {
            return Err(CaptureError::Io(
                "emitter failed; a new attempt is required".into(),
            ));
        }
        Ok(())
    }

    fn encode_event(&self, event: &Value) -> Result<Vec<u8>, CaptureError> {
        #[derive(Serialize)]
        struct Record<'a> {
            schema: &'a str,
            binding_sha256: &'a str,
            sequence: u64,
            previous_event_sha256: &'a str,
            event: &'a Value,
        }
        let record = Record {
            schema: EVENT_SCHEMA,
            binding_sha256: &self.head.binding_sha256,
            sequence: self
                .head
                .event_count
                .checked_add(1)
                .ok_or_else(|| CaptureError::Integrity("event sequence overflow".into()))?,
            previous_event_sha256: &self.head.last_event_sha256,
            event,
        };
        let mut output = LimitedBytes {
            bytes: Vec::new(),
            limit: self.limits.max_event_bytes as usize - 1,
        };
        serde_json::to_writer(&mut output, &record)
            .map_err(|_| CaptureError::LimitReached(CaptureStop::EventLimit))?;
        output.bytes.push(b'\n');
        Ok(output.bytes)
    }

    fn check_reservation(
        &self,
        bytes: usize,
        payload_count: u64,
        payload_bytes: u64,
    ) -> Result<(), CaptureError> {
        let stream = self
            .head
            .byte_end
            .checked_add(bytes as u64)
            .ok_or_else(|| CaptureError::Integrity("stream byte count overflow".into()))?;
        if self.head.event_count >= MAX_ITEMS {
            return Err(CaptureError::LimitReached(CaptureStop::EventLimit));
        }
        if stream > self.limits.max_stream_bytes {
            return Err(CaptureError::LimitReached(CaptureStop::StreamByteLimit));
        }
        if payload_count > self.limits.max_payload_count {
            return Err(CaptureError::LimitReached(CaptureStop::PayloadCountLimit));
        }
        if payload_bytes > self.limits.max_payload_bytes {
            return Err(CaptureError::LimitReached(CaptureStop::PayloadByteLimit));
        }
        self.limits
            .local_peak(stream, payload_bytes)
            .map_err(|_| CaptureError::LimitReached(CaptureStop::LocalByteLimit))
    }

    fn reject(&mut self, error: CaptureError) -> CaptureError {
        self.failed = true;
        if let CaptureError::LimitReached(reason) = &error {
            let mut next = self.head.clone();
            next.stop = Some(reason.clone());
            if let Err(write_error) = self.store_head(&next) {
                return write_error;
            }
            self.head = next;
        }
        error
    }

    fn commit_event(
        &mut self,
        bytes: &[u8],
        payload_count: u64,
        payload_bytes: u64,
    ) -> Result<(), CaptureError> {
        let mut next = self.head.clone();
        next.event_count += 1;
        next.byte_end += bytes.len() as u64;
        next.last_event_sha256 = super::digest(bytes);
        next.payload_count = payload_count;
        next.payload_bytes = payload_bytes;
        if let Err(error) = self
            .events
            .write_all(bytes)
            .and_then(|_| self.events.sync_data())
        {
            self.failed = true;
            return Err(CaptureError::Io(error.to_string()));
        }
        if let Err(error) = self.store_head(&next) {
            self.failed = true;
            return Err(error);
        }
        self.head = next;
        Ok(())
    }

    pub fn event(&mut self, event: &Value) -> Result<(), CaptureError> {
        self.ready()?;
        if event["kind"] == PAYLOAD_KIND {
            return Err(self.reject(CaptureError::Integrity(
                "payload commit events are emitter-owned".into(),
            )));
        }
        let bytes = match self.encode_event(event) {
            Ok(v) => v,
            Err(e) => return Err(self.reject(e)),
        };
        if let Err(e) = self.check_reservation(
            bytes.len(),
            self.head.payload_count,
            self.head.payload_bytes,
        ) {
            return Err(self.reject(e));
        }
        self.commit_event(&bytes, self.head.payload_count, self.head.payload_bytes)
    }

    pub fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), CaptureError> {
        self.ready()?;
        if payload.path.is_empty()
            || payload.path.len() > 255
            || payload.path.contains(['/', '\\'])
            || [".", ".."].contains(&payload.path.as_str())
            || payload.role.trim().is_empty()
            || payload.encoding.trim().is_empty()
            || payload.role.len() > 256
            || payload.encoding.len() > 256
        {
            return Err(self.reject(CaptureError::Integrity(
                "invalid capture payload identity".into(),
            )));
        }
        let artifact = Artifact {
            path: format!("payloads/{}", payload.path),
            role: payload.role.clone(),
            encoding: payload.encoding.clone(),
            sha256: super::digest(&payload.bytes),
            bytes: payload.bytes.len() as u64,
        };
        if let Some(previous) = self.payloads.get(&artifact.path) {
            if previous != &artifact {
                return Err(self.reject(CaptureError::Integrity(
                    "payload name already has different bytes".into(),
                )));
            }
            let path = storage::contained_path(&self.directory, &artifact.path, false)
                .map_err(CaptureError::Integrity)?;
            return storage::check_file(&path, &artifact.sha256, artifact.bytes)
                .map_err(CaptureError::Integrity);
        }
        let payload_count = self
            .head
            .payload_count
            .checked_add(1)
            .ok_or_else(|| CaptureError::Integrity("payload count overflow".into()))?;
        let payload_bytes = self
            .head
            .payload_bytes
            .checked_add(artifact.bytes)
            .ok_or_else(|| CaptureError::Integrity("payload byte overflow".into()))?;
        let bytes = match self.encode_event(&json!({"kind":PAYLOAD_KIND,"artifact":artifact})) {
            Ok(v) => v,
            Err(e) => return Err(self.reject(e)),
        };
        if let Err(e) = self.check_reservation(bytes.len(), payload_count, payload_bytes) {
            return Err(self.reject(e));
        }
        let written = storage::write_bytes(
            &self.directory,
            &artifact.path,
            &artifact.role,
            &artifact.encoding,
            &payload.bytes,
        )
        .map_err(CaptureError::Io);
        if let Err(e) = written {
            return Err(self.reject(e));
        }
        self.commit_event(&bytes, payload_count, payload_bytes)?;
        self.payloads.insert(artifact.path.clone(), artifact);
        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), CaptureError> {
        self.ready()?;
        self.events
            .sync_all()
            .map_err(|e| CaptureError::Io(e.to_string()))?;
        self.store_head(&self.head)
    }

    fn store_head(&self, head: &CommittedHead) -> Result<(), CaptureError> {
        let mut output = LimitedBytes {
            bytes: Vec::new(),
            limit: self.limits.metadata_reserve_bytes as usize - 1,
        };
        serde_json::to_writer(&mut output, head).map_err(|e| CaptureError::Io(e.to_string()))?;
        output.bytes.push(b'\n');
        let destination = storage::contained_path(&self.directory, "committed.json", false)
            .map_err(CaptureError::Integrity)?;
        let name = format!(
            ".head-{}-{}.tmp",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        );
        let temp = storage::contained_path(&self.directory, &name, false)
            .map_err(CaptureError::Integrity)?;
        let operation = (|| -> io::Result<()> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp)?;
            file.write_all(&output.bytes)?;
            file.sync_all()?;
            fs::rename(&temp, &destination)?;
            File::open(&self.directory)?.sync_all()
        })();
        if temp.exists() {
            let _ = fs::remove_file(&temp);
        }
        operation.map_err(|e| CaptureError::Io(e.to_string()))
    }
}

fn sync_dir(path: &Path) -> Result<(), String> {
    File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(|e| format!("sync {}: {e}", path.display()))
}

fn read_bytes(root: &Path, relative: &str, max: u64) -> Result<Vec<u8>, String> {
    let path = storage::contained_path(root, relative, false)?;
    let mut file = File::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    if file.metadata().map_err(|e| e.to_string())?.len() > max {
        return Err(format!("{relative} exceeds capture input bound"));
    }
    let mut bytes = Vec::new();
    Read::by_ref(&mut file)
        .take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > max {
        return Err(format!("{relative} grew past capture input bound"));
    }
    Ok(bytes)
}
fn read_json<T: for<'de> Deserialize<'de>>(
    root: &Path,
    relative: &str,
    max: u64,
) -> Result<T, String> {
    serde_json::from_slice(&read_bytes(root, relative, max)?)
        .map_err(|e| format!("{relative}: {e}"))
}

fn validate_cursor(cursor: &CaptureCursor, binding: &AttemptBinding) -> Result<(), String> {
    validate_sha256(&cursor.last_event_sha256)?;
    if cursor.byte_end > MAX_STREAM
        || cursor.event_count > MAX_ITEMS
        || cursor.next_chunk > MAX_ITEMS
        || cursor.payload_count > cursor.event_count
        || cursor.payload_bytes > MAX_PAYLOAD
        || (cursor.byte_end == 0) != (cursor.event_count == 0)
        || (cursor.event_count == 0 && cursor != &CaptureCursor::initial(binding)?)
        || (cursor.event_count > 0
            && (cursor.next_chunk == 0 || cursor.next_chunk > cursor.event_count))
    {
        return Err("invalid capture cursor".into());
    }
    Ok(())
}

fn validate_head(
    head: &CommittedHead,
    binding: &AttemptBinding,
    cursor: &CaptureCursor,
) -> Result<(), String> {
    validate_cursor(cursor, binding)?;
    validate_sha256(&head.last_event_sha256)?;
    if head.schema != HEAD_SCHEMA
        || head.binding_sha256 != binding.digest()?
        || head.byte_end > MAX_STREAM
        || head.event_count > MAX_ITEMS
        || head.payload_count > MAX_ITEMS
        || head.payload_bytes > MAX_PAYLOAD
        || head.byte_end < cursor.byte_end
        || head.event_count < cursor.event_count
        || head.payload_count < cursor.payload_count
        || head.payload_bytes < cursor.payload_bytes
        || (head.byte_end == cursor.byte_end) != (head.event_count == cursor.event_count)
        || (head.byte_end == cursor.byte_end && head.last_event_sha256 != cursor.last_event_sha256)
        || (head.byte_end == cursor.byte_end
            && (head.payload_count != cursor.payload_count
                || head.payload_bytes != cursor.payload_bytes))
        || (head.event_count == 0 && (head.payload_count != 0 || head.payload_bytes != 0))
    {
        return Err("capture watermark does not extend the bound prefix".into());
    }
    Ok(())
}

fn check_record(
    line: &[u8],
    binding_sha: &str,
    sequence: u64,
    previous: &str,
) -> Result<(String, Option<Artifact>), String> {
    if line.len() as u64 > MAX_EVENT || !line.ends_with(b"\n") {
        return Err("oversized or incomplete committed event".into());
    }
    let row: EventRecord =
        serde_json::from_slice(line).map_err(|e| format!("committed event: {e}"))?;
    if row.schema != EVENT_SCHEMA
        || row.binding_sha256 != binding_sha
        || row.sequence != sequence
        || row.previous_event_sha256 != previous
    {
        return Err("capture event chain/sequence/binding mismatch".into());
    }
    let artifact = if row.event["kind"] == PAYLOAD_KIND {
        let artifact: Artifact = serde_json::from_value(row.event["artifact"].clone())
            .map_err(|e| format!("payload event: {e}"))?;
        validate_sha256(&artifact.sha256)?;
        let name = artifact
            .path
            .strip_prefix("payloads/")
            .ok_or("payload outside payload directory")?;
        if name.is_empty()
            || name.len() > 255
            || name.contains(['/', '\\'])
            || [".", ".."].contains(&name)
            || artifact.bytes > MAX_PAYLOAD
            || artifact.role.trim().is_empty()
            || artifact.encoding.trim().is_empty()
        {
            return Err("invalid committed payload reference".into());
        }
        Some(artifact)
    } else {
        None
    };
    Ok((super::digest(line), artifact))
}

/// Copy only a committed event prefix. Missing live/head is normal before the
/// solver starts. Context files are captured separately by the supervisor.
pub fn capture_committed_delta(
    cell_dir: &Path,
    snapshot_dir: &Path,
    binding: &AttemptBinding,
    cursor: &CaptureCursor,
    limits: &CaptureLimits,
) -> Result<Option<CapturedDelta>, String> {
    binding.validate()?;
    limits.validate()?;
    validate_cursor(cursor, binding)?;
    let cell = storage::root_dir(cell_dir, false)?;
    let live = cell.join("live");
    match fs::symlink_metadata(&live) {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return if cursor.event_count == 0 {
                Ok(None)
            } else {
                Err("established capture stream disappeared".into())
            };
        }
        Err(e) => return Err(e.to_string()),
    }
    let live = storage::root_dir(&live, false)?;
    let head_path = storage::contained_path(&live, "committed.json", false)?;
    if !head_path.exists() {
        return if cursor.event_count == 0 {
            Ok(None)
        } else {
            Err("established capture watermark disappeared".into())
        };
    }
    let head: CommittedHead = read_json(&live, "committed.json", limits.metadata_reserve_bytes)?;
    validate_head(&head, binding, cursor)?;
    if head.byte_end > limits.max_stream_bytes
        || head.payload_bytes > limits.max_payload_bytes
        || head.payload_count > limits.max_payload_count
    {
        return Err("capture watermark exceeds frozen limits".into());
    }
    limits.local_peak(head.byte_end, head.payload_bytes)?;
    if head.byte_end == cursor.byte_end && head.stop.is_none() {
        return Ok(None);
    }
    let output = storage::root_dir(snapshot_dir, true)?;
    let input = storage::contained_path(&live, "events.jsonl", false)?;
    let mut file = File::open(input).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() < head.byte_end {
        return Err("event stream shorter than committed watermark".into());
    }
    file.seek(SeekFrom::Start(cursor.byte_end))
        .map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file.take(head.byte_end - cursor.byte_end));
    let binding_sha = binding.digest()?;
    let mut next = cursor.clone();
    let mut segments = Vec::new();
    let mut artifacts = Vec::new();
    let mut payloads = BTreeMap::<String, Artifact>::new();
    let mut chunk = Vec::new();
    let mut start = next.clone();
    loop {
        let mut line = Vec::new();
        let count = Read::by_ref(&mut reader)
            .take(limits.max_event_bytes + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        if next.event_count >= head.event_count {
            return Err("event stream has more records than committed watermark".into());
        }
        if line.len() as u64 > limits.max_event_bytes {
            return Err("committed event exceeds configured bound".into());
        }
        let (hash, payload) = check_record(
            &line,
            &binding_sha,
            next.event_count + 1,
            &next.last_event_sha256,
        )?;
        if !chunk.is_empty() && chunk.len() as u64 + line.len() as u64 > limits.chunk_bytes {
            seal_chunk(
                &output,
                &mut chunk,
                &start,
                &mut next,
                limits,
                &mut segments,
                &mut artifacts,
            )?;
            start = next.clone();
        }
        chunk.extend_from_slice(&line);
        next.byte_end += line.len() as u64;
        next.event_count += 1;
        next.last_event_sha256 = hash;
        if let Some(payload) = payload {
            next.payload_count += 1;
            next.payload_bytes = next
                .payload_bytes
                .checked_add(payload.bytes)
                .ok_or("payload accounting overflow")?;
            if payloads.insert(payload.path.clone(), payload).is_some() {
                return Err("duplicate payload commit in event delta".into());
            }
        }
    }
    if !chunk.is_empty() {
        seal_chunk(
            &output,
            &mut chunk,
            &start,
            &mut next,
            limits,
            &mut segments,
            &mut artifacts,
        )?;
    }
    if next.byte_end != head.byte_end
        || next.event_count != head.event_count
        || next.last_event_sha256 != head.last_event_sha256
        || next.payload_count != head.payload_count
        || next.payload_bytes != head.payload_bytes
    {
        return Err("captured prefix does not match committed watermark".into());
    }
    for payload in payloads.values() {
        copy_payload(&live, &output, payload)?;
        artifacts.push(payload.clone());
    }
    let delta = CapturedDelta {
        binding: binding.clone(),
        head,
        segments,
        artifacts,
        cursor: next,
    };
    verify_captured_delta(&output, &delta, cursor)?;
    Ok(Some(delta))
}

fn seal_chunk(
    output: &Path,
    chunk: &mut Vec<u8>,
    start: &CaptureCursor,
    next: &mut CaptureCursor,
    limits: &CaptureLimits,
    segments: &mut Vec<EventSegment>,
    artifacts: &mut Vec<Artifact>,
) -> Result<(), String> {
    if next.next_chunk >= limits.max_chunks {
        return Err("capture chunk limit reached; no further chunk was sealed".into());
    }
    let artifact = storage::write_bytes(
        output,
        &format!("events/{:012}.jsonl", next.next_chunk),
        "committed-solver-event-prefix",
        "jsonl",
        chunk,
    )?;
    segments.push(EventSegment {
        artifact: artifact.clone(),
        byte_start: start.byte_end,
        byte_end: next.byte_end,
        first_sequence: start.event_count + 1,
        last_sequence: next.event_count,
        preceding_event_sha256: start.last_event_sha256.clone(),
        last_event_sha256: next.last_event_sha256.clone(),
    });
    artifacts.push(artifact);
    next.next_chunk += 1;
    chunk.clear();
    Ok(())
}

fn copy_payload(source: &Path, output: &Path, artifact: &Artifact) -> Result<(), String> {
    let src = storage::contained_path(source, &artifact.path, false)?;
    storage::check_file(&src, &artifact.sha256, artifact.bytes)?;
    let dest = storage::contained_path(output, &artifact.path, true)?;
    if dest.exists() {
        return storage::check_file(&dest, &artifact.sha256, artifact.bytes);
    }
    let parent = dest.parent().ok_or("payload has no directory")?;
    let temp = parent.join(format!(
        ".payload-{}-{}.tmp",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| -> Result<(), String> {
        let mut input = File::open(&src)
            .map_err(|e| e.to_string())?
            .take(artifact.bytes + 1);
        let mut target = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)
            .map_err(|e| e.to_string())?;
        let copied = io::copy(&mut input, &mut target).map_err(|e| e.to_string())?;
        if copied != artifact.bytes {
            return Err("payload changed length during capture".into());
        }
        target.sync_all().map_err(|e| e.to_string())?;
        storage::check_file(&temp, &artifact.sha256, artifact.bytes)?;
        match fs::hard_link(&temp, &dest) {
            Ok(()) => sync_dir(parent)?,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                storage::check_file(&dest, &artifact.sha256, artifact.bytes)?
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result
}

/// Integrity validation only. Does not interpret algebra, run solvers, or turn
/// an emitted prefix into a completed-cell or resumable-solver claim.
pub fn verify_captured_delta(
    snapshot_dir: &Path,
    delta: &CapturedDelta,
    previous: &CaptureCursor,
) -> Result<(), String> {
    delta.binding.validate()?;
    validate_head(&delta.head, &delta.binding, previous)?;
    validate_cursor(&delta.cursor, &delta.binding)?;
    if delta.segments.len() as u64 > MAX_ITEMS || delta.artifacts.len() as u64 > 2 * MAX_ITEMS {
        return Err("capture manifest has too many members".into());
    }
    let root = storage::root_dir(snapshot_dir, false)?;
    let mut total_bytes = 0u64;
    for artifact in &delta.artifacts {
        let limit = if artifact.path.starts_with("events/") {
            MAX_CHUNK
        } else if artifact.path.starts_with("payloads/") {
            MAX_PAYLOAD
        } else {
            return Err("unexpected capture artifact directory".into());
        };
        total_bytes = total_bytes
            .checked_add(artifact.bytes)
            .ok_or("capture artifact byte sum overflow")?;
        let path = storage::contained_path(&root, &artifact.path, false)?;
        if artifact.bytes > limit
            || total_bytes > MAX_STREAM + MAX_PAYLOAD
            || fs::metadata(&path).map_err(|e| e.to_string())?.len() != artifact.bytes
        {
            return Err("capture artifact exceeds declared or fixed byte bounds".into());
        }
    }
    storage::verify_manifest(&root, &delta.artifacts)?;
    let all: BTreeMap<_, _> = delta
        .artifacts
        .iter()
        .map(|a| (a.path.clone(), a.clone()))
        .collect();
    let mut referenced = BTreeSet::new();
    let mut expected = previous.clone();
    let binding_sha = delta.binding.digest()?;
    for segment in &delta.segments {
        if segment.artifact.path != format!("events/{:012}.jsonl", expected.next_chunk)
            || segment.artifact.role != "committed-solver-event-prefix"
            || segment.artifact.encoding != "jsonl"
            || segment.byte_start != expected.byte_end
            || segment.first_sequence != expected.event_count + 1
            || segment.preceding_event_sha256 != expected.last_event_sha256
            || segment.artifact.bytes == 0
            || segment.artifact.bytes > MAX_CHUNK
            || all.get(&segment.artifact.path) != Some(&segment.artifact)
            || !referenced.insert(segment.artifact.path.clone())
        {
            return Err("invalid event segment metadata or order".into());
        }
        let bytes = read_bytes(&root, &segment.artifact.path, MAX_CHUNK)?;
        for line in bytes.split_inclusive(|b| *b == b'\n') {
            let (hash, payload) = check_record(
                line,
                &binding_sha,
                expected.event_count + 1,
                &expected.last_event_sha256,
            )?;
            expected.byte_end += line.len() as u64;
            expected.event_count += 1;
            expected.last_event_sha256 = hash;
            if let Some(payload) = payload {
                if all.get(&payload.path) != Some(&payload)
                    || !referenced.insert(payload.path.clone())
                {
                    return Err("payload event lacks its unique matching artifact".into());
                }
                expected.payload_count += 1;
                expected.payload_bytes = expected
                    .payload_bytes
                    .checked_add(payload.bytes)
                    .ok_or("payload byte overflow")?;
            }
        }
        if segment.byte_end != expected.byte_end
            || segment.last_sequence != expected.event_count
            || segment.last_event_sha256 != expected.last_event_sha256
        {
            return Err("event segment end does not match its bytes".into());
        }
        expected.next_chunk += 1;
    }
    if referenced.len() != all.len()
        || delta.cursor != expected
        || delta.head.byte_end != expected.byte_end
        || delta.head.event_count != expected.event_count
        || delta.head.last_event_sha256 != expected.last_event_sha256
        || expected.payload_count != delta.head.payload_count
        || expected.payload_bytes != delta.head.payload_bytes
    {
        return Err("captured delta artifact set/cursor/head mismatch".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "ml-capture-{}-{}",
                std::process::id(),
                TEMP_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            fs::create_dir(root.join("cell")).unwrap();
            Self(root)
        }
        fn cell(&self) -> PathBuf {
            self.0.join("cell")
        }
        fn live(&self) -> PathBuf {
            self.cell().join("live")
        }
        fn snapshot(&self, n: u64) -> PathBuf {
            self.0.join(format!("snapshot-{n}"))
        }
        fn emitter(&self) -> CaptureEmitter {
            CaptureEmitter::create(&self.live(), binding(), limits()).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn binding() -> AttemptBinding {
        AttemptBinding {
            schema: BINDING_SCHEMA.into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "a".repeat(40),
            binary_sha256: "5".repeat(64),
            cell_index: 7,
            attempt_sha256: "6".repeat(64),
        }
    }
    fn limits() -> CaptureLimits {
        CaptureLimits {
            max_event_bytes: 1024,
            max_stream_bytes: 64 * 1024,
            max_payload_bytes: 16 * 1024,
            max_payload_count: 8,
            max_capture_bytes: 512 * 1024,
            chunk_bytes: 1024,
            max_chunks: 128,
            metadata_reserve_bytes: 4096,
        }
    }

    #[test]
    fn acknowledged_prefix_survives_unacknowledged_torn_tail() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter
            .event(&json!({"kind":"before_opaque_call","x":3}))
            .unwrap();
        OpenOptions::new()
            .append(true)
            .open(f.live().join("events.jsonl"))
            .unwrap()
            .write_all(b"{torn")
            .unwrap();
        let initial = CaptureCursor::initial(&binding()).unwrap();
        let delta =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &initial, &limits())
                .unwrap()
                .unwrap();
        assert_eq!(delta.cursor.event_count, 1);
        verify_captured_delta(&f.snapshot(0), &delta, &initial).unwrap();
        assert!(
            !fs::read(f.snapshot(0).join(&delta.segments[0].artifact.path))
                .unwrap()
                .ends_with(b"{torn")
        );
    }

    #[test]
    fn immutable_chunks_and_payloads_validate_incremental_capture() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        for i in 0..6 {
            emitter.event(&json!({"kind":"step","i":i})).unwrap();
        }
        let payload =
            ArtifactPayload::json("basis.json", "exposed-basis", &json!({"rows":[1,2]})).unwrap();
        emitter.payload(&payload).unwrap();
        emitter.payload(&payload).unwrap();
        let initial = CaptureCursor::initial(&binding()).unwrap();
        let first =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &initial, &limits())
                .unwrap()
                .unwrap();
        assert_eq!(first.head.payload_count, 1);
        assert!(first.segments.len() > 1);
        assert_eq!(
            fs::read(f.snapshot(0).join("payloads/basis.json")).unwrap(),
            payload.bytes
        );
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(1),
            &binding(),
            &first.cursor,
            &limits()
        )
        .unwrap()
        .is_none());
        emitter.event(&json!({"kind":"after"})).unwrap();
        let second = capture_committed_delta(
            &f.cell(),
            &f.snapshot(1),
            &binding(),
            &first.cursor,
            &limits(),
        )
        .unwrap()
        .unwrap();
        verify_captured_delta(&f.snapshot(1), &second, &first.cursor).unwrap();
        assert_eq!(second.artifacts.len(), 1);
        assert_eq!(second.cursor.event_count, 8);
    }

    #[test]
    fn cap_rejection_preserves_previous_prefix_and_marks_censoring() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.event(&json!({"kind":"kept"})).unwrap();
        let before = fs::read(f.live().join("events.jsonl")).unwrap();
        assert!(matches!(
            emitter.event(&json!({"oversized":"x".repeat(3000)})),
            Err(CaptureError::LimitReached(CaptureStop::EventLimit))
        ));
        assert_eq!(fs::read(f.live().join("events.jsonl")).unwrap(), before);
        let delta = capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &binding(),
            &CaptureCursor::initial(&binding()).unwrap(),
            &limits(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(delta.head.stop, Some(CaptureStop::EventLimit));
        assert!(emitter.event(&json!({"kind":"later"})).is_err());
    }

    #[test]
    fn payload_cap_precedes_creation_and_name_conflicts_are_rejected() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        let payload = ArtifactPayload {
            path: "too-big.bin".into(),
            role: "test".into(),
            encoding: "bytes".into(),
            bytes: vec![1; 20 * 1024],
        };
        assert!(matches!(
            emitter.payload(&payload),
            Err(CaptureError::LimitReached(CaptureStop::PayloadByteLimit))
        ));
        assert!(!f.live().join("payloads/too-big.bin").exists());
        let g = Fixture::new();
        let mut emitter = g.emitter();
        let mut payload = ArtifactPayload::json("a.json", "test", &json!([1])).unwrap();
        emitter.payload(&payload).unwrap();
        payload.bytes.push(b' ');
        assert!(emitter.payload(&payload).is_err());
    }

    #[test]
    fn corrupt_chain_watermark_and_binding_fail_closed() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.event(&json!({"kind":"kept"})).unwrap();
        let initial = CaptureCursor::initial(&binding()).unwrap();
        let mut wrong = binding();
        wrong.attempt_sha256 = "f".repeat(64);
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &wrong,
            &CaptureCursor::initial(&wrong).unwrap(),
            &limits()
        )
        .is_err());
        let path = f.live().join("events.jsonl");
        let mut bytes = fs::read(&path).unwrap();
        let at = bytes.windows(4).position(|w| w == b"kept").unwrap();
        bytes[at] = b'X';
        fs::write(&path, bytes).unwrap();
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(1),
            &binding(),
            &initial,
            &limits()
        )
        .is_err());
    }

    #[test]
    fn mutation_or_extra_artifact_invalidates_captured_delta() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.event(&json!({"kind":"x"})).unwrap();
        let initial = CaptureCursor::initial(&binding()).unwrap();
        let mut delta =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &initial, &limits())
                .unwrap()
                .unwrap();
        let extra = storage::write_bytes(
            &f.snapshot(0),
            "extra.bin",
            "test",
            "bytes",
            b"unreferenced",
        )
        .unwrap();
        delta.artifacts.push(extra);
        assert!(verify_captured_delta(&f.snapshot(0), &delta, &initial).is_err());
        delta.artifacts.pop();
        fs::write(
            f.snapshot(0).join(&delta.segments[0].artifact.path),
            b"changed",
        )
        .unwrap();
        assert!(verify_captured_delta(&f.snapshot(0), &delta, &initial).is_err());
    }

    #[test]
    fn orphan_payload_is_not_acknowledged_and_restart_is_not_resume() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.event(&json!({"kind":"x"})).unwrap();
        storage::write_bytes(
            &f.live(),
            "payloads/orphan.bin",
            "test",
            "bytes",
            b"not acknowledged",
        )
        .unwrap();
        let delta = capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &binding(),
            &CaptureCursor::initial(&binding()).unwrap(),
            &limits(),
        )
        .unwrap()
        .unwrap();
        assert!(!delta.artifacts.iter().any(|a| a.path.contains("orphan")));
        drop(emitter);
        assert!(CaptureEmitter::create(&f.live(), binding(), limits()).is_err());
    }

    #[test]
    fn missing_head_is_not_completion_and_serde_is_strict() {
        let f = Fixture::new();
        let initial = CaptureCursor::initial(&binding()).unwrap();
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &binding(),
            &initial,
            &limits()
        )
        .unwrap()
        .is_none());
        let mut bad = serde_json::to_value(binding()).unwrap();
        bad["solver_resumable"] = json!(true);
        assert!(serde_json::from_value::<AttemptBinding>(bad).is_err());
        let mut invalid = limits();
        invalid.max_capture_bytes = u64::MAX;
        invalid.metadata_reserve_bytes = u64::MAX;
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn chunk_limit_never_acknowledges_missing_chunks() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        for i in 0..10 {
            emitter.event(&json!({"step":i})).unwrap();
        }
        let mut cap = limits();
        cap.max_chunks = 1;
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &binding(),
            &CaptureCursor::initial(&binding()).unwrap(),
            &cap
        )
        .is_err());
        assert!(!f.snapshot(0).join("events/000000000001.jsonl").exists());
    }

    #[cfg(unix)]
    #[test]
    fn symlink_payload_and_stream_are_rejected() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.event(&json!({"x":1})).unwrap();
        let original = f.live().join("events.jsonl");
        let moved = f.0.join("stream");
        fs::rename(&original, &moved).unwrap();
        symlink(&moved, &original).unwrap();
        assert!(capture_committed_delta(
            &f.cell(),
            &f.snapshot(0),
            &binding(),
            &CaptureCursor::initial(&binding()).unwrap(),
            &limits()
        )
        .is_err());
    }
}
