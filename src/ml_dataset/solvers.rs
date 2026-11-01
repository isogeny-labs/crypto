//! Native, bounded dataset adapters. No adapter reads the private witness.
//!
//! The finite-domain encodings below preserve the actual factor base, including
//! projected abscissae. A polynomial root is accepted only after a group replay.
//! Native operation counters retain their units; none is an instruction count.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Worker-only adapter: verified originals, bound capture, original execution
// context, and central semantic replay gate the native profile.
#[path = "binary_m3_original.rs"]
mod binary_m3_original;

#[path = "koblitz_m3.rs"]
mod koblitz_m3;

// Ordinary finite-selector construction; only original-bound supervised dispatch.
#[path = "binary_m3_resultant.rs"]
mod binary_m3_resultant;
pub(in crate::ml_dataset) use koblitz_m3::AdmittedKoblitzNative;

#[path = "prime_f4_wide.rs"]
mod prime_f4_wide;
#[path = "prime_f4_wide_dynamic.rs"]
mod prime_f4_wide_dynamic;
#[path = "prime_f4_wide_dynamic_replay.rs"]
mod prime_f4_wide_dynamic_replay;
#[path = "prime_f4_wide_m3.rs"]
mod prime_f4_wide_m3;

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

use crate::binary_ecc::{curve as binary, BinaryCurve, BinaryPoint, F2mElement};
use crate::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crate::cryptanalysis::f6_wide_geometry::F6WidePairIndex;
use crate::cryptanalysis::koblitz_groebner::{self as boolean, FieldStructure, SymElement};
use crate::cryptanalysis::matrix_f5_f2::matrix_f5_f2;
use crate::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly, Mask};
use crate::cryptanalysis::prime_root_trace::{self, RootEvent, RootLimits, RootReport, RootStop};
use crate::cryptanalysis::sat::{self, SolveResult};
use crate::ecc::Point;

use super::capture::{AttemptBinding, CaptureEmitter, CaptureLimits};
use super::curves;
use super::types::{ArtifactPayload, CaseConfig, CaseData, SolverData};

#[path = "binary_roots.rs"]
mod binary_roots;
pub(super) const BINARY_ROOT_CERTIFICATE_SCOPE: &str = binary_roots::CERTIFICATE_SCOPE;

const TRACE_EVENT_CAP: u64 = 100_000;
const TRACE_BYTE_CAP: usize = 64 * 1024 * 1024;
pub(super) const PRIME_ROOT_CERTIFICATE_SCOPE: &str =
    "distinct roots per completed prime S3 call; incomplete calls prove no root absence";
const PRIME_ROOT_EVENT_BYTE_CAP: usize = TRACE_BYTE_CAP / 4;
const PRIME_ROOT_BYTE_CAP_REASON: &str = "prime root event byte cap reached";

const SAT_EVENT_BYTE_CAP: usize = TRACE_BYTE_CAP / 4;
const SAT_EVENT_CAP_REASON: &str = "native SAT event trace budget exhausted";

#[derive(Default)]
struct SatEventLog {
    bytes: Vec<u8>,
    events: u64,
}
impl SatEventLog {
    fn append(
        &mut self,
        pattern: u32,
        event: &sat::SolverEvent,
        byte_cap: usize,
        event_cap: u64,
    ) -> Result<(), String> {
        let row = json!({"schema":"ml-dataset.sat-event/v1","pattern":pattern,"sequence":self.events+1,"native":event});
        let bytes = serde_json::to_vec(&row).map_err(|e| e.to_string())?;
        if self.events >= event_cap || self.bytes.len().saturating_add(bytes.len() + 1) > byte_cap {
            return Err(SAT_EVENT_CAP_REASON.into());
        }
        // Write before advancing the in-memory prefix; interruption preserves
        // every successful event. A sink error is never swallowed by SAT.
        live_event(&row)?;
        self.bytes.extend_from_slice(&bytes);
        self.bytes.push(b'\n');
        self.events += 1;
        Ok(())
    }
}

struct LiveSink {
    directory: PathBuf,
    events: File,
    sequence: u64,
}
#[allow(clippy::large_enum_variant)] // Keep the bounded sink inline without an additional heap allocation.
enum EventSink {
    Legacy(LiveSink),
    Bound(CaptureEmitter),
}
impl EventSink {
    fn event(&mut self, event: &Value) -> Result<(), String> {
        match self {
            Self::Legacy(s) => s.event(event),
            Self::Bound(s) => s.event(event).map_err(|e| e.to_string()),
        }
    }
    fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
        match self {
            Self::Legacy(s) => s.payload(payload),
            Self::Bound(s) => s.payload(payload).map_err(|e| e.to_string()),
        }
    }
}
thread_local! { static LIVE_SINK:RefCell<Option<EventSink>>=const { RefCell::new(None) }; }

#[cfg(test)]
thread_local! { static FAIL_PRIME_LIFT_EVENT:std::cell::Cell<bool>=const { std::cell::Cell::new(false) }; }
impl LiveSink {
    fn event(&mut self, event: &Value) -> Result<(), String> {
        #[cfg(test)]
        if event["event"]["kind"] == "group_lift"
            && FAIL_PRIME_LIFT_EVENT.with(|flag| flag.replace(false))
        {
            // Actual write failure on this event stream; other artifact paths
            // remain writable, as in the regression's canonical worker writer.
            self.events = File::open(self.directory.join("events.jsonl"))
                .map_err(|e| format!("inject readonly event stream: {e}"))?;
        }
        self.sequence += 1;
        let bytes = serde_json::to_vec(&json!({"sequence":self.sequence,"event":event}))
            .map_err(|e| e.to_string())?;
        self.events
            .write_all(&bytes)
            .and_then(|_| self.events.write_all(b"\n"))
            .and_then(|_| self.events.sync_data())
            .map_err(|e| format!("persist live solver event: {e}"))
    }
    fn payload(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
        if payload.path.contains('/') || payload.path == "." || payload.path == ".." {
            return Err("invalid live payload basename".into());
        }
        let path = self.directory.join("payloads").join(&payload.path);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("live payload path is a symlink".into());
        }
        if path.exists() {
            return if fs::read(&path).map_err(|e| e.to_string())? == payload.bytes {
                Ok(())
            } else {
                Err("live payload path collision with different bytes".into())
            };
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("create live payload {}: {e}", path.display()))?;
        file.write_all(&payload.bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("persist live payload {}: {e}", path.display()))?;
        File::open(path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|e| format!("sync live payload directory: {e}"))?;
        self.event(&json!({"kind":"artifact_persisted","path":format!("payloads/{}",payload.path),"role":payload.role,"bytes":payload.bytes.len()}))
    }
}
pub(super) fn live_event(event: &Value) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(s) => s.event(event),
        None => Ok(()),
    })
}
/// New bounded profiles require an attempt-bound sink. The caller debits the
/// buffer's requested storage once and reuses it; this bridge does not reset
/// budgets or silently downgrade to the legacy allocating event path.
pub(super) fn live_typed_event<T: serde::Serialize + ?Sized>(
    kind: super::capture::TypedEventKind,
    payload: &T,
    buffer: &mut super::capture::TypedEventBuffer,
) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(sink)) => sink
            .event_typed(kind, payload, buffer)
            .map_err(|error| error.to_string()),
        Some(EventSink::Legacy(_)) => {
            Err("typed solver events require an attempt-bound capture sink".into())
        }
        None => Err("typed solver events require an active capture sink".into()),
    })
}
/// Dedicated witness preparation/publication never downgrades to Legacy.
/// The caller supplies and reconciles remaining original checking/byte budgets;
/// `global_acknowledged` is the SAME adapter-wide4096 counter, not a new pool.
pub(super) fn prepare_bound_witness(
    budget: &mut super::capture::WitnessBudget,
) -> Result<super::capture::WitnessCaptureWorkspace, super::capture::WitnessError> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(sink)) => sink.prepare_witness(budget),
        _ => Err(super::capture::WitnessError::Unbound),
    })
}
pub(super) fn publish_bound_witness(
    bytes: &[u8],
    workspace: &mut super::capture::WitnessCaptureWorkspace,
    budget: &mut super::capture::WitnessBudget,
    global_acknowledged: &mut usize,
) -> Result<super::capture::WitnessReceipt, super::capture::WitnessError> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(sink)) => {
            sink.publish_first_witness(bytes, workspace, budget, global_acknowledged)
        }
        _ => Err(super::capture::WitnessError::Unbound),
    })
}

/// Borrowed complete artifacts from early dynamic failures use the same bound
/// outer custody path. Dedicated witness publication is never downgraded here.
pub(super) fn live_borrowed_payload(
    path: &str,
    role: &str,
    encoding: &str,
    bytes: &[u8],
) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(sink)) => sink
            .payload_borrowed(path, role, encoding, bytes)
            .map_err(|error| error.to_string()),
        _ => Err("borrowed retained artifacts require an attempt-bound capture sink".into()),
    })
}

pub(super) fn live_payload(payload: &ArtifactPayload) -> Result<(), String> {
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(s) => s.payload(payload),
        None => Ok(()),
    })
}
struct LiveSinkGuard;
impl Drop for LiveSinkGuard {
    fn drop(&mut self) {
        LIVE_SINK.with(|slot| {
            slot.borrow_mut().take();
        });
    }
}

/// Persist each exposed event and complete artifact before proceeding. The
/// outer worker retains this directory even when a native call is killed.
/// Native engines' currently unexposed internal states remain explicit gaps.
/// A worker-owned proof of completed original verification and storage.
/// It borrows public values only; it never retains the private witness.
pub(super) struct VerifiedWorkerOriginal<'a> {
    config: &'a CaseConfig,
    public: &'a Value,
    factor_base: &'a Value,
    root: &'a Path,
    binding: Option<&'a AttemptBinding>,
    public_sha256: [u8; 32],
    factor_base_sha256: [u8; 32],
    public_bytes: u64,
    factor_base_bytes: u64,
}
impl VerifiedWorkerOriginal<'_> {
    fn matches_bound(
        &self,
        config: &CaseConfig,
        case: &CaseData,
        directory: &Path,
        binding: &AttemptBinding,
    ) -> Result<(), String> {
        if !std::ptr::eq(self.config, config)
            || !std::ptr::eq(self.public, &case.public_instance)
            || !std::ptr::eq(self.factor_base, &case.factor_base)
            || directory.parent() != Some(self.root)
            || directory.file_name().is_none_or(|name| name != "live")
            || self.binding != Some(binding)
        {
            return Err("verified worker originals differ from bound solver invocation".into());
        }
        if AttemptBinding::from_cell_files(self.root)? != *binding {
            return Err("verified worker original context changed before dispatch".into());
        }
        for (path, sha256, bytes) in [
            ("public.json", &self.public_sha256, self.public_bytes),
            (
                "factor-base.json",
                &self.factor_base_sha256,
                self.factor_base_bytes,
            ),
        ] {
            super::storage::check_file(
                &super::storage::contained_path(self.root, path, false)?,
                &hex::encode(sha256),
                bytes,
            )?;
        }
        Ok(())
    }
}

/// This replaces, rather than repeats, the worker's existing verification and
/// original-file writes. Their telemetry remains separate from solver counters.
pub(super) fn verify_and_store_worker_originals<'a>(
    config: &'a CaseConfig,
    case: &'a CaseData,
    root: &'a Path,
    binding: Option<&'a AttemptBinding>,
    registry: &mut Vec<super::types::Artifact>,
) -> Result<VerifiedWorkerOriginal<'a>, String> {
    if let Some(binding) = binding {
        if AttemptBinding::from_cell_files(root)? != *binding
            || super::read_json::<CaseConfig>(&root.join("config.json"))? != *config
        {
            return Err("worker originals differ from admitted attempt/configuration".into());
        }
    }
    super::telemetry::measured(root, registry, "known-answer-replay", || {
        curves::verify(&case.public_instance, &case.private_witness)
    })?;
    super::telemetry::measured(root, registry, "factor-base-replay", || {
        curves::verify_factor_base_config(config, &case.public_instance, &case.factor_base)
    })?;
    let public = super::storage::write_json(
        root,
        "public.json",
        "public-instance",
        &case.public_instance,
    )?;
    let mut public_sha256 = [0; 32];
    hex::decode_to_slice(&public.sha256, &mut public_sha256).map_err(|e| e.to_string())?;
    let public_bytes = public.bytes;
    registry.push(public);
    registry.push(super::storage::write_json(
        root,
        "witness.json",
        "restricted-known-answer",
        &case.private_witness,
    )?);
    let factor_base =
        super::storage::write_json(root, "factor-base.json", "factor-base", &case.factor_base)?;
    let mut factor_base_sha256 = [0; 32];
    hex::decode_to_slice(&factor_base.sha256, &mut factor_base_sha256)
        .map_err(|e| e.to_string())?;
    let factor_base_bytes = factor_base.bytes;
    registry.push(factor_base);
    Ok(VerifiedWorkerOriginal {
        config,
        public: &case.public_instance,
        factor_base: &case.factor_base,
        root,
        binding,
        public_sha256,
        factor_base_sha256,
        public_bytes,
        factor_base_bytes,
    })
}

/// Retained, unacknowledged bytes and rejected actual receipts. This transport
/// performs no storage retry and makes no durable-publication claim.
#[derive(Default)]
pub(super) struct WorkerResidualCustody {
    items: Vec<WorkerResidualArtifact>,
}
pub(super) struct WorkerResidualArtifact {
    path: &'static str,
    bytes: Vec<u8>,
    untransferred_root: Option<super::types::Artifact>,
}
impl WorkerResidualCustody {
    const CAPACITY: usize = 10;
    fn admit(charge: impl FnOnce(u64, u64) -> Result<(), String>) -> Result<Self, String> {
        charge(
            4096,
            (Self::CAPACITY * std::mem::size_of::<WorkerResidualArtifact>()) as u64,
        )?;
        let mut items = Vec::new();
        items
            .try_reserve_exact(Self::CAPACITY)
            .map_err(|e| e.to_string())?;
        Ok(Self { items })
    }
    fn retain(&mut self, item: WorkerResidualArtifact) -> Result<(), WorkerResidualArtifact> {
        if self.items.len() == Self::CAPACITY || self.items.capacity() < Self::CAPACITY {
            return Err(item);
        }
        self.items.push(item);
        Ok(())
    }
    pub(super) fn has_untransferred_roots(&self) -> bool {
        self.items.iter().any(|a| a.untransferred_root.is_some())
    }
}

/// Private worker transport: acknowledged roots never re-enter payload writers.
pub(super) struct WorkerSolveOutput {
    data: SolverData,
    stored: Vec<super::types::Artifact>,
    residual: WorkerResidualCustody,
}
impl WorkerSolveOutput {
    pub(super) fn legacy(data: SolverData) -> Self {
        Self {
            data,
            stored: Vec::new(),
            residual: WorkerResidualCustody::default(),
        }
    }
    pub(super) fn into_parts(
        self,
    ) -> (
        SolverData,
        Vec<super::types::Artifact>,
        WorkerResidualCustody,
    ) {
        (self.data, self.stored, self.residual)
    }
}

pub(super) const WORKER_STORED_RECEIPT_MAX: usize = 32;

/// Reserved before producer execution; the caller charges its original budget.
/// Root Artifact strings are moved from storage acknowledgements, not cloned.
/// This admission prices the transport Vec and bounded identity comparisons;
/// worker setup/finalization retain their separately stated measurement scope.
#[allow(dead_code)] // Consumed by the staged native orchestration integration.
struct WorkerStoredReceipts<'a> {
    original: &'a VerifiedWorkerOriginal<'a>,
    records: Vec<super::types::Artifact>,
    capacity: usize,
}
#[allow(dead_code)]
impl<'a> WorkerStoredReceipts<'a> {
    fn admit(
        original: &'a VerifiedWorkerOriginal<'a>,
        capacity: usize,
        charge: impl FnOnce(u64, u64) -> Result<(), String>,
    ) -> Result<Self, String> {
        if original.binding.is_none() || capacity == 0 || capacity > WORKER_STORED_RECEIPT_MAX {
            return Err(
                "stored worker receipts require bound original and bounded capacity".into(),
            );
        }
        let bytes = (capacity * std::mem::size_of::<super::types::Artifact>()) as u64;
        let work = (capacity * capacity * 1024) as u64;
        charge(work, bytes)?;
        let mut records = Vec::new();
        records
            .try_reserve_exact(capacity)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            original,
            records,
            capacity,
        })
    }
    fn push(
        &mut self,
        artifact: super::types::Artifact,
    ) -> Result<(), (String, super::types::Artifact)> {
        if self.records.len() == self.capacity
            || artifact.path.len() > 1024
            || super::worker_reserved_artifact_path(&artifact.path)
            || self.records.iter().any(|a| a.path == artifact.path)
        {
            return Err((
                "stored worker receipt exceeds admission or collides".into(),
                artifact,
            ));
        }
        self.records.push(artifact);
        Ok(())
    }
    fn finish(self, data: SolverData) -> WorkerSolveOutput {
        // Holding original until handoff keeps the original worker lifetime.
        let _ = self.original;
        WorkerSolveOutput {
            data,
            stored: self.records,
            residual: WorkerResidualCustody::default(),
        }
    }
}

/// Dispatch only the initial native two-code/four-signed-point profile.
/// Original worker verification has already authenticated these public values.
/// This bounded shape check allocates no curve, domain, or payload buffers.
fn koblitz_worker_profile(config: &CaseConfig, case: &CaseData) -> bool {
    binary_m3_worker_shape(config, case, "koblitz")
}
fn ordinary_worker_profile(config: &CaseConfig, case: &CaseData) -> bool {
    binary_m3_worker_shape(config, case, "binary")
}
fn binary_m3_worker_shape(config: &CaseConfig, case: &CaseData, family: &str) -> bool {
    if config.family != family
        || config.m != 3
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
        || !matches!(config.solver.as_str(), "f4" | "f5")
        || config.factor_base_size != 4
        || !(1..=6).contains(&config.degree_bound)
        || config.work_limit == 0
    {
        return false;
    }
    let Some(points) = case.factor_base["points"]
        .as_array()
        .filter(|points| points.len() == 4)
    else {
        return false;
    };
    let mut signed = [(0u64, 0u64); 4];
    for (row, output) in points.iter().zip(&mut signed) {
        let parse = |name: &str| {
            row[name]
                .as_str()
                .filter(|s| s.len() <= 20)
                .and_then(|s| s.parse::<u64>().ok())
        };
        let (Some(x), Some(y)) = (parse("x"), parse("y")) else {
            return false;
        };
        if x >= (1u64 << config.n) || y >= (1u64 << config.n) {
            return false;
        }
        *output = (x, y);
    }
    if signed.iter().enumerate().any(|(i, point)| {
        signed[..i].contains(point) || !signed.contains(&(point.0, point.0 ^ point.1))
    }) {
        return false;
    }
    signed
        .iter()
        .enumerate()
        .filter(|(i, point)| !signed[..*i].iter().any(|prior| prior.0 == point.0))
        .count()
        == 2
}

/// Native causal projection seam for the separately owned exporter. Currently
/// implements independently replayed signed-group prefixes only.
pub(super) fn is_koblitz_m3_native_source(
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> bool {
    koblitz_m3::is_native_source(outcome, artifacts)
}

pub(super) fn project_koblitz_m3_prefixes(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<Value, String> {
    koblitz_m3::project_worker_prefixes(dir, config, outcome, artifacts, emit)
}

pub(super) fn is_ordinary_m3_native_source(
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> bool {
    outcome["strategy"] == "ordinary-binary-profile-unsupported"
        || koblitz_m3::ordinary_construction::persisted::native_ownership::replay::is_source(
            outcome, artifacts,
        )
}
pub(super) fn project_ordinary_m3_prefixes(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<Value, String> {
    koblitz_m3::project_ordinary_worker_prefixes(dir, config, outcome, artifacts, emit)
}

/// Always invoked by central replay, independently of mutable outcome strategy.
pub(super) fn verify_koblitz_m3_cell(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> Result<Option<Value>, String> {
    koblitz_m3::verify_worker_cell(dir, config, outcome, artifacts)
}

/// Internal worker path; unsupervised callers cannot obtain this token by
/// setting an outcome flag. Adapter consumption is integrated separately.
pub(super) fn solve_with_verified_worker_originals(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
    binding: AttemptBinding,
    limits: CaptureLimits,
    context: Option<&super::execution::SolverExecutionContext>,
    original: &VerifiedWorkerOriginal<'_>,
) -> Result<WorkerSolveOutput, String> {
    original.matches_bound(config, case, directory, &binding)?;
    if context.is_some_and(|c| c.launch.binding != binding) {
        return Err("verified worker originals differ from execution attempt".into());
    }
    if config.family == "binary" && config.m == 3 && matches!(config.solver.as_str(), "f4" | "f5") {
        if !ordinary_worker_profile(config, case) {
            return Ok(WorkerSolveOutput::legacy(SolverData {
                outcome: json!({"strategy":"ordinary-binary-profile-unsupported","status":"unsupported",
                    "reason":"requires original n23..31 or n47..61, both twists, m3, degree1..6 and four signed points/two x codes",
                    "solver_completed":false,"scalar_recovered_without_witness":false}),
                artifacts: Vec::new(),
            }));
        }
        let context = context.ok_or("ordinary m3 requires original execution context")?;
        if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
            return Err("nested live solver sink".into());
        }
        let emitter = CaptureEmitter::create(directory, binding.clone(), limits.clone())?;
        LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(emitter)));
        let _guard = LiveSinkGuard;
        return koblitz_m3::ordinary_construction::persisted::native_ownership::worker::run(
            original, case, context, &binding, &limits,
        );
    }
    if koblitz_worker_profile(config, case) {
        let context = context.ok_or("Koblitz m3 worker requires original execution context")?;
        if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
            return Err("nested live solver sink".into());
        }
        let emitter = CaptureEmitter::create(directory, binding.clone(), limits.clone())?;
        LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(emitter)));
        let _guard = LiveSinkGuard;
        // The entry owns the original ledger, first capture event, retained-root
        // handoff and capture finalization. Do not run generic wrapper effects.
        return koblitz_m3::run_verified_worker_entry(original, context, &binding, &limits);
    }
    solve_with_bound_event_sink_context(config, case, directory, binding, limits, context)
        .map(WorkerSolveOutput::legacy)
}

pub(super) fn verify_ordinary_m3_cell(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> Result<Option<Value>, String> {
    koblitz_m3::ordinary_construction::persisted::native_ownership::replay::verify(
        dir, config, outcome, artifacts,
    )
}

pub fn solve_with_event_sink(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
) -> Result<SolverData, String> {
    solve_with_event_sink_context(config, case, directory, None)
}

pub(super) fn solve_with_event_sink_context(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
    context: Option<&super::execution::SolverExecutionContext>,
) -> Result<SolverData, String> {
    if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
        return Err("nested live solver sink".into());
    }
    fs::create_dir_all(directory.join("payloads")).map_err(|e| e.to_string())?;
    let events = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join("events.jsonl"))
        .map_err(|e| format!("create exclusive solver event stream: {e}"))?;
    File::open(directory)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    if let Some(parent) = directory.parent() {
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
    }
    LIVE_SINK.with(|slot| {
        *slot.borrow_mut() = Some(EventSink::Legacy(LiveSink {
            directory: directory.to_owned(),
            events,
            sequence: 0,
        }))
    });
    let _guard = LiveSinkGuard;
    let begin = live_event(
        &json!({"kind":"solver_begin","solver":config.solver,"family":config.family,"n":config.n,"m":config.m,"work_limit":config.work_limit,"degree_bound":config.degree_bound,"format":"ml-dataset.live-events/v1","sync_policy":"fsync every emitted event and complete payload"}),
    );
    if let Err(error) = begin {
        return wide_begin_failure(config, case, context, error);
    }
    let mut result = solve_with_context(config, case, context);
    match &mut result {
        Ok(data) if retains_canonical_artifacts(data) => {
            flush_prime_chain_payloads(data);
            let terminal_write = if data.outcome.get("live_persistence_errors").is_none() {
                live_event(&json!({"kind":"solver_return","outcome":data.outcome,"error":null}))
            } else {
                Ok(())
            };
            if let Err(error) = terminal_write {
                record_prime_chain_persistence_error(data, "solver_return", &error);
            }
            // Canonical worker output is a separate persistence attempt. Keep
            // verified mathematics and staged artifacts even if live tracing
            // failed; the explicit error prevents a durable-prefix claim.
            return result;
        }
        _ => {}
    }
    if let Ok(data) = &result {
        for payload in &data.artifacts {
            live_payload(payload)?;
        }
    }
    live_event(
        &json!({"kind":"solver_return","outcome":result.as_ref().map(|r|&r.outcome).ok(),"error":result.as_ref().err()}),
    )?;
    result
}

/// Bound, capped event capture for supervised dataset attempts. The emitter
/// commits its initial watermark before the first native call. Its artifacts
/// can be copied by the supervisor while a native call is still running, but
/// they do not contain a restorable native solver state.
pub fn solve_with_bound_event_sink(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
    binding: AttemptBinding,
    limits: CaptureLimits,
) -> Result<SolverData, String> {
    solve_with_bound_event_sink_context(config, case, directory, binding, limits, None)
}

pub(super) fn solve_with_bound_event_sink_context(
    config: &CaseConfig,
    case: &CaseData,
    directory: &Path,
    binding: AttemptBinding,
    limits: CaptureLimits,
    context: Option<&super::execution::SolverExecutionContext>,
) -> Result<SolverData, String> {
    if LIVE_SINK.with(|slot| slot.borrow().is_some()) {
        return Err("nested live solver sink".into());
    }
    let emitter = CaptureEmitter::create(directory, binding, limits)?;
    LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(emitter)));
    let _guard = LiveSinkGuard;
    let begin = live_event(&json!({"kind":"solver_begin","solver":config.solver,
        "family":config.family,"n":config.n,"m":config.m,
        "work_limit":config.work_limit,"degree_bound":config.degree_bound,
        "format":"crypto.ml-dataset.live-event/v2",
        "sync_policy":"event fsync followed by atomic committed watermark",
        "solver_state_restorable":false}));
    if let Err(error) = begin {
        return wide_begin_failure(config, case, context, error);
    }
    let mut result = solve_with_context(config, case, context);
    if let Ok(data) = &mut result {
        if retains_canonical_artifacts(data) {
            flush_prime_chain_payloads(data);
            if data.outcome.get("live_persistence_errors").is_none() {
                if let Err(error) =
                    live_event(&json!({"kind":"solver_return","outcome":data.outcome,"error":null}))
                {
                    record_prime_chain_persistence_error(data, "solver_return", &error);
                }
            }
            let finished = LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
                Some(EventSink::Bound(s)) => s.finish().map_err(|e| e.to_string()),
                _ => Err("bound capture sink disappeared".into()),
            });
            if let Err(error) = finished {
                record_prime_chain_persistence_error(data, "capture_finish", &error);
            }
            return result;
        }
    }
    if let Ok(data) = &result {
        for payload in &data.artifacts {
            live_payload(payload)?;
        }
    }
    live_event(&json!({"kind":"solver_return",
        "outcome":result.as_ref().map(|r| &r.outcome).ok(),
        "error":result.as_ref().err()}))?;
    LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
        Some(EventSink::Bound(s)) => s.finish().map_err(|e| e.to_string()),
        _ => Err("bound capture sink disappeared".into()),
    })?;
    result
}

