//! Fixed-path startup admission. All retained records are restricted operational
//! evidence. This gate neither submits Jobs nor issues or refunds budget grants.
//! verify_protocol establishes frozen bytes/schema. The trusted GateConfig must
//! be reviewed against the protocol's intended schedule and resources; the gate
//! does not interpret prose stopping rules or claim general schedule validation.

use super::budget::{
    self, ActivationPhase, BudgetSession, CampaignPolicy, LaunchBinding, LaunchReservation,
    OwnerIdentity, Resources, WorkerGrant,
};
use super::{storage, BudgetArgs, BudgetCommand, DatasetPlan, GenerateArgs};
use rusqlite::{params, Connection, OpenFlags, TransactionBehavior};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

const CONFIG_SCHEMA: &str = "crypto.ml-dataset.startup-config/v1";
const BUNDLE_SCHEMA: &str = "crypto.ml-dataset.startup-bundle/v1";
const CONFIG_LIMIT: u64 = 64 * 1024;
const BUNDLE_LIMIT: u64 = 256 * 1024;
const JOURNAL_LIMIT: u64 = 64 * 1024;
const POLL_NANOS: u64 = 200_000_000;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GateConfig {
    schema: String,
    policy_sha256: String,
    reservation_sha256: String,
    startup_deadline_unix_seconds: u64,
    max_wait_seconds: u32,
    generate: GateGenerate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GateGenerate {
    max_cases: u64,
    seconds_per_case: u64,
    memory_mib: u64,
    trace_mib: u64,
    remote: String,
    lab_receipt_prefix: Option<String>,
    cell_envelope: Resources,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GateBundle {
    schema: String,
    policy: CampaignPolicy,
    reservation: LaunchReservation,
    binding: LaunchBinding,
    grant: WorkerGrant,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum GateErrorKind {
    MissingInputTimeout,
    InvalidInput,
    IdentityMismatch,
    AuthorityChanged,
    UnsupportedPlatform,
    ActivationFailure,
    GenerationFailure,
    GenerationOutcomeUnknown,
    PersistenceFailure,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct GateError {
    kind: GateErrorKind,
    message: String,
}

impl GateError {
    fn new(kind: GateErrorKind, message: impl ToString) -> Self {
        // Bound diagnostics even when an underlying parser supplies context.
        let message = message.to_string().chars().take(2048).collect();
        Self { kind, message }
    }
}
impl std::fmt::Display for GateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "startup gate {:?}: {}", self.kind, self.message)
    }
}
impl std::error::Error for GateError {}
type Result<T> = std::result::Result<T, GateError>;

fn invalid(message: impl ToString) -> GateError {
    GateError::new(GateErrorKind::InvalidInput, message)
}
fn authority(message: impl ToString) -> GateError {
    GateError::new(GateErrorKind::AuthorityChanged, message)
}
fn persistence(message: impl ToString) -> GateError {
    GateError::new(GateErrorKind::PersistenceFailure, message)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct GateOutcome {
    schema: String,
    status: String,
    owner: OwnerIdentity,
    config_sha256: String,
    accepted_bundle_sha256: String,
    generation_handler_returned_success: bool,
    kubernetes_attestation_performed: bool,
}

/// Paths are internal: only production() is reachable from the public CLI.
#[derive(Clone)]
struct Paths {
    config: PathBuf,
    plan: PathBuf,
    protocol: PathBuf,
    projection: PathBuf,
    admission: PathBuf,
    worker: PathBuf,
    dataset: PathBuf,
}
impl Paths {
    fn production() -> Self {
        Self {
            config: "/opt/crypto/dataset/startup-gate.json".into(),
            plan: "/opt/crypto/dataset/plan.json".into(),
            protocol: "/opt/crypto/dataset/protocol.json".into(),
            projection: "/run/dataset-admission".into(),
            admission: "/work/admission".into(),
            worker: "/work/budget/worker.sqlite".into(),
            dataset: "/work/dataset".into(),
        }
    }
    fn journal(&self) -> PathBuf {
        self.admission.join("gate.sqlite")
    }
    fn marker(&self) -> PathBuf {
        self.admission.join("gate.authority")
    }
    fn budget_args(&self) -> BudgetArgs {
        BudgetArgs {
            budget_policy: Some(self.admission.join("policy.json")),
            budget_grant: Some(self.admission.join("grant.json")),
            budget_owner: Some(self.admission.join("owner.json")),
            budget_state: Some(self.worker.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct LocalIdentity {
    source_commit: String,
    binary_sha256: String,
    plan_sha256: String,
    plan_file_sha256: String,
    protocol_sha256: String,
    namespace: String,
    pod_name: String,
}
struct VerifiedLocalInputs {
    identity: LocalIdentity,
    lab_receipt_uri: Option<String>,
    paths: Paths,
}
struct PreparedRun {
    generate: GenerateArgs,
    bundle: GateBundle,
    owner: OwnerIdentity,
}

fn identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
    {
        return Err(invalid("missing or invalid bounded owner/boot identifier"));
    }
    Ok(())
}
fn dns_label(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 63
        || !value.as_bytes()[0].is_ascii_alphanumeric()
        || !value.as_bytes()[value.len() - 1].is_ascii_alphanumeric()
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(invalid(
            "namespace/Pod name must be a bounded lowercase DNS label",
        ));
    }
    Ok(())
}
fn pod_name(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 253
        || !value.as_bytes()[0].is_ascii_alphanumeric()
        || !value.as_bytes()[value.len() - 1].is_ascii_alphanumeric()
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"-.".contains(&b))
        || value.split('.').any(str::is_empty)
    {
        return Err(invalid("invalid bounded Pod name"));
    }
    Ok(())
}
fn sha(value: &str) -> Result<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("expected lowercase SHA-256"));
    }
    Ok(())
}
fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    // Every object on this path is typed with deny_unknown_fields. Derived
    // struct deserializers also reject repeated known fields at every level.
    serde_json::from_slice(bytes).map_err(invalid)
}
fn multiply(value: &Resources, count: u64) -> Result<Resources> {
    let mul = |v: u64| {
        v.checked_mul(count)
            .ok_or_else(|| invalid("cell allocation overflow"))
    };
    Ok(Resources {
        money_micro_usd: mul(value.money_micro_usd)?,
        output_bytes: mul(value.output_bytes)?,
        put_attempts: mul(value.put_attempts)?,
        read_attempts: mul(value.read_attempts)?,
        download_bytes: mul(value.download_bytes)?,
        local_bytes: mul(value.local_bytes)?,
    })
}
fn validate_config(config: &GateConfig) -> Result<()> {
    if config.schema != CONFIG_SCHEMA
        || !(1..=300).contains(&config.max_wait_seconds)
        || config.startup_deadline_unix_seconds == 0
    {
        return Err(invalid("invalid startup schema or frozen wait/deadline"));
    }
    sha(&config.policy_sha256)?;
    sha(&config.reservation_sha256)?;
    let generate = &config.generate;
    if generate.max_cases == 0
        || generate.seconds_per_case == 0
        || generate.memory_mib < 128
        || generate.trace_mib == 0
        || generate.remote.len() > 2048
    {
        return Err(invalid("invalid frozen generation limits"));
    }
    // Reject numeric overflow before any activation, using the same units as
    // the existing supervisor. These checks do not claim hard write limits.
    generate
        .memory_mib
        .checked_mul(1024 * 1024)
        .ok_or_else(|| invalid("memory size overflow"))?;
    generate
        .trace_mib
        .checked_mul(1024 * 1024)
        .ok_or_else(|| invalid("trace size overflow"))?;
    multiply(&generate.cell_envelope, generate.max_cases)?;
    match crate::store::backend::RemoteSpec::parse(&generate.remote).map_err(invalid)? {
        crate::store::backend::RemoteSpec::S3 { bucket, prefix }
            if bucket == "setec-astronomy-artifacts" && !prefix.is_empty() => {}
        _ => {
            return Err(invalid(
                "startup generation requires explicit approved S3 prefix",
            ))
        }
    }
    if let Some(prefix) = &generate.lab_receipt_prefix {
        if prefix.len() > 2048 || !prefix.ends_with('/') {
            return Err(invalid(
                "lab receipt base must be bounded and slash terminated",
            ));
        }
        super::lab_receipt_remote(prefix).map_err(invalid)?;
    }
    Ok(())
}

