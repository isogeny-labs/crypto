//! Versioned synthetic research datasets with explicit capabilities and durable receipts.
pub mod budget;
pub mod curves;
pub mod export;
pub mod relations;
pub mod report;
pub mod restore;
pub mod s3_budget;
pub mod solvers;
pub mod storage;
pub mod telemetry;
pub mod types;

use budget::{
    BudgetScope, BudgetSession, CampaignLedger, CampaignPolicy, GrantRequest, LaunchBinding,
    LaunchRequest, OwnerIdentity, Resources, RunBinding, WorkerGrant,
};
use clap::{Args, Subcommand};
use s3_budget::S3BudgetContext;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use types::{Artifact, CaseConfig};

#[derive(Args)]
pub struct DatasetArgs {
    #[command(subcommand)]
    pub command: DatasetCommand,
}

#[derive(Subcommand)]
pub enum DatasetCommand {
    /// Inspect or reserve cumulative campaign allocations; admission is not launch authorization.
    Budget {
        #[command(subcommand)]
        command: BudgetCommand,
    },
    /// Freeze a deterministic cross-product; planning performs no cryptography.
    Plan(PlanArgs),
    /// Execute a bounded number of cells, preserving failures and resuming verified cells.
    Generate(GenerateArgs),
    /// Check readable frozen inputs, clean build identity and receipt routing without running cryptography.
    Preflight {
        #[arg(long)]
        plan: PathBuf,
        #[arg(long)]
        protocol: PathBuf,
    },
    /// Hash-check every artifact and independently replay known-answer identities.
    Verify {
        #[arg(long)]
        out: PathBuf,
    },
    /// Count recorded outcomes without rerunning any solver.
    Status {
        #[arg(long)]
        out: PathBuf,
    },
    /// Write an immutable partial coverage and measurement report from retained artifacts.
    Report {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        destination: PathBuf,
    },
    /// Publish all completed cell manifests and round-trip their bytes through the store.
    Publish {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        remote: Option<String>,
        #[command(flatten)]
        budget: BudgetArgs,
    },
    /// Show actual method support and instrumentation limits.
    Capabilities,
    /// Produce strict public features and separate restricted labels from replayed cells.
    Export {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        destination: PathBuf,
    },
    /// Download and independently replay a completed cell from its portable receipt.
    Restore {
        #[arg(long)]
        receipt: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        remote: Option<String>,
        #[command(flatten)]
        budget: BudgetArgs,
    },
    /// Preserve an interrupted attempt after proving no matching worker remains.
    Reconcile {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        cell: u64,
        #[arg(long)]
        reason: String,
    },
    #[command(hide = true)]
    Worker {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(Clone, Default, Args)]
pub struct BudgetArgs {
    #[arg(long)]
    pub budget_policy: Option<PathBuf>,
    #[arg(long)]
    pub budget_grant: Option<PathBuf>,
    /// Existing durable journal; normal commands never create a fresh balance.
    #[arg(long)]
    pub budget_state: Option<PathBuf>,
    /// Exact Job/Pod UID identity document bound by the grant.
    #[arg(long)]
    pub budget_owner: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum BudgetCommand {
    /// Read-only hypothetical admission. No allocation, local directory or cloud request is made.
    Admit {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// Durably allocate one disjoint worker grant, without launching or contacting S3.
    Reserve {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        out: PathBuf,
        /// Create the single authoritative campaign ledger; never use for lost-state recovery.
        #[arg(long)]
        create_ledger: bool,
    },
    /// Reserve an ownerless launch envelope before submission; dry-run is strictly read-only.
    ReserveLaunch {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long)]
        request: PathBuf,
        #[arg(long, conflicts_with_all = ["out", "create_ledger"])]
        dry_run: bool,
        #[arg(long, required_unless_present = "dry_run")]
        out: Option<PathBuf>,
        /// First initialization only; never recreate a lost allocator.
        #[arg(long)]
        create_ledger: bool,
    },
    /// Bind a retained launch reservation to reconciled owner evidence; no cloud submission.
    BindLaunch {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
        #[arg(long)]
        binding: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Activate a grant once with a charged conditional S3 PUT and verified GET.
    Activate {
        #[command(flatten)]
        budget: BudgetArgs,
        /// Retry an existing pending activation using its original journal nonce.
        #[arg(long)]
        resume: bool,
    },
    /// Fetch one known object with exact digest/length and charged S3 requests; no LIST.
    Fetch {
        #[command(flatten)]
        budget: BudgetArgs,
        #[arg(long)]
        remote: String,
        #[arg(long)]
        key: String,
        #[arg(long)]
        sha256: String,
        #[arg(long)]
        bytes: u64,
        #[arg(long)]
        out: PathBuf,
    },
    /// Inspect the authoritative local campaign ledger without contacting S3.
    Status {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        ledger: PathBuf,
    },
}

#[derive(Args)]
pub struct PlanArgs {
    #[arg(long)]
    pub out: PathBuf,
    #[arg(long, default_value = "23-31,47-61")]
    pub n: String,
    #[arg(long, default_value = "2-15")]
    pub m: String,
    #[arg(long, default_value = "koblitz,binary,prime")]
    pub families: String,
    #[arg(
        long,
        default_value = "enumerate,bsgs,sat,weil,f4,f5,f6,root,pkm,double-large-prime"
    )]
    pub solvers: String,
    #[arg(long, default_value_t = 16)]
    pub curve_seeds: u64,
    #[arg(long, default_value_t = 8)]
    pub generators: u64,
    /// Number of target-key draws per generator.
    #[arg(long, default_value_t = 64)]
    pub keys: u64,
    #[arg(long, default_value_t = 20261027)]
    pub seed: u64,
    #[arg(long, default_value_t = 8)]
    pub factor_base_size: usize,
    #[arg(long, default_value_t = 3)]
    pub degree_bound: u32,
    #[arg(long, default_value_t = 100000)]
    pub work_limit: u64,
    /// SHA-256 of the committed experiment.protocol/v1 file. Required to generate.
    #[arg(long)]
    pub protocol_sha256: Option<String>,
}

#[derive(Args)]
pub struct GenerateArgs {
    #[arg(long)]
    pub plan: PathBuf,
    /// Exact protocol bytes whose SHA-256 was frozen in the plan.
    #[arg(long)]
    pub protocol: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
    /// Maximum new cells in this invocation; never an unbounded default.
    #[arg(long)]
    pub max_cases: u64,
    #[arg(long, default_value_t = 0)]
    pub shard_index: u64,
    #[arg(long, default_value_t = 1)]
    pub shard_count: u64,
    #[arg(long, default_value_t = 60)]
    pub seconds_per_case: u64,
    #[arg(long, default_value_t = 2048)]
    pub memory_mib: u64,
    /// Cap worker output before sealing; manifest/publication metadata are counted separately.
    #[arg(long, default_value_t = 64)]
    pub trace_mib: u64,
    /// Publish each completed cell before moving on; upload failures stop the wave.
    #[arg(long)]
    pub publish: bool,
    #[arg(long)]
    pub remote: Option<String>,
    #[command(flatten)]
    pub budget: BudgetArgs,
    /// Finite Resources reservation charged once per cell, with requests spent inside it.
    #[arg(long)]
    pub budget_cell_envelope: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetPlan {
    pub schema: String,
    pub field_sizes: Vec<u32>,
    pub arities: Vec<usize>,
    pub families: Vec<String>,
    pub solvers: Vec<String>,
    pub curve_seeds: u64,
    pub generators: u64,
    pub keys: u64,
    pub seed: u64,
    pub factor_base_size: usize,
    pub degree_bound: u32,
    pub work_limit: u64,
    pub twists: bool,
    pub protocol_sha256: Option<String>,
    pub split_rule: String,
}

impl DatasetPlan {
    /// Existing run/grant identity: compact serialization of this typed plan
    /// in struct field order, not generic canonical JSON or the saved file bytes.
    fn binding_sha256(&self) -> Result<String, String> {
        Ok(digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "crypto.ml-dataset.plan/v1" || !self.twists {
            return Err("expected plan/v1 with paired twists".into());
        }
        if self.field_sizes.is_empty()
            || self.field_sizes.iter().any(|n| !(3..=61).contains(n))
            || self.arities.is_empty()
            || self.arities.iter().any(|m| !(2..=15).contains(m))
        {
            return Err("field sizes must be 3..61, arities 2..15, with nonempty axes".into());
        }
        for (axis, allowed) in [
            (&self.families, &["koblitz", "binary", "prime"][..]),
            (
                &self.solvers,
                &[
                    "enumerate",
                    "pdp",
                    "bsgs",
                    "sat",
                    "weil",
                    "weil-descent",
                    "f4",
                    "f5",
                    "f6",
                    "root",
                    "pkm",
                    "double-large-prime",
                ][..],
            ),
        ] {
            if axis.is_empty()
                || axis.iter().any(|s| !allowed.contains(&s.as_str()))
                || axis.iter().collect::<BTreeSet<_>>().len() != axis.len()
            {
                return Err("empty, duplicate or unknown family/solver axis".into());
            }
        }
        if self.field_sizes.iter().collect::<BTreeSet<_>>().len() != self.field_sizes.len()
            || self.arities.iter().collect::<BTreeSet<_>>().len() != self.arities.len()
            || self.curve_seeds == 0
            || self.generators == 0
            || self.keys == 0
            || !(2..=4096).contains(&self.factor_base_size)
            || self.work_limit == 0
            || !(1..=32).contains(&self.degree_bound)
        {
            return Err(
                "invalid or duplicate axes, factor-base size, degree or work budget".into(),
            );
        }
        if self.split_rule != "curve-twist-component-sha256-v1" {
            return Err("unknown split rule".into());
        }
        if let Some(hash) = &self.protocol_sha256 {
            crate::store::index::validate_sha256(hash)?;
        }
        self.cell_count()?;
        Ok(())
    }

    pub fn cell_count(&self) -> Result<u64, String> {
        [
            self.field_sizes.len() as u64,
            self.arities.len() as u64,
            self.families.len() as u64,
            self.solvers.len() as u64,
            self.curve_seeds,
            self.generators,
            self.keys,
            2,
        ]
        .into_iter()
        .try_fold(1u64, |a, b| {
            a.checked_mul(b)
                .ok_or_else(|| "plan cell count overflow".into())
        })
    }

    pub fn cell(&self, mut index: u64) -> Result<CaseConfig, String> {
        if index >= self.cell_count()? {
            return Err("cell index outside plan".into());
        }
        let mut digit = |radix: u64| {
            let d = index % radix;
            index /= radix;
            d
        };
        let solver = self.solvers[digit(self.solvers.len() as u64) as usize].clone();
        let twist = digit(2) == 1;
        let m = self.arities[digit(self.arities.len() as u64) as usize];
        let key_index = digit(self.keys);
        let generator_index = digit(self.generators);
        let curve_seed = digit(self.curve_seeds);
        let n = self.field_sizes[digit(self.field_sizes.len() as u64) as usize];
        let family = self.families[digit(self.families.len() as u64) as usize].clone();
        Ok(CaseConfig {
            family,
            n,
            m,
            twist,
            curve_seed,
            generator_index,
            key_index,
            seed: self.seed,
            factor_base_size: self.factor_base_size,
            degree_bound: self.degree_bound,
            work_limit: self.work_limit,
            solver,
        })
    }
}

fn numbers(s: &str) -> Result<Vec<u32>, String> {
    let mut result = Vec::new();
    for part in s.split(',') {
        if let Some((a, b)) = part.split_once('-') {
            let a: u32 = a.parse().map_err(|_| "invalid range start")?;
            let b: u32 = b.parse().map_err(|_| "invalid range end")?;
            if a > b || b > 61 {
                return Err("descending or oversized range".into());
            }
            result.extend(a..=b);
        } else {
            result.push(part.parse().map_err(|_| "invalid numeric axis")?);
        }
    }
    Ok(result)
}

pub(super) const JSON_INPUT_MAX_BYTES: u64 = 32 * 1024 * 1024;

fn read_json_bytes(path: &Path) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "{}: JSON input must be a regular file",
            path.display()
        ));
    }
    if metadata.len() > JSON_INPUT_MAX_BYTES {
        return Err(format!("{}: JSON input exceeds32MiB limit", path.display()));
    }
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(JSON_INPUT_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if bytes.len() as u64 > JSON_INPUT_MAX_BYTES {
        return Err(format!(
            "{}: JSON input grew beyond32MiB limit",
            path.display()
        ));
    }
    Ok(bytes)
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    serde_json::from_slice(&read_json_bytes(path)?).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn digest(bytes: &[u8]) -> String {
    crate::utils::encoding::to_hex(&crate::hash::sha256(bytes))
}

fn budget_inputs(
    args: &BudgetArgs,
) -> Result<Option<(CampaignPolicy, WorkerGrant, OwnerIdentity, &Path)>, String> {
    match (
        &args.budget_policy,
        &args.budget_grant,
        &args.budget_owner,
        &args.budget_state,
    ) {
        (None, None, None, None) => Ok(None),
        (Some(policy), Some(grant), Some(owner), Some(state)) => Ok(Some((
            read_json(policy)?,
            read_json(grant)?,
            read_json(owner)?,
            state,
        ))),
        _ => Err(
            "budget-policy, budget-grant, budget-owner and budget-state must be supplied together"
                .into(),
        ),
    }
}

fn validate_budget_actor(session: &BudgetSession) -> Result<(), String> {
    let source = generation_source()?;
    let binary = crate::store::hash_file(&std::env::current_exe().map_err(|e| e.to_string())?)?.0;
    let binding = &session.grant().request.binding;
    if binding.source_commit != source || binding.binary_sha256 != binary {
        return Err("current executor source/binary differs from budget grant".into());
    }
    Ok(())
}

fn open_budget(args: &BudgetArgs) -> Result<Option<Arc<BudgetSession>>, String> {
    let Some((policy, grant, owner, state)) = budget_inputs(args)? else {
        return Ok(None);
    };
    let session = Arc::new(BudgetSession::open(state, &policy, &grant, &owner)?);
    session.require_active()?;
    validate_budget_actor(&session)?;
    Ok(Some(session))
}

/// Actor identity is checked before creating the transport. Imported data keep
/// their original producer identity; the grant binds this reader/publisher.
pub(super) fn validate_budget_run(session: &BudgetSession, root: &Path) -> Result<(), String> {
    let (plan, producer, plan_sha) = bound_run(root)?;
    let protocol_sha = plan
        .protocol_sha256
        .ok_or("budgeted run lacks frozen protocol")?;
    let actor = &session.grant().request.binding;
    session.validate_binding(&RunBinding {
        source_commit: actor.source_commit.clone(),
        binary_sha256: actor.binary_sha256.clone(),
        plan_sha256: plan_sha,
        protocol_sha256: protocol_sha,
        shard_index: producer["shard_index"]
            .as_u64()
            .ok_or("missing run shard")?,
        shard_count: producer["shard_count"]
            .as_u64()
            .ok_or("missing run shard count")?,
    })
}

fn configured_remote(remote: Option<&str>) -> Result<crate::store::backend::RemoteSpec, String> {
    if let Some(remote) = remote {
        return crate::store::backend::RemoteSpec::parse(remote);
    }
    crate::store::Config::discover()?
        .remote
        .ok_or_else(|| "explicit S3 or file remote required".to_string())
}

fn require_remote_budget(
    remote: &crate::store::backend::RemoteSpec,
    session: Option<&Arc<BudgetSession>>,
) -> Result<(), String> {
    if matches!(remote, crate::store::backend::RemoteSpec::S3 { .. }) && session.is_none() {
        return Err("dataset S3 operations require policy/grant/owner and an existing activated budget journal".into());
    }
    Ok(())
}

fn budget_transport(
    session: Option<Arc<BudgetSession>>,
) -> Result<Option<Arc<S3BudgetContext>>, String> {
    session.map(S3BudgetContext::from_env).transpose()
}

fn save_budget_checkpoint(root: &Path, session: &BudgetSession) -> Result<Artifact, String> {
    let directory = checked_directory(&root.join("budget-checkpoints"), true)?;
    let nonce = unix_nanos()?;
    let filename = format!("{}-{nonce}.sqlite", session.grant().request.grant_id);
    let checkpoint = session.checkpoint(&directory.join(&filename))?;
    let snapshot = session.snapshot()?;
    storage::write_json(
        &directory,
        &format!("{filename}.json"),
        "restricted-budget-audit-checkpoint",
        &json!({"checkpoint":checkpoint,"snapshot":snapshot,
            "local_byte_allocation_is_hard_write_limit":false,"account_wide_billing_cutoff":false}),
    )?;
    let (sha256, bytes) = crate::store::hash_file(&directory.join(&filename))?;
    Ok(Artifact {
        path: format!("budget-checkpoints/{filename}"),
        role: "restricted-budget-audit-database".into(),
        encoding: "sqlite".into(),
        sha256,
        bytes,
    })
}

fn finalize_budget(
    root: &Path,
    context: Option<&Arc<S3BudgetContext>>,
    remote: Option<&crate::store::backend::RemoteSpec>,
    publish: bool,
) -> Result<(), String> {
    let Some(context) = context else {
        return Ok(());
    };
    let checkpoint = save_budget_checkpoint(root, context.session())?;
    if publish {
        let remote = remote.ok_or("budget checkpoint requires an explicit remote")?;
        if matches!(remote, crate::store::backend::RemoteSpec::S3 { .. }) {
            let mut cfg = crate::store::Config::discover()?;
            cfg.remote = Some(remote.clone());
            cfg.index_path = root.join("store-index.json");
            let result = storage::publish_budget_checkpoint(root, &checkpoint, cfg, context);
            // Retain even failed/unknown checkpoint publication attempts. This
            // final local copy is not recursively uploaded or called remote.
            let retained = save_budget_checkpoint(root, context.session());
            result?;
            retained?;
        }
    }
    Ok(())
}

fn combine_finalization<T>(
    result: Result<T, String>,
    finalization: Result<(), String>,
) -> Result<T, String> {
    match (result, finalization) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) => Err(format!(
            "budget finalization failed; retained journal remains charged: {error}"
        )),
        (Err(error), Err(final_error)) => Err(format!(
            "{error}; budget finalization also failed: {final_error}"
        )),
    }
}

