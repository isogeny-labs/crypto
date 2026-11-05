//! The session runner.
//!
//! ```text
//! spec.json ──ecbench run──► <out>/
//!                              spec.json      byte copy of the spec
//!                              plan.json      arms, workloads, every execution in order
//!                              host.json      the host capsule
//!                              session.json   identity, reservation, preflight, hashes, status
//!                              records.jsonl  one sealed record per execution, in order
//!                              exec/<seq>.stderr   a child's stderr, when it wrote any
//! ```
//!
//! Each execution is a fresh process (`ecbench exec`), so one-time
//! costs — page faults, lazy statics, a cold allocator — land on every
//! execution alike instead of on whichever arm ran first.  The child is
//! pinned and its memory bound between fork and exec, with a pinned
//! environment; the runner waits for it blocked (no polling on any CPU),
//! a watchdog thread kills its process group at the timeout, and the
//! runner verifies the answer in its own process.  A record is written
//! and flushed for every execution, warm-ups, errors and timeouts
//! included, before the next one starts.
//!
//! The output directory is claimed with an atomic `mkdir` while the lock
//! is held, so two sessions queued for one directory cannot both write
//! it.  An interruption (`signals`) or an error leaves the session marked
//! `interrupted`, every evicted thread restored, and no child running.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::canonical::{sha256_hex, short_id};
use crate::cryptanalysis::ecbench::fbm::{self, INSTRUMENT_IC_PIPELINE};
use crate::cryptanalysis::ecbench::host::{capture, HostCapsule};
use crate::cryptanalysis::ecbench::isolation::{
    allowed_cpus, conditions, conditions_scanned, evict, leave_reserved, plan_cpus, preflight,
    ticks_per_second, topology, BenchLock, CpuPlan, CpuRequest, Eviction, EvictionSummary,
    Preflight, Thresholds,
};
use crate::cryptanalysis::ecbench::methods::MethodSpec;
use crate::cryptanalysis::ecbench::record::{
    floor_s, from_str_legacy_floats, grade, Boundaries, ChildInput, ChildOutput, Cost, GradeInput,
    IsolationRecord, Outcome, Record, Timing, CHILD_INPUT_SCHEMA, GRADING_VERSION, RECORD_SCHEMA,
    UNIT,
};
use crate::cryptanalysis::ecbench::signals;
use crate::cryptanalysis::ecbench::spec::{plan, Execution, Plan, Spec};
use crate::cryptanalysis::ecbench::workload::{Workload, FLOOR_RULE_V2};

pub const SESSION_SCHEMA: &str = "ecbench.session/v1";

/// Where and how a session runs; recorded whole in `session.json`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunOptions {
    /// Explicit instrument provenance; absent preserves historical behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_launch: Option<super::diagnostic_launch::DiagnosticLaunch>,
    pub cpus: CpuRequest,
    pub lock_path: String,
    pub wait_for_lock: bool,
    /// Record a refused preflight and continue (every run then stays
    /// below L2), instead of refusing to start.
    pub allow_busy: bool,
    pub thresholds: Thresholds,
    /// The `ecbench` binary each measured child execs.  On Linux this is
    /// `/proc/self/exe`, so a rebuild during the session cannot swap the
    /// code under the hash recorded at its start.
    pub exe: PathBuf,
}

fn default_hz() -> u64 {
    100
}

/// Bind one scheduled child to the same explicit launch used by every arm.
/// This constructs metadata only; it neither starts a child nor admits a run.
pub(crate) fn prepare_lifecycle_child(
    launch: &super::diagnostic_launch::DiagnosticLaunch,
    session: &str,
    execution: &Execution,
    arm: &super::spec::Arm,
    workload: &Workload,
    output_root: &Path,
) -> Result<super::record::DiagnosticChild, String> {
    let doc = launch.document()?;
    let binding = doc
        .arms
        .iter()
        .find(|a| a.name == arm.name)
        .ok_or("scheduled arm absent from diagnostic launch")?;
    let method_hash = super::diagnostic_launch::resolved_method_binding(&arm.method)?;
    if binding.role != arm.role
        || binding.method != arm.method.id
        || binding.method_config_sha256 != method_hash
    {
        return Err("scheduled diagnostic arm/configuration mismatch".into());
    }
    let workload_value = serde_json::to_value(workload).map_err(|e| e.to_string())?;
    let workload_bytes = serde_json::to_vec(&workload_value).map_err(|e| e.to_string())?;
    let prefix = super::diagnostics::Context {
        schema: "ecbench.diagnostic_context/v1".into(),
        relative_leaf: format!("diagnostic-{:06}-attempt-{}", execution.seq, doc.attempt),
        identity: super::diagnostics::Identity {
            session: session.into(),
            execution: execution.seq,
            workload_sha256: sha256_hex(&workload_bytes),
            binary_sha256: doc.binary_sha256.clone(),
            protocol_sha256: doc.protocol_sha256.clone(),
            seed: execution.algorithm_seed,
            attempt: doc.attempt,
            source_sha256: doc.source_receipt.source_manifest_sha256.clone(),
            method_config_sha256: method_hash,
        },
    };
    let envelope = super::diagnostic_launch::ChildEnvelope {
        schema: super::diagnostic_launch::CHILD_SCHEMA.into(),
        launch_sha256: launch.raw_sha256().into(),
        scientific_run_id: doc.scientific_run_id.clone(),
        profile: doc.profile.clone(),
        source_receipt: doc.source_receipt.clone(),
        arm: binding.clone(),
        prefix,
    };
    envelope.validate(launch)?;
    Ok(super::record::DiagnosticChild {
        launch: launch.clone(),
        envelope,
        output_root: output_root.into(),
    })
}
// Diagnostic-only publication: exclusive files and checked local directory sync.
// The existing v1/session writer is unchanged; no durability is inferred from it.
fn diagnostic_write_new(path: &Path, bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > super::diagnostic_resources::MAX_JOURNAL_BYTES {
        return Err("diagnostic publication byte cap".into());
    }
    let tmp = path.with_extension("publication-tmp");
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&tmp).map_err(|e| e.to_string())?;
    file.write_all(bytes).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    std::fs::hard_link(&tmp, path).map_err(|e| e.to_string())?;
    std::fs::remove_file(tmp).map_err(|e| e.to_string())?;
    std::fs::File::open(path.parent().ok_or("publication parent absent")?)
        .and_then(|d| d.sync_all())
        .map_err(|e| e.to_string())?;
    Ok(sha256_hex(bytes))
}
fn diagnostic_json_bytes(value: &impl Serialize) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}
#[derive(Clone, Serialize)]
struct PreparationObservations {
    lock_wait_started_ns: Option<u64>,
    lock_acquired_ns: Option<u64>,
    execute_entry_ns: Option<u64>,
}
struct DiagnosticSessionClock {
    origin_ns: u64,
    boot_id: Option<String>,
    time_namespace: Option<String>,
    time_namespace_offsets: Option<String>,
    #[cfg(not(target_os = "linux"))]
    portable: Instant,
    previous: std::cell::Cell<u64>,
    preparation: PreparationObservations,
}
#[cfg(target_os = "linux")]
fn diagnostic_monotonic_ns() -> Result<u64, String> {
    let mut value = std::mem::MaybeUninit::<libc::timespec>::uninit();
    // SAFETY: clock_gettime initializes the timespec on success only.
    let rc = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, value.as_mut_ptr()) };
    if rc != 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    // SAFETY: the successful call above initialized both fields.
    let value = unsafe { value.assume_init() };
    let seconds = u64::try_from(value.tv_sec).map_err(|_| "negative monotonic seconds")?;
    let nanos = u64::try_from(value.tv_nsec).map_err(|_| "negative monotonic nanos")?;
    if nanos >= 1_000_000_000 {
        return Err("invalid monotonic nanos".into());
    }
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|n| n.checked_add(nanos))
        .ok_or("monotonic overflow".into())
}
impl DiagnosticSessionClock {
    fn begin() -> Result<Self, String> {
        #[cfg(not(target_os = "linux"))]
        let portable = Instant::now();
        #[cfg(target_os = "linux")]
        let (origin_ns, boot_id, time_namespace, time_namespace_offsets) = (
            diagnostic_monotonic_ns()?,
            Some(
                std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
                    .map_err(|e| e.to_string())?
                    .trim()
                    .to_string(),
            ),
            Some(
                std::fs::read_link("/proc/self/ns/time")
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .into_owned(),
            ),
            Some(std::fs::read_to_string("/proc/self/timens_offsets").map_err(|e| e.to_string())?),
        );
        #[cfg(not(target_os = "linux"))]
        let (origin_ns, boot_id, time_namespace, time_namespace_offsets) = (0, None, None, None);
        Ok(Self {
            origin_ns,
            boot_id,
            time_namespace,
            time_namespace_offsets,
            #[cfg(not(target_os = "linux"))]
            portable,
            previous: std::cell::Cell::new(0),
            preparation: PreparationObservations {
                lock_wait_started_ns: None,
                lock_acquired_ns: None,
                execute_entry_ns: None,
            },
        })
    }
    fn checked_offset(&self, raw_ns: u64) -> Result<u64, String> {
        let offset = raw_ns
            .checked_sub(self.origin_ns)
            .ok_or("diagnostic origin underflow")?;
        if offset < self.previous.get() {
            return Err("diagnostic clock moved backwards".into());
        }
        self.previous.set(offset);
        Ok(offset)
    }
    fn offset(&self) -> Result<u64, String> {
        #[cfg(target_os = "linux")]
        {
            let boot = std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
                .map_err(|e| e.to_string())?;
            let namespace = std::fs::read_link("/proc/self/ns/time")
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .into_owned();
            let offsets =
                std::fs::read_to_string("/proc/self/timens_offsets").map_err(|e| e.to_string())?;
            if self.boot_id.as_deref() != Some(boot.trim())
                || self.time_namespace.as_deref() != Some(namespace.as_str())
                || self.time_namespace_offsets.as_deref() != Some(offsets.as_str())
            {
                return Err(
                    "diagnostic boot/time namespace changed; restore alignment unsupported".into(),
                );
            }
            self.checked_offset(diagnostic_monotonic_ns()?)
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.checked_offset(
                u64::try_from(self.portable.elapsed().as_nanos())
                    .map_err(|_| "portable clock overflow")?,
            )
        }
    }
    fn metadata(&self) -> Value {
        json!({"schema":"ecbench.session_preparation/v1", "clock_type":if cfg!(target_os="linux") {"linux_CLOCK_MONOTONIC"} else {"portable_Instant_unalignable"},
            "origin_ns":self.origin_ns,"boot_id":self.boot_id,"time_namespace":self.time_namespace,"time_namespace_offsets":self.time_namespace_offsets,
            "origin_boundary":"run_session entry before parsing; excludes CLI/process startup and input acquisition",
            "observations":self.preparation,"preparation_wall_ns":self.preparation.execute_entry_ns,
            "lock_wait_wall_ns":self.preparation.lock_acquired_ns.zip(self.preparation.lock_wait_started_ns).and_then(|(end,start)|end.checked_sub(start)),
            "lock_wait_is_subset_of_preparation":true,"interval_kind":"wall_only", "active_cpu_ns":null,"active_worker_ns":null,
            "external_alignment_supported":cfg!(target_os="linux"),"external_alignment_conditions":"same verified boot/time namespace and raw CLOCK_MONOTONIC; authenticated owner receipt required",
            "restore_alignment_supported":false,"external_exit_observation":null,"final_publication_tail_pending":true})
    }
}
fn preparation_clock_id(runner_id: &str, preparation_sha256: &str) -> String {
    format!("{runner_id}-preparation-sha256-{preparation_sha256}")
}
fn session_offset(clock: &DiagnosticSessionClock) -> Result<u64, String> {
    clock.offset()
}
struct ResourceTracking {
    envelope: super::diagnostic_resources::ExecutionEnvelope,
    start: u64,
    bindings: super::diagnostic_resources::Bindings,
    stalled: Option<String>,
    marks: usize,
}
impl ResourceTracking {
    fn observe(
        &mut self,
        stage: super::diagnostic_resources::Stage,
        hash: Option<String>,
        clock: &DiagnosticSessionClock,
    ) {
        if self.stalled.is_some() {
            return;
        }
        let result = session_offset(clock)
            .and_then(|now| {
                now.checked_sub(self.start)
                    .ok_or("resource clock underflow".into())
            })
            .and_then(|offset| self.envelope.mark(stage, offset));
        if let Err(error) = result {
            self.stalled = Some(error);
            return;
        }
        self.marks += 1;
        use super::diagnostic_resources::Stage;
        match stage {
            Stage::RecordDurable => self.bindings.record_sha256 = hash,
            Stage::DiagnosticsDurable => self.bindings.diagnostics_sha256 = hash,
            Stage::SessionDurable => self.bindings.session_sha256 = hash,
            _ => {}
        }
    }
    fn publish(
        self,
        clock: &DiagnosticSessionClock,
        out: &Path,
        seq: u64,
        journal: &mut super::diagnostic_resources::SessionResourceJournal,
        result: &Result<(), String>,
    ) -> Result<(), String> {
        use super::diagnostic_resources::{PublicationTail, Status, MAX_PREFIX_BYTES};
        let status = match (result, &self.stalled) {
            (Err(error), _) => Status::Failed(error.clone()),
            (_, Some(error)) => Status::Failed(error.clone()),
            _ if self.marks != 5 => {
                Status::Failed("missing observed lifecycle resource stage".into())
            }
            _ => Status::Returned,
        };
        let covered = session_offset(clock)?
            .checked_sub(self.start)
            .ok_or("resource clock underflow")?;
        let prefix = self.envelope.seal_prefix(self.bindings, status, covered)?;
        let bytes = diagnostic_json_bytes(&prefix)?;
        if bytes.len() > MAX_PREFIX_BYTES {
            return Err("resource prefix byte cap".into());
        }
        let started = session_offset(clock)?;
        diagnostic_write_new(
            &out.join("exec").join(format!("{seq:06}.resources.json")),
            &bytes,
        )?;
        let durable = session_offset(clock)?;
        let tail = PublicationTail::observed(&prefix, &bytes, started, durable)?;
        journal.append_tail(tail.clone())?;
        // Publish each tail once. The session-wide journal is published once
        // after the loop, including ordinary error termination.
        diagnostic_write_new(
            &out.join("exec")
                .join(format!("{seq:06}.resource-tail.json")),
            &diagnostic_json_bytes(&tail)?,
        )?;
        // Journal publication has no self-certified exit observation. Its final
        // tail remains pending; no active-worker or payload total is manufactured.
        Ok(())
    }
}
// Exact compact serialized upper bound: actual escaped identity strings,
// u64::MAX in every numeric tail field, empty-journal header plus commas.
// This is admission for one session, not an implicit cohort partition.
pub(crate) fn resource_journal_capacity(
    journal: &super::diagnostic_resources::SessionResourceJournal,
    mut identity: super::diagnostic_resources::Identity,
    slots: usize,
) -> Result<usize, String> {
    identity.seq = u64::MAX;
    identity.attempt = u64::MAX;
    let tail = super::diagnostic_resources::PublicationTail {
        prefix_sha256: "f".repeat(64),
        identity,
        execution_start_ns: u64::MAX,
        prefix_covered_through_ns: u64::MAX,
        receipt_write_started_ns: u64::MAX,
        receipt_write_durable_ns: u64::MAX,
    };
    let header = serde_json::to_vec(journal)
        .map_err(|e| e.to_string())?
        .len();
    let width = serde_json::to_vec(&tail).map_err(|e| e.to_string())?.len();
    let bound = width
        .checked_mul(slots)
        .and_then(|n| n.checked_add(slots.saturating_sub(1)))
        .and_then(|n| n.checked_add(header))
        .ok_or("resource journal capacity overflow")?;
    if bound > super::diagnostic_resources::MAX_JOURNAL_BYTES {
        return Err(format!("resource journal admission: {slots} slots need at most {bound} bytes, exceeding cap {}; prospective session partition required before execution", super::diagnostic_resources::MAX_JOURNAL_BYTES));
    }
    Ok(bound)
}