fn validate_remote(policy: &CampaignPolicy, uri: &str) -> Result<()> {
    let crate::store::backend::RemoteSpec::S3 { bucket, prefix } =
        crate::store::backend::RemoteSpec::parse(uri).map_err(invalid)?
    else {
        return Err(invalid("startup remote is not S3"));
    };
    // Validate the entire slash-terminated destination under an allowed scope.
    policy
        .validate_remote(
            &bucket,
            &format!("{}/startup-scope-check", prefix.trim_end_matches('/')),
        )
        .map_err(invalid)
}

fn validate_delivery(
    config: &GateConfig,
    bundle: &GateBundle,
    actual_owner: &OwnerIdentity,
    local: &VerifiedLocalInputs,
) -> Result<PreparedRun> {
    validate_config(config)?;
    if bundle.schema != BUNDLE_SCHEMA
        || bundle.policy.sha256().map_err(invalid)? != config.policy_sha256
        || bundle.reservation.sha256().map_err(invalid)? != config.reservation_sha256
    {
        return Err(invalid(
            "bundle does not match frozen policy/reservation digests",
        ));
    }
    budget::validate_launch_delivery(
        &bundle.policy,
        &bundle.reservation,
        &bundle.binding,
        &bundle.grant,
        actual_owner,
    )
    .map_err(invalid)?;
    let request = &bundle.grant.request;
    let identity = &local.identity;
    if request.binding.source_commit != identity.source_commit
        || request.binding.binary_sha256 != identity.binary_sha256
        || request.binding.plan_sha256 != identity.plan_sha256
        || request.binding.protocol_sha256 != identity.protocol_sha256
        || bundle.reservation.request.target.namespace != identity.namespace
    {
        return Err(GateError::new(
            GateErrorKind::IdentityMismatch,
            "local owner/build/plan/protocol/namespace differs from pinned launch",
        ));
    }
    if request.worker_journal_path
        != local
            .paths
            .worker
            .to_str()
            .ok_or_else(|| invalid("journal path UTF-8"))?
        || config.startup_deadline_unix_seconds > request.expires_unix_seconds
        || config.startup_deadline_unix_seconds > bundle.policy.deadline_unix_seconds
    {
        return Err(invalid(
            "startup deadline/journal differs from admitted launch",
        ));
    }
    request
        .resources
        .checked_sub(&request.finalization)
        .map_err(invalid)?
        .checked_sub(&multiply(
            &config.generate.cell_envelope,
            config.generate.max_cases,
        )?)
        .map_err(invalid)?;
    // Preserve the generator's pre-cell remaining floor after a successful
    // first activation. IRSA uses separate activation/generation contexts, each
    // with an admitted 64 KiB bootstrap response. This is bootstrap headroom,
    // not a guarantee against retries, credential refresh or later exhaustion;
    // every actual request remains independently charged. This validation does
    // not spend budget or repeat activation when the retained phase is Active.
    let generator_floor = Resources {
        output_bytes: 64 * 1024,
        put_attempts: 2,
        read_attempts: 4,
        download_bytes: 256 * 1024,
        ..Resources::default()
    };
    let activation_headroom = Resources {
        output_bytes: 4096,
        put_attempts: 1,
        read_attempts: 1,
        download_bytes: 4096,
        ..Resources::default()
    };
    let credential_headroom = if request.credential_role_arn.is_some() {
        Resources {
            read_attempts: 2,
            download_bytes: 2 * 65536,
            ..Resources::default()
        }
    } else {
        Resources::default()
    };
    let finalization_floor = generator_floor
        .checked_add(&activation_headroom)
        .and_then(|floor| floor.checked_add(&credential_headroom))
        .map_err(invalid)?;
    request
        .finalization
        .checked_sub(&finalization_floor)
        .map_err(invalid)?;
    validate_remote(&bundle.policy, &config.generate.remote)?;
    match (&config.generate.lab_receipt_prefix, &local.lab_receipt_uri) {
        (None, None) => {}
        (Some(prefix), Some(actual)) => {
            let expected = format!(
                "{}{}/{}/",
                prefix, bundle.reservation.request.target.job_name, actual_owner.pod_uid
            );
            if actual != &expected {
                return Err(invalid(
                    "ISOLAB_RESULTS_URI differs from frozen Job/Pod receipt route",
                ));
            }
            super::lab_receipt_remote(actual).map_err(invalid)?;
            validate_remote(&bundle.policy, actual)?;
        }
        _ => return Err(invalid("unexpected or missing ISOLAB_RESULTS_URI")),
    }
    Ok(PreparedRun {
        generate: GenerateArgs {
            plan: local.paths.plan.clone(),
            protocol: local.paths.protocol.clone(),
            out: local.paths.dataset.clone(),
            max_cases: config.generate.max_cases,
            shard_index: request.binding.shard_index,
            shard_count: request.binding.shard_count,
            seconds_per_case: config.generate.seconds_per_case,
            memory_mib: config.generate.memory_mib,
            trace_mib: config.generate.trace_mib,
            publish: true,
            remote: Some(config.generate.remote.clone()),
            budget: local.paths.budget_args(),
            budget_cell_envelope: Some(local.paths.admission.join("cell-envelope.json")),
        },
        bundle: bundle.clone(),
        owner: actual_owner.clone(),
    })
}

#[derive(Clone, Copy)]
struct Moment {
    utc: u64,
    boottime_ns: u64,
}
trait Clock {
    fn now(&self) -> Result<Moment>;
    fn sleep(&mut self, nanos: u64);
}
struct NativeClock;
impl Clock for NativeClock {
    fn now(&self) -> Result<Moment> {
        #[cfg(target_os = "linux")]
        {
            let utc = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(invalid)?
                .as_secs();
            let mut ts = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            if unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) } != 0 {
                return Err(authority("CLOCK_BOOTTIME unavailable"));
            }
            let seconds = u64::try_from(ts.tv_sec).map_err(invalid)?;
            let nanos = u64::try_from(ts.tv_nsec).map_err(invalid)?;
            let boottime_ns = seconds
                .checked_mul(1_000_000_000)
                .and_then(|n| n.checked_add(nanos))
                .ok_or_else(|| authority("boot clock overflow"))?;
            Ok(Moment { utc, boottime_ns })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(GateError::new(
                GateErrorKind::UnsupportedPlatform,
                "startup gate requires Linux",
            ))
        }
    }
    fn sleep(&mut self, nanos: u64) {
        std::thread::sleep(Duration::from_nanos(nanos));
    }
}

#[cfg(target_os = "linux")]
fn ordinary(file: &File, private: bool) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata().map_err(authority)?;
    let uid = unsafe { libc::geteuid() };
    if !metadata.is_file()
        || metadata.nlink() != 1
        || (private && metadata.uid() != uid)
        || (!private && metadata.uid() != 0 && metadata.uid() != uid)
        || metadata.mode() & 0o022 != 0
    {
        return Err(authority(
            "file must be ordinary, singly linked and owned by the expected authority",
        ));
    }
    Ok(())
}
#[cfg(not(target_os = "linux"))]
fn ordinary(_file: &File, _private: bool) -> Result<()> {
    Err(GateError::new(
        GateErrorKind::UnsupportedPlatform,
        "startup gate requires Linux",
    ))
}

fn bounded(mut file: File, limit: u64) -> Result<Vec<u8>> {
    if file.metadata().map_err(invalid)?.len() > limit {
        return Err(invalid("startup input exceeds its byte limit"));
    }
    let mut bytes = Vec::new();
    file.by_ref()
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(invalid)?;
    if bytes.len() as u64 > limit {
        return Err(invalid("startup input grew beyond its byte limit"));
    }
    Ok(bytes)
}

#[cfg(target_os = "linux")]
fn mount_readonly(file: &File) -> Result<bool> {
    use std::os::fd::AsRawFd;
    let mut info = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    if unsafe { libc::fstatvfs(file.as_raw_fd(), info.as_mut_ptr()) } != 0 {
        return Err(authority("cannot inspect startup mount flags"));
    }
    Ok(unsafe { info.assume_init() }.f_flag & libc::ST_RDONLY != 0)
}

