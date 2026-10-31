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
        // Run/attempt identity uses the typed plan; retained context below
        // separately binds the exact saved bytes, including their formatting.
        let plan_sha256 = plan.binding_sha256()?;
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

pub fn capture_limits(trace_mib: u64) -> Result<CaptureLimits, String> {
    let total = trace_mib
        .checked_mul(1024 * 1024)
        .ok_or("capture limit overflow")?;
    if total < 1024 * 1024 {
        return Err("capture requires at least one MiB of worker output allowance".into());
    }
    let limits = CaptureLimits {
        max_event_bytes: (total / 16).min(1024 * 1024),
        max_stream_bytes: total / 8,
        max_payload_bytes: total / 8,
        max_payload_count: 4096,
        max_capture_bytes: total,
        chunk_bytes: (total / 16).min(1024 * 1024),
        max_chunks: 4096,
        metadata_reserve_bytes: (total / 8).min(1024 * 1024),
    };
    limits.validate()?;
    Ok(limits)
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

/// Closed kinds for typed solver events. Payload commits remain emitter-owned.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum TypedEventKind {
    #[serde(rename = "wide_prime_dynamic")]
    WidePrimeDynamic,
    #[serde(rename = "koblitz_m3")]
    KoblitzM3,
    #[serde(rename = "koblitz_m3_native")]
    KoblitzM3Native,
}

/// Reusable requested storage; the caller must debit its own ledger once.
/// The supplied admission is checked here, not authenticated against that ledger.
pub struct TypedEventBuffer {
    bytes: Vec<u8>,
    limit: usize,
}
impl TypedEventBuffer {
    /// Current encoded buffer length. Only a successful event_typed return
    /// establishes acknowledgement; this accessor alone does not.
    pub(super) fn encoded_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn requested_bytes(event_cap: u64) -> Result<u64, CaptureError> {
        if !(512..=MAX_EVENT).contains(&event_cap) {
            return Err(CaptureError::LimitReached(CaptureStop::EventLimit));
        }
        event_cap
            .checked_add(128)
            .ok_or_else(|| CaptureError::Integrity("typed capture storage overflow".into()))
    }

    pub fn try_new(event_cap: u64, admitted_requested_bytes: u64) -> Result<Self, CaptureError> {
        let requested = Self::requested_bytes(event_cap)?;
        if requested > admitted_requested_bytes {
            return Err(CaptureError::LimitReached(CaptureStop::LocalByteLimit));
        }
        let limit = usize::try_from(event_cap)
            .map_err(|_| CaptureError::LimitReached(CaptureStop::EventLimit))?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(limit)
            .map_err(|_| CaptureError::Io("typed capture buffer allocation failed".into()))?;
        Ok(Self { bytes, limit })
    }
}

struct CountedEventBytes {
    len: usize,
    limit: usize,
}
impl Write for CountedEventBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let next = self
            .len
            .checked_add(bytes.len())
            .filter(|&n| n <= self.limit)
            .ok_or_else(|| io::Error::other("typed event byte limit"))?;
        self.len = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
struct AdmittedEventBytes<'a> {
    bytes: &'a mut Vec<u8>,
    limit: usize,
}
impl Write for AdmittedEventBytes<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let next = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .filter(|&n| n <= self.limit && n <= self.bytes.capacity())
            .ok_or_else(|| io::Error::other("typed event byte limit"))?;
        self.bytes.extend_from_slice(bytes);
        debug_assert_eq!(self.bytes.len(), next);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn typed_encoding_error(error: serde_json::Error) -> CaptureError {
    if error.is_io() {
        CaptureError::LimitReached(CaptureStop::EventLimit)
    } else {
        CaptureError::Integrity("typed event serialization failed".into())
    }
}

#[derive(Serialize)]
struct TypedSolverEvent<'a, T: Serialize + ?Sized> {
    kind: TypedEventKind,
    payload: &'a T,
}
#[derive(Serialize)]
struct TypedEventRecord<'a, T: Serialize + ?Sized> {
    schema: &'a str,
    binding_sha256: &'a str,
    sequence: u64,
    previous_event_sha256: &'a str,
    event: TypedSolverEvent<'a, T>,
}