fn cell_budget_scope(
    context: Option<&Arc<S3BudgetContext>>,
    envelope: Option<&Resources>,
    plan: &str,
    index: u64,
) -> Result<BudgetScope, String> {
    let Some(context) = context else {
        return Ok(BudgetScope::Work);
    };
    // A minimum control allowance is protected before starting more work.
    // Actual metadata/checkpoint sizes are admitted again before transmission;
    // this is not a claim that polling bounds all local writes or artifact counts.
    context
        .session()
        .finalization_remaining()?
        .checked_sub(&Resources {
            output_bytes: 64 * 1024,
            put_attempts: 2,
            read_attempts: 4,
            download_bytes: 256 * 1024,
            ..Resources::default()
        })?;
    let id = format!("cell-{plan}-{index}");
    context.session().reserve_cell(
        &id,
        envelope.ok_or("budgeted generation requires --budget-cell-envelope")?,
    )?;
    Ok(BudgetScope::Cell(id))
}

fn write_budget_receipt<T: Serialize>(
    out: &Path,
    role: &str,
    value: &T,
) -> Result<Artifact, String> {
    let parent = out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = checked_directory(parent, true)?;
    let filename = out
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid budget receipt filename")?;
    storage::write_json(&parent, filename, role, value)
}

fn run_budget_command(command: &BudgetCommand) -> Result<Value, String> {
    match command {
        BudgetCommand::Admit {
            policy,
            ledger,
            request,
            dry_run,
        } => {
            if !dry_run {
                return Err(
                    "budget admit requires --dry-run; use budget reserve for explicit allocation"
                        .into(),
                );
            }
            let report =
                CampaignLedger::dry_admit(ledger, &read_json(policy)?, &read_json(request)?)?;
            serde_json::to_value(report).map_err(|e| e.to_string())
        }
        BudgetCommand::Reserve {
            policy,
            ledger,
            request,
            out,
            create_ledger,
        } => {
            let policy: CampaignPolicy = read_json(policy)?;
            let request: GrantRequest = read_json(request)?;
            let ledger = if *create_ledger {
                CampaignLedger::create(ledger, &policy)?
            } else {
                CampaignLedger::open(ledger, &policy)?
            };
            let snapshot = ledger.snapshot()?;
            if snapshot["launches"].get(&request.grant_id).is_some() {
                return Err("grant ID belongs to a launch reservation; use bind-launch to preserve its binding evidence".into());
            }
            let previous = snapshot["grants"][&request.grant_id].clone();
            let grant = if previous.is_null() {
                ledger.reserve(&request)?
            } else {
                let grant: WorkerGrant =
                    serde_json::from_value(previous).map_err(|e| e.to_string())?;
                if grant.request != request {
                    return Err("issued grant ID cannot be repointed to a different request".into());
                }
                grant
            };
            let parent = out
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let parent = checked_directory(parent, true)?;
            let filename = out
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or("invalid grant filename")?;
            let artifact = storage::write_json(
                &parent,
                filename,
                "restricted-campaign-budget-grant",
                &grant,
            )?;
            Ok(
                json!({"grant":grant,"artifact":artifact,"campaign":ledger.snapshot()?,"cloud_requests":0,"launched":false}),
            )
        }
        BudgetCommand::ReserveLaunch {
            policy,
            ledger,
            request,
            dry_run,
            out,
            create_ledger,
        } => {
            // Enforce the same constraints for direct native callers as clap.
            // Check before reading inputs or opening/creating allocator state.
            if *dry_run && (out.is_some() || *create_ledger) {
                return Err(
                    "reserve-launch --dry-run conflicts with --out and --create-ledger".into(),
                );
            }
            if !*dry_run && out.is_none() {
                return Err("reserve-launch requires --out unless --dry-run is set".into());
            }
            let policy: CampaignPolicy = read_json(policy)?;
            let request: LaunchRequest = read_json(request)?;
            if *dry_run {
                return serde_json::to_value(CampaignLedger::dry_admit_launch(
                    ledger, &policy, &request,
                )?)
                .map_err(|e| e.to_string());
            }
            let out = out.as_ref().ok_or("reserve-launch requires --out")?;
            let ledger = if *create_ledger {
                CampaignLedger::create(ledger, &policy)?
            } else {
                CampaignLedger::open(ledger, &policy)?
            };
            // Commit before exporting. An output failure retains the complete
            // reservation; exact retries recover it without another allocation.
            let reservation = ledger.reserve_launch(&request)?;
            let reservation_sha256 = reservation.sha256()?;
            let artifact =
                write_budget_receipt(out, "restricted-campaign-launch-reservation", &reservation)?;
            Ok(
                json!({"reservation":reservation,"reservation_sha256":reservation_sha256,
                "artifact":artifact,"campaign":ledger.snapshot()?,"cloud_requests":0,
                "launched":false,"worker_grant_issued":false}),
            )
        }
        BudgetCommand::BindLaunch {
            policy,
            ledger,
            binding,
            out,
        } => {
            let policy: CampaignPolicy = read_json(policy)?;
            let binding: LaunchBinding = read_json(binding)?;
            // Binding never initializes an allocator or creates new allowance.
            let ledger = CampaignLedger::open(ledger, &policy)?;
            let grant = ledger.bind_launch(&binding)?;
            let grant_sha256 = grant.sha256()?;
            let artifact = write_budget_receipt(out, "restricted-campaign-budget-grant", &grant)?;
            Ok(
                json!({"grant":grant,"grant_sha256":grant_sha256,"binding":binding,
                "artifact":artifact,"campaign":ledger.snapshot()?,
                "cloud_requests":0,"launched":false}),
            )
        }
        BudgetCommand::Activate { budget, resume } => {
            let (policy, grant, owner, state) =
                budget_inputs(budget)?.ok_or("activation requires all budget inputs")?;
            let session = Arc::new(if *resume {
                BudgetSession::open(state, &policy, &grant, &owner)?
            } else {
                BudgetSession::create(state, &policy, &grant, &owner)?
            });
            validate_budget_actor(&session)?;
            let context = S3BudgetContext::from_env(session.clone())?;
            let activation = context.activate();
            let parent = state
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let checkpoint = save_budget_checkpoint(parent, &session);
            activation?;
            Ok(
                json!({"activated":true,"checkpoint":checkpoint?,"budget":session.snapshot()?,"launched":false}),
            )
        }
        BudgetCommand::Fetch {
            budget,
            remote,
            key,
            sha256,
            bytes,
            out,
        } => {
            crate::store::index::validate_sha256(sha256)?;
            if *bytes > 1024 * 1024 * 1024 {
                return Err("budget fetch exceeds the 1 GiB single-object limit".into());
            }
            let remote = crate::store::backend::RemoteSpec::parse(remote)?;
            if !matches!(remote, crate::store::backend::RemoteSpec::S3 { .. }) {
                return Err("budget fetch requires an explicit S3 remote".into());
            }
            let session =
                open_budget(budget)?.ok_or("budget fetch requires an active bound session")?;
            let context = S3BudgetContext::from_env(session)?;
            fetch_budget_object(&context, &remote, key, sha256, *bytes, out)
        }
        BudgetCommand::Status { policy, ledger } => {
            CampaignLedger::open(ledger, &read_json(policy)?)?.snapshot()
        }
    }
}

fn fetch_budget_object(
    context: &Arc<S3BudgetContext>,
    remote: &crate::store::backend::RemoteSpec,
    key: &str,
    sha256: &str,
    bytes: u64,
    out: &Path,
) -> Result<Value, String> {
    let parent = checked_directory(
        out.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
        true,
    )?;
    let name = out
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or("invalid fetch destination")?;
    let destination = storage::contained_path(&parent, name, false)?;
    if fs::symlink_metadata(&destination).is_ok() {
        return Err("fetch destination exists; use a fresh path for a new readback".into());
    }
    let partial = parent.join(format!(
        ".budget-fetch-{}-{}",
        std::process::id(),
        unix_nanos()?
    ));
    let artifact = Artifact {
        path: name.into(),
        role: "budgeted-exact-object-download".into(),
        encoding: "native".into(),
        sha256: sha256.into(),
        bytes,
    };
    let result = (|| {
        storage::expect_remote_object(Some(context), remote, key, &artifact, BudgetScope::Work)?;
        let mut cfg = crate::store::Config::discover()?;
        cfg.remote = Some(remote.clone());
        cfg.index_path = parent.join("budget-fetch-unused-index.json");
        let store = storage::dataset_store(cfg, Some(context), BudgetScope::Work)?;
        store.remote()?.get(key, &partial)?;
        storage::check_file(&partial, sha256, bytes)?;
        File::open(&partial)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        fs::hard_link(&partial, &destination)
            .map_err(|e| format!("activate verified fetch without overwrite: {e}"))?;
        storage::sync_directory(&parent)?;
        fs::remove_file(&partial).map_err(|e| e.to_string())?;
        Ok(
            json!({"schema":"crypto.ml-dataset.budget-fetch/v1","remote":remote.display(),"object_key":key,
            "destination":destination,"sha256":sha256,"bytes":bytes,"fresh_readback_verified":true,
            "listing_performed":false,"remote_object_written":false}),
        )
    })();
    combine_finalization(result, finalize_budget(&parent, Some(context), None, false))
}

pub fn run(args: &DatasetArgs) -> Result<(), String> {
    match &args.command {
        DatasetCommand::Budget { command } => println!("{}", run_budget_command(command)?),
        DatasetCommand::Plan(a) => {
            let plan = DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: numbers(&a.n)?,
                arities: numbers(&a.m)?.into_iter().map(|m| m as usize).collect(),
                families: a.families.split(',').map(str::to_owned).collect(),
                solvers: a.solvers.split(',').map(str::to_owned).collect(),
                curve_seeds: a.curve_seeds,
                generators: a.generators,
                keys: a.keys,
                seed: a.seed,
                factor_base_size: a.factor_base_size,
                degree_bound: a.degree_bound,
                work_limit: a.work_limit,
                twists: true,
                protocol_sha256: a.protocol_sha256.clone(),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            plan.validate()?;
            let parent = a
                .out
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            let name = a
                .out
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or("invalid plan filename")?;
            let artifact = storage::write_json(parent, name, "frozen-plan", &plan)?;
            println!(
                "{}",
                json!({"plan":a.out,"sha256":artifact.sha256,"plan_sha256":plan.binding_sha256()?,"planned_cells":plan.cell_count()?,
                "generated_cells":0,"twists_included":true})
            );
        }
        DatasetCommand::Generate(a) => generate(a)?,
        DatasetCommand::Preflight { plan, protocol } => {
            let parsed: DatasetPlan = read_json(plan)?;
            parsed.validate()?;
            let protocol_bytes = read_json_bytes(protocol)?;
            verify_protocol(&parsed, &protocol_bytes)?;
            let source = generation_source()?;
            let receipt_uri = std::env::var("ISOLAB_RESULTS_URI")
                .ok()
                .filter(|v| !v.is_empty());
            if let Some(uri) = &receipt_uri {
                lab_receipt_remote(uri)?;
            }
            println!(
                "{}",
                json!({"schema":"crypto.ml-dataset.preflight/v1",
                "source_commit":source,"source_dirty":false,
                "binary_sha256":crate::store::hash_file(&std::env::current_exe().map_err(|e|e.to_string())?)?.0,
                "plan_file_sha256":crate::store::hash_file(plan)?.0,
                "plan_sha256":parsed.binding_sha256()?,
                "protocol_file_sha256":digest(&protocol_bytes),
                "planned_cells":parsed.cell_count()?,"generated_cells":0,
                "lab_receipt_uri":receipt_uri,"cloud_permissions_checked":false,
                "output_permissions_checked":false,"scientific_computation_performed":false})
            );
        }
        DatasetCommand::Worker { config, out } => worker(&read_json(config)?, out)?,
        DatasetCommand::Verify { out } => {
            println!("{}", verify_run(out, true)?);
        }
        DatasetCommand::Status { out } => {
            println!("{}", verify_run(out, false)?);
        }
        DatasetCommand::Report { out, destination } => {
            println!("{}", report::report_run(out, destination)?);
        }
        DatasetCommand::Publish {
            out,
            remote,
            budget,
        } => {
            let session = open_budget(budget)?;
            let remote = configured_remote(remote.as_deref())?;
            require_remote_budget(&remote, session.as_ref())?;
            if let Some(session) = &session {
                validate_budget_run(session, out)?;
            }
            let context = budget_transport(session)?;
            let _lease = Lease::acquire(out)?;
            let result = (|| {
                verify_run(out, true)?;
                for dir in completed_cells(out)? {
                    publish_cell(
                        out,
                        &dir,
                        Some(&remote.display()),
                        context.as_ref(),
                        BudgetScope::Work,
                    )?;
                }
                Ok(())
            })();
            combine_finalization(
                result,
                finalize_budget(out, context.as_ref(), Some(&remote), true),
            )?;
        }
        DatasetCommand::Capabilities => println!("{}", solvers::capabilities()),
        DatasetCommand::Export { out, destination } => {
            println!("{}", export::export(out, destination)?)
        }
        DatasetCommand::Restore {
            receipt,
            out,
            remote,
            budget,
        } => {
            let session = open_budget(budget)?;
            let context = budget_transport(session)?;
            let result =
                restore::restore_budgeted(receipt, out, remote.as_deref(), context.as_ref());
            let finalized = finalize_budget(out, context.as_ref(), None, false);
            println!("{}", combine_finalization(result, finalized)?);
        }
        DatasetCommand::Reconcile { out, cell, reason } => reconcile(out, *cell, reason)?,
    }
    Ok(())
}

