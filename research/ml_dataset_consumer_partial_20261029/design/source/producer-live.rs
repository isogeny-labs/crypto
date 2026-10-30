//! Bounded supervisor helpers for partial artifact publication.
//!
//! Network work runs in a separate process, leaving the parent free to enforce
//! the original solver deadline. Partial restore never invokes a solver.

use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::budget::{self, BudgetSession};
use super::capture::{self, AttemptBinding, CaptureCursor, CaptureLimits};
use super::snapshot::{
    self, AttemptObservation, AttemptStatus, SnapshotAnchor, SnapshotLimits, SnapshotProfile,
    SnapshotRequest,
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

pub(crate) fn context_role(path: &str) -> Option<&'static str> {
    CONTEXT_FILES
        .iter()
        .find_map(|(name, role)| (*name == path).then_some(*role))
        .or_else(|| super::telemetry::checked::role(path))
}

fn helper_schema(profile: SnapshotProfile, kind: &str) -> String {
    format!(
        "crypto.ml-dataset.{kind}/v{}",
        if profile.is_legacy() { 1 } else { 2 }
    )
}

fn acknowledged_present<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<Vec<Artifact>>, D::Error> {
    Vec::<Artifact>::deserialize(d).map(Some)
}

fn identity_set(
    profile: SnapshotProfile,
    copied: &BTreeSet<String>,
    artifacts: &Option<Vec<Artifact>>,
) -> Result<(), String> {
    if profile.is_legacy() {
        if artifacts.is_some() {
            return Err("legacy helper must omit acknowledged identities".into());
        }
        return Ok(());
    }
    let identities = artifacts
        .as_ref()
        .ok_or("checked helper lacks acknowledged identities")?;
    if identities.len() > 19 {
        return Err("checked helper context count".into());
    }
    let mut paths = BTreeSet::new();
    let mut bytes = 0u64;
    for artifact in identities {
        let name = artifact
            .path
            .strip_prefix("context/")
            .ok_or("checked identity prefix")?;
        if context_role(name) != Some(artifact.role.as_str())
            || artifact.encoding != "json"
            || !paths.insert(name.to_owned())
            || artifact.bytes > super::telemetry::checked::byte_limit(name)
        {
            return Err("invalid checked acknowledged identity".into());
        }
        crate::store::index::validate_sha256(&artifact.sha256)?;
        bytes = bytes
            .checked_add(artifact.bytes)
            .ok_or("checked context byte overflow")?;
    }
    if &paths != copied || bytes > super::telemetry::checked::MAX_CONTEXT {
        return Err("checked acknowledged path set or bytes mismatch".into());
    }
    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelperRequest {
    schema: String,
    #[serde(default, skip_serializing_if = "SnapshotProfile::is_legacy")]
    profile: SnapshotProfile,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "acknowledged_present"
    )]
    acknowledged_context: Option<Vec<Artifact>>,
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
    #[serde(default, skip_serializing_if = "SnapshotProfile::is_legacy")]
    profile: SnapshotProfile,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "acknowledged_present"
    )]
    acknowledged_context: Option<Vec<Artifact>>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "acknowledged_present"
    )]
    acknowledged_context: Option<Vec<Artifact>>,
    helper_request_sha256: String,
    snapshot: SnapshotRequest,
    copied_context: BTreeSet<String>,
    publish: bool,
}