// Dedicated bounded first-witness publication; not a dispatched solver profile.
const WITNESS_PATH: &str = "payloads/pdp-witness.json";
const WITNESS_ROLE: &str = "independently-verified-pdp";
const WITNESS_DYNAMIC_BYTES: u64 = 27_648;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WitnessError {
    Unbound,
    Failed,
    Deadline,
    Work,
    Storage,
    Allocation,
    Identity,
    Integrity,
    EventLimit,
    Encoding,
    Io(std::io::ErrorKind),
    IndeterminateIo(std::io::ErrorKind),
}
impl From<std::io::Error> for WitnessError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.kind())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WitnessCommit {
    NewCommit,
    AlreadyCommitted,
}
/// Identity provenance, not a mathematical assurance or reconstructed acknowledgement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WitnessProvenance {
    Dedicated,
    LegacyAdopted,
}
/// Returned only after this call acknowledges the verified durable identity.
/// Legacy adoption has no recorded original sequence; central replay resolves it
/// through the committed event prefix. Never substitute the current head sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct WitnessReceipt {
    pub commit: WitnessCommit,
    pub payload_sha256: [u8; 32],
    pub payload_bytes: u64,
    pub sequence: Option<u64>,
    pub provenance: WitnessProvenance,
}
#[derive(Clone, Copy, Default)]
struct WitnessIdentity {
    committed: bool,
    digest: [u8; 32],
    length: u64,
    sequence: Option<u64>,
}
impl WitnessIdentity {
    fn receipt(self, commit: WitnessCommit) -> WitnessReceipt {
        WitnessReceipt {
            commit,
            payload_sha256: self.digest,
            payload_bytes: self.length,
            sequence: self.sequence,
            provenance: if self.sequence.is_some() {
                WitnessProvenance::Dedicated
            } else {
                WitnessProvenance::LegacyAdopted
            },
        }
    }
}
/// Caller supplies only the remaining original checking pool and original deadline.
/// Reconcile these cumulative receipts even on failure; never reset per retry.
#[allow(dead_code)]
pub(super) struct WitnessBudget {
    work_limit: u64,
    bytes_limit: u64,
    deadline: std::time::Instant,
    pub reserved_work: u64,
    pub performed_work: u64,
    pub reserved_bytes: u64,
}
#[allow(dead_code)] // Bound API remains dormant until parent integration.
impl WitnessBudget {
    pub fn new(work: u64, bytes: u64, deadline: std::time::Instant) -> Self {
        Self {
            work_limit: work,
            bytes_limit: bytes,
            deadline,
            reserved_work: 0,
            performed_work: 0,
            reserved_bytes: 0,
        }
    }
    /// Exclusively borrow the caller's current original parent remainder.
    /// The caller must release and settle cumulative deltas on EVERY return;
    /// supplied amounts are assertions, not independently authenticated grants.
    /// Clock-independent reconciliation never changes the original deadline.
    pub fn set_exclusive_supply(
        &mut self,
        remaining_work: u64,
        remaining_bytes: u64,
    ) -> Result<(), WitnessError> {
        // Close stale supply even on overflow. Install both new limits atomically.
        self.release_exclusive_supply();
        let work_limit = self
            .reserved_work
            .checked_add(remaining_work)
            .ok_or(WitnessError::Work)?;
        let bytes_limit = self
            .reserved_bytes
            .checked_add(remaining_bytes)
            .ok_or(WitnessError::Storage)?;
        self.work_limit = work_limit;
        self.bytes_limit = bytes_limit;
        Ok(())
    }
    /// Withdraw only unreserved supply; charged reservations never refund.
    /// Safe Rust prevents this call while a local WitnessTicket borrows us.
    pub fn release_exclusive_supply(&mut self) {
        self.work_limit = self.reserved_work;
        self.bytes_limit = self.reserved_bytes;
    }
    fn check(&self) -> Result<(), WitnessError> {
        if std::time::Instant::now() >= self.deadline {
            Err(WitnessError::Deadline)
        } else {
            Ok(())
        }
    }
    fn reserve(&mut self, work: u64, bytes: u64) -> Result<WitnessTicket<'_>, WitnessError> {
        self.check()?;
        let w = self
            .reserved_work
            .checked_add(work)
            .ok_or(WitnessError::Work)?;
        let b = self
            .reserved_bytes
            .checked_add(bytes)
            .ok_or(WitnessError::Storage)?;
        if w > self.work_limit {
            return Err(WitnessError::Work);
        }
        if b > self.bytes_limit {
            return Err(WitnessError::Storage);
        }
        self.reserved_work = w;
        self.reserved_bytes = b;
        Ok(WitnessTicket {
            budget: self,
            remaining: work,
        })
    }
}
struct WitnessTicket<'a> {
    budget: &'a mut WitnessBudget,
    remaining: u64,
}
impl WitnessTicket<'_> {
    fn spend(&mut self, n: u64) -> Result<(), WitnessError> {
        self.budget.check()?;
        self.remaining = self.remaining.checked_sub(n).ok_or(WitnessError::Work)?;
        self.budget.performed_work = self
            .budget
            .performed_work
            .checked_add(n)
            .ok_or(WitnessError::Work)?;
        Ok(())
    }
    fn check(&self) -> Result<(), WitnessError> {
        self.budget.check()
    }
}
#[allow(dead_code)]
pub(super) struct WitnessCaptureWorkspace {
    event: Vec<u8>,
    head: Vec<u8>,
    io: Vec<u8>,
    paths: [PathBuf; 5],
    binding: [u8; 64],
    #[cfg(test)]
    fault: u8,
}
impl WitnessCaptureWorkspace {
    pub fn requested_bytes() -> u64 {
        WITNESS_DYNAMIC_BYTES
            + std::mem::size_of::<Self>() as u64
            + std::mem::size_of::<WitnessIdentity>() as u64
            + std::mem::size_of::<WitnessScratchLayout>() as u64
    }
}
// Explicit requested simultaneous helper layouts, not an unused owned scratch
// array plus a second uncharged copy on the stack; not compiler stack/RSS.
#[allow(dead_code)]
struct WitnessScratchLayout {
    hashes: [crate::hash::sha256::Sha256; 2],
    schedule: [u32; 64],
    padding: [u8; 128],
    hex: [[u8; 64]; 2],
    event: WitnessEventRecord<'static>,
    head: WitnessHead<'static>,
    writer: WitnessWriter<'static, 'static>,
    ticket: WitnessTicket<'static>,
    files: [File; 3],
    receipt: WitnessReceipt,
    scalar_cursors: [u64; 32],
}
fn witness_hash_work(n: usize) -> Result<u64, WitnessError> {
    (n as u64 / 64)
        .checked_add(2)
        .and_then(|x| x.checked_mul(65_536))
        .and_then(|x| x.checked_add((n as u64).checked_mul(16)?))
        .and_then(|x| x.checked_add(1024))
        .ok_or(WitnessError::Work)
}
/// Admission cost of confirming an already committed witness. The caller may
/// hold this within its original allowance while continuing other work.
pub(super) fn witness_confirmation_work(n: usize) -> Result<u64, WitnessError> {
    witness_hash_work(n)?
        .checked_add((n as u64).checked_mul(32).ok_or(WitnessError::Work)?)
        .and_then(|x| x.checked_add(8192))
        .ok_or(WitnessError::Work)
}
fn witness_hex(digest: &[u8; 32]) -> [u8; 64] {
    let mut hex = [0; 64];
    let alphabet = b"0123456789abcdef";
    for (i, &b) in digest.iter().enumerate() {
        hex[2 * i] = alphabet[(b >> 4) as usize];
        hex[2 * i + 1] = alphabet[(b & 15) as usize];
    }
    hex
}
fn witness_hash(bytes: &[u8], ticket: &mut WitnessTicket<'_>) -> Result<[u8; 32], WitnessError> {
    let mut h = crate::hash::sha256::Sha256::new();
    for chunk in bytes.chunks(64) {
        ticket.spend(16 * chunk.len() as u64 + if chunk.len() == 64 { 65_536 } else { 0 })?;
        h.update(chunk);
    }
    ticket.spend(2 * 65_536 + 1024)?;
    let result = h.finalize();
    ticket.check()?;
    Ok(result)
}
struct WitnessWriter<'a, 'b> {
    output: Option<&'a mut Vec<u8>>,
    ticket: &'a mut WitnessTicket<'b>,
    count: usize,
    cap: usize,
    error: Option<WitnessError>,
}
impl Write for WitnessWriter<'_, '_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let result = (|| {
            if self.error.is_some() {
                return Err(WitnessError::Encoding);
            }
            let end = self
                .count
                .checked_add(bytes.len())
                .ok_or(WitnessError::Encoding)?;
            if end > self.cap {
                return Err(WitnessError::Encoding);
            }
            // Empty writes are free; deterministic private serializers cannot spin.
            for chunk in bytes.chunks(64) {
                self.ticket.spend(40 * chunk.len() as u64)?;
                if let Some(out) = &mut self.output {
                    out.extend_from_slice(chunk);
                }
                self.count += chunk.len();
            }
            Ok(bytes.len())
        })();
        result.map_err(|e| {
            self.error = Some(e);
            io::ErrorKind::Other.into()
        })
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn witness_encode<T: Serialize>(
    value: &T,
    out: Option<&mut Vec<u8>>,
    cap: usize,
    ticket: &mut WitnessTicket<'_>,
) -> Result<usize, WitnessError> {
    let mut writer = WitnessWriter {
        output: out,
        ticket,
        count: 0,
        cap: cap.checked_sub(1).ok_or(WitnessError::Encoding)?,
        error: None,
    };
    serde_json::to_writer(&mut writer, value)
        .map_err(|_| writer.error.unwrap_or(WitnessError::Encoding))?;
    let n = writer.count.checked_add(1).ok_or(WitnessError::Encoding)?;
    writer.cap = cap;
    writer
        .write_all(b"\n")
        .map_err(|_| writer.error.unwrap_or(WitnessError::Encoding))?;
    Ok(n)
}
// Legacy json! object keys are sorted. Keep exact event/artifact ordering.
#[derive(Serialize)]
struct WitnessArtifact<'a> {
    bytes: u64,
    encoding: &'static str,
    path: &'static str,
    role: &'static str,
    sha256: &'a str,
}
#[derive(Serialize)]
struct WitnessPayloadEvent<'a> {
    artifact: WitnessArtifact<'a>,
    kind: &'static str,
}
#[derive(Serialize)]
struct WitnessEventRecord<'a> {
    schema: &'static str,
    binding_sha256: &'a str,
    sequence: u64,
    previous_event_sha256: &'a str,
    event: WitnessPayloadEvent<'a>,
}
#[derive(Serialize)]
struct WitnessHead<'a> {
    schema: &'a str,
    binding_sha256: &'a str,
    event_count: u64,
    byte_end: u64,
    last_event_sha256: &'a str,
    payload_count: u64,
    payload_bytes: u64,
    stop: Option<&'a CaptureStop>,
}
fn witness_regular(path: &Path) -> Result<(), WitnessError> {
    let m = fs::symlink_metadata(path)?;
    if !m.is_file() || m.file_type().is_symlink() {
        return Err(WitnessError::Integrity);
    }
    Ok(())
}
fn witness_file_check(
    path: &Path,
    expected: &[u8; 32],
    length: u64,
    scratch: &mut Vec<u8>,
    ticket: &mut WitnessTicket<'_>,
) -> Result<(), WitnessError> {
    ticket.check()?;
    witness_regular(path)?;
    let mut file = File::open(path)?;
    if file.metadata()?.len() != length {
        return Err(WitnessError::Integrity);
    }
    let mut h = crate::hash::sha256::Sha256::new();
    let mut remaining = length;
    scratch.resize(4096, 0);
    while remaining > 0 {
        ticket.check()?;
        let n = remaining.min(4096) as usize;
        file.read_exact(&mut scratch[..n])?;
        ticket.spend(16 * n as u64)?;
        for chunk in scratch[..n].chunks(64) {
            ticket.spend(16 * chunk.len() as u64 + if chunk.len() == 64 { 65_536 } else { 0 })?;
            h.update(chunk);
        }
        remaining -= n as u64;
    }
    let mut extra = [0u8; 1];
    if file.read(&mut extra)? != 0 {
        return Err(WitnessError::Integrity);
    }
    ticket.spend(2 * 65_536 + 1024)?;
    let digest = h.finalize();
    ticket.check()?;
    if &digest != expected {
        return Err(WitnessError::Integrity);
    }
    Ok(())
}
/// The supplied bytes were already hashed against the committed identity.
/// Exact comparison binds the file to those bytes without hashing it again.
/// Reuses the admitted I/O buffer; read and comparison each retain their
/// existing conservative 16-per-byte tariff.
fn witness_file_matches(
    path: &Path,
    expected: &[u8],
    scratch: &mut Vec<u8>,
    ticket: &mut WitnessTicket<'_>,
) -> Result<(), WitnessError> {
    ticket.check()?;
    witness_regular(path)?;
    let mut file = File::open(path)?;
    if file.metadata()?.len() != expected.len() as u64 {
        return Err(WitnessError::Integrity);
    }
    scratch.resize(4096, 0);
    for chunk in expected.chunks(4096) {
        ticket.check()?;
        ticket.spend(32 * chunk.len() as u64)?;
        file.read_exact(&mut scratch[..chunk.len()])?;
        if &scratch[..chunk.len()] != chunk {
            return Err(WitnessError::Integrity);
        }
    }
    let mut extra = [0u8; 1];
    if file.read(&mut extra)? != 0 {
        return Err(WitnessError::Integrity);
    }
    ticket.check()?;
    Ok(())
}
fn witness_write(
    file: &mut File,
    bytes: &[u8],
    ticket: &mut WitnessTicket<'_>,
) -> Result<(), WitnessError> {
    for chunk in bytes.chunks(4096) {
        ticket.spend(16 * chunk.len() as u64)?;
        file.write_all(chunk)?;
    }
    Ok(())
}
impl CaptureEmitter {
    /// Prepare under the caller's remaining shared byte/checking pools, before native.
    #[allow(dead_code)]
    pub(super) fn prepare_witness(
        &mut self,
        budget: &mut WitnessBudget,
    ) -> Result<WitnessCaptureWorkspace, WitnessError> {
        if self.failed || self.head.stop.is_some() {
            return Err(WitnessError::Failed);
        }
        let result = (|| {
            let root_len = self.directory.as_os_str().as_encoded_bytes().len();
            if root_len > 3968
                || self.head.binding_sha256.len() != 64
                || self.head.last_event_sha256.len() != 64
                || self.head.last_event_sha256.capacity() < 64
            {
                return Err(WitnessError::Identity);
            }
            let suffixes = [
                "payloads",
                WITNESS_PATH,
                "payloads/.witness-payload.tmp",
                "committed.json",
                ".witness-head.tmp",
            ];
            let path_bytes = suffixes
                .iter()
                .try_fold(0u64, |sum, s| {
                    sum.checked_add((root_len + 1 + s.len()) as u64)
                })
                .ok_or(WitnessError::Storage)?;
            let work = 8 * WITNESS_DYNAMIC_BYTES + 4096 + 16 * path_bytes;
            let mut ticket = budget.reserve(work, WitnessCaptureWorkspace::requested_bytes())?;
            ticket.spend(work)?;
            let mut event = Vec::new();
            event
                .try_reserve_exact(1024)
                .map_err(|_| WitnessError::Allocation)?;
            let mut head = Vec::new();
            head.try_reserve_exact(1024)
                .map_err(|_| WitnessError::Allocation)?;
            let mut buffer = Vec::new();
            buffer
                .try_reserve_exact(4096)
                .map_err(|_| WitnessError::Allocation)?;
            buffer.resize(4096, 0); // initialization paid by preparation ticket
            let mut paths: [PathBuf; 5] = std::array::from_fn(|_| PathBuf::new());
            for (path, suffix) in paths.iter_mut().zip(suffixes) {
                path.try_reserve_exact(4096)
                    .map_err(|_| WitnessError::Allocation)?;
                path.push(&self.directory);
                path.push(suffix);
                if path.as_os_str().as_encoded_bytes().len() > 4096 {
                    return Err(WitnessError::Identity);
                }
            }
            let root = fs::symlink_metadata(&self.directory)?;
            if !root.is_dir() || root.file_type().is_symlink() {
                return Err(WitnessError::Identity);
            }
            match fs::create_dir(&paths[0]) {
                Ok(()) => File::open(&self.directory)?.sync_all()?,
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e.into()),
            }
            let dir = fs::symlink_metadata(&paths[0])?;
            if !dir.is_dir() || dir.file_type().is_symlink() {
                return Err(WitnessError::Identity);
            }
            for i in [1, 2, 3, 4] {
                match fs::symlink_metadata(&paths[i]) {
                    Ok(m) if i == 2 || i == 4 || !m.is_file() || m.file_type().is_symlink() => {
                        return Err(WitnessError::Identity)
                    }
                    Ok(_) => {}
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e.into()),
                }
            }
            let mut binding = [0; 64];
            binding.copy_from_slice(self.head.binding_sha256.as_bytes());
            Ok(WitnessCaptureWorkspace {
                event,
                head,
                io: buffer,
                paths,
                binding,
                #[cfg(test)]
                fault: 0,
            })
        })();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn witness_limits(
        &self,
        event_bytes: usize,
        count: u64,
        bytes: u64,
    ) -> Result<(), WitnessError> {
        let stream = self
            .head
            .byte_end
            .checked_add(event_bytes as u64)
            .ok_or(WitnessError::EventLimit)?;
        if self.head.event_count >= MAX_ITEMS
            || stream > self.limits.max_stream_bytes
            || count > self.limits.max_payload_count
            || bytes > self.limits.max_payload_bytes
        {
            return Err(WitnessError::EventLimit);
        }
        // Exact existing CaptureLimits::local_peak arithmetic, without allocating errors.
        let peak = stream
            .checked_add(bytes)
            .and_then(|x| x.checked_mul(2))
            .and_then(|x| x.checked_add(self.limits.chunk_bytes))
            .and_then(|x| x.checked_add(self.limits.metadata_reserve_bytes.checked_mul(2)?))
            .ok_or(WitnessError::Storage)?;
        if peak > self.limits.max_capture_bytes {
            return Err(WitnessError::Storage);
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub(super) fn publish_first_witness(
        &mut self,
        bytes: &[u8],
        workspace: &mut WitnessCaptureWorkspace,
        budget: &mut WitnessBudget,
        global_acknowledged: &mut usize,
    ) -> Result<WitnessReceipt, WitnessError> {
        if self.failed || self.head.stop.is_some() {
            return Err(WitnessError::Failed);
        }
        let result =
            self.publish_first_witness_inner(bytes, workspace, budget, global_acknowledged);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn publish_first_witness_inner(
        &mut self,
        bytes: &[u8],
        w: &mut WitnessCaptureWorkspace,
        budget: &mut WitnessBudget,
        global: &mut usize,
    ) -> Result<WitnessReceipt, WitnessError> {
        {
            let mut t = budget.reserve(4096, 0)?;
            t.spend(4096)?;
        }
        if self.head.binding_sha256.as_bytes() != w.binding
            || w.paths[3].parent() != Some(self.directory.as_path())
        {
            return Err(WitnessError::Identity);
        }
        let digest = {
            let mut t = budget.reserve(witness_hash_work(bytes.len())?, 0)?;
            witness_hash(bytes, &mut t)?
        };
        let hex = witness_hex(&digest);
        let hex_str = std::str::from_utf8(&hex).map_err(|_| WitnessError::Encoding)?;
        let previous = self.payloads.get(WITNESS_PATH);
        if self.witness.committed || previous.is_some() {
            if self.witness.committed
                && (self.witness.digest != digest || self.witness.length != bytes.len() as u64)
            {
                return Err(WitnessError::Identity);
            }
            if let Some(p) = previous {
                if p.role != WITNESS_ROLE
                    || p.encoding != "json"
                    || p.sha256 != hex_str
                    || p.bytes != bytes.len() as u64
                {
                    return Err(WitnessError::Identity);
                }
            }
            let cost = (bytes.len() as u64)
                .checked_mul(32)
                .and_then(|n| n.checked_add(4096))
                .ok_or(WitnessError::Work)?;
            let mut t = budget.reserve(cost, 0)?;
            t.spend(4096)?;
            witness_file_matches(&w.paths[1], bytes, &mut w.io, &mut t)?;
            self.witness = WitnessIdentity {
                committed: true,
                digest,
                length: bytes.len() as u64,
                sequence: self.witness.sequence,
            };
            return Ok(self.witness.receipt(WitnessCommit::AlreadyCommitted));
        }
        if *global >= 4096 {
            return Err(WitnessError::EventLimit);
        }
        let event_count = self
            .head
            .event_count
            .checked_add(1)
            .ok_or(WitnessError::EventLimit)?;
        let payload_count = self
            .head
            .payload_count
            .checked_add(1)
            .ok_or(WitnessError::EventLimit)?;
        let payload_bytes = self
            .head
            .payload_bytes
            .checked_add(bytes.len() as u64)
            .ok_or(WitnessError::Storage)?;
        let event = WitnessEventRecord {
            schema: EVENT_SCHEMA,
            binding_sha256: &self.head.binding_sha256,
            sequence: event_count,
            previous_event_sha256: &self.head.last_event_sha256,
            event: WitnessPayloadEvent {
                artifact: WitnessArtifact {
                    bytes: bytes.len() as u64,
                    encoding: "json",
                    path: WITNESS_PATH,
                    role: WITNESS_ROLE,
                    sha256: hex_str,
                },
                kind: PAYLOAD_KIND,
            },
        };
        let event_cap = 1024.min(self.limits.max_event_bytes as usize);
        let event_len = {
            let mut t = budget.reserve(40 * 1024, 0)?;
            witness_encode(&event, None, event_cap, &mut t)?
        };
        self.witness_limits(event_len, payload_count, payload_bytes)?;
        let byte_end = self
            .head
            .byte_end
            .checked_add(event_len as u64)
            .ok_or(WitnessError::EventLimit)?;
        let placeholder = "0000000000000000000000000000000000000000000000000000000000000000";
        let head = WitnessHead {
            schema: &self.head.schema,
            binding_sha256: &self.head.binding_sha256,
            event_count,
            byte_end,
            last_event_sha256: placeholder,
            payload_count,
            payload_bytes,
            stop: None,
        };
        let head_cap = 1024.min(self.limits.metadata_reserve_bytes as usize);
        let head_len = {
            let mut t = budget.reserve(40 * 1024, 0)?;
            witness_encode(&head, None, head_cap, &mut t)?
        };
        let mutation = (event_len as u64 + head_len as u64)
            .checked_mul(40)
            .and_then(|x| x.checked_add(witness_hash_work(event_len).ok()?))
            .and_then(|x| x.checked_add(witness_hash_work(bytes.len()).ok()?))
            .and_then(|x| {
                x.checked_add(
                    (bytes.len() as u64)
                        .checked_mul(2)?
                        .checked_add(event_len as u64)?
                        .checked_add(head_len as u64)?
                        .checked_mul(16)?,
                )
            })
            .and_then(|x| x.checked_add(4096 + 81920 + 4096))
            .ok_or(WitnessError::Work)?;
        let mut ticket = budget.reserve(mutation, 0)?;
        ticket.spend(4096)?;
        w.event.clear();
        w.head.clear();
        if witness_encode(&event, Some(&mut w.event), event_cap, &mut ticket)? != event_len {
            return Err(WitnessError::Encoding);
        }
        let event_digest = witness_hash(&w.event, &mut ticket)?;
        let event_hex = witness_hex(&event_digest);
        let event_hex_str = std::str::from_utf8(&event_hex).map_err(|_| WitnessError::Encoding)?;
        let head = WitnessHead {
            last_event_sha256: event_hex_str,
            ..head
        };
        if witness_encode(&head, Some(&mut w.head), head_cap, &mut ticket)? != head_len {
            return Err(WitnessError::Encoding);
        }
        // All reservations/encodings completed before any payload/event/head mutation.
        let mut payload_temp_owned = false;
        let mut head_temp_owned = false;
        let operation = (|| {
            ticket.check()?;
            match fs::symlink_metadata(&w.paths[1]) {
                Ok(_) => witness_file_check(
                    &w.paths[1],
                    &digest,
                    bytes.len() as u64,
                    &mut w.io,
                    &mut ticket,
                )?,
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    let mut temp = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&w.paths[2])?;
                    payload_temp_owned = true;
                    witness_write(&mut temp, bytes, &mut ticket)?;
                    ticket.check()?;
                    temp.sync_all()?;
                    ticket.check()?;
                    match fs::hard_link(&w.paths[2], &w.paths[1]) {
                        Ok(()) => {}
                        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => witness_file_check(
                            &w.paths[1],
                            &digest,
                            bytes.len() as u64,
                            &mut w.io,
                            &mut ticket,
                        )?,
                        Err(e) => return Err(e.into()),
                    }
                    File::open(&w.paths[0])?.sync_all()?;
                }
                Err(e) => return Err(e.into()),
            }
            #[cfg(test)]
            if w.fault == 1 {
                return Err(WitnessError::Io(io::ErrorKind::Other));
            }
            ticket.check()?;
            witness_write(&mut self.events, &w.event, &mut ticket)?;
            self.events.sync_data()?;
            #[cfg(test)]
            if w.fault == 2 {
                return Err(WitnessError::Io(io::ErrorKind::Other));
            }
            ticket.check()?;
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&w.paths[4])?;
            head_temp_owned = true;
            witness_write(&mut file, &w.head, &mut ticket)?;
            file.sync_all()?;
            ticket.check()?;
            fs::rename(&w.paths[4], &w.paths[3])?;
            head_temp_owned = false;
            // Do not clock-check after possible head installation. A sync error
            // is indeterminate I/O, never a clean deadline rollback.
            File::open(&self.directory)
                .and_then(|directory| directory.sync_all())
                .map_err(|error| WitnessError::IndeterminateIo(error.kind()))?;
            #[cfg(test)]
            if w.fault == 4 {
                return Err(WitnessError::IndeterminateIo(io::ErrorKind::Other));
            }
            #[cfg(test)]
            if w.fault == 3 {
                ticket.budget.deadline = std::time::Instant::now();
            }
            self.head.event_count = event_count;
            self.head.byte_end = byte_end;
            self.head.payload_count = payload_count;
            self.head.payload_bytes = payload_bytes;
            self.head.last_event_sha256.clear();
            self.head.last_event_sha256.push_str(event_hex_str);
            self.witness = WitnessIdentity {
                committed: true,
                digest,
                length: bytes.len() as u64,
                sequence: Some(event_count),
            };
            *global += 1;
            Ok(self.witness.receipt(WitnessCommit::NewCommit))
        })();
        if payload_temp_owned {
            let _ = fs::remove_file(&w.paths[2]);
        }
        if head_temp_owned {
            let _ = fs::remove_file(&w.paths[4]);
        }
        operation
    }
}