fn local_file(path: &Path, limit: u64, private: bool, frozen: bool) -> Result<Vec<u8>> {
    let parent = path
        .parent()
        .ok_or_else(|| authority("input has no parent"))?;
    super::checked_directory(parent, false).map_err(authority)?;
    #[cfg(target_os = "linux")]
    if frozen {
        frozen_ancestors(parent)?;
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK);
    }
    let file = options.open(path).map_err(authority)?;
    ordinary(&file, private)?;
    #[cfg(target_os = "linux")]
    if frozen {
        use std::os::unix::fs::MetadataExt;
        let metadata = file.metadata().map_err(authority)?;
        if !mount_readonly(&file)? && metadata.uid() == unsafe { libc::geteuid() } {
            return Err(authority("frozen input is writable by the worker"));
        }
    }
    #[cfg(not(target_os = "linux"))]
    let _ = frozen;
    bounded(file, limit)
}

#[cfg(target_os = "linux")]
fn frozen_ancestors(path: &Path) -> Result<()> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component);
        let directory = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(&current)
            .map_err(authority)?;
        let metadata = directory.metadata().map_err(authority)?;
        let uid = unsafe { libc::geteuid() };
        if (metadata.uid() != 0 && metadata.uid() != uid)
            || metadata.mode() & 0o022 != 0
            || (metadata.uid() == uid && !mount_readonly(&directory)?)
        {
            return Err(authority(
                "frozen input ancestor can be replaced by the worker or another principal",
            ));
        }
    }
    Ok(())
}

trait Delivery {
    fn read(&mut self) -> Result<Option<Vec<u8>>>;
}

#[cfg(target_os = "linux")]
struct Projection {
    directory: File,
    path: PathBuf,
    readonly: bool,
}
#[cfg(target_os = "linux")]
impl Projection {
    fn open(path: &Path, readonly: bool) -> Result<Self> {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        super::checked_directory(path, false).map_err(authority)?;
        if readonly {
            frozen_ancestors(
                path.parent()
                    .ok_or_else(|| authority("projection has no parent"))?,
            )?;
        }
        let directory = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)
            .map_err(authority)?;
        let metadata = directory.metadata().map_err(authority)?;
        let uid = unsafe { libc::geteuid() };
        if !metadata.is_dir()
            || (metadata.uid() != 0 && metadata.uid() != uid)
            || metadata.mode() & 0o022 != 0
            || (readonly && !mount_readonly(&directory)?)
        {
            return Err(authority(
                "projection directory ownership/type/read-only mount mismatch",
            ));
        }
        Ok(Self {
            directory,
            path: path.into(),
            readonly,
        })
    }
}
#[cfg(target_os = "linux")]
impl Delivery for Projection {
    fn read(&mut self) -> Result<Option<Vec<u8>>> {
        use std::os::fd::{AsRawFd, FromRawFd};
        use std::os::unix::fs::MetadataExt;
        let opened = self.directory.metadata().map_err(authority)?;
        let visible = fs::symlink_metadata(&self.path).map_err(authority)?;
        if !visible.is_dir() || visible.dev() != opened.dev() || visible.ino() != opened.ino() {
            return Err(authority("projection directory changed after opening"));
        }
        #[repr(C)]
        struct OpenHow {
            flags: u64,
            mode: u64,
            resolve: u64,
        }
        let how = OpenHow {
            flags: (libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NONBLOCK) as u64,
            mode: 0,
            resolve: 0x08 | 0x02, // BENEATH | NO_MAGICLINKS; relative symlinks allowed.
        };
        let fd = unsafe {
            libc::syscall(
                libc::SYS_openat2,
                self.directory.as_raw_fd(),
                c"bundle.json".as_ptr(),
                &how,
                std::mem::size_of::<OpenHow>(),
            )
        };
        if fd < 0 {
            let error = std::io::Error::last_os_error();
            if error.kind() == std::io::ErrorKind::NotFound {
                return Ok(None);
            }
            return Err(authority(format!(
                "confined projection open failed: {error}"
            )));
        }
        let file = unsafe { File::from_raw_fd(fd as i32) };
        ordinary(&file, false)?;
        if self.readonly && !mount_readonly(&file)? {
            return Err(authority("projected file is not on a read-only mount"));
        }
        bounded(file, BUNDLE_LIMIT).map(Some)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct GateAuthority {
    schema: String,
    config_sha256: String,
    owner: OwnerIdentity,
    host_boot_id: String,
    journal_path: PathBuf,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Waiting,
    InputsAccepted,
    ActivationStarted,
    Activated,
    GenerationStarted,
    Complete,
    Failed,
    Rejected,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GateState {
    schema: String,
    authority: GateAuthority,
    local: LocalIdentity,
    first_start_unix_seconds: u64,
    first_start_boottime_ns: u64,
    deadline_boottime_ns: u64,
    startup_deadline_unix_seconds: u64,
    phase: Phase,
    sequence: u64,
    accepted_bundle_sha256: Option<String>,
    outcome: Option<GateOutcome>,
    error: Option<GateError>,
}
struct Journal {
    connection: Connection,
    state: GateState,
    root: PathBuf,
}

fn check_private_path(path: &Path) -> Result<()> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK);
    }
    ordinary(&options.open(path).map_err(authority)?, true)
}
fn exists(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(authority(e)),
    }
}
fn create_private(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let mut file = options.open(path).map_err(persistence)?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(persistence)?;
    super::sync_dir(
        path.parent()
            .ok_or_else(|| persistence("file has no parent"))?,
    )
    .map_err(persistence)
}
fn private_directory(path: &Path) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::{DirBuilderExt, MetadataExt};
        super::checked_directory(
            path.parent()
                .ok_or_else(|| authority("directory has no parent"))?,
            true,
        )
        .map_err(authority)?;
        match fs::DirBuilder::new().mode(0o700).create(path) {
            Ok(()) => super::sync_dir(path.parent().unwrap()).map_err(persistence)?,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(authority(e)),
        }
        let metadata = fs::symlink_metadata(path).map_err(authority)?;
        if !metadata.is_dir()
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.mode() & 0o077 != 0
        {
            return Err(authority(
                "admission directory must be private and owned by this worker",
            ));
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        Err(GateError::new(
            GateErrorKind::UnsupportedPlatform,
            "startup gate requires Linux",
        ))
    }
}

