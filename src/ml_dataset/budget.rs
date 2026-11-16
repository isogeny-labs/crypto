//! Durable campaign allocations and conservative per-attempt accounting.
//!
//! SQLite IMMEDIATE transactions serialize admission; DELETE journals with EXTRA
//! synchronization make successful commits durable before a network attempt.
//! See https://www.sqlite.org/lang_transaction.html and
//! https://www.sqlite.org/pragma.html#pragma_synchronous. This is a cooperative
//! campaign ledger, not an AWS billing cutoff or an authority to launch a job.
//! Immutable remote activation is required before a new worker balance is usable.

use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const POLICY_SCHEMA: &str = "crypto.ml-dataset.budget-policy/v1";
pub const GRANT_SCHEMA: &str = "crypto.ml-dataset.budget-grant/v1";
pub const LAUNCH_REQUEST_SCHEMA: &str = "crypto.ml-dataset.budget-launch-request/v1";
pub const LAUNCH_RESERVATION_SCHEMA: &str = "crypto.ml-dataset.budget-launch-reservation/v1";
pub const LAUNCH_BINDING_SCHEMA: &str = "crypto.ml-dataset.budget-launch-binding/v1";
const STATE_MAX_BYTES: usize = 32 * 1024 * 1024;
const MAX_GRANTS: usize = 4096;
const MONEY_CEILING: u64 = 200_000_000;
const OUTPUT_CEILING: u64 = 20 * 1024 * 1024 * 1024;

/// GET and HEAD share read_attempts; all values include failed/unknown attempts.
/// Money is conservative reserved micro-USD, never an observed billing total.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub money_micro_usd: u64,
    pub output_bytes: u64,
    pub put_attempts: u64,
    pub read_attempts: u64,
    pub download_bytes: u64,
    pub local_bytes: u64,
}

impl Resources {
    pub fn checked_add(&self, other: &Self) -> Result<Self, String> {
        macro_rules! add {
            ($f:ident) => {
                self.$f
                    .checked_add(other.$f)
                    .ok_or(concat!(stringify!($f), " overflow"))?
            };
        }
        Ok(Self {
            money_micro_usd: add!(money_micro_usd),
            output_bytes: add!(output_bytes),
            put_attempts: add!(put_attempts),
            read_attempts: add!(read_attempts),
            download_bytes: add!(download_bytes),
            local_bytes: add!(local_bytes),
        })
    }

    pub fn checked_sub(&self, other: &Self) -> Result<Self, String> {
        macro_rules! sub {
            ($f:ident) => {
                self.$f
                    .checked_sub(other.$f)
                    .ok_or(concat!(stringify!($f), " budget exceeded"))?
            };
        }
        Ok(Self {
            money_micro_usd: sub!(money_micro_usd),
            output_bytes: sub!(output_bytes),
            put_attempts: sub!(put_attempts),
            read_attempts: sub!(read_attempts),
            download_bytes: sub!(download_bytes),
            local_bytes: sub!(local_bytes),
        })
    }