fn ingest_lifecycle_v2(input: &ChildInput, line: &[u8]) -> Result<Value, String> {
    let child = input
        .diagnostic_lifecycle
        .as_ref()
        .ok_or("v2 context absent")?;
    child.validate(input)?;
    super::canonical::reject_duplicate_keys(std::str::from_utf8(line).map_err(|e| e.to_string())?)?;
    let record: Value = serde_json::from_slice(line).map_err(|e| e.to_string())?;
    let identity = &child.envelope.prefix.identity;
    let seed = identity.seed.to_string();
    if record["session_id"] != identity.session
        || record["seq"].as_u64() != Some(identity.execution)
        || record["algorithm_seed"].as_str() != Some(seed.as_str())
        || record["binary_sha256"] != identity.binary_sha256
        || sha256_hex(&serde_json::to_vec(&record["workload"]).map_err(|e| e.to_string())?)
            != identity.workload_sha256
        || sha256_hex(&serde_json::to_vec(&record["method"]).map_err(|e| e.to_string())?)
            != identity.method_config_sha256
    {
        return Err("v2 diagnostic sealed-record association mismatch".into());
    }
    let lifecycle = super::diagnostic_runtime::read_lifecycle(
        &child.lifecycle_directory(),
        &child.envelope.prefix.identity,
    );
    let candidate = child.envelope.arm.method == "ic.three_leads_genus2_pencil_symbolic_v2";
    let prefix = if candidate {
        Some(super::diagnostics::ingest(
            &child.output_root,
            &child.envelope.prefix,
            line,
        ))
    } else {
        None
    };
    let observed = |result: Result<Value, String>| match result {
        Ok(value) => json!({"observation":value,"error":null}),
        Err(error) => json!({"observation":null,"error":error}),
    };
    Ok(
        json!({"schema":"ecbench.diagnostic_attachment/v2","record_sha256":sha256_hex(line),
        "context":child,"context_sha256":sha256_hex(&diagnostic_json_bytes(child)?),
        "lifecycle":observed(lifecycle.and_then(|v|serde_json::to_value(v).map_err(|e|e.to_string()))),
        "prefix":prefix.map(|v|observed(v.and_then(|p|serde_json::to_value(p).map_err(|e|e.to_string())))),
        "prefix_applicable":candidate,"accepted_result":null,"accepted_cost":null,
        "scope":"diagnostic custody only; missing or invalid sidecars remain explicit"}),
    )
}
fn grading_v1() -> u32 {
    1
}

/// The session document.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub schema: String,
    pub session_id: String,
    pub label: String,
    pub spec_id: String,
    pub spec_sha256: String,
    pub env_class_id: String,
    pub host_sha256: String,
    pub plan_sha256: String,
    pub binary_sha256: Option<String>,
    pub git_commit: Option<String>,
    pub git_dirty: Option<bool>,
    pub options: RunOptions,
    pub cpu_plan: Option<CpuPlan>,
    pub preflight: Option<Preflight>,
    pub eviction: Option<EvictionSummary>,
    /// The runner shared the run CPU (it had nowhere else to go).
    #[serde(default)]
    pub runner_on_run_cpu: bool,
    /// The CPUs the runner moved itself to.
    #[serde(default)]
    pub runner_cpus: Option<Vec<u32>>,
    /// `_SC_CLK_TCK` on the host, which grading divides ticks by.
    #[serde(default = "default_hz")]
    pub ticks_per_second: u64,
    /// The rules the records' levels were graded under; an audit regrades
    /// a session only under the rules it was graded with.
    #[serde(default = "grading_v1")]
    pub grading_version: u32,
    /// The versions of the counted-work definitions the session was made
    /// under (`record::current_accounting_versions`).  Empty on a session
    /// written before they were recorded, and then not written at all, so
    /// such a session reads and re-serialises byte for byte as before.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub accounting_versions: BTreeMap<String, u32>,
    /// SHA-256 of the sorted `k=v` lines of the child environment.
    pub child_env_sha256: String,
    pub child_env: BTreeMap<String, String>,
    pub started_unix_ms: u128,
    pub finished_unix_ms: Option<u128>,
    /// `running`, `complete` or `interrupted`.
    pub status: String,
    pub executions_planned: u64,
    pub records_written: u64,
    pub records_sha256: Option<String>,
    pub status_counts: BTreeMap<String, u64>,
    /// The sequence number of the last record written (rewritten after
    /// every record, so a session killed outright still says how far it got).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seq: Option<u64>,
    /// Why and where the session stopped, when it did not run to the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<SessionStop>,
}

/// Why a session ended early.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStop {
    /// `interrupt` (a signal to the runner), `spawn_error` (a child could
    /// not be started or pinned) or `error`.
    pub kind: String,
    /// The execution the session was at: for an interruption or a spawn
    /// error, the one that did not complete; otherwise the last written.
    pub seq: Option<u64>,
    /// The signal, for an interruption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    pub message: String,
}

/// The plan as written to `plan.json`.
#[derive(Serialize, Deserialize)]
pub struct PlanDoc {
    pub schema: String,
    pub spec_id: String,
    pub arms: Vec<crate::cryptanalysis::ecbench::spec::Arm>,
    pub workloads: Vec<Workload>,
    pub executions: Vec<Execution>,
}

fn now_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

/// The child's environment: nothing inherited but `PATH` and `HOME`,
/// thread counts and locale pinned.
fn child_env() -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    for k in ["PATH", "HOME"] {
        if let Ok(v) = std::env::var(k) {
            env.insert(k.to_string(), v);
        }
    }
    for (k, v) in [
        ("LC_ALL", "C"),
        ("RAYON_NUM_THREADS", "1"),
        ("OMP_NUM_THREADS", "1"),
        ("RUST_BACKTRACE", "0"),
        ("RUST_MIN_STACK", "8388608"),
    ] {
        env.insert(k.to_string(), v.to_string());
    }
    env
}

fn write_json(path: &Path, v: &impl Serialize) -> Result<String, String> {
    let text = serde_json::to_string_pretty(v).map_err(|e| e.to_string())? + "\n";
    write_atomic(path, text.as_bytes())?;
    Ok(sha256_hex(text.as_bytes()))
}

/// Write `bytes` to `path` so that a reader (or a crash) sees the old
/// file or the new one, never a torn mix: write a sibling temporary
/// file, fsync it, rename it over `path`, fsync the directory.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let tmp = path.with_file_name(format!(".{name}.tmp"));
    let fail = |e: std::io::Error| format!("{}: {e}", path.display());
    {
        let mut f = std::fs::File::create(&tmp).map_err(fail)?;
        f.write_all(bytes).map_err(fail)?;
        f.sync_all().map_err(fail)?;
    }
    std::fs::rename(&tmp, path).map_err(fail)?;
    // Best effort: make the rename itself durable.
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
    }
    Ok(())
}

/// What the runner observed of one finished child.
struct Reaped {
    output: Option<ChildOutput>,
    parse_error: Option<String>,
    stderr: Vec<u8>,
    timing: Timing,
    exit: Option<String>,
    /// True only after wait4 returned the child pid or child.wait succeeded.
    reap_observed: bool,
    /// The watchdog's SIGKILL ended the child at the deadline.
    timed_out: bool,
    /// The session's interruption (a signal to the runner) killed the child.
    interrupted: bool,
    /// The child could not be started: spawn, pinning or feeding failed.
    spawn_error: Option<String>,
}

impl Reaped {
    fn spawn_failed(message: String) -> Self {
        Reaped {
            output: None,
            parse_error: None,
            stderr: Vec::new(),
            timing: Timing::default(),
            exit: None,
            reap_observed: false,
            timed_out: false,
            interrupted: false,
            spawn_error: Some(message),
        }
    }
}

/// A returned Result alone never establishes child quiescence. Both resource
/// marking and sidecar reading share this observation gate.
struct ReapObservation(bool);
impl ReapObservation {
    fn from_result(result: &Result<Reaped, String>) -> Self {
        Self(result.as_ref().is_ok_and(|reaped| reaped.reap_observed))
    }
    fn when_observed<T>(&self, callback: impl FnOnce() -> T) -> Option<T> {
        self.0.then(callback)
    }
    fn mark_resources(&self, resource: &mut ResourceTracking, clock: &DiagnosticSessionClock) {
        if self
            .when_observed(|| {
                resource.observe(super::diagnostic_resources::Stage::ChildReaped, None, clock)
            })
            .is_none()
        {
            resource.stalled = Some(
                "child spawn/reap observation incomplete; later contiguous marks omitted".into(),
            );
        }
    }
}

/// Why a child that died of SIGKILL died: `(timed_out, interrupted)`.
///
/// Only a SIGKILL ends a child on the runner's behalf, and only the
/// watchdog (the deadline) or the interruption handler sends it.  A child
/// that exited by itself, even in the instant the watchdog fired, was not
/// killed and is not a timeout: its answer stands.  An interruption takes
/// precedence, since it ends the session whatever the deadline said.
fn classify_stop(
    watchdog_fired: bool,
    interruption: bool,
    killed_by_sigkill: bool,
) -> (bool, bool) {
    if !killed_by_sigkill {
        return (false, false);
    }
    if interruption {
        (false, true)
    } else {
        (watchdog_fired, false)
    }
}

/// Spawn, pin, feed, wait, reap.
fn run_child(
    opts: &RunOptions,
    plan: Option<&CpuPlan>,
    env: &BTreeMap<String, String>,
    input: &ChildInput,
    timeout: Duration,
) -> Result<Reaped, String> {
    run_child_observed(opts, plan, env, input, timeout, None)
}