pub struct CaptureEmitter {
    directory: PathBuf,
    events: File,
    head: CommittedHead,
    limits: CaptureLimits,
    payloads: BTreeMap<String, Artifact>,
    witness: WitnessIdentity,
    failed: bool,
}

impl CaptureEmitter {
    /// Read-only runtime admission. The caller must separately authenticate
    /// execution parameters against the original run; this binds the live sink.
    pub(super) fn verify_attempt_limits(
        &self,
        binding: &AttemptBinding,
        limits: &CaptureLimits,
    ) -> Result<(), String> {
        binding.validate()?;
        self.verify_digest_limits(binding.digest()?.as_bytes(), limits)
    }

    /// Compare a previously admitted original binding digest without encoding
    /// and hashing the unchanged binding again for every bounded event.
    pub(super) fn verify_digest_limits(
        &self,
        binding_digest: &[u8],
        limits: &CaptureLimits,
    ) -> Result<(), String> {
        limits.validate()?;
        if self.failed || self.head.stop.is_some() {
            return Err("capture sink already refused further work".into());
        }
        if binding_digest.len() != 64
            || self.head.binding_sha256.as_bytes() != binding_digest
            || &self.limits != limits
        {
            return Err("capture sink differs from original execution attempt or limits".into());
        }
        Ok(())
    }