impl Journal {
    fn open(
        paths: &Paths,
        resume: bool,
        authority: GateAuthority,
        local: LocalIdentity,
        config: &GateConfig,
        now: Moment,
    ) -> Result<Self> {
        let journal = paths.journal();
        if resume {
            let marker: GateAuthority =
                decode(&local_file(&paths.marker(), JOURNAL_LIMIT, true, false)?)?;
            if marker != authority {
                return Err(authority_error("retained gate authority changed"));
            }
            check_private_path(&journal)?;
        } else {
            if exists(&paths.marker())? || exists(&journal)? {
                return Err(authority_error(
                    "gate authority exists; only exact --resume is allowed",
                ));
            }
            create_private(
                &paths.marker(),
                &serde_json::to_vec(&authority).map_err(persistence)?,
            )?;
            create_private(&journal, b"")?;
        }
        for suffix in ["-journal", "-wal", "-shm"] {
            let path = PathBuf::from(format!("{}{suffix}", journal.display()));
            if exists(&path)? {
                check_private_path(&path)?;
            }
        }
        let connection = Connection::open_with_flags(&journal, OpenFlags::SQLITE_OPEN_READ_WRITE)
            .map_err(persistence)?;
        connection
            .busy_timeout(Duration::from_secs(1))
            .map_err(persistence)?;
        connection
            .execute_batch(
                "PRAGMA journal_mode=DELETE; PRAGMA synchronous=EXTRA; PRAGMA trusted_schema=OFF;",
            )
            .map_err(persistence)?;
        if resume {
            let length: u64 = connection
                .query_row(
                    "SELECT length(CAST(json AS BLOB)) FROM state WHERE id=1",
                    [],
                    |r| r.get(0),
                )
                .map_err(authority_error)?;
            if length > JOURNAL_LIMIT {
                return Err(authority_error("gate state exceeds its bound"));
            }
            let (bytes, sha): (String, String) = connection
                .query_row("SELECT json,sha FROM state WHERE id=1", [], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })
                .map_err(authority_error)?;
            if bytes.len() as u64 > JOURNAL_LIMIT || super::digest(bytes.as_bytes()) != sha {
                return Err(authority_error("gate state length/hash mismatch"));
            }
            let state: GateState = decode(bytes.as_bytes())?;
            let seconds = u64::from(config.max_wait_seconds).min(
                config
                    .startup_deadline_unix_seconds
                    .saturating_sub(state.first_start_unix_seconds),
            );
            let original_deadline = state
                .first_start_boottime_ns
                .checked_add(seconds * 1_000_000_000)
                .ok_or_else(|| authority_error("retained deadline overflow"))?;
            if state.schema != "crypto.ml-dataset.startup-state/v1"
                || state.authority != authority
                || state.local != local
                || state.startup_deadline_unix_seconds != config.startup_deadline_unix_seconds
                || state.deadline_boottime_ns != original_deadline
                || now.boottime_ns < state.first_start_boottime_ns
            {
                return Err(authority_error(
                    "retained gate state differs from original authority/inputs",
                ));
            }
            validate_state(&state)?;
            Ok(Self {
                connection,
                state,
                root: paths.admission.clone(),
            })
        } else {
            connection.execute_batch("CREATE TABLE state(id INTEGER PRIMARY KEY CHECK(id=1),json TEXT NOT NULL,sha TEXT NOT NULL);").map_err(persistence)?;
            let seconds = u64::from(config.max_wait_seconds)
                .min(config.startup_deadline_unix_seconds.saturating_sub(now.utc));
            let deadline_boottime_ns = now
                .boottime_ns
                .checked_add(seconds * 1_000_000_000)
                .ok_or_else(|| invalid("startup deadline overflow"))?;
            let state = GateState {
                schema: "crypto.ml-dataset.startup-state/v1".into(),
                authority,
                local,
                first_start_unix_seconds: now.utc,
                first_start_boottime_ns: now.boottime_ns,
                deadline_boottime_ns,
                startup_deadline_unix_seconds: config.startup_deadline_unix_seconds,
                phase: Phase::Waiting,
                sequence: 0,
                accepted_bundle_sha256: None,
                outcome: None,
                error: None,
            };
            let mut result = Self {
                connection,
                state,
                root: paths.admission.clone(),
            };
            result.persist()?;
            Ok(result)
        }
    }
    fn persist(&mut self) -> Result<()> {
        let text = serde_json::to_string(&self.state).map_err(persistence)?;
        if text.len() as u64 > JOURNAL_LIMIT {
            return Err(persistence("gate state exceeds bound"));
        }
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(persistence)?;
        tx.execute(
            "INSERT OR REPLACE INTO state(id,json,sha) VALUES(1,?1,?2)",
            params![text, super::digest(text.as_bytes())],
        )
        .map_err(persistence)?;
        tx.commit().map_err(persistence)?;
        storage::write_json(
            &self.root,
            &format!("receipts/{:04}-state.json", self.state.sequence),
            "restricted-startup-gate-state",
            &self.state,
        )
        .map_err(persistence)?;
        Ok(())
    }
    fn transition(&mut self, phase: Phase) -> Result<()> {
        self.state.phase = phase;
        self.state.sequence = self
            .state
            .sequence
            .checked_add(1)
            .ok_or_else(|| persistence("gate sequence overflow"))?;
        self.persist()
    }
    fn terminal(&self) -> Option<Result<GateOutcome>> {
        match self.state.phase {
            Phase::Complete => Some(
                self.state
                    .outcome
                    .clone()
                    .ok_or_else(|| authority_error("complete gate lacks retained outcome")),
            ),
            Phase::Failed | Phase::Rejected => {
                Some(Err(self.state.error.clone().unwrap_or_else(|| {
                    authority_error("terminal gate lacks retained error")
                })))
            }
            _ => None,
        }
    }
    fn fail(&mut self, error: GateError) -> GateError {
        self.state.error = Some(error.clone());
        self.state.outcome = None;
        let phase = match error.kind {
            GateErrorKind::ActivationFailure
            | GateErrorKind::GenerationFailure
            | GateErrorKind::GenerationOutcomeUnknown => Phase::Failed,
            _ => Phase::Rejected,
        };
        match self.transition(phase) {
            Ok(()) => error,
            Err(save) => persistence(format!(
                "{error}; could not retain terminal evidence: {save}"
            )),
        }
    }
}
// Distinct name avoids shadowing the authority parameter in Journal::open.
fn authority_error(message: impl ToString) -> GateError {
    authority(message)
}

fn validate_state(state: &GateState) -> Result<()> {
    if let Some(hash) = &state.accepted_bundle_sha256 {
        sha(hash)?;
    }
    let accepted = state.accepted_bundle_sha256.is_some();
    let valid = match state.phase {
        Phase::Waiting => {
            !accepted && state.sequence == 0 && state.error.is_none() && state.outcome.is_none()
        }
        Phase::InputsAccepted
        | Phase::ActivationStarted
        | Phase::Activated
        | Phase::GenerationStarted => accepted && state.error.is_none() && state.outcome.is_none(),
        Phase::Complete => {
            accepted
                && state.error.is_none()
                && state.outcome.as_ref().is_some_and(|o| {
                    o.schema == "crypto.ml-dataset.startup-outcome/v1"
                        && o.status == "complete"
                        && o.owner == state.authority.owner
                        && o.config_sha256 == state.authority.config_sha256
                        && Some(&o.accepted_bundle_sha256) == state.accepted_bundle_sha256.as_ref()
                        && o.generation_handler_returned_success
                        && !o.kubernetes_attestation_performed
                })
        }
        Phase::Failed | Phase::Rejected => state.error.is_some() && state.outcome.is_none(),
    };
    if !valid {
        return Err(authority("inconsistent retained gate phase/evidence"));
    }
    Ok(())
}

fn immutable_inputs(
    paths: &Paths,
    config: &GateConfig,
    prepared: &PreparedRun,
    bytes: &[u8],
    initial: bool,
) -> Result<()> {
    let pretty = |value: serde_json::Value| -> Result<Vec<u8>> {
        let mut bytes = serde_json::to_vec_pretty(&value).map_err(invalid)?;
        bytes.push(b'\n');
        Ok(bytes)
    };
    let inputs = [
        ("bundle.json", bytes.to_vec()),
        (
            "policy.json",
            pretty(serde_json::to_value(&prepared.bundle.policy).map_err(invalid)?)?,
        ),
        (
            "grant.json",
            pretty(serde_json::to_value(&prepared.bundle.grant).map_err(invalid)?)?,
        ),
        (
            "owner.json",
            pretty(serde_json::to_value(&prepared.owner).map_err(invalid)?)?,
        ),
        (
            "cell-envelope.json",
            pretty(serde_json::to_value(&config.generate.cell_envelope).map_err(invalid)?)?,
        ),
    ];
    for (name, data) in inputs {
        let path = paths.admission.join(name);
        if exists(&path)? {
            check_private_path(&path)?;
            storage::check_file(&path, &super::digest(&data), data.len() as u64)
                .map_err(authority)?;
        } else if !initial {
            return Err(authority("accepted local input disappeared"));
        }
        if initial {
            storage::write_bytes(
                &paths.admission,
                name,
                "restricted-startup-accepted-input",
                "json",
                &data,
            )
            .map_err(persistence)?;
        }
    }
    Ok(())
}

trait Driver {
    fn ensure_budget_absent(&mut self, prepared: &PreparedRun) -> Result<()>;
    fn phase(&mut self, prepared: &PreparedRun) -> Result<ActivationPhase>;
    fn activate(&mut self, prepared: &PreparedRun, resume: bool) -> Result<()>;
    fn generate(&mut self, prepared: &PreparedRun) -> Result<()>;
}
struct NativeDriver;
impl Driver for NativeDriver {
    fn ensure_budget_absent(&mut self, prepared: &PreparedRun) -> Result<()> {
        let state = prepared
            .generate
            .budget
            .budget_state
            .as_ref()
            .ok_or_else(|| authority("missing internal budget path"))?;
        private_directory(
            state
                .parent()
                .ok_or_else(|| authority("budget path has no parent"))?,
        )?;
        for suffix in ["", ".authority", ".lock", "-journal", "-wal", "-shm"] {
            if exists(&PathBuf::from(format!("{}{suffix}", state.display())))? {
                return Err(authority(
                    "unexpected worker budget state before initial activation",
                ));
            }
        }
        Ok(())
    }
    fn phase(&mut self, prepared: &PreparedRun) -> Result<ActivationPhase> {
        let state = prepared
            .generate
            .budget
            .budget_state
            .as_ref()
            .ok_or_else(|| authority("missing internal budget path"))?;
        // Do not repair a disappeared parent or relax the original private mode.
        if !exists(
            state
                .parent()
                .ok_or_else(|| authority("budget path has no parent"))?,
        )? {
            return Err(authority("retained budget directory disappeared"));
        }
        private_directory(state.parent().unwrap())?;
        for suffix in ["", ".authority", ".lock", "-journal", "-wal", "-shm"] {
            let path = PathBuf::from(format!("{}{suffix}", state.display()));
            if matches!(suffix, "" | ".authority" | ".lock") || exists(&path)? {
                check_private_path(&path)?;
            }
        }
        let session = BudgetSession::open(
            state,
            &prepared.bundle.policy,
            &prepared.bundle.grant,
            &prepared.owner,
        )
        .map_err(authority)?;
        super::validate_budget_actor(&session).map_err(authority)?;
        let phase = session.activation_phase().map_err(authority)?;
        if matches!(phase, ActivationPhase::Active) {
            session.require_active().map_err(authority)?;
        }
        // session/lease is dropped here, before a separate native handler opens it.
        Ok(phase)
    }
    fn activate(&mut self, prepared: &PreparedRun, resume: bool) -> Result<()> {
        super::run_budget_command(&BudgetCommand::Activate {
            budget: prepared.generate.budget.clone(),
            resume,
        })
        .map(|_| ())
        .map_err(|e| GateError::new(GateErrorKind::ActivationFailure, e))
    }
    fn generate(&mut self, prepared: &PreparedRun) -> Result<()> {
        super::generate(&prepared.generate)
            .map_err(|e| GateError::new(GateErrorKind::GenerationFailure, e))
    }
}