fn run_child_observed(
    opts: &RunOptions,
    plan: Option<&CpuPlan>,
    env: &BTreeMap<String, String>,
    input: &ChildInput,
    timeout: Duration,
    mut spawned: Option<&mut dyn FnMut()>,
) -> Result<Reaped, String> {
    let mut cmd = Command::new(&opts.exe);
    cmd.arg("exec")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_clear()
        .envs(env);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // `ps` shows `ecbench exec`, not `/proc/self/exe exec`.
        cmd.arg0("ecbench");
        let pin = plan.map(|p| (p.run_cpu, p.node));
        let multi_node = topology().nodes.len() > 1;
        let parent = std::process::id();
        // SAFETY: only async-signal-safe system calls between fork and exec.
        unsafe {
            cmd.pre_exec(move || {
                if libc::setpgid(0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                signals::prepare_child(parent)?;
                #[cfg(target_os = "linux")]
                if let Some((cpu, node)) = pin {
                    crate::cryptanalysis::ecbench::isolation::pin_in_child(
                        cpu,
                        if multi_node { node } else { None },
                    )?;
                }
                #[cfg(not(target_os = "linux"))]
                let _ = (pin, multi_node);
                Ok(())
            });
        }
    }
    let payload = serde_json::to_vec(input).map_err(|e| e.to_string())?;
    let started = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("cannot exec {}: {e}", opts.exe.display()))?;
    if let Some(observe) = spawned.as_mut() {
        observe();
    }
    let pid = child.id() as i32;
    signals::set_child(pid);
    {
        let mut stdin = child.stdin.take().ok_or("no child stdin")?;
        let _ = stdin.write_all(&payload);
    }
    let mut stdout = child.stdout.take().ok_or("no child stdout")?;
    let mut stderr = child.stderr.take().ok_or("no child stderr")?;
    let out_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout.read_to_end(&mut buf);
        buf
    });
    let err_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr.read_to_end(&mut buf);
        buf
    });
    // The watchdog sleeps until the deadline or until told the child is
    // done; it costs no CPU while the child runs.
    let done = Arc::new((Mutex::new(false), Condvar::new()));
    let fired = Arc::new(Mutex::new(false));
    let watchdog = {
        let done = Arc::clone(&done);
        let fired = Arc::clone(&fired);
        std::thread::spawn(move || {
            let (lock, cv) = &*done;
            let guard = lock.lock().unwrap_or_else(|e| e.into_inner());
            let (guard, _) = cv
                .wait_timeout_while(guard, timeout, |finished| !*finished)
                .unwrap_or_else(|e| e.into_inner());
            if !*guard {
                *fired.lock().unwrap_or_else(|e| e.into_inner()) = true;
                #[cfg(unix)]
                // SAFETY: signalling the child's own process group.
                unsafe {
                    libc::kill(-pid, libc::SIGKILL);
                }
            }
        })
    };
    let mut timing = Timing::default();
    let mut exit = None;
    let mut reap_observed = false;
    let mut killed_by_sigkill = false;
    #[cfg(unix)]
    {
        // Wait until the child has ended without reaping it: while it is
        // a zombie its pid cannot be recycled, so a watchdog signal that
        // lands now is harmless, and it cannot land on a stranger's group
        // after the reap.
        #[cfg(target_os = "linux")]
        // SAFETY: waiting for our own child; `info` is filled.
        unsafe {
            loop {
                let mut info: libc::siginfo_t = std::mem::zeroed();
                let rc = libc::waitid(
                    libc::P_PID,
                    pid as libc::id_t,
                    &mut info,
                    libc::WEXITED | libc::WNOWAIT,
                );
                if rc == -1
                    && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
                {
                    continue;
                }
                break;
            }
        }
        #[cfg(target_os = "linux")]
        {
            signals::clear_child();
            let (lock, cv) = &*done;
            *lock.lock().unwrap_or_else(|e| e.into_inner()) = true;
            cv.notify_all();
        }
        // SAFETY: waiting for our own child; status and rusage are filled.
        unsafe {
            let mut status: libc::c_int = 0;
            let mut ru: libc::rusage = std::mem::zeroed();
            let rc = loop {
                let rc = libc::wait4(pid, &mut status, 0, &mut ru);
                if rc == -1
                    && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted
                {
                    continue;
                }
                break rc;
            };
            timing.process_wall_ns = started.elapsed().as_nanos() as u64;
            if rc == pid {
                reap_observed = true;
                let tv =
                    |t: libc::timeval| t.tv_sec as u64 * 1_000_000_000 + t.tv_usec as u64 * 1000;
                timing.user_ns = tv(ru.ru_utime);
                timing.sys_ns = tv(ru.ru_stime);
                // Linux reports kibibytes, macOS bytes.
                timing.max_rss_kib = if cfg!(target_os = "macos") {
                    ru.ru_maxrss as u64 / 1024
                } else {
                    ru.ru_maxrss as u64
                };
                timing.minor_faults = ru.ru_minflt as u64;
                timing.major_faults = ru.ru_majflt as u64;
                timing.voluntary_switches = ru.ru_nvcsw as u64;
                timing.involuntary_switches = ru.ru_nivcsw as u64;
                killed_by_sigkill =
                    libc::WIFSIGNALED(status) && libc::WTERMSIG(status) == libc::SIGKILL;
                exit = Some(if libc::WIFEXITED(status) {
                    format!("exit {}", libc::WEXITSTATUS(status))
                } else if libc::WIFSIGNALED(status) {
                    format!("signal {}", libc::WTERMSIG(status))
                } else {
                    format!("status {status}")
                });
            }
        }
    }
    #[cfg(not(unix))]
    {
        let st = child.wait().map_err(|e| e.to_string())?;
        reap_observed = true;
        timing.process_wall_ns = started.elapsed().as_nanos() as u64;
        exit = Some(format!("{st}"));
    }
    signals::clear_child();
    {
        let (lock, cv) = &*done;
        *lock.lock().unwrap_or_else(|e| e.into_inner()) = true;
        cv.notify_all();
    }
    let _ = watchdog.join();
    // The child is reaped; std must not wait for it again.
    std::mem::forget(child);
    let out = out_reader.join().unwrap_or_default();
    let err = err_reader.join().unwrap_or_default();
    let watchdog_fired = *fired.lock().unwrap_or_else(|e| e.into_inner());
    let (timed_out, interrupted) = classify_stop(
        watchdog_fired,
        signals::interrupted().is_some(),
        killed_by_sigkill,
    );
    let (output, parse_error) = match serde_json::from_slice::<ChildOutput>(&out) {
        Ok(o) => (Some(o), None),
        Err(e) if !timed_out && !interrupted => (None, Some(format!("child output: {e}"))),
        Err(_) => (None, None),
    };
    if let Some(o) = output.as_ref().filter(|_| !timed_out && !interrupted) {
        timing.schedstat_solve = o.schedstat_solve;
        timing.hw_solve = o.hw_solve.clone();
        timing.solve = o.solve_obs.clone();
        timing.solve_wall_ns = o.report.as_ref().map(|r| r.solve_wall_ns);
        timing.method_call_wall_ns = o.method_call_wall_ns;
    }
    Ok(Reaped {
        output,
        parse_error,
        stderr: err,
        timing,
        exit,
        reap_observed,
        timed_out,
        interrupted,
        spawn_error: None,
    })
}

fn interrupted_error() -> Option<String> {
    signals::interrupted().map(|sig| format!("interrupted by signal {sig}"))
}

/// Size and modification time of the binary: a cheap check, where
/// `/proc/self/exe` is not available, that it was not rebuilt mid-session.
fn exe_fingerprint(exe: &Path) -> Option<(u64, std::time::SystemTime)> {
    let m = std::fs::metadata(exe).ok()?;
    Some((m.len(), m.modified().ok()?))
}

fn validate_diagnostic_launch_before_plan(
    launch: &super::diagnostic_launch::DiagnosticLaunch,
    spec: &Spec,
    executable: &Path,
) -> Result<(), String> {
    let (_, spec_digest) = short_id("ECS1", &spec.identity_view())?;
    // Hash actual configured bytes; a declaration alone cannot activate this
    // path. The child checks its running inode and execute_session rechecks the
    // captured parent build plus the full resolved plan before spawning.
    let (binary_sha256, _) = crate::store::hash_file(executable)
        .map_err(|error| format!("diagnostic executable identity: {error}"))?;
    launch.validate_spec(spec, &spec_digest, Some(&binary_sha256))
}

/// Run a whole session.  Refuses an existing output directory; refuses
/// to start on a busy host unless `allow_busy`.
pub fn run_session(
    spec_text: &str,
    out: &Path,
    opts: RunOptions,
    mut progress: impl FnMut(&str),
) -> Result<Session, String> {
    let mut diagnostic_clock = opts
        .diagnostic_launch
        .as_ref()
        .map(|_| DiagnosticSessionClock::begin())
        .transpose()?;
    signals::install();
    // A fast refusal; the authoritative one is the atomic mkdir below.
    if out.exists() {
        return Err(format!(
            "{} exists; a session never overwrites another",
            out.display()
        ));
    }
    // A spec that does not parse is refused before waiting for anything.
    let spec = Spec::from_json(spec_text)?;
    if let Some(launch) = &opts.diagnostic_launch {
        // Validate before planning can construct even a known-answer workload.
        validate_diagnostic_launch_before_plan(launch, &spec, &opts.exe)?;
    }
    // The lock comes first.  Planning (a prime search can burn minutes of
    // CPU), the host capture and the choice of CPUs all disturb whatever
    // timed session holds the lock, and a CPU choice made while queued
    // would be made from a mask that session had narrowed.
    if let Some(clock) = diagnostic_clock.as_mut() {
        clock.preparation.lock_wait_started_ns = Some(clock.offset()?);
    }
    let lock = BenchLock::acquire(&opts.lock_path, opts.wait_for_lock)?;
    if let Some(clock) = diagnostic_clock.as_mut() {
        clock.preparation.lock_acquired_ns = Some(clock.offset()?);
    }
    let plan_: Plan = plan(spec)?;
    for w in &plan_.warnings {
        progress(&format!("warning: {w}"));
    }
    let capsule: HostCapsule = capture()?;
    let topo = capsule.stable.topology.clone();
    let allowed = allowed_cpus();
    let cpu_plan = plan_cpus(&opts.cpus, &topo, allowed.as_deref())?;

    // Check and preflight before anything is written.
    let mut runner_on_run_cpu = false;
    let mut runner_cpus = None;
    // Restores on drop: a failure or panic from here on puts the host back.
    let mut eviction: Option<Eviction> = None;
    let mut pre = None;
    if let Some(p) = &cpu_plan {
        // The runner leaves the reservation.  Under an inherited
        // placement (an isolab cgroup) there may be nothing outside it;
        // the runner then takes an idle sibling, where it sleeps while
        // the child runs, and only as a last resort the run CPU itself.
        match leave_reserved(&p.reserved).or_else(|_| leave_reserved(&[p.run_cpu])) {
            Ok(cpus) => runner_cpus = Some(cpus),
            Err(_) => runner_on_run_cpu = true,
        }
        eviction = Some(evict(&p.reserved));
        let pf = preflight(p, &opts.thresholds);
        if let Some(e) = interrupted_error() {
            return Err(e);
        }
        if !pf.quiet && !opts.allow_busy {
            return Err(format!(
                "host is not quiet ({}); wait, or pass --allow-busy to record runs below L2",
                pf.reasons.join("; ")
            ));
        }
        pre = Some(pf);
    }
    // Claim the directory atomically, under the lock: a session queued
    // behind another for the same directory fails here instead of
    // writing over it.
    if let Some(parent) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    std::fs::create_dir(out).map_err(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            format!(
                "{} exists; a session never overwrites another",
                out.display()
            )
        } else {
            format!("{}: {e}", out.display())
        }
    })?;
    let result = execute_session(
        out,
        &plan_,
        &capsule,
        cpu_plan.clone(),
        pre,
        eviction.as_ref().map(|e| e.summary.clone()),
        runner_on_run_cpu,
        runner_cpus,
        spec_text,
        opts,
        &mut progress,
        diagnostic_clock.as_mut(),
    );
    // Put the host back before the session is declared finished either
    // way; the lock is released last.
    drop(eviction);
    let result = match result {
        Ok(mut session) => {
            session.status = "complete".into();
            session.finished_unix_ms = Some(now_ms());
            write_json(&out.join("session.json"), &session)?;
            Ok(session)
        }
        Err(e) => {
            // Mark what was written as interrupted; never delete it.
            if let Ok(text) = std::fs::read_to_string(out.join("session.json")) {
                if let Ok(mut s) = serde_json::from_str::<Session>(&text) {
                    s.status = "interrupted".into();
                    s.finished_unix_ms = Some(now_ms());
                    let _ = write_json(&out.join("session.json"), &s);
                }
            }
            Err(e)
        }
    };
    drop(lock);
    result
}