pub fn capabilities() -> Value {
    let ordinary_m3_profile = json!({"adapter_id":"binary-m3-native-v1","family":"binary","m":[3,3],"n_ranges_inclusive":[[23,31],[47,61]],"twists":[false,true],"degree_bound_inclusive":[1,6],"factor_base_size":4,"signed_domain_points":4,"distinct_x_codes":2,"requires_sign_closed_factor_base":true,"requires_distinct_signed_points":true,"requires_positive_work_limit":true,"requires_verified_worker_originals":true,"requires_original_execution_context":true,"requires_bound_capture":true,"curve":"authenticated original a, nonzero b, modulus and twist; no Koblitz substitution","construction":"finite-selector determinant/coordinate ANF with independent replay","extraction":"bounded ordered signed triples including infinity prefixes","replay":"original equations, actual native/signed committed prefix, group witness and receipt delivery","costs":"one original cumulative ledger; modeled reservations and actual group-addition calls","scope":"all listed field degrees and both twists are eligible; original policy/domain and resource refusals are retained","gaps":["independent matrix basis derivation","retired hardware instructions","first and last fall degree certification","larger factor bases","m greater than 3"]});
    let koblitz_m3_profile = json!({"adapter_id":"koblitz-m3-native-v1","family":"koblitz","m":[3,3],"n_ranges_inclusive":[[23,31],[47,61]],"degree_bound_inclusive":[1,6],"factor_base_size":4,"signed_domain_points":4,"distinct_x_codes":2,"requires_sign_closed_factor_base":true,"requires_positive_work_limit":true,"requires_original_execution_context":true,"requires_bound_capture":true,"twists":[false,true],"extraction":"bounded ordered signed triples including infinity prefixes","replay":"original input, native boundary consistency, signed group prefix and canonical delivery","costs":"modeled work and memory reservations; actual group addition calls","gaps":["independent matrix basis derivation","retired hardware instructions","first and last fall degree certification","larger factor bases","generic binary curves","m greater than 3"],"requires_verified_worker_originals":true,"requires_distinct_signed_points":true,"scope":"dispatch eligibility; completion remains subject to original resource and evidence gates"});
    json!({
        "schema":"ml-dataset.capabilities/v1",
        "solvers":[
            {"id":"f4-wide-dynamic-v1","families":["prime"],"n":[33,61],"m":[4,15],"degree_bound":12,"requires_original_execution_context":true,"requires_bound_capture":true,"requires_positive_work_limit":true,"requires_positive_original_memory_trace":true,"requires_whole_mib_trace_limit":true,"minimum_signed_domain_points":1,"minimum_x_codes":1,"selection":"explicit f4-wide-dynamic-v1 solver id; not selected automatically by f4","maximum_signed_domain_points":64,"maximum_x_codes":32,"kind":"bounded native dynamic prime F4 and independently replayed finite-domain PDP evidence","trace":"original equations, native boundary reports, linear transitions, group prefixes and durable witnesses","unsupported_cells":"retained with explicit status","gaps":["independent native basis derivation","full instruction trajectory"]},
            {"id":"enumerate","families":["koblitz","binary","prime"],"m":[2,15],"kind":"bounded exact signed finite-domain PDP","trace":"all visited group transitions"},
            {"id":"bsgs","families":["koblitz","binary","prime"],"m":[2,15],"kind":"independent ECDLP baseline","trace":"baby table and giant probes"},
            {"id":"sat","families":["koblitz","binary"],"m":[2,15],"kind":"native chained S3 CNF/XOR subject to preflight caps","trace":"bounded native decisions, assignments and copied reasons, conflicts, analysis reasons, learned/deleted clauses, backjumps, restarts, XOR row operations; original formulas and checked models","gaps":["independently checked UNSAT proof","internal watch/heap operations and minimization derivations","full gate semantic map"]},
            {"id":"weil","families":["koblitz","binary"],"m":[2,2],"kind":"exact finite-domain selector Weil restriction","trace":"complete ANF equations and coordinate lift map","solver_completed":false},
            {"id":"f4","families":["koblitz","binary","prime"],"m":[2,15],"m_by_family":{"koblitz":[2,2],"binary":[2,2],"prime":[2,15]},"prime_modulus_limit_exclusive":"4294967296","supervised_wide_prime_profile":{"m":[2,2],"modulus_min_inclusive":"4294967296","modulus_max_inclusive":"2305843009213693951","requires_original_execution_context":true},"supervised_wide_prime_m3_profile":{"m":[3,3],"modulus_min_inclusive":"4294967296","modulus_max_inclusive":"2305843009213693951","requires_original_execution_context":true,"extraction":"bounded ordered signed triples including infinity prefixes"},"m_scope":"base routes; additional supervised profiles are separately gated","supervised_koblitz_m3_profile":koblitz_m3_profile.clone(),"supervised_ordinary_binary_m3_profile":ordinary_m3_profile.clone(),"kind":"native Boolean matrix F4 plus splitting; prime m2 finite-domain reference; prime m3..15 affine S3-chain traced visitor","trace":"original systems also retained for unsupported wide primes; bounded recursive systems, branch assignments, native bases and useful-row schedules, candidate and independent group-lift events","gaps":["wide-prime elimination under solver id f4 requires supervised context and m2 or m3; m4..15 uses separate f4-wide-dynamic-v1","affine-chain exceptional infinity branches prevent general negative PDP conclusions","private pivot operations and zero rows","internal univariate factorization/GCD states","complete critical-pair stream"]},
            {"id":"f5","families":["koblitz","binary"],"m":[2,2],"m_scope":"base routes; additional supervised profiles are separately gated","supervised_koblitz_m3_profile":koblitz_m3_profile.clone(),"supervised_ordinary_binary_m3_profile":ordinary_m3_profile.clone(),"kind":"native Boolean matrix F5 row criterion plus splitting; not general incremental F5","trace":"original ANF, bounded reduced rows, F5 report, splitting nodes","gaps":["per-row signatures and rejected signature witnesses"]},
            {"id":"f6","families":["koblitz","binary"],"m":[4,4],"kind":"native F6WidePairIndex geometric four-sum; not a general Groebner algorithm","trace":"pair sums, domain, verified indices","gaps":["native internal lookup sequence"]},
            {"id":"root","families":["koblitz","binary","prime"],"m":[2,2],"kind":"native S3 univariate root finding with exact domain and group lift","trace":"prime bounded field, power, GCD and split states; binary bounded quadratic, trace, half-trace and matrix states; independent distinct-root certificates","gaps":["higher-m root elimination","complete execution and total cost verification"]},
            {"id":"pkm","families":["prime"],"m":[2,3],"modulus_limit_exclusive":"2147483648","kind":"native PKM Kummer-tower F4 with finite-grid extraction","trace":"exact alternative Kummer base, quotient rules, prepared and specialized equations, native F4 report, extracted roots","gaps":["private elimination pivots and row operations","m>3 or p>=2^31"]},
            {"id":"double-large-prime","families":["koblitz","binary","prime"],"m":[2,15],"kind":"native verified two-residual elimination with signed Frobenius for admitted Koblitz models or identity/sign action with generic sparse filtering; recovery of determined target columns","trace":"residual discovery, exact domains, partial candidates, source coefficients, native sparse filtering and modular rank/recovery certificates","gaps":["internal pivot-selection events","complete private filter mutation stream"]}
        ],
        "global_gaps":["hardware instructions require permitted worker PMU; interrupted calls may lose counter readback","exact first fall degree certificates","exact last fall degree certificates","private native matrix internals and SAT watch/heap/minimization internals"],
        "checkpoint_mid_call":"exposed events and completed payloads when using solve_with_event_sink; native opaque calls retain their last checkpoint",
        "interrupted_trace_loss_possible":true,
        "checkpoint_reason":"Each emitted event and completed payload is fsynced; interruption inside an uninstrumented native call can lose private intermediate states",
        "bounded_search_event_cap":TRACE_EVENT_CAP,"sat_formula_trace_byte_cap":TRACE_BYTE_CAP,
        "sat_native_event_byte_cap":SAT_EVENT_BYTE_CAP,"sat_native_event_count_cap":TRACE_EVENT_CAP
    })
}

#[derive(Clone)]
enum Group {
    Binary(Box<BinaryCurve>),
    Prime(curves::ExactPrimeCurve),
}
#[derive(Clone, Debug, PartialEq)]
enum Element {
    Binary(BinaryPoint),
    Prime(Point),
}

impl Element {
    fn json(&self) -> Value {
        match self {
            Self::Binary(BinaryPoint::Infinity) | Self::Prime(Point::Infinity) => Value::Null,
            Self::Binary(BinaryPoint::Affine { x, y }) => {
                json!({"x":x.to_biguint().to_string(),"y":y.to_biguint().to_string()})
            }
            Self::Prime(Point::Affine { x, y }) => {
                json!({"x":x.value.to_string(),"y":y.value.to_string()})
            }
        }
    }
    fn key(&self) -> String {
        self.json().to_string()
    }
    fn x(&self) -> Option<BigUint> {
        match self {
            Self::Binary(BinaryPoint::Affine { x, .. }) => Some(x.to_biguint()),
            Self::Prime(Point::Affine { x, .. }) => Some(x.value.clone()),
            _ => None,
        }
    }
}

impl Group {
    fn read(case: &CaseData) -> Result<Self, String> {
        if case.public_instance["family"] == "prime" {
            Ok(Self::Prime(curves::prime_curve(&case.public_instance)?))
        } else {
            Ok(Self::Binary(Box::new(curves::binary_curve(
                &case.public_instance,
            )?)))
        }
    }
    fn point(&self, value: &Value) -> Result<Element, String> {
        match self {
            Self::Binary(c) => curves::binary_point(value, c.m).map(Element::Binary),
            Self::Prime(c) => curves::prime_point(value, c).map(Element::Prime),
        }
    }
    fn zero(&self) -> Element {
        match self {
            Self::Binary(_) => Element::Binary(BinaryPoint::Infinity),
            Self::Prime(_) => Element::Prime(Point::Infinity),
        }
    }
    fn add(&self, left: &Element, right: &Element) -> Element {
        match (self, left, right) {
            (Self::Binary(c), Element::Binary(p), Element::Binary(q)) => {
                Element::Binary(binary::point_add(c, p, q))
            }
            (Self::Prime(c), Element::Prime(p), Element::Prime(q)) => {
                Element::Prime(p.add_vartime(q, &c.a_fe()))
            }
            _ => unreachable!("validated homogeneous group"),
        }
    }
    fn neg(&self, p: &Element) -> Element {
        match p {
            Element::Binary(p) => Element::Binary(binary::point_neg(p)),
            Element::Prime(p) => Element::Prime(p.neg_vartime()),
        }
    }
    fn mul(&self, p: &Element, k: u64) -> Element {
        match (self, p) {
            (Self::Binary(c), Element::Binary(p)) => {
                Element::Binary(binary::scalar_mul(c, p, &BigUint::from(k)))
            }
            (Self::Prime(c), Element::Prime(p)) => {
                Element::Prime(p.scalar_mul_vartime(&BigUint::from(k), &c.a_fe()))
            }
            _ => unreachable!("validated homogeneous group"),
        }
    }
    fn on_curve(&self, p: &Element) -> bool {
        match (self, p) {
            (Self::Binary(c), Element::Binary(p)) => c.is_on_curve(p),
            (Self::Prime(c), Element::Prime(p)) => c.is_on_curve(p),
            _ => false,
        }
    }
    fn order(&self) -> Option<u64> {
        match self {
            Self::Binary(c) => c.order.to_u64(),
            Self::Prime(c) => c.n.to_u64(),
        }
    }
}

fn domain(group: &Group, case: &CaseData) -> Result<Vec<Element>, String> {
    let rows = case.factor_base["points"]
        .as_array()
        .ok_or("factor_base.points must be an array")?;
    let mut points = Vec::new();
    for row in rows {
        let p = group.point(row)?;
        if p == group.zero() || !group.on_curve(&p) {
            return Err("invalid factor-base point".into());
        }
        for signed in [p.clone(), group.neg(&p)] {
            if !points.contains(&signed) {
                points.push(signed);
            }
        }
    }
    Ok(points)
}

fn outcome(status: &str, reason: &str) -> Value {
    json!({"schema":"ml-dataset.solver-outcome/v1","status":status,"reason":reason,
        "solver_completed":false,"pdp_witness_verified":false,"scalar_recovered_without_witness":false,"scalar_replay_verified":false,
        "first_fall_degree":null,"last_fall_degree":null,"retired_hardware_instructions":null,
        "instruction_count_reason":"Worker-level PMU records, when permitted, are saved separately; this adapter does not invent per-operation instruction attribution",
        "trace_complete":false,"counters":{},"counter_scope":"native adapter operations, not retired instructions"})
}
fn unsupported(reason: &str) -> SolverData {
    SolverData {
        outcome: outcome("unsupported", reason),
        artifacts: vec![],
    }
}
fn save(data: &mut SolverData, path: &str, role: &str, value: &Value) -> Result<(), String> {
    let payload = ArtifactPayload::json(path, role, value)?;
    live_payload(&payload)?;
    data.artifacts.push(payload);
    Ok(())
}
fn set_witness(
    data: &mut SolverData,
    group: &Group,
    points: &[Element],
    target: &Element,
) -> Result<(), String> {
    set_witness_with_definition(data, group, points, target, None)
}
fn set_witness_with_definition(
    data: &mut SolverData,
    group: &Group,
    points: &[Element],
    target: &Element,
    definition: Option<&Value>,
) -> Result<(), String> {
    let mut sum = group.zero();
    for p in points {
        if !group.on_curve(p) {
            return Err("off-curve PDP lift".into());
        }
        sum = group.add(&sum, p);
    }
    if &sum != target {
        return Err("PDP group replay failed".into());
    }
    data.outcome["status"] = json!("verified");
    data.outcome["pdp_witness_verified"] = json!(true);
    data.outcome["solver_completed"] = json!(true);
    data.outcome["reason"] =
        json!("A signed factor-base decomposition was independently replayed in the curve group");
    let mut label = json!({"summands":points.iter().map(Element::json).collect::<Vec<_>>(),"target":target.json(),"sum":sum.json(),"replay_group_add_calls":points.len()});
    if let Some(definition) = definition {
        label["factor_base_definition"] = definition.clone();
    }
    save(
        data,
        "pdp-witness.json",
        "independently-verified-pdp",
        &label,
    )
}

pub fn solve(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
    solve_with_context(config, case, None)
}

pub(super) fn solve_with_context(
    config: &CaseConfig,
    case: &CaseData,
    context: Option<&super::execution::SolverExecutionContext>,
) -> Result<SolverData, String> {
    if !(2..=15).contains(&config.m) {
        return Err("decomposition arity must be 2..15".into());
    }
    if config.solver == DYNAMIC_PROFILE && !dynamic_supported(config) {
        let mut unsupported = outcome(
            "unsupported",
            "dynamic profile requires prime n33..61, m4..15, degree12 and positive work",
        );
        unsupported["strategy"] = json!(DYNAMIC_STRATEGY);
        return Ok(SolverData {
            outcome: unsupported,
            artifacts: Vec::new(),
        });
    }
    if case.factor_base["schema"] == "ml-ecdlp-factor-base/v2"
        && case.factor_base["construction_status"] == "unsupported"
    {
        return Ok(unsupported(
            case.factor_base["unsupported_reason"]
                .as_str()
                .unwrap_or("The requested factor-base policy is unsupported"),
        ));
    }
    let group = Group::read(case)?;
    let target = group.point(&case.public_instance["target"])?;
    if !group.on_curve(&target) {
        return Err("target is not on the declared curve".into());
    }
    let points = domain(&group, case)?;
    let field_tally = crate::binary_ecc::operation_tally::OperationTally::begin();
    let mut result = match config.solver.as_str() {
        DYNAMIC_PROFILE => solve_dynamic_bound(
            config,
            case,
            context.ok_or("dynamic profile requires original execution context")?,
        )?,
        "enumerate" | "pdp" => enumerate(config, &group, &points, &target)?,
        "bsgs" => bsgs(config, case, &group, &target)?,
        "sat" => sat_solve(config, &group, &points, &target)?,
        "f4" if matches!(group, Group::Prime(_)) => {
            let result = prime_f4(config, &group, &points, &target, context)?;
            if let Some(input) = result
                .artifacts
                .iter()
                .find(|a| a.path == "prime-f4-input.json")
                .filter(|_| config.m > 2)
            {
                verify_prime_f4_chain_input(
                    &case.public_instance,
                    &case.factor_base,
                    &serde_json::from_slice(&input.bytes).map_err(|e| e.to_string())?,
                )?;
            }
            result
        }
        "weil" | "weil-descent" | "f4" | "f5" => boolean_solve(config, &group, &points, &target)?,
        "f6" => f6_solve(config, &group, &points, &target)?,
        "root" => root_solve(config, case, &group, &points, &target)?,
        "pkm" => pkm_solve(config, case, &group, &points, &target)?,
        "double-large-prime" => super::relations::solve(config, case)?,
        other => return Err(format!("unknown solver {other}")),
    };
    result.outcome["solver"] = json!(config.solver);
    result.outcome["requested_work_limit"] = json!(config.work_limit);
    result.outcome["m"] = json!(config.m);
    if result.outcome.get("ecdlp_pipeline_completed").is_none() {
        result.outcome["ecdlp_pipeline_completed"] = json!(false);
    }
    result.outcome["checkpoint_mid_call"] = json!(LIVE_SINK.with(|slot| slot.borrow().is_some()));
    result.outcome["interrupted_trace_loss_possible"] = json!(true);
    result.outcome["binary_field_operation_tally"] = json!({
        "counts":field_tally.finish(),
        "scope":"calling-thread instrumented binary field APIs during adapter solve and replay; excludes setup before solve and optimized kernels bypassing these APIs",
        "inversion_count_includes_its_mul_and_square_cost":true,
        "complete_machine_instruction_count":false
    });
    let solver_domain = json!({"points":points.iter().map(Element::json).collect::<Vec<_>>(),"negation_closed":true,"repetitions_allowed":true});
    if retains_canonical_artifacts(&result) {
        result.artifacts.push(ArtifactPayload::json(
            "solver-domain.json",
            "exact-signed-factor-base",
            &solver_domain,
        )?);
        flush_prime_chain_payloads(&mut result);
    } else {
        save(
            &mut result,
            "solver-domain.json",
            "exact-signed-factor-base",
            &solver_domain,
        )?;
    }
    Ok(result)
}

/// Replay security-relevant labels from public data, independently of the
/// generator's known scalar. Call after checking the artifact's digest.
pub fn verify_solution_artifact(
    public: &Value,
    factor_base: &Value,
    role: &str,
    artifact: &Value,
) -> Result<(), String> {
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let group = Group::read(&case)?;
    let target = group.point(&public["target"])?;
    if artifact["target"] != public["target"] {
        return Err("solution artifact target differs from public target".into());
    }
    match role {
        "independently-verified-pdp" => {
            let domain = if artifact["factor_base_definition"].is_object() {
                verify_kummer_domain(&group, &case, &artifact["factor_base_definition"])?
            } else {
                domain(&group, &case)?
            };
            let points = artifact["summands"]
                .as_array()
                .ok_or("missing PDP summands")?;
            let m = public["m"].as_u64().ok_or("missing public arity")?;
            if points.len() as u64 != m {
                return Err("PDP witness has wrong arity".into());
            }
            let mut sum = group.zero();
            for row in points {
                let p = group.point(row)?;
                if !group.on_curve(&p) || !domain.contains(&p) {
                    return Err("PDP witness has a point outside the signed factor base".into());
                }
                sum = group.add(&sum, &p);
            }
            if sum != target || artifact["sum"] != target.json() {
                return Err("PDP witness group replay failed".into());
            }
        }
        "restricted-independent-scalar-label" => {
            if artifact["generator"] != public["generator"] {
                return Err("scalar artifact generator differs from public generator".into());
            }
            let scalar = artifact["scalar"]
                .as_str()
                .and_then(|s| s.parse::<u64>().ok())
                .ok_or("invalid recovered scalar")?;
            let order = group.order().ok_or("invalid subgroup order")?;
            if scalar >= order {
                return Err("recovered scalar is not canonical".into());
            }
            let generator = group.point(&public["generator"])?;
            if group.mul(&generator, scalar) != target {
                return Err("recovered scalar replay failed".into());
            }
        }
        _ => return Err("not a solution artifact role".into()),
    }
    Ok(())
}

struct Search<'a> {
    group: &'a Group,
    target: &'a Element,
    domain: &'a [Element],
    limit: u64,
    operations: u64,
    trace: Vec<Value>,
    exhausted: bool,
    io_error: Option<String>,
}
impl Search<'_> {
    fn descend(
        &mut self,
        depth: usize,
        m: usize,
        sum: &Element,
        path: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        if depth == m {
            return (sum == self.target).then(|| path.clone());
        }
        for i in 0..self.domain.len() {
            if self.operations >= self.limit {
                self.exhausted = true;
                return None;
            }
            let next = self.group.add(sum, &self.domain[i]);
            self.operations += 1;
            let event = json!({"kind":"pdp_transition","event":self.operations,"depth":depth,"domain_index":i,"parent_sum":sum.json(),"sum":next.json()});
            if let Err(error) = live_event(&event) {
                self.io_error = Some(error);
                self.exhausted = true;
                return None;
            }
            self.trace.push(event);
            path.push(i);
            if let Some(found) = self.descend(depth + 1, m, &next, path) {
                return Some(found);
            }
            path.pop();
            if self.exhausted {
                return None;
            }
        }
        None
    }
}
fn enumerate(
    config: &CaseConfig,
    group: &Group,
    points: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let mut search = Search {
        group,
        target,
        domain: points,
        limit: config.work_limit.min(TRACE_EVENT_CAP),
        operations: 0,
        trace: vec![],
        exhausted: false,
        io_error: None,
    };
    let found = search.descend(0, config.m, &group.zero(), &mut vec![]);
    if let Some(error) = search.io_error.take() {
        return Err(error);
    }
    let mut data = SolverData {
        outcome: outcome(
            if search.exhausted {
                "budget_exhausted"
            } else {
                "verified"
            },
            if search.exhausted {
                "Finite-domain search stopped at the recorded transition budget"
            } else {
                "Finite-domain search completed"
            },
        ),
        artifacts: vec![],
    };
    data.outcome["solver_completed"] = json!(!search.exhausted);
    data.outcome["exhaustive_no_decomposition"] = json!(found.is_none() && !search.exhausted);
    data.outcome["trace_complete"] = json!(true);
    data.outcome["effective_work_limit"] = json!(search.limit);
    data.outcome["counters"] = json!({"group_add_api_calls":search.operations,"complete_candidate_tests":search.trace.iter().filter(|x|x["depth"]==json!(config.m-1)).count()});
    save(
        &mut data,
        "enumeration-events.json",
        "all-visited-pdp-transitions",
        &json!(search.trace),
    )?;
    if let Some(indices) = found {
        set_witness(
            &mut data,
            group,
            &indices
                .iter()
                .map(|&i| points[i].clone())
                .collect::<Vec<_>>(),
            target,
        )?;
    }
    Ok(data)
}

fn bsgs(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    target: &Element,
) -> Result<SolverData, String> {
    let generator = group.point(&case.public_instance["generator"])?;
    if !group.on_curve(&generator) || generator == group.zero() {
        return Err("invalid BSGS generator".into());
    }
    let Some(order) = group.order().filter(|&r| r > 1) else {
        return Ok(unsupported(
            "BSGS needs a certified subgroup order fitting u64",
        ));
    };
    let mut width = (order as f64).sqrt() as u64;
    while (width as u128) * (width as u128) < order as u128 {
        width += 1;
    }
    while width > 0 && ((width - 1) as u128) * ((width - 1) as u128) >= order as u128 {
        width -= 1;
    }
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut data = SolverData {
        outcome: outcome(
            "budget_exhausted",
            "Independent BSGS budget insufficient for the complete baby table",
        ),
        artifacts: vec![],
    };
    data.outcome["required_baby_entries"] = json!(width);
    data.outcome["effective_work_limit"] = json!(limit);
    if width > limit {
        return Ok(data);
    }
    let mut baby = HashMap::new();
    let mut rows = Vec::new();
    let mut p = group.zero();
    let mut ops = 0;
    for j in 0..width {
        baby.entry(p.key()).or_insert(j);
        let event = json!({"kind":"bsgs_baby","j":j.to_string(),"point":p.json()});
        live_event(&event)?;
        rows.push(event);
        p = group.add(&p, &generator);
        ops += 1;
    }
    let stride = group.neg(&p);
    let mut giant = target.clone();
    let mut probes = vec![];
    let mut answer = None;
    let mut scalar_checks = 0u64;
    for i in 0..=width {
        if ops >= limit {
            break;
        }
        let hit = baby.get(&giant.key()).copied();
        let event = json!({"kind":"bsgs_giant","i":i.to_string(),"point":giant.json(),"baby_index":hit.map(|j|j.to_string())});
        live_event(&event)?;
        probes.push(event);
        if let Some(j) = hit {
            let k = ((i as u128 * width as u128 + j as u128) % order as u128) as u64;
            scalar_checks += 1;
            if group.mul(&generator, k) == *target {
                answer = Some(k);
                break;
            }
        }
        giant = group.add(&giant, &stride);
        ops += 1;
    }
    save(
        &mut data,
        "bsgs-baby-table.json",
        "independent-scalar-search-table",
        &json!(rows),
    )?;
    save(
        &mut data,
        "bsgs-giant-probes.json",
        "independent-scalar-search-probes",
        &json!(probes),
    )?;
    data.outcome["counters"] = json!({"group_add_api_calls_search":ops,"baby_entries":width,"giant_probes":probes.len(),"scalar_verification_calls":scalar_checks});
    data.outcome["trace_complete"] = json!(true);
    if let Some(k) = answer {
        data.outcome["status"] = json!("verified");
        data.outcome["solver_completed"] = json!(true);
        data.outcome["scalar_recovered_without_witness"] = json!(true);
        data.outcome["scalar_replay_verified"] = json!(true);
        data.outcome["reason"]=json!("Independent bounded BSGS recovered a scalar and replayed Q=[k]G; this is a generic baseline, not index calculus");
        save(
            &mut data,
            "scalar-recovery.json",
            "restricted-independent-scalar-label",
            &json!({"scalar":k.to_string(),"generator":generator.json(),"target":target.json(),"method":"native-bsgs","read_private_witness":false}),
        )?;
    }
    Ok(data)
}

fn lift(
    group: &Group,
    domain: &[Element],
    xs: &[BigUint],
    target: &Element,
    limit: u64,
) -> (Option<Vec<Element>>, u64, bool) {
    let choices: Vec<Vec<Element>> = xs
        .iter()
        .map(|x| {
            domain
                .iter()
                .filter(|p| p.x().as_ref() == Some(x))
                .cloned()
                .collect()
        })
        .collect();
    fn visit(
        group: &Group,
        choices: &[Vec<Element>],
        depth: usize,
        sum: &Element,
        target: &Element,
        path: &mut Vec<Element>,
        used: &mut u64,
        limit: u64,
    ) -> Option<Vec<Element>> {
        if depth == choices.len() {
            return (sum == target).then(|| path.clone());
        }
        for p in &choices[depth] {
            if *used >= limit {
                return None;
            }
            *used += 1;
            let next = group.add(sum, p);
            path.push(p.clone());
            if let Some(found) = visit(group, choices, depth + 1, &next, target, path, used, limit)
            {
                return Some(found);
            }
            path.pop();
        }
        None
    }
    let mut used = 0;
    let result = visit(
        group,
        &choices,
        0,
        &group.zero(),
        target,
        &mut vec![],
        &mut used,
        limit,
    );
    let exhausted = result.is_none() && used >= limit && choices.iter().all(|c| !c.is_empty());
    (result, used, exhausted)
}

fn sat_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported(
            "Native chained-S3 SAT currently requires a binary curve",
        ));
    };
    if crate::cryptanalysis::binary_semaev_chain_sat::chain_s3_max_variables(curve.m, config.m)
        .is_none()
    {
        return Ok(unsupported(
            "Native chained-S3 circuit rejects this n/m; no substitute solve was performed",
        ));
    }
    let target_x = match target {
        Element::Binary(BinaryPoint::Affine { x, .. }) => Some(x),
        _ => None,
    };
    let mut codes: Vec<BigUint> = domain.iter().filter_map(Element::x).collect();
    codes.sort();
    codes.dedup();
    let patterns = 1u32 << (config.m - 2);
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut data = SolverData {
        outcome: outcome(
            "budget_exhausted",
            "Native SAT search has not covered all intermediate-identity patterns",
        ),
        artifacts: vec![],
    };
    let mut summaries = vec![];
    let mut lift_calls = 0;
    let mut conflicts = 0;
    let mut attempts = 0;
    let mut complete_patterns = 0;
    let deadline = Instant::now() + Duration::from_secs(10);
    let trace = Arc::new(Mutex::new(SatEventLog::default()));
    'patterns: for pattern in 0..patterns {
        if attempts >= limit || Instant::now() >= deadline {
            break;
        }
        let mut enc = match ChainedS3Encoding::build(
            config.m,
            &curve.irreducible,
            &curve.b,
            target_x,
            pattern,
            100_000,
        ) {
            Ok(x) => x,
            Err(e) => {
                data.outcome["reason"] = json!(format!("Circuit construction cap: {e:?}"));
                break;
            }
        };
        if let Err(e) = enc.constrain_summands(&codes, 1_000_000) {
            data.outcome["reason"] = json!(format!("Finite-domain construction cap: {e:?}"));
            break;
        }
        let formula = sat::to_dimacs_xor(&enc.solver);
        let used_bytes: usize = data.artifacts.iter().map(|a| a.bytes.len()).sum();
        if used_bytes.saturating_add(formula.len()) > TRACE_BYTE_CAP {
            data.outcome["reason"] = json!("Formula trace byte budget exhausted before solve");
            break;
        }
        let payload = ArtifactPayload {
            path: format!("sat-pattern-{pattern}.xor.cnf"),
            role: "complete-original-cnf-xor".into(),
            encoding: "dimacs-xor".into(),
            bytes: formula.into_bytes(),
        };
        live_payload(&payload)?;
        data.artifacts.push(payload);
        enc.solver.conflict_budget = limit.saturating_sub(conflicts);
        enc.solver.deadline = Some(deadline);
        // One model per pattern is a bounded diagnostic. A spurious lift does
        // not refute this pattern; it remains explicitly incomplete.
        attempts += 1;
        live_event(
            &json!({"kind":"sat_call_begin","pattern":pattern,"conflict_budget":enc.solver.conflict_budget}),
        )?;
        let output = Arc::clone(&trace);
        // Initial clauses/units are included by the native attachment snapshot.
        // Installation failure is sticky and solve returns Unknown below.
        let installation = enc.solver.set_event_observer(move |event| {
            output
                .lock()
                .map_err(|_| "SAT trace mutex poisoned".to_string())?
                .append(pattern, event, SAT_EVENT_BYTE_CAP, TRACE_EVENT_CAP)
        });
        let verdict = enc.solver.solve();
        let observer_error = enc.solver.event_observer_error().map(str::to_owned);
        if let Err(error) = installation {
            if observer_error.is_none() {
                return Err(error);
            }
        }
        let stats = &enc.solver.stats;
        conflicts = conflicts.saturating_add(stats.conflicts);
        let mut row = json!({"pattern":pattern,"verdict":format!("{verdict:?}"),"variables":enc.solver.n_vars(),"and_gates":enc.n_and_gates,"s3_nodes":enc.n_s3_nodes,
            "native_counters":{"decisions":stats.decisions,"conflicts":stats.conflicts,"propagations":stats.propagations,"restarts":stats.restarts,"xor_passes":stats.xor_passes,"xor_row_ops":stats.xor_row_ops,"xor_row_scans":stats.xor_row_scans,"learnt_clauses":stats.learnt_clauses,"clause_visits":stats.clause_visits,"clause_lit_visits":stats.clause_lit_visits,"analyze_lit_visits":stats.analyze_lit_visits,"word_ops":enc.solver.word_ops()},"unsat_proof_checked":false});
        if let Some(error) = observer_error {
            if verdict != SolveResult::Unknown {
                return Err("SAT observer failure was not censored".into());
            }
            row["observer_error"] = json!(error);
            live_event(&json!({"kind":"sat_call_result","result":row}))?;
            summaries.push(row);
            if error != SAT_EVENT_CAP_REASON {
                return Err(format!("native SAT event sink failed: {error}"));
            }
            data.outcome["reason"] = json!(error);
            data.outcome["trace_budget_exhausted"] = json!(true);
            break;
        }
        match verdict {
            SolveResult::Sat => {
                let model = enc.solver.model();
                if !enc.verify_external_assignment(&model) {
                    return Err("native SAT model failed original constraint verification".into());
                }
                let xs = enc
                    .decode_summands_assignment(&model)
                    .ok_or("SAT model cannot be decoded")?;
                let xs: Vec<BigUint> = xs.iter().map(F2mElement::to_biguint).collect();
                let (witness, adds, exhausted) =
                    lift(group, domain, &xs, target, limit.saturating_sub(lift_calls));
                lift_calls += adds;
                row["model"] = json!(model);
                row["x_coordinates"] =
                    json!(xs.iter().map(ToString::to_string).collect::<Vec<_>>());
                row["group_lift_verified"] = json!(witness.is_some());
                row["lift_budget_exhausted"] = json!(exhausted);
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
                if let Some(w) = witness {
                    set_witness(&mut data, group, &w, target)?;
                    break 'patterns;
                }
            }
            SolveResult::Unsat => {
                complete_patterns += 1;
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
            }
            SolveResult::Unknown => {
                live_event(&json!({"kind":"sat_call_result","result":row}))?;
                summaries.push(row);
                break;
            }
        }
    }
    if complete_patterns == patterns {
        data.outcome["status"] = json!("partial");
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("All identity patterns were solver-reported UNSAT; no independently checked UNSAT proof is available");
    }
    data.outcome["identity_patterns_required"] = json!(patterns);
    data.outcome["identity_patterns_attempted"] = json!(attempts);
    data.outcome["counters"] =
        json!({"sat_conflicts":conflicts,"group_lift_add_calls":lift_calls,"sat_calls":attempts});
    let mut log = trace
        .lock()
        .map_err(|_| "SAT trace mutex poisoned".to_string())?;
    data.outcome["native_event_trace"] = json!({"events":log.events,"bytes":log.bytes.len(),"event_cap":TRACE_EVENT_CAP,"byte_cap":SAT_EVENT_BYTE_CAP,
        "proof_checked":false,"scope":"native semantic search events; clause arena IDs and XOR matrix row operations","gate_map":"summand blocks are DIMACS 1..=m*n; bit j is coefficient of field basis element x^j; remaining gate provenance not exported"});
    let payload = ArtifactPayload {
        path: "sat-native-events.jsonl".into(),
        role: "native-sat-search-events".into(),
        encoding: "jsonl".into(),
        bytes: std::mem::take(&mut log.bytes),
    };
    live_payload(&payload)?;
    data.artifacts.push(payload);
    data.outcome["trace_gaps"]=json!(["No independently checked UNSAT proof; analysis events do not certify clause minimization","Private watch movements and heap mutations are not exported","Only the first SAT model of each pattern is attempted; rejected lifts leave the pattern inconclusive"]);
    save(
        &mut data,
        "sat-attempts.json",
        "native-sat-models-and-counters",
        &json!(summaries),
    )?;
    Ok(data)
}