    fn sqlite_safe(&self) -> Result<(), String> {
        for value in [
            self.money_micro_usd,
            self.output_bytes,
            self.put_attempts,
            self.read_attempts,
            self.download_bytes,
            self.local_bytes,
        ] {
            if value > i64::MAX as u64 {
                return Err("resource exceeds signed SQLite range".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemoteScope {
    pub bucket: String,
    pub prefix: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NoncomputePrices {
    pub put_micro_usd_per_1000: u64,
    pub read_micro_usd_per_1000: u64,
    pub storage_micro_usd_per_gib_day: u64,
    pub download_micro_usd_per_gib: u64,
    /// Retained EBS/other noncompute obligations inside the same prior reserve.
    pub fixed_micro_usd: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CampaignPolicy {
    pub schema: String,
    pub campaign_id: String,
    pub deadline_unix_seconds: u64,
    pub caps: Resources,
    pub prior: Resources,
    /// Part of prior.money_micro_usd, never added again as grant funding.
    pub prior_noncompute_reserve_micro_usd: u64,
    pub noncompute_prices: NoncomputePrices,
    pub prior_usage_reconciled: bool,
    pub prior_evidence_sha256: Vec<String>,
    pub region: String,
    pub pricing_region: String,
    pub price_evidence_sha256: String,
    pub resource_envelope_sha256: String,
    pub resource_lifetime_bound: bool,
    pub compute_rate_micro_usd_per_hour: u64,
    pub billable_resource_seconds: u64,
    pub billable_resource_count: u32,
    /// One authoritative allocator; copied controller ledgers cannot issue grants.
    pub allocator_ledger_path: String,
    pub allocator_host_boot_id: String,
    pub storage_horizon_days: u32,
    pub allowed_remotes: Vec<RemoteScope>,
    /// Must lie inside an allowed prefix; one immutable activation per grant.
    pub coordination_prefix: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OwnerIdentity {
    pub job_uid: String,
    pub pod_uid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunBinding {
    pub source_commit: String,
    pub binary_sha256: String,
    pub plan_sha256: String,
    pub protocol_sha256: String,
    pub shard_index: u64,
    pub shard_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GrantRequest {
    pub grant_id: String,
    pub worker_journal_path: String,
    pub owner: OwnerIdentity,
    pub binding: RunBinding,
    pub expires_unix_seconds: u64,
    pub resource_envelope_sha256: String,
    /// Inclusive total; finalization is a protected partition of this envelope.
    pub resources: Resources,
    pub finalization: Resources,
    /// Exact IRSA role; absent means only explicitly provided static credentials.
    #[serde(default)]
    pub credential_role_arn: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkerGrant {
    pub schema: String,
    pub campaign_id: String,
    pub policy_sha256: String,
    pub request: GrantRequest,
    pub activation_bucket: String,
    pub activation_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct LaunchTarget {
    pub cluster_identity: String,
    pub namespace: String,
    pub job_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LaunchRequest {
    pub schema: String,
    /// This is also the eventual worker grant ID; there is no second balance ID.
    pub launch_id: String,
    pub target: LaunchTarget,
    pub worker_journal_path: String,
    pub binding: RunBinding,
    pub expires_unix_seconds: u64,
    pub resource_envelope_sha256: String,
    pub resources: Resources,
    pub finalization: Resources,
    #[serde(default)]
    pub credential_role_arn: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LaunchReservation {
    pub schema: String,
    pub campaign_id: String,
    pub policy_sha256: String,
    pub request: LaunchRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LaunchBinding {
    pub schema: String,
    pub launch_id: String,
    pub reservation_sha256: String,
    pub observed_target: LaunchTarget,
    pub owner: OwnerIdentity,
    /// Identity of retained external observations, not Kubernetes attestation.
    pub observation_evidence_sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LaunchAdmissionReport {
    pub schema: String,
    pub admitted: bool,
    pub dry_run: bool,
    pub reasons: Vec<String>,
    pub policy_sha256: String,
    pub prior_and_allocated: Resources,
    pub requested: Resources,
    pub additional_requested: Resources,
    pub remaining: Option<Resources>,
    pub existing_reservation_sha256: Option<String>,
    pub already_bound: bool,
    pub account_wide_billing_cutoff: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdmissionReport {
    pub schema: String,
    pub admitted: bool,
    pub dry_run: bool,
    pub reasons: Vec<String>,
    pub policy_sha256: String,
    pub prior_and_allocated: Resources,
    pub requested: Resources,
    pub remaining: Option<Resources>,
    pub account_wide_billing_cutoff: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BudgetScope {
    Work,
    Finalization,
    Cell(String),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Head,
    Get,
    Put,
    StsWebIdentity,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HttpOperation {
    pub operation_id: String,
    pub method: HttpMethod,
    pub bucket: String,
    pub key: String,
    pub expected_sha256: Option<String>,
    pub expected_bytes: u64,
    pub scope: BudgetScope,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttemptTicket {
    pub grant_sha256: String,
    pub session_nonce: String,
    pub attempt_id: u64,
    pub operation: HttpOperation,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttemptOutcome {
    pub status: Option<u16>,
    pub request_id: Option<String>,
    pub observed_bytes: u64,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObjectReservation {
    pub grant_sha256: String,
    pub bucket: String,
    pub key: String,
    pub sha256: String,
    pub bytes: u64,
    pub scope: BudgetScope,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CellReservation {
    pub id: String,
    pub envelope: Resources,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CampaignState {
    policy: CampaignPolicy,
    grants: BTreeMap<String, WorkerGrant>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    launches: BTreeMap<String, LaunchRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LaunchRecord {
    reservation: LaunchReservation,
    /// Permanent once present. Expiration and failures do not release a target.
    binding: Option<LaunchBinding>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerState {
    policy: CampaignPolicy,
    grant: WorkerGrant,
    nonce: String,
    phase: String,
    work_used: Resources,
    finalization_used: Resources,
    next_attempt: u64,
    journal_path: String,
    host_boot_id: String,
}

pub struct CampaignLedger {
    connection: Mutex<Connection>,
    policy: CampaignPolicy,
}

/// Share with Arc. The held flock prevents a second local owner; remote
/// activation fences a freshly created balance with a distinct session nonce.
pub struct BudgetSession {
    connection: Mutex<Connection>,
    _lease: File,
    policy: CampaignPolicy,
    grant: WorkerGrant,
    nonce: String,
}

/// Live worker state used by the local startup gate after an interrupted handoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ActivationPhase {
    Pending,
    Active,
}

fn now() -> Result<u64, String> {
    #[cfg(all(test, target_os = "linux"))]
    if let Some(value) = tests::CLOCK.with(std::cell::Cell::get) {
        return Ok(value);
    }
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|e| e.to_string())
}
fn encode<T: Serialize>(value: &T) -> Result<String, String> {
    let value = serde_json::to_string(value).map_err(|e| e.to_string())?;
    if value.len() > STATE_MAX_BYTES {
        return Err("budget record exceeds metadata limit".into());
    }
    Ok(value)
}
fn hash<T: Serialize>(value: &T) -> Result<String, String> {
    Ok(hex::encode(crate::hash::sha256::sha256(
        encode(value)?.as_bytes(),
    )))
}
fn decode<T: serde::de::DeserializeOwned>(value: &str) -> Result<T, String> {
    if value.len() > STATE_MAX_BYTES {
        return Err("budget record exceeds metadata limit".into());
    }
    serde_json::from_str(value).map_err(|e| e.to_string())
}
fn sha(value: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("expected lowercase SHA-256".into());
    }
    Ok(())
}
fn identifier(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
    {
        return Err("invalid bounded budget identifier".into());
    }
    Ok(())
}
fn key(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 1024
        || value.starts_with('/')
        || value.contains('\\')
        || value.bytes().any(|b| b.is_ascii_control())
        || value
            .split('/')
            .any(|v| v.is_empty() || v == "." || v == "..")
    {
        return Err("invalid remote budget key".into());
    }
    Ok(())
}

impl CampaignPolicy {
    pub fn sha256(&self) -> Result<String, String> {
        hash(self)
    }

    pub fn compute_floor_micro_usd(&self) -> Result<u64, String> {
        let numerator = u128::from(self.compute_rate_micro_usd_per_hour)
            .checked_mul(u128::from(self.billable_resource_seconds))
            .and_then(|v| v.checked_mul(u128::from(self.billable_resource_count)))
            .ok_or("compute price multiplication overflow")?;
        let rounded = numerator
            .checked_add(3599)
            .ok_or("compute price rounding overflow")?
            / 3600;
        u64::try_from(rounded).map_err(|_| "compute reservation exceeds integer range".into())
    }

    pub fn noncompute_floor_micro_usd(&self) -> Result<u64, String> {
        fn ceil(value: u128, denominator: u128) -> Result<u64, String> {
            let rounded = value
                .checked_add(denominator - 1)
                .ok_or("noncompute price rounding overflow")?
                / denominator;
            u64::try_from(rounded).map_err(|_| "noncompute cost exceeds integer range".into())
        }
        let p = &self.noncompute_prices;
        let puts = ceil(
            u128::from(self.caps.put_attempts) * u128::from(p.put_micro_usd_per_1000),
            1000,
        )?;
        let reads = ceil(
            u128::from(self.caps.read_attempts) * u128::from(p.read_micro_usd_per_1000),
            1000,
        )?;
        let storage = u128::from(self.caps.output_bytes)
            .checked_mul(u128::from(p.storage_micro_usd_per_gib_day))
            .and_then(|v| v.checked_mul(u128::from(self.storage_horizon_days)))
            .ok_or("storage price multiplication overflow")?;
        let storage = ceil(storage, 1024 * 1024 * 1024)?;
        let download = ceil(
            u128::from(self.caps.download_bytes) * u128::from(p.download_micro_usd_per_gib),
            1024 * 1024 * 1024,
        )?;
        [puts, reads, storage, download, p.fixed_micro_usd]
            .into_iter()
            .try_fold(0u64, |sum, v| {
                sum.checked_add(v)
                    .ok_or("noncompute price total overflow".into())
            })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != POLICY_SCHEMA {
            return Err("unsupported budget policy schema".into());
        }
        identifier(&self.campaign_id)?;
        self.caps.sqlite_safe()?;
        self.prior.sqlite_safe()?;
        self.caps.checked_sub(&self.prior)?;
        let prices = &self.noncompute_prices;
        if prices.put_micro_usd_per_1000 == 0
            || prices.read_micro_usd_per_1000 == 0
            || prices.storage_micro_usd_per_gib_day == 0
            || prices.download_micro_usd_per_gib == 0
            || self.prior_noncompute_reserve_micro_usd > self.prior.money_micro_usd
            || self.noncompute_floor_micro_usd()? > self.prior_noncompute_reserve_micro_usd
        {
            return Err("retained prior noncompute reserve must cover conservative regional request/storage/transfer floors for the entire campaign caps".into());
        }
        if self.caps.money_micro_usd == 0
            || self.caps.money_micro_usd > MONEY_CEILING
            || self.caps.output_bytes == 0
            || self.caps.output_bytes > OUTPUT_CEILING
            || self.caps.put_attempts == 0
            || self.caps.put_attempts > 100_000
            || self.caps.read_attempts == 0
            || self.caps.read_attempts > 200_000
            || self.caps.download_bytes == 0
            || self.caps.local_bytes == 0
        {
            return Err("campaign caps must be finite and within the existing USD 200 / 20 GiB / 100k PUT / 200k read ceilings".into());
        }
        if !self.prior_usage_reconciled
            || self.prior_evidence_sha256.is_empty()
            || self.prior.money_micro_usd == 0
        {
            return Err(
                "prior campaign usage and obligations must be reconciled; unknown is not zero"
                    .into(),
            );
        }
        for evidence in &self.prior_evidence_sha256 {
            sha(evidence)?;
        }
        if self.prior_evidence_sha256.len() > 256 {
            return Err("too many prior evidence records".into());
        }
        sha(&self.price_evidence_sha256)?;
        sha(&self.resource_envelope_sha256)?;
        if self.region != "us-east-2" || self.pricing_region != self.region {
            return Err("actual dataset region and pricing region must both be us-east-2".into());
        }
        if !self.resource_lifetime_bound
            || self.storage_horizon_days == 0
            || self.compute_rate_micro_usd_per_hour == 0
            || self.billable_resource_seconds == 0
            || self.billable_resource_count == 0
        {
            return Err("billable resource lifetime and storage horizon must be bounded".into());
        }
        if self.compute_floor_micro_usd()? > self.caps.money_micro_usd {
            return Err("actual resource price floor exceeds campaign cap".into());
        }
        let authority = Path::new(&self.allocator_ledger_path);
        if !authority.is_absolute()
            || authority
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err("allocator authority must name its exact absolute journal path".into());
        }
        identifier(&self.allocator_host_boot_id)?;
        if self.deadline_unix_seconds <= now()? {
            return Err("campaign deadline expired".into());
        }
        if self.allowed_remotes.is_empty() || self.allowed_remotes.len() > 16 {
            return Err("one to sixteen approved remote prefixes required".into());
        }
        for remote in &self.allowed_remotes {
            if remote.bucket != "setec-astronomy-artifacts" || !remote.prefix.ends_with('/') {
                return Err("budget requires approved bucket and slash-terminated prefix".into());
            }
            key(remote.prefix.trim_end_matches('/'))?;
        }
        if !self.coordination_prefix.ends_with('/') {
            return Err("coordination prefix must end in slash".into());
        }
        self.validate_remote(
            "setec-astronomy-artifacts",
            &format!("{}activation", self.coordination_prefix),
        )
    }

    pub fn validate_remote(&self, bucket: &str, object_key: &str) -> Result<(), String> {
        key(object_key)?;
        if !self
            .allowed_remotes
            .iter()
            .any(|scope| scope.bucket == bucket && object_key.starts_with(&scope.prefix))
        {
            return Err("remote is outside frozen campaign prefixes".into());
        }
        Ok(())
    }
}

/// Borrowed common fields keep pending allocations ownerless. Historical checks
/// use the same structural rules without turning expiry into a quota refund.
struct GrantEnvelope<'a> {
    id: &'a str,
    worker_journal_path: &'a str,
    binding: &'a RunBinding,
    expires_unix_seconds: u64,
    resource_envelope_sha256: &'a str,
    resources: &'a Resources,
    finalization: &'a Resources,
    credential_role_arn: Option<&'a str>,
}

impl GrantEnvelope<'_> {
    fn validate(&self, policy: &CampaignPolicy, live: bool) -> Result<(), String> {
        identifier(self.id)?;
        let journal = Path::new(self.worker_journal_path);
        if !journal.is_absolute()
            || journal
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err("grant must bind the exact absolute worker journal path".into());
        }
        if self.binding.source_commit.len() != 40
            || !self
                .binding
                .source_commit
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return Err("grant requires an exact source commit".into());
        }
        for value in [
            &self.binding.binary_sha256,
            &self.binding.plan_sha256,
            &self.binding.protocol_sha256,
        ] {
            sha(value)?;
        }
        if self.binding.shard_count == 0 || self.binding.shard_index >= self.binding.shard_count {
            return Err("invalid grant shard".into());
        }
        if self.resource_envelope_sha256 != policy.resource_envelope_sha256 {
            return Err("grant resource envelope differs from priced actual resources".into());
        }
        if self.expires_unix_seconds == 0
            || (live && self.expires_unix_seconds <= now()?)
            || self.expires_unix_seconds > policy.deadline_unix_seconds
        {
            return Err("grant deadline is expired or extends the original campaign".into());
        }
        self.resources.sqlite_safe()?;
        self.finalization.sqlite_safe()?;
        self.resources.checked_sub(self.finalization)?;
        if self.resources.money_micro_usd < policy.compute_floor_micro_usd()? {
            return Err(
                "grant money is below the conservative actual resource lifetime price".into(),
            );
        }
        if self.resources.money_micro_usd == 0
            || self.finalization.put_attempts < 1
            || self.finalization.read_attempts < 1
            || self.finalization.output_bytes < 4096
            || self.finalization.download_bytes < 4096
        {
            return Err(
                "grant needs a priced reservation and protected activation/finalization capacity"
                    .into(),
            );
        }
        if let Some(role) = self.credential_role_arn {
            let parts: Vec<_> = role.splitn(6, ':').collect();
            if parts.len() != 6
                || parts[0] != "arn"
                || parts[1] != "aws"
                || parts[2] != "iam"
                || !parts[3].is_empty()
                || parts[4].len() != 12
                || !parts[4].bytes().all(|b| b.is_ascii_digit())
                || !parts[5].starts_with("role/")
                || parts[5].len() <= 5
                || role.len() > 512
                || role
                    .bytes()
                    .any(|b| !b.is_ascii_alphanumeric() && !b":/+=,.@_-".contains(&b))
            {
                return Err("credential role must be an exact bounded AWS IAM role ARN".into());
            }
            if self.finalization.read_attempts < 2
                || self.finalization.download_bytes < 65536 + 4096
            {
                return Err("IRSA requires protected STS and activation response capacity".into());
            }
        }
        Ok(())
    }
}

impl GrantRequest {
    fn envelope(&self) -> GrantEnvelope<'_> {
        GrantEnvelope {
            id: &self.grant_id,
            worker_journal_path: &self.worker_journal_path,
            binding: &self.binding,
            expires_unix_seconds: self.expires_unix_seconds,
            resource_envelope_sha256: &self.resource_envelope_sha256,
            resources: &self.resources,
            finalization: &self.finalization,
            credential_role_arn: self.credential_role_arn.as_deref(),
        }
    }

    fn validate(&self, policy: &CampaignPolicy) -> Result<(), String> {
        identifier(&self.owner.job_uid)?;
        identifier(&self.owner.pod_uid)?;
        self.envelope().validate(policy, true)
    }
}

impl LaunchTarget {
    fn validate(&self, policy: &CampaignPolicy) -> Result<(), String> {
        let parts: Vec<_> = self.cluster_identity.splitn(6, ':').collect();
        if parts.len() != 6
            || parts[0] != "arn"
            || parts[1] != "aws"
            || parts[2] != "eks"
            || parts[3] != policy.region
            || parts[4].len() != 12
            || !parts[4].bytes().all(|b| b.is_ascii_digit())
        {
            return Err(
                "launch target requires an exact EKS cluster ARN in the admitted region".into(),
            );
        }
        let name = parts[5]
            .strip_prefix("cluster/")
            .ok_or("launch target requires an EKS cluster ARN")?;
        if name.is_empty()
            || name.len() > 100
            || !name.as_bytes()[0].is_ascii_alphanumeric()
            || !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
        {
            return Err("invalid bounded EKS cluster name".into());
        }
        for label in [&self.namespace, &self.job_name] {
            if label.is_empty()
                || label.len() > 63
                || !label.as_bytes()[0].is_ascii_alphanumeric()
                || !label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
                || !label
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            {
                return Err(
                    "launch namespace and Job name must be bounded lowercase DNS labels".into(),
                );
            }
        }
        Ok(())
    }
}

impl LaunchRequest {
    fn envelope(&self) -> GrantEnvelope<'_> {
        GrantEnvelope {
            id: &self.launch_id,
            worker_journal_path: &self.worker_journal_path,
            binding: &self.binding,
            expires_unix_seconds: self.expires_unix_seconds,
            resource_envelope_sha256: &self.resource_envelope_sha256,
            resources: &self.resources,
            finalization: &self.finalization,
            credential_role_arn: self.credential_role_arn.as_deref(),
        }
    }

    fn validate(&self, policy: &CampaignPolicy, live: bool) -> Result<(), String> {
        if self.schema != LAUNCH_REQUEST_SCHEMA {
            return Err("unsupported budget launch request schema".into());
        }
        self.target.validate(policy)?;
        self.envelope().validate(policy, live)
    }

    fn with_owner(&self, owner: &OwnerIdentity) -> GrantRequest {
        GrantRequest {
            grant_id: self.launch_id.clone(),
            worker_journal_path: self.worker_journal_path.clone(),
            owner: owner.clone(),
            binding: self.binding.clone(),
            expires_unix_seconds: self.expires_unix_seconds,
            resource_envelope_sha256: self.resource_envelope_sha256.clone(),
            resources: self.resources.clone(),
            finalization: self.finalization.clone(),
            credential_role_arn: self.credential_role_arn.clone(),
        }
    }
}

impl LaunchReservation {
    pub fn sha256(&self) -> Result<String, String> {
        hash(self)
    }

    fn validate(&self, policy: &CampaignPolicy) -> Result<(), String> {
        if self.schema != LAUNCH_RESERVATION_SCHEMA
            || self.campaign_id != policy.campaign_id
            || self.policy_sha256 != policy.sha256()?
        {
            return Err("launch reservation differs from frozen campaign".into());
        }
        self.request.validate(policy, false)
    }
}

impl LaunchBinding {
    fn validate(&self, reservation: &LaunchReservation) -> Result<(), String> {
        if self.schema != LAUNCH_BINDING_SCHEMA
            || self.launch_id != reservation.request.launch_id
            || self.reservation_sha256 != reservation.sha256()?
            || self.observed_target != reservation.request.target
        {
            return Err("launch binding differs from the reserved target or digest".into());
        }
        identifier(&self.owner.job_uid)?;
        identifier(&self.owner.pod_uid)?;
        sha(&self.observation_evidence_sha256)
    }
}

fn worker_grant(policy: &CampaignPolicy, request: &GrantRequest) -> Result<WorkerGrant, String> {
    Ok(WorkerGrant {
        schema: GRANT_SCHEMA.into(),
        campaign_id: policy.campaign_id.clone(),
        policy_sha256: policy.sha256()?,
        request: request.clone(),
        activation_bucket: "setec-astronomy-artifacts".into(),
        activation_key: format!(
            "{}activations/{}/{}.json",
            policy.coordination_prefix,
            policy.campaign_id,
            hash(&request.grant_id)?
        ),
    })
}

/// Check a delivered grant against the pinned reservation and observed local
/// owner. This validates the relationship, not the truth of Kubernetes evidence.
pub(super) fn validate_launch_delivery(
    policy: &CampaignPolicy,
    reservation: &LaunchReservation,
    binding: &LaunchBinding,
    grant: &WorkerGrant,
    actual_owner: &OwnerIdentity,
) -> Result<(), String> {
    policy.validate()?;
    reservation.validate(policy)?;
    binding.validate(reservation)?;
    grant.validate(policy)?;
    if binding.owner != *actual_owner {
        return Err("delivered launch owner differs from actual worker identity".into());
    }
    let expected = worker_grant(policy, &reservation.request.with_owner(actual_owner))?;
    if *grant != expected {
        return Err("delivered grant differs from the complete reserved allocation".into());
    }
    Ok(())
}

impl WorkerGrant {
    pub fn sha256(&self) -> Result<String, String> {
        hash(self)
    }
    pub fn validate(&self, policy: &CampaignPolicy) -> Result<(), String> {
        policy.validate()?;
        if self.schema != GRANT_SCHEMA
            || self.campaign_id != policy.campaign_id
            || self.policy_sha256 != policy.sha256()?
        {
            return Err("grant does not match frozen campaign policy".into());
        }
        self.request.validate(policy)?;
        policy
            .caps
            .checked_sub(&policy.prior.checked_add(&self.request.resources)?)?;
        let expected = format!(
            "{}activations/{}/{}.json",
            policy.coordination_prefix,
            policy.campaign_id,
            hash(&self.request.grant_id)?
        );
        if self.activation_bucket != "setec-astronomy-artifacts" || self.activation_key != expected
        {
            return Err("grant activation key is not canonically bound".into());
        }
        policy.validate_remote(&self.activation_bucket, &self.activation_key)
    }
}

fn db_path(path: &Path, create: bool) -> Result<PathBuf, String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = super::checked_directory(parent, create)?;
    let name = path
        .file_name()
        .ok_or("budget database requires a filename")?;
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(meta) if !meta.is_file() => {
            return Err("budget database must be a regular file, never a symlink".into())
        }
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.to_string()),
    }
    // SQLite follows sidecar paths itself, so reject pre-existing links too.
    for suffix in ["-journal", "-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{suffix}", path.display()));
        if let Ok(meta) = fs::symlink_metadata(sidecar) {
            if !meta.is_file() {
                return Err("budget SQLite sidecar is not a regular file".into());
            }
        }
    }
    Ok(path)
}

fn database(path: &Path, create: bool, readonly: bool) -> Result<Connection, String> {
    let path = db_path(path, create)?;
    if create {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        File::open(path.parent().ok_or("missing database parent")?)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
    }
    let flags = if readonly {
        OpenFlags::SQLITE_OPEN_READ_ONLY
    } else {
        OpenFlags::SQLITE_OPEN_READ_WRITE
    };
    let conn = Connection::open_with_flags(&path, flags).map_err(|e| e.to_string())?;
    conn.busy_timeout(Duration::from_secs(5))
        .map_err(|e| e.to_string())?;
    if readonly {
        conn.execute_batch("PRAGMA query_only=ON;")
            .map_err(|e| e.to_string())?;
    } else {
        conn.execute_batch("PRAGMA journal_mode=DELETE; PRAGMA synchronous=EXTRA; PRAGMA foreign_keys=ON; PRAGMA trusted_schema=OFF;").map_err(|e| e.to_string())?;
    }
    if create {
        conn.execute_batch("CREATE TABLE state(id INTEGER PRIMARY KEY CHECK(id=1),kind TEXT NOT NULL,json TEXT NOT NULL,sha TEXT NOT NULL);
            CREATE TABLE events(seq INTEGER PRIMARY KEY,previous TEXT NOT NULL,sha TEXT NOT NULL,body TEXT NOT NULL);
            CREATE TABLE cells(id TEXT PRIMARY KEY,envelope TEXT NOT NULL,used TEXT NOT NULL);
            CREATE TABLE objects(bucket TEXT NOT NULL,key TEXT NOT NULL,json TEXT NOT NULL,verified INTEGER NOT NULL DEFAULT 0,PRIMARY KEY(bucket,key));
            CREATE TABLE attempts(id INTEGER PRIMARY KEY,ticket TEXT NOT NULL,outcome TEXT);").map_err(|e| e.to_string())?;
    }
    Ok(conn)
}

fn read_state<T: serde::de::DeserializeOwned>(conn: &Connection, kind: &str) -> Result<T, String> {
    let (actual, text, digest): (String, String, String) = conn
        .query_row("SELECT kind,json,sha FROM state WHERE id=1", [], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .map_err(|e| e.to_string())?;
    if actual != kind || hex::encode(crate::hash::sha256::sha256(text.as_bytes())) != digest {
        return Err("budget state kind/hash mismatch or incomplete initialization".into());
    }
    decode(&text)
}

fn write_state<T: Serialize>(conn: &Connection, kind: &str, value: &T) -> Result<(), String> {
    let text = encode(value)?;
    let digest = hex::encode(crate::hash::sha256::sha256(text.as_bytes()));
    conn.execute("INSERT INTO state(id,kind,json,sha) VALUES(1,?1,?2,?3) ON CONFLICT(id) DO UPDATE SET kind=excluded.kind,json=excluded.json,sha=excluded.sha", params![kind,text,digest]).map_err(|e| e.to_string())?;
    Ok(())
}

fn event(conn: &Connection, kind: &str, details: Value) -> Result<(), String> {
    let previous: Option<(u64, String)> = conn
        .query_row(
            "SELECT seq,sha FROM events ORDER BY seq DESC LIMIT 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let (seq, previous) = match previous {
        Some((n, p)) => (n.checked_add(1).ok_or("event sequence overflow")?, p),
        None => (1, "0".repeat(64)),
    };
    let state_sha: String = conn
        .query_row("SELECT sha FROM state WHERE id=1", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let body = json!({"schema":"crypto.ml-dataset.budget-event/v1","sequence":seq,"previous":previous,"kind":kind,"state_sha256":state_sha,"details":details});
    let digest = hash(&body)?;
    conn.execute(
        "INSERT INTO events(seq,previous,sha,body) VALUES(?1,?2,?3,?4)",
        params![seq, previous, digest, encode(&body)?],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn verify_events(conn: &Connection) -> Result<(), String> {
    let integrity: String = conn
        .query_row("PRAGMA quick_check", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    if integrity != "ok" {
        return Err("budget database integrity check failed".into());
    }
    let mut stmt = conn
        .prepare("SELECT seq,previous,sha,body FROM events ORDER BY seq")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut expected_seq = 1u64;
    let mut previous = "0".repeat(64);
    let mut state_sha = String::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let seq: u64 = row.get(0).map_err(|e| e.to_string())?;
        let prev: String = row.get(1).map_err(|e| e.to_string())?;
        let digest: String = row.get(2).map_err(|e| e.to_string())?;
        let body: String = row.get(3).map_err(|e| e.to_string())?;
        let value: Value = decode(&body)?;
        if seq != expected_seq
            || prev != previous
            || hash(&value)? != digest
            || value["sequence"].as_u64() != Some(seq)
            || value["previous"].as_str() != Some(prev.as_str())
        {
            return Err("budget journal chain is corrupt or incomplete".into());
        }
        previous = digest;
        state_sha = value["state_sha256"]
            .as_str()
            .ok_or("missing journal state digest")?
            .into();
        expected_seq = expected_seq
            .checked_add(1)
            .ok_or("journal sequence overflow")?;
    }
    let current: String = conn
        .query_row("SELECT sha FROM state WHERE id=1", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    if state_sha != current {
        return Err("budget state is not the journal's committed state".into());
    }
    Ok(())
}

fn allocated(state: &CampaignState) -> Result<Resources, String> {
    validate_launches(state)?;
    let grants = state
        .grants
        .values()
        .try_fold(state.policy.prior.clone(), |sum, g| {
            sum.checked_add(&g.request.resources)
        })?;
    state
        .launches
        .values()
        .filter(|record| record.binding.is_none())
        .try_fold(grants, |sum, record| {
            sum.checked_add(&record.reservation.request.resources)
        })
}

fn allocation_slots(state: &CampaignState) -> Result<usize, String> {
    state
        .grants
        .len()
        .checked_add(
            state
                .launches
                .values()
                .filter(|record| record.binding.is_none())
                .count(),
        )
        .ok_or_else(|| "campaign allocation count overflow".into())
}

fn validate_launches(state: &CampaignState) -> Result<(), String> {
    if allocation_slots(state)? > MAX_GRANTS {
        return Err("maximum campaign allocation count exceeded".into());
    }
    let mut targets = BTreeSet::new();
    for (id, record) in &state.launches {
        let reservation = &record.reservation;
        if id != &reservation.request.launch_id {
            return Err("launch map key differs from its reserved ID".into());
        }
        reservation.validate(&state.policy)?;
        if !targets.insert(&reservation.request.target) {
            return Err("multiple allocations occupy the same launch target".into());
        }
        match &record.binding {
            None if state.grants.contains_key(id) => {
                return Err("pending launch also has a worker grant".into());
            }
            None => {}
            Some(binding) => {
                binding.validate(reservation)?;
                let expected = worker_grant(
                    &state.policy,
                    &reservation.request.with_owner(&binding.owner),
                )?;
                if state.grants.get(id) != Some(&expected) {
                    return Err("bound launch is missing its exact worker grant".into());
                }
                if state.grants.iter().any(|(other_id, grant)| {
                    other_id != id
                        && (grant.request.owner.job_uid == binding.owner.job_uid
                            || grant.request.owner.pod_uid == binding.owner.pod_uid)
                }) {
                    return Err("bound launch owner also has another allocation".into());
                }
            }
        }
    }
    Ok(())
}

fn admission(
    state: &CampaignState,
    request: &GrantRequest,
    dry_run: bool,
) -> Result<AdmissionReport, String> {
    let sum = allocated(state)?;
    let mut reasons = Vec::new();
    if let Err(e) = state.policy.validate() {
        reasons.push(e);
    }
    if let Err(e) = request.validate(&state.policy) {
        reasons.push(e);
    }
    if allocation_slots(state)? >= MAX_GRANTS {
        reasons.push("maximum campaign grant count reached".into());
    }
    if state.launches.contains_key(&request.grant_id) {
        reasons.push("grant ID belongs to a retained launch reservation; use bind-launch".into());
    }
    if state.grants.contains_key(&request.grant_id) {
        reasons.push("grant ID already issued; resume its existing allocation".into());
    }
    if state
        .grants
        .values()
        .any(|g| g.request.owner == request.owner)
    {
        reasons.push("owner already has a grant; do not issue a second balance".into());
    }
    if state.launches.values().any(|record| {
        record.binding.as_ref().is_some_and(|binding| {
            binding.owner.job_uid == request.owner.job_uid
                || binding.owner.pod_uid == request.owner.pod_uid
        })
    }) {
        reasons.push(
            "owner belongs to a retained launch; replacement balances require reconciliation"
                .into(),
        );
    }
    let remaining = sum
        .checked_add(&request.resources)
        .and_then(|v| state.policy.caps.checked_sub(&v));
    if let Err(e) = &remaining {
        reasons.push(e.clone());
    }
    Ok(AdmissionReport {
        schema: "crypto.ml-dataset.budget-admission/v1".into(),
        admitted: reasons.is_empty(),
        dry_run,
        reasons,
        policy_sha256: state.policy.sha256()?,
        prior_and_allocated: sum,
        requested: request.resources.clone(),
        remaining: remaining.ok(),
        account_wide_billing_cutoff: false,
    })
}

fn launch_admission(
    state: &CampaignState,
    request: &LaunchRequest,
    dry_run: bool,
) -> Result<LaunchAdmissionReport, String> {
    let sum = allocated(state)?;
    let mut reasons = Vec::new();
    if let Err(e) = state.policy.validate() {
        reasons.push(e);
    }
    if let Err(e) = request.validate(&state.policy, true) {
        reasons.push(e);
    }
    let mut additional = request.resources.clone();
    let mut existing = None;
    let mut already_bound = false;
    match state.launches.get(&request.launch_id) {
        Some(record) if record.reservation.request == *request => {
            additional = Resources::default();
            existing = Some(record.reservation.sha256()?);
            already_bound = record.binding.is_some();
            if already_bound {
                reasons.push(
                    "launch already bound; exact retry may export its existing evidence only"
                        .into(),
                );
            }
        }
        Some(_) => reasons.push("launch ID cannot be repointed to a different request".into()),
        None => {
            if state.grants.contains_key(&request.launch_id) {
                reasons.push("launch ID already belongs to a direct worker grant".into());
            }
            if state
                .launches
                .values()
                .any(|record| record.reservation.request.target == request.target)
            {
                reasons.push("launch target already has a retained allocation".into());
            }
            if allocation_slots(state)? >= MAX_GRANTS {
                reasons.push("maximum campaign allocation count reached".into());
            }
        }
    }
    let remaining = sum
        .checked_add(&additional)
        .and_then(|value| state.policy.caps.checked_sub(&value));
    if let Err(e) = &remaining {
        reasons.push(e.clone());
    }
    Ok(LaunchAdmissionReport {
        schema: "crypto.ml-dataset.budget-launch-admission/v1".into(),
        admitted: reasons.is_empty(),
        dry_run,
        reasons,
        policy_sha256: state.policy.sha256()?,
        prior_and_allocated: sum,
        requested: request.resources.clone(),
        additional_requested: additional,
        remaining: remaining.ok(),
        existing_reservation_sha256: existing,
        already_bound,
        account_wide_billing_cutoff: false,
    })
}

impl CampaignLedger {
    fn authority(path: &Path, policy: &CampaignPolicy) -> Result<(), String> {
        let path = db_path(path, false)?;
        if path.to_str() != Some(policy.allocator_ledger_path.as_str())
            || boot_id()? != policy.allocator_host_boot_id
        {
            return Err("allocator authority mismatch; copied/migrated ledger cannot issue another campaign balance".into());
        }
        Ok(())
    }

    fn authority_path(path: &Path) -> PathBuf {
        PathBuf::from(format!("{}.authority", path.display()))
    }

    fn create_authority(path: &Path, policy: &CampaignPolicy) -> Result<(), String> {
        use std::io::Write;
        let path = Self::authority_path(path);
        let marker = encode(
            &json!({"schema":"crypto.ml-dataset.budget-allocator-authority/v1","policy_sha256":policy.sha256()?,"ledger_path":policy.allocator_ledger_path,"host_boot_id":policy.allocator_host_boot_id}),
        )?;
        let mut file = OpenOptions::new().create_new(true).write(true).open(&path).map_err(|e| format!("allocator authority already exists or cannot be created; never reset a missing ledger: {e}"))?;
        file.write_all(marker.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        File::open(path.parent().ok_or("missing authority parent")?)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn check_authority(path: &Path, policy: &CampaignPolicy) -> Result<(), String> {
        let marker = Self::authority_path(path);
        let metadata = fs::symlink_metadata(&marker)
            .map_err(|_| "allocator authority marker missing; refusing reconstructed balance")?;
        if !metadata.is_file() || metadata.len() > 4096 {
            return Err("allocator authority must be a bounded regular file".into());
        }
        let expected = encode(
            &json!({"schema":"crypto.ml-dataset.budget-allocator-authority/v1","policy_sha256":policy.sha256()?,"ledger_path":policy.allocator_ledger_path,"host_boot_id":policy.allocator_host_boot_id}),
        )?;
        let mut actual = String::new();
        File::open(marker)
            .and_then(|f| f.take(4097).read_to_string(&mut actual))
            .map_err(|e| e.to_string())?;
        if actual != expected {
            return Err("allocator authority marker differs from frozen policy".into());
        }
        Ok(())
    }

    pub fn create(path: &Path, policy: &CampaignPolicy) -> Result<Self, String> {
        policy.validate()?;
        Self::authority(path, policy)?;
        if fs::symlink_metadata(path).is_ok() {
            return Err("allocator database already exists; open its original authority without recreating it".into());
        }
        // Kept if SQLite initialization fails. A subsequent create must stop,
        // not infer that zero grants were issued from the missing database.
        Self::create_authority(path, policy)?;
        let mut conn = database(path, true, false)?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        write_state(
            &tx,
            "campaign",
            &CampaignState {
                policy: policy.clone(),
                grants: BTreeMap::new(),
                launches: BTreeMap::new(),
            },
        )?;
        event(
            &tx,
            "campaign_created",
            json!({"policy_sha256":policy.sha256()?}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(Self {
            connection: Mutex::new(conn),
            policy: policy.clone(),
        })
    }

    pub fn open(path: &Path, policy: &CampaignPolicy) -> Result<Self, String> {
        Self::authority(path, policy)?;
        Self::check_authority(path, policy)?;
        let conn = database(path, false, false)?;
        verify_events(&conn)?;
        let state: CampaignState = read_state(&conn, "campaign")?;
        if state.policy != *policy {
            return Err("campaign policy changed; refusing balance reset".into());
        }
        policy.caps.checked_sub(&allocated(&state)?)?;
        Ok(Self {
            connection: Mutex::new(conn),
            policy: policy.clone(),
        })
    }

    /// Opens existing state READ_ONLY. A missing ledger is a hypothetical initial
    /// state only; neither a directory, SQLite database nor sidecar is created.
    pub fn dry_admit(
        path: &Path,
        policy: &CampaignPolicy,
        request: &GrantRequest,
    ) -> Result<AdmissionReport, String> {
        admission(&Self::dry_state(path, policy)?, request, true)
    }

    fn dry_state(path: &Path, policy: &CampaignPolicy) -> Result<CampaignState, String> {
        Self::authority(path, policy)?;
        let state = match fs::symlink_metadata(path) {
            Ok(_) => {
                Self::check_authority(path, policy)?;
                let conn = database(path, false, true)?;
                verify_events(&conn)?;
                let state: CampaignState = read_state(&conn, "campaign")?;
                if state.policy != *policy {
                    return Err("campaign policy differs from existing ledger".into());
                }
                state
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                match fs::symlink_metadata(Self::authority_path(path)) {
                    Ok(_) => return Err("allocator database missing with existing authority; reconcile, never reset".into()),
                    Err(e) if e.kind()==std::io::ErrorKind::NotFound => {},
                    Err(e) => return Err(e.to_string()),
                }
                CampaignState {
                    policy: policy.clone(),
                    grants: BTreeMap::new(),
                    launches: BTreeMap::new(),
                }
            }
            Err(e) => return Err(e.to_string()),
        };
        Ok(state)
    }

    /// Pure read-only admission; an exact retry requests no additional capacity.
    pub fn dry_admit_launch(
        path: &Path,
        policy: &CampaignPolicy,
        request: &LaunchRequest,
    ) -> Result<LaunchAdmissionReport, String> {
        launch_admission(&Self::dry_state(path, policy)?, request, true)
    }

    pub fn reserve(&self, request: &GrantRequest) -> Result<WorkerGrant, String> {
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned campaign ledger")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: CampaignState = read_state(&tx, "campaign")?;
        if state.policy != self.policy {
            return Err("campaign binding changed".into());
        }
        let report = admission(&state, request, false)?;
        if !report.admitted {
            return Err(format!(
                "budget admission rejected: {}",
                report.reasons.join("; ")
            ));
        }
        let grant = worker_grant(&self.policy, request)?;
        state.grants.insert(request.grant_id.clone(), grant.clone());
        write_state(&tx, "campaign", &state)?;
        event(&tx, "grant_reserved", json!({"grant":grant}))?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(grant)
    }

    /// Commit the complete finite allocation before any external Job submission.
    /// Exact retries export the retained record, even after expiry; they never
    /// authorize another submission or renew its original deadline.
    pub fn reserve_launch(&self, request: &LaunchRequest) -> Result<LaunchReservation, String> {
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned campaign ledger")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: CampaignState = read_state(&tx, "campaign")?;
        if state.policy != self.policy {
            return Err("campaign binding changed".into());
        }
        self.policy.caps.checked_sub(&allocated(&state)?)?;
        if let Some(record) = state.launches.get(&request.launch_id) {
            if record.reservation.request != *request {
                return Err("launch ID cannot be repointed to a different request".into());
            }
            return Ok(record.reservation.clone());
        }
        let report = launch_admission(&state, request, false)?;
        if !report.admitted {
            return Err(format!(
                "launch budget admission rejected: {}",
                report.reasons.join("; ")
            ));
        }
        let reservation = LaunchReservation {
            schema: LAUNCH_RESERVATION_SCHEMA.into(),
            campaign_id: self.policy.campaign_id.clone(),
            policy_sha256: self.policy.sha256()?,
            request: request.clone(),
        };
        state.launches.insert(
            request.launch_id.clone(),
            LaunchRecord {
                reservation: reservation.clone(),
                binding: None,
            },
        );
        self.policy.caps.checked_sub(&allocated(&state)?)?;
        write_state(&tx, "campaign", &state)?;
        event(&tx, "launch_reserved", json!({"reservation":reservation}))?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(reservation)
    }

    /// Bind one retained allocation once. External observations are pinned by
    /// digest; this method does not query or attest Kubernetes owner identities.
    pub fn bind_launch(&self, binding: &LaunchBinding) -> Result<WorkerGrant, String> {
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned campaign ledger")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: CampaignState = read_state(&tx, "campaign")?;
        if state.policy != self.policy {
            return Err("campaign binding changed".into());
        }
        let before = allocated(&state)?;
        self.policy.caps.checked_sub(&before)?;
        let record = state
            .launches
            .get(&binding.launch_id)
            .ok_or("launch reservation missing; binding cannot create an allocation")?
            .clone();
        binding.validate(&record.reservation)?;
        if let Some(previous) = &record.binding {
            if previous != binding {
                return Err(
                    "launch already bound; owner and observation evidence cannot change".into(),
                );
            }
            return state
                .grants
                .get(&binding.launch_id)
                .cloned()
                .ok_or_else(|| "bound launch worker grant missing".into());
        }
        self.policy.validate()?;
        record.reservation.request.validate(&self.policy, true)?;
        if state.grants.values().any(|grant| {
            grant.request.owner.job_uid == binding.owner.job_uid
                || grant.request.owner.pod_uid == binding.owner.pod_uid
        }) {
            return Err(
                "observed Job or Pod already has a grant; do not issue another balance".into(),
            );
        }
        let grant = worker_grant(
            &self.policy,
            &record.reservation.request.with_owner(&binding.owner),
        )?;
        grant.validate(&self.policy)?;
        state
            .grants
            .insert(binding.launch_id.clone(), grant.clone());
        state
            .launches
            .get_mut(&binding.launch_id)
            .ok_or("launch reservation disappeared")?
            .binding = Some(binding.clone());
        if allocated(&state)? != before {
            return Err("launch binding changed total allocation".into());
        }
        write_state(&tx, "campaign", &state)?;
        event(
            &tx,
            "launch_bound",
            json!({"binding":binding,"grant_sha256":grant.sha256()?}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(grant)
    }

    pub fn snapshot(&self) -> Result<Value, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned campaign ledger")?;
        let state: CampaignState = read_state(&conn, "campaign")?;
        let sum = allocated(&state)?;
        let pending: Vec<_> = state
            .launches
            .values()
            .filter(|record| record.binding.is_none())
            .collect();
        let pending_resources = pending
            .iter()
            .try_fold(Resources::default(), |sum, record| {
                sum.checked_add(&record.reservation.request.resources)
            })?;
        Ok(
            json!({"schema":"crypto.ml-dataset.budget-campaign-status/v1","policy_sha256":state.policy.sha256()?,"prior_and_allocated":sum,"remaining":state.policy.caps.checked_sub(&sum)?,"grants":state.grants,"launches":state.launches,"pending_launch_count":pending.len(),"bound_launch_count":state.launches.len()-pending.len(),"pending_launch_resources":pending_resources,"automatic_refunds":false,"account_wide_billing_cutoff":false,"total_allocator_directory_loss_safe":false,"allocator_authority_state_must_be_retained":true,"automatic_allocator_recovery_supported":false}),
        )
    }
}

fn lease(path: &Path) -> Result<File, String> {
    let path = PathBuf::from(format!("{}.lock", path.display()));
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let file = options.open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("budget lease is not a regular file".into());
    }
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err("budget session already has a live local owner".into());
        }
    }
    #[cfg(not(unix))]
    {
        return Err("durable budget sessions require Unix flock".into());
    }
    Ok(file)
}

fn boot_id() -> Result<String, String> {
    let mut text = String::new();
    File::open("/proc/sys/kernel/random/boot_id")
        .and_then(|f| f.take(129).read_to_string(&mut text))
        .map_err(|e| format!("host boot identity unavailable: {e}"))?;
    let text = text.trim();
    identifier(text)?;
    Ok(text.into())
}

fn debit(
    conn: &Connection,
    state: &mut WorkerState,
    scope: &BudgetScope,
    charge: &Resources,
) -> Result<(), String> {
    charge.sqlite_safe()?;
    match scope {
        BudgetScope::Work => {
            let next = state.work_used.checked_add(charge)?;
            state
                .grant
                .request
                .resources
                .checked_sub(&state.grant.request.finalization)?
                .checked_sub(&next)?;
            state.work_used = next;
        }
        BudgetScope::Finalization => {
            let next = state.finalization_used.checked_add(charge)?;
            state.grant.request.finalization.checked_sub(&next)?;
            state.finalization_used = next;
        }
        BudgetScope::Cell(id) => {
            let (envelope, used): (String, String) = conn
                .query_row("SELECT envelope,used FROM cells WHERE id=?1", [id], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })
                .map_err(|e| format!("cell budget missing: {e}"))?;
            let envelope: Resources = decode(&envelope)?;
            let used: Resources = decode(&used)?;
            let next = used.checked_add(charge)?;
            envelope.checked_sub(&next)?;
            conn.execute(
                "UPDATE cells SET used=?1 WHERE id=?2",
                params![encode(&next)?, id],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

impl BudgetSession {
    fn check_path(path: &Path, grant: &WorkerGrant) -> Result<(), String> {
        if path.to_str() != Some(grant.request.worker_journal_path.as_str()) {
            return Err("worker journal path differs from the grant; bootstrap balance may not move to a new path".into());
        }
        Ok(())
    }

    fn authority_record(
        path: &Path,
        grant: &WorkerGrant,
        nonce: &str,
        boot: &str,
    ) -> Result<String, String> {
        encode(
            &json!({"schema":"crypto.ml-dataset.budget-worker-authority/v1","grant_sha256":grant.sha256()?,"worker_journal_path":path,"session_nonce":nonce,"host_boot_id":boot}),
        )
    }

    fn create_authority(
        path: &Path,
        grant: &WorkerGrant,
        nonce: &str,
        boot: &str,
    ) -> Result<(), String> {
        use std::io::Write;
        let authority = PathBuf::from(format!("{}.authority", path.display()));
        let text = Self::authority_record(path, grant, nonce, boot)?;
        let mut file=OpenOptions::new().create_new(true).write(true).open(&authority).map_err(|e|format!("worker authority already exists or cannot be created; never reset bootstrap quota: {e}"))?;
        file.write_all(text.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        File::open(path.parent().ok_or("missing worker authority parent")?)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn check_authority(path: &Path, state: &WorkerState) -> Result<(), String> {
        let authority = PathBuf::from(format!("{}.authority", path.display()));
        let meta = fs::symlink_metadata(&authority)
            .map_err(|_| "worker authority missing; cannot reconstruct bootstrap quota")?;
        if !meta.is_file() || meta.len() > 4096 {
            return Err("worker authority must be a bounded regular file".into());
        }
        let mut text = String::new();
        File::open(authority)
            .and_then(|f| f.take(4097).read_to_string(&mut text))
            .map_err(|e| e.to_string())?;
        if text != Self::authority_record(path, &state.grant, &state.nonce, &state.host_boot_id)? {
            return Err("worker authority differs from journal identity".into());
        }
        Ok(())
    }

    pub fn create(
        path: &Path,
        policy: &CampaignPolicy,
        grant: &WorkerGrant,
        owner: &OwnerIdentity,
    ) -> Result<Self, String> {
        grant.validate(policy)?;
        if grant.request.owner != *owner {
            return Err("grant owner mismatch".into());
        }
        let path = db_path(path, true)?;
        Self::check_path(&path, grant)?;
        let held = lease(&path)?;
        if fs::symlink_metadata(&path).is_ok() {
            return Err("worker journal already exists; resume, never recreate its balance".into());
        }
        let mut random = [0u8; 32];
        File::open("/dev/urandom")
            .and_then(|mut f| f.read_exact(&mut random))
            .map_err(|e| format!("activation nonce: {e}"))?;
        let nonce = hex::encode(random);
        let host_boot_id = boot_id()?;
        Self::create_authority(&path, grant, &nonce, &host_boot_id)?;
        let mut conn = database(&path, true, false)?;
        let state = WorkerState {
            policy: policy.clone(),
            grant: grant.clone(),
            nonce: nonce.clone(),
            phase: "activation_pending".into(),
            work_used: Resources::default(),
            finalization_used: Resources::default(),
            next_attempt: 1,
            journal_path: path
                .to_str()
                .ok_or("budget journal path must be UTF-8")?
                .into(),
            host_boot_id,
        };
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        write_state(&tx, "worker", &state)?;
        event(
            &tx,
            "worker_created",
            json!({"grant_sha256":grant.sha256()?,"session_nonce":nonce,"owner":owner}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(Self {
            connection: Mutex::new(conn),
            _lease: held,
            policy: policy.clone(),
            grant: grant.clone(),
            nonce,
        })
    }

    pub fn open(
        path: &Path,
        policy: &CampaignPolicy,
        grant: &WorkerGrant,
        owner: &OwnerIdentity,
    ) -> Result<Self, String> {
        let path = db_path(path, false)?;
        Self::check_path(&path, grant)?;
        if !path.exists() {
            return Err(
                "budget session state is missing; a grant may not reset its balance".into(),
            );
        }
        let held = lease(&path)?;
        let conn = database(&path, false, false)?;
        verify_events(&conn)?;
        let state: WorkerState = read_state(&conn, "worker")?;
        Self::check_authority(&path, &state)?;
        if state.policy != *policy || state.grant != *grant || grant.request.owner != *owner {
            return Err("worker policy/grant/owner binding changed".into());
        }
        if state.journal_path != path.to_str().ok_or("budget journal path must be UTF-8")?
            || state.host_boot_id != boot_id()?
        {
            return Err("worker journal was copied or host changed; reconcile original allocation without resetting it".into());
        }
        grant
            .request
            .resources
            .checked_sub(&grant.request.finalization)?
            .checked_sub(&state.work_used)?;
        grant
            .request
            .finalization
            .checked_sub(&state.finalization_used)?;
        sha(&state.nonce)?;
        Ok(Self {
            connection: Mutex::new(conn),
            _lease: held,
            policy: policy.clone(),
            grant: grant.clone(),
            nonce: state.nonce,
        })
    }

    pub fn policy(&self) -> &CampaignPolicy {
        &self.policy
    }
    pub fn grant(&self) -> &WorkerGrant {
        &self.grant
    }
    pub fn validate_binding(&self, binding: &RunBinding) -> Result<(), String> {
        if self.grant.request.binding != *binding {
            return Err("run source/binary/protocol/plan/shard differs from budget grant".into());
        }
        self.require_active()
    }
    pub fn validate_remote(&self, bucket: &str, object_key: &str) -> Result<(), String> {
        self.policy.validate_remote(bucket, object_key)
    }
    pub fn activation_location(&self) -> (&str, &str) {
        (&self.grant.activation_bucket, &self.grant.activation_key)
    }
    pub fn activation_payload(&self) -> Result<Vec<u8>, String> {
        Ok(encode(&json!({"schema":"crypto.ml-dataset.budget-activation/v1","grant_sha256":self.grant.sha256()?,"session_nonce":self.nonce,"owner":self.grant.request.owner}))?.into_bytes())
    }

    fn live(&self, state: &WorkerState) -> Result<(), String> {
        if state.policy != self.policy || state.grant != self.grant || state.nonce != self.nonce {
            return Err("live budget binding changed".into());
        }
        if now()? >= self.grant.request.expires_unix_seconds
            || now()? >= self.policy.deadline_unix_seconds
        {
            return Err("budget grant/campaign deadline expired".into());
        }
        if state.phase == "closed" {
            return Err("budget session is closed; allocation remains consumed".into());
        }
        Ok(())
    }

    pub fn require_active(&self) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let state: WorkerState = read_state(&conn, "worker")?;
        self.live(&state)?;
        if state.phase != "active" {
            return Err("budget session requires verified immutable remote activation".into());
        }
        Ok(())
    }

    /// Read the retained live phase without credentials, remote requests or
    /// accounting changes. In particular, an already active worker must not
    /// repeat activation merely because the gate's later transition was lost.
    pub(super) fn activation_phase(&self) -> Result<ActivationPhase, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let state: WorkerState = read_state(&conn, "worker")?;
        self.live(&state)?;
        match state.phase.as_str() {
            "activation_pending" => Ok(ActivationPhase::Pending),
            "active" => Ok(ActivationPhase::Active),
            _ => Err("worker activation phase is invalid for startup recovery".into()),
        }
    }

    fn operation_allowed(&self, state: &WorkerState, op: &HttpOperation) -> Result<(), String> {
        self.live(state)?;
        if op.method == HttpMethod::StsWebIdentity {
            let role = self
                .grant
                .request
                .credential_role_arn
                .as_ref()
                .ok_or("grant does not admit web-identity credentials")?;
            let role_hash = hex::encode(crate::hash::sha256::sha256(role.as_bytes()));
            identifier(&op.operation_id)?;
            if op.bucket != "sts.us-east-2.amazonaws.com"
                || op.key != role_hash
                || op.expected_bytes != 65536
                || op.expected_sha256.is_some()
                || op.scope != BudgetScope::Finalization
            {
                return Err(
                    "STS attempt differs from the exact role/region/response envelope".into(),
                );
            }
            return Ok(());
        }
        self.validate_remote(&op.bucket, &op.key)?;
        identifier(&op.operation_id)?;
        if let Some(digest) = &op.expected_sha256 {
            sha(digest)?;
        }
        if op.expected_bytes > i64::MAX as u64 {
            return Err("request size outside accounting range".into());
        }
        if op.method == HttpMethod::Head && op.expected_bytes != 0 {
            return Err("HEAD must reserve zero body bytes".into());
        }
        if op.method == HttpMethod::Put && op.expected_sha256.is_none() {
            return Err("PUT requires an expected digest".into());
        }
        if state.phase != "active" {
            let bytes = self.activation_payload()?;
            let digest = hex::encode(crate::hash::sha256::sha256(&bytes));
            if state.phase != "activation_pending"
                || op.bucket != self.grant.activation_bucket
                || op.key != self.grant.activation_key
                || op.scope != BudgetScope::Finalization
                || (op.method != HttpMethod::Head
                    && (op.expected_bytes != bytes.len() as u64
                        || op.expected_sha256.as_deref() != Some(digest.as_str())))
            {
                return Err(
                    "pending budget permits only its exact charged activation object".into(),
                );
            }
        }
        Ok(())
    }

    pub fn reserve_cell(&self, id: &str, envelope: &Resources) -> Result<CellReservation, String> {
        identifier(id)?;
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: WorkerState = read_state(&tx, "worker")?;
        self.live(&state)?;
        if state.phase != "active" {
            return Err("activate budget before reserving a cell".into());
        }
        let previous: Option<String> = tx
            .query_row("SELECT envelope FROM cells WHERE id=?1", [id], |r| r.get(0))
            .optional()
            .map_err(|e| e.to_string())?;
        if let Some(previous) = previous {
            if decode::<Resources>(&previous)? != *envelope {
                return Err("cell reservation cannot change on resume".into());
            }
            return Ok(CellReservation {
                id: id.into(),
                envelope: envelope.clone(),
            });
        }
        debit(&tx, &mut state, &BudgetScope::Work, envelope)?;
        tx.execute(
            "INSERT INTO cells(id,envelope,used) VALUES(?1,?2,?3)",
            params![id, encode(envelope)?, encode(&Resources::default())?],
        )
        .map_err(|e| e.to_string())?;
        write_state(&tx, "worker", &state)?;
        event(&tx, "cell_reserved", json!({"id":id,"envelope":envelope}))?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(CellReservation {
            id: id.into(),
            envelope: envelope.clone(),
        })
    }

    pub fn reserve_object(
        &self,
        bucket: &str,
        object_key: &str,
        digest: &str,
        bytes: u64,
        scope: &BudgetScope,
    ) -> Result<ObjectReservation, String> {
        sha(digest)?;
        let probe = HttpOperation {
            operation_id: "object-reservation".into(),
            method: HttpMethod::Put,
            bucket: bucket.into(),
            key: object_key.into(),
            expected_sha256: Some(digest.into()),
            expected_bytes: bytes,
            scope: scope.clone(),
        };
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: WorkerState = read_state(&tx, "worker")?;
        self.operation_allowed(&state, &probe)?;
        let previous: Option<String> = tx
            .query_row(
                "SELECT json FROM objects WHERE bucket=?1 AND key=?2",
                params![bucket, object_key],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        if let Some(previous) = previous {
            let previous: ObjectReservation = decode(&previous)?;
            if previous.sha256 != digest || previous.bytes != bytes {
                return Err("remote key already reserved with different bytes/digest".into());
            }
            return Ok(previous);
        }
        let reservation = ObjectReservation {
            grant_sha256: self.grant.sha256()?,
            bucket: bucket.into(),
            key: object_key.into(),
            sha256: digest.into(),
            bytes,
            scope: scope.clone(),
        };
        debit(
            &tx,
            &mut state,
            scope,
            &Resources {
                output_bytes: bytes,
                ..Resources::default()
            },
        )?;
        tx.execute(
            "INSERT INTO objects(bucket,key,json) VALUES(?1,?2,?3)",
            params![bucket, object_key, encode(&reservation)?],
        )
        .map_err(|e| e.to_string())?;
        write_state(&tx, "worker", &state)?;
        event(&tx, "object_reserved", json!({"reservation":reservation}))?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(reservation)
    }

    pub fn before_http_attempt(&self, operation: &HttpOperation) -> Result<AttemptTicket, String> {
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: WorkerState = read_state(&tx, "worker")?;
        self.operation_allowed(&state, operation)?;
        if operation.method == HttpMethod::Put {
            let reserved: String = tx
                .query_row(
                    "SELECT json FROM objects WHERE bucket=?1 AND key=?2",
                    params![operation.bucket, operation.key],
                    |r| r.get(0),
                )
                .map_err(|_| "reserve object bytes before PUT")?;
            let reserved: ObjectReservation = decode(&reserved)?;
            if Some(&reserved.sha256) != operation.expected_sha256.as_ref()
                || reserved.bytes != operation.expected_bytes
            {
                return Err("PUT differs from its durable object reservation".into());
            }
        }
        let charge = match operation.method {
            HttpMethod::Head => Resources {
                read_attempts: 1,
                ..Resources::default()
            },
            HttpMethod::Get => Resources {
                read_attempts: 1,
                download_bytes: operation.expected_bytes,
                ..Resources::default()
            },
            HttpMethod::Put => Resources {
                put_attempts: 1,
                ..Resources::default()
            },
            HttpMethod::StsWebIdentity => Resources {
                read_attempts: 1,
                download_bytes: 65536,
                ..Resources::default()
            },
        };
        debit(&tx, &mut state, &operation.scope, &charge)?;
        let ticket = AttemptTicket {
            grant_sha256: self.grant.sha256()?,
            session_nonce: self.nonce.clone(),
            attempt_id: state.next_attempt,
            operation: operation.clone(),
        };
        state.next_attempt = state
            .next_attempt
            .checked_add(1)
            .filter(|v| *v <= i64::MAX as u64)
            .ok_or("attempt counter overflow")?;
        tx.execute(
            "INSERT INTO attempts(id,ticket,outcome) VALUES(?1,?2,NULL)",
            params![ticket.attempt_id, encode(&ticket)?],
        )
        .map_err(|e| e.to_string())?;
        write_state(&tx, "worker", &state)?;
        event(
            &tx,
            "attempt_charged",
            json!({"ticket":ticket,"charge":charge,"outcome":"unknown_until_recorded"}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(ticket)
    }

    fn stored_ticket(
        &self,
        conn: &Connection,
        ticket: &AttemptTicket,
    ) -> Result<Option<AttemptOutcome>, String> {
        if ticket.grant_sha256 != self.grant.sha256()? || ticket.session_nonce != self.nonce {
            return Err("attempt belongs to another grant/session".into());
        }
        let (stored, outcome): (String, Option<String>) = conn
            .query_row(
                "SELECT ticket,outcome FROM attempts WHERE id=?1",
                [ticket.attempt_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|e| e.to_string())?;
        if decode::<AttemptTicket>(&stored)? != *ticket {
            return Err("attempt ticket was altered".into());
        }
        outcome.map(|v| decode(&v)).transpose()
    }

    pub fn finish_attempt(
        &self,
        ticket: &AttemptTicket,
        outcome: &AttemptOutcome,
    ) -> Result<(), String> {
        if outcome.status.is_some_and(|s| !(100..=599).contains(&s))
            || outcome.request_id.as_ref().is_some_and(|s| s.len() > 512)
            || outcome.error.as_ref().is_some_and(|s| s.len() > 4096)
        {
            return Err("invalid bounded request outcome".into());
        }
        if matches!(
            ticket.operation.method,
            HttpMethod::Get | HttpMethod::StsWebIdentity
        ) && outcome.observed_bytes > ticket.operation.expected_bytes
        {
            return Err("response exceeded its admitted byte bound".into());
        }
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        if let Some(previous) = self.stored_ticket(&tx, ticket)? {
            if previous != *outcome {
                return Err("attempt outcome is immutable; unknown is not refunded".into());
            }
            return Ok(());
        }
        tx.execute(
            "UPDATE attempts SET outcome=?1 WHERE id=?2",
            params![encode(outcome)?, ticket.attempt_id],
        )
        .map_err(|e| e.to_string())?;
        event(
            &tx,
            "attempt_observed",
            json!({"attempt_id":ticket.attempt_id,"outcome":outcome}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn verified_get(
        &self,
        conn: &Connection,
        ticket: &AttemptTicket,
        bucket: &str,
        object_key: &str,
        digest: &str,
        bytes: u64,
    ) -> Result<(), String> {
        let outcome = self
            .stored_ticket(conn, ticket)?
            .ok_or("GET has no observed outcome")?;
        if ticket.operation.method != HttpMethod::Get
            || ticket.operation.bucket != bucket
            || ticket.operation.key != object_key
            || ticket.operation.expected_bytes != bytes
            || ticket.operation.expected_sha256.as_deref() != Some(digest)
            || outcome.status != Some(200)
            || outcome.error.is_some()
            || outcome.observed_bytes != bytes
        {
            return Err("object verification requires a matching successful full GET and exact digest/length".into());
        }
        Ok(())
    }

    pub fn verify_object(
        &self,
        reservation: &ObjectReservation,
        get: &AttemptTicket,
        digest: &str,
        bytes: u64,
    ) -> Result<(), String> {
        if digest != reservation.sha256
            || bytes != reservation.bytes
            || reservation.grant_sha256 != self.grant.sha256()?
        {
            return Err("object readback differs from reservation".into());
        }
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let stored: String = tx
            .query_row(
                "SELECT json FROM objects WHERE bucket=?1 AND key=?2",
                params![reservation.bucket, reservation.key],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        if decode::<ObjectReservation>(&stored)? != *reservation {
            return Err("object reservation changed".into());
        }
        self.verified_get(
            &tx,
            get,
            &reservation.bucket,
            &reservation.key,
            digest,
            bytes,
        )?;
        tx.execute(
            "UPDATE objects SET verified=1 WHERE bucket=?1 AND key=?2",
            params![reservation.bucket, reservation.key],
        )
        .map_err(|e| e.to_string())?;
        event(
            &tx,
            "object_verified",
            json!({"reservation":reservation,"get_attempt":get.attempt_id}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn confirm_activation(
        &self,
        get: &AttemptTicket,
        digest: &str,
        bytes: u64,
    ) -> Result<(), String> {
        let payload = self.activation_payload()?;
        if payload.len() as u64 != bytes
            || hex::encode(crate::hash::sha256::sha256(&payload)) != digest
        {
            return Err("activation body differs from this journal nonce".into());
        }
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: WorkerState = read_state(&tx, "worker")?;
        self.live(&state)?;
        self.verified_get(
            &tx,
            get,
            &self.grant.activation_bucket,
            &self.grant.activation_key,
            digest,
            bytes,
        )?;
        if state.phase == "active" {
            return Ok(());
        }
        if state.phase != "activation_pending" {
            return Err("invalid activation phase".into());
        }
        state.phase = "active".into();
        write_state(&tx, "worker", &state)?;
        event(
            &tx,
            "activation_verified",
            json!({"get_attempt":get.attempt_id,"sha256":digest,"bytes":bytes}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// No automatic reclaim: closing keeps the entire campaign allocation held.
    pub fn close(&self, reason: &str) -> Result<(), String> {
        if reason.is_empty() || reason.len() > 4096 {
            return Err("bounded close reason required".into());
        }
        let mut conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| e.to_string())?;
        let mut state: WorkerState = read_state(&tx, "worker")?;
        state.phase = "closed".into();
        write_state(&tx, "worker", &state)?;
        event(
            &tx,
            "session_closed_without_refund",
            json!({"reason":reason}),
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<Value, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let state: WorkerState = read_state(&conn, "worker")?;
        let attempts: u64 = conn
            .query_row("SELECT COUNT(*) FROM attempts", [], |r| r.get(0))
            .map_err(|e| e.to_string())?;
        let mut unknown = 0u64;
        let mut methods = [0u64; 4];
        let mut statement = conn
            .prepare("SELECT ticket,outcome FROM attempts ORDER BY id")
            .map_err(|e| e.to_string())?;
        let mut rows = statement.query([]).map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let ticket: AttemptTicket =
                decode(&row.get::<_, String>(0).map_err(|e| e.to_string())?)?;
            let outcome: Option<String> = row.get(1).map_err(|e| e.to_string())?;
            if outcome
                .map(|s| decode::<AttemptOutcome>(&s))
                .transpose()?
                .is_none_or(|o| o.status.is_none())
            {
                unknown += 1;
            }
            methods[match ticket.operation.method {
                HttpMethod::Head => 0,
                HttpMethod::Get => 1,
                HttpMethod::Put => 2,
                HttpMethod::StsWebIdentity => 3,
            }] += 1;
        }
        let mut verified_bytes = 0u64;
        let mut reserved_bytes = 0u64;
        let mut statement = conn
            .prepare("SELECT json,verified FROM objects")
            .map_err(|e| e.to_string())?;
        let mut rows = statement.query([]).map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let reservation: ObjectReservation =
                decode(&row.get::<_, String>(0).map_err(|e| e.to_string())?)?;
            reserved_bytes = reserved_bytes
                .checked_add(reservation.bytes)
                .ok_or("object bytes overflow")?;
            if row.get::<_, bool>(1).map_err(|e| e.to_string())? {
                verified_bytes = verified_bytes
                    .checked_add(reservation.bytes)
                    .ok_or("verified bytes overflow")?;
            }
        }
        Ok(
            json!({"schema":"crypto.ml-dataset.budget-session-status/v1","grant_sha256":self.grant.sha256()?,"policy_sha256":self.policy.sha256()?,"phase":state.phase,"work_reserved_or_charged":state.work_used,"finalization_charged":state.finalization_used,"finalization_remaining":state.grant.request.finalization.checked_sub(&state.finalization_used)?,"charged_attempts":attempts,"head_attempts":methods[0],"get_attempts":methods[1],"put_attempts":methods[2],"sts_attempts":methods[3],"outcome_unknown_attempts":unknown,"reserved_unique_output_bytes":reserved_bytes,"verified_unique_output_bytes":verified_bytes,"allocation_retained":self.grant.request.resources,"automatic_refunds":false,"local_byte_allocation_is_hard_write_limit":false,"account_wide_billing_cutoff":false,"cross_host_fencing":"immutable_activation_nonce_and_journal_path_and_host_boot_binding","rollback_protection_scope":"authoritative_local_database_not_restored_from_stale_backup"}),
        )
    }

    pub fn finalization_remaining(&self) -> Result<Resources, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        let state: WorkerState = read_state(&conn, "worker")?;
        state
            .grant
            .request
            .finalization
            .checked_sub(&state.finalization_used)
    }

    /// A consistent audit copy, including the full hash chain and attempts. It
    /// is deliberately bound to the original journal path and boot identity;
    /// never restore a stale copy as a new spendable balance.
    pub fn checkpoint(&self, destination: &Path) -> Result<Value, String> {
        let path = db_path(destination, true)?;
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        let conn = self
            .connection
            .lock()
            .map_err(|_| "poisoned budget session")?;
        conn.execute(
            "VACUUM INTO ?1",
            [path.to_str().ok_or("checkpoint path must be UTF-8")?],
        )
        .map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        File::open(path.parent().ok_or("missing checkpoint parent")?)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        let (digest, bytes) = crate::store::hash_file(&path)?;
        let (sequence, head): (u64, String) = conn
            .query_row(
                "SELECT seq,sha FROM events ORDER BY seq DESC LIMIT 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|e| e.to_string())?;
        Ok(
            json!({"schema":"crypto.ml-dataset.budget-checkpoint/v1","grant_sha256":self.grant.sha256()?,"sha256":digest,"bytes":bytes,"journal_sequence":sequence,"journal_head_sha256":head,"audit_only":true,"resume_from_copy_permitted":false}),
        )
    }
}

/// Shared non-scientific fixture for CLI/transport tests; this is not launch
/// authority or real pricing evidence. Callers supply an existing temp directory.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn test_policy_request(root: &Path) -> (CampaignPolicy, GrantRequest) {
    let allocator = fs::canonicalize(root).unwrap().join("campaign.sqlite");
    let policy = CampaignPolicy {
        schema: POLICY_SCHEMA.into(),
        campaign_id: "test-campaign".into(),
        deadline_unix_seconds: now().unwrap() + 3600,
        caps: Resources {
            money_micro_usd: 200_000_000,
            output_bytes: 1024 * 1024,
            put_attempts: 1000,
            read_attempts: 2000,
            download_bytes: 8 * 1024 * 1024,
            local_bytes: 8 * 1024 * 1024,
        },
        prior: Resources {
            money_micro_usd: 188_388_800,
            ..Resources::default()
        },
        prior_noncompute_reserve_micro_usd: 25_000_000,
        noncompute_prices: NoncomputePrices {
            put_micro_usd_per_1000: 5000,
            read_micro_usd_per_1000: 400,
            storage_micro_usd_per_gib_day: 1000,
            download_micro_usd_per_gib: 100_000,
            fixed_micro_usd: 1_000_000,
        },
        prior_usage_reconciled: true,
        prior_evidence_sha256: vec!["1".repeat(64)],
        region: "us-east-2".into(),
        pricing_region: "us-east-2".into(),
        price_evidence_sha256: "2".repeat(64),
        resource_envelope_sha256: "3".repeat(64),
        resource_lifetime_bound: true,
        compute_rate_micro_usd_per_hour: 3600,
        billable_resource_seconds: 1,
        billable_resource_count: 1,
        allocator_ledger_path: allocator.to_str().unwrap().into(),
        allocator_host_boot_id: boot_id().unwrap(),
        storage_horizon_days: 30,
        allowed_remotes: vec![RemoteScope {
            bucket: "setec-astronomy-artifacts".into(),
            prefix: "ml-dataset/test/".into(),
        }],
        coordination_prefix: "ml-dataset/test/budget/".into(),
    };
    let request = GrantRequest {
        grant_id: "test-grant".into(),
        worker_journal_path: fs::canonicalize(root)
            .unwrap()
            .join("worker.sqlite")
            .to_str()
            .unwrap()
            .into(),
        owner: OwnerIdentity {
            job_uid: "job-1".into(),
            pod_uid: "pod-1".into(),
        },
        binding: RunBinding {
            source_commit: "a".repeat(40),
            binary_sha256: "b".repeat(64),
            plan_sha256: "c".repeat(64),
            protocol_sha256: "d".repeat(64),
            shard_index: 0,
            shard_count: 1,
        },
        expires_unix_seconds: policy.deadline_unix_seconds,
        resource_envelope_sha256: policy.resource_envelope_sha256.clone(),
        resources: Resources {
            money_micro_usd: 100,
            output_bytes: 128 * 1024,
            put_attempts: 30,
            read_attempts: 50,
            download_bytes: 1024 * 1024,
            local_bytes: 1024 * 1024,
        },
        finalization: Resources {
            money_micro_usd: 0,
            output_bytes: 16 * 1024,
            put_attempts: 10,
            read_attempts: 10,
            download_bytes: 256 * 1024,
            local_bytes: 4096,
        },
        credential_role_arn: None,
    };
    (policy, request)
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Barrier,
    };

    thread_local! {
        pub(super) static CLOCK: Cell<Option<u64>> = const { Cell::new(None) };
    }

    struct ClockGuard(Option<u64>);
    impl ClockGuard {
        fn at(timestamp: u64) -> Self {
            Self(CLOCK.replace(Some(timestamp)))
        }
    }
    impl Drop for ClockGuard {
        fn drop(&mut self) {
            CLOCK.set(self.0);
        }
    }

    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "dataset-budget-{}-{unique}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn grant(root: &Path) -> (CampaignPolicy, WorkerGrant) {
        let (policy, request) = test_policy_request(root);
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let grant = ledger.reserve(&request).unwrap();
        (policy, grant)
    }

    fn launch_request(request: &GrantRequest) -> LaunchRequest {
        LaunchRequest {
            schema: LAUNCH_REQUEST_SCHEMA.into(),
            launch_id: request.grant_id.clone(),
            target: LaunchTarget {
                cluster_identity: "arn:aws:eks:us-east-2:123456789012:cluster/software-test".into(),
                namespace: "isolab".into(),
                job_name: "software-only-launch".into(),
            },
            worker_journal_path: request.worker_journal_path.clone(),
            binding: request.binding.clone(),
            expires_unix_seconds: request.expires_unix_seconds,
            resource_envelope_sha256: request.resource_envelope_sha256.clone(),
            resources: request.resources.clone(),
            finalization: request.finalization.clone(),
            credential_role_arn: request.credential_role_arn.clone(),
        }
    }

    fn launch_binding(reservation: &LaunchReservation) -> LaunchBinding {
        LaunchBinding {
            schema: LAUNCH_BINDING_SCHEMA.into(),
            launch_id: reservation.request.launch_id.clone(),
            reservation_sha256: reservation.sha256().unwrap(),
            observed_target: reservation.request.target.clone(),
            owner: OwnerIdentity {
                job_uid: "observed-test-job-uid".into(),
                pod_uid: "observed-test-pod-uid".into(),
            },
            observation_evidence_sha256: "e".repeat(64),
        }
    }

    fn events(ledger: &CampaignLedger) -> u64 {
        ledger
            .connection
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn startup_delivery_checks_complete_grant_owner_target_and_live_expiry() {
        let temp = Temp::new();
        let (policy, request) = test_policy_request(&temp.0);
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let reservation = ledger.reserve_launch(&launch_request(&request)).unwrap();
        let binding = launch_binding(&reservation);
        let grant = ledger.bind_launch(&binding).unwrap();
        let validate = |r: &LaunchReservation, b: &LaunchBinding, g: &WorkerGrant| {
            validate_launch_delivery(&policy, r, b, g, &binding.owner)
        };
        let before = events(&ledger);
        validate(&reservation, &binding, &grant).unwrap();
        assert_eq!(events(&ledger), before);

        let mut changed = grant.clone();
        changed.request.resources.put_attempts += 1;
        // The new allowance is independently valid, but was never reserved.
        changed.validate(&policy).unwrap();
        assert!(validate(&reservation, &binding, &changed).is_err());
        let mut changed = grant.clone();
        changed.request.owner.pod_uid = "other-pod".into();
        assert!(validate(&reservation, &binding, &changed).is_err());
        let mut changed = grant.clone();
        changed.activation_key.push_str("-other");
        assert!(validate(&reservation, &binding, &changed).is_err());

        let mut changed = binding.clone();
        changed.observed_target.job_name = "other-job".into();
        assert!(validate(&reservation, &changed, &grant).is_err());
        let mut changed = binding.clone();
        changed.owner.job_uid = "other-job-uid".into();
        assert!(validate(&reservation, &changed, &grant).is_err());
        let mut changed = binding.clone();
        changed.observation_evidence_sha256 = "not-a-digest".into();
        assert!(validate(&reservation, &changed, &grant).is_err());
        let mut changed = reservation.clone();
        changed.policy_sha256 = "f".repeat(64);
        assert!(validate(&changed, &binding, &grant).is_err());
        let other = OwnerIdentity {
            job_uid: binding.owner.job_uid.clone(),
            pod_uid: "different-observed-pod".into(),
        };
        assert!(validate_launch_delivery(&policy, &reservation, &binding, &grant, &other).is_err());
        let _clock = ClockGuard::at(grant.request.expires_unix_seconds);
        assert!(validate(&reservation, &binding, &grant).is_err());
    }

    #[test]
    fn launch_dry_admission_preserves_legacy_state_and_never_creates_files() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let request = launch_request(&direct);
        let path = Path::new(&policy.allocator_ledger_path);
        let report = CampaignLedger::dry_admit_launch(path, &policy, &request).unwrap();
        assert!(report.admitted && report.dry_run);
        assert_eq!(report.additional_requested, request.resources);
        assert_eq!(fs::read_dir(&temp.0).unwrap().count(), 0);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        let raw: String = ledger
            .connection
            .lock()
            .unwrap()
            .query_row("SELECT json FROM state WHERE id=1", [], |row| row.get(0))
            .unwrap();
        assert!(serde_json::from_str::<Value>(&raw)
            .unwrap()
            .get("launches")
            .is_none());
        let legacy: CampaignState = decode(&raw).unwrap();
        assert!(legacy.launches.is_empty());
        assert_eq!(encode(&legacy).unwrap(), raw);
        let before = fs::read(path).unwrap();
        let event_count = events(&ledger);
        assert!(
            CampaignLedger::dry_admit_launch(path, &policy, &request)
                .unwrap()
                .admitted
        );
        ledger.snapshot().unwrap();
        drop(CampaignLedger::open(path, &policy).unwrap());
        assert_eq!(fs::read(path).unwrap(), before);
        assert_eq!(events(&ledger), event_count);
        assert_eq!(fs::read_dir(&temp.0).unwrap().count(), 2);
        let original_grant = ledger.reserve(&direct).unwrap();
        assert_eq!(original_grant.request, direct);
        assert_eq!(original_grant.schema, GRANT_SCHEMA);
        assert!(
            !CampaignLedger::dry_admit_launch(path, &policy, &request)
                .unwrap()
                .admitted
        );
        assert!(ledger.reserve_launch(&request).is_err());
    }

    #[test]
    fn launch_binding_preserves_all_resource_coordinates_and_exact_retries() {
        let temp = Temp::new();
        let (policy, mut direct) = test_policy_request(&temp.0);
        let request = launch_request(&direct);
        let path = Path::new(&policy.allocator_ledger_path);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        let reservation = ledger.reserve_launch(&request).unwrap();
        let before = ledger.snapshot().unwrap();
        assert_eq!(before["pending_launch_count"], 1);
        assert_eq!(before["bound_launch_count"], 0);
        assert_eq!(before["grants"], json!({}));
        assert_eq!(events(&ledger), 2);
        let dry = CampaignLedger::dry_admit_launch(path, &policy, &request).unwrap();
        assert!(dry.admitted && !dry.already_bound);
        assert_eq!(dry.additional_requested, Resources::default());
        assert_eq!(
            dry.existing_reservation_sha256,
            Some(reservation.sha256().unwrap())
        );
        assert_eq!(ledger.reserve_launch(&request).unwrap(), reservation);
        assert_eq!(events(&ledger), 2);
        assert!(ledger.reserve(&direct).is_err());
        direct.grant_id = "separate-direct-grant".into();
        let dry_direct = CampaignLedger::dry_admit(path, &policy, &direct).unwrap();
        assert_eq!(
            serde_json::to_value(dry_direct.prior_and_allocated).unwrap(),
            before["prior_and_allocated"]
        );
        let binding = launch_binding(&reservation);
        let grant = ledger.bind_launch(&binding).unwrap();
        grant.validate(&policy).unwrap();
        let after = ledger.snapshot().unwrap();
        assert_eq!(before["prior_and_allocated"], after["prior_and_allocated"]);
        assert_eq!(before["remaining"], after["remaining"]);
        assert_eq!(
            after["pending_launch_resources"],
            serde_json::to_value(Resources::default()).unwrap()
        );
        assert_eq!(after["pending_launch_count"], 0);
        assert_eq!(after["bound_launch_count"], 1);
        assert_eq!(grant.request, request.with_owner(&binding.owner));
        assert_eq!(ledger.bind_launch(&binding).unwrap(), grant);
        assert_eq!(ledger.reserve_launch(&request).unwrap(), reservation);
        assert_eq!(events(&ledger), 3);
        let dry = CampaignLedger::dry_admit_launch(path, &policy, &request).unwrap();
        assert!(!dry.admitted && dry.already_bound);
        assert_eq!(dry.additional_requested, Resources::default());
        direct.owner = binding.owner.clone();
        direct.owner.pod_uid = "replacement-pod".into();
        assert!(ledger.reserve(&direct).is_err());
    }

    #[test]
    fn launch_expiry_retains_capacity_targets_and_exact_evidence_recovery() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let path = Path::new(&policy.allocator_ledger_path);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        let mut request = launch_request(&direct);
        request.expires_unix_seconds = now().unwrap() + 60;
        let reservation = ledger.reserve_launch(&request).unwrap();
        let binding = launch_binding(&reservation);
        let grant = ledger.bind_launch(&binding).unwrap();
        let mut pending = request.clone();
        pending.launch_id = "unknown-submission".into();
        pending.target.job_name = "unknown-submission".into();
        let pending_reservation = ledger.reserve_launch(&pending).unwrap();
        let before = ledger.snapshot().unwrap();
        let count = events(&ledger);
        {
            let _grant_expired = ClockGuard::at(request.expires_unix_seconds + 1);
            policy.validate().unwrap();
            assert_eq!(ledger.bind_launch(&binding).unwrap(), grant);
            assert!(ledger
                .bind_launch(&launch_binding(&pending_reservation))
                .is_err());
            let mut reuse = pending.clone();
            reuse.launch_id = "reuse-expired-target".into();
            reuse.expires_unix_seconds = policy.deadline_unix_seconds;
            assert!(ledger.reserve_launch(&reuse).is_err());
        }
        drop(ledger);
        let _expired = ClockGuard::at(policy.deadline_unix_seconds + 1);
        assert!(policy.validate().is_err());
        assert!(grant.validate(&policy).is_err());
        let ledger = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(ledger.bind_launch(&binding).unwrap(), grant);
        assert_eq!(ledger.reserve_launch(&request).unwrap(), reservation);
        assert_eq!(
            ledger.reserve_launch(&pending).unwrap(),
            pending_reservation
        );
        assert!(ledger
            .bind_launch(&launch_binding(&pending_reservation))
            .is_err());
        assert!(
            !CampaignLedger::dry_admit_launch(path, &policy, &pending)
                .unwrap()
                .admitted
        );
        let mut reused = pending.clone();
        reused.launch_id = "new-id-same-expired-target".into();
        assert!(ledger.reserve_launch(&reused).is_err());
        assert_eq!(ledger.snapshot().unwrap(), before);
        assert_eq!(events(&ledger), count);
    }

    #[test]
    fn launch_requests_targets_and_observation_bindings_are_immutable() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let request = launch_request(&direct);
        let reservation = ledger.reserve_launch(&request).unwrap();
        let mut duplicate = request.clone();
        duplicate.launch_id = "duplicate-target".into();
        assert!(ledger.reserve_launch(&duplicate).is_err());
        let mut changed = request.clone();
        changed.resources.put_attempts += 1;
        assert!(ledger.reserve_launch(&changed).is_err());
        changed = request.clone();
        changed.target.job_name = "repointed-job".into();
        assert!(ledger.reserve_launch(&changed).is_err());
        let binding = launch_binding(&reservation);
        let before = ledger.snapshot().unwrap();
        let mut wrong = binding.clone();
        wrong.reservation_sha256 = "a".repeat(64);
        assert!(ledger.bind_launch(&wrong).is_err());
        wrong = binding.clone();
        wrong.observed_target.job_name = "wrong-job".into();
        assert!(ledger.bind_launch(&wrong).is_err());
        assert_eq!(ledger.snapshot().unwrap(), before);
        ledger.bind_launch(&binding).unwrap();
        for wrong in [
            LaunchBinding {
                owner: OwnerIdentity {
                    pod_uid: "replacement".into(),
                    ..binding.owner.clone()
                },
                ..binding.clone()
            },
            LaunchBinding {
                observation_evidence_sha256: "f".repeat(64),
                ..binding.clone()
            },
        ] {
            assert!(ledger.bind_launch(&wrong).is_err());
        }
        assert!(ledger.reserve_launch(&duplicate).is_err());
        assert_eq!(events(&ledger), 3);
        let mut invalid = request.clone();
        invalid.target.cluster_identity = "default-context".into();
        assert!(invalid.validate(&policy, true).is_err());
        invalid = request.clone();
        invalid.target.namespace = "Uppercase".into();
        assert!(invalid.validate(&policy, true).is_err());
        invalid = request.clone();
        invalid.target.job_name = "x".repeat(64);
        assert!(invalid.validate(&policy, true).is_err());
        let mut extra = serde_json::to_value(&request).unwrap();
        extra["owner"] = json!({"job_uid":"invented","pod_uid":"invented"});
        assert!(serde_json::from_value::<LaunchRequest>(extra).is_err());
        let mut extra = serde_json::to_value(&binding).unwrap();
        extra["replacement_allowed"] = json!(true);
        assert!(serde_json::from_value::<LaunchBinding>(extra).is_err());
    }

    #[test]
    fn concurrent_launch_and_direct_allocators_share_one_finite_balance() {
        let temp = Temp::new();
        let (mut policy, mut direct) = test_policy_request(&temp.0);
        policy.caps.money_micro_usd = policy.prior.money_micro_usd + 100;
        direct.resources.money_micro_usd = 60;
        let mut request = launch_request(&direct);
        request.launch_id = "launch-racer".into();
        let path = Path::new(&policy.allocator_ledger_path);
        let first = CampaignLedger::create(path, &policy).unwrap();
        let second = CampaignLedger::open(path, &policy).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let ready = barrier.clone();
        let a = std::thread::spawn(move || {
            ready.wait();
            first.reserve_launch(&request).is_ok()
        });
        let b = std::thread::spawn(move || {
            barrier.wait();
            second.reserve(&direct).is_ok()
        });
        assert_eq!(
            usize::from(a.join().unwrap()) + usize::from(b.join().unwrap()),
            1
        );
        let ledger = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(
            ledger.snapshot().unwrap()["remaining"]["money_micro_usd"],
            40
        );
        assert_eq!(events(&ledger), 2);
    }

    #[test]
    fn concurrent_duplicate_targets_and_competing_owner_bindings_have_one_winner() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let request = launch_request(&direct);
        let path = Path::new(&policy.allocator_ledger_path);
        let first = CampaignLedger::create(path, &policy).unwrap();
        let second = CampaignLedger::open(path, &policy).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let handles: Vec<_> = [first, second]
            .into_iter()
            .enumerate()
            .map(|(i, ledger)| {
                let ready = barrier.clone();
                let mut request = request.clone();
                request.launch_id = format!("launch-{i}");
                std::thread::spawn(move || {
                    ready.wait();
                    ledger.reserve_launch(&request).ok()
                })
            })
            .collect();
        let winners: Vec<_> = handles
            .into_iter()
            .filter_map(|h| h.join().unwrap())
            .collect();
        assert_eq!(winners.len(), 1);
        let reservation = &winners[0];
        let barrier = Arc::new(Barrier::new(2));
        let handles: Vec<_> = (0..2)
            .map(|i| {
                let ledger = CampaignLedger::open(path, &policy).unwrap();
                let ready = barrier.clone();
                let mut binding = launch_binding(reservation);
                binding.owner.pod_uid = format!("observed-pod-{i}");
                std::thread::spawn(move || {
                    ready.wait();
                    ledger.bind_launch(&binding).is_ok()
                })
            })
            .collect();
        assert_eq!(
            handles
                .into_iter()
                .map(|h| usize::from(h.join().unwrap()))
                .sum::<usize>(),
            1
        );
        let ledger = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(events(&ledger), 3);
        assert_eq!(ledger.snapshot().unwrap()["bound_launch_count"], 1);
    }

    #[test]
    fn launch_transaction_rollback_and_lost_export_never_create_a_second_balance() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let path = Path::new(&policy.allocator_ledger_path);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        let request = launch_request(&direct);
        let reservation = ledger.reserve_launch(&request).unwrap();
        let binding = launch_binding(&reservation);
        let pending = ledger.snapshot().unwrap();
        {
            let mut conn = ledger.connection.lock().unwrap();
            let tx = conn
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .unwrap();
            let mut state: CampaignState = read_state(&tx, "campaign").unwrap();
            let grant = worker_grant(&policy, &request.with_owner(&binding.owner)).unwrap();
            state.grants.insert(request.launch_id.clone(), grant);
            state.launches.get_mut(&request.launch_id).unwrap().binding = Some(binding.clone());
            write_state(&tx, "campaign", &state).unwrap();
            event(&tx, "launch_bound", json!({"binding":binding})).unwrap();
            // Simulate failure before commit: SQLite drops this complete transition.
        }
        assert_eq!(ledger.snapshot().unwrap(), pending);
        assert_eq!(events(&ledger), 2);
        let grant = ledger.bind_launch(&binding).unwrap();
        drop(ledger); // Commit survived, caller's output may have been lost.
        let ledger = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(ledger.reserve_launch(&request).unwrap(), reservation);
        assert_eq!(ledger.bind_launch(&binding).unwrap(), grant);
        assert_eq!(events(&ledger), 3);
        drop(ledger);
        fs::remove_file(path).unwrap();
        assert!(CampaignLedger::create(path, &policy).is_err());
        assert!(CampaignLedger::dry_admit_launch(path, &policy, &request).is_err());
    }

    #[test]
    fn launch_integrity_checks_reject_forged_linkage_even_with_valid_journal_hashes() {
        for pending_with_grant in [false, true] {
            let temp = Temp::new();
            let (policy, direct) = test_policy_request(&temp.0);
            let path = Path::new(&policy.allocator_ledger_path);
            let ledger = CampaignLedger::create(path, &policy).unwrap();
            let request = launch_request(&direct);
            let reservation = ledger.reserve_launch(&request).unwrap();
            let binding = launch_binding(&reservation);
            {
                let mut conn = ledger.connection.lock().unwrap();
                let tx = conn
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .unwrap();
                let mut state: CampaignState = read_state(&tx, "campaign").unwrap();
                if pending_with_grant {
                    state.grants.insert(
                        request.launch_id.clone(),
                        worker_grant(&policy, &request.with_owner(&binding.owner)).unwrap(),
                    );
                } else {
                    state.launches.get_mut(&request.launch_id).unwrap().binding = Some(binding);
                }
                write_state(&tx, "campaign", &state).unwrap();
                event(&tx, "test_corrupt_linkage", json!({})).unwrap();
                tx.commit().unwrap();
                verify_events(&conn).unwrap();
            }
            assert!(ledger.snapshot().is_err());
            assert!(CampaignLedger::open(path, &policy).is_err());
            assert!(CampaignLedger::dry_admit_launch(path, &policy, &request).is_err());
        }
    }

    #[test]
    fn launch_admission_checks_every_resource_coordinate_and_integer_boundary() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let request = launch_request(&direct);
        let path = Path::new(&policy.allocator_ledger_path);
        for coordinate in [
            "money_micro_usd",
            "output_bytes",
            "put_attempts",
            "read_attempts",
            "download_bytes",
            "local_bytes",
        ] {
            let mut changed = request.clone();
            let mut resources = serde_json::to_value(&changed.resources).unwrap();
            resources[coordinate] = json!(
                serde_json::to_value(&policy.caps).unwrap()[coordinate]
                    .as_u64()
                    .unwrap()
                    + 1
            );
            changed.resources = serde_json::from_value(resources).unwrap();
            let report = CampaignLedger::dry_admit_launch(path, &policy, &changed).unwrap();
            assert!(!report.admitted, "unbounded coordinate {coordinate}");
            assert!(report.remaining.is_none());
            let mut single = serde_json::to_value(Resources::default()).unwrap();
            single[coordinate] = json!(1);
            let single: Resources = serde_json::from_value(single).unwrap();
            let mut maximum = serde_json::to_value(Resources::default()).unwrap();
            maximum[coordinate] = json!(u64::MAX);
            let maximum: Resources = serde_json::from_value(maximum).unwrap();
            assert!(maximum.checked_add(&single).is_err());
            assert!(Resources::default().checked_sub(&single).is_err());
        }
        assert_eq!(fs::read_dir(&temp.0).unwrap().count(), 0);
    }

    #[test]
    fn launch_slot_limit_counts_bound_transfers_once_and_refuses_oversized_metadata() {
        let temp = Temp::new();
        let (mut policy, direct) = test_policy_request(&temp.0);
        policy.caps.output_bytes = OUTPUT_CEILING;
        policy.caps.download_bytes = OUTPUT_CEILING;
        policy.caps.put_attempts = 100_000;
        policy.caps.read_attempts = 200_000;
        policy.validate().unwrap();
        let mut request = launch_request(&direct);
        request.resources = Resources {
            money_micro_usd: 1,
            output_bytes: 4096,
            put_attempts: 1,
            read_attempts: 1,
            download_bytes: 4096,
            local_bytes: 0,
        };
        request.finalization = Resources {
            money_micro_usd: 0,
            ..request.resources.clone()
        };
        let mut state = CampaignState {
            policy: policy.clone(),
            grants: BTreeMap::new(),
            launches: BTreeMap::new(),
        };
        for i in 0..MAX_GRANTS {
            let mut request = request.clone();
            request.launch_id = format!("launch-{i}");
            request.target.job_name = format!("job-{i}");
            let reservation = LaunchReservation {
                schema: LAUNCH_RESERVATION_SCHEMA.into(),
                campaign_id: policy.campaign_id.clone(),
                policy_sha256: policy.sha256().unwrap(),
                request,
            };
            state.launches.insert(
                reservation.request.launch_id.clone(),
                LaunchRecord {
                    reservation,
                    binding: None,
                },
            );
        }
        let total = allocated(&state).unwrap();
        assert_eq!(allocation_slots(&state).unwrap(), MAX_GRANTS);
        assert!(launch_admission(&state, &request, true)
            .unwrap()
            .reasons
            .iter()
            .any(|r| r.contains("maximum campaign")));
        assert!(admission(&state, &direct, true)
            .unwrap()
            .reasons
            .iter()
            .any(|r| r.contains("maximum campaign")));
        let record = state.launches.get_mut("launch-0").unwrap();
        let binding = launch_binding(&record.reservation);
        let grant = worker_grant(
            &policy,
            &record.reservation.request.with_owner(&binding.owner),
        )
        .unwrap();
        record.binding = Some(binding);
        state.grants.insert("launch-0".into(), grant);
        assert_eq!(allocation_slots(&state).unwrap(), MAX_GRANTS);
        assert_eq!(allocated(&state).unwrap(), total);
        let mut extra = state.launches["launch-1"].clone();
        extra.reservation.request.launch_id = "one-too-many".into();
        extra.reservation.request.target.job_name = "one-too-many".into();
        state.launches.insert("one-too-many".into(), extra);
        assert!(allocated(&state).is_err());
        drop(state);
        let path = Path::new(&policy.allocator_ledger_path);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        request.worker_journal_path = format!("/{}", "x".repeat(STATE_MAX_BYTES));
        let before = ledger.snapshot().unwrap();
        assert!(ledger
            .reserve_launch(&request)
            .unwrap_err()
            .contains("metadata limit"));
        assert_eq!(ledger.snapshot().unwrap(), before);
        assert_eq!(events(&ledger), 1);
    }

    #[test]
    fn concurrent_direct_and_pending_requests_cannot_share_the_same_id() {
        let temp = Temp::new();
        let (policy, direct) = test_policy_request(&temp.0);
        let request = launch_request(&direct);
        let path = Path::new(&policy.allocator_ledger_path);
        let first = CampaignLedger::create(path, &policy).unwrap();
        let second = CampaignLedger::open(path, &policy).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let ready = barrier.clone();
        let a = std::thread::spawn(move || {
            ready.wait();
            first.reserve_launch(&request).is_ok()
        });
        let b = std::thread::spawn(move || {
            barrier.wait();
            second.reserve(&direct).is_ok()
        });
        assert_eq!(
            usize::from(a.join().unwrap()) + usize::from(b.join().unwrap()),
            1
        );
        let ledger = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(events(&ledger), 2);
    }

    fn session(root: &Path) -> (CampaignPolicy, WorkerGrant, BudgetSession) {
        let (policy, grant) = grant(root);
        let session = BudgetSession::create(
            &root.join("worker.sqlite"),
            &policy,
            &grant,
            &grant.request.owner,
        )
        .unwrap();
        (policy, grant, session)
    }

    fn activate_mock(session: &BudgetSession) {
        let bytes = session.activation_payload().unwrap();
        let sha = hex::encode(crate::hash::sha256::sha256(&bytes));
        let (bucket, key) = session.activation_location();
        let object = session
            .reserve_object(
                bucket,
                key,
                &sha,
                bytes.len() as u64,
                &BudgetScope::Finalization,
            )
            .unwrap();
        let mut operation = HttpOperation {
            operation_id: "activate".into(),
            method: HttpMethod::Put,
            bucket: bucket.into(),
            key: key.into(),
            expected_sha256: Some(sha.clone()),
            expected_bytes: bytes.len() as u64,
            scope: BudgetScope::Finalization,
        };
        let put = session.before_http_attempt(&operation).unwrap();
        session
            .finish_attempt(
                &put,
                &AttemptOutcome {
                    status: Some(200),
                    request_id: None,
                    observed_bytes: 0,
                    error: None,
                },
            )
            .unwrap();
        operation.method = HttpMethod::Get;
        let get = session.before_http_attempt(&operation).unwrap();
        session
            .finish_attempt(
                &get,
                &AttemptOutcome {
                    status: Some(200),
                    request_id: None,
                    observed_bytes: bytes.len() as u64,
                    error: None,
                },
            )
            .unwrap();
        session
            .verify_object(&object, &get, &sha, bytes.len() as u64)
            .unwrap();
        session
            .confirm_activation(&get, &sha, bytes.len() as u64)
            .unwrap();
    }

    #[test]
    fn startup_activation_phase_recovery_is_live_and_does_not_spend_again() {
        let temp = Temp::new();
        let (policy, grant, session) = session(&temp.0);
        assert_eq!(
            session.activation_phase().unwrap(),
            ActivationPhase::Pending
        );
        activate_mock(&session);
        let before = {
            let conn = session.connection.lock().unwrap();
            encode(&read_state::<WorkerState>(&conn, "worker").unwrap()).unwrap()
        };
        drop(session);
        let session = BudgetSession::open(
            &temp.0.join("worker.sqlite"),
            &policy,
            &grant,
            &grant.request.owner,
        )
        .unwrap();
        assert_eq!(session.activation_phase().unwrap(), ActivationPhase::Active);
        session.require_active().unwrap();
        {
            let conn = session.connection.lock().unwrap();
            let state: WorkerState = read_state(&conn, "worker").unwrap();
            assert_eq!(encode(&state).unwrap(), before);
        }
        {
            let _clock = ClockGuard::at(grant.request.expires_unix_seconds);
            assert!(session.activation_phase().is_err());
        }
        for phase in ["closed", "unrecognized-state"] {
            let mut conn = session.connection.lock().unwrap();
            let tx = conn.transaction().unwrap();
            let mut state: WorkerState = read_state(&tx, "worker").unwrap();
            state.phase = phase.into();
            write_state(&tx, "worker", &state).unwrap();
            tx.commit().unwrap();
            drop(conn);
            assert!(session.activation_phase().is_err());
        }
    }

    fn head(scope: BudgetScope) -> HttpOperation {
        HttpOperation {
            operation_id: "head".into(),
            method: HttpMethod::Head,
            bucket: "setec-astronomy-artifacts".into(),
            key: "ml-dataset/test/object".into(),
            expected_sha256: None,
            expected_bytes: 0,
            scope,
        }
    }

    #[test]
    fn integer_bounds_and_actual_region_price_are_required() {
        let temp = Temp::new();
        let (mut policy, mut request) = test_policy_request(&temp.0);
        assert!(Resources {
            put_attempts: u64::MAX,
            ..Resources::default()
        }
        .checked_add(&Resources {
            put_attempts: 1,
            ..Resources::default()
        })
        .is_err());
        policy.compute_rate_micro_usd_per_hour = 3601;
        assert_eq!(policy.compute_floor_micro_usd().unwrap(), 2);
        request.resources.money_micro_usd = 1;
        assert!(request.validate(&policy).unwrap_err().contains("price"));
        request.resources.money_micro_usd = 100;
        policy.pricing_region = "us-east-1".into();
        assert!(policy.validate().unwrap_err().contains("region"));
        policy.pricing_region = "us-east-2".into();
        policy.prior_usage_reconciled = false;
        assert!(policy
            .validate()
            .unwrap_err()
            .contains("unknown is not zero"));
        policy.prior_usage_reconciled = true;
        policy.caps.money_micro_usd = 200_000_001;
        assert!(policy.validate().is_err());
        policy.compute_rate_micro_usd_per_hour = u64::MAX;
        policy.billable_resource_seconds = u64::MAX;
        policy.billable_resource_count = u32::MAX;
        assert!(policy.compute_floor_micro_usd().is_err());
    }

    #[test]
    fn dry_admission_never_creates_or_changes_a_ledger() {
        let temp = Temp::new();
        let (policy, request) = test_policy_request(&temp.0);
        let path = Path::new(&policy.allocator_ledger_path);
        let report = CampaignLedger::dry_admit(path, &policy, &request).unwrap();
        assert!(report.admitted && report.dry_run);
        assert!(!path.exists());
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        ledger.reserve(&request).unwrap();
        let before = fs::read(path).unwrap();
        let report = CampaignLedger::dry_admit(path, &policy, &request).unwrap();
        assert!(!report.admitted);
        assert_eq!(before, fs::read(path).unwrap());
        assert_eq!(fs::read_dir(&temp.0).unwrap().count(), 2);
    }

    #[test]
    fn concurrent_allocators_cannot_overbook_or_issue_owner_twice() {
        let temp = Temp::new();
        let (mut policy, mut request) = test_policy_request(&temp.0);
        policy.caps.money_micro_usd = policy.prior.money_micro_usd + 100;
        request.resources.money_micro_usd = 60;
        let path = Path::new(&policy.allocator_ledger_path);
        let first = CampaignLedger::create(path, &policy).unwrap();
        let second = CampaignLedger::open(path, &policy).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let handles: Vec<_> = [first, second]
            .into_iter()
            .enumerate()
            .map(|(i, ledger)| {
                let barrier = barrier.clone();
                let mut request = request.clone();
                request.grant_id = format!("grant-{i}");
                request.owner.pod_uid = format!("pod-{i}");
                std::thread::spawn(move || {
                    barrier.wait();
                    ledger.reserve(&request).is_ok()
                })
            })
            .collect();
        assert_eq!(
            handles
                .into_iter()
                .map(|h| usize::from(h.join().unwrap()))
                .sum::<usize>(),
            1
        );
        let reopened = CampaignLedger::open(path, &policy).unwrap();
        assert_eq!(
            reopened.snapshot().unwrap()["remaining"]["money_micro_usd"],
            40
        );
        assert!(reopened.reserve(&request).is_err());
    }

    #[test]
    fn pending_activation_cannot_spend_work_or_reuse_another_nonce() {
        let temp = Temp::new();
        let (policy, grant, session) = session(&temp.0);
        assert!(session
            .before_http_attempt(&head(BudgetScope::Work))
            .is_err());
        let original = session.activation_payload().unwrap();
        assert!(BudgetSession::create(
            &temp.0.join("lost-state-replacement.sqlite"),
            &policy,
            &grant,
            &grant.request.owner,
        )
        .is_err());
        let operation = HttpOperation {
            operation_id: "copied-activation".into(),
            method: HttpMethod::Get,
            bucket: grant.activation_bucket.clone(),
            key: grant.activation_key.clone(),
            expected_sha256: Some("e".repeat(64)),
            expected_bytes: original.len() as u64,
            scope: BudgetScope::Finalization,
        };
        assert!(session
            .before_http_attempt(&operation)
            .unwrap_err()
            .contains("exact charged activation"));
        activate_mock(&session);
        assert!(session
            .before_http_attempt(&head(BudgetScope::Work))
            .is_ok());
    }

    #[test]
    fn crash_between_charge_and_response_survives_resume_without_refund() {
        let temp = Temp::new();
        let (policy, grant, session) = session(&temp.0);
        activate_mock(&session);
        let ticket = session
            .before_http_attempt(&head(BudgetScope::Work))
            .unwrap();
        assert_eq!(session.snapshot().unwrap()["outcome_unknown_attempts"], 1);
        assert!(BudgetSession::open(
            &temp.0.join("worker.sqlite"),
            &policy,
            &grant,
            &grant.request.owner
        )
        .is_err());
        drop(session);
        let resumed = BudgetSession::open(
            &temp.0.join("worker.sqlite"),
            &policy,
            &grant,
            &grant.request.owner,
        )
        .unwrap();
        assert_eq!(
            resumed.snapshot().unwrap()["work_reserved_or_charged"]["read_attempts"],
            1
        );
        resumed
            .finish_attempt(
                &ticket,
                &AttemptOutcome {
                    status: None,
                    request_id: None,
                    observed_bytes: 0,
                    error: Some("connection lost; remote outcome unknown".into()),
                },
            )
            .unwrap();
        assert_eq!(resumed.snapshot().unwrap()["outcome_unknown_attempts"], 1);
        let retry = resumed
            .before_http_attempt(&head(BudgetScope::Work))
            .unwrap();
        assert_ne!(ticket.attempt_id, retry.attempt_id);
        assert_eq!(
            resumed.snapshot().unwrap()["work_reserved_or_charged"]["read_attempts"],
            2
        );
        assert!(resumed
            .finish_attempt(
                &ticket,
                &AttemptOutcome {
                    status: Some(200),
                    request_id: None,
                    observed_bytes: 0,
                    error: None
                }
            )
            .is_err());
    }

    #[test]
    fn cell_envelope_is_held_once_and_finalization_cannot_be_consumed_by_work() {
        let temp = Temp::new();
        let (_, grant, session) = session(&temp.0);
        activate_mock(&session);
        let envelope = grant
            .request
            .resources
            .checked_sub(&grant.request.finalization)
            .unwrap();
        session.reserve_cell("cell-1", &envelope).unwrap();
        session.reserve_cell("cell-1", &envelope).unwrap();
        assert!(session
            .before_http_attempt(&head(BudgetScope::Work))
            .is_err());
        for _ in 0..envelope.read_attempts {
            session
                .before_http_attempt(&head(BudgetScope::Cell("cell-1".into())))
                .unwrap();
        }
        assert!(session
            .before_http_attempt(&head(BudgetScope::Cell("cell-1".into())))
            .is_err());
        assert_eq!(
            session.snapshot().unwrap()["work_reserved_or_charged"],
            serde_json::to_value(&envelope).unwrap()
        );
        session
            .before_http_attempt(&head(BudgetScope::Finalization))
            .unwrap();
        assert_eq!(
            session.finalization_remaining().unwrap().read_attempts,
            grant.request.finalization.read_attempts - 2
        );
    }

    #[test]
    fn immutable_object_reservation_requires_independent_successful_get() {
        let temp = Temp::new();
        let (_, _, session) = session(&temp.0);
        activate_mock(&session);
        let op = head(BudgetScope::Work);
        let digest = "f".repeat(64);
        let before = session.snapshot().unwrap()["reserved_unique_output_bytes"]
            .as_u64()
            .unwrap();
        let reservation = session
            .reserve_object(&op.bucket, &op.key, &digest, 10, &op.scope)
            .unwrap();
        assert_eq!(
            reservation,
            session
                .reserve_object(&op.bucket, &op.key, &digest, 10, &op.scope)
                .unwrap()
        );
        assert_eq!(
            session.snapshot().unwrap()["reserved_unique_output_bytes"],
            before + 10
        );
        assert!(session
            .reserve_object(&op.bucket, &op.key, &"e".repeat(64), 10, &op.scope)
            .is_err());
        let get = HttpOperation {
            method: HttpMethod::Get,
            expected_sha256: Some(digest.clone()),
            expected_bytes: 10,
            ..op
        };
        let ticket = session.before_http_attempt(&get).unwrap();
        assert!(session
            .verify_object(&reservation, &ticket, &digest, 10)
            .is_err());
        session
            .finish_attempt(
                &ticket,
                &AttemptOutcome {
                    status: Some(200),
                    request_id: None,
                    observed_bytes: 10,
                    error: None,
                },
            )
            .unwrap();
        assert!(session
            .verify_object(&reservation, &ticket, &"e".repeat(64), 10)
            .is_err());
        session
            .verify_object(&reservation, &ticket, &digest, 10)
            .unwrap();
        let verified = session.snapshot().unwrap()["verified_unique_output_bytes"]
            .as_u64()
            .unwrap();
        assert_eq!(verified, before + 10);
        session
            .verify_object(&reservation, &ticket, &digest, 10)
            .unwrap();
        assert_eq!(
            session.snapshot().unwrap()["verified_unique_output_bytes"],
            verified
        );
    }

    #[test]
    fn concurrent_requests_stop_at_the_last_unit_before_returning_a_ticket() {
        let temp = Temp::new();
        let (policy, mut request) = test_policy_request(&temp.0);
        request.resources.read_attempts = request.finalization.read_attempts + 5;
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let grant = ledger.reserve(&request).unwrap();
        let session = Arc::new(
            BudgetSession::create(
                &temp.0.join("worker.sqlite"),
                &policy,
                &grant,
                &request.owner,
            )
            .unwrap(),
        );
        activate_mock(&session);
        let handles: Vec<_> = (0..12)
            .map(|_| {
                let session = session.clone();
                std::thread::spawn(move || {
                    session
                        .before_http_attempt(&head(BudgetScope::Work))
                        .is_ok()
                })
            })
            .collect();
        assert_eq!(
            handles
                .into_iter()
                .map(|h| usize::from(h.join().unwrap()))
                .sum::<usize>(),
            5
        );
        assert_eq!(
            session.snapshot().unwrap()["work_reserved_or_charged"]["read_attempts"],
            5
        );
    }

    #[test]
    fn copies_missing_state_and_tampered_chain_fail_closed() {
        let temp = Temp::new();
        let (policy, grant, session) = session(&temp.0);
        activate_mock(&session);
        let checkpoint = temp.0.join("checkpoint.sqlite");
        let receipt = session.checkpoint(&checkpoint).unwrap();
        assert_eq!(receipt["audit_only"], true);
        assert!(session.checkpoint(&checkpoint).is_err());
        assert!(BudgetSession::open(&checkpoint, &policy, &grant, &grant.request.owner).is_err());
        assert!(BudgetSession::open(
            &temp.0.join("missing.sqlite"),
            &policy,
            &grant,
            &grant.request.owner
        )
        .is_err());
        assert!(CampaignLedger::open(&checkpoint, &policy).is_err());
        drop(session);
        let path = temp.0.join("worker.sqlite");
        let conn = Connection::open(&path).unwrap();
        conn.execute("UPDATE events SET body='{}' WHERE seq=1", [])
            .unwrap();
        drop(conn);
        assert!(
            BudgetSession::open(&path, &policy, &grant, &grant.request.owner)
                .err()
                .unwrap()
                .contains("journal")
        );
    }

    #[test]
    fn sts_bootstrap_is_bound_and_metered_before_activation() {
        let temp = Temp::new();
        let (policy, mut request) = test_policy_request(&temp.0);
        let role = "arn:aws:iam::123456789012:role/dataset-worker";
        request.credential_role_arn = Some(role.into());
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let grant = ledger.reserve(&request).unwrap();
        let session = BudgetSession::create(
            &temp.0.join("worker.sqlite"),
            &policy,
            &grant,
            &request.owner,
        )
        .unwrap();
        let operation = HttpOperation {
            operation_id: "irsa-bootstrap".into(),
            method: HttpMethod::StsWebIdentity,
            bucket: "sts.us-east-2.amazonaws.com".into(),
            key: hex::encode(crate::hash::sha256::sha256(role.as_bytes())),
            expected_sha256: None,
            expected_bytes: 65536,
            scope: BudgetScope::Finalization,
        };
        session.before_http_attempt(&operation).unwrap();
        assert_eq!(
            session.finalization_remaining().unwrap().download_bytes,
            request.finalization.download_bytes - 65536
        );
        let mut wrong = operation;
        wrong.key = "a".repeat(64);
        assert!(session.before_http_attempt(&wrong).is_err());
        assert_eq!(session.snapshot().unwrap()["sts_attempts"], 1);
    }

    #[test]
    fn close_keeps_allocations_and_allows_recording_pending_outcome() {
        let temp = Temp::new();
        let (_, grant, session) = session(&temp.0);
        activate_mock(&session);
        let ticket = session
            .before_http_attempt(&head(BudgetScope::Work))
            .unwrap();
        session
            .close("cancelled; do not release uncertain obligations")
            .unwrap();
        assert!(session
            .before_http_attempt(&head(BudgetScope::Work))
            .is_err());
        session
            .finish_attempt(
                &ticket,
                &AttemptOutcome {
                    status: None,
                    request_id: None,
                    observed_bytes: 0,
                    error: Some("cancelled".into()),
                },
            )
            .unwrap();
        assert_eq!(
            session.snapshot().unwrap()["allocation_retained"],
            serde_json::to_value(grant.request.resources).unwrap()
        );
    }

    #[test]
    fn lost_allocator_database_cannot_reissue_balances() {
        let temp = Temp::new();
        let (policy, request) = test_policy_request(&temp.0);
        let path = Path::new(&policy.allocator_ledger_path);
        let ledger = CampaignLedger::create(path, &policy).unwrap();
        ledger.reserve(&request).unwrap();
        drop(ledger);
        fs::remove_file(path).unwrap();
        assert!(CampaignLedger::create(path, &policy)
            .err()
            .unwrap()
            .contains("never reset"));
        assert!(CampaignLedger::dry_admit(path, &policy, &request)
            .err()
            .unwrap()
            .contains("never reset"));
        assert!(!path.exists());
    }

    #[test]
    fn lost_worker_database_cannot_refresh_bootstrap_allowance() {
        let temp = Temp::new();
        let (policy, grant, session) = session(&temp.0);
        assert_eq!(session.snapshot().unwrap()["charged_attempts"], 0);
        drop(session);
        let path = temp.0.join("worker.sqlite");
        fs::remove_file(&path).unwrap();
        assert!(
            BudgetSession::create(&path, &policy, &grant, &grant.request.owner)
                .err()
                .unwrap()
                .contains("never reset bootstrap")
        );
        let replacement = temp.0.join("replacement.sqlite");
        assert!(
            BudgetSession::create(&replacement, &policy, &grant, &grant.request.owner)
                .err()
                .unwrap()
                .contains("path differs")
        );
        assert!(!path.exists() && !replacement.exists());
    }

    #[test]
    fn prior_noncompute_component_covers_campaign_caps_without_double_count() {
        let temp = Temp::new();
        let (mut policy, request) = test_policy_request(&temp.0);
        policy.caps.put_attempts = 1001;
        policy.caps.read_attempts = 1001;
        policy.caps.output_bytes = 1024 * 1024 * 1024;
        policy.caps.download_bytes = 2 * 1024 * 1024 * 1024;
        policy.storage_horizon_days = 31;
        policy.noncompute_prices = NoncomputePrices {
            put_micro_usd_per_1000: 5000,
            read_micro_usd_per_1000: 400,
            storage_micro_usd_per_gib_day: 1000,
            download_micro_usd_per_gib: 90000,
            fixed_micro_usd: 10000,
        };
        assert_eq!(policy.noncompute_floor_micro_usd().unwrap(), 226406);
        policy.prior_noncompute_reserve_micro_usd = 226405;
        assert!(policy
            .validate()
            .unwrap_err()
            .contains("noncompute reserve"));
        policy.prior_noncompute_reserve_micro_usd = 226406;
        policy.validate().unwrap();
        let report =
            CampaignLedger::dry_admit(Path::new(&policy.allocator_ledger_path), &policy, &request)
                .unwrap();
        assert_eq!(
            report.prior_and_allocated.money_micro_usd,
            policy.prior.money_micro_usd
        );
        policy.noncompute_prices.fixed_micro_usd = u64::MAX;
        assert!(policy.noncompute_floor_micro_usd().is_err());
    }
}