#[allow(clippy::too_many_arguments)]
fn execute_session(
    out: &Path,
    p: &Plan,
    capsule: &HostCapsule,
    cpu_plan: Option<CpuPlan>,
    pre: Option<Preflight>,
    eviction: Option<EvictionSummary>,
    runner_on_run_cpu: bool,
    runner_cpus: Option<Vec<u32>>,
    spec_text: &str,
    opts: RunOptions,
    progress: &mut impl FnMut(&str),
    diagnostic_clock: Option<&mut DiagnosticSessionClock>,
) -> Result<Session, String> {
    if opts.diagnostic_launch.is_some() != diagnostic_clock.is_some() {
        return Err("diagnostic launch/clock presence mismatch".into());
    }
    if let Some(clock) = diagnostic_clock.as_deref() {
        if opts.diagnostic_launch.is_none() {
            return Err("diagnostic clock without launch".into());
        }
        let _ = clock.offset()?;
    }
    let resource_clock = diagnostic_clock
        .map(|clock| -> Result<&DiagnosticSessionClock, String> {
            clock.preparation.execute_entry_ns = Some(clock.offset()?);
            Ok(clock)
        })
        .transpose()?;
    let mut exec_directory = std::fs::DirBuilder::new();
    #[cfg(unix)]
    if opts.diagnostic_launch.is_some() {
        use std::os::unix::fs::DirBuilderExt;
        exec_directory.mode(0o700);
    }
    exec_directory
        .create(out.join("exec"))
        .map_err(|e| format!("{}: {e}", out.display()))?;
    let diagnostic_root = if let Some(launch) = &opts.diagnostic_launch {
        launch.validate_spec(
            &p.spec,
            &p.spec_sha256,
            capsule.build.binary_sha256.as_deref(),
        )?;
        launch.validate_plan(p)?;
        let root = out.join("diagnostic-output");
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&root).map_err(|e| e.to_string())?;
        Some(std::fs::canonicalize(root).map_err(|e| e.to_string())?)
    } else {
        None
    };
    std::fs::write(out.join("spec.json"), spec_text).map_err(|e| e.to_string())?;
    let host_sha256 = write_json(&out.join("host.json"), capsule)?;
    let plan_doc = PlanDoc {
        schema: "ecbench.plan/v1".into(),
        spec_id: p.spec_id.clone(),
        arms: p.arms.clone(),
        workloads: p.workloads.clone(),
        executions: p.executions.clone(),
    };
    let plan_sha256 = write_json(&out.join("plan.json"), &plan_doc)?;
    let env = child_env();
    let env_lines: Vec<String> = env.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let started = now_ms();
    let (session_id, _) = short_id(
        "ECBS1",
        &json!({
            "spec_sha256": p.spec_sha256,
            "host_sha256": host_sha256,
            "plan_sha256": plan_sha256,
            "started_unix_ms": started.to_string(),
        }),
    )?;
    let hz = ticks_per_second();
    let resource_runner_id = format!("{session_id}-runner-{}", std::process::id());
    let resource_clock_id = if let Some(clock) = resource_clock {
        let mut metadata = clock.metadata();
        metadata["session_id"] = json!(session_id);
        metadata["runner_id"] = json!(resource_runner_id);
        metadata["clock_identity_rule"] = json!("runner_id + '-preparation-sha256-' + lowercase SHA256 of exact session-preparation.json bytes; file deliberately omits clock_id to avoid a cycle");
        metadata["launch_sha256"] = json!(opts
            .diagnostic_launch
            .as_ref()
            .ok_or("launch absent")?
            .raw_sha256());
        metadata["spec_sha256"] = json!(p.spec_sha256);
        metadata["plan_sha256"] = json!(plan_sha256);
        metadata["host_sha256"] = json!(host_sha256);
        let preparation_sha256 = diagnostic_write_new(
            &out.join("session-preparation.json"),
            &diagnostic_json_bytes(&metadata)?,
        )?;
        preparation_clock_id(&resource_runner_id, &preparation_sha256)
    } else {
        format!("{resource_runner_id}-monotonic-v1")
    };

    let mut resource_journal = opts
        .diagnostic_launch
        .as_ref()
        .map(|_| {
            super::diagnostic_resources::SessionResourceJournal::begin(
                session_id.clone(),
                resource_runner_id.clone(),
                resource_clock_id.clone(),
                0,
            )
        })
        .transpose()?;
    if let (Some(launch), Some(journal)) = (&opts.diagnostic_launch, &resource_journal) {
        let doc = launch.document()?;
        resource_journal_capacity(
            journal,
            super::diagnostic_resources::Identity {
                session_id: session_id.clone(),
                clock_id: resource_clock_id.clone(),
                runner_id: resource_runner_id.clone(),
                scientific_run_id: doc.scientific_run_id.clone(),
                attempt: doc.attempt,
                seq: 0,
                protocol_sha256: doc.protocol_sha256.clone(),
                source_manifest_sha256: doc.source_receipt.source_manifest_sha256.clone(),
                binary_sha256: doc.binary_sha256.clone(),
            },
            p.executions.len(),
        )?;
    }

    let mut session = Session {
        schema: SESSION_SCHEMA.into(),
        session_id: session_id.clone(),
        label: p.spec.label.clone(),
        spec_id: p.spec_id.clone(),
        spec_sha256: p.spec_sha256.clone(),
        env_class_id: capsule.env_class_id.clone(),
        host_sha256,
        plan_sha256,
        binary_sha256: capsule.build.binary_sha256.clone(),
        git_commit: capsule.build.git_commit.clone(),
        git_dirty: capsule.build.git_dirty,
        options: opts.clone(),
        cpu_plan: cpu_plan.clone(),
        preflight: pre.clone(),
        eviction: eviction.clone(),
        runner_on_run_cpu,
        runner_cpus,
        ticks_per_second: hz,
        grading_version: GRADING_VERSION,
        accounting_versions: crate::cryptanalysis::ecbench::record::current_accounting_versions(),
        child_env_sha256: sha256_hex(env_lines.join("\n").as_bytes()),
        child_env: env.clone(),
        started_unix_ms: started,
        finished_unix_ms: None,
        status: "running".into(),
        executions_planned: p.executions.len() as u64,
        records_written: 0,
        records_sha256: None,
        status_counts: BTreeMap::new(),
        last_seq: None,
        stop: None,
    };
    write_json(&out.join("session.json"), &session)?;

    let mut records = std::fs::OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(out.join("records.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut reps: BTreeMap<(String, String), u32> = BTreeMap::new();
    let timeout = Duration::from_secs(p.spec.measurement.timeout_seconds.max(1));
    let observed = cpu_plan
        .as_ref()
        .map(|c| c.observed_cpus())
        .unwrap_or_default();
    let exe_at_start = exe_fingerprint(&opts.exe);
    let reserved: Vec<u32> = cpu_plan
        .as_ref()
        .map(|c| c.reserved.clone())
        .unwrap_or_default();
    // Conditions around a run, with a scan of the reservation for threads
    // that could run on it when there is one.
    let snapshot = || {
        if cpu_plan.is_some() {
            conditions_scanned(&observed, &reserved)
        } else {
            conditions(&observed)
        }
    };
    let mut stop: Option<SessionStop> = None;
    let result = (|| -> Result<(), String> {
        for ex in &p.executions {
            if let Some(sig) = signals::interrupted() {
                let message = format!("interrupted by signal {sig}");
                stop = Some(SessionStop {
                    kind: "interrupt".into(),
                    seq: Some(ex.seq),
                    signal: Some(sig),
                    message: message.clone(),
                });
                return Err(message);
            }
            if !opts.exe.starts_with("/proc") && exe_fingerprint(&opts.exe) != exe_at_start {
                let message = format!(
                    "{} changed during the session; its records would not match the recorded binary hash",
                    opts.exe.display()
                );
                stop = Some(SessionStop {
                    kind: "error".into(),
                    seq: Some(ex.seq),
                    signal: None,
                    message: message.clone(),
                });
                return Err(message);
            }
            let mut resources = opts
                .diagnostic_launch
                .as_ref()
                .map(|launch| {
                    let doc = launch.document()?;
                    let start = session_offset(resource_clock.ok_or("diagnostic clock absent")?)?;
                    Ok::<_, String>(ResourceTracking {
                        envelope: super::diagnostic_resources::ExecutionEnvelope::begin(
                            super::diagnostic_resources::Identity {
                                session_id: session_id.clone(),
                                clock_id: resource_clock_id.clone(),
                                runner_id: resource_runner_id.clone(),
                                scientific_run_id: doc.scientific_run_id.clone(),
                                attempt: doc.attempt,
                                seq: ex.seq,
                                protocol_sha256: doc.protocol_sha256.clone(),
                                source_manifest_sha256: doc
                                    .source_receipt
                                    .source_manifest_sha256
                                    .clone(),
                                binary_sha256: doc.binary_sha256.clone(),
                            },
                            start,
                        )?,
                        start,
                        bindings: super::diagnostic_resources::Bindings {
                            context_sha256: None,
                            record_sha256: None,
                            diagnostics_sha256: None,
                            session_sha256: None,
                        },
                        stalled: None,
                        marks: 0,
                    })
                })
                .transpose()?;
            let execution_result = (|| -> Result<(), String> {
                let arm = &p.arms[ex.arm];
                let w = &p.workloads[ex.workload];
                let inst = p.instance(ex.workload);
                let input = ChildInput {
                    schema: CHILD_INPUT_SCHEMA.into(),
                    diagnostics: None,
                    diagnostic_lifecycle: opts
                        .diagnostic_launch
                        .as_ref()
                        .map(|launch| {
                            prepare_lifecycle_child(
                                launch,
                                &session_id,
                                ex,
                                arm,
                                w,
                                diagnostic_root.as_deref().ok_or("diagnostic root absent")?,
                            )
                        })
                        .transpose()?,
                    curve: w.curve_spec.for_child(inst),
                    target_seed: w.target_seed,
                    target_index: w.target_index,
                    target_kind: w.kind(),
                    expected_workload_id: w.workload_id.clone(),
                    method: MethodSpec {
                        id: arm.method.id.clone(),
                        params: arm.method.params.clone(),
                    },
                    expected_method_id: arm.method.method_id.clone(),
                    algorithm_seed: ex.algorithm_seed,
                };
                if let Some(child) = &input.diagnostic_lifecycle {
                    child.validate(&input)?;
                    let bytes = diagnostic_json_bytes(child)?;
                    if bytes.len() > super::record::MAX_DIAGNOSTIC_CHILD_BYTES {
                        return Err("diagnostic child byte cap".into());
                    }
                    let hash = diagnostic_write_new(
                        &out.join("exec")
                            .join(format!("{:06}.diagnostic-context.json", ex.seq)),
                        &bytes,
                    )?;
                    if let Some(resource) = &mut resources {
                        resource.bindings.context_sha256 = Some(hash);
                    }
                }
                let before = snapshot();
                // A child that cannot be started (spawn, pinning) is a record
                // of its own, not a silent end of the session.
                let observe_child = resources.is_some();
                let mut spawned = || {
                    if let (Some(resource), Some(clock)) = (&mut resources, resource_clock) {
                        resource.observe(
                            super::diagnostic_resources::Stage::ChildStarted,
                            None,
                            clock,
                        );
                    }
                };
                let child_result = if observe_child {
                    run_child_observed(
                        &opts,
                        cpu_plan.as_ref(),
                        &env,
                        &input,
                        timeout,
                        Some(&mut spawned),
                    )
                } else {
                    run_child(&opts, cpu_plan.as_ref(), &env, &input, timeout)
                };
                let child_reaped = ReapObservation::from_result(&child_result);
                if let Some(resource) = &mut resources {
                    child_reaped
                        .mark_resources(resource, resource_clock.ok_or("diagnostic clock absent")?);
                }
                let reaped = child_result.unwrap_or_else(Reaped::spawn_failed);
                let after = snapshot();
                // n counts every execution of this (method, workload) in the
                // session, warm-ups and A/A arms included, so a run id is
                // unique in its session and n ≥ 1 as the convention requires.
                let rep_n = {
                    let n = reps
                        .entry((arm.method.method_id.clone(), w.workload_id.clone()))
                        .or_insert(0);
                    *n += 1;
                    *n
                };
                let mut rec = assemble(
                    &session_id,
                    ex,
                    rep_n,
                    arm,
                    w,
                    inst,
                    capsule,
                    &reaped,
                    before,
                    after,
                    &GradeContext {
                        plan: cpu_plan.as_ref(),
                        preflight: pre.as_ref(),
                        eviction: eviction.as_ref(),
                        runner_on_run_cpu,
                        thresholds: &opts.thresholds,
                        hz: hz as f64,
                    },
                );
                let interrupted = interrupted_error();
                if let (Some(e), true) = (&interrupted, reaped.interrupted) {
                    // The child was killed by the interruption: say so.
                    rec.outcome.error = Some(match rec.outcome.error.take() {
                        Some(prev) => format!("{prev}; session {e}"),
                        None => format!("session {e}"),
                    });
                }
                if !reaped.stderr.is_empty() {
                    let _ = std::fs::write(
                        out.join("exec").join(format!("{:06}.stderr", ex.seq)),
                        &reaped.stderr,
                    );
                }
                // Sealed over the exact bytes written (see `Record::seal`).
                let line = rec.seal() + "\n";
                records
                    .write_all(line.as_bytes())
                    .map_err(|e| e.to_string())?;
                records.sync_data().map_err(|e| e.to_string())?;
                if let Some(resource) = &mut resources {
                    records.sync_all().map_err(|e| e.to_string())?;
                    std::fs::File::open(out)
                        .and_then(|d| d.sync_all())
                        .map_err(|e| e.to_string())?;
                    resource.observe(
                        super::diagnostic_resources::Stage::RecordDurable,
                        Some(sha256_hex(line.as_bytes())),
                        resource_clock.ok_or("diagnostic clock absent")?,
                    );
                }
                if input.diagnostic_lifecycle.is_some() {
                    let mut attachment = match child_reaped
                        .when_observed(|| ingest_lifecycle_v2(&input, line.as_bytes()))
                    {
                        Some(result) => result?,
                        None => json!({"schema":"ecbench.diagnostic_attachment/v2",
                            "record_sha256":sha256_hex(line.as_bytes()),
                            "context":input.diagnostic_lifecycle,
                            "error":"child quiescence not observed; sidecars not read",
                            "lifecycle":null,"prefix":null,"accepted_result":null,"accepted_cost":null}),
                    };
                    // Child fields are observer declarations, never accepted
                    // mathematics. Preserve even if the watchdog rejected output.
                    attachment["child_reported_lifecycle"] = reaped
                        .output
                        .as_ref()
                        .and_then(|o| o.diagnostic_lifecycle.clone())
                        .unwrap_or(Value::Null);
                    attachment["child_reported_call_wall_ns"] = serde_json::to_value(
                        reaped
                            .output
                            .as_ref()
                            .and_then(|o| o.diagnostic_call_wall_ns),
                    )
                    .map_err(|e| e.to_string())?;
                    let bytes = diagnostic_json_bytes(&attachment)?;
                    if bytes.len() > 8 * 1024 * 1024 {
                        return Err("diagnostic attachment byte cap".into());
                    }
                    let hash = diagnostic_write_new(
                        &out.join("exec")
                            .join(format!("{:06}.diagnostics-v2.json", ex.seq)),
                        &bytes,
                    )?;
                    if let Some(resource) = &mut resources {
                        resource.observe(
                            super::diagnostic_resources::Stage::DiagnosticsDurable,
                            Some(hash),
                            resource_clock.ok_or("diagnostic clock absent")?,
                        );
                    }
                }
                // Producer contexts are currently always None. This custody path is
                // separate from accepted result/cost and cannot promote partial data.
                if let Some(context) = &input.diagnostics {
                    let ingestion = super::diagnostics::ingest(out, context, line.as_bytes());
                    let attachment = match ingestion {
                        Ok(manifest) => {
                            let bytes = serde_json::to_vec(&manifest).map_err(|e| e.to_string())?;
                            serde_json::json!({"schema":"ecbench.diagnostic_attachment/v1",
                            "record_sha256":sha256_hex(line.as_bytes()),
                            "manifest_sha256":sha256_hex(&bytes),"manifest":manifest,
                            "accepted_result":null,"accepted_cost":null})
                        }
                        Err(error) => {
                            serde_json::json!({"schema":"ecbench.diagnostic_attachment/v1",
                        "record_sha256":sha256_hex(line.as_bytes()),"manifest":null,
                        "error":error,"accepted_result":null,"accepted_cost":null})
                        }
                    };
                    write_json(
                        &out.join("exec")
                            .join(format!("{:06}.diagnostics.json", ex.seq)),
                        &attachment,
                    )?;
                }
                *session
                    .status_counts
                    .entry(rec.outcome.status.clone())
                    .or_insert(0) += 1;
                session.records_written += 1;
                session.last_seq = Some(ex.seq);
                // Atomic, after every record: a session killed outright still
                // says how many records it wrote and how far it got.
                write_json(&out.join("session.json"), &session)?;
                if let Some(resource) = &mut resources {
                    // The legacy writer's directory sync is best-effort. Observe an
                    // explicit successful sync before recording durability, then keep
                    // immutable exact session bytes for this execution's binding.
                    std::fs::File::open(out)
                        .and_then(|d| d.sync_all())
                        .map_err(|e| e.to_string())?;
                    let bytes = diagnostic_json_bytes(&session)?;
                    let hash = diagnostic_write_new(
                        &out.join("exec")
                            .join(format!("{:06}.session-snapshot.json", ex.seq)),
                        &bytes,
                    )?;
                    resource.observe(
                        super::diagnostic_resources::Stage::SessionDurable,
                        Some(hash),
                        resource_clock.ok_or("diagnostic clock absent")?,
                    );
                }
                progress(&format!(
                    "{:>4}/{} {:<14} {} {} {:>9} S={} {}",
                    ex.seq + 1,
                    p.executions.len(),
                    arm.name,
                    w.curve.slug,
                    w.workload_id,
                    rec.outcome.status,
                    rec.cost
                        .s
                        .map(|s| format!("{s:.3}"))
                        .unwrap_or_else(|| "-".into()),
                    rec.isolation.level.name(),
                ));
                if let Some(message) = reaped.spawn_error.clone() {
                    stop = Some(SessionStop {
                        kind: "spawn_error".into(),
                        seq: Some(ex.seq),
                        signal: None,
                        message: message.clone(),
                    });
                    return Err(message);
                }
                if let Some(e) = interrupted {
                    stop = Some(SessionStop {
                        kind: "interrupt".into(),
                        seq: Some(ex.seq),
                        signal: signals::interrupted(),
                        message: e.clone(),
                    });
                    return Err(e);
                }
                Ok(())
            })();
            let publication = if let Some(resource) = resources {
                resource.publish(
                    resource_clock.ok_or("diagnostic clock absent")?,
                    out,
                    ex.seq,
                    resource_journal.as_mut().ok_or("resource journal absent")?,
                    &execution_result,
                )
            } else {
                Ok(())
            };
            match (execution_result, publication) {
                (Ok(()), Ok(())) => {}
                (Err(error), Ok(())) => return Err(error),
                (Ok(()), Err(error)) => {
                    return Err(format!("resource publication failed: {error}"))
                }
                (Err(error), Err(publication)) => {
                    return Err(format!(
                        "{error}; resource publication failed: {publication}"
                    ))
                }
            }
        }
        Ok(())
    })();
    drop(records);
    // Publish once even when the execution loop returned an error. No external
    // exit observation exists here; the journal's own publication remains open.
    let journal_result = if let Some(journal) = &resource_journal {
        serde_json::to_vec(journal)
            .map_err(|e| e.to_string())
            .and_then(|bytes| {
                diagnostic_write_new(&out.join("resource-journal.json"), &bytes).map(|_| ())
            })
    } else {
        Ok(())
    };
    let result = match (result, journal_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(format!(
            "final resource journal publication failed: {error}"
        )),
        (Err(error), Err(publication)) => Err(format!(
            "{error}; final resource journal publication failed: {publication}"
        )),
    };

    if stop.is_none() {
        if let Err(e) = &result {
            stop = Some(SessionStop {
                kind: "error".into(),
                seq: session.last_seq,
                signal: None,
                message: e.clone(),
            });
        }
    }
    session.stop = stop;
    // The counts and the hash so far, whatever happened: an interrupted
    // session keeps what it measured.
    if let Ok(body) = std::fs::read(out.join("records.jsonl")) {
        session.records_sha256 = Some(sha256_hex(&body));
    }
    write_json(&out.join("session.json"), &session)?;
    result.map(|()| session)
}

/// What grading needs from the session, beside the record's own
/// observations.
pub struct GradeContext<'a> {
    pub plan: Option<&'a CpuPlan>,
    pub preflight: Option<&'a Preflight>,
    pub eviction: Option<&'a EvictionSummary>,
    pub runner_on_run_cpu: bool,
    pub thresholds: &'a Thresholds,
    pub hz: f64,
}