fn finite_selector(
    codes: &[BigUint],
    offset: usize,
    bits: usize,
    n: u32,
    n_vars: usize,
) -> SymElement {
    let count = 1usize << bits;
    let coords = (0..n)
        .map(|field_bit| {
            let mut table: Vec<bool> = (0..count)
                .map(|i| codes[i % codes.len()].bit(field_bit as u64))
                .collect();
            for bit in 0..bits {
                for mask in 0..count {
                    if mask & (1 << bit) != 0 {
                        table[mask] ^= table[mask ^ (1 << bit)];
                    }
                }
            }
            F2BoolPoly::from_monos(
                table
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| **v)
                    .map(|(m, _)| F2BoolMono::from_mask((m as Mask) << offset))
                    .collect(),
                n_vars,
            )
        })
        .collect();
    SymElement { coords }
}
fn polys_json(polys: &[F2BoolPoly]) -> Value {
    json!(polys.iter().map(|p|json!({"n_vars":p.n_vars,"monomial_masks_hex":p.terms.iter().map(|t|format!("{:x}",t.mask)).collect::<Vec<_>>()})).collect::<Vec<_>>())
}

fn boolean_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported(
            "This F5/Weil lane requires characteristic two; use f4 for the native prime-field adapter",
        ));
    };
    if config.m != 2 {
        return Ok(unsupported("Exact finite-domain selector Weil/F4/F5 adapter currently supports m=2; larger arities remain separate planned cells"));
    }
    let Some(xr) = target.x() else {
        return Ok(unsupported("Boolean S3 lane currently requires an affine target; use enumerative PDP for identity targets"));
    };
    let mut codes: Vec<BigUint> = domain.iter().filter_map(Element::x).collect();
    codes.sort();
    codes.dedup();
    if codes.is_empty() {
        return Ok(unsupported("Factor base is empty"));
    }
    let bits = (usize::BITS - (codes.len().saturating_sub(1)).leading_zeros()) as usize;
    if bits > 6 {
        return Ok(unsupported(
            "Finite-domain selector trace cap is 64 distinct x-coordinates",
        ));
    }
    let n_vars = 2 * bits;
    let st = FieldStructure::new(curve.m, &curve.irreducible);
    let left = finite_selector(&codes, 0, bits, curve.m, n_vars);
    let right = finite_selector(&codes, bits, bits, curve.m, n_vars);
    let target_symbol =
        SymElement::constant(&F2mElement::from_biguint(&xr, curve.m), curve.m, n_vars);
    let equations = boolean::sym_semaev_s3(&left, &right, &target_symbol, &curve.b, &st);
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Exact finite-domain S3 system was Weil-restricted to the Boolean quotient",
        ),
        artifacts: vec![],
    };
    save(
        &mut data,
        "weil-system.json",
        "complete-original-boolean-polynomials",
        &json!({"coefficient_field":"F2","quotient":"v_i^2+v_i","monomial_order":"degrevlex","n_vars":n_vars,"selector_bits_per_summand":bits,"selector_codes":(0..1usize<<bits).map(|i|codes[i%codes.len()].to_string()).collect::<Vec<_>>(),"coordinate_maps":[polys_json(&left.coords),polys_json(&right.coords)],"equations":polys_json(&equations),"target_x":xr.to_string(),"field_degree":curve.m,"irreducible_low_terms":curve.irreducible.low_terms,"lifting_required":true}),
    )?;
    data.outcome["polynomial_encoding_verified"] = json!(true);
    if config.solver == "weil" || config.solver == "weil-descent" {
        data.outcome["trace_complete"] = json!(true);
        return Ok(data);
    }
    let is_f5 = config.solver == "f5";
    let degree = config.degree_bound.min(6);
    // Save the complete degree-bounded Macaulay input as explicit polynomials.
    // It is small under this selector cap and does not claim to expose every
    // private elimination state at descendant nodes.
    let mut matrix_rows = Vec::new();
    let mut matrix_truncated = false;
    for (equation, p) in equations.iter().enumerate() {
        let d = p.terms.iter().map(|t| t.degree()).max().unwrap_or(0);
        if d > degree {
            continue;
        }
        for mask in 0usize..1usize << n_vars {
            if (mask.count_ones() + d) <= degree {
                if matrix_rows.len() as u64 >= config.work_limit.min(TRACE_EVENT_CAP) {
                    matrix_truncated = true;
                    break;
                }
                let q = p.mul_mono(F2BoolMono::from_mask(mask as Mask));
                matrix_rows.push(json!({"equation":equation,"multiplier_hex":format!("{mask:x}"),"row":polys_json(&[q])}));
            }
        }
        if matrix_truncated {
            break;
        }
    }
    if matrix_truncated {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Macaulay input trace cap reached before native elimination");
        save(
            &mut data,
            "macaulay-input.json",
            "partial-macaulay-input",
            &json!(matrix_rows),
        )?;
        return Ok(data);
    }
    save(
        &mut data,
        "macaulay-input.json",
        "complete-unpruned-degree-bounded-macaulay-candidates",
        &json!(matrix_rows),
    )?;
    let reduced = if is_f5 {
        matrix_f5_f2(&equations, n_vars, degree).map(|(rows, report)| (rows, json!(report)))
    } else {
        boolean::matrix_f4_f2_counted(&equations, n_vars, degree)
            .map(|(rows, word_ops)| (rows, json!({"elimination_word_xors":word_ops})))
    };
    if let Some((rows, report)) = reduced {
        save(
            &mut data,
            "macaulay-output.json",
            "native-reduced-macaulay-rows",
            &json!({"rows":polys_json(&rows),"report":report}),
        )?;
    } else {
        data.outcome["macaulay_step"] = json!("native_size_cap");
    }
    let before = boolean::f4_word_ops_thread();
    live_event(
        &json!({"kind":"boolean_native_solve_begin","engine":config.solver,"n_vars":n_vars,"node_budget":config.work_limit.min(4096)}),
    )?;
    boolean::set_node_trace(true);
    let opts = boolean::SolveOptions {
        engine: if is_f5 {
            boolean::SolverEngine::MatrixF5 { max_degree: degree }
        } else {
            boolean::SolverEngine::MatrixF4 { max_degree: degree }
        },
        max_solutions: usize::MAX,
        node_budget: config.work_limit.min(4096) as usize,
        split_rule: boolean::SplitRule::LowestFree,
    };
    let mut witnessed = None;
    let mut root_rows = vec![];
    let mut lift_ops = 0;
    let mut lift_censored = false;
    let (roots, stats) = boolean::solve_boolean_system_filtered(
        &equations,
        n_vars,
        &opts,
        |root| {
            let mask = (1usize << bits) - 1;
            let xs = [
                codes[(root as usize & mask) % codes.len()].clone(),
                codes[((root >> bits) as usize & mask) % codes.len()].clone(),
            ];
            let (found, adds, exhausted) = lift(
                group,
                domain,
                &xs,
                target,
                config
                    .work_limit
                    .min(TRACE_EVENT_CAP)
                    .saturating_sub(lift_ops),
            );
            lift_ops += adds;
            lift_censored |= exhausted;
            root_rows.push(json!({"assignment_hex":format!("{root:x}"),"x":xs.iter().map(ToString::to_string).collect::<Vec<_>>(),"group_lift_verified":found.is_some(),"lift_budget_exhausted":exhausted}));
            if found.is_some() {
                witnessed = found;
            }
            witnessed.is_some()
        },
    );
    let events = boolean::take_node_trace();
    boolean::set_node_trace(false);
    save(
        &mut data,
        "boolean-solve.json",
        "native-algebra-result-and-node-trace",
        &json!({"stats":stats,"roots_hex":roots.iter().map(|r|format!("{r:x}")).collect::<Vec<_>>(),"root_lifts":root_rows,"nodes":events.iter().map(|e|json!({"assigned_hex":format!("{:x}",e.assigned),"values_hex":format!("{:x}",e.values),"thread_cumulative_word_ops":e.word_ops})).collect::<Vec<_>>()}),
    )?;
    data.outcome["counters"] = json!({"native_f4_word_ops_search":boolean::f4_word_ops_thread().saturating_sub(before),"group_lift_add_calls":lift_ops,"native_solve_stats":stats});
    data.outcome["observed_max_macaulay_degree"] = json!(stats.max_degree_built);
    data.outcome["algebra_completed"] = json!(!stats.exhausted);
    data.outcome["lift_completed"] = json!(!lift_censored);
    data.outcome["trace_gaps"] = json!([
        "Internal row/pivot and signature operations at recursive solver nodes are not exported",
        "Maximum built degree is an observation, not first or last fall degree"
    ]);
    if let Some(w) = witnessed {
        set_witness(&mut data, group, &w, target)?;
    } else if stats.exhausted {
        data.outcome["status"] = json!(if stats.unsupported {
            "unsupported"
        } else {
            "budget_exhausted"
        });
        data.outcome["reason"] = json!("Native Boolean solve was incomplete");
    } else if lift_censored {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Native algebra completed but group lifting exhausted its separate budget");
    } else {
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("Native Boolean root search completed; no accepted group lift was found within the separate lift budget");
    }
    Ok(data)
}

fn verify_kummer_domain(
    group: &Group,
    case: &CaseData,
    definition: &Value,
) -> Result<Vec<Element>, String> {
    let Group::Prime(curve) = group else {
        return Err("Kummer factor-base override requires a prime curve".into());
    };
    if definition["kind"] != "prime-kummer" {
        return Err("unknown factor-base override".into());
    }
    let t = definition["t"].as_u64().ok_or("invalid Kummer height")?;
    let g = definition["g"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid Kummer coset")?;
    let p = curve.p.to_u64().ok_or("invalid Kummer modulus")?;
    if !(1..=4).contains(&t) || g == 0 || g >= p || !(p - 1).is_multiple_of(1 << t) {
        return Err("invalid Kummer base definition".into());
    }
    let original = domain(group, case)?;
    let expected_g = original
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .filter(|&x| x != 0)
        .min()
        .ok_or("no original nonzero abscissa")?;
    if g != expected_g {
        return Err("Kummer coset is not bound to the canonical original factor base".into());
    }
    let points = definition["points"]
        .as_array()
        .ok_or("missing Kummer points")?;
    let mut out = Vec::new();
    let inverse = BigUint::from(g).modpow(&BigUint::from(p - 2), &curve.p);
    for row in points {
        let point = group.point(row)?;
        let x = point.x().ok_or("infinite Kummer point")?;
        if !group.on_curve(&point)
            || ((&x * &inverse) % &curve.p).modpow(&BigUint::from(1u64 << t), &curve.p)
                != BigUint::from(1u8)
        {
            return Err("point outside defined Kummer base".into());
        }
        if out.contains(&point) {
            return Err("duplicate Kummer point".into());
        }
        out.push(point);
    }
    Ok(out)
}

/// Reconstruct the small public Kummer base without running PKM elimination.
/// This also rejects a proper subset or forged column/sign map, so an exported
/// feature graph describes the actual domain used by the native oracle.
pub(super) fn verify_pkm_factor_base(
    public: &Value,
    factor_base: &Value,
    artifact: &Value,
) -> Result<(), String> {
    use crate::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve, PrimeInstance, PrimePoint};
    use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
    use crate::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let group = Group::read(&case)?;
    let definition = &artifact["definition"];
    let actual_points = verify_kummer_domain(&group, &case, definition)?;
    if definition["subgroup_filtered"] != false || definition["known_logs"] != false {
        return Err("unexpected PKM factor-base provenance flags".into());
    }
    let Group::Prime(curve) = &group else {
        return Err("PKM base requires a prime curve".into());
    };
    let p = curve.p.to_u64().ok_or("invalid PKM modulus")?;
    if p >= 1 << 31 {
        return Err("PKM base exceeds native adapter modulus bound".into());
    }
    let point = |name: &str| -> Result<PrimePoint, String> {
        match group.point(&public[name])? {
            Element::Prime(Point::Affine { x, y }) => Ok(PrimePoint::affine(
                x.value.to_u64().ok_or("PKM coordinate exceeds u64")?,
                y.value.to_u64().ok_or("PKM coordinate exceeds u64")?,
            )),
            _ => Err("PKM public point must be affine".into()),
        }
    };
    let generator = point("generator")?;
    let target = point("target")?;
    let r = group.order().ok_or("missing PKM subgroup order")?;
    let cofactor = public["curve"]["cofactor"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid PKM cofactor")?;
    let instance = PrimeInstance {
        name: "dataset-prime-kummer-verification".into(),
        curve: PrimeCurve {
            p,
            a: curve.a.to_u64().ok_or("invalid PKM coefficient")?,
            b: curve.b.to_u64().ok_or("invalid PKM coefficient")?,
        },
        group_order: r.checked_mul(cofactor).ok_or("PKM order overflow")?,
        r,
        cofactor,
        generator: (generator.x, generator.y),
    };
    let ctx = InstanceCtx {
        group: &instance.curve,
        generator,
        target,
        r,
        cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };
    let mut params = Params::default();
    params
        .set("t", definition["t"].as_u64().unwrap().to_string())
        .set("g", definition["g"].as_str().unwrap());
    let base = PrimeKummerBase {
        instance: &instance,
    }
    .build(&ctx, &params, &mut GroupOps::default())?;
    let expected_points: Vec<_> = base
        .points
        .iter()
        .map(|p| json!({"x":p.x.to_string(),"y":p.y.to_string()}))
        .collect();
    if json!(actual_points.iter().map(Element::json).collect::<Vec<_>>()) != json!(expected_points)
        || artifact["columns"] != json!(base.columns)
        || artifact["column_of"] != json!(base.col_of)
        || artifact["coefficient_of"]
            != json!(base
                .coef_of
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>())
        || artifact["abscissae"]
            != json!(base
                .abscissa_list
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>())
    {
        return Err(
            "PKM domain or column/sign map differs from native public reconstruction".into(),
        );
    }
    Ok(())
}

fn pkm_solve(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    use crate::cryptanalysis::ic_boundary::{
        GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
    };
    use crate::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
    use crate::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
    use crate::cryptanalysis::ic_framework::stages::{
        DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
    };
    let Group::Prime(curve) = group else {
        return Ok(unsupported(
            "PKM Kummer-tower adapter requires a prime curve",
        ));
    };
    let p = curve.p.to_u64().ok_or("PKM modulus exceeds u64")?;
    if p >= 1 << 31 || !(2..=3).contains(&config.m) {
        return Ok(unsupported(
            "Native PKM tower currently supports only m=2 or 3 and p<2^31",
        ));
    }
    let Some(g) = domain
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .filter(|&x| x != 0)
        .min()
    else {
        return Ok(unsupported("PKM base needs a nonzero source abscissa"));
    };
    let mut t = ((p - 1).trailing_zeros()).min(4);
    while t > 1 && (1u64 << t).pow(config.m as u32) > config.work_limit.min(TRACE_EVENT_CAP) {
        t -= 1;
    }
    if (1u64 << t).pow(config.m as u32) > config.work_limit {
        return Ok(SolverData {
            outcome: outcome("budget_exhausted", "Minimum PKM grid exceeds work limit"),
            artifacts: vec![],
        });
    }
    let word_point = |element: &Element| -> Result<PrimePoint, String> {
        match element {
            Element::Prime(Point::Affine { x, y }) => Ok(PrimePoint::affine(
                x.value.to_u64().ok_or("x too wide")?,
                y.value.to_u64().ok_or("y too wide")?,
            )),
            Element::Prime(Point::Infinity) => Ok(PrimePoint::INFINITY),
            _ => Err("wrong group".into()),
        }
    };
    let generator = word_point(&group.point(&case.public_instance["generator"])?)?;
    let q = word_point(target)?;
    if q.infinity {
        return Ok(unsupported("Native PKM oracle requires an affine target"));
    }
    let r = group.order().ok_or("missing subgroup order")?;
    let cofactor = case.public_instance["curve"]["cofactor"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid cofactor")?;
    let instance = PrimeInstance {
        name: "dataset-prime-kummer".into(),
        curve: PrimeCurve {
            p,
            a: curve.a.to_u64().unwrap(),
            b: curve.b.to_u64().unwrap(),
        },
        group_order: r.checked_mul(cofactor).ok_or("order overflow")?,
        r,
        cofactor,
        generator: (generator.x, generator.y),
    };
    let ctx = InstanceCtx {
        group: &instance.curve,
        generator,
        target: q,
        r,
        cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };
    let mut params = Params::default();
    params
        .set("t", t.to_string())
        .set("g", g.to_string())
        .set("degree", config.degree_bound.min(12).to_string())
        .set(
            "max_nnz",
            config
                .work_limit
                .min(TRACE_EVENT_CAP)
                .saturating_mul(64)
                .to_string(),
        );
    let mut ops = GroupOps::default();
    let base = PrimeKummerBase {
        instance: &instance,
    }
    .build(&ctx, &params, &mut ops)?;
    let base_points: Vec<Element> = base
        .points
        .iter()
        .map(|p| group.point(&json!({"x":p.x.to_string(),"y":p.y.to_string()})))
        .collect::<Result<_, _>>()?;
    let definition = json!({"kind":"prime-kummer","t":t,"g":g.to_string(),"points":base_points.iter().map(Element::json).collect::<Vec<_>>(),"subgroup_filtered":false,"variant":"rational Kummer points on the same full curve; separate from small-x-cofactor-projection","known_logs":false});
    verify_kummer_domain(group, case, &definition)?;
    let mut data=SolverData{outcome:outcome("partial","Native PKM Kummer-tower algebra and bounded grid extraction ran on an explicitly separate factor base"),artifacts:vec![]};
    save(
        &mut data,
        "pkm-factor-base.json",
        "explicit-alternative-kummer-factor-base",
        &json!({"definition":definition,"columns":base.columns,"column_of":base.col_of,"coefficient_of":base.coef_of.iter().map(ToString::to_string).collect::<Vec<_>>(),"construction_cost":base.cost,"abscissae":base.abscissa_list.iter().map(ToString::to_string).collect::<Vec<_>>()}),
    )?;
    let mut oracle =
        PkmSemaevOracle::new(config.m as u32, t, g).with_budget(Some(Duration::from_secs(5)));
    oracle.enable_dataset_trace();
    oracle.prepare(&ctx, &base, &params, &mut ops)?;
    if let Some(snapshot) = oracle.prepared_system_artifact() {
        save(
            &mut data,
            "pkm-prepared-system.json",
            "exact-prepared-pkm-system-before-solve",
            &snapshot,
        )?;
    }
    live_event(
        &json!({"kind":"pkm_native_solve_begin","m":config.m,"t":t,"grid_bound":(1u64<<t).pow(config.m as u32)}),
    )?;
    let mut counters = OracleCounters::default();
    let found = oracle.decompose(&ctx, &base, &mut ops, &mut counters, q);
    if let Some(snapshot) = oracle.dataset_artifact() {
        save(
            &mut data,
            "pkm-algebra.json",
            "native-pkm-exact-system-and-solver-artifacts",
            snapshot,
        )?;
    }
    data.outcome["factor_base_variant"] = json!("prime-kummer");
    data.outcome["solver_completed"] = json!(true);
    data.outcome["counters"] = json!({"native_group_ops":ops,"native_setup":oracle.setup_native(),"native_relation_counters":oracle.relation_counters(),"native_solver_totals":oracle.solver_totals(),"lift_failures":counters.lift_failures,"unliftable_systems":counters.unliftable_systems});
    data.outcome["trace_gaps"]=json!(["Native F4 report exports aggregate step traces and final basis, not every private pivot or rejected pair","PKM includes separately counted finite-grid fallback after inconclusive F4","Kummer factor base includes rational points outside the target subgroup; this is PDP-only evidence"]);
    if let Some(indices) = found {
        let witness = indices
            .iter()
            .map(|&i| {
                base_points
                    .get(i)
                    .cloned()
                    .ok_or("PKM returned out-of-range index")
            })
            .collect::<Result<Vec<_>, _>>()?;
        set_witness_with_definition(&mut data, group, &witness, target, Some(&definition))?;
        let payload = data
            .artifacts
            .iter_mut()
            .find(|a| a.role == "independently-verified-pdp")
            .ok_or("missing PKM witness artifact")?;
        let label: Value = serde_json::from_slice(&payload.bytes).map_err(|e| e.to_string())?;
        verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            &payload.role,
            &label,
        )?;
    }
    Ok(data)
}

const PRIME_CHAIN_STRATEGY: &str = "prime-affine-s3-chain-f4-visitor/v1";
const PRIME_CHAIN_TRACE_CAP: &str = "prime F4 chain event or byte limit reached";
const DYNAMIC_STRATEGY: &str = "prime_f4_wide_dynamic";

fn retains_canonical_artifacts(data: &SolverData) -> bool {
    data.outcome["strategy"] == PRIME_CHAIN_STRATEGY
        || data.outcome["strategy"] == prime_f4_wide::STRATEGY
        || data.outcome["strategy"] == prime_f4_wide_m3::STRATEGY
        || data.outcome["strategy"] == DYNAMIC_STRATEGY
}
fn record_prime_chain_persistence_error(data: &mut SolverData, phase: &str, error: &str) {
    if data.outcome["strategy"] == prime_f4_wide::STRATEGY
        || data.outcome["strategy"] == prime_f4_wide_m3::STRATEGY
        || data.outcome["strategy"] == DYNAMIC_STRATEGY
    {
        data.outcome["solver_completed"] = json!(false);
        data.outcome["status"] = json!("partial");
        if data.outcome.get("live_persistence_errors").is_some() {
            return;
        }
    }
    data.outcome["live_persistence_complete"] = json!(false);
    data.outcome["exposed_event_stream_complete"] = json!(false);
    data.outcome["canonical_artifact_write_required"] = json!(true);
    data.outcome["persistence_scope"] = json!("Live persistence failed; staged artifacts require a separate successful canonical worker write. Total storage failure does not establish durability.");
    if data.outcome.get("live_persistence_errors").is_none() {
        data.outcome["live_persistence_errors"] = json!([]);
    }
    data.outcome["live_persistence_errors"]
        .as_array_mut()
        .unwrap()
        .push(json!({"phase":phase,"error":error.chars().take(512).collect::<String>()}));
}
fn flush_prime_chain_payloads(data: &mut SolverData) {
    if data.outcome.get("live_persistence_errors").is_some() {
        return;
    }
    let dynamic = data.outcome["strategy"] == DYNAMIC_STRATEGY;
    let failure = data.artifacts.iter().find_map(|payload| {
        let persisted = if dynamic {
            // This exact artifact has already crossed dedicated publication and
            // final confirmation. Ordinary capture must never republish it.
            if payload.path == "pdp-witness.json"
                && payload.role == "independently-verified-pdp"
                && payload.encoding == "json"
            {
                return None;
            }
            live_borrowed_payload(
                &payload.path,
                &payload.role,
                &payload.encoding,
                &payload.bytes,
            )
        } else {
            live_payload(payload)
        };
        persisted.err().map(|error| (payload.path.clone(), error))
    });
    if let Some((path, error)) = failure {
        record_prime_chain_persistence_error(data, &path, &error);
    }
}

#[derive(Clone, Copy)]
struct PrimeChainLimits {
    events: u64,
    bytes: usize,
    lift_additions: u64,
    time: Duration,
}
impl From<&CaseConfig> for PrimeChainLimits {
    fn from(config: &CaseConfig) -> Self {
        Self {
            events: config.work_limit.min(TRACE_EVENT_CAP),
            bytes: TRACE_BYTE_CAP / 4,
            lift_additions: config.work_limit.min(TRACE_EVENT_CAP),
            time: Duration::from_secs(5),
        }
    }
}

/// Bound serialization itself, before constructing an owned JSON tree. Native
/// F4 still materializes its own rows; this is not a native matrix memory cap.
struct LimitedJsonWriter {
    bytes: Vec<u8>,
    limit: usize,
}
impl Write for LimitedJsonWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(std::io::Error::other(PRIME_CHAIN_TRACE_CAP));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Default)]
struct PrimeChainEvents {
    bytes: Vec<u8>,
    events: u64,
    error: Option<String>,
}
impl PrimeChainEvents {
    fn append<T: serde::Serialize>(
        &mut self,
        event: &T,
        limits: PrimeChainLimits,
    ) -> Result<(), String> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        let result: Result<(), String> = (|| {
            if self.events >= limits.events || self.bytes.len() >= limits.bytes {
                return Err(PRIME_CHAIN_TRACE_CAP.into());
            }
            #[derive(serde::Serialize)]
            struct Row<'a, T> {
                schema: &'static str,
                sequence: u64,
                event: &'a T,
            }
            let row = Row {
                schema: "ml-dataset.prime-f4-chain-event/v1",
                sequence: self.events + 1,
                event,
            };
            let mut writer = LimitedJsonWriter {
                bytes: vec![],
                limit: limits.bytes.saturating_sub(self.bytes.len() + 1),
            };
            serde_json::to_writer(&mut writer, &row).map_err(|e| {
                if e.is_io() {
                    PRIME_CHAIN_TRACE_CAP.to_string()
                } else {
                    format!("serialize prime F4 trace: {e}")
                }
            })?;
            // Only this bounded event is materialized for the existing sink.
            let value: Value = serde_json::from_slice(&writer.bytes).map_err(|e| e.to_string())?;
            live_event(&value)?;
            self.bytes.extend_from_slice(&writer.bytes);
            self.bytes.push(b'\n');
            self.events += 1;
            Ok(())
        })();
        if let Err(error) = &result {
            self.error = Some(error.clone());
        }
        result
    }
}

