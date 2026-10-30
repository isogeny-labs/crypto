//! Immutable partial-artifact snapshots. These are never completed dataset cells
//! or resumable solver-memory checkpoints. Every acknowledgement follows fresh
//! downloads of the current objects, manifest, receipt and ordinal catalog.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::budget::BudgetSession;
use super::capture::{self, AttemptBinding, CaptureCursor, CaptureStop, CapturedDelta};
use super::storage;
use super::types::Artifact;
use crate::hash::sha256::Sha256;
use crate::store::backend::{object_key, Backend, Phase, RemoteSpec, SINGLE_PUT_MAX};
use crate::store::index::validate_sha256;
use crate::store::{Config, Store};

const MANIFEST_SCHEMA: &str = "ml-dataset.partial-snapshot/v1";
const CHECKED_MANIFEST_SCHEMA: &str = "ml-dataset.partial-snapshot/v2";
const RECEIPT_SCHEMA: &str = "ml-dataset.partial-receipt/v1";
const CATALOG_SCHEMA: &str = "ml-dataset.partial-catalog/v1";
const ANCHOR_SCHEMA: &str = "ml-dataset.partial-anchor/v1";
static TEMP_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotProfile {
    #[default]
    LegacyV1,
    CheckedPhaseV2,
}
impl SnapshotProfile {
    pub fn is_legacy(&self) -> bool {
        *self == Self::LegacyV1
    }
    fn schema(self) -> &'static str {
        if self.is_legacy() {
            MANIFEST_SCHEMA
        } else {
            CHECKED_MANIFEST_SCHEMA
        }
    }
    fn from_schema(schema: &str) -> Result<Self, String> {
        match schema {
            MANIFEST_SCHEMA => Ok(Self::LegacyV1),
            CHECKED_MANIFEST_SCHEMA => Ok(Self::CheckedPhaseV2),
            _ => Err("unknown partial snapshot schema".into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    Running,
    Exited,
    Interrupted,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttemptObservation {
    pub status: AttemptStatus,
    pub observed_unix_seconds: u64,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub reason: Option<String>,
}

impl AttemptObservation {
    fn validate(&self) -> Result<(), String> {
        if self.reason.as_ref().is_some_and(|s| s.len() > 4096)
            || self.signal.is_some_and(|s| !(1..=127).contains(&s))
            || (self.status == AttemptStatus::Running
                && (self.exit_code.is_some() || self.signal.is_some()))
        {
            return Err("invalid partial attempt observation".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SnapshotLimits {
    pub max_object_bytes: u64,
    pub max_metadata_bytes: u64,
    pub max_snapshot_artifacts: u64,
    pub max_total_objects: u64,
    pub max_total_bytes: u64,
    pub max_chain_length: u64,
}

impl Default for SnapshotLimits {
    fn default() -> Self {
        Self {
            max_object_bytes: 64 * 1024 * 1024,
            max_metadata_bytes: 4 * 1024 * 1024,
            max_snapshot_artifacts: 8192,
            max_total_objects: 65_536,
            max_total_bytes: 8 * 1024 * 1024 * 1024,
            max_chain_length: 256,
        }
    }
}

impl SnapshotLimits {
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=SINGLE_PUT_MAX).contains(&self.max_object_bytes)
            || !(1024..=16 * 1024 * 1024).contains(&self.max_metadata_bytes)
            || !(1..=100_000).contains(&self.max_snapshot_artifacts)
            || !(1..=1_000_000).contains(&self.max_total_objects)
            || !(1..=64 * 1024 * 1024 * 1024).contains(&self.max_total_bytes)
            || !(1..=1024).contains(&self.max_chain_length)
        {
            return Err("invalid partial snapshot limits".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObjectRef {
    pub key: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SnapshotAnchor {
    pub schema: String,
    pub remote: String,
    pub binding_sha256: String,
    pub run_sha256: String,
    pub attempt_sha256: String,
    pub ordinal: u64,
    pub receipt: ObjectRef,
    pub catalog: ObjectRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRequest {
    #[serde(default, skip_serializing_if = "SnapshotProfile::is_legacy")]
    pub profile: SnapshotProfile,
    pub binding: AttemptBinding,
    pub ordinal: u64,
    pub previous: Option<SnapshotAnchor>,
    pub context: Vec<Artifact>,
    pub delta: Option<CapturedDelta>,
    pub observation: AttemptObservation,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaptureCoverage {
    pub capture_present: bool,
    pub committed_events: u64,
    pub captured_event_end: u64,
    pub capture_stopped: bool,
    pub stop: Option<CaptureStop>,
    pub artifact_only: bool,
    pub in_flight_solver_memory_captured: bool,
    pub uncommitted_event_tail_captured: bool,
    pub unemitted_backend_internals_captured: bool,
    pub solver_resume_supported: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct SnapshotObject {
    artifact: Artifact,
    object: ObjectRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotManifest {
    schema: String,
    binding: AttemptBinding,
    ordinal: u64,
    previous: Option<SnapshotAnchor>,
    context: Vec<Artifact>,
    delta: Option<CapturedDelta>,
    observation: AttemptObservation,
    coverage: CaptureCoverage,
    objects: Vec<SnapshotObject>,
    cell_complete: bool,
    solver_state_restorable: bool,
    scientific_result_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct PartialReceipt {
    schema: String,
    remote: String,
    binding_sha256: String,
    ordinal: u64,
    previous: Option<SnapshotAnchor>,
    manifest: ObjectRef,
    artifact_count: u64,
    artifact_bytes: u64,
    cell_complete: bool,
    solver_state_restorable: bool,
    scientific_result_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct PartialCatalog {
    schema: String,
    remote: String,
    binding_sha256: String,
    run_sha256: String,
    attempt_sha256: String,
    ordinal: u64,
    receipt: ObjectRef,
    cell_complete: bool,
    solver_state_restorable: bool,
    scientific_result_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SnapshotAck {
    pub schema: String,
    pub anchor: SnapshotAnchor,
    pub cursor: Option<CaptureCursor>,
    pub manifest: Artifact,
    pub receipt: Artifact,
    pub catalog: Artifact,
    pub confirmation: Artifact,
    pub cell_complete: bool,
    pub solver_state_restorable: bool,
    pub scientific_result_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RestoreReport {
    pub schema: String,
    pub anchor: SnapshotAnchor,
    pub restored_directory: PathBuf,
    pub snapshot_count: u64,
    pub artifact_count: u64,
    pub artifact_bytes: u64,
    pub cursor: Option<CaptureCursor>,
    pub observation: AttemptObservation,
    pub coverage: CaptureCoverage,
    pub solver_resumed: bool,
    pub cell_complete: bool,
    pub solver_state_restorable: bool,
    pub scientific_result_verified: bool,
}

fn claims_false(complete: bool, restorable: bool, verified: bool) -> Result<(), String> {
    if complete || restorable || verified {
        Err(
            "partial snapshot cannot claim completion, solver recovery or scientific verification"
                .into(),
        )
    } else {
        Ok(())
    }
}

fn safe_relative(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path.len() > 1024
        || path.contains('\\')
        || path.bytes().any(|b| b.is_ascii_control())
        || path
            .split('/')
            .any(|s| s.is_empty() || s == "." || s == "..")
        || Path::new(path)
            .components()
            .any(|p| !matches!(p, Component::Normal(_)))
    {
        return Err("unsafe partial artifact or object path".into());
    }
    Ok(())
}

fn artifact_valid(artifact: &Artifact, max: u64) -> Result<(), String> {
    safe_relative(&artifact.path)?;
    validate_sha256(&artifact.sha256)?;
    if artifact.bytes > max
        || artifact.role.trim().is_empty()
        || artifact.role.len() > 256
        || artifact.encoding.trim().is_empty()
        || artifact.encoding.len() > 64
    {
        return Err("invalid or oversized partial artifact".into());
    }
    Ok(())
}

fn reference(artifact: &Artifact) -> Result<ObjectRef, String> {
    Ok(ObjectRef {
        key: object_key(&artifact.sha256)?,
        sha256: artifact.sha256.clone(),
        bytes: artifact.bytes,
    })
}

fn reference_valid(reference: &ObjectRef, max: u64, cas: bool) -> Result<(), String> {
    safe_relative(&reference.key)?;
    validate_sha256(&reference.sha256)?;
    if reference.bytes > max || (cas && reference.key != object_key(&reference.sha256)?) {
        return Err("invalid or oversized partial object reference".into());
    }
    Ok(())
}

fn catalog_key(run: &str, attempt: &str, ordinal: u64) -> String {
    format!("ml-dataset/interrupted-attempts/{run}/{attempt}/snapshots/{ordinal:020}.json")
}

fn anchor_valid(
    anchor: &SnapshotAnchor,
    remote: &str,
    limits: &SnapshotLimits,
) -> Result<(), String> {
    validate_sha256(&anchor.binding_sha256)?;
    validate_sha256(&anchor.run_sha256)?;
    validate_sha256(&anchor.attempt_sha256)?;
    reference_valid(&anchor.receipt, limits.max_metadata_bytes, true)?;
    reference_valid(&anchor.catalog, limits.max_metadata_bytes, false)?;
    if anchor.schema != ANCHOR_SCHEMA
        || anchor.remote != remote
        || anchor.ordinal >= limits.max_chain_length
        || anchor.catalog.key
            != catalog_key(&anchor.run_sha256, &anchor.attempt_sha256, anchor.ordinal)
    {
        return Err("partial anchor identity, remote or ordinal mismatch".into());
    }
    Ok(())
}

#[derive(Default)]
struct Accounting {
    objects: u64,
    bytes: u64,
}
impl Accounting {
    fn add(&mut self, bytes: u64, limits: &SnapshotLimits) -> Result<(), String> {
        self.objects = self
            .objects
            .checked_add(1)
            .ok_or("snapshot count overflow")?;
        self.bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or("snapshot byte overflow")?;
        if self.objects > limits.max_total_objects || self.bytes > limits.max_total_bytes {
            return Err("partial snapshot aggregate object/byte limit exhausted".into());
        }
        Ok(())
    }
}

fn open_regular(path: &Path) -> Result<File, String> {
    if !fs::symlink_metadata(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .is_file()
    {
        return Err("partial artifact input is not a regular file".into());
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("partial artifact input changed to nonregular file".into());
    }
    Ok(file)
}

fn read_bounded(path: &Path, max: u64) -> Result<Vec<u8>, String> {
    let file = open_regular(path)?;
    if file.metadata().map_err(|e| e.to_string())?.len() > max {
        return Err("partial metadata exceeds byte bound".into());
    }
    let mut bytes = Vec::new();
    file.take(max.checked_add(1).ok_or("read limit overflow")?)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > max {
        return Err("partial metadata grew beyond byte bound".into());
    }
    Ok(bytes)
}

fn check_regular(path: &Path, expected: &ObjectRef) -> Result<(), String> {
    let mut file = open_regular(path)?;
    if file.metadata().map_err(|e| e.to_string())?.len() != expected.bytes {
        return Err("partial artifact size mismatch".into());
    }
    let mut hash = Sha256::new();
    let mut count = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        count = count.checked_add(n as u64).ok_or("file size overflow")?;
        if count > expected.bytes {
            return Err("partial artifact grew during read".into());
        }
        hash.update(&buffer[..n]);
    }
    if count != expected.bytes || hex::encode(hash.finalize()) != expected.sha256 {
        return Err("partial artifact hash mismatch".into());
    }
    Ok(())
}

struct TempFile {
    path: PathBuf,
    file: File,
}
impl TempFile {
    fn new(root: &Path) -> Result<Self, String> {
        let path = root.join(format!(
            ".partial-{}-{}.tmp",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        Ok(Self { path, file })
    }
}
impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Stream to a create-if-absent file; neither the directory backend nor restore
/// is allowed to overwrite an earlier object, even on a conflicting retry.
fn install_copy(
    root: &Path,
    relative: &str,
    source: &Path,
    expected: &ObjectRef,
) -> Result<PathBuf, String> {
    let destination = storage::contained_path(root, relative, true)?;
    if destination.exists() {
        check_regular(&destination, expected)?;
        return Ok(destination);
    }
    let parent = destination.parent().ok_or("artifact has no parent")?;
    let mut temp = TempFile::new(parent)?;
    let mut input = open_regular(source)?;
    let mut count = 0_u64;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let n = input.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        count = count.checked_add(n as u64).ok_or("copy overflow")?;
        if count > expected.bytes {
            return Err("partial source grew while copying".into());
        }
        hash.update(&buffer[..n]);
        temp.file
            .write_all(&buffer[..n])
            .map_err(|e| e.to_string())?;
    }
    if count != expected.bytes || hex::encode(hash.finalize()) != expected.sha256 {
        return Err("partial source changed while copying".into());
    }
    temp.file.sync_all().map_err(|e| e.to_string())?;
    match fs::hard_link(&temp.path, &destination) {
        Ok(()) => storage::sync_directory(parent)?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            check_regular(&destination, expected)?;
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(destination)
}

fn download(backend: &dyn Backend, object: &ObjectRef, scratch: &Path) -> Result<TempFile, String> {
    let meta = backend.head(&object.key)?.ok_or("partial object missing")?;
    // Directory HEAD reports the final key component as its hash hint, which
    // is not a digest for fixed ordinal catalogs. Verify bytes on every GET.
    if meta.bytes != object.bytes {
        return Err("partial object remote length mismatch".into());
    }
    let temp = TempFile::new(scratch)?;
    backend.get(&object.key, &temp.path)?;
    check_regular(&temp.path, object)?;
    Ok(temp)
}

fn read_remote<T: DeserializeOwned>(
    backend: &dyn Backend,
    object: &ObjectRef,
    scratch: &Path,
    limits: &SnapshotLimits,
    accounting: &mut Accounting,
) -> Result<T, String> {
    reference_valid(object, limits.max_metadata_bytes, false)?;
    accounting.add(object.bytes, limits)?;
    let temp = download(backend, object, scratch)?;
    let bytes = read_bounded(&temp.path, limits.max_metadata_bytes)?;
    // Hash and parse the identical in-memory bytes, avoiding a pathname
    // replacement between the integrity check and the structured parse.
    if bytes.len() as u64 != object.bytes || super::digest(&bytes) != object.sha256 {
        return Err("partial metadata changed before parse".into());
    }
    serde_json::from_slice(&bytes).map_err(|e| format!("invalid partial metadata: {e}"))
}

fn publish_object(
    backend: &dyn Backend,
    remote: &RemoteSpec,
    root: &Path,
    artifact: &Artifact,
    object: &ObjectRef,
) -> Result<(), String> {
    let source = storage::contained_path(root, &artifact.path, false)?;
    check_regular(&source, object)?;
    match remote {
        RemoteSpec::Dir(directory) => {
            let directory = storage::root_dir(directory, true)?;
            install_copy(&directory, &object.key, &source, object)?;
        }
        RemoteSpec::S3 { .. } => {
            storage::prepare_s3_publication(backend, remote, &object.key, &source, artifact, root)?
        }
    }
    // Deliberately do this on retries too. A successful PUT/HEAD is insufficient.
    let _readback = download(backend, object, root)?;
    Ok(())
}

fn write_metadata<T: Serialize>(
    root: &Path,
    path: &str,
    role: &str,
    value: &T,
    limits: &SnapshotLimits,
) -> Result<Artifact, String> {
    let value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    let mut bytes = serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    if bytes.len() as u64 > limits.max_metadata_bytes {
        return Err("partial publication metadata exceeds limit".into());
    }
    storage::write_bytes(root, path, role, "json", &bytes)
}

fn expected_context<'a>(binding: &'a AttemptBinding, path: &str) -> Option<&'a str> {
    match path {
        "context/run.json" => Some(&binding.run_sha256),
        "context/plan.json" => Some(&binding.plan_sha256),
        "context/protocol.json" => Some(&binding.protocol_sha256),
        "context/config.json" => Some(&binding.config_sha256),
        "context/attempt.json" => Some(&binding.attempt_sha256),
        _ => None,
    }
}

#[derive(Default)]
struct ChainState {
    profile: Option<SnapshotProfile>,
    context: BTreeSet<String>,
    paths: BTreeMap<String, Artifact>,
    cursor: Option<CaptureCursor>,
    stop: Option<CaptureStop>,
    binding: Option<AttemptBinding>,
    last_observation: Option<AttemptObservation>,
}

fn coverage(state: &ChainState) -> CaptureCoverage {
    CaptureCoverage {
        capture_present: state.cursor.is_some(),
        committed_events: state.cursor.as_ref().map_or(0, |c| c.event_count),
        captured_event_end: state.cursor.as_ref().map_or(0, |c| c.byte_end),
        capture_stopped: state.stop.is_some(),
        stop: state.stop.clone(),
        artifact_only: true,
        in_flight_solver_memory_captured: false,
        uncommitted_event_tail_captured: false,
        unemitted_backend_internals_captured: false,
        solver_resume_supported: false,
    }
}

fn request_objects(request: &SnapshotRequest) -> Result<Vec<SnapshotObject>, String> {
    request
        .context
        .iter()
        .chain(request.delta.iter().flat_map(|d| &d.artifacts))
        .map(|a| {
            Ok(SnapshotObject {
                artifact: a.clone(),
                object: reference(a)?,
            })
        })
        .collect()
}

/// Verify chain semantics before touching referenced data. Full event-byte
/// replay belongs to verify_captured_delta and is mandatory for local publish
/// and artifact restore. Reading historical metadata alone never claims it.
fn advance(
    manifest: &SnapshotManifest,
    state: &mut ChainState,
    limits: &SnapshotLimits,
    accounting: &mut Accounting,
) -> Result<(), String> {
    let profile = SnapshotProfile::from_schema(&manifest.schema)?;
    manifest.binding.validate()?;
    manifest.observation.validate()?;
    claims_false(
        manifest.cell_complete,
        manifest.solver_state_restorable,
        manifest.scientific_result_verified,
    )?;
    if state.profile.is_some_and(|previous| previous != profile)
        || manifest.ordinal >= limits.max_chain_length
        || manifest.objects.len() as u64 > limits.max_snapshot_artifacts
        || state
            .binding
            .as_ref()
            .is_some_and(|b| b != &manifest.binding)
    {
        return Err("invalid partial manifest identity, count or binding".into());
    }
    if let Some(old) = &state.last_observation {
        if manifest.observation.observed_unix_seconds < old.observed_unix_seconds
            || (old.status != AttemptStatus::Running
                && manifest.observation.status == AttemptStatus::Running)
        {
            return Err("partial attempt observation regressed".into());
        }
    }
    let request = SnapshotRequest {
        profile,
        binding: manifest.binding.clone(),
        ordinal: manifest.ordinal,
        previous: manifest.previous.clone(),
        context: manifest.context.clone(),
        delta: manifest.delta.clone(),
        observation: manifest.observation.clone(),
    };
    if manifest.objects != request_objects(&request)? {
        return Err("partial manifest object membership mismatch".into());
    }
    let mut paths = BTreeSet::new();
    for object in &manifest.objects {
        artifact_valid(&object.artifact, limits.max_object_bytes)?;
        reference_valid(&object.object, limits.max_object_bytes, true)?;
        if !paths.insert(&object.artifact.path) {
            return Err("duplicate partial artifact path".into());
        }
        if state.paths.contains_key(&object.artifact.path) {
            return Err("partial chain redefines an earlier artifact path".into());
        }
        accounting.add(object.artifact.bytes, limits)?;
        state
            .paths
            .insert(object.artifact.path.clone(), object.artifact.clone());
    }
    for context in &manifest.context {
        if context.encoding != "json" || !state.context.insert(context.path.clone()) {
            return Err("invalid or redefined partial context".into());
        }
        if profile == SnapshotProfile::CheckedPhaseV2 {
            let local = context
                .path
                .strip_prefix("context/")
                .ok_or("invalid checked context prefix")?;
            let expected =
                super::live::context_role(local).ok_or("unknown checked context path")?;
            if context.role != expected
                || context.bytes > super::telemetry::checked::byte_limit(local)
            {
                return Err("invalid checked context role or bound".into());
            }
        }
        if let Some(expected) = expected_context(&manifest.binding, &context.path) {
            if manifest.ordinal != 0 || context.sha256 != expected {
                return Err("partial immutable context digest mismatch".into());
            }
        } else if !(matches!(
            context.path.as_str(),
            "context/public.json"
                | "context/witness.json"
                | "context/factor-base.json"
                | "context/host.json"
                | "context/worker-process.json"
        ) || (profile == SnapshotProfile::CheckedPhaseV2
            && context
                .path
                .strip_prefix("context/")
                .and_then(super::telemetry::checked::role)
                .is_some()))
        {
            return Err("unrecognized partial context file".into());
        }
    }
    if profile == SnapshotProfile::CheckedPhaseV2 {
        if !state.context.contains("context/measurement-policy.json") {
            return Err("checked snapshot lacks policy".into());
        }
        let mut previous_complete = true;
        for phase in super::telemetry::checked::types::Phase::ALL {
            let start = state
                .context
                .contains(&format!("context/{}", phase.path(true)));
            let end = state
                .context
                .contains(&format!("context/{}", phase.path(false)));
            if (end && !start) || (start && !previous_complete) {
                return Err("checked snapshot is not dependency-closed".into());
            }
            previous_complete = end;
        }
        let context_bytes = state.context.iter().try_fold(0u64, |sum, path| {
            sum.checked_add(state.paths[path].bytes)
                .ok_or("checked context bytes overflow")
        })?;
        if state.context.len() > 19 || context_bytes > super::telemetry::checked::MAX_CONTEXT {
            return Err("checked context envelope exceeded".into());
        }
    }
    if manifest.ordinal == 0 {
        for name in ["run", "plan", "protocol", "config", "attempt"] {
            if !state.context.contains(&format!("context/{name}.json")) {
                return Err("initial partial snapshot lacks mandatory context".into());
            }
        }
    }
    if let Some(delta) = &manifest.delta {
        if delta.binding != manifest.binding {
            return Err("partial capture binding mismatch".into());
        }
        let previous = state
            .cursor
            .clone()
            .unwrap_or(CaptureCursor::initial(&manifest.binding)?);
        if delta.cursor.event_count < previous.event_count
            || delta.cursor.byte_end < previous.byte_end
            || delta.cursor.next_chunk < previous.next_chunk
            || delta.cursor.payload_count < previous.payload_count
            || delta.cursor.payload_bytes < previous.payload_bytes
            || delta.head.binding_sha256 != manifest.binding.digest()?
            || delta.head.byte_end != delta.cursor.byte_end
            || delta.head.event_count != delta.cursor.event_count
            || delta.head.last_event_sha256 != delta.cursor.last_event_sha256
            || delta.head.payload_count != delta.cursor.payload_count
            || delta.head.payload_bytes != delta.cursor.payload_bytes
            || (state.stop.is_some() && (delta.head.stop != state.stop || delta.cursor != previous))
        {
            return Err("partial capture head/cursor regressed or mismatched".into());
        }
        for a in &delta.artifacts {
            if !(a.path.starts_with("events/") || a.path.starts_with("payloads/")) {
                return Err("capture artifact outside events/payloads namespace".into());
            }
        }
        state.cursor = Some(delta.cursor.clone());
        state.stop = delta.head.stop.clone();
    }
    state.binding = Some(manifest.binding.clone());
    state.profile = Some(profile);
    state.last_observation = Some(manifest.observation.clone());
    if manifest.coverage != coverage(state) {
        return Err("partial coverage disagrees with committed capture".into());
    }
    Ok(())
}

struct LoadedSnapshot {
    anchor: SnapshotAnchor,
    receipt: PartialReceipt,
    manifest: SnapshotManifest,
}

fn validate_checked_local_context(root: &Path, state: &ChainState) -> Result<(), String> {
    let mut files = BTreeMap::new();
    for path in &state.context {
        let artifact = &state.paths[path];
        let local = path
            .strip_prefix("context/")
            .ok_or("checked context prefix")?;
        let bytes = super::telemetry::checked::read(
            root,
            path,
            super::telemetry::checked::byte_limit(local),
        )?;
        if bytes.len() as u64 != artifact.bytes || super::digest(&bytes) != artifact.sha256 {
            return Err("checked restored or prepared context identity mismatch".into());
        }
        files.insert(local.to_owned(), bytes);
    }
    let binding = AttemptBinding::from_cell_files(&root.join("context"))?;
    if state.binding.as_ref() != Some(&binding) {
        return Err("checked restored authoritative binding mismatch".into());
    }
    super::telemetry::checked::validate_context(&binding, &files)?;
    Ok(())
}

fn load_chain(
    backend: &dyn Backend,
    remote: &str,
    anchor: &SnapshotAnchor,
    scratch: &Path,
    limits: &SnapshotLimits,
    accounting: &mut Accounting,
) -> Result<Vec<LoadedSnapshot>, String> {
    let mut pending = Some(anchor.clone());
    let mut chain = Vec::new();
    let mut seen = BTreeSet::new();
    while let Some(anchor) = pending {
        if chain.len() as u64 >= limits.max_chain_length {
            return Err("partial receipt chain exceeds length limit".into());
        }
        anchor_valid(&anchor, remote, limits)?;
        if !seen.insert(anchor.receipt.sha256.clone()) {
            return Err("partial receipt chain cycle".into());
        }
        let catalog: PartialCatalog =
            read_remote(backend, &anchor.catalog, scratch, limits, accounting)?;
        claims_false(
            catalog.cell_complete,
            catalog.solver_state_restorable,
            catalog.scientific_result_verified,
        )?;
        if catalog.schema != CATALOG_SCHEMA
            || catalog.remote != remote
            || catalog.binding_sha256 != anchor.binding_sha256
            || catalog.run_sha256 != anchor.run_sha256
            || catalog.attempt_sha256 != anchor.attempt_sha256
            || catalog.ordinal != anchor.ordinal
            || catalog.receipt != anchor.receipt
        {
            return Err("partial catalog does not match anchor".into());
        }
        let receipt: PartialReceipt =
            read_remote(backend, &anchor.receipt, scratch, limits, accounting)?;
        claims_false(
            receipt.cell_complete,
            receipt.solver_state_restorable,
            receipt.scientific_result_verified,
        )?;
        if receipt.schema != RECEIPT_SCHEMA
            || receipt.remote != remote
            || receipt.binding_sha256 != anchor.binding_sha256
            || receipt.ordinal != anchor.ordinal
        {
            return Err("partial receipt does not match anchor".into());
        }
        reference_valid(&receipt.manifest, limits.max_metadata_bytes, true)?;
        let manifest: SnapshotManifest =
            read_remote(backend, &receipt.manifest, scratch, limits, accounting)?;
        let bytes = manifest.objects.iter().try_fold(0_u64, |sum, o| {
            sum.checked_add(o.artifact.bytes)
                .ok_or("artifact byte overflow")
        })?;
        if manifest.binding.digest()? != anchor.binding_sha256
            || manifest.binding.run_sha256 != anchor.run_sha256
            || manifest.binding.attempt_sha256 != anchor.attempt_sha256
            || manifest.ordinal != anchor.ordinal
            || manifest.previous != receipt.previous
            || receipt.artifact_count != manifest.objects.len() as u64
            || receipt.artifact_bytes != bytes
        {
            return Err("partial manifest does not match receipt".into());
        }
        match &manifest.previous {
            None if anchor.ordinal != 0 => {
                return Err("partial chain is missing its predecessor".into())
            }
            Some(previous)
                if previous.ordinal.checked_add(1) != Some(anchor.ordinal)
                    || previous.binding_sha256 != anchor.binding_sha256
                    || previous.run_sha256 != anchor.run_sha256
                    || previous.attempt_sha256 != anchor.attempt_sha256
                    || previous.remote != anchor.remote =>
            {
                return Err("partial receipt predecessor identity/ordinal mismatch".into());
            }
            _ => {}
        }
        pending = manifest.previous.clone();
        chain.push(LoadedSnapshot {
            anchor,
            receipt,
            manifest,
        });
    }
    chain.reverse();
    Ok(chain)
}

fn open_store(
    cfg: Config,
    budget: Option<Arc<BudgetSession>>,
    phase: Phase,
) -> Result<Store, String> {
    match cfg.remote.as_ref() {
        Some(RemoteSpec::S3 { bucket, .. }) => {
            if bucket != "setec-astronomy-artifacts" {
                return Err("partial S3 snapshots require the approved artifact bucket".into());
            }
            let budget =
                budget.ok_or("partial S3 operations require cumulative budget admission")?;
            Store::open_guarded(cfg, budget.guard(phase))
        }
        Some(RemoteSpec::Dir(_)) => Store::open(cfg),
        None => Err("partial snapshots require an explicit remote".into()),
    }
}

pub fn publish_partial(
    snapshot_dir: &Path,
    cfg: Config,
    budget: Option<Arc<BudgetSession>>,
    request: &SnapshotRequest,
    limits: &SnapshotLimits,
) -> Result<SnapshotAck, String> {
    limits.validate()?;
    let phase = if request.observation.status == AttemptStatus::Running {
        Phase::Snapshot
    } else {
        Phase::Finalize
    };
    let store = open_store(cfg, budget, phase)?;
    publish_with_backend(
        snapshot_dir,
        store.remote()?,
        store.cfg.remote.as_ref().ok_or("no remote")?,
        request,
        limits,
    )
}

fn publish_with_backend(
    snapshot_dir: &Path,
    backend: &dyn Backend,
    remote: &RemoteSpec,
    request: &SnapshotRequest,
    limits: &SnapshotLimits,
) -> Result<SnapshotAck, String> {
    limits.validate()?;
    request.binding.validate()?;
    request.observation.validate()?;
    let root = storage::root_dir(snapshot_dir, false)?;
    let remote_name = remote.display();
    let mut accounting = Accounting::default();
    let mut state = ChainState::default();
    if let Some(previous) = &request.previous {
        if previous.ordinal.checked_add(1) != Some(request.ordinal)
            || previous.binding_sha256 != request.binding.digest()?
        {
            return Err("partial request does not extend its predecessor".into());
        }
        let chain = load_chain(
            backend,
            &remote_name,
            previous,
            &root,
            limits,
            &mut accounting,
        )?;
        for entry in &chain {
            advance(&entry.manifest, &mut state, limits, &mut accounting)?;
        }
    } else if request.ordinal != 0 {
        return Err("initial partial ordinal must be zero".into());
    }
    let previous_cursor = state
        .cursor
        .clone()
        .unwrap_or(CaptureCursor::initial(&request.binding)?);
    let mut next_coverage = coverage(&state);
    if let Some(delta) = &request.delta {
        next_coverage.capture_present = true;
        next_coverage.committed_events = delta.cursor.event_count;
        next_coverage.captured_event_end = delta.cursor.byte_end;
        next_coverage.stop = delta.head.stop.clone();
        next_coverage.capture_stopped = delta.head.stop.is_some();
    }
    let manifest = SnapshotManifest {
        schema: request.profile.schema().into(),
        binding: request.binding.clone(),
        ordinal: request.ordinal,
        previous: request.previous.clone(),
        context: request.context.clone(),
        delta: request.delta.clone(),
        observation: request.observation.clone(),
        coverage: next_coverage,
        objects: request_objects(request)?,
        cell_complete: false,
        solver_state_restorable: false,
        scientific_result_verified: false,
    };
    advance(&manifest, &mut state, limits, &mut accounting)?;
    if request.profile == SnapshotProfile::CheckedPhaseV2 {
        validate_checked_local_context(&root, &state)?;
    }
    for object in &manifest.objects {
        let source = storage::contained_path(&root, &object.artifact.path, false)?;
        check_regular(&source, &object.object)?;
    }
    if let Some(delta) = &manifest.delta {
        capture::verify_captured_delta(&root, delta, &previous_cursor)?;
    }
    // Freeze all metadata before network mutations. A retry in this directory
    // with a different observation or member set fails before a PUT.
    let prefix = format!("partial-publication/{:020}", request.ordinal);
    let manifest_artifact = write_metadata(
        &root,
        &format!("{prefix}/manifest.json"),
        "partial-artifact-manifest",
        &manifest,
        limits,
    )?;
    let receipt = PartialReceipt {
        schema: RECEIPT_SCHEMA.into(),
        remote: remote_name.clone(),
        binding_sha256: request.binding.digest()?,
        ordinal: request.ordinal,
        previous: request.previous.clone(),
        manifest: reference(&manifest_artifact)?,
        artifact_count: manifest.objects.len() as u64,
        artifact_bytes: manifest.objects.iter().try_fold(0_u64, |s, o| {
            s.checked_add(o.artifact.bytes)
                .ok_or("artifact bytes overflow")
        })?,
        cell_complete: false,
        solver_state_restorable: false,
        scientific_result_verified: false,
    };
    let receipt_artifact = write_metadata(
        &root,
        &format!("{prefix}/receipt.json"),
        "partial-artifact-receipt",
        &receipt,
        limits,
    )?;
    let catalog = PartialCatalog {
        schema: CATALOG_SCHEMA.into(),
        remote: remote_name.clone(),
        binding_sha256: request.binding.digest()?,
        run_sha256: request.binding.run_sha256.clone(),
        attempt_sha256: request.binding.attempt_sha256.clone(),
        ordinal: request.ordinal,
        receipt: reference(&receipt_artifact)?,
        cell_complete: false,
        solver_state_restorable: false,
        scientific_result_verified: false,
    };
    let catalog_artifact = write_metadata(
        &root,
        &format!("{prefix}/catalog.json"),
        "partial-artifact-catalog",
        &catalog,
        limits,
    )?;
    for a in [&manifest_artifact, &receipt_artifact, &catalog_artifact] {
        accounting.add(a.bytes, limits)?;
    }
    let catalog_reference = ObjectRef {
        key: catalog_key(
            &request.binding.run_sha256,
            &request.binding.attempt_sha256,
            request.ordinal,
        ),
        sha256: catalog_artifact.sha256.clone(),
        bytes: catalog_artifact.bytes,
    };
    for object in &manifest.objects {
        publish_object(backend, remote, &root, &object.artifact, &object.object)?;
    }
    publish_object(
        backend,
        remote,
        &root,
        &manifest_artifact,
        &receipt.manifest,
    )?;
    publish_object(backend, remote, &root, &receipt_artifact, &catalog.receipt)?;
    // Fixed ordinal key is installed last, conditionally. A different producer
    // cannot repoint an already acknowledged ordinal to another receipt.
    publish_object(
        backend,
        remote,
        &root,
        &catalog_artifact,
        &catalog_reference,
    )?;
    let anchor = SnapshotAnchor {
        schema: ANCHOR_SCHEMA.into(),
        remote: remote_name,
        binding_sha256: request.binding.digest()?,
        run_sha256: request.binding.run_sha256.clone(),
        attempt_sha256: request.binding.attempt_sha256.clone(),
        ordinal: request.ordinal,
        receipt: catalog.receipt,
        catalog: catalog_reference,
    };
    let confirmation = write_metadata(
        &root,
        &format!("{prefix}/confirmation.json"),
        "verified-partial-publication-confirmation",
        &serde_json::json!({
            "schema":"ml-dataset.partial-confirmation/v1", "anchor":anchor,
            "current_objects_roundtrip_verified":true,
            "cell_complete":false,"solver_state_restorable":false,"scientific_result_verified":false
        }),
        limits,
    )?;
    Ok(SnapshotAck {
        schema: "ml-dataset.partial-ack/v1".into(),
        anchor,
        cursor: state.cursor,
        manifest: manifest_artifact,
        receipt: receipt_artifact,
        catalog: catalog_artifact,
        confirmation,
        cell_complete: false,
        solver_state_restorable: false,
        scientific_result_verified: false,
    })
}

pub fn restore_partial(
    cfg: Config,
    budget: Option<Arc<BudgetSession>>,
    anchor: &SnapshotAnchor,
    destination: &Path,
    limits: &SnapshotLimits,
) -> Result<RestoreReport, String> {
    limits.validate()?;
    let store = open_store(cfg, budget, Phase::Restore)?;
    restore_with_backend(
        store.remote()?,
        store.cfg.remote.as_ref().ok_or("no remote")?,
        anchor,
        destination,
        limits,
    )
}

fn restore_with_backend(
    backend: &dyn Backend,
    remote: &RemoteSpec,
    anchor: &SnapshotAnchor,
    destination: &Path,
    limits: &SnapshotLimits,
) -> Result<RestoreReport, String> {
    limits.validate()?;
    anchor_valid(anchor, &remote.display(), limits)?;
    let destination = storage::root_dir(destination, true)?;
    let relative = format!(
        "interrupted-attempts/{}/{}/artifacts/.reserved",
        anchor.run_sha256, anchor.attempt_sha256
    );
    let reserved = storage::contained_path(&destination, &relative, true)?;
    let artifacts = reserved.parent().ok_or("restore namespace has no parent")?;
    let root = artifacts.parent().ok_or("restore namespace missing")?;
    let mut accounting = Accounting::default();
    let chain = load_chain(
        backend,
        &remote.display(),
        anchor,
        root,
        limits,
        &mut accounting,
    )?;
    let mut state = ChainState::default();
    // Preflight *all* structure, cumulative bytes and member counts before any
    // artifact GET. Metadata GETs are separately bounded and budgeted.
    for entry in &chain {
        advance(&entry.manifest, &mut state, limits, &mut accounting)?;
    }
    let mut previous_cursor =
        CaptureCursor::initial(state.binding.as_ref().ok_or("empty partial chain")?)?;
    let mut artifact_count = 0_u64;
    let mut artifact_bytes = 0_u64;
    for entry in &chain {
        for object in &entry.manifest.objects {
            let downloaded = download(backend, &object.object, root)?;
            install_copy(
                artifacts,
                &object.artifact.path,
                &downloaded.path,
                &object.object,
            )?;
            artifact_count = artifact_count
                .checked_add(1)
                .ok_or("restore object overflow")?;
            artifact_bytes = artifact_bytes
                .checked_add(object.artifact.bytes)
                .ok_or("restore bytes overflow")?;
        }
        if let Some(delta) = &entry.manifest.delta {
            capture::verify_captured_delta(artifacts, delta, &previous_cursor)?;
            previous_cursor = delta.cursor.clone();
        }
        // Retain the original hash-bound metadata as well as artifacts. These
        // extra fresh GETs are charged by the same session, not a new budget.
        for (name, object) in [
            ("catalog", &entry.anchor.catalog),
            ("receipt", &entry.anchor.receipt),
            ("manifest", &entry.receipt.manifest),
        ] {
            let downloaded = download(backend, object, root)?;
            install_copy(
                root,
                &format!("snapshots/{:020}/{name}.json", entry.anchor.ordinal),
                &downloaded.path,
                object,
            )?;
        }
    }
    if state.profile == Some(SnapshotProfile::CheckedPhaseV2) {
        validate_checked_local_context(artifacts, &state)?;
    }
    let report = RestoreReport {
        schema: "ml-dataset.artifact-only-restore/v1".into(),
        anchor: anchor.clone(),
        restored_directory: root.to_path_buf(),
        snapshot_count: chain.len() as u64,
        artifact_count,
        artifact_bytes,
        cursor: state.cursor.clone(),
        observation: state
            .last_observation
            .clone()
            .ok_or("partial chain has no observation")?,
        coverage: coverage(&state),
        solver_resumed: false,
        cell_complete: false,
        solver_state_restorable: false,
        scientific_result_verified: false,
    };
    write_metadata(
        root,
        &format!("restores/{:020}.json", anchor.ordinal),
        "verified-artifact-only-restore",
        &report,
        limits,
    )?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::backend::{DirBackend, Meta};
    use std::sync::Mutex;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "ml-partial-{}-{}",
                std::process::id(),
                TEMP_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            for name in ["source", "remote", "cell"] {
                fs::create_dir(root.join(name)).unwrap();
            }
            Self(root)
        }
        fn source(&self) -> PathBuf {
            self.0.join("source")
        }
        fn remote(&self) -> RemoteSpec {
            RemoteSpec::Dir(self.0.join("remote"))
        }
        fn config(&self) -> Config {
            Config {
                index_path: self.0.join("never-written-index.json"),
                cache_dir: self.0.join("cache"),
                remote: Some(self.remote()),
                endpoint: None,
                aws: "/must-not-execute-aws".into(),
            }
        }
        fn backend(&self) -> RecordingBackend {
            RecordingBackend {
                inner: DirBackend::new(self.0.join("remote")),
                gets: Mutex::new(Vec::new()),
                corrupt: Mutex::new(None),
                denied: Mutex::new(None),
            }
        }
        fn initial(&self) -> SnapshotRequest {
            let mut context = Vec::new();
            for name in ["run", "plan", "protocol", "config", "attempt"] {
                context.push(
                    storage::write_bytes(
                        &self.source(),
                        &format!("context/{name}.json"),
                        "restricted-provenance",
                        "json",
                        format!("{{\"fixture\":\"{name}\"}}\n").as_bytes(),
                    )
                    .unwrap(),
                );
            }
            let hash = |name: &str| {
                context
                    .iter()
                    .find(|a| a.path == format!("context/{name}.json"))
                    .unwrap()
                    .sha256
                    .clone()
            };
            let binding = AttemptBinding {
                schema: "crypto.ml-dataset.attempt-binding/v1".into(),
                run_sha256: hash("run"),
                plan_sha256: hash("plan"),
                protocol_sha256: hash("protocol"),
                config_sha256: hash("config"),
                attempt_sha256: hash("attempt"),
                source_commit: "a".repeat(40),
                binary_sha256: "b".repeat(64),
                cell_index: 12,
            };
            SnapshotRequest {
                profile: SnapshotProfile::LegacyV1,
                binding,
                ordinal: 0,
                previous: None,
                context,
                delta: None,
                observation: AttemptObservation {
                    status: AttemptStatus::Running,
                    observed_unix_seconds: 100,
                    exit_code: None,
                    signal: None,
                    reason: None,
                },
            }
        }
        fn publish(&self, request: &SnapshotRequest) -> SnapshotAck {
            publish_partial(
                &self.source(),
                self.config(),
                None,
                request,
                &SnapshotLimits::default(),
            )
            .unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    struct RecordingBackend {
        inner: DirBackend,
        gets: Mutex<Vec<String>>,
        corrupt: Mutex<Option<String>>,
        denied: Mutex<Option<String>>,
    }
    impl Backend for RecordingBackend {
        fn describe(&self) -> String {
            self.inner.describe()
        }
        fn head(&self, key: &str) -> Result<Option<Meta>, String> {
            self.inner.head(key)
        }
        fn put(&self, _key: &str, _src: &Path, _sha256: &str) -> Result<(), String> {
            panic!("partial file publisher must never use overwriting directory PUT")
        }
        fn get(&self, key: &str, destination: &Path) -> Result<(), String> {
            self.gets.lock().unwrap().push(key.into());
            if self.denied.lock().unwrap().as_deref() == Some(key) {
                return Err("injected GET failure".into());
            }
            self.inner.get(key, destination)?;
            if self.corrupt.lock().unwrap().as_deref() == Some(key) {
                let mut bytes = fs::read(destination).unwrap();
                bytes[0] ^= 1;
                fs::write(destination, bytes).unwrap();
            }
            Ok(())
        }
        fn list(&self, _prefix: &str) -> Result<Vec<(String, u64)>, String> {
            panic!("partial publisher/restore must not list unbounded remote inventories")
        }
    }

    fn terminal(initial: &SnapshotRequest, ack: &SnapshotAck) -> SnapshotRequest {
        SnapshotRequest {
            profile: initial.profile,
            binding: initial.binding.clone(),
            ordinal: ack.anchor.ordinal + 1,
            previous: Some(ack.anchor.clone()),
            context: vec![],
            delta: None,
            observation: AttemptObservation {
                status: AttemptStatus::Interrupted,
                observed_unix_seconds: 101,
                exit_code: Some(143),
                signal: Some(15),
                reason: Some("fixture interruption".into()),
            },
        }
    }

    #[test]
    fn context_only_and_empty_terminal_restore_as_artifacts_never_completed_cells() {
        let f = Fixture::new();
        let initial = f.initial();
        let first = f.publish(&initial);
        assert!(first.cursor.is_none());
        let final_ack = f.publish(&terminal(&initial, &first));
        let output = f.0.join("restored");
        let report = restore_partial(
            f.config(),
            None,
            &final_ack.anchor,
            &output,
            &SnapshotLimits::default(),
        )
        .unwrap();
        assert_eq!(report.snapshot_count, 2);
        assert_eq!(report.artifact_count, 5);
        assert_eq!(report.observation.status, AttemptStatus::Interrupted);
        assert!(
            !report.cell_complete
                && !report.solver_state_restorable
                && !report.scientific_result_verified
        );
        assert!(!report.coverage.capture_present);
        assert!(report.coverage.artifact_only);
        assert!(!report.coverage.in_flight_solver_memory_captured);
        assert!(!report.restored_directory.join("manifest.json").exists());
        assert!(!output.join("cells").exists());
        assert!(!f.config().index_path.exists());
        for artifact in &initial.context {
            assert_eq!(
                fs::read(f.source().join(&artifact.path)).unwrap(),
                fs::read(
                    report
                        .restored_directory
                        .join("artifacts")
                        .join(&artifact.path)
                )
                .unwrap()
            );
        }
        // Restoration is idempotent, but every retry still downloads all data.
        let again = restore_partial(
            f.config(),
            None,
            &final_ack.anchor,
            &output,
            &SnapshotLimits::default(),
        )
        .unwrap();
        assert_eq!(report, again);
    }

    #[test]
    fn committed_events_payloads_and_capture_stop_survive_roundtrip() {
        use super::super::capture::{CaptureEmitter, CaptureLimits};
        use super::super::types::ArtifactPayload;
        let f = Fixture::new();
        let initial = f.initial();
        let first = f.publish(&initial);
        let capture_limits = CaptureLimits {
            max_event_bytes: 1024,
            max_stream_bytes: 32 * 1024,
            max_payload_bytes: 4096,
            max_payload_count: 16,
            max_capture_bytes: 256 * 1024,
            chunk_bytes: 2048,
            max_chunks: 64,
            metadata_reserve_bytes: 4096,
        };
        let cell = f.0.join("cell");
        let mut emitter = CaptureEmitter::create(
            &cell.join("live"),
            initial.binding.clone(),
            capture_limits.clone(),
        )
        .unwrap();
        emitter
            .event(&serde_json::json!({"kind":"opaque_backend_enter"}))
            .unwrap();
        emitter
            .payload(
                &ArtifactPayload::json(
                    "matrix.json",
                    "exposed-matrix",
                    &serde_json::json!({"rows":[1,2]}),
                )
                .unwrap(),
            )
            .unwrap();
        assert!(emitter
            .event(&serde_json::json!({"kind":"oversized","value":"x".repeat(2048)}))
            .is_err());
        let delta = capture::capture_committed_delta(
            &cell,
            &f.source(),
            &initial.binding,
            &CaptureCursor::initial(&initial.binding).unwrap(),
            &capture_limits,
        )
        .unwrap()
        .unwrap();
        let mut next = terminal(&initial, &first);
        next.observation.status = AttemptStatus::Running;
        next.observation.exit_code = None;
        next.observation.signal = None;
        next.delta = Some(delta);
        let ack = f.publish(&next);
        let report = restore_partial(
            f.config(),
            None,
            &ack.anchor,
            &f.0.join("restored"),
            &SnapshotLimits::default(),
        )
        .unwrap();
        assert_eq!(report.observation.status, AttemptStatus::Running);
        assert!(report.coverage.capture_stopped);
        assert_eq!(report.coverage.committed_events, 2);
        assert_eq!(report.cursor.unwrap().payload_count, 1);
        assert!(report
            .restored_directory
            .join("artifacts/payloads/matrix.json")
            .is_file());
        assert!(!report.solver_state_restorable);
    }

    #[test]
    fn same_ordinal_retry_gets_every_object_again_and_conflict_never_overwrites_catalog() {
        let f = Fixture::new();
        let initial = f.initial();
        let backend = f.backend();
        let limits = SnapshotLimits::default();
        let first =
            publish_with_backend(&f.source(), &backend, &f.remote(), &initial, &limits).unwrap();
        assert_eq!(backend.gets.lock().unwrap().len(), 8);
        let second =
            publish_with_backend(&f.source(), &backend, &f.remote(), &initial, &limits).unwrap();
        assert_eq!(first, second);
        assert_eq!(backend.gets.lock().unwrap().len(), 16);
        assert_eq!(
            backend.gets.lock().unwrap().last().unwrap(),
            &first.anchor.catalog.key
        );
        let original = fs::read(f.0.join("remote").join(&first.anchor.catalog.key)).unwrap();
        let mut changed = initial.clone();
        changed.observation.observed_unix_seconds += 1;
        assert!(
            publish_with_backend(&f.source(), &backend, &f.remote(), &changed, &limits).is_err()
        );
        assert_eq!(
            backend.gets.lock().unwrap().len(),
            16,
            "local freeze must precede remote writes"
        );
        let another = f.0.join("another-helper");
        fs::create_dir(&another).unwrap();
        for a in &initial.context {
            install_copy(
                &another,
                &a.path,
                &f.source().join(&a.path),
                &reference(a).unwrap(),
            )
            .unwrap();
        }
        assert!(publish_with_backend(&another, &backend, &f.remote(), &changed, &limits).is_err());
        assert_eq!(
            original,
            fs::read(f.0.join("remote").join(&first.anchor.catalog.key)).unwrap()
        );
        assert!(!another
            .join("partial-publication/00000000000000000000/confirmation.json")
            .exists());
    }

    #[test]
    fn final_catalog_get_failure_never_acknowledges_and_exact_retry_can_finish() {
        let f = Fixture::new();
        let request = f.initial();
        let backend = f.backend();
        let key = catalog_key(
            &request.binding.run_sha256,
            &request.binding.attempt_sha256,
            0,
        );
        *backend.denied.lock().unwrap() = Some(key);
        let limits = SnapshotLimits::default();
        assert!(
            publish_with_backend(&f.source(), &backend, &f.remote(), &request, &limits).is_err()
        );
        assert!(!f
            .source()
            .join("partial-publication/00000000000000000000/confirmation.json")
            .exists());
        *backend.denied.lock().unwrap() = None;
        publish_with_backend(&f.source(), &backend, &f.remote(), &request, &limits).unwrap();
    }

    #[test]
    fn corrupt_gets_of_data_manifest_receipt_and_catalog_are_rejected() {
        let f = Fixture::new();
        let request = f.initial();
        let ack = f.publish(&request);
        let backend = f.backend();
        let keys = [
            reference(&request.context[0]).unwrap().key,
            reference(&ack.manifest).unwrap().key,
            ack.anchor.receipt.key.clone(),
            ack.anchor.catalog.key.clone(),
        ];
        for (i, key) in keys.iter().enumerate() {
            *backend.corrupt.lock().unwrap() = Some(key.clone());
            assert!(restore_with_backend(
                &backend,
                &f.remote(),
                &ack.anchor,
                &f.0.join(format!("corrupt-{i}")),
                &SnapshotLimits::default()
            )
            .is_err());
            assert!(publish_with_backend(
                &f.source(),
                &backend,
                &f.remote(),
                &request,
                &SnapshotLimits::default()
            )
            .is_err());
        }
    }

    #[test]
    fn mandatory_context_source_integrity_and_limits_fail_before_remote_io() {
        let f = Fixture::new();
        let request = f.initial();
        let backend = f.backend();
        let limits = SnapshotLimits::default();
        let mut missing = request.clone();
        missing.context.pop();
        assert!(
            publish_with_backend(&f.source(), &backend, &f.remote(), &missing, &limits).is_err()
        );
        let mut bad_hash = request.clone();
        bad_hash.context[0].sha256 = "f".repeat(64);
        assert!(
            publish_with_backend(&f.source(), &backend, &f.remote(), &bad_hash, &limits).is_err()
        );
        let mut invalid = request.clone();
        invalid.context[0].path = "context/../run.json".into();
        assert!(
            publish_with_backend(&f.source(), &backend, &f.remote(), &invalid, &limits).is_err()
        );
        for bounded in [
            SnapshotLimits {
                max_object_bytes: 1,
                ..limits.clone()
            },
            SnapshotLimits {
                max_snapshot_artifacts: 4,
                ..limits.clone()
            },
            SnapshotLimits {
                max_total_objects: 4,
                ..limits.clone()
            },
            SnapshotLimits {
                max_total_bytes: 1,
                ..limits.clone()
            },
            SnapshotLimits {
                max_metadata_bytes: 1024,
                ..limits.clone()
            },
        ] {
            assert!(
                publish_with_backend(&f.source(), &backend, &f.remote(), &request, &bounded)
                    .is_err()
            );
        }
        assert!(backend.gets.lock().unwrap().is_empty());
    }

    #[test]
    fn chain_bound_missing_predecessor_and_context_redefinition_fail_closed() {
        let f = Fixture::new();
        let initial = f.initial();
        let first = f.publish(&initial);
        let backend = f.backend();
        let mut next = terminal(&initial, &first);
        next.context.push(initial.context[0].clone());
        assert!(publish_with_backend(
            &f.source(),
            &backend,
            &f.remote(),
            &next,
            &SnapshotLimits::default()
        )
        .is_err());
        next.context.clear();
        next.ordinal = 2;
        assert!(publish_with_backend(
            &f.source(),
            &backend,
            &f.remote(),
            &next,
            &SnapshotLimits::default()
        )
        .is_err());
        next.ordinal = 1;
        let second = f.publish(&next);
        let bounded = SnapshotLimits {
            max_chain_length: 1,
            ..SnapshotLimits::default()
        };
        backend.gets.lock().unwrap().clear();
        assert!(restore_with_backend(
            &backend,
            &f.remote(),
            &second.anchor,
            &f.0.join("limited"),
            &bounded
        )
        .is_err());
        assert!(backend.gets.lock().unwrap().is_empty());
        fs::remove_file(f.0.join("remote").join(&first.anchor.receipt.key)).unwrap();
        assert!(restore_with_backend(
            &backend,
            &f.remote(),
            &second.anchor,
            &f.0.join("missing"),
            &SnapshotLimits::default()
        )
        .is_err());
    }

    #[test]
    fn optional_host_process_context_is_retained_once() {
        let f = Fixture::new();
        let initial = f.initial();
        let first = f.publish(&initial);
        let mut next = terminal(&initial, &first);
        for name in ["host", "worker-process"] {
            next.context.push(
                storage::write_bytes(
                    &f.source(),
                    &format!("context/{name}.json"),
                    "restricted-provenance",
                    "json",
                    b"{}\n",
                )
                .unwrap(),
            );
        }
        let ack = f.publish(&next);
        let report = restore_partial(
            f.config(),
            None,
            &ack.anchor,
            &f.0.join("restored"),
            &SnapshotLimits::default(),
        )
        .unwrap();
        assert_eq!(report.artifact_count, 7);
    }

    #[test]
    fn missing_s3_budget_or_unapproved_bucket_never_runs_aws() {
        let f = Fixture::new();
        let request = f.initial();
        for bucket in ["setec-astronomy-artifacts", "unapproved-bucket"] {
            let mut cfg = f.config();
            cfg.remote = Some(RemoteSpec::S3 {
                bucket: bucket.into(),
                prefix: "".into(),
            });
            let error =
                publish_partial(&f.source(), cfg, None, &request, &SnapshotLimits::default())
                    .unwrap_err();
            assert!(error.contains("budget") || error.contains("approved"));
        }
    }

    #[test]
    fn typed_partial_metadata_rejects_unknown_duplicate_and_true_claims() {
        let f = Fixture::new();
        let request = f.initial();
        let ack = f.publish(&request);
        let raw = fs::read(f.source().join(&ack.manifest.path)).unwrap();
        let mut manifest: SnapshotManifest = serde_json::from_slice(&raw).unwrap();
        manifest.scientific_result_verified = true;
        assert!(advance(
            &manifest,
            &mut ChainState::default(),
            &SnapshotLimits::default(),
            &mut Accounting::default()
        )
        .is_err());
        let unknown =
            String::from_utf8(raw.clone())
                .unwrap()
                .replacen('{', "{\"completion\":true,", 1);
        assert!(serde_json::from_str::<SnapshotManifest>(&unknown).is_err());
        let duplicate = String::from_utf8(raw)
            .unwrap()
            .replacen('{', "{\"ordinal\":0,", 1);
        assert!(serde_json::from_str::<SnapshotManifest>(&duplicate).is_err());
        assert!(SnapshotLimits {
            max_total_bytes: u64::MAX,
            ..SnapshotLimits::default()
        }
        .validate()
        .is_err());
        let mut account = Accounting {
            objects: u64::MAX,
            bytes: 0,
        };
        assert!(account.add(0, &SnapshotLimits::default()).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_fifos_and_restore_namespace_symlinks_are_rejected_without_blocking() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        let request = f.initial();
        let ack = f.publish(&request);
        let source = f.source().join(&request.context[0].path);
        fs::remove_file(&source).unwrap();
        symlink(f.source().join(&request.context[1].path), &source).unwrap();
        assert!(publish_partial(
            &f.source(),
            f.config(),
            None,
            &request,
            &SnapshotLimits::default()
        )
        .is_err());
        fs::remove_file(&source).unwrap();
        let fifo = std::ffi::CString::new(source.as_os_str().as_encoded_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
        assert!(read_bounded(&source, 1024).is_err());
        let output = f.0.join("restored");
        fs::create_dir(&output).unwrap();
        symlink(f.0.join("cell"), output.join("interrupted-attempts")).unwrap();
        assert!(restore_partial(
            f.config(),
            None,
            &ack.anchor,
            &output,
            &SnapshotLimits::default()
        )
        .is_err());
        assert!(!f.0.join("cell").join(&request.binding.run_sha256).exists());
    }
}