/// Build the record for one execution from what the runner observed.
#[allow(clippy::too_many_arguments)]
fn assemble(
    session_id: &str,
    ex: &Execution,
    rep_n: u32,
    arm: &crate::cryptanalysis::ecbench::spec::Arm,
    w: &Workload,
    inst: &crate::cryptanalysis::ecbench::workload::Instance,
    capsule: &HostCapsule,
    reaped: &Reaped,
    before: crate::cryptanalysis::ecbench::isolation::Conditions,
    after: crate::cryptanalysis::ecbench::isolation::Conditions,
    ctx: &GradeContext,
) -> Record {
    // The watchdog can fire after stdout was written but before the runner
    // acknowledges completion. Keep its decision authoritative: a censored
    // execution must not also carry an accepted scalar or final cost. Preserve
    // the parsed late protocol message inside the sealed diagnostic instead.
    // Do not change historical records or weaken the auditor's status check.
    let report = super::diagnostics::accepted_after_stop(
        reaped.output.as_ref(),
        reaped.timed_out,
        reaped.interrupted,
    )
    .and_then(|o| o.report.clone());
    let child_error = if let Some(message) = &reaped.spawn_error {
        Some(format!("spawn_error: {message}"))
    } else if reaped.timed_out {
        Some(match &reaped.output {
            Some(output) => format!(
                "watchdog expired; unaccepted parsed child output: {}",
                serde_json::to_string(output).expect("child protocol serializes")
            ),
            None => "watchdog expired".to_string(),
        })
    } else {
        reaped
            .output
            .as_ref()
            .and_then(|o| o.error.clone())
            .or_else(|| reaped.parse_error.clone())
    };
    let exited_cleanly = reaped.exit.as_deref() == Some("exit 0");
    let recovered = report.as_ref().and_then(|r| r.recovered);
    // Verification in the runner's own process: [k]G against the target.
    // This audit clock is separate from child timing and its isolation grade.
    let audit_verification = recovered.map(|k| {
        let started = Instant::now();
        let matches = inst.mul_generator_hex(k).as_ref() == Some(&w.target);
        let elapsed_ns = u64::try_from(started.elapsed().as_nanos())
            .expect("bounded parent audit verification interval");
        (matches, elapsed_ns)
    });
    let matches_target = audit_verification.map(|(matches, _)| matches);
    // A public target has no planted answer: [k]G = Q decides alone.
    let matches_planted = w.planted.and_then(|p| recovered.map(|k| k == p));
    let status = if reaped.spawn_error.is_some() {
        "spawn_error"
    } else if reaped.interrupted {
        "interrupted"
    } else if reaped.timed_out {
        "timeout"
    } else if !exited_cleanly && report.is_none() {
        "crashed"
    } else if child_error.is_some() || report.is_none() {
        "error"
    } else if matches_target == Some(true) && matches_planted != Some(false) {
        "verified"
    } else if recovered.is_some() {
        "wrong_answer"
    } else {
        "exhausted"
    };
    let sqrt_r = (w.curve.r as f64).sqrt();
    let floor = floor_s(w.curve.automorphisms_available);
    let s = report.as_ref().map(|r| r.total_gae / sqrt_r);
    let placement = reaped
        .output
        .as_ref()
        .map(|o| o.placement.clone())
        .unwrap_or_default();
    let g = grade(&GradeInput {
        plan: ctx.plan,
        preflight: ctx.preflight,
        eviction: ctx.eviction,
        runner_on_run_cpu: ctx.runner_on_run_cpu,
        capsule,
        thresholds: ctx.thresholds,
        placement: &placement,
        before: &before,
        after: &after,
        timing: &reaped.timing,
        hz: ctx.hz,
    });
    let mut record = Record {
        schema: RECORD_SCHEMA.into(),
        record_id: String::new(),
        session_id: session_id.into(),
        run_id: format!("{}{}R{}", arm.method.method_id, w.workload_id, rep_n),
        seq: ex.seq,
        round: ex.round,
        warmup: ex.warmup,
        arm: arm.name.clone(),
        role: serde_json::to_value(arm.role)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default(),
        method: arm.method.clone(),
        workload: w.clone(),
        algorithm_seed: ex.algorithm_seed.to_string(),
        outcome: Outcome {
            status: status.into(),
            recovered: recovered.map(|k| k.to_string()),
            matches_target,
            matches_planted,
            error: child_error,
            exit: reaped.exit.clone(),
            stderr_sha256: (!reaped.stderr.is_empty()).then(|| sha256_hex(&reaped.stderr)),
        },
        cost: Cost {
            unit: UNIT.into(),
            total_gae: report.as_ref().map(|r| r.total_gae),
            s,
            lower_bound: report
                .as_ref()
                .map(|r| !r.unpriced.is_empty())
                .unwrap_or(false),
            unpriced: report
                .as_ref()
                .map(|r| r.unpriced.clone())
                .unwrap_or_default(),
            deterministic: report.as_ref().map(|r| r.deterministic).unwrap_or(false),
            nondeterminism: report
                .as_ref()
                .map(|r| r.nondeterminism.clone())
                .unwrap_or_default(),
        },
        boundaries: Boundaries {
            automorphisms_available: w.curve.automorphisms_available,
            automorphisms_used: report.as_ref().map(|r| r.automorphisms_used),
            floor_s: floor,
            ratio_to_floor: s.map(|s| s / floor),
            floor_rule: Some(FLOOR_RULE_V2.into()),
        },
        phases: report
            .as_ref()
            .map(|r| r.phases.clone())
            .unwrap_or_default(),
        counters: report
            .as_ref()
            .map(|r| r.counters.clone())
            .unwrap_or_default(),
        factor_base: report.as_ref().and_then(|r| r.factor_base.clone()),
        fbm_id: report.as_ref().and_then(|r| {
            let fb = r.factor_base.as_ref()?;
            fbm::config_from_ic_method(&w.curve.slug, fb, &arm.method, INSTRUMENT_IC_PIPELINE)
                .seal()
                .ok()
                .map(|id| id.fbm_id)
        }),
        detail: report
            .as_ref()
            .map(|r| r.detail.clone())
            .unwrap_or(serde_json::Value::Null),
        online: report.as_ref().and_then(|r| r.online.clone()),
        solver: report.as_ref().and_then(|r| r.solver.clone()),
        allocator: reaped
            .output
            .as_ref()
            .filter(|_| !reaped.timed_out && !reaped.interrupted)
            .and_then(|o| o.allocator.clone()),
        field_ops: report.as_ref().and_then(|r| r.field_ops),
        time: reaped.timing.clone(),
        isolation: IsolationRecord {
            level: g.level,
            blockers: g.blockers,
            run_cpu: ctx.plan.map(|c| c.run_cpu),
            reserved: ctx.plan.map(|c| c.reserved.clone()).unwrap_or_default(),
            node: ctx.plan.and_then(|c| c.node),
            placement,
            foreign_busy_ticks: g.foreign_busy_ticks,
            steal_ticks: g.steal_ticks,
            idle_sibling_busy_ticks: g.idle_sibling_busy_ticks,
            before,
            after,
        },
        env_class_id: capsule.env_class_id.clone(),
        binary_sha256: capsule.build.binary_sha256.clone(),
        work: None,
    };
    record.time.parent_audit_verification_ns = audit_verification.map(|(_, ns)| ns);
    record.time.online_clock_diagnostics =
        report
            .as_ref()
            .map(|r| super::record::OnlineClockDiagnostics {
                error: r.online_error.clone(),
            });
    // The record's work in bitop/v1, derived from what it carries; a record
    // that cannot be priced (no report) carries none.
    if report.is_some() {
        record.work = crate::cryptanalysis::ecbench::work::price_record(&record).ok();
    }
    record
}