fn check_wait(state: &GateState, now: Moment) -> Result<u64> {
    if now.utc >= state.startup_deadline_unix_seconds
        || now.boottime_ns >= state.deadline_boottime_ns
    {
        return Err(GateError::new(
            GateErrorKind::MissingInputTimeout,
            "original startup deadline expired",
        ));
    }
    Ok((state.deadline_boottime_ns - now.boottime_ns).min(POLL_NANOS))
}

fn drive(
    journal: &mut Journal,
    config: &GateConfig,
    local: &VerifiedLocalInputs,
    delivery: &mut impl Delivery,
    clock: &mut impl Clock,
    driver: &mut impl Driver,
) -> Result<GateOutcome> {
    if let Some(result) = journal.terminal() {
        return result;
    }
    if journal.state.phase == Phase::GenerationStarted {
        return Err(journal.fail(GateError::new(
            GateErrorKind::GenerationOutcomeUnknown,
            "generation was already started; reconcile its outcome separately",
        )));
    }
    let result = (|| {
        let initial = journal.state.phase == Phase::Waiting;
        let bytes = if initial {
            loop {
                let remaining = check_wait(&journal.state, clock.now()?)?;
                if let Some(bytes) = delivery.read()? {
                    break bytes;
                }
                clock.sleep(remaining);
            }
        } else {
            let accepted = local_file(
                &local.paths.admission.join("bundle.json"),
                BUNDLE_LIMIT,
                true,
                false,
            )?;
            if Some(super::digest(&accepted)) != journal.state.accepted_bundle_sha256 {
                return Err(authority("accepted bundle differs from gate journal"));
            }
            if let Some(projected) = delivery.read()? {
                if projected != accepted {
                    return Err(authority("visible projection changed after acceptance"));
                }
            }
            accepted
        };
        let bundle: GateBundle = decode(&bytes)?;
        let prepared = validate_delivery(config, &bundle, &journal.state.authority.owner, local)?;
        if initial {
            check_wait(&journal.state, clock.now()?)?;
        }
        immutable_inputs(&local.paths, config, &prepared, &bytes, initial)?;
        if initial {
            journal.state.accepted_bundle_sha256 = Some(super::digest(&bytes));
            journal.transition(Phase::InputsAccepted)?;
        }
        match journal.state.phase {
            Phase::InputsAccepted => {
                driver.ensure_budget_absent(&prepared)?;
                journal.transition(Phase::ActivationStarted)?;
                driver.activate(&prepared, false)?;
            }
            Phase::ActivationStarted => {
                match driver.phase(&prepared)? {
                    ActivationPhase::Active => {} // Already paid/verified: no SDK/provider call.
                    ActivationPhase::Pending => driver.activate(&prepared, true)?,
                }
            }
            Phase::Activated => {}
            _ => return Err(authority("unexpected nonterminal gate phase")),
        }
        if !matches!(driver.phase(&prepared)?, ActivationPhase::Active) {
            return Err(authority(
                "activation handoff did not produce an active session",
            ));
        }
        if journal.state.phase != Phase::Activated {
            journal.transition(Phase::Activated)?;
        }
        journal.transition(Phase::GenerationStarted)?;
        driver.generate(&prepared)?;
        let outcome = GateOutcome {
            schema: "crypto.ml-dataset.startup-outcome/v1".into(),
            status: "complete".into(),
            owner: journal.state.authority.owner.clone(),
            config_sha256: journal.state.authority.config_sha256.clone(),
            accepted_bundle_sha256: journal
                .state
                .accepted_bundle_sha256
                .clone()
                .ok_or_else(|| authority("missing accepted bundle hash"))?,
            generation_handler_returned_success: true,
            kubernetes_attestation_performed: false,
        };
        journal.state.outcome = Some(outcome.clone());
        journal.transition(Phase::Complete)?;
        Ok(outcome)
    })();
    result.map_err(|error| journal.fail(error))
}

fn local_inputs(
    paths: &Paths,
) -> Result<(
    GateConfig,
    Vec<u8>,
    VerifiedLocalInputs,
    OwnerIdentity,
    String,
)> {
    let config_bytes = local_file(&paths.config, CONFIG_LIMIT, false, true)?;
    let config: GateConfig = decode(&config_bytes)?;
    validate_config(&config)?;
    let plan_bytes = local_file(&paths.plan, super::JSON_INPUT_MAX_BYTES, false, true)?;
    let plan: DatasetPlan = decode(&plan_bytes)?;
    plan.validate().map_err(invalid)?;
    let protocol = local_file(&paths.protocol, super::JSON_INPUT_MAX_BYTES, false, true)?;
    super::verify_protocol(&plan, &protocol).map_err(invalid)?;
    let owner = OwnerIdentity {
        job_uid: std::env::var("JOB_CONTROLLER_UID").map_err(invalid)?,
        pod_uid: std::env::var("POD_UID").map_err(invalid)?,
    };
    identifier(&owner.job_uid)?;
    identifier(&owner.pod_uid)?;
    let namespace = std::env::var("POD_NAMESPACE").map_err(invalid)?;
    let pod_name = std::env::var("POD_NAME").map_err(invalid)?;
    dns_label(&namespace)?;
    self::pod_name(&pod_name)?;
    let mut boot = String::new();
    File::open("/proc/sys/kernel/random/boot_id")
        .and_then(|f| f.take(129).read_to_string(&mut boot))
        .map_err(authority)?;
    let boot = boot.trim().to_owned();
    identifier(&boot)?;
    let identity = LocalIdentity {
        source_commit: super::generation_source().map_err(invalid)?.into(),
        binary_sha256: crate::store::hash_file(&std::env::current_exe().map_err(invalid)?)
            .map_err(invalid)?
            .0,
        plan_sha256: plan.binding_sha256().map_err(invalid)?,
        plan_file_sha256: super::digest(&plan_bytes),
        protocol_sha256: super::digest(&protocol),
        namespace,
        pod_name,
    };
    let local = VerifiedLocalInputs {
        identity,
        paths: paths.clone(),
        lab_receipt_uri: std::env::var("ISOLAB_RESULTS_URI")
            .ok()
            .filter(|s| !s.is_empty()),
    };
    Ok((config, config_bytes, local, owner, boot))
}