struct Lease {
    _file: File,
}
impl Lease {
    fn acquire(root: &Path) -> Result<Self, String> {
        let root = checked_directory(root, true)?;
        let mut options = OpenOptions::new();
        options.create(true).truncate(false).read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let file = options
            .open(root.join(".writer.lock"))
            .map_err(|e| e.to_string())?;
        if !file.metadata().map_err(|e| e.to_string())?.is_file() {
            return Err("writer lock is not a regular file".into());
        }
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            // OS releases this advisory lock when the process exits, including crashes.
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
                return Err("output already has a live writer".into());
            }
        }
        #[cfg(not(unix))]
        {
            return Err("dataset supervisor currently requires Unix flock".into());
        }
        Ok(Self { _file: file })
    }
}

/// Follow no directory symlinks, including components above the output root.
fn checked_directory(path: &Path, create: bool) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path)
    };
    let mut current = PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::RootDir | std::path::Component::Normal(_) => {
                current.push(component)
            }
            std::path::Component::CurDir => continue,
            _ => {
                return Err(
                    "output directory must not contain parent traversal or a platform prefix"
                        .into(),
                )
            }
        }
        match fs::symlink_metadata(&current) {
            Ok(m) if m.is_dir() && !m.file_type().is_symlink() => {}
            Ok(_) => {
                return Err(format!(
                    "{} is not an ordinary directory",
                    current.display()
                ))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && create => {
                fs::create_dir(&current).map_err(|e| format!("{}: {e}", current.display()))?;
                if let Some(parent) = current.parent() {
                    sync_dir(parent)?;
                }
            }
            Err(e) => return Err(format!("{}: {e}", current.display())),
        }
    }
    Ok(current)
}

fn sync_dir(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        File::open(path)
            .and_then(|f| f.sync_all())
            .map_err(|e| format!("sync {}: {e}", path.display()))?;
    }
    Ok(())
}

struct SupervisedChild {
    child: Child,
    finished: bool,
}
impl SupervisedChild {
    fn kill_group(&mut self) {
        #[cfg(unix)]
        unsafe {
            libc::kill(-(self.child.id() as i32), libc::SIGKILL);
        }
        let _ = self.child.kill();
    }
    fn stop(&mut self) -> Result<ExitStatus, String> {
        self.kill_group();
        let exit = self.child.wait().map_err(|e| e.to_string())?;
        self.finished = true;
        Ok(exit)
    }
    fn poll(&mut self) -> Result<Option<ExitStatus>, String> {
        let exit = self.child.try_wait().map_err(|e| e.to_string())?;
        if exit.is_some() {
            // A backend may have spawned children. Its process group must not
            // survive the supervised invocation even after the worker exits.
            self.kill_group();
            self.finished = true;
        }
        Ok(exit)
    }
}
impl Drop for SupervisedChild {
    fn drop(&mut self) {
        if !self.finished {
            self.kill_group();
            let _ = self.child.wait();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CellManifest {
    schema: String,
    cell_index: u64,
    plan_sha256: String,
    config: CaseConfig,
    outcome: Value,
    artifacts: Vec<Artifact>,
}

fn worker(config: &CaseConfig, out: &Path) -> Result<(), String> {
    let out = checked_directory(out, false)?;
    let mut registry = Vec::<Artifact>::new();
    let mut known_answer = false;
    let mut factor_base_verified = false;
    let result = (|| {
        let case = telemetry::measured(&out, &mut registry, "curve-construction", || {
            curves::generate(config)
        })?;
        telemetry::measured(&out, &mut registry, "known-answer-replay", || {
            curves::verify(&case.public_instance, &case.private_witness)
        })?;
        telemetry::measured(&out, &mut registry, "factor-base-replay", || {
            curves::verify_factor_base(&case.public_instance, &case.factor_base)
        })?;
        registry.push(storage::write_json(
            &out,
            "public.json",
            "public-instance",
            &case.public_instance,
        )?);
        registry.push(storage::write_json(
            &out,
            "witness.json",
            "restricted-known-answer",
            &case.private_witness,
        )?);
        registry.push(storage::write_json(
            &out,
            "factor-base.json",
            "factor-base",
            &case.factor_base,
        )?);
        known_answer = true;
        factor_base_verified = true;
        let solved = telemetry::measured(&out, &mut registry, "solver", || {
            solvers::solve_with_event_sink(config, &case, &out.join("live"))
        })?;
        let mut names = BTreeSet::new();
        for artifact in solved.artifacts {
            if [
                "config.json",
                "public.json",
                "witness.json",
                "factor-base.json",
                "result.json",
                "manifest.json",
                "stdout.log",
                "stderr.log",
                "plan.json",
                "run.json",
                "protocol.json",
                "attempt.json",
                "worker-process.json",
                "host.json",
                "worker-artifacts.json",
            ]
            .contains(&artifact.path.as_str())
                || artifact.path.starts_with("publication/")
                || artifact.path.starts_with("telemetry/")
                || !names.insert(artifact.path.clone())
            {
                return Err("solver artifact collides with reserved output".into());
            }
            registry.push(storage::write_bytes(
                &out,
                &artifact.path,
                &artifact.role,
                &artifact.encoding,
                &artifact.bytes,
            )?);
        }
        Ok::<Value, String>(solved.outcome)
    })();
    let mut outcome = match result {
        Ok(v) => v,
        Err(e) => json!({"status":"error","reason":e,
        "solver_completed":false,"scalar_recovered_without_witness":false}),
    };
    outcome["known_answer_verified"] = json!(known_answer);
    outcome["factor_base_verified"] = json!(factor_base_verified);
    registry.push(storage::write_json(
        &out,
        "result.json",
        "solver-outcome",
        &outcome,
    )?);
    storage::write_json(
        &out,
        "worker-artifacts.json",
        "artifact-registry",
        &registry,
    )?;
    Ok(())
}

fn verify_resumed_cell(
    dir: &Path,
    binding: &Value,
    config: &CaseConfig,
    index: u64,
    plan_sha: &str,
) -> Result<(), String> {
    let manifest = verify_cell(dir, true)?;
    if manifest.plan_sha256 != plan_sha
        || &manifest.config != config
        || manifest.cell_index != index
        || read_json::<Value>(&dir.join("run.json"))? != *binding
    {
        return Err(format!(
            "cell {index} mismatches frozen plan or run binding"
        ));
    }
    Ok(())
}

fn generation_source() -> Result<&'static str, String> {
    let source = crate::build_provenance::git_commit()
        .ok_or("build with CRYPTO_BUILD_GIT_COMMIT before generation")?;
    if source.len() != 40 || !source.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("build source must be a complete 40-character Git commit".into());
    }
    if crate::build_provenance::git_dirty() != Some(false) {
        return Err("generation requires CRYPTO_BUILD_GIT_DIRTY=false build provenance".into());
    }
    Ok(source)
}

fn generate(a: &GenerateArgs) -> Result<(), String> {
    if a.max_cases == 0
        || a.shard_count == 0
        || a.shard_index >= a.shard_count
        || a.seconds_per_case == 0
        || a.memory_mib < 128
        || a.trace_mib == 0
    {
        return Err("invalid cell, shard, time, memory or trace limit".into());
    }
    let plan: DatasetPlan = read_json(&a.plan)?;
    plan.validate()?;
    let protocol = read_json_bytes(&a.protocol)?;
    verify_protocol(&plan, &protocol)?;
    let source = generation_source()?;
    if a.publish {
        if let Ok(uri) = std::env::var("ISOLAB_RESULTS_URI") {
            if !uri.is_empty() {
                lab_receipt_remote(&uri)?;
            }
        }
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let binary_sha = crate::store::hash_file(&exe)?.0;
    let session = open_budget(&a.budget)?;
    let remote = if a.publish {
        Some(configured_remote(a.remote.as_deref())?)
    } else {
        None
    };
    if let Some(remote) = &remote {
        require_remote_budget(remote, session.as_ref())?;
    }
    if a.publish
        && std::env::var("ISOLAB_RESULTS_URI").is_ok_and(|uri| !uri.is_empty())
        && session.is_none()
    {
        return Err("S3 lab receipt routing requires an active budget even when the data remote is a file backend".into());
    }
    let envelope: Option<Resources> = a
        .budget_cell_envelope
        .as_ref()
        .map(|path| read_json(path))
        .transpose()?;
    if session.is_some() && envelope.is_none() {
        return Err("budgeted generation requires a finite --budget-cell-envelope".into());
    }
    if session.is_none() && envelope.is_some() {
        return Err("cell envelope requires a bound budget session".into());
    }
    let _lease = Lease::acquire(&a.out)?;
    let out = checked_directory(&a.out, false)?;
    assert_no_workers(&out)?;
    let plan_sha = plan.binding_sha256()?;
    if let Some(session) = &session {
        session.validate_binding(&RunBinding {
            source_commit: source.into(),
            binary_sha256: binary_sha.clone(),
            plan_sha256: plan_sha.clone(),
            protocol_sha256: digest(&protocol),
            shard_index: a.shard_index,
            shard_count: a.shard_count,
        })?;
    }
    let context = budget_transport(session)?;
    let binding = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,
        "source_commit":source,"binary_sha256":binary_sha,"shard_index":a.shard_index,
        "shard_count":a.shard_count,"seconds_per_case":a.seconds_per_case,"memory_mib":a.memory_mib,
        "trace_mib":a.trace_mib,"output_budget_scope":"all_cell_files_before_manifest_seal",
        "clock_valid_for_comparison":false});
    let result = (|| {
        storage::write_json(&out, "run.json", "run-binding", &binding)?;
        storage::write_json(&out, "plan.json", "frozen-plan", &plan)?;
        storage::write_bytes(&out, "protocol.json", "frozen-protocol", "json", &protocol)?;
        let cells = checked_directory(&out.join("cells"), true)?;
        let mut attempted = 0;
        for index in (a.shard_index..plan.cell_count()?)
            .step_by(usize::try_from(a.shard_count).map_err(|_| "shard count too large")?)
        {
            let config = plan.cell(index)?;
            let dir = cells.join(format!("{index:012}"));
            if dir.join("manifest.json").exists() {
                verify_resumed_cell(&dir, &binding, &config, index, &plan_sha)?;
                if a.publish {
                    let scope =
                        cell_budget_scope(context.as_ref(), envelope.as_ref(), &plan_sha, index)?;
                    publish_cell(&out, &dir, a.remote.as_deref(), context.as_ref(), scope)?;
                    finalize_budget(&out, context.as_ref(), remote.as_ref(), true)?;
                }
                continue;
            }
            if attempted >= a.max_cases {
                break;
            }
            // An unfinished attempt is retained under its original directory. Retrying
            // requires explicit reconciliation; no surviving child is duplicated.
            if dir.exists() {
                return Err(format!(
                    "cell {index} has an incomplete attempt; reconcile it before retrying"
                ));
            }
            let scope = cell_budget_scope(context.as_ref(), envelope.as_ref(), &plan_sha, index)?;
            fs::create_dir(&dir).map_err(|e| e.to_string())?;
            sync_dir(&cells)?;
            storage::write_json(&dir, "run.json", "restricted-run-binding", &binding)?;
            storage::write_json(&dir, "plan.json", "restricted-frozen-plan", &plan)?;
            storage::write_bytes(
                &dir,
                "protocol.json",
                "restricted-frozen-protocol",
                "json",
                &protocol,
            )?;
            storage::write_json(&dir, "config.json", "restricted-provenance", &config)?;
            storage::write_json(
                &dir,
                "host.json",
                "host-measurement-context",
                &host_context(),
            )?;
            storage::write_json(
                &dir,
                "attempt.json",
                "attempt-provenance",
                &json!({
            "schema":"crypto.ml-dataset.attempt/v1","cell_index":index,"plan_sha256":plan_sha,
            "supervisor_pid":std::process::id(),"started_unix_nanos":unix_nanos()?.to_string(),
            "status":"attempt_started"}),
            )?;
            let stdout = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dir.join("stdout.log"))
                .map_err(|e| e.to_string())?;
            let stderr = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dir.join("stderr.log"))
                .map_err(|e| e.to_string())?;
            let mut command = Command::new(&exe);
            command
                .args(["dataset", "worker", "--config"])
                .arg(dir.join("config.json"))
                .arg("--out")
                .arg(&dir)
                .env("RAYON_NUM_THREADS", "1")
                .stdout(Stdio::from(stdout))
                .stderr(Stdio::from(stderr));
            let memory = a
                .memory_mib
                .checked_mul(1024 * 1024)
                .ok_or("memory limit overflow")?;
            #[cfg(target_os = "linux")]
            {
                use std::os::unix::process::CommandExt;
                let parent = std::process::id();
                // Limit the child only. Kill it if its supervising process vanishes.
                unsafe {
                    command.pre_exec(move || {
                        if libc::setpgid(0, 0) != 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        if libc::getppid() as u32 != parent {
                            return Err(std::io::Error::other("supervisor exited"));
                        }
                        let limit = libc::rlimit {
                            rlim_cur: memory,
                            rlim_max: memory,
                        };
                        if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        Ok(())
                    });
                }
            }
            #[cfg(all(unix, not(target_os = "linux")))]
            {
                use std::os::unix::process::CommandExt;
                command.process_group(0);
            }
            let start = Instant::now();
            let mut child = SupervisedChild {
                child: command.spawn().map_err(|e| e.to_string())?,
                finished: false,
            };
            storage::write_json(
                &dir,
                "worker-process.json",
                "process-provenance",
                &json!({
            "worker_pid":child.child.id(),"supervisor_pid":std::process::id(),
            "output":dir,"process_group":child.child.id()}),
            )?;
            let mut stop_reason = None;
            let exit = loop {
                if let Some(status) = child.poll()? {
                    break status;
                }
                let bytes = tree_bytes_while_running(&dir)?;
                if start.elapsed() >= Duration::from_secs(a.seconds_per_case) {
                    stop_reason = Some("timeout");
                }
                if bytes > a.trace_mib.saturating_mul(1024 * 1024) {
                    stop_reason = Some("trace_budget_exhausted");
                }
                if stop_reason.is_some() {
                    break child.stop()?;
                }
                std::thread::sleep(Duration::from_millis(100));
            };
            let final_bytes = tree_bytes_while_running(&dir)?;
            if final_bytes > a.trace_mib.saturating_mul(1024 * 1024) {
                stop_reason = Some("trace_budget_exhausted");
            }
            let mut outcome: Value = if dir.join("result.json").exists() {
                read_json(&dir.join("result.json"))?
            } else {
                json!({"status":"worker_failed","solver_completed":false})
            };
            if let Some(reason) = stop_reason {
                outcome = json!({"status":reason,"solver_completed":false});
            }
            if !exit.success() && stop_reason.is_none() {
                outcome = json!({"status":"worker_failed","exit_code":exit.code(),"solver_completed":false});
            }
            outcome["supervisor"] = json!({"exit_code":exit.code(),"elapsed_ms_diagnostic":start.elapsed().as_secs_f64()*1000.0,
            "clock_valid_for_comparison":false,"retired_instructions":null,
            "instruction_counter_status":"whole_worker_not_counted; see phase telemetry for available hardware counts","memory_limit_bytes":memory,
            "worker_output_bytes_before_seal":final_bytes,"output_budget_bytes":a.trace_mib.saturating_mul(1024*1024),
            "output_budget_scope":"all_cell_files_before_manifest_seal",
            "checkpoint_mid_call":false,"interrupted_trace_loss_possible":true});
            let registry: Vec<Artifact> = if dir.join("worker-artifacts.json").exists() {
                read_json(&dir.join("worker-artifacts.json"))?
            } else {
                Vec::new()
            };
            storage::verify_manifest(&dir, &registry)?;
            let registered: BTreeMap<_, _> =
                registry.iter().map(|a| (a.path.as_str(), a)).collect();
            let mut artifacts = Vec::new();
            for path in tree_files(&dir)? {
                let relative = path
                    .strip_prefix(&dir)
                    .map_err(|e| e.to_string())?
                    .to_str()
                    .ok_or("non-UTF8 output")?
                    .to_string();
                File::open(&path)
                    .and_then(|f| f.sync_all())
                    .map_err(|e| format!("sync {}: {e}", path.display()))?;
                if let Some(artifact) = registered.get(relative.as_str()) {
                    artifacts.push((**artifact).clone());
                    continue;
                }
                let (sha256, bytes) = crate::store::hash_file(&path)?;
                let (role, encoding) = match relative.as_str() {
                    "witness.json" => ("restricted-known-answer", "json"),
                    "config.json" => ("restricted-provenance", "json"),
                    "plan.json" => ("restricted-frozen-plan", "json"),
                    "run.json" => ("restricted-run-binding", "json"),
                    "protocol.json" => ("restricted-frozen-protocol", "json"),
                    "attempt.json" | "worker-process.json" => ("process-provenance", "json"),
                    "host.json" => ("host-measurement-context", "json"),
                    "worker-artifacts.json" => ("artifact-registry", "json"),
                    "stdout.log" | "stderr.log" => ("process-log", "utf8"),
                    _ => ("unregistered-partial-artifact", "native"),
                };
                artifacts.push(Artifact {
                    role: role.into(),
                    path: relative,
                    sha256,
                    bytes,
                    encoding: encoding.into(),
                });
            }
            sync_dir(&dir)?;
            let has = |path: &str| artifacts.iter().any(|a| a.path == path);
            outcome["known_answer_verified"] = json!(has("public.json") && has("witness.json"));
            outcome["factor_base_verified"] = json!(has("public.json") && has("factor-base.json"));
            let manifest = CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan_sha.clone(),
                config,
                outcome,
                artifacts,
            };
            storage::write_json(&dir, "manifest.json", "cell-manifest", &manifest)?;
            verify_cell(&dir, true)?;
            if a.publish {
                publish_cell(&out, &dir, a.remote.as_deref(), context.as_ref(), scope)?;
            }
            finalize_budget(&out, context.as_ref(), remote.as_ref(), a.publish)?;
            attempted += 1;
            let manifest_bytes = crate::store::hash_file(&dir.join("manifest.json"))?.1;
            let publication_bytes = if dir.join("publication").exists() {
                tree_bytes_while_running(&dir.join("publication"))?
            } else {
                0
            };
            println!(
                "{}",
                json!({"cell_index":index,"outcome":manifest.outcome,"durable_remote":a.publish,
            "worker_output_bytes_before_seal":final_bytes,"sealed_manifest_bytes":manifest_bytes,
            "publication_metadata_bytes":publication_bytes})
            );
        }
        println!("{}", verify_run(&out, true)?);
        Ok(())
    })();
    combine_finalization(
        result,
        finalize_budget(&out, context.as_ref(), remote.as_ref(), a.publish),
    )
}