/// Read a session directory's records.
pub fn read_records(dir: &Path) -> Result<Vec<Record>, String> {
    Ok(read_record_lines(dir)?
        .into_iter()
        .map(|(_, r)| r)
        .collect())
}

/// Each record with the raw line it was read from, for the seal check.
pub fn read_record_lines(dir: &Path) -> Result<Vec<(String, Record)>, String> {
    read_record_lines_with(dir, from_str_legacy_floats)
}

/// Each record parsed with the repository's current correctly-rounded JSON
/// semantics. This is for explicit evidence migrations; v1 protocol replay
/// continues to use [`read_record_lines`].
pub fn read_record_lines_exact(dir: &Path) -> Result<Vec<(String, Record)>, String> {
    read_record_lines_with(dir, from_str_exact_floats)
}

fn from_str_exact_floats<T: serde::de::DeserializeOwned>(line: &str) -> Result<T, String> {
    crate::cryptanalysis::ecbench::canonical::reject_duplicate_keys(line)?;
    // Validate the complete typed shape first. In particular, serde's derived
    // record visitors reject duplicate fields; Value alone would keep only the
    // last occurrence. Discard the legacy f64 values from this pass.
    let _: T = serde_json::from_str(line).map_err(|error| error.to_string())?;
    // `arbitrary_precision` preserves the decimal token in Value until f64
    // conversion. Direct `deserialize_f64` uses the legacy significand path.
    let value: serde_json::Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}

fn read_record_lines_with(
    dir: &Path,
    parse: impl Fn(&str) -> Result<Record, String>,
) -> Result<Vec<(String, Record)>, String> {
    let text = std::fs::read_to_string(dir.join("records.jsonl"))
        .map_err(|e| format!("{}: {e}", dir.join("records.jsonl").display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(i, l)| {
            parse(l)
                .map(|r| (l.to_string(), r))
                .map_err(|e| format!("records.jsonl line {}: {e}", i + 1))
        })
        .collect()
}

pub fn read_session(dir: &Path) -> Result<Session, String> {
    let text = std::fs::read_to_string(dir.join("session.json"))
        .map_err(|e| format!("{}: {e}", dir.join("session.json").display()))?;
    serde_json::from_str(&text).map_err(|e| format!("session.json: {e}"))
}

#[cfg(test)]
mod exact_record_reader_tests {
    use super::{from_str_exact_floats, Record};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct FloatFixture {
        rho_s: f64,
    }

    #[test]
    fn exact_reader_preserves_recorded_float_and_rejects_duplicate_fields() {
        let line = r#"{"rho_s":0.37222665730402227}"#;
        let parsed: FloatFixture = from_str_exact_floats(line).unwrap();
        assert_eq!(
            parsed.rho_s.to_bits(),
            "0.37222665730402227".parse::<f64>().unwrap().to_bits()
        );

        let record_line = include_str!(
            "../../../research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/records.jsonl"
        )
        .lines()
        .next()
        .unwrap();
        assert!(from_str_exact_floats::<Record>(record_line).is_ok());
        let duplicate = record_line.replacen("{", r#"{"schema":"ecbench.record/v1","#, 1);
        assert!(from_str_exact_floats::<Record>(&duplicate).is_err());
    }
}

#[cfg(test)]
mod watchdog_record_tests {
    use super::*;

    #[test]
    fn late_checked_scalar_is_preserved_but_timeout_has_no_accepted_result() {
        let spec = Spec::from_json(r#"{
          "schema":"ecbench.spec/v1","label":"watchdog regression",
          "workloads":{"curves":[{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}],"targets_per_curve":1,"target_seed":20261017,"target_kind":"public"},
          "arms":[{"name":"bsgs","role":"reference","method":{"id":"bsgs.textbook"}}],
          "measurement":{"rounds":1,"warmup":0,"seed":1,"isolation_required":"L0"}
        }"#).unwrap();
        let p = plan(spec).unwrap();
        let ex = &p.executions[0];
        let arm = &p.arms[ex.arm];
        let w = &p.workloads[ex.workload];
        let input = ChildInput {
            schema: CHILD_INPUT_SCHEMA.into(),
            diagnostics: None,
            diagnostic_lifecycle: None,
            curve: w.curve_spec.clone(),
            target_seed: w.target_seed,
            target_index: w.target_index,
            target_kind: w.kind(),
            expected_workload_id: w.workload_id.clone(),
            method: MethodSpec {
                id: arm.method.id.clone(),
                params: arm.method.params.clone(),
            },
            expected_method_id: arm.method.method_id.clone(),
            algorithm_seed: ex.algorithm_seed,
        };
        let output = crate::cryptanalysis::ecbench::record::child_main(
            &serde_json::to_string(&input).unwrap(),
        );
        assert!(output.report.as_ref().unwrap().recovered.is_some());
        let capsule = capture().unwrap();
        let thresholds = Thresholds::default();
        let ctx = GradeContext {
            plan: None,
            preflight: None,
            eviction: None,
            runner_on_run_cpu: false,
            thresholds: &thresholds,
            hz: 100.0,
        };
        let mut reaped = Reaped {
            output: Some(output.clone()),
            parse_error: None,
            stderr: vec![],
            timing: Timing::default(),
            exit: Some("exit 0".into()),
            reap_observed: true,
            timed_out: false,
            interrupted: false,
            spawn_error: None,
        };
        let make = |r: &Reaped| {
            assemble(
                "watchdog-test",
                ex,
                1,
                arm,
                w,
                p.instance(ex.workload),
                &capsule,
                r,
                conditions(&[]),
                conditions(&[]),
                &ctx,
            )
        };
        let completed = make(&reaped);
        assert_eq!(completed.outcome.status, "verified");
        assert_eq!(completed.outcome.matches_target, Some(true));
        assert!(completed.time.parent_audit_verification_ns.is_some());
        assert_eq!(
            completed
                .time
                .online_clock_diagnostics
                .as_ref()
                .unwrap()
                .error,
            output.report.as_ref().unwrap().online_error
        );
        assert_eq!(completed.time.solve_wall_ns, reaped.timing.solve_wall_ns);
        assert_eq!(
            completed.cost.total_gae,
            output.report.as_ref().map(|r| r.total_gae)
        );
        reaped.timed_out = true;
        let censored = make(&reaped);
        assert_eq!(censored.outcome.status, "timeout");
        assert_eq!(censored.outcome.exit.as_deref(), Some("exit 0"));
        assert_eq!(censored.outcome.recovered, None);
        assert_eq!(censored.outcome.matches_target, None);
        assert_eq!(censored.time.parent_audit_verification_ns, None);
        assert_eq!(censored.time.online_clock_diagnostics, None);
        assert_eq!(censored.cost.total_gae, None);
        assert_eq!(censored.cost.s, None);
        assert!(!censored.cost.deterministic);
        assert!(censored.phases.is_empty());
        assert!(censored.counters.is_empty());
        let diagnostic = censored.outcome.error.unwrap();
        let retained: ChildOutput = serde_json::from_str(
            diagnostic
                .strip_prefix("watchdog expired; unaccepted parsed child output: ")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(retained, output);
        // A completed exhaustion arriving after the deadline is censored too.
        let late = reaped.output.as_mut().unwrap().report.as_mut().unwrap();
        late.recovered = None;
        late.exhausted = true;
        let exhausted_late = make(&reaped);
        assert_eq!(exhausted_late.outcome.status, "timeout");
        assert!(exhausted_late.cost.total_gae.is_none());
    }
}

#[cfg(test)]
mod robustness_tests {
    use super::*;

    const SPEC: &str = r#"{
      "schema":"ecbench.spec/v1","label":"robustness",
      "workloads":{"curves":[{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}],"targets_per_curve":1,"target_seed":20261017,"target_kind":"public"},
      "arms":[{"name":"bsgs","role":"reference","method":{"id":"bsgs.textbook"}}],
      "measurement":{"rounds":2,"warmup":0,"seed":1,"isolation_required":"L0"}
    }"#;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "ecbench-runner-{name}-{}-{}",
            std::process::id(),
            now_ms()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn options(lock: &Path, exe: &str, wait: bool) -> RunOptions {
        RunOptions {
            diagnostic_launch: None,
            cpus: CpuRequest::None,
            lock_path: lock.display().to_string(),
            wait_for_lock: wait,
            allow_busy: true,
            thresholds: Thresholds::default(),
            exe: PathBuf::from(exe),
        }
    }

    #[test]
    fn only_a_sigkill_that_the_watchdog_or_the_interruption_sent_is_a_stop() {
        // (watchdog fired, interruption, killed by SIGKILL) -> (timed out, interrupted)
        assert_eq!(classify_stop(true, false, true), (true, false));
        assert_eq!(classify_stop(false, true, true), (false, true));
        // The interruption wins when both are pending.
        assert_eq!(classify_stop(true, true, true), (false, true));
        // The child finished by itself in the instant the watchdog fired:
        // it was not killed, its answer stands.
        assert_eq!(classify_stop(true, false, false), (false, false));
        assert_eq!(classify_stop(false, true, false), (false, false));
        // Killed by something else (the OOM killer): a crash.
        assert_eq!(classify_stop(false, false, true), (false, false));
    }

    fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join(name);
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn input() -> ChildInput {
        ChildInput {
            schema: CHILD_INPUT_SCHEMA.into(),
                diagnostics: None,
                diagnostic_lifecycle: None,
            curve: serde_json::from_str(
                r#"{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}"#,
            )
            .unwrap(),
            target_seed: 1,
            target_index: 0,
            target_kind: Default::default(),
            expected_workload_id: String::new(),
            method: MethodSpec {
                id: "bsgs.textbook".into(),
                params: Default::default(),
            },
            expected_method_id: String::new(),
            algorithm_seed: 1,
        }
    }

    /// `run_child` on a script, retrying the `ETXTBSY` another test thread's
    /// fork can cause while the script file was still open for writing.
    fn run_script(exe: &Path, timeout: Duration) -> Reaped {
        let opts = options(
            &exe.with_extension("lock"),
            &exe.display().to_string(),
            false,
        );
        for _ in 0..50 {
            let r = run_child(&opts, None, &child_env(), &input(), timeout);
            match r {
                Err(e) if e.contains("Text file busy") => {
                    std::thread::sleep(Duration::from_millis(50))
                }
                other => return other.unwrap(),
            }
        }
        panic!("script stayed busy");
    }

    #[test]
    fn diagnostic_reap_gate_observes_real_shell_child_without_algorithm() {
        let root = tmp("diagnostic-reap-shell");
        let executable = script(&root, "drain-stdin.sh", "cat >/dev/null\nexit 0");
        let opts = options(
            &root.join("observed.lock"),
            &executable.display().to_string(),
            false,
        );
        let mut observed = None;
        for _ in 0..50 {
            let spawned = std::cell::Cell::new(0usize);
            let result = run_child_observed(
                &opts,
                None,
                &child_env(),
                &input(),
                Duration::from_secs(30),
                Some(&mut || spawned.set(spawned.get() + 1)),
            );
            match result {
                Err(error) if error.contains("Text file busy") => {
                    assert_eq!(spawned.get(), 0);
                    std::thread::sleep(Duration::from_millis(50));
                }
                other => {
                    assert_eq!(spawned.get(), 1);
                    observed = Some(other.unwrap());
                    break;
                }
            }
        }
        let reaped = observed.expect("script stayed busy");
        assert_eq!(reaped.exit.as_deref(), Some("exit 0"));
        assert!(reaped.reap_observed);
        assert!(reaped.output.is_none());
        let observation = ReapObservation::from_result(&Ok(reaped));
        assert_eq!(observation.when_observed(|| "permitted"), Some("permitted"));
        let missing = root.join("does-not-exist");
        let opts = options(
            &root.join("missing.lock"),
            &missing.display().to_string(),
            false,
        );
        let spawned = std::cell::Cell::new(false);
        let failure = run_child_observed(
            &opts,
            None,
            &child_env(),
            &input(),
            Duration::from_secs(30),
            Some(&mut || spawned.set(true)),
        );
        assert!(failure.is_err());
        assert!(!spawned.get());
        assert!(ReapObservation::from_result(&failure)
            .when_observed(|| panic!("must not read"))
            .is_none());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_child_that_finishes_is_not_a_timeout_and_a_killed_one_is() {
        let d = tmp("deadline");
        // Finishes at once: exit 0, nothing censored.
        let fast = script(&d, "fast.sh", "exit 0");
        let r = run_script(&fast, Duration::from_secs(30));
        assert_eq!(r.exit.as_deref(), Some("exit 0"));
        assert!(!r.timed_out && !r.interrupted && r.spawn_error.is_none());
        // Still running at the deadline: killed by the watchdog's SIGKILL.
        let slow = script(&d, "slow.sh", "sleep 30");
        let t = Instant::now();
        let r = run_script(&slow, Duration::from_secs(1));
        assert!(t.elapsed() < Duration::from_secs(20));
        assert_eq!(r.exit.as_deref(), Some("signal 9"));
        assert!(r.timed_out && !r.interrupted);
        // Killed by something else (not the watchdog): a crash, not a timeout.
        let killed = script(&d, "killed.sh", "kill -9 $$");
        let r = run_script(&killed, Duration::from_secs(30));
        assert_eq!(r.exit.as_deref(), Some("signal 9"));
        assert!(!r.timed_out && !r.interrupted);
    }

    #[test]
    fn session_json_is_replaced_atomically() {
        let d = tmp("atomic");
        let path = d.join("session.json");
        write_atomic(&path, b"{\"a\":1}\n").unwrap();
        write_atomic(&path, b"{\"a\":2}\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"a\":2}\n");
        // No temporary file is left behind to be mistaken for the session.
        let names: Vec<String> = std::fs::read_dir(&d)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["session.json".to_string()], "{names:?}");
    }

    #[test]
    fn a_session_written_before_the_stop_fields_still_reads() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ecbench_numa_vm_20261002/sessions/v2-auto");
        if !dir.join("session.json").exists() {
            return;
        }
        let s = read_session(&dir).unwrap();
        assert!(s.last_seq.is_none() && s.stop.is_none());
        // And a new field never appears in an old session's serialisation.
        let text = serde_json::to_string(&s).unwrap();
        assert!(!text.contains("last_seq") && !text.contains("\"stop\""));
    }

    #[test]
    fn a_spawn_failure_is_a_record_and_a_stopped_session_not_a_silent_end() {
        let d = tmp("spawn");
        let out = d.join("session");
        let opts = options(&d.join("lock"), "/nonexistent/ecbench", false);
        let err = run_session(SPEC, &out, opts, |_| {}).expect_err("stops");
        assert!(err.contains("cannot exec"), "{err}");
        let session = read_session(&out).unwrap();
        assert_eq!(session.status, "interrupted");
        assert_eq!(session.records_written, 1);
        assert_eq!(session.last_seq, Some(0));
        let stop = session.stop.expect("the stop is recorded");
        assert_eq!(stop.kind, "spawn_error");
        assert_eq!(stop.seq, Some(0));
        assert!(session.records_sha256.is_some());
        let records = read_records(&out).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].outcome.status, "spawn_error");
        assert!(!records[0].counts());
        assert!(records[0]
            .outcome
            .error
            .as_deref()
            .unwrap()
            .starts_with("spawn_error: "));
        assert!(!out.join(".session.json.tmp").exists());
    }

    #[test]
    fn the_lock_is_taken_before_the_spec_is_planned() {
        let d = tmp("lock");
        let lock_path = d.join("lock");
        let held = BenchLock::acquire(&lock_path.display().to_string(), false).unwrap();
        // A spec that parses but cannot be planned (an unknown method).
        let bad = SPEC.replace("bsgs.textbook", "no.such.method");
        let opts = options(&lock_path, "/bin/true", false);
        let err = run_session(&bad, &d.join("never"), opts, |_| {})
            .err()
            .unwrap();
        // Before the change this was the planning error, produced without
        // waiting for the lock; now planning happens inside it.
        assert!(err.contains("another benchmark holds"), "{err}");
        assert!(!d.join("never").exists());
        drop(held);
        let opts = options(&lock_path, "/bin/true", false);
        let err = run_session(&bad, &d.join("never"), opts, |_| {})
            .err()
            .unwrap();
        assert!(!err.contains("another benchmark holds"), "{err}");
    }

    #[test]
    fn an_interrupted_child_is_not_a_crash_and_carries_no_result() {
        let spec = Spec::from_json(SPEC).unwrap();
        let p = plan(spec).unwrap();
        let ex = &p.executions[0];
        let arm = &p.arms[ex.arm];
        let w = &p.workloads[ex.workload];
        let capsule = capture().unwrap();
        let thresholds = Thresholds::default();
        let ctx = GradeContext {
            plan: None,
            preflight: None,
            eviction: None,
            runner_on_run_cpu: false,
            thresholds: &thresholds,
            hz: 100.0,
        };
        let reaped = Reaped {
            output: None,
            parse_error: None,
            stderr: vec![],
            timing: Timing::default(),
            exit: Some("signal 9".into()),
            reap_observed: true,
            timed_out: false,
            interrupted: true,
            spawn_error: None,
        };
        let rec = assemble(
            "t",
            ex,
            1,
            arm,
            w,
            p.instance(ex.workload),
            &capsule,
            &reaped,
            conditions(&[]),
            conditions(&[]),
            &ctx,
        );
        assert_eq!(rec.outcome.status, "interrupted");
        assert!(!rec.counts());
        let crashed = Reaped {
            interrupted: false,
            ..reaped
        };
        let rec = assemble(
            "t",
            ex,
            1,
            arm,
            w,
            p.instance(ex.workload),
            &capsule,
            &crashed,
            conditions(&[]),
            conditions(&[]),
            &ctx,
        );
        assert_eq!(rec.outcome.status, "crashed");
    }
}

