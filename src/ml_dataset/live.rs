//! Bounded supervisor helpers for partial artifact publication.
//!
//! Network work runs in a separate process, leaving the parent free to enforce
//! the original solver deadline. Partial restore never invokes a solver.

use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::budget::{self, BudgetSession};
use super::capture::{self, AttemptBinding, CaptureCursor, CaptureLimits};
use super::snapshot::{
    self, AttemptObservation, AttemptStatus, SnapshotAnchor, SnapshotLimits, SnapshotRequest,
};
use super::{storage, Artifact, SupervisedChild};
use crate::store::backend::{Phase, RemoteSpec, S3Guard};
use crate::store::Config;

#[derive(Args, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetArgs {
    #[arg(long = "budget-policy")]
    pub policy: Option<PathBuf>,
    #[arg(long = "budget-allocation")]
    pub allocation: Option<PathBuf>,
    #[arg(long = "budget-journal")]
    pub journal: Option<PathBuf>,
}

impl BudgetArgs {
    pub fn open(&self) -> Result<Option<Arc<BudgetSession>>, String> {
        match (&self.policy, &self.allocation, &self.journal) {
            (None, None, None) => Ok(None),
            (Some(policy), Some(allocation), Some(journal)) => {
                BudgetSession::open(policy, allocation, journal).map(Some)
            }
            _ => Err(
                "provide all of --budget-policy, --budget-allocation and --budget-journal".into(),
            ),
        }
    }
}

pub fn store_config(root: &Path, remote: Option<&str>) -> Result<Config, String> {
    let mut cfg = Config::discover()?;
    cfg.index_path = root.join("store-index.json");
    if let Some(remote) = remote {
        cfg.remote = Some(RemoteSpec::parse(remote)?);
    }
    Ok(cfg)
}

pub fn require_budget(cfg: &Config, budget: &Option<Arc<BudgetSession>>) -> Result<(), String> {
    if matches!(cfg.remote, Some(RemoteSpec::S3 { .. })) && budget.is_none() {
        return Err(
            "S3 dataset operations require a cumulative budget policy, allocation and journal"
                .into(),
        );
    }
    Ok(())
}

pub fn guard(budget: &Option<Arc<BudgetSession>>, phase: Phase) -> Option<S3Guard> {
    budget.as_ref().map(|b| b.guard(phase))
}