fn tree_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_symlink() {
            return Err("symlink in artifact tree".into());
        }
        if ty.is_dir() {
            files.extend(tree_files(&entry.path())?)
        } else if ty.is_file() {
            files.push(entry.path())
        } else {
            return Err("non-regular artifact".into());
        }
    }
    files.sort();
    Ok(files)
}

fn tree_bytes_while_running(root: &Path) -> Result<u64, String> {
    let mut bytes = 0u64;
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = match fs::symlink_metadata(entry.path()) {
            Ok(value) => value,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e.to_string()),
        };
        if metadata.file_type().is_symlink() {
            return Err("symlink in running artifact tree".into());
        }
        if metadata.is_dir() {
            bytes = bytes.saturating_add(tree_bytes_while_running(&entry.path())?);
        } else if metadata.is_file() {
            bytes = bytes.saturating_add(metadata.len());
        } else {
            return Err("non-regular running artifact".into());
        }
    }
    Ok(bytes)
}

fn verify_protocol(plan: &DatasetPlan, bytes: &[u8]) -> Result<(), String> {
    let expected = plan
        .protocol_sha256
        .as_deref()
        .ok_or("generation requires a frozen protocol_sha256 in the plan")?;
    if digest(bytes) != expected {
        return Err("protocol file SHA-256 differs from frozen plan".into());
    }
    let text = std::str::from_utf8(bytes).map_err(|e| format!("protocol UTF-8: {e}"))?;
    let mut value = protocol_unique_json(text, 0)?;
    if value["schema"] != "experiment.protocol/v1" {
        return Err("expected experiment.protocol/v1 protocol".into());
    }
    let freeze = value.get("freeze").ok_or("protocol has no freeze record")?;
    if freeze["status"] != "frozen" || freeze["filed_before_affected_data"] != true {
        return Err("protocol must be frozen and filed before affected data".into());
    }
    if freeze["frozen_at"]
        .as_str()
        .is_none_or(|s| s.trim().is_empty())
    {
        return Err("frozen protocol requires frozen_at".into());
    }
    let frozen_hash = freeze["protocol_sha256"]
        .as_str()
        .ok_or("missing frozen protocol SHA-256")?
        .to_owned();
    crate::store::index::validate_sha256(&frozen_hash)?;
    value
        .as_object_mut()
        .ok_or("protocol must be an object")?
        .remove("freeze");
    if digest(&protocol_canonical_bytes(&value)?) != frozen_hash {
        return Err("protocol canonical body differs from its freeze hash; file an amendment before new data".into());
    }
    Ok(())
}

/// Match the pinned official checker's duplicate-key rejection while preserving
/// arbitrary-sized JSON integers. RawValue avoids converting integers to f64.
fn protocol_unique_json(text: &str, depth: usize) -> Result<Value, String> {
    if depth > 128 {
        return Err("protocol JSON nesting exceeds 128 levels".into());
    }
    let text = text.trim();
    if text.starts_with('{') {
        struct UniqueMembers(BTreeMap<String, Box<serde_json::value::RawValue>>);
        impl<'de> Deserialize<'de> for UniqueMembers {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = UniqueMembers;
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str("an object without duplicate keys")
                    }
                    fn visit_map<A: serde::de::MapAccess<'de>>(
                        self,
                        mut map: A,
                    ) -> Result<Self::Value, A::Error> {
                        let mut members = BTreeMap::new();
                        while let Some((key, value)) =
                            map.next_entry::<String, Box<serde_json::value::RawValue>>()?
                        {
                            if members.insert(key.clone(), value).is_some() {
                                return Err(serde::de::Error::custom(format!(
                                    "duplicate protocol key {key:?}"
                                )));
                            }
                        }
                        Ok(UniqueMembers(members))
                    }
                }
                deserializer.deserialize_map(Visitor)
            }
        }
        let members: UniqueMembers =
            serde_json::from_str(text).map_err(|e| format!("protocol JSON: {e}"))?;
        let mut object = serde_json::Map::new();
        for (key, raw) in members.0 {
            object.insert(key, protocol_unique_json(raw.get(), depth + 1)?);
        }
        Ok(Value::Object(object))
    } else if text.starts_with('[') {
        let values: Vec<Box<serde_json::value::RawValue>> =
            serde_json::from_str(text).map_err(|e| format!("protocol JSON: {e}"))?;
        Ok(Value::Array(
            values
                .iter()
                .map(|raw| protocol_unique_json(raw.get(), depth + 1))
                .collect::<Result<_, _>>()?,
        ))
    } else {
        serde_json::from_str(text).map_err(|e| format!("protocol JSON: {e}"))
    }
}

/// Python json.dumps uses repr's shortest finite binary64 digits with fixed
/// notation for decimal exponents -4..15, otherwise signed, padded exponents.
/// serde_json supplies the same shortest round-trip digits; explicitly reformat
/// them so Rust's different exponent/notation choices cannot alter the hash.
fn protocol_float(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err("non-finite protocol number".into());
    }
    if value == 0.0 {
        return Ok(if value.is_sign_negative() {
            "-0.0"
        } else {
            "0.0"
        }
        .into());
    }
    let raw = serde_json::to_string(&value).map_err(|e| e.to_string())?;
    let (sign, unsigned) = if let Some(s) = raw.strip_prefix('-') {
        ("-", s)
    } else {
        ("", raw.as_str())
    };
    let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (
            mantissa,
            exponent
                .parse::<i32>()
                .map_err(|_| "invalid finite float exponent")?,
        ),
        None => (unsigned, 0),
    };
    let point = mantissa.find('.').unwrap_or(mantissa.len()) as i32;
    let all_digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let first = all_digits
        .find(|c| c != '0')
        .ok_or("invalid nonzero float digits")?;
    let digits = all_digits[first..].trim_end_matches('0');
    let scientific_exponent = point + exponent - first as i32 - 1;
    if !(-4..16).contains(&scientific_exponent) {
        let tail = if digits.len() > 1 {
            format!(".{}", &digits[1..])
        } else {
            String::new()
        };
        let exponent_sign = if scientific_exponent < 0 { '-' } else { '+' };
        return Ok(format!(
            "{sign}{}{tail}e{exponent_sign}{:02}",
            &digits[..1],
            scientific_exponent.unsigned_abs()
        ));
    }
    let position = scientific_exponent + 1;
    if position <= 0 {
        Ok(format!(
            "{sign}0.{}{digits}",
            "0".repeat((-position) as usize)
        ))
    } else if position as usize >= digits.len() {
        Ok(format!(
            "{sign}{digits}{}.0",
            "0".repeat(position as usize - digits.len())
        ))
    } else {
        let position = position as usize;
        Ok(format!(
            "{sign}{}.{}",
            &digits[..position],
            &digits[position..]
        ))
    }
}

/// Canonical bytes of the protocol body, matching experiment_protocol.py at
/// crypto-autoresearcher commit 37e193bf: sorted keys, UTF-8, no whitespace,
/// finite JSON numbers, and the entire top-level freeze block excluded by the
/// caller. Full schema/semantic validation remains the official checker's job.
fn protocol_canonical_bytes(value: &Value) -> Result<Vec<u8>, String> {
    fn append(value: &Value, out: &mut Vec<u8>) -> Result<(), String> {
        match value {
            Value::Null => out.extend_from_slice(b"null"),
            Value::Bool(true) => out.extend_from_slice(b"true"),
            Value::Bool(false) => out.extend_from_slice(b"false"),
            Value::String(s) => out.extend(serde_json::to_vec(s).map_err(|e| e.to_string())?),
            Value::Number(n) => {
                let raw = n.to_string();
                let canonical = if raw.contains(['.', 'e', 'E']) {
                    protocol_float(n.as_f64().filter(|v| v.is_finite()).ok_or(
                        "protocol floating number cannot be represented as finite binary64",
                    )?)?
                } else {
                    num_bigint::BigInt::parse_bytes(raw.as_bytes(), 10)
                        .ok_or("invalid protocol integer")?
                        .to_string()
                };
                out.extend_from_slice(canonical.as_bytes());
            }
            Value::Array(values) => {
                out.push(b'[');
                for (i, item) in values.iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    append(item, out)?;
                }
                out.push(b']');
            }
            Value::Object(values) => {
                out.push(b'{');
                let mut keys: Vec<_> = values.keys().collect();
                keys.sort();
                for (i, key) in keys.iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    out.extend(serde_json::to_vec(key).map_err(|e| e.to_string())?);
                    out.push(b':');
                    append(&values[*key], out)?;
                }
                out.push(b'}');
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    append(value, &mut out)?;
    Ok(out)
}

fn bound_run(root: &Path) -> Result<(DatasetPlan, Value, String), String> {
    checked_directory(root, false)?;
    for name in ["plan.json", "run.json", "protocol.json"] {
        let path = root.join(name);
        let metadata =
            fs::symlink_metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(format!("run provenance {name} must be a regular file"));
        }
    }
    let plan: DatasetPlan = read_json(&root.join("plan.json"))?;
    plan.validate()?;
    verify_protocol(&plan, &read_json_bytes(&root.join("protocol.json"))?)?;
    let hash = plan.binding_sha256()?;
    let binding: Value = read_json(&root.join("run.json"))?;
    if binding["schema"] != "crypto.ml-dataset.run/v1" || binding["plan_sha256"] != hash {
        return Err("run/plan digest mismatch".into());
    }
    crate::store::index::validate_sha256(
        binding["binary_sha256"]
            .as_str()
            .ok_or("missing binary SHA-256")?,
    )?;
    if binding["source_commit"]
        .as_str()
        .is_none_or(|s| s.len() != 40 || !s.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err("missing full source commit in run binding".into());
    }
    Ok((plan, binding, hash))
}