    /// Bind an adapter's shared counter to the actual committed stream before
    /// it records an anticipated capture sequence.
    pub(super) fn verify_digest_limits_at_count(
        &self,
        binding_digest: &[u8],
        limits: &CaptureLimits,
        event_count: u64,
    ) -> Result<(), String> {
        self.verify_digest_limits(binding_digest, limits)?;
        if self.head.event_count != event_count {
            return Err("capture counter differs from original adapter counter".into());
        }
        Ok(())
    }

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
            witness: WitnessIdentity::default(),
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

    /// Capture owns the top-level kind: caller serialization is nested, never flattened.
    /// Both passes are bounded; custom Serialize implementations remain trusted code.
    pub fn event_typed<T: Serialize + ?Sized>(
        &mut self,
        kind: TypedEventKind,
        payload: &T,
        buffer: &mut TypedEventBuffer,
    ) -> Result<(), CaptureError> {
        self.event_typed_precommit(kind, payload, buffer, &mut |_| Ok(()))
    }

    /// Inspect/reuse the complete internal encoding before durable publication.
    /// Callback refusal leaves the head unchanged and does not poison the sink:
    /// no capture I/O has happened. Encoded bytes alone are not an acknowledgement.
    pub(super) fn event_typed_precommit<T: Serialize + ?Sized>(
        &mut self,
        kind: TypedEventKind,
        payload: &T,
        buffer: &mut TypedEventBuffer,
        before_commit: &mut impl FnMut(&[u8]) -> Result<(), CaptureError>,
    ) -> Result<(), CaptureError> {
        self.ready()?;
        buffer.bytes.clear();
        let result = (|| {
            let limit = buffer.limit.min(self.limits.max_event_bytes as usize);
            let record = TypedEventRecord {
                schema: EVENT_SCHEMA,
                binding_sha256: &self.head.binding_sha256,
                sequence: self
                    .head
                    .event_count
                    .checked_add(1)
                    .ok_or_else(|| CaptureError::Integrity("event sequence overflow".into()))?,
                previous_event_sha256: &self.head.last_event_sha256,
                event: TypedSolverEvent { kind, payload },
            };
            let mut counted = CountedEventBytes {
                len: 0,
                limit: limit - 1,
            };
            serde_json::to_writer(&mut counted, &record).map_err(typed_encoding_error)?;
            let expected = counted.len + 1;
            self.check_reservation(expected, self.head.payload_count, self.head.payload_bytes)?;
            let mut writer = AdmittedEventBytes {
                bytes: &mut buffer.bytes,
                limit: limit - 1,
            };
            serde_json::to_writer(&mut writer, &record).map_err(typed_encoding_error)?;
            if writer.bytes.len() != counted.len {
                return Err(CaptureError::Integrity(
                    "typed event changed length between passes".into(),
                ));
            }
            // Capacity and logical cap include the newline admitted above.
            writer.bytes.push(b'\n');
            Ok(())
        })();
        if let Err(error) = result {
            return Err(self.reject(error));
        }
        before_commit(&buffer.bytes)?;
        self.commit_event(
            &buffer.bytes,
            self.head.payload_count,
            self.head.payload_bytes,
        )
    }