fn prime_chain_polynomials(
    m: usize,
    p: u64,
    a: u64,
    b: u64,
    target_x: u64,
    codes: &[u64],
) -> Result<Vec<crate::cryptanalysis::f4_fp::Poly>, String> {
    use crate::cryptanalysis::groebner_f4::{cmp_monomial, Ordering};
    if !(3..=15).contains(&m)
        || p < 5
        || [a, b, target_x].iter().any(|&x| x >= p)
        || codes.is_empty()
        || codes.len() > 32
        || codes.iter().any(|&x| x >= p)
        || codes.windows(2).any(|w| w[0] >= w[1])
    {
        return Err("invalid prime affine-chain parameters or canonical domain".into());
    }
    let n_vars = 2 * m - 2;
    let mul = |x: u64, y: u64| ((u128::from(x) * u128::from(y)) % u128::from(p)) as u64;
    let add = |x: u64, y: u64| ((u128::from(x) + u128::from(y)) % u128::from(p)) as u64;
    let neg = |x: u64| if x == 0 { 0 } else { p - x };
    let canonical = |terms: Vec<(Vec<u32>, u64)>| {
        let mut merged = std::collections::BTreeMap::new();
        for (exponents, coefficient) in terms {
            let old = merged.entry(exponents).or_insert(0u64);
            *old = add(*old, coefficient);
        }
        let mut terms: Vec<_> = merged.into_iter().filter(|(_, c)| *c != 0).collect();
        terms.sort_by(|x, y| cmp_monomial(&y.0, &x.0, Ordering::Grevlex));
        terms
    };
    // Symmetric expansion of the odd-characteristic S3 formula. None fixes
    // that local coordinate to the public target x; no native narrow reducer.
    let s3 = |variables: [Option<usize>; 3]| {
        let raw = [
            ([2, 2, 0], 1),
            ([2, 0, 2], 1),
            ([0, 2, 2], 1),
            ([2, 1, 1], neg(2)),
            ([1, 2, 1], neg(2)),
            ([1, 1, 2], neg(2)),
            ([1, 1, 0], neg(mul(2, a))),
            ([1, 0, 1], neg(mul(2, a))),
            ([0, 1, 1], neg(mul(2, a))),
            ([1, 0, 0], neg(mul(4, b))),
            ([0, 1, 0], neg(mul(4, b))),
            ([0, 0, 1], neg(mul(4, b))),
            ([0, 0, 0], mul(a, a)),
        ];
        canonical(
            raw.into_iter()
                .map(|(powers, mut coefficient)| {
                    let mut exponents = vec![0; n_vars];
                    for (axis, &power) in powers.iter().enumerate() {
                        if let Some(variable) = variables[axis] {
                            exponents[variable] += power;
                        } else {
                            for _ in 0..power {
                                coefficient = mul(coefficient, target_x);
                            }
                        }
                    }
                    (exponents, coefficient)
                })
                .collect(),
        )
    };
    let mut polynomials = vec![s3([Some(0), Some(1), Some(m)])];
    for auxiliary in 1..m - 2 {
        polynomials.push(s3([
            Some(m + auxiliary - 1),
            Some(auxiliary + 1),
            Some(m + auxiliary),
        ]));
    }
    polynomials.push(s3([Some(2 * m - 3), Some(m - 1), None]));
    let mut membership = vec![1u64];
    for &x in codes {
        let mut next = vec![0; membership.len() + 1];
        for (power, &coefficient) in membership.iter().enumerate() {
            next[power] = add(next[power], neg(mul(coefficient, x)));
            next[power + 1] = add(next[power + 1], coefficient);
        }
        membership = next;
    }
    for leaf in 0..m {
        polynomials.push(canonical(
            membership
                .iter()
                .enumerate()
                .map(|(power, &coefficient)| {
                    let mut exponents = vec![0; n_vars];
                    exponents[leaf] = power as u32;
                    (exponents, coefficient)
                })
                .collect(),
        ));
    }
    Ok(polynomials)
}

struct PrimeChainInput {
    modulus: u64,
    polynomials: Vec<crate::cryptanalysis::f4_fp::Poly>,
    domains: Vec<Option<Vec<u64>>>,
    artifact: Value,
}
fn prime_chain_input(
    m: usize,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<PrimeChainInput, String> {
    let Group::Prime(curve) = group else {
        return Err("prime chain requires a prime curve".into());
    };
    let p = curve.p.to_u64().ok_or("prime chain modulus exceeds u64")?;
    let a = curve
        .a
        .to_u64()
        .ok_or("prime chain coefficient exceeds u64")?;
    let b = curve
        .b
        .to_u64()
        .ok_or("prime chain coefficient exceeds u64")?;
    let target_x = target
        .x()
        .and_then(|x| x.to_u64())
        .ok_or("prime chain requires an affine target")?;
    let mut codes: Vec<_> = domain
        .iter()
        .map(|point| {
            point
                .x()
                .and_then(|x| x.to_u64())
                .ok_or("prime chain domain is not affine u64")
        })
        .collect::<Result<_, _>>()?;
    codes.sort_unstable();
    codes.dedup();
    let polynomials = prime_chain_polynomials(m, p, a, b, target_x, &codes)?;
    let domains: Vec<_> = (0..2 * m - 2)
        .map(|i| (i < m).then(|| codes.clone()))
        .collect();
    let variables: Vec<_> = (0..m)
        .map(|i| format!("x{i}"))
        .chain((0..m - 2).map(|i| format!("u{i}")))
        .collect();
    let roles: Vec<_> = (0..2*m-2).map(|i|json!({"index":i,"name":variables[i],"role":if i<m{"factor_base_leaf"}else{"unrestricted_affine_auxiliary"},"domain":if i<m{"factor_base_x"}else{"all_Fp"}})).collect();
    let artifact = json!({
        "schema":"ml-dataset.prime-f4-chain-input/v1","strategy":PRIME_CHAIN_STRATEGY,
        "m":m,"n_vars":2*m-2,"modulus":p.to_string(),"curve":{"a":a.to_string(),"b":b.to_string()},
        "target":target.json(),"target_x":target_x.to_string(),"ring":"Fp[x0,...,x(m-1),u0,...,u(m-3)]",
        "variables":variables,"variable_roles":roles,"monomial_order":"degrevlex",
        "domains":domains,"factor_base_x":codes.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "signed_factor_base":domain.iter().map(Element::json).collect::<Vec<_>>(),
        "polynomials":polynomials.iter().map(|f|f.iter().map(|(e,c)|json!({"exponents":e,"coefficient":c.to_string()})).collect::<Vec<_>>()).collect::<Vec<_>>(),
        "chain_equation_count":m-1,"membership_equation_count":m,"membership_variables":(0..m).collect::<Vec<_>>(),
        "native_normalization_applied":false,"coefficient_arithmetic":"u128 modular intermediates; merged nonzero terms, leading-first, no monic rescaling",
        "native_modulus_limit_exclusive":"4294967296","affine_chain_exceptional_infinity_branches_covered":false,
        "negative_scope":"encoding-relative only; no exhaustive PDP nonexistence claim"
    });
    Ok(PrimeChainInput {
        modulus: p,
        polynomials,
        domains,
        artifact,
    })
}

/// Reconstruct the public chain encoding and domain map, without solving or
/// consulting construction witnesses. Unknown metadata cannot replace a field.
pub(super) fn verify_prime_f4_chain_input(
    public: &Value,
    factor_base: &Value,
    artifact: &Value,
) -> Result<(), String> {
    let m = public["m"]
        .as_u64()
        .and_then(|m| usize::try_from(m).ok())
        .ok_or("invalid public chain arity")?;
    let case = CaseData {
        public_instance: public.clone(),
        factor_base: factor_base.clone(),
        private_witness: Value::Null,
    };
    let group = Group::read(&case)?;
    let target = group.point(&public["target"])?;
    if !group.on_curve(&target) {
        return Err("prime chain target is off curve".into());
    }
    let domain = domain(&group, &case)?;
    let expected = prime_chain_input(m, &group, &domain, &target)?;
    for (key, value) in expected
        .artifact
        .as_object()
        .ok_or("invalid internal chain schema")?
    {
        if artifact.get(key) != Some(value) {
            return Err(format!("prime F4 chain input differs at {key}"));
        }
    }
    Ok(())
}

fn prime_f4_chain(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: PrimeChainLimits,
) -> Result<SolverData, String> {
    use crate::cryptanalysis::f4_fp::{self, F4Options};
    use crate::cryptanalysis::groebner_f4::Ordering;
    if target.x().is_none() {
        return Ok(unsupported(
            "Affine S3 chain requires an affine target; infinity branches are not encoded",
        ));
    }
    let input = match prime_chain_input(config.m, group, domain, target) {
        Ok(input) => input,
        Err(error) => return Ok(unsupported(&error)),
    };
    let mut data = SolverData {
        outcome: outcome("partial", "Affine S3-chain F4 search is encoding-relative"),
        artifacts: vec![],
    };
    data.outcome["strategy"] = json!(PRIME_CHAIN_STRATEGY);
    data.outcome["exhaustive_no_decomposition"] = json!(false);
    data.outcome["native_backend_invoked"] = json!(false);
    data.outcome["algebra_completed"] = json!(false);
    data.outcome["encoding_search_completed"] = json!(false);
    // Retain the canonical equations before attempting the bounded live sink.
    // A rejected first payload must not erase already constructed mathematics.
    data.artifacts.push(ArtifactPayload::json(
        "prime-f4-input.json",
        "complete-prime-field-polynomial-system",
        &input.artifact,
    )?);
    flush_prime_chain_payloads(&mut data);
    if data.outcome.get("live_persistence_errors").is_some() {
        data.outcome["termination"] = json!("trace_sink_error");
        data.outcome["reason"] = json!("Initial live equation persistence failed; equations retained for canonical worker write and native elimination not invoked");
        return Ok(data);
    }
    if input.modulus >= 1u64 << 32 {
        data.outcome["status"] = json!("unsupported");
        data.outcome["reason"]=json!("Native prime F4 requires p<2^32; exact affine-chain and leaf-membership equations retained using u128 arithmetic without invoking elimination");
        data.outcome["termination"] = json!("unsupported_native_modulus");
        return Ok(data);
    }
    let mut opts =
        F4Options::new(Ordering::Grevlex, config.degree_bound.min(12)).with_budget(limits.time);
    opts.max_pairs_per_step = Some(config.work_limit.clamp(1, 256) as usize);
    let events = RefCell::new(PrimeChainEvents::default());
    let mut found = None;
    let mut lift_additions = 0u64;
    let mut lift_candidates = 0u64;
    let mut lift_exhausted = false;
    let mut truncated_native_calls = 0usize;
    let mut native_pairs_above_bound = 0usize;
    let report = f4_fp::solve_visit_traced(
        &input.polynomials,
        input.domains.len(),
        input.modulus,
        &opts,
        &input.domains,
        |assignment| {
            lift_candidates += 1;
            let xs: Vec<_> = assignment[..config.m]
                .iter()
                .copied()
                .map(BigUint::from)
                .collect();
            let (witness, adds, exhausted) = lift(
                group,
                domain,
                &xs,
                target,
                limits.lift_additions.saturating_sub(lift_additions),
            );
            lift_additions += adds;
            lift_exhausted |= exhausted;
            // Preserve the independently verified positive before a fallible
            // observation write: trace censoring does not invalidate a witness.
            if witness.is_some() {
                found = witness;
            }
            let row = json!({"kind":"group_lift","candidate":lift_candidates,"leaf_x":assignment[..config.m],"group_lift_verified":found.is_some(),"group_add_calls":adds,"lift_budget_exhausted":exhausted});
            let trace_failed = events.borrow_mut().append(&row, limits).is_err();
            found.is_some() || exhausted || trace_failed
        },
        |event| {
            if let f4_fp::VisitEvent::NativeResult { report, trace, .. } = &event {
                native_pairs_above_bound =
                    native_pairs_above_bound.saturating_add(report.pairs_above_bound);
                if report.pairs_above_bound > 0
                    || report.timed_out
                    || report.staircase_at_stop.is_some()
                    || trace.stopped
                {
                    truncated_native_calls += 1;
                }
            }
            events.borrow_mut().append(&event, limits)
        },
    )?;
    let events = events.into_inner();
    let trace_error = events.error.as_deref().or(report.trace_error.as_deref());
    let termination = if trace_error == Some(PRIME_CHAIN_TRACE_CAP) {
        "trace_budget_exhausted"
    } else if trace_error.is_some() {
        "trace_sink_error"
    } else if lift_exhausted {
        "lift_budget_exhausted"
    } else if report.timed_out {
        "native_deadline"
    } else if found.is_some() {
        "verified_positive_stop"
    } else if report.complete {
        "encoding_search_complete"
    } else {
        "native_incomplete"
    };
    let summary = json!({"schema":"ml-dataset.prime-f4-chain-result/v1","strategy":PRIME_CHAIN_STRATEGY,
        "termination":termination,"native_report":report,"trace_error":trace_error,"trace_events":events.events,"trace_bytes":events.bytes.len(),
        "limits":{"events":limits.events,"bytes":limits.bytes,"group_lift_add_calls":limits.lift_additions,"native_deadline_milliseconds":limits.time.as_millis(),"degree_bound":opts.max_degree,"pairs_per_batch":opts.max_pairs_per_step},
        "limit_scope":"work_limit bounds emitted events and separate group-lift additions; pair limit is batch width, not total native work; outer supervisor also bounds process lifetime/memory",
        "group_lift_candidates":lift_candidates,"group_lift_add_calls":lift_additions,"group_lift_budget_exhausted":lift_exhausted,
        "native_calls_with_truncation_or_stop":truncated_native_calls,"native_pairs_above_degree_bound":native_pairs_above_bound,
        "encoding_search_completed":report.complete,"encoding_completion_implies_complete_groebner_basis":false,
        "pdp_witness_verified":found.is_some(),"exhaustive_no_decomposition":false,
        "trace_gaps":["Useful-row traces omit zero rows and private pivot operations","Internal univariate factorization/GCD states are not exposed","Native rows are materialized before bounded event serialization; no whole-call checkpoint recovery","Observed degrees are not exact first/last fall degrees"]});
    let payload = ArtifactPayload {
        path: "prime-f4-chain-events.jsonl".into(),
        role: "restricted-native-prime-f4-recursion-and-lift-events".into(),
        encoding: "jsonl".into(),
        bytes: events.bytes,
    };
    data.artifacts.push(payload);
    data.artifacts.push(ArtifactPayload::json(
        "prime-f4-chain-result.json",
        "restricted-native-prime-f4-chain-termination-and-counters",
        &summary,
    )?);
    data.outcome["native_backend_invoked"] = json!(report.f4_runs > 0);
    data.outcome["termination"] = json!(termination);
    data.outcome["encoding_search_completed"] = json!(report.complete);
    data.outcome["algebra_completed"] = json!(
        report.complete
            && report.f4_runs > 0
            && truncated_native_calls == 0
            && trace_error.is_none()
    );
    data.outcome["algebra_completed_scope"]=json!("All visited native bases and the full encoding traversal completed; distinct from encoding_search_completed, which can hold after degree-truncated elimination");
    data.outcome["lift_completed"] = json!(!lift_exhausted);
    data.outcome["lift_completed_scope"]=json!("Every attempted candidate lift completed within its own budget; does not assert all roots or PDP decompositions were attempted");
    data.outcome["exposed_event_stream_complete"] = json!(trace_error.is_none());
    data.outcome["trace_gaps"] = summary["trace_gaps"].clone();
    data.outcome["counters"] = json!({"recursive_native_f4_runs":report.f4_runs,"native_elimination_fp_multiplies":report.field_ops,"native_domain_evaluations":report.domain_evaluations,"native_factored_univariates":report.factored_univariates,"native_root_finder_coefficient_multiplications":report.root_field_multiplications,"native_full_field_branches":report.full_field_branches,"equation_checked_candidates":report.solutions_visited,"group_lift_candidates":lift_candidates,"group_lift_add_calls":lift_additions,"persisted_trace_events":events.events});
    if let Some(witness) = found {
        let mut sum = group.zero();
        for point in &witness {
            if !group.on_curve(point) || !domain.contains(point) {
                return Err("prime-chain witness outside actual signed domain".into());
            }
            sum = group.add(&sum, point);
        }
        if &sum != target || witness.len() != config.m {
            return Err("prime-chain witness group replay failed".into());
        }
        data.outcome["status"] = json!("verified");
        data.outcome["pdp_witness_verified"] = json!(true);
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("A signed factor-base decomposition was independently replayed in the curve group; trace persistence is reported separately");
        let payload = ArtifactPayload::json(
            "pdp-witness.json",
            "independently-verified-pdp",
            &json!({"summands":witness.iter().map(Element::json).collect::<Vec<_>>(),"target":target.json(),"sum":sum.json(),"replay_group_add_calls":witness.len()}),
        )?;
        // Place the certificate before final trace artifacts for successful
        // flushes; construction is independent of the live stream's health.
        data.artifacts.insert(1, payload);
    } else if report.complete && trace_error.is_none() && !lift_exhausted {
        data.outcome["solver_completed"] = json!(true);
        data.outcome["reason"]=json!("The supplied affine-chain encoding search completed without a verified lift; exceptional infinity branches prevent a general PDP nonexistence claim");
    } else {
        data.outcome["status"] = json!(if termination == "trace_sink_error" {
            "partial"
        } else {
            "budget_exhausted"
        });
        data.outcome["reason"] = json!(format!(
            "Affine-chain search stopped with {termination}; no PDP nonexistence claim"
        ));
    }
    if termination == "trace_sink_error" {
        record_prime_chain_persistence_error(
            &mut data,
            "recursive_event_stream",
            trace_error.unwrap_or("unknown trace sink error"),
        );
    }
    Ok(data)
}

fn prime_f4(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
    context: Option<&super::execution::SolverExecutionContext>,
) -> Result<SolverData, String> {
    use crate::cryptanalysis::f4_fp::{self, F4Options};
    use crate::cryptanalysis::groebner_f4::Ordering;
    if config.m == 3
        && context.is_some()
        && matches!(group, Group::Prime(curve) if curve.p.to_u64().is_some_and(|p| p >= 1u64 << 32))
    {
        return prime_f4_wide_m3::solve(config, group, domain, target, context);
    }
    if config.m != 2 {
        return prime_f4_chain(
            config,
            group,
            domain,
            target,
            PrimeChainLimits::from(config),
        );
    }
    let Group::Prime(curve) = group else {
        unreachable!()
    };
    let Some(xr) = target.x().and_then(|x| x.to_u64()) else {
        return Ok(unsupported("Prime F4 needs an affine target"));
    };
    let p = curve.p.to_u64().ok_or("modulus exceeds u64")?;
    if p >= 1u64 << 32 && context.is_some() {
        return prime_f4_wide::solve(config, group, domain, target, context);
    }
    let a = curve.a.to_u64().ok_or("a exceeds u64")?;
    let b = curve.b.to_u64().ok_or("b exceeds u64")?;
    let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    let add = |x: u64, y: u64| ((x as u128 + y as u128) % p as u128) as u64;
    let neg = |x: u64| if x == 0 { 0 } else { p - x };
    let r2 = mul(xr, xr);
    let linear = neg(add(mul(2, mul(a, xr)), mul(4, b)));
    let terms = vec![
        (vec![2, 2], 1),
        (vec![2, 1], neg(mul(2, xr))),
        (vec![1, 2], neg(mul(2, xr))),
        (vec![2, 0], r2),
        (vec![0, 2], r2),
        (vec![1, 1], neg(mul(2, add(r2, a)))),
        (vec![1, 0], linear),
        (vec![0, 1], linear),
        (vec![0, 0], add(mul(a, a), neg(mul(4, mul(b, xr))))),
    ];
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .filter_map(|x| x.to_u64())
        .collect();
    codes.sort();
    codes.dedup();
    if codes.is_empty() || codes.len() > 32 {
        return Ok(unsupported(
            "Prime F4 domain must contain 1..32 distinct abscissae",
        ));
    }
    let mut member = vec![1u64];
    for &x in &codes {
        let mut next = vec![0; member.len() + 1];
        for (i, &c) in member.iter().enumerate() {
            next[i] = add(next[i], neg(mul(c, x)));
            next[i + 1] = add(next[i + 1], c);
        }
        member = next;
    }
    let order = Ordering::Grevlex;
    // f4_fp::Barrett::new asserts this narrower bound, even though some
    // polynomial helpers support larger u64 primes. Preserve the public
    // equations without entering any native elimination for unsupported p.
    let backend_applicable = (2..1u64 << 32).contains(&p);
    let normalize = |terms: &[(Vec<u32>, u64)]| {
        if backend_applicable {
            f4_fp::normalise(terms, p, order)
        } else {
            terms.iter().filter(|(_, c)| *c != 0).cloned().collect()
        }
    };
    let mut input = vec![normalize(&terms)];
    for axis in 0..2 {
        input.push(normalize(
            &member
                .iter()
                .enumerate()
                .map(|(d, &c)| {
                    let mut e = vec![0, 0];
                    e[axis] = d as u32;
                    (e, c)
                })
                .collect::<Vec<_>>(),
        ));
    }
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Native prime F4 was run on S3 plus exact finite-domain membership polynomials",
        ),
        artifacts: vec![],
    };
    let encode = |polys: &[f4_fp::Poly]| {
        json!(polys
            .iter()
            .map(|f| f
                .iter()
                .map(|(e, c)| json!({"exponents":e,"coefficient":c.to_string()}))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
    };
    save(
        &mut data,
        "prime-f4-input.json",
        "complete-prime-field-polynomial-system",
        &json!({"modulus":p.to_string(),"variables":["x0","x1"],"monomial_order":"degrevlex","native_normalization_applied":backend_applicable,"native_modulus_limit_exclusive":"4294967296","polynomials":encode(&input),"factor_base_x":codes.iter().map(ToString::to_string).collect::<Vec<_>>(),"target_x":xr.to_string()}),
    )?;
    if !backend_applicable {
        data.outcome = outcome(
            "unsupported",
            "Native prime F4 elimination requires 2 <= p < 2^32; the public S3 and exact finite-domain membership system was retained without running F4",
        );
        data.outcome["native_backend_invoked"] = json!(false);
        data.outcome["algebra_completed"] = json!(false);
        data.outcome["native_modulus_limit_exclusive"] = json!("4294967296");
        return Ok(data);
    }
    let mut opts = F4Options::new(order, config.degree_bound.min(12))
        .with_budget(Duration::from_secs(5))
        .capturing_run_log();
    opts.max_pairs_per_step = Some(config.work_limit.clamp(1, 256) as usize);
    live_event(&json!({"kind":"prime_f4_native_begin","degree_bound":opts.max_degree,"n_vars":2}))?;
    let (report, trace) = f4_fp::f4_record(&input, 2, p, &opts);
    data.outcome["native_backend_invoked"] = json!(true);
    let log = f4_fp::take_run_log();
    save(
        &mut data,
        "prime-f4-output.json",
        "native-f4-basis-useful-row-trace-and-counters",
        &json!({"basis":encode(&report.basis),"inconsistent":report.inconsistent,"degree_reached":report.degree_reached,"observed_productive_degree_max":report.solving_degree_max,"pairs_above_bound":report.pairs_above_bound,"timed_out":report.timed_out,"max_rows":report.max_rows,"max_cols":report.max_cols,"elimination_fp_multiplies":report.field_ops,
        "trace":{"input_leading_monomials":trace.input_lms,"stopped":trace.stopped,"steps":trace.steps.iter().map(|s|json!({"degree":s.degree,"contributing_rows_and_multipliers":s.rows,"new_leading_monomials":s.new_lms})).collect::<Vec<_>>()},
        "step_log":log.steps.iter().map(|s|json!({"degree":s.degree,"pairs":s.pairs,"rows":s.rows,"all_rows":s.all_rows,"ops":s.ops,"new_polynomials":s.new_polys,"staircase":s.staircase})).collect::<Vec<_>>()}),
    )?;
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut tests = 0;
    let mut lift_ops = 0;
    let mut checks = vec![];
    let mut found = None;
    'grid: for &x in &codes {
        for &y in &codes {
            if tests >= limit || lift_ops >= limit {
                break 'grid;
            }
            tests += 1;
            let is_root = input.iter().all(|f| f4_fp::eval(f, &[x, y], p) == 0);
            if !is_root {
                checks
                    .push(json!({"x":[x.to_string(),y.to_string()],"original_system_root":false}));
                continue;
            }
            if report.inconsistent {
                return Err("Native prime F4 refutation contradicted by independently evaluated finite-domain root".into());
            }
            let (w, adds, exhausted) = lift(
                group,
                domain,
                &[BigUint::from(x), BigUint::from(y)],
                target,
                limit.saturating_sub(lift_ops),
            );
            lift_ops += adds;
            checks.push(json!({"x":[x.to_string(),y.to_string()],"original_system_root":true,"group_lift_verified":w.is_some(),"lift_budget_exhausted":exhausted}));
            if w.is_some() {
                found = w;
                break 'grid;
            }
        }
    }
    save(
        &mut data,
        "prime-f4-root-extraction.json",
        "bounded-finite-domain-root-extraction-after-f4",
        &json!(checks),
    )?;
    data.outcome["counters"] = json!({"native_elimination_fp_multiplies":report.field_ops,"finite_domain_root_tests":tests,"group_lift_add_calls":lift_ops});
    data.outcome["algebra_completed"] = json!(!report.timed_out && report.pairs_above_bound == 0);
    data.outcome["trace_gaps"]=json!(["Useful-row replay schedule omits zero rows and private pivots; not a complete event stream","Root extraction is separately counted finite-domain enumeration, not native F4 substitution"]);
    if let Some(w) = found {
        set_witness(&mut data, group, &w, target)?;
    } else if tests < (codes.len() * codes.len()) as u64 || lift_ops >= limit {
        data.outcome["status"] = json!("budget_exhausted");
    } else {
        data.outcome["solver_completed"] = json!(true);
        data.outcome["exhaustive_no_decomposition"] = json!(true);
    }
    Ok(data)
}

fn f6_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Binary(curve) = group else {
        return Ok(unsupported("F6WidePairIndex requires a binary curve"));
    };
    if config.m != 4 {
        return Ok(unsupported(
            "Native F6 geometric pair index supports exactly four summands",
        ));
    }
    let points: Vec<BinaryPoint> = domain
        .iter()
        .map(|p| match p {
            Element::Binary(p) => p.clone(),
            _ => unreachable!(),
        })
        .collect();
    let target_native = match target {
        Element::Binary(p) => p,
        _ => unreachable!(),
    };
    let cap = config.work_limit.min(TRACE_EVENT_CAP) as usize;
    let Some(index) = F6WidePairIndex::new(curve, &points, cap) else {
        return Ok(SolverData {
            outcome: outcome(
                "budget_exhausted",
                "Native F6 pair construction exceeded its cap or rejected the input",
            ),
            artifacts: vec![],
        });
    };
    let mut pairs = vec![];
    for i in 0..domain.len() {
        for j in i..domain.len() {
            pairs.push(json!({"i":i,"j":j,"sum":group.add(&domain[i],&domain[j]).json()}));
        }
    }
    let found = index.solve4(target_native);
    let mut data = SolverData {
        outcome: outcome(
            "verified",
            "Native four-sum geometric search completed over the signed finite domain",
        ),
        artifacts: vec![],
    };
    data.outcome["solver_completed"] = json!(true);
    data.outcome["exhaustive_no_decomposition"] = json!(found.is_none());
    data.outcome["algorithm"] = json!("F6WidePairIndex exact geometric four-sum; not Groebner F6");
    data.outcome["counters"] = json!({"native_pair_count":index.pair_count(),"artifact_replay_group_add_calls":pairs.len(),"native_query_group_add_calls":null});
    data.outcome["trace_gaps"]=json!(["Private native lookup and batch-arithmetic sequence is not exported; pair sums are independently reconstructed and charged separately"]);
    save(
        &mut data,
        "f6-pair-table.json",
        "independently-reconstructed-native-pair-domain",
        &json!(pairs),
    )?;
    if let Some(indices) = found {
        set_witness(
            &mut data,
            group,
            &indices
                .iter()
                .map(|&i| domain[i].clone())
                .collect::<Vec<_>>(),
            target,
        )?;
    }
    Ok(data)
}

fn root_solve(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    if config.m != 2 {
        return Ok(unsupported(
            "Native S3 univariate root adapter currently supports m=2",
        ));
    }
    if matches!(group, Group::Prime(_)) {
        return prime_root_solve(config, group, domain, target);
    }
    binary_roots::solve(config, case, group, domain, target)
}

pub(super) fn verify_binary_root_artifact(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    artifact: &Value,
) -> Result<(), String> {
    binary_roots::verify(public, base, config, artifact)
}

pub(super) fn binary_root_trace_totals(artifact: &Value) -> Result<(u64, u64), String> {
    binary_roots::trace_totals(artifact)
}

// This fixed recipe performs exactly 15 field operations: 3 additions,
// 4 subtractions and 8 multiplications. Integer encoding/validation is separate.
const PRIME_S3_SETUP_WORK: u64 = 15;
fn prime_root_coefficients(p: u64, a: u64, b: u64, u: u64, r: u64) -> Vec<u64> {
    let add = |x: u64, y: u64| ((x as u128 + y as u128) % p as u128) as u64;
    let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    let sub = |x: u64, y: u64| ((x as u128 + p as u128 - y as u128) % p as u128) as u64;
    let product = mul(u, r);
    let sum = add(u, r);
    let t = sub(product, a);
    let c0 = sub(mul(t, t), mul(4 % p, mul(b, sum)));
    let c1 = sub(0, add(mul(2 % p, mul(sum, add(product, a))), mul(4 % p, b)));
    let d = sub(u, r);
    vec![c0, c1, mul(d, d)]
}

fn prime_root_event_envelope(index: usize, fixed_x: &str, event: &RootEvent) -> Value {
    json!({"schema":"ml-dataset.prime-root-event/v1","call_index":index,
        "fixed_x":fixed_x,"native":event})
}

// Count only acknowledged envelopes. A byte refusal is the only native sink
// failure that the adapter can seal: real I/O errors return Err before sealing.
// The rejected event was computed and charged by the native engine, but is not
// an acknowledged envelope or a separately saved intermediate state.
fn verify_prime_root_event_budget(
    report: &RootReport,
    index: usize,
    fixed_x: &str,
    byte_cap_reached: bool,
    byte_cap: usize,
    totals: &mut (u64, usize),
) -> Result<(), String> {
    let event_bytes = |event: &RootEvent| -> Result<usize, String> {
        Ok(
            serde_json::to_vec(&prime_root_event_envelope(index, fixed_x, event))
                .map_err(|e| e.to_string())?
                .len(),
        )
    };
    for event in &report.events {
        totals.0 = totals.0.checked_add(1).ok_or("root event count overflow")?;
        totals.1 = totals
            .1
            .checked_add(event_bytes(event)?)
            .ok_or("root event bytes overflow")?;
        if totals.0 > TRACE_EVENT_CAP || totals.1 > byte_cap {
            return Err("prime root acknowledged event cap exceeded".into());
        }
    }
    if report.stop == RootStop::SinkFailure {
        let rejected = report
            .unacknowledged
            .as_ref()
            .ok_or("root byte refusal lacks rejected event")?;
        if !byte_cap_reached
            || report.sink_error.as_deref() != Some(PRIME_ROOT_BYTE_CAP_REASON)
            || totals.0 >= TRACE_EVENT_CAP
            || totals
                .1
                .checked_add(event_bytes(rejected)?)
                .ok_or("root event bytes overflow")?
                <= byte_cap
        {
            return Err("prime root byte-cap refusal is not justified".into());
        }
    } else if byte_cap_reached || report.unacknowledged.is_some() || report.sink_error.is_some() {
        return Err("prime root byte-cap flag or failed-sink state differs".into());
    }
    Ok(())
}