fn unix_nanos() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .map_err(|e| e.to_string())
}

#[cfg(target_os = "linux")]
fn assert_no_workers(root: &Path) -> Result<(), String> {
    let root = checked_directory(root, false)?;
    // A worker's external descendants may outlive a supervisor killed before
    // its guard runs. Refuse reconciliation while any recorded process group
    // remains, even if its original worker command has disappeared.
    for dir in cell_directories(&root)? {
        let identity = dir.join("worker-process.json");
        if !identity.exists() || dir.join("manifest.json").exists() {
            continue;
        }
        let value: Value = read_json(&identity)?;
        let pgid = value["process_group"]
            .as_u64()
            .filter(|p| *p > 0 && *p <= i32::MAX as u64)
            .ok_or("invalid retained worker process group")?;
        let alive = unsafe { libc::kill(-(pgid as i32), 0) };
        if alive == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
            return Err(format!(
                "recorded worker process group {pgid} still exists or cannot be inspected"
            ));
        }
    }
    for entry in fs::read_dir("/proc").map_err(|e| format!("cannot inspect live workers: {e}"))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let Some(pid) = entry
            .file_name()
            .to_str()
            .and_then(|s| s.parse::<u32>().ok())
        else {
            continue;
        };
        if pid == std::process::id() {
            continue;
        }
        // Workers execute under this account. Permission errors inspecting a
        // same-user process fail closed instead of assuming it has exited.
        use std::os::unix::fs::MetadataExt;
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e.to_string()),
        };
        if metadata.uid() != unsafe { libc::geteuid() } {
            continue;
        }
        let command = match fs::read(entry.path().join("cmdline")) {
            Ok(value) => value,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(format!("cannot inspect process {pid}: {e}")),
        };
        let args: Vec<_> = command
            .split(|b| *b == 0)
            .filter(|v| !v.is_empty())
            .collect();
        let worker = args
            .windows(2)
            .any(|pair| pair[0] == b"dataset" && pair[1] == b"worker");
        if !worker {
            continue;
        }
        for pair in args.windows(2) {
            if pair[0] != b"--out" {
                continue;
            }
            use std::os::unix::ffi::OsStrExt;
            let path = Path::new(std::ffi::OsStr::from_bytes(pair[1]));
            let resolved = if path.is_absolute() {
                path.to_path_buf()
            } else {
                fs::read_link(entry.path().join("cwd"))
                    .map_err(|e| format!("cannot resolve worker {pid} cwd: {e}"))?
                    .join(path)
            };
            let resolved = resolved
                .canonicalize()
                .map_err(|e| format!("cannot resolve worker {pid} output: {e}"))?;
            if resolved.starts_with(&root) {
                return Err(format!(
                    "live worker {pid} still owns output under {}; reconcile only after it exits",
                    root.display()
                ));
            }
        }
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn assert_no_workers(_root: &Path) -> Result<(), String> {
    Err("safe worker reconciliation currently requires Linux /proc".into())
}

fn reconcile(root: &Path, cell: u64, reason: &str) -> Result<(), String> {
    if reason.trim().is_empty() {
        return Err("reconciliation requires a concrete reason".into());
    }
    let _lease = Lease::acquire(root)?;
    let root = checked_directory(root, false)?;
    let (plan, binding, hash) = bound_run(&root)?;
    let source =
        crate::build_provenance::git_commit().ok_or("reconciliation requires source provenance")?;
    let binary = crate::store::hash_file(&std::env::current_exe().map_err(|e| e.to_string())?)?.0;
    if binding["source_commit"] != source
        || binding["binary_sha256"] != binary
        || crate::build_provenance::git_dirty() != Some(false)
    {
        return Err(
            "reconcile with the original clean source and binary recorded in run.json".into(),
        );
    }
    assert_no_workers(&root)?;
    let expected = plan.cell(cell)?;
    let shard_count = binding["shard_count"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("invalid shard count")?;
    if cell % shard_count
        != binding["shard_index"]
            .as_u64()
            .ok_or("invalid shard index")?
    {
        return Err("cell does not belong to run shard".into());
    }
    let dir = checked_directory(&root.join("cells").join(format!("{cell:012}")), false)?;
    if dir.join("manifest.json").exists() {
        return Err("completed cell must be verified, not reconciled as interrupted".into());
    }
    if dir.join("config.json").exists()
        && read_json::<CaseConfig>(&dir.join("config.json"))? != expected
    {
        return Err("interrupted cell config differs from plan".into());
    }
    let archive = retain_interrupted(
        &root,
        cell,
        &json!({
        "schema":"crypto.ml-dataset.reconciliation/v1","cell_index":cell,"plan_sha256":hash,
        "reason":reason,"owner_scan":"no_same_user_matching_workers","source_commit":source,
        "binary_sha256":binary,"reconciled_unix_nanos":unix_nanos()?.to_string(),
        "previous_attempt_retained":true,"retry_performed":false}),
    )?;
    println!(
        "{}",
        json!({"cell_index":cell,"retained_at":archive,"retry_allowed":true,"retry_performed":false})
    );
    Ok(())
}

/// Called only with the run lease and a successful source/live-owner audit.
fn retain_interrupted(root: &Path, cell: u64, record: &Value) -> Result<PathBuf, String> {
    let dir = checked_directory(&root.join("cells").join(format!("{cell:012}")), false)?;
    if dir.join("manifest.json").exists() {
        return Err("refusing to archive a completed cell".into());
    }
    let archive_parent = checked_directory(
        &root
            .join("interrupted-attempts")
            .join(format!("{cell:012}")),
        true,
    )?;
    let archive = archive_parent.join(format!("{}-{}", unix_nanos()?, std::process::id()));
    if archive.exists() {
        return Err("reconciliation archive collision".into());
    }
    // A run lock plus /proc check excludes an authorized surviving owner. The
    // original bytes are retained before the cell name becomes reusable.
    fs::rename(&dir, &archive).map_err(|e| e.to_string())?;
    sync_dir(dir.parent().expect("cell parent"))?;
    sync_dir(&archive_parent)?;
    storage::write_json(
        &archive,
        "reconciliation.json",
        "reconciliation-receipt",
        record,
    )?;
    Ok(archive)
}

fn completed_cells(root: &Path) -> Result<Vec<PathBuf>, String> {
    Ok(cell_directories(root)?
        .into_iter()
        .filter(|dir| dir.join("manifest.json").exists())
        .collect())
}

fn cell_directories(root: &Path) -> Result<Vec<PathBuf>, String> {
    let cells = root.join("cells");
    if !cells.exists() {
        return Ok(Vec::new());
    }
    let cells = checked_directory(&cells, false)?;
    let mut result = Vec::new();
    for entry in fs::read_dir(cells).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            return Err("unexpected entry in cells".into());
        }
        let name = entry
            .file_name()
            .to_str()
            .ok_or("non-UTF8 cell name")?
            .to_owned();
        let index: u64 = name.parse().map_err(|_| "nonnumeric cell directory")?;
        if name != format!("{index:012}") {
            return Err("noncanonical cell directory".into());
        }
        result.push(entry.path());
    }
    result.sort();
    Ok(result)
}

fn verify_cell(dir: &Path, replay: bool) -> Result<CellManifest, String> {
    checked_directory(dir, false)?;
    let metadata = fs::symlink_metadata(dir.join("manifest.json")).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("manifest must be a regular file".into());
    }
    let manifest: CellManifest = read_json(&dir.join("manifest.json"))?;
    if manifest.schema != "crypto.ml-dataset.cell/v1" {
        return Err("unknown cell schema".into());
    }
    storage::verify_manifest(dir, &manifest.artifacts)?;
    for required in [
        "config.json",
        "run.json",
        "plan.json",
        "protocol.json",
        "attempt.json",
        "worker-process.json",
        "stdout.log",
        "stderr.log",
    ] {
        if !manifest.artifacts.iter().any(|a| a.path == required) {
            return Err(format!("manifest omits mandatory artifact {required}"));
        }
    }
    let (plan, _binding, hash) = bound_run(dir)?;
    if hash != manifest.plan_sha256
        || plan.cell(manifest.cell_index)? != manifest.config
        || read_json::<CaseConfig>(&dir.join("config.json"))? != manifest.config
    {
        return Err("cell provenance differs from frozen plan".into());
    }
    let has = |path: &str| manifest.artifacts.iter().any(|a| a.path == path);
    let complete_known_answer = has("public.json") && has("witness.json");
    let complete_factor_base = has("public.json") && has("factor-base.json");
    if manifest.outcome["known_answer_verified"] == true && !complete_known_answer {
        return Err("known-answer label lacks required artifacts".into());
    }
    if manifest.outcome["factor_base_verified"] == true && !complete_factor_base {
        return Err("factor-base label lacks required artifacts".into());
    }
    if manifest.outcome["scalar_recovered_without_witness"] == true
        && !manifest
            .artifacts
            .iter()
            .any(|a| a.role == "restricted-independent-scalar-label")
    {
        return Err("independent scalar label lacks its recovery certificate".into());
    }
    if manifest.outcome["pdp_witness_verified"] == true
        && !manifest
            .artifacts
            .iter()
            .any(|a| a.role == "independently-verified-pdp")
    {
        return Err("PDP label lacks its group replay certificate".into());
    }
    if replay && complete_known_answer {
        curves::verify(
            &read_json(&dir.join("public.json"))?,
            &read_json(&dir.join("witness.json"))?,
        )?;
    }
    if replay && complete_factor_base {
        let public = read_json(&dir.join("public.json"))?;
        let base = read_json(&dir.join("factor-base.json"))?;
        curves::verify_factor_base(&public, &base)?;
        for artifact in &manifest.artifacts {
            if artifact.role == "restricted-large-prime-membership-provenance" {
                relations::verify_membership_provenance(
                    &public,
                    &base,
                    &read_json(&dir.join(&artifact.path))?,
                )?;
            }
            if artifact.role == "large-prime-elimination-transcript" {
                let transcript: Value = read_json(&dir.join(&artifact.path))?;
                let requested_work =
                    if transcript["schema"] == "ml-ecdlp-generic-large-prime-transcript/v1" {
                        transcript["requested_work"].as_u64()
                    } else {
                        transcript["requested_work_limit"].as_u64()
                    };
                if requested_work != Some(manifest.config.work_limit) {
                    return Err("relation transcript work limit differs from frozen cell".into());
                }
                relations::verify_transcript(&public, &base, &transcript)?;
            }
            if [
                "independently-verified-pdp",
                "restricted-independent-scalar-label",
            ]
            .contains(&artifact.role.as_str())
            {
                solvers::verify_solution_artifact(
                    &public,
                    &base,
                    &artifact.role,
                    &read_json(&dir.join(&artifact.path))?,
                )?;
            }
        }
    }
    Ok(manifest)
}

fn publication_confirmations(dir: &Path, manifest_sha: &str) -> Result<usize, String> {
    let publication = dir.join("publication");
    if !publication.exists() {
        return Ok(0);
    }
    checked_directory(&publication, false)?;
    let mut confirmations = 0;
    for entry in fs::read_dir(&publication).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.starts_with("verified-") || !name.ends_with(".json") {
            continue;
        }
        if !entry.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("publication confirmation is not a regular file".into());
        }
        let confirmation: storage::PublicationConfirmation = read_json(&entry.path())?;
        if confirmation.schema != "ml-dataset.publication-confirmation/v1"
            || confirmation.manifest_sha256 != manifest_sha
            || !confirmation.completion.roundtrip_verified
        {
            return Err("invalid publication confirmation".into());
        }
        storage::verify_manifest(dir, std::slice::from_ref(&confirmation.completion.artifact))?;
        let record: Value = read_json(&dir.join(&confirmation.completion.artifact.path))?;
        if record["schema"] != "ml-dataset.publication/v1"
            || record["manifest_sha256"] != manifest_sha
            || record["remote"] != confirmation.remote
            || record["namespace"] != confirmation.namespace
        {
            return Err("publication confirmation is not bound to its completion record".into());
        }
        confirmations += 1;
    }
    Ok(confirmations)
}

fn catalog_confirmations(dir: &Path, manifest_sha: &str) -> Result<usize, String> {
    let publication = dir.join("publication");
    if !publication.exists() {
        return Ok(0);
    }
    checked_directory(&publication, false)?;
    let mut count = 0;
    for entry in fs::read_dir(&publication).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.starts_with("catalog-verified-") || !name.ends_with(".json") {
            continue;
        }
        if !entry.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("catalog acknowledgement is not a regular file".into());
        }
        let ack: storage::CatalogConfirmation = read_json(&entry.path())?;
        if ack.schema != "ml-dataset.publication-catalog-confirmation/v1"
            || !ack.roundtrip_verified
            || ack.manifest_sha256 != manifest_sha
            || ack.sha256 != ack.receipt.sha256
            || ack.bytes != ack.receipt.bytes
            || ack.key != format!("ml-dataset/catalog/{}/{}.json", ack.namespace, manifest_sha)
        {
            return Err("invalid catalog acknowledgement".into());
        }
        storage::verify_manifest(dir, std::slice::from_ref(&ack.receipt))?;
        let receipt: storage::PublishReceipt = read_json(&dir.join(&ack.receipt.path))?;
        if receipt.schema != "ml-dataset.publication/v1"
            || receipt.remote != ack.remote
            || receipt.namespace != ack.namespace
            || receipt.manifest_sha256 != manifest_sha
            || receipt.catalog.is_some()
        {
            return Err("catalog acknowledgement is not bound to its portable receipt".into());
        }
        if receipt.namespace
            != publication_namespace(
                &read_json(&dir.join("run.json"))?,
                read_json::<CellManifest>(&dir.join("manifest.json"))?.cell_index,
            )?
        {
            return Err("catalog namespace differs from bound run".into());
        }
        let manifest: CellManifest = read_json(&dir.join("manifest.json"))?;
        let mut expected = manifest.artifacts.clone();
        expected.push(Artifact {
            path: "manifest.json".into(),
            role: "cell-manifest".into(),
            sha256: manifest_sha.into(),
            bytes: fs::metadata(dir.join("manifest.json"))
                .map_err(|e| e.to_string())?
                .len(),
            encoding: "json".into(),
        });
        let actual: Vec<_> = receipt.objects.iter().map(|o| o.artifact.clone()).collect();
        let key = |a: &Artifact| a.path.clone();
        expected.sort_by_key(key);
        let mut actual_sorted = actual.clone();
        actual_sorted.sort_by_key(key);
        if actual_sorted != expected
            || receipt.objects.iter().any(|o| {
                !o.roundtrip_verified
                    || o.name != format!("{}/{}", receipt.namespace, o.artifact.path)
                    || crate::store::backend::object_key(&o.artifact.sha256)
                        .ok()
                        .as_ref()
                        != Some(&o.object_key)
            })
        {
            return Err("catalog object set differs from verified cell".into());
        }
        storage::verify_manifest(dir, std::slice::from_ref(&receipt.completion.artifact))?;
        let completion: Value = read_json(&dir.join(&receipt.completion.artifact.path))?;
        if !receipt.completion.roundtrip_verified
            || completion["objects"]
                != serde_json::to_value(&receipt.objects).map_err(|e| e.to_string())?
            || completion["namespace"] != receipt.namespace
            || completion["manifest_sha256"] != manifest_sha
            || completion["remote"] != receipt.remote
        {
            return Err("catalog completion does not bind its object set".into());
        }
        count += 1;
    }
    Ok(count)
}