fn prepared_identities(request: &HelperRequest, prepared: &PreparedSnapshot) -> Result<(), String> {
    identity_set(
        request.profile,
        &prepared.copied_context,
        &prepared.acknowledged_context,
    )?;
    if request.profile == SnapshotProfile::CheckedPhaseV2 {
        let mut expected = request
            .acknowledged_context
            .clone()
            .ok_or("missing prior identities")?;
        expected.extend(prepared.snapshot.context.iter().cloned());
        if prepared.acknowledged_context.as_ref() != Some(&expected) {
            return Err(
                "prepared identities do not match prior identities plus published delta".into(),
            );
        }
        if !prepared.publish
            && (!prepared.snapshot.context.is_empty() || prepared.snapshot.delta.is_some())
        {
            return Err("unpublished checked helper advances context or capture".into());
        }
    }
    Ok(())
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
    if request.schema != helper_schema(request.profile, "snapshot-helper") {
        return Err("unknown snapshot helper schema".into());
    }
    request.binding.validate()?;
    identity_set(
        request.profile,
        &request.copied_context,
        &request.acknowledged_context,
    )?;
    if (request.profile == SnapshotProfile::CheckedPhaseV2)
        != super::telemetry::checked::policy_present(&request.cell)?
    {
        return Err("helper profile differs from measurement policy".into());
    }
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
        if prepared.schema != helper_schema(request.profile, "prepared-snapshot")
            || prepared.snapshot.profile != request.profile
            || prepared.helper_request_sha256 != super::digest(&request_bytes)
            || prepared.snapshot.binding != request.binding
            || prepared.snapshot.ordinal != request.ordinal
            || prepared.snapshot.previous != request.previous
            || prepared.snapshot.observation != request.observation
            || !request.copied_context.is_subset(&prepared.copied_context)
        {
            return Err("prepared snapshot belongs to a different helper request".into());
        }
        identity_set(
            request.profile,
            &prepared.copied_context,
            &prepared.acknowledged_context,
        )?;
        if request.profile == SnapshotProfile::CheckedPhaseV2 {
            recheck_acknowledged(&request)?;
            for identity in request
                .acknowledged_context
                .as_ref()
                .ok_or("checked identities absent")?
            {
                if !prepared
                    .acknowledged_context
                    .as_ref()
                    .ok_or("prepared identities absent")?
                    .contains(identity)
                {
                    return Err("prepared snapshot changes acknowledged identity".into());
                }
            }
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
    prepared_identities(&request, &prepared)?;
    let mut result = HelperResult {
        schema: helper_schema(request.profile, "snapshot-helper-result"),
        profile: request.profile,
        acknowledged_context: prepared.acknowledged_context,
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
    if request.profile == SnapshotProfile::CheckedPhaseV2 {
        return prepare_checked_snapshot(request, request_bytes, |_| Ok(()));
    }
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
        acknowledged_context: None,
        helper_request_sha256: super::digest(request_bytes),
        snapshot: SnapshotRequest {
            profile: SnapshotProfile::LegacyV1,
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

fn recheck_acknowledged(request: &HelperRequest) -> Result<BTreeMap<String, Vec<u8>>, String> {
    identity_set(
        request.profile,
        &request.copied_context,
        &request.acknowledged_context,
    )?;
    let mut files = BTreeMap::new();
    for identity in request
        .acknowledged_context
        .as_ref()
        .ok_or("checked identities absent")?
    {
        let local = identity
            .path
            .strip_prefix("context/")
            .ok_or("checked identity prefix")?;
        // The saved identity length bounds the read, including after file growth.
        let bytes = super::telemetry::checked::read(&request.cell, local, identity.bytes)?;
        if bytes.len() as u64 != identity.bytes || super::digest(&bytes) != identity.sha256 {
            return Err("acknowledged context changed".into());
        }
        files.insert(local.to_owned(), bytes);
    }
    Ok(files)
}

fn prepare_checked_snapshot(
    request: &HelperRequest,
    request_bytes: &[u8],
    mut after_probe: impl FnMut(&str) -> Result<(), String>,
) -> Result<PreparedSnapshot, String> {
    use super::telemetry::checked;
    let mut files = recheck_acknowledged(request)?;
    let mut copied_context = request.copied_context.clone();
    let mut identities = request
        .acknowledged_context
        .clone()
        .ok_or("checked identities absent")?;
    let mut context = Vec::new();
    let mut total = files.values().map(|b| b.len() as u64).sum::<u64>();
    // Prior context is copied locally for authoritative full-prefix validation;
    // only newly discovered identities enter the next immutable delta manifest.
    for identity in &identities {
        let local = identity
            .path
            .strip_prefix("context/")
            .ok_or("context prefix")?;
        storage::write_bytes(
            &request.snapshot,
            &identity.path,
            &identity.role,
            "json",
            &files[local],
        )?;
    }
    let mut probe = |name: &str, role: &str, mandatory: bool| -> Result<bool, String> {
        if copied_context.contains(name) {
            return Ok(true);
        }
        let exists = match fs::symlink_metadata(request.cell.join(name)) {
            Ok(_) => true,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => return Err(e.to_string()),
        };
        // Test hook runs after the membership decision, never changing it.
        after_probe(name)?;
        if !exists {
            if mandatory {
                return Err(format!("required checked context absent: {name}"));
            }
            return Ok(false);
        }
        let remaining = checked::MAX_CONTEXT
            .checked_sub(total)
            .ok_or("checked context total")?;
        let bytes = checked::read(
            &request.cell,
            name,
            checked::byte_limit(name).min(remaining),
        )?;
        total = total
            .checked_add(bytes.len() as u64)
            .ok_or("checked context overflow")?;
        let artifact = storage::write_bytes(
            &request.snapshot,
            &format!("context/{name}"),
            role,
            "json",
            &bytes,
        )?;
        files.insert(name.to_owned(), bytes);
        copied_context.insert(name.to_owned());
        context.push(artifact.clone());
        identities.push(artifact);
        Ok(true)
    };
    // Policy is always captured before any phase, including the pre-worker flush.
    probe(checked::POLICY_PATH, "restricted-measurement-policy", true)?;
    for (index, &(name, role)) in CONTEXT_FILES.iter().enumerate() {
        probe(name, role, index < 5)?;
    }
    for phase in checked::types::Phase::ALL {
        if !probe(&phase.path(true), "phase-start", false)? {
            break;
        }
        if !probe(&phase.path(false), "hardware-and-resource-telemetry", false)? {
            break;
        }
    }
    checked::validate_context(&request.binding, &files)?;
    identity_set(request.profile, &copied_context, &Some(identities.clone()))?;
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
        schema: helper_schema(request.profile, "prepared-snapshot"),
        acknowledged_context: Some(identities),
        helper_request_sha256: super::digest(request_bytes),
        snapshot: SnapshotRequest {
            profile: request.profile,
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
    profile: SnapshotProfile,
    acknowledged_context: Option<Vec<Artifact>>,
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
        let profile = if super::telemetry::checked::policy_present(cell)? {
            SnapshotProfile::CheckedPhaseV2
        } else {
            SnapshotProfile::LegacyV1
        };
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
            profile,
            acknowledged_context: (!profile.is_legacy()).then(Vec::new),
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
            schema: helper_schema(self.profile, "snapshot-helper"),
            profile: self.profile,
            acknowledged_context: self.acknowledged_context.clone(),
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
        if result.schema != helper_schema(self.profile, "snapshot-helper-result")
            || result.profile != self.profile
            || result.request_sha256 != pending.request.sha256
        {
            return Err("snapshot helper result is not bound to its request".into());
        }
        identity_set(
            self.profile,
            &result.copied_context,
            &result.acknowledged_context,
        )?;
        if let Some(previous) = &self.acknowledged_context {
            if previous.iter().any(|a| {
                !result
                    .acknowledged_context
                    .as_ref()
                    .is_some_and(|next| next.contains(a))
            }) {
                return Err("snapshot acknowledgement changes existing context identity".into());
            }
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
        self.acknowledged_context = result.acknowledged_context;
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
                profile: SnapshotProfile::LegacyV1,
                acknowledged_context: None,
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

    fn checked_request(
        fixture: &Fixture,
        binding: AttemptBinding,
        previous: Option<HelperResult>,
    ) -> HelperRequest {
        let acknowledged = previous
            .as_ref()
            .and_then(|r| r.acknowledged_context.clone())
            .unwrap_or_default();
        let mut request = fixture.request(binding, previous);
        request.profile = SnapshotProfile::CheckedPhaseV2;
        request.schema = helper_schema(request.profile, "snapshot-helper");
        request.acknowledged_context = Some(acknowledged);
        request
    }

    #[test]
    fn checked_initial_policy_and_start_only_restore_use_authoritative_context() {
        use super::super::telemetry::checked;
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        checked::install_policy(&fixture.cell()).unwrap();
        let initial = fixture.run(&checked_request(&fixture, binding.clone(), None));
        assert_eq!(initial.profile, SnapshotProfile::CheckedPhaseV2);
        assert_eq!(initial.copied_context.len(), 6);
        assert_eq!(initial.acknowledged_context.as_ref().unwrap().len(), 6);
        checked::tests::write_phase(
            &fixture.cell(),
            &binding,
            checked::types::Phase::CurveConstruction,
            true,
        );
        assert!(!fixture.cell().join("worker-artifacts.json").exists());
        let roles = checked::validate_directory(&fixture.cell()).unwrap();
        assert_eq!(
            roles["telemetry/curve-construction-start.json"],
            "phase-start"
        );
        let (kind, _) =
            checked::restricted_record(&fixture.cell(), "telemetry/curve-construction-start.json")
                .unwrap();
        assert_eq!(kind, "interrupted_or_unfinished");
        let mut request = checked_request(&fixture, binding.clone(), Some(initial));
        request.observation =
            observation(AttemptStatus::Interrupted, None, Some("mock interruption"));
        let terminal = fixture.run(&request);
        let cfg = store_config(&fixture.0, Some(&request.remote)).unwrap();
        let report = snapshot::restore_partial(
            cfg,
            None,
            terminal.anchor.as_ref().unwrap(),
            &fixture.0.join("restored"),
            &SnapshotLimits::default(),
        )
        .unwrap();
        let restored = report.restored_directory.join("artifacts/context");
        assert_eq!(AttemptBinding::from_cell_files(&restored).unwrap(), binding);
        assert_eq!(checked::validate_directory(&restored).unwrap(), roles);
        assert!(!restored.join("telemetry/curve-construction.json").exists());
        assert!(
            !report.solver_resumed && !report.cell_complete && !report.scientific_result_verified
        );
    }

    #[test]
    fn checked_scan_defers_end_created_after_missing_start_probe() {
        use super::super::telemetry::checked;
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        checked::install_policy(&fixture.cell()).unwrap();
        let request = checked_request(&fixture, binding.clone(), None);
        fs::create_dir(&request.snapshot).unwrap();
        let bytes = serde_json::to_vec(&request).unwrap();
        let prepared = prepare_checked_snapshot(&request, &bytes, |name| {
            if name == "telemetry/curve-construction-start.json" {
                checked::tests::write_phase(
                    &fixture.cell(),
                    &binding,
                    checked::types::Phase::CurveConstruction,
                    false,
                );
            }
            Ok(())
        })
        .unwrap();
        assert!(fixture
            .cell()
            .join("telemetry/curve-construction.json")
            .exists());
        assert!(!prepared
            .copied_context
            .iter()
            .any(|p| p.starts_with("telemetry/")));
        assert_eq!(prepared.copied_context.len(), 6);
        assert!(checked::validate_directory(&request.snapshot.join("context")).is_ok());
    }

    #[test]
    fn checked_acknowledged_mutation_deletion_and_policy_downgrade_fail() {
        use super::super::telemetry::checked;
        for delete in [false, true] {
            let fixture = Fixture::new();
            let binding = fixture.prepare();
            checked::install_policy(&fixture.cell()).unwrap();
            storage::write_json(
                &fixture.cell(),
                "host.json",
                "host-measurement-context",
                &json!({"fixture":true}),
            )
            .unwrap();
            let initial = fixture.run(&checked_request(&fixture, binding.clone(), None));
            let request = checked_request(&fixture, binding, Some(initial));
            if delete {
                fs::remove_file(fixture.cell().join("host.json")).unwrap();
            } else {
                fs::write(fixture.cell().join("host.json"), b"{}").unwrap();
            }
            assert!(recheck_acknowledged(&request).is_err());
        }
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        checked::install_policy(&fixture.cell()).unwrap();
        let request = fixture.request(binding, None);
        let dir = fixture.0.join("legacy-request");
        fs::create_dir(&dir).unwrap();
        storage::write_json(&dir, "request.json", "request", &request).unwrap();
        assert!(snapshot_worker(&dir.join("request.json"))
            .unwrap_err()
            .contains("profile"));
        let mut value = serde_json::to_value(&request).unwrap();
        value["acknowledged_context"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<HelperRequest>(value).is_err());
        let value = serde_json::to_value(&request).unwrap();
        assert!(value.get("profile").is_none());
        assert!(value.get("acknowledged_context").is_none());
    }

    #[test]
    fn checked_scan_cannot_adopt_policy_and_phases_after_missing_policy_probe() {
        use super::super::telemetry::checked;
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        let request = checked_request(&fixture, binding.clone(), None);
        fs::create_dir(&request.snapshot).unwrap();
        let error =
            prepare_checked_snapshot(&request, &serde_json::to_vec(&request).unwrap(), |name| {
                if name == checked::POLICY_PATH {
                    checked::install_policy(&fixture.cell()).unwrap();
                    checked::tests::write_phase(
                        &fixture.cell(),
                        &binding,
                        checked::types::Phase::CurveConstruction,
                        false,
                    );
                }
                Ok(())
            })
            .err()
            .unwrap();
        assert!(error.contains("required checked context absent"));
        assert!(!request.snapshot.join("context").exists());
        assert!(fixture
            .cell()
            .join("telemetry/curve-construction.json")
            .exists());
    }

    #[test]
    fn checked_snapshot_cannot_continue_a_legacy_chain() {
        use super::super::telemetry::checked;
        let fixture = Fixture::new();
        let binding = fixture.prepare();
        let legacy = fixture.run(&fixture.request(binding.clone(), None));
        checked::install_policy(&fixture.cell()).unwrap();
        let mut request = checked_request(&fixture, binding, Some(legacy));
        // Even supplying self-consistent identities for the existing v1 files
        // cannot change the remote manifest chain's profile to v2.
        request.acknowledged_context = Some(
            request
                .copied_context
                .iter()
                .map(|name| {
                    let (sha256, bytes) =
                        crate::store::hash_file(&fixture.cell().join(name)).unwrap();
                    Artifact {
                        path: format!("context/{name}"),
                        role: context_role(name).unwrap().into(),
                        encoding: "json".into(),
                        sha256,
                        bytes,
                    }
                })
                .collect(),
        );
        let directory = fixture.0.join("mixed-helper");
        fs::create_dir(&directory).unwrap();
        storage::write_json(&directory, "request.json", "request", &request).unwrap();
        let error = snapshot_worker(&directory.join("request.json")).unwrap_err();
        assert!(error.contains("partial manifest identity"), "{error}");
        assert!(directory.join("prepared.json").is_file());
        assert!(!directory.join("result.json").exists());
    }

    #[test]
    fn checked_local_publication_rejects_role_and_rehashed_policy_substitution() {
        use super::super::telemetry::checked;
        for replace_policy in [false, true] {
            let fixture = Fixture::new();
            let binding = fixture.prepare();
            checked::install_policy(&fixture.cell()).unwrap();
            let request = checked_request(&fixture, binding, None);
            fs::create_dir(&request.snapshot).unwrap();
            let mut prepared =
                prepare_checked_snapshot(&request, &serde_json::to_vec(&request).unwrap(), |_| {
                    Ok(())
                })
                .unwrap();
            let policy = prepared
                .snapshot
                .context
                .iter_mut()
                .find(|a| a.path == "context/measurement-policy.json")
                .unwrap();
            if replace_policy {
                let bytes = b"{}\n";
                fs::write(request.snapshot.join(&policy.path), bytes).unwrap();
                policy.sha256 = super::super::digest(bytes);
                policy.bytes = bytes.len() as u64;
            } else {
                policy.role = "public-instance".into();
            }
            let cfg = store_config(&request.snapshot, Some(&request.remote)).unwrap();
            assert!(snapshot::publish_partial(
                &request.snapshot,
                cfg,
                None,
                &prepared.snapshot,
                &SnapshotLimits::default()
            )
            .is_err());
        }
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