/// Validate the adapter's aggregate event envelope cap and the exact recorded
/// refusal. This verifies the supplied records, not a complete execution trace.
pub(super) fn prime_root_trace_totals(certificate: &Value) -> Result<(u64, u64), String> {
    let queries = certificate["queries"]
        .as_array()
        .ok_or("missing root queries")?;
    let mut totals = (0, 0);
    for (index, query) in queries.iter().enumerate() {
        let report: RootReport =
            serde_json::from_value(query["report"].clone()).map_err(|e| e.to_string())?;
        let fixed_x = query["fixed_x"]
            .as_str()
            .ok_or("root query lacks fixed x")?;
        let byte_cap = query["trace_byte_cap_reached"]
            .as_bool()
            .ok_or("root query lacks byte-cap flag")?;
        verify_prime_root_event_budget(
            &report,
            index,
            fixed_x,
            byte_cap,
            PRIME_ROOT_EVENT_BYTE_CAP,
            &mut totals,
        )?;
        if byte_cap && index + 1 != queries.len() {
            return Err("root queries continue after byte-cap refusal".into());
        }
    }
    Ok((totals.0, totals.1 as u64))
}

fn prime_root_solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    let Group::Prime(curve) = group else {
        return Err("prime root group mismatch".into());
    };
    let p = curve.p.to_u64().ok_or("prime root modulus exceeds u64")?;
    let a = curve.a.to_u64().ok_or("prime coefficient exceeds u64")?;
    let b = curve.b.to_u64().ok_or("prime coefficient exceeds u64")?;
    let Some(xr) = target.x().and_then(|v| v.to_u64()) else {
        return Ok(unsupported("S3 root adapter requires affine target"));
    };
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .map(|x| x.to_u64().ok_or("factor-base x exceeds u64"))
        .collect::<Result<_, _>>()?;
    codes.sort_unstable();
    codes.dedup();
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let mut used = 0u64;
    let mut events = 0u64;
    let mut event_bytes = 0usize;
    let mut queries = vec![];
    let mut found = None;
    let mut censored = false;
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Bounded native prime roots with independently replayable distinct-root certificates",
        ),
        artifacts: vec![],
    };
    for (call_index, &x) in codes.iter().enumerate() {
        if limit.saturating_sub(used) < PRIME_S3_SETUP_WORK {
            censored = true;
            break;
        }
        let before = used;
        let coefficients = prime_root_coefficients(p, a, b, x, xr);
        used += PRIME_S3_SETUP_WORK;
        let mut io_error = None;
        let mut byte_cap = false;
        let report = prime_root_trace::solve(
            &coefficients,
            p,
            config.seed ^ call_index as u64,
            RootLimits {
                field_work: limit - used,
                split_attempts: 1000,
                events: TRACE_EVENT_CAP.saturating_sub(events),
                degree: 2,
            },
            &mut |event| {
                let row = prime_root_event_envelope(call_index, &x.to_string(), event);
                let size = serde_json::to_vec(&row).map_err(|e| e.to_string())?.len();
                if event_bytes.saturating_add(size) > PRIME_ROOT_EVENT_BYTE_CAP {
                    byte_cap = true;
                    return Err(PRIME_ROOT_BYTE_CAP_REASON.into());
                }
                if let Err(error) = live_event(&row) {
                    io_error = Some(error.clone());
                    return Err(error);
                }
                events += 1;
                event_bytes += size;
                if matches!(event, RootEvent::PowerEnd { .. } | RootEvent::GcdEnd { .. }) {
                    let path = format!("root-state-{call_index:04}-{events:06}.json");
                    if let Err(error) = save(
                        &mut data,
                        &path,
                        "native-prime-root-intermediate-state",
                        &row,
                    ) {
                        io_error = Some(error.clone());
                        return Err(error);
                    }
                }
                Ok(())
            },
        )?;
        used += report.counters.field_work();
        if let Some(error) = io_error {
            return Err(error);
        }
        // Replay is an integrity check, excluded from the solver-work counters.
        prime_root_trace::verify(&report)?;
        let mut checks = vec![];
        let roots = if report.stop == RootStop::AllFieldElements {
            codes.clone()
        } else {
            report.roots.clone()
        };
        if report.complete() {
            for root in roots {
                let evaluation_work = 2 * coefficients.len() as u64;
                if limit.saturating_sub(used) < evaluation_work {
                    censored = true;
                    break;
                }
                used += evaluation_work;
                let value = coefficients.iter().rev().fold(0u64, |acc, &c| {
                    ((acc as u128 * root as u128 + c as u128) % p as u128) as u64
                });
                if value != 0 {
                    return Err("native prime root failed original polynomial evaluation".into());
                }
                let (witness, adds, exhausted) = lift(
                    group,
                    domain,
                    &[BigUint::from(x), BigUint::from(root)],
                    target,
                    limit - used,
                );
                used += adds;
                checks.push(
                    json!({"x":root.to_string(),"evaluation_work":evaluation_work,
                    "group_lift_add_calls":adds,"in_factor_base":codes.contains(&root),
                    "group_lift_verified":witness.is_some(),"lift_budget_exhausted":exhausted}),
                );
                if witness.is_some() {
                    found = witness;
                    break;
                }
                if exhausted {
                    censored = true;
                    break;
                }
            }
        } else {
            censored = true;
        }
        let row = json!({"call_index":call_index,"fixed_x":x.to_string(),
            "coefficients_constant_first":coefficients,"work_before":before,
            "coefficient_setup_work":PRIME_S3_SETUP_WORK,"work_after":used,
            "trace_byte_cap_reached":byte_cap,"report":report,"checks":checks});
        save(
            &mut data,
            &format!("root-call-{call_index:04}.json"),
            "native-prime-root-call",
            &row,
        )?;
        queries.push(row);
        if found.is_some() || censored {
            break;
        }
    }
    let certificate = json!({"schema":"ml-dataset.prime-root-certificates/v1",
        "target":target.json(),"p":p.to_string(),"requested_work_limit":config.work_limit,
        "effective_work_limit":limit,"charged_work":used,"queries":queries,
        "root_queries_completed":!censored && queries.len()==codes.len(),
        "censored":censored,"found_pdp":found.is_some(),
        "distinct_root_certificates_verified":true,"execution_trace_complete_verified":false,
        "total_operation_count_verified":false,
        "counter_scope":"coefficient recipe field operations + native field add/sub/mul/square including inverse exponentiation + original-root evaluation + group lift additions; excludes validation, serialization and replay",
        "all_field_polynomial_policy":"explicit all-field certificate, then bounded actual factor-base x enumeration"});
    save(
        &mut data,
        "prime-root-certificates.json",
        "prime-root-certificates/v1",
        &certificate,
    )?;
    data.outcome["counters"] = json!({"aggregate_root_field_and_group_work":used,
        "root_solver_calls":queries.len(),"acknowledged_native_events":events,
        "acknowledged_native_event_bytes":event_bytes});
    data.outcome["root_certificate_scope"] = json!(PRIME_ROOT_CERTIFICATE_SCOPE);
    data.outcome["root_execution_trace_complete_verified"] = json!(false);
    data.outcome["root_total_operation_count_verified"] = json!(false);
    data.outcome["trace_gaps"] = json!([
        "No higher-m elimination or binary root internals",
        "Event records are retained solver artifacts, not a restorable process checkpoint"
    ]);
    if let Some(witness) = found {
        set_witness(&mut data, group, &witness, target)?;
    } else if censored {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Aggregate prime-root/lift work, split or trace cap reached");
    } else {
        data.outcome["solver_completed"] = json!(true);
    }
    Ok(data)
}

/// Bind per-call root certificates to the exact public S3 instances and finite
/// domain. No private scalar is read; completed root sets are not ECDLP labels.
pub(super) fn verify_prime_root_artifact(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    artifact: &Value,
) -> Result<(), String> {
    if base["schema"] == "ml-ecdlp-factor-base/v2" && base["construction_status"] == "unsupported" {
        return Err("root certificate uses an unsupported factor-base policy".into());
    }
    if config.solver != "root"
        || config.family != "prime"
        || config.m != 2
        || public["family"] != "prime"
        || public["m"] != 2
        || public["n"] != config.n
        || public["twist"] != config.twist
        || artifact["schema"] != "ml-dataset.prime-root-certificates/v1"
        || artifact["target"] != public["target"]
        || artifact["requested_work_limit"] != config.work_limit
        || artifact["distinct_root_certificates_verified"] != true
        || artifact["execution_trace_complete_verified"] != false
        || artifact["total_operation_count_verified"] != false
    {
        return Err("prime root artifact public/config binding differs".into());
    }
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: base.clone(),
    };
    let group = Group::read(&case)?;
    let Group::Prime(curve) = &group else {
        return Err("prime root curve required".into());
    };
    let target = group.point(&public["target"])?;
    if !group.on_curve(&target) {
        return Err("root certificate target is off curve".into());
    }
    let p = curve.p.to_u64().ok_or("prime root modulus exceeds u64")?;
    let a = curve.a.to_u64().ok_or("prime root a exceeds u64")?;
    let b = curve.b.to_u64().ok_or("prime root b exceeds u64")?;
    let xr = target
        .x()
        .and_then(|x| x.to_u64())
        .ok_or("affine target required")?;
    let domain = domain(&group, &case)?;
    let mut codes: Vec<u64> = domain
        .iter()
        .filter_map(Element::x)
        .map(|x| x.to_u64().ok_or("x exceeds u64"))
        .collect::<Result<_, _>>()?;
    codes.sort_unstable();
    codes.dedup();
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let modulus_decimal = p.to_string();
    if artifact["p"].as_str() != Some(modulus_decimal.as_str())
        || artifact["effective_work_limit"] != limit
    {
        return Err("prime root modulus or budget binding differs".into());
    }
    let queries = artifact["queries"]
        .as_array()
        .ok_or("missing root queries")?;
    if queries.len() > codes.len() {
        return Err("too many root queries".into());
    }
    prime_root_trace_totals(artifact)?;
    let mut used = 0u64;
    let mut events = 0u64;
    let mut found = false;
    let mut censored = false;
    for (index, row) in queries.iter().enumerate() {
        if found || censored {
            return Err("root queries continue past terminal state".into());
        }
        let x = codes[index];
        let fixed_x_decimal = x.to_string();
        let coeff = prime_root_coefficients(p, a, b, x, xr);
        if row["call_index"] != index
            || row["fixed_x"].as_str() != Some(fixed_x_decimal.as_str())
            || row["work_before"] != used
            || row["coefficient_setup_work"] != PRIME_S3_SETUP_WORK
            || row["coefficients_constant_first"] != json!(coeff)
            || limit.saturating_sub(used) < PRIME_S3_SETUP_WORK
        {
            return Err("root query identity or work prefix differs".into());
        }
        used += PRIME_S3_SETUP_WORK;
        let report: RootReport =
            serde_json::from_value(row["report"].clone()).map_err(|e| e.to_string())?;
        let mut canonical_coeff = coeff.clone();
        while canonical_coeff.last() == Some(&0) {
            canonical_coeff.pop();
        }
        if report.p != p
            || report.polynomial != canonical_coeff
            || report.seed != config.seed ^ index as u64
            || report.limits
                != (RootLimits {
                    field_work: limit - used,
                    split_attempts: 1000,
                    events: TRACE_EVENT_CAP - events,
                    degree: 2,
                })
        {
            return Err("root report differs from frozen call".into());
        }
        prime_root_trace::verify(&report)?;
        used += report.counters.field_work();
        events += report.events.len() as u64;
        let checks = row["checks"].as_array().ok_or("missing root lift checks")?;
        let roots = if report.stop == RootStop::AllFieldElements {
            codes.clone()
        } else {
            report.roots.clone()
        };
        if checks.len() > roots.len() || (!report.complete() && !checks.is_empty()) {
            return Err("invalid censored root lifts".into());
        }
        for (j, check) in checks.iter().enumerate() {
            if found || censored {
                return Err("root lifts continue after terminal state".into());
            }
            let root = roots[j];
            let root_decimal = root.to_string();
            let eval = 2 * coeff.len() as u64;
            if check["x"].as_str() != Some(root_decimal.as_str())
                || check["evaluation_work"] != eval
                || limit.saturating_sub(used) < eval
            {
                return Err("root evaluation identity/work differs".into());
            }
            used += eval;
            let (w, adds, exhausted) = lift(
                &group,
                &domain,
                &[BigUint::from(x), BigUint::from(root)],
                &target,
                limit - used,
            );
            if check["group_lift_add_calls"] != adds
                || check["in_factor_base"] != codes.contains(&root)
                || check["group_lift_verified"] != w.is_some()
                || check["lift_budget_exhausted"] != exhausted
            {
                return Err("root group lift replay differs".into());
            }
            used += adds;
            found = w.is_some();
            censored = exhausted && !found;
        }
        if report.complete() && !found && checks.len() < roots.len() {
            if !censored && limit.saturating_sub(used) >= 2 * coeff.len() as u64 {
                return Err("unjustified omitted root lift".into());
            }
            censored = true;
        }
        if !report.complete() {
            censored = true;
        }
        if row["work_after"] != used || used > limit {
            return Err("aggregate root work differs".into());
        }
    }
    if !found && !censored && queries.len() < codes.len() {
        if limit.saturating_sub(used) >= PRIME_S3_SETUP_WORK {
            return Err("unjustified omitted root query".into());
        }
        censored = true;
    }
    if artifact["charged_work"] != used
        || artifact["found_pdp"] != found
        || artifact["censored"] != censored
        || artifact["root_queries_completed"] != (!censored && queries.len() == codes.len())
    {
        return Err("prime root aggregate completion differs".into());
    }
    Ok(())
}

/// Explicit result of the profile verifier; only a validated censored prefix
/// may bypass later mathematical parsing of incomplete bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WidePrimeReplay {
    Ordinary,
    CensoredM3Prefix,
}
/// Semantic wide-prime replay, independent of native basis derivation.
// Versioned supervised profile; original-input validation is shared by runtime
// dispatch and independent central replay.
const DYNAMIC_PROFILE: &str = "f4-wide-dynamic-v1";
struct DynamicOriginal {
    p: u64,
    a: u64,
    b: u64,
    m: usize,
    target: prime_f4_wide_dynamic_replay::Point,
    domain: Vec<prime_f4_wide_dynamic_replay::Point>,
    codes: Vec<u64>,
    work_limit: u64,
    memory_bytes: u64,
    trace_bytes: u64,
}
impl DynamicOriginal {
    fn expected(&self) -> prime_f4_wide_dynamic_replay::Expected<'_> {
        prime_f4_wide_dynamic_replay::Expected {
            p: self.p,
            a: self.a,
            b: self.b,
            m: self.m,
            target: self.target,
            domain: &self.domain,
            codes: &self.codes,
            work_limit: self.work_limit,
            memory_bytes: self.memory_bytes,
            trace_bytes: self.trace_bytes,
        }
    }
}
/// The parent must first authenticate config and execution limits against the
/// original run. This independently validates/reconstructs public mathematical
/// inputs; it never consults private_witness or any solver-produced input file.
fn dynamic_original(
    config: &CaseConfig,
    case: &CaseData,
    memory_bytes: u64,
    trace_bytes: u64,
) -> Result<DynamicOriginal, String> {
    use prime_f4_wide_dynamic_replay::Point as ReplayPoint;
    if config.solver != DYNAMIC_PROFILE
        || config.family != "prime"
        || !(33..=61).contains(&config.n)
        || !(4..=15).contains(&config.m)
        || config.degree_bound != 12
        || config.work_limit == 0
        || memory_bytes == 0
        || trace_bytes == 0
    {
        return Err("configuration is outside the frozen dynamic profile".into());
    }
    let public = &case.public_instance;
    if public["family"] != config.family
        || public["n"] != json!(config.n)
        || public["m"] != json!(config.m)
        || public["twist"] != json!(config.twist)
        || case.factor_base["requested_size"] != json!(config.factor_base_size)
    {
        return Err("dynamic public inputs differ from original configuration".into());
    }
    curves::verify_public(public)?;
    curves::verify_factor_base_config(config, public, &case.factor_base)?;
    let group = Group::read(case)?;
    let Group::Prime(curve) = &group else {
        return Err("dynamic profile requires a prime curve".into());
    };
    let p = curve.p.to_u64().ok_or("dynamic modulus exceeds u64")?;
    if 64 - p.leading_zeros() != config.n {
        return Err("dynamic modulus width differs from original configuration".into());
    }
    let point = |element: &Element| -> Result<ReplayPoint, String> {
        match element {
            Element::Prime(Point::Affine { x, y }) => Ok(ReplayPoint {
                x: x.value.to_u64().ok_or("dynamic x exceeds u64")?,
                y: y.value.to_u64().ok_or("dynamic y exceeds u64")?,
            }),
            _ => Err("dynamic original point must be affine over the prime field".into()),
        }
    };
    let target = point(&group.point(&public["target"])?)?;
    let signed = domain(&group, case)?;
    if signed.is_empty() || signed.len() > 64 {
        return Err("dynamic signed domain exceeds original profile capacity".into());
    }
    let domain = signed.iter().map(point).collect::<Result<Vec<_>, _>>()?;
    let mut codes: Vec<_> = domain.iter().map(|q| q.x).collect();
    codes.sort_unstable();
    codes.dedup();
    if codes.is_empty() || codes.len() > 32 {
        return Err("dynamic x domain exceeds original profile capacity".into());
    }
    Ok(DynamicOriginal {
        p,
        a: curve.a.to_u64().ok_or("dynamic a exceeds u64")?,
        b: curve.b.to_u64().ok_or("dynamic b exceeds u64")?,
        m: config.m,
        target,
        domain,
        codes,
        work_limit: config.work_limit,
        memory_bytes,
        trace_bytes,
    })
}

/// Reconcile outer labels with independently checked mathematics and separately
/// reported native boundary facts. Does not select/admit a profile or run it.
fn verify_dynamic_claims(
    outcome: &Value,
    verified: &prime_f4_wide_dynamic_replay::Verified,
) -> Result<(), String> {
    if !outcome.is_object() {
        return Err("dynamic outcome must be an object".into());
    }
    prime_f4_wide::verify_unproved_claims(outcome)?;
    for key in [
        "independent_derivation_verified",
        "operation_trajectory_verified",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "fall_degree_verified",
        "linear_extraction_complete",
        "extraction_complete",
    ] {
        if !outcome[key].is_null() && outcome[key] != false {
            return Err(format!("dynamic replay does not certify outer {key}"));
        }
    }
    let supervisor_censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    for key in ["solver_completed", "pdp_witness_verified"] {
        // terminal_outcome intentionally drops worker witness claims on a
        // supervisor stop. Absence is not a positive assertion. Explicit null
        // or a missing completion flag still fails the typed outcome contract.
        if key == "pdp_witness_verified" && supervisor_censored && outcome.get(key).is_none() {
            continue;
        }
        if !outcome[key].is_boolean() {
            return Err(format!("dynamic outcome lacks boolean {key}"));
        }
    }
    for (key, supported) in [
        ("solver_completed", verified.solver_completed_verified),
        (
            "pdp_witness_verified",
            verified.first_witness_verified && verified.witness_capture_committed,
        ),
        (
            "certificate_durably_persisted",
            verified.witness_capture_committed,
        ),
        (
            "group_domain_complete",
            verified.independent_group_domain_complete,
        ),
        ("exhaustive_no_decomposition", verified.negative_verified),
    ] {
        if outcome[key].is_null() {
            continue;
        }
        let claimed = outcome[key]
            .as_bool()
            .ok_or_else(|| format!("nonboolean dynamic claim {key}"))?;
        if claimed && !supported {
            return Err(format!("unsupported dynamic outcome claim {key}"));
        }
    }
    for (key, reported) in [
        (
            "native_backend_invoked",
            verified.native_backend_invoked_reported,
        ),
        (
            "native_statistics_complete",
            verified.native_statistics_complete_reported,
        ),
        (
            "algebra_completed",
            verified.native_statistics_complete_reported,
        ),
    ] {
        if !outcome[key].is_null() && outcome[key] != json!(reported) {
            return Err(format!("dynamic native metadata mismatch: {key}"));
        }
    }
    if outcome.get("live_persistence_errors").is_some() && outcome["solver_completed"] == true {
        return Err("dynamic completion cannot survive outer persistence failure".into());
    }
    match outcome["status"].as_str() {
        Some("partial") => {}
        Some("verified")
            if outcome["solver_completed"] == true && outcome["pdp_witness_verified"] == true => {}
        Some("timeout" | "trace_budget_exhausted" | "worker_failed") => {
            prime_f4_wide::verify_censored_outcome(outcome)?;
            for key in [
                "certificate_durably_persisted",
                "group_domain_complete",
                "exhaustive_no_decomposition",
            ] {
                if outcome[key] == true {
                    return Err("censored outer dynamic outcome promotes a claim".into());
                }
            }
        }
        _ => return Err("dynamic outcome status contradicts verified scope".into()),
    }
    Ok(())
}

/// Execute the versioned dynamic profile under the original worker context.
/// Caller authenticates config/context and owns the attempt-bound sink.
fn solve_dynamic_bound(
    config: &CaseConfig,
    case: &CaseData,
    context: &super::execution::SolverExecutionContext,
) -> Result<SolverData, String> {
    use prime_f4_wide_dynamic as d;
    const MIB: u64 = 1024 * 1024;
    if !context.launch.trace_bytes.is_multiple_of(MIB) {
        return Err("dynamic original trace allowance is not whole MiB".into());
    }
    let capture_limits = super::capture::capture_limits(context.launch.trace_bytes / MIB)?;
    LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
        Some(EventSink::Bound(sink)) => {
            sink.verify_attempt_limits(&context.launch.binding, &capture_limits)
        }
        _ => Err("dynamic runtime requires attempt-bound capture".into()),
    })?;
    if Instant::now() >= context.deadline {
        return Err("dynamic original worker deadline expired".into());
    }
    let original = dynamic_original(
        config,
        case,
        context.launch.memory_bytes,
        context.launch.trace_bytes,
    )?;
    let target = d::AffinePoint {
        x: original.target.x,
        y: original.target.y,
    };
    // Bounded original-domain reconstruction belongs to the existing outer
    // adapter, separately from the child's admitted polynomial workspaces.
    let points: Vec<_> = original
        .domain
        .iter()
        .map(|p| d::AffinePoint { x: p.x, y: p.y })
        .collect();
    let mut forward = |event: &d::PhaseEvent, buffer: &mut super::capture::TypedEventBuffer| {
        live_typed_event(
            super::capture::TypedEventKind::WidePrimeDynamic,
            event,
            buffer,
        )
        .map_err(|_| ())
    };
    let mut prepared = match d::prepare_original(
        original.p,
        d::Parameters {
            m: original.m,
            a: original.a,
            b: original.b,
            target_x: target.x,
            codes: &original.codes,
        },
        target,
        &points,
        d::AttemptLimits {
            work_limit: config.work_limit,
            memory_bytes: context.launch.memory_bytes,
            trace_bytes: context.launch.trace_bytes,
            degree_bound: config.degree_bound,
            original_deadline: context.deadline,
        },
        d::EventPersistence::BoundCapture,
        &mut forward,
    ) {
        Ok(prepared) => prepared,
        Err(failure) => {
            let retained =
                d::visit_prepare_failure_artifacts(&failure, |path, role, encoding, bytes| {
                    live_borrowed_payload(path, role, encoding, bytes).map_err(|_| ())
                });
            return Ok(dynamic_partial_result(
                "prepare",
                &format!("{:?}", failure.stop),
                retained.is_err(),
            ));
        }
    };
    let mut publisher = d::BoundWitnessPublisher::default();
    if let Err(stop) = d::prepare_step4(&mut prepared, &mut publisher) {
        let retained =
            d::visit_step4_retained_artifacts(&prepared, |path, role, encoding, bytes| {
                live_borrowed_payload(path, role, encoding, bytes).map_err(|_| ())
            });
        return Ok(dynamic_partial_result(
            "capture_admission",
            &format!("{stop:?}"),
            retained.is_err(),
        ));
    }
    let mut phase = match d::invoke_original(prepared, &mut forward) {
        Ok(phase) => phase,
        Err(failure) => {
            let retained =
                d::visit_prepare_failure_artifacts(&failure, |path, role, encoding, bytes| {
                    live_borrowed_payload(path, role, encoding, bytes).map_err(|_| ())
                });
            return Ok(dynamic_partial_result(
                "native_admission",
                &format!("{:?}", failure.stop),
                retained.is_err(),
            ));
        }
    };
    let linear = d::extract_reported_linear(&mut phase);
    let linear_retained = d::retain_linear(&mut phase, &linear);
    if linear_retained.is_ok() {
        d::run_group_step4(&mut phase, Some(&linear), &mut forward, &mut publisher);
        // Confirmation records its own failure in the terminal state and uses
        // the persistent budget and original deadline. Never silently retry.
        let _ = d::confirm_step4_witness(&mut phase.prepared, &mut publisher);
    }
    let terminal = d::retain_step4_terminal(&mut phase);
    let facts = terminal
        .as_ref()
        .ok()
        .and_then(|()| d::step4_outcome(&phase).ok());
    let artifacts = d::take_step4_artifacts(&mut phase.prepared)
        .map_err(|e| format!("dynamic admitted owner transfer failed: {e:?}"))?;
    let mut result = if let Some(facts) = facts {
        let claims = facts.claims;
        SolverData {
            outcome: json!({
                "strategy":DYNAMIC_STRATEGY,
                "status":if claims.solver_completed && claims.pdp_witness_verified {"verified"} else {"partial"},
                "solver_completed":claims.solver_completed,
                "pdp_witness_verified":claims.pdp_witness_verified && claims.certificate_durably_persisted,
                "certificate_durably_persisted":claims.certificate_durably_persisted,
                "group_domain_complete":claims.group_domain_complete,
                "exhaustive_no_decomposition":claims.negative_verified,
                "native_backend_invoked":facts.native_backend_invoked,
                "native_statistics_complete":facts.native_statistics_complete,
                "algebra_completed":facts.native_statistics_complete,
                "native_basis_derivation_verified":false,
                "independent_derivation_verified":false,
                "operation_trajectory_verified":false,
                "exact_instruction_count_verified":false,
                "full_execution_trace_verified":false,
                "fall_degree_verified":false,
                "scalar_recovered_without_witness":false,
                "native_metadata_scope":"producer-reported boundary facts; independent replay required"
            }),
            artifacts,
        }
    } else {
        let mut result =
            dynamic_partial_result("terminal_retention", &format!("{terminal:?}"), false);
        result.artifacts = artifacts;
        result
    };
    if let Err(stop) = linear_retained {
        result.outcome["linear_retention_stop"] = json!(format!("{stop:?}"));
    }
    // The parent flush preserves these owners and never republishes a dedicated
    // witness. Outer errors suppress completion but retain acknowledged evidence.
    flush_prime_chain_payloads(&mut result);
    Ok(result)
}
fn dynamic_supported(config: &CaseConfig) -> bool {
    config.family == "prime"
        && (33..=61).contains(&config.n)
        && (4..=15).contains(&config.m)
        && config.degree_bound == 12
        && config.work_limit > 0
}

fn dynamic_partial_result(stage: &str, reason: &str, custody_failed: bool) -> SolverData {
    let mut result = SolverData {
        outcome: json!({"strategy":DYNAMIC_STRATEGY,"status":"partial",
            "stage":stage,"reason":reason,"solver_completed":false,
            "pdp_witness_verified":false,"group_domain_complete":false,
            "exhaustive_no_decomposition":false,"scalar_recovered_without_witness":false}),
        artifacts: Vec::new(),
    };
    if custody_failed {
        record_prime_chain_persistence_error(&mut result, stage, "retained prefix capture failed");
    }
    result
}

// Draft addition to solvers.rs; parent must authenticate frozen plan/config first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DynamicReplayDisposition {
    Ordinary,
    Mathematical,
    RawPrefix,
}

fn dynamic_reserved(a: &super::types::Artifact) -> bool {
    let path = a.path.strip_prefix("live/payloads/").unwrap_or(&a.path);
    path.starts_with("prime-f4-wide-dynamic-")
        || a.role.starts_with("restricted-wide-prime-dynamic-")
}

const DYNAMIC_SLOTS: [(&str, &str, &str); 6] = [
    (
        "prime-f4-wide-dynamic-setup.json",
        "restricted-wide-prime-dynamic-setup/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-native.json",
        "restricted-wide-prime-dynamic-native/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-events.jsonl",
        "restricted-wide-prime-dynamic-events/v1",
        "jsonl",
    ),
    (
        "prime-f4-wide-dynamic-linear.json",
        "restricted-wide-prime-dynamic-linear/v1",
        "json",
    ),
    (
        "prime-f4-wide-dynamic-group.json",
        "restricted-wide-prime-dynamic-group/v1",
        "jsonl",
    ),
    (
        "prime-f4-wide-dynamic-result.json",
        "restricted-wide-prime-dynamic-result/v1",
        "json",
    ),
];