    pub fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), CaptureError> {
        self.payload_parts(
            &payload.path,
            &payload.role,
            &payload.encoding,
            &payload.bytes,
        )
    }

    /// Outer custody boundary for complete retained bytes. No data-sized clone.
    /// Witness publication must use its persistent dedicated budget and receipt.
    #[allow(dead_code)] // Parent borrowed custody bridge is integrated separately.
    pub(crate) fn payload_borrowed(
        &mut self,
        path: &str,
        role: &str,
        encoding: &str,
        bytes: &[u8],
    ) -> Result<(), CaptureError> {
        self.ready()?;
        if path == "pdp-witness.json" || role == WITNESS_ROLE {
            return Err(self.reject(CaptureError::Integrity(
                "borrowed witness requires dedicated publication".into(),
            )));
        }
        self.payload_parts(path, role, encoding, bytes)
    }

    fn payload_parts(
        &mut self,
        path: &str,
        role: &str,
        encoding: &str,
        payload_bytes: &[u8],
    ) -> Result<(), CaptureError> {
        self.ready()?;
        if path.is_empty()
            || path.len() > 255
            || path.contains(['/', '\\'])
            || [".", ".."].contains(&path)
            || role.trim().is_empty()
            || encoding.trim().is_empty()
            || role.len() > 256
            || encoding.len() > 256
        {
            return Err(self.reject(CaptureError::Integrity(
                "invalid capture payload identity".into(),
            )));
        }
        let artifact = Artifact {
            path: format!("payloads/{path}"),
            role: role.to_owned(),
            encoding: encoding.to_owned(),
            sha256: super::digest(payload_bytes),
            bytes: payload_bytes.len() as u64,
        };
        if artifact.path == WITNESS_PATH && self.witness.committed {
            let expected = witness_hex(&self.witness.digest);
            if artifact.role != WITNESS_ROLE
                || artifact.encoding != "json"
                || artifact.bytes != self.witness.length
                || artifact.sha256.as_bytes() != expected
            {
                return Err(self.reject(CaptureError::Integrity(
                    "first witness identity collision".into(),
                )));
            }
            // Legacy caller remains an allocating outer boundary, but cannot
            // duplicate or replace the dedicated committed identity.
            let path = match storage::contained_path(&self.directory, WITNESS_PATH, false) {
                Ok(path) => path,
                Err(error) => return Err(self.reject(CaptureError::Integrity(error))),
            };
            return match storage::check_file(&path, &artifact.sha256, artifact.bytes) {
                Ok(()) => Ok(()),
                Err(error) => Err(self.reject(CaptureError::Integrity(error))),
            };
        }
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
        let next_payload_bytes = self
            .head
            .payload_bytes
            .checked_add(artifact.bytes)
            .ok_or_else(|| CaptureError::Integrity("payload byte overflow".into()))?;
        let bytes = match self.encode_event(&json!({"kind":PAYLOAD_KIND,"artifact":artifact})) {
            Ok(v) => v,
            Err(e) => return Err(self.reject(e)),
        };
        if let Err(e) = self.check_reservation(bytes.len(), payload_count, next_payload_bytes) {
            return Err(self.reject(e));
        }
        let written = storage::write_bytes(
            &self.directory,
            &artifact.path,
            &artifact.role,
            &artifact.encoding,
            payload_bytes,
        )
        .map_err(CaptureError::Io);
        if let Err(e) = written {
            return Err(self.reject(e));
        }
        self.commit_event(&bytes, payload_count, next_payload_bytes)?;
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

/// Read-only custody evidence for a sealed cell. The caller must bind `binding`
/// and `limits` to the original run before using these publication sequences.
#[allow(dead_code)] // Central dynamic admission is not yet activated.
pub(super) struct VerifiedLivePrefix {
    pub payloads: Vec<(u64, Artifact)>,
}

/// Validate exactly the committed prefix, ignoring an uncommitted trailing tail.
/// No snapshots, new receipts, or scientific claims are produced. Allocation and
/// hashing here belong to outer artifact custody, not the inner solver ledger.
#[allow(dead_code)]
pub(super) fn verify_live_committed_prefix(
    cell_dir: &Path,
    binding: &AttemptBinding,
    limits: &CaptureLimits,
) -> Result<Option<VerifiedLivePrefix>, String> {
    verify_live_committed_prefix_visit(cell_dir, binding, limits, &mut |_| Ok(()))
}

/// Visit checked records while authenticating the entire committed prefix.
/// Callback observations remain provisional until this function returns Some;
/// later chain, watermark or payload failures invalidate every observation.
pub(super) fn verify_live_committed_prefix_visit(
    cell_dir: &Path,
    binding: &AttemptBinding,
    limits: &CaptureLimits,
    visit: &mut impl FnMut(&[u8]) -> Result<(), String>,
) -> Result<Option<VerifiedLivePrefix>, String> {
    binding.validate()?;
    limits.validate()?;
    let cell = storage::root_dir(cell_dir, false)?;
    let live = cell.join("live");
    match fs::symlink_metadata(&live) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }
    let live = storage::root_dir(&live, false)?;
    let head_path = storage::contained_path(&live, "committed.json", false)?;
    if !head_path.exists() {
        return Ok(None);
    }
    let head: CommittedHead = read_json(&live, "committed.json", limits.metadata_reserve_bytes)?;
    let mut cursor = CaptureCursor::initial(binding)?;
    validate_head(&head, binding, &cursor)?;
    if head.byte_end > limits.max_stream_bytes
        || head.payload_bytes > limits.max_payload_bytes
        || head.payload_count > limits.max_payload_count
    {
        return Err("capture watermark exceeds frozen limits".into());
    }
    limits.local_peak(head.byte_end, head.payload_bytes)?;
    let file = File::open(storage::contained_path(&live, "events.jsonl", false)?)
        .map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() < head.byte_end {
        return Err("event stream shorter than committed watermark".into());
    }
    let mut reader = BufReader::new(file.take(head.byte_end));
    let binding_sha = binding.digest()?;
    let mut payloads = BTreeMap::<String, (u64, Artifact)>::new();
    loop {
        let mut line = Vec::new();
        let count = Read::by_ref(&mut reader)
            .take(limits.max_event_bytes + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        if cursor.event_count >= head.event_count || line.len() as u64 > limits.max_event_bytes {
            return Err("committed event exceeds configured record bounds".into());
        }
        let (hash, payload) = check_record(
            &line,
            &binding_sha,
            cursor.event_count + 1,
            &cursor.last_event_sha256,
        )?;
        visit(&line)?;
        cursor.event_count += 1;
        cursor.byte_end += line.len() as u64;
        cursor.last_event_sha256 = hash;
        if let Some(payload) = payload {
            cursor.payload_count += 1;
            cursor.payload_bytes = cursor
                .payload_bytes
                .checked_add(payload.bytes)
                .ok_or("payload accounting overflow")?;
            if cursor.payload_count > head.payload_count
                || cursor.payload_bytes > head.payload_bytes
            {
                return Err("payload records exceed committed watermark".into());
            }
            if payloads
                .insert(payload.path.clone(), (cursor.event_count, payload))
                .is_some()
            {
                return Err("duplicate payload commit in event prefix".into());
            }
        }
    }
    if cursor.byte_end != head.byte_end
        || cursor.event_count != head.event_count
        || cursor.last_event_sha256 != head.last_event_sha256
        || cursor.payload_count != head.payload_count
        || cursor.payload_bytes != head.payload_bytes
    {
        return Err("verified prefix does not match committed watermark".into());
    }
    for (_, payload) in payloads.values() {
        storage::verify_manifest(&live, std::slice::from_ref(payload))?;
    }
    Ok(Some(VerifiedLivePrefix {
        payloads: payloads.into_values().collect(),
    }))
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

    fn typed_buffer(cap: u64) -> TypedEventBuffer {
        TypedEventBuffer::try_new(cap, TypedEventBuffer::requested_bytes(cap).unwrap()).unwrap()
    }

    #[test]
    fn typed_capture_requested_storage_and_logical_caps() {
        assert!(TypedEventBuffer::requested_bytes(511).is_err());
        assert!(TypedEventBuffer::requested_bytes(MAX_EVENT + 1).is_err());
        assert!(TypedEventBuffer::requested_bytes(u64::MAX).is_err());
        assert!(TypedEventBuffer::try_new(512, 639).is_err());
        let mut buffer = TypedEventBuffer::try_new(512, 640).unwrap();
        assert_eq!(buffer.limit, 512);
        let mut writer = AdmittedEventBytes {
            bytes: &mut buffer.bytes,
            limit: 3,
        };
        writer.write_all(b"abc").unwrap();
        assert!(writer.write_all(b"d").is_err());
        assert_eq!(writer.bytes.as_slice(), b"abc");
    }

    #[test]
    fn typed_capture_matches_legacy_bytes_hash_chain_and_reuses_buffer() {
        #[derive(Serialize)]
        struct Payload {
            a: u64,
            z: bool,
        }
        let f = Fixture::new();
        let mut emitter = f.emitter();
        let mut buffer = typed_buffer(1024);
        let pointer = buffer.bytes.as_ptr();
        let capacity = buffer.bytes.capacity();
        let mut expected_stream = Vec::new();
        for a in [7, 11] {
            let expected = emitter
                .encode_event(&json!({
                    "kind":"wide_prime_dynamic", "payload":{"a":a,"z":true}
                }))
                .unwrap();
            emitter
                .event_typed(
                    TypedEventKind::WidePrimeDynamic,
                    &Payload { a, z: true },
                    &mut buffer,
                )
                .unwrap();
            assert_eq!(buffer.bytes, expected);
            assert_eq!(
                emitter.head.last_event_sha256,
                super::super::digest(&expected)
            );
            assert_eq!(buffer.bytes.as_ptr(), pointer);
            assert_eq!(buffer.bytes.capacity(), capacity);
            expected_stream.extend_from_slice(&expected);
        }
        assert_eq!(
            fs::read(f.live().join("events.jsonl")).unwrap(),
            expected_stream
        );
        assert_eq!(emitter.head.event_count, 2);
    }

    #[test]
    fn typed_capture_event_limit_includes_newline_and_rejection_is_sticky() {
        let probe = Fixture::new();
        let emitter = probe.emitter();
        let payload = "x".repeat(300);
        let length = emitter
            .encode_event(&json!({
                "kind":"wide_prime_dynamic", "payload":payload
            }))
            .unwrap()
            .len() as u64;
        assert!((512..=1024).contains(&length));
        for cap in [length, length - 1] {
            let f = Fixture::new();
            let mut limits = limits();
            limits.max_event_bytes = cap;
            let mut emitter = CaptureEmitter::create(&f.live(), binding(), limits).unwrap();
            let mut buffer = typed_buffer(1024);
            let result =
                emitter.event_typed(TypedEventKind::WidePrimeDynamic, &payload, &mut buffer);
            if cap == length {
                result.unwrap();
                assert_eq!(buffer.bytes.len() as u64, length);
            } else {
                assert!(matches!(
                    result,
                    Err(CaptureError::LimitReached(CaptureStop::EventLimit))
                ));
                assert_eq!(emitter.head.event_count, 0);
                assert!(emitter
                    .event_typed(TypedEventKind::WidePrimeDynamic, &0, &mut buffer)
                    .is_err());
                assert!(fs::read(f.live().join("events.jsonl")).unwrap().is_empty());
            }
        }
    }

    #[test]
    fn typed_capture_aggregate_exact_and_one_less_and_count_limit() {
        let probe = Fixture::new();
        let emitter = probe.emitter();
        let payload = "x".repeat(300);
        let length = emitter
            .encode_event(&json!({
                "kind":"wide_prime_dynamic", "payload":payload
            }))
            .unwrap()
            .len() as u64;
        for cap in [2 * length, 2 * length - 1] {
            let f = Fixture::new();
            let mut limits = limits();
            limits.max_stream_bytes = cap;
            let mut emitter = CaptureEmitter::create(&f.live(), binding(), limits).unwrap();
            let mut buffer = typed_buffer(1024);
            emitter
                .event_typed(TypedEventKind::WidePrimeDynamic, &payload, &mut buffer)
                .unwrap();
            let result =
                emitter.event_typed(TypedEventKind::WidePrimeDynamic, &payload, &mut buffer);
            if cap == 2 * length {
                result.unwrap();
                assert_eq!(emitter.head.byte_end, cap);
            } else {
                assert!(matches!(
                    result,
                    Err(CaptureError::LimitReached(CaptureStop::StreamByteLimit))
                ));
                assert_eq!(emitter.head.event_count, 1);
                assert!(buffer.bytes.is_empty()); // rejected after counting, before writing
            }
        }
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.head.event_count = MAX_ITEMS;
        assert!(matches!(
            emitter.event_typed(
                TypedEventKind::WidePrimeDynamic,
                &0,
                &mut typed_buffer(1024)
            ),
            Err(CaptureError::LimitReached(CaptureStop::EventLimit))
        ));
    }

    #[test]
    fn typed_capture_nested_reserved_kind_cannot_forge_payload_commit() {
        #[derive(Serialize)]
        struct FakePayload<'a> {
            kind: &'a str,
        }
        let f = Fixture::new();
        let mut emitter = f.emitter();
        let mut buffer = typed_buffer(1024);
        emitter
            .event_typed(
                TypedEventKind::WidePrimeDynamic,
                &FakePayload { kind: PAYLOAD_KIND },
                &mut buffer,
            )
            .unwrap();
        let record: Value = serde_json::from_slice(&buffer.bytes).unwrap();
        assert_eq!(record["event"]["kind"], "wide_prime_dynamic");
        assert_eq!(record["event"]["payload"]["kind"], PAYLOAD_KIND);
        assert_eq!(emitter.head.payload_count, 0);
        assert!(emitter.payloads.is_empty());
        assert!(emitter.event(&json!({"kind":PAYLOAD_KIND})).is_err());
    }

    #[test]
    fn typed_capture_changed_serializer_length_and_error_do_not_commit() {
        struct Changes(std::cell::Cell<bool>);
        impl Serialize for Changes {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(if self.0.replace(true) { "changed" } else { "x" })
            }
        }
        struct Fails;
        impl Serialize for Fails {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom(
                    "controlled serialization failure",
                ))
            }
        }
        let f = Fixture::new();
        let mut emitter = f.emitter();
        let mut buffer = typed_buffer(1024);
        assert!(matches!(
            emitter.event_typed(
                TypedEventKind::WidePrimeDynamic,
                &Changes(std::cell::Cell::new(false)),
                &mut buffer
            ),
            Err(CaptureError::Integrity(_))
        ));
        assert_eq!(emitter.head.event_count, 0);
        assert!(emitter
            .event_typed(TypedEventKind::WidePrimeDynamic, &0, &mut buffer)
            .is_err());
        let f = Fixture::new();
        let mut emitter = f.emitter();
        assert!(emitter
            .event_typed(TypedEventKind::WidePrimeDynamic, &Fails, &mut buffer)
            .is_err());
        assert!(emitter.failed);
        assert!(fs::read(f.live().join("events.jsonl")).unwrap().is_empty());
    }

    #[test]
    fn typed_capture_second_pass_still_enforces_cap_and_head_failure_is_sticky() {
        struct Grows(std::cell::Cell<bool>);
        impl Serialize for Grows {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let large = [b'x'; 1024];
                serializer.serialize_str(if self.0.replace(true) {
                    std::str::from_utf8(&large).unwrap()
                } else {
                    "x"
                })
            }
        }
        let f = Fixture::new();
        let mut emitter = f.emitter();
        let mut buffer = typed_buffer(512);
        assert!(matches!(
            emitter.event_typed(
                TypedEventKind::WidePrimeDynamic,
                &Grows(std::cell::Cell::new(false)),
                &mut buffer
            ),
            Err(CaptureError::LimitReached(CaptureStop::EventLimit))
        ));
        assert!(buffer.bytes.len() < 512);
        assert_eq!(emitter.head.event_count, 0);
        assert!(emitter.failed);

        let f = Fixture::new();
        let mut emitter = f.emitter();
        let old_head = emitter.head.last_event_sha256.clone();
        fs::remove_file(f.live().join("committed.json")).unwrap();
        fs::create_dir(f.live().join("committed.json")).unwrap();
        assert!(emitter
            .event_typed(TypedEventKind::WidePrimeDynamic, &0, &mut buffer)
            .is_err());
        assert!(!fs::read(f.live().join("events.jsonl")).unwrap().is_empty());
        assert_eq!(emitter.head.event_count, 0);
        assert_eq!(emitter.head.last_event_sha256, old_head);
        assert!(emitter.failed);
        assert!(emitter
            .event_typed(TypedEventKind::WidePrimeDynamic, &0, &mut buffer)
            .is_err());
    }

    #[test]
    fn typed_capture_actual_write_failure_is_sticky() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        emitter.events = File::open(f.live().join("events.jsonl")).unwrap();
        let mut buffer = typed_buffer(1024);
        assert!(matches!(
            emitter.event_typed(TypedEventKind::WidePrimeDynamic, &3, &mut buffer),
            Err(CaptureError::Io(_))
        ));
        assert_eq!(emitter.head.event_count, 0);
        assert!(emitter.failed);
        assert!(emitter
            .event_typed(TypedEventKind::WidePrimeDynamic, &4, &mut buffer)
            .is_err());
        assert!(emitter.finish().is_err());
    }

    #[test]
    fn capture_limits_preserve_worker_allowance_and_reject_invalid_sizes() {
        assert!(capture_limits(0).is_err());
        assert!(capture_limits(u64::MAX).is_err());
        for mib in [1, 8, 64, 128] {
            let cap = capture_limits(mib).unwrap();
            assert_eq!(cap.max_capture_bytes, mib * 1024 * 1024);
            assert_eq!(cap.max_stream_bytes, cap.max_capture_bytes / 8);
            assert_eq!(cap.max_payload_bytes, cap.max_capture_bytes / 8);
            assert_eq!(cap.max_event_bytes, cap.chunk_bytes);
            // The helper admits simultaneous maximum stream and payload
            // content, including the snapshot copy and reserved metadata.
            cap.local_peak(cap.max_stream_bytes, cap.max_payload_bytes)
                .unwrap();
        }
        assert!(capture_limits(32769).is_err());
    }

    #[test]
    fn cell_binding_distinguishes_typed_plan_identity_from_saved_file_hash() {
        let f = Fixture::new();
        let mut protocol = json!({"schema":"experiment.protocol/v1",
            "purpose":"capture binding software fixture"});
        let protocol_body = super::super::protocol_canonical_bytes(&protocol).unwrap();
        protocol["freeze"] = json!({"status":"frozen",
            "protocol_sha256":super::super::digest(&protocol_body),
            "filed_before_affected_data":true,"frozen_at":"2026-10-29T00:00:00Z"});
        let protocol_bytes = serde_json::to_vec(&protocol).unwrap();
        let plan = super::super::DatasetPlan {
            schema: "crypto.ml-dataset.plan/v1".into(),
            field_sizes: vec![23],
            arities: vec![2],
            families: vec!["prime".into()],
            solvers: vec!["root".into()],
            curve_seeds: 1,
            generators: 1,
            keys: 1,
            seed: 19,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 100,
            twists: true,
            protocol_sha256: Some(super::super::digest(&protocol_bytes)),
            split_rule: "curve-twist-component-sha256-v1".into(),
        };
        let typed_hash = plan.binding_sha256().unwrap();
        let config = plan.cell(0).unwrap();
        let run = json!({"schema":"crypto.ml-dataset.run/v1",
            "plan_sha256":typed_hash,"source_commit":"a".repeat(40),
            "binary_sha256":"b".repeat(64)});
        let attempt = json!({"schema":"crypto.ml-dataset.attempt/v1",
            "plan_sha256":typed_hash,"cell_index":0});
        storage::write_json(&f.cell(), "run.json", "run", &run).unwrap();
        storage::write_json(&f.cell(), "attempt.json", "attempt", &attempt).unwrap();
        storage::write_json(&f.cell(), "config.json", "config", &config).unwrap();
        storage::write_json(&f.cell(), "plan.json", "plan", &plan).unwrap();
        storage::write_bytes(
            &f.cell(),
            "protocol.json",
            "protocol",
            "json",
            &protocol_bytes,
        )
        .unwrap();

        let saved_bytes = fs::read(f.cell().join("plan.json")).unwrap();
        let saved_hash = super::super::digest(&saved_bytes);
        assert_ne!(saved_hash, typed_hash);
        let first = AttemptBinding::from_cell_files(&f.cell()).unwrap();
        assert_eq!(first.plan_sha256, saved_hash);

        // Equivalent formatting preserves the frozen typed identity but must
        // change the byte-bound capture context.
        fs::write(
            f.cell().join("plan.json"),
            serde_json::to_vec(&plan).unwrap(),
        )
        .unwrap();
        let reformatted = AttemptBinding::from_cell_files(&f.cell()).unwrap();
        assert_eq!(reformatted.plan_sha256, typed_hash);
        assert_ne!(first.digest().unwrap(), reformatted.digest().unwrap());

        let mut wrong_run = run.clone();
        wrong_run["plan_sha256"] = json!(saved_hash);
        fs::write(
            f.cell().join("run.json"),
            serde_json::to_vec(&wrong_run).unwrap(),
        )
        .unwrap();
        assert!(AttemptBinding::from_cell_files(&f.cell())
            .unwrap_err()
            .contains("attempt/run/plan mismatch"));
        fs::write(f.cell().join("run.json"), serde_json::to_vec(&run).unwrap()).unwrap();
        let mut wrong_config = config;
        wrong_config.seed += 1;
        fs::write(
            f.cell().join("config.json"),
            serde_json::to_vec(&wrong_config).unwrap(),
        )
        .unwrap();
        assert!(AttemptBinding::from_cell_files(&f.cell())
            .unwrap_err()
            .contains("config does not match frozen plan cell"));
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
    fn witness_budget() -> WitnessBudget {
        WitnessBudget::new(
            16_777_216,
            1024 * 1024,
            std::time::Instant::now() + std::time::Duration::from_secs(30),
        )
    }
    fn witness_payload() -> ArtifactPayload {
        ArtifactPayload::json(
            "pdp-witness.json",
            WITNESS_ROLE,
            &json!({"summands":[],"target":null}),
        )
        .unwrap()
    }
    #[test]
    fn witness_borrowed_wire_head_and_restored_payload_match_legacy() {
        let f = Fixture::new();
        let mut e = f.emitter();
        let p = witness_payload();
        let expected = e
            .encode_event(&json!({"kind":PAYLOAD_KIND,"artifact":{
                "path":WITNESS_PATH,"role":WITNESS_ROLE,"encoding":"json",
                "sha256":super::super::digest(&p.bytes),"bytes":p.bytes.len()
            }}))
            .unwrap();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        let pointers = (w.event.as_ptr(), w.head.as_ptr(), w.io.as_ptr());
        let mut global = 0;
        assert_eq!(
            e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit),
            Ok(WitnessCommit::NewCommit)
        );
        assert_eq!(w.event, expected);
        assert_eq!(fs::read(f.live().join("events.jsonl")).unwrap(), expected);
        assert_eq!(e.head.last_event_sha256, super::super::digest(&expected));
        let mut expected_head = serde_json::to_vec(&e.head).unwrap();
        expected_head.push(b'\n');
        assert_eq!(w.head, expected_head);
        assert_eq!(
            fs::read(f.live().join("committed.json")).unwrap(),
            expected_head
        );
        assert_eq!(pointers, (w.event.as_ptr(), w.head.as_ptr(), w.io.as_ptr()));
        assert!(!e.payloads.contains_key(WITNESS_PATH));
        assert_eq!(global, 1);
        let cursor = CaptureCursor::initial(&binding()).unwrap();
        let delta =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &cursor, &limits())
                .unwrap()
                .unwrap();
        verify_captured_delta(&f.snapshot(0), &delta, &cursor).unwrap();
        assert_eq!(fs::read(f.snapshot(0).join(WITNESS_PATH)).unwrap(), p.bytes);
        assert_eq!(delta.head.payload_count, 1);
    }
    #[test]
    fn witness_cross_api_idempotence_both_orders_no_duplicate_events() {
        for legacy_first in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let p = witness_payload();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            if legacy_first {
                e.payload(&p).unwrap();
            }
            let expected = if legacy_first {
                WitnessCommit::AlreadyCommitted
            } else {
                WitnessCommit::NewCommit
            };
            assert_eq!(
                e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                    .map(|receipt| receipt.commit),
                Ok(expected)
            );
            e.payload(&p).unwrap();
            assert_eq!(
                e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                    .map(|receipt| receipt.commit),
                Ok(WitnessCommit::AlreadyCommitted)
            );
            assert_eq!(e.head.payload_count, 1);
            assert_eq!(e.head.event_count, 1);
            assert_eq!(global, usize::from(!legacy_first));
        }
    }
    #[test]
    fn witness_changed_identity_and_file_are_sticky_in_both_apis() {
        for mode in 0..4 {
            let f = Fixture::new();
            let mut e = f.emitter();
            let p = witness_payload();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit)
                .unwrap();
            match mode {
                0 => assert!(e
                    .publish_first_witness(b"different", &mut w, &mut b, &mut global)
                    .map(|receipt| receipt.commit)
                    .is_err()),
                1 => {
                    let mut changed = p.clone();
                    changed.role = "different".into();
                    assert!(e.payload(&changed).is_err());
                }
                2 => {
                    fs::write(f.live().join(WITNESS_PATH), b"tampered").unwrap();
                    assert!(e
                        .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                        .map(|receipt| receipt.commit)
                        .is_err());
                }
                _ => {
                    fs::write(f.live().join(WITNESS_PATH), b"tampered").unwrap();
                    assert!(e.payload(&p).is_err());
                }
            }
            assert!(e.failed);
            assert_eq!(global, 1);
            assert_eq!(e.head.event_count, 1);
            assert_eq!(
                e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                    .map(|receipt| receipt.commit),
                Err(WitnessError::Failed)
            );
        }
    }
    #[test]
    fn witness_exact_requested_storage_and_publication_work_admission() {
        for less in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            b.bytes_limit = WitnessCaptureWorkspace::requested_bytes() - u64::from(less);
            assert_eq!(e.prepare_witness(&mut b).is_ok(), !less);
            assert_eq!(
                b.reserved_bytes,
                if less {
                    0
                } else {
                    WitnessCaptureWorkspace::requested_bytes()
                }
            );
        }
        let f = Fixture::new();
        let mut e = f.emitter();
        let p = witness_payload();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        let before = b.reserved_work;
        e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut 0)
            .map(|receipt| receipt.commit)
            .unwrap();
        let needed = b.reserved_work - before;
        for less in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            b.work_limit = b.reserved_work + needed - u64::from(less);
            let mut global = 0;
            let result = e
                .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit);
            if less {
                assert_eq!(result, Err(WitnessError::Work));
                assert!(!f.live().join(WITNESS_PATH).exists());
                assert_eq!(global, 0);
                assert_eq!(e.head.event_count, 0);
            } else {
                assert_eq!(result, Ok(WitnessCommit::NewCommit));
                assert_eq!(global, 1);
            }
            assert!(b.performed_work <= b.reserved_work && b.reserved_work <= b.work_limit);
        }
    }
    #[test]
    fn witness_exact_stream_payload_and_global_event_admission() {
        let p = witness_payload();
        for less in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let expected=e.encode_event(&json!({"kind":PAYLOAD_KIND,"artifact":{"path":WITNESS_PATH,"role":WITNESS_ROLE,"encoding":"json","sha256":super::super::digest(&p.bytes),"bytes":p.bytes.len()}})).unwrap();
            e.limits.max_stream_bytes = expected.len() as u64 - u64::from(less);
            let mut global = 4095;
            let result = e
                .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit);
            assert_eq!(result.is_ok(), !less);
            if !less {
                assert_eq!(global, 4096);
                assert_eq!(
                    e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                        .map(|receipt| receipt.commit),
                    Ok(WitnessCommit::AlreadyCommitted)
                );
                assert_eq!(global, 4096);
            } else {
                assert!(!f.live().join(WITNESS_PATH).exists());
            }
        }
        for failure in 0..4 {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            match failure {
                0 => global = 4096,
                1 => e.limits.max_payload_count = 0,
                2 => e.limits.max_payload_bytes = p.bytes.len() as u64 - 1,
                _ => e.limits.max_capture_bytes = 0,
            }
            assert!(e
                .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit)
                .is_err());
            assert!(!f.live().join(WITNESS_PATH).exists());
            assert_eq!(e.head.event_count, 0);
        }
    }
    #[test]
    fn readonly_live_prefix_binds_witness_sequence_and_rejects_corruption() {
        for mutation in 0..4 {
            let f = Fixture::new();
            let mut emitter = f.emitter();
            let mut budget = witness_budget();
            let mut workspace = emitter.prepare_witness(&mut budget).unwrap();
            let mut global = 0;
            let payload = witness_payload();
            let receipt = emitter
                .publish_first_witness(&payload.bytes, &mut workspace, &mut budget, &mut global)
                .unwrap();
            emitter.finish().unwrap();
            let before = fs::read(f.live().join("committed.json")).unwrap();
            let verified = verify_live_committed_prefix(&f.cell(), &binding(), &limits())
                .unwrap()
                .unwrap();
            assert_eq!(verified.payloads.len(), 1);
            assert_eq!(Some(verified.payloads[0].0), receipt.sequence);
            assert_eq!(verified.payloads[0].1.path, WITNESS_PATH);
            assert_eq!(
                verified.payloads[0].1.sha256,
                super::super::digest(&payload.bytes)
            );
            assert_eq!(fs::read(f.live().join("committed.json")).unwrap(), before);
            assert!(!f.snapshot(0).exists());
            let mut wrong_binding = binding();
            wrong_binding.cell_index += 1;
            assert!(verify_live_committed_prefix(&f.cell(), &wrong_binding, &limits()).is_err());
            match mutation {
                0 => {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open(f.live().join("events.jsonl"))
                        .unwrap();
                    file.write_all(b"uncommitted incomplete tail").unwrap();
                    assert!(verify_live_committed_prefix(&f.cell(), &binding(), &limits()).is_ok());
                    let mut bytes = payload.bytes.clone();
                    bytes[0] ^= 1;
                    fs::write(f.live().join(WITNESS_PATH), bytes).unwrap();
                }
                1 => {
                    fs::remove_file(f.live().join(WITNESS_PATH)).unwrap();
                }
                2 => {
                    let mut bytes = fs::read(f.live().join("events.jsonl")).unwrap();
                    bytes[0] ^= 1;
                    fs::write(f.live().join("events.jsonl"), bytes).unwrap();
                }
                _ => {
                    let mut head: CommittedHead = serde_json::from_slice(&before).unwrap();
                    head.payload_count += 1;
                    fs::write(
                        f.live().join("committed.json"),
                        serde_json::to_vec(&head).unwrap(),
                    )
                    .unwrap();
                }
            }
            assert!(verify_live_committed_prefix(&f.cell(), &binding(), &limits()).is_err());
        }
    }
    #[test]
    fn witness_confirmation_quote_matches_exact_admission_boundary() {
        for short in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            let payload = witness_payload();
            let first = e
                .publish_first_witness(&payload.bytes, &mut w, &mut b, &mut global)
                .unwrap();
            assert_eq!(first.commit, WitnessCommit::NewCommit);
            let quoted = witness_confirmation_work(payload.bytes.len()).unwrap();
            let before = b.reserved_work;
            b.set_exclusive_supply(quoted - u64::from(short), 0)
                .unwrap();
            let result = e.publish_first_witness(&payload.bytes, &mut w, &mut b, &mut global);
            b.release_exclusive_supply();
            assert_eq!(global, 1);
            assert!(b.reserved_work - before <= quoted);
            if short {
                assert_eq!(result.unwrap_err(), WitnessError::Work);
            } else {
                let receipt = result.unwrap();
                assert_eq!(receipt.commit, WitnessCommit::AlreadyCommitted);
                assert_eq!(receipt.payload_sha256, first.payload_sha256);
                assert_eq!(receipt.sequence, first.sequence);
                assert_eq!(b.reserved_work - before, quoted);
            }
        }
    }
    #[test]
    fn witness_confirmation_compares_full_bytes_across_io_boundaries() {
        for length in [624usize, 4102] {
            for mutation in [
                None,
                Some(0usize),
                Some(length - 1),
                Some(4096.min(length - 1)),
                Some(length),
                Some(length + 1),
            ] {
                let f = Fixture::new();
                let mut e = f.emitter();
                let mut b = witness_budget();
                let mut w = e.prepare_witness(&mut b).unwrap();
                let mut global = 0;
                // Valid JSON bytes exercise custody only, not witness mathematics.
                let bytes = format!("\"{}\"", "x".repeat(length - 2)).into_bytes();
                e.publish_first_witness(&bytes, &mut w, &mut b, &mut global)
                    .unwrap();
                let head = fs::read(f.live().join("committed.json")).unwrap();
                if let Some(index) = mutation {
                    let mut changed = bytes.clone();
                    if index < length {
                        changed[index] ^= 1;
                    } else if index == length {
                        changed.pop();
                    } else {
                        changed.push(b' ');
                    }
                    fs::write(f.live().join(WITNESS_PATH), changed).unwrap();
                }
                let quoted = witness_confirmation_work(bytes.len()).unwrap();
                b.set_exclusive_supply(quoted, 0).unwrap();
                let result = e.publish_first_witness(&bytes, &mut w, &mut b, &mut global);
                if mutation.is_some() {
                    assert_eq!(result.unwrap_err(), WitnessError::Integrity);
                    assert!(e.failed);
                } else {
                    assert_eq!(result.unwrap().commit, WitnessCommit::AlreadyCommitted);
                }
                assert_eq!(global, 1);
                assert_eq!(fs::read(f.live().join("committed.json")).unwrap(), head);
            }
        }
    }
    #[test]
    fn witness_failure_prefixes_are_unacknowledged_and_restore_excludes_orphans() {
        for fault in [1, 2] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            w.fault = fault;
            let old_head = fs::read(f.live().join("committed.json")).unwrap();
            let mut global = 0;
            assert!(e
                .publish_first_witness(&witness_payload().bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit)
                .is_err());
            assert!(e.failed);
            assert!(!e.witness.committed);
            assert_eq!(global, 0);
            assert_eq!(fs::read(f.live().join("committed.json")).unwrap(), old_head);
            assert!(f.live().join(WITNESS_PATH).exists());
            let cursor = CaptureCursor::initial(&binding()).unwrap();
            assert!(capture_committed_delta(
                &f.cell(),
                &f.snapshot(0),
                &binding(),
                &cursor,
                &limits()
            )
            .unwrap()
            .is_none());
        }
    }
    #[test]
    fn witness_deadline_before_mutation_and_after_commit_keep_acknowledgement_correct() {
        for after_commit in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            if after_commit {
                w.fault = 3;
            } else {
                b.deadline = std::time::Instant::now();
            }
            let result = e
                .publish_first_witness(&witness_payload().bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit);
            if after_commit {
                assert_eq!(result, Ok(WitnessCommit::NewCommit));
                assert_eq!(global, 1);
                assert!(e.witness.committed);
                assert!(!e.failed);
            } else {
                assert_eq!(result, Err(WitnessError::Deadline));
                assert_eq!(global, 0);
                assert!(!f.live().join(WITNESS_PATH).exists());
            }
        }
    }
    #[test]
    fn witness_foreign_workspace_and_preexisting_temp_are_rejected() {
        let f = Fixture::new();
        let other = Fixture::new();
        let mut e = f.emitter();
        let mut e2 = other.emitter();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        assert_eq!(
            e2.publish_first_witness(&witness_payload().bytes, &mut w, &mut b, &mut 0)
                .map(|receipt| receipt.commit),
            Err(WitnessError::Identity)
        );
        let f = Fixture::new();
        let mut e = f.emitter();
        fs::write(f.live().join(".witness-head.tmp"), b"foreign").unwrap();
        assert!(e.prepare_witness(&mut witness_budget()).is_err());
        assert_eq!(
            fs::read(f.live().join(".witness-head.tmp")).unwrap(),
            b"foreign"
        );
    }
    #[test]
    fn witness_exact_event_and_head_caps_and_indeterminate_head_failure() {
        let p = witness_payload();
        let f = Fixture::new();
        let mut e = f.emitter();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut 0)
            .map(|receipt| receipt.commit)
            .unwrap();
        let event_len = w.event.len();
        let head_len = w.head.len();
        for head in [false, true] {
            for less in [false, true] {
                let f = Fixture::new();
                let mut e = f.emitter();
                let mut b = witness_budget();
                let mut w = e.prepare_witness(&mut b).unwrap();
                if head {
                    e.limits.metadata_reserve_bytes = (head_len - usize::from(less)) as u64;
                } else {
                    e.limits.max_event_bytes = (event_len - usize::from(less)) as u64;
                }
                let result = e
                    .publish_first_witness(&p.bytes, &mut w, &mut b, &mut 0)
                    .map(|receipt| receipt.commit);
                assert_eq!(result.is_ok(), !less);
                if less {
                    assert!(!f.live().join(WITNESS_PATH).exists());
                    assert_eq!(e.head.event_count, 0);
                }
            }
        }
        let f = Fixture::new();
        let mut e = f.emitter();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        w.fault = 4;
        let mut global = 0;
        assert_eq!(
            e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .map(|receipt| receipt.commit),
            Err(WitnessError::IndeterminateIo(io::ErrorKind::Other))
        );
        assert_eq!(global, 0);
        assert!(e.failed);
        assert!(!e.witness.committed);
        // The possibly installed head is authoritative for recovery; a caller
        // error is not a promise that bytes were rolled back.
        let cursor = CaptureCursor::initial(&binding()).unwrap();
        let delta =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &cursor, &limits())
                .unwrap()
                .unwrap();
        verify_captured_delta(&f.snapshot(0), &delta, &cursor).unwrap();
        assert_eq!(delta.head.event_count, 1);
    }
    #[test]
    fn witness_receipt_preserves_original_identity_and_sequence_across_later_events() {
        for legacy_first in [false, true] {
            let f = Fixture::new();
            let mut e = f.emitter();
            let p = witness_payload();
            let mut b = witness_budget();
            let mut w = e.prepare_witness(&mut b).unwrap();
            let mut global = 0;
            if legacy_first {
                e.payload(&p).unwrap();
            }
            let first = e
                .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .unwrap();
            assert_eq!(
                first.commit,
                if legacy_first {
                    WitnessCommit::AlreadyCommitted
                } else {
                    WitnessCommit::NewCommit
                }
            );
            assert_eq!(first.payload_bytes, p.bytes.len() as u64);
            assert_eq!(
                std::str::from_utf8(&witness_hex(&first.payload_sha256)).unwrap(),
                super::super::digest(&p.bytes)
            );
            assert_eq!(first.sequence, if legacy_first { None } else { Some(1) });
            assert_eq!(
                first.provenance,
                if legacy_first {
                    WitnessProvenance::LegacyAdopted
                } else {
                    WitnessProvenance::Dedicated
                }
            );
            e.event(&json!({"kind":"unrelated_later_event"})).unwrap();
            assert_eq!(e.head.event_count, 2);
            let again = e
                .publish_first_witness(&p.bytes, &mut w, &mut b, &mut global)
                .unwrap();
            assert_eq!(
                again,
                WitnessReceipt {
                    commit: WitnessCommit::AlreadyCommitted,
                    ..first
                }
            );
            assert_eq!(e.head.event_count, 2);
            assert_eq!(global, usize::from(!legacy_first));
        }
    }
    #[test]
    fn witness_indeterminate_publication_returns_no_receipt_or_later_synthetic_ack() {
        let f = Fixture::new();
        let mut e = f.emitter();
        let p = witness_payload();
        let mut b = witness_budget();
        let mut w = e.prepare_witness(&mut b).unwrap();
        let mut global = 0;
        w.fault = 4;
        let receipt = e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global);
        assert_eq!(
            receipt,
            Err(WitnessError::IndeterminateIo(io::ErrorKind::Other))
        );
        assert!(!e.witness.committed);
        assert_eq!(global, 0);
        assert_eq!(
            e.publish_first_witness(&p.bytes, &mut w, &mut b, &mut global),
            Err(WitnessError::Failed)
        );
        let cursor = CaptureCursor::initial(&binding()).unwrap();
        let delta =
            capture_committed_delta(&f.cell(), &f.snapshot(0), &binding(), &cursor, &limits())
                .unwrap()
                .unwrap();
        verify_captured_delta(&f.snapshot(0), &delta, &cursor).unwrap();
        assert_eq!(delta.head.payload_count, 1);
        assert_eq!(global, 0);
    }
    #[test]
    fn witness_exclusive_supply_preserves_history_and_admits_only_parent_remainder() {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut b = WitnessBudget::new(0, 0, deadline);
        let (mut parent_work, mut parent_bytes) = (100, 80);
        b.set_exclusive_supply(parent_work, parent_bytes).unwrap();
        {
            let mut ticket = b.reserve(30, 20).unwrap();
            ticket.spend(7).unwrap();
        }
        b.release_exclusive_supply();
        parent_work -= b.reserved_work;
        parent_bytes -= b.reserved_bytes;
        assert_eq!(
            (b.reserved_work, b.performed_work, b.reserved_bytes),
            (30, 7, 20)
        );
        assert!(matches!(b.reserve(1, 0), Err(WitnessError::Work)));
        assert!(matches!(b.reserve(0, 1), Err(WitnessError::Storage)));
        b.set_exclusive_supply(parent_work, parent_bytes).unwrap();
        assert_eq!(b.deadline, deadline);
        assert!(matches!(
            b.reserve(parent_work + 1, 0),
            Err(WitnessError::Work)
        ));
        assert!(matches!(
            b.reserve(0, parent_bytes + 1),
            Err(WitnessError::Storage)
        ));
        let watermark = (b.reserved_work, b.performed_work, b.reserved_bytes);
        {
            let mut ticket = b.reserve(parent_work, parent_bytes).unwrap();
            ticket.spend(11).unwrap();
        }
        b.release_exclusive_supply();
        assert_eq!(
            (b.reserved_work, b.performed_work, b.reserved_bytes),
            (100, 18, 80)
        );
        assert_eq!(
            (
                b.reserved_work - watermark.0,
                b.performed_work - watermark.1,
                b.reserved_bytes - watermark.2
            ),
            (70, 11, 60)
        );
        b.set_exclusive_supply(0, 0).unwrap();
        assert!(matches!(b.reserve(1, 0), Err(WitnessError::Work)));
        assert_eq!(b.deadline, deadline);
    }
    #[test]
    fn witness_exclusive_supply_overflow_closes_both_limits_without_resetting_history() {
        for (work, bytes, expected) in [
            (u64::MAX, 0, WitnessError::Work),
            (0, u64::MAX, WitnessError::Storage),
        ] {
            let mut b = witness_budget();
            {
                let mut ticket = b.reserve(10, 20).unwrap();
                ticket.spend(3).unwrap();
            }
            let deadline = b.deadline;
            assert_eq!(b.set_exclusive_supply(work, bytes), Err(expected));
            assert_eq!((b.work_limit, b.bytes_limit), (10, 20));
            assert_eq!(
                (b.reserved_work, b.performed_work, b.reserved_bytes),
                (10, 3, 20)
            );
            assert_eq!(b.deadline, deadline);
            assert!(matches!(b.reserve(1, 0), Err(WitnessError::Work)));
            assert!(matches!(b.reserve(0, 1), Err(WitnessError::Storage)));
        }
    }
    #[test]
    fn witness_exclusive_supply_reconciliation_does_not_revive_expired_deadline() {
        let deadline = std::time::Instant::now();
        let mut b = WitnessBudget::new(0, 0, deadline);
        b.set_exclusive_supply(100, 100).unwrap();
        assert_eq!(b.deadline, deadline);
        assert!(matches!(b.reserve(1, 1), Err(WitnessError::Deadline)));
        b.release_exclusive_supply();
        assert_eq!(
            (
                b.work_limit,
                b.bytes_limit,
                b.reserved_work,
                b.performed_work,
                b.reserved_bytes
            ),
            (0, 0, 0, 0, 0)
        );
        b.set_exclusive_supply(100, 100).unwrap();
        assert_eq!(b.deadline, deadline);
        assert!(matches!(b.reserve(0, 0), Err(WitnessError::Deadline)));
    }
    #[test]
    fn borrowed_payload_matches_owned_custody_and_idempotence() {
        let f = Fixture::new();
        let g = Fixture::new();
        let mut owned = f.emitter();
        let mut borrowed = g.emitter();
        let payload =
            ArtifactPayload::json("basis.json", "exposed-basis", &json!({"rows":[1,2]})).unwrap();
        owned.payload(&payload).unwrap();
        borrowed
            .payload_borrowed(
                &payload.path,
                &payload.role,
                &payload.encoding,
                &payload.bytes,
            )
            .unwrap();
        for path in ["events.jsonl", "committed.json", "payloads/basis.json"] {
            assert_eq!(
                fs::read(f.live().join(path)).unwrap(),
                fs::read(g.live().join(path)).unwrap()
            );
        }
        let before = fs::read(g.live().join("events.jsonl")).unwrap();
        borrowed
            .payload_borrowed(
                &payload.path,
                &payload.role,
                &payload.encoding,
                &payload.bytes,
            )
            .unwrap();
        assert_eq!(fs::read(g.live().join("events.jsonl")).unwrap(), before);
        assert!(borrowed
            .payload_borrowed(&payload.path, &payload.role, &payload.encoding, b"changed")
            .is_err());
    }
    #[test]
    fn borrowed_payload_preserves_cap_and_refuses_witness_route() {
        let f = Fixture::new();
        let mut emitter = f.emitter();
        assert!(matches!(
            emitter.payload_borrowed("large.bin", "test", "bytes", &[1; 20 * 1024]),
            Err(CaptureError::LimitReached(CaptureStop::PayloadByteLimit))
        ));
        assert!(!f.live().join("payloads/large.bin").exists());
        let g = Fixture::new();
        let mut emitter = g.emitter();
        assert!(emitter
            .payload_borrowed(
                "pdp-witness.json",
                "independently-verified-pdp",
                "json",
                b"{}"
            )
            .is_err());
        assert!(!g.live().join("payloads/pdp-witness.json").exists());
        assert_eq!(emitter.head.payload_count, 0);
    }
}