#[cfg(test)]
mod diagnostic_acceptance_tests {
    use super::*;
    #[test]
    fn synthetic_reaped_stop_excludes_child_without_solver() {
        let mut reaped = Reaped::spawn_failed("synthetic".into());
        reaped.output = Some(ChildOutput {
            schema: super::super::record::CHILD_OUTPUT_SCHEMA.into(),
            workload_id: None,
            method_id: None,
            report: None,
            method_call_wall_ns: Some(17),
            diagnostic_lifecycle: None,
            diagnostic_call_wall_ns: None,
            error: None,
            placement: Default::default(),
            schedstat_solve: None,
            hw_solve: None,
            solve_obs: None,
            allocator: None,
        });
        reaped.spawn_error = None;
        for (timeout, interrupted) in [(true, false), (false, true), (true, true)] {
            reaped.timed_out = timeout;
            reaped.interrupted = interrupted;
            assert!(super::super::diagnostics::accepted_after_stop(
                reaped.output.as_ref(),
                reaped.timed_out,
                reaped.interrupted
            )
            .is_none());
        }
        reaped.timed_out = false;
        reaped.interrupted = false;
        assert!(super::super::diagnostics::accepted_after_stop(
            reaped.output.as_ref(),
            false,
            false
        )
        .is_some());
    }
    #[test]
    fn assemble_censors_complete_synthetic_late_report() {
        // Build only a tiny known-answer unit fixture; never invoke any solver.
        let spec=Spec::from_json(r#"{"schema":"ecbench.spec/v1","label":"diagnostic assembly fixture","workloads":{"curves":[{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}],"targets_per_curve":1,"target_seed":7,"target_kind":"planted"},"arms":[{"name":"fixture","role":"reference","method":{"id":"bsgs.textbook"}}],"measurement":{"rounds":1,"warmup":0,"seed":1,"isolation_required":"L0"}}"#).unwrap();
        let p = plan(spec).unwrap();
        let ex = &p.executions[0];
        let arm = &p.arms[ex.arm];
        let w = &p.workloads[ex.workload];
        let late:super::super::methods::SolveReport=serde_json::from_value(serde_json::json!({
            "recovered":w.planted.unwrap(),"exhausted":false,
            "phases":[{"name":"synthetic","adds":3,"doubles":1,"scalar_mults":0,"gae":4.0,"wall_ns":9}],
            "total_gae":4.0,"automorphisms_used":1,"counters":{"synthetic":99},
            "unpriced":[],"deterministic":true,"nondeterminism":[],"solve_wall_ns":9,
            "factor_base":null,"detail":{"synthetic":true},"online":null,"online_error":null,"solver":null,"field_ops":null
        })).unwrap();
        let output = ChildOutput {
            schema: super::super::record::CHILD_OUTPUT_SCHEMA.into(),
            workload_id: Some(w.workload_id.clone()),
            method_id: Some(arm.method.method_id.clone()),
            report: Some(late),
            method_call_wall_ns: Some(17),
            diagnostic_lifecycle: None,
            diagnostic_call_wall_ns: None,
            error: None,
            placement: Default::default(),
            schedstat_solve: None,
            hw_solve: None,
            solve_obs: None,
            allocator: None,
        };
        let capsule = capture().unwrap();
        let thresholds = Thresholds::default();
        let ctx = GradeContext {
            plan: None,
            preflight: None,
            eviction: None,
            runner_on_run_cpu: false,
            thresholds: &thresholds,
            hz: 100.0,
        };
        for has_output in [false, true] {
            for (timed_out, interrupted, status) in
                [(true, false, "timeout"), (false, true, "interrupted")]
            {
                let reaped = Reaped {
                    output: has_output.then(|| output.clone()),
                    parse_error: None,
                    stderr: vec![],
                    timing: Timing::default(),
                    exit: Some("exit 0".into()),
                    reap_observed: true,
                    timed_out,
                    interrupted,
                    spawn_error: None,
                };
                let record = assemble(
                    "diagnostic-fixture",
                    ex,
                    1,
                    arm,
                    w,
                    p.instance(ex.workload),
                    &capsule,
                    &reaped,
                    conditions(&[]),
                    conditions(&[]),
                    &ctx,
                );
                assert_eq!(record.outcome.status, status);
                assert!(record.outcome.recovered.is_none());
                assert!(record.outcome.matches_target.is_none());
                assert!(record.outcome.matches_planted.is_none());
                assert!(record.cost.total_gae.is_none());
                assert!(record.cost.s.is_none());
                assert!(!record.cost.deterministic);
                assert!(record.phases.is_empty());
                assert!(record.counters.is_empty());
                assert!(record.detail.is_null());
                assert!(record.online.is_none());
                assert!(record.time.parent_audit_verification_ns.is_none());
                assert!(record.time.online_clock_diagnostics.is_none());
            }
        }
        assert!(
            super::super::diagnostics::accepted_after_stop(Some(&output), false, false)
                .unwrap()
                .report
                .as_ref()
                .unwrap()
                .recovered
                .is_some()
        );
    }
}

#[cfg(test)]
mod diagnostic_publication_fixture_tests {
    use super::*;
    #[test]
    fn exclusive_publication_preserves_existing_and_hashes_exact_bytes() {
        let root =
            std::env::temp_dir().join(format!("diagnostic-publication-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let path = root.join("fixture.json");
        let bytes = b"{\"fixture\":true}\n";
        assert_eq!(
            diagnostic_write_new(&path, bytes).unwrap(),
            sha256_hex(bytes)
        );
        assert!(diagnostic_write_new(&path, b"changed").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&path).unwrap().permissions().mode() & 0o077,
                0
            );
        }
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn missing_spawn_preserves_partial_failed_prefix() {
        use super::super::diagnostic_resources::*;
        let identity = Identity {
            session_id: "fixture".into(),
            clock_id: "fake".into(),
            runner_id: "runner".into(),
            scientific_run_id: "science".into(),
            attempt: 0,
            seq: 0,
            protocol_sha256: "a".repeat(64),
            source_manifest_sha256: "b".repeat(64),
            binary_sha256: "c".repeat(64),
        };
        let prefix = ExecutionEnvelope::begin(identity, 10)
            .unwrap()
            .seal_prefix(
                Bindings {
                    context_sha256: Some("d".repeat(64)),
                    record_sha256: None,
                    diagnostics_sha256: None,
                    session_sha256: None,
                },
                Status::Failed("synthetic spawn failure".into()),
                4,
            )
            .unwrap();
        assert!(prefix.marks.is_empty());
        assert!(prefix.total_active_worker_ns.is_none());
        assert!(prefix.final_publication_tail_pending);
        let mut journal =
            SessionResourceJournal::begin("fixture".into(), "runner".into(), "fake".into(), 0)
                .unwrap();
        let bytes = diagnostic_json_bytes(&prefix).unwrap();
        journal
            .append_tail(PublicationTail::observed(&prefix, &bytes, 15, 16).unwrap())
            .unwrap();
        let closure = journal
            .close_on_external_exit(&diagnostic_json_bytes(&journal).unwrap(), None)
            .unwrap();
        assert!(closure.closed_payload_wall_ns.is_none());
        assert!(closure.total_active_worker_ns.is_none());
    }
    #[test]
    fn journal_capacity_rejects_whole_large_session_before_execution() {
        use super::super::diagnostic_resources::*;
        let identity = Identity {
            session_id: "fixture\"escaped".into(),
            clock_id: "clock".into(),
            runner_id: "runner".into(),
            scientific_run_id: "scientific".into(),
            attempt: 0,
            seq: 0,
            protocol_sha256: "a".repeat(64),
            source_manifest_sha256: "b".repeat(64),
            binary_sha256: "c".repeat(64),
        };
        let journal = SessionResourceJournal::begin(
            identity.session_id.clone(),
            identity.runner_id.clone(),
            identity.clock_id.clone(),
            0,
        )
        .unwrap();
        assert!(resource_journal_capacity(&journal, identity.clone(), 3240).is_ok());
        assert!(resource_journal_capacity(&journal, identity.clone(), 22680).is_err());
        assert!(resource_journal_capacity(&journal, identity.clone(), usize::MAX).is_err());
        let bound = resource_journal_capacity(&journal, identity.clone(), 1).unwrap();
        let mut observed = journal.clone();
        observed
            .append_tail(PublicationTail {
                prefix_sha256: "f".repeat(64),
                identity,
                execution_start_ns: 0,
                prefix_covered_through_ns: 1,
                receipt_write_started_ns: 2,
                receipt_write_durable_ns: 3,
            })
            .unwrap();
        assert!(serde_json::to_vec(&observed).unwrap().len() <= bound);
    }
}

#[cfg(test)]
mod diagnostic_reap_publication_regression_tests {
    use super::super::diagnostic_resources::{
        Bindings, ExecutionEnvelope, Identity, ResourcePrefix, SessionResourceJournal, Stage,
        Status,
    };
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Root(PathBuf);
    impl Root {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "reap-publication-fixture-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            std::fs::create_dir(path.join("exec")).unwrap();
            Self(path)
        }
    }
    impl Drop for Root {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn tracking() -> ResourceTracking {
        ResourceTracking {
            envelope: ExecutionEnvelope::begin(
                Identity {
                    session_id: "fixture".into(),
                    clock_id: "fixture-clock".into(),
                    runner_id: "fixture-runner".into(),
                    scientific_run_id: "fixture-science".into(),
                    attempt: 0,
                    seq: 0,
                    protocol_sha256: "a".repeat(64),
                    source_manifest_sha256: "b".repeat(64),
                    binary_sha256: "c".repeat(64),
                },
                0,
            )
            .unwrap(),
            start: 0,
            bindings: Bindings {
                context_sha256: Some("d".repeat(64)),
                record_sha256: None,
                diagnostics_sha256: None,
                session_sha256: None,
            },
            stalled: None,
            marks: 0,
        }
    }
    fn journal() -> SessionResourceJournal {
        SessionResourceJournal::begin(
            "fixture".into(),
            "fixture-runner".into(),
            "fixture-clock".into(),
            0,
        )
        .unwrap()
    }
    fn assert_pending(journal: &SessionResourceJournal) {
        let status = journal
            .close_on_external_exit(&diagnostic_json_bytes(journal).unwrap(), None)
            .unwrap();
        assert!(status.final_publication_tail_pending);
        assert_eq!(status.closed_payload_wall_ns, None);
        assert_eq!(status.total_active_worker_ns, None);
    }
    #[test]
    fn ok_without_observed_reap_cannot_mark_or_read_sidecars() {
        for observed in [false, true] {
            let clock = DiagnosticSessionClock::begin().unwrap();
            let mut reaped = Reaped::spawn_failed("fixture".into());
            reaped.spawn_error = None;
            reaped.reap_observed = observed;
            let result = Ok(reaped); // Deliberately Ok even when reap was not observed.
            let gate = ReapObservation::from_result(&result);
            let mut resource = tracking();
            resource.observe(Stage::ChildStarted, None, &clock);
            gate.mark_resources(&mut resource, &clock);
            let root = Root::new();
            let sidecar = root.0.join("sidecar");
            std::fs::write(&sidecar, b"retained-sidecar").unwrap();
            let reads = std::cell::Cell::new(0);
            let bytes = gate.when_observed(|| {
                reads.set(reads.get() + 1);
                std::fs::read(&sidecar).unwrap()
            });
            assert_eq!(reads.get(), usize::from(observed));
            assert_eq!(bytes.is_some(), observed);
            assert_eq!(resource.marks, if observed { 2 } else { 1 });
            assert_eq!(resource.stalled.is_some(), !observed);
            let mut journal = journal();
            resource
                .publish(&clock, &root.0, 0, &mut journal, &Ok(()))
                .unwrap();
            let prefix: ResourcePrefix = serde_json::from_slice(
                &std::fs::read(root.0.join("exec/000000.resources.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(
                prefix.marks.iter().any(|m| m.stage == Stage::ChildReaped),
                observed
            );
            assert!(matches!(prefix.status, Status::Failed(_))); // Other stages intentionally absent.
            assert_pending(&journal);
        }
    }
    #[test]
    fn publication_collision_preserves_previous_bytes_and_pending_failure() {
        for block_tail in [false, true] {
            let root = Root::new();
            let clock = DiagnosticSessionClock::begin().unwrap();
            let mut journal = journal();
            let prior_journal = diagnostic_json_bytes(&journal).unwrap();
            let record = root.0.join("records.jsonl");
            std::fs::write(&record, b"prior-record\n").unwrap();
            let blocked = root.0.join(if block_tail {
                "exec/000000.resource-tail.json"
            } else {
                "exec/000000.resources.json"
            });
            std::fs::write(&blocked, b"prior-object\n").unwrap();
            let result = tracking().publish(
                &clock,
                &root.0,
                0,
                &mut journal,
                &Err("injected execution failure".into()),
            );
            assert!(result.is_err());
            assert_eq!(std::fs::read(&blocked).unwrap(), b"prior-object\n");
            assert_eq!(std::fs::read(&record).unwrap(), b"prior-record\n");
            if block_tail {
                // The prefix was durable before tail publication collided. Retain it and its failed status.
                let prefix: ResourcePrefix = serde_json::from_slice(
                    &std::fs::read(root.0.join("exec/000000.resources.json")).unwrap(),
                )
                .unwrap();
                assert_eq!(
                    prefix.status,
                    Status::Failed("injected execution failure".into())
                );
                assert!(prefix.final_publication_tail_pending);
                assert_eq!(prefix.total_active_worker_ns, None);
                assert_ne!(diagnostic_json_bytes(&journal).unwrap(), prior_journal);
            } else {
                assert_eq!(diagnostic_json_bytes(&journal).unwrap(), prior_journal);
                assert!(!root.0.join("exec/000000.resource-tail.json").exists());
            }
            assert_pending(&journal);
        }
    }
    #[test]
    fn diagnostic_raw_clock_offset_rejects_underflow_and_regression() {
        let clock = DiagnosticSessionClock {
            origin_ns: 100,
            boot_id: None,
            time_namespace: None,
            time_namespace_offsets: None,
            #[cfg(not(target_os = "linux"))]
            portable: Instant::now(),
            previous: std::cell::Cell::new(0),
            preparation: PreparationObservations {
                lock_wait_started_ns: None,
                lock_acquired_ns: None,
                execute_entry_ns: None,
            },
        };
        assert!(clock.checked_offset(99).is_err());
        assert_eq!(clock.checked_offset(105).unwrap(), 5);
        assert!(clock.checked_offset(104).is_err());
        assert_eq!(clock.checked_offset(105).unwrap(), 5);
        assert_eq!(clock.checked_offset(110).unwrap(), 10);
    }
    #[test]
    fn clock_identity_binds_exact_preparation_bytes_without_cycle() {
        let first = br#"{"origin_ns":1,"runner_id":"fixture"}"#;
        let changed = br#"{"origin_ns":2,"runner_id":"fixture"}"#;
        let first_hash = sha256_hex(first);
        let bound = preparation_clock_id("fixture", &first_hash);
        assert_eq!(bound, format!("fixture-preparation-sha256-{first_hash}"));
        assert_ne!(bound, preparation_clock_id("fixture", &sha256_hex(changed)));
        assert_ne!(bound, preparation_clock_id("other-runner", &first_hash));
        let mut same_json_different_bytes = first.to_vec();
        same_json_different_bytes.push(b'\n');
        assert_ne!(
            bound,
            preparation_clock_id("fixture", &sha256_hex(&same_json_different_bytes))
        );
        assert!(serde_json::from_slice::<Value>(first)
            .unwrap()
            .get("clock_id")
            .is_none());
    }
}

#[cfg(test)]
mod diagnostic_activation_tests {
    use super::super::diagnostic_launch::{DiagnosticLaunch, LAUNCH_SCHEMA, PROFILE_VERSION};
    use super::*;

    #[test]
    fn diagnostic_preplan_requires_actual_file_hash_and_exact_spec_without_planning() {
        let profile = json!({"version":PROFILE_VERSION,"sha256":"a".repeat(64)});
        let arms: Vec<_> = [
            (
                "pencil",
                "candidate",
                "ic.three_leads_genus2_pencil_symbolic_v2",
            ),
            ("rho", "reference", "rho.three_leads_negation_v1"),
            ("rho_AA", "control", "rho.three_leads_negation_v1"),
        ]
        .iter()
        .map(|(name, role, method)| {
            json!({"name":name,"role":role,"method":method,
            "method_config_sha256":"b".repeat(64),"profile":profile})
        })
        .collect();
        // Deliberately unplannable empty workload: this fixture exercises only
        // the production pre-plan gate and never generates a target or child.
        let spec:Spec=serde_json::from_value(json!({"schema":"ecbench.spec/v1","label":"fixture",
            "workloads":{"curves":[],"targets_per_curve":0,"target_seed":0},
            "arms":arms.iter().map(|a|json!({"name":a["name"],"role":a["role"],"method":{"id":a["method"]}})).collect::<Vec<_>>(),
            "measurement":{"rounds":0,"seed":0}})).unwrap();
        let (_, spec_digest) = short_id("ECS1", &spec.identity_view()).unwrap();
        let mut value = json!({"schema":LAUNCH_SCHEMA,"expected_spec_sha256":spec_digest,
            "binary_sha256":sha256_hex(b"fake executable fixture"),"protocol_sha256":"c".repeat(64),
            "source_receipt":{"scope":"external_manifest_binding_only","receipt_sha256":"d".repeat(64),"source_manifest_sha256":"e".repeat(64)},
            "scientific_run_id":"synthetic-fixture","attempt":0,"profile":profile,"arms":arms});
        let root = std::env::temp_dir().join(format!(
            "diagnostic-preplan-{}-{}",
            std::process::id(),
            now_ms()
        ));
        std::fs::create_dir(&root).unwrap();
        let exe = root.join("fake-executable");
        std::fs::write(&exe, b"fake executable fixture").unwrap();
        let launch = DiagnosticLaunch::from_bytes(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(validate_diagnostic_launch_before_plan(&launch, &spec, &exe).is_ok());
        std::fs::write(&exe, b"changed executable fixture").unwrap();
        assert!(validate_diagnostic_launch_before_plan(&launch, &spec, &exe).is_err());
        std::fs::write(&exe, b"fake executable fixture").unwrap();
        value["expected_spec_sha256"] = json!("f".repeat(64));
        let wrong = DiagnosticLaunch::from_bytes(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(validate_diagnostic_launch_before_plan(&wrong, &spec, &exe).is_err());
        std::fs::remove_file(&exe).unwrap();
        assert!(validate_diagnostic_launch_before_plan(&launch, &spec, &exe).is_err());
        std::fs::remove_dir(root).unwrap();
    }
}
