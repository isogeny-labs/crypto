//! Fixed-policy calling-thread phase diagnostics and strict bounded replay.
mod native;
pub(crate) mod types;

use crate::cryptanalysis::ecbench::isolation::checked::{
    CheckedCounters, CounterPolicy, CyclesPolicy, TaskScope,
};
use crate::hash::sha256::Sha256;
use crate::ml_dataset::{capture::AttemptBinding, digest, storage, types::Artifact};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;
use types::*;

pub(crate) const POLICY_PATH: &str = "measurement-policy.json";
pub(crate) const POLICY_BYTES: &[u8] = include_bytes!("policy.json");
pub(crate) const POLICY_HASH: &str =
    "45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50";
pub(crate) const MAX_CONTEXT: u64 = 16 * 1024 * 1024;
const START_MAX: usize = 65536;
const METRICS_MAX: usize = 524288;
const PREVIEW_MAX: usize = 4096;
const USAGE_SCOPE: &str =
    "cumulative worker process; peak RSS is a high-water mark, not an additive phase cost";

pub(crate) fn install_policy(root: &Path) -> Result<Artifact, String> {
    storage::write_bytes(
        root,
        POLICY_PATH,
        "restricted-measurement-policy",
        "json",
        POLICY_BYTES,
    )
}

// The local integration uses this classifier only for registry assertions.
// Remote snapshot callers are not part of this integration.
#[cfg(test)]
pub(crate) fn role(path: &str) -> Option<&'static str> {
    if path == POLICY_PATH {
        return Some("restricted-measurement-policy");
    }
    for phase in Phase::ALL {
        if path == phase.path(true) {
            return Some("phase-start");
        }
        if path == phase.path(false) {
            return Some("hardware-and-resource-telemetry");
        }
    }
    None
}

pub(crate) fn byte_limit(path: &str) -> u64 {
    if path == POLICY_PATH {
        16384
    } else if path.ends_with("-start.json") {
        START_MAX as u64
    } else if path.starts_with("telemetry/") {
        METRICS_MAX as u64
    } else {
        MAX_CONTEXT
    }
}

/// Reads one ordinary immutable file, checking bounds before and after reading.
pub(crate) fn read(root: &Path, path: &str, max: u64) -> Result<Vec<u8>, String> {
    read_before_open(root, path, max, |_| {})
}