pub(super) fn start(resume: bool) -> Result<GateOutcome> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = resume;
        Err(GateError::new(
            GateErrorKind::UnsupportedPlatform,
            "startup gate requires Linux openat2 and CLOCK_BOOTTIME",
        ))
    }
    #[cfg(target_os = "linux")]
    {
        let paths = Paths::production();
        let (config, bytes, local, owner, boot) = local_inputs(&paths)?;
        private_directory(&paths.admission)?;
        let lock = paths.admission.join(".writer.lock");
        if exists(&lock)? {
            check_private_path(&lock)?;
        }
        let _lease = super::Lease::acquire(&paths.admission).map_err(authority)?;
        check_private_path(&lock)?;
        let mut clock = NativeClock;
        let marker = GateAuthority {
            schema: "crypto.ml-dataset.startup-authority/v1".into(),
            config_sha256: super::digest(&bytes),
            owner,
            host_boot_id: boot,
            journal_path: paths.journal(),
        };
        let mut journal = Journal::open(
            &paths,
            resume,
            marker,
            local.identity.clone(),
            &config,
            clock.now()?,
        )?;
        if let Some(result) = journal.terminal() {
            return result;
        }
        let mut delivery =
            Projection::open(&paths.projection, true).map_err(|e| journal.fail(e))?;
        drive(
            &mut journal,
            &config,
            &local,
            &mut delivery,
            &mut clock,
            &mut NativeDriver,
        )
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::VecDeque;
    use std::os::unix::fs::{symlink, PermissionsExt};

    struct Fixture {
        root: PathBuf,
        paths: Paths,
        config: GateConfig,
        local: VerifiedLocalInputs,
        owner: OwnerIdentity,
        bundle: GateBundle,
        bytes: Vec<u8>,
        marker: GateAuthority,
        now: Moment,
    }
    impl Fixture {
        fn new() -> Self {
            Self::with_finalization(None, false)
        }
        fn with_finalization(finalization: Option<Resources>, irsa: bool) -> Self {
            let root = std::env::temp_dir().join(format!(
                "dataset-startup-test-{}-{}",
                std::process::id(),
                super::super::unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            let paths = Paths {
                config: root.join("image/startup-gate.json"),
                plan: root.join("image/plan.json"),
                protocol: root.join("image/protocol.json"),
                projection: root.join("projection"),
                admission: root.join("admission"),
                worker: root.join("budget/worker.sqlite"),
                dataset: root.join("dataset"),
            };
            fs::create_dir(root.join("image")).unwrap();
            fs::create_dir(&paths.projection).unwrap();
            private_directory(&paths.admission).unwrap();
            let now = Moment {
                utc: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                boottime_ns: 100_000_000_000,
            };
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"startup software correctness fixture"});
            let body_hash =
                super::super::digest(&super::super::protocol_canonical_bytes(&protocol).unwrap());
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":body_hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-29T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![7],
                arities: vec![2],
                families: vec!["koblitz".into()],
                solvers: vec!["bsgs".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 19,
                factor_base_size: 4,
                degree_bound: 3,
                work_limit: 64,
                twists: true,
                protocol_sha256: Some(super::super::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let plan_bytes = serde_json::to_vec_pretty(&plan).unwrap();
            fs::write(&paths.plan, &plan_bytes).unwrap();
            fs::write(&paths.protocol, &protocol).unwrap();
            let (policy, mut request) = budget::test_policy_request(&root);
            request.worker_journal_path = paths.worker.to_str().unwrap().into();
            request.binding.plan_sha256 = plan.binding_sha256().unwrap();
            request.binding.protocol_sha256 = plan.protocol_sha256.clone().unwrap();
            request.finalization.output_bytes = 80 * 1024;
            request.finalization.download_bytes = 512 * 1024;
            if let Some(finalization) = finalization {
                request.finalization = finalization;
            }
            if irsa {
                request.credential_role_arn =
                    Some("arn:aws:iam::104186811093:role/dataset-startup-test".into());
            }
            let owner = request.owner.clone();
            let ledger =
                budget::CampaignLedger::create(&root.join("campaign.sqlite"), &policy).unwrap();
            let reservation = ledger
                .reserve_launch(&budget::LaunchRequest {
                    schema: budget::LAUNCH_REQUEST_SCHEMA.into(),
                    launch_id: request.grant_id,
                    target: budget::LaunchTarget {
                        cluster_identity: "arn:aws:eks:us-east-2:104186811093:cluster/startup-test"
                            .into(),
                        namespace: "isolab".into(),
                        job_name: "startup-test".into(),
                    },
                    worker_journal_path: request.worker_journal_path,
                    binding: request.binding.clone(),
                    expires_unix_seconds: request.expires_unix_seconds,
                    resource_envelope_sha256: request.resource_envelope_sha256,
                    resources: request.resources,
                    finalization: request.finalization,
                    credential_role_arn: request.credential_role_arn,
                })
                .unwrap();
            let binding = LaunchBinding {
                schema: budget::LAUNCH_BINDING_SCHEMA.into(),
                launch_id: reservation.request.launch_id.clone(),
                reservation_sha256: reservation.sha256().unwrap(),
                observed_target: reservation.request.target.clone(),
                owner: owner.clone(),
                observation_evidence_sha256: "e".repeat(64),
            };
            let grant = ledger.bind_launch(&binding).unwrap();
            let bundle = GateBundle {
                schema: BUNDLE_SCHEMA.into(),
                policy,
                reservation,
                binding,
                grant,
            };
            let config = GateConfig {
                schema: CONFIG_SCHEMA.into(),
                policy_sha256: bundle.policy.sha256().unwrap(),
                reservation_sha256: bundle.reservation.sha256().unwrap(),
                startup_deadline_unix_seconds: now.utc + 20,
                max_wait_seconds: 3,
                generate: GateGenerate {
                    max_cases: 1,
                    seconds_per_case: 1,
                    memory_mib: 128,
                    trace_mib: 1,
                    remote: "s3://setec-astronomy-artifacts/ml-dataset/test/data".into(),
                    lab_receipt_prefix: None,
                    cell_envelope: Resources {
                        output_bytes: 4096,
                        put_attempts: 2,
                        read_attempts: 4,
                        download_bytes: 8192,
                        local_bytes: 4096,
                        ..Resources::default()
                    },
                },
            };
            let config_bytes = serde_json::to_vec(&config).unwrap();
            fs::write(&paths.config, &config_bytes).unwrap();
            let identity = LocalIdentity {
                source_commit: request.binding.source_commit,
                binary_sha256: request.binding.binary_sha256,
                plan_sha256: plan.binding_sha256().unwrap(),
                plan_file_sha256: super::super::digest(&plan_bytes),
                protocol_sha256: super::super::digest(&protocol),
                namespace: "isolab".into(),
                pod_name: "startup-test-pod".into(),
            };
            let marker = GateAuthority {
                schema: "crypto.ml-dataset.startup-authority/v1".into(),
                config_sha256: super::super::digest(&config_bytes),
                owner: owner.clone(),
                host_boot_id: "test-boot".into(),
                journal_path: paths.journal(),
            };
            let local = VerifiedLocalInputs {
                identity,
                lab_receipt_uri: None,
                paths: paths.clone(),
            };
            let bytes = serde_json::to_vec_pretty(&bundle).unwrap();
            Self {
                root,
                paths,
                config,
                local,
                owner,
                bundle,
                bytes,
                marker,
                now,
            }
        }
        fn journal(&self, resume: bool) -> Journal {
            Journal::open(
                &self.paths,
                resume,
                self.marker.clone(),
                self.local.identity.clone(),
                &self.config,
                self.now,
            )
            .unwrap()
        }
        fn accept(&self, journal: &mut Journal, phase: Phase) {
            let prepared =
                validate_delivery(&self.config, &self.bundle, &self.owner, &self.local).unwrap();
            immutable_inputs(&self.paths, &self.config, &prepared, &self.bytes, true).unwrap();
            journal.state.accepted_bundle_sha256 = Some(super::super::digest(&self.bytes));
            for next in [
                Phase::InputsAccepted,
                Phase::ActivationStarted,
                Phase::Activated,
                Phase::GenerationStarted,
            ] {
                journal.transition(next).unwrap();
                if next == phase {
                    break;
                }
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    struct TestClock {
        now: Moment,
        slept: u64,
    }
    impl Clock for TestClock {
        fn now(&self) -> Result<Moment> {
            Ok(self.now)
        }
        fn sleep(&mut self, nanos: u64) {
            self.slept += nanos;
            self.now.boottime_ns += nanos;
        }
    }
    struct TestDelivery {
        queue: VecDeque<Option<Vec<u8>>>,
        reads: usize,
    }
    impl TestDelivery {
        fn one(bytes: Vec<u8>) -> Self {
            Self {
                queue: VecDeque::from([Some(bytes)]),
                reads: 0,
            }
        }
        fn absent() -> Self {
            Self {
                queue: VecDeque::new(),
                reads: 0,
            }
        }
    }
    impl Delivery for TestDelivery {
        fn read(&mut self) -> Result<Option<Vec<u8>>> {
            self.reads += 1;
            Ok(self.queue.pop_front().flatten())
        }
    }
    #[derive(Default)]
    struct TestDriver {
        active: Option<bool>,
        actions: Vec<String>,
        fail_generation: bool,
        crash_after_activation: bool,
    }
    impl Driver for TestDriver {
        fn ensure_budget_absent(&mut self, _: &PreparedRun) -> Result<()> {
            if self.active.is_some() {
                Err(authority("unexpected budget"))
            } else {
                Ok(())
            }
        }
        fn phase(&mut self, _: &PreparedRun) -> Result<ActivationPhase> {
            match self.active {
                Some(true) => Ok(ActivationPhase::Active),
                Some(false) => Ok(ActivationPhase::Pending),
                None => Err(authority("missing budget authority")),
            }
        }
        fn activate(&mut self, _: &PreparedRun, resume: bool) -> Result<()> {
            self.actions.push(format!("activate:{resume}"));
            self.active = Some(true);
            assert!(
                !self.crash_after_activation,
                "simulated crash after durable activation"
            );
            Ok(())
        }
        fn generate(&mut self, _: &PreparedRun) -> Result<()> {
            self.actions.push("generate".into());
            if self.fail_generation {
                Err(GateError::new(
                    GateErrorKind::GenerationFailure,
                    "retained native failure",
                ))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn late_delivery_is_accepted_once_and_terminal_resume_does_no_work() {
        let f = Fixture::new();
        let mut journal = f.journal(false);
        let mut clock = TestClock {
            now: f.now,
            slept: 0,
        };
        let mut delivery = TestDelivery {
            queue: VecDeque::from([None, None, Some(f.bytes.clone())]),
            reads: 0,
        };
        let mut driver = TestDriver::default();
        let outcome = drive(
            &mut journal,
            &f.config,
            &f.local,
            &mut delivery,
            &mut clock,
            &mut driver,
        )
        .unwrap();
        assert_eq!(driver.actions, ["activate:false", "generate"]);
        assert_eq!(clock.slept, 2 * POLL_NANOS);
        drop(journal);
        let mut reopened = f.journal(true);
        let mut no_delivery = TestDelivery::absent();
        let mut no_driver = TestDriver::default();
        assert_eq!(
            drive(
                &mut reopened,
                &f.config,
                &f.local,
                &mut no_delivery,
                &mut clock,
                &mut no_driver
            )
            .unwrap(),
            outcome
        );
        assert!(no_driver.actions.is_empty());
        assert_eq!(no_delivery.reads, 0);
    }

    #[test]
    fn recovery_after_activation_commit_does_not_repeat_sdk_handoff() {
        let f = Fixture::new();
        let mut journal = f.journal(false);
        let mut clock = TestClock {
            now: f.now,
            slept: 0,
        };
        let mut delivery = TestDelivery::one(f.bytes.clone());
        let mut driver = TestDriver {
            crash_after_activation: true,
            ..TestDriver::default()
        };
        let crash = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            drive(
                &mut journal,
                &f.config,
                &f.local,
                &mut delivery,
                &mut clock,
                &mut driver,
            )
        }));
        assert!(crash.is_err());
        assert_eq!(journal.state.phase, Phase::ActivationStarted);
        drop(journal);
        let mut resumed = f.journal(true);
        driver.crash_after_activation = false;
        drive(
            &mut resumed,
            &f.config,
            &f.local,
            &mut TestDelivery::absent(),
            &mut clock,
            &mut driver,
        )
        .unwrap();
        assert_eq!(driver.actions, ["activate:false", "generate"]);
    }

    #[test]
    fn pending_activation_resumes_but_missing_authority_and_started_generation_stop() {
        for (phase, budget, expected) in [
            (
                Phase::InputsAccepted,
                None,
                vec!["activate:false", "generate"],
            ),
            (
                Phase::ActivationStarted,
                Some(false),
                vec!["activate:true", "generate"],
            ),
            (Phase::Activated, Some(true), vec!["generate"]),
            (Phase::ActivationStarted, None, vec![]),
            (Phase::GenerationStarted, Some(true), vec![]),
        ] {
            let f = Fixture::new();
            let mut journal = f.journal(false);
            f.accept(&mut journal, phase);
            drop(journal);
            let mut journal = f.journal(true);
            let mut driver = TestDriver {
                active: budget,
                ..TestDriver::default()
            };
            let mut clock = TestClock {
                now: f.now,
                slept: 0,
            };
            let result = drive(
                &mut journal,
                &f.config,
                &f.local,
                &mut TestDelivery::absent(),
                &mut clock,
                &mut driver,
            );
            assert_eq!(driver.actions, expected);
            if phase == Phase::GenerationStarted {
                assert_eq!(
                    result.unwrap_err().kind,
                    GateErrorKind::GenerationOutcomeUnknown
                );
            } else if phase == Phase::ActivationStarted && budget.is_none() {
                assert_eq!(result.unwrap_err().kind, GateErrorKind::AuthorityChanged);
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn malformed_delivery_and_failed_generation_remain_terminal() {
        for malformed in [true, false] {
            let f = Fixture::new();
            let mut journal = f.journal(false);
            let mut clock = TestClock {
                now: f.now,
                slept: 0,
            };
            let mut delivery = TestDelivery::one(if malformed {
                b"{not valid JSON".to_vec()
            } else {
                f.bytes.clone()
            });
            let mut driver = TestDriver {
                fail_generation: true,
                ..TestDriver::default()
            };
            let error = drive(
                &mut journal,
                &f.config,
                &f.local,
                &mut delivery,
                &mut clock,
                &mut driver,
            )
            .unwrap_err();
            if malformed {
                assert!(driver.actions.is_empty());
            } else {
                assert_eq!(driver.actions, ["activate:false", "generate"]);
            }
            drop(journal);
            let mut resumed = f.journal(true);
            let mut untouched = TestDriver::default();
            assert_eq!(
                drive(
                    &mut resumed,
                    &f.config,
                    &f.local,
                    &mut TestDelivery::one(f.bytes.clone()),
                    &mut clock,
                    &mut untouched
                )
                .unwrap_err(),
                error
            );
            assert!(untouched.actions.is_empty());
        }
    }

    #[test]
    fn wait_deadline_does_not_renew_or_follow_backwards_wall_clock() {
        let f = Fixture::new();
        let journal = f.journal(false);
        let deadline = journal.state.deadline_boottime_ns;
        drop(journal);
        assert!(Journal::open(
            &f.paths,
            false,
            f.marker.clone(),
            f.local.identity.clone(),
            &f.config,
            f.now
        )
        .is_err());
        let now = Moment {
            utc: f.now.utc - 3600,
            boottime_ns: deadline,
        };
        let mut resumed = Journal::open(
            &f.paths,
            true,
            f.marker.clone(),
            f.local.identity.clone(),
            &f.config,
            now,
        )
        .unwrap();
        assert_eq!(resumed.state.deadline_boottime_ns, deadline);
        let mut clock = TestClock { now, slept: 0 };
        let mut delivery = TestDelivery::absent();
        let mut driver = TestDriver::default();
        let error = drive(
            &mut resumed,
            &f.config,
            &f.local,
            &mut delivery,
            &mut clock,
            &mut driver,
        )
        .unwrap_err();
        assert_eq!(error.kind, GateErrorKind::MissingInputTimeout);
        assert_eq!(delivery.reads, 0);
        assert!(driver.actions.is_empty());
        assert!(check_wait(
            &resumed.state,
            Moment {
                utc: f.config.startup_deadline_unix_seconds,
                boottime_ns: f.now.boottime_ns
            }
        )
        .is_err());
    }

    #[test]
    fn bootstrap_headroom_is_required_before_any_driver_activity() {
        let generator_floor = Resources {
            output_bytes: 64 * 1024,
            put_attempts: 2,
            read_attempts: 4,
            download_bytes: 256 * 1024,
            ..Resources::default()
        };
        let static_minimum = generator_floor
            .checked_add(&Resources {
                output_bytes: 4096,
                put_attempts: 1,
                read_attempts: 1,
                download_bytes: 4096,
                ..Resources::default()
            })
            .unwrap();
        for (finalization, irsa) in [(generator_floor, false), (static_minimum.clone(), true)] {
            let f = Fixture::with_finalization(Some(finalization), irsa);
            let mut journal = f.journal(false);
            let mut delivery = TestDelivery::one(f.bytes.clone());
            let mut driver = TestDriver::default();
            let mut clock = TestClock {
                now: f.now,
                slept: 0,
            };
            let error = drive(
                &mut journal,
                &f.config,
                &f.local,
                &mut delivery,
                &mut clock,
                &mut driver,
            )
            .unwrap_err();
            assert_eq!(error.kind, GateErrorKind::InvalidInput);
            assert!(driver.actions.is_empty());
            assert_eq!(journal.state.phase, Phase::Rejected);
        }
        let f = Fixture::with_finalization(Some(static_minimum.clone()), false);
        validate_delivery(&f.config, &f.bundle, &f.owner, &f.local).unwrap();
        let irsa_minimum = static_minimum
            .checked_add(&Resources {
                read_attempts: 2,
                download_bytes: 2 * 65536,
                ..Resources::default()
            })
            .unwrap();
        let f = Fixture::with_finalization(Some(irsa_minimum), true);
        validate_delivery(&f.config, &f.bundle, &f.owner, &f.local).unwrap();
    }

    #[test]
    fn owner_build_hash_route_and_all_resource_coordinates_are_checked() {
        let f = Fixture::new();
        validate_delivery(&f.config, &f.bundle, &f.owner, &f.local).unwrap();
        let mut wrong = f.owner.clone();
        wrong.pod_uid = "another-pod".into();
        assert!(validate_delivery(&f.config, &f.bundle, &wrong, &f.local).is_err());
        for field in [
            "source_commit",
            "binary_sha256",
            "plan_sha256",
            "protocol_sha256",
            "namespace",
        ] {
            let mut identity = serde_json::to_value(&f.local.identity).unwrap();
            identity[field] = json!("incorrect");
            let local = VerifiedLocalInputs {
                identity: serde_json::from_value(identity).unwrap(),
                lab_receipt_uri: None,
                paths: f.paths.clone(),
            };
            assert!(
                validate_delivery(&f.config, &f.bundle, &f.owner, &local).is_err(),
                "{field}"
            );
        }
        assert_ne!(
            f.local.identity.plan_sha256,
            f.local.identity.plan_file_sha256
        );
        let mut file_digest = f.local.identity.clone();
        file_digest.plan_sha256 = file_digest.plan_file_sha256.clone();
        assert!(validate_delivery(
            &f.config,
            &f.bundle,
            &f.owner,
            &VerifiedLocalInputs {
                identity: file_digest,
                lab_receipt_uri: None,
                paths: f.paths.clone()
            }
        )
        .is_err());
        for field in [
            "money_micro_usd",
            "output_bytes",
            "put_attempts",
            "read_attempts",
            "download_bytes",
            "local_bytes",
        ] {
            let mut config = serde_json::to_value(&f.config).unwrap();
            config["generate"]["cell_envelope"][field] = json!(u64::MAX);
            assert!(
                validate_delivery(
                    &decode::<GateConfig>(&serde_json::to_vec(&config).unwrap()).unwrap(),
                    &f.bundle,
                    &f.owner,
                    &f.local
                )
                .is_err(),
                "{field}"
            );
        }
        let mut config = f.config.clone();
        config.generate.lab_receipt_prefix =
            Some("s3://setec-astronomy-artifacts/ml-dataset/test/receipts/".into());
        let mut local = VerifiedLocalInputs {
            identity: f.local.identity.clone(),
            lab_receipt_uri: Some(format!(
                "s3://setec-astronomy-artifacts/ml-dataset/test/receipts/startup-test/{}/",
                f.owner.pod_uid
            )),
            paths: f.paths.clone(),
        };
        validate_delivery(&config, &f.bundle, &f.owner, &local).unwrap();
        local.lab_receipt_uri = Some("s3://setec-astronomy-artifacts/unapproved/".into());
        assert!(validate_delivery(&config, &f.bundle, &f.owner, &local).is_err());
        assert!(validate_delivery(&f.config, &f.bundle, &f.owner, &local).is_err());
    }

    #[test]
    fn accepted_bundle_change_and_missing_authority_never_restart() {
        let f = Fixture::new();
        let mut journal = f.journal(false);
        f.accept(&mut journal, Phase::ActivationStarted);
        drop(journal);
        let mut resumed = f.journal(true);
        let mut changed = f.bytes.clone();
        changed.push(b'\n');
        let mut driver = TestDriver {
            active: Some(true),
            ..TestDriver::default()
        };
        let mut clock = TestClock {
            now: f.now,
            slept: 0,
        };
        assert!(drive(
            &mut resumed,
            &f.config,
            &f.local,
            &mut TestDelivery::one(changed),
            &mut clock,
            &mut driver
        )
        .is_err());
        assert!(driver.actions.is_empty());
        drop(resumed);
        fs::remove_file(f.paths.marker()).unwrap();
        assert!(Journal::open(
            &f.paths,
            true,
            f.marker.clone(),
            f.local.identity.clone(),
            &f.config,
            f.now
        )
        .is_err());
        assert!(Journal::open(
            &f.paths,
            false,
            f.marker.clone(),
            f.local.identity.clone(),
            &f.config,
            f.now
        )
        .is_err());
    }

    #[test]
    fn local_authority_rejects_aliases_and_concurrent_owners() {
        let f = Fixture::new();
        let journal = f.journal(false);
        drop(journal);
        fs::hard_link(f.paths.marker(), f.root.join("marker-alias")).unwrap();
        assert!(Journal::open(
            &f.paths,
            true,
            f.marker.clone(),
            f.local.identity.clone(),
            &f.config,
            f.now
        )
        .is_err());
        fs::remove_file(f.root.join("marker-alias")).unwrap();
        let _lease = super::super::Lease::acquire(&f.paths.admission).unwrap();
        assert!(super::super::Lease::acquire(&f.paths.admission).is_err());
        let file = f.root.join("readonly-looking.json");
        fs::write(&file, b"{}").unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o444)).unwrap();
        assert!(local_file(&file, 64, false, true).is_err());
    }

    #[test]
    fn projection_accepts_confined_atomic_symlinks_and_rejects_escapes_and_oversize() {
        let f = Fixture::new();
        let root = &f.paths.projection;
        fs::create_dir(root.join("v1")).unwrap();
        fs::write(root.join("v1/bundle.json"), b"first").unwrap();
        symlink("v1", root.join("..data")).unwrap();
        symlink("..data/bundle.json", root.join("bundle.json")).unwrap();
        let mut reader = Projection::open(root, false).unwrap();
        assert_eq!(reader.read().unwrap().unwrap(), b"first");
        fs::create_dir(root.join("v2")).unwrap();
        fs::write(root.join("v2/bundle.json"), b"second").unwrap();
        symlink("v2", root.join("..new")).unwrap();
        fs::rename(root.join("..new"), root.join("..data")).unwrap();
        assert_eq!(reader.read().unwrap().unwrap(), b"second");
        fs::remove_file(root.join("bundle.json")).unwrap();
        assert!(reader.read().unwrap().is_none());
        symlink("../image/plan.json", root.join("bundle.json")).unwrap();
        assert!(reader.read().is_err());
        fs::remove_file(root.join("bundle.json")).unwrap();
        symlink(&f.paths.plan, root.join("bundle.json")).unwrap();
        assert!(reader.read().is_err());
        fs::remove_file(root.join("bundle.json")).unwrap();
        let file = File::create(root.join("bundle.json")).unwrap();
        file.set_len(BUNDLE_LIMIT + 1).unwrap();
        assert!(reader.read().is_err());
        fs::remove_file(root.join("bundle.json")).unwrap();
        fs::create_dir(root.join("bundle.json")).unwrap();
        assert!(reader.read().is_err());
    }

    #[test]
    fn typed_input_rejects_duplicate_unknown_and_invalid_wait_fields() {
        let f = Fixture::new();
        let bytes = serde_json::to_string(&f.config).unwrap();
        let duplicate = bytes.replacen('{', "{\"schema\":\"duplicate\",", 1);
        assert!(decode::<GateConfig>(duplicate.as_bytes()).is_err());
        let mut value = serde_json::to_value(&f.config).unwrap();
        value["unexpected"] = json!(true);
        assert!(decode::<GateConfig>(&serde_json::to_vec(&value).unwrap()).is_err());
        let mut config = f.config.clone();
        config.max_wait_seconds = 301;
        assert!(validate_config(&config).is_err());
        config.max_wait_seconds = 0;
        assert!(validate_config(&config).is_err());
        let duplicate = String::from_utf8(f.bytes.clone()).unwrap().replacen(
            "\"policy\": {",
            "\"policy\": {\"schema\":\"duplicate\",",
            1,
        );
        assert!(decode::<GateBundle>(duplicate.as_bytes()).is_err());
    }
}