fn dynamic_slot_identities(artifacts: &[super::types::Artifact]) -> Result<(), String> {
    for a in artifacts.iter().filter(|a| dynamic_reserved(a)) {
        if artifacts
            .iter()
            .filter(|other| other.path == a.path)
            .count()
            != 1
        {
            return Err("duplicate dynamic artifact path".into());
        }
        let live = a.path.strip_prefix("live/payloads/");
        let path = live.unwrap_or(&a.path);
        let Some((_, role, encoding)) = DYNAMIC_SLOTS.iter().find(|slot| slot.0 == path) else {
            return Err("unknown or displaced reserved dynamic artifact".into());
        };
        let raw = a.role == "unregistered-partial-artifact" && a.encoding == "native";
        if live.is_some() {
            // Collector assigns raw custody identity to the captured copy.
            // Its mathematical identity is in the checked committed chain.
            if !raw {
                return Err("dynamic live copy has canonical manifest identity".into());
            }
        } else if !raw && (a.role != *role || a.encoding != *encoding) {
            return Err("dynamic canonical artifact identity mismatch".into());
        }
    }
    Ok(())
}

fn dynamic_no_promoted_claims(outcome: &Value) -> Result<(), String> {
    prime_f4_wide::verify_unproved_claims(outcome)?;
    prime_f4_wide::verify_censored_outcome(outcome)?;
    for key in [
        "certificate_durably_persisted",
        "group_domain_complete",
        "independent_derivation_verified",
        "operation_trajectory_verified",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "fall_degree_verified",
        "linear_extraction_complete",
        "extraction_complete",
    ] {
        if !outcome[key].is_null() && outcome[key] != false {
            return Err(format!("raw dynamic prefix promotes {key}"));
        }
    }
    Ok(())
}

pub(super) fn verify_dynamic_cell(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> Result<DynamicReplayDisposition, String> {
    use super::execution;
    use prime_f4_wide::artifact;
    let selected = config.solver == DYNAMIC_PROFILE;
    if !selected
        && outcome["strategy"] != DYNAMIC_STRATEGY
        && !artifacts.iter().any(dynamic_reserved)
    {
        return Ok(DynamicReplayDisposition::Ordinary);
    }
    if !selected {
        return Err("dynamic evidence differs from original profile".into());
    }
    let context = execution::load(dir)?.ok_or("dynamic replay lacks original execution context")?;
    artifact(artifacts, execution::PATH, execution::ROLE, "json", true)?;
    let (binding, limits) = super::admit_capture_context(dir)?
        .ok_or("dynamic replay lacks original capture context")?;
    verify_dynamic_bound_files(dir, config, outcome, artifacts, &context, &binding, &limits)
}

/// Checks files against caller-authenticated context; original-run admission is
/// performed by verify_dynamic_cell. Direct controls test this narrower layer.
fn verify_dynamic_bound_files(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
    context: &super::execution::LaunchContext,
    binding: &AttemptBinding,
    limits: &CaptureLimits,
) -> Result<DynamicReplayDisposition, String> {
    use super::{capture, storage};
    use prime_f4_wide::artifact;
    dynamic_slot_identities(artifacts)?;
    if &context.binding != binding {
        return Err("dynamic execution and capture attempts differ".into());
    }
    // Bind the committed capture root in the manifest; its payload hashes are
    // checked transitively by the capture reader and independent child.
    for path in ["live/committed.json", "live/events.jsonl"] {
        if fs::symlink_metadata(dir.join(path)).is_ok()
            && artifacts.iter().filter(|a| a.path == path).count() != 1
        {
            return Err(format!(
                "dynamic capture root lacks manifest binding: {path}"
            ));
        }
    }
    let load_value = |path: &str| -> Result<Value, String> {
        storage::contained_path(dir, path, false)?;
        serde_json::from_slice(&prime_f4_wide::read_bounded(dir, path, 1024 * 1024)?)
            .map_err(|e| e.to_string())
    };
    let censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    let ready = [
        "prime-f4-input.json",
        "prime-f4-wide-dynamic-setup.json",
        "prime-f4-wide-dynamic-native.json",
        "prime-f4-wide-dynamic-events.jsonl",
    ]
    .iter()
    .all(|path| {
        artifacts
            .iter()
            .any(|a| a.path == *path && a.role != "unregistered-partial-artifact")
    });
    let registered_terminal = artifacts.iter().any(|a| {
        (a.path == "prime-f4-wide-dynamic-result.json"
            || a.role == "restricted-wide-prime-dynamic-result/v1")
            && a.role != "unregistered-partial-artifact"
    });
    let registered_witness = artifacts.iter().any(|a| {
        (a.path == "pdp-witness.json" || a.role == "independently-verified-pdp")
            && a.role != "unregistered-partial-artifact"
    });
    if !ready && !registered_terminal && !registered_witness {
        if !censored
            && !matches!(
                outcome["status"].as_str(),
                Some("partial" | "error" | "unsupported")
            )
        {
            return Err("incomplete dynamic evidence has nonpartial status".into());
        }
        dynamic_no_promoted_claims(outcome)?;
        if artifacts
            .iter()
            .any(|a| a.path == "result.json" && a.role == "solver-outcome")
        {
            artifact(artifacts, "result.json", "solver-outcome", "json", true)?;
            let retained = load_value("result.json")?;
            dynamic_no_promoted_claims(&retained)?;
            if !censored {
                let mut outer = outcome.clone();
                outer
                    .as_object_mut()
                    .ok_or("dynamic raw outcome object")?
                    .remove("supervisor");
                if retained != outer {
                    return Err("raw dynamic worker and outer outcome differ".into());
                }
            }
        } else if !censored {
            return Err("noncensored dynamic prefix lacks retained worker outcome".into());
        }
        // Retain oversize raw output for trace-budget failures. It is hash-bound
        // custody, never admitted to the mathematical parser as a valid bundle.
        capture::verify_live_committed_prefix(dir, binding, limits)?;
        return Ok(DynamicReplayDisposition::RawPrefix);
    }
    // A registered terminal/witness cannot escape strict replay by removing a
    // prerequisite or relabelling its status as partial/censored.
    for (path, role) in [
        ("public.json", "public-instance"),
        ("factor-base.json", "factor-base"),
    ] {
        artifact(artifacts, path, role, "json", true)?;
    }
    let case = CaseData {
        public_instance: load_value("public.json")?,
        factor_base: load_value("factor-base.json")?,
        private_witness: Value::Null,
    };
    let original = dynamic_original(config, &case, context.memory_bytes, context.trace_bytes)?;
    if artifact(
        artifacts,
        "solver-domain.json",
        "exact-signed-factor-base",
        "json",
        false,
    )?
    .is_some()
    {
        let group = Group::read(&case)?;
        let expected = json!({"points":domain(&group, &case)?.iter().map(Element::json).collect::<Vec<_>>(),
            "negation_closed":true,"repetitions_allowed":true});
        if load_value("solver-domain.json")? != expected {
            return Err("dynamic solver domain differs from original factor base".into());
        }
    } else if fs::symlink_metadata(dir.join("solver-domain.json")).is_ok() {
        return Err("dynamic solver domain lacks manifest identity".into());
    }
    let verified = prime_f4_wide_dynamic_replay::verify_files(
        dir,
        &original.expected(),
        artifacts,
        binding,
        limits,
        prime_f4_wide_dynamic_replay::ReplayLimits {
            work: 10_000_000_000,
            allocation_bytes: 128 << 20,
            deadline: Instant::now() + Duration::from_secs(30),
        },
    )?;
    verify_dynamic_claims(outcome, &verified)?;
    let worker = artifact(
        artifacts,
        "result.json",
        "solver-outcome",
        "json",
        !censored,
    )?;
    if worker.is_some() {
        let retained = load_value("result.json")?;
        verify_dynamic_claims(&retained, &verified)?;
        if !censored {
            let mut outer = outcome.clone();
            outer
                .as_object_mut()
                .ok_or("dynamic outcome object")?
                .remove("supervisor");
            if retained != outer {
                return Err("dynamic worker and outer outcome differ".into());
            }
        }
    } else if fs::symlink_metadata(dir.join("result.json")).is_ok() {
        return Err("dynamic retained outcome lacks manifest binding".into());
    }
    Ok(DynamicReplayDisposition::Mathematical)
}

#[cfg(test)]
mod dynamic_admission_tests {
    use super::*;
    fn record(path: &str, role: &str, encoding: &str) -> super::super::types::Artifact {
        super::super::types::Artifact {
            path: path.into(),
            role: role.into(),
            encoding: encoding.into(),
            sha256: "0".repeat(64),
            bytes: 0,
        }
    }
    #[test]
    fn dynamic_slot_identity_keeps_raw_live_copies_distinct_from_canonical_slots() {
        for (path, role, encoding) in DYNAMIC_SLOTS {
            let canonical = record(path, role, encoding);
            assert!(dynamic_reserved(&canonical));
            let raw = record(path, "unregistered-partial-artifact", "native");
            let live = record(
                &format!("live/payloads/{path}"),
                "unregistered-partial-artifact",
                "native",
            );
            dynamic_slot_identities(&[canonical.clone(), live.clone()]).unwrap();
            dynamic_slot_identities(&[raw]).unwrap();
            for bad in [
                record(path, "other", encoding),
                record(path, role, "utf8"),
                record("other.json", role, encoding),
                record(&live.path, role, encoding),
                record(path, "restricted-wide-prime-dynamic-result/v999", encoding),
            ] {
                assert!(dynamic_slot_identities(&[bad]).is_err());
            }
            assert!(dynamic_slot_identities(&[canonical.clone(), canonical]).is_err());
        }
        let unknown = record(
            "prime-f4-wide-dynamic-unknown.json",
            "unregistered-partial-artifact",
            "native",
        );
        assert!(dynamic_reserved(&unknown));
        assert!(dynamic_slot_identities(&[unknown]).is_err());
        for shared in [
            record(
                "prime-f4-input.json",
                "complete-prime-field-polynomial-system",
                "json",
            ),
            record("pdp-witness.json", "independently-verified-pdp", "json"),
        ] {
            assert!(!dynamic_reserved(&shared));
        }
    }
}

pub(super) fn verify_prime_f4_wide(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::types::Artifact],
) -> Result<WidePrimeReplay, String> {
    let disposition = prime_f4_wide_m3::verify(dir, config, outcome, artifacts)?;
    if disposition == WidePrimeReplay::Ordinary {
        prime_f4_wide::verify(dir, config, outcome, artifacts)?;
    }
    Ok(disposition)
}