#[derive(Args)]
pub struct AllocateArgs {
    #[arg(long)]
    pub policy: PathBuf,
    #[arg(long)]
    pub allocation_id: String,
    #[arg(long)]
    pub limits: PathBuf,
    #[arg(long)]
    pub finalization_reserve: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

pub fn allocate(args: &AllocateArgs) -> Result<serde_json::Value, String> {
    budget::allocate(
        &args.policy,
        &args.allocation_id,
        super::read_json(&args.limits)?,
        super::read_json(&args.finalization_reserve)?,
        &args.out,
    )
}

#[derive(Args)]
pub struct RestorePartialArgs {
    /// The verified snapshot anchor saved in a snapshot acknowledgement.
    #[arg(long)]
    pub anchor: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
    #[arg(long)]
    pub remote: Option<String>,
    #[command(flatten)]
    pub budget: BudgetArgs,
}

pub fn restore_partial(args: &RestorePartialArgs) -> Result<serde_json::Value, String> {
    let cfg = store_config(&args.out, args.remote.as_deref())?;
    let budget = args.budget.open()?;
    require_budget(&cfg, &budget)?;
    let anchor: SnapshotAnchor = super::read_json(&args.anchor)?;
    let report =
        snapshot::restore_partial(cfg, budget, &anchor, &args.out, &SnapshotLimits::default())?;
    serde_json::to_value(report).map_err(|e| e.to_string())
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

const CONTEXT_FILES: &[(&str, &str)] = &[
    ("run.json", "restricted-run-binding"),
    ("plan.json", "restricted-frozen-plan"),
    ("protocol.json", "restricted-frozen-protocol"),
    ("config.json", "restricted-provenance"),
    ("attempt.json", "attempt-provenance"),
    ("host.json", "host-measurement-context"),
    ("worker-process.json", "process-provenance"),
    ("public.json", "public-instance"),
    ("witness.json", "restricted-known-answer"),
    ("factor-base.json", "factor-base"),
];

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelperRequest {
    schema: String,
    cell: PathBuf,
    snapshot: PathBuf,
    binding: AttemptBinding,
    cursor: CaptureCursor,
    limits: CaptureLimits,
    ordinal: u64,
    previous: Option<SnapshotAnchor>,
    copied_context: BTreeSet<String>,
    observation: AttemptObservation,
    remote: String,
    budget: BudgetArgs,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelperResult {
    schema: String,
    request_sha256: String,
    cursor: CaptureCursor,
    anchor: Option<SnapshotAnchor>,
    copied_context: BTreeSet<String>,
    published: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedSnapshot {
    schema: String,
    helper_request_sha256: String,
    snapshot: SnapshotRequest,
    copied_context: BTreeSet<String>,
    publish: bool,
}

/// Executes only artifact copying, integrity checks and bounded store commands.
pub fn snapshot_worker(request_path: &Path) -> Result<(), String> {
    if fs::metadata(request_path).map_err(|e| e.to_string())?.len() > 4 * 1024 * 1024 {
        return Err("snapshot helper request exceeds four MiB".into());
    }
    let request_bytes = fs::read(request_path).map_err(|e| e.to_string())?;
    if request_bytes.len() > 4 * 1024 * 1024 {
        return Err("snapshot helper request exceeds four MiB".into());
    }
    let request: HelperRequest =
        serde_json::from_slice(&request_bytes).map_err(|e| e.to_string())?;
    if request.schema != "crypto.ml-dataset.snapshot-helper/v1" {
        return Err("unknown snapshot helper schema".into());
    }
    request.binding.validate()?;
    request.limits.validate()?;
    if AttemptBinding::from_cell_files(&request.cell)? != request.binding {
        return Err("cell binding changed before snapshot".into());
    }
    let cfg = store_config(&request.snapshot, Some(&request.remote))?;
    let budget = request.budget.open()?;
    require_budget(&cfg, &budget)?;
    super::checked_directory(&request.snapshot, true)?;
    let prepared_dir = request_path.parent().ok_or("request has no parent")?;
    let prepared_path = prepared_dir.join("prepared.json");
    let prepared = if prepared_path.exists() {
        if fs::metadata(&prepared_path)
            .map_err(|e| e.to_string())?
            .len()
            > 4 * 1024 * 1024
        {
            return Err("prepared snapshot exceeds four MiB".into());
        }
        let prepared: PreparedSnapshot = super::read_json(&prepared_path)?;
        if prepared.schema != "crypto.ml-dataset.prepared-snapshot/v1"
            || prepared.helper_request_sha256 != super::digest(&request_bytes)
            || prepared.snapshot.binding != request.binding
            || prepared.snapshot.ordinal != request.ordinal
            || prepared.snapshot.previous != request.previous
            || prepared.snapshot.observation != request.observation
            || !request.copied_context.is_subset(&prepared.copied_context)
        {
            return Err("prepared snapshot belongs to a different helper request".into());
        }
        prepared
    } else {
        let prepared = prepare_snapshot(&request, &request_bytes)?;
        storage::write_json(
            prepared_dir,
            "prepared.json",
            "frozen-snapshot-request",
            &prepared,
        )?;
        prepared
    };
    let mut result = HelperResult {
        schema: "crypto.ml-dataset.snapshot-helper-result/v1".into(),
        request_sha256: super::digest(&request_bytes),
        cursor: request.cursor.clone(),
        anchor: request.previous.clone(),
        copied_context: prepared.copied_context,
        published: prepared.publish,
    };
    if prepared.publish {
        let ack = snapshot::publish_partial(
            &request.snapshot,
            cfg,
            budget,
            &prepared.snapshot,
            &SnapshotLimits::default(),
        )?;
        if let Some(cursor) = ack.cursor.clone() {
            result.cursor = cursor;
        }
        result.anchor = Some(ack.anchor.clone());
        storage::write_json(
            &request.snapshot,
            "ack.json",
            "verified-partial-snapshot-ack",
            &ack,
        )?;
        storage::write_json(
            &request.snapshot,
            "anchor.json",
            "verified-partial-snapshot-anchor",
            &ack.anchor,
        )?;
    }
    storage::write_json(
        prepared_dir,
        "result.json",
        "snapshot-helper-result",
        &result,
    )?;
    Ok(())
}

fn prepare_snapshot(
    request: &HelperRequest,
    request_bytes: &[u8],
) -> Result<PreparedSnapshot, String> {
    let mut copied_context = request.copied_context.clone();
    let mut context = Vec::new();
    let mut context_bytes = 0u64;
    for &(name, role) in CONTEXT_FILES {
        if copied_context.contains(name) {
            continue;
        }
        let path = storage::contained_path(&request.cell, name, false)?;
        if !path.exists() {
            if request.ordinal == 0 && context.len() < 5 {
                return Err(format!("required initial snapshot context missing: {name}"));
            }
            continue;
        }
        let len = fs::metadata(&path).map_err(|e| e.to_string())?.len();
        context_bytes = context_bytes
            .checked_add(len)
            .ok_or("snapshot context size overflow")?;
        if context_bytes > 16 * 1024 * 1024 {
            return Err("snapshot context exceeds sixteen MiB".into());
        }
        let bytes = fs::read(&path).map_err(|e| e.to_string())?;
        if bytes.len() as u64 != len {
            return Err("immutable context changed during copy".into());
        }
        context.push(storage::write_bytes(
            &request.snapshot,
            &format!("context/{name}"),
            role,
            "json",
            &bytes,
        )?);
        copied_context.insert(name.into());
    }
    let delta = capture::capture_committed_delta(
        &request.cell,
        &request.snapshot,
        &request.binding,
        &request.cursor,
        &request.limits,
    )?;
    let publish = !context.is_empty()
        || delta.is_some()
        || !matches!(request.observation.status, AttemptStatus::Running);
    Ok(PreparedSnapshot {
        schema: "crypto.ml-dataset.prepared-snapshot/v1".into(),
        helper_request_sha256: super::digest(request_bytes),
        snapshot: SnapshotRequest {
            binding: request.binding.clone(),
            ordinal: request.ordinal,
            previous: request.previous.clone(),
            context,
            delta,
            observation: request.observation.clone(),
        },
        copied_context,
        publish,
    })
}

struct Pending {
    child: SupervisedChild,
    started: Instant,
    request: Artifact,
    directory: PathBuf,
}

pub struct LiveSupervisor {
    root: PathBuf,
    cell: PathBuf,
    binding: AttemptBinding,
    limits: CaptureLimits,
    cursor: CaptureCursor,
    previous: Option<SnapshotAnchor>,
    context: BTreeSet<String>,
    ordinal: u64,
    helper_ordinal: u64,
    remote: String,
    budget: BudgetArgs,
    interval: Duration,
    timeout: Duration,
    last_attempt: Instant,
    pending: Option<Pending>,
}

impl LiveSupervisor {
    pub fn new(root: &Path, cell: &Path, args: &super::GenerateArgs) -> Result<Self, String> {
        if args.snapshot_seconds == 0 || args.snapshot_timeout_seconds == 0 {
            return Err("snapshot interval and timeout must be positive".into());
        }
        let binding = AttemptBinding::from_cell_files(cell)?;
        let cursor = CaptureCursor::initial(&binding)?;
        let limits = capture_limits(args.trace_mib)?;
        storage::write_json(
            cell,
            "capture-binding.json",
            "partial-capture-binding",
            &binding,
        )?;
        storage::write_json(
            cell,
            "capture-limits.json",
            "partial-capture-limits",
            &limits,
        )?;
        let remote = store_config(root, args.remote.as_deref())?
            .remote
            .ok_or("partial publication requires a remote")?
            .display();
        let base = super::checked_directory(
            &root
                .join("partial-snapshots")
                .join(format!("{:012}", binding.cell_index))
                .join(&binding.attempt_sha256),
            true,
        )?;
        Ok(Self {
            root: base,
            cell: cell.to_owned(),
            binding,
            limits,
            cursor,
            previous: None,
            context: BTreeSet::new(),
            ordinal: 0,
            helper_ordinal: 0,
            remote,
            budget: args.budget.clone(),
            interval: Duration::from_secs(args.snapshot_seconds),
            timeout: Duration::from_secs(args.snapshot_timeout_seconds),
            last_attempt: Instant::now(),
            pending: None,
        })
    }

    fn launch(&mut self, observation: AttemptObservation) -> Result<(), String> {
        if self.pending.is_some() {
            return Err("a partial snapshot is already in flight".into());
        }
        if self.helper_ordinal >= 4096 {
            return Err(
                "snapshot helper count limit reached; wave stopped with previous receipts retained"
                    .into(),
            );
        }
        let directory = super::checked_directory(
            &self
                .root
                .join("helpers")
                .join(format!("{:012}", self.helper_ordinal)),
            true,
        )?;
        let request = HelperRequest {
            schema: "crypto.ml-dataset.snapshot-helper/v1".into(),
            cell: self.cell.clone(),
            snapshot: self
                .root
                .join("snapshots")
                .join(format!("{:012}", self.ordinal)),
            binding: self.binding.clone(),
            cursor: self.cursor.clone(),
            limits: self.limits.clone(),
            ordinal: self.ordinal,
            previous: self.previous.clone(),
            copied_context: self.context.clone(),
            observation,
            remote: self.remote.clone(),
            budget: self.budget.clone(),
        };
        let artifact = storage::write_json(
            &directory,
            "request.json",
            "snapshot-helper-request",
            &request,
        )?;
        let stdout = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(directory.join("stdout.log"))
            .map_err(|e| e.to_string())?;
        let stderr = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(directory.join("stderr.log"))
            .map_err(|e| e.to_string())?;
        let mut command = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
        command
            .args(["dataset", "snapshot-worker", "--request"])
            .arg(directory.join("request.json"))
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::process::CommandExt;
            let parent = std::process::id();
            unsafe {
                command.pre_exec(move || {
                    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
                        || libc::getppid() as u32 != parent
                    {
                        return Err(std::io::Error::other("snapshot supervisor unavailable"));
                    }
                    Ok(())
                });
            }
        }
        let child = SupervisedChild {
            child: command.spawn().map_err(|e| e.to_string())?,
            finished: false,
        };
        self.pending = Some(Pending {
            child,
            started: Instant::now(),
            request: artifact,
            directory,
        });
        self.helper_ordinal = self
            .helper_ordinal
            .checked_add(1)
            .ok_or("snapshot helper counter overflow")?;
        self.last_attempt = Instant::now();
        Ok(())
    }

    fn collect(&mut self) -> Result<(), String> {
        let Some(pending) = self.pending.as_mut() else {
            return Ok(());
        };
        let Some(exit) = pending.child.poll()? else {
            if pending.started.elapsed() >= self.timeout {
                pending.child.stop()?;
                return Err("snapshot publication deadline exceeded; local artifacts retained, wave stopped".into());
            }
            return Ok(());
        };
        if !exit.success() {
            return Err(format!(
                "snapshot helper failed ({exit}); see {}, wave stopped",
                pending.directory.display()
            ));
        }
        let result: HelperResult = super::read_json(&pending.directory.join("result.json"))?;
        if result.schema != "crypto.ml-dataset.snapshot-helper-result/v1"
            || result.request_sha256 != pending.request.sha256
        {
            return Err("snapshot helper result is not bound to its request".into());
        }
        if result.published {
            if result.anchor.is_none() {
                return Err("snapshot publication omitted its verified anchor".into());
            }
            self.ordinal = self
                .ordinal
                .checked_add(1)
                .ok_or("snapshot ordinal overflow")?;
        }
        self.cursor = result.cursor;
        self.previous = result.anchor;
        self.context = result.copied_context;
        self.pending = None;
        Ok(())
    }

    /// Called from the existing 100ms solver monitor; never performs network I/O.
    pub fn poll(&mut self) -> Result<(), String> {
        self.collect()?;
        if self.pending.is_none()
            && !self.capacity_exhausted()
            && self.last_attempt.elapsed() >= self.interval
        {
            self.launch(observation(AttemptStatus::Running, None, None))?;
        }
        Ok(())
    }

    /// Leave one publication and helper slot for the terminal observation.
    pub fn capacity_exhausted(&self) -> bool {
        self.pending.is_none()
            && (self.ordinal >= SnapshotLimits::default().max_chain_length - 1
                || self.helper_ordinal >= 4095)
    }

    pub fn capture_stopped(&self) -> Result<bool, String> {
        let head = self.cell.join("live/committed.json");
        if !head.exists() {
            return Ok(false);
        }
        let head: capture::CommittedHead = super::read_json(&head)?;
        if head.binding_sha256 != self.binding.digest()? {
            return Err("capture watermark binding changed".into());
        }
        Ok(head.stop.is_some())
    }

    /// Called before worker launch and after worker exit, never inside its clock.
    pub fn flush(
        &mut self,
        status: AttemptStatus,
        exit_code: Option<i32>,
        reason: Option<&str>,
    ) -> Result<(), String> {
        while self.pending.is_some() {
            self.collect()?;
            if self.pending.is_some() {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        self.launch(observation(status, exit_code, reason))?;
        while self.pending.is_some() {
            self.collect()?;
            if self.pending.is_some() {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        Ok(())
    }
}

fn observation(
    status: AttemptStatus,
    exit_code: Option<i32>,
    reason: Option<&str>,
) -> AttemptObservation {
    AttemptObservation {
        status,
        observed_unix_seconds: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        exit_code,
        signal: None,
        reason: reason.map(str::to_owned),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-live-integration-{}-{}",
                std::process::id(),
                super::super::unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn cell(&self) -> PathBuf {
            self.0.join("cell")
        }
        fn prepare(&self) -> AttemptBinding {
            let cell = self.cell();
            fs::create_dir(&cell).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"artifact-only software fixture"});
            let hash =
                super::super::digest(&super::super::protocol_canonical_bytes(&protocol).unwrap());
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = super::super::DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![23],
                arities: vec![2],
                families: vec!["prime".into()],
                solvers: vec!["bsgs".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 1,
                factor_base_size: 4,
                degree_bound: 3,
                work_limit: 10,
                twists: true,
                protocol_sha256: Some(super::super::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
                factor_base_policies: Vec::new(),
            };
            let canonical_plan = super::super::digest(&serde_json::to_vec(&plan).unwrap());
            storage::write_json(&cell, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&cell,"run.json","run",&json!({"schema":"crypto.ml-dataset.run/v1",
                "plan_sha256":canonical_plan,"source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64)})).unwrap();
            storage::write_json(&cell, "config.json", "config", &plan.cell(0).unwrap()).unwrap();
            storage::write_json(
                &cell,
                "attempt.json",
                "attempt",
                &json!({"schema":"crypto.ml-dataset.attempt/v1",
                "cell_index":0,"plan_sha256":canonical_plan,"started_unix_nanos":"1"}),
            )
            .unwrap();
            storage::write_bytes(&cell, "protocol.json", "protocol", "json", &protocol).unwrap();
            AttemptBinding::from_cell_files(&cell).unwrap()
        }
        fn request(
            &self,
            binding: AttemptBinding,
            previous: Option<HelperResult>,
        ) -> HelperRequest {
            let (ordinal, cursor, anchor, copied) = match previous {
                Some(r) => (1, r.cursor, r.anchor, r.copied_context),
                None => (
                    0,
                    CaptureCursor::initial(&binding).unwrap(),
                    None,
                    BTreeSet::new(),
                ),
            };
            HelperRequest {
                schema: "crypto.ml-dataset.snapshot-helper/v1".into(),
                cell: self.cell(),
                snapshot: self.0.join(format!("snapshot-{ordinal}")),
                binding,
                cursor,
                limits: capture_limits(64).unwrap(),
                ordinal,
                previous: anchor,
                copied_context: copied,
                observation: observation(AttemptStatus::Running, None, None),
                remote: format!("file://{}", self.0.join("remote").display()),
                budget: BudgetArgs::default(),
            }
        }
        fn run(&self, request: &HelperRequest) -> HelperResult {
            let dir = self.0.join(format!("helper-{}", request.ordinal));
            fs::create_dir(&dir).unwrap();
            storage::write_json(&dir, "request.json", "request", request).unwrap();
            snapshot_worker(&dir.join("request.json")).unwrap();
            super::super::read_json(&dir.join("result.json")).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn helper_preserves_initial_context_then_only_acknowledged_intermediates() {
        use std::io::Write;
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        let initial = fixture.run(&fixture.request(binding.clone(), None));
        assert!(initial.published);
        assert_eq!(initial.cursor.event_count, 0);
        assert_eq!(initial.copied_context.len(), 5);
        let mut emitter = capture::CaptureEmitter::create(
            &fixture.cell().join("live"),
            binding.clone(),
            capture_limits(64).unwrap(),
        )
        .unwrap();
        emitter
            .event(&json!({"kind":"fixture_step","row":[1,0,1]}))
            .unwrap();
        emitter
            .payload(
                &super::super::types::ArtifactPayload::json(
                    "fixture-basis.json",
                    "software-fixture",
                    &json!({"rows":[[1,0,1]]}),
                )
                .unwrap(),
            )
            .unwrap();
        // Simulate a killed append after the last committed watermark.
        OpenOptions::new()
            .append(true)
            .open(fixture.cell().join("live/events.jsonl"))
            .unwrap()
            .write_all(b"{torn")
            .unwrap();
        let mut request = fixture.request(binding, Some(initial));
        request.observation = observation(
            AttemptStatus::Interrupted,
            None,
            Some("software interruption fixture"),
        );
        let final_result = fixture.run(&request);
        assert_eq!(final_result.cursor.event_count, 2);
        assert!(final_result.published);
        let cfg = store_config(&fixture.0, Some(&request.remote)).unwrap();
        let restored = fixture.0.join("restored");
        let report = snapshot::restore_partial(
            cfg,
            None,
            final_result.anchor.as_ref().unwrap(),
            &restored,
            &SnapshotLimits::default(),
        )
        .unwrap();
        let report = serde_json::to_value(report).unwrap();
        assert_eq!(report["solver_resumed"], false);
        assert!(!restored.join("cells").exists());
        assert!(!fixture.cell().join("manifest.json").exists());
    }

    #[test]
    fn changed_attempt_binding_rejects_helper_before_remote_publication() {
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        let request = fixture.request(binding, None);
        let dir = fixture.0.join("helper");
        fs::create_dir(&dir).unwrap();
        storage::write_json(&dir, "request.json", "request", &request).unwrap();
        fs::write(fixture.cell().join("attempt.json"), b"{}").unwrap();
        assert!(snapshot_worker(&dir.join("request.json")).is_err());
        assert!(!fixture.0.join("remote").exists());
        assert!(!dir.join("result.json").exists());
    }
    #[test]
    fn partial_budget_arguments_fail_before_opening_any_session() {
        let args = BudgetArgs {
            policy: Some("absent-policy.json".into()),
            ..Default::default()
        };
        assert!(matches!(args.open(), Err(e) if e.contains("provide all")));
    }
    #[test]
    fn capture_limits_reject_zero_and_arithmetic_overflow() {
        assert!(capture_limits(0).is_err());
        assert!(capture_limits(u64::MAX).is_err());
        assert!(capture_limits(64).is_ok());
    }
}