fn read_before_open(
    root: &Path,
    path: &str,
    max: u64,
    before_open: impl FnOnce(&Path),
) -> Result<Vec<u8>, String> {
    let root = storage::root_dir(root, false)?;
    let path = storage::contained_path(&root, path, false)?;
    let before = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if !before.is_file() || before.file_type().is_symlink() || before.len() > max {
        return Err("checked context is not an ordinary bounded file".into());
    }
    before_open(&path);
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // A regular path may become a FIFO after the metadata check. Open it
        // without waiting for a writer, then reject its descriptor type below.
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(&path).map_err(|e| e.to_string())?;
    let opened = file.metadata().map_err(|e| e.to_string())?;
    if !opened.is_file() || opened.len() != before.len() {
        return Err("checked context changed before read".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if before.dev() != opened.dev() || before.ino() != opened.ino() {
            return Err("checked context replaced before read".into());
        }
    }
    let mut bytes = Vec::new();
    file.take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let after = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if bytes.len() as u64 != before.len()
        || after.len() != before.len()
        || after.file_type().is_symlink()
    {
        return Err("checked context changed during read".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if before.dev() != after.dev() || before.ino() != after.ino() {
            return Err("checked context replaced during read".into());
        }
    }
    Ok(bytes)
}

pub(crate) fn policy_present(root: &Path) -> Result<bool, String> {
    match fs::symlink_metadata(root.join(POLICY_PATH)) {
        Ok(_) => {
            let bytes = read(root, POLICY_PATH, 16384)?;
            let policy: FixedPolicy = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            let expected: FixedPolicy =
                serde_json::from_slice(POLICY_BYTES).map_err(|e| e.to_string())?;
            if bytes != POLICY_BYTES || policy != expected {
                return Err("measurement policy differs from fixed bytes".into());
            }
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

struct Bounded {
    bytes: Vec<u8>,
    max: usize,
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.max.saturating_sub(self.bytes.len()) {
            return Err(std::io::Error::other("phase record exceeds bound"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn encode(value: &impl Serialize, max: usize) -> Result<Vec<u8>, String> {
    let mut sink = Bounded {
        bytes: Vec::new(),
        max,
    };
    serde_json::to_writer(&mut sink, value).map_err(|e| e.to_string())?;
    sink.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(sink.bytes)
}

struct ErrorSink {
    preview: String,
    bytes: u64,
    digest: Sha256,
    stopped_preview: bool,
}
impl fmt::Write for ErrorSink {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.bytes = self.bytes.checked_add(s.len() as u64).ok_or(fmt::Error)?;
        self.digest.update(s.as_bytes());
        if !self.stopped_preview {
            let mut n = s.len().min(PREVIEW_MAX.saturating_sub(self.preview.len()));
            while !s.is_char_boundary(n) {
                n -= 1;
            }
            self.preview.push_str(&s[..n]);
            self.stopped_preview = n != s.len();
        }
        Ok(())
    }
}
fn summarize(error: &impl fmt::Display) -> Result<ErrorSummary, String> {
    let mut sink = ErrorSink {
        preview: String::new(),
        bytes: 0,
        digest: Sha256::new(),
        stopped_preview: false,
    };
    write!(&mut sink, "{error}").map_err(|_| "error display or byte count failed")?;
    Ok(ErrorSummary {
        truncated: sink.bytes > sink.preview.len() as u64,
        message_preview: sink.preview,
        message_bytes: sink.bytes,
        message_sha256: sink
            .digest
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect(),
    })
}

fn validate_usage(usage: &Option<Usage>) -> Result<(), String> {
    if usage.as_ref().is_some_and(|u| {
        u.scope != USAGE_SCOPE || !matches!(u.peak_rss_unit.as_str(), "bytes" | "KiB")
    }) {
        return Err("invalid process usage scope or RSS unit".into());
    }
    Ok(())
}

fn validate_start(start: &Start, phase: Phase, binding: &AttemptBinding) -> Result<(), String> {
    if start.schema != "crypto.ml-dataset.phase-start/v2"
        || start.phase != phase
        || start.ordinal != phase.ordinal()
        || &start.binding != binding
        || start.policy_sha256 != POLICY_HASH
        || start.recording_pid == 0
        || start.counter_boundary != "durable_start_before_open"
    {
        return Err("invalid checked phase start or binding".into());
    }
    validate_usage(&start.usage_before)
}

fn validate_metrics(m: &Metrics, start: &Start, start_bytes: &[u8]) -> Result<(), String> {
    if m.schema != "crypto.ml-dataset.phase-metrics/v2"
        || m.phase != start.phase
        || m.ordinal != start.ordinal
        || m.binding != start.binding
        || m.policy_sha256 != POLICY_HASH
        || m.start_sha256 != digest(start_bytes)
        || m.returned_ok == m.returned_error.is_some()
        || m.training_cost_eligible
        || m.whole_worker_retired_instructions.is_some()
        || m.whole_worker_coverage_verified
        || m.independently_verified
        || m.platform_admitted
        || m.clock_valid_for_comparison
        || m.interrupted_phase_readback
        || m.consumer_v2_admitted
    {
        return Err("invalid checked phase metrics, start binding or claim".into());
    }
    let elapsed = m
        .elapsed_ns_diagnostic
        .parse::<u128>()
        .map_err(|_| "invalid diagnostic elapsed")?;
    if elapsed.to_string() != m.elapsed_ns_diagnostic {
        return Err("noncanonical diagnostic elapsed".into());
    }
    if let Some(e) = &m.returned_error {
        crate::store::index::validate_sha256(&e.message_sha256)?;
        if e.message_preview.len() > PREVIEW_MAX
            || e.message_bytes < e.message_preview.len() as u64
            || e.truncated != (e.message_bytes > e.message_preview.len() as u64)
            || (!e.truncated && e.message_sha256 != digest(e.message_preview.as_bytes()))
        {
            return Err("invalid bounded phase error summary".into());
        }
    }
    validate_usage(&m.usage_after)?;
    let (started, observations) = native::validate(&m.collection, start.recording_pid)?;
    if started != m.counter_start_succeeded || observations != m.observations {
        return Err("checked phase observation projection mismatch".into());
    }
    Ok(())
}

/// Validates a finite dependency-closed map. Unrelated context keys are ignored.
/// Returns exact roles only for validated policy/start/end artifacts.
pub(crate) fn validate_context(
    binding: &AttemptBinding,
    files: &BTreeMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, &'static str>, String> {
    binding.validate()?;
    if files.get(POLICY_PATH).map(Vec::as_slice) != Some(POLICY_BYTES) {
        return Err("checked context lacks fixed measurement policy".into());
    }
    let mut roles = BTreeMap::from([(POLICY_PATH.into(), "restricted-measurement-policy")]);
    let mut may_start = true;
    for phase in Phase::ALL {
        let start_bytes = files.get(&phase.path(true));
        let end_bytes = files.get(&phase.path(false));
        match (start_bytes, end_bytes) {
            (None, Some(_)) => return Err("checked phase end lacks start".into()),
            (None, None) => {
                may_start = false;
            }
            (Some(bytes), end) => {
                if !may_start || bytes.len() > START_MAX {
                    return Err("checked phase order or size".into());
                }
                let start: Start = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
                validate_start(&start, phase, binding)?;
                roles.insert(phase.path(true), "phase-start");
                may_start = false;
                if let Some(bytes_end) = end {
                    if bytes_end.len() > METRICS_MAX {
                        return Err("checked phase metrics size".into());
                    }
                    let m: Metrics =
                        serde_json::from_slice(bytes_end).map_err(|e| e.to_string())?;
                    validate_metrics(&m, &start, bytes)?;
                    may_start = m.returned_ok;
                    roles.insert(phase.path(false), "hardware-and-resource-telemetry");
                }
            }
        }
    }
    Ok(roles)
}

pub(crate) fn context_files(root: &Path) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut files = BTreeMap::new();
    files.insert(POLICY_PATH.into(), read(root, POLICY_PATH, 16384)?);
    for phase in Phase::ALL {
        for start in [true, false] {
            let path = phase.path(start);
            match fs::symlink_metadata(root.join(&path)) {
                Ok(_) => {
                    files.insert(path.clone(), read(root, &path, byte_limit(&path))?);
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.to_string()),
            }
        }
    }
    Ok(files)
}

pub(crate) fn validate_directory(root: &Path) -> Result<BTreeMap<String, &'static str>, String> {
    let binding = AttemptBinding::from_cell_files(root)?;
    validate_context(&binding, &context_files(root)?)
}

/// Legacy cells historically hash-checked opaque diagnostics, including torn or
/// oversized JSON. This is only a bounded v2 discriminator, never v1 admission.
/// Every valid v2 record fits the metrics bound. Larger/unparseable legacy bytes
/// remain opaque unless a decoded top-level v2 schema was already encountered;
/// restricted reporting performs its own schema/size validation.
fn legacy_declares_checked(root: &Path, artifact: &Artifact) -> Result<bool, String> {
    struct Discriminator<'a>(&'a mut bool);
    impl<'de> serde::de::Visitor<'de> for Discriminator<'_> {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("legacy diagnostic object")
        }
        fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
            while let Some(key) = map.next_key::<String>()? {
                if key == "schema" {
                    let schema = map.next_value::<String>()?;
                    if matches!(
                        schema.as_str(),
                        "crypto.ml-dataset.phase-start/v2" | "crypto.ml-dataset.phase-metrics/v2"
                    ) {
                        *self.0 = true;
                        return Ok(());
                    }
                } else {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
            Ok(())
        }
    }
    let path = storage::contained_path(root, &artifact.path, false)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path).map_err(|e| e.to_string())?;
    let metadata = file.metadata().map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() != artifact.bytes {
        return Err("legacy diagnostic changed after hash verification".into());
    }
    // The reader limit also bounds string scratch allocation for schema/key
    // decoding; arbitrary legacy returned_error strings use IgnoredAny.
    let mut deserializer = serde_json::Deserializer::from_reader(std::io::BufReader::new(
        file.take(METRICS_MAX as u64 + 1),
    ));
    let mut checked = false;
    let _ = serde::Deserializer::deserialize_map(&mut deserializer, Discriminator(&mut checked));
    Ok(checked)
}

/// Strict v2 dispatch without changing the legacy record representation.
pub(crate) fn verify_artifacts(root: &Path, artifacts: &[Artifact]) -> Result<(), String> {
    let checked = policy_present(root)?;
    let roles = if checked {
        validate_directory(root)?
    } else {
        BTreeMap::new()
    };
    for (path, role) in &roles {
        if !artifacts
            .iter()
            .any(|a| a.path == *path && a.role == *role && a.encoding == "json")
        {
            return Err(
                "checked phase artifact absent or incorrectly classified in manifest".into(),
            );
        }
    }
    for artifact in artifacts {
        if artifact.path.starts_with("telemetry/") && artifact.path.ends_with(".json") {
            if checked {
                if roles.get(&artifact.path).copied() != Some(artifact.role.as_str())
                    || artifact.encoding != "json"
                {
                    return Err("unknown or invalid checked phase artifact".into());
                }
            } else if legacy_declares_checked(root, artifact)? {
                return Err("checked telemetry is missing its measurement policy".into());
            }
        }
    }
    Ok(())
}

pub(crate) fn restricted_record(
    root: &Path,
    path: &str,
) -> Result<(&'static str, serde_json::Value), String> {
    if !policy_present(root)? {
        return Err("v2 phase record lacks fixed policy".into());
    }
    let roles = validate_directory(root)?;
    if !roles.contains_key(path) {
        return Err("v2 phase record outside validated prefix".into());
    }
    let bytes = read(root, path, byte_limit(path))?;
    let raw = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let kind = if path.ends_with("-start.json") {
        let end = path.replace("-start.json", ".json");
        if roles.contains_key(&end) {
            "native_phase_record"
        } else {
            "interrupted_or_unfinished"
        }
    } else {
        "native_phase_record"
    };
    Ok((kind, raw))
}

trait Counter {
    fn start(&mut self) -> bool;
    fn finish(self) -> Result<Collection, String>;
}
impl Counter for CheckedCounters {
    fn start(&mut self) -> bool {
        CheckedCounters::start(self)
    }
    fn finish(self) -> Result<Collection, String> {
        let bytes = encode(&CheckedCounters::finish(self), METRICS_MAX)?;
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())
    }
}

pub(crate) fn measured<T>(
    out: &Path,
    registry: &mut Vec<Artifact>,
    phase: &str,
    op: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    let binding = AttemptBinding::from_cell_files(out)?;
    let phase = Phase::ALL
        .into_iter()
        .find(|p| p.name() == phase)
        .ok_or("unknown checked phase")?;
    #[cfg(test)]
    if tests::mock_factory_present() {
        return measured_with(
            out,
            registry,
            &binding,
            phase,
            || tests::open_mock(out, phase),
            op,
            |path, role, bytes| storage::write_bytes(out, path, role, "json", bytes),
        );
    }
    measured_with(
        out,
        registry,
        &binding,
        phase,
        || {
            CheckedCounters::open(CounterPolicy {
                scope: TaskScope::CallingThread,
                cycles: CyclesPolicy::OptionalWithStandaloneFallback,
            })
        },
        op,
        |path, role, bytes| storage::write_bytes(out, path, role, "json", bytes),
    )
}

fn measured_with<T, C: Counter>(
    out: &Path,
    registry: &mut Vec<Artifact>,
    binding: &AttemptBinding,
    phase: Phase,
    open: impl FnOnce() -> C,
    op: impl FnOnce() -> Result<T, String>,
    mut persist: impl FnMut(&str, &str, &[u8]) -> Result<Artifact, String>,
) -> Result<T, String> {
    let mut files = context_files(out)?;
    validate_context(binding, &files)?;
    if files.contains_key(&phase.path(true)) {
        return Err("checked phase cannot repeat".into());
    }
    let usage = || -> Result<Option<Usage>, String> {
        super::usage()
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| e.to_string())
    };
    let start = Start {
        schema: "crypto.ml-dataset.phase-start/v2".into(),
        phase,
        ordinal: phase.ordinal(),
        binding: binding.clone(),
        policy_sha256: POLICY_HASH.into(),
        recording_pid: std::process::id(),
        usage_before: usage()?,
        counter_boundary: "durable_start_before_open".into(),
    };
    let start_bytes = encode(&start, START_MAX)?;
    files.insert(phase.path(true), start_bytes.clone());
    validate_context(binding, &files)?;
    registry.push(persist(&phase.path(true), "phase-start", &start_bytes)?);
    let mut counter = open();
    let now = Instant::now();
    let counter_start_succeeded = counter.start();
    let result = op();
    // Preserve the original result while obtaining and persisting diagnostics.
    let finish = (|| {
        let collection = counter.finish()?;
        let elapsed_ns_diagnostic = now.elapsed().as_nanos().to_string();
        let (_, observations) = native::validate(&collection, start.recording_pid)?;
        let m = Metrics {
            schema: "crypto.ml-dataset.phase-metrics/v2".into(),
            phase,
            ordinal: phase.ordinal(),
            binding: binding.clone(),
            policy_sha256: POLICY_HASH.into(),
            start_sha256: digest(&start_bytes),
            returned_ok: result.is_ok(),
            returned_error: result.as_ref().err().map(summarize).transpose()?,
            counter_start_succeeded,
            collection,
            observations,
            usage_after: usage()?,
            elapsed_ns_diagnostic,
            training_cost_eligible: false,
            whole_worker_retired_instructions: None,
            whole_worker_coverage_verified: false,
            independently_verified: false,
            platform_admitted: false,
            clock_valid_for_comparison: false,
            interrupted_phase_readback: false,
            consumer_v2_admitted: false,
        };
        validate_metrics(&m, &start, &start_bytes)?;
        let bytes = encode(&m, METRICS_MAX)?;
        persist(
            &phase.path(false),
            "hardware-and-resource-telemetry",
            &bytes,
        )
    })();
    match (result, finish) {
        (result, Ok(artifact)) => {
            registry.push(artifact);
            result
        }
        (Ok(_), Err(persistence)) => {
            Err(format!("phase metrics persistence failed: {persistence}"))
        }
        (Err(operation), Err(persistence)) => Err(format!(
            "operation failed: {operation}; phase metrics persistence failed: {persistence}"
        )),
    }
}

#[cfg(test)]
pub(crate) mod tests;