fn wide_begin_failure(
    config: &CaseConfig,
    case: &CaseData,
    context: Option<&super::execution::SolverExecutionContext>,
    error: String,
) -> Result<SolverData, String> {
    if config.solver == DYNAMIC_PROFILE {
        return Ok(dynamic_partial_result("solver_begin", &error, true));
    }
    let Some(context) = context else {
        return Err(error);
    };
    if config.family != "prime" || config.solver != "f4" || !matches!(config.m, 2 | 3) {
        return Err(error);
    }
    let group = Group::read(case)?;
    let Group::Prime(curve) = &group else {
        return Err(error);
    };
    if curve.p.to_u64().is_none_or(|p| p < 1u64 << 32) {
        return Err(error);
    }
    let target = group.point(&case.public_instance["target"])?;
    if config.m == 3 {
        return prime_f4_wide_m3::begin_failure(
            config,
            &group,
            &domain(&group, case)?,
            &target,
            context,
            &error,
        );
    }
    prime_f4_wide::begin_failure(
        config,
        &group,
        &domain(&group, case)?,
        &target,
        context,
        &error,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dynamic_unsupported_axes_preserve_outcomes_without_reading_curve_bytes() {
        let base = CaseConfig {
            factor_base_policy: None,
            family: "prime".into(),
            n: 47,
            m: 4,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 12,
            work_limit: 16384,
            solver: DYNAMIC_PROFILE.into(),
        };
        let case = CaseData {
            public_instance: Value::Null,
            private_witness: Value::Null,
            factor_base: Value::Null,
        };
        for mutation in 0..5 {
            let mut config = base.clone();
            match mutation {
                0 => config.family = "binary".into(),
                1 => config.family = "koblitz".into(),
                2 => config.n = 23,
                3 => config.m = 2,
                _ => config.degree_bound = 8,
            }
            let result = solve_with_context(&config, &case, None).unwrap();
            assert_eq!(result.outcome["status"], "unsupported");
            assert_eq!(result.outcome["solver_completed"], false);
            assert_eq!(result.outcome["pdp_witness_verified"], false);
            assert!(result.artifacts.is_empty());
        }
        let refused = wide_begin_failure(
            &base,
            &case,
            None,
            "fixture begin persistence refusal".into(),
        )
        .unwrap();
        assert_eq!(refused.outcome["solver_completed"], false);
        assert_eq!(refused.outcome["stage"], "solver_begin");
        assert!(refused.outcome.get("live_persistence_errors").is_some());
    }

    #[test]
    fn dynamic_runtime_composes_actual_bound_capture_and_independent_replay() {
        for twist in [false, true] {
            let config = CaseConfig {
                factor_base_policy: None,
                family: "prime".into(),
                n: 47,
                m: 4,
                twist,
                curve_seed: 7,
                generator_index: 1,
                key_index: 1,
                seed: 42,
                factor_base_size: 2,
                degree_bound: 12,
                work_limit: 16384,
                solver: DYNAMIC_PROFILE.into(),
            };
            let mut case = curves::generate(&config).unwrap();
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let mut target = group.zero();
            for _ in 0..config.m {
                target = group.add(&target, &points[0]);
            }
            case.public_instance["target"] = target.json();
            case.private_witness = json!({"must_not_be_consulted":true});
            curves::verify_public(&case.public_instance).unwrap();
            // Synthetic attempt identity for a software integration control;
            // central frozen-run admission is deliberately a separate gate.
            let binding = AttemptBinding {
                schema: "crypto.ml-dataset.attempt-binding/v1".into(),
                run_sha256: "1".repeat(64),
                plan_sha256: "2".repeat(64),
                protocol_sha256: "3".repeat(64),
                config_sha256: "4".repeat(64),
                source_commit: "5".repeat(40),
                binary_sha256: "6".repeat(64),
                cell_index: 0,
                attempt_sha256: "7".repeat(64),
            };
            let context = super::super::execution::SolverExecutionContext {
                launch: super::super::execution::LaunchContext {
                    schema: "crypto.ml-dataset.execution-context/v1".into(),
                    binding: binding.clone(),
                    boot_id: "software-fixture".into(),
                    start_monotonic_ns: 1,
                    deadline_monotonic_ns: 30_000_000_001,
                    seconds_per_case: 30,
                    memory_bytes: 256 << 20,
                    trace_bytes: 128 << 20,
                },
                deadline: Instant::now() + Duration::from_secs(30),
            };
            assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
            assert!(solve_dynamic_bound(&config, &case, &context).is_err());
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir()
                .join(format!("dynamic-runtime-{}-{nonce}", std::process::id()));
            let cell = root.join("cell");
            fs::create_dir_all(&cell).unwrap();
            let limits = super::super::capture::capture_limits(128).unwrap();
            let sink = CaptureEmitter::create(&cell.join("live"), binding.clone(), limits.clone())
                .unwrap();
            LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(sink)));
            let guard = LiveSinkGuard;
            let before = fs::read(cell.join("live/committed.json")).unwrap();
            let mut wrong_attempt = context.clone();
            wrong_attempt.launch.binding.attempt_sha256 = "8".repeat(64);
            assert!(solve_dynamic_bound(&config, &case, &wrong_attempt).is_err());
            let mut wrong_limits = context.clone();
            wrong_limits.launch.trace_bytes = 64 << 20;
            assert!(solve_dynamic_bound(&config, &case, &wrong_limits).is_err());
            assert_eq!(fs::read(cell.join("live/committed.json")).unwrap(), before);
            assert!(fs::read(cell.join("live/events.jsonl")).unwrap().is_empty());
            let mut expired = context.clone();
            expired.deadline = Instant::now();
            assert!(solve_dynamic_bound(&config, &case, &expired).is_err());
            let result = solve_dynamic_bound(&config, &case, &context).unwrap();
            let mut artifacts: Vec<_> = result
                .artifacts
                .iter()
                .map(|a| {
                    super::super::storage::write_bytes(
                        &cell,
                        &a.path,
                        &a.role,
                        &a.encoding,
                        &a.bytes,
                    )
                    .unwrap()
                })
                .collect();
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{:?}",
                result.outcome
            );
            assert!(
                result.outcome.get("live_persistence_errors").is_none(),
                "{:?}",
                result.outcome
            );
            LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
                Some(EventSink::Bound(sink)) => sink.finish().unwrap(),
                _ => panic!("bound sink lost"),
            });
            drop(guard);
            for (path, role, value) in [
                ("public.json", "public-instance", &case.public_instance),
                ("factor-base.json", "factor-base", &case.factor_base),
                ("result.json", "solver-outcome", &result.outcome),
            ] {
                artifacts
                    .push(super::super::storage::write_json(&cell, path, role, value).unwrap());
            }
            for path in ["live/committed.json", "live/events.jsonl"] {
                let (sha256, bytes) = crate::store::hash_file(&cell.join(path)).unwrap();
                artifacts.push(super::super::types::Artifact {
                    path: path.into(),
                    role: "unregistered-partial-artifact".into(),
                    encoding: "native".into(),
                    sha256,
                    bytes,
                });
            }
            assert_eq!(
                verify_dynamic_bound_files(
                    &cell,
                    &config,
                    &result.outcome,
                    &artifacts,
                    &context.launch,
                    &binding,
                    &limits
                )
                .unwrap(),
                DynamicReplayDisposition::Mathematical
            );
            for stop in [Some("timeout"), Some("trace_budget_exhausted"), None] {
                let censored = super::super::execution::terminal_outcome(
                    result.outcome.clone(),
                    stop,
                    false,
                    Some(137),
                );
                assert_eq!(
                    verify_dynamic_bound_files(
                        &cell,
                        &config,
                        &censored,
                        &artifacts,
                        &context.launch,
                        &binding,
                        &limits
                    )
                    .unwrap(),
                    DynamicReplayDisposition::Mathematical
                );
            }
            for missing in ["live/committed.json", "public.json", "pdp-witness.json"] {
                let changed: Vec<_> = artifacts
                    .iter()
                    .filter(|a| a.path != missing)
                    .cloned()
                    .collect();
                assert!(
                    verify_dynamic_bound_files(
                        &cell,
                        &config,
                        &result.outcome,
                        &changed,
                        &context.launch,
                        &binding,
                        &limits
                    )
                    .is_err(),
                    "{missing}"
                );
            }
            let raw = root.join("raw");
            fs::create_dir(&raw).unwrap();
            let partial = json!({"status":"error","reason":"interrupted setup",
                "solver_completed":false,"pdp_witness_verified":false});
            let raw_artifacts = vec![super::super::storage::write_json(
                &raw,
                "result.json",
                "solver-outcome",
                &partial,
            )
            .unwrap()];
            assert_eq!(
                verify_dynamic_bound_files(
                    &raw,
                    &config,
                    &partial,
                    &raw_artifacts,
                    &context.launch,
                    &binding,
                    &limits
                )
                .unwrap(),
                DynamicReplayDisposition::RawPrefix
            );
            let mut promoted = partial.clone();
            promoted["pdp_witness_verified"] = json!(true);
            assert!(verify_dynamic_bound_files(
                &raw,
                &config,
                &promoted,
                &raw_artifacts,
                &context.launch,
                &binding,
                &limits
            )
            .is_err());
            let mut falsely_complete = raw_artifacts;
            falsely_complete.push(
                super::super::storage::write_bytes(
                    &raw,
                    "prime-f4-wide-dynamic-result.json",
                    "restricted-wide-prime-dynamic-result/v1",
                    "json",
                    b"{broken",
                )
                .unwrap(),
            );
            assert!(verify_dynamic_bound_files(
                &raw,
                &config,
                &partial,
                &falsely_complete,
                &context.launch,
                &binding,
                &limits
            )
            .is_err());
            let original = dynamic_original(
                &config,
                &case,
                context.launch.memory_bytes,
                context.launch.trace_bytes,
            )
            .unwrap();
            let verified = prime_f4_wide_dynamic_replay::verify_files(
                &cell,
                &original.expected(),
                &artifacts,
                &binding,
                &limits,
                prime_f4_wide_dynamic_replay::ReplayLimits {
                    work: 10_000_000_000,
                    allocation_bytes: 128 << 20,
                    deadline: Instant::now() + Duration::from_secs(20),
                },
            )
            .unwrap();
            verify_dynamic_claims(&result.outcome, &verified).unwrap();
            for stop in [Some("timeout"), Some("trace_budget_exhausted"), None] {
                let censored = super::super::execution::terminal_outcome(
                    result.outcome.clone(),
                    stop,
                    false,
                    Some(137),
                );
                assert!(censored.get("pdp_witness_verified").is_none());
                verify_dynamic_claims(&censored, &verified).unwrap();
                for claim in [
                    "solver_completed",
                    "pdp_witness_verified",
                    "certificate_durably_persisted",
                    "group_domain_complete",
                    "exhaustive_no_decomposition",
                ] {
                    let mut promoted = censored.clone();
                    promoted[claim] = json!(true);
                    assert!(
                        verify_dynamic_claims(&promoted, &verified).is_err(),
                        "{claim}"
                    );
                }
                let mut malformed = censored.clone();
                malformed["pdp_witness_verified"] = Value::Null;
                assert!(verify_dynamic_claims(&malformed, &verified).is_err());
                malformed = censored;
                malformed
                    .as_object_mut()
                    .unwrap()
                    .remove("solver_completed");
                assert!(verify_dynamic_claims(&malformed, &verified).is_err());
            }
            assert!(verified.first_witness_verified && verified.witness_capture_committed);
            assert!(!verified.independent_derivation_verified);
            let initial = super::super::capture::CaptureCursor::initial(&binding).unwrap();
            let restored = root.join("restored");
            let delta = super::super::capture::capture_committed_delta(
                &cell, &restored, &binding, &initial, &limits,
            )
            .unwrap()
            .unwrap();
            super::super::capture::verify_captured_delta(&restored, &delta, &initial).unwrap();
            // Simulate interruption after durable publication but before the
            // terminal file survives. Preserve the witnessed prefix without
            // promoting a completed search or a producer lifecycle claim.
            let terminal_path = "prime-f4-wide-dynamic-result.json";
            fs::remove_file(cell.join(terminal_path)).unwrap();
            let prefix_artifacts: Vec<_> = artifacts
                .iter()
                .filter(|a| a.path != terminal_path)
                .cloned()
                .collect();
            let prefix = prime_f4_wide_dynamic_replay::verify_files(
                &cell,
                &original.expected(),
                &prefix_artifacts,
                &binding,
                &limits,
                prime_f4_wide_dynamic_replay::ReplayLimits {
                    work: 10_000_000_000,
                    allocation_bytes: 128 << 20,
                    deadline: Instant::now() + Duration::from_secs(20),
                },
            )
            .unwrap();
            assert!(prefix.first_witness_verified && prefix.witness_capture_committed);
            assert!(!prefix.solver_completed_verified);
            assert!(!prefix.independent_group_domain_complete && !prefix.negative_verified);
            for missing in ["pdp-witness.json", "prime-f4-wide-dynamic-group.json"] {
                let incomplete: Vec<_> = prefix_artifacts
                    .iter()
                    .filter(|a| a.path != missing)
                    .cloned()
                    .collect();
                assert!(
                    prime_f4_wide_dynamic_replay::verify_files(
                        &cell,
                        &original.expected(),
                        &incomplete,
                        &binding,
                        &limits,
                        prime_f4_wide_dynamic_replay::ReplayLimits {
                            work: 10_000_000_000,
                            allocation_bytes: 128 << 20,
                            deadline: Instant::now() + Duration::from_secs(20),
                        },
                    )
                    .is_err(),
                    "missing {missing} must not support witnessed prefix"
                );
            }
            let partial = json!({"status":"partial", "solver_completed":false,
                "pdp_witness_verified":true, "certificate_durably_persisted":true});
            verify_dynamic_claims(&partial, &prefix).unwrap();
            for claim in [
                "solver_completed",
                "group_domain_complete",
                "exhaustive_no_decomposition",
            ] {
                let mut forged = partial.clone();
                forged[claim] = json!(true);
                assert!(verify_dynamic_claims(&forged, &prefix).is_err());
            }
            fs::remove_dir_all(root).unwrap();
        }
    }
    #[test]
    fn dynamic_original_uses_verified_public_inputs_at_requested_boundaries() {
        for n in [47, 61] {
            for m in [4, 15] {
                for twist in [false, true] {
                    let config = CaseConfig {
                        factor_base_policy: None,
                        family: "prime".into(),
                        n,
                        m,
                        twist,
                        curve_seed: 7,
                        generator_index: 1,
                        key_index: 1,
                        seed: 42,
                        factor_base_size: 2,
                        degree_bound: 12,
                        work_limit: 16384,
                        solver: DYNAMIC_PROFILE.into(),
                    };
                    let mut case = curves::generate(&config).unwrap();
                    case.private_witness = json!({"must_not_be_consulted":true});
                    let original = dynamic_original(&config, &case, 256 << 20, 128 << 20).unwrap();
                    let expected = original.expected();
                    assert_eq!(expected.m, m);
                    assert_eq!(expected.work_limit, config.work_limit);
                    assert_eq!(expected.memory_bytes, 256 << 20);
                    assert_eq!(expected.trace_bytes, 128 << 20);
                    assert_eq!(
                        expected.target.x.to_string(),
                        case.public_instance["target"]["x"].as_str().unwrap()
                    );
                    assert_eq!(
                        expected.target.y.to_string(),
                        case.public_instance["target"]["y"].as_str().unwrap()
                    );
                    assert!(expected.codes.windows(2).all(|w| w[0] < w[1]));
                    for q in expected.domain {
                        assert!(expected.codes.contains(&q.x));
                        let negative_y = if q.y == 0 { 0 } else { expected.p - q.y };
                        assert!(expected
                            .domain
                            .iter()
                            .any(|r| r.x == q.x && r.y == negative_y));
                    }
                    if n == 61 {
                        assert_eq!(expected.p, (1u64 << 61) - 1);
                    }
                    let mut changed = config.clone();
                    changed.factor_base_size += 1;
                    assert!(dynamic_original(&changed, &case, 256 << 20, 128 << 20).is_err());
                    changed = config.clone();
                    changed.twist = !changed.twist;
                    assert!(dynamic_original(&changed, &case, 256 << 20, 128 << 20).is_err());
                    let mut corrupted = case.clone();
                    corrupted.factor_base["points"][0]["x"] = json!(expected.p.to_string());
                    assert!(dynamic_original(&config, &corrupted, 256 << 20, 128 << 20).is_err());
                }
            }
        }
    }
    #[test]
    fn dynamic_original_rejects_unbound_or_different_profile_limits() {
        let config = CaseConfig {
            factor_base_policy: None,
            family: "prime".into(),
            n: 47,
            m: 4,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 12,
            work_limit: 16384,
            solver: DYNAMIC_PROFILE.into(),
        };
        let case = curves::generate(&config).unwrap();
        for mutation in 0..4 {
            let mut changed = config.clone();
            match mutation {
                0 => changed.solver = "f4".into(),
                1 => changed.degree_bound = 8,
                2 => changed.work_limit = 0,
                _ => changed.m = 3,
            }
            assert!(dynamic_original(&changed, &case, 256 << 20, 128 << 20).is_err());
        }
        assert!(dynamic_original(&config, &case, 0, 128 << 20).is_err());
        assert!(dynamic_original(&config, &case, 256 << 20, 0).is_err());
    }

    #[test]
    fn typed_solver_bridge_requires_bound_capture_and_preserves_sticky_failure() {
        use super::super::capture::{TypedEventBuffer, TypedEventKind};
        let mut buffer =
            TypedEventBuffer::try_new(1024, TypedEventBuffer::requested_bytes(1024).unwrap())
                .unwrap();
        assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
        assert!(live_typed_event(TypedEventKind::WidePrimeDynamic, &7u64, &mut buffer).is_err());
        assert!(live_borrowed_payload("input.json", "test", "json", b"{}").is_err());
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "typed-solver-bridge-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        let legacy_path = directory.join("legacy.jsonl");
        LIVE_SINK.with(|slot| {
            *slot.borrow_mut() = Some(EventSink::Legacy(LiveSink {
                directory: directory.clone(),
                events: File::create(&legacy_path).unwrap(),
                sequence: 0,
            }));
        });
        let guard = LiveSinkGuard;
        assert!(live_typed_event(TypedEventKind::WidePrimeDynamic, &7u64, &mut buffer).is_err());
        assert!(live_borrowed_payload("input.json", "test", "json", b"{}").is_err());
        assert_eq!(fs::metadata(&legacy_path).unwrap().len(), 0);
        let binding = AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "5".repeat(40),
            binary_sha256: "6".repeat(64),
            cell_index: 0,
            attempt_sha256: "7".repeat(64),
        };
        let live = directory.join("live");
        let emitter = CaptureEmitter::create(
            &live,
            binding.clone(),
            super::super::capture::capture_limits(64).unwrap(),
        )
        .unwrap();
        LIVE_SINK.with(|slot| *slot.borrow_mut() = Some(EventSink::Bound(emitter)));
        live_typed_event(TypedEventKind::WidePrimeDynamic, &7u64, &mut buffer).unwrap();
        let original = fs::read(live.join("events.jsonl")).unwrap();
        let event: Value = serde_json::from_slice(&original).unwrap();
        assert_eq!(event["event"]["kind"], "wide_prime_dynamic");
        assert_eq!(event["event"]["payload"], 7);
        assert!(live_typed_event(
            TypedEventKind::WidePrimeDynamic,
            &"x".repeat(2048),
            &mut buffer,
        )
        .is_err());
        assert!(live_typed_event(TypedEventKind::WidePrimeDynamic, &8u64, &mut buffer).is_err());
        assert_eq!(fs::read(live.join("events.jsonl")).unwrap(), original);
        let initial = super::super::capture::CaptureCursor::initial(&binding).unwrap();
        let snapshot = directory.join("snapshot");
        let delta = super::super::capture::capture_committed_delta(
            &directory,
            &snapshot,
            &binding,
            &initial,
            &super::super::capture::capture_limits(64).unwrap(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(delta.cursor.event_count, 1);
        super::super::capture::verify_captured_delta(&snapshot, &delta, &initial).unwrap();
        drop(guard);
        assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn native_sat_trace_caps_preserve_an_exact_accepted_prefix() {
        let mut log = SatEventLog::default();
        let event = sat::SolverEvent::SolveBegin {
            conflicts: 0,
            conflict_budget: 10,
        };
        log.append(3, &event, 4096, 1).unwrap();
        let prefix = log.bytes.clone();
        assert_eq!(
            log.append(3, &event, 4096, 1).unwrap_err(),
            SAT_EVENT_CAP_REASON
        );
        assert_eq!(log.bytes, prefix);
        assert_eq!(log.events, 1);
        let mut tiny = SatEventLog::default();
        assert_eq!(
            tiny.append(0, &event, 1, 100).unwrap_err(),
            SAT_EVENT_CAP_REASON
        );
        assert!(tiny.bytes.is_empty());
        let row: Value = serde_json::from_slice(&prefix).unwrap();
        assert_eq!(row["sequence"], 1);
        assert_eq!(row["pattern"], 3);
        assert_eq!(
            serde_json::from_value::<sat::SolverEvent>(row["native"].clone()).unwrap(),
            event
        );
    }
    #[test]
    fn finite_domain_selector_is_exact_for_non_subspace_codes() {
        let codes = vec![
            BigUint::from(3u32),
            BigUint::from(9u32),
            BigUint::from(22u32),
        ];
        let s = finite_selector(&codes, 0, 2, 5, 4);
        for i in 0..4 {
            assert_eq!(s.eval(i as Mask, 5).to_biguint(), codes[i % 3]);
        }
    }
    #[test]
    fn wrong_target_does_not_pass_group_lift() {
        let c = BinaryCurve::test_curve_f256();
        let group = Group::Binary(Box::new(c.clone()));
        let p = Element::Binary(c.generator.clone());
        let domain = vec![p.clone(), group.neg(&p)];
        let xs = vec![p.x().unwrap(), p.x().unwrap()];
        let target = group.mul(&p, 3);
        let (found, _, _) = lift(&group, &domain, &xs, &target, 100);
        assert!(found.is_none());
    }
    #[test]
    fn exhaustive_pdp_trace_replays_every_transition() {
        let c = BinaryCurve::test_curve_f256();
        let group = Group::Binary(Box::new(c.clone()));
        let p = Element::Binary(c.generator.clone());
        let target = group.mul(&p, 3);
        let mut search = Search {
            group: &group,
            target: &target,
            domain: std::slice::from_ref(&p),
            limit: 3,
            operations: 0,
            trace: vec![],
            exhausted: false,
            io_error: None,
        };
        assert_eq!(
            search.descend(0, 3, &group.zero(), &mut vec![]),
            Some(vec![0, 0, 0])
        );
        assert_eq!(search.trace.len(), 3);
        assert!(!search.exhausted);
    }

    #[test]
    fn prime_root_certificates_bind_public_target_and_domain_on_both_twists() {
        for twist in [false, true] {
            let mut cfg = config("prime", "root");
            cfg.twist = twist;
            cfg.work_limit = TRACE_EVENT_CAP;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let point = points.iter().min_by_key(|p| p.x()).unwrap();
            let target = group.add(point, point);
            assert_ne!(target, group.zero());
            case.public_instance["target"] = target.json();
            case.public_instance["population"] =
                json!("software-public-planted-prime-root-fixture");
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "prime-root-certificates/v1")
                .unwrap();
            let certificate: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &certificate,
            )
            .unwrap();
            assert_eq!(certificate["total_operation_count_verified"], false);
            let mut tampered = certificate.clone();
            tampered["queries"][0]["fixed_x"] = json!("0");
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &tampered
            )
            .is_err());
            let mut tampered = certificate.clone();
            tampered["charged_work"] = json!(0);
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &tampered
            )
            .is_err());
            let mut wrong_public = case.public_instance.clone();
            wrong_public["twist"] = json!(!twist);
            assert!(verify_prime_root_artifact(
                &wrong_public,
                &case.factor_base,
                &cfg,
                &certificate
            )
            .is_err());
            let mut wrong_base = case.factor_base.clone();
            wrong_base["points"] = json!([]);
            assert!(verify_prime_root_artifact(
                &case.public_instance,
                &wrong_base,
                &cfg,
                &certificate
            )
            .is_err());
            let witness = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &witness.role,
                &serde_json::from_slice(&witness.bytes).unwrap(),
            )
            .unwrap();
        }
    }

    #[test]
    fn prime_root_aggregate_caps_preserve_replayable_censoring() {
        let mut cfg = config("prime", "root");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        for limit in [
            0,
            PRIME_S3_SETUP_WORK - 1,
            PRIME_S3_SETUP_WORK,
            PRIME_S3_SETUP_WORK + 20,
        ] {
            cfg.work_limit = limit;
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(result.outcome["status"], "budget_exhausted");
            assert_eq!(result.outcome["solver_completed"], false);
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "prime-root-certificates/v1")
                .unwrap();
            let certificate = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_prime_root_artifact(
                &case.public_instance,
                &case.factor_base,
                &cfg,
                &certificate,
            )
            .unwrap();
        }
    }

    #[test]
    fn prime_root_envelope_byte_admission_checks_exact_boundary_and_failed_finish() {
        let limits = RootLimits {
            field_work: 10_000,
            split_attempts: 1000,
            events: 10_000,
            degree: 2,
        };
        let full =
            prime_root_trace::solve(&[1, 0, 1], 13, 42, limits.clone(), &mut |_| Ok(())).unwrap();
        prime_root_trace::verify(&full).unwrap();
        assert_eq!(full.stop, RootStop::Complete);
        assert_eq!(full.roots, vec![5, 8]);
        assert!(matches!(full.events.last(), Some(RootEvent::Finish { .. })));
        let bytes = |index, event: &RootEvent| {
            serde_json::to_vec(&prime_root_event_envelope(index, "7", event))
                .unwrap()
                .len()
        };
        let full_bytes = full.events.iter().map(|e| bytes(0, e)).sum::<usize>();
        let mut totals = (0, 0);
        verify_prime_root_event_budget(&full, 0, "7", false, full_bytes, &mut totals).unwrap();
        assert_eq!(totals, (full.events.len() as u64, full_bytes));
        assert!(
            verify_prime_root_event_budget(&full, 0, "7", false, full_bytes - 1, &mut (0, 0),)
                .unwrap_err()
                .contains("acknowledged event cap")
        );

        // Reject the actual last callback at one byte below its exact envelope
        // boundary. Native work is charged, but Finish remains unacknowledged.
        let cap = full_bytes - 1;
        let mut acknowledged_bytes = 0;
        let refused = prime_root_trace::solve(&[1, 0, 1], 13, 42, limits, &mut |event| {
            let size = bytes(0, event);
            if acknowledged_bytes + size > cap {
                return Err(PRIME_ROOT_BYTE_CAP_REASON.into());
            }
            acknowledged_bytes += size;
            Ok(())
        })
        .unwrap();
        prime_root_trace::verify(&refused).unwrap();
        assert_eq!(refused.stop, RootStop::SinkFailure);
        assert!(matches!(
            refused.unacknowledged.as_ref(),
            Some(RootEvent::Finish { .. })
        ));
        assert!(refused.roots.is_empty());
        let mut totals = (0, 0);
        verify_prime_root_event_budget(&refused, 0, "7", true, cap, &mut totals).unwrap();
        assert_eq!(totals, (refused.events.len() as u64, acknowledged_bytes));
        assert_eq!(refused.events.len() + 1, full.events.len());
        for (flag, allowed) in [(false, cap), (true, full_bytes)] {
            assert!(
                verify_prime_root_event_budget(&refused, 0, "7", flag, allowed, &mut (0, 0),)
                    .unwrap_err()
                    .contains("refusal is not justified")
            );
        }
        let mut io_failure = refused.clone();
        io_failure.sink_error = Some("selected payload I/O failure".into());
        prime_root_trace::verify(&io_failure).unwrap();
        assert!(
            verify_prime_root_event_budget(&io_failure, 0, "7", true, cap, &mut (0, 0),)
                .unwrap_err()
                .contains("refusal is not justified")
        );
        assert!(
            verify_prime_root_event_budget(&full, 0, "7", true, full_bytes, &mut (0, 0))
                .unwrap_err()
                .contains("byte-cap flag")
        );
    }

    #[test]
    fn prime_root_envelope_byte_admission_accumulates_across_queries() {
        let report = prime_root_trace::solve(
            &[1, 0, 1],
            13,
            42,
            RootLimits {
                field_work: 10_000,
                split_attempts: 1000,
                events: 10_000,
                degree: 2,
            },
            &mut |_| Ok(()),
        )
        .unwrap();
        prime_root_trace::verify(&report).unwrap();
        assert!(report.complete());
        let query =
            |fixed_x| json!({"fixed_x":fixed_x,"trace_byte_cap_reached":false,"report":report});
        let certificate = json!({"queries":[query("7"), query("8")]});
        let (events, bytes) = prime_root_trace_totals(&certificate).unwrap();
        assert_eq!(events, 2 * report.events.len() as u64);
        let mut first = (0, 0);
        verify_prime_root_event_budget(&report, 0, "7", false, usize::MAX, &mut first).unwrap();
        let mut second = (0, 0);
        verify_prime_root_event_budget(&report, 1, "8", false, usize::MAX, &mut second).unwrap();
        assert_eq!(bytes as usize, first.1 + second.1);
        let exact_cap = first.1 + second.1;
        let mut totals = (0, 0);
        verify_prime_root_event_budget(&report, 0, "7", false, exact_cap, &mut totals).unwrap();
        verify_prime_root_event_budget(&report, 1, "8", false, exact_cap, &mut totals).unwrap();
        assert_eq!(totals, (events, exact_cap));
        // Each query alone fits this cap. Reusing the aggregate instead of
        // resetting it is what rejects the second query's final envelope.
        verify_prime_root_event_budget(&report, 1, "8", false, exact_cap - 1, &mut (0, 0)).unwrap();
        let mut totals = (0, 0);
        verify_prime_root_event_budget(&report, 0, "7", false, exact_cap - 1, &mut totals).unwrap();
        assert!(
            verify_prime_root_event_budget(&report, 1, "8", false, exact_cap - 1, &mut totals,)
                .unwrap_err()
                .contains("acknowledged event cap")
        );
    }

    #[test]
    fn prime_root_payload_failure_preserves_live_prefix_without_completed_certificate() {
        let mut cfg = config("prime", "root");
        // The n=7 fixture has a subgroup of order three: doubling preserves x
        // and makes the first S3 query linear. Use a larger fixed software
        // fixture so this control actually reaches a selected power/GCD state.
        cfg.n = 11;
        cfg.work_limit = TRACE_EVENT_CAP;
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let first = points.iter().min_by_key(|p| p.x()).unwrap();
        let target = group.add(first, first);
        assert_ne!(target, group.zero());
        assert_ne!(first.x(), target.x());
        case.public_instance["target"] = target.json();
        let expected = solve(&cfg, &case).unwrap();
        let state = expected
            .artifacts
            .iter()
            .find(|a| a.role == "native-prime-root-intermediate-state")
            .unwrap();
        let expected_row: Value = serde_json::from_slice(&state.bytes).unwrap();
        let directory = std::env::temp_dir().join(format!(
            "prime-root-payload-failure-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        struct Cleanup(PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        let _cleanup = Cleanup(directory.clone());
        // The live native event is fsynced first. Conflicting sentinel bytes at
        // the selected artifact's exact filename make its payload write fail.
        // The live stream may therefore contain one native event beyond the
        // callback-acknowledged prefix; no equality between them is claimed.
        fs::create_dir_all(directory.join("payloads")).unwrap();
        let sentinel = b"preexisting unrelated payload bytes";
        fs::write(directory.join("payloads").join(&state.path), sentinel).unwrap();
        assert!(solve_with_event_sink(&cfg, &case, &directory).is_err());
        let rows: Vec<Value> = fs::read_to_string(directory.join("events.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert!(!rows.is_empty());
        let last_native = rows
            .iter()
            .map(|row| &row["event"])
            .rfind(|event| event["schema"] == "ml-dataset.prime-root-event/v1")
            .unwrap();
        assert_eq!(*last_native, expected_row);
        let terminal = &rows.last().unwrap()["event"];
        assert_eq!(terminal["kind"], "solver_return");
        assert!(terminal["error"].as_str().is_some_and(|error| {
            error.contains("live payload path collision with different bytes")
        }));
        assert!(terminal["outcome"].is_null());
        assert!(matches!(
            expected_row["native"]["kind"].as_str(),
            Some("power_end" | "gcd_end")
        ));
        assert_eq!(
            fs::read(directory.join("payloads").join(&state.path)).unwrap(),
            sentinel
        );
        assert!(rows
            .iter()
            .all(|row| row["event"]["native"]["kind"] != "finish"));
        assert!(!directory
            .join("payloads/prime-root-certificates.json")
            .exists());
        assert!(!directory.join("payloads/root-call-0000.json").exists());
        assert!(!directory.join("payloads/pdp-witness.json").exists());
        assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
    }

    fn config(family: &str, solver: &str) -> CaseConfig {
        CaseConfig {
            factor_base_policy: None,
            family: family.into(),
            n: 7,
            m: 2,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            degree_bound: 4,
            work_limit: 1000,
            solver: solver.into(),
        }
    }

    #[test]
    fn independent_bsgs_never_needs_the_private_scalar_and_rejects_wrong_labels() {
        for family in ["koblitz", "binary", "prime"] {
            let cfg = config(family, "bsgs");
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["scalar_recovered_without_witness"], true,
                "{family}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "restricted-independent-scalar-label")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            label["scalar"] = json!("0");
            assert!(verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label
            )
            .is_err());
        }
    }

    #[test]
    fn native_algebra_and_root_adapters_replay_a_public_planted_pdp() {
        let mut cfg = config("koblitz", "enumerate");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.add(&points[0], &points[0]);
        assert_ne!(target, group.zero());
        case.public_instance["target"] = target.json();
        case.public_instance["population"] = json!("unit-test-public-planted-pdp");
        for solver in ["enumerate", "root", "f4", "f5", "sat"] {
            cfg.solver = solver.into();
            let result = solve(&cfg, &case).unwrap();
            if solver == "sat" {
                let trace = result
                    .artifacts
                    .iter()
                    .find(|a| a.role == "native-sat-search-events")
                    .unwrap();
                let events: Vec<Value> = String::from_utf8(trace.bytes.clone())
                    .unwrap()
                    .lines()
                    .map(|line| serde_json::from_str(line).unwrap())
                    .collect();
                assert!(events.iter().any(|e| e["native"]["kind"] == "assignment"));
                assert!(events.iter().any(|e| e["native"]["kind"] == "solve_end"));
                assert_eq!(result.outcome["native_event_trace"]["proof_checked"], false);
            }
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{solver}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            label["summands"] = json!([]);
            assert!(verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label
            )
            .is_err());
        }
    }

    fn tiny_prime_chain_case(m: usize, twist: bool) -> (CaseConfig, CaseData) {
        // E/F7: y²=x³+3, P=(1,2), order 13. The d=3 quadratic twist
        // has B=4 and P=(0,2) of order 3. No construction scalar is saved.
        let (b, x, order): (u64, u64, u64) = if twist { (4, 0, 3) } else { (3, 1, 13) };
        let mut cfg = config("prime", "f4");
        cfg.n = 3;
        cfg.m = m;
        cfg.twist = twist;
        cfg.work_limit = 10_000;
        let point = json!({"x":x.to_string(),"y":"2"});
        let mut case = CaseData {
            public_instance: json!({"family":"prime","n":3,"m":m,"twist":twist,
                "field":{"modulus":"7","characteristic":"7","degree":1},
                "curve":{"a":"0","b":b.to_string(),"group_order":order.to_string(),"subgroup_order":order.to_string(),"cofactor":"1"},
                "generator":point,"target":point}),
            private_witness: Value::Null,
            factor_base: json!({"rule":"explicit-public-software-fixture","points":[point]}),
        };
        let group = Group::read(&case).unwrap();
        let p = group.point(&point).unwrap();
        assert!(group.on_curve(&p));
        assert_eq!(group.mul(&p, order), group.zero());
        let summands: Vec<_> = (0..m)
            .map(|i| {
                if twist && i >= 2 && i % 2 == 0 {
                    group.neg(&p)
                } else {
                    p.clone()
                }
            })
            .collect();
        let target = summands
            .iter()
            .fold(group.zero(), |sum, p| group.add(&sum, p));
        assert_ne!(target, group.zero());
        case.public_instance["target"] = target.json();
        (cfg, case)
    }

    fn assert_prime_equations_at(artifact: &Value, coordinates: &[BigUint]) {
        let modulus: BigUint = artifact["modulus"].as_str().unwrap().parse().unwrap();
        for polynomial in artifact["polynomials"].as_array().unwrap() {
            let value =
                polynomial
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(BigUint::from(0u8), |sum, term| {
                        let coefficient: BigUint =
                            term["coefficient"].as_str().unwrap().parse().unwrap();
                        let exponents = term["exponents"].as_array().unwrap();
                        assert_eq!(exponents.len(), coordinates.len());
                        let product =
                            coordinates
                                .iter()
                                .zip(exponents)
                                .fold(coefficient, |value, (x, e)| {
                                    value * x.modpow(&BigUint::from(e.as_u64().unwrap()), &modulus)
                                        % &modulus
                                });
                        (sum + product) % &modulus
                    });
            assert_eq!(value, BigUint::from(0u8), "polynomial {polynomial}");
        }
    }

    #[test]
    fn prime_chain_m3_m4_positive_lifts_replay_without_private_witness() {
        for twist in [false, true] {
            for m in [3, 4] {
                let (cfg, case) = tiny_prime_chain_case(m, twist);
                assert!(case.private_witness.is_null());
                let group = Group::read(&case).unwrap();
                let p = group.point(&case.public_instance["generator"]).unwrap();
                if !twist {
                    assert_eq!(group.mul(&p, 2).json(), json!({"x":"6","y":"3"}));
                    assert_eq!(group.mul(&p, 3).json(), json!({"x":"2","y":"2"}));
                    assert_eq!(group.mul(&p, 4).json(), json!({"x":"4","y":"5"}));
                }
                let result = solve(&cfg, &case).unwrap();
                assert_eq!(
                    result.outcome["pdp_witness_verified"], true,
                    "{}",
                    result.outcome
                );
                assert_eq!(result.outcome["exhaustive_no_decomposition"], false);
                assert_eq!(result.outcome["termination"], "verified_positive_stop");
                let input: Value = serde_json::from_slice(
                    &result
                        .artifacts
                        .iter()
                        .find(|a| a.path == "prime-f4-input.json")
                        .unwrap()
                        .bytes,
                )
                .unwrap();
                verify_prime_f4_chain_input(&case.public_instance, &case.factor_base, &input)
                    .unwrap();
                let certificate = result
                    .artifacts
                    .iter()
                    .find(|a| a.role == "independently-verified-pdp")
                    .unwrap();
                let label: Value = serde_json::from_slice(&certificate.bytes).unwrap();
                verify_solution_artifact(
                    &case.public_instance,
                    &case.factor_base,
                    &certificate.role,
                    &label,
                )
                .unwrap();
                let mut bad = label.clone();
                bad["summands"][0] = Value::Null;
                assert!(verify_solution_artifact(
                    &case.public_instance,
                    &case.factor_base,
                    &certificate.role,
                    &bad
                )
                .is_err());
                for field in [
                    "schema",
                    "m",
                    "modulus",
                    "target",
                    "variable_roles",
                    "domains",
                    "polynomials",
                ] {
                    let mut bad = input.clone();
                    bad[field] = Value::Null;
                    assert!(
                        verify_prime_f4_chain_input(&case.public_instance, &case.factor_base, &bad)
                            .is_err(),
                        "{field}"
                    );
                }
                let events = result
                    .artifacts
                    .iter()
                    .find(|a| a.path == "prime-f4-chain-events.jsonl")
                    .unwrap();
                let rows: Vec<Value> = events
                    .bytes
                    .split(|b| *b == b'\n')
                    .filter(|line| !line.is_empty())
                    .map(|line| serde_json::from_slice(line).unwrap())
                    .collect();
                let calls = rows
                    .iter()
                    .filter(|r| r["event"]["kind"] == "native_result")
                    .count();
                assert_eq!(
                    result.outcome["counters"]["recursive_native_f4_runs"],
                    json!(calls)
                );
                assert!(calls > 0);
                assert!(rows.iter().any(|r| r["event"]["kind"] == "group_lift"
                    && r["event"]["group_lift_verified"] == true));
            }
        }
    }

    #[test]
    fn prime_chain_equations_match_group_prefixes_and_keep_auxiliaries_unrestricted() {
        for twist in [false, true] {
            for m in [2, 3, 4, 15] {
                let (mut cfg, mut case) = tiny_prime_chain_case(if m == 15 { 3 } else { m }, twist);
                cfg.m = m;
                case.public_instance["m"] = json!(m);
                let group = Group::read(&case).unwrap();
                let p = group.point(&case.public_instance["generator"]).unwrap();
                // Alternating signs avoid every intermediate infinity even for
                // m15 on the order-3 twist. This is a public group fixture.
                let summands: Vec<_> = (0..m)
                    .map(|i| {
                        if i >= 2 && i % 2 == 0 {
                            group.neg(&p)
                        } else {
                            p.clone()
                        }
                    })
                    .collect();
                let mut sum = group.zero();
                let mut auxiliaries = vec![];
                for (i, point) in summands.iter().enumerate() {
                    sum = group.add(&sum, point);
                    assert_ne!(sum, group.zero());
                    if i >= 1 && i < m - 1 {
                        auxiliaries.push(sum.x().unwrap());
                    }
                }
                case.public_instance["target"] = sum.json();
                let domain = domain(&group, &case).unwrap();
                let mut coordinates: Vec<_> = summands.iter().map(|p| p.x().unwrap()).collect();
                coordinates.extend(auxiliaries);
                let artifact = if m == 2 {
                    let result = solve(&cfg, &case).unwrap();
                    serde_json::from_slice(
                        &result
                            .artifacts
                            .iter()
                            .find(|a| a.path == "prime-f4-input.json")
                            .unwrap()
                            .bytes,
                    )
                    .unwrap()
                } else {
                    let input = prime_chain_input(m, &group, &domain, &sum).unwrap();
                    assert_eq!(input.polynomials.len(), 2 * m - 1);
                    assert_eq!(input.domains.len(), 2 * m - 2);
                    assert!(input.domains[..m].iter().all(Option::is_some));
                    assert!(input.domains[m..].iter().all(Option::is_none));
                    assert_eq!(
                        input.artifact["membership_variables"],
                        json!((0..m).collect::<Vec<_>>())
                    );
                    verify_prime_f4_chain_input(
                        &case.public_instance,
                        &case.factor_base,
                        &input.artifact,
                    )
                    .unwrap();
                    input.artifact
                };
                assert_prime_equations_at(&artifact, &coordinates);
            }
        }
    }

    #[test]
    fn prime_chain_negative_and_bounded_outcomes_preserve_their_scope() {
        let (cfg, mut case) = tiny_prime_chain_case(3, false);
        let group = Group::read(&case).unwrap();
        let domain = domain(&group, &case).unwrap();
        let target = group.point(&case.public_instance["target"]).unwrap();
        let baseline = solve(&cfg, &case).unwrap();
        let count = baseline.outcome["counters"]["persisted_trace_events"]
            .as_u64()
            .unwrap();
        let mut cap = PrimeChainLimits::from(&cfg);
        cap.events = count - 1;
        let positive = prime_f4_chain(&cfg, &group, &domain, &target, cap).unwrap();
        assert_eq!(positive.outcome["pdp_witness_verified"], true);
        assert_eq!(positive.outcome["termination"], "trace_budget_exhausted");
        for expected in [
            "trace_budget_exhausted",
            "native_deadline",
            "lift_budget_exhausted",
        ] {
            let mut limits = PrimeChainLimits::from(&cfg);
            match expected {
                "trace_budget_exhausted" => limits.bytes = 1,
                "native_deadline" => limits.time = Duration::ZERO,
                "lift_budget_exhausted" => limits.lift_additions = 0,
                _ => unreachable!(),
            }
            let result = prime_f4_chain(&cfg, &group, &domain, &target, limits).unwrap();
            assert_eq!(result.outcome["termination"], expected);
            assert_eq!(result.outcome["status"], "budget_exhausted");
            assert_eq!(result.outcome["solver_completed"], false);
            assert_eq!(result.outcome["exhaustive_no_decomposition"], false);
            if expected == "trace_budget_exhausted" {
                assert_eq!(result.outcome["counters"]["recursive_native_f4_runs"], 0);
            }
        }
        // Three ±P sum to ±P or ±3P, never 2P in the order-13 group.
        case.public_instance["target"] = group.mul(&domain[0], 2).json();
        let negative = solve(&cfg, &case).unwrap();
        assert_eq!(
            negative.outcome["encoding_search_completed"], true,
            "{}",
            negative.outcome
        );
        assert_eq!(negative.outcome["pdp_witness_verified"], false);
        assert_eq!(negative.outcome["exhaustive_no_decomposition"], false);
        assert!(negative.outcome["first_fall_degree"].is_null());
        assert!(negative.outcome["last_fall_degree"].is_null());
    }

    #[test]
    fn prime_chain_trace_serialization_stops_before_materializing_large_events() {
        use serde::ser::SerializeSeq;
        struct LargeEvent<'a>(&'a std::cell::Cell<usize>);
        impl serde::Serialize for LargeEvent<'_> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut sequence = serializer.serialize_seq(Some(1_000_000))?;
                for _ in 0..1_000_000 {
                    self.0.set(self.0.get() + 1);
                    sequence.serialize_element(&12345u64)?;
                }
                sequence.end()
            }
        }
        let mut events = PrimeChainEvents::default();
        let limits = PrimeChainLimits {
            events: 10,
            bytes: 256,
            lift_additions: 10,
            time: Duration::from_secs(1),
        };
        events.append(&json!({"kind":"first"}), limits).unwrap();
        let prefix = events.bytes.clone();
        let calls = std::cell::Cell::new(0);
        assert_eq!(
            events.append(&LargeEvent(&calls), limits).unwrap_err(),
            PRIME_CHAIN_TRACE_CAP
        );
        assert!(calls.get() < 100);
        assert_eq!(events.bytes, prefix);
        let count = calls.get();
        assert!(events.append(&LargeEvent(&calls), limits).is_err());
        assert_eq!(calls.get(), count);
    }

    #[test]
    fn prime_chain_event_io_failure_retains_positive_for_canonical_worker_write() {
        let (cfg, case) = tiny_prime_chain_case(3, false);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("prime-chain-io-{}-{nonce}", std::process::id()));
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                FAIL_PRIME_LIFT_EVENT.with(|flag| flag.set(false));
            }
        }
        let _reset = Reset;
        FAIL_PRIME_LIFT_EVENT.with(|flag| flag.set(true));
        let result = solve_with_event_sink(&cfg, &case, &directory.join("live")).unwrap();
        assert_eq!(result.outcome["pdp_witness_verified"], true);
        assert_eq!(result.outcome["termination"], "trace_sink_error");
        assert_eq!(result.outcome["live_persistence_complete"], false);
        assert_eq!(result.outcome["exposed_event_stream_complete"], false);
        assert_eq!(result.outcome["canonical_artifact_write_required"], true);
        assert!(!result.outcome["live_persistence_errors"]
            .as_array()
            .unwrap()
            .is_empty());
        let prefix = fs::read(directory.join("live/events.jsonl")).unwrap();
        assert!(!prefix.is_empty());
        for line in prefix
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
        {
            let _: Value = serde_json::from_slice(line).unwrap();
        }
        assert!(directory
            .join("live/payloads/prime-f4-input.json")
            .is_file());
        let canonical = directory.join("canonical");
        fs::create_dir(&canonical).unwrap();
        for payload in &result.artifacts {
            super::super::storage::write_bytes(
                &canonical,
                &payload.path,
                &payload.role,
                &payload.encoding,
                &payload.bytes,
            )
            .unwrap();
        }
        let certificate: Value =
            serde_json::from_slice(&fs::read(canonical.join("pdp-witness.json")).unwrap()).unwrap();
        verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            "independently-verified-pdp",
            &certificate,
        )
        .unwrap();
        let input: Value =
            serde_json::from_slice(&fs::read(canonical.join("prime-f4-input.json")).unwrap())
                .unwrap();
        verify_prime_f4_chain_input(&case.public_instance, &case.factor_base, &input).unwrap();
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn bound_prime_chain_payload_cap_preserves_canonical_equations() {
        let (cfg, case) = tiny_prime_chain_case(3, false);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("bound-chain-cap-{}-{nonce}", std::process::id()));
        let binding = AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "5".repeat(40),
            binary_sha256: "6".repeat(64),
            cell_index: 0,
            attempt_sha256: "7".repeat(64),
        };
        let mut limits = super::super::capture::capture_limits(64).unwrap();
        limits.max_payload_bytes = 0;
        let result = solve_with_bound_event_sink(&cfg, &case, &directory, binding, limits).unwrap();
        assert_eq!(result.outcome["live_persistence_complete"], false);
        assert_eq!(result.outcome["canonical_artifact_write_required"], true);
        assert!(!result.outcome["live_persistence_errors"]
            .as_array()
            .unwrap()
            .is_empty());
        let input = result
            .artifacts
            .iter()
            .find(|a| a.path == "prime-f4-input.json")
            .unwrap();
        let value: Value = serde_json::from_slice(&input.bytes).unwrap();
        verify_prime_f4_chain_input(&case.public_instance, &case.factor_base, &value).unwrap();
        assert!(directory.join("committed.json").is_file());
        assert_eq!(result.outcome["native_backend_invoked"], false);
        assert_eq!(result.outcome["solver_completed"], false);
        assert_eq!(result.outcome["termination"], "trace_sink_error");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn prime61_chain_m3_m15_retains_exact_wide_equations_without_elimination() {
        for twist in [false, true] {
            let mut cfg = config("prime", "f4");
            cfg.n = 61;
            cfg.twist = twist;
            cfg.factor_base_size = 4;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let group = Group::read(&case).unwrap();
            let p = domain(&group, &case).unwrap()[0].clone();
            for m in [3, 15] {
                cfg.m = m;
                case.public_instance["m"] = json!(m);
                let mut sum = group.zero();
                let mut coordinates = vec![];
                let mut auxiliaries = vec![];
                for i in 0..m {
                    let point = if i >= 2 && i % 2 == 0 {
                        group.neg(&p)
                    } else {
                        p.clone()
                    };
                    coordinates.push(point.x().unwrap());
                    sum = group.add(&sum, &point);
                    if i >= 1 && i < m - 1 {
                        auxiliaries.push(sum.x().unwrap());
                    }
                }
                coordinates.extend(auxiliaries);
                case.public_instance["target"] = sum.json();
                let result = solve(&cfg, &case).unwrap();
                assert_eq!(result.outcome["status"], "unsupported");
                assert_eq!(result.outcome["native_backend_invoked"], false);
                assert_eq!(result.outcome["exhaustive_no_decomposition"], false);
                assert!(!result
                    .artifacts
                    .iter()
                    .any(|a| a.path == "prime-f4-chain-events.jsonl"));
                let input: Value = serde_json::from_slice(
                    &result
                        .artifacts
                        .iter()
                        .find(|a| a.path == "prime-f4-input.json")
                        .unwrap()
                        .bytes,
                )
                .unwrap();
                assert_eq!(input["modulus"], "2305843009213693951");
                assert_prime_equations_at(&input, &coordinates);
                verify_prime_f4_chain_input(&case.public_instance, &case.factor_base, &input)
                    .unwrap();
            }
        }
    }

    #[test]
    fn prime61_f4_retains_public_equations_without_entering_small_prime_backend() {
        for twist in [false, true] {
            let mut cfg = config("prime", "f4");
            cfg.n = 61;
            cfg.twist = twist;
            cfg.factor_base_size = 4;
            cfg.work_limit = 64;
            cfg.degree_bound = 2;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let p = &points[0];
            case.public_instance["target"] = group.add(p, p).json();
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(result.outcome["status"], "unsupported");
            assert_eq!(result.outcome["native_backend_invoked"], false);
            assert_eq!(result.outcome["solver_completed"], false);
            assert_eq!(result.outcome["algebra_completed"], false);
            assert_eq!(result.outcome["pdp_witness_verified"], false);
            assert!(!result
                .artifacts
                .iter()
                .any(|a| a.path == "prime-f4-output.json"));
            let input = result
                .artifacts
                .iter()
                .find(|a| a.path == "prime-f4-input.json")
                .unwrap();
            let input: Value = serde_json::from_slice(&input.bytes).unwrap();
            let modulus = (BigUint::from(1u8) << 61usize) - BigUint::from(1u8);
            assert_eq!(input["modulus"], modulus.to_string());
            assert_eq!(input["native_normalization_applied"], false);
            let x = p.x().unwrap();
            // Independently evaluate S3 and both membership polynomials at the
            // public planted two-point decomposition using arbitrary precision.
            let polys = input["polynomials"].as_array().unwrap();
            assert_eq!(polys.len(), 3);
            for polynomial in polys {
                let value =
                    polynomial
                        .as_array()
                        .unwrap()
                        .iter()
                        .fold(BigUint::from(0u8), |sum, term| {
                            let coefficient: BigUint =
                                term["coefficient"].as_str().unwrap().parse().unwrap();
                            let exponent: u32 = term["exponents"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|e| e.as_u64().unwrap() as u32)
                                .sum();
                            (sum + coefficient * x.modpow(&BigUint::from(exponent), &modulus))
                                % &modulus
                        });
                assert_eq!(value, BigUint::from(0u8));
            }
        }
        let caps = capabilities();
        let f4 = caps["solvers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == "f4")
            .unwrap();
        assert_eq!(f4["prime_modulus_limit_exclusive"], "4294967296");
        assert_eq!(f4["supervised_wide_prime_profile"]["m"], json!([2, 2]));
        assert_eq!(
            f4["supervised_wide_prime_profile"]["modulus_max_inclusive"],
            "2305843009213693951"
        );
        assert_eq!(
            f4["supervised_wide_prime_profile"]["requires_original_execution_context"],
            true
        );
    }

    #[test]
    #[ignore = "360-cell endpoint integration sweep; native SAT has an independent 10-second per-call deadline"]
    fn all_adapter_endpoints_return_structured_outcomes() {
        let methods = [
            "enumerate",
            "bsgs",
            "sat",
            "weil",
            "f4",
            "f5",
            "f6",
            "root",
            "pkm",
            "double-large-prime",
        ];
        let mut visited = 0;
        for n in [23, 61] {
            for family in ["koblitz", "binary", "prime"] {
                for twist in [false, true] {
                    let mut cfg = config(family, "enumerate");
                    cfg.n = n;
                    cfg.twist = twist;
                    cfg.factor_base_size = 4;
                    cfg.work_limit = 64;
                    cfg.degree_bound = 2;
                    let mut case = curves::generate(&cfg).unwrap();
                    case.private_witness = Value::Null;
                    for m in [2, 4, 15] {
                        cfg.m = m;
                        case.public_instance["m"] = json!(m);
                        for method in methods {
                            cfg.solver = method.into();
                            let label =
                                format!("{family} n={n} twist={twist} m={m} solver={method}");
                            let result =
                                solve(&cfg, &case).unwrap_or_else(|e| panic!("{label}: {e}"));
                            assert_eq!(
                                result.outcome["schema"], "ml-dataset.solver-outcome/v1",
                                "{label}"
                            );
                            assert!(
                                matches!(
                                    result.outcome["status"].as_str(),
                                    Some(
                                        "verified" | "partial" | "unsupported" | "budget_exhausted"
                                    )
                                ),
                                "{label}: {}",
                                result.outcome
                            );
                            assert!(
                                result.outcome["reason"]
                                    .as_str()
                                    .is_some_and(|s| !s.is_empty()),
                                "{label}"
                            );
                            assert!(result.outcome["solver_completed"].is_boolean(), "{label}");
                            assert!(result.outcome["first_fall_degree"].is_null(), "{label}");
                            assert!(result.outcome["last_fall_degree"].is_null(), "{label}");
                            if matches!(
                                result.outcome["status"].as_str(),
                                Some("unsupported" | "budget_exhausted")
                            ) {
                                assert_eq!(result.outcome["solver_completed"], false, "{label}");
                            }
                            for artifact in result.artifacts.iter().filter(|a| {
                                matches!(
                                    a.role.as_str(),
                                    "independently-verified-pdp"
                                        | "restricted-independent-scalar-label"
                                )
                            }) {
                                verify_solution_artifact(
                                    &case.public_instance,
                                    &case.factor_base,
                                    &artifact.role,
                                    &serde_json::from_slice(&artifact.bytes).unwrap(),
                                )
                                .unwrap_or_else(|e| panic!("{label}: {e}"));
                            }
                            visited += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(visited, 360);
    }

    #[test]
    fn prime_f4_and_native_pkm_bind_their_actual_domains() {
        let mut cfg = config("prime", "f4");
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let p = points
            .iter()
            .filter(|p| p.x().is_some_and(|x| x != BigUint::from(0u8)))
            .min_by_key(|p| p.x())
            .unwrap();
        case.public_instance["target"] = group.add(p, p).json();
        for solver in ["root", "f4", "pkm"] {
            cfg.solver = solver.into();
            let result = solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["pdp_witness_verified"], true,
                "{solver}: {}",
                result.outcome
            );
            let artifact = result
                .artifacts
                .iter()
                .find(|a| a.role == "independently-verified-pdp")
                .unwrap();
            let mut label: Value = serde_json::from_slice(&artifact.bytes).unwrap();
            verify_solution_artifact(
                &case.public_instance,
                &case.factor_base,
                &artifact.role,
                &label,
            )
            .unwrap();
            if solver == "pkm" {
                let base_artifact = result
                    .artifacts
                    .iter()
                    .find(|a| a.path == "pkm-factor-base.json")
                    .unwrap();
                let base: Value = serde_json::from_slice(&base_artifact.bytes).unwrap();
                verify_pkm_factor_base(&case.public_instance, &case.factor_base, &base).unwrap();
                let mut subset = base.clone();
                subset["definition"]["points"].as_array_mut().unwrap().pop();
                assert!(
                    verify_pkm_factor_base(&case.public_instance, &case.factor_base, &subset)
                        .is_err()
                );
                let mut wrong_map = base.clone();
                wrong_map["coefficient_of"][0] = json!("0");
                assert!(verify_pkm_factor_base(
                    &case.public_instance,
                    &case.factor_base,
                    &wrong_map
                )
                .is_err());
                assert_eq!(label["factor_base_definition"]["kind"], "prime-kummer");
                label["factor_base_definition"]["g"] = json!("0");
                assert!(verify_solution_artifact(
                    &case.public_instance,
                    &case.factor_base,
                    &artifact.role,
                    &label
                )
                .is_err());
                assert!(result
                    .artifacts
                    .iter()
                    .any(|a| a.role == "native-pkm-exact-system-and-solver-artifacts"));
            }
        }
    }

    #[test]
    fn live_sink_persists_exposed_prefix_and_refuses_existing_stream() {
        let mut cfg = config("koblitz", "enumerate");
        cfg.work_limit = 5;
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "crypto-dataset-live-{}-{nonce}",
            std::process::id()
        ));
        let result = solve_with_event_sink(&cfg, &case, &dir).unwrap();
        assert_eq!(result.outcome["checkpoint_mid_call"], true);
        let stream = fs::read_to_string(dir.join("events.jsonl")).unwrap();
        let events = stream
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        assert!(events
            .iter()
            .any(|row| row["event"]["kind"] == "pdp_transition"));
        for (i, row) in events.iter().enumerate() {
            assert_eq!(row["sequence"].as_u64(), Some(i as u64 + 1));
        }
        for payload in result.artifacts {
            assert_eq!(
                fs::read(dir.join("payloads").join(payload.path)).unwrap(),
                payload.bytes
            );
        }
        assert!(solve_with_event_sink(&cfg, &case, &dir).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod bounded_witness_bridge_tests {
    use super::*;
    #[test]
    fn witness_bridge_requires_bound_sink_before_allocation() {
        LIVE_SINK.with(|slot| assert!(slot.borrow().is_none()));
        let mut budget = super::super::capture::WitnessBudget::new(
            16_777_216,
            1024 * 1024,
            std::time::Instant::now() + std::time::Duration::from_secs(30),
        );
        assert!(matches!(
            prepare_bound_witness(&mut budget),
            Err(super::super::capture::WitnessError::Unbound)
        ));
        assert_eq!(budget.reserved_bytes, 0);
        #[cfg(unix)]
        {
            LIVE_SINK.with(|slot| {
                *slot.borrow_mut() = Some(EventSink::Legacy(LiveSink {
                    directory: PathBuf::new(),
                    events: File::open("/dev/null").unwrap(),
                    sequence: 0,
                }))
            });
            let result = prepare_bound_witness(&mut budget);
            LIVE_SINK.with(|slot| {
                slot.borrow_mut().take();
            });
            assert!(matches!(
                result,
                Err(super::super::capture::WitnessError::Unbound)
            ));
            assert_eq!(budget.reserved_bytes, 0);
        }
    }
}

#[cfg(test)]
mod verified_worker_original_tests {
    use super::*;
    use crate::ml_dataset::{self as dataset, storage};
    pub(super) struct Fixture {
        pub(super) root: PathBuf,
        pub(super) config: CaseConfig,
        pub(super) case: CaseData,
        pub(super) binding: AttemptBinding,
    }
    impl Fixture {
        fn new() -> Self {
            Self::with_shape(23, false, 2)
        }
        pub(super) fn with_shape(n: u32, twist: bool, factor_base_size: usize) -> Self {
            Self::with_profile(n, twist, factor_base_size, "enumerate", 100, false, false)
        }
        /// Actual frozen software-control plan; no edits to a live attempt.
        pub(super) fn with_runtime_shape(
            n: u32,
            twist: bool,
            factor_base_size: usize,
            solver: &str,
        ) -> Self {
            Self::with_profile(n, twist, factor_base_size, solver, 16384, true, false)
        }
        pub(super) fn with_positive_runtime_shape(
            n: u32,
            twist: bool,
            factor_base_size: usize,
            solver: &str,
        ) -> Self {
            Self::with_profile(n, twist, factor_base_size, solver, 16384, true, true)
        }
        fn with_profile(
            n: u32,
            twist: bool,
            factor_base_size: usize,
            solver: &str,
            work_limit: u64,
            execution_required: bool,
            positive_control: bool,
        ) -> Self {
            Self::with_policy_profile(
                n,
                twist,
                factor_base_size,
                solver,
                work_limit,
                execution_required,
                positive_control,
                None,
            )
        }
        pub(super) fn with_policy_profile(
            n: u32,
            twist: bool,
            factor_base_size: usize,
            solver: &str,
            work_limit: u64,
            execution_required: bool,
            positive_control: bool,
            policy: Option<crate::ml_dataset::types::FactorBasePolicy>,
        ) -> Self {
            Self::with_family_policy_profile(
                n,
                twist,
                factor_base_size,
                solver,
                work_limit,
                execution_required,
                positive_control,
                policy,
                "koblitz",
            )
        }
        pub(super) fn with_ordinary_runtime_shape(n: u32, twist: bool, solver: &str) -> Self {
            Self::with_ordinary_control_shape(n, twist, solver, false)
        }
        pub(super) fn with_ordinary_control_shape(
            n: u32,
            twist: bool,
            solver: &str,
            controlled: bool,
        ) -> Self {
            let fixture = Self::with_family_policy_profile(
                n,
                twist,
                4,
                solver,
                16384,
                true,
                controlled,
                Some(crate::ml_dataset::types::FactorBasePolicy::Legacy {}),
                "binary",
            );
            // Test-only supervisor setup: these are the original frozen attempt
            // and trace limits, not permission to activate ordinary dispatch.
            dataset::telemetry::checked::install_policy(&fixture.root).unwrap();
            let binding = AttemptBinding::from_cell_files(&fixture.root).unwrap();
            assert_eq!(binding, fixture.binding);
            let run: Value = dataset::read_json(&fixture.root.join("run.json")).unwrap();
            let limits =
                dataset::capture::capture_limits(run["trace_mib"].as_u64().unwrap()).unwrap();
            storage::write_json(
                &fixture.root,
                "capture-binding.json",
                "partial-capture-binding",
                &binding,
            )
            .unwrap();
            storage::write_json(
                &fixture.root,
                "capture-limits.json",
                "partial-capture-limits",
                &limits,
            )
            .unwrap();
            let admitted = dataset::admit_capture_context(&fixture.root)
                .unwrap()
                .unwrap();
            assert_eq!(admitted, (binding, limits));
            assert!(!fixture.root.join("public.json").exists());
            assert!(!fixture.root.join("factor-base.json").exists());
            fixture
        }
        fn with_family_policy_profile(
            n: u32,
            twist: bool,
            factor_base_size: usize,
            solver: &str,
            work_limit: u64,
            execution_required: bool,
            positive_control: bool,
            policy: Option<crate::ml_dataset::types::FactorBasePolicy>,
            family: &str,
        ) -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir()
                .join(format!("verified-worker-{}-{nonce}", std::process::id()));
            fs::create_dir(&root).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1", "purpose":"verified worker input software controls"});
            if positive_control {
                protocol["software_control"] = json!({
                    "population":"conditioned-positive-pdp-software-control",
                    "field_degree":23,
                    "target_rule":"first nonzero retained factor-base point P; valid tuple P,P,-P",
                    "negative_control_rule":"first scalar in 1..=65 whose nonzero generator multiple is outside all 64 ordered signed factor-base triples; preserve curve generator and factor base",
                    "known_answer_rule":"bounded native-arithmetic BSGS relative to unchanged original generator, then independent curves::verify",
                    "bsgs_baby_slot_cap":4096,
                    "freeze_boundary":"control rule before generation; all original bytes final before token/capture admission",
                    "root_failure_rule":"conflicting raw pdp-witness.json before producer entry"
                });
            }
            let body = dataset::protocol_canonical_bytes(&protocol).unwrap();
            protocol["freeze"] = json!({"status":"frozen", "protocol_sha256":dataset::digest(&body), "filed_before_affected_data":true, "frozen_at":"2026-10-31T00:00:00Z"});
            let bytes = serde_json::to_vec(&protocol).unwrap();
            let plan = dataset::DatasetPlan {
                factor_base_policies: policy.clone().into_iter().collect(),
                schema: if policy.is_some() {
                    "crypto.ml-dataset.plan/v2"
                } else {
                    "crypto.ml-dataset.plan/v1"
                }
                .into(),
                field_sizes: vec![n],
                arities: vec![3],
                families: vec![family.into()],
                solvers: vec![solver.into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 42,
                factor_base_size,
                degree_bound: 3,
                work_limit,
                twists: true,
                protocol_sha256: Some(dataset::digest(&bytes)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let hash = plan.binding_sha256().unwrap();
            let index = u64::from(twist);
            let config = plan.cell(index).unwrap();
            assert_eq!(config.twist, twist);
            let mut run = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":hash,"source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64)});
            if execution_required {
                run["execution_context_required"] = true.into();
                run["seconds_per_case"] = 60.into();
                run["memory_mib"] = 256.into();
                run["trace_mib"] = 128.into();
            }
            storage::write_json(&root, "run.json", "run", &run).unwrap();
            storage::write_json(&root, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&root, "config.json", "config", &config).unwrap();
            storage::write_json(
                &root,
                "attempt.json",
                "attempt",
                &json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":index,"plan_sha256":hash}),
            )
            .unwrap();
            storage::write_bytes(&root, "protocol.json", "protocol", "json", &bytes).unwrap();
            let binding = AttemptBinding::from_cell_files(&root).unwrap();
            let case = curves::generate(&config).unwrap();
            Self {
                root,
                config,
                case,
                binding,
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            if std::thread::panicking() {
                eprintln!(
                    "preserved failed verified-worker fixture: {}",
                    self.root.display()
                );
            } else {
                let _ = fs::remove_dir_all(&self.root);
            }
        }
    }
    #[test]
    fn worker_token_rejects_self_consistent_policy_substitution_before_original_writes() {
        use crate::ml_dataset::types::FactorBasePolicy;
        for replace_policy in [true, false] {
            let policy = FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 64,
            };
            // Recipe, plan, config and binding are frozen before generating or verifying input.
            let f = Fixture::with_policy_profile(
                11,
                false,
                4,
                "enumerate",
                16384,
                false,
                false,
                Some(policy.clone()),
            );
            let seed = dataset::factor_bases::sampling_seed(&f.config);
            let wrong = if replace_policy {
                FactorBasePolicy::SampledXProjectedControl {
                    candidate_limit: 32,
                }
            } else {
                policy
            };
            let mut case = f.case.clone();
            case.factor_base = dataset::factor_bases::build(
                &case.public_instance,
                &wrong,
                f.config.factor_base_size,
                if replace_policy { seed } else { seed ^ 1 },
                false,
            )
            .unwrap()
            .0;
            curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
            let mut registry = Vec::new();
            let error = verify_and_store_worker_originals(
                &f.config,
                &case,
                &f.root,
                Some(&f.binding),
                &mut registry,
            )
            .err()
            .expect("substituted recipe must not mint token");
            assert!(error.contains("recipe or control seed differs from frozen config"));
            assert!(!f.root.join("public.json").exists());
            assert!(!f.root.join("factor-base.json").exists());
        }
        let f = Fixture::with_policy_profile(
            11,
            false,
            4,
            "enumerate",
            16384,
            false,
            false,
            Some(FactorBasePolicy::Legacy {}),
        );
        let mut registry = Vec::new();
        verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        assert!(f.root.join("factor-base.json").exists());
    }

    #[test]
    fn worker_koblitz_dispatch_rejects_missing_context_before_capture_creation() {
        let f = Fixture::with_runtime_shape(23, false, 4, "f4");
        let mut registry = Vec::new();
        let original = verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let live = f.root.join("live");
        let error = solve_with_verified_worker_originals(
            &f.config,
            &f.case,
            &live,
            f.binding.clone(),
            dataset::capture::capture_limits(16).unwrap(),
            None,
            &original,
        )
        .err()
        .expect("missing original execution context must be refused");
        assert!(error.contains("requires original execution context"));
        assert!(!live.exists());
        assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
    }

    #[test]
    fn worker_koblitz_dispatch_requires_exact_supported_profile() {
        for twist in [false, true] {
            let mut f = Fixture::with_shape(23, twist, 4);
            f.config.solver = "f4".into();
            assert!(koblitz_worker_profile(&f.config, &f.case));
            f.config.solver = "f5".into();
            assert!(koblitz_worker_profile(&f.config, &f.case));
            for variant in 0..8 {
                let mut config = f.config.clone();
                match variant {
                    0 => config.family = "binary".into(),
                    1 => config.m = 4,
                    2 => config.n = 32,
                    3 => config.factor_base_size = 2,
                    4 => config.degree_bound = 0,
                    5 => config.degree_bound = 7,
                    6 => config.work_limit = 0,
                    _ => config.solver = "f6".into(),
                }
                assert!(
                    !koblitz_worker_profile(&config, &f.case),
                    "variant {variant}"
                );
            }
            let mut malformed = f.case.clone();
            malformed.factor_base["points"][1] = malformed.factor_base["points"][0].clone();
            assert!(!koblitz_worker_profile(&f.config, &malformed));
        }
    }

    #[test]
    fn verified_worker_originals_bind_real_files_and_reject_substitution() {
        let f = Fixture::new();
        let mut registry = Vec::new();
        let original = verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let live = f.root.join("live");
        original
            .matches_bound(&f.config, &f.case, &live, &f.binding)
            .unwrap();
        assert_eq!(
            hex::encode(original.public_sha256),
            dataset::digest(&fs::read(f.root.join("public.json")).unwrap())
        );
        assert_eq!(
            hex::encode(original.factor_base_sha256),
            dataset::digest(&fs::read(f.root.join("factor-base.json")).unwrap())
        );
        assert!(original
            .matches_bound(&f.config.clone(), &f.case, &live, &f.binding)
            .is_err());
        assert!(original
            .matches_bound(&f.config, &f.case.clone(), &live, &f.binding)
            .is_err());
        assert!(original
            .matches_bound(&f.config, &f.case, &f.root.join("other"), &f.binding)
            .is_err());
        let mut wrong = f.binding.clone();
        wrong.cell_index += 1;
        assert!(original
            .matches_bound(&f.config, &f.case, &live, &wrong)
            .is_err());
        for name in ["public.json", "factor-base.json", "config.json"] {
            let path = f.root.join(name);
            let saved = fs::read(&path).unwrap();
            fs::write(&path, b"{}").unwrap();
            assert!(
                original
                    .matches_bound(&f.config, &f.case, &live, &f.binding)
                    .is_err(),
                "{name}"
            );
            fs::write(path, saved).unwrap();
        }
        let result = solve_with_verified_worker_originals(
            &f.config,
            &f.case,
            &live,
            f.binding.clone(),
            dataset::capture::capture_limits(16).unwrap(),
            None,
            &original,
        )
        .unwrap();
        assert!(result.data.outcome.is_object());
        assert!(result.stored.is_empty());
        assert!(live.join("committed.json").is_file());
    }
    #[test]
    fn verified_worker_originals_require_both_checks_and_successful_storage() {
        for failure in 0..4 {
            let mut f = Fixture::new();
            let mut registry = Vec::new();
            match failure {
                0 => f.case.private_witness["schema"] = json!("invalid"),
                1 => f.case.factor_base["schema"] = json!("invalid"),
                2 => {
                    fs::write(f.root.join("public.json"), b"conflict").unwrap();
                }
                _ => f.binding.cell_index += 1,
            }
            assert!(
                verify_and_store_worker_originals(
                    &f.config,
                    &f.case,
                    &f.root,
                    Some(&f.binding),
                    &mut registry
                )
                .is_err(),
                "{failure}"
            );
            assert!(!f.root.join("factor-base.json").exists());
            assert!(!f.root.join("live").exists());
        }
    }
}
#[cfg(test)]
mod capability_dispatch_metadata_controls {
    use super::*;
    fn config() -> CaseConfig {
        CaseConfig {
            factor_base_policy: None,
            family: "koblitz".into(),
            n: 23,
            m: 3,
            twist: false,
            curve_seed: 1,
            generator_index: 0,
            key_index: 0,
            seed: 42,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 16384,
            solver: "f4".into(),
        }
    }
    fn entry(caps: &Value, id: &str) -> Value {
        caps["solvers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["id"] == id)
            .unwrap()
            .clone()
    }
    #[test]
    fn koblitz_metadata_describes_only_the_supervised_dispatch_extension() {
        let caps = capabilities();
        let f4 = entry(&caps, "f4");
        let f5 = entry(&caps, "f5");
        assert_eq!(f4["m_by_family"]["binary"], json!([2, 2]));
        assert_eq!(f4["m_by_family"]["koblitz"], json!([2, 2]));
        assert_eq!(f5["m"], json!([2, 2]));
        let p = &f4["supervised_koblitz_m3_profile"];
        assert_eq!(p, &f5["supervised_koblitz_m3_profile"]);
        assert_eq!(p["family"], "koblitz");
        assert_eq!(p["m"], json!([3, 3]));
        assert_eq!(p["n_ranges_inclusive"], json!([[23, 31], [47, 61]]));
        assert_eq!(p["degree_bound_inclusive"], json!([1, 6]));
        assert_eq!(p["factor_base_size"], 4);
        assert_eq!(p["signed_domain_points"], 4);
        assert_eq!(p["distinct_x_codes"], 2);
        assert_eq!(p["twists"], json!([false, true]));
        for key in [
            "requires_verified_worker_originals",
            "requires_original_execution_context",
            "requires_bound_capture",
            "requires_positive_work_limit",
            "requires_distinct_signed_points",
            "requires_sign_closed_factor_base",
        ] {
            assert_eq!(p[key], true, "{key}");
        }
        for id in ["weil", "f6", "root", DYNAMIC_PROFILE] {
            assert!(entry(&caps, id)["supervised_koblitz_m3_profile"].is_null());
        }
        // Shape-only dispatcher control, not a mathematical curve fixture.
        // Original-input verification remains a separate mandatory gate.
        let case = CaseData {
            public_instance: Value::Null,
            private_witness: Value::Null,
            factor_base: json!({"points":[{"x":"1","y":"2"},{"x":"1","y":"3"},{"x":"2","y":"4"},{"x":"2","y":"6"}]}),
        };
        for n in 20..=63 {
            for solver in ["f4", "f5"] {
                let mut c = config();
                c.n = n;
                c.solver = solver.into();
                assert_eq!(
                    koblitz_worker_profile(&c, &case),
                    (23..=31).contains(&n) || (47..=61).contains(&n)
                );
            }
        }
        for family in ["binary", "prime"] {
            let mut c = config();
            c.family = family.into();
            assert!(!koblitz_worker_profile(&c, &case));
        }
        for m in 2..=15 {
            let mut c = config();
            c.m = m;
            assert_eq!(koblitz_worker_profile(&c, &case), m == 3);
        }
        for degree in 0..=7 {
            let mut c = config();
            c.degree_bound = degree;
            assert_eq!(koblitz_worker_profile(&c, &case), (1..=6).contains(&degree));
        }
    }
    #[test]
    fn dynamic_metadata_preserves_its_separate_solver_and_runtime_gates() {
        let p = entry(&capabilities(), DYNAMIC_PROFILE);
        assert_eq!(p["families"], json!(["prime"]));
        assert_eq!(p["n"], json!([33, 61]));
        assert_eq!(p["m"], json!([4, 15]));
        assert_eq!(p["degree_bound"], 12);
        assert_eq!(p["minimum_signed_domain_points"], 1);
        assert_eq!(p["maximum_signed_domain_points"], 64);
        assert_eq!(p["minimum_x_codes"], 1);
        assert_eq!(p["maximum_x_codes"], 32);
        for key in [
            "requires_original_execution_context",
            "requires_bound_capture",
            "requires_positive_work_limit",
            "requires_positive_original_memory_trace",
            "requires_whole_mib_trace_limit",
        ] {
            assert_eq!(p[key], true, "{key}");
        }
        for family in ["prime", "binary", "koblitz"] {
            for n in [23, 32, 33, 47, 61, 62] {
                for m in [2, 3, 4, 15] {
                    let mut c = config();
                    c.family = family.into();
                    c.n = n;
                    c.m = m;
                    c.degree_bound = 12;
                    c.solver = DYNAMIC_PROFILE.into();
                    assert_eq!(
                        dynamic_supported(&c),
                        family == "prime" && (33..=61).contains(&n) && (4..=15).contains(&m)
                    );
                }
            }
        }
        let mut c = config();
        c.family = "prime".into();
        c.n = 33;
        c.m = 4;
        c.solver = DYNAMIC_PROFILE.into();
        c.degree_bound = 12;
        c.work_limit = 0;
        assert!(!dynamic_supported(&c));
        c.work_limit = 1;
        c.degree_bound = 11;
        assert!(!dynamic_supported(&c));
    }
}
