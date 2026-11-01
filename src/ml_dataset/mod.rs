//! Versioned synthetic research datasets with explicit capabilities and durable receipts.
pub mod budget;
pub mod capture;
pub mod curves;
mod execution;
pub mod export;
pub mod factor_bases;
pub mod relations;
pub mod report;
pub mod restore;
pub mod s3_budget;
pub mod solvers;
mod startup_gate;
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
use types::{Artifact, CaseConfig, FactorBasePolicy};

#[derive(Args)]
pub struct DatasetArgs {
    #[command(subcommand)]
    pub command: DatasetCommand,
}

#[derive(Subcommand)]
pub enum DatasetCommand {
    /// Validate a delivered launch grant and start the frozen bounded worker.
    Start {
        #[arg(long)]
        resume: bool,
    },
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
    /// JSON array of explicit factor-base recipes; selects plan/v2. Omit for exact legacy plan/v1.
    #[arg(long)]
    pub factor_base_policies: Option<PathBuf>,
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
    /// Empty and absent for frozen plan/v1. Explicit nonempty policy axis for plan/v2.
    #[serde(
        default,
        deserialize_with = "nonempty_policy_axis",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub factor_base_policies: Vec<FactorBasePolicy>,
}

fn nonempty_policy_axis<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<FactorBasePolicy>, D::Error> {
    let policies = Vec::<FactorBasePolicy>::deserialize(deserializer)?;
    if policies.is_empty() || policies.len() > 32 {
        return Err(serde::de::Error::custom(
            "a present policy axis must contain 1..32 recipes",
        ));
    }
    Ok(policies)
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
        let policy_plan = match self.schema.as_str() {
            "crypto.ml-dataset.plan/v1" if self.factor_base_policies.is_empty() => false,
            "crypto.ml-dataset.plan/v2" if !self.factor_base_policies.is_empty() => true,
            _ => {
                return Err(
                    "expected plan/v1 without policies or plan/v2 with explicit policies".into(),
                )
            }
        };
        if !self.twists {
            return Err("expected paired twists".into());
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
                    "f4-wide-dynamic-v1",
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
        if policy_plan {
            if self.factor_base_policies.len() > 32 {
                return Err("at most 32 factor-base policies are supported per plan".into());
            }
            let mut recipes = BTreeSet::new();
            for policy in &self.factor_base_policies {
                for &n in &self.field_sizes {
                    policy.validate(n)?;
                }
                if !recipes.insert(serde_json::to_vec(policy).map_err(|e| e.to_string())?) {
                    return Err("duplicate factor-base policy recipe".into());
                }
            }
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
            self.factor_base_policies.len().max(1) as u64,
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
        let factor_base_policy = if self.factor_base_policies.is_empty() {
            None
        } else {
            Some(
                self.factor_base_policies[digit(self.factor_base_policies.len() as u64) as usize]
                    .clone(),
            )
        };
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
            factor_base_policy,
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
        DatasetCommand::Start { resume } => {
            let outcome = startup_gate::start(*resume).map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string(&outcome).map_err(|e| e.to_string())?
            );
        }
        DatasetCommand::Budget { command } => println!("{}", run_budget_command(command)?),
        DatasetCommand::Plan(a) => {
            let factor_base_policies: Vec<FactorBasePolicy> = a
                .factor_base_policies
                .as_ref()
                .map(|path| read_json(path))
                .transpose()?
                .unwrap_or_default();
            if a.factor_base_policies.is_some() && factor_base_policies.is_empty() {
                return Err("explicit factor-base policy array must not be empty".into());
            }
            let plan = DatasetPlan {
                schema: if factor_base_policies.is_empty() {
                    "crypto.ml-dataset.plan/v1"
                } else {
                    "crypto.ml-dataset.plan/v2"
                }
                .into(),
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
                factor_base_policies,
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

const CAPTURE_CONTEXT: [(&str, &str); 2] = [
    ("capture-binding.json", "partial-capture-binding"),
    ("capture-limits.json", "partial-capture-limits"),
];

fn admit_capture_context(
    dir: &Path,
) -> Result<Option<(capture::AttemptBinding, capture::CaptureLimits)>, String> {
    let present = CAPTURE_CONTEXT
        .iter()
        .any(|(path, _)| fs::symlink_metadata(dir.join(path)).is_ok());
    if !present {
        return Ok(None);
    }
    if !telemetry::checked::policy_present(dir)? {
        return Err("bound capture requires the frozen checked measurement policy".into());
    }
    let binding: capture::AttemptBinding =
        read_json(&storage::contained_path(dir, CAPTURE_CONTEXT[0].0, false)?)?;
    let limits: capture::CaptureLimits =
        read_json(&storage::contained_path(dir, CAPTURE_CONTEXT[1].0, false)?)?;
    if binding != capture::AttemptBinding::from_cell_files(dir)? {
        return Err("capture binding differs from original attempt".into());
    }
    let run: Value = read_json(&dir.join("run.json"))?;
    let trace_mib = run["trace_mib"]
        .as_u64()
        .ok_or("capture run lacks frozen trace_mib")?;
    if limits != capture::capture_limits(trace_mib)? {
        return Err("capture limits differ from frozen run allowance".into());
    }
    Ok(Some((binding, limits)))
}

fn verify_capture_context(dir: &Path, artifacts: &[Artifact]) -> Result<(), String> {
    let admitted = admit_capture_context(dir)?.is_some();
    for (path, role) in CAPTURE_CONTEXT {
        let matching: Vec<_> = artifacts
            .iter()
            .filter(|a| a.path == path || a.role == role)
            .collect();
        if admitted {
            if matching.len() != 1
                || matching[0].path != path
                || matching[0].role != role
                || matching[0].encoding != "json"
            {
                return Err("capture context manifest identity/role/encoding mismatch".into());
            }
        } else if !matching.is_empty() {
            return Err("capture context manifest has no admitted original files".into());
        }
    }
    Ok(())
}

fn worker_reserved_artifact_path(path: &str) -> bool {
    [
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
        "measurement-policy.json",
        "capture-binding.json",
        "capture-limits.json",
        execution::PATH,
    ]
    .contains(&path)
        || path.starts_with("publication/")
        || path.starts_with("telemetry/")
}

/// Adopt validated stored roots before pending validation; retain them when a
/// later pending write fails. No payload/root/capture write occurs for stored.
fn adopt_worker_solver_artifacts(
    out: &Path,
    registry: &mut Vec<Artifact>,
    data: types::SolverData,
    stored: Vec<Artifact>,
) -> Result<Value, String> {
    if stored.len() > solvers::WORKER_STORED_RECEIPT_MAX {
        return Err("stored worker receipt count exceeds admission".into());
    }
    // Bounded stored channel uses capacity reserved before solver execution.
    if registry.capacity() - registry.len() < stored.len() {
        return Err("worker receipt registry capacity was not reserved".into());
    }
    // Authenticate the stored channel first. Pending validation must not erase
    // successful root writes that have already happened.
    for (i, artifact) in stored.iter().enumerate() {
        if worker_reserved_artifact_path(&artifact.path)
            || registry.iter().any(|a| a.path == artifact.path)
            || stored[..i].iter().any(|a| a.path == artifact.path)
        {
            return Err("stored solver artifact collides with reserved output".into());
        }
    }
    registry.extend(stored);
    // Check the entire pending list before any new write, against the adopted
    // roots as well as worker outputs and earlier pending identities.
    for (i, artifact) in data.artifacts.iter().enumerate() {
        if worker_reserved_artifact_path(&artifact.path)
            || registry.iter().any(|a| a.path == artifact.path)
            || data.artifacts[..i].iter().any(|a| a.path == artifact.path)
        {
            return Err("pending solver artifact collides with reserved output".into());
        }
    }
    for artifact in data.artifacts {
        registry.push(storage::write_bytes(
            out,
            &artifact.path,
            &artifact.role,
            &artifact.encoding,
            &artifact.bytes,
        )?);
    }
    Ok(data.outcome)
}

/// Preserve native retained evidence when post-operation telemetry fails.
/// Generic legacy failures retain their existing error behavior.
fn finish_worker_solver_artifacts(
    out: &Path,
    registry: &mut Vec<Artifact>,
    mut data: types::SolverData,
    stored: Vec<Artifact>,
    measurement: Result<(), String>,
) -> Result<Value, String> {
    if let Err(error) = measurement {
        let native = ["koblitz-m3-native-v1", "binary-m3-native-v1"]
            .into_iter()
            .any(|id| data.outcome["adapter_id"] == id && data.outcome["strategy"] == id);
        // No new pending writes after a telemetry failure. Already acknowledged
        // roots remain in the registry before handling the censored outcome.
        data.artifacts.clear();
        let mut outcome = adopt_worker_solver_artifacts(out, registry, data, stored)?;
        if !native {
            return Err(error);
        }
        outcome["status"] = json!("worker_failed");
        for key in [
            "solver_completed",
            "pdp_witness_verified",
            "certificate_durably_persisted",
            "group_domain_complete",
            "exhaustive_no_decomposition",
            "native_backend_invoked",
            "algebra_completed",
            "native_statistics_complete",
            "scalar_recovered_without_witness",
        ] {
            outcome[key] = json!(false);
        }
        // Keep retained_pdp_witness_verified and original ledger/custody fields.
        // Preserve prior persistence errors rather than replacing them.
        outcome["worker_telemetry_error"] = json!(error);
        return Ok(outcome);
    }
    adopt_worker_solver_artifacts(out, registry, data, stored)
}

fn worker(config: &CaseConfig, out: &Path) -> Result<(), String> {
    let out = checked_directory(out, false)?;
    let mut registry = Vec::<Artifact>::new();
    let capture_context = admit_capture_context(&out)?;
    let execution_context = execution::admit(&out)?;
    if telemetry::checked::policy_present(&out)? {
        capture::AttemptBinding::from_cell_files(&out)?;
        if read_json::<CaseConfig>(&out.join("config.json"))? != *config {
            return Err("checked worker configuration differs from attempt binding".into());
        }
        registry.push(telemetry::checked::install_policy(&out)?);
    }
    let mut known_answer = false;
    let mut factor_base_verified = false;
    // Keep residual owner bytes and actual rejected receipts alive through
    // result/registry finalization, including telemetry failure. No retry.
    let mut retained_solver_custody = solvers::WorkerResidualCustody::default();
    let result = (|| {
        let case = telemetry::measured(&out, &mut registry, "curve-construction", || {
            curves::generate(config)
        })?;
        let verified_originals = solvers::verify_and_store_worker_originals(
            config,
            &case,
            &out,
            capture_context.as_ref().map(|(binding, _)| binding),
            &mut registry,
        )?;
        known_answer = true;
        factor_base_verified = true;
        // Worker setup reservation, before producer execution. Adapter receipt
        // admission separately accounts its transport Vec in the original budget.
        registry
            .try_reserve_exact(solvers::WORKER_STORED_RECEIPT_MAX + 2)
            .map_err(|e| e.to_string())?;
        let mut produced = None;
        let measurement = telemetry::measured(&out, &mut registry, "solver", || {
            let output = if let Some((binding, limits)) = &capture_context {
                solvers::solve_with_verified_worker_originals(
                    config,
                    &case,
                    &out.join("live"),
                    binding.clone(),
                    limits.clone(),
                    execution_context.as_ref(),
                    &verified_originals,
                )
            } else {
                solvers::solve_with_event_sink_context(
                    config,
                    &case,
                    &out.join("live"),
                    execution_context.as_ref(),
                )
                .map(solvers::WorkerSolveOutput::legacy)
            }?;
            // Keep custody outside measured: its terminal telemetry write may fail.
            produced = Some(output);
            Ok(())
        });
        let solved = match produced {
            Some(solved) => solved,
            None => {
                return Err(measurement
                    .err()
                    .unwrap_or_else(|| "solver returned no output".into()))
            }
        };
        let (data, stored, residual) = solved.into_parts();
        retained_solver_custody = residual;
        finish_worker_solver_artifacts(&out, &mut registry, data, stored, measurement)
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
    // Owner destruction is after finalization; failed raw storage is recorded
    // as censored in the outcome, never silently retried or called durable.
    drop(retained_solver_custody);
    Ok(())
}

fn collect_cell_artifacts(dir: &Path) -> Result<Vec<Artifact>, String> {
    let registry: Vec<Artifact> = if dir.join("worker-artifacts.json").exists() {
        read_json(&dir.join("worker-artifacts.json"))?
    } else {
        Vec::new()
    };
    storage::verify_manifest(dir, &registry)?;
    let registered: BTreeMap<_, _> = registry.iter().map(|a| (a.path.as_str(), a)).collect();
    // A killed worker may never write its registry. Promote only an exact,
    // fully validated finite phase prefix; malformed bytes remain partial.
    let recovered_phase_roles = if dir.join(telemetry::checked::POLICY_PATH).exists() {
        telemetry::checked::validate_directory(dir).unwrap_or_default()
    } else {
        BTreeMap::new()
    };
    let mut artifacts = Vec::new();
    for path in tree_files(dir)? {
        let relative = path
            .strip_prefix(dir)
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
        let (role, encoding) = if let Some(role) = recovered_phase_roles.get(&relative) {
            (*role, "json")
        } else {
            match relative.as_str() {
                "witness.json" => ("restricted-known-answer", "json"),
                "config.json" => ("restricted-provenance", "json"),
                "plan.json" => ("restricted-frozen-plan", "json"),
                "run.json" => ("restricted-run-binding", "json"),
                "protocol.json" => ("restricted-frozen-protocol", "json"),
                "attempt.json" | "worker-process.json" => ("process-provenance", "json"),
                "host.json" => ("host-measurement-context", "json"),
                "worker-artifacts.json" => ("artifact-registry", "json"),
                "capture-binding.json" => ("partial-capture-binding", "json"),
                "capture-limits.json" => ("partial-capture-limits", "json"),
                execution::PATH => (execution::ROLE, "json"),
                "stdout.log" | "stderr.log" => ("process-log", "utf8"),
                _ => ("unregistered-partial-artifact", "native"),
            }
        };
        artifacts.push(Artifact {
            role: role.into(),
            path: relative,
            sha256,
            bytes,
            encoding: encoding.into(),
        });
    }
    sync_dir(dir)?;
    Ok(artifacts)
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
        "trace_mib":a.trace_mib,"execution_context_required":cfg!(target_os = "linux"),"output_budget_scope":"all_cell_files_before_manifest_seal",
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
            telemetry::checked::install_policy(&dir)?;
            let capture_binding = capture::AttemptBinding::from_cell_files(&dir)?;
            storage::write_json(
                &dir,
                "capture-binding.json",
                "partial-capture-binding",
                &capture_binding,
            )?;
            storage::write_json(
                &dir,
                "capture-limits.json",
                "partial-capture-limits",
                &capture::capture_limits(a.trace_mib)?,
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
            let launch_context = if cfg!(target_os = "linux") {
                Some(execution::create(&dir)?)
            } else {
                None
            };
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
                let polled = child.poll()?;
                // A successful exit observed at/after the original deadline is
                // conservatively censored, even if it occurred between polls.
                let expired = if let Some(launch) = &launch_context {
                    execution::deadline_expired(
                        launch.deadline_monotonic_ns,
                        execution::monotonic_ns()?,
                    )
                } else {
                    start.elapsed() >= Duration::from_secs(a.seconds_per_case)
                };
                if expired {
                    stop_reason = Some("timeout");
                }
                if let Some(status) = polled {
                    break status;
                }
                let bytes = tree_bytes_while_running(&dir)?;
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
            outcome =
                execution::terminal_outcome(outcome, stop_reason, exit.success(), exit.code());
            outcome["supervisor"] = json!({"exit_code":exit.code(),"elapsed_ms_diagnostic":start.elapsed().as_secs_f64()*1000.0,
            "clock_valid_for_comparison":false,"retired_instructions":null,
            "instruction_counter_status":"whole_worker_not_counted; see phase telemetry for available hardware counts","memory_limit_bytes":memory,
            "worker_output_bytes_before_seal":final_bytes,"output_budget_bytes":a.trace_mib.saturating_mul(1024*1024),
            "output_budget_scope":"all_cell_files_before_manifest_seal",
            "checkpoint_mid_call":false,"interrupted_trace_loss_possible":true});
            let artifacts = collect_cell_artifacts(&dir)?;
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
fn process_entry<T>(pid: u32, result: std::io::Result<T>) -> Result<Option<T>, String> {
    match result {
        Ok(value) => Ok(Some(value)),
        // Linux proc_pid_cmdline_read returns ESRCH if the task exits after
        // directory enumeration. It is as absent as an unlinked proc entry.
        Err(error)
            if error.kind() == std::io::ErrorKind::NotFound
                || error.raw_os_error() == Some(libc::ESRCH) =>
        {
            Ok(None)
        }
        Err(error) => Err(format!("cannot inspect process {pid}: {error}")),
    }
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
    'processes: for entry in
        fs::read_dir("/proc").map_err(|e| format!("cannot inspect live workers: {e}"))?
    {
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
        let Some(metadata) = process_entry(pid, entry.metadata())? else {
            continue;
        };
        if metadata.uid() != unsafe { libc::geteuid() } {
            continue;
        }
        let Some(command) = process_entry(pid, fs::read(entry.path().join("cmdline")))? else {
            continue;
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
                let Some(cwd) = process_entry(pid, fs::read_link(entry.path().join("cwd")))? else {
                    continue 'processes;
                };
                cwd.join(path)
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

/// A replayed root-set certificate does not certify the native operation stream.
/// Keep the exported outcome labels no stronger than the certificate's terminal
/// state. This also runs during hash-only verification as a consistency check;
/// mathematical replay remains the separate `replay` gate below.
fn verify_prime_root_outcome(outcome: &Value, certificate: &Value) -> Result<(), String> {
    verify_root_outcome(outcome, certificate, solvers::PRIME_ROOT_CERTIFICATE_SCOPE)?;
    let queries = certificate["queries"]
        .as_array()
        .ok_or("prime-root certificate lacks query records")?;
    let work = certificate["charged_work"]
        .as_u64()
        .ok_or("prime-root certificate lacks aggregate work")?;
    let (events, bytes) = solvers::prime_root_trace_totals(certificate)?;
    for (key, expected) in [
        ("aggregate_root_field_and_group_work", work),
        ("root_solver_calls", queries.len() as u64),
        ("acknowledged_native_events", events),
        ("acknowledged_native_event_bytes", bytes),
    ] {
        if outcome["counters"][key].as_u64() != Some(expected) {
            return Err(format!(
                "prime-root outcome aggregate counter differs: {key}"
            ));
        }
    }
    Ok(())
}

fn verify_binary_root_outcome(outcome: &Value, certificate: &Value) -> Result<(), String> {
    verify_root_outcome(outcome, certificate, solvers::BINARY_ROOT_CERTIFICATE_SCOPE)?;
    let (events, bytes) = solvers::binary_root_trace_totals(certificate)?;
    if certificate["schema"] != "ml-dataset.binary-root-certificates/v1"
        || certificate["distinct_root_certificates_verified"] != true
        || outcome["counters"]["aggregate_root_primitive_and_group_work"].as_u64()
            != certificate["charged_work"].as_u64()
        || certificate["charged_work"].as_u64().is_none()
        || outcome["counters"]["root_solver_calls"].as_u64()
            != certificate["queries"].as_array().map(|q| q.len() as u64)
        || certificate["queries"].as_array().is_none()
    {
        return Err("binary-root outcome aggregate work/call labels differ".into());
    }
    for (key, expected) in [
        ("acknowledged_native_events", events),
        ("acknowledged_native_event_bytes", bytes),
    ] {
        if outcome["counters"][key].as_u64() != Some(expected) {
            return Err("binary-root outcome aggregate event labels differ".into());
        }
    }
    Ok(())
}

fn is_root_auxiliary(artifact: &Artifact) -> bool {
    matches!(
        artifact.role.as_str(),
        "native-prime-root-call"
            | "native-prime-root-intermediate-state"
            | "native-binary-root-call"
            | "native-binary-root-intermediate-state"
    ) || artifact.path.starts_with("root-call-")
        || artifact.path.starts_with("root-state-")
        || artifact.path.starts_with("binary-root-call-")
        || artifact.path.starts_with("binary-root-state-")
}

/// Bind the duplicated call/state files to the certificate's supplied events.
/// This does not assert that the native execution emitted every possible event.
fn verify_root_auxiliary_artifacts(
    dir: &Path,
    artifacts: &[Artifact],
    certificate: &Value,
    binary: bool,
    replay: bool,
) -> Result<(), String> {
    let family = if binary { "binary" } else { "prime" };
    let file_prefix = if binary { "binary-root" } else { "root" };
    let call_role = format!("native-{family}-root-call");
    let state_role = format!("native-{family}-root-intermediate-state");
    let mut supplied = BTreeMap::new();
    for artifact in artifacts.iter().filter(|a| is_root_auxiliary(a)) {
        if supplied.insert(artifact.path.as_str(), artifact).is_some() {
            return Err(format!("duplicate {family} root auxiliary path"));
        }
    }
    let mut check = |name: String, role: &str, expected: &Value| -> Result<(), String> {
        let artifact = supplied
            .remove(name.as_str())
            .ok_or_else(|| format!("missing {family} root auxiliary artifact: {name}"))?;
        if artifact.role != role || artifact.encoding != "json" {
            return Err(format!(
                "{family} root auxiliary role or encoding differs: {name}"
            ));
        }
        if replay && read_json::<Value>(&dir.join(&artifact.path))? != *expected {
            return Err(format!("{family} root auxiliary payload differs: {name}"));
        }
        Ok(())
    };
    let mut event_sequence = 0u64;
    for (index, query) in certificate["queries"]
        .as_array()
        .ok_or("missing root queries")?
        .iter()
        .enumerate()
    {
        check(
            format!("{file_prefix}-call-{index:04}.json"),
            &call_role,
            query,
        )?;
        let fixed_x = query["fixed_x"]
            .as_str()
            .ok_or("root query lacks fixed x")?;
        for event in query["report"]["events"]
            .as_array()
            .ok_or("root report lacks acknowledged events")?
        {
            event_sequence = event_sequence
                .checked_add(1)
                .ok_or("root event sequence overflow")?;
            let selected = if binary {
                matches!(
                    event["kind"].as_str(),
                    Some("normalized" | "trace_end" | "half_trace_end" | "matrix_end")
                )
            } else {
                matches!(event["kind"].as_str(), Some("power_end" | "gcd_end"))
            };
            if selected {
                let expected = json!({"schema":format!("ml-dataset.{family}-root-event/v1"), "call_index":index,
                    "fixed_x":fixed_x, "native":event});
                check(
                    format!("{file_prefix}-state-{index:04}-{event_sequence:06}.json"),
                    &state_role,
                    &expected,
                )?;
            }
        }
    }
    if !supplied.is_empty() {
        return Err(format!(
            "unexpected or duplicate {family} root auxiliary artifact"
        ));
    }
    Ok(())
}

fn verify_root_outcome(outcome: &Value, certificate: &Value, scope: &str) -> Result<(), String> {
    let found = certificate["found_pdp"]
        .as_bool()
        .ok_or("prime-root certificate lacks a boolean PDP result")?;
    let censored = certificate["censored"]
        .as_bool()
        .ok_or("prime-root certificate lacks a boolean censoring result")?;
    let all_queries = certificate["root_queries_completed"]
        .as_bool()
        .ok_or("prime-root certificate lacks a boolean query completion result")?;
    if (found && censored) || (censored && all_queries) || (!found && !censored && !all_queries) {
        return Err("inconsistent prime-root aggregate termination".into());
    }
    let expected_status = if found {
        "verified"
    } else if censored {
        "budget_exhausted"
    } else {
        "partial"
    };
    if outcome["root_execution_trace_complete_verified"] != false
        || outcome["root_total_operation_count_verified"] != false
        || outcome["trace_complete"] != false
        || certificate["execution_trace_complete_verified"] != false
        || certificate["total_operation_count_verified"] != false
        || outcome["scalar_recovered_without_witness"] != false
        || outcome["scalar_replay_verified"] != false
        || outcome["ecdlp_pipeline_completed"] != false
        || outcome["pdp_witness_verified"].as_bool() != Some(found)
        || outcome["solver_completed"].as_bool() != Some(found || !censored)
        || outcome["status"] != expected_status
        || outcome["root_certificate_scope"] != scope
    {
        return Err(
            "prime-root outcome labels differ from verified certificate scope or termination"
                .into(),
        );
    }
    Ok(())
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
    telemetry::checked::verify_artifacts(dir, &manifest.artifacts)?;
    verify_capture_context(dir, &manifest.artifacts)?;
    execution::verify(dir, &manifest.artifacts)?;
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
    // Bind the original cell before admitting any specialized replay claim.
    let dynamic_replay = solvers::verify_dynamic_cell(
        dir,
        &manifest.config,
        &manifest.outcome,
        &manifest.artifacts,
    )?;
    // Reserved Koblitz artifacts must be checked even if strategy/labels were
    // stripped, and even when optional known-answer replay was not requested.
    solvers::verify_koblitz_m3_cell(
        dir,
        &manifest.config,
        &manifest.outcome,
        &manifest.artifacts,
    )?;
    // Reserved ordinary paths/roles cannot evade semantic replay by removing
    // strategy labels or disabling optional known-answer replay.
    solvers::verify_ordinary_m3_cell(
        dir,
        &manifest.config,
        &manifest.outcome,
        &manifest.artifacts,
    )?;
    let wide_prime_replay = solvers::verify_prime_f4_wide(
        dir,
        &manifest.config,
        &manifest.outcome,
        &manifest.artifacts,
    )?;
    let has = |path: &str| manifest.artifacts.iter().any(|a| a.path == path);
    let complete_known_answer = has("public.json") && has("witness.json");
    let complete_factor_base = has("public.json") && has("factor-base.json");
    // Reserved native artifacts cannot evade replay by losing their role and
    // outcome fields together. Legacy root-polynomials.json is not a native
    // root-set certificate and remains compatible with its historical scope.
    if manifest.artifacts.iter().any(|a| {
        a.path == "prime-root-certificates.json"
            && (a.role != "prime-root-certificates/v1" || a.encoding != "json")
    }) {
        return Err("prime-root certificate lacks its reserved JSON role".into());
    }
    if manifest.artifacts.iter().any(|a| {
        a.path == "binary-root-certificates.json"
            && (a.role != "binary-root-certificates/v1" || a.encoding != "json")
    }) {
        return Err("binary-root certificate lacks its reserved JSON role".into());
    }
    let native_root_artifacts = manifest.artifacts.iter().any(|a| {
        matches!(
            a.path.as_str(),
            "prime-root-certificates.json" | "binary-root-certificates.json"
        ) || is_root_auxiliary(a)
    });
    let root_certificates: Vec<_> = manifest
        .artifacts
        .iter()
        .filter(|a| {
            matches!(
                a.role.as_str(),
                "prime-root-certificates/v1" | "binary-root-certificates/v1"
            )
        })
        .collect();
    if (native_root_artifacts
        || [
            "root_certificate_scope",
            "root_execution_trace_complete_verified",
            "root_total_operation_count_verified",
        ]
        .iter()
        .any(|name| manifest.outcome.get(*name).is_some()))
        && root_certificates.is_empty()
    {
        return Err("root outcome lacks its distinct-root certificate artifact".into());
    }
    let binary_root = matches!(manifest.config.family.as_str(), "binary" | "koblitz");
    let expected_root_role = if binary_root {
        "binary-root-certificates/v1"
    } else {
        "prime-root-certificates/v1"
    };
    if !root_certificates.is_empty()
        && (!complete_factor_base
            || manifest.config.solver != "root"
            || !matches!(
                manifest.config.family.as_str(),
                "prime" | "binary" | "koblitz"
            )
            || manifest.config.m != 2
            || root_certificates.len() != 1
            || root_certificates[0].role != expected_root_role
            || root_certificates[0].encoding != "json")
    {
        let kind = if binary_root {
            "binary-root"
        } else {
            "prime-root"
        };
        return Err(format!(
            "{kind} certificate lacks its unique public family/root/m2 domain"
        ));
    }
    let root_certificate: Option<Value> = root_certificates
        .first()
        .map(|artifact| read_json(&dir.join(&artifact.path)))
        .transpose()?;
    if let Some(certificate) = &root_certificate {
        if binary_root {
            verify_binary_root_outcome(&manifest.outcome, certificate)?;
        } else {
            verify_prime_root_outcome(&manifest.outcome, certificate)?;
        }
        verify_root_auxiliary_artifacts(dir, &manifest.artifacts, certificate, binary_root, false)?;
    }
    let censored_m3_prefix = wide_prime_replay == solvers::WidePrimeReplay::CensoredM3Prefix;
    let prime_chain_declared = !censored_m3_prefix
        && manifest.outcome["strategy"] == "prime-affine-s3-chain-f4-visitor/v1";
    let prime_chain_input_expected = !censored_m3_prefix
        && manifest.config.family == "prime"
        && manifest.config.solver == "f4"
        && (3..=15).contains(&manifest.config.m)
        && (has("prime-f4-input.json")
            || has("prime-f4-chain-result.json")
            || has("prime-f4-chain-events.jsonl")
            || manifest.outcome["native_backend_invoked"] == true
            || manifest.outcome["pdp_witness_verified"] == true
            || manifest.outcome["solver_completed"] == true);
    if prime_chain_declared || prime_chain_input_expected {
        if !has("prime-f4-input.json") {
            return Err("prime affine-chain strategy lacks its equation/domain artifact".into());
        }
        if !complete_factor_base {
            return Err("prime affine-chain strategy lacks public instance or factor base".into());
        }
        if manifest.config.family != "prime"
            || manifest.config.solver != "f4"
            || !(3..=15).contains(&manifest.config.m)
        {
            return Err(
                "prime affine-chain strategy differs from frozen cell configuration".into(),
            );
        }
    }
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
    if replay
        && complete_known_answer
        && (dynamic_replay != solvers::DynamicReplayDisposition::RawPrefix
            || manifest.outcome["known_answer_verified"] == true)
    {
        curves::verify(
            &read_json(&dir.join("public.json"))?,
            &read_json(&dir.join("witness.json"))?,
        )?;
    }
    if replay
        && complete_factor_base
        && (dynamic_replay != solvers::DynamicReplayDisposition::RawPrefix
            || manifest.outcome["factor_base_verified"] == true)
    {
        let public = read_json(&dir.join("public.json"))?;
        let base = read_json(&dir.join("factor-base.json"))?;
        curves::verify_factor_base_config(&manifest.config, &public, &base)?;
        if has("prime-f4-input.json")
            && !censored_m3_prefix
            && dynamic_replay == solvers::DynamicReplayDisposition::Ordinary
        {
            let input: Value = read_json(&dir.join("prime-f4-input.json"))?;
            // The m=2 reference uses the same filename with a legacy schema.
            // A declared chain may not evade reconstruction by changing schema.
            if prime_chain_declared
                || prime_chain_input_expected
                || input["schema"] == "ml-dataset.prime-f4-chain-input/v1"
                || input["strategy"] == "prime-affine-s3-chain-f4-visitor/v1"
            {
                if manifest.config.family != "prime"
                    || manifest.config.solver != "f4"
                    || !(3..=15).contains(&manifest.config.m)
                    || public["family"].as_str() != Some(manifest.config.family.as_str())
                    || public["m"].as_u64() != Some(manifest.config.m as u64)
                    || public["n"].as_u64() != Some(u64::from(manifest.config.n))
                    || public["twist"].as_bool() != Some(manifest.config.twist)
                {
                    return Err(
                        "prime affine-chain input differs from frozen cell configuration".into(),
                    );
                }
                solvers::verify_prime_f4_chain_input(&public, &base, &input)?;
            }
        }
        for artifact in &manifest.artifacts {
            if artifact.role == "prime-root-certificates/v1" {
                solvers::verify_prime_root_artifact(
                    &public,
                    &base,
                    &manifest.config,
                    root_certificate
                        .as_ref()
                        .ok_or("missing parsed root certificate")?,
                )?;
                verify_root_auxiliary_artifacts(
                    dir,
                    &manifest.artifacts,
                    root_certificate
                        .as_ref()
                        .ok_or("missing parsed root certificate")?,
                    false,
                    true,
                )?;
            }
            if artifact.role == "binary-root-certificates/v1" {
                let certificate = root_certificate
                    .as_ref()
                    .ok_or("missing parsed root certificate")?;
                solvers::verify_binary_root_artifact(
                    &public,
                    &base,
                    &manifest.config,
                    certificate,
                )?;
                verify_root_auxiliary_artifacts(dir, &manifest.artifacts, certificate, true, true)?;
            }
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
            factor_base_policies: Vec::new(),
        }
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn vanished_procfs_entries_do_not_block_worker_inspection() {
        for errno in [libc::ENOENT, libc::ESRCH] {
            let entry = process_entry::<Vec<u8>>(42, Err(std::io::Error::from_raw_os_error(errno)))
                .unwrap();
            assert!(entry.is_none(), "a vanished process is not a live owner");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn procfs_permission_and_io_errors_still_block_worker_inspection() {
        for errno in [libc::EPERM, libc::EACCES, libc::EIO, libc::ENOTDIR] {
            assert!(
                process_entry::<Vec<u8>>(42, Err(std::io::Error::from_raw_os_error(errno)))
                    .is_err(),
                "an uninspectable process cannot be assumed absent: errno {errno}"
            );
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn successful_procfs_inspection_preserves_the_worker_command() {
        let command = b"ic\0dataset\0worker\0--out\0/tmp/unit-output\0".to_vec();
        assert_eq!(
            process_entry(42, Ok(command.clone())).unwrap(),
            Some(command)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn process_exit_between_procfs_open_and_read_is_not_a_live_worker() {
        use std::io::Read;
        let mut child = std::process::Command::new("sleep")
            .arg("30")
            .spawn()
            .unwrap();
        let pid = child.id();
        let path = format!("/proc/{pid}/cmdline");
        let opened = std::fs::File::open(&path);
        // Always terminate and reap the owned fixture before any assertion.
        let killed = child.kill();
        let waited = child.wait();
        killed.unwrap();
        waited.unwrap();
        let missing = std::fs::read(&path);
        assert_eq!(
            missing.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );
        assert!(process_entry(pid, missing).unwrap().is_none());
        let mut opened = opened.unwrap();
        let read = opened.read_to_end(&mut Vec::new());
        assert_eq!(read.as_ref().unwrap_err().raw_os_error(), Some(libc::ESRCH));
        assert!(process_entry(pid, read).unwrap().is_none());
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

    #[test]
    fn frozen_720_cell_pilot_keeps_bytes_and_legacy_mapping() {
        let bytes = br###"{
  "arities": [
    2,
    4,
    15
  ],
  "curve_seeds": 1,
  "degree_bound": 3,
  "factor_base_size": 4,
  "families": [
    "koblitz",
    "binary",
    "prime"
  ],
  "field_sizes": [
    23
  ],
  "generators": 2,
  "keys": 2,
  "protocol_sha256": "05a54dacc052b49bf7b9d6ffd776c461f49757e01488088c9a79bbf136f11604",
  "schema": "crypto.ml-dataset.plan/v1",
  "seed": 20261027,
  "solvers": [
    "bsgs",
    "enumerate",
    "root",
    "sat",
    "f4",
    "f5",
    "f6",
    "weil",
    "pkm",
    "double-large-prime"
  ],
  "split_rule": "curve-twist-component-sha256-v1",
  "twists": true,
  "work_limit": 20000
}
"###;
        assert_eq!(
            digest(bytes),
            "d698efdad6d674e944ebf3741962eb71b38829bdaf882fb8a8407a7fc57f753a"
        );
        let plan: DatasetPlan = serde_json::from_slice(bytes).unwrap();
        plan.validate().unwrap();
        assert_eq!(plan.cell_count().unwrap(), 720);
        assert_eq!(
            digest(&serde_json::to_vec(&plan).unwrap()),
            "c905e878e3dcb29e407e743874d18b6b7091c097984f829820f26673ccd47f21"
        );
        let mut with_empty_axis = serde_json::to_value(&plan).unwrap();
        with_empty_axis["factor_base_policies"] = json!([]);
        assert!(serde_json::from_value::<DatasetPlan>(with_empty_axis).is_err());
        let solvers = [
            "bsgs",
            "enumerate",
            "root",
            "sat",
            "f4",
            "f5",
            "f6",
            "weil",
            "pkm",
            "double-large-prime",
        ];
        for i in 0..720 {
            let c = plan.cell(i).unwrap();
            assert_eq!(c.solver, solvers[(i % 10) as usize]);
            assert_eq!(c.twist, (i / 10) % 2 == 1);
            assert_eq!(c.m, [2, 4, 15][((i / 20) % 3) as usize]);
            assert_eq!(c.key_index, (i / 60) % 2);
            assert_eq!(c.generator_index, (i / 120) % 2);
            assert_eq!(c.curve_seed, 0);
            assert_eq!(c.family, ["koblitz", "binary", "prime"][(i / 240) as usize]);
            assert!(c.factor_base_policy.is_none());
            assert!(!serde_json::to_string(&c)
                .unwrap()
                .contains("factor_base_policy"));
        }
        assert_eq!(
            serde_json::to_string(&plan.cell(0).unwrap()).unwrap(),
            r#"{"family":"koblitz","n":23,"m":2,"twist":false,"curve_seed":0,"generator_index":0,"key_index":0,"seed":20261027,"factor_base_size":4,"degree_bound":3,"work_limit":20000,"solver":"bsgs"}"#
        );
        assert_eq!(
            serde_json::to_string(&plan.cell(719).unwrap()).unwrap(),
            r#"{"family":"prime","n":23,"m":15,"twist":true,"curve_seed":0,"generator_index":1,"key_index":1,"seed":20261027,"factor_base_size":4,"degree_bound":3,"work_limit":20000,"solver":"double-large-prime"}"#
        );
    }

    #[test]
    fn policy_axis_preserves_every_matched_problem_and_rejects_ambiguous_versions() {
        let legacy = plan();
        let mut explicit = legacy.clone();
        explicit.schema = "crypto.ml-dataset.plan/v2".into();
        assert!(explicit.validate().is_err());
        explicit.factor_base_policies = vec![
            FactorBasePolicy::Legacy {},
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 256,
            },
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4],
                candidate_limit: 8,
            },
        ];
        explicit.validate().unwrap();
        assert_eq!(
            explicit.cell_count().unwrap(),
            3 * legacy.cell_count().unwrap()
        );
        let solvers = legacy.solvers.len() as u64;
        for index in 0..legacy.cell_count().unwrap() {
            for policy_index in 0..3 {
                let new_index =
                    (index / solvers) * (solvers * 3) + policy_index * solvers + index % solvers;
                let mut cell = explicit.cell(new_index).unwrap();
                assert_eq!(
                    cell.factor_base_policy.take().unwrap(),
                    explicit.factor_base_policies[policy_index as usize]
                );
                assert_eq!(
                    serde_json::to_vec(&cell).unwrap(),
                    serde_json::to_vec(&legacy.cell(index).unwrap()).unwrap()
                );
            }
        }
        explicit.schema = legacy.schema.clone();
        assert!(explicit.validate().is_err());
        explicit.schema = "crypto.ml-dataset.plan/v2".into();
        explicit
            .factor_base_policies
            .push(FactorBasePolicy::Legacy {});
        assert!(explicit.validate().is_err());
        explicit.factor_base_policies = vec![FactorBasePolicy::BinarySubspaceIntersection {
            basis: vec![1, 2, 3],
            candidate_limit: 8,
        }];
        assert!(explicit.validate().is_err());
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
            self.failed_cell_at(p, binding, protocol, 1)
        }
        fn failed_cell_at(
            &self,
            p: &DatasetPlan,
            binding: &Value,
            protocol: &[u8],
            index: u64,
        ) -> PathBuf {
            let dir = self.0.join(format!("cells/{index:012}"));
            fs::create_dir_all(&dir).unwrap();
            let config = p.cell(index).unwrap();
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
                schema:"crypto.ml-dataset.cell/v1".into(),cell_index:index,plan_sha256:binding["plan_sha256"].as_str().unwrap().into(),
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
    fn orphan_root_certificate_is_rejected_even_after_valid_rehash() {
        let fixture = TempRun::new();
        let (plan, binding, protocol) = fixture.bound();
        let dir = fixture.failed_cell(&plan, &binding, &protocol);
        let mut manifest: CellManifest = read_json(&dir.join("manifest.json")).unwrap();
        manifest.artifacts.push(
            storage::write_json(
                &dir,
                "root-certificates.json",
                "prime-root-certificates/v1",
                &json!({"forged":true}),
            )
            .unwrap(),
        );
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        for replay in [false, true] {
            assert!(verify_cell(&dir, replay)
                .unwrap_err()
                .contains("prime-root certificate lacks"));
        }
        assert!(export::export(&fixture.0, &fixture.0.join("export"))
            .unwrap_err()
            .contains("prime-root certificate lacks"));
    }

    #[test]
    fn prime_root_outcome_contract_covers_found_exhausted_and_censored() {
        for (found, censored, all_queries, status, completed) in [
            (true, false, false, "verified", true),
            (true, false, true, "verified", true),
            (false, false, true, "partial", true),
            (false, true, false, "budget_exhausted", false),
        ] {
            let certificate = json!({"found_pdp":found,"censored":censored,
                "queries":[],"charged_work":0,
                "root_queries_completed":all_queries,"execution_trace_complete_verified":false,
                "total_operation_count_verified":false});
            let outcome = json!({"root_certificate_scope":solvers::PRIME_ROOT_CERTIFICATE_SCOPE,
                "root_execution_trace_complete_verified":false,"root_total_operation_count_verified":false,
                "trace_complete":false,"scalar_recovered_without_witness":false,
                "scalar_replay_verified":false,"ecdlp_pipeline_completed":false,
                "pdp_witness_verified":found,"solver_completed":completed,"status":status,"counters":{
                    "aggregate_root_field_and_group_work":0,"root_solver_calls":0,
                    "acknowledged_native_events":0,"acknowledged_native_event_bytes":0}});
            verify_prime_root_outcome(&outcome, &certificate).unwrap();
            for field in [
                "root_execution_trace_complete_verified",
                "root_total_operation_count_verified",
                "trace_complete",
                "scalar_recovered_without_witness",
                "scalar_replay_verified",
                "ecdlp_pipeline_completed",
            ] {
                let mut bad = outcome.clone();
                bad[field] = json!(true);
                assert!(
                    verify_prime_root_outcome(&bad, &certificate).is_err(),
                    "{field}"
                );
            }
            for (field, value) in [
                ("pdp_witness_verified", json!(!found)),
                ("solver_completed", json!(!completed)),
                ("status", json!("unsupported")),
                (
                    "root_certificate_scope",
                    json!("full execution trace verified"),
                ),
            ] {
                let mut bad = outcome.clone();
                bad[field] = value;
                assert!(
                    verify_prime_root_outcome(&bad, &certificate).is_err(),
                    "{field}"
                );
            }
        }
    }

    #[test]
    fn stored_prime_root_certificates_replay_both_twists_and_reject_rehashed_tampering() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture(&fixture, twist, 100_000);
            let manifest = verify_cell(&dir, true).unwrap();
            assert_eq!(manifest.outcome["pdp_witness_verified"], true);
            let root = manifest
                .artifacts
                .iter()
                .find(|a| a.role == "prime-root-certificates/v1")
                .unwrap();
            let bad = altered_cell(&dir, &fixture.0.join("bad-root"), |path, _| {
                alter_json(&path.join(&root.path), |certificate| {
                    let query = certificate["queries"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|q| {
                            q["report"]["roots"]
                                .as_array()
                                .is_some_and(|r| !r.is_empty())
                        })
                        .unwrap();
                    query["report"]["roots"].as_array_mut().unwrap().pop();
                });
            });
            verify_cell(&bad, false).unwrap();
            assert!(verify_cell(&bad, true).is_err());
            let bad = altered_cell(&dir, &fixture.0.join("bad-query"), |path, _| {
                alter_json(&path.join(&root.path), |certificate| {
                    let length = certificate["queries"][0]["fixed_x"].as_str().unwrap().len();
                    certificate["queries"][0]["fixed_x"] = json!("x".repeat(length));
                });
            });
            verify_cell(&bad, false).unwrap();
            assert!(verify_cell(&bad, true)
                .unwrap_err()
                .contains("root query identity"));
        }
    }

    #[test]
    fn stored_prime_root_requires_unique_role_and_encoding_bound_certificate() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture(&fixture, twist, 100_000);
            for mutation in [
                "missing",
                "missing_and_stripped",
                "wrong_role_and_stripped",
                "duplicate",
                "wrong_role",
                "wrong_encoding",
                "wrong_solver",
            ] {
                let bad = altered_cell(&dir, &fixture.0.join(mutation), |path, manifest| {
                    let index = manifest
                        .artifacts
                        .iter()
                        .position(|a| a.role == "prime-root-certificates/v1")
                        .unwrap();
                    match mutation {
                        "missing" | "missing_and_stripped" => {
                            manifest.artifacts.remove(index);
                        }
                        "duplicate" => {
                            let mut extra = manifest.artifacts[index].clone();
                            extra.path = "duplicate-root.json".into();
                            fs::copy(
                                path.join(&manifest.artifacts[index].path),
                                path.join(&extra.path),
                            )
                            .unwrap();
                            manifest.artifacts.push(extra);
                        }
                        "wrong_role" | "wrong_role_and_stripped" => {
                            manifest.artifacts[index].role = "unverified-root-summary".into()
                        }
                        "wrong_encoding" => manifest.artifacts[index].encoding = "jsonl".into(),
                        "wrong_solver" => {
                            let mut plan: DatasetPlan = read_json(&path.join("plan.json")).unwrap();
                            plan.solvers = vec!["enumerate".into()];
                            manifest.config = plan.cell(manifest.cell_index).unwrap();
                            manifest.plan_sha256 = plan.binding_sha256().unwrap();
                            let mut binding: Value = read_json(&path.join("run.json")).unwrap();
                            binding["plan_sha256"] = json!(manifest.plan_sha256);
                            fs::write(path.join("plan.json"), serde_json::to_vec(&plan).unwrap())
                                .unwrap();
                            fs::write(path.join("run.json"), serde_json::to_vec(&binding).unwrap())
                                .unwrap();
                            fs::write(
                                path.join("config.json"),
                                serde_json::to_vec(&manifest.config).unwrap(),
                            )
                            .unwrap();
                        }
                        _ => unreachable!(),
                    }
                    if mutation.ends_with("_stripped") {
                        for key in [
                            "root_certificate_scope",
                            "root_execution_trace_complete_verified",
                            "root_total_operation_count_verified",
                        ] {
                            manifest.outcome.as_object_mut().unwrap().remove(key);
                        }
                    }
                });
                for replay in [false, true] {
                    let error = verify_cell(&bad, replay).unwrap_err();
                    assert!(
                        error.contains("root outcome lacks")
                            || error.contains("prime-root certificate lacks"),
                        "{mutation}/{replay}: {error}"
                    );
                }
            }
        }
    }

    #[test]
    fn stored_prime_root_aggregate_labels_match_acknowledged_certificate_records() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture(&fixture, twist, 100_000);
            for key in [
                "aggregate_root_field_and_group_work",
                "root_solver_calls",
                "acknowledged_native_events",
                "acknowledged_native_event_bytes",
            ] {
                let bad = altered_cell(&dir, &fixture.0.join(key), |_, manifest| {
                    let value = manifest.outcome["counters"][key].as_u64().unwrap();
                    manifest.outcome["counters"][key] = json!(value + 1);
                });
                for replay in [false, true] {
                    assert!(
                        verify_cell(&bad, replay)
                            .unwrap_err()
                            .contains("prime-root outcome aggregate counter"),
                        "{key}/{replay}"
                    );
                }
            }
        }
    }

    #[test]
    fn stored_prime_root_auxiliary_payloads_match_certificate_calls_and_states() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture_at_size(&fixture, twist, 100_000, 11);
            let manifest = verify_cell(&dir, true).unwrap();
            for role in [
                "native-prime-root-call",
                "native-prime-root-intermediate-state",
            ] {
                let artifact = manifest.artifacts.iter().find(|a| a.role == role).unwrap();
                for mutation in ["identity", "native_payload"] {
                    let bad = altered_cell(
                        &dir,
                        &fixture.0.join(format!("{role}-{mutation}")),
                        |path, _| {
                            alter_json(&path.join(&artifact.path), |payload| {
                                if mutation == "identity" {
                                    payload["call_index"] = json!(999);
                                } else if role == "native-prime-root-call" {
                                    payload["report"]["polynomial"] = json!([0]);
                                } else {
                                    assert!(matches!(
                                        payload["native"]["kind"].as_str(),
                                        Some("power_end" | "gcd_end")
                                    ));
                                    payload["native"]["value"] = json!([999_999]);
                                }
                            });
                        },
                    );
                    // These are rehashed, independently saved payloads; the
                    // aggregate certificate and its arithmetic are unchanged.
                    verify_cell(&bad, false).unwrap();
                    assert!(verify_cell(&bad, true)
                        .unwrap_err()
                        .contains("prime root auxiliary payload differs"));
                }
            }
        }
    }

    #[test]
    fn stored_prime_root_auxiliaries_require_unique_expected_paths_roles_and_encodings() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture_at_size(&fixture, twist, 100_000, 11);
            for role in [
                "native-prime-root-call",
                "native-prime-root-intermediate-state",
            ] {
                for mutation in [
                    "missing",
                    "duplicate",
                    "wrong_role",
                    "wrong_encoding",
                    "wrong_path",
                ] {
                    let bad = altered_cell(
                        &dir,
                        &fixture.0.join(format!("{role}-{mutation}")),
                        |path, manifest| {
                            let index = manifest
                                .artifacts
                                .iter()
                                .position(|a| a.role == role)
                                .unwrap();
                            match mutation {
                                "missing" => {
                                    manifest.artifacts.remove(index);
                                }
                                "duplicate" => {
                                    let mut extra = manifest.artifacts[index].clone();
                                    extra.path = "duplicate-root-auxiliary.json".into();
                                    fs::copy(
                                        path.join(&manifest.artifacts[index].path),
                                        path.join(&extra.path),
                                    )
                                    .unwrap();
                                    manifest.artifacts.push(extra);
                                }
                                "wrong_role" => {
                                    manifest.artifacts[index].role =
                                        "unregistered-partial-artifact".into();
                                }
                                "wrong_encoding" => {
                                    manifest.artifacts[index].encoding = "jsonl".into()
                                }
                                "wrong_path" => {
                                    let old = path.join(&manifest.artifacts[index].path);
                                    manifest.artifacts[index].path =
                                        "renamed-root-auxiliary.json".into();
                                    fs::rename(old, path.join(&manifest.artifacts[index].path))
                                        .unwrap();
                                }
                                _ => unreachable!(),
                            }
                        },
                    );
                    for replay in [false, true] {
                        let error = verify_cell(&bad, replay).unwrap_err();
                        assert!(
                            error.contains("prime root auxiliary"),
                            "{role}/{mutation}/{replay}: {error}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn stored_prime_root_completed_queries_reject_byte_flag_and_extra_event_tampering() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture(&fixture, twist, 100_000);
            for mutation in [
                "flag_true",
                "flag_missing",
                "flag_nonboolean",
                "extra_event_field",
            ] {
                let bad = altered_cell(&dir, &fixture.0.join(mutation), |path, _| {
                    alter_json(&path.join("prime-root-certificates.json"), |certificate| {
                        let query = &mut certificate["queries"][0];
                        assert_eq!(query["report"]["stop"], "complete");
                        assert_eq!(query["trace_byte_cap_reached"], false);
                        match mutation {
                            "flag_true" => query["trace_byte_cap_reached"] = json!(true),
                            "flag_missing" => {
                                query
                                    .as_object_mut()
                                    .unwrap()
                                    .remove("trace_byte_cap_reached");
                            }
                            "flag_nonboolean" => query["trace_byte_cap_reached"] = json!("false"),
                            "extra_event_field" => {
                                query["report"]["events"][0]["unaccounted_payload"] =
                                    json!("unexpected");
                            }
                            _ => unreachable!(),
                        }
                    });
                });
                for replay in [false, true] {
                    let error = verify_cell(&bad, replay).unwrap_err();
                    assert!(
                        error.contains("byte-cap flag") || error.contains("unknown field"),
                        "{mutation}/{replay}: {error}"
                    );
                }
            }
        }
    }

    #[test]
    fn legacy_prime_root_summary_remains_compatible_without_native_claims() {
        let fixture = TempRun::new();
        let dir = prime_root_cell_fixture(&fixture, false, 100_000);
        let legacy = altered_cell(&dir, &fixture.0.join("legacy-root"), |path, manifest| {
            let certificate: Value = read_json(&path.join("prime-root-certificates.json")).unwrap();
            manifest.artifacts.retain(|a| {
                !matches!(
                    a.role.as_str(),
                    "prime-root-certificates/v1"
                        | "native-prime-root-call"
                        | "native-prime-root-intermediate-state"
                )
            });
            for key in [
                "root_certificate_scope",
                "root_execution_trace_complete_verified",
                "root_total_operation_count_verified",
            ] {
                manifest.outcome.as_object_mut().unwrap().remove(key);
            }
            let rows: Vec<_> = certificate["queries"].as_array().unwrap().iter().map(|q| json!({
                "kind":"root_result", "fixed_x":q["fixed_x"], "coefficients_constant_first":q["coefficients_constant_first"],
                "all_returned_roots":q["report"]["roots"], "checks":q["checks"]})).collect();
            manifest.artifacts.push(
                storage::write_json(
                    path,
                    "root-polynomials.json",
                    "native-univariate-inputs-roots-and-group-lifts",
                    &json!(rows),
                )
                .unwrap(),
            );
        });
        verify_cell(&legacy, false).unwrap();
        verify_cell(&legacy, true).unwrap();
    }

    #[test]
    fn stored_prime_root_censoring_cannot_be_promoted_by_rehashed_outcomes() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = prime_root_cell_fixture(&fixture, twist, 1);
            let original = verify_cell(&dir, true).unwrap();
            assert_eq!(original.outcome["status"], "budget_exhausted");
            for (field, value) in [
                ("root_execution_trace_complete_verified", json!(true)),
                ("root_total_operation_count_verified", json!(true)),
                ("trace_complete", json!(true)),
                ("solver_completed", json!(true)),
                ("pdp_witness_verified", json!(true)),
                ("scalar_recovered_without_witness", json!(true)),
                ("scalar_replay_verified", json!(true)),
                ("ecdlp_pipeline_completed", json!(true)),
                (
                    "root_certificate_scope",
                    json!("full execution trace verified"),
                ),
                ("status", json!("verified")),
            ] {
                let bad = altered_cell(&dir, &fixture.0.join(field), |_, manifest| {
                    manifest.outcome[field] = value;
                });
                for replay in [false, true] {
                    assert!(
                        verify_cell(&bad, replay)
                            .unwrap_err()
                            .contains("prime-root outcome labels"),
                        "{field}/{replay}"
                    );
                }
            }
        }
    }

    /// A real constructed curve/base and native solver certificate. Plant only
    /// a public sum; neither a construction scalar nor a witness reaches solve.
    fn prime_f4_cell_fixture(fixture: &TempRun, m: usize, twist: bool) -> PathBuf {
        let (mut plan, mut binding, protocol) = fixture.bound();
        plan.field_sizes = vec![7];
        plan.arities = vec![m];
        plan.solvers = vec!["f4".into()];
        plan.curve_seeds = 8;
        plan.generators = 2;
        plan.keys = 2;
        plan.seed = 42;
        plan.factor_base_size = 2;
        plan.degree_bound = 8;
        plan.work_limit = 10_000;
        plan.validate().unwrap();
        let index = 62 + u64::from(twist);
        let config = plan.cell(index).unwrap();
        assert_eq!(
            (config.curve_seed, config.generator_index, config.key_index),
            (7, 1, 1)
        );
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        binding["shard_index"] = json!(0);
        binding["shard_count"] = json!(1);

        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let arithmetic = curves::arithmetic(&case.public_instance).unwrap();
        let point = curves::point_words(&case.factor_base["points"][0]).unwrap();
        let mut sum = None;
        // Prefixes alternate P,2P,P,2P,... and are affine for odd prime r.
        // This constructs a group witness independently of S3 equations.
        for i in 0..m {
            let summand = if i < 2 || i % 2 == 1 {
                point
            } else {
                arithmetic.neg(point)
            };
            sum = arithmetic.reference_add(sum, summand);
            assert!(sum.is_some());
        }
        case.public_instance["target"] = curves::point_json(sum);
        case.public_instance["population"] = json!("software-test-public-planted-pdp");
        case.public_instance["known_answer_verified"] = json!(false);
        curves::verify_public(&case.public_instance).unwrap();
        curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
        assert!(case.private_witness.is_null());
        let solved = solvers::solve(&config, &case).unwrap();
        assert_eq!(
            solved.outcome["pdp_witness_verified"], true,
            "{}",
            solved.outcome
        );

        let dir = fixture.0.join(format!("cell-m{m}-twist{twist}"));
        fs::create_dir(&dir).unwrap();
        let mut artifacts = vec![
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
            storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol).unwrap(),
            storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            storage::write_json(
                &dir,
                "public.json",
                "public-instance",
                &case.public_instance,
            )
            .unwrap(),
            storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                .unwrap(),
        ];
        for path in ["attempt.json", "worker-process.json"] {
            artifacts.push(
                storage::write_json(&dir, path, "provenance", &json!({"software_test":true}))
                    .unwrap(),
            );
        }
        for path in ["stdout.log", "stderr.log"] {
            artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
        }
        for payload in solved.artifacts {
            artifacts.push(
                storage::write_bytes(
                    &dir,
                    &payload.path,
                    &payload.role,
                    &payload.encoding,
                    &payload.bytes,
                )
                .unwrap(),
            );
        }
        storage::write_json(
            &dir,
            "manifest.json",
            "manifest",
            &CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan.binding_sha256().unwrap(),
                config,
                outcome: solved.outcome,
                artifacts,
            },
        )
        .unwrap();
        assert!(!dir.join("witness.json").exists());
        verify_cell(&dir, true).unwrap();
        dir
    }

    fn wide_prime_f4_cell_fixture(fixture: &TempRun, twist: bool) -> PathBuf {
        wide_prime_f4_cell_fixture_kind(fixture, twist, false)
    }
    fn dynamic_frozen_cell_fixture(fixture: &TempRun, twist: bool, m: usize) -> PathBuf {
        let (mut plan, mut binding, protocol) = fixture.bound();
        plan.field_sizes = vec![47];
        plan.arities = vec![m];
        plan.solvers = vec!["f4-wide-dynamic-v1".into()];
        plan.curve_seeds = 8;
        plan.generators = 2;
        plan.keys = 2;
        plan.seed = 42;
        plan.factor_base_size = 2;
        plan.degree_bound = 12;
        plan.work_limit = 16384;
        plan.validate().unwrap();
        let index = 62 + u64::from(twist);
        let config = plan.cell(index).unwrap();
        assert_eq!(
            (config.curve_seed, config.generator_index, config.key_index),
            (7, 1, 1)
        );
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        binding["shard_index"] = json!(0);
        binding["shard_count"] = json!(1);
        binding["seconds_per_case"] = json!(30);
        binding["memory_mib"] = json!(256);
        binding["trace_mib"] = json!(128);
        binding["execution_context_required"] = json!(true);

        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let arithmetic = curves::arithmetic(&case.public_instance).unwrap();
        let point = curves::point_words(&case.factor_base["points"][0]).unwrap();
        let mut sum = None;
        for _ in 0..m {
            sum = arithmetic.reference_add(sum, point);
            assert!(sum.is_some());
        }
        case.public_instance["target"] = curves::point_json(sum);
        case.public_instance["population"] = json!("software-test-public-planted-pdp");
        case.public_instance["known_answer_verified"] = json!(false);
        curves::verify_public(&case.public_instance).unwrap();
        curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
        assert!(case.private_witness.is_null());

        let dir = fixture
            .0
            .join(format!("cell-dynamic-frozen-twist{twist}-m{m}"));
        fs::create_dir(&dir).unwrap();
        let mut artifacts = vec![
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
            storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol).unwrap(),
            storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            storage::write_json(
                &dir,
                "public.json",
                "public-instance",
                &case.public_instance,
            )
            .unwrap(),
            storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                .unwrap(),
        ];
        for path in ["attempt.json", "worker-process.json"] {
            artifacts.push(
                storage::write_json(&dir, path, "provenance", &if path=="attempt.json" {json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":index,"plan_sha256":plan.binding_sha256().unwrap()})} else {json!({"software_test":true})})
                    .unwrap(),
            );
        }
        for path in ["stdout.log", "stderr.log"] {
            artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
        }
        let launch = execution::create(&dir).unwrap();
        let (sha256, bytes) = crate::store::hash_file(&dir.join(execution::PATH)).unwrap();
        artifacts.push(Artifact {
            path: execution::PATH.into(),
            role: execution::ROLE.into(),
            encoding: "json".into(),
            sha256,
            bytes,
        });
        artifacts.push(telemetry::checked::install_policy(&dir).unwrap());
        let capture_binding = capture::AttemptBinding::from_cell_files(&dir).unwrap();
        let capture_limits = capture::capture_limits(128).unwrap();
        artifacts.push(
            storage::write_json(
                &dir,
                "capture-binding.json",
                "partial-capture-binding",
                &capture_binding,
            )
            .unwrap(),
        );
        artifacts.push(
            storage::write_json(
                &dir,
                "capture-limits.json",
                "partial-capture-limits",
                &capture_limits,
            )
            .unwrap(),
        );
        let context = execution::admit(&dir).unwrap().unwrap();
        assert_eq!(context.launch, launch);
        let solved = solvers::solve_with_bound_event_sink_context(
            &config,
            &case,
            &dir.join("live"),
            capture_binding,
            capture_limits,
            Some(&context),
        )
        .unwrap();
        if m == 4 {
            assert_eq!(
                solved.outcome["pdp_witness_verified"], true,
                "{}",
                solved.outcome
            );
            assert_eq!(
                solved.outcome["native_backend_invoked"], true,
                "{}",
                solved.outcome
            );
        } else {
            assert_eq!(
                solved.outcome["status"], "unsupported",
                "{}",
                solved.outcome
            );
            assert_eq!(solved.outcome["pdp_witness_verified"], false);
        }
        for payload in solved.artifacts {
            artifacts.push(
                storage::write_bytes(
                    &dir,
                    &payload.path,
                    &payload.role,
                    &payload.encoding,
                    &payload.bytes,
                )
                .unwrap(),
            );
        }
        artifacts.push(
            storage::write_json(&dir, "result.json", "solver-outcome", &solved.outcome).unwrap(),
        );
        artifacts.extend(
            collect_cell_artifacts(&dir)
                .unwrap()
                .into_iter()
                .filter(|a| a.path.starts_with("live/")),
        );
        storage::write_json(
            &dir,
            "manifest.json",
            "manifest",
            &CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan.binding_sha256().unwrap(),
                config,
                outcome: solved.outcome,
                artifacts,
            },
        )
        .unwrap();
        assert!(!dir.join("witness.json").exists());
        verify_cell(&dir, true).unwrap();
        dir
    }

    #[test]
    fn dynamic_frozen_run_replay_both_twists_and_censored_outcomes() {
        let fixture = TempRun::new();
        for twist in [false, true] {
            let dir = dynamic_frozen_cell_fixture(&fixture, twist, 4);
            let unsupported = dynamic_frozen_cell_fixture(&fixture, twist, 2);
            assert_eq!(
                verify_cell(&unsupported, true).unwrap().outcome["status"],
                "unsupported"
            );
            for status in ["timeout", "trace_budget_exhausted", "worker_failed"] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("dynamic-{twist}-{status}")),
                    |_, manifest| {
                        manifest.outcome = json!({"status":status,"solver_completed":false});
                    },
                );
                verify_cell(&changed, true).unwrap();
            }
            let changed = altered_cell(
                &dir,
                &fixture.0.join(format!("dynamic-{twist}-bad-input")),
                |path, manifest| {
                    manifest.outcome.as_object_mut().unwrap().remove("strategy");
                    alter_json(&path.join("prime-f4-input.json"), |value| {
                        value["schema"] = json!("invalid");
                    });
                },
            );
            assert!(verify_cell(&changed, true).is_err());
            let changed = altered_cell(
                &dir,
                &fixture.0.join(format!("dynamic-{twist}-wrong-attempt")),
                |path, _| {
                    alter_json(&path.join("capture-binding.json"), |value| {
                        value["attempt_sha256"] = json!("8".repeat(64));
                    });
                },
            );
            assert!(verify_cell(&changed, true).is_err());
        }
    }

    fn wide_prime_f4_cell_fixture_kind(fixture: &TempRun, twist: bool, negative: bool) -> PathBuf {
        let m = 2;
        let (mut plan, mut binding, protocol) = fixture.bound();
        plan.field_sizes = vec![61];
        plan.arities = vec![m];
        plan.solvers = vec!["f4".into()];
        plan.curve_seeds = 8;
        plan.generators = 2;
        plan.keys = 2;
        plan.seed = 42;
        plan.factor_base_size = 2;
        plan.degree_bound = 8;
        plan.work_limit = 50_000;
        plan.validate().unwrap();
        let index = 62 + u64::from(twist);
        let config = plan.cell(index).unwrap();
        assert_eq!(
            (config.curve_seed, config.generator_index, config.key_index),
            (7, 1, 1)
        );
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        binding["shard_index"] = json!(0);
        binding["shard_count"] = json!(1);
        binding["seconds_per_case"] = json!(30);
        binding["memory_mib"] = json!(512);
        binding["trace_mib"] = json!(64);
        binding["execution_context_required"] = json!(true);

        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let arithmetic = curves::arithmetic(&case.public_instance).unwrap();
        let point = curves::point_words(&case.factor_base["points"][0]).unwrap();
        let mut sum = None;
        // Prefixes alternate P,2P,P,2P,... and are affine for odd prime r.
        // This constructs a group witness independently of S3 equations.
        for i in 0..if negative { 3 } else { m } {
            let summand = if negative || i < 2 || i % 2 == 1 {
                point
            } else {
                arithmetic.neg(point)
            };
            sum = arithmetic.reference_add(sum, summand);
            assert!(sum.is_some());
        }
        case.public_instance["target"] = curves::point_json(sum);
        case.public_instance["population"] = json!("software-test-public-planted-pdp");
        case.public_instance["known_answer_verified"] = json!(false);
        curves::verify_public(&case.public_instance).unwrap();
        curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
        assert!(case.private_witness.is_null());

        let dir = fixture
            .0
            .join(format!("cell-wide-f4-twist{twist}-negative{negative}"));
        fs::create_dir(&dir).unwrap();
        let mut artifacts = vec![
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
            storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol).unwrap(),
            storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            storage::write_json(
                &dir,
                "public.json",
                "public-instance",
                &case.public_instance,
            )
            .unwrap(),
            storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                .unwrap(),
        ];
        for path in ["attempt.json", "worker-process.json"] {
            artifacts.push(
                storage::write_json(&dir, path, "provenance", &if path=="attempt.json" {json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":index,"plan_sha256":plan.binding_sha256().unwrap()})} else {json!({"software_test":true})})
                    .unwrap(),
            );
        }
        for path in ["stdout.log", "stderr.log"] {
            artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
        }
        let launch = execution::create(&dir).unwrap();
        let (sha256, bytes) = crate::store::hash_file(&dir.join(execution::PATH)).unwrap();
        artifacts.push(Artifact {
            path: execution::PATH.into(),
            role: execution::ROLE.into(),
            encoding: "json".into(),
            sha256,
            bytes,
        });
        let context = execution::admit(&dir).unwrap().unwrap();
        assert_eq!(context.launch, launch);
        let solved = solvers::solve_with_context(&config, &case, Some(&context)).unwrap();
        assert_eq!(
            solved.outcome["pdp_witness_verified"], !negative,
            "{}",
            solved.outcome
        );
        assert_eq!(
            solved.outcome["native_backend_invoked"], true,
            "{}",
            solved.outcome
        );
        for payload in solved.artifacts {
            artifacts.push(
                storage::write_bytes(
                    &dir,
                    &payload.path,
                    &payload.role,
                    &payload.encoding,
                    &payload.bytes,
                )
                .unwrap(),
            );
        }
        storage::write_json(
            &dir,
            "manifest.json",
            "manifest",
            &CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan.binding_sha256().unwrap(),
                config,
                outcome: solved.outcome,
                artifacts,
            },
        )
        .unwrap();
        assert!(!dir.join("witness.json").exists());
        verify_cell(&dir, true).unwrap();
        dir
    }

    fn wide_prime_f4_m3_cell_fixture_kind(
        fixture: &TempRun,
        twist: bool,
        negative: bool,
    ) -> PathBuf {
        let m = 3;
        let (mut plan, mut binding, protocol) = fixture.bound();
        plan.field_sizes = vec![61];
        plan.arities = vec![m];
        plan.solvers = vec!["f4".into()];
        plan.curve_seeds = 8;
        plan.generators = 2;
        plan.keys = 2;
        plan.seed = 42;
        plan.factor_base_size = 2;
        plan.degree_bound = 8;
        plan.work_limit = 50_000;
        plan.validate().unwrap();
        let index = 62 + u64::from(twist);
        let config = plan.cell(index).unwrap();
        assert_eq!(
            (config.curve_seed, config.generator_index, config.key_index),
            (7, 1, 1)
        );
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        binding["shard_index"] = json!(0);
        binding["shard_count"] = json!(1);
        binding["seconds_per_case"] = json!(30);
        binding["memory_mib"] = json!(512);
        binding["trace_mib"] = json!(64);
        binding["execution_context_required"] = json!(true);

        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let arithmetic = curves::arithmetic(&case.public_instance).unwrap();
        let point = curves::point_words(&case.factor_base["points"][0]).unwrap();
        let mut sum = None;
        // Prefixes alternate P,2P,P,2P,... and are affine for odd prime r.
        // This constructs a group witness independently of S3 equations.
        for i in 0..if negative { 4 } else { m } {
            let summand = if negative || i < m {
                point
            } else {
                arithmetic.neg(point)
            };
            sum = arithmetic.reference_add(sum, summand);
            assert!(sum.is_some());
        }
        case.public_instance["target"] = curves::point_json(sum);
        case.public_instance["population"] = json!("software-test-public-planted-pdp");
        case.public_instance["known_answer_verified"] = json!(false);
        curves::verify_public(&case.public_instance).unwrap();
        curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
        assert!(case.private_witness.is_null());

        let dir = fixture
            .0
            .join(format!("cell-wide-f4-m3-twist{twist}-negative{negative}"));
        fs::create_dir(&dir).unwrap();
        let mut artifacts = vec![
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
            storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol).unwrap(),
            storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            storage::write_json(
                &dir,
                "public.json",
                "public-instance",
                &case.public_instance,
            )
            .unwrap(),
            storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                .unwrap(),
        ];
        for path in ["attempt.json", "worker-process.json"] {
            artifacts.push(
                storage::write_json(&dir, path, "provenance", &if path=="attempt.json" {json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":index,"plan_sha256":plan.binding_sha256().unwrap()})} else {json!({"software_test":true})})
                    .unwrap(),
            );
        }
        for path in ["stdout.log", "stderr.log"] {
            artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
        }
        let launch = execution::create(&dir).unwrap();
        let (sha256, bytes) = crate::store::hash_file(&dir.join(execution::PATH)).unwrap();
        artifacts.push(Artifact {
            path: execution::PATH.into(),
            role: execution::ROLE.into(),
            encoding: "json".into(),
            sha256,
            bytes,
        });
        let context = execution::admit(&dir).unwrap().unwrap();
        assert_eq!(context.launch, launch);
        let solved = solvers::solve_with_context(&config, &case, Some(&context)).unwrap();
        assert_eq!(
            solved.outcome["pdp_witness_verified"], !negative,
            "{}",
            solved.outcome
        );
        assert_eq!(
            solved.outcome["native_backend_invoked"], true,
            "{}",
            solved.outcome
        );
        for payload in solved.artifacts {
            artifacts.push(
                storage::write_bytes(
                    &dir,
                    &payload.path,
                    &payload.role,
                    &payload.encoding,
                    &payload.bytes,
                )
                .unwrap(),
            );
        }
        storage::write_json(
            &dir,
            "manifest.json",
            "manifest",
            &CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan.binding_sha256().unwrap(),
                config,
                outcome: solved.outcome,
                artifacts,
            },
        )
        .unwrap();
        assert!(!dir.join("witness.json").exists());
        verify_cell(&dir, true).unwrap();
        dir
    }

    #[test]
    fn wide_prime_m3_central_rehashed_tamper_and_censoring() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = wide_prime_f4_m3_cell_fixture_kind(&fixture, twist, false);
            let negative = wide_prime_f4_m3_cell_fixture_kind(&fixture, twist, true);
            assert_eq!(
                verify_cell(&negative, true).unwrap().outcome["exhaustive_no_decomposition"],
                true
            );
            for (label, file, key, value) in [
                ("public-m", "public.json", "m", json!(2)),
                (
                    "aux-domain",
                    "prime-f4-input.json",
                    "domains",
                    json!([[1], [1], [1], [1]]),
                ),
                (
                    "prefix-claim",
                    "prime-f4-wide-m3-group-extraction.json",
                    "domain_complete",
                    json!(false),
                ),
                (
                    "counter",
                    "prime-f4-wide-m3-group-extraction.json",
                    "group_adds",
                    json!(0),
                ),
                (
                    "first-certificate",
                    "prime-f4-wide-m3-group-extraction.json",
                    "first_verified_cursor",
                    json!(7),
                ),
                (
                    "derivation",
                    "prime-f4-wide-m3-result.json",
                    "native_basis_derivation_verified",
                    json!(true),
                ),
            ] {
                let changed = altered_cell(&dir, &fixture.0.join(label), |path, manifest| {
                    let mut data: Value = read_json(&path.join(file)).unwrap();
                    data[key] = value;
                    let index = manifest
                        .artifacts
                        .iter()
                        .position(|a| a.path == file)
                        .unwrap();
                    let role = manifest.artifacts[index].role.clone();
                    fs::remove_file(path.join(file)).unwrap();
                    manifest.artifacts[index] =
                        storage::write_json(path, file, &role, &data).unwrap();
                });
                assert!(verify_cell(&changed, true).is_err(), "{label}");
            }
            for missing in [
                "public.json",
                "factor-base.json",
                "prime-f4-wide-m3-result.json",
            ] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("missing-{missing}")),
                    |_, manifest| manifest.artifacts.retain(|a| a.path != missing),
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            let omitted = altered_cell(
                &dir,
                &fixture.0.join("combined-omission"),
                |path, manifest| {
                    for name in [
                        "prime-f4-input.json",
                        "prime-f4-wide-m3-result.json",
                        "prime-f4-wide-m3-events.jsonl",
                        "prime-f4-wide-m3-group-extraction.json",
                    ] {
                        manifest.artifacts.retain(|a| a.path != name);
                        fs::remove_file(path.join(name)).unwrap();
                    }
                    manifest.outcome.as_object_mut().unwrap().remove("strategy");
                },
            );
            assert!(verify_cell(&omitted, true).is_err());
            let raw = altered_cell(
                &dir,
                &fixture.0.join("raw-censored-prefix"),
                |path, manifest| {
                    let paths = [
                        "prime-f4-input.json",
                        "prime-f4-wide-m3-result.json",
                        "prime-f4-wide-m3-events.jsonl",
                        "prime-f4-wide-m3-group-extraction.json",
                        "pdp-witness.json",
                        "solver-domain.json",
                    ];
                    for name in paths {
                        manifest.artifacts.retain(|a| a.path != name);
                        if path.join(name).exists() {
                            fs::remove_file(path.join(name)).unwrap();
                        }
                    }
                    manifest.artifacts.push(
                        storage::write_bytes(
                            path,
                            "prime-f4-input.json",
                            "unregistered-partial-artifact",
                            "native",
                            b"{\"schema\":",
                        )
                        .unwrap(),
                    );
                    manifest.outcome = json!({"status":"timeout","solver_completed":false});
                },
            );
            verify_cell(&raw, true).unwrap();
            for key in [
                "solver_completed",
                "group_domain_complete",
                "group_prefix_complete",
                "group_infinity_prefixes_covered",
                "first_certificate_retained",
            ] {
                let forged = altered_cell(
                    &raw,
                    &fixture.0.join(format!("raw-promoted-{key}")),
                    |_, manifest| manifest.outcome[key] = json!(true),
                );
                assert!(verify_cell(&forged, true).is_err());
            }
            let early = altered_cell(
                &raw,
                &fixture.0.join("raw-early-certificate"),
                |path, manifest| {
                    let nested = "live/payloads/pdp-witness.json";
                    let bytes = fs::read(dir.join("pdp-witness.json")).unwrap();
                    storage::write_bytes(
                        path,
                        nested,
                        "producer-role-not-registered",
                        "json",
                        &bytes,
                    )
                    .unwrap();
                    let collected = collect_cell_artifacts(path).unwrap();
                    let certificate = collected.into_iter().find(|a| a.path == nested).unwrap();
                    assert_eq!(certificate.role, "unregistered-partial-artifact");
                    assert_eq!(certificate.encoding, "native");
                    manifest.artifacts.push(certificate);
                },
            );
            verify_cell(&early, true).unwrap();
            let promoted = altered_cell(
                &early,
                &fixture.0.join("early-certificate-promoted"),
                |_, manifest| manifest.outcome["pdp_witness_verified"] = json!(true),
            );
            assert!(verify_cell(&promoted, true).is_err());
            let retained = altered_cell(&dir, &fixture.0.join("censored"), |path, manifest| {
                manifest.artifacts.push(
                    storage::write_json(path, "result.json", "solver-outcome", &manifest.outcome)
                        .unwrap(),
                );
                manifest.outcome = json!({"status":"timeout","solver_completed":false});
            });
            verify_cell(&retained, true).unwrap();
            for stop in ["missing_execution_context", "profile_admission"] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("false-admission-{stop}")),
                    |path, manifest| {
                        alter_json(&path.join("prime-f4-wide-m3-result.json"), |value| {
                            value["stop"] = json!(stop);
                            value["native_statistics_complete"] = json!(false);
                        });
                        manifest.outcome["termination"] = json!(stop);
                        manifest.outcome["native_statistics_complete"] = json!(false);
                        manifest.outcome["algebra_completed"] = json!(false);
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            for (field, counter, limit_key) in [
                ("work_reserved", "native_work_reserved", "work"),
                (
                    "bytes_reserved",
                    "native_cumulative_bytes_reserved",
                    "bytes",
                ),
            ] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("terminal-{field}")),
                    |path, manifest| {
                        let events =
                            fs::read_to_string(path.join("prime-f4-wide-m3-events.jsonl")).unwrap();
                        let terminal: Value =
                            serde_json::from_str(events.lines().last().unwrap()).unwrap();
                        assert_eq!(terminal["boundary"], "terminal");
                        let mut new_count = 0;
                        alter_json(&path.join("prime-f4-wide-m3-result.json"), |value| {
                            let old = value[field].as_u64().unwrap();
                            new_count = old + 1;
                            assert!(new_count <= value["limits"][limit_key].as_u64().unwrap());
                            value[field] = json!(new_count);
                        });
                        manifest.outcome["counters"][counter] = json!(new_count);
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            for kind in [
                "points",
                "negation_closed",
                "repetitions_allowed",
                "role",
                "missing",
            ] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("solver-domain-{kind}")),
                    |path, manifest| {
                        if kind == "missing" {
                            manifest
                                .artifacts
                                .retain(|a| a.path != "solver-domain.json");
                        } else if kind == "role" {
                            manifest
                                .artifacts
                                .iter_mut()
                                .find(|a| a.path == "solver-domain.json")
                                .unwrap()
                                .role = "incorrect-role".into();
                        } else {
                            alter_json(&path.join("solver-domain.json"), |value| {
                                if kind == "points" {
                                    value["points"].as_array_mut().unwrap().swap(0, 1);
                                } else {
                                    value[kind] = json!(false);
                                }
                            });
                        }
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            for key in [
                "native_work_reserved",
                "native_cumulative_bytes_reserved",
                "acknowledged_native_boundaries",
            ] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("counter-{key}")),
                    |_, manifest| {
                        let old = manifest.outcome["counters"][key].as_u64().unwrap();
                        manifest.outcome["counters"][key] = json!(old + 1);
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
                let changed = altered_cell(
                    &retained,
                    &fixture.0.join(format!("retained-counter-{key}")),
                    |path, _| {
                        alter_json(&path.join("result.json"), |value| {
                            let old = value["counters"][key].as_u64().unwrap();
                            value["counters"][key] = json!(old + 1);
                        });
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            for key in [
                "trace_complete",
                "scalar_replay_verified",
                "extraction_complete",
            ] {
                let changed = altered_cell(
                    &retained,
                    &fixture.0.join(format!("outer-{key}")),
                    |_, manifest| manifest.outcome[key] = json!(true),
                );
                assert!(verify_cell(&changed, true).is_err());
            }
            let changed = altered_cell(
                &retained,
                &fixture.0.join("retained-false-proof"),
                |path, manifest| {
                    let mut data: Value = read_json(&path.join("result.json")).unwrap();
                    data["last_fall_degree"] = json!(5);
                    let index = manifest
                        .artifacts
                        .iter()
                        .position(|a| a.path == "result.json")
                        .unwrap();
                    fs::remove_file(path.join("result.json")).unwrap();
                    manifest.artifacts[index] =
                        storage::write_json(path, "result.json", "solver-outcome", &data).unwrap();
                },
            );
            assert!(verify_cell(&changed, true).is_err());
            for file in ["prime-f4-input.json", "prime-f4-wide-m3-events.jsonl"] {
                let changed = altered_cell(
                    &dir,
                    &fixture.0.join(format!("oversize-{file}")),
                    |path, manifest| {
                        let index = manifest
                            .artifacts
                            .iter()
                            .position(|a| a.path == file)
                            .unwrap();
                        let a = manifest.artifacts[index].clone();
                        let mut bytes = fs::read(path.join(file)).unwrap();
                        bytes.extend(vec![b' '; 2 * 1024 * 1024 + 1]);
                        fs::remove_file(path.join(file)).unwrap();
                        manifest.artifacts[index] =
                            storage::write_bytes(path, file, &a.role, &a.encoding, &bytes).unwrap();
                        manifest.outcome.as_object_mut().unwrap().remove("strategy");
                    },
                );
                assert!(verify_cell(&changed, true).is_err());
            }
        }
    }

    #[test]
    fn wide_prime_f4_central_replay_and_rehashed_tamper_both_twists() {
        for twist in [false, true] {
            let fixture = TempRun::new();
            let dir = wide_prime_f4_cell_fixture(&fixture, twist);
            verify_cell(&dir, true).unwrap();
            let negative = wide_prime_f4_cell_fixture_kind(&fixture, twist, true);
            let negative_manifest = verify_cell(&negative, true).unwrap();
            assert_eq!(negative_manifest.outcome["status"], "partial");
            assert_eq!(
                negative_manifest.outcome["exhaustive_no_decomposition"],
                true
            );
            let mislabeled = altered_cell(
                &negative,
                &fixture.0.join("negative-mislabeled-verified"),
                |_, manifest| manifest.outcome["status"] = json!("verified"),
            );
            assert!(verify_cell(&mislabeled, true).is_err());
            let retained = altered_cell(
                &dir,
                &fixture.0.join("retained-censored"),
                |path, manifest| {
                    let record = storage::write_json(
                        path,
                        "result.json",
                        "solver-outcome",
                        &manifest.outcome,
                    )
                    .unwrap();
                    manifest.artifacts.push(record);
                    manifest.outcome = json!({"status":"timeout","solver_completed":false});
                },
            );
            for flag in [
                "native_backend_invoked",
                "algebra_completed",
                "native_statistics_complete",
                "extraction_complete",
            ] {
                let bad = altered_cell(
                    &retained,
                    &fixture.0.join(format!("outer-censored-{flag}")),
                    |_, manifest| manifest.outcome[flag] = json!(true),
                );
                assert!(
                    verify_cell(&bad, true).is_err(),
                    "outer censored outcome accepted {flag}"
                );
            }
            // First prove the unchanged registered producer result is valid
            // beneath outer censorship; each following failure is nonvacuous.
            verify_cell(&retained, true).unwrap();
            for field in [
                "trace_complete",
                "first_fall_degree",
                "last_fall_degree",
                "retired_hardware_instructions",
                "scalar_replay_verified",
                "ecdlp_pipeline_completed",
            ] {
                let bad = altered_cell(
                    &retained,
                    &fixture.0.join(format!("retained-forbidden-{field}")),
                    |path, _| {
                        alter_json(&path.join("result.json"), |v| {
                            v[field] = if [
                                "first_fall_degree",
                                "last_fall_degree",
                                "retired_hardware_instructions",
                            ]
                            .contains(&field)
                            {
                                json!(5)
                            } else {
                                json!(true)
                            }
                        })
                    },
                );
                assert!(
                    verify_cell(&bad, true).is_err(),
                    "retained result accepted {field}"
                );
            }
            let unbound_retained = altered_cell(
                &retained,
                &fixture.0.join("retained-unbound"),
                |_, manifest| manifest.artifacts.retain(|a| a.path != "result.json"),
            );
            assert!(verify_cell(&unbound_retained, true).is_err());
            let partial = altered_cell(
                &dir,
                &fixture.0.join("partial-censored"),
                |path, manifest| {
                    manifest.outcome = json!({"status":"timeout","solver_completed":false});
                    for artifact in &mut manifest.artifacts {
                        if [
                            "prime-f4-input.json",
                            "prime-f4-wide-result.json",
                            "prime-f4-wide-events.jsonl",
                            "prime-f4-root-extraction.json",
                        ]
                        .contains(&artifact.path.as_str())
                        {
                            artifact.role = "unregistered-partial-artifact".into();
                            artifact.encoding = "native".into();
                        }
                    }
                    fs::write(path.join("prime-f4-wide-result.json"), b"{incomplete").unwrap();
                },
            );
            verify_cell(&partial, true).unwrap();
            for claim in [
                "native_backend_invoked",
                "extraction_complete",
                "exhaustive_no_decomposition",
                "solver_completed",
            ] {
                let bad = altered_cell(
                    &dir,
                    &fixture.0.join(format!("all-omitted-{claim}")),
                    |path, manifest| {
                        for name in [
                            "prime-f4-input.json",
                            "prime-f4-wide-result.json",
                            "prime-f4-wide-events.jsonl",
                            "prime-f4-root-extraction.json",
                        ] {
                            fs::remove_file(path.join(name)).unwrap();
                            manifest.artifacts.retain(|a| a.path != name);
                        }
                        manifest.outcome.as_object_mut().unwrap().remove("strategy");
                        manifest.outcome[claim] = json!(true);
                    },
                );
                assert!(
                    verify_cell(&bad, true).is_err(),
                    "stripped artifacts accepted {claim}"
                );
            }
            let presolver = altered_cell(
                &dir,
                &fixture.0.join("presolver-censored"),
                |path, manifest| {
                    for name in [
                        "prime-f4-input.json",
                        "prime-f4-wide-result.json",
                        "prime-f4-wide-events.jsonl",
                        "prime-f4-root-extraction.json",
                    ] {
                        fs::remove_file(path.join(name)).unwrap();
                        manifest.artifacts.retain(|a| a.path != name);
                    }
                    manifest.outcome = json!({"status":"timeout","solver_completed":false});
                },
            );
            verify_cell(&presolver, true).unwrap();
            let setup_error = altered_cell(
                &presolver,
                &fixture.0.join("presolver-error"),
                |path, manifest| {
                    for name in ["pdp-witness.json", "solver-domain.json"] {
                        fs::remove_file(path.join(name)).unwrap();
                        manifest.artifacts.retain(|a| a.path != name);
                    }
                    manifest.outcome = json!({"status":"error","reason":"fixed software fixture: curve setup failed","solver_completed":false,"scalar_recovered_without_witness":false,"known_answer_verified":false,"factor_base_verified":false});
                    manifest.artifacts.push(
                        storage::write_json(
                            path,
                            "result.json",
                            "solver-outcome",
                            &manifest.outcome,
                        )
                        .unwrap(),
                    );
                },
            );
            verify_cell(&setup_error, true).unwrap();
            let hidden = altered_cell(
                &dir,
                &fixture.0.join("completed-relabeled-error"),
                |path, manifest| {
                    manifest.outcome = json!({"status":"error","reason":"fabricated","solver_completed":false,"scalar_recovered_without_witness":false});
                    manifest.artifacts.push(
                        storage::write_json(
                            path,
                            "result.json",
                            "solver-outcome",
                            &manifest.outcome,
                        )
                        .unwrap(),
                    );
                },
            );
            assert!(verify_cell(&hidden, true).is_err());

            for forbidden in [
                "trace_complete",
                "first_fall_degree",
                "last_fall_degree",
                "retired_hardware_instructions",
                "scalar_replay_verified",
                "ecdlp_pipeline_completed",
            ] {
                let bad = altered_cell(
                    &dir,
                    &fixture.0.join(format!("forbidden-{forbidden}")),
                    |_, manifest| {
                        manifest.outcome[forbidden] = if [
                            "first_fall_degree",
                            "last_fall_degree",
                            "retired_hardware_instructions",
                        ]
                        .contains(&forbidden)
                        {
                            json!(5)
                        } else {
                            json!(true)
                        }
                    },
                );
                assert!(verify_cell(&bad, true).is_err(), "accepted {forbidden}");
                let bad_partial = altered_cell(
                    &partial,
                    &fixture.0.join(format!("partial-forbidden-{forbidden}")),
                    |_, manifest| manifest.outcome[forbidden] = json!(true),
                );
                assert!(verify_cell(&bad_partial, true).is_err());
            }
            for mutation in ["public-m", "omit-public", "omit-base"] {
                let bad =
                    altered_cell(
                        &dir,
                        &fixture.0.join(mutation),
                        |path, manifest| match mutation {
                            "public-m" => {
                                alter_json(&path.join("public.json"), |v| v["m"] = json!(3))
                            }
                            "omit-public" => manifest.artifacts.retain(|a| a.path != "public.json"),
                            "omit-base" => {
                                manifest.artifacts.retain(|a| a.path != "factor-base.json")
                            }
                            _ => unreachable!(),
                        },
                    );
                assert!(verify_cell(&bad, true).is_err(), "accepted {mutation}");
            }
            for extra in [0usize, 1] {
                let padded = altered_cell(
                    &dir,
                    &fixture.0.join(format!("aggregate-pad-{extra}")),
                    |path, manifest| {
                        let record: Value =
                            read_json(&path.join("prime-f4-wide-result.json")).unwrap();
                        let cap = record["limits"]["encoded_bytes"].as_u64().unwrap() as usize;
                        let total: usize = manifest
                            .artifacts
                            .iter()
                            .filter(|a| {
                                [
                                    "prime-f4-input.json",
                                    "prime-f4-wide-result.json",
                                    "prime-f4-wide-events.jsonl",
                                    "prime-f4-root-extraction.json",
                                    "pdp-witness.json",
                                ]
                                .contains(&a.path.as_str())
                            })
                            .map(|a| a.bytes as usize)
                            .sum();
                        let mut bytes = fs::read(path.join("prime-f4-wide-result.json")).unwrap();
                        bytes.extend(std::iter::repeat_n(b' ', cap - total + extra));
                        fs::write(path.join("prime-f4-wide-result.json"), bytes).unwrap();
                    },
                );
                assert_eq!(
                    verify_cell(&padded, true).is_ok(),
                    extra == 0,
                    "aggregate boundary{extra}"
                );
            }
            for excess in [false, true] {
                let bad = altered_cell(
                    &dir,
                    &fixture.0.join(format!("event-padding-{excess}")),
                    |path, _| {
                        let record: Value =
                            read_json(&path.join("prime-f4-wide-result.json")).unwrap();
                        let cap = (record["limits"]["encoded_bytes"].as_u64().unwrap() / 8)
                            .min(256 * 1024) as usize;
                        let mut bytes = fs::read(path.join("prime-f4-wide-events.jsonl")).unwrap();
                        let count = if excess { cap + 1 - bytes.len() } else { 1 };
                        bytes.extend(std::iter::repeat_n(b'\n', count));
                        fs::write(path.join("prime-f4-wide-events.jsonl"), bytes).unwrap();
                    },
                );
                assert!(verify_cell(&bad, true).is_err());
            }
            for mutation in [
                "equation", "limit", "basis", "counter", "outcome", "deadline", "unknown", "omit",
                "relocate", "encoding",
            ] {
                let bad =
                    altered_cell(
                        &dir,
                        &fixture.0.join(mutation),
                        |path, manifest| match mutation {
                            "equation" => alter_json(&path.join("prime-f4-input.json"), |v| {
                                v["polynomials"][0][0]["coefficient"] = json!("2305843009213693951")
                            }),
                            "limit" => alter_json(&path.join("prime-f4-wide-result.json"), |v| {
                                v["limits"]["work"] = json!(1)
                            }),
                            "basis" => alter_json(&path.join("prime-f4-wide-result.json"), |v| {
                                v["native_basis_derivation_verified"] = json!(true)
                            }),
                            "counter" => alter_json(&path.join("prime-f4-wide-result.json"), |v| {
                                v["acknowledged_events"] = json!(999999)
                            }),
                            "outcome" => {
                                manifest.outcome["total_operation_count_verified"] = json!(true)
                            }
                            "deadline" => alter_json(&path.join(execution::PATH), |v| {
                                v["deadline_monotonic_ns"] = json!(0)
                            }),
                            "unknown" => alter_json(&path.join("prime-f4-wide-result.json"), |v| {
                                v["extra"] = json!(true)
                            }),
                            "omit" => manifest
                                .artifacts
                                .retain(|a| a.path != "prime-f4-wide-result.json"),
                            "relocate" => {
                                let a = manifest
                                    .artifacts
                                    .iter_mut()
                                    .find(|a| a.path == "prime-f4-wide-result.json")
                                    .unwrap();
                                fs::rename(path.join(&a.path), path.join("relocated.json"))
                                    .unwrap();
                                a.path = "relocated.json".into();
                            }
                            "encoding" => {
                                manifest
                                    .artifacts
                                    .iter_mut()
                                    .find(|a| a.path == "prime-f4-wide-result.json")
                                    .unwrap()
                                    .encoding = "native".into()
                            }
                            _ => unreachable!(),
                        },
                    );
                assert!(verify_cell(&bad, true).is_err(), "accepted {mutation}");
            }
        }
    }

    #[test]
    fn stored_binary_root_replays_odd_even_families_twists_and_rejects_rehashed_roots() {
        for family in ["binary", "koblitz"] {
            for n in [11, 12] {
                for twist in [false, true] {
                    let fixture = TempRun::new();
                    let dir = root_cell_fixture(&fixture, family, twist, 100_000, n);
                    let manifest = verify_cell(&dir, true).unwrap();
                    assert_eq!(manifest.outcome["pdp_witness_verified"], true);
                    assert!(manifest
                        .artifacts
                        .iter()
                        .any(|a| a.role == "native-binary-root-intermediate-state"));
                    let bad = altered_cell(&dir, &fixture.0.join("bad-roots"), |path, _| {
                        alter_json(&path.join("binary-root-certificates.json"), |certificate| {
                            let query = certificate["queries"]
                                .as_array_mut()
                                .unwrap()
                                .iter_mut()
                                .find(|q| !q["report"]["roots"].as_array().unwrap().is_empty())
                                .unwrap();
                            query["report"]["roots"].as_array_mut().unwrap().pop();
                        });
                    });
                    verify_cell(&bad, false).unwrap();
                    assert!(verify_cell(&bad, true).is_err());
                }
            }
        }
    }

    #[test]
    fn stored_binary_root_requires_unique_bound_certificate_even_when_labels_are_stripped() {
        for family in ["binary", "koblitz"] {
            for twist in [false, true] {
                let fixture = TempRun::new();
                let dir = root_cell_fixture(&fixture, family, twist, 100_000, 12);
                for mutation in [
                    "missing",
                    "missing_and_stripped",
                    "wrong_role_and_stripped",
                    "wrong_encoding",
                    "duplicate",
                    "wrong_family_role",
                    "wrong_solver",
                ] {
                    let bad = altered_cell(&dir, &fixture.0.join(mutation), |path, manifest| {
                        let index = manifest
                            .artifacts
                            .iter()
                            .position(|a| a.role == "binary-root-certificates/v1")
                            .unwrap();
                        match mutation {
                            "missing" | "missing_and_stripped" => {
                                manifest.artifacts.remove(index);
                            }
                            "wrong_role_and_stripped" => {
                                manifest.artifacts[index].role =
                                    "unregistered-partial-artifact".into()
                            }
                            "wrong_encoding" => manifest.artifacts[index].encoding = "jsonl".into(),
                            "duplicate" => {
                                let mut extra = manifest.artifacts[index].clone();
                                extra.path = "duplicate-binary-root.json".into();
                                fs::copy(
                                    path.join(&manifest.artifacts[index].path),
                                    path.join(&extra.path),
                                )
                                .unwrap();
                                manifest.artifacts.push(extra);
                            }
                            "wrong_family_role" => {
                                manifest.artifacts[index].role = "prime-root-certificates/v1".into()
                            }
                            "wrong_solver" => {
                                let mut plan: DatasetPlan =
                                    read_json(&path.join("plan.json")).unwrap();
                                plan.solvers = vec!["enumerate".into()];
                                manifest.config = plan.cell(manifest.cell_index).unwrap();
                                manifest.plan_sha256 = plan.binding_sha256().unwrap();
                                let mut binding: Value = read_json(&path.join("run.json")).unwrap();
                                binding["plan_sha256"] = json!(manifest.plan_sha256);
                                fs::write(
                                    path.join("plan.json"),
                                    serde_json::to_vec(&plan).unwrap(),
                                )
                                .unwrap();
                                fs::write(
                                    path.join("run.json"),
                                    serde_json::to_vec(&binding).unwrap(),
                                )
                                .unwrap();
                                fs::write(
                                    path.join("config.json"),
                                    serde_json::to_vec(&manifest.config).unwrap(),
                                )
                                .unwrap();
                            }
                            _ => unreachable!(),
                        }
                        if mutation.ends_with("_stripped") {
                            for key in [
                                "root_certificate_scope",
                                "root_execution_trace_complete_verified",
                                "root_total_operation_count_verified",
                            ] {
                                manifest.outcome.as_object_mut().unwrap().remove(key);
                            }
                        }
                    });
                    for replay in [false, true] {
                        let error = verify_cell(&bad, replay).unwrap_err();
                        assert!(
                            error.contains("root outcome lacks")
                                || error.contains("binary-root certificate lacks"),
                            "{family}/{twist}/{mutation}/{replay}: {error}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn stored_binary_root_auxiliaries_bind_payloads_roles_encodings_and_inventory() {
        for family in ["binary", "koblitz"] {
            for twist in [false, true] {
                let fixture = TempRun::new();
                let dir = root_cell_fixture(&fixture, family, twist, 100_000, 12);
                for role in [
                    "native-binary-root-call",
                    "native-binary-root-intermediate-state",
                ] {
                    for mutation in [
                        "identity",
                        "native_payload",
                        "missing",
                        "duplicate",
                        "wrong_role",
                        "wrong_encoding",
                        "wrong_path",
                    ] {
                        let bad = altered_cell(
                            &dir,
                            &fixture.0.join(format!("{role}-{mutation}")),
                            |path, manifest| {
                                let index = manifest
                                    .artifacts
                                    .iter()
                                    .position(|a| a.role == role)
                                    .unwrap();
                                match mutation {
                                    "identity" | "native_payload" => {
                                        alter_json(
                                            &path.join(&manifest.artifacts[index].path),
                                            |payload| {
                                                if mutation == "identity" {
                                                    payload["call_index"] = json!(999);
                                                } else if role == "native-binary-root-call" {
                                                    payload["report"]["polynomial"] =
                                                        json!([999_999]);
                                                } else {
                                                    payload["native"] = json!({"kind":"normalized", "argument":999_999});
                                                }
                                            },
                                        );
                                    }
                                    "missing" => {
                                        manifest.artifacts.remove(index);
                                    }
                                    "duplicate" => {
                                        let mut extra = manifest.artifacts[index].clone();
                                        extra.path = "duplicate-binary-auxiliary.json".into();
                                        fs::copy(
                                            path.join(&manifest.artifacts[index].path),
                                            path.join(&extra.path),
                                        )
                                        .unwrap();
                                        manifest.artifacts.push(extra);
                                    }
                                    "wrong_role" => {
                                        manifest.artifacts[index].role =
                                            "unregistered-partial-artifact".into()
                                    }
                                    "wrong_encoding" => {
                                        manifest.artifacts[index].encoding = "jsonl".into()
                                    }
                                    "wrong_path" => {
                                        let old = path.join(&manifest.artifacts[index].path);
                                        manifest.artifacts[index].path =
                                            "renamed-binary-auxiliary.json".into();
                                        fs::rename(old, path.join(&manifest.artifacts[index].path))
                                            .unwrap();
                                    }
                                    _ => unreachable!(),
                                }
                            },
                        );
                        if matches!(mutation, "identity" | "native_payload") {
                            verify_cell(&bad, false).unwrap();
                        } else {
                            assert!(verify_cell(&bad, false)
                                .unwrap_err()
                                .contains("binary root auxiliary"));
                        }
                        assert!(verify_cell(&bad, true)
                            .unwrap_err()
                            .contains("binary root auxiliary"));
                    }
                }
            }
        }
    }

    #[test]
    fn stored_binary_root_caps_censoring_and_outcome_labels_survive_rehashed_tampering() {
        for family in ["binary", "koblitz"] {
            for twist in [false, true] {
                let fixture = TempRun::new();
                let complete = root_cell_fixture(&fixture, family, twist, 100_000, 11);
                for mutation in [
                    "byte_flag",
                    "missing_byte_flag",
                    "extra_event_field",
                    "event_cap",
                    "byte_cap",
                    "joint_event_count",
                    "joint_event_bytes",
                ] {
                    let bad =
                        altered_cell(&complete, &fixture.0.join(mutation), |path, manifest| {
                            alter_json(
                                &path.join("binary-root-certificates.json"),
                                |certificate| {
                                    assert_eq!(
                                        certificate["queries"][0]["report"]["stop"],
                                        "complete"
                                    );
                                    match mutation {
                                        "byte_flag" => {
                                            certificate["queries"][0]["trace_byte_cap_reached"] =
                                                json!(true)
                                        }
                                        "missing_byte_flag" => {
                                            certificate["queries"][0]
                                                .as_object_mut()
                                                .unwrap()
                                                .remove("trace_byte_cap_reached");
                                        }
                                        "extra_event_field" => {
                                            certificate["queries"][0]["report"]["events"][0]
                                                ["unaccounted_payload"] = json!("extra")
                                        }
                                        "event_cap" | "byte_cap" => {
                                            let key = if mutation == "event_cap" {
                                                "event_cap"
                                            } else {
                                                "event_byte_cap"
                                            };
                                            certificate[key] =
                                                json!(certificate[key].as_u64().unwrap() + 1);
                                        }
                                        "joint_event_count" | "joint_event_bytes" => {
                                            let key = if mutation == "joint_event_count" {
                                                "acknowledged_native_events"
                                            } else {
                                                "acknowledged_native_event_bytes"
                                            };
                                            certificate[key] =
                                                json!(certificate[key].as_u64().unwrap() + 1);
                                            manifest.outcome["counters"][key] =
                                                certificate[key].clone();
                                        }
                                        _ => unreachable!(),
                                    }
                                },
                            );
                        });
                    for replay in [false, true] {
                        assert!(
                            verify_cell(&bad, replay).is_err(),
                            "{family}/{twist}/{mutation}/{replay}"
                        );
                    }
                }
                for key in [
                    "aggregate_root_primitive_and_group_work",
                    "root_solver_calls",
                    "acknowledged_native_events",
                    "acknowledged_native_event_bytes",
                ] {
                    let bad = altered_cell(&complete, &fixture.0.join(key), |_, manifest| {
                        manifest.outcome["counters"][key] =
                            json!(manifest.outcome["counters"][key].as_u64().unwrap() + 1);
                    });
                    for replay in [false, true] {
                        assert!(verify_cell(&bad, replay)
                            .unwrap_err()
                            .contains("binary-root outcome aggregate"));
                    }
                }
                let censored = root_cell_fixture(&fixture, family, twist, 1, 11);
                assert_eq!(
                    verify_cell(&censored, true).unwrap().outcome["status"],
                    "budget_exhausted"
                );
                for (field, value) in [
                    ("root_execution_trace_complete_verified", json!(true)),
                    ("root_total_operation_count_verified", json!(true)),
                    ("trace_complete", json!(true)),
                    ("solver_completed", json!(true)),
                    ("pdp_witness_verified", json!(true)),
                    ("scalar_recovered_without_witness", json!(true)),
                    ("scalar_replay_verified", json!(true)),
                    ("ecdlp_pipeline_completed", json!(true)),
                    ("status", json!("verified")),
                ] {
                    let bad = altered_cell(&censored, &fixture.0.join(field), |_, manifest| {
                        manifest.outcome[field] = value;
                    });
                    for replay in [false, true] {
                        assert!(verify_cell(&bad, replay)
                            .unwrap_err()
                            .contains("root outcome labels"));
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_binary_root_summaries_remain_compatible_without_native_claims() {
        for family in ["binary", "koblitz"] {
            let fixture = TempRun::new();
            let dir = root_cell_fixture(&fixture, family, false, 100_000, 11);
            let legacy = altered_cell(
                &dir,
                &fixture.0.join("legacy-binary-root"),
                |path, manifest| {
                    let certificate: Value =
                        read_json(&path.join("binary-root-certificates.json")).unwrap();
                    manifest.artifacts.retain(|a| {
                        !matches!(
                            a.role.as_str(),
                            "binary-root-certificates/v1"
                                | "native-binary-root-call"
                                | "native-binary-root-intermediate-state"
                        )
                    });
                    for key in [
                        "root_certificate_scope",
                        "root_execution_trace_complete_verified",
                        "root_total_operation_count_verified",
                    ] {
                        manifest.outcome.as_object_mut().unwrap().remove(key);
                    }
                    let rows: Vec<_> = certificate["queries"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|q| {
                            json!({
                    "kind":"root_result", "fixed_x":q["fixed_x"],
                    "coefficients_constant_first":q["coefficients_constant_first"],
                    "all_returned_roots":q["report"]["roots"], "checks":q["checks"]})
                        })
                        .collect();
                    manifest.artifacts.push(
                        storage::write_json(
                            path,
                            "root-polynomials.json",
                            "native-univariate-inputs-roots-and-group-lifts",
                            &json!(rows),
                        )
                        .unwrap(),
                    );
                },
            );
            verify_cell(&legacy, false).unwrap();
            verify_cell(&legacy, true).unwrap();
        }
    }

    /// A real constructed curve/base and native solver certificate. Plant only
    /// a public sum; neither a construction scalar nor a witness reaches solve.
    fn prime_root_cell_fixture(fixture: &TempRun, twist: bool, work_limit: u64) -> PathBuf {
        prime_root_cell_fixture_at_size(fixture, twist, work_limit, 7)
    }

    fn prime_root_cell_fixture_at_size(
        fixture: &TempRun,
        twist: bool,
        work_limit: u64,
        field_size: u32,
    ) -> PathBuf {
        root_cell_fixture(fixture, "prime", twist, work_limit, field_size)
    }

    fn root_cell_fixture(
        fixture: &TempRun,
        family: &str,
        twist: bool,
        work_limit: u64,
        field_size: u32,
    ) -> PathBuf {
        let m = 2;
        let (mut plan, mut binding, protocol) = fixture.bound();
        plan.families = vec![family.into()];
        plan.field_sizes = vec![field_size];
        plan.arities = vec![m];
        plan.solvers = vec!["root".into()];
        plan.curve_seeds = 8;
        plan.generators = 2;
        plan.keys = 2;
        plan.seed = 42;
        plan.factor_base_size = if family == "prime" { 2 } else { 8 };
        plan.degree_bound = 8;
        plan.work_limit = work_limit;
        plan.validate().unwrap();
        let index = 62 + u64::from(twist);
        let config = plan.cell(index).unwrap();
        assert_eq!(
            (config.curve_seed, config.generator_index, config.key_index),
            (7, 1, 1)
        );
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        binding["shard_index"] = json!(0);
        binding["shard_count"] = json!(1);

        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let arithmetic = curves::arithmetic(&case.public_instance).unwrap();
        let point = if family == "prime" {
            curves::point_words(&case.factor_base["points"][0]).unwrap()
        } else {
            case.factor_base["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| curves::point_words(p).unwrap())
                .min_by_key(|p| p.expect("affine factor-base point").0)
                .unwrap()
        };
        let mut sum = None;
        // Prefixes alternate P,2P,P,2P,... and are affine for odd prime r.
        // This constructs a group witness independently of S3 equations.
        for i in 0..m {
            let summand = if i < 2 || i % 2 == 1 {
                point
            } else {
                arithmetic.neg(point)
            };
            sum = arithmetic.reference_add(sum, summand);
            assert!(sum.is_some());
        }
        case.public_instance["target"] = curves::point_json(sum);
        case.public_instance["population"] = json!("software-test-public-planted-pdp");
        case.public_instance["known_answer_verified"] = json!(false);
        curves::verify_public(&case.public_instance).unwrap();
        curves::verify_factor_base(&case.public_instance, &case.factor_base).unwrap();
        assert!(case.private_witness.is_null());
        let solved = solvers::solve(&config, &case).unwrap();
        assert_eq!(
            solved.outcome["pdp_witness_verified"],
            work_limit == 100_000,
            "{}",
            solved.outcome
        );

        let dir = fixture.0.join(format!(
            "cell-root-{family}-n{field_size}-twist{twist}-work{work_limit}"
        ));
        fs::create_dir(&dir).unwrap();
        let mut artifacts = vec![
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
            storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol).unwrap(),
            storage::write_json(&dir, "config.json", "config", &config).unwrap(),
            storage::write_json(
                &dir,
                "public.json",
                "public-instance",
                &case.public_instance,
            )
            .unwrap(),
            storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                .unwrap(),
        ];
        for path in ["attempt.json", "worker-process.json"] {
            artifacts.push(
                storage::write_json(&dir, path, "provenance", &json!({"software_test":true}))
                    .unwrap(),
            );
        }
        for path in ["stdout.log", "stderr.log"] {
            artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
        }
        for payload in solved.artifacts {
            artifacts.push(
                storage::write_bytes(
                    &dir,
                    &payload.path,
                    &payload.role,
                    &payload.encoding,
                    &payload.bytes,
                )
                .unwrap(),
            );
        }
        storage::write_json(
            &dir,
            "manifest.json",
            "manifest",
            &CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index: index,
                plan_sha256: plan.binding_sha256().unwrap(),
                config,
                outcome: solved.outcome,
                artifacts,
            },
        )
        .unwrap();
        assert!(!dir.join("witness.json").exists());
        verify_cell(&dir, true).unwrap();
        dir
    }

    /// Re-seal deliberately changed bytes, so replay failures cannot be
    /// explained merely by stale hashes or lengths in the artifact manifest.
    fn altered_cell(
        source: &Path,
        destination: &Path,
        edit: impl FnOnce(&Path, &mut CellManifest),
    ) -> PathBuf {
        fs::create_dir(destination).unwrap();
        let mut manifest: CellManifest = read_json(&source.join("manifest.json")).unwrap();
        for artifact in &manifest.artifacts {
            let path = destination.join(&artifact.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::copy(source.join(&artifact.path), path).unwrap();
        }
        edit(destination, &mut manifest);
        for artifact in &mut manifest.artifacts {
            (artifact.sha256, artifact.bytes) =
                crate::store::hash_file(&destination.join(&artifact.path)).unwrap();
        }
        fs::write(
            destination.join("manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        storage::verify_manifest(destination, &manifest.artifacts).unwrap();
        destination.to_owned()
    }

    fn alter_json(path: &Path, edit: impl FnOnce(&mut Value)) {
        let mut value: Value = read_json(path).unwrap();
        edit(&mut value);
        fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
    }

    #[test]
    fn central_prime_f4_replays_generated_m3_m4_and_legacy_m2_cells() {
        for m in [2, 3, 4] {
            for twist in [false, true] {
                let fixture = TempRun::new();
                let dir = prime_f4_cell_fixture(&fixture, m, twist);
                let replayed = verify_cell(&dir, true).unwrap();
                assert_eq!(replayed.outcome["pdp_witness_verified"], true);
                let input: Value = read_json(&dir.join("prime-f4-input.json")).unwrap();
                if m == 2 {
                    assert!(input.get("schema").is_none());
                    assert!(input.get("strategy").is_none());
                } else {
                    assert_eq!(input["schema"], "ml-dataset.prime-f4-chain-input/v1");
                    assert_eq!(replayed.outcome["exhaustive_no_decomposition"], false);
                }
                let bad = altered_cell(&dir, &fixture.0.join("bad-certificate"), |dir, _| {
                    alter_json(&dir.join("pdp-witness.json"), |witness| {
                        witness["summands"][0] = Value::Null
                    });
                });
                verify_cell(&bad, false).unwrap();
                assert!(verify_cell(&bad, true).is_err());
            }
        }
    }

    #[test]
    fn central_prime_chain_rejects_rehashed_equation_domain_and_schema_tampering() {
        let fixture = TempRun::new();
        let source = prime_f4_cell_fixture(&fixture, 3, false);
        for mutation in [
            "coefficient",
            "leaf-domain",
            "auxiliary-domain",
            "membership",
            "roles",
            "schema-removed",
            "schema-changed",
            "strategy-removed",
            "strategy-changed",
        ] {
            let bad = altered_cell(&source, &fixture.0.join(mutation), |dir, manifest| {
                // The frozen configuration still identifies a chain when the
                // mutable outcome declaration has been stripped as well.
                manifest.outcome.as_object_mut().unwrap().remove("strategy");
                alter_json(&dir.join("prime-f4-input.json"), |input| match mutation {
                    "coefficient" => input["polynomials"][0][0]["coefficient"] = json!("0"),
                    "leaf-domain" => input["domains"][0] = Value::Null,
                    "auxiliary-domain" => input["domains"][3] = input["domains"][0].clone(),
                    "membership" => input["membership_variables"][0] = json!(3),
                    "roles" => input["variable_roles"][3]["role"] = json!("factor_base_leaf"),
                    "schema-removed" => {
                        input.as_object_mut().unwrap().remove("schema");
                    }
                    "schema-changed" => input["schema"] = json!("legacy-reference"),
                    "strategy-removed" => {
                        input.as_object_mut().unwrap().remove("strategy");
                    }
                    "strategy-changed" => input["strategy"] = json!("legacy-reference"),
                    _ => unreachable!(),
                });
            });
            verify_cell(&bad, false).unwrap();
            let error = verify_cell(&bad, true).unwrap_err();
            assert!(
                error.contains("prime F4 chain input differs"),
                "{mutation}: {error}"
            );
        }
    }

    #[test]
    fn central_prime_chain_binds_reconstructed_input_to_frozen_cell_configuration() {
        let first = TempRun::new();
        let second = TempRun::new();
        let m3 = prime_f4_cell_fixture(&first, 3, false);
        let m4 = prime_f4_cell_fixture(&second, 4, false);
        // Transplant valid m4 public algebra and a valid m4 group certificate
        // into a frozen m3 cell. Neither artifact's own replay is sufficient.
        let bad = altered_cell(&m3, &first.0.join("valid-other-arity"), |dir, _| {
            for path in ["public.json", "prime-f4-input.json", "pdp-witness.json"] {
                fs::copy(m4.join(path), dir.join(path)).unwrap();
            }
        });
        verify_cell(&bad, false).unwrap();
        // The shared original-input guard now rejects the transplanted public
        // arity before the later prime-chain reconstruction guard runs.
        assert_eq!(
            verify_cell(&bad, true).unwrap_err(),
            "factor base or public identity differs from frozen config"
        );

        let changed_config = altered_cell(&m3, &first.0.join("changed-config"), |dir, manifest| {
            manifest.config.m = 4;
            fs::write(
                dir.join("config.json"),
                serde_json::to_vec(&manifest.config).unwrap(),
            )
            .unwrap();
        });
        assert!(verify_cell(&changed_config, true)
            .unwrap_err()
            .contains("frozen plan"));
    }

    #[test]
    fn central_prime_chain_requires_artifacts_even_when_strategy_is_stripped() {
        let fixture = TempRun::new();
        let source = prime_f4_cell_fixture(&fixture, 3, false);
        for (label, path) in [
            ("missing-input", "prime-f4-input.json"),
            ("missing-public", "public.json"),
            ("missing-base", "factor-base.json"),
            ("missing-certificate", "pdp-witness.json"),
        ] {
            let bad = altered_cell(&source, &fixture.0.join(label), |dir, manifest| {
                manifest.outcome.as_object_mut().unwrap().remove("strategy");
                manifest.artifacts.retain(|artifact| artifact.path != path);
                fs::remove_file(dir.join(path)).unwrap();
            });
            assert!(verify_cell(&bad, false).is_err(), "{label}");
            assert!(verify_cell(&bad, true).is_err(), "{label}");
        }
        // Every individual execution indicator must survive loss of the
        // mutable strategy and all three named equation/trace artifacts.
        for indicator in [
            "native_backend_invoked",
            "pdp_witness_verified",
            "solver_completed",
        ] {
            let bad = altered_cell(&source, &fixture.0.join(indicator), |dir, manifest| {
                manifest.outcome = json!({"status":"partial"});
                manifest.outcome[indicator] = json!(true);
                for path in [
                    "prime-f4-input.json",
                    "prime-f4-chain-result.json",
                    "prime-f4-chain-events.jsonl",
                ] {
                    manifest.artifacts.retain(|artifact| artifact.path != path);
                    fs::remove_file(dir.join(path)).unwrap();
                }
            });
            assert!(
                verify_cell(&bad, true)
                    .unwrap_err()
                    .contains("equation/domain artifact"),
                "{indicator}"
            );
        }

        let early = TempRun::new();
        let (mut plan, mut binding, protocol) = early.bound();
        plan.arities = vec![3];
        plan.solvers = vec!["f4".into()];
        binding["plan_sha256"] = json!(plan.binding_sha256().unwrap());
        let dir = early.failed_cell(&plan, &binding, &protocol);
        verify_cell(&dir, true).unwrap();
        let unsupported = altered_cell(&dir, &early.0.join("early-unsupported"), |_, manifest| {
            manifest.outcome = json!({"status":"unsupported", "native_backend_invoked":false,
                "pdp_witness_verified":false,"solver_completed":false,
                "reason":"constructor stopped before producing an algebraic input"});
        });
        verify_cell(&unsupported, true).unwrap();
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

#[cfg(test)]
mod worker_receipt_adoption_tests {
    use super::*;
    fn data(paths: &[&str]) -> types::SolverData {
        types::SolverData {
            outcome: json!({"status":"partial"}),
            artifacts: paths
                .iter()
                .map(|path| types::ArtifactPayload {
                    path: (*path).into(),
                    role: "test-payload".into(),
                    encoding: "native".into(),
                    bytes: b"new".to_vec(),
                })
                .collect(),
        }
    }
    fn record(path: &str) -> Artifact {
        Artifact {
            path: path.into(),
            role: "test-receipt".into(),
            encoding: "native".into(),
            sha256: digest(b"stored"),
            bytes: 6,
        }
    }
    #[test]
    fn worker_receipt_union_rejects_reserved_and_duplicate_paths_before_writes() {
        let absent = std::env::temp_dir().join("worker-receipt-validation-must-not-create");
        for path in [
            "result.json",
            "public.json",
            "telemetry/x",
            "publication/x",
            execution::PATH,
        ] {
            for stored_channel in [false, true] {
                let mut registry = Vec::with_capacity(32);
                let (pending, stored) = if stored_channel {
                    (data(&[]), vec![record(path)])
                } else {
                    (data(&[path]), vec![])
                };
                assert!(
                    adopt_worker_solver_artifacts(&absent, &mut registry, pending, stored).is_err()
                );
                assert!(registry.is_empty());
            }
        }
        for mode in 0..4 {
            let mut registry = Vec::with_capacity(32);
            if mode == 3 {
                registry.push(record("same.bin"));
            }
            let stored = if mode == 0 {
                vec![record("same.bin"), record("same.bin")]
            } else if mode == 2 || mode == 3 {
                vec![record("same.bin")]
            } else {
                vec![]
            };
            let pending = if mode == 1 {
                data(&["same.bin", "same.bin"])
            } else if mode == 2 {
                data(&["same.bin"])
            } else {
                data(&[])
            };
            let before = registry.clone();
            assert!(
                adopt_worker_solver_artifacts(&absent, &mut registry, pending, stored).is_err()
            );
            if mode == 2 {
                assert_eq!(registry, vec![record("same.bin")]);
            } else {
                assert_eq!(registry, before);
            }
        }
    }
    #[test]
    fn worker_receipts_survive_pending_validation_failures_without_new_writes() {
        let root = std::env::temp_dir().join(format!(
            "worker-receipt-validation-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let stored =
            storage::write_bytes(&root, "ack.bin", "test-receipt", "native", b"stored").unwrap();
        for paths in [
            vec!["first.bin", "result.json"],
            vec!["first.bin", "duplicate.bin", "duplicate.bin"],
            vec!["first.bin", "ack.bin"],
        ] {
            let mut registry = Vec::with_capacity(32);
            assert!(adopt_worker_solver_artifacts(
                &root,
                &mut registry,
                data(&paths),
                vec![stored.clone()]
            )
            .is_err());
            assert_eq!(registry, vec![stored.clone()]);
            storage::verify_manifest(&root, &registry).unwrap();
            assert_eq!(fs::read(root.join("ack.bin")).unwrap(), b"stored");
            assert!(!root.join("first.bin").exists());
            assert!(!root.join("duplicate.bin").exists());
            assert!(!root.join("result.json").exists());
            assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn real_terminal_telemetry_failure_retains_native_receipt_and_censors_outcome() {
        let root = std::env::temp_dir().join(format!(
            "worker-telemetry-censor-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        // Real legacy telemetry backend: its final immutable write fails after
        // the operation has produced and stored an artifact.
        storage::write_bytes(
            &root,
            "telemetry/solver.json",
            "control-conflict",
            "native",
            b"occupied",
        )
        .unwrap();
        let mut registry = Vec::with_capacity(40);
        let mut produced = None;
        let measurement = telemetry::measured(&root, &mut registry, "solver", || {
            let receipt =
                storage::write_bytes(&root, "ack.bin", "test-receipt", "native", b"stored")?;
            let mut output = data(&["must-not-write.bin"]);
            output.outcome = json!({
                "adapter_id":"koblitz-m3-native-v1", "strategy":"koblitz-m3-native-v1",
                "status":"verified", "solver_completed":true,
                "pdp_witness_verified":true,"retained_pdp_witness_verified":true,
                "certificate_durably_persisted":true,"group_domain_complete":true,
                "native_backend_invoked":true,"native_statistics_complete":true,
                "algebra_completed":true,"performed_group_additions":195,
                "live_persistence_errors":["earlier retained condition"]
            });
            produced = Some((output, vec![receipt]));
            Ok(())
        });
        assert!(measurement.is_err());
        let (data, roots) = produced.expect("operation ran before terminal telemetry failed");
        let outcome =
            finish_worker_solver_artifacts(&root, &mut registry, data, roots, measurement).unwrap();
        assert_eq!(outcome["status"], "worker_failed");
        for key in [
            "solver_completed",
            "pdp_witness_verified",
            "certificate_durably_persisted",
            "group_domain_complete",
            "native_backend_invoked",
            "native_statistics_complete",
            "algebra_completed",
        ] {
            assert_eq!(outcome[key], false, "{key}");
        }
        assert_eq!(outcome["retained_pdp_witness_verified"], true);
        assert_eq!(outcome["performed_group_additions"], 195);
        assert_eq!(
            outcome["live_persistence_errors"],
            json!(["earlier retained condition"])
        );
        assert!(outcome["worker_telemetry_error"].is_string());
        assert_eq!(registry.iter().filter(|a| a.path == "ack.bin").count(), 1);
        storage::verify_manifest(&root, &registry).unwrap();
        assert!(!root.join("must-not-write.bin").exists());
        assert_eq!(
            fs::read(root.join("telemetry/solver.json")).unwrap(),
            b"occupied"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn worker_receipts_survive_later_pending_storage_failure() {
        let root = std::env::temp_dir().join(format!(
            "worker-receipt-adoption-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let stored =
            storage::write_bytes(&root, "ack.bin", "test-receipt", "native", b"stored").unwrap();
        storage::write_bytes(&root, "conflict.bin", "test-conflict", "native", b"old").unwrap();
        let mut registry = Vec::with_capacity(32);
        assert!(adopt_worker_solver_artifacts(
            &root,
            &mut registry,
            data(&["conflict.bin"]),
            vec![stored.clone()]
        )
        .is_err());
        assert_eq!(registry, vec![stored]);
        assert_eq!(fs::read(root.join("ack.bin")).unwrap(), b"stored");
        assert_eq!(fs::read(root.join("conflict.bin")).unwrap(), b"old");
        storage::verify_manifest(&root, &registry).unwrap();
        fs::remove_dir_all(root).unwrap();
    }
}