fn verify_run(root: &Path, replay: bool) -> Result<Value, String> {
    let (plan, binding, hash) = bound_run(root)?;
    let shard_count = binding["shard_count"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("invalid shard count")?;
    let shard_index = binding["shard_index"]
        .as_u64()
        .filter(|n| *n < shard_count)
        .ok_or("invalid shard index")?;
    let mut counts = BTreeMap::<String, u64>::new();
    let mut seen = BTreeSet::new();
    let mut published_receipts = 0usize;
    let mut published_cells = 0usize;
    let mut catalog_cells = 0usize;
    let mut known_answer_cells = 0usize;
    let mut independently_solved_cells = 0usize;
    for dir in completed_cells(root)? {
        let m = verify_cell(&dir, replay)?;
        if m.plan_sha256 != hash
            || m.config != plan.cell(m.cell_index)?
            || m.cell_index % shard_count != shard_index
            || !seen.insert(m.cell_index)
        {
            return Err("cell not in bound shard or duplicate cell".into());
        }
        if dir.file_name().and_then(|s| s.to_str())
            != Some(format!("{:012}", m.cell_index).as_str())
        {
            return Err("manifest index differs from cell directory".into());
        }
        if read_json::<Value>(&dir.join("run.json"))? != binding {
            return Err("cell source/run binding differs from root".into());
        }
        *counts
            .entry(m.outcome["status"].as_str().unwrap_or("unspecified").into())
            .or_default() += 1;
        if m.outcome["known_answer_verified"] == true {
            known_answer_cells += 1;
        }
        if m.outcome["scalar_recovered_without_witness"] == true
            && m.outcome["scalar_replay_verified"] == true
        {
            independently_solved_cells += 1;
        }
        let confirmed = publication_confirmations(
            &dir,
            &crate::store::hash_file(&dir.join("manifest.json"))?.0,
        )?;
        published_receipts += confirmed;
        if confirmed > 0 {
            published_cells += 1;
        }
        if catalog_confirmations(
            &dir,
            &crate::store::hash_file(&dir.join("manifest.json"))?.0,
        )? > 0
        {
            catalog_cells += 1;
        }
    }
    let incomplete: Vec<_> = cell_directories(root)?
        .into_iter()
        .filter(|dir| !dir.join("manifest.json").exists())
        .map(|dir| dir.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    let total = plan.cell_count()?;
    let assigned = if shard_index >= total {
        0
    } else {
        (total - 1 - shard_index) / shard_count + 1
    };
    Ok(
        json!({"schema":"crypto.ml-dataset.status/v1","planned_cells":plan.cell_count()?,
        "assigned_shard_cells":assigned,"completed_shard_receipts":seen.len(),"completed_receipts":seen.len(),
        "incomplete_attempts":incomplete,"outcomes":counts,"artifact_hashes_verified":true,
        "published_completion_receipts":published_receipts,"published_cells":published_cells,
        "discoverable_catalog_cells":catalog_cells,
        "remote_rechecked_during_status":false,"known_answer_verified_cells":known_answer_cells,
        "independently_solved_ecdlp_cells":independently_solved_cells,
        "known_answer_replay_performed":replay,"shard_attempts_complete":seen.len() as u64==assigned,
        "campaign_attempts_complete":seen.len() as u64==total,"full_scope_complete":false,
        "full_scope_complete_reason":"accounted attempts and known answers do not establish all requested methods or end-to-end ECDLP solves"}),
    )
}

fn host_context() -> Value {
    let read = |path: &str| fs::read_to_string(path).ok().map(|s| s.trim().to_owned());
    let cpu = read("/proc/cpuinfo").map(|s| {
        s.lines()
            .filter(|line| {
                [
                    "vendor_id",
                    "model name",
                    "cpu family",
                    "model\t",
                    "stepping",
                    "microcode",
                    "flags",
                ]
                .iter()
                .any(|key| line.starts_with(key))
            })
            .take(7)
            .collect::<Vec<_>>()
            .join("\n")
    });
    json!({"schema":"crypto.ml-dataset.host/v1","os":std::env::consts::OS,
        "architecture":std::env::consts::ARCH,"kernel_release":read("/proc/sys/kernel/osrelease"),
        "cpu":cpu,"available_parallelism":std::thread::available_parallelism().ok().map(|n|n.get()),
        "memory_info":read("/proc/meminfo"),"perf_event_paranoid":read("/proc/sys/kernel/perf_event_paranoid"),
        "cgroup_cpu_max":read("/sys/fs/cgroup/cpu.max"),"cgroup_memory_max":read("/sys/fs/cgroup/memory.max"),
        "process_status":read("/proc/self/status").map(|s|s.lines().filter(|l|l.starts_with("Cpus_allowed_list:") || l.starts_with("Mems_allowed_list:")).collect::<Vec<_>>().join("\n")),
        "rayon_num_threads":"1","isolation_tier":"L0","clock_valid_for_comparison":false,
        "pmu_event_scope":"user space; exact availability and multiplexing recorded separately for each phase"})
}

fn publication_namespace(binding: &Value, cell: u64) -> Result<String, String> {
    // The run digest includes source, executable, runtime caps and shard axes.
    // A plan can therefore be reused by a separately identified execution
    // without repointing logical names from a prior worker index.
    let run_id = digest(&serde_json::to_vec(binding).map_err(|e| e.to_string())?);
    Ok(format!("ml-ecdlp/{run_id}/cells/{cell:012}"))
}

fn publish_cell(
    root: &Path,
    dir: &Path,
    remote: Option<&str>,
    budget: Option<&Arc<S3BudgetContext>>,
    scope: BudgetScope,
) -> Result<(), String> {
    let manifest = verify_cell(dir, true)?;
    let path = dir.join("manifest.json");
    let (sha256, bytes) = crate::store::hash_file(&path)?;
    let manifest_artifact = Artifact {
        path: "manifest.json".into(),
        role: "cell-manifest".into(),
        sha256,
        bytes,
        encoding: "json".into(),
    };
    let mut cfg = crate::store::Config::discover()?;
    cfg.index_path = root.join("store-index.json");
    if let Some(remote) = remote {
        cfg.remote = Some(crate::store::backend::RemoteSpec::parse(remote)?);
    }
    let namespace = publication_namespace(&read_json(&dir.join("run.json"))?, manifest.cell_index)?;
    if let Some(context) = budget {
        validate_budget_run(context.session(), dir)?;
    }
    let receipt = storage::publish_budgeted(
        dir,
        &manifest.artifacts,
        &manifest_artifact,
        cfg,
        &namespace,
        budget,
        scope,
    )?;
    let receipt_file = storage::write_json(
        root,
        &format!(
            "receipts/{:012}-{}.json",
            manifest.cell_index, receipt.manifest_sha256
        ),
        "publication-receipt",
        &receipt,
    )?;
    publish_lab_receipt(root, &receipt_file, budget)?;
    println!(
        "{}",
        serde_json::to_string(&receipt).map_err(|e| e.to_string())?
    );
    Ok(())
}

/// The EKS command contract supplies this explicit results destination. Keep a
/// receipt under that attempt prefix as well as the dataset's stable catalog,
/// so lab results remain discoverable after Pod/log TTL cleanup.
fn publish_lab_receipt(
    root: &Path,
    artifact: &Artifact,
    budget: Option<&Arc<S3BudgetContext>>,
) -> Result<(), String> {
    let Some(uri) = std::env::var("ISOLAB_RESULTS_URI")
        .ok()
        .filter(|s| !s.is_empty())
    else {
        return Ok(());
    };
    let remote = lab_receipt_remote(&uri)?;
    publish_lab_receipt_to(root, artifact, &remote, budget)
}

fn publish_lab_receipt_to(
    root: &Path,
    artifact: &Artifact,
    remote: &crate::store::backend::RemoteSpec,
    budget: Option<&Arc<S3BudgetContext>>,
) -> Result<(), String> {
    let remote = lab_receipt_remote(&remote.display())?;
    let uri = remote.display();
    let mut cfg = crate::store::Config::discover()?;
    cfg.remote = Some(remote);
    cfg.index_path = root.join("store-index.json");
    let store = storage::dataset_store(cfg, budget, BudgetScope::Finalization)?;
    let backend = store.remote()?;
    let key = format!("dataset-receipts/{}.json", artifact.sha256);
    storage::reserve_remote_output(
        budget,
        store.cfg.remote.as_ref().expect("validated remote"),
        &key,
        artifact,
        BudgetScope::Finalization,
    )?;
    storage::verify_manifest(root, std::slice::from_ref(artifact))?;
    storage::prepare_s3_publication(
        backend,
        store
            .cfg
            .remote
            .as_ref()
            .expect("validated lab receipt remote"),
        &key,
        &root.join(&artifact.path),
        artifact,
        root,
    )?;
    let scratch = checked_directory(&root.join("remote-verification"), true)?;
    let temporary = scratch.join(format!("{}-{}.json", artifact.sha256, unix_nanos()?));
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    // On failure the downloaded bytes remain available for diagnosis.
    backend.get(&key, &temporary)?;
    if crate::store::hash_file(&temporary)? != (artifact.sha256.clone(), artifact.bytes) {
        return Err("lab receipt remote readback mismatch".into());
    }
    storage::verify_remote_output(
        budget,
        store.cfg.remote.as_ref().expect("validated remote"),
        &key,
        BudgetScope::Finalization,
    )?;
    fs::remove_file(&temporary).map_err(|e| e.to_string())?;
    storage::write_json(
        root,
        &format!("lab-receipts/{}.json", artifact.sha256),
        "lab-publication-acknowledgement",
        &json!({"remote":uri,"object_key":key,"sha256":artifact.sha256,"bytes":artifact.bytes,"roundtrip_verified":true}),
    )?;
    Ok(())
}

fn lab_receipt_remote(uri: &str) -> Result<crate::store::backend::RemoteSpec, String> {
    let remote = crate::store::backend::RemoteSpec::parse(uri)?;
    if !matches!(&remote, crate::store::backend::RemoteSpec::S3 {bucket, prefix} if bucket == "setec-astronomy-artifacts" && !prefix.is_empty())
    {
        return Err(
            "lab receipt destination must be a nonempty prefix in setec-astronomy-artifacts".into(),
        );
    }
    Ok(remote)
}

#[cfg(all(test, target_os = "linux"))]
pub(super) mod budget_test_support {
    use super::*;
    use aws_smithy_runtime_api::client::http::{
        HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpClient,
        SharedHttpConnector,
    };
    use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
    use aws_smithy_types::body::SdkBody;
    use std::sync::Mutex;

    /// Full SDK serializer/interceptor/connector path, with no sockets or AWS.
    /// Missing keys deliberately return403, exercising immutable PUT fallback.
    #[derive(Clone)]
    pub struct MemoryWire {
        session: Arc<BudgetSession>,
        objects: Arc<Mutex<BTreeMap<String, Vec<u8>>>>,
        pub calls: Arc<Mutex<Vec<(String, String)>>>,
        pub fail_next_get: Arc<Mutex<bool>>,
    }
    impl std::fmt::Debug for MemoryWire {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("MemoryWire(no network)")
        }
    }
    impl HttpClient for MemoryWire {
        fn http_connector(
            &self,
            _: &HttpConnectorSettings,
            _: &RuntimeComponents,
        ) -> SharedHttpConnector {
            SharedHttpConnector::new(self.clone())
        }
    }
    impl HttpConnector for MemoryWire {
        fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
            let method = request.method().to_owned();
            let uri = request.uri().to_string();
            let uri = uri.split('?').next().unwrap().to_owned();
            let charged = self.session.snapshot().unwrap()["charged_attempts"]
                .as_u64()
                .unwrap();
            let mut calls = self.calls.lock().unwrap();
            assert!(
                charged > calls.len() as u64,
                "HTTP reached memory wire before durable debit"
            );
            calls.push((method.clone(), uri.clone()));
            drop(calls);
            if method == "PUT" {
                assert_eq!(request.headers().get("if-none-match"), Some("*"));
            }
            let objects = self.objects.clone();
            let fail_next_get = self.fail_next_get.clone();
            HttpConnectorFuture::new(async move {
                let incoming = aws_smithy_types::byte_stream::ByteStream::new(request.into_body())
                    .collect()
                    .await
                    .expect("collect offline SDK request")
                    .into_bytes()
                    .to_vec();
                let mut objects = objects.lock().unwrap();
                let (status, body, length) = match method.as_str() {
                    "PUT" => {
                        if let std::collections::btree_map::Entry::Vacant(entry) =
                            objects.entry(uri)
                        {
                            entry.insert(incoming);
                            (200, Vec::new(), 0)
                        } else {
                            (412, Vec::new(), 0)
                        }
                    }
                    "HEAD" => match objects.get(&uri) {
                        Some(bytes) => (200, Vec::new(), bytes.len()),
                        None => (403, Vec::new(), 0),
                    },
                    "GET" if std::mem::take(&mut *fail_next_get.lock().unwrap()) => {
                        (403, Vec::new(), 0)
                    }
                    "GET" => match objects.get(&uri) {
                        Some(bytes) => (200, bytes.clone(), bytes.len()),
                        None => (403, Vec::new(), 0),
                    },
                    _ => panic!("unexpected offline request method {method}"),
                };
                let mut response =
                    HttpResponse::new(status.try_into().unwrap(), SdkBody::from(body));
                response
                    .headers_mut()
                    .insert("content-length", length.to_string());
                response
                    .headers_mut()
                    .insert("x-amz-request-id", "offline-integration");
                Ok(response)
            })
        }
    }
    pub fn context(root: &Path, binding: Option<RunBinding>) -> (Arc<S3BudgetContext>, MemoryWire) {
        let (mut policy, mut request) = budget::test_policy_request(root);
        policy.caps.output_bytes = 16 * 1024 * 1024;
        policy.caps.download_bytes = 256 * 1024 * 1024;
        policy.caps.put_attempts = 10_000;
        policy.caps.read_attempts = 20_000;
        request.resources.output_bytes = 8 * 1024 * 1024;
        request.resources.download_bytes = 128 * 1024 * 1024;
        request.resources.put_attempts = 4000;
        request.resources.read_attempts = 10000;
        request.finalization.output_bytes = 4 * 1024 * 1024;
        request.finalization.download_bytes = 64 * 1024 * 1024;
        request.finalization.put_attempts = 2000;
        request.finalization.read_attempts = 5000;
        if let Some(binding) = binding {
            request.binding = binding;
        }
        let ledger = CampaignLedger::create(&root.join("campaign.sqlite"), &policy).unwrap();
        let grant = ledger.reserve(&request).unwrap();
        let session = Arc::new(
            BudgetSession::create(&root.join("worker.sqlite"), &policy, &grant, &request.owner)
                .unwrap(),
        );
        let wire = MemoryWire {
            session: session.clone(),
            objects: Arc::new(Mutex::new(BTreeMap::new())),
            calls: Arc::new(Mutex::new(Vec::new())),
            fail_next_get: Arc::new(Mutex::new(false)),
        };
        let context = S3BudgetContext::test_context(session, SharedHttpClient::new(wire.clone()));
        context.activate().unwrap();
        (context, wire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_budget_test_args(args: &[&str]) -> Result<BudgetCommand, clap::Error> {
        use clap::Parser;
        #[derive(Parser)]
        struct TestCli {
            #[command(flatten)]
            dataset: DatasetArgs,
        }
        let cli = TestCli::try_parse_from(args)?;
        let DatasetCommand::Budget { command } = cli.dataset.command else {
            panic!("wrong CLI route")
        };
        Ok(command)
    }

    #[test]
    fn launch_cli_requires_output_and_rejects_mutating_dry_run_flags() {
        let base = [
            "test",
            "budget",
            "reserve-launch",
            "--policy",
            "policy.json",
            "--ledger",
            "campaign.sqlite",
            "--request",
            "request.json",
        ];
        let invalid: &[&[&str]] = &[
            &[],
            &["--create-ledger"],
            &["--dry-run", "--out", "reservation.json"],
            &["--dry-run", "--create-ledger"],
        ];
        for suffix in invalid {
            let mut args = base.to_vec();
            args.extend_from_slice(suffix);
            assert!(
                parse_budget_test_args(&args).is_err(),
                "accepted {suffix:?}"
            );
        }
        let mut args = base.to_vec();
        args.push("--dry-run");
        assert!(matches!(
            parse_budget_test_args(&args).unwrap(),
            BudgetCommand::ReserveLaunch {
                dry_run: true,
                out: None,
                create_ledger: false,
                ..
            }
        ));
        let mut args = base.to_vec();
        args.extend_from_slice(&["--out", "reservation.json", "--create-ledger"]);
        assert!(matches!(
            parse_budget_test_args(&args).unwrap(),
            BudgetCommand::ReserveLaunch {
                dry_run: false,
                out: Some(_),
                create_ledger: true,
                ..
            }
        ));
        assert!(parse_budget_test_args(&[
            "test",
            "budget",
            "bind-launch",
            "--policy",
            "policy.json",
            "--ledger",
            "campaign.sqlite",
            "--binding",
            "binding.json",
            "--out",
            "grant.json",
            "--create-ledger",
        ])
        .is_err());
        // Direct native callers cannot bypass clap's read-only constraints.
        let invalid = BudgetCommand::ReserveLaunch {
            policy: PathBuf::from("not-read-policy.json"),
            ledger: PathBuf::from("not-created-ledger.sqlite"),
            request: PathBuf::from("not-read-request.json"),
            dry_run: true,
            out: Some(PathBuf::from("not-written-reservation.json")),
            create_ledger: false,
        };
        assert!(run_budget_command(&invalid)
            .unwrap_err()
            .contains("conflicts"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn launch_cli_preserves_binding_origin_and_reexports_identical_receipts() {
        let temp = TempRun::new();
        let (policy, direct) = budget::test_policy_request(&temp.0);
        let request = LaunchRequest {
            schema: budget::LAUNCH_REQUEST_SCHEMA.into(),
            launch_id: direct.grant_id.clone(),
            target: budget::LaunchTarget {
                cluster_identity: "arn:aws:eks:us-east-2:104186811093:cluster/offline-test".into(),
                namespace: "isolab".into(),
                job_name: "offline-launch-test".into(),
            },
            worker_journal_path: direct.worker_journal_path.clone(),
            binding: direct.binding.clone(),
            expires_unix_seconds: direct.expires_unix_seconds,
            resource_envelope_sha256: direct.resource_envelope_sha256.clone(),
            resources: direct.resources.clone(),
            finalization: direct.finalization.clone(),
            credential_role_arn: direct.credential_role_arn.clone(),
        };
        let policy_path = temp.0.join("policy.json");
        let request_path = temp.0.join("launch-request.json");
        let direct_path = temp.0.join("direct-request.json");
        let ledger_path = temp.0.join("campaign.sqlite");
        let reservation_path = temp.0.join("reservation.json");
        fs::write(&policy_path, serde_json::to_vec(&policy).unwrap()).unwrap();
        fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
        fs::write(&direct_path, serde_json::to_vec(&direct).unwrap()).unwrap();
        let reserved = run_budget_command(
            &parse_budget_test_args(&[
                "test",
                "budget",
                "reserve-launch",
                "--policy",
                policy_path.to_str().unwrap(),
                "--ledger",
                ledger_path.to_str().unwrap(),
                "--request",
                request_path.to_str().unwrap(),
                "--out",
                reservation_path.to_str().unwrap(),
                "--create-ledger",
            ])
            .unwrap(),
        )
        .unwrap();
        let reservation: budget::LaunchReservation = read_json(&reservation_path).unwrap();
        assert_eq!(
            reserved["reservation_sha256"],
            reservation.sha256().unwrap()
        );
        assert_ne!(
            reserved["reservation_sha256"],
            reserved["artifact"]["sha256"]
        );
        assert_eq!(reserved["campaign"]["pending_launch_count"], 1);
        assert_eq!(reserved["cloud_requests"], 0);
        assert_eq!(reserved["worker_grant_issued"], false);
        let direct_out = temp.0.join("must-not-issue-direct.json");
        let reject_direct = || {
            let before = crate::store::hash_file(&ledger_path).unwrap();
            let error = run_budget_command(&BudgetCommand::Reserve {
                policy: policy_path.clone(),
                ledger: ledger_path.clone(),
                request: direct_path.clone(),
                out: direct_out.clone(),
                create_ledger: false,
            })
            .unwrap_err();
            assert!(error.contains("use bind-launch"), "{error}");
            assert!(!direct_out.exists());
            assert_eq!(crate::store::hash_file(&ledger_path).unwrap(), before);
        };
        reject_direct();
        let binding = LaunchBinding {
            schema: budget::LAUNCH_BINDING_SCHEMA.into(),
            launch_id: request.launch_id,
            reservation_sha256: reservation.sha256().unwrap(),
            observed_target: request.target,
            owner: direct.owner,
            observation_evidence_sha256: "e".repeat(64),
        };
        let binding_path = temp.0.join("binding.json");
        let grant_path = temp.0.join("grant.json");
        fs::write(&binding_path, serde_json::to_vec(&binding).unwrap()).unwrap();
        let bind = parse_budget_test_args(&[
            "test",
            "budget",
            "bind-launch",
            "--policy",
            policy_path.to_str().unwrap(),
            "--ledger",
            ledger_path.to_str().unwrap(),
            "--binding",
            binding_path.to_str().unwrap(),
            "--out",
            grant_path.to_str().unwrap(),
        ])
        .unwrap();
        let bound = run_budget_command(&bind).unwrap();
        assert_eq!(
            bound["grant"]["schema"],
            "crypto.ml-dataset.budget-grant/v1"
        );
        assert_eq!(bound["campaign"]["bound_launch_count"], 1);
        assert_eq!(bound["campaign"]["pending_launch_count"], 0);
        assert_eq!(bound["binding"], serde_json::to_value(&binding).unwrap());
        assert_eq!(
            reserved["campaign"]["prior_and_allocated"],
            bound["campaign"]["prior_and_allocated"]
        );
        // Even an exact v1 grant request must retain its launch route and evidence.
        assert_eq!(
            bound["grant"]["request"],
            read_json::<Value>(&direct_path).unwrap()
        );
        reject_direct();
        let before = crate::store::hash_file(&ledger_path).unwrap();
        let again = run_budget_command(&bind).unwrap();
        assert_eq!(again, bound);
        assert_eq!(crate::store::hash_file(&ledger_path).unwrap(), before);
        let written: WorkerGrant = read_json(&grant_path).unwrap();
        assert_eq!(bound["grant_sha256"], written.sha256().unwrap());
        assert_ne!(bound["grant_sha256"], bound["artifact"]["sha256"]);
        assert_eq!(serde_json::to_value(written).unwrap(), bound["grant"]);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn budget_cli_dry_admission_is_read_only_and_reserve_reexports_same_grant() {
        use clap::Parser;
        #[derive(Parser)]
        struct TestCli {
            #[command(flatten)]
            dataset: DatasetArgs,
        }
        let temp = TempRun::new();
        let (policy, request) = budget::test_policy_request(&temp.0);
        let policy_file = temp.0.join("policy.json");
        let request_file = temp.0.join("request.json");
        fs::write(&policy_file, serde_json::to_vec(&policy).unwrap()).unwrap();
        fs::write(&request_file, serde_json::to_vec(&request).unwrap()).unwrap();
        let ledger = temp.0.join("campaign.sqlite");
        let cli = TestCli::try_parse_from([
            "test",
            "budget",
            "admit",
            "--policy",
            policy_file.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "--request",
            request_file.to_str().unwrap(),
            "--dry-run",
        ])
        .unwrap();
        let DatasetCommand::Budget { command } = cli.dataset.command else {
            panic!("wrong CLI route")
        };
        let before = tree_files(&temp.0).unwrap();
        let report = run_budget_command(&command).unwrap();
        assert_eq!(report["dry_run"], true);
        assert_eq!(report["admitted"], true);
        assert!(!ledger.exists());
        assert_eq!(tree_files(&temp.0).unwrap(), before);
        let grant_file = temp.0.join("grant.json");
        let reservation = BudgetCommand::Reserve {
            policy: policy_file.clone(),
            ledger: ledger.clone(),
            request: request_file.clone(),
            out: grant_file.clone(),
            create_ledger: true,
        };
        let first = run_budget_command(&reservation).unwrap();
        let before = crate::store::hash_file(&ledger).unwrap();
        let report = run_budget_command(&command).unwrap();
        assert_eq!(report["admitted"], false);
        assert_eq!(crate::store::hash_file(&ledger).unwrap(), before);
        let second = run_budget_command(&BudgetCommand::Reserve {
            policy: policy_file,
            ledger: ledger.clone(),
            request: request_file,
            out: grant_file,
            create_ledger: false,
        })
        .unwrap();
        assert_eq!(first["grant"], second["grant"]);
        assert_eq!(crate::store::hash_file(&ledger).unwrap(), before);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn budget_cell_resume_reuses_reservation_and_missing_budget_has_no_s3_fallback() {
        let temp = TempRun::new();
        let (context, wire) = budget_test_support::context(&temp.0, None);
        let envelope = Resources {
            output_bytes: 4096,
            put_attempts: 3,
            read_attempts: 20,
            download_bytes: 65536,
            ..Resources::default()
        };
        let first = cell_budget_scope(Some(&context), Some(&envelope), &"c".repeat(64), 0).unwrap();
        let before = context.session().snapshot().unwrap();
        let again = cell_budget_scope(Some(&context), Some(&envelope), &"c".repeat(64), 0).unwrap();
        assert_eq!(first, again);
        assert_eq!(before, context.session().snapshot().unwrap());
        assert_eq!(wire.calls.lock().unwrap().len(), 2);
        let mut cfg = crate::store::Config::discover().unwrap();
        cfg.remote = Some(
            crate::store::backend::RemoteSpec::parse(
                "s3://setec-astronomy-artifacts/ml-dataset/test",
            )
            .unwrap(),
        );
        cfg.index_path = temp.0.join("unbudgeted-index.json");
        assert!(storage::dataset_store(cfg, None, BudgetScope::Work)
            .err()
            .unwrap()
            .contains("active bound budget"));
        assert!(!temp.0.join("unbudgeted-index.json").exists());
        assert_eq!(wire.calls.lock().unwrap().len(), 2);
    }
    #[test]
    fn lab_receipt_routing_rejects_legacy_bucket_before_generation() {
        assert!(
            lab_receipt_remote("s3://setec-astronomy-artifacts/ml-dataset-lab/runs/test/").is_ok()
        );
        for invalid in [
            "s3://isolab-results-104186811093-us-east-2/runs/test/",
            "s3://setec-astronomy-artifacts",
            "file:///tmp/dataset-receipts",
            "https://example.invalid/receipts",
        ] {
            assert!(lab_receipt_remote(invalid).is_err(), "accepted {invalid}");
        }
    }

    fn plan() -> DatasetPlan {
        DatasetPlan {
            schema: "crypto.ml-dataset.plan/v1".into(),
            field_sizes: vec![23, 31],
            arities: vec![2, 15],
            families: vec!["koblitz".into(), "binary".into(), "prime".into()],
            solvers: vec!["enumerate".into(), "sat".into()],
            curve_seeds: 2,
            generators: 2,
            keys: 3,
            seed: 19,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 100,
            twists: true,
            protocol_sha256: None,
            split_rule: "curve-twist-component-sha256-v1".into(),
        }
    }
    #[test]
    fn cross_product_has_no_duplicate_and_includes_twists_and_endpoints() {
        let p = plan();
        p.validate().unwrap();
        let mut cells = BTreeSet::new();
        for i in 0..p.cell_count().unwrap() {
            assert!(cells.insert(serde_json::to_string(&p.cell(i).unwrap()).unwrap()));
        }
        assert_eq!(cells.len(), 576);
        assert!(p.cell(576).is_err());
    }
    #[test]
    fn reject_invalid_and_duplicate_axes() {
        let mut p = plan();
        p.arities.push(2);
        assert!(p.validate().is_err());
        p = plan();
        p.twists = false;
        assert!(p.validate().is_err());
        p = plan();
        p.solvers.push("fiction".into());
        assert!(p.validate().is_err());
    }
    #[test]
    fn ranges_are_bounded() {
        assert_eq!(numbers("2-4,15").unwrap(), vec![2, 3, 4, 15]);
        assert!(numbers("3-2").is_err());
        assert!(numbers("0-4294967295").is_err());
    }
    #[test]
    fn matched_backends_do_not_change_instance_seed() {
        let p = plan();
        let a = p.cell(0).unwrap();
        let mut b = p.cell(1).unwrap();
        b.solver = a.solver.clone();
        assert_eq!(a, b);
    }

    struct TempRun(PathBuf);
    impl TempRun {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-dataset-run-test-{}-{}",
                std::process::id(),
                unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn bound(&self) -> (DatasetPlan, Value, Vec<u8>) {
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"unit correctness fixture","alpha":0.05});
            let frozen_hash = digest(&protocol_canonical_bytes(&protocol).unwrap());
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":frozen_hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let mut p = plan();
            p.field_sizes = vec![23];
            p.arities = vec![2];
            p.families = vec!["prime".into()];
            p.solvers = vec!["bsgs".into()];
            p.curve_seeds = 1;
            p.generators = 1;
            p.keys = 1;
            p.protocol_sha256 = Some(digest(&protocol));
            let binding = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":digest(&serde_json::to_vec(&p).unwrap()),
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),"shard_index":1,"shard_count":2});
            storage::write_json(&self.0, "plan.json", "plan", &p).unwrap();
            storage::write_json(&self.0, "run.json", "run", &binding).unwrap();
            storage::write_bytes(&self.0, "protocol.json", "protocol", "json", &protocol).unwrap();
            (p, binding, protocol)
        }
        fn failed_cell(&self, p: &DatasetPlan, binding: &Value, protocol: &[u8]) -> PathBuf {
            let dir = self.0.join("cells/000000000001");
            fs::create_dir_all(&dir).unwrap();
            let config = p.cell(1).unwrap();
            let mut artifacts = vec![
                storage::write_json(&dir, "plan.json", "plan", p).unwrap(),
                storage::write_json(&dir, "run.json", "run", binding).unwrap(),
                storage::write_bytes(&dir, "protocol.json", "protocol", "json", protocol).unwrap(),
                storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            ];
            for path in ["attempt.json", "worker-process.json"] {
                artifacts.push(storage::write_json(&dir, path, "provenance", &json!({})).unwrap());
            }
            for path in ["stdout.log", "stderr.log"] {
                artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
            }
            storage::write_json(&dir,"manifest.json","manifest",&CellManifest {
                schema:"crypto.ml-dataset.cell/v1".into(),cell_index:1,plan_sha256:binding["plan_sha256"].as_str().unwrap().into(),
                config,outcome:json!({"status":"worker_failed","known_answer_verified":false,"solver_completed":false}),artifacts,
            }).unwrap();
            dir
        }
    }
    impl Drop for TempRun {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn native_plan_binding_ignores_file_format_and_key_order_but_preserves_content() {
        let fixture = TempRun::new();
        let (plan, binding, _) = fixture.bound();
        // Freeze the pre-existing generation contract independently of the
        // helper now shared by Plan, Preflight, generation and reader replay.
        let compact = serde_json::to_vec(&plan).unwrap();
        let expected = digest(&compact);
        assert_eq!(binding["plan_sha256"], expected);
        let pretty = fs::read(fixture.0.join("plan.json")).unwrap();
        let object = serde_json::to_value(&plan).unwrap();
        let reversed_members: Vec<String> = object
            .as_object()
            .unwrap()
            .iter()
            .rev()
            .map(|(key, value)| {
                format!(
                    "{}:{}",
                    serde_json::to_string(key).unwrap(),
                    serde_json::to_string(value).unwrap()
                )
            })
            .collect();
        let reordered = format!("{{{}}}\n", reversed_members.join(",")).into_bytes();
        let mut file_hashes = BTreeSet::new();
        for bytes in [pretty, compact, reordered] {
            let path = fixture.0.join("plan.json");
            fs::write(&path, &bytes).unwrap();
            let parsed: DatasetPlan = read_json(&path).unwrap();
            parsed.validate().unwrap();
            file_hashes.insert(crate::store::hash_file(&path).unwrap().0);
            assert_eq!(parsed.binding_sha256().unwrap(), expected);
            // Existing run records retain the original generation binding;
            // all equivalent file presentations must still verify against it.
            let (_, saved_binding, replayed_hash) = bound_run(&fixture.0).unwrap();
            assert_eq!(saved_binding, binding);
            assert_eq!(replayed_hash, expected);
        }
        assert_eq!(file_hashes.len(), 3);
        let mut changed = plan;
        changed.keys += 1;
        changed.validate().unwrap();
        assert_ne!(changed.binding_sha256().unwrap(), expected);
        fs::write(
            fixture.0.join("plan.json"),
            serde_json::to_vec(&changed).unwrap(),
        )
        .unwrap();
        assert!(bound_run(&fixture.0).is_err());
    }

    #[test]
    fn protocol_requires_exact_frozen_bytes() {
        let f = TempRun::new();
        let (p, _, protocol) = f.bound();
        verify_protocol(&p, &protocol).unwrap();
        let mut changed = protocol.clone();
        changed.push(b'\n');
        assert!(verify_protocol(&p, &changed)
            .unwrap_err()
            .contains("SHA-256"));
        fs::write(f.0.join("protocol.json"), changed).unwrap();
        assert!(bound_run(&f.0).is_err());
    }

    #[test]
    fn resume_rejects_valid_same_plan_cell_from_another_run_binding() {
        let fixture = TempRun::new();
        let (plan, binding, protocol) = fixture.bound();
        let dir = fixture.failed_cell(&plan, &binding, &protocol);
        let config = plan.cell(1).unwrap();
        let hash = binding["plan_sha256"].as_str().unwrap();
        verify_resumed_cell(&dir, &binding, &config, 1, hash).unwrap();
        for (field, changed) in [
            ("source_commit", json!("c".repeat(40))),
            ("binary_sha256", json!("d".repeat(64))),
            ("seconds_per_case", json!(17)),
            ("memory_mib", json!(1024)),
            ("trace_mib", json!(32)),
            ("shard_count", json!(3)),
        ] {
            let mut other = binding.clone();
            other[field] = changed;
            assert!(
                verify_resumed_cell(&dir, &other, &config, 1, hash)
                    .unwrap_err()
                    .contains("run binding"),
                "{field}"
            );
        }
        // The individually valid retained cell is never edited by rejection.
        verify_cell(&dir, true).unwrap();
    }

    #[test]
    fn oversized_json_input_is_rejected_before_parsing_or_allocation() {
        let fixture = TempRun::new();
        let path = fixture.0.join("oversized.json");
        let file = File::create(&path).unwrap();
        file.set_len(JSON_INPUT_MAX_BYTES + 1).unwrap();
        assert!(read_json::<Value>(&path)
            .unwrap_err()
            .contains("exceeds32MiB"));
        assert!(read_json_bytes(&fixture.0)
            .unwrap_err()
            .contains("regular file"));
    }

    #[test]
    fn protocol_rejects_draft_postdata_and_changed_canonical_body() {
        let f = TempRun::new();
        let (mut p, _, bytes) = f.bound();
        let original: Value = serde_json::from_slice(&bytes).unwrap();
        for (field, value, expected) in [
            ("status", json!("draft"), "frozen and filed"),
            (
                "filed_before_affected_data",
                json!(false),
                "frozen and filed",
            ),
            ("protocol_sha256", json!("0".repeat(64)), "canonical body"),
            ("protocol_sha256", json!("bad"), "SHA-256"),
        ] {
            let mut changed = original.clone();
            changed["freeze"][field] = value;
            let data = serde_json::to_vec(&changed).unwrap();
            p.protocol_sha256 = Some(digest(&data));
            assert!(
                verify_protocol(&p, &data).unwrap_err().contains(expected),
                "{field}"
            );
        }
        let mut changed = original;
        changed["alpha"] = json!(0.01);
        let data = serde_json::to_vec(&changed).unwrap();
        p.protocol_sha256 = Some(digest(&data));
        assert!(verify_protocol(&p, &data)
            .unwrap_err()
            .contains("canonical body"));
    }

    #[test]
    fn protocol_canonicalization_matches_official_numeric_and_unicode_rules() {
        let value = protocol_unique_json(
            r#"{"z":[1e-6,0.0001,1e15,1e16,1e20,-0.0,-0,184467440737095516160],"α":"☃","a":0.05}"#,
            0,
        )
        .unwrap();
        let expected = r#"{"a":0.05,"z":[1e-06,0.0001,1000000000000000.0,1e+16,1e+20,-0.0,0,184467440737095516160],"α":"☃"}"#;
        assert_eq!(
            String::from_utf8(protocol_canonical_bytes(&value).unwrap()).unwrap(),
            expected
        );
        assert!(
            protocol_unique_json(r#"{"a":{"duplicate":1,"duplicate":2}}"#, 0)
                .unwrap_err()
                .contains("duplicate")
        );
        let overflow = protocol_unique_json("1e400", 0).unwrap();
        assert!(protocol_canonical_bytes(&overflow).is_err());
    }

    #[test]
    fn publication_names_bind_source_and_runtime_caps() {
        let binding = json!({"plan_sha256":"a".repeat(64),"source_commit":"b".repeat(40),
            "binary_sha256":"c".repeat(64),"seconds_per_case":60,"memory_mib":2048,"trace_mib":64,
            "shard_index":0,"shard_count":1});
        let original = publication_namespace(&binding, 17).unwrap();
        assert_eq!(original, publication_namespace(&binding, 17).unwrap());
        for field in [
            "source_commit",
            "binary_sha256",
            "seconds_per_case",
            "memory_mib",
            "trace_mib",
            "shard_count",
        ] {
            let mut changed = binding.clone();
            changed[field] = json!("changed");
            assert_ne!(
                original,
                publication_namespace(&changed, 17).unwrap(),
                "{field}"
            );
        }
        assert_ne!(original, publication_namespace(&binding, 18).unwrap());
        crate::store::index::validate_name(&format!(
            "{original}/publication/{}.json",
            "d".repeat(64)
        ))
        .unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn root_provenance_symlinks_are_rejected() {
        use std::os::unix::fs::symlink;
        for name in ["plan.json", "run.json", "protocol.json"] {
            let f = TempRun::new();
            f.bound();
            let source = f.0.join(name);
            let retained = f.0.join(format!("retained-{name}"));
            fs::rename(&source, &retained).unwrap();
            symlink(&retained, &source).unwrap();
            assert!(
                bound_run(&f.0)
                    .unwrap_err()
                    .contains("must be a regular file"),
                "{name}"
            );
        }
    }

    #[test]
    fn status_distinguishes_shard_attempts_from_solved_and_remote_durability() {
        let f = TempRun::new();
        let (p, binding, protocol) = f.bound();
        f.failed_cell(&p, &binding, &protocol);
        let status = verify_run(&f.0, true).unwrap();
        assert_eq!(status["assigned_shard_cells"], 1);
        assert_eq!(status["completed_shard_receipts"], 1);
        assert_eq!(status["shard_attempts_complete"], true);
        assert_eq!(status["campaign_attempts_complete"], false);
        assert_eq!(status["full_scope_complete"], false);
        assert_eq!(status["independently_solved_ecdlp_cells"], 0);
        assert_eq!(status["published_completion_receipts"], 0);
    }

    #[test]
    fn cell_verification_rejects_missing_provenance_and_fabricated_solve_labels() {
        let f = TempRun::new();
        let (p, binding, protocol) = f.bound();
        let dir = f.failed_cell(&p, &binding, &protocol);
        let mut m: CellManifest = read_json(&dir.join("manifest.json")).unwrap();
        m.outcome["scalar_recovered_without_witness"] = json!(true);
        fs::write(dir.join("manifest.json"), serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(verify_cell(&dir, false)
            .unwrap_err()
            .contains("recovery certificate"));
        m.outcome["scalar_recovered_without_witness"] = json!(false);
        m.artifacts.retain(|a| a.path != "protocol.json");
        fs::write(dir.join("manifest.json"), serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(verify_cell(&dir, false)
            .unwrap_err()
            .contains("mandatory artifact"));
    }

    #[test]
    fn lease_is_exclusive_and_released_on_drop() {
        let f = TempRun::new();
        let lease = Lease::acquire(&f.0).unwrap();
        assert!(Lease::acquire(&f.0).is_err());
        drop(lease);
        Lease::acquire(&f.0).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn output_and_lock_symlinks_are_rejected() {
        use std::os::unix::fs::symlink;
        let f = TempRun::new();
        fs::create_dir(f.0.join("real")).unwrap();
        symlink(f.0.join("real"), f.0.join("alias")).unwrap();
        assert!(Lease::acquire(&f.0.join("alias")).is_err());
        fs::write(f.0.join("original"), b"unchanged").unwrap();
        symlink(f.0.join("original"), f.0.join(".writer.lock")).unwrap();
        assert!(Lease::acquire(&f.0).is_err());
        assert_eq!(fs::read(f.0.join("original")).unwrap(), b"unchanged");
    }

    #[cfg(unix)]
    #[test]
    fn supervisor_guard_reaps_child_on_error_path() {
        use std::os::unix::process::CommandExt;
        let child = Command::new("sleep")
            .arg("30")
            .process_group(0)
            .spawn()
            .unwrap();
        let pid = child.id();
        let start = Instant::now();
        drop(SupervisedChild {
            child,
            finished: false,
        });
        assert!(start.elapsed() < Duration::from_secs(3));
        let mut status = 0;
        assert_eq!(
            unsafe { libc::waitpid(pid as i32, &mut status, libc::WNOHANG) },
            -1
        );
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn reconciliation_refuses_surviving_external_process_group() {
        use std::os::unix::process::CommandExt;
        let f = TempRun::new();
        let dir = f.0.join("cells/000000000000");
        fs::create_dir_all(&dir).unwrap();
        let child = Command::new("sleep")
            .arg("30")
            .process_group(0)
            .spawn()
            .unwrap();
        let guard = SupervisedChild {
            child,
            finished: false,
        };
        storage::write_json(
            &dir,
            "worker-process.json",
            "identity",
            &json!({"process_group":guard.child.id()}),
        )
        .unwrap();
        assert!(assert_no_workers(&f.0)
            .unwrap_err()
            .contains("process group"));
        drop(guard);
        assert_no_workers(&f.0).unwrap();
    }

    #[test]
    fn reconciliation_retains_partial_bytes_and_does_not_reuse_attempt_names() {
        let f = TempRun::new();
        let dir = f.0.join("cells/000000000000");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("partial.bin"), b"incomplete but valuable trace").unwrap();
        let first = retain_interrupted(&f.0, 0, &json!({"reason":"test stopped worker"})).unwrap();
        assert!(!dir.exists());
        assert_eq!(
            fs::read(first.join("partial.bin")).unwrap(),
            b"incomplete but valuable trace"
        );
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("partial.bin"), b"second attempt").unwrap();
        let second =
            retain_interrupted(&f.0, 0, &json!({"reason":"test second stopped worker"})).unwrap();
        assert_ne!(first, second);
        assert_eq!(
            fs::read(first.join("partial.bin")).unwrap(),
            b"incomplete but valuable trace"
        );
    }
}
